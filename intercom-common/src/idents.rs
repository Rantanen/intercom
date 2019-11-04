use crate::prelude::*;
use crate::tyhandlers::ModelTypeSystem;
use syn::*;

pub trait SomeIdent
{
    fn get_some_ident(&self) -> Option<Ident>;
}

impl SomeIdent for Path
{
    fn get_some_ident(&self) -> Option<Ident>
    {
        self.get_ident()
            .cloned()
            .or_else(|| self.segments.last().map(|l| l.ident.clone()))
    }
}

pub fn with_ts(ident: &Ident, ts: ModelTypeSystem) -> Ident
{
    Ident::new(&format!("{}_{:?}", ident, ts), Span::call_site())
}

pub fn clsid_path(struct_path: &Path) -> Path
{
    let mut clsid_path = struct_path.clone();
    if let Some(mut last) = clsid_path.segments.last_mut() {
        last.ident = clsid(&last.ident);
    }
    clsid_path
}

pub fn clsid(struct_name: &Ident) -> Ident
{
    new_ident(&format!("CLSID_{}", struct_name))
}

pub fn iid(itf_name: &Ident, span: Span) -> Ident
{
    Ident::new(&format!("IID_{}", itf_name), span)
}

pub fn method_impl<TMethod: std::fmt::Display>(
    struct_ident: &Ident,
    itf_ident: &Ident,
    method_name: TMethod,
    ts: ModelTypeSystem,
) -> Ident
{
    new_ident(&format!(
        "__{}_{}_{}_{:?}",
        struct_ident, itf_ident, method_name, ts
    ))
}

fn new_ident(s: &str) -> Ident
{
    Ident::new(s, Span::call_site())
}
