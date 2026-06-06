fn main() {
    println!("Hello, World!");
}

enum HistoryCommands {
    Pop,
    Push,
    Undo,
}

struct Node<T> {
    value: T,
    commnad: HistoryCommands,
    next: Option<Box<Node<T>>>,
}

struct Stack<T> {
    stack: Vec<T>,
    history: Vec<Node<T>>,
    history_count: u32,
}

impl Stack<T> {
    fn push(T data, &mut self) {
        
    }
}