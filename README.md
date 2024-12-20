# String Simple
This utility library contains a collection of string functions that I use in my other projects. 

# How to use

### 1. Add Dependency
```toml
[dependencies]
string-simple = "0.1.0"
```

### 2. Use Crate
```rust
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
 - [ ] Add more data to benchmark functions for more accurate results.
 - [x] 'string builder' struct.
   - [x] Implementation
   - [x] Test cases
   - [x] Documentation
   - [x] Benchmark
   - [ ] SIMD?
 - [x] 'replace all' string function.
   - [x] Implementation
   - [x] Test cases
   - [x] Documentation
   - [x] Benchmark
   - [ ] SIMD?
 - [x] 'find first' string function.
   - [x] Implementation 
   - [x] Test Cases
   - [x] Documentation
   - [x] Benchmark
   - [x] SIMD
 - [x] 'append' string function.
   - [x] Implementation
   - [x] Test cases
   - [x] Documentation
   - [x] Benchmark
 - [x] 'find all' string function.
   - [x] Implementation
   - [x] Test cases
   - [x] Documentation
   - [x] Benchmark
   - [x] SIMD
 - [x] 'character count' string function
   - [x] Implementation
   - [x] Test cases
   - [x] Documentation
   - [x] Benchmark
   - [x] SIMD
 - [x] 'all possible substring from chars' string function
   - [x] Implementation
   - [x] Test cases
   - [x] Documentation
   - [x] Benchmark
   - [ ] SIMD?