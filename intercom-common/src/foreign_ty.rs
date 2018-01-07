use model::ComCrate;
use syn;

pub trait ForeignTyHandler
{
    /// Gets the name for the 'ty'.
    fn get_name( &self, krate : &ComCrate, ty : &syn::Ident ) -> String;

    /// Gets the COM type for a Rust type.
    fn get_ty( &self, krate : &ComCrate, ty : &syn::Ty ) -> Option< String >;
}

pub struct CTyHandler;

impl ForeignTyHandler for CTyHandler
{
    /// Tries to apply renaming to the name.
    fn get_name(
        &self,
        krate : &ComCrate,
        ident : &syn::Ident,
    ) -> String
    {
        self.get_name_for_ty( krate, ident.as_ref() )
    }

    fn get_ty(
        &self,
        krate : &ComCrate,
        ty : &syn::Ty,
    ) -> Option< String >
    {
        Some( match *ty {

            // Pointer types.
            syn::Ty::Slice( ref ty )
                => format!( "*{}", self.get_ty( krate, ty )? ),
            syn::Ty::Ptr( ref mutty ) | syn::Ty::Rptr( .., ref mutty )
                => match mutty.mutability {
                    syn::Mutability::Mutable
                        => format!( "*{}", self.get_ty( krate, &mutty.ty )? ),
                    syn::Mutability::Immutable
                        => format!( "*const {}", self.get_ty( krate, &mutty.ty )? ),
                },

            // This is quite experimental. Do IDLs even support staticly sized
            // arrays? Currently this turns [u8; 3] into "uint8[3]" IDL type.
            syn::Ty::Array( ref ty, ref count )
                => format!( "{}[{:?}]", self.get_ty( krate, ty.as_ref() )?, count ),

            // Normal Rust struct/trait type.
            syn::Ty::Path( .., ref path )
                => self.segment_to_ty( krate, path.segments.last().unwrap() )?,

            // Tuple with length 0, ie. Unit tuple: (). This is void-equivalent.
            syn::Ty::Tup( ref l ) if l.is_empty()
                => "void".to_owned(),

            syn::Ty::BareFn(..)
                | syn::Ty::Never
                | syn::Ty::Tup(..)
                | syn::Ty::TraitObject(..)
                | syn::Ty::ImplTrait(..)
                | syn::Ty::Paren(..)
                | syn::Ty::Infer
                | syn::Ty::Mac(..)
                => return None,
        } )
    }
}

impl CTyHandler
{
    /// Converts a path segment to a Ty.
    ///
    /// The path segment should be the last segment for this to make any sense.
    fn segment_to_ty(
        &self,
        krate : &ComCrate,
        segment : &syn::PathSegment,
    ) -> Option<String>
    {
        // Get the segment as a string.
        let ty = format!( "{}", segment.ident );

        // Get the type information.
        let args = match segment.parameters {
            syn::PathParameters::AngleBracketed( ref data )
                    => &data.types,

            // Parenthesized path parameters should be valid only for Fn-types.
            // These types are unsupported, but we'll match for them here anyway.
            syn::PathParameters::Parenthesized( ref data )
                    => &data.inputs,
        };

        Some( match ty.as_str() {
            
            // Hardcoded handling for parameter types.
            "ComRc" | "ComItf"
                => format!( "{}*", self.get_ty( krate, &args[0] )? ),
            "RawComPtr" => "*void".to_owned(),
            "String" => "BSTR".to_owned(),
            "usize" => "size_t".to_owned(),
            "u64" => "uint64".to_owned(),
            "i64" => "int64".to_owned(),
            "u32" => "uint32".to_owned(),
            "i32" => "int32".to_owned(),
            "u16" => "uint16".to_owned(),
            "i16" => "int16".to_owned(),
            "u8" => "uint8".to_owned(),
            "i8" => "int8".to_owned(),
            "f64" => "double".to_owned(),
            "f32" => "float".to_owned(),

            // Default handling checks if we need to rename the type, such as
            // Foo -> IFoo for implicit interfaces.
            t => self.get_name_for_ty( krate, t ),
        } )
    }

    fn get_name_for_ty(
        &self,
        krate : &ComCrate,
        ty_name : &str
    ) -> String
    {
        let itf = if let Some( itf ) = krate.interfaces().get( ty_name ) {
            itf
        } else {
            return ty_name.to_owned()
        };

        if itf.item_type() == ::utils::InterfaceType::Struct {
            format!( "I{}", itf.name() )
        } else {
            ty_name.to_owned()
        }
    }
}

/// Converts a Rust type into applicable C++ type.
pub fn to_cpp_type(
    c: &ComCrate,
    ty: &syn::Ty,
) -> Option<String> { //, GeneratorError> {

    let foreign = CTyHandler;
    let name = foreign.get_ty( c, ty )?;
            
    Some( match name.as_ref() {
        "int8" => "int8_t",
        "uint8" => "uint8_t",
        "int16" => "int16_t",
        "uint16" => "uint16_t",
        "int32" => "int32_t",
        "uint32" => "uint32_t",
        "int64" => "int64_t",
        "uint64" => "uint64_t",
        "BSTR" => "intercom::BSTR",
        "HRESULT" => "intercom::HRESULT",
        _ => return Some( name ),
    }.to_owned() )
}

