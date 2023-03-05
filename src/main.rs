use std::fs;

fn main() {
    // This project aims to « cartography » IPv4 adresses assigned to France. Nothing more.
    // Why to do this ? Out of curiosity.
    const MY_FILE_IP2LOCATION: &str = "./data/ip2location_ipv4.csv";
    let contents = fs::read_to_string(MY_FILE_IP2LOCATION).expect("Can read the file ?");
    println!("{}", contents);
    // For each line of the file,
    // Get the range if it is assigned to France
    // For each range, extract all IP addresses
    // For each adresses, make the tests needed.
    //
    // WHAT TESTS ?
    // -> Is reachable ?
    // -> get hostname
    // -> other to add here
}
