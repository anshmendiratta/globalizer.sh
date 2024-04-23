use std::process::Command;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let current_dir: String = match std::env::current_dir() {
        Ok(dir) => dir.to_str().unwrap().to_owned(),
        _ => String::new(),
    };

    if args.len() != 2 {
        println!("Wrong number of arguments. Simply reference the binary file to move.")
    }

    let binary_file_path: String = format!("{}/{}", current_dir, args[1]);
    let binary_file_name: &str = {
        let reversed_name_idx: usize =
            match binary_file_path.chars().rev().collect::<String>().find('/') {
                Some(idx) => idx,
                _ => 0,
            };
        let name_idx = binary_file_path.len() - reversed_name_idx;
        &binary_file_path[name_idx..]
    };
    let scripts_dir: String = "/Users/anshmendiratta/scripts/".to_owned();
    let copied_file_path: String = format!("{}{}", scripts_dir, binary_file_name);

    if let Ok(status) = Command::new("cp")
        .args([&binary_file_path[..], &copied_file_path[..]])
        .status()
    {
        if status.success() {
            println!(
                "Copied \x1b[1;32m{}\x1b[0m over to scripts/",
                binary_file_name
            );
        }
    };
}
