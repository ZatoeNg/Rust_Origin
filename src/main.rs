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
    } else if length <= 0 {
        println!("{password} is shorter enough!");
    } else {
        println!("{password} is Not long enough!");
    }
}

fn loop_function() {
    let mut count:i32 = 0;
    loop {
        count += 1;
        println!("Loop forever!={:?}",count);

        if count == 10 {
            println!("Loop Break!");
            break;
        }else if count == 6 {
            println!("Loop continue!");
            continue;
        }
    }
}

fn main() {
    loop_function();
}
