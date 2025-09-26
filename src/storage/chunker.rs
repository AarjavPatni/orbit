use std::fs::File;
use std::io::BufReader;
use std::io::Error;
use std::io::Read;
use std::os::unix::fs::MetadataExt;

const KB: u64 = 1024;
const MB: u64 = 1024 * KB;
const GB: u64 = 1024 * MB;

pub struct Chunk {
    pub data: Vec<u8>,
    metadata_path: String,
    index: u32,
}

pub struct ChunkIterator {
    chunk_size: u64,
    file_reader: BufReader<File>,
    current_index: u32,
}

impl Iterator for ChunkIterator {
    type Item = Chunk;

    fn next(&mut self) -> Option<Self::Item> {
        /*
        Q. Why casted to usize? Does this mean that half the bits will be truncated upon a large value of chunk_size?
        A. No, I won't hit the limits because the max chunk size I'm using is 4MB, which is well within the limits of 32-bit systems.
         */

        // TODO: Understand the limits of 32-bit systems. Prompt: "Explain 32-bit vs 64-bit memory addressing and why 32-bit systems have a 4GB RAM limit per process"

        let mut chunk_data_vec = vec![0; self.chunk_size as usize];
        self.current_index += 1;

        match self.file_reader.read(&mut chunk_data_vec) {
            Ok(_data) => Some(Chunk {
                data: chunk_data_vec,
                // TODO: Use the correct metadata path
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
    match File::open(filepath) {
        Ok(f) => {
            let file_size = f.metadata()?.size();
            let chunk_size: u64;

            if file_size < 1 * MB {
                chunk_size = 64 * KB;
            } else if file_size < 100 * MB {
                chunk_size = 1 * MB;
            } else {
                chunk_size = 4 * MB;
            }

            let reader = BufReader::new(f);

            Ok(ChunkIterator {
                chunk_size: chunk_size,
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
