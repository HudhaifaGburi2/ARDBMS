#[test]
fn test_arabic_buffer_pool() {
    let path = Path::new("buffer_test.data");
    let mut dm = DiskManager::new(path).unwrap();
    let mut pool = BufferPool::new(2);

    // Write Arabic data to page 1
    let mut page1 = pool.fetch_arabic_page(1, &mut dm).unwrap();
    page1.write_arabic_string(4, "اسم الجدول").unwrap();
    
    // Write Arabic data to page 2
    let mut page2 = pool.fetch_arabic_page(2, &mut dm).unwrap();
    page2.write_arabic_string(4, "وصف الجدول").unwrap();
    
    // Force eviction
    let _ = pool.fetch_arabic_page(3, &mut dm).unwrap();
    
    // Verify data persistence
    let page1_reloaded = pool.fetch_arabic_page(1, &mut dm).unwrap();
    assert_eq!(page1_reloaded.read_arabic_string(4).unwrap(), "اسم الجدول");
    
    std::fs::remove_file(path).unwrap();
}
