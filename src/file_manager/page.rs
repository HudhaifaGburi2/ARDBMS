
use std::convert::TryInto;
use std::io::{Read, Write, Seek, SeekFrom};

const PAGE_SIZE: usize = 4096; // 4KB pages
const ARABIC_CODEPAGE_HEADER: [u8; 4] = [0xEF, 0xBB, 0xBF, 0x06]; // UTF-8 BOM + Arabic indicator

#[derive(Debug, Clone)]
pub struct Page {
    data: [u8; PAGE_SIZE],
    page_id: u64,
    is_dirty: bool,
}

impl Page {
    pub fn new(page_id: u64) -> Self {
        let mut data = [0u8; PAGE_SIZE];
        // Set Arabic encoding header
        data[..4].copy_from_slice(&ARABIC_CODEPAGE_HEADER);
        Page {
            data,
            page_id,
            is_dirty: true,
        }
    }

    // Arabic-aware string writing
    pub fn write_arabic_string(&mut self, offset: usize, text: &str) -> Result<usize, StorageError> {
        let bytes = text.as_bytes();
        let len = bytes.len();
        if offset + len + 4 > PAGE_SIZE {
            return Err(StorageError::PageOverflow);
        }
        
        // Store length prefix
        self.data[offset..offset+4].copy_from_slice(&(len as u32).to_be_bytes());
        self.data[offset+4..offset+4+len].copy_from_slice(bytes);
        
        self.is_dirty = true;
        Ok(offset + len + 4)
    }

    // Arabic string reading
    pub fn read_arabic_string(&self, offset: usize) -> Result<String, StorageError> {
        let len_bytes = &self.data[offset..offset+4];
        let len = u32::from_be_bytes(len_bytes.try_into()?) as usize;
        
        let bytes = &self.data[offset+4..offset+4+len];
        String::from_utf8(bytes.to_vec())
            .map_err(|_| StorageError::EncodingError)
    }
}

