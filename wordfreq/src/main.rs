/**!
 * wordfreq
 *
 * Reads a sequence of string from the standard input and
 * writes count on each word in the input
 *
 * INPUT
 *
 * The input format is a sequence of words represented as
 * strings, written in ASCII (one string can contain any number
 * of words(from 0 to many)):
 *
 *     hello, world
 *     bye, world.
 *
 * The input terminates with either EOF or a line 999.
 *
 * OUTPUT
 *
 * The program counts the frequency of certain word.
 * and output in descend order.
 * It prints the results in this format:
 *
 *    hello 2
 *    world 1
 *    bye 1
 *
 * ASSUMPTIONS
 *
 *  - All kinds of non-alphabetical symbols will not be considered
 *  as part of a word.
 *
 *  - All kinds of non-alphabetical symbols will be considered split
 *  between 2 words.
 *  e.g "this_string" will be considered as word "this" and word "string"
 *
 *  - The terminator is a line of text "999", not a line of text that
 *    when interpreted is merely the number 999.0.
 *
 *  - If there are no input to read then there is no count
 *    to print
 */

use std::io::{BufRead,BufReader,Read,stdin};

type CountTable = std::collections::HashMap<String, usize>;

fn main(){

    let res = words_to_count(string_to_words(stdin()));
    for c in res {
        println!("{}  {}", c.word, c.count);
    }
}

struct Entry {
    word: String,
    count: usize,
}

impl Entry {
    fn new(s: String, c: usize) -> Self {
        Entry {
            word: s,
            count: c,
        }
    }
}

fn increment_word(map: &mut CountTable, word: String) {
    *map.entry(word).or_insert(0) += 1;
}

#[cfg(test)]
mod increment_word_tests {
    use super::{increment_word, CountTable};

    #[test]
    fn inserts_if_empty() {
        let mut h = CountTable::new();
        increment_word(&mut h, "one".to_owned());

        assert_eq!(Some(&1), h.get("one"));
        assert_eq!(1, h.len());
    }

    #[test]
    fn increments_if_present() {
        let mut under_test = fixture();
        let mut expected   = fixture();

        increment_word(&mut under_test, "three".to_owned());
        expected.insert("three".to_owned(), 4);

        assert_eq!(expected, under_test);
    }

    #[test]
    fn insert_if_absent() {
        let mut under_test = fixture();
        let mut expected   = fixture();

        increment_word(&mut under_test, "one".to_owned());
        expected.insert("one".to_owned(), 1);

        assert_eq!(expected, under_test);
    }

    fn fixture() -> CountTable {
        let mut h = CountTable::new();
        h.insert("two".to_owned(), 2);
        h.insert("three".to_owned(), 3);

        assert_eq!(None, h.get("one"));
        assert_eq!(Some(&2), h.get("two"));
        assert_eq!(Some(&3), h.get("three"));
        assert_eq!(2, h.len());

        h
    }
}

fn string_to_words<R: Read>(reader: R) -> Vec<String> {
    let mut vec_of_content: Vec<String> = Vec::<String>::new();
    let mut begin = false;
    let mut char_buffer: Vec<char> = Vec::<char>::new();;

    let mut lines = BufReader::new(reader).lines();
    while let Some(Ok(content)) = lines.next() {
        if content == "999" {break}
        for c in content.chars(){
            if c.is_alphabetic() {
                char_buffer.push(c);
                begin = true;
            } else {
                if begin {
                    vec_of_content.push(char_buffer.iter().cloned().collect());
                    char_buffer.clear();
                    begin = false;
                }
            }
        }
        if !char_buffer.is_empty() && begin {
            vec_of_content.push(char_buffer.iter().cloned().collect());
        }
        char_buffer.clear();
        begin = false;
    }

    vec_of_content
}


#[cfg(test)]
mod string_to_words_test {
    use super::{string_to_words};
    use std::io::{Read, Result};

        #[test]
        fn text_empty() {
            assert_read(Vec::<String>::new(), "");
        }

        #[test]
        fn text_something() {
            let mut some_vec = Vec::<String>::new();
            some_vec.push("This".to_string());
            some_vec.push("is".to_string());
            some_vec.push("a".to_string());
            some_vec.push("string".to_string());
            assert_read(some_vec, "This is a string");
        }

        #[test]
        fn text_weird() {
            let mut weird_vec = Vec::<String>::new();
            weird_vec.push("There".to_string());
            weird_vec.push("is".to_string());
            weird_vec.push("something".to_string());
            weird_vec.push("weird".to_string());
            weird_vec.push("here".to_string());
            assert_read(weird_vec, "32There38 is* !something∆∑ weird:)here.");
        }

    fn assert_read(expected: Vec<String>, input: &str) {
        let mock_read = StringReader::new(input.to_owned());
        let contents = string_to_words(mock_read);
        assert_eq!(expected.to_owned(), contents);
    }

    struct StringReader {
        contents: Vec<u8>,
        position: usize,
    }

    impl StringReader {
        fn new(s: String) -> Self {
            StringReader {
                contents: s.into_bytes(),
                position: 0,
            }
        }
    }

    impl Read for StringReader {
        fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
            let mut count = 0;

            while self.position < self.contents.len() && count < buf.len() {
                buf[count] = self.contents[self.position];
                count += 1;
                self.position += 1;
            }

            return Ok(count);
        }
    }
}

fn words_to_count(words: Vec<String>) -> Vec<Entry> {
    let mut hash = CountTable::new();
    for c in words {
        increment_word(&mut hash, c);
    }
    let mut summary: Vec<Entry> = Vec::new();
    for (key, val) in hash.iter() {
        summary.push(Entry::new(key.to_owned(), val.to_owned()));
    }
    summary.sort_by(|a, b| a.count.cmp(&b.count).reverse());
    summary
}

#[cfg(test)]
mod words_to_count_tests {
    use super::{words_to_count, Entry};

    #[test]
    fn count_empty() {
        assert_vec(Vec::<String>::new(), Vec::<Entry>::new());
    }

    #[test]
    fn count_hello() {
        let mut test_string = Vec::<String>::new();
        test_string.push("bye".to_string());
        test_string.push("bye".to_string());
        test_string.push("world".to_string());
        let mut test_vec = Vec::<Entry>::new();
        test_vec.push(Entry::new("bye".to_string(), 2));
        test_vec.push(Entry::new("world".to_string(), 1));
        assert_vec(test_string, test_vec);
    }

    #[test]
    fn count_hmore() {
        let mut test_string = Vec::<String>::new();
        test_string.push("one".to_string());
        test_string.push("two".to_string());
        test_string.push("three".to_string());
        test_string.push("four".to_string());
        test_string.push("one".to_string());
        test_string.push("two".to_string());
        test_string.push("three".to_string());
        test_string.push("one".to_string());
        test_string.push("two".to_string());
        test_string.push("one".to_string());
        let mut test_vec = Vec::<Entry>::new();
        test_vec.push(Entry::new("one".to_string(), 4));
        test_vec.push(Entry::new("two".to_string(), 3));
        test_vec.push(Entry::new("three".to_string(), 2));
        test_vec.push(Entry::new("four".to_string(), 1));
        assert_vec(test_string, test_vec);
    }

    fn assert_vec(expected: Vec<String>, input: Vec<Entry>) {
        let result = words_to_count(expected);
        assert_eq!(result.len(), input.len());
        for x in 0..result.len() {
            assert_eq!(result[x].word, input[x].word);
            assert_eq!(result[x].count, input[x].count);
        }
    }
}
