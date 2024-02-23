fn main() {
    /* variable are mutable by default  */
    let mut x= 10;
    println!("The value of x is : {}",x);
    x=100;
    println!("The value of x is : {}",x);

    /* const  */
    const MY_RESULT : u32= 1000_000;
    println!("The value of MY_RESULT is : {}", MY_RESULT);

    /* Shadowing of varibles  */
    let  s= 10;
    println!("The value of s is : {}",s);
    let s ="ten";
    println!("The value of s is : {}",s);

}   
