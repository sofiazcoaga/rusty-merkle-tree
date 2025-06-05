/// sha256::digest("") - the hash of an empty string
const DEFAULT_HASH_OF_EMTPY: &str = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";

fn main() {
    println!("Hello, world!");
}

//type MerkleTree = Vec<Vec<String>>;
pub struct MerkleTree {
    merkle_tree: Vec<Vec<String>>,
    last_element: usize
}
impl MerkleTree {
    pub fn new() -> Self {
        MerkleTree {
            merkle_tree: Vec::new(),
            last_element: 0
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

    pub fn set_last_element(&mut self, element_index: usize) {
        self.last_element = element_index;
    }
}
type Input = Vec<Vec<u8>>;

pub fn create_merkle_tree_from_data(inputs: &Input) -> MerkleTree {
    let mut merkle_tree = MerkleTree::new();
    let data_hashes_level = create_data_hashes(inputs);
    merkle_tree.push_level_back(data_hashes_level.clone());

    let mut previous_level = data_hashes_level;
    let mut level_len = previous_level.len();

    while level_len != 1 {
        let new_level = create_new_level(&previous_level);
        merkle_tree.push_level_front(new_level.clone());
        previous_level = new_level;
        level_len = previous_level.len();
    }

    merkle_tree.set_last_element(inputs.len() -1 );
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
        data_hashes.append(&mut vec![DEFAULT_HASH_OF_EMTPY.to_string().clone(); fill_amount]);
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
        let merkle_tree_struct = create_merkle_tree_from_data(&mock_data);
        let merkle_tree = merkle_tree_struct.merkle_tree;
        assert_eq!(merkle_tree.len(), 3);
        assert_eq!(merkle_tree_struct.last_element, mock_data.len()-1);
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

    #[test]
    fn verify_element_in_tree() {
        let mock_data = create_mock_data_from_strings(vec!["12313414", "1345", "124214", "125151"]);
        let merkle_tree_struct = create_merkle_tree_from_data(&mock_data);
        let merkle_tree = merkle_tree_struct.merkle_tree;
        let mut is_element_present = verify_element(
            mock_data[0].clone(),
            &vec![merkle_tree[2][1].clone(), merkle_tree[1][1].clone()],
            merkle_tree[0][0].clone(),
            0,
        );
        assert!(is_element_present);

        is_element_present = verify_element(
            b"absent_element".to_vec(),
            &vec![merkle_tree[2][1].clone(), merkle_tree[1][1].clone()],
            merkle_tree[0][0].clone(),
            0,
        );
        assert!(!is_element_present);
    }

    #[test]
    fn fill_until_power_of_two() {
        let mock_data = create_mock_data_from_strings(vec!["data1", "data2", "data3", "data4", "data5"]);
        let data_hashes = create_data_hashes(&mock_data);
        let merkle_tree_struct = create_merkle_tree_from_data(&mock_data);
        let fill_amount = mock_data.len().next_power_of_two() - mock_data.len();
        let default_data = data_hashes[data_hashes.len()-fill_amount..data_hashes.len()].to_vec();
        assert_eq!(merkle_tree_struct.last_element, mock_data.len() -1 );
        assert!(default_data.iter().all(|x| *x == DEFAULT_HASH_OF_EMTPY.to_string()));
        assert_eq!(data_hashes.len(), mock_data.len().next_power_of_two());

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
