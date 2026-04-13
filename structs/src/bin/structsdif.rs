use std::collections::btree_map::Keys;

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
// We can also define empty structs
struct emptyStruct;
fn main() {
    let block = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}
