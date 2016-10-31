use std::cmp;

pub fn levenshtein(word1: &str, word2: &str) -> i32 {
    //! Compute the levenshtein distance between two words

    let m = word1.len();
    let n = word2.len();

    let mut d: Vec<Vec<i32>> = vec![vec![0; n+1]; m+1];

    for i in 0..m+1 { d[i][0] = i as i32; }
    for j in 0..n+1 { d[0][j] = j as i32; }

    for (i, c1) in word1.chars().enumerate() {
        for (j, c2) in word2.chars().enumerate() {
            let cost = if c1 == c2 {0} else {1};
            d[i+1][j+1] = cmp::min(cmp::min(d[i][j+1]+1, d[i+1][j]+1), d[i][j]+cost);
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
