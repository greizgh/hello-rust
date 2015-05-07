use std::cmp;

pub fn levenshtein(word1: &str, word2: &str) -> i32 {
    //! Compute the levenshtein distance between two words
    let m = word1.len();
    let n = word2.len();

    let mut d: Vec<Vec<i32>> = vec![vec![0; n+1]; m+1];

    for i in 0..m+1 { d[i][0] = i as i32; }
    for j in 0..n+1 { d[0][j] = j as i32; }

    for i in 1..m+1 {
        for j in 1..n+1 {

            let mut cost;

            if word1.chars().nth(i-1).unwrap() == word2.chars().nth(j-1).unwrap() {
                cost = 0;
            } else {
                cost = 1;
            }

            d[i][j] = cmp::min(cmp::min(d[i-1][j]+1, d[i][j-1]+1), d[i-1][j-1]+cost);
        }
    }

    d[m][n]
}

#[test]
fn test_levenshtein() {
    assert_eq!(levenshtein("aaaa", "aaaa"), 0);
    assert_eq!(levenshtein("aaaa", "aaab"), 1);
    assert_eq!(levenshtein("aaaa", "a"), 3);
    assert_eq!(levenshtein("a", "aaaa"), 3);
}
