trait AppendBar {
    fn append_bar(self) -> Self;
}


impl AppendBar for Vec<String> {
    fn append_bar(mut self) -> Self {
        let n = "Bar".to_string();
        self.push(n);
        self
    }
}

fn main() {
    let mut n = vec!["It's a new string".to_string()];
    n = n.append_bar();
    println!("{:?}", n)
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_vec_pop_eq_bar() {
        let mut foo = vec![String::from("Foo")].append_bar();
        assert_eq!(foo.pop().unwrap(), "Bar");
        assert_eq!(foo.pop().unwrap(), "Foo");
    }
}
