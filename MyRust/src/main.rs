#![allow(unused_variables,non_snake_case)]
fn main() {
    println!("Hello, world!");

    //変数の束縛
    let a = 5;
    println!("{}",a);

    //パターン
    let (x,y) = (1,2);
    println!("({},{})",x,y);

    //型アノテーション
    let b:i32 = 6;
    println!("{}",b);

    //可変性
    let mut c = 8;
    println!("{}",c);
    c = 7;
    println!("{}",c);

    //スコープとシャドーイング
    let d:i32 = 17;
    {
        println!("{}",d);
        let d:i32 = 3;
        println!("{}",d);
    }
    println!("{}",d);
    let d = 42;
    println!("{}",d);
}
