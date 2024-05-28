#![allow(dead_code)] // 이 줄은 컴파일러 경고를 방지해줌

fn do_something_that_might_fail(i : i32) -> Result<f32, String>{
    if i == 42{
        Ok(13.0)
    }
    else{
        Err(String::from("this is wrong!"))
    }
}

fn main() -> Result<(), String>{
    let v = do_something_that_might_fail(42).unwrap();
    println!("{}", v);

    let v = do_something_that_might_fail(42).unwrap();
    println!("{}", v);

    Ok(())
}




// ============= ? 연산자 ==============
// fn do_something_that_might_fail(i : i32) -> Result<f32, String>{
//     if i == 42{
//         Ok(13.0)
//     }
//     else {
//         Err(String::from("this is not 42!"))
//     }
// }

// fn main() -> Result<(), String>{
//     let v = do_something_that_might_fail(41)?;
//     println!("{}", v);
//     Ok(())
// }

// ============= Main 함수에서의 Result 리턴 ==============
// fn do_something_that_might_fail(i : i32) -> Result<f32, String>{
//     if i == 42{
//         Ok(13.0)
//     }
//     else {        
//         Err(String::from("this is not 42!"))
//     }
// }

// fn main() -> Result<(), String>{
//     let result = do_something_that_might_fail(12);

//     match result{
//         Ok(v) => println!("{}", v),
//         Err(_e) => {
//             return Err(String::from("Error Occured in main!"));
//         }
//     }

//     Ok(())
// }

// ============= Result Enum ==============
// // enum Result<T, E> {
// //     Ok(T),
// //     Error(E),
// // }

// fn do_something_that_might_fail(i : i32) -> Result<f32, String>{
//     if i == 42{
//         Ok(13.0)
//     }
//     else {        
//         Err(String::from("this is not 42!"))
//     }
// }

// fn main(){
//     let result = do_something_that_might_fail(42);

//     match result{
//         Ok(v) => println!("{}", v),
//         Err(e) => println!("Error : {}", e),
//     }
// }

// ============= Option Enum ==============
// // enum Option<T> {
// //     None,
// //     Some(T),
// // }

// struct BagOfHolding<T>{
//     item : Option<T>, //Option이라는 이미 내장된 generic enum이 존재한다
// }

// fn main(){
//     let i32_bag = BagOfHolding::<i32> { item : None };

//     if i32_bag.item.is_none(){
//         println!("not exist at all in bag");
//     }
//     else{
//         println!("exist something in bag");
//     }

//     let i32_bag = BagOfHolding::<i32> { item : Some(42) };

//     if i32_bag.item.is_some(){
//         println!("exist something in bag");
//     }
//     else {
//         println!("not exist at all in bag");
//     }

//     match i32_bag.item{
//         Some(v) => println!("i find {} in bag!", v),
//         None => println!("i can't find nothing at all in bag!"),
//     }
// }

// ============= 아무 값도 없음을 표현 (NULL 형식같은) ==============
// enum Item{
//     Inventory(String),
//     None,
// }

// struct BagOfHolding{
//     item : Item,
// }

// ============= Generic 자료형 (Templete 형식같은) ==============
// struct BagOfHolding<T> {
//     item : T,
// }

// fn main(){
//     let i32_bag = BagOfHolding::<i32> { item : 42 };
//     let bool_bag = BagOfHolding::<bool> { item : true };
//     let float_bag = BagOfHolding { item : 3.14 }; //Generic에서도 자료형 유추 가능

//     let bag_in_bag = BagOfHolding{
//         item : BagOfHolding { item : "boom" },
//     };

//     println!("{} {} {} {}", i32_bag.item, bool_bag.item, float_bag.item, bag_in_bag.item.item);
// }

// ============= 열거형 (enum) 데이터의 사용법 ==============
// enum Species{
//     Crab,
//     Octopus,
//     Fish,
//     Clam,
// }
// enum PoisonType {
//     Acidic,
//     Painful,
//     Lethal,
// }
// enum Size {
//     Big,
//     Small,
// }
// enum Weapon{
//     Claw (i32, Size),
//     Position(PoisonType),
//     None,
// }
// struct SeaCreature
// {
//     species: Species,
//     name : String,
//     arms : i32,
//     legs : i32,
//     weapon : Weapon,
// }

// fn main()
// {
//     let ferris = SeaCreature{
//         species: Species::Crab,
//         name : String::from("Ferris"),
//         arms : 2,
//         legs : 4,
//         weapon : Weapon::Claw(2, Size::Big),
//     };

//     match ferris.species{
//         Species::Crab => match ferris.weapon{
//             Weapon::Claw(num_claws, size) => {
//                 let size_decription = match size {
//                     Size::Big => "Big",
//                     Size::Small => "Small",
//                 };

//                 println!("ferris has {} {} claw(s)", num_claws, size_decription);
//             },
//             _ => println!("ferris weapon unknown.."),
//         },

//         _ => println!("ferris species unknown.."),
//     }
// }

// ============= 열거형 (enum) 데이터 ==============
// enum Species{
//     Crab,
//     Octopus,
//     Fish,
//     Clam,
// }
// struct SeaCreature
// {
//     species: Species,
//     name : String,
//     arms : i32,
//     legs : i32,
//     weapon : String,
// }
// fn main() 
// {
//     let ferris = SeaCreature{
//         species: Species::Crab,
//         name : String::from("Ferris"),
//         arms : 2,
//         legs : 4,
//         weapon : String::from("Claw"),
//     };

//     match ferris.species{
//         Species::Crab => println!("{} is Crab", ferris.name), 
//         Species::Octopus => println!("{} is Octopus", ferris.name), 
//         Species::Fish => println!("{} is Fish", ferris.name), 
//         Species::Clam => println!("{} is Clam", ferris.name), 
//     }
// }

// ============= 튜플 형태의 구조체 ==============
// struct Location(i32, i32);

// fn main() 
// {
//     let loc = Location(42, 32);
//     println!("x : {}, y : {}", loc.0, loc.1);   
// }

// ============= 메모리에 데이터 생성 ==============
// struct SeaCreature
// {
//     animal_type: String,
//     name : String,
//     arms : i32,
//     legs : i32,
//     weapon : String,
// }

// fn main() 
// {
//     let ferris = SeaCreature{
//         animal_type: String::from("Crab"),
//         name : String::from("Ferris"),
//         arms : 2,
//         legs : 4,
//         weapon : String::from("Claw"),
//     };

//     let craken = SeaCreature{
//         animal_type: String::from("Octopus"),
//         name : String::from("Craken"),
//         arms : 8,
//         legs : 0,
//         weapon : String::from("Legs"),
//     };

//     println!("{} type is {} and he has {} arms, {} legs, {} weapon.", ferris.name, ferris.animal_type, ferris.arms, ferris.legs, ferris.weapon);
//     println!("{} name is {} and he has {} arms, {} legs, {} weapon.", craken.name, craken.animal_type, craken.arms, craken.legs, craken.weapon);   
// }

// ============= 구조체 (struct) 형식과 메소드 호출 방법 ==============
// struct SeaCreature
// {
//     animal_type: String,
//     name : String,
//     arms : i32,
//     legs : i32,
//     weapon : String,
// }

// fn main()
// {
//     let s = String::from("Hello world!");
//     println!("length of {} is {}", s, s.len());
// }

// ============= if, match 리턴 방식 ==============
// fn example() -> i32
// {
//     let food = "hamburger";
//     let result = match food {
//      "hotdog" => "This is hotdog!",
//      _ => "this is not hotdog..",
//     };
 
//     println!("Result is {}", result);
    
//     let v = {
//         let a = 1;
//         let b = 2;
//         a + b
//     };

//     println!("v = {}", v);

//     v + 4
// }

// fn main() 
// {
//     println!("example result : {}", example());
// }

// ============= loop문에서 값 리턴하기 (break와 동시에 리턴) ==============
// fn main() 
// {
//    let mut  x = 0;
//    let v = loop {
//        x += 1;

//        if x == 13
//        {
//             break "found 13";
//        }
//    };

//    println!("{}", v);
// }

// ============= match문 (like switch) ==============
// fn main() 
// {
//     let x = 1;

//     match x
//     {
//         0 => 
//         {
//             println!("zero!");
//         }
//         1 | 2 => 
//         {
//             println!("one or two");
//         }
//         3..=9 => 
//         {
//             println!("3 to 9");
//         }
//         matched_num @ 10..=100 => 
//         {
//             println!("{} in 10 to 100", matched_num);
//         }
//         _ =>
//         {
//             println!("else");
//         }
//     }
// }

// // ============= for 반복문 ==============
// fn main() 
// {
//    for x in 0..5
//    {
//         println!("{}", x);
//    }

//    for x in 0..=5
//    {
//         println!("{}", x);
//    }
// }

// // ============= while 반복문 ==============
// fn main() 
// {
//    let mut x = 0;

//    while x != 42
//    {
//         x += 1;
//    }

//    println!("{}", x);
// }

// ============= loop 반복문 ==============
// fn main() 
// {
//    let mut x = 0;
   
//    loop 
//    {
//         x += 1;

//         if x == 50
//         {
//             break;
//         }
//    }

//    println!("{}", x);
// }

// ============= if문 삼항식 표현 ==============
// fn main() 
// {
//    let x = 42;
//    let v = if x < 42 { -1} else {1};

//    println!("{}", v);
// }

//============= if else 문 ==============
// fn main() 
// {
//    let x = 43;

//    if x < 42
//    {
//         println!("down then 42");
//    }
//    else if x == 42
//    {
//         println!("equal with 42");
//    }
//    else 
//    {
//         println!("up to 42");       
//    }
// }

//============= void 반환형 함수들 표현 ==============
// fn nothing1() -> ()
// {
//     return ();
// }

// fn nothing2()
// {

// }

// fn main() 
// {
//     println!("{:?}", nothing1());
//     println!("{:?}", nothing2());
// }

//============= 리턴 값이 여러 개인 함수 ==============
// fn swap(x: i32, y : i32) -> (i32, i32)
// {
//     return (y,x);
// }

// fn main() 
// {
//     let result = swap(1,9);
//     println!("{} {}", result.0, result.1);

//     let (a,b) = swap(result.0, result.1);
//     println!("{} {}", a,b);
// }

//============= 함수 표현 ==============
// fn add(x : i32, y : i32) -> i32 
// {
//     return x + y;
// }
// fn main() 
// {
//     println!("{}", add(3,2));
// }

//============= 배열 표현 ==============
// fn main() 
// {
//     let nums : [i32; 3] = [1,2,3];
//     println!("{:?}", nums);
//     println!("\n");
//     println!("{}", nums[1]);
// }

//============= 상수 표현 ==============
// fn main() 
// {
//     const A : f32 = 3.14; //상수에서는 자료형을 반드시 명시해줘야 함
//     println!("hehe {} he", A);
// }

//============= 타입이 다른 변수들끼리의 연산 ==============
// fn main() 
// {
//     let x= 13u32;
//     let y = 14u8;
//     let z = x + y as u32;

//     println!("{}", z);
// }

//============= 변수 자료형 ==============
// fn main() {

//     let x = 13;
//     println!("{}", x);

//     let x = 16;
//     println!("{}", x);

//     let y = 18;
//     println!("{} {}", x, y);

//     let mut a = 4.36;
//     a = 20.6;  
//     println!("{}", a);

//     let mut string = "aaaa"; //mut가 있어야 변수의 값을 변경 가능
//     string = "bbbb";
//     println!("{}", string);
// }