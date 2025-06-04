fn main() {
    println!("Hello, world!");
}

type MerkleTree = Vec<Vec<String>>;
type Input = Vec<Vec<u8>>;

pub fn create_tree_from_data(inputs: &Input) -> MerkleTree {
    let mut merkle_tree: MerkleTree = Vec::new();
    let data_hashes_level = create_data_hashes(inputs);
    merkle_tree.push(data_hashes_level.clone());
    let mut previous_level = data_hashes_level;
    let mut level_len = previous_level.len();
    while level_len != 1 {
        let new_level = create_new_level(&previous_level);
        merkle_tree.insert(0, new_level.clone());
        previous_level = new_level;
        level_len = previous_level.len();
    }
    merkle_tree
}

fn create_data_hashes(inputs: &Input) -> Vec<String> {
    let mut data_hashes = Vec::new();
    for i in inputs {
        data_hashes.push(sha256::digest(i));
    }
    data_hashes
}

fn create_new_level(previous_level: &[String]) -> Vec<String> {
    let mut index = 0;
    let mut new_level = Vec::new();
    while index < previous_level.len() {
        let element_concat = previous_level[index].clone() + &previous_level[index + 1];
        let element_hash = sha256::digest(element_concat);
        new_level.push(element_hash);
        index += 2;
    }
    new_level
}
