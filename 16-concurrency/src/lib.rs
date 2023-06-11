
fn add_one(x: i32) -> i32 { x + 1 }

fn apply_ops(x: i32) -> i32 { x }

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn add() {
        assert_eq!(add_one(12), 13);
    }
}

