fn main() {

    // Declaration of Variable
    let x:i32 = 5;
    println!("The Value of x is : {}",x);
    let x:u32 = 6;
    println!("The Value of x is : {}",x);

    const y:i32 = -64;

    // Data Types in Rust

    // Single Type Data Types in Rust

        // Integers
        // Default Type is i32
        let a:i32 = 98_222; //Decimal
        let b:i32 = 0xff; //Hexadecimals
        let c:i32 = 0o77; //Octal
        let d:i32 = 0b1111_0000; //Binary
        let e:u8 = b'A'; //Byte (u8 Only)

        let f:u8 = 255; // Max Value is reached will print 0 is 256

        // Floating Point Number
        // Default Float type is f64
        let f:f64 = 2.0;
        let g:f32 = 3.0;

        // Arithematic Operations in Rust
        // Addition
        let sum:i32 = 5+10;
        // Subtraction
        let difference:f64 = 95.5 - 4.3;
        // Multiplication
        let Product = 4*30;
        // Division
        let Quoteint = 56.7/32.2;
        //Remainder
        let Remainder = 43%5;

        // Boolean Types
        let t:bool = true;
        let f:bool = false;

        // Character
        let c:char = 'z';
        let f:char = 'Z';
    
    
    // Compound Data types in Rust
        // Tuples Store data of different types
        let tup = ("Aditya",100_000); // (0,1)
        // To Access using Destructuring
        let (name,count) = tup;
        // To Acess using dot notations 
        let count = tup.1;

        // Arrays store multiple data of same types
        let codes:[i32; 3] = [200,400,500]; // [0,1,2]
        // To Acess the items in Array
        let found = codes[1];
        // We can also declare arrays by
        let byte:[i32; 8] = [0;8]; // Create 8 valuesall set to 0
        println!("{}",my_function(22,45));
        let summ:i32 = sum_of_numbers(4,5);
        println!("{}",summ);


        // Control Flow in Rust
        let number:i32 = 5;
        if number<10{
            println!("First Condition is true");
        }else if number<22{
            println!("Second Condition is true");
        }else{
            println!("Condition is false");
        }

        // We can also use this format for conditions
        let condition:bool = true;
        let number:i32 = if condition {5} else {6};

        // Rust has 3 Types of Loops
        // Condition Loops
        loop{
            println!("Again");
            break;
        }

        // While Loop
        let mut counter = 3;
        while counter!= 0{
            println!("{}!",counter);
            counter -= 1;
        }
        println!("LISTOFF!!!");

        // For Loop
        let array = [10,20,30,40];
        for element in array.iter(){
            println!("The Value is {}",element);
        }

            // We can also use range
        for number in 1..4{
            println!("{}",number);
        }
}

fn my_function(x:i32,y:i32) ->i32 {
    println!("The Value of x is : {} ",x);
    println!("The Value of y is : {} ",y);
    let sum:i32 = x+y;
    return sum;
}

fn sum_of_numbers(x:i32,y:i32) -> i32{
    println!("The Value of x is : {} ",x);
    println!("The Value of y is : {} ",y);
    x+y
}
