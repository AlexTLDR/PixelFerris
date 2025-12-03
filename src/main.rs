fn main() {
    let mut args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() {
        print_usage_and_exit();
    }
    let subcommand = args.remove(0);
    match subcommand.as_str() {
        "blur" => {
            if args.len() != 2 {
                print_usage_and_exit();
            }
            let infile = args.remove(0);
            let outfile = args.remove(0);

            blur(infile, outfile);
        }
        _ => {
            print_usage_and_exit();
        }
    }
}

fn print_usage_and_exit() {
    println!("USAGE (when in doubt, use a .png extension on your filenames)");
    println!("blur INFILE OUTFILE");
    println!("fractal OUTFILE");

    std::process::exit(-1);
}

fn blur(infile: String, outfile: String) {
    let img = image::open(infile).expect("Failed to open INFILE");
    let img2 = img.blur(3.0);
    img2.save(outfile).expect("Failed to save OUTFILE");
}