pub mod server {
    use std::io;
    use std::time;
    use std ::net::{TcpListener,TcpStream};
    use std::io::{Read,Write};
    use std::thread;

    pub fn sender_input(mut stream: TcpStream) -> io::Result<()> {
        let mut buf = [0;512];
        for _ in 0..100 {
            let bytes_read = stream.read(&mut buf)?;
            
            if bytes_read == 0 {
                return Ok(());
            }

            stream.write(&buf[..bytes_read])?;

            println!("from the sender: {}", String::from_utf8_lossy(&buf));

            thread::sleep(time::Duration::from_secs(1));

        }

        Ok(())
    }

    pub fn run_server() -> io::Result<()> {

        let reciever_listener = TcpListener::bind("127.0.0.1:314").expect("Failed to connect stuff idk");

        let mut thread_vec: Vec<thread::JoinHandle<()>> = Vec::new();

        for stream in reciever_listener.incoming() {
            let stream = stream.expect("failed");

            let handle = thread::spawn(move || {
                sender_input(stream).unwrap_or_else(|error| eprint!("{:?}", error))
            });

            thread_vec.push(handle);
        }

        for handle in thread_vec {
            handle.join().unwrap();
        }

        Ok(())
    }
    
}