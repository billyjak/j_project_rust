
pub struct Text{}

impl Text {
    pub fn example (a: String,b: String) -> String {
        format!("Input the {} for {}", a, b)
    }
}