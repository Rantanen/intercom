#![feature(test)]
extern crate test;

extern crate intercom;
extern crate simple_logger;

#[intercom::com_class(IInterface)]
struct S;

#[intercom::com_interface]
trait IInterface
{
    fn call(&self) -> intercom::ComResult<String>;
}

#[intercom::com_interface]
trait IAnotherInterface
{
}

impl IInterface for S
{
    fn call(&self) -> intercom::ComResult<String>
    {
        Ok("result".to_string())
    }
}

#[bench]
fn run_with_logging(bencher: &mut test::Bencher)
{
    simple_logger::init().unwrap();
    log::set_max_level(log::LevelFilter::Trace);

    bencher.iter(|| {
        let combox = test::black_box(intercom::ComBox::new(S));
        let rc: intercom::ComRc<dyn IInterface> = intercom::ComRc::from(combox);

        assert_eq!(&test::black_box(&rc).call().unwrap(), "result");

        let rc: intercom::ComResult<intercom::ComRc<dyn IAnotherInterface>> =
            intercom::ComItf::query_interface(&rc);

        rc
    });
}

#[bench]
fn run_without_logging(bencher: &mut test::Bencher)
{
    log::set_max_level(log::LevelFilter::Off);

    bencher.iter(|| {
        let combox = test::black_box(intercom::ComBox::new(S));
        let rc: intercom::ComRc<dyn IInterface> = intercom::ComRc::from(combox);

        assert_eq!(&test::black_box(&rc).call().unwrap(), "result");

        let rc: intercom::ComResult<intercom::ComRc<dyn IAnotherInterface>> =
            intercom::ComItf::query_interface(&rc);

        rc
    });
}
