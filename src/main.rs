use std::vec;

/// sha256::digest("") - the hash of an empty string
const DEFAULT_ZERO_HASH: &str = "6e340b9cffb37a989ca544e6bb780a2c78901d3fb33738768511a30617afa01d";
const DEFAULT_ZERO: [u8; 1] = [0];

fn main() {
    println!("Hello, world!");
}

#[derive(Default)]
pub struct MerkleTree {
    merkle_tree: Vec<Vec<String>>,
    last_element_index: usize,
}
impl MerkleTree {
    pub fn new(merkle_tree: Vec<Vec<String>>, last_element_index: usize) -> Self {
        MerkleTree {
            merkle_tree,
            last_element_index,
        }
    }

    pub fn push_level_front(&mut self, level: Vec<String>) {
        self.merkle_tree.insert(0, level);
    }

    pub fn push_level_back(&mut self, level: Vec<String>) {
        self.merkle_tree.push(level);
    }

    pub fn len(&self) -> usize {
        self.merkle_tree.len()
    }

    pub fn set_last_element_index(&mut self, element_index: usize) {
        self.last_element_index = element_index;
    }

    pub fn get_tree_element(&self, level: usize, index: usize) -> &String {
        &self.merkle_tree[level][index]
    }

    pub fn last_element_index(&self) -> usize {
        self.last_element_index
    }

    pub fn get_level(&self, level: usize) -> &Vec<String> {
        &self.merkle_tree[level]
    }

    pub fn is_empty(&self) -> bool {
        self.merkle_tree.is_empty()
    }

    pub fn leaves_amount(&self) -> usize {
        self.get_level(self.len() - 1).len()
    }

    fn set_element(&mut self, level: usize, index: usize, element: String) {
        self.merkle_tree[level][index] = element;
    }

    pub fn add_element(&mut self, new_element: Vec<u8>) {
        let next_index = self.last_element_index() + 1;

        // In case the tree already has a data amount that is a power of two, an extension must be done
        // recalculating fill data
        if next_index == self.leaves_amount() {
            self.extend_tree(new_element);
        } else {
            self.modify_existing_leaf_and_update_parents(new_element);
        }
        self.set_last_element_index(next_index);
    }

    fn extend_tree(&mut self, new_element: Vec<u8>) {
        let leaves_amount = self.leaves_amount();
        let fill_amount = ((leaves_amount + 1).next_power_of_two() - leaves_amount) - 1;
        let mut leaves = vec![sha256::digest(new_element)];
        leaves.append(&mut vec![DEFAULT_ZERO_HASH.to_string(); fill_amount]);

        let mut half_merkle_tree = create_merkle_tree_from_leaves(leaves);
        // Push new merkle tree levels into previous one
        for i in 0..self.len() {
            self.merkle_tree[i].append(&mut half_merkle_tree[i]);
        }

        // create new zero level
        let root =
            sha256::digest(self.get_tree_element(0, 0).clone() + self.get_tree_element(0, 1));
        self.push_level_front(vec![root]);
    }

    fn modify_existing_leaf_and_update_parents(&mut self, new_element: Vec<u8>) {
        let mut element_index = self.last_element_index + 1;
        let mut parent_index = element_index / 2;
        let mut level = self.len() - 1;
        let mut element_hash = sha256::digest(new_element.clone());
        self.set_element(level, element_index, element_hash.clone());
        while level != 0 {
            let concat = match element_index % 2 {
                0 => element_hash.clone() + self.get_tree_element(level, element_index + 1),
                _ => self.get_tree_element(level, element_index - 1).clone() + &element_hash,
            };
            element_hash = sha256::digest(concat);
            self.set_element(level - 1, parent_index, element_hash.clone());
            level -= 1;
            element_index = parent_index;
            parent_index = element_index / 2;
        }
    }
}

type Input = Vec<Vec<u8>>;

fn create_merkle_tree_from_leaves(leaves: Vec<String>) -> Vec<Vec<String>> {
    let mut merkle_tree = Vec::new();
    merkle_tree.push(leaves.clone());
    let mut previous_level = leaves;
    let mut level_len = previous_level.len();

    while level_len != 1 {
        let new_level = create_new_level(&previous_level);
        merkle_tree.insert(0, new_level.clone());
        previous_level = new_level;
        level_len = previous_level.len();
    }
    merkle_tree
}

pub fn create_merkle_tree_from_data(inputs: &Input) -> MerkleTree {
    let mut merkle_tree = MerkleTree::default();
    let data_hashes_level = create_data_hashes(inputs);

    let tree = create_merkle_tree_from_leaves(data_hashes_level);

    merkle_tree.merkle_tree = tree;
    merkle_tree.set_last_element_index(inputs.len() - 1);
    merkle_tree
}

fn create_data_hashes(inputs: &Input) -> Vec<String> {
    let mut data_hashes = Vec::new();

    for i in inputs {
        data_hashes.push(sha256::digest(i));
    }
    let data_len = inputs.len();
    if !data_len.is_power_of_two() {
        let fill_amount = data_len.next_power_of_two() - data_len;
        data_hashes.append(&mut vec![
            DEFAULT_ZERO_HASH.to_string().clone();
            fill_amount
        ]);
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

pub fn verify_element(
    element_data: Vec<u8>,
    proof: &Vec<String>,
    root: String,
    element_index: usize,
) -> bool {
    let mut hash = sha256::digest(element_data);
    let mut index = element_index;
    for p in proof {
        let concat = match index % 2 {
            0 => hash.clone() + p,
            _ => p.to_owned() + &hash,
        };
        hash = sha256::digest(concat);
        index /= 2;
    }
    hash == root
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn creates_data_hashes() {
        let mock_data = create_mock_data_from_strings(vec!["sofi", "fran", "nati", "lola"]);
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
        let mock_data = create_mock_data_from_strings(vec!["pizza", "chocolate", "helado", "coca"]);
        let merkle_tree = create_merkle_tree_from_data(&mock_data);

        assert_eq!(merkle_tree.len(), 3);
        assert_eq!(merkle_tree.last_element_index(), mock_data.len() - 1);
        for i in 0..merkle_tree.len() {
            if i != merkle_tree.len() - 1 {
                let higher_level = merkle_tree.get_level(i);
                let lower_level = merkle_tree.get_level(i + 1);
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

    #[test]
    fn verify_element_in_tree() {
        let mock_data = create_mock_data_from_strings(vec!["12313414", "1345", "124214", "125151"]);
        let merkle_tree = create_merkle_tree_from_data(&mock_data);

        let mut is_element_present = verify_element(
            mock_data[0].clone(),
            &vec![
                merkle_tree.get_tree_element(2, 1).clone(),
                merkle_tree.get_tree_element(1, 1).clone(),
            ],
            merkle_tree.get_tree_element(0, 0).clone(),
            0,
        );
        assert!(is_element_present);

        is_element_present = verify_element(
            b"absent_element".to_vec(),
            &vec![
                merkle_tree.get_tree_element(2, 1).clone(),
                merkle_tree.get_tree_element(1, 1).clone(),
            ],
            merkle_tree.get_tree_element(0, 0).clone(),
            0,
        );
        assert!(!is_element_present);
    }

    #[test]
    fn fill_until_power_of_two() {
        let mock_data =
            create_mock_data_from_strings(vec!["data1", "data2", "data3", "data4", "data5"]);
        let data_hashes = create_data_hashes(&mock_data);
        let merkle_tree = create_merkle_tree_from_data(&mock_data);
        let fill_amount = mock_data.len().next_power_of_two() - mock_data.len();
        let default_data = data_hashes[data_hashes.len() - fill_amount..data_hashes.len()].to_vec();
        assert_eq!(merkle_tree.last_element_index(), mock_data.len() - 1);
        assert!(
            default_data
                .iter()
                .all(|x| *x == DEFAULT_ZERO_HASH.to_string())
        );
        assert_eq!(data_hashes.len(), mock_data.len().next_power_of_two());
    }

    #[test]
    fn add_an_element_1() {
        let mock_data = create_mock_data_from_strings(vec!["12345", "6789", "3645738"]);
        let mut merkle_tree = create_merkle_tree_from_data(&mock_data);
        // should have completed data to four
        assert_eq!(
            merkle_tree.leaves_amount(),
            mock_data.len().next_power_of_two()
        );
        merkle_tree.add_element(b"128746124".to_vec());
        // should still have same amount of elements
        assert_eq!(
            merkle_tree.leaves_amount(),
            mock_data.len().next_power_of_two()
        );

        merkle_tree.add_element(b"295873459817".to_vec());
        assert_eq!(merkle_tree.leaves_amount(), 8);
    }

    /// Util Functions
    fn create_mock_data_from_strings(data: Vec<&str>) -> Vec<Vec<u8>> {
        let mut mock_data = Vec::new();
        for e in data {
            let bytes = e.as_bytes().to_vec();
            mock_data.push(bytes);
        }
        mock_data
    }
}
