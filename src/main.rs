use std::ops::BitXorAssign;

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

    struct Person {
        name: String,
        age: u32,
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
}

fn print(s: Box<[u8]>) {
    println!("{:?}", s);
}

pub enum Option<T> {
    None,
    Some(T),
}