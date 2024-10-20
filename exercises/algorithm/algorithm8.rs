#[derive(Debug)]
pub struct Queue<T> {
    elements: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }

    pub fn enqueue(&mut self, value: T) {
        self.elements.push(value);
    }

    pub fn dequeue(&mut self) -> Result<T, &str> {
        if !self.elements.is_empty() {
            Ok(self.elements.remove(0)) // Remove the first element
        } else {
            Err("Queue is empty")
        }
    }

    pub fn peek(&self) -> Result<&T, &str> {
        match self.elements.first() {
            Some(value) => Ok(value),
            None => Err("Queue is empty"),
        }
    }

    pub fn size(&self) -> usize {
        self.elements.len()
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}

impl<T> Default for Queue<T> {
    fn default() -> Queue<T> {
        Queue::new()
    }
}

pub struct MyStack<T> {
    q1: Queue<T>,
    q2: Queue<T>,
}

impl<T> MyStack<T> {
    pub fn new() -> Self {
        Self {
            q1: Queue::new(),
            q2: Queue::new(),
        }
    }

    // Push element onto the stack
    pub fn push(&mut self, elem: T) {
        self.q1.enqueue(elem);
    }

    // Pop the top element from the stack
    pub fn pop(&mut self) -> Result<T, &str> {
        if self.q1.is_empty() {
            return Err("Stack is empty");
        }

        // Move elements from q1 to q2, leaving only the last element in q1
        while self.q1.size() > 1 {
            let elem = self.q1.dequeue().unwrap();
            self.q2.enqueue(elem);
        }

        // Pop the last element from q1 (which is the "top" of the stack)
        let top = self.q1.dequeue().unwrap();

        // Swap q1 and q2 so q1 is ready for further operations
        std::mem::swap(&mut self.q1, &mut self.q2);

        Ok(top)
    }

    // Check if the stack is empty
    pub fn is_empty(&self) -> bool {
        self.q1.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_queue() {
        let mut s = MyStack::<i32>::new();

        // Test popping from an empty stack
        assert_eq!(s.pop(), Err("Stack is empty"));

        // Push elements
        s.push(1);
        s.push(2);
        s.push(3);

        // Pop elements and check if they follow LIFO order
        assert_eq!(s.pop(), Ok(3));
        assert_eq!(s.pop(), Ok(2));

        // Push more elements
        s.push(4);
        s.push(5);

        // Check if the stack is not empty
        assert_eq!(s.is_empty(), false);

        // Continue popping elements
        assert_eq!(s.pop(), Ok(5));
        assert_eq!(s.pop(), Ok(4));
        assert_eq!(s.pop(), Ok(1));

        // Ensure stack is now empty
        assert_eq!(s.pop(), Err("Stack is empty"));
        assert_eq!(s.is_empty(), true);
    }
}
