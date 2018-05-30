use std::io;
fn main() {
    println!("Please enter the number!");
    let mut num = String::new();
    io::stdin().read_line( &mut num)
        .expect("please enter integer number");
    let num: i32= num.trim().parse()
        .expect("please enter integer number");
    let  mut fibs: Vec<i32> = Vec::new();
    fibs.push(0);
    fibs.push(1);

    let  mut i= 2;
    //let  lastval: Option<&i32> = fibs.get(fibs.len()-1);
    while i<num {
       let  lastval:i32  = fibs.get(fibs.len()-1).unwrap()+ fibs.get(fibs.len()-2).unwrap();
       //println!("{}",lastval);
        fibs.push(lastval);
        i = i+1;

    }

    println!("{:?}",fibs);

}
