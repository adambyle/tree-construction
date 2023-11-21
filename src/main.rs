use tree::Tree;

fn main() {
    let stdin = std::io::stdin();

    // The program will run until the user chooses to stop.
    loop {
        println!("\nChoose your traversal modes:");
        println!("  1. Inorder and preorder");
        println!("  2. Inorder and postorder");
        println!("  3. Preorder and postorder");
        println!("  4. Quit");

        let traversal_mode = loop {
            let mut trav_mode = String::new();
            stdin
                .read_line(&mut trav_mode)
                .expect("Error: could not read standard input.");
            if let Ok(trav_mode) = trav_mode.trim().parse() {
                if (1..=4).contains(&trav_mode) {
                    break trav_mode;
                }
            }
            println!("Please enter a number between 1 and 4.");
        };

        if traversal_mode == 4 {
            break;
        }

        let first_traversal;
        let second_traversal;
        match traversal_mode {
            1 => {
                println!("\nEnter inorder traversal:");
                first_traversal = get_number_sequence();
                println!("Enter preorder traversal:");
                second_traversal = get_number_sequence();
            }
            2 => {
                println!("\nEnter inorder traversal:");
                first_traversal = get_number_sequence();
                println!("Enter postorder traversal:");
                second_traversal = get_number_sequence();
            }
            3 => {
                println!("\nEnter preorder traversal:");
                first_traversal = get_number_sequence();
                println!("Enter postorder traversal:");
                second_traversal = get_number_sequence();
            }
            _ => unreachable!(),
        };

        if first_traversal.len() > 7 {
            println!("\nEntering more than 7 elements is an expensive operation.");
            println!("Tree construction consumes a lot of RAM and could crash your computer.");
            println!("Please remove this check and recompile to construct larger trees.");
            continue;
        }

        // Generate all trees that can contain numbers from the first traversal.
        // The program assumes that the second traversal contains the same numbers.
        let all_trees = Tree::all_trees_with_items(&first_traversal);

        println!("\nTrees that match:");
        let mut match_count = 0;
        for tree in all_trees {
            let passes = match traversal_mode {
                1 => tree.inorder() == first_traversal && tree.preorder() == second_traversal,
                2 => tree.inorder() == first_traversal && tree.postorder() == second_traversal,
                3 => tree.preorder() == first_traversal && tree.postorder() == second_traversal,
                _ => unreachable!(),
            };
            if passes {
                match_count += 1;
                println!("{:?}", tree);
            }
        }
        if match_count == 0 {
            println!("  None.");
        }
    }
}

fn get_number_sequence() -> Vec<i32> {
    let stdin = std::io::stdin();
    'seq_input: loop {
        let mut raw_numbers = String::new();
        stdin
            .read_line(&mut raw_numbers)
            .expect("Error: could not read standard input.");
        let mut numbers = Vec::new();
        for num in raw_numbers.trim().split(' ') {
            if let Ok(num) = num.parse() {
                numbers.push(num);
            } else {
                println!("Please enter a valid sequence of numbers separated by spaces.");
                continue 'seq_input;
            }
        }
        break numbers;
    }
}

mod tree;
