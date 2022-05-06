use clap::{App, Arg, ArgMatches};
use std::{fs, io, path::Path, vec};

#[derive(Debug)]
enum CLIResultType {
    File,
    Directory,
}

#[derive(Debug)]
struct CLIResult {
    file_stem: String,
    full_path: String,
    kind: CLIResultType,
}

impl CLIResult {
    fn new(file_stem: String, full_path: String, kind: CLIResultType) -> CLIResult {
        CLIResult {
            file_stem: String::from(file_stem),
            full_path: String::from(full_path),
            kind,
        }
    }
}

fn cli<'a>() -> ArgMatches<'a> {
    let matches = App::new("Temper")
    .version("0.1")
    .about("A CLI-based tool that searches for temp directories or application directories based on an inputted string of text. Intended for Windows machines.")
    .arg(Arg::with_name("search term")
        .short("s").long("search")
        .help("The search term that will be compared against directories and files")
        .takes_value(true)
        .required(true),
        )
        .get_matches();
    matches
}

fn dir_search(search_term: &str) -> io::Result<Vec<CLIResult>> {
    let base_dir = match home::home_dir() {
        Some(path) => path.display().to_string(),
        None => "Unable to get home directory".to_string(),
    };

    let search_paths = [
        base_dir.to_owned(),
        (base_dir.to_owned() + "\\AppData\\Local"),
        (base_dir.to_owned() + "\\AppData\\LocalLow"),
        (base_dir.to_owned() + "\\AppData\\Roaming"),
        (base_dir.to_owned() + "\\AppData\\Local\\Programs"),
        (base_dir.to_owned() + "\\AppData\\Roaming\\Microsoft\\Windows\\Start Menu\\Programs"),
    ];

    let mut search_matches: Vec<CLIResult> = vec![];

    for search_path in search_paths {
        for entry in fs::read_dir(Path::new(&search_path))? {
            let path = entry.expect("Unable to get entry").path();

            let result_name = path.file_stem().unwrap().to_owned().into_string().unwrap();

            let was_matched = result_name
                .to_lowercase()
                .contains(&search_term.to_ascii_lowercase());
            if was_matched {
                search_matches.push(CLIResult::new(
                    result_name,
                    path.display().to_string(),
                    match path.is_file() {
                        true => CLIResultType::File,
                        false => CLIResultType::Directory,
                    },
                ));
            }
        }
    }
    Ok(search_matches)
}

fn main() {
    if cfg!(target_os = "windows") {
        let matches = cli();
        let search_term = matches.value_of("search term").unwrap();

        let search_result = dir_search(search_term).unwrap();
        println!("Results: {:#?}", search_result);
    } else {
        println!("Please use a Windows machine");
    }
}
