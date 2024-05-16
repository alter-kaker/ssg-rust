use std::sync::{Arc, Mutex};

use crate::error::GeneratorError;

#[derive(Debug)]
struct NodeHead<T>(Mutex<Option<NodeRef<T>>>);

impl<T: Clone> Clone for NodeHead<T> {
    fn clone(&self) -> Self {
        Self(Mutex::new(self.0.try_lock().unwrap().clone()))
    }
}

impl<T> NodeHead<T> {
    fn set(&self, value: NodeRef<T>) -> Result<(), GeneratorError> {
        let mut mutex = self.0.try_lock().unwrap();
        match *mutex {
            Some(_) => Err(GeneratorError::HeadAlreadySet),
            None => {
                mutex.replace(value);
                Ok(())
            }
        }
    }

    fn new() -> NodeHead<T> {
        Self(Mutex::new(None))
    }

    fn get(&self) -> Option<NodeRef<T>> {
        self.0.lock().unwrap().clone()
    }
}

#[derive(Debug, Clone)]
struct Node<T> {
    data: Arc<T>,
    head: NodeHead<T>,
}

#[derive(Debug)]
pub struct NodeRef<T>(Arc<Node<T>>);

impl<T> NodeRef<T> {
    pub fn new(data: T) -> NodeRef<T> {
        NodeRef(Arc::new(Node {
            data: Arc::new(data),
            head: NodeHead::new(),
        }))
    }

    pub fn push(self, mut value: NodeRef<T>) -> Result<NodeRef<T>, GeneratorError> {
        value.set_head(self)?;
        Ok(value.clone())
    }

    pub fn get_head(&self) -> Option<Self> {
        self.0.head.get()
    }

    fn set_head(&mut self, value: NodeRef<T>) -> Result<(), GeneratorError> {
        self.0.as_ref().head.set(value)
    }
}

impl<T> Clone for NodeRef<T> {
    fn clone(&self) -> Self {
        NodeRef(self.0.clone())
    }
}

struct NodeIterator<T> {
    next_node: Option<NodeRef<T>>,
}

impl<T> Iterator for NodeIterator<T> {
    type Item = NodeRef<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next_node.is_some() {
            let node = self.next_node.as_ref().unwrap().clone();
            self.next_node = node.get_head();
            return Some(node);
        } else {
            return None;
        }
    }
}

#[derive(Debug)]
pub struct Cascade<T>(Vec<Node<T>>);

impl<T> Cascade<T> {
    fn push(&mut self, value: Node<T>) {
        self.0.push(value)
    }

    fn pop(&mut self) -> Option<Node<T>> {
        self.0.pop()
    }
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use super::*;

    #[test]
    fn create_branch() {
        let branch = NodeRef::new("Avrohom".to_string())
            .push(NodeRef::new("Yitzhok".to_string()))
            .unwrap()
            .push(NodeRef::new("Yaakov".to_string()))
            .unwrap();

        assert_eq!("Yaakov".to_string(), *branch.0.data);
        assert_eq!("Yitzhok".to_string(), *branch.get_head().as_ref().unwrap().0.data);
        assert_eq!("Avrohom".to_string(), *branch.get_head().as_ref().unwrap().get_head().as_ref().unwrap().0.data);
    }

    #[test]
    fn branching_branch() {
        fn build(
        ) -> Result<(NodeRef<String>, NodeRef<String>, NodeRef<String>, NodeRef<String>), GeneratorError>
        {
            let avi = NodeRef::new("Avrohom".to_string());
            let yitz = avi.clone().push(NodeRef::new("Yitzhok".to_string()))?;
            let yank = yitz.clone().push(NodeRef::new("Yaakov".to_string()))?;
            let esau = yitz.clone().push(NodeRef::new("Esau".to_string()))?;

            Ok((avi, yitz, yank, esau))
        }
        match build() {
            Ok((avi, yitz, yank, esau)) => {
                assert!(avi.get_head().is_none());
                assert_eq!("Avrohom".to_string(), *yitz.get_head().as_ref().unwrap().0.data);
                assert_eq!("Yitzhok".to_string(), *yank.get_head().as_ref().unwrap().0.data);
                assert_eq!("Yitzhok".to_string(), *esau.get_head().as_ref().unwrap().0.data);
            }
            Err(err) => {
                assert!(false, "Error: {:?}", err)
            }
        }
    }
}
