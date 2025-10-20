pub struct Queue<T> {
    elements: Vec<T>,
}

impl<T> Queue<T> {

    pub fn new() -> Queue<T> {
        Queue { elements: Vec::new() }
    }

    pub fn push(&mut self, element: T) {
        self.elements.push(element);
    }

    pub fn pop(&mut self) -> Result<T, &'static str> {
        if self.elements.is_empty() {
            Err("Queue is empty")
        } else {
            let value = self.elements.remove(0usize);
            Ok(value)
        }
    }

    pub fn peek(&self) -> Result<&T, &'static str> {
        match self.elements.first() {
            Some(element) => Ok(element),
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

impl <T> Default for Queue<T> {

    fn default() -> Queue<T> {
        Queue{ elements: Vec::new() }
    }

}