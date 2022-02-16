#[derive(Debug, PartialEq, Clone)]
struct LinkedList {
    val: u32,
    next: Option<Box<LinkedList>>,
}

impl LinkedList {
    fn new(val: u32) -> Self {
        Self {
            val, next: None
        }
    }

    fn from_vec(vec: Vec<u32>) -> Self {
        if vec.is_empty() {panic!("Empty vectors not supported");}
        let mut output = Self::new(vec[0]);
        if vec.len() == 1 {return output;}
        output.next = Some(Box::new(Self::from_vec(vec[1..].to_vec())));
        output
    }

    fn add(&mut self, item: u32) {
        match self.next {
            None => self.next = Some(Box::new(Self::new(item))),
            Some(ref mut next) => next.add(item),
        }
    }

    fn remove_item(&mut self, item: u32) {
        if self.val != item {
            match self.next {
                None => {}
                Some(ref mut next) => next.remove_item(item),
            }
        } else {
            *self = *self.next.as_ref().unwrap().clone();
        }
    }

    fn contains(&self, item: u32) -> bool {
        if self.val != item {
            match self.next {
                None => false,
                Some(ref next) => next.contains(item),
            }
        } else {
            true
        }
    }
}

impl std::fmt::Display for LinkedList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.val, {if self.next.is_none() {String::new()} else {format!("-> {}", (*self.next.as_ref().unwrap().clone())).trim().to_string()}})
    }
}

#[test]
fn test() {
    let mut ll = LinkedList::from_vec(vec![1,2,3]);
    ll.add(4);
    ll.add(5);
    ll.add(6);
    ll.remove_item(4);

    assert_eq!(ll.to_string(), "1 -> 2 -> 3 -> 5 -> 6".to_string());
    assert_eq!(ll.contains(4), false);
    assert_eq!(ll.contains(5), true);
}

fn main() {}