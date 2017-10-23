# COM-export-rs
### Utilities for writing COM components in Rust

The COM export allows one to write reusable components in Rust, which can then
be consumed in any COM-compatible language. This includes C++, C#, VB.Net, etc.

## Example

Following is a `Calculator` interface written in Rust.

```rust
#![feature(plugin, custom_attribute)]
#![plugin(com_library)]

#[com_library(
    Calculator
)]

struct Calculator{
    value: i32
}

#[com_visible("{12341234-1234-1234-1234-123412340001}")]
impl Calculator
{
    pub fn new() -> Calculator { Calculator { value : 0 } }

    pb fn add( &mut self, value : i32 ) -> com_runtime::ComResult<i32>
    {
        self.value += value;
        Ok( self.value )
    }
}
```

The COM service can be consumed for example from C#:

```c#
class Program
{
    [STAThread]
    static void Main( string[] args )
    {
        var calc = new Calculator.Interop.Calculator();
        calc.Add( 10 );
        var result = calc.Add( 100 );
        Console.WriteLine( result );
    }
}
```

## Status

- [x] Basic vtable and delegate construction.
- [ ] Proper QueryInterface implementation with proper IID checking.
- [ ] More complex parameter values, such as the dreaded structs.
- [ ] Multiple interface implementation.
- [ ] Test harness...

### Maybe one day
- [ ] Automatic IDispatch derive.
- [ ] Automatic IDL generation for MIDL.
