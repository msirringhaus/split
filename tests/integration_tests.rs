use assert_cmd::{cargo, crate_name};
use duct::cmd;

#[derive(Debug)]
struct Test {
    input: &'static str,
    cmd_args: Vec<&'static str>,
    expected_output: &'static str,
}

impl Test {
    fn new(
        input: &'static str,
        cmd_args: Vec<&'static str>,
        expected_output: &'static str,
    ) -> Self {
        Test {
            input,
            cmd_args,
            expected_output,
        }
    }
}

macro_rules! testcase {
    ( $input:expr, $cmd_args:expr, $expected_output:expr ) => {
        let bin = cargo::cargo_bin(crate_name!());
        let test = Test::new($input, $cmd_args, $expected_output);

        let stdout = cmd!("echo", test.input)
            .pipe(cmd(&bin, test.cmd_args))
            .read()
            .unwrap();
        assert_eq!(stdout, test.expected_output);
    };
}

#[test]
fn test_readme_example_strip_empty_elements() {
    testcase!("I'm................thinking", vec!["."], "I'm thinking");
}

#[test]
fn test_readme_example_multi_char_join() {
    testcase!(
        "How did that awk-command work again?",
        vec!["-c", "1,2,3,5,6", "-j", "==="],
        "How===did===that===work===again?"
    );
}

#[test]
fn test_readme_example_multi_char_split() {
    testcase!(
        "I was...like...thinking...like...how did you...like...come up with that?",
        vec!["-c", "3,4", "...like..."],
        "how did you come up with that?"
    );
}

#[test]
fn test_readme_example_strip_empty_elements_with_id() {
    testcase!(
        "I'm................thinking",
        vec!["-c", "2", "."],
        "thinking"
    );
}

#[test]
fn test_readme_example_keep_empty_elements_with_id() {
    testcase!(
        "I'm................thinking",
        vec!["-k", "-c", "17", "."],
        "thinking"
    );
}

#[test]
fn test_readme_example_multi_line_replace() {
    testcase!(
        "Part1..................Something
Part10.................Another
Part100................Thing
Part1000...............End",
        vec![".", "-j", ": ", "-c", "1,-1"],
        "Part1: Something
Part10: Another
Part100: Thing
Part1000: End"
    );
}

#[test]
fn test_readme_example_change_order_with_id() {
    testcase!("1 2 3 4 5 6", vec!["-c", "3,2,1"], "3 2 1");
}

#[test]
fn test_complement_with_unordered_ids() {
    testcase!(
        "1 2 3 4 5 6 7 8 9 10",
        vec!["--complement", "-c", "9,7,8"],
        "1 2 3 4 5 6 10"
    );
}
#[test]
fn test_complement_with_plusminus_ids() {
    testcase!(
        "1 2 3 4 5 6 7 8 9 10",
        vec!["--complement", "-c", "8,-3,8"],
        "1 2 3 4 5 6 7 9 10"
    );
}
#[test]
fn test_complement_with_double_ids() {
    testcase!(
        "1 2 3 4 5 6 7 8 9 10",
        vec!["--complement", "-c", "9,8,-3,8,5,3"],
        "1 2 4 6 7 10"
    );
}
#[test]
fn test_complement_with_single_id() {
    testcase!(
        "I'm...thinking",
        vec!["--complement", "-k", "-c", "4", "."],
        "I'm  "
    );
}
#[test]
fn test_complement_non_whitespace_unordered_ids() {
    testcase!(
        "I'm...thinking",
        vec!["--complement", "-k", "-c", "3,2", "."],
        "I'm thinking"
    );
}
