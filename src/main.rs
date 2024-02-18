struct Task {
    id: usize,
    description: String,
    completed: bool,
}

impl Task {
    fn new(id: usize, description: String) -> Task {
        Task {
            id,
            description,
            completed: false,
        }
    }

    fn complete(&mut self) {
        self.completed = true;
    }
}

struct TodoList {
    tasks: Vec<Task>,
}

impl TodoList {
    fn new() -> TodoList {
        TodoList { tasks: Vec::new() }
    }

    fn add_task(&mut self, description: String) -> usize {
        let id = self.tasks.len() + 1;
        let task = Task::new(id, description);
        self.tasks.push(task);
        id
    }

    fn complete_task(&mut self, id: usize) -> Option<&Task> {
        for task in &mut self.tasks {
            if task.id == id {
                task.complete();
                return Some(task);
            }
        }
        None
    }

    fn list_tasks(&self) {
        for task in &self.tasks {
            println!("ID: {}, Description: {}, Completed: {}", task.id, task.description, task.completed);
        }
    }
}

fn main() {
    let mut todo_list = TodoList::new();

    let _task1_id = todo_list.add_task(String::from("Task 1"));
    let _task2_id = todo_list.add_task(String::from("Task 2"));
    let _task3_id = todo_list.add_task(String::from("Task 3"));

    todo_list.complete_task(_task2_id);

    todo_list.list_tasks();
}
