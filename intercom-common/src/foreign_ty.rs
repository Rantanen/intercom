use model::ComCrate;
use syn;

pub trait ForeignTypeHandler
{
    /// Gets the name for the 'ty'.
    fn get_name( &self, krate : &ComCrate, ty : &syn::Ident ) -> String;

    /// Gets the COM type for a Rust type.
    fn get_ty( &self, krate : &ComCrate, ty : &syn::Type ) -> Option< String >;
}

pub struct CTypeHandler;

impl ForeignTypeHandler for CTypeHandler
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
        ty : &syn::Type,
    ) -> Option< String >
    {
        Some( match *ty {

            // Pointer types.
            syn::Type::Slice( ref slice )
                => format!( "{}*", self.get_ty( krate, &slice.elem )? ),
            syn::Type::Reference( syn::TypeReference { ref mutability, ref elem, .. } )
                | syn::Type::Ptr( syn::TypePtr { ref mutability, ref elem, .. } )
                => match *mutability {
                    Some(_) => format!( "{}*", self.get_ty( krate, elem )? ),
                    None => format!( "const {}*", self.get_ty( krate, elem )? ),
                },

            // This is quite experimental. Do IDLs even support staticly sized
            // arrays? Currently this turns [u8; 3] into "uint8[3]" IDL type.
            syn::Type::Array( ref arr )
                => format!( "{}[{:?}]", self.get_ty( krate, &arr.elem )?, arr.len ),

            // Normal Rust struct/trait type.
            syn::Type::Path( ref p )
                => self.segment_to_ty( krate, p.path.segments.last().unwrap().value() )?,

            // Tuple with length 0, ie. Unit tuple: (). This is void-equivalent.
            syn::Type::Tuple( ref t ) if t.elems.is_empty()
                => "void".to_owned(),

            syn::Type::BareFn(..)
                | syn::Type::Never(..)
                | syn::Type::Tuple(..)
                | syn::Type::TraitObject(..)
                | syn::Type::ImplTrait(..)
                | syn::Type::Paren(..)
                | syn::Type::Infer(..)
                | syn::Type::Macro(..)
                | syn::Type::Verbatim(..)
                | syn::Type::Group(..)
                => return None,
        } )
    }
}

impl CTypeHandler
{
    /// Converts a path segment to a Type.
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
        let args = match segment.arguments {
            syn::PathArguments::None
                    => None,

            syn::PathArguments::AngleBracketed( ref data )
                    => Some( &data.args ),

            // Parenthesized path parameters should be valid only for Fn-types.
            // These types are unsupported, but we'll match for them here anyway.
            syn::PathArguments::Parenthesized( .. )
                    => panic!( "Fn-types are unsupported." ),
        };

        Some( match ty.as_str() {
            
            // Hardcoded handling for parameter types.
            "ComRc" | "ComItf"
                => format!( "{}*", self.get_ty(
                        krate,
                        match **args.unwrap().first().unwrap().value() {
                            syn::GenericArgument::Type( ref t ) => t,
                            _ => return None,
                        } )? ),
            "RawComPtr" => "*void".to_owned(),
            "String" | "BStr" => "BSTR".to_owned(),
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
            "c_void" => "void".to_owned(),

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
    ty: &syn::Type,
) -> Option<String> { //, GeneratorError> {

    let foreign = CTypeHandler;
    let name = foreign.get_ty( c, ty )?;
            
    Some( match name.as_ref() {
        "int8" => "int8_t",
        "uint8" => "uint8_t",
        "int16" => "int16_t",
        "uint16" => "uint16_t",
        "uint16*" => "uint16_t*",
        "int32" => "int32_t",
        "uint32" => "uint32_t",
        "int64" => "int64_t",
        "uint64" => "uint64_t",
        "BSTR" => "intercom::BSTR",
        "HRESULT" => "intercom::HRESULT",
        _ => return Some( name ),
    }.to_owned() )
}

