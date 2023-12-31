// MACRO CAPTURES

/* expr
    matches to a valid rust expression
    "hello".to_string(), vec![1, 2, 3], 1 + 2, 1
*/

/* stmt
    matches to a rust statement
    let x = 5, x.push(1), return Some(x)
*/

/* ident
    matches to a rust identifier
    variable name, function name, module name
*/

/* ty
    matches to a rust type
    i32, Vec<String>, Option<T>
*/

/* path
    matches to a rust path
    std::collections::HashMap
*/

// REPITITION SPECIFIER

// * - match zero or more repititions
// + - match one or more repititions
// ? - Match zero or one repetition

#[cfg(test)]
mod tests {
  use super::*;  
    
  #[test]
  fn tests_declarative_macro() {
    
    println!("Hello 1");
    dbg!("Hello 2");
    let x: Vec<i32> = vec!(1,2,3);
    let formatted: String = format!("Hello 3 with vec {:?}", x);
    dbg!(formatted);

  }
}