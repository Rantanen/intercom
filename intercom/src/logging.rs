#[inline(always)]
#[allow(unused_variables)]
pub fn trace(args: std::fmt::Arguments<'_>)
{
    #[cfg(feature = "log")]
    log::logger().log(
        &log::Record::builder()
            .args(args)
            .level(log::Level::Trace)
            .target("intercom")
            .build(),
    );
}

#[inline(always)]
#[allow(unused_variables)]
pub fn error(args: std::fmt::Arguments<'_>)
{
    #[cfg(feature = "log")]
    log::logger().log(
        &log::Record::builder()
            .args(args)
            .level(log::Level::Error)
            .target("intercom")
            .build(),
    );
}
