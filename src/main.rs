use std::fs;
use std::net::Ipv4Addr;
use std::str::Lines;

fn main() {
    // This project aims to « cartography » IPv4 adresses assigned to France. Nothing more.
    // Why to do this ? Out of curiosity.
    const MY_FILE_IP2LOCATION: &str = "./data/ip2location_ipv4.csv";
    let contents = fs::read_to_string(MY_FILE_IP2LOCATION).expect("Can read the file ?");
    let ranges: Lines = contents.lines();
    let mut ipv4_addresses: Vec<Ipv4Addr> = Vec::new();
    for line in ranges {
        let data_line: Vec<&str> = line.split(',').collect();
        if data_line.len()==4 && data_line[3]=="\"France\""{
            // First strip the double quotes then cast to unsigned integer.
            let range_start: u32 = data_line[0].get(1..data_line[0].len()-1).unwrap().parse::<u32>().unwrap();
            let range_end: u32 = data_line[1].get(1..data_line[1].len()-1).unwrap().parse::<u32>().unwrap();
            for ip_long in range_start..range_end {
                let addr: Ipv4Addr = Ipv4Addr::from(ip_long);
                ipv4_addresses.push(addr);
            }
        }
    }
    println!("{:?}", ipv4_addresses);

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
