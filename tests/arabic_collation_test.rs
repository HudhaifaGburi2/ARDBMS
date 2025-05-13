#[test]
fn test_arabic_ordering() {
    let words = vec!["أحمد", "ابراهيم", "اسماء", "آية"];
    let mut sorted = words.clone();
    sorted.sort_by(|a, b| ArabicCollator::compare(a, b));
    
    assert_eq!(sorted, vec!["آية", "اسماء", "ابراهيم", "أحمد"]);
}

#[test]
fn test_arabic_case_insensitive() {
    let upper = "القاهرة";
    let lower = "القاهرة";
    assert_eq!(ArabicCollator::compare(upper, lower), std::cmp::Ordering::Equal);
}

