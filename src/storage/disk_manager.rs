use std::fs::{File, OpenOptions};
use std::io::{self, Seek, SeekFrom, Write, Read};
use std::path::Path;

pub struct DiskManager {
    file: File,
    next_page_id: u64,
}

impl DiskManager {
    pub fn new(path: &Path) -> io::Result<Self> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path)?;

        // Initialize or read existing file
        let file_size = file.metadata()?.len();
        let next_page_id = file_size / PAGE_SIZE as u64;

        Ok(DiskManager { file, next_page_id })
    }

    pub fn allocate_page(&mut self) -> u64 {
        let page_id = self.next_page_id;
        self.next_page_id += 1;
        page_id
    }

    pub fn read_page(&mut self, page_id: u64, page: &mut Page) -> io::Result<()> {
        let offset = page_id * PAGE_SIZE as u64;
        self.file.seek(SeekFrom::Start(offset))?;
        self.file.read_exact(&mut page.data)?;
        Ok(())
    }

    pub fn write_page(&mut self, page: &Page) -> io::Result<()> {
        let offset = page.page_id * PAGE_SIZE as u64;
        self.file.seek(SeekFrom::Start(offset))?;
        self.file.write_all(&page.data)?;
        Ok(())
    }
}
