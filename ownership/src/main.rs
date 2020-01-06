

fn main() {
    //
    // {
    //     let s = String::from("hello"); // 从此处起，s 是有效的

    //     // 使用 s
    //     println!("string is {}", s);
    // }                                  // 此作用域已结束， // s 不再有效

    let x = 5;
    let y = x;

    println!("y is {}", y);
    println!("x is {}", x);

    let s1 = String::from("hello");
    let s2 = &s1;
    println!("s1 is {}", s1);
    println!("s2 is {}", s2);

    let s = String::from("hello");  // s 进入作用域

    takes_ownership(s);             // s 的值移动到函数里 ...
                                    // ... 所以到这里不再有效
    let x = 5;                      // x 进入作用域

    makes_copy(x);

    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);


    let mut s = String::from("hello");
    change(&mut s);
    println!("s is {}", s);

    dangle();
    let s = String::from("hello");
    // let slice = &s[0..2];
    // println!("slice is {}", slice);
    println!("first word is {}", first_word(&s));
}

fn takes_ownership(some_string: String) { // some_string 进入作用域
    println!("{}", some_string);
} // 这里，some_string 移出作用域并调用 `drop` 方法。占用的内存被释放

fn makes_copy(some_integer: i32) { // some_integer 进入作用域
    println!("{}", some_integer);
} // 这里，some_integer 移出作用域。不会有特

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(", world")
}

fn dangle() -> String {
    let s = String::from("hello");
    s
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

