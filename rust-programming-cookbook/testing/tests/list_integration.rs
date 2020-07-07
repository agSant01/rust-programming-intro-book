use testing::List;

#[test]
fn test_list_insert_10k_items() {
    let mut list = List::new_empty();
    for _ in 0..5000 {
        list.append(100);
    }
    assert_eq!(list.length, 5000);
}
