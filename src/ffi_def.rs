

pub mod ffi_def{
    
    use our_fractal_core::Type;

    #[repr(C)]
    pub struct FFIDef {
        /// tag value.  
        /// The first 4 digits : the group number  
        /// The last 4 digits  : the element number.
        pub tag: u32,
        /// value type
        pub data_type: Type,
        /// is multiple.
        pub is_multiple: bool,
    }

    impl FFIDef{
        pub fn new(tag: u32, data_type: Type, is_multiple: bool) -> FFIDef {
            FFIDef {
                tag: tag,
                data_type: data_type,
                is_multiple: is_multiple,
            }
        }
    }
}