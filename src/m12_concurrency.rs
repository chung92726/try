

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};
    use std::fs::{File, OpenOptions};
    use std::io::prelude::*;
    use std::thread;
    


    #[test]
    #[allow(dead_code, unused_variables)]
    fn tests_concurrency() {
        let file_mutex: Arc<Mutex<File>> = Arc::new(Mutex::new(OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .append(true)
            .open("increments.txt")
            .unwrap()));
        let mut handles: Vec<thread::JoinHandle<()>> = vec![];

        for i in 0..10 {
            let file_mutex_clone = Arc::clone(&file_mutex);
            let handle = thread::spawn(move || {
                let mut file = file_mutex_clone.lock().unwrap();
                writeln!(file, "{}", i).unwrap()
            });
            handles.push(handle);
        }
        for handle in handles {
            handle.join().unwrap();
        }
    }


        

    
}