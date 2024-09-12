use string::kmp::knuth_morris_pratt;

fn main() {
    let index = knuth_morris_pratt("ABC ABCDAB ABCDABCDABDE".to_string(), "ABCDABD".to_string());
    assert_eq!(index, vec![15]);
}
