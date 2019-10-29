extern crate intercom;
extern crate log;
extern crate simple_logger;

use submodule::ISubInterface;

#[intercom::com_class(IInterface, ISubInterface)]
struct S;

#[intercom::com_interface]
trait IInterface
{
    fn call(&self);
}

#[intercom::com_interface]
trait IAnotherInterface
{
}

#[intercom::com_impl]
impl IInterface for S
{
    fn call(&self)
    {
        println!("Call");
    }
}

mod submodule
{
    use super::*;

    #[intercom::com_interface]
    pub trait ISubInterface
    {
    }

    #[intercom::com_impl]
    impl ISubInterface for S {}
}

fn main()
{
    simple_logger::init().unwrap();

    log::info!("Acquire S as IInterface");
    let combox = intercom::ComBox::new(S);
    let rc: intercom::ComRc<dyn IInterface> = intercom::ComRc::from(combox);

    log::info!("Call IInterface::call");
    rc.call();

    log::info!("Query ISubInterface");
    let _: intercom::ComRc<dyn submodule::ISubInterface> =
        intercom::ComItf::query_interface(&rc).unwrap();

    log::info!("Query IAnotherInterface");
    let rc: intercom::ComResult<intercom::ComRc<dyn IAnotherInterface>> =
        intercom::ComItf::query_interface(&rc);

    log::info!("Cleanup");
}
