fn main(){
let logical:bool = true;
let a_float: f64 = 1.0; // Regular annotation
let an_integer = 3i32; // Suffix annotation i32

// Default data type that is f64 and i32
let default_float = 2.0; // f64
let default_integer = 23; // i32

// Type can also be inferred from the value set to the variable
let mut inferred_type = 12;
inferred_type = 4354224554345i64; //inferred_type inferring from i64 from this line

// Initially all the variable in rust are immutable
// But it can be made mutable with `mut` keyword
let mut mutable = 12;
mutable = 32;

// Shadowjing allows the variables to get overwritten with data as well it's type
let mutable = true;
}
