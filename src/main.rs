use weasel_string::*;

fn main() {
    let conf = Config::new(
        "ABCDEFGHIJKLMNOPQRSTUVWXYZ ",
        "METHINKS IT IS LIKE A WEASEL",
        0.05,
        100,
    );

    ea_simple(conf);
}
