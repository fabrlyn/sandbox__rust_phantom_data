use std::fs::read_to_string;
use std::fs::write;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

pub struct Reader {
    path: PathBuf,
    handle: Option<thread::JoinHandle<()>>,
    keep_reading: Arc<Mutex<bool>>,
}

pub struct Writer {
    path: PathBuf,
}

impl Writer {
    fn write(&self, content: String) {
        write(&self.path, content).unwrap();
    }
}

impl Drop for Reader {
    fn drop(&mut self) {
        println!("Dropping Reader");
        {
            let mut keep_reading = self.keep_reading.lock().unwrap();
            *keep_reading = false;
        }

        if let Some(handle) = self.handle.take() {
            if let Err(e) = handle.join() {
                println!("Failed to wait for thread: {:?}", e);
            }
        }
    }
}

impl Reader {
    pub fn new(path: PathBuf) -> Reader {
        Reader {
            path,
            handle: None,
            keep_reading: Arc::new(Mutex::new(true)),
        }
    }

    pub fn create_writer(&self) -> Writer {
        Writer {
            path: self.path.clone(),
        }
    }

    pub fn start(&mut self) {
        let keep_reading = self.keep_reading.clone();
        let path = self.path.clone();

        let handle = thread::spawn(move || {
            while *keep_reading.lock().unwrap() {
                thread::sleep(Duration::from_secs(1));
                let content = read_to_string(&path);
                println!("Content is: {:?}", content);
            }
        });
        self.handle = Some(handle)
    }
}

fn main() {
    let path = PathBuf::from("./file-to-write-to.txt");
    let mut writer = None;
    {
        let mut reader = Reader::new(path);
        reader.start();
        writer = Some(reader.create_writer());
    }

    if let Some(writer) = writer {
        println!("Writing first value");
        writer.write("Hello".to_string());
        thread::sleep(Duration::from_millis(1500));

        println!("Writing second value");
        writer.write("Good bye".to_string());
        thread::sleep(Duration::from_millis(1500));
    }
}
