
use cls1::Class1;
use cls2::Class2;

#[com_library( "00000001-0000-0000-0000-000000000000", Class1, Class2 )]

mod itfs {

    #[com_interface( "00000002-0000-0000-0000-000000000000")]
    trait Interface1 {}

    #[com_interface( "00000003-0000-0000-0000-000000000000")]
    trait Interface2 {}
}

mod cls1 {

    #[com_class( "00000004-0000-0000-0000-000000000000", Class1)]
    pub struct Class1;

    #[com_interface( "00000006-0000-0000-0000-000000000000")]
    #[com_impl]
    impl Class1 {}
}

mod cls2 {

    use super::itfs;

    #[com_class( "00000005-0000-0000-0000-000000000000", Interface1, Interface2)]
    struct Class2;

    #[com_impl]
    impl itfs::Interface1 for Class2 {}

    #[com_impl]
    impl itfs::Interface2 for Class2 {}
}

mod no_guid {

    use super::itfs;

    #[com_class( NO_GUID, Interface1, Interface2)]
    #[derive(Debug)]
    pub struct NoGuid
    {
        test: String
    }

    #[com_impl]
    impl itfs::Interface1 for NoGuid {}

    #[com_impl]
    impl itfs::Interface2 for NoGuid {}
}
