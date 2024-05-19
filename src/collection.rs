use ureq::serde::Deserialize;
use ureq::serde_json::Value;

use crate::cascade::Cascade;

#[derive(Deserialize)]
pub struct Collection {
    data: Value,
    pages: Vec<Value>,
}

impl From<Collection> for Vec<Cascade<Value>> {
    fn from(value: Collection) -> Self {
        Cascade::new()
            .push(value.data)
            .branch(value.pages)
            .into_iter()
            .collect()
    }
}

#[cfg(test)]
mod tests {

    use ureq::serde_json::{self, Map};

    use crate::cascade::Cascade;

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

        let mut result: Vec<Value> = Into::<Vec<Cascade<Value>>>::into(collection)
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
            .collect();

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

        dbg!(result);
    }
}
