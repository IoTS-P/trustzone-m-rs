pub fn enable_idau_nsc_code() {
    unsafe {
        const NSCCFG_ADDR: u32 = 0x50080014u32;
        let nsc_cfg = NSCCFG_ADDR as *mut u32;
        // Allows SAU to define the code region as a NSC
        *nsc_cfg |= 1;
    }
}
