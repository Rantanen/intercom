
# Intercom style guidelines

## General

### Whitespace

- Spaces inside parentheses/braces/brackets/etc. In cases where the contents
  contain no spaces or special characters of their own, spaces may be omitted.
    ```rust
    let v : Option< Vec<u32> > = get_result( a, b, c[0] );
    //                   ^                           ^
    //  Single generic --'          Simple index ----'
    ```

- Long method signatures use chopped parameters.
    ```rust
    fn write_com_attribute<T>(
        &self,
        attribute : &syn::Attribute,
        libid : &GUID,
    ) -> String
        where T: 'static
    {
        // ...
    }
    ```

- Braces pretend to use K&R style. Items (functions, traits, structs, etc.)
    prefer spaces on their own lines. Expressions use trailing spaces. Short
    items make an exception.
    ```rust
    impl<T: Debug> MyTrait for MyStruct<T>
    {
        fn long_method( &self ) -> Result< u32, String >
        {
            // ...
        }

        fn is_valid( &self ) -> bool {
            self.foo.is_some() && self.bar.len() > 0
        }

        fn has_foo( &self ) -> bool { self.foo.is_some() }
    }
    ```

## Rust

Any style items not specified in this document use style defined in
[Rust code formatting RFCs](https://github.com/rust-lang-nursery/fmt-rfcs).


## Other languages

Other languages default to the Rust style where applicable (spacing,
indentation, etc.) unless the language strongly prefers some other style,
such as using `camelCase` variable names in C#.

### C++ documentation style

C++ code uses JavaDoc inspired Doxygen format for function/class/etc.
documentation. The prose uses Markdown syntax similar to what Rust does.

```C++
/**
 * Reference counting COM pointer
 *
 * Implements the IUnknown reference counting and query interface mechanisms.
 * Note that `add_ref` and `release` should not be called manually for pointers
 * handled through the `ComPtr` type.
 */
class ComPtr
{
public:

    /**
     * Wraps an existing raw pointer into a managed ComPtr.
     *
     * @param ptr Pointer to wrap.
     */
    ComPtr( void* ptr );
}
```
