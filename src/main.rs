use lazy_static::lazy_static;
use std::vec;

// The hash of the zero value
lazy_static! {
    static ref DEFAULT_ZERO_HASH: String = sha256::digest(&[0]);
}

fn main() {
    println!("Hello, world!");
}

#[derive(Default)]
pub struct MerkleTree {
    merkle_tree: HashTree,
    last_element_index: usize,
}
impl MerkleTree {
    pub fn new(merkle_tree: HashTree, last_element_index: usize) -> Self {
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

    pub fn height(&self) -> usize {
        self.merkle_tree.len()
    }

    fn set_last_element_index(&mut self, element_index: usize) {
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
        self.get_level(self.height() - 1).len()
    }

    fn set_element(&mut self, level: usize, index: usize, element: String) {
        self.merkle_tree[level][index] = element;
    }

    /// Allows the addition of a new element to the tree and updates the rest of it accordingly.
    /// Because the Merkle Tree always has a leaves amount that is a power of two there are two cases to be considered:
    /// Case 1 -> The tree was filled with default data to reach a power of two
    ///     In this case, the new element will replace the first default element after the data and consequently its parents
    ///     will be updated.
    /// Case 2 -> The tree's current data elements amount naturally reaches a power of two
    ///     In this case, the tree will need to fill its leaves with default data. This occurs because if we consider N the current number of leaves
    ///     and N is a power of two (as a rule) then in no case will N + 1 (adding the new element) remain a power of two. Therefore a matching size
    ///     tree will be constructed to do this and then will be appended to the original tree by calculating a new root that is built from.
    ///     Consider the example:
    ///                     R
    ///                 H1      H2      --> adding a new element will require to have 8 leaves (next power of two).
    ///             H3  H4      H5 H6
    /// Therefore another "half tree" is constructed and both are appended converging in a calculated new root
    ///                                 R'
    ///                     R1                   R2
    ///                 H1      H2            H7     H8                 Where everything under R1 is the original tree,
    ///             H3  H4      H5 H6      H9 H10    H11 H12            everything under R2 is the new subtree and H9 is the hash of the new element
    ///                                                                 (H10, H11, H12 are default data) and R' is the new calculated root (hash of (R1+R2))
    ///
    pub fn add_element(&mut self, new_element: Vec<u8>) {
        let next_index = self.last_element_index() + 1;

        // Contemplates Case 2. The element requires a new index past the existing ones and the tree must be extended.
        if next_index == self.leaves_amount() {
            self.extend_tree(new_element);
        }
        // Contemplates Case 1. The new element replaces an existing index and the tree must be updated.
        else {
            self.modify_existing_leaf_and_update_parents(new_element);
        }

        // After any of the cases, the last index with real data must be updated.
        self.set_last_element_index(next_index);
    }

    #[allow(clippy::needless_range_loop)] // Allow this to make code more understandable
    /// Manages the case when the tree must be extended to add an element.
    fn extend_tree(&mut self, new_element: Vec<u8>) {
        let leaves_amount = self.leaves_amount();
        // The needed amount of default data to fill the leaves. Given by the calculation:
        // Example: if the current tree has 4 leaves then:
        // current_amount_of_leaves + new_leaf = 4 + 1 = 5
        // next_power_of_two(5) = 8
        // needed leaves to reach next power of two from current state -> 8 - 4 = 4
        // taking into account that 1 of those leaves is the actual new data the fill amount is 3
        let fill_amount = ((leaves_amount + 1).next_power_of_two() - leaves_amount) - 1;

        // Create the leaves level of the new subtree
        let mut leaves = vec![sha256::digest(new_element)];
        leaves.append(&mut vec![DEFAULT_ZERO_HASH.to_string(); fill_amount]);

        // Create a new merkle tree for the new data subset
        let half_merkle_tree = create_hash_tree_from_leaves(leaves);

        // Append the new merkle tree's levels to the previous tree
        for i in 0..self.height() {
            self.merkle_tree[i].extend_from_slice(&half_merkle_tree[i]);
        }

        // Obtain the new root from the roots of the two subtrees.
        let root =
            sha256::digest(self.get_tree_element(0, 0).clone() + self.get_tree_element(0, 1));
        self.push_level_front(vec![root]);
    }

    /// Manages the case when an element must be added in an existing position and the tree must be updated afterwards.
    fn modify_existing_leaf_and_update_parents(&mut self, new_element: Vec<u8>) {
        let mut element_index = self.last_element_index + 1;
        let mut parent_index = element_index / 2;
        let mut level = self.height() - 1;
        let mut element_hash = sha256::digest(new_element.clone());

        // Insert the new element in the first index that was filled with default data
        self.set_element(level, element_index, element_hash.clone());

        // While we are not in the top level, keep updating the parents
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

    /// Provides an vector of hashes known as "proof" that give a user the possibility to verify
    /// that an element in a specific index is a part of the tree. This pairs with the `verify_element`
    /// function to prove an element is a part of a tree.
    pub fn get_merkle_proof(&self, element_index: usize) -> Vec<String> {
        let mut proof = Vec::new();
        let mut element_index = element_index;
        let mut level = self.height() - 1;

        while level != 0 {
            match element_index % 2 {
                0 => element_index += 1,
                _ => element_index -= 1,
            }
            let proof_element = self.get_tree_element(level, element_index).clone();
            proof.push(proof_element);
            level = level - 1;
            element_index /= 2;
        }

        proof
    }

    pub fn get_root(&self) -> String {
        self.get_tree_element(0, 0).clone()
    }
}

type Input = Vec<Vec<u8>>;
type HashTree = Vec<Vec<String>>;
/// Allows the creation of a Merkle Tree from its base level (or leaves level) which are the hashes of the original
/// data. This function is useful to extend the Merkle Tree when adding a new element.
fn create_hash_tree_from_leaves(leaves: Vec<String>) -> HashTree {
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

/// Allows the creation of a Merkle Tree from a vector of buffers as data elements. If necessary, it will add default
/// data elements to the leaves level so the amount is a power of two
pub fn create_merkle_tree_from_data(inputs: &Input) -> MerkleTree {
    let mut merkle_tree = MerkleTree::default();
    let data_hashes_level = create_data_hashes(inputs);

    let tree = create_hash_tree_from_leaves(data_hashes_level);

    merkle_tree.merkle_tree = tree;
    merkle_tree.set_last_element_index(inputs.len() - 1);
    merkle_tree
}

/// Creates the leaves level from a specific input. Meaning it hashes every data element from the input and returns
/// a vector of those hashes that can later be used as the lower level of the tree.
/// Notice that it fills with default elements if necessary to reach an amount that is a power of two.
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

/// Given the previous constructed level of the Merkle Tree it allows to create a parent level for it by concatenating
/// elements in pairs and hashing the results. It is helpful to construct a Merkle Tree from bottom to top.
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

/// Given a proof provided by the Merkle Tree it allows to verify that a certain element has been stored at a certain index
/// in the Merkle Tree.
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

        assert_eq!(merkle_tree.height(), 3);
        assert_eq!(merkle_tree.last_element_index(), mock_data.len() - 1);
        assert_merkle_tree_is_consistent(merkle_tree);
    }

    #[test]
    fn verify_element_in_tree() {
        let mock_data = create_mock_data_from_strings(vec!["12313414", "1345", "124214", "125151"]);
        let merkle_tree = create_merkle_tree_from_data(&mock_data);

        let proof = merkle_tree.get_merkle_proof(0);
        let mut is_element_present = verify_element(
            mock_data[0].clone(),
            &proof,
            merkle_tree.get_tree_element(0, 0).clone(),
            0,
        );
        assert!(is_element_present);

        is_element_present = verify_element(
            b"absent_element".to_vec(),
            &proof,
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
    fn get_merkle_proof() {
        let mock_data =
            create_mock_data_from_strings(vec!["data1", "data2", "data3", "data4", "data5"]);
        let merkle_tree = create_merkle_tree_from_data(&mock_data);
        // We want to verify that "data3" is a part of the tree
        let merkle_proof = merkle_tree.get_merkle_proof(2);
        assert_eq!(
            merkle_proof.len(),
            merkle_tree.leaves_amount().ilog2() as usize
        );
        assert!(verify_element(
            mock_data[2].clone(),
            &merkle_proof,
            merkle_tree.get_root(),
            2
        ));
        assert!(!verify_element(
            mock_data[3].clone(),
            &merkle_proof,
            merkle_tree.get_root(),
            3
        ));
    }

    // Add an element without needing to extend the tree
    #[test]
    fn add_element_in_existing_position() {
        let mock_data = create_mock_data_from_strings(vec!["12345", "6789", "3645738"]);
        let mut merkle_tree = create_merkle_tree_from_data(&mock_data);
        // Should have completed data to four
        assert_eq!(
            merkle_tree.leaves_amount(),
            mock_data.len().next_power_of_two()
        );
        merkle_tree.add_element(b"128746124".to_vec());
        // Should still have same amount of elements
        assert_eq!(
            merkle_tree.leaves_amount(),
            mock_data.len().next_power_of_two()
        );

        assert_merkle_tree_is_consistent(merkle_tree);
    }

    // Add an element by extending the tree
    #[test]
    fn add_element_in_new_position() {
        // Amount of elements is 8 which is 2^3 and tree will no need default data
        let mock_data = create_mock_data_from_strings(vec![
            "123", "1234", "12345", "123456", "1234567", "12312458", "1241423", "141251",
        ]);
        let mut merkle_tree = create_merkle_tree_from_data(&mock_data);
        let previous_height = merkle_tree.height();
        assert_eq!(merkle_tree.leaves_amount(), mock_data.len());
        merkle_tree.add_element(b"295873459817".to_vec());
        // Make sure the tree had to fill data
        let new_amount_of_elements = mock_data.len() + 1; // Include the new element
        assert_ne!(
            new_amount_of_elements,
            new_amount_of_elements.next_power_of_two()
        );
        assert_eq!(
            merkle_tree.leaves_amount(),
            new_amount_of_elements.next_power_of_two()
        );
        assert_eq!(merkle_tree.height(), previous_height + 1);
        assert_merkle_tree_is_consistent(merkle_tree);
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

    /// Util function to assert every level of the tree is well constructed
    /// and every element is a hash of its children's concatenation hash
    fn assert_merkle_tree_is_consistent(merkle_tree: MerkleTree) {
        for i in 0..merkle_tree.height() {
            if i != merkle_tree.height() - 1 {
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
}
