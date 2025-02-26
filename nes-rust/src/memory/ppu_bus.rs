struct PPUBus {
    memory: Box<[u8]>,
    last_read_value: Cell<u8>,
    cycle_counter: Cell<u8>,
}

impl Bus for PPUBus {
    
}