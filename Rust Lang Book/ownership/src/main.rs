fn main() {
    let s1 = gives_ownership();
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);
    println!("s1 = {}, s3 = {}",s1,s3);
}

fn gives_ownership()->String{
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string : String)-> String{
    a_string
}


// The Ownership goes like this: s2->a->s3
// To Avoid this we will use references