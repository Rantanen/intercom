extern crate serde;
extern crate serde_derive;
extern crate serde_json;

use self::serde_derive::{Deserialize, Serialize};
use super::type_system::{ComItemCategory, TypeSystemName};
use std::collections::HashMap;

/// Type representation in json format.
#[derive(Serialize, Deserialize)]
pub struct ComItem<TItem> {
    /// The category of the type.
    category: ComItemCategory,

    /// The name of the type.
    type_name: String,

    /// Details of the type.
    item: TItem,
}

/// Type representation for a COM class.
#[derive(Serialize, Deserialize)]
pub struct ComClass {
    /// The name of the class.
    name: String,

    /// Interfaces implemented by the class.
    interfaces: HashMap<TypeSystemName, Vec<ComInterfaceVariant>>,
}

/// Type representation for a COM interface.
#[derive(Serialize, Deserialize)]
pub struct ComInterfaceVariant {
    /// The name of the interface.
    name: String,

    /// Type system of the interface variant.
    type_system: TypeSystemName,
}

/// Type representation for a COM interface.
#[derive(Serialize, Deserialize)]
pub struct ComMethod {
    /// The name of the name.
    name: String,
}

impl ComClass {
    /// Initializes a new ComClass object.
    pub fn new(name: String, interfaces: Vec<ComInterfaceVariant>) -> ComClass {
        //
        let mut itf_by_type_system: HashMap<TypeSystemName, Vec<ComInterfaceVariant>> =
            HashMap::with_capacity(interfaces.len());
        for itf in interfaces {
            let entry = itf_by_type_system.entry(itf.type_system);
            let interfaces = entry.or_insert_with(|| vec![]);
            interfaces.push(itf);
        }

        ComClass {
            name,
            interfaces: itf_by_type_system,
        }
    }
}

impl ComInterfaceVariant {
    /// Initializes a new ComClass object.
    pub fn new(name: String, type_system: TypeSystemName) -> ComInterfaceVariant {
        ComInterfaceVariant { name, type_system }
    }
}

impl ComMethod {
    /// Initializes a new ComClass object.
    pub fn new(name: String) -> ComMethod {
        ComMethod { name }
    }
}
