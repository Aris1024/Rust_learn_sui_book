use std::{collections::HashMap, num::ParseIntError};

fn main() {
    println!("课程地址:https://rust.sui-book.com/03_std/01_option.html");
    main1();
    main2();
    main3();
    main4();
    main5();
    main6();
    main7();
    main8();
    main9();
    main10();
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
fn main6() {
    println!("--------- main6");
    fn find(haystack: &str, needle: char) -> Option<usize> {
        for (offset, c) in haystack.char_indices() {
            if c == needle {
                return Some(offset);
            }
        }
        None
    }
    let position = find("hello", 'l');
    if let Some(pos) = position {
        println!("找到位置: {}", pos); //找到位置: 2
    }
}
fn main7() {
    println!("--------- main7");
    let mut socres = HashMap::new();
    socres.insert("Alice", 50);

    let alice_score = socres.get("Alice");
    println!("alice_score: {:?}", alice_score); // alice_score: Some(50)

    /*
    HashMap::get() 方法返回 Option<&V>，也就是一个包含值引用的 Option
    copied() 方法将 Option<&T> 转换为 Option<T>，前提是 T 实现了 Copy trait
    Option<&i32> -> Option<i32>
     */
    let bob_score = socres.get("Bob").copied().unwrap_or(0);
    println!("bob_score: {:?}", bob_score); // bob_score: 0
}
fn main8() {
    println!("--------- main8");
    println!("\n=== transpose: 当你有一个 Option<Result<T, E>> 但需要 Result<Option<T>, E> 时 ===");
    fn parse(s: &str) -> Option<Result<i32, ParseIntError>> {
        if s.is_empty() { None } else { Some(s.parse()) }
    }
    // transpose()：Option<Result<T, E>> 转为 Result<Option, E>。
    let result = parse("42").transpose();
    println!(" {:?}", result); // Ok(Some(42))

    let empty = parse("").transpose();
    println!(" {:?}", empty); //  Ok(None)

    println!("\n=== flatten: 展平嵌套的 Option，将 Option<Option<T>> 变为 Option<T> ===");
    fn find_user_id(username: &str) -> Option<u32> {
        match username {
            "alice" => Some(1),
            "bob" => Some(2),
            _ => None,
        }
    }

    fn find_user_email(user_id: u32) -> Option<String> {
        match user_id {
            1 => Some("alice@example.com".to_string()),
            2 => Some("bob@example.com".to_string()),
            _ => None,
        }
    }
    let usernames = vec!["alice", "charlie", "bob"];
    for username in usernames {
        // 不使用 flatten - 返回 Option<Option<String>>
        let nested_email = find_user_id(username).map(|id| find_user_email(id));
        println!("nested_email: {:?}", nested_email); // nested_email: Some(Some("bob@example.com"))

        let email = find_user_id(username)
            .map(|id| find_user_email(id))
            .flatten();
        println!("email: {:?}", email); // email: Some("bob@example.com")
    }

    println!("\n=== zip: zip() - 结合两个 Option 为 Option<(T, U)> ===");
    println!("=== zip: zip() - 只有当两个 Option 都是 Some 时，结果才是 Some ===");
    println!("=== zip: zip() - 典型场景：表单验证、需要多个值都存在才能进行下一步操作 ===");

    let opt1 = Some(1);
    let opt2 = Some("hello");
    let opt3: Option<i32> = None;
    println!("Some(1).zip(Some(\"hello\")) = {:?}", opt1.zip(opt2)); // Some(1).zip(Some("hello")) = Some((1, "hello"))
    println!("Some(\"hello\").zip(Some(1)) = {:?}", opt2.zip(opt1)); // Some("hello").zip(Some(1)) = Some(("hello", 1))
    println!("Some(1).zip(None) = {:?}", opt1.zip(opt3)); // Some(1).zip(None) = None

    println!("=== 表单验证 ===");
    fn validate_emial(emial: &str) -> Option<String> {
        if emial.contains('@') {
            Some(emial.to_string())
        } else {
            None
        }
    }
    fn validate_password(password: &str) -> Option<String> {
        if password.len() >= 6 {
            Some(password.to_string())
        } else {
            None
        }
    }
    let test_case = vec![
        ("user@example.com", "password123"),
        ("invalid-email", "password123"),
        ("user@example.com", "12345"),
        ("invalid", "short"),
    ];
    for (email, password) in test_case {
        let validated = validate_emial(email).zip(validate_password(password));
        match validated {
            Some((valid_email, valid_password)) => {
                println!("✅ 验证成功: {} / {}", valid_email, valid_password); // ✅ 验证成功: user@example.com / password123
            }
            None => {
                println!("❌ 验证失败: {} / {}", email, password); // ❌ 验证失败: invalid-email / password123
            }
        }
    }
}

fn main9() {
    println!("--------- main9");
    println!("=== 编写函数，返回字符串中第一个元音的位置（Option）。===");
    fn get_index(s: &str) -> Option<usize> {
        let a = vec!['a', 'e', 'i', 'o', 'u'];
        for (offset, value) in s.char_indices() {
            let value = value.to_ascii_lowercase();
            if a.contains(&value) {
                return Some(offset);
            }
        }
        None
    }
    let test_cases = vec![
        String::from("hello"),
        String::from("name"),
        String::from("bE"),
    ];
    for string in test_cases {
        let idx = get_index(&string);
        println!("idx = {:?}", idx);
    }
}

fn main10() {
    println!("\n=== 用 map 和 unwrap_or 处理 Option<Vec>，计算平均值或默认 0.0 ===");

    fn calculate_average<T>(numbers: Option<Vec<T>>) -> f64
    where
        T: Copy + Into<f64>,
    {
        numbers
            .map(|arr| {
                if arr.is_empty() {
                    0.0
                } else {
                    let sum: f64 = arr.iter().map(|&x| x.into()).sum();
                    sum / arr.len() as f64
                }
            })
            .unwrap_or(0.0)
    }
    let nums = Some(vec![1, 2, 3, 4, 5, 6, 7, 8]);
    let result = calculate_average(nums);
    println!("result = {result}");
}
