fn main() {
    let arguments = clap::Command::new("cat")
        .arg(
            clap::Arg::new("FILES")
                .multiple_occurrences(true)
                .default_value("-"),
        )
        .arg(
            clap::Arg::new("show-ends")
                .short('E').long("show-ends")
                .help("display $ at end of each line"),
        )
        .arg(
            clap::Arg::new("show-tabs")
                .short('T').long("show-tabs")
                .help("display TAB characters as ^I"),
        )
        .arg(
            clap::Arg::new("squeeze-blank")
                .short('s').long("squeeze-blank")
                .help("suppress repeated empty output lines"),
        )
        .arg(
            clap::Arg::new("number")
                .short('n').long("number")
                .help("number all output lines"),
        )
        .arg(
            clap::Arg::new("number-nonblank")
                .short('b').long("number-nonblank")
                .help("number nonempty output lines, overrides -n"),
        )
        .arg(
            clap::Arg::new("show-nonprinting")
            .short('v').long("show-nonprinting").help("use ^ and M- notation, except for LFD and TAB")
        )
        .get_matches();
    let readers = arguments
        .values_of("FILES")
        .unwrap()
        .map(|x| std::fs::File::open(x).unwrap())
        .collect::<Vec<std::fs::File>>();

    let option_mapping = std::collections::HashMap::from([
        ("show-ends", catlib::Options::ShowEnds),
        ("show-tabs", catlib::Options::ShowTabs),
        ("squeeze-blank", catlib::Options::SqueezeBlank),
        ("number", catlib::Options::Number),
        ("number-nonblank", catlib::Options::NumberNonblank),
        ("show-nonprinting", catlib::Options::ShowNonprinting),
    ]);

    let option_mapping = option_mapping
        .into_iter()
        .filter(|x| arguments.is_present(x.0))
        .map(|x| x.1)
        .collect::<std::collections::HashSet<catlib::Options>>();

    catlib::cat(std::io::stdout(), readers, option_mapping);
}
