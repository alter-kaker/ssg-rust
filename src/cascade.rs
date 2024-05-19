use std::{
    collections::{vec_deque::IntoIter, VecDeque},
    sync::Arc,
};

use crate::error::GeneratorError;

#[derive(Debug)]
pub struct Cascade<T>(VecDeque<Arc<T>>);

impl<T> Cascade<T> {
    pub fn new() -> Self {
        Self(VecDeque::new())
    }

    pub fn push(&mut self, value: T) {
        self.0.push_back(Arc::new(value))
    }

    pub fn branch(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T> IntoIterator for Cascade<T> {
    type Item = Arc<T>;

    type IntoIter = IntoIter<Arc<T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn iteration() {
        let mut cascade = Cascade::new();
        vec![
            "Avrohom".to_string(),
            "Yitzhok".to_string(),
            "Yaakov".to_string(),
        ]
        .into_iter()
        .for_each(|value| cascade.push(value));

        let mut result = cascade
            .into_iter()
            .map(|value| value.as_ref().clone())
            .collect::<Vec<String>>();

        assert_eq!("Yaakov".to_string(), result.pop().unwrap());
        assert_eq!("Avrohom".to_string(), result.pop().unwrap());
        assert_eq!("Yitzhok".to_string(), result.pop().unwrap());
    }

    #[test]
    fn cascade_with_branches() {
        let mut cascade = Cascade::new();

        vec!["Avrohom".to_string(), "Yitzhok".to_string()]
            .into_iter()
            .for_each(|value| cascade.push(value));

        let mut result = vec!["Eisov".to_string(), "Yakov".to_string()]
            .into_iter()
            .map(|value| {
                let mut branch = cascade.branch();
                branch.push(value);
                branch
            })
            .map(|branch| {
                branch.into_iter().fold(String::new(), |acc, value| {
                    if acc.is_empty() {
                        value.as_ref().clone()
                    } else {
                        format!("{}, {}", acc, value)
                    }
                })
            })
            .collect::<Vec<String>>();

        dbg!(&result);
        assert_eq!("Avrohom, Yitzhok, Yakov".to_string(), result.pop().unwrap());
        assert_eq!("Avrohom, Yitzhok, Eisov".to_string(), result.pop().unwrap());
    }
}
