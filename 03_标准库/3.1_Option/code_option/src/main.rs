fn main() {
    println!("课程地址:https://rust.sui-book.com/03_std/01_option.html");
    main1();
    main2();
    main3();
    main4();
    main5();
}
fn main1() {
    println!("--------- main1");
    let some_number = Some(5); // 编译器自动推断类型: Option<i32>
    let none_number: Option<i32> = None; // 编译器无法推断类型,必须显示标注!
    println!("{:?}", some_number); // Some(5)
    println!("{:?}", none_number); // None
}
fn divide(dividend: f64, divisor: f64) -> Option<f64> {
    if divisor == 0.0 {
        None
    } else {
        Some(dividend / divisor)
    }
}

fn main2() {
    println!("--------- main2");
    match divide(10.0, 2.0) {
        Some(result) => println!("结果: {}", result), // 结果: 5
        None => println!("不能除以 0"),
    }
    match divide(10.0, 0.0) {
        Some(_) => {}                   // 忽略值,用 _
        None => println!("不能除以 0"), // 不能除以 0
    }
}

fn main3() {
    println!("--------- main3");

    // if let 只处理 Some，else 处理 None。更简洁于 match。
    let config = Some(42);
    if let Some(value) = config {
        println!("配置值: {}", value); // 配置值: 42
    } else {
        println!("无配置");
    }
}
/*
Option 有许多实用方法，避免手动 match。
    - unwrap()：返回 Some 值，否则 panic!（不推荐生产）。
    - expect("msg")：类似 unwrap，但自定义 panic 消息。
    - unwrap_or(default)：返回 Some 值或默认值。
    - unwrap_or_else(closure)：懒惰计算默认值。
    - map(f)：转换 Some 值，None 保持。
    - and_then(f)：链式操作，返回 Option。
    - ok_or(err)：转为 Result。
    - is_some() / is_none()：检查变体。
    - as_ref() / as_mut()：借用内部值。
*/
fn main4() {
    println!("--------- main4");
    // unwrap() 和 expect() - 直接取值，失败时 panic
    let some_value = Some(42);
    let _none_value: Option<i32> = None;
    println!("\n=== unwrap() ===");
    println!("Some(42).unwrap = {}", some_value.unwrap()); // Some(42).unwrap = 42
    // _none_value.unwrap(); // panic: called `Option::unwrap()` on a `None` value

    println!("\n=== expect() ===");
    println!("Some(42).expect = {}", some_value.expect("应该有值")); // Some(42).expect = 42
    // _none_value.expect("自定义错误消息:失败时panic"); // 自定义错误消息:失败时panic

    // unwrap_or() 和 unwrap_or_else() - 安全取值，提供默认值
    println!("\n=== unwrap_or() ===");
    println!("some_value.unwrap_or(0) = {}", some_value.unwrap_or(0)); // some_value.unwrap_or(0) = 42
    println!("_none_value.unwrap_or(0) = {}", _none_value.unwrap_or(0)); // _none_value.unwrap_or(0) = 0

    println!("\n=== unwrap_or_else() ===");
    println!(
        "some_value.unwrap_or_else(|| 999) = {}",
        some_value.unwrap_or_else(|| 999)
    ); // some_value.unwrap_or_else(|| 999) = 42
    println!(
        "_none_value.unwrap_or_else(|| 999) = {}",
        _none_value.unwrap_or_else(|| 999)
    ); // _none_value.unwrap_or_else(|| 000) = 999

    // map() - 转换 Some 值，保持 None 不变
    println!("\n=== map() ===");
    let doubled = some_value.map(|x| x * 2);
    let none_doubled = _none_value.map(|x| x * 2);
    println!("some_value.map(|x| x *2) = {:?}", doubled); // some_value.map(|x| x *2) = Some(84)
    println!("_none_value.map(|x| x * 2) = {:?}", none_doubled); // _none_value.map(|x| x * 2) = None

    // and_then() - 链式操作，用于可能失败的连续操作
    println!("\n=== and_then() ===");
    let divede_by_2 = |x| if x % 2 == 0 { Some(x / 2) } else { None };
    // some_value.and_then(divede_by_2)
    println!(
        "some_value.and_then(divede_by_2) = {:?}",
        some_value.and_then(divede_by_2)
    ); // some_value.and_then(divede_by_2) = Some(21)
    // Some(43).and_then(divede_by_2)
    println!(
        "Some(43).and_then(divede_by_2) = {:?}",
        Some(43).and_then(divede_by_2)
    ); // Some(43).and_then(divede_by_2) = None

    // ok_or() - 将 Option 转换为 Result
    println!("\n=== ok_or() ===");
    let result1 = some_value.ok_or("没有值");
    let result2 = _none_value.ok_or("没有值");
    println!("Some(42).ok_or(\"没有值\") = {:?}", result1); // Some(42).ok_or("没有值") = Ok(42)
    println!("None.ok_or(\"没有值\") = {:?}", result2); // None.ok_or("没有值") = Err("没有值")

    // is_some() / is_none() - 检查变体类型
    println!("\n=== is_some() / is_none() ===");
    println!("some_value.is_some() = {:?}", some_value.is_some()); // some_value.is_some() = true
    println!("some_value.is_none() = {:?}", some_value.is_none()); // some_value.is_none() = false
    println!("_none_value.is_some() = {:?}", _none_value.is_some()); // _none_value.is_some() = false
    println!("_none_value.is_none() = {:?}", _none_value.is_none()); // _none_value.is_none() = true

    // as_ref() / as_mut() - 借用内部值而不消费 Option
    println!("\n=== as_ref() / as_mut() ===");
    let mut opt_string = Some(String::from("Hello"));
    // as_ref() 借用内部值,不消费 Option
    // s 类型: & String
    if let Some(s) = opt_string.as_ref() {
        println!("as_ref(): 字符串长度 = {}", s.len()); // as_ref(): 字符串长度 = 5
    }
    println!("opt_string 仍然可用: {:?}", opt_string); // opt_string 仍然可用: Some("Hello")

    // s 类型: &mut String
    if let Some(s) = opt_string.as_mut() {
        s.push_str(", World!");
        println!("as_mut(): 修改后 = {}", s); // as_mut(): 修改后 = Hello, World!
    }
    println!("最终值: {:?}", opt_string); // 最终值: Some("Hello, World!")
}

fn main5() {
    println!("--------- main5");
    fn parse_number(s: &str) -> Option<i32> {
        s.parse().ok()
    }
    let input = "123";
    let number = parse_number(input).map(|n| n * 2).unwrap_or_else(|| {
        println!("解析失败,使用默认值");
        0
    });
    println!("解析 \"{}\" 并乘以2: {}", input, number); // 解析 "123" 并乘以2: 246

    // 2. 链式处理
    fn safe_devide(x: i32, y: i32) -> Option<i32> {
        if y != 0 { Some(x / y) } else { None }
    }
    let result = Some(100)
        .and_then(|x| safe_devide(x, 2)) // 100 / 2 =50
        .and_then(|x| safe_devide(x, 5)) // 50 / 5 = 10
        .map(|x| x + 1); // 10 + 1 =11
    println!("链式计算结果: {:?}", result); // 链式计算结果: Some(11)

    // 3. 配置处理
    struct Config {
        timeout: Option<u64>,
        host: Option<String>,
    }
    let config = Config {
        timeout: Some(30),
        host: None,
    };
    let timeout = config.timeout.unwrap_or(60);
    let host = config.host.unwrap_or_else(|| "localhost".to_string());
    println!("超时设置: {}秒", timeout); // 超时设置: 30秒
    println!("主机地址: {}", host); // 主机地址: localhost
}
