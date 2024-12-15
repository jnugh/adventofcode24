#[derive(Clone, Debug, PartialEq, Eq)]
enum Data {
    Empty,
    Occupied(usize),
}

#[derive(Clone, Debug)]
struct Block {
    len: usize,
    data: Data,
}

impl Data {
    fn is_file(&self) -> bool {
        match self {
            Data::Occupied(_) => true,
            Data::Empty => false,
        }
    }
}

#[derive(Clone, Debug)]
struct Disk {
    data: Vec<Block>,

    last_empty_index: usize,
}

fn parse_input(input: String) -> Disk {
    let mut data = Vec::new();
    let mut block_id = 0;
    for (n, char) in input.trim().chars().enumerate() {
        if n % 2 == 0 {
            let count = char as usize - '0' as usize;

            data.push(Block {
                data: Data::Occupied(block_id),
                len: count,
            });
            block_id += 1;
        } else {
            let count = char as usize - '0' as usize;
            data.push(Block {
                data: Data::Empty,
                len: count,
            });
        }
    }

    Disk {
        data,
        last_empty_index: 0,
    }
}

impl Disk {
    fn get_next_empty_slot(&mut self) -> Option<usize> {
        for pos in self.last_empty_index..self.data.len() {
            if self.data[pos].data == Data::Empty {
                self.last_empty_index = pos;
                return Some(self.last_empty_index);
            }
        }
        None
    }

    fn get_last_data(&self) -> usize {
        for i in (0..self.data.len()).rev() {
            if self.data[i].data != Data::Empty {
                return i;
            }
        }

        panic!("Data is empty!")
    }

    fn migrate(&mut self, src: usize, dst: usize) {
        match self.data[dst].len.cmp(&self.data[src].len) {
            std::cmp::Ordering::Greater => {
                self.data[dst].len -= self.data[src].len;
                let old_item = self.data.remove(src);
                self.data.insert(
                    src,
                    Block {
                        data: Data::Empty,
                        len: old_item.len,
                    },
                );
                self.data.insert(dst, old_item);
            }
            std::cmp::Ordering::Less => {
                let dst_size = self.data[dst].len;
                self.data[dst] = self.data[src].clone();
                self.data[dst].len = dst_size;
                self.data[src].len -= dst_size;
            }
            std::cmp::Ordering::Equal => {
                self.data[dst] = self.data[src].clone();
                self.data[src].data = Data::Empty;
            }
        }

        let mut compacts: Vec<usize> = Vec::new();
        for i in 0..self.data.len() - 1 {
            if (&self.data[i].data, &self.data[i + 1].data) == (&Data::Empty, &Data::Empty) {
                compacts.push(i);
            }
        }

        compacts.reverse();
        for compact in compacts {
            self.data[compact].len += self.data[compact + 1].len;
            self.data.remove(compact + 1);
        }
    }

    fn files_reverse(&self) -> impl Iterator<Item = (usize, &Block)> {
        self.data
            .iter()
            .enumerate()
            .rev()
            .filter(|(_, v)| v.data.is_file())
    }

    fn get_empty_slots(&self) -> impl Iterator<Item = (usize, &Block)> {
        self.data
            .iter()
            .enumerate()
            .filter(|(_, v)| !v.data.is_file())
    }
}

fn defrag(data: &mut Disk, keep_file_sequence: bool) {
    if keep_file_sequence {
        defrag_file_level(data);
    } else {
        defrag_block_level(data);
    }
}

fn defrag_block_level(data: &mut Disk) {
    while let Some(next_empty_slot) = data.get_next_empty_slot() {
        let last_data_slot = data.get_last_data();
        if last_data_slot < next_empty_slot {
            break;
        }

        data.migrate(last_data_slot, next_empty_slot);
    }
}

fn defrag_file_level(data: &mut Disk) {
    let mut changed = true;
    let mut last_id = usize::MAX;
    while changed {
        changed = false;
        let orig = data.clone();
        let files: Vec<(usize, &Block)> = orig
            .files_reverse()
            .filter(|(_, d)| matches!(d.data, Data::Occupied(x) if x < last_id))
            .collect();
        let empty_slots: Vec<(usize, &Block)> = orig.get_empty_slots().collect();

        'outer: for (f_idx, f) in files {
            for (space_idx, space) in &empty_slots {
                if f_idx < *space_idx {
                    break;
                }

                if f.len <= space.len {
                    data.migrate(f_idx, *space_idx);
                    changed = true;
                    last_id = match f.data {
                        Data::Occupied(data) => data,
                        _ => last_id,
                    };
                    break 'outer;
                }
            }
        }
    }
}

fn calculate_checksum(disk: Disk) -> usize {
    let mut offset: usize = 0;
    let mut result: usize = 0;
    for block in disk.data {
        for _ in 0..block.len {
            result += match block.data {
                Data::Empty => 0,
                Data::Occupied(data) => data * offset,
            };
            offset += 1;
        }
    }

    result
}

pub fn day9_part1(input: String) -> usize {
    let mut disk = parse_input(input);
    defrag(&mut disk, false);

    calculate_checksum(disk)
}

pub fn day9_part2(input: String) -> usize {
    let mut disk = parse_input(input);
    defrag(&mut disk, true);

    calculate_checksum(disk)
}

#[cfg(test)]
mod test {
    use crate::day9::{day9_part1, day9_part2};

    #[test]
    fn test_part1() {
        assert_eq!(1928, day9_part1("2333133121414131402".to_string()));
    }

    #[test]
    fn test_part2() {
        assert_eq!(2858, day9_part2("2333133121414131402".to_string()));
    }
}
