use std::process::Command;




fn main() {
    let my_str = include_str!("help.txt");
/*
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
                .args(["/C", "echo hello"])
                .output()
                .expect("failed to execute process")
    } else {
        Command::new("sh")
                .arg("-c")
                .arg("cat help.txt:w")
                .output()
                .expect("failed to execute process")
    };
    
    let hello = output.stdout;
    
*/

   println!("{}",my_str);
}
