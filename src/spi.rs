#[derive(Debug)]
pub enum Error {
    ConfigFault,
    RecvOverrun,
    Format,
    Crc,
}
