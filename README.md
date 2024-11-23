# String Simple
This utility library contains a collection of string functions that I use in my other projects. 

# How to use

### 1. Add Dependency
```toml
[dependencies]
string-simple = "0.1.0"
```

### 2. Use Crate
```doctestinjectablerust

 use string_simple::builder::StringBuilder;
 const LOOP_COUNT: u8 = 10;

 fn main() {
     let mut new_builder = StringBuilder::new();
     let mut counter = 0;

     while counter < LOOP_COUNT {
         if counter % 2 == 0 {
             new_builder.append("even");
         } else {
             new_builder.append("odd");
         }
         if counter + 1 != LOOP_COUNT {
             new_builder.append(" ");
         }
         counter += 1;
     }
     // result = "even odd even odd..."
     let result = new_builder.build();
 }
```

# Task Checklist

 - [x] 'string builder' struct.
   - [x] Implementation
   - [x] Test cases
   - [x] Documentation
   - [x] Benchmark
 - [ ] 'replace all' string function.
   - [x] Implementation
   - [x] Test cases
   - [x] Documentation
   - [ ] Benchmark
 - [x] 'find first' string function.
   - [x] Implementation 
   - [x] Test Cases
   - [x] Documentation
   - [x] Benchmark
 - [ ] 'append' string function.
   - [x] Implementation
   - [x] Test cases
   - [x] Documentation
   - [x] Benchmark
 - [ ] 'find all' string function.
   - [x] Implementation
   - [x] Test cases
   - [x] Documentation
   - [ ] Benchmark