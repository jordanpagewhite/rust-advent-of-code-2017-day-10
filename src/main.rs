use core::array::from_fn;

const LIST_SIZE: usize = 256;
const DEBUG: bool = true;
//const LENGTHS: [usize; 4] = [3,4,1,5];
const LENGTHS: [usize; 16] = [129,154,49,198,200,133,97,254,41,6,2,1,255,0,191,108];

fn main() {
    macro_rules! debug {
        // println! macro below.
        ($($arg:tt)*) => {{
            if DEBUG {
                println!($($arg)*);
            }
        }};
    }

    let mut list: List = from_fn(|i| i + 1);
    let mut current_position: usize = 0;
    let mut skip_size: usize = 0;

    debug!("list: {:?}", list);
    for length in LENGTHS {
        debug!("---------------------------------");
        debug!("length: {length}");
        // @todo Why can't I use a const to declare array length?
        let mut temp_list: [usize; 256] = list;
        debug!("temp_list: {:?}", temp_list);
        for i in current_position..(current_position + length) {
            debug!("i: {i}");
            temp_list[i % LIST_SIZE] = list[(current_position + (length - 1) - (i - current_position)) % LIST_SIZE];
            debug!("temp_list: {:?}", temp_list);
        }
        list = temp_list;
        debug!("list: {:?}", list);
        current_position = (current_position + length + skip_size) % LIST_SIZE;
        debug!("current_position: {current_position}");
        skip_size += 1;
        debug!("skip_size: {skip_size}");
    }
    println!("list: {:?}", list);
    println!("Part 1: {}", list[0] * list[1]);
}
