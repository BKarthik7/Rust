fn main() {
    println!("{}",fibonachi(6));
}
fn fibonachi(a:u32)->u32{
    if a==0 || a == 1 {return a;}
    fibonachi(a-1)+fibonachi(a-2)
}
