### Scalars:

1. Integers:
   - u8, u16, u32, u64
   - i8, i16, i32, i64
2. Floating Point
   - f32 , f64
3. Boolean
   - bool
   - two fixed values: `true` , `false`
4. Character
   - encloses the Unicode scaler value in single quotes: eg: 'a'
   - can represent all the characters in UTF-8, even special characters: eg: '😁'

### Compound:

1. Arrays: List of same, fixed number of elements.
   - Comma seperated elements enclosed by square brackets.
   - Number of elements are fixed
   - syntax: [dataType,number_of_elements]
   - Example: let a:[i32,6] = [1,2,3,4,5,6];
   - Access: arrayName[Index]; // a[0] gives 1
   - length: `.len()`

2. Tuple: List of different, fixed number of elements
   - Comma seperated elements enclosed by paranthesis.
   - syntax: (dataTypes,dataTypes,...)
   - Example: let a = (1,1.2,10);
   - Access: let (x,y,z) = a; , each item can be access with dot notation with their index value eg: a.o (this gives 1), a.3 (gives 10)

3. Strings
   - collection of character enclosed in double quotes
   - It has some additional properties to change allocaton details, such as:
      - `b` prefix: b"Hello World!" - Returns collection of ASCII characters
      - `r#` prefix: r#"I am raw string" - Allows to skip the escaping characters(such as \n, \t etc). 
   - Syntax: let mut variableName = String::new();
   - Char to String: charVariable.to_string();