pub trait MemoryView {
    fn read(&self, memory: &[u8], address: u16) -> u8;
    fn write(&mut self, memory: &mut [u8], address: u16, value: u8);
}