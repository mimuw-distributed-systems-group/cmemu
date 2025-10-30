pub trait HwRegister: Clone + From<u32> {
    const RESERVED_BITS_MASK: u32;
    const READ_ONLY_BITS_MASK: u32;
    const WRITE_ONLY_BITS_MASK: u32;

    fn read(&self) -> u32;
    fn mutate(&mut self, val: u32);
    #[must_use]
    fn with_mutation(&self, val: u32) -> Self {
        let mut copy = self.clone();
        copy.mutate(val);
        copy
    }
    fn read_1st_byte(&self) -> u8 {
        (self.read() & 0xFF).try_into().unwrap()
    }
    fn read_2nd_byte(&self) -> u8 {
        ((self.read() & 0xFF_00) >> 8).try_into().unwrap()
    }
    fn read_3rd_byte(&self) -> u8 {
        ((self.read() & 0xFF_00_00) >> 16).try_into().unwrap()
    }
    fn read_4th_byte(&self) -> u8 {
        ((self.read() & 0xFF_00_00_00) >> 24).try_into().unwrap()
    }
    #[must_use]
    fn with_mutated_2nd_byte(&self, val: u8) -> Self {
        self.with_mutation(self.read() & !0xFF_00 | (u32::from(val) << 8))
    }
}
