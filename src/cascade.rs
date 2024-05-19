use std::{
    collections::{vec_deque::IntoIter, VecDeque},
    sync::Arc,
};

#[derive(Debug)]
pub struct Cascade<T>(VecDeque<Arc<T>>);

impl<T> Cascade<T> {
    pub fn new() -> Self {
        Self(VecDeque::new())
    }

    pub fn push(mut self, value: T) -> Self {
        self.0.push_back(Arc::new(value));
        self
    }

    pub fn branch(&self, values: Vec<T>) -> Vec<Self> {
        values
            .into_iter()
            .map(|value| Self(self.0.clone()).push(value))
            .collect()
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
        let mut result = vec![
            "Avrohom".to_string(),
            "Yitzhok".to_string(),
            "Yaakov".to_string(),
        ]
        .into_iter()
        .fold(Cascade::new(), Cascade::push)
        .into_iter()
        .map(|value| value.as_ref().clone())
        .collect::<Vec<String>>();

        assert_eq!("Yaakov".to_string(), result.pop().unwrap());
        assert_eq!("Yitzhok".to_string(), result.pop().unwrap());
        assert_eq!("Avrohom".to_string(), result.pop().unwrap());
    }

    #[test]
    fn cascade_with_branches() {
        let mut result = vec!["Avrohom".to_string(), "Yitzhok".to_string()]
            .into_iter()
            .fold(Cascade::new(), |cascade, value| cascade.push(value))
            .branch(vec!["Eisov".to_string(), "Yakov".to_string()])
            .into_iter()
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
