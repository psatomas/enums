enum OperatingSystem {
    Windows,
    MacOS,
    Linux,
}
fn main() {
    let my_computer = OperatingSystem::MacOS;
    let age = years_since_release(my_computer);
        println!("My computer's operating system is {age} years old.");
}

fn years_since_release(os: OperatingSystem) -> u32 {
    match os {
        OperatingSystem::Windows => 39,
        OperatingSystem::MacOS => 23,
        OperatingSystem::Linux => 34,
    }
}