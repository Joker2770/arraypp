// A `no_std` and no `alloc` library for more efficient array processing.
// Copyright (C) 2025  joker2770

// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation; either version 2 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License along
// with this program; if not, write to the Free Software Foundation, Inc.,
// 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA.

#[derive(Debug, Clone, Copy)]
struct TreeNode<T> {
    data: T,
    left: Option<usize>,
    right: Option<usize>,
}

pub struct ArrayBinaryTree<T, const N: usize> {
    nodes: [Option<TreeNode<T>>; N],
    root: Option<usize>,
    next_index: usize,
}

impl<T: Copy, const N: usize> ArrayBinaryTree<T, N> {
    /// Create a new empty tree
    pub fn new() -> Self {
        let nodes = [None; N];
        Self {
            nodes,
            root: None,
            next_index: 0,
        }
    }

    /// Insert root node
    pub fn insert_root(&mut self, data: T) -> Result<(), &'static str> {
        if self.root.is_some() {
            return Err("Root already exists");
        }
        self.insert_node(data, None, None)
            .map(|idx| self.root = Some(idx))
    }

    /// Insert left child node
    pub fn insert_left(&mut self, parent: usize, data: T) -> Result<usize, &'static str> {
        if self.nodes[parent].is_none() {
            return Err("Invalid parent index");
        }
        if self.nodes[parent].as_ref().unwrap().left.is_some() {
            return Err("Left child already exists");
        }
        self.insert_node(data, None, None).map(|idx| {
            self.nodes[parent].as_mut().unwrap().left = Some(idx);
            idx
        })
    }

    /// Insert right child node
    pub fn insert_right(&mut self, parent: usize, data: T) -> Result<usize, &'static str> {
        if self.nodes[parent].is_none() {
            return Err("Invalid parent index");
        }
        if self.nodes[parent].as_ref().unwrap().right.is_some() {
            return Err("Right child already exists");
        }
        self.insert_node(data, None, None).map(|idx| {
            self.nodes[parent].as_mut().unwrap().right = Some(idx);
            idx
        })
    }

    /// Internal method: insert a new node
    fn insert_node(
        &mut self,
        data: T,
        left: Option<usize>,
        right: Option<usize>,
    ) -> Result<usize, &'static str> {
        if self.next_index >= N {
            return Err("Tree is full");
        }
        let idx = self.next_index;
        self.nodes[idx] = Some(TreeNode { data, left, right });
        self.next_index += 1;
        Ok(idx)
    }

    /// Preorder traversal (iterative implementation)
    pub fn preorder<F: FnMut(&T)>(&self, mut visit: F) {
        let mut stack = [None; N];
        let mut sp = 0;

        if let Some(root_idx) = self.root {
            stack[sp] = Some(root_idx);
            sp += 1;
        }

        while sp > 0 {
            sp -= 1;
            let idx = stack[sp].unwrap();
            let node = self.nodes[idx].as_ref().unwrap();
            visit(&node.data);

            if let Some(right) = node.right {
                if sp < N {
                    stack[sp] = Some(right);
                    sp += 1;
                }
            }

            if let Some(left) = node.left {
                if sp < N {
                    stack[sp] = Some(left);
                    sp += 1;
                }
            }
        }
    }

    /// Inorder traversal (iterative implementation)
    pub fn inorder<F: FnMut(&T)>(&self, mut visit: F) {
        let mut stack = [None; N];
        let mut sp = 0;
        let mut current = self.root;

        while current.is_some() || sp > 0 {
            while let Some(idx) = current {
                if sp >= N {
                    break;
                }
                stack[sp] = Some(idx);
                sp += 1;
                current = self.nodes[idx].as_ref().unwrap().left;
            }

            if sp > 0 {
                sp -= 1;
                let idx = stack[sp].unwrap();
                let node = self.nodes[idx].as_ref().unwrap();
                visit(&node.data);
                current = node.right;
            }
        }
    }

    /// Postorder traversal (iterative implementation)
    pub fn postorder<F: FnMut(&T)>(&self, mut visit: F) {
        let mut stack = [None; N];
        let mut sp = 0;
        let mut last_visited = None;
        let mut current = self.root;

        while current.is_some() || sp > 0 {
            while let Some(idx) = current {
                if sp >= N {
                    break;
                }
                stack[sp] = Some(idx);
                sp += 1;
                current = self.nodes[idx].as_ref().unwrap().left;
            }

            if sp > 0 {
                let top_idx = stack[sp - 1].unwrap();
                let top_node = self.nodes[top_idx].as_ref().unwrap();

                if top_node.right.is_some() && top_node.right != last_visited {
                    current = top_node.right;
                } else {
                    sp -= 1;
                    visit(&top_node.data);
                    last_visited = Some(top_idx);
                }
            }
        }
    }

    /// Get the total number of nodes in the tree
    pub fn node_count(&self) -> usize {
        self.next_index
    }

    /// Calculate the maximum depth of the tree (recursive)
    pub fn depth(&self) -> usize {
        self.depth_helper(self.root)
    }

    /// Recursive helper function to calculate depth
    fn depth_helper(&self, node: Option<usize>) -> usize {
        match node {
            Some(idx) => {
                let node_ref = self.nodes[idx].as_ref().unwrap();
                let left_depth = self.depth_helper(node_ref.left);
                let right_depth = self.depth_helper(node_ref.right);
                1 + left_depth.max(right_depth)
            }
            None => 0,
        }
    }

    /// Iteratively calculate the maximum depth of the tree
    pub fn depth_iterative(&self) -> usize {
        if self.root.is_none() {
            return 0;
        }

        // Use two queues: current level and next level
        let mut current_queue = [None; N];
        let mut next_queue = [None; N];
        let mut depth = 0;
        let mut current_size;
        let mut next_size;

        // Initialize root node
        current_queue[0] = self.root;
        current_size = 1;

        while current_size > 0 {
            depth += 1;
            next_size = 0;

            // Process all nodes at the current level
            for i in 0..current_size {
                let idx = current_queue[i].unwrap();
                let node = self.nodes[idx].as_ref().unwrap();

                // Add child nodes to the next level queue
                if let Some(left) = node.left {
                    if next_size < N {
                        next_queue[next_size] = Some(left);
                        next_size += 1;
                    }
                }
                if let Some(right) = node.right {
                    if next_size < N {
                        next_queue[next_size] = Some(right);
                        next_size += 1;
                    }
                }
            }

            // Swap queues, prepare for the next level
            core::mem::swap(&mut current_queue, &mut next_queue);
            current_size = next_size;
        }

        depth
    }

    /// Get the root node index
    pub fn root_index(&self) -> Option<usize> {
        self.root
    }

    /// Get node data
    pub fn get_data(&self, index: usize) -> Option<&T> {
        self.nodes
            .get(index)
            .and_then(|node| node.as_ref().map(|n| &n.data))
    }
}

// Example tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tree_operations() {
        const MAX_SIZE: usize = 5;
        let mut tree = ArrayBinaryTree::<i32, MAX_SIZE>::new();

        tree.insert_root(1).unwrap();
        let root = tree.root_index().unwrap();
        let left = tree.insert_left(root, 2).unwrap();
        tree.insert_right(root, 3).unwrap();
        tree.insert_left(left, 4).unwrap();
        tree.insert_right(left, 5).unwrap();

        // Verify node data
        assert_eq!(tree.get_data(root), Some(&1));
        assert_eq!(tree.get_data(left), Some(&2));

        // Test preorder traversal
        let mut pre_result = [0; 5];
        let mut count = 0;
        tree.preorder(|&data| {
            pre_result[count] = data;
            count += 1;
        });
        assert_eq!(pre_result, [1, 2, 4, 5, 3]);

        // Test inorder traversal
        let mut in_result = [0; 5];
        count = 0;
        tree.inorder(|&data| {
            in_result[count] = data;
            count += 1;
        });
        assert_eq!(in_result, [4, 2, 5, 1, 3]);

        // Test postorder traversal
        let mut post_result = [0; 5];
        count = 0;
        tree.postorder(|&data| {
            post_result[count] = data;
            count += 1;
        });
        assert_eq!(post_result, [4, 5, 2, 3, 1]);

        // Test node count
        assert_eq!(tree.node_count(), 5);

        // Test depth
        assert_eq!(tree.depth(), 3);
        assert_eq!(tree.depth_iterative(), 3);
    }

    #[test]
    fn test_depth_calculations() {
        const MAX_SIZE: usize = 7;
        let mut tree = ArrayBinaryTree::<i32, MAX_SIZE>::new();

        // Depth of empty tree is 0
        assert_eq!(tree.depth(), 0);
        assert_eq!(tree.depth_iterative(), 0);
        assert_eq!(tree.node_count(), 0);

        // Only root node
        tree.insert_root(1).unwrap();
        assert_eq!(tree.depth(), 1);
        assert_eq!(tree.depth_iterative(), 1);
        assert_eq!(tree.node_count(), 1);

        // Add left subtree
        let root = tree.root_index().unwrap();
        tree.insert_left(root, 2).unwrap();
        assert_eq!(tree.depth(), 2);
        assert_eq!(tree.depth_iterative(), 2);
        assert_eq!(tree.node_count(), 2);

        // Add right subtree
        tree.insert_right(root, 3).unwrap();
        assert_eq!(tree.depth(), 2);
        assert_eq!(tree.depth_iterative(), 2);
        assert_eq!(tree.node_count(), 3);

        // Add deeper level
        let left = 1; // Left child index is 1
        tree.insert_left(left, 4).unwrap();
        assert_eq!(tree.depth(), 3);
        assert_eq!(tree.depth_iterative(), 3);
        assert_eq!(tree.node_count(), 4);

        tree.insert_right(left, 5).unwrap();
        assert_eq!(tree.depth(), 3);
        assert_eq!(tree.depth_iterative(), 3);
        assert_eq!(tree.node_count(), 5);

        // Add fourth level
        tree.insert_left(3, 6).unwrap(); // Node 4's index is 3
        assert_eq!(tree.depth(), 4);
        assert_eq!(tree.depth_iterative(), 4);
        assert_eq!(tree.node_count(), 6);
    }

    #[test]
    fn test_unbalanced_tree() {
        const MAX_SIZE: usize = 10;
        let mut tree = ArrayBinaryTree::<i32, MAX_SIZE>::new();

        tree.insert_root(1).unwrap();
        let root = tree.root_index().unwrap();

        // Create right-skewed tree
        let mut current = root;
        for i in 2..=5 {
            tree.insert_right(current, i).unwrap();
            current = tree.node_count() - 1; // Index of newly inserted node
        }

        assert_eq!(tree.node_count(), 5);
        assert_eq!(tree.depth(), 5);
        assert_eq!(tree.depth_iterative(), 5);

        // Add left subtree
        let root = tree.root_index().unwrap();
        tree.insert_left(root, 10).unwrap();
        assert_eq!(tree.depth(), 5); // Depth unchanged
        assert_eq!(tree.node_count(), 6);
    }

    #[test]
    fn test_full_tree() {
        const MAX_SIZE: usize = 3;
        let mut tree = ArrayBinaryTree::<i32, MAX_SIZE>::new();

        tree.insert_root(1).unwrap();
        let root = tree.root_index().unwrap();
        tree.insert_left(root, 2).unwrap();
        tree.insert_right(root, 3).unwrap();

        assert_eq!(tree.node_count(), 3);
        assert_eq!(tree.depth(), 2);
        assert_eq!(tree.depth_iterative(), 2);

        // Tree should be full
        // Try to insert into a used-up space without left child, should return "Tree is full"
        assert_eq!(tree.insert_left(1, 4), Err("Tree is full"));
    }

    #[test]
    fn test_single_node() {
        const MAX_SIZE: usize = 1;
        let mut tree = ArrayBinaryTree::<i32, MAX_SIZE>::new();

        tree.insert_root(1).unwrap();

        let mut pre_result = [0; 1];
        let mut count = 0;
        tree.preorder(|&data| {
            pre_result[count] = data;
            count += 1;
        });
        assert_eq!(pre_result, [1]);

        let mut in_result = [0; 1];
        count = 0;
        tree.inorder(|&data| {
            in_result[count] = data;
            count += 1;
        });
        assert_eq!(in_result, [1]);

        let mut post_result = [0; 1];
        count = 0;
        tree.postorder(|&data| {
            post_result[count] = data;
            count += 1;
        });
        assert_eq!(post_result, [1]);
    }

    #[test]
    fn test_empty_tree() {
        const MAX_SIZE: usize = 3;
        let tree = ArrayBinaryTree::<i32, MAX_SIZE>::new();

        let mut visited = false;
        tree.preorder(|_| visited = true);
        assert!(!visited);

        tree.inorder(|_| visited = true);
        assert!(!visited);

        tree.postorder(|_| visited = true);
        assert!(!visited);
    }
}
