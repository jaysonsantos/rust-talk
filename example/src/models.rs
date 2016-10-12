#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct User {
    id: i32,
    name: String,
    age: Option<i32>
}
