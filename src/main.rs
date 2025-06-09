use crate::merkle_tree::{MerkleTree, create_merkle_tree_from_data, verify_element};

pub mod merkle_tree;
pub fn main() {
    // Create a Merkle Tree with 3 data elements will generate a tree with 4 leaves
    let my_data = vec![
        b"Hello".to_vec(),
        b"i am a".to_vec(),
        b"Merkle Tree".to_vec(),
    ];
    for d in my_data.clone() {
        println!(
            "\nFor data: {:?} the hash is: {:?}",
            String::from_utf8(d.clone()).unwrap(),
            sha256::digest(d)
        );
    }

    // Create Merkle Tree
    let mut merkle_tree = create_merkle_tree_from_data(&my_data).unwrap();
    println!("\n The initial Merkle tree is (hashes can be verified in leaves): ");
    print_three_level_tree(&merkle_tree);

    println!(
        "Now let's add the element {:?} with hash {:?}",
        "!",
        sha256::digest(b"!".to_vec())
    );

    // Add an element
    let add_result = merkle_tree.add_element(b"!".to_vec());
    assert!(add_result.is_ok());

    println!("The new tree is: ");
    print_three_level_tree(&merkle_tree);

    println!("The last leaf that had been filled with default values was replaced!");

    // Get a proof - as we are chosing index "1" we are generating a proof for "i am a";
    let element_index = 1;
    let proof = merkle_tree.get_merkle_proof(element_index);
    println!("\nNow we generate a proof for our value: \"i am a\" which is in index 1...");
    println!("\nOur calculated Merkle Proof is: {:?}", proof.clone());
    println!(
        "\nThe length of the proof is {:?} because for a {:?} amount of leaves we need log2({:?}) elements in the proof to verify the element.",
        proof.len(),
        merkle_tree.leaves_amount(),
        merkle_tree.leaves_amount()
    );
    // Verify the element
    println!("Now we use this proof to verify the element...");
    let result = verify_element(
        my_data[element_index].clone(),
        &proof,
        merkle_tree.get_root(),
        element_index,
    );
    assert!(result);
    println!("The element was verified!\n");
}

// Only for example purposes
fn print_three_level_tree(tree: &MerkleTree) {
    println!(
        "\n                 {:?}        \n",
        &tree.get_tree_element(0, 0)[..8]
    );
    println!(
        "      {:?}              {:?}    \n",
        &tree.get_tree_element(1, 0)[..8],
        &tree.get_tree_element(1, 1)[..8]
    );
    println!(
        " {:?} {:?}   {:?} {:?} \n",
        &tree.get_tree_element(2, 0)[..8],
        &tree.get_tree_element(2, 1)[..8],
        &tree.get_tree_element(2, 2)[..8],
        &tree.get_tree_element(2, 3)[..8]
    );
}
