extern crate serde;
extern crate serde_json;
extern crate serde_derive;

use prelude::*;
use self::serde::{Deserialize, Serialize};
use self::serde_derive::{Deserialize, Serialize};
use super::type_system::{ComItemCategory};

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
}

/// Type representation for a COM interface.
#[derive(Serialize, Deserialize)]
pub struct ComInterface {

    /// The name of the interface.
    name: String,
}

impl ComClass {

    /// Initializes a new ComClass object.
    pub fn new(
        name: String
    ) -> ComClass {
        ComClass {
            name
        }
    }

}

impl ComInterface {

    /// Initializes a new ComClass object.
    pub fn new(
        name: String
    ) -> ComInterface {
        ComInterface {
            name
        }
    }
}
