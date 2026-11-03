// Mush8 - shmsl.rs
// Constantin Clerc (c22dev) - 0.1
fn main() {
    let mut args: Vec<String> = std::env::args().skip(1).collect();
    let compute = args.first().is_some_and(|a| a == "--compute");
    if compute {
        args.remove(0);
    }
    let blob = std::fs::read(&args[0]).expect("read");
    let r = if compute { nxshader::translate_compute(&blob, 0, [1, 1, 1]) } else { nxshader::translate(&blob) };
    match r {
        Ok(t) => {
            if let Some(o) = args.get(1) {
                std::fs::write(o, &t.msl).unwrap();
            } else {
                print!("{}", t.msl);
            }
            eprintln!("{:?} {} interface {:?}", t.stage, t.entry, t.interface);
        }
        Err(e) => {
            eprintln!("error: {e:#}");
            std::process::exit(1);
        }
    }
}
