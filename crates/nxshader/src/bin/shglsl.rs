// Mush8 - shglsl.rs
fn main() {
    let mut args: Vec<String> = std::env::args().skip(1).collect();
    let compute = args.first().is_some_and(|a| a == "--compute");
    if compute {
        args.remove(0);
    }
    let mut lmem = 0u32;
    let mut dims = [1u32; 3];
    if compute && args.len() >= 4 {
        if let [Ok(l), Ok(x), Ok(y), Ok(z)] = [
            args[0].parse::<u32>(),
            args[1].parse::<u32>(),
            args[2].parse::<u32>(),
            args[3].parse::<u32>(),
        ] {
            lmem = l;
            dims = [x, y, z];
            args.drain(..4);
        }
    }
    let blob = std::fs::read(&args[0]).expect("read");
    let r = if compute { nxshader::glsl::translate_compute(&blob, lmem, dims) } else { nxshader::glsl::translate(&blob) };
    match r {
        Ok(t) => {
            if let Some(o) = args.get(1) {
                std::fs::write(o, &t.source).unwrap();
            } else {
                print!("{}", t.source);
            }
            eprintln!("{:?} {} interface {:?}", t.stage, t.entry, t.interface);
        }
        Err(e) => {
            eprintln!("error: {e:#}");
            std::process::exit(1);
        }
    }
}
