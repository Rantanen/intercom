extern crate intercom;

mod interface {

    #[intercom::com_interface]
    pub trait MyInterface {
        fn interface_method(&self) -> u32;
    }
}

mod class {
    #[intercom::com_class(MyStruct, crate::interface::MyInterface)]
    pub struct MyStruct;
}

mod interface_impl {
    #[intercom::com_impl]
    impl crate::interface::MyInterface for crate::class::MyStruct {
        fn interface_method(&self) -> u32 { 0 }
    }
}

mod class_impl {
    #[intercom::com_impl]
    #[intercom::com_interface]
    impl crate::class::MyStruct {
        pub fn new() -> Self { Self }

        fn struct_method(&self) -> u32 { 0 }
    }
}

intercom::com_library!(
    class::MyStruct
);
