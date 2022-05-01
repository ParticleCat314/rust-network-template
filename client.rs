
pub mod client {
    use std::str;
    use std::net::TcpStream;
    use std::io::{self,prelude::*,BufReader,Write};


    pub fn run_client() -> io::Result<( )> {

        let mut stream = TcpStream::connect("127.0.0.1:314")?;

        for _ in 0..1000 {

            let mut input = String::new();

            io::stdin().read_line(&mut input).expect("Failed to read input stream");
            stream.write(input.as_bytes()).expect("Failed to write input");

            let mut reader = BufReader::new(&stream);
            let mut buffer: Vec<u8> = Vec::new();
            reader.read_until(b'\n', &mut buffer)?;

            println!("read from server: {}", str::from_utf8(&buffer).unwrap());
            println!("");

        }

        Ok(())
    }
}