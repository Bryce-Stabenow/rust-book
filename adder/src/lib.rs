pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }


    #[test]
    fn it_works_2() {
        let result = add(2, 3);
        assert_ne!(result, 9); // Good
    }


    #[test]
    #[should_panic(expected = "AH!")]
    fn it_works_3() {
        panic!("AH!"); 
    }


    #[test]
    fn it_works_4() {
        let error: Result<(), &str> = Err("New Error");
        assert!(error.is_err()); 
    }
}
