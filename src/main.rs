use chrono::prelude::*;

#[allow(dead_code)]

const PI: f32 = 3.14159f32;

// 1.7 Functions
fn add(x: i32, y: i32) -> i32 {
    return x + y;
}

fn subtract(x: i32, y: i32) -> i32 {
    return x - y;
}

// 1.7,1 multiple return values
fn swap(x: i32, y: i32) -> (i32, i32) {
    return (y, x);
}

// 1.7.2 return noting
fn make_nothing() -> () {
    return ();
}

fn make_nothing2() {
}

fn example() -> i32 {
    let x = 42;
    let v = if x < 42 { -1 } else { 1 };
    println!("from if: {}", v);

    let food = "hambuger";
    let result = match food {
        "hootdog" => "is hotdong",
        _ => "is not hotdong",
    };
    println!("identifying food: {}", result);

    let v = {
        let a = 1;
        let b = 2;
        a + b
    };
    println!("from block: {}", v);

    v + 4
}

// 3.2 tuple like structure
struct Location(i32, i32);

// 3.3 unit like structure
struct Marker;

// 3.4 enmerationsG

enum Species {
    Crab,
    Otopus,
    Fish,
    Clam
}

enum PoisonType {Acidic, Painful, Lethal}

enum Size {Big, Small}

enum Weapon {
    Claw(i32, Size),
    Poison(PoisonType),
    None
}


union myUnion {
    f1: u32,
    f2: f32,
}
struct SeaCreature {
    species: Species,
    name: String,
    arms: i32,
    legs: i32,
    weapon: Weapon,
}

impl SeaCreature {
    fn get_name(&self) -> &String {
        &self.name
    }
}

// 4. Generic types
struct BagOfHoliding<T> {
    item: Option<T>,
}

enum Item {
    Inventory(String),
    None,
}

// 4.2 Result types
fn do_something_that_minght_fail(i: i32) -> Result<f32, String> {
    if i == 42 {
        Ok(13.0)
    } else  {
        Err(String::from("this is not the right number"))
    }
}

// 5.1 Ownership
struct Foo {
    x: i32,
}

// 5.2 Moving ownership
fn do_something(f: Foo) {
    println!("{}", f.x);
}

fn main() -> Result<(), String> {
    // Hello Rust
    println!("Hello, Rust!");

    // 1. The Basic
    println!("Welcome to the playground! you can modify the code in here.");
    
    // 1.1 Variables
    let x: i32= 13;
    println!("{}", x);

    let x: f64 = 3.14159;
    println!("{}", x);

    let x: i32;
    x = 0;
    println!("{}", x);

    // 1.2 Changing Varibles
    let mut x: i32 = 42;
    println!("{}", x);
    x = 13;
    println!("{}", x);

    // 1.3 Basic types
    //      bool
    //      u8 u16 u32 u64 u128
    //      i8 i16 i32 i64 i128
    //      usize isize
    //      f32   f64
    //      tuple - (value, value, ...)
    //      array - [value, vjalue, ...]
    //      slices
    //      str(string slices)
    let bx: bool = true;
    let v32x: i32 = 12_i32; // integer default type is  i32
    let v8x: u8 = 12u8;
    let sentence: &str = "Hello, world!";
    let t: (i32, bool) = (13, false);
    println!("{} {} {} {} {} {}", bx, v32x, v8x, sentence, t.0, t.1);

    // 1.4 Basic types Conversion
    let a: u8 = 13u8;
    let b: u32 = 7u32;
    let c: u32 = a as u32 + b;
    println!("{}", c);

    let t: bool = true;
    println!("{}", t as u8);

    // 1.5 Constants
    println!("{}", PI);

    // 1.6 arrays
    let nums: [i32; 3] = [1, 2, 3];
    println!("{:?}", nums);
    println!("{}", nums[1]);

    // 1.7 Functions
    println!("42 + 13 {}", add(42, 13));
    println!("42 - 13 {}", subtract(42, 13));

    // 1.7.1 multiple return values
    let result = swap(123, 321);
    println!("{} {}", result.0, result.1);

    let (a, b) = swap(result.0, result.1);
    println!("{} {}", a, b);

    // 1.7.2 Return nothing
    let a = make_nothing();
    let b  = make_nothing2();
    println!("The value of a: {:?}", a);
    println!("The value of b: {:?}", b);

    // 2. Basic Control
    // 2.1 if/else == != < > <= >= ! || &&
    let x: u8 = 43_u8;
    if x < 42 {
        println!("Less than 42");
    } else if  x == 42 {
        println!("is 42");
    } else {
        println!("greater than 42");
    }

    // 2.2 loop
    let mut x: u32 = 0;
    loop {
        x += 1;
        if  x == 42 {
            break;
        }
    }
    println!("{}", x);

    x = 0;
    let v = loop {
        x += 1;
        if x == 13 {
            break "found the 13";
        }
    };
    println!("from loop: {}", v);

    // 2.3 while
    let mut x: u32 = 0_u32;
    while x != 42 {
        x += 1; 
    }
    println!("{}", x);

    // 2.4 for
    // .. means [)
    for x in 0..5 {
        println!("{}", x);
    }

    // ..= means []
    for x in 0..=5 {
        println!("{}", x);
    }

    // 2.5 match
    let x: u32 = 42;
    match x {
        0 => { println!("foud zero");}
        1 | 2 => {println!("found 1 or 2");}
        3..=9 => {println!("found a num 3 to 9 inclusively");}
        matched_num @ 10..=100 => {
            println!("found {} number [10, 100]", matched_num);
        }
        _ => { println!("found something else!")}
    }

    // 2.6 returning values from block expressions
    println!("from function : {}", example());

    // 3. basic data structure types
    //struct SeaCreature {
    //    animal_type: String,
    //    name: String,
    //    arms: i32,
    //    legs: i32,
    //    weapon: String,
    //}
    
    //let ferris = SeaCreature{
    //    animal_type: String::from("Crab"),
    //    name: String::from("Ferris"),
    //    arms: 2,
    //    legs: 4,
    //    weapon: String::from("claw"),
    //};

    //println!("{} is a {}, They have {} arms, {} legs, and a {} weapon", ferris.name, ferris.animal_type, ferris.arms, ferris.legs, ferris.weapon);

    // 3.1 call methods
    //     :: for static methods
    //     .  for instance methods
    let s = String::from("Hello world");
    println!("{} is {} characters long", s, s.len());

    // 3.2 tuple-like struct
    let loc = Location(42, 32);
    println!("{}, {}", loc.0, loc.1);

    // 3.3 Unit-like struts
    let _m = Marker;

    // 3.4 enumerations
    let ferris = SeaCreature {
        species: Species::Crab,
        name: String::from("Ferris"),
        arms: 2,
        legs: 4,
        weapon: Weapon::Claw(2, Size::Small),
    };

    println!("{}", ferris.get_name());
    match ferris.species {
        Species::Crab => {
            match ferris.weapon {
                Weapon::Claw(num_claws, size) =>  {
                    let size_des = match size {
                        Size::Big => "big",
                        Size::Small => "small",
                    };
                    println!("ferris is a crab with {} {} claw", num_claws, size_des);
                },
                _ => println!("ferris is a crab with some other weapon")
            }
        },
        Species::Otopus => println!("{} is a otopus", ferris.name),
        Species::Fish => println!("{} is a fish", ferris.name),
        Species::Clam => println!("{} is a clam", ferris.name),
    }

    // 4. Generic types
    let _i32_bag = BagOfHoliding::<i32> {item: Some(42)};
    let _bool_bag = BagOfHoliding::<bool> {item: Some(true)};
    let _float_bag = BagOfHoliding {item: Some(3.14)};
    //let bag_in_bag = BagOfHoliding {item: Some(BagOfHoliding{ item: Some("boom!")})};

    //println!("{} {} {} ", i32_bag.item.value, bool_bag.item.value, float_bag.item.value);

    // 4.1 Option
    let mut i32_bag = BagOfHoliding::<i32> {item: None};
    if i32_bag.item.is_none() {
        println!("there is nothig in the bag");
    } else  {
        println!("there is something in the bag");
    }

    match i32_bag.item {
        Some(v) => println!("found {} in bag", v),
        None => println!("found nothing in bag"),
    }

    i32_bag = BagOfHoliding::<i32> {item: Some(42)};
    // unwrapping a 'Some' variant will extract the value wrapped
    println!("i32_bag.item value {}", i32_bag.item.unwrap());

    // 4.2 Result
    let mut result = do_something_that_minght_fail(12);
    result = Result::Ok(42.0);

    match result {
        Ok(v) => println!("found {}", v),
        Err(e) =>  {
            return Err(String::from("Something went wrong in main"));
        }
    }

    let v = do_something_that_minght_fail(42)?;
    // do_something_that_might_fail()?
    // 
    // means:
    // 
    // match do_something_that_might_fail() {
    //        Ok(v) => v,
    //        Err(e) => return Err(e),
    //} 

    println!("found v {}", v);

    // 4.3 Vector
    let mut i32_vec = Vec::<i32>::new();
    i32_vec.push(1);
    i32_vec.push(2);
    i32_vec.push(3);

    let mut float_vec = Vec::new();
    float_vec.push(1.2);
    float_vec.push(2.3);
    float_vec.push(3.4);

    let string_vec = vec![String::from("hello"), String::from("world")];

    for word in string_vec.iter() {
        println!("{}", word);
    }

    // 5.1 Ownership
    let foo = Foo {x: 42};
    let foo_b = Foo {x: 13};
    println!("foo value {}", foo.x);
    println!("foo_b value {}", foo_b.x);

    let foo_c = &foo;

    println!("foo_c {}", foo_c.x);

    do_something(foo);

    let mut foo_d = 42;
    let f = &mut foo_d;
    let bar = *f;
    *f = 13;
    println!("{}", bar);
    println!("{}",foo_d);

    //println!("foo has droped {} ", foo.x);

    let utc: DateTime<Utc> = Utc::now();
    let local: DateTime<Local> = Local::now();

    println!("{:?}", utc);
    println!("{:?}", local);
    
    println!("{:?}", local.format("%Y-%m-%d %H:%M:%S").to_string());


    let mut u = myUnion {f1:1};
    unsafe {u.f1 = 5};
    let value = unsafe{u.f1};

    unsafe {
        match u {
            myUnion {f1 : 10} => {println!("10");}
            myUnion {f2 : 0.01} => {println!("{}", u.f2);}
            _ => {println!("aaaaa");}
        }
    }

    println!("{:?}", value);

    Ok(())
}