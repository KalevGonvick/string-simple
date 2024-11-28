pub mod builder {
    use std::ops::AddAssign;

    /// # Description
    /// A simple struct to wrap the process of building strings.
    /// You can append anything to the `StringBuilder` as long as the type provided implements the `ToString` trait.
    ///
    /// # Examples
    ///
    /// Simple example:
    /// ```
    /// use string_simple::builder::StringBuilder;
    ///
    /// let mut new_builder = StringBuilder::new();
    ///
    /// new_builder.append("This string has ")
    ///     .append(30)
    ///     .append(" characters.");
    ///
    /// // result = "This string has 30 characters"
    /// let result = new_builder.build();
    /// ```
    ///
    /// Simple loop example:
    /// ```
    /// use string_simple::builder::StringBuilder;
    ///
    /// let mut new_builder = StringBuilder::new();
    /// let mut counter = 0;
    /// const LOOP_COUNT: u8 = 10;
    /// while counter < LOOP_COUNT {
    ///     if counter % 2 == 0 {
    ///         new_builder.append("even");
    ///     } else {
    ///         new_builder.append("odd");
    ///     }
    ///     if counter + 1 != LOOP_COUNT {
    ///         new_builder.append(" ");
    ///     }
    ///     counter += 1;
    /// }
    /// // result = "even odd even odd..."
    /// let result = new_builder.build();
    /// ```
    pub struct StringBuilder {
        full_string: String,
        current_len: usize
    }

    impl StringBuilder {
        pub fn new() -> Self {
            StringBuilder {
                full_string: String::with_capacity(0),
                current_len: 0
            }
        }

        pub fn append<T>(
            &mut self,
            t: T
        ) -> &mut Self
            where T: ToString
        {
            let str = t.to_string();
            let mut len = self.current_len;
            len.add_assign(AsRef::<str>::as_ref(&str).len());
            let mut buf = String::with_capacity(len);
            buf.push_str(self.full_string.as_ref());
            buf.push_str(str.as_ref());
            self.current_len = len;
            self.full_string = buf;
            self
        }

        pub fn build(&self) -> String {
            self.full_string.clone()
        }
    }
}


pub mod modify {
    use std::ops::AddAssign;

    /// # Description
    ///
    /// Adds a new string onto the base string. The provided base string will be modified.
    /// All arguments assume that all characters are UTF-8.
    ///
    /// # Arguments
    ///
    /// * `base` - The base string that will be modified.
    /// * `append` - The new string that will be added onto the base.
    ///
    /// # Examples
    ///)
    /// ```
    /// use string_simple::modify::append;
    /// let mut str1 = String::from("base string");
    /// let str2 = String::from("!");
    ///
    /// // str1 will now be: "base string!"
    /// append(&mut str1, &str2);
    /// ```
    pub fn append<S>(base: &mut String, append: &S)
        where S: ToString
    {
        let str = append.to_string();
        let mut len = base.len();
        len.add_assign(AsRef::<str>::as_ref(&str).len());
        let mut buf = String::with_capacity(len);
        buf.push_str(base.as_ref());
        buf.push_str(str.as_ref());
        *base = buf;
    }

    /// # Description
    ///
    /// Replaces all occurrences of a substring in a base string with another given term. The base string provided will be modified.
    /// All provided arguments are assumed to be valid UTF-8 chars.
    ///
    /// # Arguments
    ///
    /// * `base` -  The full base string. The base string will be modified by the function call.
    /// * `find` - The substring we are going to replace in the `base` string.
    /// * `replace` - The new string that replaces all occurrences of the `find` string.
    ///
    /// # Examples
    ///
    /// ```
    /// use string_simple::modify::replace;
    ///
    /// let mut base_string = String::from("This is my base string!");
    /// let find_string = String::from("base");
    /// let replace_string = String::from("modified");
    ///
    /// // The base string will be "This is my modified string!"
    /// replace(&mut base_string, &find_string, &replace_string);
    /// ```
    ///
    pub fn replace<S, R>(base: &mut String, find: &S, replace: &R)
        where S: ToString, R: ToString
    {
        let t = base.to_string();
        let base_str_bytes = t.as_bytes();
        let t = find.to_string();
        let sub_str_bytes = t.as_bytes();
        let t = replace.to_string();

        assert!(base_str_bytes.len() >= sub_str_bytes.len());

        let repl_len = AsRef::<str>::as_ref(&t).len();
        let mut replaced_string = String::with_capacity(0);
        let mut replaced_len = 0usize;
        let mut current_base_pos = 0usize;

        while current_base_pos < base_str_bytes.len() {
            let mut current_sub_pos = 0usize;
            let mut current_base_test = current_base_pos;

            'inner: while current_sub_pos < sub_str_bytes.len()
                && current_base_test < base_str_bytes.len() {

                match (&base_str_bytes[current_base_test] == &sub_str_bytes[current_sub_pos],
                       current_sub_pos == sub_str_bytes.len() - 1,
                        sub_str_bytes.len() < base_str_bytes.len() - current_base_pos )
                {

                    (true, true, true) => {
                        let mut l = replaced_len;
                        l.add_assign(repl_len);
                        let mut s = String::with_capacity(l);
                        s.push_str(replaced_string.as_ref());
                        s.push_str(t.as_ref());
                        replaced_len = l;
                        replaced_string = s;
                        current_base_pos = current_base_test;
                        break 'inner;
                    }

                    (true, false, true) => {
                        current_sub_pos += 1;
                        current_base_test += 1;
                    }

                    (_,_,_) => {
                        let mut l = replaced_len;
                        l.add_assign(1);
                        let mut s = String::with_capacity(l);
                        s.push_str(replaced_string.as_ref());
                        let c = base_str_bytes[current_base_pos] as char;
                        s.push(c);
                        replaced_len = l;
                        replaced_string = s;
                        break 'inner;
                    }
                }
            }
            current_base_pos += 1;
        }
        *base = replaced_string
    }
}


pub mod compare {
    use std::collections::HashMap;

    /// # Description
    ///
    /// Finds all substrings containing a set of specified characters.
    /// Returns a HashMap containing all substrings as keys and the value being the number of times the substring occurs in the base string.
    ///
    /// # Arguments
    ///
    /// * `base` - The base string being searched.
    /// * `char_group` - the specified chars being looked for in substrings.
    ///
    /// # Outputs
    ///
    /// * HashMap<String, usize> - HashMap of all strings as keys and the number of times the substring occurs as the value.
    ///
    /// # Examples
    ///
    /// ```
    /// use string_simple::compare::substring_char_group_count;
    ///
    /// let str1 = String::from("abcc");
    /// let chars = vec!['a', 'b', 'c'];
    ///
    /// // The result will look like this: {'abc': 1, 'abcc': 1}
    /// let result = substring_char_group_count(&str1, &chars);
    /// ```
    pub fn substring_char_group_count<B>(
        base: &B,
        char_group: &Vec<char>
    ) -> HashMap<String, usize>
        where B: ToString
    {
        let binding = base.to_string();
        let base_bytes = binding.as_bytes();
        let mut base_pos = 0usize;
        let mut sub_string_count: HashMap<String, usize> = HashMap::new();
        let mut sub_byte_count: HashMap<&[u8], usize> = HashMap::new();

        while base_pos < base_bytes.len() {
            let mut end_pos = base_bytes.len();
            let current = base_pos;

            while current < end_pos {
                let mut found_count_down = char_group.len();

                for char in char_group {
                    let byte = *char as u8;
                    if base_bytes[current..end_pos].contains(&byte) {
                        found_count_down -= 1
                    }
                }

                match (found_count_down == 0, sub_byte_count.get(&base_bytes[current..end_pos])) {
                    (true, Some(count)) => sub_byte_count.insert(&base_bytes[current..end_pos], count + 1usize),
                    (true, None) => sub_byte_count.insert(&base_bytes[current..end_pos], 1usize),
                    _ => None
                };
                end_pos -= 1;
            }
            base_pos += 1;
        }

        for (byte_arr, count) in sub_byte_count {
            sub_string_count.insert(unsafe { std::str::from_utf8_unchecked(byte_arr).to_string() }, count);
        }
        sub_string_count
    }

    /// # Description
    ///
    /// Gets the count of all characters fom a provided base string given a group of characters.
    ///
    /// # Arguments
    ///
    /// * `base` - The base string being searched.
    /// * `chars` - A vector of characters being searched and tallied.
    ///
    /// # Output
    ///
    /// * HashMap<char, u32> - A hashmap containing the different characters and the number of times they appear in the base string.
    ///
    /// # Examples
    ///
    /// ```
    /// use string_simple::compare::get_char_count;
    ///
    /// let str1 = String::from("abc");
    /// let chars = vec!['a', 'b', 'c'];
    ///
    /// // The result will look like this: {'a': 1, 'b': 1, 'c': 1}
    /// let result = get_char_count(&str1, &chars);
    /// ```
    pub fn get_char_count<B>(
        base: &B,
        chars: &Vec<char>
    ) -> HashMap<char, u32>
        where B: ToString
    {
        let binding = base.to_string();
        let bytes = binding.as_bytes();
        let mut char_count: HashMap<char, u32> = HashMap::new();
        for c_char in chars {
            char_count.insert(*c_char, 0);
        }
        let mut pos = 0usize;
        while pos < bytes.len() {
            if char_count.contains_key(&(bytes[pos] as char)) {
                char_count.insert(
                    bytes[pos] as char,
                    char_count.get(&(bytes[pos] as char)).unwrap() + 1
                );
            }
            pos += 1;
        }
        char_count
    }

    /// # Description
    ///
    /// Finds all sub-string occurrences and ranges the sub-strings occur at.
    /// All provided arguments are assumed to be valid UTF-8 chars.
    ///
    /// # Arguments
    /// * `base` - The base string we are searching.
    /// * `find` - The sub-string we are trying to find all occurrences of.
    ///
    /// # Output
    /// * `Vec<(usize, usize)>` - A vector of tuples containing 2 usize numbers. The first number is the starting position of the occurrence, the second number is where the occurrence ends. The vector will be empty if no occurrences were found.
    ///
    /// # Examples
    ///
    /// ```
    /// use string_simple::compare::find_all_exact;
    ///
    /// let base_string = String::from("This is my test string! test test!");
    /// let find_string = String::from("test");
    ///
    /// // output in this case will look like this: [(11, 15), (24, 28), (29, 33)]
    /// let result = find_all_exact(&base_string, &find_string);
    /// ```
    pub fn find_all_exact<B, S>(
        base: &B,
        find: &S
    ) -> Vec<(usize, usize)>
        where B: ToString, S: ToString
    {
        let t = base.to_string();
        let base_string_bytes = t.as_bytes();
        let t = find.to_string();
        let find_string_bytes = t.as_bytes();
        assert!(base_string_bytes.len() >= find_string_bytes.len());
        let mut matches: Vec<(usize, usize)> = vec![];

        let mut current_base_pos = 0usize;
        while current_base_pos < base_string_bytes.len() {
            let mut current_find_pos = 0usize;
            let mut current_base_test = current_base_pos;
            'inner: while current_find_pos < find_string_bytes.len()
                && current_base_test < base_string_bytes.len() {
                match (&base_string_bytes[current_base_test] == &find_string_bytes[current_find_pos],
                       current_find_pos == find_string_bytes.len() - 1) {
                    (true, true) => {
                        matches.push((current_base_pos, current_base_test + 1));
                        break 'inner;
                    }
                    (true, false) => {
                        current_find_pos += 1;
                        current_base_test += 1;
                        continue 'inner;
                    }
                    (_, _) => break 'inner
                }
            }
            current_base_pos += 1;
        }
        matches
    }

    /// # Description
    /// Find the first occurrence of a sub-string within a base string.
    /// All arguments are assumed to be valid UTF-8 characters.
    ///
    /// # Arguments
    /// * `base` - The provided base string we are searching.
    /// * `find` - The substring we are trying to find.
    ///
    /// # Output
    /// * `Option<(usize, usize)>` - Optional tuple containing the start and end positions in the `base`  where the first `find` can be found. Returns `None` if the `find` was not found.
    ///
    /// # Examples
    ///
    /// ```
    /// use string_simple::compare::contains;
    /// let base_string = String::from("This is my test string! test test!");
    /// let find_string = String::from("test");
    ///
    /// // output in this case will look like this: Some((11, 15))
    /// let result = contains(&base_string, &find_string);
    /// ```
    pub fn contains<B, S>(
        base: &B,
        find: &S
    ) -> Option<(usize, usize)>
        where B: ToString, S: ToString
    {
        let t = base.to_string();
        let base_string_bytes = t.as_bytes();
        let t = find.to_string();
        let find_string_bytes = t.as_bytes();
        assert!(base_string_bytes.len() >= find_string_bytes.len());

        let mut current_base_pos = 0usize;
        while current_base_pos < base_string_bytes.len() {
            let mut current_find_pos = 0usize;
            let mut base_pos_test = current_base_pos;
            'inner: while current_find_pos < find_string_bytes.len()
                && base_pos_test < base_string_bytes.len() {
                if &base_string_bytes[base_pos_test] == &find_string_bytes[current_find_pos] {
                    if current_find_pos == find_string_bytes.len() - 1 {
                        return Some((current_base_pos, base_pos_test + 1));
                    }

                    current_find_pos = current_find_pos + 1;
                    base_pos_test = base_pos_test + 1;
                    continue 'inner;
                }
                break 'inner;
            }
            current_base_pos = current_base_pos + 1;
        }
        return None;
    }
}


#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::fmt::Display;
    use super::*;

    #[test]
    fn test_substring_char_group_count() {
        let str1 = String::from("aabbccba");
        let char_group = vec!['c', 'a', 'b'];
        let result = compare::substring_char_group_count(&str1, &char_group);
        let mut expected: HashMap<String, usize> = HashMap::new();
        expected.insert("cba".to_string(), 1);
        expected.insert("abbc".to_string(), 1);
        expected.insert("abbccba".to_string(), 1);
        expected.insert("bbccba".to_string(), 1);
        expected.insert("ccba".to_string(), 1);
        expected.insert("aabbccb".to_string(), 1);
        expected.insert("aabbc".to_string(), 1);
        expected.insert("abbccb".to_string(), 1);
        expected.insert("abbcc".to_string(), 1);
        expected.insert("bccba".to_string(), 1);
        expected.insert("aabbcc".to_string(), 1);
        expected.insert("aabbccba".to_string(), 1);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_char_count() {
        let str1 = String::from("abcdefgaaa b c ddd c");
        let chars = vec!['a', 'b', 'c'];
        let result = compare::get_char_count(&str1, &chars);
        let mut expected: HashMap<char, u32> = HashMap::new();
        expected.insert('c', 3);
        expected.insert('a', 4);
        expected.insert('b', 2);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_append1() {
        let mut str1 = String::from("123");
        let str2 = String::from("test");
        modify::append(&mut str1, &str2);
        assert_eq!(String::from("123test"), str1);
    }

    #[test]
    fn test_append2() {
        let mut str1 = String::from("123");
        let str2 = String::from("test");
        modify::append(&mut str1, &str2);
        assert_eq!(String::from("123test"), str1);
    }

    #[test]
    fn test_replace() {
        let mut str1 = String::from("123123123test123123123test12teest");
        let str2 = String::from("test");
        let str3 = String::from("replaced");
        modify::replace(&mut str1, &str2, &str3);
        assert_eq!("123123123replaced123123123replaced12teest", str1);
    }

    #[test]
    fn test_contains() {
        let str1 = String::from("123123123test123123123");
        let str2 = String::from("test");
        let result = compare::contains(&str1, &str2);
        assert_eq!(str2, &str1[result.unwrap().0..result.unwrap().1]);
    }

    #[test]
    fn test_find_all_exact1() {
        let str1 = String::from("123test113test444testtest");
        let str2 = String::from("test");
        let result = compare::find_all_exact(&str1, &str2);
        let expected: Vec<(usize, usize)> = vec![(3, 7), (10, 14), (17, 21), (21, 25)];
        assert_eq!(expected, result);
    }

    #[test]
    fn test_find_all_exact2() {
        let str1 = String::from("bbbbbbbbbbbbbbbbbb");
        let str2 = String::from("bbb");
        let result = compare::find_all_exact(&str1, &str2);
        let expected: Vec<(usize, usize)> = vec![(0, 3), (1, 4), (2, 5), (3, 6), (4, 7), (5, 8), (6, 9), (7, 10), (8, 11), (9, 12), (10, 13), (11, 14), (12, 15), (13, 16), (14, 17), (15, 18)];
        assert_eq!(expected, result);
    }

    struct ToStringStruct {
        a_string: String,
        a_number: i32
    }

    impl Display for ToStringStruct {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, r#"{{ "a_string": "{}", "a_number": "{}" }}"#, self.a_string, self.a_number)
        }
    }


    #[test]
    fn test_stringbuilder1() {
        let mut string_builder = builder::StringBuilder::new();
        string_builder
            .append("this")
            .append("is")
            .append("a")
            .append("test");

        assert_eq!(string_builder.build(), "thisisatest".to_string());
    }

    #[test]
    fn test_stringbuilder2() {
        let mut string_builder = builder::StringBuilder::new();
        string_builder.append("this")
            .append("is")
            .append("another")
            .append("test");
        assert_eq!(string_builder.build(), "thisisanothertest".to_string());

        string_builder
            .append("this")
            .append("is")
            .append("another")
            .append("test");
        assert_eq!(string_builder.build(), "thisisanothertestthisisanothertest".to_string());
    }

    #[test]
    fn test_stringbuilder3() {
        let mut string_builder = builder::StringBuilder::new();
        let to_string_struct = ToStringStruct {
            a_string: String::from("struct_string"),
            a_number: 4321
        };
        string_builder
            .append(1234)
            .append('c')
            .append("test")
            .append(55usize)
            .append(to_string_struct);

        assert_eq!(string_builder.build(), "1234ctest55{ \"a_string\": \"struct_string\", \"a_number\": \"4321\" }".to_string());
    }
}
