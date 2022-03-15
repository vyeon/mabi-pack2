use clap::{arg, Command};

mod common;
mod encryption;
mod extract;
mod list;
//mod pack;

fn main() {
    let args = Command::new("Mabinogi pack utilities 2")
        .version("v1.0.0")
        .author("regomne <fallingsunz@gmail.com>")
        .subcommand(
            Command::new("pack")
                .about("Create a .it pack (not supported yet)")
                .arg(arg!(-i --input <FOLDER> "Set the input folder to pack"))
                .arg(arg!(-o --output <PACK_NAME> "Set the output .it file name")),
        )
        .subcommand(
            Command::new("extract")
                .about("Extract a .it pack")
                .arg(arg!(-i --input <PACK_NAME> "Set the input pack name to extract"))
                .arg(arg!(-o --output <FOLDER> "Set the output folder"))
                .arg(
                    arg!(-f --filter <FILTER> ... "Set a filter when extracting, in regexp, multiple occurrences mean OR")
                        .required(false)
                        .number_of_values(1)
                )
                .arg(
                    arg!(-c --validate_checksum "validate checksum of files, break when failed")
                ),
        )
        .subcommand(
            Command::new("list")
                .about("Output the file list of a .it pack")
                .arg(arg!(-i --input <PACK_NAME> "Set the input pack name to extract"))
                .arg(
                    arg!(-o --output <LIST_FILE_NAME> "Set the list file name, output to stdout if not set")
                        .required(false)
                )
                .arg(
                    arg!(-c --validate_checksum "validate checksum of files, break when failed")
                ),
        )
        .get_matches();

    let ret = match if let Some(matches) = args.subcommand_matches("list") {
        list::run_list(
            matches.value_of("input").unwrap(),
            matches.value_of("output"),
            matches.is_present("validate_checksum"),
        )
    } else if let Some(matches) = args.subcommand_matches("extract") {
        extract::run_extract(
            matches.value_of("input").unwrap(),
            matches.value_of("output").unwrap(),
            matches
                .values_of("filter")
                .map(|e| e.collect())
                .unwrap_or(vec![]),
            matches.is_present("validate_checksum"),
        )
    } else if let Some(_matches) = args.subcommand_matches("pack") {
        //pack::run_pack(
        //    matches.value_of("input").unwrap(),
        //    matches.value_of("output").unwrap(),
        //)
        println!("packing not supported yet");
        Ok(())
    } else {
        println!("please select a subcommand (type --help to get details)");
        Ok(())
    } {
        Err(e) => {
            println!("Err: {:?}", e);
            1
        }
        _ => 0,
    };
    std::process::exit(ret);
}