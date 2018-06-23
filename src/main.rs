fn main() {
    println!("HW");
}

#[cfg(test)]
mod tests {
    use super::*;

    # [test]
    fn test_fetch_url(){
        main();
    }
}