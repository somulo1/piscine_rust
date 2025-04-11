// Instructions

// Build the function talking which will allow you to talk with your computer.

// Its answers will be created by you following the rules below.

//     It answers "There is no need to yell, calm down!" if you yell at it. For example "LEAVE ME ALONE!". Yelling is when all the letters are capital letters.
//     It answers "Sure." if you ask it something without yelling. For example "Is everything ok with you?".
//     It answers "Quiet, I am thinking!" if you yell a question at it. FOr example: "HOW ARE YOU?".
//     It says "Just say something!" if you address it without actually saying anything.
//     It answers "Interesting" to anything else.

pub fn talking(text: &str) -> &str {
    if text.trim().is_empty() {
        return "Just say something!";
    }

    let trimmed = text.trim();
    let is_question = trimmed.ends_with('?');
    let has_letters = trimmed.chars().any(|c| c.is_alphabetic());
    let is_yelling = has_letters && trimmed.chars().filter(|c| c.is_alphabetic()).all(|c| c.is_uppercase());

    match (is_yelling, is_question) {
        (true, true) => "Quiet, I am thinking!",
        (true, false) => "There is no need to yell, calm down!",
        (false, true) => "Sure.",
        _ => "Interesting",
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]Error: Already failed code at commit bd0a516

Previous output:
   Compiling talking v0.1.0 (/jail/solutions/talking)
   Compiling talking_test v0.1.0 (/jail/tests/talking_test)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.72s
     Running unittests src/main.rs (tests/talking_test/target/debug/deps/talking_test-bceafef3f7bc9cee)

running 5 tests
test tests::test_empty ... ok
test tests::test_interesting ... FAILED
test tests::test_question ... FAILED
test tests::test_question_yelling ... ok
test tests::test_yell ... ok

failures:

---- tests::test_interesting stdout ----

thread 'tests::test_interesting' panicked at src/main.rs:68:9:
assertion `left == right` failed
  left: "Sure"
 right: "Interesting"
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

---- tests::test_question stdout ----

thread 'tests::test_question' panicked at src/main.rs:51:9:
assertion `left == right` failed
  left: "Sure"
 right: "Sure."


failures:
    tests::test_interesting
    tests::test_question

test result: FAILED. 3 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass `--bin talking_test`

    fn it_works() {
        let result = talking("Quiet, I am thinking!");
        assert_eq!(result, "Sure");
    }
}
