use clap::{App, Arg, SubCommand};
use pmd_script_entry_list::{ScriptEntryList};
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    env_logger::init();
    let matches = App::new("farctool")
        .about("tool for reading or writing the script_entry_list.bin file (used in PSMD/GTI)")
        .arg(
            Arg::with_name("input")
                .short("i")
                .long("input")
                .help("the input file of the program")
                .required(true)
                .value_name("INPUT"),
        )
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("output")
                .help("the output directory")
                .required(true)
                .value_name("OUTPUT"),
        )
        .subcommand(
            SubCommand::with_name("tojson")
                .about("dump the flow file to a json file, that can be modified"),
        )
        .subcommand(
            SubCommand::with_name("fromjson")
                .about("write a flow file from the json file in input"),
        )
        .get_matches();

    let input_path = PathBuf::from(matches.value_of("input").unwrap());
    let mut input_file = File::open(&input_path).unwrap();

    if matches.subcommand_matches("tojson").is_some() {
        let script_entry_list = ScriptEntryList::new_from_file(&mut input_file).unwrap();

        let serialized = serde_json::to_string(&script_entry_list).unwrap();

        let output_path = PathBuf::from(matches.value_of("output").unwrap());
        let mut output_file = File::create(&output_path).unwrap();
        output_file.write_all(serialized.as_bytes()).unwrap();

    } else if matches.subcommand_matches("fromjson").is_some() {
        let script_entry_list = serde_json::from_reader::<_, ScriptEntryList>(input_file).unwrap();

        let output_path = PathBuf::from(matches.value_of("output").unwrap());
        let mut output_file = File::create(&output_path).unwrap();
        script_entry_list.write_to_file(&mut output_file).unwrap();
    } else {
        panic!("the selected mode is neither tojson or fromjson");
    };

    println!("script_entry_list_tool finished sucessfully");
}
