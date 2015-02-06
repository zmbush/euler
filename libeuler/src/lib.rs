extern crate getopts;
pub use getopts::Options;

#[macro_export]
macro_rules! solutions {
    (inputs: ($($n:ident : $ty:ty = $def:expr),*) $(sol $name:ident $content:block)+) => (
        {
            let args: Vec<String> = ::std::os::args();
            let mut opts = $crate::Options::new();

            $(
                opts.optopt("", stringify!($n), "", "VALUE");
            )*
            opts.optflag("h", "help", "Display this help text");

            let help = || {
                let brief = format!("Usage: {} [options] [versions] \nVersions: {:?}", &args[0], [$(stringify!($name)),+]);
                print!("{}", opts.usage(brief.as_slice()));
            };

            let matches = match opts.parse(args.tail()) {
                Ok(m) => m,
                Err(_) => { help(); return; }
            };

            if matches.opt_present("h") {
                help();
                return;
            }

            $(
                let $n: $ty = match matches.opt_str(stringify!($n)) {
                    Some(s) => { s.parse().unwrap_or($def) },
                    None => $def
                };
            )*

            $(
                let $name = || {
                    $content
                };
            )+
            $(
                if matches.free.len() <= 0 || matches.free.contains(&stringify!($name).to_string()) {
                    println!("Running: {}", stringify!($name));
                    println!("Result: {:?}", $name());
                    println!("");
                }
            )+
        }
    )
}
