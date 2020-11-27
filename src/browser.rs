use std::process::Command;


pub fn run(port: u16) {
    let result = Command::new("/Applications/Google Chrome.app/Contents/MacOS/Google Chrome")
        .arg(&format!(
            "--app=http://localhost:{}/index.html",
            port
        ))
        .arg("--disable-background-mode")
        .arg("--disable-extensions")
        .output()
        .expect("failed to execute process");
    std::process::exit(1);
}