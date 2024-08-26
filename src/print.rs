pub fn print() {
    println!("Hello, locha from print file!");

    let name = "Locha";
    let mut age = 30;
    println!("{} is {} years old", name, age);

    age = 25;
    const ID: i64 = 117;
    println!("Wrong! {} is {} and his ID is {}", name, age, ID);
}