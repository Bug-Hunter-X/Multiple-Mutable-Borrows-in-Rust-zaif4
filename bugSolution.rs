fn main() {
    let mut x = 5;
    { //This is a solution, you can use this type of method, by creating a scope where the mutable borrow is only valid inside the scope. 
        let y = &mut x;
        *y += 1;
    }
    { //another mutable borrow is available here
        let z = &mut x;
        *z += 1;
    }
    println!("x = {}", x);
}