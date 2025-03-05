use std::io::Write; //write, writelnマクロを使うため

struct Droppable; 

impl Drop for Droppable {
    fn drop (&mut self) {
        println!("Resource will be releaced!");
    }
}

fn make_tuple<T, S>(t: T, s: S) -> (T, S) {
    (t, s)
}

trait Tweet {
    fn tweet(&self);
    fn tweet_twice(&self) {
        self.tweet();
        self.tweet();
    }

    fn shout(&self) {
        println!("Uooooooooooohhhh!!!!!!!!!");
    }
}

struct Dove;
struct Duck;

impl Tweet for Dove {
    fn tweet(&self) {
        println!("Coo!");
    }
}

impl Tweet for Duck {
    fn tweet(&self) {
        println!("Quack!");
    }
}

struct Penguin {
    name: &'static str,
}



//Eqを実装するために　PartialEqが必要
#[derive(Eq, PartialEq)]
struct A(i32);
//PartialOrdを実装するためにpartialEqが必要
#[derive(PartialEq, PartialOrd)]
struct B(f32);
//Copyを実装するためにCloneが必要
#[derive(Copy, Clone)]
struct C;
#[derive(Clone)]
struct D;
#[derive(Debug)]
struct E;
#[derive(Default)]
struct F;




fn f(x: i32) -> &'static str {
    match x {
        n if n * n % 3 == 0 => "3n",
        n if n * n % 3 == 1 => "3n+1 or 3n+2",
        _ => unreachable!(), // コンパイラは上記条件で網羅していることを判定できない
    }
}
struct Person {
    name: String,
    age: u32,
}

enum Emotion {
    Anger,
    Happy,
}

trait Emotional {
    fn get_happy(&mut self) -> String;
    fn get_anger(&mut self) -> String;
    fn tell_state(&self) -> String;
}

struct HappyPerson {
    name: String,
    state: Emotion,
}

impl Emotional for HappyPerson {
    fn get_anger(&mut self) -> String {
        // この関数は呼ばれないので実装しないが、String型を返さなくても型検査を通過させる
        unimplemented!()
    }
    fn get_happy(&mut self) -> String {
        format!("{} is always happy", self.name)
    }
    fn tell_state(&self) -> String {
        //この関数はあとで実装したいが、一旦String型を返さなくても型検査を通過させる
        todo!()
    }
}

fn main() {
    println!("Hello, world!");

    let s1: String = String::from("Hello, World!");
    let s2: &str = &s1;
    let _s3: String = s2.to_string();

    let mut t  = (1, "2");
    t.0 = 2;
    t.1 = "3";

    let mut a: [i32; 3] = [0, 1, 2];
    let b: [i32; 3] = [0; 3];
    a[1] = b[1];
    a[2] = b[2];
    println!("{:?}", &a[1..3]);

    pub enum Option<T> {
        None,
        Some(T),
    }

    let _p: Person = Person {
        name: String::from("John"),
        age: 8,
    };

    enum Event {
        Quit,
        KeyDown(u8),
        MouseDown { x: i32, y: i32 },
    }

    let e1 = Event::Quit;
    let e2 = Event::MouseDown {x: 10, y: 20};

    let result: Result<i32, String> = Ok(200);

    match result {
        Ok(code) => println!("code: {}", code),
        Err(err) => println!("Err: {}", err),
    }

    let result: Result<i32, String> = Ok(200);

    if let Ok(code) = result {
        println!("code: {}", code);
    }

    //一行前のlet文二つをunwrap_or()で書き直した場合
    let result: Result<i32, String> = Ok(200);
    println!("code: {}", result.unwrap_or(-1)); //=> "code: 200"
    let result: Result<i32, String> = Err("error".to_string());
    println!("code: {}", result.unwrap_or(-1)); //=> "code: -1"
    
    fn func(code: i32) -> Result<i32, String> {
        println!("code: {}", code);
        Ok(100)
    }

    let result: Result<i32, String> = Ok(200);
    let next_result = result.and_then(func); //func()は実行される
    let result: Result<i32, String> = Err("error".to_string());
    let next_result = result.and_then(func); //func()は実行されない

    fn error_handling(result: Result<i32, String>) -> Result<i32, String> {
        let result = result?; //エラーの場合はここでreturn result;
        println!("code: {}", result);
        Ok(100)
    }

    let v1 = vec![1, 2, 3, 4, 5]; //1～5の数字を入れて初期化
    let v2 = vec![0; 5]; //0を5つ入れて初期化

    let v = vec![1, 2, 3, 4, 5];
    println!("{}", v[0]);

    let v = vec![1, 2, 3, 4, 5];
    for element in &v {
        println!("{}", element);
    }

    let byte_array = [b'h', b'e', b'l', b'O'];
    print(Box::new(byte_array));
    let byte_array = [b'h', b'e', b'l', b'o', b'!'];
    print(Box::new(byte_array));

    let immut_val = 10;
    let mut mut_val = 20;

    mut_val += immut_val;

    let v1: u64 = 10;
    let v2 = 10u64;

    let number = 1;
    if 0 < number {
        println!("0 < number");
    } else if number < 0 {
        println!("number < 0");
    } else {
        println!("0 == number");
    }

    let number = 1;
    let result = if 0 <= number {
        number
    } else {
        -number
    };

    let mut count = 0;

    let result = loop {
        println!("count: {}", count);
        count += 1;
        if count == 10 {
            break count;
        }
    };

    let mut count = 0;
    while count < 10 {
        println!("count: {}", count);
        count += 1;
    }

    let count: i32;

    for count in 0..10 {
        println!("count: {}", count);
    }

    let array = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

    for element in &array {
        println!("element: {}", element);
    }

    'main: loop {
        println!("main loop start");
        'sub: loop {
            println!("sub loop start");

            break 'main;

            println!("sub loop end"); //表示されない
        }
        println!("main loop end"); //表示されない
    }

    let i: i32 = 1;
    match i {
        1 => println!("1"),
        2 => println!("2"),
        3 => println!("3"),
        _ => println!("misc"),
    }

    enum Color {
        Red,
        Blue,
        Green,
    }

    let c = Color::Red;
    match c {
        Color::Red => println!("Red"),
        Color::Blue => println!("Blue"),
        Color::Green => println!("Green"),
    }

    let result: Result<i32, String> = Ok(100);
    let result_number = match result {
        Ok(number) => number,
        Err(message) => {
            println!("Error: {}", message);
            -1
        },
    };

    for number in 1..5 {
        println!("{}", number);
    }

    let it = Iter {
        current: 0,
        max: 10,
    };
    for num in it {
        println!("{}", num);
    }

    let mut v = vec![1, 2, 3, 4, 5];
    for i in 0..v.len() {
        v[i] += 1;
    }

    let x = add(1, 2);
    println!("x = {}", x);
    
    let p = Person {
        name: String::from("Taro"),
        age: 20,
    };

    p.say_name().say_age();

    let s = format!("{}-{:?}", "Ab2", ("D", 5)); //s = String::form("Ab23(\"D\", 5)")と同じ
    let s = format!("{}{}", "abc", "def"); //s = String::from("abcdef")と同じ
    let s = concat!("A", "b2", 3); //s = String::from("Ab23")と同じ

    print!("hello");
    println!("hello {}", "world");
    eprint!("hello {}", "error");
    eprint!("hello");

    let mut w = Vec::new(); //バイト列書き込み用のVecを宣言
    write!(&mut w, "{}", "ABC"); //UTF-8バイト列で65, 66, 67
    writeln!(&mut w, "is 123"); //UTF-8バイト列で32, 105, 115, 32, 49, 50, 51, 10
    dbg!(w); //フォーマット文字列を受け取らない

    panic!("it will be panic");
    // thread 'main' panicked at 'it will be panic', src/main.rs:229:5

    let v = vec![1, 2, 3];

    println!("defined in file: {}", file!());
    println!("defined in line: {}", line!());
    println!("is test: {}", cfg!(unix));
    println!("CARGO_HOME: {}", env!("CARGO_HOME"));

    assert!(true);
    assert_eq!(1, 1);
    assert_ne!(1, 0);
    debug_assert!(false);
    debug_assert_eq!(1, 1);
    debug_assert_ne!(1, 0);

    let mut p = HappyPerson {
        name: "Takashi".to_string(),
        state: Emotion::Happy,
    };
    println!("{}", p.get_happy());

    // Aは一致比較可能
    println!("{:?}", A(0) == A(1));

    // Bは大小比較可能
    println!("{:?}", B(1.0) > B(0.0));

    // Cはムーブではなくコピーされる
    let c0 = C;
    let _c1 = c0;
    let _c2 = c0; // Cがムーブならc0は_c1へムーブしているのでここでコンパイルエラー

    // D　はclone可能
    let d0 = D;
    let _d1 = d0.clone();

    // Eはデバッグプリント{:?}可能
    println!("{:?}", E);

    // Fはdefault可能
    let _f = F::default();

    let dove = Dove {};
    dove.tweet();
    dove.tweet_twice();
    dove.shout();

    let duck = Duck {};

    let bird_vec: Vec<Box<dyn Tweet>> =vec![Box::new(dove), Box::new(duck)];
    for bird in bird_vec {
        bird.tweet();
    }

    let t1 = make_tuple(1, 2);
    let t2 = make_tuple("Hello", "World!");
    let t3 = make_tuple(vec![1, 2, 3], vec![4, 5]);
    let t4 = make_tuple(3, "years old");

    let mut important_data = "Hello, World!".to_string();

    important_data = calc_data(important_data); //値の所有権を渡し、返してもらう
    println!("{}", important_data);

    let important_data = "Hello,World!".to_string();
    calc_data_ref(&important_data); //参照渡し
    println!("{}", important_data);

    let x = 5;
    let y = &x;
    let z = &x;
    dbg!(x);
    dbg!(y);
    dbg!(z);

    let mut x = 5;
    {
        let y = &mut x; // 1回目可変な参照渡し
        let z = &mut x; // 2回目の可変な参照渡し（エラー）
        dbg!(y);
        dbg!(z);
    }

    {
        let y = &x; // 1回目の不変な参照渡し
        let z = &mut x; // 可変な参照渡し（エラー）
        dbg!(y);
        dbg!(z);
    }

    let y;
    {
        let x = 5; // xのライフタイム（始まり）
        y = &x; // yのライフタイム（始まり）
        dbg!(y); // xのライフタイム（終わり）
    }
    dbg!(y); // yのライフタイム（終わり）
    // xよりもyが長く生存することはできない（エラー）

    {
        let d = Droppable;
    }
    println!("The Droppable should be releaced at the end of block.");
}

fn calc_data_ref(data: &String) {
    println!("{}", data);
}

fn calc_data(data: String) -> String {
    println!("{}", data);
    data
}

fn print(s: Box<[u8]>) {
    println!("{:?}", s);
}

struct Iter {
    current: usize,
    max: usize,
}

impl Iterator for Iter {
    type Item = usize; //出力する型の紐づけ

    //next()メソッドの実装
    fn next(&mut self) -> Option<usize> {
        self.current += 1;
        if self.current - 1 < self.max {
            Some(self.current - 1)
        } else {
            None
        } 
    }
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn abs(number: i32) -> i32 {
    if number < 0 {
        return -number;
    }
    number
}

impl Person {
    fn say_name(&self) -> &Self {
        println!("I am {}.", self.name);
        self
    }

    fn say_age(&self) -> &Self {
        println!("I am {} years old.", self.age);
        self
    }
}