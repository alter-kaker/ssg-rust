use ureq::serde::Deserialize;
use ureq::serde_json::{Map, Value};

use crate::cascade::Cascade;

#[derive(Deserialize, Debug)]
pub struct Collection {
    data: Value,
    pages: Vec<Value>,
}

impl Collection {
    pub fn cascade(self) -> Vec<Value> {
        Cascade::new()
            .push(self.data)
            .branch(self.pages)
            .into_iter()
            .map(|cascade| {
                cascade
                    .into_iter()
                    .fold(Value::Object(Map::new()), |mut acc, value| {
                        acc.as_object_mut()
                            .unwrap()
                            .append(&mut value.as_ref().clone().as_object_mut().unwrap());
                        acc
                    })
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {

    use ureq::serde_json;

    use super::*;

    #[test]
    fn map_collection_to_cascade() {
        let collection = Collection {
            data: serde_json::from_str::<Value>("{\"category\": \"brothers\"}").unwrap(),
            pages: vec![
                serde_json::from_str::<Value>("{\"name\": \"Binyomin\"}").unwrap(),
                serde_json::from_str::<Value>("{\"name\": \"Yosef\"}").unwrap(),
                serde_json::from_str::<Value>("{\"name\": \"Zevulun\"}").unwrap(),
                serde_json::from_str::<Value>("{\"name\": \"Yissoschor\"}").unwrap(),
                serde_json::from_str::<Value>("{\"name\": \"Asher\"}").unwrap(),
                serde_json::from_str::<Value>("{\"name\": \"Gad\"}").unwrap(),
                serde_json::from_str::<Value>("{\"name\": \"Naftuli\"}").unwrap(),
                serde_json::from_str::<Value>("{\"name\": \"Dan\"}").unwrap(),
                serde_json::from_str::<Value>("{\"name\": \"Yehuda\"}").unwrap(),
                serde_json::from_str::<Value>("{\"name\": \"Levi\"}").unwrap(),
                serde_json::from_str::<Value>("{\"name\": \"Shimon\"}").unwrap(),
                serde_json::from_str::<Value>("{\"name\": \"Reuven\"}").unwrap(),
            ],
        };

        let mut result = collection.cascade();

        assert_eq!(
            serde_json::from_str::<Value>("{\"category\": \"brothers\", \"name\": \"Reuven\"}")
                .ok(),
            result.pop()
        );
        assert_eq!(
            serde_json::from_str::<Value>("{\"category\": \"brothers\", \"name\": \"Shimon\"}")
                .ok(),
            result.pop()
        );
        assert_eq!(
            serde_json::from_str::<Value>("{\"category\": \"brothers\", \"name\": \"Levi\"}").ok(),
            result.pop()
        );
        assert_eq!(
            serde_json::from_str::<Value>("{\"category\": \"brothers\", \"name\": \"Yehuda\"}")
                .ok(),
            result.pop()
        );
        assert_eq!(
            serde_json::from_str::<Value>("{\"category\": \"brothers\", \"name\": \"Dan\"}").ok(),
            result.pop()
        );
        assert_eq!(
            serde_json::from_str::<Value>("{\"category\": \"brothers\", \"name\": \"Naftuli\"}")
                .ok(),
            result.pop()
        );
        assert_eq!(
            serde_json::from_str::<Value>("{\"category\": \"brothers\", \"name\": \"Gad\"}").ok(),
            result.pop()
        );
        assert_eq!(
            serde_json::from_str::<Value>("{\"category\": \"brothers\", \"name\": \"Asher\"}")
                .ok(),
            result.pop()
        );
        assert_eq!(
            serde_json::from_str::<Value>(
                "{\"category\": \"brothers\", \"name\": \"Yissoschor\"}"
            )
            .ok(),
            result.pop()
        );
        assert_eq!(
            serde_json::from_str::<Value>("{\"category\": \"brothers\", \"name\": \"Zevulun\"}")
                .ok(),
            result.pop()
        );
        assert_eq!(
            serde_json::from_str::<Value>("{\"category\": \"brothers\", \"name\": \"Yosef\"}")
                .ok(),
            result.pop()
        );
        assert_eq!(
            serde_json::from_str::<Value>("{\"category\": \"brothers\", \"name\": \"Binyomin\"}")
                .ok(),
            result.pop()
        );
    }

    #[test]
    fn specificity_override() {
        let collection = Collection {
            data: serde_json::from_str::<Value>("{\"category\": \"brothers\", \"garment\": \"plain\"}").unwrap(),
            pages: vec![
                serde_json::from_str::<Value>("{\"name\": \"Binyomin\"}").unwrap(),
                serde_json::from_str::<Value>("{\"name\": \"Yosef\", \"garment\": \"colorful\"}").unwrap(),
            ],
        };

        let mut result = collection.cascade();

        assert_eq!(
            serde_json::from_str::<Value>("{\"category\": \"brothers\", \"name\": \"Yosef\", \"garment\": \"colorful\"}")
                .ok(),
            result.pop()
        );
        assert_eq!(
            serde_json::from_str::<Value>("{\"category\": \"brothers\", \"name\": \"Binyomin\", \"garment\": \"plain\"}")
                .ok(),
            result.pop()
        );
    }
}
