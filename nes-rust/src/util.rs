use std::ops::Range;

pub fn print_hex_dump(mut data: Box<[u8]>, range: Option<Range<usize>>) {
    let start = range.as_ref().map_or(0, |r| r.start);
    let end = range.as_ref().map_or(data.len(), |r| r.end.min(data.len()));

    if start >= end {
        println!("Invalid range or empty data.");
        return;
    }

    const BYTES_PER_LINE: usize = 16;

    let slice = &data[start..end];

    for (i, chunk) in slice.chunks(BYTES_PER_LINE).enumerate() {
        let offset = start + i * BYTES_PER_LINE;
        print!("{:08X}  ", offset); // Offset

        // Hex representation
        for &byte in chunk {
            print!("{:02X} ", byte);
        }
        for _ in 0..(BYTES_PER_LINE - chunk.len()) {
            print!("   "); // Padding for alignment
        }

        print!(" |");

        // ASCII representation
        for &byte in chunk {
            let c = byte;
            print!("{}", if c.is_ascii_graphic() || c == b' ' { c as char } else { '.' });
        }

        println!("|");
    }

    // Move `data` back into itself to ensure ownership is retained
    std::mem::drop(data);
}
