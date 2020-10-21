# pascals_nth
Display the nth row of Pascal's Triangle

## Build

```
git clone https://github.com/jeremymv2/pascals_nth.git
cd pascals_nth
cargo build
```

## Run

```
09:26 $ ./target/debug/pascals_nth
error: The following required arguments were not provided:
--row <row>

USAGE:
pascals_nth --row <row>

For more information try --help

09:42 $ ./target/debug/pascals_nth --help
pascals_nth 0.1.0

USAGE:
pascals_nth --row <row>

FLAGS:
-h, --help       Prints help information
-V, --version    Prints version information

OPTIONS:
-r, --row <row>    Row of Pascal's triangle to display

09:42 $ ./target/debug/pascals_nth --row 3

Pascal's Triangle row '3' is: [1, 3, 3, 1]

09:42 $
```
