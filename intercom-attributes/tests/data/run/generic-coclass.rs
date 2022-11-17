extern crate intercom;
use intercom::attributes::HasInterface;
use intercom::prelude::*;
use std::cell::RefCell;

#[com_interface]
trait IAnimal
{
    fn get_name(&self) -> ComResult<String>;
    fn describe(&self) -> ComResult<String>;
}

// Crab

#[com_interface]
trait ICrab
{
    fn pick_up(&self, item: &str) -> ComResult<()>;
}

#[com_class(IAnimal, ICrab)]
#[derive(Default)]
struct Crab
{
    name: &'static str,
    item: RefCell<Option<String>>,
}

impl Crab
{
    fn new(name: &'static str) -> Crab
    {
        Crab {
            name,
            item: RefCell::new(None),
        }
    }
}

impl ICrab for Crab
{
    fn pick_up(&self, item: &str) -> ComResult<()>
    {
        self.item.replace(Some(item.to_string()));
        Ok(())
    }
}

impl IAnimal for Crab
{
    fn get_name(&self) -> ComResult<String>
    {
        Ok(self.name.to_string())
    }

    fn describe(&self) -> ComResult<String>
    {
        match *self.item.borrow() {
            Some(ref item) => Ok(format!("A crab called {} carrying {}", self.name, item)),
            None => Ok(format!("A crab called {} who is empty pincered", self.name)),
        }
    }
}

// Dog

#[com_interface]
trait IDog
{
}

#[com_class(IAnimal, IDog)]
#[derive(Default)]
struct Dog
{
    name: &'static str,
}

impl IDog for Dog {}
impl IAnimal for Dog
{
    fn get_name(&self) -> ComResult<String>
    {
        Ok(self.name.to_string())
    }

    fn describe(&self) -> ComResult<String>
    {
        Ok(format!("A dog called {}", self.name))
    }
}

// Generic collection

#[com_interface]
trait ICollection
{
    fn get_count(&self) -> usize;
    fn get_item(&self, idx: usize) -> ComResult<ComRc<dyn IAnimal>>;
}

#[com_class(ICollection)]
struct Collection<T: HasInterface<dyn IAnimal> + Default>
{
    items: Vec<ComBox<T>>,
}

impl<T: HasInterface<dyn IAnimal> + Default> ICollection for Collection<T>
{
    fn get_count(&self) -> usize
    {
        self.items.len()
    }

    fn get_item(&self, idx: usize) -> ComResult<ComRc<dyn IAnimal>>
    {
        Ok(self
            .items
            .get(idx)
            .map(|item| item.into())
            .unwrap_or_else(|| ComBox::new(T::default()).into()))
    }
}

// A factory that returns generic collections.

#[com_class(Self)]
struct Factory;

#[com_interface]
impl Factory
{
    fn get_crabs(&self) -> ComResult<ComRc<dyn ICollection>>
    {
        let collection = ComBox::new(Collection {
            items: vec![
                ComBox::new(Crab::new("Ferris")),
                ComBox::new(Crab::new("Pincers")),
                ComBox::new(Crab::new("Shelly")),
            ],
        });

        Ok(collection.into())
    }

    fn get_dogs(&self) -> ComResult<ComRc<dyn ICollection>>
    {
        let collection = ComBox::new(Collection {
            items: vec![
                ComBox::new(Dog { name: "Spot" }),
                ComBox::new(Dog { name: "Stripe" }),
                ComBox::new(Dog { name: "Checkers" }),
            ],
        });

        Ok(collection.into())
    }
}

fn main()
{
    // Instantiate Factory as a COM interface.
    let factory = ComBox::new(Factory);
    let factory: ComRc<Factory> = factory.into();

    // Make crabs pick up stuff.
    let crab_collection = factory.get_crabs().expect("Failed to get crabs");
    let items = ["borrowck", "fish", "gear"];
    for i in 0..crab_collection.get_count() {
        let animal = crab_collection.get_item(i).expect("Failed to get item");
        let crab: ComRc<dyn ICrab> = intercom::ComItf::query_interface(&animal)
            .expect("Crab collection had a non-crab item");
        crab.pick_up(items[i]);
    }

    // Describe each crab.
    for i in 0..crab_collection.get_count() {
        let animal = crab_collection.get_item(i).expect("Failed to get item");
        println!("{}", animal.describe().expect("Failed to describe"));
    }
}
