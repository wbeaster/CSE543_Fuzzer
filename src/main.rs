use std::{env, process};
use std::fs::File;
use std::io::{BufReader, Read, Write};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha20Rng;
use std::process::{Command, ExitStatus, Stdio};

fn main() {

    // Command::new("ls")
    //     .arg("-l")
    //     .arg("-a")
    //     .spawn()
    //     .expect("ls command failed to start");

    /*
    Command::new("./prog_0")
        .spawn()
        .expect("prog_0 command failed to start");
    */

    let path = "_seed_";
    let iteraton_gate: i32 = 500;
    let bytes_increase: i32 = 10;
    let change_probability:i32 = 13;

    let args: Vec<_> = env::args().collect();
    if args.len() > 1 {
        eprintln!("The zeroth argument is {}", args[0]);
        eprintln!("The first argument is {}", args[1]);
        eprintln!("The second argument is {}", args[2]);
    }

    // put the fist arg in the seed
    let seed: i32 = args[1].parse().unwrap();
    eprintln!("seed is: {}", seed);
    let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(seed as u64);

    // the second arg is the number of iterations
    let iterations: i32 = args[2].parse().unwrap();
    eprintln!("iterations are: {}", iterations);

    // read the seed file
    /*
    let mut file = File::open(&path).expect("Error opening File");
    let mut contents = String::new();

    file.read_to_string(&mut contents).expect("Unable to read to string");
    println!("Contents are: {}", contents);
    */

    // Read the seed file as bytes in to a vector
    let f = File::open(path).expect("Error opening File");
    let mut reader = BufReader::new(f);
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer).expect("Error reading File");

    // Diagnostic
    /*
    for value in &buffer {
        println!("BYTE: {}", value);
    }
    */
    eprintln!("Buffer length is {}", buffer.len());
    eprintln!("buffer");
    // Make sure we write the seed value of the buffer to stdout
    std::io::stdout().write_all(&buffer).unwrap();

    // TODO: Make a second thread that works ahead, determining if a byte should be changed or not

    // Every iteration_gate iterations add bytes_increase random bytes to the fuzz
    for n in 1..=iterations {
        eprintln!("Iteration {}", n);
        if n % iteraton_gate == 0 {
            eprintln!("iteration {}", iteraton_gate);
            // TODO: There is certainly a more efficient way of doing this!
            for _i in 0..bytes_increase {
                buffer.push(1);
            }
        }
        // Determine for each byte if it will change
        // If it should change, then change it
        for o in 0..buffer.len() {
            let num = rng.gen_range(1..=100);
            if num <= change_probability {
                buffer[o] = rand::thread_rng().gen_range(0..=255);
            }
        }
        // Everything is done for this iteration, write the bytes to stdout
        std::io::stdout().write_all(&buffer).unwrap();
        // std::io::stdout().flush().expect("some error message");
        std::io::stdout().flush().unwrap();
/*
        let simpleIO = Command::new("./simpleIO")
            .spawn()
            .expect("error starting simpleIO");



        eprintln!("About to start simpleIO");
        let output = Command::new("./simpleIO")
            .status()
            .expect("prog_0 command failed to start");
        std::io::stdout().write_all(&buffer).unwrap();
        eprintln!("status: {}", output);

        if output.success() {
            eprintln!("No crash");
        } else {
            eprintln!("CRASH");
            std::process::exit(0);
        }
        eprintln!();
*/

    }

    /*
    println!("\n");
    for value in &buffer {
        println!("BYTE: {}", value);
    }
    */
    eprintln!("Buffer length is {}", buffer.len());
    eprintln!();

    process::exit(0);
}