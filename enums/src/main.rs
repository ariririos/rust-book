#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String),
}

fn print_ip_addr(ip: &IpAddr) -> () {
    match ip {
        IpAddr::V4(address) => println!("V4 address: {address}"),
        IpAddr::V6(address) => println!("V6 address: {address}"),
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn describe_only_ipv4(ip: &IpAddr) -> Option<String> {
    let IpAddr::V4(address) = ip else {
        return None;
    };
    Some(format!("IPV4 address is {}", address))
}

fn main() {
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));
    print_ip_addr(&home);
    print_ip_addr(&loopback);
    let home_str = describe_only_ipv4(&home);
    println!("Home if IPV4: {}", home_str.unwrap_or_default());
    let loopback_str = describe_only_ipv4(&loopback);
    println!("Loopback if IPV4: {}", loopback_str.unwrap_or_default());

    let five = Some(5);
    let six = plus_one(five);
    let nothing: Option<i32> = None;
    let still_nothing = plus_one(nothing);
    println!("Five plus one: {}", six.unwrap());
    println!("None plus one: {}", still_nothing.unwrap_or_default());

    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("Max is {max}");
    };
    let config_none: Option<u8> = None;
    if let Some(_) = config_none {
        println!("Unreachable");
    } else {
        println!("None");
    }
}
