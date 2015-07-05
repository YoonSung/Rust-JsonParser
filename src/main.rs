use std::collections::HashMap;
struct Node {
    container : HashMap<String, String>,
    childNodes : Vec<Option<Node>>
}

impl Node {
    fn new() -> Node {
       Node{container : HashMap::new(), childNodes : Vec::new()} 
    }

    fn from_string(jsonString : &str) -> Node {
        let mut node = Node::new();
        node.container.insert("key1".to_string(), "value1".to_string());
        node
    }
}

#[test]
fn simpleTest() {
    let mut jsonString = "{'key1' : 'value1'}"; 
    let mut resultNode = Node::from_string(jsonString);
    assert_eq!("value1", resultNode.container.get(&("key1".to_string())).unwrap());
}






/*
use std::collections::HashMap;
struct Node<'a> {
    container : HashMap<&'a str, &'a str>,
    childNodes : Vec<Option<Node<'a>>>
}

impl<'a> Node<'a> {
    fn new() -> Node<'a> {
       Node{container : HashMap::new(), childNodes : Vec::new()} 
    }

    fn from_string(jsonString : &str) -> Node {
        let mut node = Node::new();
        node.container.insert("key1", "value1");
        node
    }
}
/*
trait PartialEq<str> {
    fn eq(&self, other: str) -> bool;

    fn ne(&self, other: str) -> bool { false; }
}
*/
impl<'a> PartialEq<&'a str> for str {

    fn eq(&self, other: str) -> bool {  
        false
    }
}
#[test]
fn simpleTest() {
    let mut jsonString = "{'key1' : 'value1'}"; 
    let mut resultNode = Node::from_string(jsonString);
    assert_eq!("value1", resultNode.container.get(&"key1").unwrap());
}
*/
