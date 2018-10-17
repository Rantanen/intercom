
use intercom::*;

#[com_interface]
trait IRefCount
{
    fn get_ref_count( &self ) -> u32;
}

#[com_class( ClassCreator )]
pub struct ClassCreator { }

#[com_interface]
#[com_impl]
impl ClassCreator {
    pub fn new() -> ClassCreator { ClassCreator {} }

    pub fn create_root( &self, id : i32 ) -> ComResult<ComItf<CreatedClass>> {
        Ok( ComStruct::new( CreatedClass::new_with_id( id ) ).into() )
    }

    pub fn create_child(
        &self,
        id : i32,
        parent : ComItf<IParent>
    ) -> ComResult<ComItf<CreatedClass>>
    {
        Ok( ComStruct::new(
            CreatedClass::new_child( id, parent.get_id() )
        ).into() )
    }
}

#[com_class( CreatedClass, IParent, IRefCount )]
pub struct CreatedClass { id : i32, parent: i32 }

#[com_interface]
#[com_impl]
impl CreatedClass {
    pub fn new() -> CreatedClass { unreachable!() }
    pub fn new_with_id( id : i32 ) -> CreatedClass { CreatedClass { id, parent: 0 } }
    pub fn new_child( id : i32, parent : i32 ) -> CreatedClass { CreatedClass { id, parent } }

    pub fn get_id( &self ) -> ComResult<i32> { Ok( self.id ) }
    pub fn get_parent_id( &self ) -> ComResult<i32> { Ok( self.parent ) }
}

#[com_impl]
impl IRefCount for CreatedClass {
    fn get_ref_count( &self ) -> u32
    {
        let combox = unsafe { ComBox::of( self ) };
        combox.get_ref_count()
    }
}

#[com_interface]
pub trait IParent {
    fn get_id( &self ) -> i32;
}

#[com_impl]
impl IParent for CreatedClass {
    fn get_id( &self ) -> i32 { self.id }
}

#[com_class( RefCountOperations )]
pub struct RefCountOperations {}

#[com_interface]
#[com_impl]
impl RefCountOperations {
    pub fn new() -> RefCountOperations { RefCountOperations { } }

    pub fn get_new( &self ) -> ComResult<ComItf<RefCountOperations>> {
        Ok( ComStruct::new( RefCountOperations::new() ).into() )
    }

    pub fn get_ref_count( &self ) -> u32 {
        let combox = unsafe { ComBox::of( self ) };
        combox.get_ref_count()
    }
}

