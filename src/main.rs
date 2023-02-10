use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

#[derive(Clone, Debug)]
struct Interrupt {
    id: u32,
    priority: u32,
}

struct InterruptHandler {
    interrupts: Arc<Mutex<Vec<Interrupt>>>,
}

impl InterruptHandler {
    fn new() -> Self {
        InterruptHandler {
            interrupts: Arc::new(Mutex::new(Vec::new())),
        }
    }

    fn receive_interrupt(&self, interrupt: Interrupt) {
        let mut interrupts = self.interrupts.lock().unwrap();
        let interrupt_copy = interrupt.clone();
        interrupts.push(interrupt);
        println!(
            "Received interrupt with id: {} and priority: {}",
            interrupt_copy.id, interrupt_copy.priority
        );
        interrupts.sort_by_key(|interrupt| interrupt.priority);
        println!("Interrupts list after sorting: {:?}", &*interrupts);
    }

    fn handle_interrupts(&self) {
        let mut interrupts = self.interrupts.lock().unwrap();
        while !interrupts.is_empty() {
            let interrupt = interrupts.pop().unwrap();
            println!(
                "Handling interrupt with id: {} and priority: {}",
                interrupt.id, interrupt.priority
            );
            thread::sleep(Duration::from_millis(1000));
        }
    }
}

fn main() {
    let handler = InterruptHandler::new();
    let interrupt1 = Interrupt { id: 1, priority: 2 };
    let interrupt2 = Interrupt { id: 2, priority: 1 };
    let interrupt3 = Interrupt { id: 3, priority: 3 };

    let handler_thread = handler.interrupts.clone();
    thread::spawn(move || {
        handler_thread.lock().unwrap().push(interrupt1);
        handler_thread.lock().unwrap().push(interrupt2);
        handler_thread.lock().unwrap().push(interrupt3);
    });

    thread::sleep(Duration::from_millis(100));
    handler.handle_interrupts();
}
