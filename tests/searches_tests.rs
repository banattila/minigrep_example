use new_minigrep::searches::*;

#[test]
fn sensitive_test(){
    let query = "Bebe";
    let content = "\
Bebe egy nagyon aranyos fiú.
Mindig is az volt";
    assert_eq!(vec!["Bebe egy nagyon aranyos fiú."], sensitive_search(query, content));
}

#[test]
fn insensitive_test(){
    let query = "NaGYoN";
    let content = "\
Bebe egy nagyon aranyos fiú.
Mindig is az volt";
    assert_eq!(vec!["Bebe egy nagyon aranyos fiú."], insensitive_search(query, content));
}