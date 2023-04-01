// You may be familiar with native enum types if you've programmed in Java, C, or C#. Though Rust enums are in fact
// *much* more powerful than their counterparts. Rust enums are more formally an kind of algebraic data type (ADT), and
// more specifially they are a *sum type* that also goes by the names *tagged union*, *variant*, or *discriminated
// union*. The key feature of sum type ADTs is that they allow us to express many different kinds of data in a single
// type while retaining type safety and readability.

fn main() {
    // ******************** Data-less Enums ********************

    // Like in Java, C, and C#, we can express data-less enums.
    #[derive(PartialEq)]
    enum LightColor {
        Red,
        Yellow,
        Green,
    }

    // We can then use these enums by pattern matching, primarily in a match statement.
    // Here we construct a simple state machine
    let mut light = LightColor::Red;
    while light != LightColor::Green {
        match light {
            LightColor::Red => {
                println!("Red light!");
                light = LightColor::Yellow;
            },
            LightColor::Yellow => {
                println!("Yellow light!");
                light = LightColor::Green;
            },
            LightColor::Green => {
                // This branch is unreachable
                println!("Green light!");
            },
        }
    }
    println!("Go!");

    // ******************** Data-ful Enums ********************

    // Enums can also contain data.
    enum IpAddr {
        V4(String),
        V6(String),
    }

    let ip_addrs = [
        IpAddr::V4("127.0.0.1".to_string()),
        IpAddr::V6("2001:0db8:85a3:0000:0000:8a2e:0370:7334".to_string())
    ];
    for ip_addr in &ip_addrs {
        match ip_addr {
            IpAddr::V4(ip) => println!("IP v4: {}", ip),
            IpAddr::V6(ip) => println!("IP v6: {}", ip),
        }
    }

    // But the types don't have to be the same type. We can flexibly mix and match types.
    // amongst enum variants. In this way it may now be clear why enums are called tagged unions.

    // enum IpAddr {
    //     V4(u8, u8, u8, u8),
    //     V6(String),
    // }

    // enum IpAddr {
    //     V4(u8, u8, u8, u8),
    //     V6(u8, u8, u8, u8, u8, u8, u8, u8),
    // }
    
    // struct IpV4Addr {
    //     octets: [u8; 4],
    // }

    // struct IpV6Addr {
    //     octets: [u8; 8],
    // }

    // enum IpAddr {
    //     V4(IpV4Addr),
    //     V6(IpV6Addr),
    // }

}