use std::process::{Command, exit};

pub fn build(out: &str) {
    let obj = Command::new("fasm")
        .arg(format!("{out}.asm"))
        .output()
        .expect("error while compiling");

    if obj.status.success() {
        let linker = Command::new("gcc")
            .arg("-no-pie")
            .arg("-o")
            .arg(out)
            .arg(format!("{out}.o"))
            .output()
            .expect("error while linking");
        if !linker.status.success() {
            eprint!("{}", String::from_utf8_lossy(&linker.stderr));
            exit(1);
        }
    } else {
        eprint!("{}", String::from_utf8_lossy(&obj.stderr));
        exit(1);
    }
}
