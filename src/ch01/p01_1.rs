use std::{fs::File, io::{BufReader, BufRead, Write}};

pub fn run() {
  let mut stack = Vec::new();

  // Read all
  let work_dir = std::env::current_dir().unwrap();
  let file_path = work_dir.join("fixtures/data.txt");
  let file = File::open(file_path).unwrap();
  let mut reader = BufReader::new(file);
  let mut buf = String::new();
  loop {
    match reader.read_line(&mut buf) {
      Ok(n) => {
        if n == 0 {
          break;
        } else {
          stack.push(buf.clone());
          buf.clear();
        }
      },
      _ => panic!("Unexpected error in reader.read_line()")
    }
  }

  // Write all
  let file_path = work_dir.join("results/p01_1.txt");
  let mut file = File::create(file_path).unwrap();
  loop {
    match stack.pop() {
      Some(line) => {
        let _ = file.write_all(line.as_bytes());
      },
      _ => break
    }
  }
}
