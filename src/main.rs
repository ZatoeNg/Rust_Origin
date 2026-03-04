mod SimpleGrep;

use std::fs::File;
use std::io::{ErrorKind, Read};

fn base_operation() {
    let money: i32 = 5_000;
    let tuple: (u8, f32, bool) = (10, 3.5, false);
    let array: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
}

fn my_function(values: i32) -> i32 {
    return values;
}

fn control_function(password: &str) {
    let length = password.len();
    if length >= 10 {
        println!("{password} is long enough!");
    } else if length == 0 {
        println!("{password} is shorter enough!");
    } else {
        println!("{password} is Not long enough!");
    }
}

fn loop_function() {
    let mut count: i32 = 0;
    'main: loop {
        count += 1;

        if count == 10 {
            println!("Loop Break!");
            break;
        } else if count == 6 {
            println!("Loop continue!");
            continue;
        } else if count == 3 {
            println!("continue 'main loop!");
            continue 'main;
        }
        println!("Loop forever!={:?}", count);
    }
}

fn while_function() {
    let mut count: i32 = 0;

    while count < 10 {
        count += 1;
        println!("While forever!={:?}", count);
    }

    println!("While End!");
}

fn for_function() {
    for x in 0..10 {
        println!("For：{}", x);
    }
}

fn refer_function(text: &String) -> &String {
    text
}

fn struct_function() {
    struct Fruit {
        name: String,
        grams: i32,
        price: f32,
    }

    let fruit = Fruit {
        name: String::from("Apple"),
        grams: 0,
        price: 3.0,
    };

    let update_fruit = Fruit {
        name: String::from("Banana"),
        ..fruit
    };
}

fn method() {
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }
    impl Rectangle {
        // 简写形式:&self,这个类似于普通方法
        fn get_area(self: &Self) -> u32 {
            self.width * self.height
        }
    }
}
fn none_function() {
    let some_number: Option<i32> = Some(42);
    let no_number: Option<i32> = None;
}

enum DataSize {
    Byte,
    KB,
    MB,
    GB(u64),
}

fn match_function(size: DataSize) -> u64 {
    match size {
        DataSize::Byte => 1,
        DataSize::KB => 1000,
        DataSize::MB => 1000 * 1000,
        DataSize::GB(var) => 1000 * 1000 * (if var == 0 { 1000 } else { var }),
    }
}

fn if_let_function() {
    let size: DataSize = DataSize::GB(1);

    if let DataSize::GB(var) = size {
        println!("var = {}", var);
    }

    match size {
        DataSize::GB(var) => println!("var = {}", var),
        _ => println!("other"),
    }
}

fn error_function() {
    let path = "bob.png";
    let result_txt = File::open(path);

    let txt = match result_txt {
        Ok(file) => {
            println!("file = {:?}", file);
            file
        }
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(path) {
                Ok(file) => {
                    println!("File created successfully: {}", path);
                    file
                }
                Err(error) => panic!("Problem creating the file: {:?}", error),
            },
            _ => panic!("Problem opening the file: {:?}", error),
        },
    };
}

/*与下面等价
fn try_function(text: &str) -> Option<char> {
    // 步骤1：获取第一行
    let first_line = match text.lines().next() {
        Some(line) => line, // 有第一行，解包继续
        None => return None, // 无第一行，返回 None
    };
    // 步骤2：获取第一行的最后一个字符
    first_line.chars().last()}
*/
fn try_function(text: &str) -> Option<char> {
    // 核心逻辑：链式调用 + ? 操作符简化错误处理
    text.lines().next()?.chars().last()
}

fn main() {}
