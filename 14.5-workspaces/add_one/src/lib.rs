pub fn add_one(left: usize) -> usize {
    left + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add_one(3);
        assert_eq!(result, 4);
    }
}
