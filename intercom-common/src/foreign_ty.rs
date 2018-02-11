extern crate std;

use model::ComCrate;
use syn;

/// Detailed information of a Rust type.
pub enum RustType<'e>
{
    Ident( &'e syn::Ident ),
    Void,
}

/// Defines how a value is passed to/from Rust function.
#[derive(PartialEq, PartialOrd, Clone)]
pub enum PassBy
{
    Value,

    Reference,

    Ptr,
}

/// Details of a Rust type. Used for translation to other programming languages.
pub struct TypeInfo<'s>
{
    /// The crate this type is associated with.
    pub krate: &'s ComCrate,

    /// The type in rust.
    pub rust_type: RustType<'s>,

    /// Specifies how the value is passed to/from a function. Default: PassBy::Value
    pub pass_by: PassBy,

    /// Is the Rust type defined as mutable? Default: false
    pub is_mutable: bool,

    /// The length of the array if the Rust type denotes an array.
    pub array_length: Option<&'s syn::Expr>
}

pub trait ForeignTypeHandler
{
    /// Gets the name for the 'ty'.
    fn get_name( &self, krate : &ComCrate, ty : &syn::Ident ) -> String;

    /// Gets the COM type for a Rust type.
    fn get_ty<'a, 'b: 'a>( &self, krate : &'b ComCrate, ty : &'b syn::Type ) -> Option< TypeInfo<'a> >;
}

pub struct CTypeHandler;

/// Collects details of a Rust type when the Rust crate is parsed.
struct TypeInfoResolver<'s>
{
    /// The crate this type is associated with.
    krate: &'s ComCrate,

    /// The type in rust.
    rust_type: RustType<'s>,

    /// Specifies how the value is passed. Default: PassBy::Value
    pass_by: Option<PassBy>,

    /// Is the Rust type defined as mutable? Default: false
    is_mutable: Option<bool>,

    /// The length of the array if the Rust type denotes an array.
    array_length: Option<&'s syn::Expr>
}


impl ForeignTypeHandler for CTypeHandler
{
    /// Tries to apply renaming to the name.
    fn get_name(
        &self,
        krate: &ComCrate,
        ident: &syn::Ident,
    ) -> String
    {
        self.get_name_for_ty( krate, ident.as_ref() )
    }

    fn get_ty<'a, 'b: 'a>(
        &self,
        krate: &'b ComCrate,
        ty: &'b syn::Type,
    ) -> Option<TypeInfo<'a>>
    {
        match TypeInfoResolver::from_type( krate, ty )
        {
            Some( resolver ) => Some( TypeInfo::from_resolver( resolver ) ),
            None => None,
        }
    }
}

impl<'s, 'p: 's> TypeInfo<'s> {

    /// Gets the name of the type in Rust.
    pub fn get_name(
        &self
    ) -> String
    {
        format!( "{}", self.rust_type )
    }

    /// Initializes the type info from resolver which has resolved the type.
    fn from_resolver(
        resolver: TypeInfoResolver<'p>
    ) -> TypeInfo<'s>
    {
        // Resolve default values.
        // NOTE: The existence of the array length value identifies an array type and
        // is therefor passed as-is here.
        let pass_by;
        let is_mutable;
        {
            pass_by = match resolver.pass_by {
                Some( ref v ) => v,
                None => &PassBy::Value,
            };

            is_mutable = match resolver.is_mutable {
                Some( ref v ) => v.clone(),
                None => false,
            };
        };

        TypeInfo{
            krate: resolver.krate,
            rust_type: resolver.rust_type,
            pass_by: pass_by.clone(),
            is_mutable: is_mutable,
            array_length: resolver.array_length,
        }
    }
}

impl<'s> std::fmt::Display for RustType<'s> {

    fn fmt(
        &self,
        f: &mut std::fmt::Formatter
    ) -> std::fmt::Result {
        match self {
            &RustType::Ident( syn_ident ) => write!( f, "{}", syn_ident ),
            &RustType::Void => write!( f, "void" ),
        }
    }
}

impl<'s, 'p: 's> TypeInfoResolver<'s> {

    /// Parses the type info from the specified Type.
    fn from_type(
        krate: &'p ComCrate,
        syn_type: &'p syn::Type,
    ) -> Option<TypeInfoResolver<'s>>
    {
        match *syn_type {

            // Delegate to appropriate conversion.
            syn::Type::Slice( ref slice ) => TypeInfoResolver::from_slice( krate, slice ),
            syn::Type::Reference( ref reference ) => TypeInfoResolver::from_reference( krate, reference ),
            syn::Type::Ptr( ref ptr ) => TypeInfoResolver::from_pointer( krate, ptr ),
            syn::Type::Array( ref arr ) => TypeInfoResolver::from_array( krate, arr ),
            syn::Type::Path( ref p ) => TypeInfoResolver::from_path( krate, p ),
            syn::Type::Tuple( ref t ) if t.elems.is_empty() => Some( TypeInfoResolver::void( krate ) ),

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
                => None,
        }
    }

    fn new(
        krate: &'p ComCrate,
        rust_type: RustType<'s>
    ) -> TypeInfoResolver<'s>
    {
        TypeInfoResolver {
            krate: krate,
            rust_type: rust_type,
            pass_by: None,
            is_mutable: None,
            array_length: None,
        }
    }

    fn void(
        krate: &'p ComCrate,
    ) -> TypeInfoResolver<'s>
    {
        TypeInfoResolver::new( krate, RustType::Void )
    }

    fn pass_by_option(
        resolver: Option<TypeInfoResolver<'s>>,
        pass_by: PassBy,
    ) -> Option<TypeInfoResolver<'s>>
    {
        match resolver {
            Some( r ) => Some( TypeInfoResolver::pass_by( r, pass_by ) ),
            None => None,
        }
    }

    fn pass_by(
        resolver: TypeInfoResolver<'s>,
        pass_by: PassBy,
    ) -> TypeInfoResolver<'s>
    {
        match resolver.pass_by {
            Some( _ ) => panic!("Cannot set pass_by twice."),
            None => {},
        }

        TypeInfoResolver {
            krate: resolver.krate,
            rust_type: resolver.rust_type,
            pass_by: Some( pass_by ),
            is_mutable: resolver.is_mutable,
            array_length: resolver.array_length,
        }
    }

    fn mutable_option(
        resolver: Option<TypeInfoResolver<'s>>,
        is_mutable: bool,
    ) -> Option<TypeInfoResolver<'s>>
    {
        match resolver {
            Some( r ) => Some( TypeInfoResolver::mutable( r, is_mutable ) ),
            None => None,
        }
    }

    fn mutable(
        resolver: TypeInfoResolver<'s>,
        is_mutable: bool,
    ) -> TypeInfoResolver<'s>
    {
        match resolver.is_mutable {
            Some(_) => panic!("Cannot set is_mutable twice."),
            None => {}
        }

        TypeInfoResolver {
            krate: resolver.krate,
            rust_type: resolver.rust_type,
            pass_by: resolver.pass_by,
            is_mutable: Some( is_mutable ),
            array_length: resolver.array_length,
        }
    }

    fn array_option(
        resolver: Option<TypeInfoResolver<'s>>,
        array_length: &'p syn::Expr,
    ) -> Option<TypeInfoResolver<'s>>
    {
        match resolver {
            Some( r ) => Some( TypeInfoResolver::array( r, array_length ) ),
            None => None,
        }
    }

    fn array(
        resolver: TypeInfoResolver<'s>,
        array_length: &'p syn::Expr,
    ) -> TypeInfoResolver<'s>
    {
        match resolver.array_length {
            Some(_) => panic!("Cannot set array_length twice."),
            None => {}
        }

        TypeInfoResolver {
            krate: resolver.krate,
            rust_type: resolver.rust_type,
            pass_by: resolver.pass_by,
            is_mutable: resolver.is_mutable,
            array_length: Some( array_length ),
        }
    }

    fn from_array(
        krate : &'p ComCrate,
        array: &'p syn::TypeArray,
    ) -> Option<TypeInfoResolver<'s>>
    {
        TypeInfoResolver::array_option(
            TypeInfoResolver::from_type( krate, &array.elem ),
            &array.len,
        )
    }

    fn from_slice(
        krate : &'p ComCrate,
        slice: &'p syn::TypeSlice,
    ) -> Option<TypeInfoResolver<'s>>
    {
        TypeInfoResolver::from_type( krate, &slice.elem )
    }

    fn from_reference(
        krate : &'p ComCrate,
        reference : &'p syn::TypeReference,
    ) -> Option<TypeInfoResolver<'s>>
    {
        TypeInfoResolver::pass_by_option(
                TypeInfoResolver::mutable_option(
                            TypeInfoResolver::from_type( krate, &reference.elem ),
                            TypeInfoResolver::is_mutable( &reference.mutability )
                        ),
                PassBy::Reference,
        )
    }

    fn from_pointer(
        krate : &'p ComCrate,
        ptr : &'p syn::TypePtr,
    ) -> Option<TypeInfoResolver<'s>>
    {
        TypeInfoResolver::pass_by_option(
            TypeInfoResolver::mutable_option(
                TypeInfoResolver::from_type( krate, &ptr.elem ),
                TypeInfoResolver::is_mutable( &ptr.mutability ),
            ),
            PassBy::Ptr,
        )
    }

    fn from_path(
        krate: &'p ComCrate,
        type_path: &'p syn::TypePath,
    ) -> Option<TypeInfoResolver<'s>>
    {
        TypeInfoResolver::from_segment( krate, type_path.path.segments.last().unwrap().value() )
    }

    fn from_segment(
        krate: &'p ComCrate,
        segment: &'p syn::PathSegment,
    ) -> Option<TypeInfoResolver<'s>>
    {
        // Get the segment as a string.
        let rust_type = format!( "{}", segment.ident );

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

        match rust_type.as_str() {

            // Extract a wrapped type.
            "ComRc" | "ComItf" | "ComResult"
                => TypeInfoResolver::pass_by_option( TypeInfoResolver::from_type(
                        krate,
                        match **args.unwrap().first().unwrap().value() {
                            syn::GenericArgument::Type( ref t ) => t,
                            _ => return None,
                        } ), PassBy::Ptr ),

            // Bare type.
            _t => Some( TypeInfoResolver::new( krate, RustType::Ident( &segment.ident ) ) ),
        }
    }

    /// Determines if the given type is mutable
    fn is_mutable(
        mutability: &Option<syn::token::Mut>
    ) -> bool
    {
        match mutability {
            &Some( _ ) => true,
            &None => false,
        }
    }
}

impl CTypeHandler
{
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
