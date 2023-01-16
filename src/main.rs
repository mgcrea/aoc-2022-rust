/*
 * This file contains template code.
 * There is no need to edit this file unless you want to change template functionality.
 */
use advent_of_code::{ANSI_BOLD, ANSI_ITALIC, ANSI_RESET};
use debug_rs::debug;
use std::path::{Path, PathBuf};
use std::{fs, io, process::Command};

// fn read_bin_files() -> io::Result<Vec<PathBuf>> {
//   fs::read_dir("./src/bin")?
//     .filter_map(|entry| entry.ok())
//     .map(|entry| entry.path())
//     .filter(|path| path.is_file() && path.extension().unwrap_or_default() == "rs")
//     .collect::<io::Result<Vec<PathBuf>>>()
// }

fn main() -> io::Result<()> {
  let days = fs::read_dir("./src/bin")?
    .filter_map(|entry| entry.ok())
    .map(|entry| entry.path())
    .filter(|path| path.is_file() && path.extension().unwrap_or_default() == "rs")
    .map(|path| String::from(path.file_stem().unwrap().to_str().unwrap()))
    .collect::<Vec<String>>();

  let total: f64 = days
    .iter()
    .map(|day| {
      // let day = format!("{:02}", day);

      let mut args = vec!["run", "--bin", &day];
      if cfg!(not(debug_assertions)) {
        args.push("--release");
      }

      let cmd = Command::new("cargo").args(&args).output().unwrap();

      println!("----------");
      println!("{}| Day {} |{}", ANSI_BOLD, day, ANSI_RESET);
      println!("----------");

      let output = String::from_utf8(cmd.stdout).unwrap();
      let is_empty = output.is_empty();

      println!(
        "{}",
        if is_empty {
          "Not solved."
        } else {
          output.trim()
        }
      );

      if is_empty {
        0_f64
      } else {
        advent_of_code::parse_exec_time(&output)
      }
    })
    .sum();

  println!(
    "{}Total:{} {}{:.2}ms{}",
    ANSI_BOLD, ANSI_RESET, ANSI_ITALIC, total, ANSI_RESET
  );
  Ok(())
}
