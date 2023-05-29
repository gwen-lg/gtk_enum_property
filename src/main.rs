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
    use gtk::glib::{self, ParamSpec};
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

	impl ObjectImpl for MyObject {
        fn properties() -> &'static [ParamSpec] {
            use glib::once_cell::sync::Lazy;
            static PROPERTIES: Lazy<[glib::ParamSpec; 1usize]> = Lazy::new(|| [
                <<RefCell<
                    MyEnum,
                > as glib::Property>::Value as glib::HasParamSpec>::param_spec_builder()(
                        "enum-value",
                    )
                    .readwrite()
                    .build(),
            ]);
            PROPERTIES.as_ref()
		}
    }
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
