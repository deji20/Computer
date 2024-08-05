#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct AdditionArgs {
    a: i32,
    b: i32,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct AdditionResponse {
    result: i32
}

pub async fn addition(args: AdditionArgs) -> AdditionResponse {
    AdditionResponse {
        result: args.a + args.b
    }
} 