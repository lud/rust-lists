pub struct List {
  head: Link,
}

enum Link {
  Empty,
  // Try with just Node, it compiles
  More(Box<Node>),
}

struct Node {
  elem: i32,
  next: Link,
}

