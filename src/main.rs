use bpaf::*;

#[derive(Debug, Clone)]
struct Options {
    speed: i32,
}

fn options() -> impl Parser<Options> {
    let speed = long("speed")
        .help("Set the speed value")
        .argument::<i32>("SPEED")
        .fallback(0);

    construct!(Options { speed })
}

fn main() {
    let opts = options().run();
    dbg!(opts.speed);
}
