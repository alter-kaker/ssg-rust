use std::sync::{Arc, Mutex};

use crate::error::GeneratorError;

#[derive(Debug)]
struct NodeHead<T>(Mutex<Option<NodeRef<T>>>);

impl<T: Clone> Clone for NodeHead<T> {
    fn clone(&self) -> Self {
        Self(Mutex::new(self.0.lock().unwrap().clone())) // todo: don't unwrap
    }
}

impl<T> NodeHead<T> {
    fn set(&self, value: NodeRef<T>) -> Result<(), GeneratorError> {
        let mut mutex = self.0.try_lock()?; // todo: don't unwrap
        match *mutex {
            Some(_) => Err(GeneratorError::HeadAlreadySet),
            None => {
                mutex.replace(value);
                Ok(())
            }
        }
    }

    fn empty() -> Self {
        Self(Mutex::new(None))
    }

    fn new(head: NodeRef<T>) -> Self {
        Self(Mutex::new(Some(head)))
    }

    fn get(&self) -> Option<NodeRef<T>> {
        self.0.lock().unwrap().clone()
    }
}

#[derive(Debug, Clone)]
struct Node<T> {
    value: Arc<T>,
    head: NodeHead<T>,
}

#[derive(Debug)]
pub struct NodeRef<T>(Arc<Node<T>>);

impl<T> NodeRef<T> {
    pub fn new(value: T) -> Self {
        Self(Arc::new(Node {
            value: Arc::new(value),
            head: NodeHead::empty(),
        }))
    }

    pub fn push(self, value: T) -> Result<Self, GeneratorError> {
        Ok(Self(Arc::new(Node {
            value: Arc::new(value),
            head: NodeHead::new(self),
        })))
    }

    pub fn branch(&self, value: T) -> Result<Self, GeneratorError> {
        self.clone().push(value)
    }

    pub fn into_iter(self) -> NodeIterator<T> {
        NodeIterator {
            next_node: Some(self),
        }
    }

    pub fn data(self) -> Arc<T> {
        self.0.value.clone()
    }

    fn get_head(&self) -> Option<Self> {
        self.0.head.get()
    }
}

impl<T> Clone for NodeRef<T> {
    fn clone(&self) -> Self {
        NodeRef(self.0.clone())
    }
}

pub struct NodeIterator<T> {
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

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn create_branch() {
        let branch = NodeRef::new("Avrohom".to_string())
            .push("Yitzhok".to_string())
            .unwrap()
            .push("Yaakov".to_string())
            .unwrap();

        assert_eq!("Yaakov".to_string(), branch.clone().data().as_ref().clone());
        assert_eq!(
            "Yitzhok".to_string(),
            *branch.get_head().unwrap().clone().data()
        );
        assert_eq!(
            "Avrohom".to_string(),
            *branch.get_head().unwrap().get_head().unwrap().data()
        );
    }

    #[test]
    fn iteration() {
        let result: Result<Vec<Arc<String>>, GeneratorError> = (|| {
            Ok(NodeRef::new("Avrohom".to_string())
                .push("Yitzhok".to_string())?
                .push("Yakov".to_string())?
                .into_iter()
                .map(NodeRef::data)
                .collect())
        })();

        match result {
            Ok(names) => {
                dbg!(names);
            }
            Err(err) => assert!(false, "{:?}", err),
        }
    }

    #[test]
    fn branching_branch() {
        let yitzhok = NodeRef::new("Avrohom".to_string())
            .push("Yitzhok".to_string())
            .unwrap();

        let result: Result<(Vec<Arc<String>>, Vec<Arc<String>>), GeneratorError> = (|| {
            Ok((
                yitzhok
                    .branch("Eisov".to_string())?
                    .into_iter()
                    .map(NodeRef::data)
                    .collect(),
                yitzhok
                    .branch("Yakov".to_string())?
                    .into_iter()
                    .map(NodeRef::data)
                    .collect(),
            ))
        })();

        match result {
            Ok((eisov, yakov)) => {
                println!("Eisov: {:?}\nYitzhok: {:?}", eisov, yakov);
            }
            Err(err) => assert!(false, "{:?}", err),
        }
    }
}
