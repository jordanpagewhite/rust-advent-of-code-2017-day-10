const DEBUG: bool = true;
const LENGTHS: [usize; 4] = [3,4,1,5];
//const LENGTHS: [usize; 16] = [129,154,49,198,200,133,97,254,41,6,2,1,255,0,191,108];

struct KnotHash {
    list: Vec<usize>,
    list_size: usize,
    current_position: usize,
    skip_size: usize,
}

impl KnotHash {
    pub fn new(list_size: usize) -> Self {
        Self {
            list: (0..list_size).collect::<Vec<_>>(),
            list_size,
            current_position: 0,
            skip_size: 0,
        }
    }
    pub fn reverse_list_length(&mut self, length: usize) {
        //self.list[self.current_position..((self.current_position + length) % self.list_size)].reverse();
        self.list.iter()
            .cycle()
            .skip(self.current_position)
            .take(length)
            .collect::<Vec<&usize>>()
            .reverse();
        macros::debug!("{:?}", self.list);
    }
    pub fn update_current_position(&mut self, length: usize) {
        self.current_position = (self.current_position + length + self.skip_size) % self.list_size;
    }
    pub fn increment_skip_size(&mut self) {
        self.skip_size += 1;
    }
}

fn main() {
    let mut knot_hash: KnotHash = KnotHash::new(5);

    macros::debug!("list: {:?}", knot_hash.list);
    for length in LENGTHS {
        macros::debug!("---------------------------------");
        macros::debug!("length: {length}");
        knot_hash.reverse_list_length(length);
        macros::debug!("list: {:?}", knot_hash.list);
        knot_hash.update_current_position(length);
        macros::debug!("current_position: {}", knot_hash.current_position);
        knot_hash.increment_skip_size();
        macros::debug!("skip_size: {}", knot_hash.skip_size);
    }
    println!("list: {:?}", knot_hash.list);
    println!("Part 1: {}", knot_hash.list[0] * knot_hash.list[1]);
}

mod macros {
    macro_rules! debug {
        // println! macro below.
        ($($arg:tt)*) => {{
            if DEBUG {
                println!($($arg)*);
            }
        }};
    }
    pub(crate) use debug;
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests__unit__reverse_list_length {
    use super::*;

    #[test]
    fn one() {
        let mut knot_hash = KnotHash::new(1);
        knot_hash.reverse_list_length(1);
        assert_eq!(knot_hash.list, vec![0]);
    }

    #[test]
    fn five() {
        let mut knot_hash = KnotHash::new(5);
        knot_hash.reverse_list_length(5);
        assert_eq!(knot_hash.list, [4,3,2,1,0]);
    }

    #[test]
    fn five_multiple_reverses() {
        let mut knot_hash = KnotHash::new(5);
        // 4,3,2,1,0
        knot_hash.reverse_list_length(5);
        // 3,4,2,1,0
        knot_hash.reverse_list_length(2);
        assert_eq!(knot_hash.list, [3,4,2,1,0]);
    }

    #[test]
    fn five_move_current_position() {
        let mut knot_hash = KnotHash::new(5);
        knot_hash.update_current_position(3);
        knot_hash.reverse_list_length(2);
        assert_eq!(knot_hash.list, [0,1,2,4,3]);
    }

    #[test]
    fn five_wrap() {
        let mut knot_hash = KnotHash::new(5);
        knot_hash.update_current_position(3);
        knot_hash.reverse_list_length(4);
        assert_eq!(knot_hash.list, [4,3,2,1,0]);
    }

}
