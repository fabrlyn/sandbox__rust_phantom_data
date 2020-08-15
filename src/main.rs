use std::fs::read_to_string;
use std::fs::write;
use std::marker::PhantomData;
use std::path::PathBuf;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

pub struct Reader {
    path: PathBuf,
    handle: Option<thread::JoinHandle<()>>,
    keep_reading: Arc<AtomicBool>,
}

pub struct Writer<'a> {
    path: PathBuf,
    _phantom_data: PhantomData<&'a ()>,
}

impl Writer<'_> {
    fn write(&self, content: String) {
        write(&self.path, content).unwrap();
    }
}

impl Drop for Reader {
    fn drop(&mut self) {
        println!("Dropping Reader");
        self.keep_reading.store(false, Ordering::Relaxed);

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
            keep_reading: Arc::new(AtomicBool::new(true)),
        }
    }

    pub fn create_writer(&self) -> Writer {
        Writer {
            path: self.path.clone(),
            _phantom_data: PhantomData,
        }
    }

    pub fn start(&mut self) {
        let keep_reading = self.keep_reading.clone();
        let path = self.path.clone();

        let handle = thread::spawn(move || loop {
            thread::sleep(Duration::from_millis(100));
            let content = read_to_string(&path);
            println!("Content is: {:?}", content);

            if !keep_reading.load(Ordering::Relaxed) {
                break;
            }
        });
        self.handle = Some(handle)
    }
}

fn main() {
    let path = PathBuf::from("./file-to-write-to.txt");

    let mut writer = None;
    let mut reader = Reader::new(path);
    reader.start();
    writer = Some(reader.create_writer());

    if let Some(writer) = writer {
        println!("Writing first value");
        writer.write("First Value".to_string());
        thread::sleep(Duration::from_millis(400));

        println!("Writing second value");
        writer.write("Second Value".to_string());
        thread::sleep(Duration::from_millis(400));
    }
}
