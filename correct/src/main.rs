/**!
 * correct
 *
 * 1.Train the correcor with a .txt file which contains correct
 * words.
 * 2.Reads a sequence of string from the standard input and
 * writes corrected result next to it.
 *
 * INPUT
 *
 * The input format is a sequence of words represented as
 * strings, written in ASCII (one line one word, can input
 * multiple lines (from 0 to many)):
 *
 *     hello
 *     hel
 *     wolrd
 *     wor
 *     w
 *
 * The input terminates with either EOF or a line 999.
 *
 * OUTPUT
 *
 * The program spellings or input words.
 * It first print the orginal word, and then print the corrected one.
 * If input is alreay correct, it just shift to next line.
 * If the input word can not be corrected, print -
 * It prints the results in this format:
 *
 *    hello
 *    hell hello
 *    wor word
 *    w -
 *
 * ASSUMPTIONS
 *
 *  - All kinds of non-alphabetical symbols will not be considered
 *  as part of a word.
 *
 *  - The terminator is a line of text "999", not a line of text that
 *    when interpreted is merely the number 999.0.
 *
 *  - If there are no input to read then there is no count
 *    to print
 *
 *  - The user will assign a training sample, the spelling correction is
 *    based on the sample. Thus, what's not inside the sample may not be
 *    correted.
 *
 *  - There's no promise that the correction is 100% right against what
 *    the user thinks. But a optimal possibility on current knowledge from
 *    the training samples.
 */

use std::env;
use std::fs::File;
use std::io::{BufRead,BufReader,Read,stdin};

type CountTable = std::collections::HashMap<String, usize>;
type CountSet = std::collections::HashSet<String>;

fn main() {
    let args: Vec<_> = env::args().collect();
    let trainee = generate_dictionary(readtrainer(&args[1]));
    edit(stdin(), &trainee);
}

fn readtrainer(filename: &str) -> Vec<String>{
    let mut file = match File::open(&filename) {
        Ok(file) => file,
        Err(_) => panic!("no such file"),
    };

    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)
        .ok()
        .expect("failed to read!");
        file_contents.to_lowercase();
    let lines: Vec<String> = file_contents.split(" ")
        .map(|s: &str| s.to_string())
        .collect();
    lines
}

#[cfg(test)]
mod check_reader {
    use super::{readtrainer};

    #[test]
    #[should_panic(expected = "no such file")]
    fn empty() {
        readtrainer("");
    }

    #[test]
    fn corpus_test() {
        let mut s = Vec::<String>::new();
        s.push("hello".to_owned());
        s.push("world".to_owned());
        s.push("hello".to_owned());
        s.push("word".to_owned());
        s.push("hello".to_owned());
        s.push("world".to_owned());
        assert_eq!(s, readtrainer("corpus.txt"));
    }
}



//delete one charactor
fn generate_delete(word: String, set: &mut CountSet){
    for i in 0..word.len() {
        let mut tmp = word.clone();
        tmp.remove(i);
        set.insert(tmp);
    }
}

#[cfg(test)]
mod check_delete{
    use super::{generate_delete, CountSet};

    #[test]
    fn delete_empty(){
        let mut h = CountSet::new();
        generate_delete("".to_owned(), &mut h);
        assert_eq!(true, h.is_empty());
        assert_eq!(false, h.contains("a"));
        assert_eq!(false, h.contains("hello"));
        assert_eq!(false, h.contains("word"));
    }
    #[test]
    fn delete_one(){
        let mut h = CountSet::new();
        generate_delete("a".to_owned(), &mut h);
        assert_eq!(false, h.contains("ww"));
        assert_eq!(true, h.contains(""));
        assert_eq!(false, h.contains("a"));
    }
    #[test]
    fn delete_more(){
        let mut h = CountSet::new();
        generate_delete("abc".to_owned(), &mut h);
        assert_eq!(false, h.contains("abc"));
        assert_eq!(true, h.contains("ab"));
        assert_eq!(true, h.contains("ac"));
        assert_eq!(true, h.contains("ac"));
    }
}


//swith two ajacent charactors
fn generate_switch(word: String, set: &mut CountSet){
    if word.len() > 2 {
        for i in 0..word.len()-1 {
            let mut tmp = word.clone();
            let x = tmp.remove(i);
            tmp.insert(i+1, x);
            set.insert(tmp);
        }
    }
}

#[cfg(test)]
mod check_switch{
    use super::{generate_switch, CountSet};

    #[test]
    fn switch_nothing(){
        let mut h = CountSet::new();
        generate_switch("".to_owned(), &mut h);
        assert_eq!(false, h.contains("b"));
        assert_eq!(false, h.contains("hello"));
        assert_eq!(true, h.is_empty());
    }
    #[test]
    fn switch_one(){
        let mut h = CountSet::new();
        generate_switch("a".to_owned(), &mut h);
        assert_eq!(false, h.contains("a"));
        assert_eq!(false, h.contains("what"));
        assert_eq!(true, h.is_empty());
    }
    #[test]
    fn switch_more(){
        let mut h = CountSet::new();
        generate_switch("apple".to_owned(), &mut h);
        assert_eq!(true, h.contains("apple"));
        assert_eq!(true, h.contains("paple"));
        assert_eq!(false, h.is_empty());
    }
}
//insert one charactor
fn generate_insert(word: String, set: &mut CountSet){
    let base = "abcdefghijklmnopqrstuvwxyz";
    if word.len() > 0 {
        for i in 0..word.len()+1{
            let mut chars = base.chars();
            for _j in 0..26{
                let mut tmp = word.clone();
                tmp.insert(i, chars.next().unwrap());
                set.insert(tmp);
            }
        }
    }
}
#[cfg(test)]
mod check_insert{
    use super::{generate_insert, CountSet};

    #[test]
    fn insert_something(){
        let mut h = CountSet::new();
        generate_insert("and".to_owned(), &mut h);
        assert_eq!(true, h.contains("aand"));
        assert_eq!(true, h.contains("andd"));
        assert_eq!(true, h.contains("andx"));
    }

    #[test]
    fn switch_nothing(){
        let mut h = CountSet::new();
        generate_insert("".to_owned(), &mut h);
        assert_eq!(false, h.contains("a"));
        assert_eq!(true, h.is_empty());
    }
}
//change one charactor
fn generate_change(word: String, set: &mut CountSet){
    let base = "abcdefghijklmnopqrstuvwxyz";
    for i in 0..word.len(){
        let mut chars = base.chars();
        for _j in 0..26{
            let mut tmp = word.clone();
            tmp.remove(i);
            tmp.insert(i, chars.next().unwrap());
            set.insert(tmp);
        }
    }
}
#[cfg(test)]
mod check_change{
    use super::{generate_change, CountSet};

    #[test]
    fn change_nothing(){
        let mut h = CountSet::new();
        generate_change("".to_owned(), &mut h);
        assert_eq!(false, h.contains("anything"));
        assert_eq!(true, h.is_empty());
    }

    #[test]
    fn change_something(){
        let mut h = CountSet::new();
        generate_change("ab".to_owned(), &mut h);
        assert_eq!(true, h.contains("ac"));
        assert_eq!(true, h.contains("xb"));
        assert_eq!(false, h.is_empty());
    }
}
//edit. the main function dealing with correction
fn edit<R: Read>(reader: R, table: &CountTable){
    let mut one_step_set = CountSet::new();
    let mut two_step_set = CountSet::new();
    let mut lines = BufReader::new(reader).lines();
    while let Some(Ok(content)) = lines.next() {
        if content == "999" {break}
        //finding it's a correct word. move to next line
        print!("{}  ", content.clone());
        if table.contains_key(&content){
            println!{""};
            continue;
        }
        //initializing
        let mut stop_flag1 = false;
        let mut stop_flag2 = false;
        one_step_set.clear();
        two_step_set.clear();
        //correct in 1st edit
        train(content, &mut one_step_set);
        for i in one_step_set.to_owned() {
            if table.contains_key(&i){
                println!("{}", i);
                stop_flag1 = true;
                break;
            }
        }
        if stop_flag1 {continue;}
        //correct in 2nd edit
        for i in one_step_set.to_owned() {
            train(i, &mut two_step_set);
        }
        for i in two_step_set.to_owned() {
            if table.contains_key(&i){
                println!("{}", i);
                stop_flag2 = true;
                break;
            }
        }
        if stop_flag2 {continue;}
        println!("-");
    }
}

//generate possibilities
fn train(content: String, set: &mut CountSet){
    generate_delete(content.to_owned(), set);
    generate_switch(content.to_owned(), set);
    generate_insert(content.to_owned(), set);
    generate_change(content.to_owned(), set);
}
#[cfg(test)]
mod check_trainer{
    use super::{train, CountSet};

    #[test]
    fn train_nothing(){
        let mut h = CountSet::new();
        train("".to_owned(), &mut h);
        assert_eq!(false, h.contains("anything"));
        assert_eq!(true, h.is_empty());
    }

    #[test]
    fn train_something(){
        let mut h = CountSet::new();
        train("this".to_owned(), &mut h);
        assert_eq!(true, h.contains("thi"));
        assert_eq!(true, h.contains("athis"));
        assert_eq!(true, h.contains("tihs"));
        assert_eq!(true, h.contains("ths"));
        assert_eq!(false, h.is_empty());
    }
}
//get word dictionary from input trainning sample
fn generate_dictionary (words: Vec<String>) -> CountTable{
    let mut dictionary = CountTable::new();
    for i in words{
        increment_word(&mut dictionary, i);
    }
    dictionary
}
#[cfg(test)]
mod check_dictionary{
    use super::{generate_dictionary, CountTable};

    #[test]
    fn get_nothing(){
        let mut h = generate_dictionary(Vec::<String>::new());
        assert_eq!(false, h.contains_key("anything"));
        assert_eq!(true, h.is_empty());
    }
    fn get_corpus(){
        let mut s = Vec::<String>::new();
        s.push("hello".to_owned());
        s.push("world".to_owned());
        s.push("hello".to_owned());
        s.push("world".to_owned());
        s.push("hello".to_owned());
        s.push("word".to_owned());
        let mut h = generate_dictionary(s);
        assert_eq!(true, h.contains_key("hello"));
        assert_eq!(true, h.contains_key("world"));
        assert_eq!(true, h.contains_key("word"));
        assert_eq!(Some(&3), h.get("hello"));
        assert_eq!(Some(&2), h.get("world"));
        assert_eq!(Some(&1), h.get("word"));
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
