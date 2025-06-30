enum OperatingSystem {
    Windows,
    MacOS,
    Linux,
}
fn main() {
    let my_computer = OperatingSystem::MacOS;
    let age = years_since_release(my_computer);
        println!("My computer's operating system is {age} years old.");

        let dads_computer = OperatingSystem::Windows;
        let age = years_since_release(dads_computer);
        println!("My dad's computer is {age} years old.");
}

fn years_since_release(os: OperatingSystem) -> u32 {
    match os {
        OperatingSystem::Windows => {
            println!("Quite an old operating system!");
            39
        }
        OperatingSystem::MacOS => 23,
        OperatingSystem::Linux => 34,
    }
}