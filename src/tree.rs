use std::fmt::Debug;

#[derive(Clone)]
pub struct Tree<T> {
    value: T,
    left: Option<Box<Tree<T>>>,
    right: Option<Box<Tree<T>>>,
}

impl<T: Clone> Tree<T> {
    pub fn all_trees_with_items(items: &[T]) -> Vec<Tree<T>> {
        if items.len() == 0 {
            return Vec::new();
        }

        if items.len() == 1 {
            return vec![Tree {
                value: items[0].clone(),
                left: None,
                right: None,
            }];
        }

        let mut trees = Vec::new();
        for i in 0..items.len() {
            let main_item = &items[i];
            let others = items_without(items, i);
            for perm in split_permutations(&others) {
                let left_trees = Tree::all_trees_with_items(&perm.0);
                let right_trees = Tree::all_trees_with_items(&perm.1);

                if left_trees.len() == 0 {
                    for right in right_trees {
                        trees.push(Tree {
                            value: main_item.clone(),
                            left: None,
                            right: Some(Box::new(right)),
                        });
                    }
                    continue;
                }

                if right_trees.len() == 0 {
                    for left in left_trees {
                        trees.push(Tree {
                            value: main_item.clone(),
                            left: Some(Box::new(left)),
                            right: None,
                        });
                    }
                    continue;
                }

                for left in &left_trees {
                    for right in &right_trees {
                        trees.push(Tree {
                            value: main_item.clone(),
                            left: Some(Box::new(left.clone())),
                            right: Some(Box::new(right.clone())),
                        });
                    }
                }
            }
        }

        trees
    }

    pub fn inorder(&self) -> Vec<T> {
        let mut inorder = Vec::new();
        self.internal_inorder(&mut inorder);
        inorder
    }

    pub fn preorder(&self) -> Vec<T> {
        let mut preorder = Vec::new();
        self.internal_preorder(&mut preorder);
        preorder
    }

    pub fn postorder(&self) -> Vec<T> {
        let mut postorder = Vec::new();
        self.internal_postorder(&mut postorder);
        postorder
    }

    fn internal_inorder(&self, out: &mut Vec<T>) {
        if let Some(ref left) = self.left {
            left.internal_inorder(out);
        }
        out.push(self.value.clone());
        if let Some(ref right) = self.right {
            right.internal_inorder(out);
        }
    }

    fn internal_preorder(&self, out: &mut Vec<T>) {
        out.push(self.value.clone());
        if let Some(ref left) = self.left {
            left.internal_preorder(out);
        }
        if let Some(ref right) = self.right {
            right.internal_preorder(out);
        }
    }

    fn internal_postorder(&self, out: &mut Vec<T>) {
        if let Some(ref left) = self.left {
            left.internal_postorder(out);
        }
        if let Some(ref right) = self.right {
            right.internal_postorder(out);
        }
        out.push(self.value.clone());
    }
}

fn items_without<T: Clone>(items: &[T], index: usize) -> Vec<T> {
    items
        .iter()
        .enumerate()
        .filter(|(i, _)| *i != index)
        .map(|(_, item)| item.clone())
        .collect()
}

fn split_permutations<T: Clone>(items: &[T]) -> Vec<(Vec<T>, Vec<T>)> {
    if items.len() == 1 {
        return vec![
            (vec![items[0].clone()], Vec::new()),
            (Vec::new(), vec![items[0].clone()]),
        ];
    }

    let mut permutations = Vec::new();

    let main_item = &items[0];
    let others = &items[1..];
    for perm in split_permutations(others) {
        let mut left_perm = perm.clone();
        left_perm.0.push(main_item.clone());
        permutations.push(left_perm);

        let mut right_perm = perm;
        right_perm.1.push(main_item.clone());
        permutations.push(right_perm);
    }

    permutations
}

impl<T: Debug> Debug for Tree<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match (&self.left, &self.right) {
            (Some(left), Some(right)) => {
                write!(f, "{:?} {{ L {:?} R {:?} }}", self.value, left, right)
            }
            (Some(left), None) => {
                write!(f, "{:?} {{ L {:?} }}", self.value, left)
            }
            (None, Some(right)) => {
                write!(f, "{:?} {{ R {:?} }}", self.value, right)
            }
            (None, None) => {
                write!(f, "{:?}", self.value)
            }
        }
    }
}
