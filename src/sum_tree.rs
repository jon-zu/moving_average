use core::ops::Add;

/// N must be sum_tree_size(n), where n is the window size
pub struct SumTree<Sample, const N: usize,> {
	// TODO: Convert this to an array and use it as SumTreeSMA's main data storage, once
	// https://github.com/rust-lang/rust/issues/76560 is stable
	nodes: [Sample; N],
}

enum Position {
	Left,
	Right,
}

const ROOT_NODE_IDX: usize = 1;

impl<Sample, const N: usize> SumTree<Sample, N>
where
	Sample: Copy + Add<Output = Sample>,
{
	pub fn get_root_sum(&self) -> Sample {
		self.nodes[ROOT_NODE_IDX]
	}

	pub fn get_leaf_node_sum(&self, leaf_node_idx: &usize) -> Sample {
		self.nodes[self.get_leaf_nodes_offset() + leaf_node_idx]
	}

	pub fn update_leaf_node_sample(&mut self, leaf_node_idx: usize, new_sample: Sample) {
		let node_idx = self.get_leaf_nodes_offset() + leaf_node_idx;
		*self.get_node_mut(node_idx) = new_sample;
		self.update_parent_recursive(node_idx, new_sample);
	}

	fn update_parent_recursive(&mut self, child_node_idx: usize, new_child_subtree_sum: Sample) {
		let node_idx = get_parent_node_idx(child_node_idx);

		let other_child_subtree_sum = match get_position(child_node_idx) {
			Position::Left => *self.get_node(get_right_child_node_idx(node_idx)),
			Position::Right => *self.get_node(get_left_child_node_idx(node_idx)),
		};

		let node = self.get_node_mut(node_idx);
		let new_subtree_sum = new_child_subtree_sum + other_child_subtree_sum;
		*node = new_subtree_sum;

		if node_idx != ROOT_NODE_IDX {
			self.update_parent_recursive(node_idx, new_subtree_sum)
		}
	}

	fn get_node(&mut self, node_idx: usize) -> &Sample {
		self.get_node_mut(node_idx)
	}

	fn get_node_mut(&mut self, node_idx: usize) -> &mut Sample {
		&mut self.nodes[node_idx]
	}

	fn get_leaf_nodes_offset(&self) -> usize {
		self.nodes.len() / 2
	}

	pub fn get_leaf_nodes(&self, num_nodes: usize) -> &[Sample] {
		let leaf_nodes_start = self.get_leaf_nodes_offset();
		let leaf_nodes_end = leaf_nodes_start + num_nodes;
		&self.nodes[leaf_nodes_start..leaf_nodes_end]
	}
}

impl<Sample, const N: usize,> SumTree<Sample, N>
where
	Sample: Copy,
{
	pub fn new(zero: Sample) -> Self {
		// Let's create a perfect binary tree, large enough to accomodate all leaf nodes.
		// The extra nodes will contain only zeros, which is alright for our purposes.
		//let num_leaf_nodes = 2 * num_leaf_nodes.checked_next_power_of_two().unwrap();
		Self {
			nodes: [zero; N],
		}
	}
}

fn get_position(node_idx: usize) -> Position {
	if node_idx % 2 == 0 {
		Position::Left
	} else {
		Position::Right
	}
}

fn get_parent_node_idx(node_idx: usize) -> usize {
	node_idx / 2
}

fn get_left_child_node_idx(node_idx: usize) -> usize {
	2 * node_idx
}

fn get_right_child_node_idx(node_idx: usize) -> usize {
	2 * node_idx + 1
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn basics() {


		const N: usize = crate::sum_tree_size(6);
		let mut sum_tree = SumTree::<i32, N>::new(0);

		// Insert new nodes

		sum_tree.update_leaf_node_sample(0, 1);
		assert_eq!(sum_tree.get_root_sum(), 1);

		sum_tree.update_leaf_node_sample(1, 2);
		assert_eq!(sum_tree.get_root_sum(), 3);

		sum_tree.update_leaf_node_sample(2, 3);
		assert_eq!(sum_tree.get_root_sum(), 6);

		sum_tree.update_leaf_node_sample(3, 4);
		assert_eq!(sum_tree.get_root_sum(), 10);

		sum_tree.update_leaf_node_sample(4, 5);
		assert_eq!(sum_tree.get_root_sum(), 15);

		sum_tree.update_leaf_node_sample(5, 6);
		assert_eq!(sum_tree.get_root_sum(), 21);

		// Update exisitng nodes

		sum_tree.update_leaf_node_sample(0, 7); // 1 -> 7
		assert_eq!(sum_tree.get_root_sum(), 27);

		sum_tree.update_leaf_node_sample(1, 8); // 2 -> 8
		assert_eq!(sum_tree.get_root_sum(), 33);
	}
}
