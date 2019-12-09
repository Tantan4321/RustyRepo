fn main() {
    println!("Hello, world!");
}

fn vectorize(n: String) -> Vec<u32>{
    let mut ret: Vec<u32> = Vec::new();
    for i in 0..n.len(){
        let splice = &n[i..i+1];
        ret.push(splice.to_string().parse().unwrap());
    }
    ret
}
