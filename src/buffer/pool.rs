impl BufferPool {
    pub fn fetch_arabic_page(&mut self, page_id: u64, dm: &mut DiskManager) -> Result<&mut Page, StorageError> {
        if let Some(page) = self.pages.iter_mut().find(|p| p.page_id == page_id) {
            return Ok(page);
        }
        
        if self.pages.len() >= self.capacity {
            // Implement clock replacement algorithm
            let victim = self.clock_hand;
            self.clock_hand = (self.clock_hand + 1) % self.capacity;
            
            if self.pages[victim].is_dirty {
                dm.write_page(&self.pages[victim])?;
            }
        }
        
        let mut new_page = Page::new(page_id);
        dm.read_page(page_id, &mut new_page)?;
        self.pages.push(new_page);
        
        Ok(self.pages.last_mut().unwrap())
    }
}
