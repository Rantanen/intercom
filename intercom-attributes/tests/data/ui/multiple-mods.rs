extern crate intercom;

mod interface
{

    #[intercom::com_interface]
    pub trait MyInterface
    {
        fn interface_method(&self) -> u32;
    }
}

mod class
{
    #[intercom::com_class(MyStruct, crate::interface::MyInterface)]
    #[derive(Default)]
    pub struct MyStruct;
}

mod interface_impl
{
    impl crate::interface::MyInterface for crate::class::MyStruct
    {
        fn interface_method(&self) -> u32
        {
            0
        }
    }
}

mod class_impl
{
    #[intercom::com_interface]
    impl crate::class::MyStruct
    {
        fn struct_method(&self) -> u32
        {
            0
        }
    }
}

mod submodule
{
    intercom::com_module!(
        class SubmoduleClass
    );

    #[intercom::com_class(Self)]
    #[derive(Default)]
    pub struct SubmoduleClass;

    #[intercom::com_interface]
    impl SubmoduleClass {}
}

intercom::com_library!(
    class class::MyStruct,
    module submodule,
);
