use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "pascals_nth")]
struct Opt {
    /// Row of Pascal's triangle to display
    #[structopt(short, long)]
    row: u128,
}

fn pascals_triangle_nth_row(row: u128) -> Vec<u128> {
    let mut prev: u128 = 1;
    let mut nth: Vec<u128> = vec![];
    nth.push(prev);

    for i in 1..row + 1 {
        let curr = (prev * (row - i + 1)) / i;
        nth.push(curr);
        prev = curr;
    }
    nth
}

fn main() {
    let opt = Opt::from_args();
    match opt {
        input => println!(
            "\nPascal's Triangle row '{}' is: {:?}\n",
            input.row,
            pascals_triangle_nth_row(input.row)
        ),
    };
}
