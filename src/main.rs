mod cli;
mod file;
mod md5;

fn main() {

    cli::parse_cli();
    // file_list.iter().for_each(|file| {
    //     if !all_files.contains_key(file) {
    //         println!("{} not found", file);
    //     } else {
    //         println!("{} found", file);
    //     }
    // });
}

// fn parse_cli() -> PathBuf {
//     let arg = Command::new(crate_name!())
//         .version(crate_version!())
//         .author(env!("CARGO_PKG_AUTHORS"))
//         .about(crate_description!())
//         .arg(
//             Arg::new("input")
//                 .help("Sets the input file to use")
//                 .required(true),
//         )
//         .get_matches();
//     let input = PathBuf::from(arg.value_of("input").unwrap());
//     input
// }


