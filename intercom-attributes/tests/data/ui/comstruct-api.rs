
extern crate intercom;

use intercom::{ComRc, ComItf, ComResult};

#[intercom::com_interface]
pub trait MyInterface {
    fn interface_method(&self) {}
}

#[intercom::com_class(MyStruct, MyInterface)]
pub struct MyStruct;

#[intercom::com_impl]
impl MyInterface for MyStruct {
    fn interface_method(&self) {}
}

#[intercom::com_interface]
#[intercom::com_impl]
impl MyStruct {
    fn struct_method(&self) {}
}

pub fn from_trait_itf(itf: &ComItf<dyn MyInterface>) {
    let _ : ComRc<dyn MyInterface> = ComRc::from(itf);
    let _ : ComRc<dyn MyInterface> = itf.to_owned();

    let _ : ComResult<ComRc<MyStruct>> = ComItf::query_interface(itf);
}

pub fn from_struct_itf(itf: &ComItf<MyStruct>) {
    let _ : ComRc<MyStruct> = ComRc::from(itf);
    let _ : ComRc<MyStruct> = itf.to_owned();

    let _ : ComResult<ComRc<dyn MyInterface>> = ComItf::query_interface(itf);
}

pub fn from_struct(
    s1: intercom::ComBox<MyStruct>,
    s2: intercom::ComBox<MyStruct>,
) {

    let _ : ComRc<dyn MyInterface> = ComRc::from(s1);
    let _ : ComRc<MyStruct> = ComRc::from(s2);
}

pub fn from_struct_ref(s: &intercom::ComBox<MyStruct>) {
    let _ : ComRc<dyn MyInterface> = ComRc::from(s);
    let _ : ComRc<MyStruct> = ComRc::from(s);
}

pub fn pass_comrc(rc: ComRc<dyn MyInterface>) {
    from_trait_itf(&rc);
}
