use gtk::glib;
use gtk::prelude::*;
use glib::{Object, Enum};

#[derive(Default, Clone, Copy, Enum)]
#[enum_type(name = "MyEnum")]
enum MyEnum {
	#[default]
    Value1,
    Value2,
}

mod imp {
    use std::cell::RefCell;

	use gtk::prelude::*;
	use gtk::subclass::prelude::*;
    use gtk::glib;
    use glib::Properties;

    use super::MyEnum;

	#[derive(Properties, Default)]
	#[properties(wrapper_type = super::MyObject)]
    pub struct MyObject {
		#[property(get, set)]
		pub enum_value: RefCell<MyEnum>,
    }

    #[glib::object_subclass]
	impl ObjectSubclass for MyObject {
		const NAME: &'static str = "MyObject";
		type Type = super::MyObject;
	}

	impl ObjectImpl for MyObject {}
}
glib::wrapper! {
	pub struct MyObject(ObjectSubclass<imp::MyObject>);
}

impl MyObject {
	pub fn new(enum_value: MyEnum) -> Self {
		Object::builder()
			.property("enum_value", enum_value)
			.build()
	}
}


fn main() {
    println!("Hello, world!");
}
