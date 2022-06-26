### cat

This is an implementation of the commonly used unix utility `cat`. It is not
perfect in any way, neither is it supposed to be fast. This is just me trying
my hands on Rust to get familiar with the language.

# Help

```
USAGE:
    cat [OPTIONS] [FILES]...

ARGS:
    <FILES>...    [default: -]

OPTIONS:
    -b, --number-nonblank     number nonempty output lines, overrides -n
    -E, --show-ends           display $ at end of each line
    -h, --help                Print help information
    -n, --number              number all output lines
    -s, --squeeze-blank       suppress repeated empty output lines
    -T, --show-tabs           display TAB characters as ^I
    -v, --show-nonprinting    use ^ and M- notation, except for LFD and TAB
```