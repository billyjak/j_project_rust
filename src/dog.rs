// Rust
struct Animal;
struct Dog {
 parent: Animal,
}
impl Animal {
 fn eat(&self) {
   println!(“eating”);
 }
}
impl Dog {
 fn bark(&self) {
   println!(“baf!”);
 }
 fn eat(&self) {
   self.parent.eat();
 }
}
fn main() {
 let d = Dog { parent: Animal };
 d.eat();
 d.bark();
}