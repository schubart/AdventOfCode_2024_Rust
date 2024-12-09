use std::iter::repeat_n;

type Id = usize;
type Block = Option<Id>;
type Disk = Vec<Block>;
type Start = usize;
type Length = usize;
type Span = (Start, Length);
type Files = Vec<Span>;
type Gaps = Vec<Span>;
type Checksum = usize;

pub fn part1(input: &str) -> Checksum {
    let (mut disk, _, _) = parse(input);

    for next in 0..(disk.len()) {
        // Ensure disk ends with non-empty block.
        while disk.last() == Some(&None) {
            disk.truncate(disk.len() - 1);
        }
        // If next block is empty, bring last block forward.
        if disk.get(next) == Some(&None) {
            disk.swap_remove(next);
        }
    }

    checksum(&disk)
}

pub fn part2(input: &str) -> Checksum {
    let (mut disk, files, mut gaps) = parse(input);

    // Try to move each file, starting from the end.
    for (file_start, file_len) in files.iter().rev() {
        // Try to find suitable gap, searching from the start.
        // Only consider gaps that come before the file.
        // Find gap that is large enough for the file.
        if let Some((gap_start, gap_len)) = gaps
            .iter_mut()
            .take_while(|(start, _)| start < file_start)
            .find(|(_, len)| len >= file_len)
        {
            // Swap file blocks and gap blocks.
            (0..*file_len).for_each(|i| disk.swap(file_start + i, *gap_start + i));

            // Update gap.
            *gap_len -= file_len;
            *gap_start += file_len;
        }
    }

    checksum(&disk)
}

fn checksum(disk: &Disk) -> Checksum {
    disk.iter()
        .enumerate()
        .filter_map(|(pos, id)| id.map(|id| id * pos))
        .sum()
}

fn parse(input: &str) -> (Disk, Files, Gaps) {
    let mut disk = Disk::new();
    let mut files = Files::new();
    let mut gaps = Gaps::new();

    let chars = input.trim().chars();
    let mut lengths = chars.map(|c| c.to_digit(10).unwrap() as usize);

    while let Some(file_len) = lengths.next() {
        let file_id = files.len();
        let file_start = disk.len();
        files.push((file_start, file_len));
        disk.extend(repeat_n(Some(file_id), file_len));

        let gap_len = lengths.next().unwrap_or(0);
        let gap_start = disk.len();
        gaps.push((gap_start, gap_len));
        disk.extend(repeat_n(None, gap_len));
    }

    (disk, files, gaps)
}

#[test]
fn test_part1() {
    assert_eq!(1928, part1(include_str!("example.txt")));
    assert_eq!(6332189866718, part1(include_str!("input.txt")));
}

#[test]
fn test_part2() {
    assert_eq!(2858, part2(include_str!("example.txt")));
    assert_eq!(6353648390778, part2(include_str!("input.txt")));
}
