struct User{
    username: String,
    email:String,
    sign_in_count:u64,
    active:bool,
}

#[derive(Debug)]
struct Rectangle{
    lenght:i32,
    widht:i32,
}

impl Rectangle {
    fn square(size: i32) -> Self {
        Self {
            widht: size,
            lenght: size,
        }
    }
}

fn main() {
    let mut user1:User = User{
        email:String::from("ichigo@gmail.com"),
        username:String::from("Ichigo"),
        active:true,
        sign_in_count:1
    };

    let name = user1.username;
    user1.username = String::from("Kurasaki");
    let user2 = build_user(String::from("aizen@gmail.com"),String::from("Aizen"));

    let user3 = User{
        email:String::from("urahara@gmail.com"),
        username:String::from("Urahara"),
        ..user2
    };

    struct Color(i32,i32,i32);
    struct Point(i32,i32,i32);

    let rect = Rectangle{
        lenght:4,
        widht:10
    };
    println!("rect is {:#?}",rect);
    let area_of_rect:i32 = area(rect);
    println!("Area of Rectangle is {}",area_of_rect);
    let square  = Rectangle::square(25);
}

fn area(rectangle:Rectangle)->i32{
    rectangle.lenght*rectangle.widht
}

fn build_user(email:String,username:String) -> User{
    User{
        email,
        username,
        active:true,
        sign_in_count:1,
    }
}
