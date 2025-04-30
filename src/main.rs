#[derive(Debug, PartialEq, Clone)]
struct LinkedList<T: PartialEq + Clone + std::fmt::Display> {
    val: T,
    next: Option<Box<LinkedList<T>>>,
}

impl<T: PartialEq + Clone + std::fmt::Display> LinkedList<T> {
    fn new(val: T) -> Self {
        Self {
            val, next: None
        }
    }

    fn add(&mut self, item: T) {
        match self.next {
            None => self.next = Some(Box::new(Self::new(item))),
            Some(ref mut next) => next.add(item),
        }
    }

    fn remove_item(&mut self, item: T) {
        if self.val != item {
            match self.next {
                None => {}
                Some(ref mut next) => next.remove_item(item),
            }
        } else {
            *self = *self.next.as_ref().unwrap().clone();
        }
    }

    fn contains(&self, item: T) -> bool {
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

impl<T: PartialEq + Clone + std::fmt::Display> std::fmt::Display for LinkedList<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.val, {if self.next.is_none() {String::new()} else {format!("-> {}", (*self.next.as_ref().unwrap().clone())).trim().to_string()}})
    }
}

impl<T: PartialEq + Clone + std::fmt::Display> TryFrom<Vec<T>> for LinkedList<T> {
    type Error = &'static str;

    fn try_from(vec: Vec<T>) -> Result<Self, Self::Error> {
        if vec.is_empty() {return Err("Empty vectors not supported");}
        let mut output = Self::new(vec[0].clone());
        if vec.len() != 1 {output.next = Some(Box::new(LinkedList::try_from(vec[1..].to_vec()).unwrap()));}
        Ok(output)
    }
}

#[test]
fn test() {
    let mut ll = LinkedList::try_from(vec![1,2,3]).unwrap();
    ll.add(4);
    ll.add(5);
    ll.add(6);
    ll.remove_item(4);

    assert_eq!(ll.to_string(), "1 -> 2 -> 3 -> 5 -> 6".to_string());
    assert_eq!(ll.contains(4), false);
    assert_eq!(ll.contains(5), true);
}

fn main() {}