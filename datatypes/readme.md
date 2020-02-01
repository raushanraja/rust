### Scalars:

1. Integers:
   - u32, u64
   - i32, i64
2. Floating Point
   - f32 , f64
3. Boolean
   - bool
4. Character
   - enclose the value in double quotes: "hello, world!"

### Compound:

1. Arrays: List of same, fixed number of elements.

   - syntax: [dataType,number_of_elements]
   - Example: let a:[i32,6] = [1,2,3,4,5,6];
   - Access: arrayName[Index]; // a[0] gives 1

2. Tuple: List of different, fixed number of elements

   - Comma seperated elements enclosed by paranthesis.
   - syntax: (dataTypes,dataTypes,...)
   - Example: let a = (1,1.2,10);
   - Access: let (x,y,z) = a;

3. Strings
   - Syntax: let mut variableName = String::new();
   - Char to String: charVariable.to_string();
