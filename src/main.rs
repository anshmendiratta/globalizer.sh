use std::process::Command;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let current_dir: String = match std::env::current_dir() {
        Ok(dir) => dir.to_str().unwrap().to_owned(),
        _ => String::new()
    };
    
    if args.len() != 2 {
        panic!("Wrong number of arguments. Simply reference the binary file to move.")
    }

    let binary_file_path: String = format!("{}/{}", current_dir, args[1]);
    let binary_file_name: &str = {
        let reversed_name_idx: usize = match binary_file_path.chars().rev().collect::<String>().find('/') {
            Some(idx) => idx,
            _ => 0
        };
        let name_idx = binary_file_path.len() - reversed_name_idx; 
        &binary_file_path[name_idx..]
    };
    // Add your script directory below
    let scripts_dir: String = "".to_owned();
    let copied_file_path: String = format!("{}{}", scripts_dir, binary_file_name);
    // NOTE: `cp` does not return a `stderr` value, but instead an `ExitStatus`
    match Command::new("cp").args([&binary_file_path[..], &copied_file_path[..]]).status() {
        Ok(status) => {
            if status.success() {
                println!("Copied \x1b[1;32m{}\x1b[0m over to scripts/", binary_file_name);
            }
        },
        _ => ()
    };
}
