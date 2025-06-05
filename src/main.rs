fn main() {
    println!("Hello, world!");
}

type MerkleTree = Vec<Vec<String>>;
type Input = Vec<Vec<u8>>;

pub fn create_merkle_tree_from_data(inputs: &Input) -> MerkleTree {
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn creates_data_hashes() {
        let data_1 = Vec::from(b"sofi");
        let data_2 = Vec::from(b"fran");
        let data_3 = Vec::from(b"nati");
        let data_4 = Vec::from(b"lola");
        let mock_data: Input = vec![data_1, data_2, data_3, data_4];
        let hashes = create_data_hashes(&mock_data);

        for i in 0..hashes.len() {
            assert_eq!(hashes[i], sha256::digest(mock_data[i].clone()));
        }
    }

    #[test]
    fn creates_merkle_tree() {
        // for 4 elements of data the tree would look something like
        //           R
        //        H5   H6
        //      H1 H2 H3 H4   --> three levels
        let data_1 = Vec::from(b"sofi");
        let data_2 = Vec::from(b"fran");
        let data_3 = Vec::from(b"nati");
        let data_4 = Vec::from(b"lola");

        let mock_data: Input = vec![data_1, data_2, data_3, data_4];

        let merkle_tree = create_merkle_tree_from_data(&mock_data);
        assert_eq!(merkle_tree.len(), 3);

        for i in 0..merkle_tree.len() {
            if i != merkle_tree.len() - 1 {
                let higher_level = merkle_tree[i].clone();
                let lower_level = merkle_tree[i + 1].clone();
                for x in 0..higher_level.len() {
                    let l_child = lower_level[x * 2].clone();
                    let r_child = lower_level[x * 2 + 1].clone();
                    let concat_children = l_child + &r_child;
                    let hashed_children = sha256::digest(concat_children);
                    assert_eq!(higher_level[x], hashed_children);
                }
            }
        }
    }
}
