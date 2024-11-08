#[cfg(windows)]
fn build_cmds(bios_path: &'static str) -> std::process::Command {
    println!("[*] attempting to run with MSYS2 MINGW64 qemu-system-x86_64");
    println!("[!] if not installed, binary can be run with qemu yourself");

    let mut cmds = std::process::Command::new("C:\\msys64\\usr\\bin\\env");
    cmds.arg("MSYSTEM=MINGW64")
        .arg("qemu-system-x86_64")
        .arg("-drive")
        .arg(format!("format=raw,file={bios_path}"));

    cmds
}

#[cfg(unix)]
fn build_cmds(bios_path: &'static str) -> std::process::Command {
    println!("[*] attempting to run with qemu-system-x86_64");

    let mut cmds = std::process::Command::new("qemu-system-x86_64");
    cmds.arg("-drive")
        .arg(format!("format=raw,file={bios_path}"));

    cmds
}

fn main() {
    let bios_path = env!("BIOS_PATH");
    println!("[*] generated bin at: {}", bios_path);
    let mut cmds = build_cmds(bios_path);
    let mut child = cmds.spawn().unwrap();
    child.wait().unwrap();
}
