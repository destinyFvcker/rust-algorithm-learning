struct MinStack {
    stack: Vec<i32>,
    min_stack: Vec<i32>,
}

impl MinStack {
    fn new() -> Self {
        Self {
            stack: Vec::new(),
            min_stack: Vec::new(),
        }
    }

    fn push(&mut self, val: i32) {
        self.stack.push(val);
        // 如果最小栈为空，或者当前值小于等于最小栈的栈顶元素，推入最小栈
        if self.min_stack.is_empty() || val <= *self.min_stack.last().unwrap() {
            self.min_stack.push(val);
        }
    }

    fn pop(&mut self) {
        // 如果弹出的元素等于最小栈的栈顶元素，最小栈也需要弹出
        if let Some(val) = self.stack.pop() {
            if val == *self.min_stack.last().unwrap() {
                self.min_stack.pop();
            }
        }
    }

    fn top(&self) -> i32 {
        *self.stack.last().unwrap()
    }

    fn get_min(&self) -> i32 {
        *self.min_stack.last().unwrap()
    }
}
