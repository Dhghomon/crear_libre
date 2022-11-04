use std::io::Write;

fn trovar_ligament(version: OldNov, libre: &str, capitul: i32) -> Result<String, String> {
    let version = version.s();
    let url = format!(
        "https://wikisource.org/wiki/Biblia/{version}_Testamento/{libre}/{libre}_{capitul}"
    );
    match ureq::get(&url).call() {
        Ok(response) => {
            let response = response.into_string().unwrap();
            let mut is_body = false;
            let mut output = String::new();
            for line in response.lines() {
                if line.contains("<body") {
                    is_body = true;
                }
                if line.contains("NewPP limit report") {
                    break;
                }
                if is_body {
                    output.push_str(line);
                }
            }
            Ok(output.trim_end_matches("<!--").into())
        }
        Err(_) => Err(format!("capitul {capitul} a {url}")),
    }
}

#[derive(Copy, Clone)]
enum OldNov {
    Vetule,
    Nove,
}

impl OldNov {
    fn s(&self) -> String {
        match self {
            OldNov::Vetule => "Vetule".into(),
            OldNov::Nove => "Nove".into(),
        }
    }
}

impl TryFrom<&str> for OldNov {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "vetule" => Ok(Self::Vetule),
            "nove" => Ok(Self::Nove),
            _ => Err("Ples scrir Vetule o Nove".into()),
        }
    }
}

fn main() {
    let mut args = std::env::args().skip(1);
    let old_nov =
        OldNov::try_from(args.next().expect("Ples scrir Vetule o Nove").as_str()).unwrap();
    let libre = args.next().expect("Ples scrir li nómine del libre");
    let nómine_del_file = args.next().expect("Ples scrir li nómine del file");
 

    println!(
        "Un file nominat {nómine_del_file} va esser creat. Si un file con li sam nómine ja existe,
it va esset deletet. Ples rescrir li nómine del file por confirmar"
    );
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    if input.trim() != nómine_del_file {
        panic!("Alor on ne va crear li file. Exeant li programma");
    }
    let mut file = std::fs::File::create(nómine_del_file.clone()).unwrap();

    for capitul in 1..=50 {
        match trovar_ligament(old_nov, &libre, capitul) {
            Ok(resultate) => {
                write!(file, "{}\n\n", resultate).unwrap();
                println!("Capitul {capitul} finit");
            }
            Err(loc) => println!("Null contenete trovat por {loc}"),
        }
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}
