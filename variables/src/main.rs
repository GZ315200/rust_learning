fn main() {
    let mut x = 5;
    println!("The value of x is:{}", x);
    x = 6;
    println!("The value of x is:{}", x);
    // println!("Hello, world!");
    const MAX_POINTS: u32 = 100_000;
    println!("max points => {}", MAX_POINTS);

    let x = x + 1;
    let x = x * 2;

    println!("The value of x is: {}", x);

    let spaces = "     ";
    let spaces = spaces.len();
    println!("spaces => {} ", spaces);

    let guess: u32 = "42".parse().expect("Not a number!");
    println!("guess number is => {} ", guess);
    // 整型
    let a: u8 = 255;
    println!("a => {}", a);
    // 浮点
    let x = 2.0;
    let y: f32 = 3.0;

    println!("x => {}", x);
    println!("y = {}", y);

    // 加法
    let sum = 5 + 10;

    // 减法
    let difference = 95.5 - 4.3;

    // 乘法
    let product = 4 * 30;

    // 除法
    let quotient = 56.7 / 32.2;

    // 取余
    let remainder = 43 % 5;

    // let f: bool = false;
    // let t = true;

    println!("sum => {}", sum);
    println!("difference => {}", difference);
    println!("product => {}", product);
    println!("quotient => {}", quotient);
    println!("remainder => {}", remainder);

    // char
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("{}", c);
    println!("z => {}", z);
    println!("hello, {}", heart_eyed_cat);

    // 元组类型
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);

    let x: (i32, f64, u8) = (500, 6.4, 1);

    let one = x.2;
    println!("one => {}", one);

    // 数组类型
    let a = [1, 2, 3, 4, 5];

    let b: [i32; 5] = a;
    let c = [3; 5];
    println!("b => {}", b[2]);
    println!("c => {}", c[0]);

    //
    another_funciton(12);

    let x = five();
    println!("The value of x : {}", x);

    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else {
        println!("numer is not divisible by 4, 3, or 2");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("number is {}", number);

    let mut counter = 0;

    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("result => {}", result);

    let mut number = 3;

    while number != 0 {
        println!("{} !", number);

        number = number - 1;
    }
    println!("LIFTOFF!!!!!");

    let a = [10 ,20, 30 , 40 ,50];
    let mut index  = 0;

    while index < a.len() {
        println!("the value is: {}", a[index]);
        index = index + 1;
    }

    for element in a.iter() {
        println!("the value is: {}", element);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }

    let mut input = String::new();
    std::io::stdin().read_line(&mut input)
    .expect("Falied to read line");

    let n: u32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    let mut i = 0;

    while i < n {
        print!("{}  ", fibonacci(i));
        i = i + 1;
    }
}

// 函数
fn another_funciton(x: i32) {
    println!("The value is x : {}", x);
}

fn five() -> i32 {
    5
}

fn fibonacci(n: u32) -> u32 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    } else {
        return fibonacci(n-1) + fibonacci(n - 2);
    }
}