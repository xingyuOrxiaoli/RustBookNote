
#[derive(Debug)]
enum IpAddrKind{
    V4,
    V6,
}
#[derive(Debug)]
struct IpAddr{
    kind: IpAddrKind,
    address: String,
}
fn route(ip_kind: IpAddrKind) {}
fn main() {
    println!("第六章 枚举和模式匹配");
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    println!("four = {:?}",four);
    println!("six = {:?}",six);
    route(IpAddrKind::V4);
    route(IpAddrKind::V6);
    let home = IpAddr{
        kind:IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    let loopback = IpAddr{
        kind:IpAddrKind::V6,
        address: String::from("::1"),
    };
    println!(" home = {:?} ,\n loopback = {:?}",home,loopback);
    
    println!("==================================================");
    #[derive(Debug)]
    enum IpAddr2{
        V4(u8, u8, u8, u8),
        V6(String),
    }
    let home = IpAddr2::V4(127, 0, 0, 1);
    let loopback = IpAddr2::V6(String::from("::1"));
    println!(" home = {:?} ,\n loopback = {:?}",home,loopback);

    println!("=====================Option的引入=============================");
    
    let some_number = Some(5);
    let some_char = Some('e');
    let absent_number: Option<i32> = None;
    println!("some_number = {:?} ,\n some_char = {:?}",some_number,some_char);
    println!("absent_number = {:?} ",absent_number);
}
