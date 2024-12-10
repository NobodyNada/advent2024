pub use aoc_2024::prelude::*;

fn main() -> Result<()> {
    let c = chal()?;
    let input = c.input.lines().next().unwrap()?;

    let files = input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u64)
        .chunks(2);
    let files = files
        .into_iter()
        .map(|mut chunk| (chunk.next().unwrap(), chunk.next().unwrap_or(0)));

    if c.part1 {
        let mut disk = Vec::<Option<u64>>::new();
        let mut ids = 0..;
        for (used, free) in files {
            disk.extend(std::iter::repeat_n(ids.next(), used as usize));
            disk.extend(std::iter::repeat_n(None, free as usize));
        }

        let mut start = 0;
        let mut end = disk.len() - 1;

        while start < end {
            // find free space on left and used space on right
            if disk[start].is_some() {
                start += 1;
            } else if disk[end].is_none() {
                end -= 1;
            } else {
                // move from used space to free space
                disk[start] = disk[end].take();
                start += 1;
                end -= 1;
            }
        }

        let checksum = disk
            .iter()
            .enumerate()
            .filter_map(|(i, &file)| Some(i as u64 * file?))
            .sum::<u64>();
        println!("{checksum}");
    } else {
        #[derive(Debug)]
        struct File {
            id: u64,
            pos: u64,
            size: u64,
        }
        let mut disk = Vec::<File>::new();

        let mut pos = 0;
        for (size, free) in files {
            disk.push(File {
                id: disk.len() as u64,
                pos,
                size,
            });
            pos += size + free;
        }

        let mut i = disk.len().saturating_sub(1);
        while i > 0 {
            // Try to move file i
            let file = &disk[i];
            let first_fit = disk
                .iter()
                .take(i + 1)
                .enumerate()
                .tuple_windows()
                .map(|((i, file1), (_, file2))| (i, file2.pos - (file1.pos + file1.size)))
                .find(|&(_, free)| free >= file.size);
            if let Some((free_idx, _)) = first_fit {
                // We found a spot where the file fits
                disk[i].pos = disk[free_idx].pos + disk[free_idx].size;
                disk[free_idx + 1..=i].rotate_right(1);
            } else {
                i -= 1;
            }
        }

        let checksum = disk
            .iter()
            .flat_map(|file| (file.pos..file.pos + file.size).map(|pos| pos * file.id))
            .sum::<u64>();
        println!("{checksum}");
    }

    Ok(())
}
