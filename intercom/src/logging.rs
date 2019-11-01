#[inline(always)]
#[allow(unused_variables)]
pub fn trace<FArgs>(f: FArgs)
where
    FArgs: FnOnce(fn(&str, std::fmt::Arguments<'_>)),
{
    if log::log_enabled!(log::Level::Trace) {
        f(|module, args| {
            log::logger().log(
                &log::Record::builder()
                    .level(log::Level::Trace)
                    .target(&format!("generated::{}", module))
                    .module_path(Some(module))
                    .args(args)
                    .build(),
            )
        });
    }
}

#[inline(always)]
#[allow(unused_variables)]
pub fn error<FArgs>(f: FArgs)
where
    FArgs: FnOnce(fn(&str, std::fmt::Arguments<'_>)),
{
    if log::log_enabled!(log::Level::Error) {
        f(|module, args| {
            log::logger().log(
                &log::Record::builder()
                    .args(args)
                    .level(log::Level::Error)
                    .target(&format!("generated::{}", module))
                    .module_path(Some(module))
                    .build(),
            )
        });
    }
}
