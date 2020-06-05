/// Assignment 3: Build a binary tree and then put the data in sorted, and then pull it out in order.
use std::vec::Vec;

pub enum BinaryTree<T> {
    Tail,
    Head(T, Box<BinaryTree<T>>, Box<BinaryTree<T>>)
}

use self::BinaryTree::*;

impl<T> BinaryTree<T> {
    pub fn new_empty() -> Self {
        Tail
    }

    pub fn new(v:T) -> Self {
        Head(v, Box::new(Tail), Box::new(Tail))
    }

    pub fn convert_to_vec(self) -> Vec<T> {
        match self {
            Tail => return vec![],
            Head(v,l,r) => return BinaryTree::concatinate((*l).convert_to_vec(), v, (*r).convert_to_vec())
        }
    }

    fn concatinate(lhs:Vec<T>, v:T, rhs:Vec<T>) -> Vec<T> {
        vec![v]
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_empty_tree_when_convert_to_vec_then_returns_empty_vector() {
        // Arrange
        let bt:BinaryTree<i32> = BinaryTree::new_empty();

        // Act
        let vec = bt.convert_to_vec();

        // Assert
        assert_eq!(vec.len(), 0);
    }

    #[test]
    fn given_binary_tree_when_convert_to_vec_then_returns_single_element() {
        // Arrange
        let bt = BinaryTree::new(5);

        // Act
        let vec = bt.convert_to_vec();

        // Assert
        assert_eq!(vec, vec![5]);

    }
}
