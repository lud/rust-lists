pub struct List {
  head: Link,
}

struct Node {
  elem: i32,
  next: Link,
}

pub enum Link {
  Empty,
  // Try with just Node, it compiles
  More(Box<Node>),
}