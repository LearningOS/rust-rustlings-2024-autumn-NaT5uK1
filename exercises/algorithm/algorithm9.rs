use std::cmp::Ord;
use std::default::Default;

pub struct Heap<T>
where
    T: Default,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool, // Comparator to handle Min/Max heap behavior
}

impl<T> Heap<T>
where
    T: Default,
{
    // Create a new heap with the provided comparator function
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: vec![T::default()], // First element is unused for easier indexing
            comparator,
        }
    }

    // Get the current length of the heap
    pub fn len(&self) -> usize {
        self.count
    }

    // Check if the heap is empty
    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

    // Add a new element to the heap and maintain the heap property
    pub fn add(&mut self, value: T) {
        self.items.push(value);
        self.count += 1;
        self.heapify_up(self.count); // Restore heap property by bubbling up
    }

    // Swap two elements in the heap
    fn swap(&mut self, i: usize, j: usize) {
        self.items.swap(i, j);
    }

    // Restore heap property by moving the element at `idx` upward
    fn heapify_up(&mut self, mut idx: usize) {
        while idx > 1 {
            let parent = self.parent_idx(idx);
            if (self.comparator)(&self.items[idx], &self.items[parent]) {
                self.swap(idx, parent);
                idx = parent;
            } else {
                break;
            }
        }
    }

    // Restore heap property by moving the element at `idx` downward
    fn heapify_down(&mut self, mut idx: usize) {
        while self.children_present(idx) {
            let smallest = self.smallest_child_idx(idx);
            if (self.comparator)(&self.items[smallest], &self.items[idx]) {
                self.swap(idx, smallest);
                idx = smallest;
            } else {
                break;
            }
        }
    }

    // Get the parent index of a given index
    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    // Check if the current node has children
    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) <= self.count
    }

    // Get the index of the left child of a given index
    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }

    // Get the index of the right child of a given index
    fn right_child_idx(&self, idx: usize) -> usize {
        idx * 2 + 1
    }

    // Find the index of the smallest child for the current node
    fn smallest_child_idx(&self, idx: usize) -> usize {
        let left = self.left_child_idx(idx);
        let right = self.right_child_idx(idx);

        if right <= self.count && (self.comparator)(&self.items[right], &self.items[left]) {
            right
        } else {
            left
        }
    }
}

impl<T> Heap<T>
where
    T: Default + Ord,
{
    // Create a MinHeap
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    // Create a MaxHeap
    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

impl<T> Iterator for Heap<T>
where
    T: Default,
{
    type Item = T;

    // Get the next element from the heap (remove the top element)
    fn next(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        self.swap(1, self.count); // Move the top element to the end
        let result = self.items.pop(); // Remove the last element (original top)
        self.count -= 1;

        if !self.is_empty() {
            self.heapify_down(1); // Restore heap property
        }

        result
    }
}

pub struct MinHeap;

impl MinHeap {
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new_min()
    }
}

pub struct MaxHeap;

impl MaxHeap {
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new_max()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_heap() {
        let mut heap = MaxHeap::new::<i32>();
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = MinHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);

        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));

        heap.add(1);
        assert_eq!(heap.next(), Some(1));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);

        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));

        heap.add(1);
        assert_eq!(heap.next(), Some(2));
    }
}
