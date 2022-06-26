#[derive(Eq, Hash, PartialEq, Debug)]
pub enum Options {
    ShowTabs,
    ShowEnds,
    SqueezeBlank,
    Number,
    NumberNonblank,
    ShowNonprinting,
}

pub fn cat<W: std::io::Write, R: std::io::Read>(
    mut output: W,
    inputs: Vec<R>,
    options: std::collections::HashSet<Options>,
) {
    for input in inputs {
        let mut prev_char: char = '\n';
        let mut idx = 0;

        let mut lineno = || {
            idx += 1;
            idx
        };

        let caret = |ch: char, force_m: bool| {
            if (ch as u8 >= 32 && ch as u8 <= 126)
                || (!force_m && (ch as u8 == 9 || ch as u8 == 10))
            {
                format!("{}", ch)
            } else if ch as u8 == 127 {
                "^?".to_owned()
            } else {
                format!("^{}", ((ch as u8) + 64) as char)
            }
        };
        let non_print_str = |ch: char| {
            if (ch as u8) >= 128u8 {
                format!("M-{}", caret(((ch as u8) - 128u8) as char, true))
            } else {
                format!("{}", caret(ch, false))
            }
        };

        for c in input.bytes() {
            let ch = *c.as_ref().unwrap() as char;

            if prev_char == '\n' {
                // first character in new line
                // before this do line numbering.
                if options.contains(&Options::Number)
                    || (options.contains(&Options::NumberNonblank) && ch != '\n')
                {
                    write!(output, "{:>6}  ", lineno()).unwrap();
                }
            }

            if (ch == '\t') && options.contains(&Options::ShowTabs) {
                output.write("^I".as_bytes()).unwrap();
            } else if ch == '\n' {
                if prev_char == '\n' && options.contains(&Options::SqueezeBlank) {
                    continue;
                }
                if options.contains(&Options::ShowEnds) {
                    output.write("$\n".as_bytes()).unwrap();
                } else {
                    output.write("\n".as_bytes()).unwrap();
                }
            } else {
                if options.contains(&Options::ShowNonprinting) {
                    write!(output, "{}", non_print_str(c.unwrap() as char)).unwrap();
                } else {
                    write!(output, "{}", c.unwrap() as char).unwrap();
                }
            }
            prev_char = ch;
        }
    }
}

mod tests {
    #[test]
    fn cat_simple() {
        let read_buf: Vec<u8> = Vec::from("hello\nworld".as_bytes());
        let mut write_buf: Vec<u8> = Vec::new();
        let options = std::collections::HashSet::new();
        crate::cat(&mut write_buf, vec![read_buf.as_slice()], options);

        assert_eq!(
            String::from_utf8(write_buf).unwrap(),
            String::from_utf8(read_buf).unwrap()
        );
    }
    #[test]
    fn cat_multiple() {
        let read_buf: Vec<u8> = Vec::from("hello\nworld".as_bytes());
        let read_buf2: Vec<u8> = Vec::from("\tbye bye\nworld".as_bytes());
        let read_buf3: Vec<u8> = Vec::from("foo\nbar".as_bytes());
        let mut write_buf: Vec<u8> = Vec::new();
        let options = std::collections::HashSet::new();
        crate::cat(
            &mut write_buf,
            vec![
                read_buf.as_slice(),
                read_buf2.as_slice(),
                read_buf3.as_slice(),
            ],
            options,
        );
        let res: String = String::from_utf8(read_buf).unwrap()
            + &String::from_utf8(read_buf2).unwrap()
            + &String::from_utf8(read_buf3).unwrap();
        assert_eq!(String::from_utf8(write_buf).unwrap(), res);
    }

    #[test]
    fn cat_show_ends() {
        let read_buf: Vec<u8> = Vec::from("hello\nworld".as_bytes());
        let read_buf2: Vec<u8> = Vec::from("\tbye bye\nworld".as_bytes());
        let read_buf3: Vec<u8> = Vec::from("foo\nbar".as_bytes());
        let mut write_buf: Vec<u8> = Vec::new();
        let options = std::collections::HashSet::from([crate::Options::ShowEnds]);
        crate::cat(
            &mut write_buf,
            vec![
                read_buf.as_slice(),
                read_buf2.as_slice(),
                read_buf3.as_slice(),
            ],
            options,
        );
        let res: String = String::from_utf8(read_buf).unwrap()
            + &String::from_utf8(read_buf2).unwrap()
            + &String::from_utf8(read_buf3).unwrap();
        let res = res.replace("\n", "$\n");
        assert_eq!(String::from_utf8(write_buf).unwrap(), res);
    }

    #[test]
    fn cat_show_tabs() {
        let read_buf: Vec<u8> = Vec::from("hello\nworld".as_bytes());
        let read_buf2: Vec<u8> = Vec::from("\tbye bye\nworld".as_bytes());
        let read_buf3: Vec<u8> = Vec::from("foo\nbar".as_bytes());
        let mut write_buf: Vec<u8> = Vec::new();
        let options = std::collections::HashSet::from([crate::Options::ShowTabs]);
        crate::cat(
            &mut write_buf,
            vec![
                read_buf.as_slice(),
                read_buf2.as_slice(),
                read_buf3.as_slice(),
            ],
            options,
        );
        let res: String = String::from_utf8(read_buf).unwrap()
            + &String::from_utf8(read_buf2).unwrap()
            + &String::from_utf8(read_buf3).unwrap();
        let res = res.replace("\t", "^I");
        assert_eq!(String::from_utf8(write_buf).unwrap(), res);
    }

    #[test]
    fn cat_show_ends_show_tabs() {
        let read_buf: Vec<u8> = Vec::from("hello\nworld".as_bytes());
        let read_buf2: Vec<u8> = Vec::from("\tbye bye\nworld".as_bytes());
        let read_buf3: Vec<u8> = Vec::from("foo\nbar".as_bytes());
        let mut write_buf: Vec<u8> = Vec::new();
        let options =
            std::collections::HashSet::from([crate::Options::ShowTabs, crate::Options::ShowEnds]);
        crate::cat(
            &mut write_buf,
            vec![
                read_buf.as_slice(),
                read_buf2.as_slice(),
                read_buf3.as_slice(),
            ],
            options,
        );
        let res: String = String::from_utf8(read_buf).unwrap()
            + &String::from_utf8(read_buf2).unwrap()
            + &String::from_utf8(read_buf3).unwrap();
        let res = res.replace("\t", "^I").replace("\n", "$\n");
        assert_eq!(String::from_utf8(write_buf).unwrap(), res);
    }

    #[test]
    fn cat_squeeze_lines() {
        let read_buf: Vec<u8> = Vec::from("hello\n\n\t\nworld".as_bytes());
        let mut write_buf: Vec<u8> = Vec::new();
        let options = std::collections::HashSet::from([crate::Options::SqueezeBlank]);
        crate::cat(&mut write_buf, vec![read_buf.as_slice()], options);
        let mut res: String = String::from_utf8(read_buf).unwrap();

        // Eating all the blank lines.
        while res.contains("\n\n") {
            res = res.replace("\n\n", "\n");
        }

        assert_eq!(String::from_utf8(write_buf).unwrap(), res);
    }

    #[test]
    fn cat_show_ends_squeeze_lines() {
        let read_buf: Vec<u8> = Vec::from("hello\n\n\t\nworld".as_bytes());
        let mut write_buf: Vec<u8> = Vec::new();
        let options = std::collections::HashSet::from([
            crate::Options::ShowEnds,
            crate::Options::SqueezeBlank,
        ]);
        crate::cat(&mut write_buf, vec![read_buf.as_slice()], options);
        let mut res: String = String::from_utf8(read_buf).unwrap();

        // Eating all the blank lines.
        while res.contains("\n\n") {
            res = res.replace("\n\n", "\n");
        }

        // Showing newlines.
        let res = res.replace("\n", "$\n");
        assert_eq!(String::from_utf8(write_buf).unwrap(), res);
    }

    #[test]
    fn cat_show_tabs_squeeze_lines() {
        let read_buf: Vec<u8> = Vec::from("hello\n\n\t\nworld".as_bytes());
        let mut write_buf: Vec<u8> = Vec::new();
        let options = std::collections::HashSet::from([
            crate::Options::ShowTabs,
            crate::Options::SqueezeBlank,
        ]);
        crate::cat(&mut write_buf, vec![read_buf.as_slice()], options);
        let mut res: String = String::from_utf8(read_buf).unwrap();

        // Eating all the blank lines.
        while res.contains("\n\n") {
            res = res.replace("\n\n", "\n");
        }

        // Show tabs
        let res = res.replace("\t", "^I");
        assert_eq!(String::from_utf8(write_buf).unwrap(), res);
    }

    #[test]
    fn cat_show_ends_show_tabs_squeeze_lines() {
        let read_buf: Vec<u8> = Vec::from("hello\n\n\t\nworld".as_bytes());
        let mut write_buf: Vec<u8> = Vec::new();
        let options = std::collections::HashSet::from([
            crate::Options::ShowTabs,
            crate::Options::ShowEnds,
            crate::Options::SqueezeBlank,
        ]);
        crate::cat(&mut write_buf, vec![read_buf.as_slice()], options);
        let mut res: String = String::from_utf8(read_buf).unwrap();

        // Eating up all the blank lines.
        while res.contains("\n\n") {
            res = res.replace("\n\n", "\n");
        }

        // Showing tabs and ends.
        let res = res.replace("\t", "^I").replace("\n", "$\n");
        assert_eq!(String::from_utf8(write_buf).unwrap(), res);
    }

    #[test]
    fn cat_number() {
        let read_buf: Vec<u8> = Vec::from("hello\n\n\t\nworld".as_bytes());
        let mut write_buf: Vec<u8> = Vec::new();
        let options = std::collections::HashSet::from([crate::Options::Number]);

        crate::cat(&mut write_buf, vec![read_buf.as_slice()], options);

        let raw_str: String = String::from_utf8(read_buf).unwrap();
        let mut res = String::new();

        // Helper to store the index
        let mut idx = 0;
        let mut index = || {
            idx += 1;
            format!("{:>6}  ", idx)
        };

        for line in raw_str.split("\n") {
            res += &(index() + line + "\n");
        }

        // If the orig string doesn't end with new line, the last loop must have added a new line. Remove that.
        if !raw_str.ends_with("\n") {
            res = String::from(&res[0..res.len() - 1])
        }

        assert_eq!(String::from_utf8(write_buf).unwrap(), res);
    }

    #[test]
    fn cat_number_nonblank() {
        let read_buf: Vec<u8> = Vec::from("hello\n\n\t\nworld".as_bytes());
        let mut write_buf: Vec<u8> = Vec::new();
        let options = std::collections::HashSet::from([crate::Options::NumberNonblank]);

        crate::cat(&mut write_buf, vec![read_buf.as_slice()], options);

        let raw_str: String = String::from_utf8(read_buf).unwrap();
        let mut res = String::new();

        // Helper to store the index
        let mut idx = 0;
        let mut index = || {
            idx += 1;
            format!("{:>6}  ", idx)
        };

        for line in raw_str.split("\n") {
            if String::from(line).eq("") {
                res += "\n";
                continue;
            }
            res += &(index() + line + "\n");
        }

        // If the orig string doesn't end with new line, the last loop must have added a new line. Remove that.
        if !raw_str.ends_with("\n") {
            res = String::from(&res[0..res.len() - 1])
        }

        assert_eq!(String::from_utf8(write_buf).unwrap(), res);
    }

    #[test]
    fn cat_number_nonblank2() {
        let read_buf: Vec<u8> = Vec::from("\n\n\nhello\n\n\t\nworld".as_bytes());
        let mut write_buf: Vec<u8> = Vec::new();
        let options = std::collections::HashSet::from([crate::Options::NumberNonblank]);

        crate::cat(&mut write_buf, vec![read_buf.as_slice()], options);

        let raw_str: String = String::from_utf8(read_buf).unwrap();
        let mut res = String::new();

        // Helper to store the index
        let mut idx = 0;
        let mut index = || {
            idx += 1;
            format!("{:>6}  ", idx)
        };

        for line in raw_str.split("\n") {
            if String::from(line).eq("") {
                res += "\n";
                continue;
            }
            res += &(index() + line + "\n");
        }

        // If the orig string doesn't end with new line, the last loop must have added a new line. Remove that.
        if !raw_str.ends_with("\n") {
            res = String::from(&res[0..res.len() - 1])
        }

        assert_eq!(String::from_utf8(write_buf).unwrap(), res);
    }

    #[test]
    fn cant_show_nonprinting() {
        
    }
}
