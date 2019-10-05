extern crate std;

use std::rc::Rc;
use syn;
use syn::TypeParamBound;

/// Detailed information of a Rust type.
#[derive(Clone, Debug)]
pub enum RustType<'e>
{
    Ident( &'e syn::Ident ),
    Void,
    Wrapper( &'e syn::Ident, Rc< TypeInfo<'e> > ),
}

/// Defines how a value is passed to/from Rust function.
#[derive(PartialEq, PartialOrd, Clone, Copy, Debug)]
pub enum PassBy
{
    Value,

    Reference,

    Ptr,
}

/// Details of a Rust type. Used for translation to other programming languages.
#[derive(Clone, Debug)]
pub struct TypeInfo<'s>
{
    /// The type in rust.
    pub rust_type: RustType<'s>,

    /// Specifies how the value is passed to/from a function. Default: PassBy::Value
    pub pass_by: PassBy,

    /// Is the Rust type defined as mutable? Default: false
    pub is_mutable: bool,

    /// The length of the array if the Rust type denotes an array.
    pub array_length: Option<&'s syn::Expr>,

    /// Reference to the original type that this type info represents.
    pub original: &'s syn::Type,
}

pub fn parse<'a, 'b: 'a>(
        ty: &'b syn::Type,
) -> Option<TypeInfo<'a>>
{
    let resolver = TypeInfoResolver::from_type( ty )?;

    Some( TypeInfo::from_resolver( resolver, ty ) )
}

/// Collects details of a Rust type when the Rust crate is parsed.
struct TypeInfoResolver<'s>
{
    /// The type in rust.
    rust_type: RustType<'s>,

    /// Specifies how the value is passed. Default: PassBy::Value
    pass_by: Option<PassBy>,

    /// Is the Rust type defined as mutable? Default: false
    is_mutable: Option<bool>,

    /// The length of the array if the Rust type denotes an array.
    array_length: Option<&'s syn::Expr>
}


impl<'s, 'p: 's> TypeInfo<'s> {

    /// Gets the name of the type in Rust.
    pub fn get_name(
        &self
    ) -> String
    {
        // Exclude the wrappee from the name.
        match self.rust_type {
            RustType::Wrapper( wrapper, _ ) => format!( "{}", wrapper ),
            ref t => format!( "{}", t ),
        }
    }

    /// Returns the nested type info or self.
    pub fn get_leaf(
        &self
    ) -> &TypeInfo<'s>
    {
        match self.rust_type {
            RustType::Wrapper( _, ref wrappee ) => wrappee.get_leaf(),
            _ => self
        }
    }

    /// Initializes the type info from resolver which has resolved the type.
    fn from_resolver(
        resolver: TypeInfoResolver<'p>,
        original: &'p syn::Type,
    ) -> TypeInfo<'s>
    {
        // Resolve default values.
        // NOTE: The existence of the array length value identifies an array type and
        // is therefor passed as-is here.
        let pass_by = resolver.pass_by.unwrap_or( PassBy::Value );
        let is_mutable = resolver.is_mutable.unwrap_or( false );

        TypeInfo{
            rust_type: resolver.rust_type,
            pass_by,
            is_mutable,
            array_length: resolver.array_length,
            original,
        }
    }


}

impl<'s> std::fmt::Display for RustType<'s> {

    fn fmt(
        &self,
        f: &mut std::fmt::Formatter
    ) -> std::fmt::Result {
        match *self {
            RustType::Ident( syn_ident ) => write!( f, "{}", syn_ident ),
            RustType::Void => write!( f, "void" ),
            RustType::Wrapper( wrapper, ref wrapped ) => write!( f, "{}<{}>",
                    wrapper, wrapped.rust_type )
        }
    }
}

impl<'s, 'p: 's> TypeInfoResolver<'s> {

    /// Parses the type info from the specified Type.
    fn from_type(
        syn_type: &'p syn::Type,
    ) -> Option<TypeInfoResolver<'s>>
    {
        match *syn_type {

            // Delegate to appropriate conversion.
            syn::Type::Slice( ref slice ) => TypeInfoResolver::from_slice( slice ),
            syn::Type::Reference( ref reference ) => TypeInfoResolver::from_reference( reference ),
            syn::Type::Ptr( ref ptr ) => TypeInfoResolver::from_pointer( ptr ),
            syn::Type::Array( ref arr ) => TypeInfoResolver::from_array( arr ),
            syn::Type::Path( ref p ) => TypeInfoResolver::from_path( p ),
            syn::Type::Tuple( ref t ) if t.elems.is_empty() => Some( TypeInfoResolver::void() ),
            syn::Type::TraitObject( ref trait_object ) =>
                    TypeInfoResolver::from_trait_object( trait_object ),

            syn::Type::BareFn(..)
                | syn::Type::Never(..)
                | syn::Type::Tuple(..)
                | syn::Type::ImplTrait(..)
                | syn::Type::Paren(..)
                | syn::Type::Infer(..)
                | syn::Type::Macro(..)
                | syn::Type::Verbatim(..)
                | syn::Type::Group(..)
                => { dbg!( syn_type ); None },
        }
    }

    fn new(
        rust_type: RustType<'s>
    ) -> TypeInfoResolver<'s>
    {
        TypeInfoResolver {
            rust_type,
            pass_by: None,
            is_mutable: None,
            array_length: None,
        }
    }

    fn void() -> TypeInfoResolver<'s>
    {
        TypeInfoResolver::new( RustType::Void )
    }

    fn pass_by(
        resolver: TypeInfoResolver<'s>,
        pass_by: PassBy,
    ) -> TypeInfoResolver<'s>
    {
        if resolver.pass_by.is_some() {
            panic!("Cannot set pass_by twice.")
        }

        TypeInfoResolver {
            rust_type: resolver.rust_type,
            pass_by: Some( pass_by ),
            is_mutable: resolver.is_mutable,
            array_length: resolver.array_length,
        }
    }

    fn mutable(
        resolver: TypeInfoResolver<'s>,
        is_mutable: bool,
    ) -> TypeInfoResolver<'s>
    {
        if resolver.is_mutable.is_some() {
            panic!("Cannot set is_mutable twice.")
        }

        TypeInfoResolver {
            rust_type: resolver.rust_type,
            pass_by: resolver.pass_by,
            is_mutable: Some( is_mutable ),
            array_length: resolver.array_length,
        }
    }

    fn array(
        resolver: TypeInfoResolver<'s>,
        array_length: &'p syn::Expr,
    ) -> TypeInfoResolver<'s>
    {
        if resolver.array_length.is_some() {
            panic!("Cannot set array_length twice.")
        }

        TypeInfoResolver {
            rust_type: resolver.rust_type,
            pass_by: resolver.pass_by,
            is_mutable: resolver.is_mutable,
            array_length: Some( array_length ),
        }
    }

    fn wrapped(
        resolver: &TypeInfoResolver<'s>,
        args: &'p syn::punctuated::Punctuated< syn::GenericArgument, syn::token::Comma >,
    ) -> Option<TypeInfoResolver<'s>>
    {
        if let RustType::Wrapper( _, _ ) = resolver.rust_type {
            panic!("Nested wrappers are not allowed.")
        }

        // Determine the TypeInfo of the nested type.
        let nested_type = match **args.first().unwrap().value() {
                                    syn::GenericArgument::Type( ref t ) => t,
                                    _ => return None,
                            };
        let nested_type = TypeInfo::from_resolver(
                        TypeInfoResolver::from_type( nested_type )?, nested_type );

        Some( TypeInfoResolver {
            rust_type: RustType::Wrapper(
                    resolver.get_ident_for_wrapping(), Rc::new( nested_type ) ),
            pass_by: resolver.pass_by,
            is_mutable: resolver.is_mutable,
            array_length: resolver.array_length,
        } )
    }

    fn from_array(
        array: &'p syn::TypeArray,
    ) -> Option<TypeInfoResolver<'s>>
    {
        let resolver = TypeInfoResolver::from_type( &array.elem )?;

        Some( TypeInfoResolver::array( resolver, &array.len ) )
    }

    fn from_slice(
        slice: &'p syn::TypeSlice,
    ) -> Option<TypeInfoResolver<'s>>
    {
        TypeInfoResolver::from_type( &slice.elem )
    }

    fn from_reference(
        reference : &'p syn::TypeReference,
    ) -> Option<TypeInfoResolver<'s>>
    {
        let resolver = TypeInfoResolver::from_type( &reference.elem )?;
        let resolver = TypeInfoResolver::mutable( resolver,
                TypeInfoResolver::is_mutable( reference.mutability ) );

        Some( TypeInfoResolver::pass_by( resolver, PassBy::Reference ) )
    }

    fn from_pointer(
        ptr : &'p syn::TypePtr,
    ) -> Option<TypeInfoResolver<'s>>
    {
        let resolver = TypeInfoResolver::from_type( &ptr.elem )?;
        let resolver = TypeInfoResolver::mutable( resolver,
                TypeInfoResolver::is_mutable( ptr.mutability ) );

        Some( TypeInfoResolver::pass_by( resolver, PassBy::Ptr ) )
    }

    fn from_path(
        type_path: &'p syn::TypePath,
    ) -> Option<TypeInfoResolver<'s>>
    {
        TypeInfoResolver::from_segment( type_path.path.segments.last().unwrap().value() )
    }

    fn from_segment(
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
            "ComRc" | "ComItf" | "ComResult" | "InterfacePtr"
                => TypeInfoResolver::wrapped(
                        &TypeInfoResolver::new( RustType::Ident( &segment.ident ) ),
                        args.expect( "Wrapper types requires valid wrappee.")
                    ),

            // Bare type.
            _t => Some( TypeInfoResolver::new( RustType::Ident( &segment.ident ) ) ),
        }
    }

    /// Resolves the type from a trait object.
    fn from_trait_object(
        trait_object: &'p syn::TypeTraitObject,
    ) -> Option<TypeInfoResolver<'s>>
    {
        // Find the first actual trait. Fro example lifetime parameters are ignored.
        let trait_bound = trait_object.bounds.iter().find_map( |parameter: &TypeParamBound|
                                                 if let syn::TypeParamBound::Trait( ref tr ) = parameter  { Some( tr ) }
                                                 else { None } )?;
        TypeInfoResolver::from_segment( trait_bound.path.segments.last().unwrap().value() )
    }

    /// Determines if the given type is mutable
    fn is_mutable(
        mutability: Option<syn::token::Mut>
    ) -> bool
    {
        mutability.is_some()
    }

    /// Gets the identifier used to wrap another type.
    fn get_ident_for_wrapping(
        &self
    ) -> &'s syn::Ident
    {
        match self.rust_type
        {
            RustType::Ident( ident ) => ident,
            _ => panic!( "Only identifiers can wrap other rust types." ),
        }
    }
}
