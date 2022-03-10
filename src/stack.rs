use std::fmt;

/// A stack.
///
/// Supports only the most basic stack operations needed for the machine.
/// Implemending using a `Vec`.
///
/// ```
/// use stack_vm::Stack;
/// let mut stack: Stack<usize> = Stack::new();
/// assert!(stack.is_empty());
///
/// stack.push(13);
/// assert!(!stack.is_empty());
///
/// let value = stack.pop();
/// assert_eq!(value, 13);
/// ```
#[derive(Debug, Default)]
pub struct Stack<T>(Vec<T>);

impl<T: fmt::Debug> Stack<T> {
    /// Create a new empty `Stack` and return it.
    pub fn new() -> Stack<T> {
        Stack(vec![])
    }

    pub fn with_capacity(size: usize) -> Stack<T> {
        Stack(Vec::with_capacity(size))
    }


    /// Returns `true` if the stack contains no elements.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Push an element onto the top of the stack.
    pub fn push(&mut self, value: T) {
        self.0.push(value);
    }

    /// Pop the top element off the stack and return it.
    pub fn pop(&mut self) -> T {
        self.0.pop().expect("Unable to pop from empty stack!")
    }

    pub fn safe_pop(&mut self) -> Option<T> {
        self.0.pop()
    }

    /// Pop the top element off the stack and return it.
    pub fn get(&mut self, idx: usize) -> &T {
        self.0.get(idx).expect("Unable to get index from stack")
    }
    pub fn replace(&mut self, idx: usize, value: T)  {
        std::mem::replace(&mut self.0[idx], value);
    }


    /// Take a sneaky look at the top element on the stack.
    pub fn peek(&self, dist: usize) -> &T {
        let len = self.0.len();
        if len == 0 {
            panic!("Cannot peek into empty stack!")
        }
        &self.0[len - (1 +  dist)]
    }




    /// Make a sneaky change to the top element on the stack.
    pub fn peek_mut(&mut self) -> &mut T {
        let len = self.0.len();
        if len == 0 {
            panic!("Cannot peek into empty stack!")
        }
        &mut self.0[len - 1]
    }


    /// Make a sneaky change to the top element on the stack.
    pub fn reset_stack(&mut self)  {
        self.0.clear();
    }

    pub fn as_slice(&self) -> &[T] {
        self.0.as_slice()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new() {
        let stack: Stack<usize> = Stack::new();
        assert!(stack.is_empty());
    }

    #[test]
    fn push() {
        let mut stack: Stack<usize> = Stack::new();
        stack.push(13);
        assert!(!stack.is_empty());
    }

    #[test]
    fn pop() {
        let mut stack: Stack<usize> = Stack::new();
        stack.push(13);
        let value = stack.pop();
        assert_eq!(value, 13);
    }

    #[test]
    #[should_panic(expected = "empty stack")]
    fn empty_pop() {
        let mut stack: Stack<usize> = Stack::new();
        stack.pop();
    }

    #[test]
    fn peek() {
        let mut stack: Stack<usize> = Stack::new();
        stack.push(13);
        assert_eq!(*stack.peek(0), 13)
    }

    #[test]
    #[should_panic(expected = "empty stack")]
    fn empty_peek() {
        let stack: Stack<usize> = Stack::new();
        stack.peek(0);
    }
}
