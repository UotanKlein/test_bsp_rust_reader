use std::fs::File;
use std::io::{Seek, Read, SeekFrom};
use std::mem::size_of;

const I32_SIZE: usize = size_of::<i32>();
const HEADER_LUMPS: usize = 64;

#[derive(Debug)]
struct LumpT {
    file_ofs: i32,
    file_len: i32,
    version: i32,
    four_cc: [u8; I32_SIZE],
}

#[derive(Debug)]
struct DHeaderT {
    ident: [u8; I32_SIZE],
    version: i32,
    lumps: [LumpT; HEADER_LUMPS],
    map_revision: i32,
}

const HEADER_SIZE: usize = size_of::<DHeaderT>();
const LUMP_SIZE: usize = size_of::<LumpT>();

fn get_i32_from_bytes(bytes: &[u8], start: usize) -> i32 {
    let slice = bytes.get(start..start + I32_SIZE).unwrap();
    i32::from_ne_bytes(slice.try_into().unwrap())
}

fn get_bytes_4(bytes: &[u8], start: usize) -> [u8; I32_SIZE] {
    bytes[start..start + I32_SIZE].try_into().unwrap()
}

fn read_exact_from_file(f: &mut File, start: u64, size: usize) -> Vec<u8> {
    f.seek(SeekFrom::Start(start)).unwrap();
    let mut buf = vec![0; size];
    f.read_exact(&mut buf).unwrap();
    buf
}

impl LumpT {
    fn new(header_bytes: &[u8], lump_num: usize) -> Self {
        let offset = I32_SIZE * 2 + lump_num * LUMP_SIZE;
        Self {
            file_ofs: get_i32_from_bytes(header_bytes, offset),
            file_len: get_i32_from_bytes(header_bytes, offset + I32_SIZE),
            version: get_i32_from_bytes(header_bytes, offset + I32_SIZE * 2),
            four_cc: get_bytes_4(header_bytes, offset + I32_SIZE * 3),
        }
    }
}

impl DHeaderT {
    fn new(path: &str) -> Self {
        let mut f = File::open(path).unwrap();
        let header_bytes = read_exact_from_file(&mut f, 0, HEADER_SIZE);
        Self {
            ident: get_bytes_4(&header_bytes, 0),
            version: get_i32_from_bytes(&header_bytes, I32_SIZE),
            lumps: std::array::from_fn(|i| LumpT::new(&header_bytes, i)),
            map_revision: get_i32_from_bytes(&header_bytes, I32_SIZE * 2 + LUMP_SIZE * HEADER_LUMPS),
        }
    }
}

fn main() {
    let bsp_path = "pasha.bsp";
    let d_header_t = DHeaderT::new(bsp_path);
    println!("{:?}", d_header_t);
}
