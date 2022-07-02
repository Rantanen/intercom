extern crate intercom;
use intercom::*;

fn custom_load() -> ComResult<()>
{
    Ok(())
}

fn custom_register() -> ComResult<()>
{
    Ok(())
}

fn custom_unregister() -> ComResult<()>
{
    Ok(())
}

com_library!(
    libid = "00000000-0000-0000-0000-000000000000",
    on_load = custom_load,
    on_register = custom_register,
    on_unregister = custom_unregister,
);
