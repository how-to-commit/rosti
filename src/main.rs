fn main() {
    let bios_path = env!("BIOS_PATH");
    /* let cmd = std::process::Command::new("qemu-system-x86-64")
        .arg("-drive")
        .arg(format!("format=raw,file={bios_path}"));
    let mut child = cmd.spawn().unwrap();
    child.wait().unwrap(); */
    println!("{}", bios_path);
}
