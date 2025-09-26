use std::fs::File;
use std::io::BufReader;
use std::io::Error;
use std::io::Read;

pub struct Chunk {
    pub data: Vec<u8>,
    metadata_path: String,
    index: u32,
}

pub struct ChunkIterator {
    chunk_size: u32,
    file_reader: BufReader<File>,
    current_index: u32,
}

impl Iterator for ChunkIterator {
    type Item = Chunk;

    fn next(&mut self) -> Option<Self::Item> {
        // TODO: Add adadptive size
        let mut chunk_data_vec = vec![0; 64 * 1024];
        self.current_index += 1;

        match self.file_reader.read(&mut chunk_data_vec) {
            Ok(_data) => Some(Chunk {
                data: chunk_data_vec,
                metadata_path: "".to_string(),
                index: self.current_index,
            }),

            Err(e) => {
                eprintln!("ERROR: Couldn't read from buffer. {}", e);
                None
            }
        }
    }
}

pub fn chunk_file(filepath: &str) -> Result<ChunkIterator, Error> {
    // Open the file, create a BufReader, attach it to an iterator, return the iterator

    let adaptive_size = 64;

    match File::open(filepath) {
        Ok(f) => {
            let reader = BufReader::new(f);

            Ok(ChunkIterator {
                chunk_size: adaptive_size * 1024,
                file_reader: reader,
                current_index: 0,
            })
        }

        Err(e) => {
            eprint!("ERROR: {}", e);
            Err(e)
        }
    }
}
