/*enum Option<T>{
    Some(T),  Some Value              How Option Enum is structured
    None, Null Value
}*/

fn main() {
    let some_number = Some(5);
    let some_string = Some("A String");

    let absent_number:Option<i32> = None;

    let x:i8 = 5;
    let y:Option<i8> = Some(6); // Some Type or None Type
    let sum:i8 = x + y.unwrap_or(0); // WE are saying if none then default value is 0
    println!("{}",&sum);
}
