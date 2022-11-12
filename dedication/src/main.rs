use itertools::Itertools;

fn main() {
    let i: i32 = 0x01B99644; // index of the dedication permutation
    let items: &str = " DFaeeillnor"; // a permutation of the dedication

    // get the permutation at i using nth function
    match items
        .chars()
        .permutations(items.len())
        .unique()
        .nth(i as usize)
    {
        None => panic!(),
        Some(perm) => {
            // output the dedication "For Danielle"
            println!("{}", perm.iter().cloned().collect::<String>());
        }
    }
}
