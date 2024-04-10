pub struct Node {
    elem: i32,
    next: Link,
}

enum Link {
    Empty,
    More(Box<Node>),
}

pub enum List {
    Empty,
    More(Box<Node>),
}
