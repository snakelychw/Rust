

fn main() {
let mut x = 5;
let y = &mut x;
*y =  *y + 1;
println!("{}", y);
}