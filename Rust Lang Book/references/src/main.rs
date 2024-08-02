fn main() {
    // To Pass a Reference
    /* 
   let s1: String = String::from("hello");
   let len = calculate_length(&s1);
   println!("The Lenght of '{}' is {}",s1,len);
   */

  // Mutable References in rust
  let mut s1 = String::from("hello");
  change(&mut s1);
  println!("{}",s1);

  // Slices
  let a = [1,2,3,4,5];
  let slice = &a[0..2];
}

fn change(some_string: &mut String){
    some_string.push_str(" ,world");
}

fn calculate_length(s:&String)->usize{
    let lenght = s.len();
    lenght
}
