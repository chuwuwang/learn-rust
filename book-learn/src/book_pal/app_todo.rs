use std::collections::HashMap;

struct TodoApp {
    tasks: Vec<String>,
    status: HashMap<String, bool>,
}

impl TodoApp {

    fn new() -> Self {
        TodoApp {
            tasks: Vec::new(),
            status: HashMap::new(),
        }
    }

    fn add_task(&mut self, task: String) {
        let ctask = task.clone();
        self.tasks.push(ctask);
        self.status.insert(task, false);
    }

    fn complete_task(&mut self, task: &str) {
        if let Some(status) = self.status.get_mut(task) {
            *status = true;
        }
    }

    fn show_tasks(&self) {
        for (i, task) in self.tasks.iter().enumerate() {
            let done = if *self.status.get(task).unwrap_or(&false) {
                "✓"
            } else {
                " "
            };
            tracing::info!("{}. [{}] {}", i + 1, done, task);
        }
    }

}

pub fn todo_app_example() {
    let mut app = TodoApp::new();
    app.add_task( "Learn Rust".to_string() );
    app.add_task( "Build a web app".to_string() );
    app.show_tasks();
    app.complete_task("Learn Rust");
    app.show_tasks();
}