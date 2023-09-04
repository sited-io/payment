pub mod peoplesmarkets {
    pub const FILE_DESCRIPTOR_SET: &[u8] =
        include_bytes!("./FILE_DESCRIPTOR_SET");

    pub mod payments {
        pub mod v1 {
            include!("peoplesmarkets.payments.v1.rs");
        }
    }
}
