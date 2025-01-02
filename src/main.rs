use std::net::{IpAddr, Ipv4Addr};

use bpaf::*;

#[derive(Debug, Clone)]
struct Options {
	skip_wizard: bool,
	port:        u16,
	host:        IpAddr,
}

fn options() -> impl Parser<Options> {
	let skip_wizard = long("skip-wizard")
		.short('s')
		.help("")
		.argument::<bool>("SKIP-WIZARD")
		.fallback(false);

	let port = long("port")
		.short('p')
		.help("")
		.argument::<u16>("PORT")
		.fallback(8080);

	let host = long("host")
		.short('h')
		.help("")
		.argument::<IpAddr>("HOST")
		.fallback(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)));

	construct!(Options {
		skip_wizard,
		port,
		host
	})
}

fn main() {
	let opts = options().run();
	dbg!(opts);
}
