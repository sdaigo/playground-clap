use clap::{load_yaml, App};

fn main() {
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from(yaml).get_matches();

    match matches.subcommand_name() {
        Some("media") => {
            // detect subcommand Lv1:
            // ex) % app media
            let media = matches.subcommand_matches("media").unwrap();

            // detect subcommand Lv2:
            // ex) % app media convert
            let convert = media.subcommand_matches("convert").unwrap();

            // it's safe to unwrap since `file` argument is required by clap
            println!("file: {}", convert.value_of("file").unwrap());
        }

        Some("fortune") => {
            let _fortune = matches.subcommand_matches("fortune").unwrap();

            println!("fortune is run");
        }

        None => println!("No subcommand was used"),

        _ => println!("??"),
    }
}
