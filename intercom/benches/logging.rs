#![feature(test)]
extern crate test;

extern crate intercom;
extern crate simple_logger;

#[intercom::com_class(IInterface)]
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

#[bench]
fn run_with_logging(bencher: &mut test::Bencher)
{
    simple_logger::init().unwrap();

    bencher.iter(|| {
        let combox = test::black_box(intercom::ComBox::new(S));
        let rc: intercom::ComRc<dyn IInterface> = intercom::ComRc::from(combox);

        test::black_box(&rc).call();

        let rc: intercom::ComResult<intercom::ComRc<dyn IAnotherInterface>> =
            intercom::ComItf::query_interface(&rc);

        rc
    });
}

#[bench]
fn run_without_logging(bencher: &mut test::Bencher)
{
    bencher.iter(|| {
        let combox = test::black_box(intercom::ComBox::new(S));
        let rc: intercom::ComRc<dyn IInterface> = intercom::ComRc::from(combox);

        test::black_box(&rc).call();

        let rc: intercom::ComResult<intercom::ComRc<dyn IAnotherInterface>> =
            intercom::ComItf::query_interface(&rc);

        rc
    });
}
