#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use gtk::glib;
use gtk::prelude::*;
use glib::{Object, Enum};
#[enum_type(name = "MyEnum")]
enum MyEnum {
    #[default]
    Value1,
    Value2,
}
#[automatically_derived]
impl ::core::default::Default for MyEnum {
    #[inline]
    fn default() -> MyEnum {
        Self::Value1
    }
}
#[automatically_derived]
impl ::core::clone::Clone for MyEnum {
    #[inline]
    fn clone(&self) -> MyEnum {
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for MyEnum {}
impl glib::translate::IntoGlib for MyEnum {
    type GlibType = i32;
    #[inline]
    fn into_glib(self) -> i32 {
        self as i32
    }
}
impl glib::translate::TryFromGlib<i32> for MyEnum {
    type Error = i32;
    #[inline]
    unsafe fn try_from_glib(value: i32) -> ::core::result::Result<Self, i32> {
        let from_glib = || {
            if value == MyEnum::Value1 as i32 {
                return ::core::option::Option::Some(MyEnum::Value1);
            }
            if value == MyEnum::Value2 as i32 {
                return ::core::option::Option::Some(MyEnum::Value2);
            }
            ::core::option::Option::None
        };
        from_glib().ok_or(value)
    }
}
impl glib::translate::FromGlib<i32> for MyEnum {
    #[inline]
    unsafe fn from_glib(value: i32) -> Self {
        use glib::translate::TryFromGlib;
        Self::try_from_glib(value).unwrap()
    }
}
impl glib::value::ValueType for MyEnum {
    type Type = Self;
}
unsafe impl<'a> glib::value::FromValue<'a> for MyEnum {
    type Checker = glib::value::GenericValueTypeChecker<Self>;
    #[inline]
    unsafe fn from_value(value: &'a glib::value::Value) -> Self {
        glib::translate::from_glib(
            glib::gobject_ffi::g_value_get_enum(
                glib::translate::ToGlibPtr::to_glib_none(value).0,
            ),
        )
    }
}
impl glib::value::ToValue for MyEnum {
    #[inline]
    fn to_value(&self) -> glib::value::Value {
        let mut value = glib::value::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_enum(
                glib::translate::ToGlibPtrMut::to_glib_none_mut(&mut value).0,
                glib::translate::IntoGlib::into_glib(*self),
            )
        }
        value
    }
    #[inline]
    fn value_type(&self) -> glib::Type {
        <Self as glib::StaticType>::static_type()
    }
}
impl ::std::convert::From<MyEnum> for glib::Value {
    #[inline]
    fn from(v: MyEnum) -> Self {
        glib::value::ToValue::to_value(&v)
    }
}
impl glib::StaticType for MyEnum {
    #[inline]
    fn static_type() -> glib::Type {
        static ONCE: ::std::sync::Once = ::std::sync::Once::new();
        static mut TYPE: glib::Type = glib::Type::INVALID;
        ONCE.call_once(|| {
            static mut VALUES: [glib::gobject_ffi::GEnumValue; 3usize] = [
                glib::gobject_ffi::GEnumValue {
                    value: MyEnum::Value1 as i32,
                    value_name: "Value1\0" as *const _ as *const _,
                    value_nick: "value1\0" as *const _ as *const _,
                },
                glib::gobject_ffi::GEnumValue {
                    value: MyEnum::Value2 as i32,
                    value_name: "Value2\0" as *const _ as *const _,
                    value_nick: "value2\0" as *const _ as *const _,
                },
                glib::gobject_ffi::GEnumValue {
                    value: 0,
                    value_name: ::std::ptr::null(),
                    value_nick: ::std::ptr::null(),
                },
            ];
            let name = ::std::ffi::CString::new("MyEnum").expect("CString::new failed");
            unsafe {
                let type_ = glib::gobject_ffi::g_enum_register_static(
                    name.as_ptr(),
                    VALUES.as_ptr(),
                );
                let type_: glib::Type = glib::translate::from_glib(type_);
                if !type_.is_valid() {
                    ::core::panicking::panic("assertion failed: type_.is_valid()")
                }
                TYPE = type_;
            }
        });
        unsafe { TYPE }
    }
}
impl glib::HasParamSpec for MyEnum {
    type ParamSpec = glib::ParamSpecEnum;
    type SetValue = Self;
    type BuilderFn = fn(
        &::core::primitive::str,
        Self,
    ) -> glib::ParamSpecEnumBuilder<Self>;
    fn param_spec_builder() -> Self::BuilderFn {
        |name, default_value| Self::ParamSpec::builder_with_default(name, default_value)
    }
}
mod imp {
    use std::cell::RefCell;
    use gtk::prelude::*;
    use gtk::subclass::prelude::*;
    use gtk::glib;
    use glib::Properties;
    use super::MyEnum;
    #[properties(wrapper_type = super::MyObject)]
    pub struct MyObject {
        #[property(get, set)]
        pub enum_value: RefCell<MyEnum>,
    }
    use glib::{PropertyGet, PropertySet, ToValue};
    #[repr(usize)]
    enum DerivedPropertiesEnum {
        EnumValue,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for DerivedPropertiesEnum {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(f, "EnumValue")
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for DerivedPropertiesEnum {}
    #[automatically_derived]
    impl ::core::clone::Clone for DerivedPropertiesEnum {
        #[inline]
        fn clone(&self) -> DerivedPropertiesEnum {
            *self
        }
    }
    impl std::convert::TryFrom<usize> for DerivedPropertiesEnum {
        type Error = usize;
        fn try_from(
            item: usize,
        ) -> ::core::result::Result<
            Self,
            <Self as std::convert::TryFrom<usize>>::Error,
        > {
            match item {
                0usize => ::core::result::Result::Ok(Self::EnumValue),
                _ => ::core::result::Result::Err(item),
            }
        }
    }
    impl glib::subclass::object::DerivedObjectProperties for MyObject {
        fn derived_properties() -> &'static [glib::ParamSpec] {
            use glib::ParamSpecBuilderExt;
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
        fn derived_property(&self, id: usize, pspec: &glib::ParamSpec) -> glib::Value {
            let prop: DerivedPropertiesEnum = std::convert::TryFrom::try_from(id - 1)
                .unwrap_or_else(|_| ::core::panicking::panic_fmt(
                    format_args!("property not defined {0}", pspec.name()),
                ));
            match prop {
                DerivedPropertiesEnum::EnumValue => {
                    glib::PropertyGet::get(
                        &self.enum_value,
                        |v| ::std::convert::From::from(v),
                    )
                }
                _ => {
                    ::core::panicking::panic_fmt(
                        format_args!("missing getter for property {0}", pspec.name()),
                    )
                }
            }
        }
        fn derived_set_property(
            &self,
            id: usize,
            value: &glib::Value,
            pspec: &glib::ParamSpec,
        ) {
            let prop: DerivedPropertiesEnum = std::convert::TryFrom::try_from(id - 1)
                .unwrap_or_else(|_| ::core::panicking::panic_fmt(
                    format_args!("property not defined {0}", pspec.name()),
                ));
            match prop {
                DerivedPropertiesEnum::EnumValue => {
                    glib::PropertySet::set(
                        &self.enum_value,
                        glib::Value::get(value)
                            .unwrap_or_else(|err| ::core::panicking::panic_fmt(
                                format_args!(
                                    "Invalid conversion from `glib::value::Value` to `{0}` inside setter for property `{1}`: {2:?}",
                                    ::std::any::type_name::<< RefCell < MyEnum > as
                                    glib::Property >::Value > (), "enum-value", err
                                ),
                            )),
                    );
                }
                _ => {
                    ::core::panicking::panic_fmt(
                        format_args!("missing setter for property {0}", pspec.name()),
                    )
                }
            }
        }
    }
    #[allow(dead_code)]
    impl super::MyObject {
        pub fn enum_value(&self) -> <RefCell<MyEnum> as glib::Property>::Value {
            self.property::<<RefCell<MyEnum> as glib::Property>::Value>("enum-value")
        }
        pub fn set_enum_value<'a>(
            &self,
            value: impl std::borrow::Borrow<
                <<RefCell<
                    MyEnum,
                > as glib::Property>::Value as glib::HasParamSpec>::SetValue,
            >,
        ) {
            self.set_property_from_value(
                "enum-value",
                &::std::convert::From::from(std::borrow::Borrow::borrow(&value)),
            )
        }
        pub fn connect_enum_value_notify<F: Fn(&Self) + 'static>(
            &self,
            f: F,
        ) -> glib::SignalHandlerId {
            self.connect_notify_local(
                ::core::option::Option::Some("enum-value"),
                move |this, _| { f(this) },
            )
        }
        pub fn notify_enum_value(&self) {
            self.notify_by_pspec(
                &<<Self as glib::object::ObjectSubclassIs>::Subclass as glib::subclass::object::DerivedObjectProperties>::derived_properties()[DerivedPropertiesEnum::EnumValue
                    as usize],
            );
        }
    }
    #[automatically_derived]
    impl ::core::default::Default for MyObject {
        #[inline]
        fn default() -> MyObject {
            MyObject {
                enum_value: ::core::default::Default::default(),
            }
        }
    }
    impl ObjectSubclass for MyObject {
        type ParentType = glib::Object;
        type Interfaces = ();
        type Class = glib::subclass::basic::ClassStruct<Self>;
        type Instance = glib::subclass::basic::InstanceStruct<Self>;
        #[inline]
        fn new() -> Self {
            ::std::default::Default::default()
        }
        const NAME: &'static str = "MyObject";
        type Type = super::MyObject;
    }
    unsafe impl glib::subclass::types::ObjectSubclassType for MyObject {
        #[inline]
        fn type_data() -> ::std::ptr::NonNull<glib::subclass::TypeData> {
            static mut DATA: glib::subclass::TypeData = glib::subclass::types::INIT_TYPE_DATA;
            unsafe { ::std::ptr::NonNull::from(&mut DATA) }
        }
        #[inline]
        fn type_() -> glib::Type {
            static ONCE: ::std::sync::Once = ::std::sync::Once::new();
            ONCE.call_once(|| {
                glib::subclass::register_type::<Self>();
            });
            unsafe {
                let data = Self::type_data();
                let type_ = data.as_ref().type_();
                type_
            }
        }
    }
    #[doc(hidden)]
    impl glib::subclass::types::FromObject for MyObject {
        type FromObjectType = <Self as glib::subclass::types::ObjectSubclass>::Type;
        #[inline]
        fn from_object(obj: &Self::FromObjectType) -> &Self {
            <Self as glib::subclass::types::ObjectSubclassExt>::from_obj(obj)
        }
    }
    #[doc(hidden)]
    impl glib::clone::Downgrade for MyObject {
        type Weak = glib::subclass::ObjectImplWeakRef<MyObject>;
        #[inline]
        fn downgrade(&self) -> Self::Weak {
            let ref_counted = glib::subclass::prelude::ObjectSubclassExt::ref_counted(
                self,
            );
            glib::clone::Downgrade::downgrade(&ref_counted)
        }
    }
    impl MyObject {
        #[inline]
        pub fn downgrade(&self) -> <Self as glib::clone::Downgrade>::Weak {
            glib::clone::Downgrade::downgrade(self)
        }
    }
    #[doc(hidden)]
    impl ::std::borrow::ToOwned for MyObject {
        type Owned = glib::subclass::ObjectImplRef<MyObject>;
        #[inline]
        fn to_owned(&self) -> Self::Owned {
            glib::subclass::prelude::ObjectSubclassExt::ref_counted(self)
        }
    }
    #[doc(hidden)]
    impl ::std::borrow::Borrow<MyObject> for glib::subclass::ObjectImplRef<MyObject> {
        #[inline]
        fn borrow(&self) -> &MyObject {
            self
        }
    }
    impl ObjectImpl for MyObject {}
}
#[repr(transparent)]
pub struct MyObject {
    inner: ::glib::object::TypedObjectRef<imp::MyObject, ()>,
    phantom: std::marker::PhantomData<()>,
}
impl std::clone::Clone for MyObject {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            inner: std::clone::Clone::clone(&self.inner),
            phantom: std::marker::PhantomData,
        }
    }
}
impl std::hash::Hash for MyObject {
    #[inline]
    fn hash<H>(&self, state: &mut H)
    where
        H: std::hash::Hasher,
    {
        std::hash::Hash::hash(&self.inner, state);
    }
}
impl<OT: ::glib::object::ObjectType> std::cmp::PartialEq<OT> for MyObject {
    #[inline]
    fn eq(&self, other: &OT) -> bool {
        std::cmp::PartialEq::eq(
            &*self.inner,
            ::glib::object::ObjectType::as_object_ref(other),
        )
    }
}
impl std::cmp::Eq for MyObject {}
impl<OT: ::glib::object::ObjectType> std::cmp::PartialOrd<OT> for MyObject {
    #[inline]
    fn partial_cmp(&self, other: &OT) -> Option<std::cmp::Ordering> {
        std::cmp::PartialOrd::partial_cmp(
            &*self.inner,
            ::glib::object::ObjectType::as_object_ref(other),
        )
    }
}
impl std::cmp::Ord for MyObject {
    #[inline]
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        std::cmp::Ord::cmp(
            &*self.inner,
            ::glib::object::ObjectType::as_object_ref(other),
        )
    }
}
impl std::fmt::Debug for MyObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("MyObject").field("inner", &self.inner).finish()
    }
}
#[doc(hidden)]
impl From<MyObject> for ::glib::object::ObjectRef {
    #[inline]
    fn from(s: MyObject) -> ::glib::object::ObjectRef {
        s.inner.into_inner()
    }
}
#[doc(hidden)]
impl ::glib::translate::UnsafeFrom<::glib::object::ObjectRef> for MyObject {
    #[inline]
    unsafe fn unsafe_from(t: ::glib::object::ObjectRef) -> Self {
        MyObject {
            inner: ::glib::object::TypedObjectRef::new(t),
            phantom: std::marker::PhantomData,
        }
    }
}
#[doc(hidden)]
impl ::glib::translate::GlibPtrDefault for MyObject {
    type GlibType = *mut <imp::MyObject as ::glib::subclass::types::ObjectSubclass>::Instance;
}
#[doc(hidden)]
unsafe impl ::glib::translate::TransparentPtrType for MyObject {}
#[doc(hidden)]
unsafe impl ::glib::object::ObjectType for MyObject {
    type GlibType = <imp::MyObject as ::glib::subclass::types::ObjectSubclass>::Instance;
    type GlibClassType = <imp::MyObject as ::glib::subclass::types::ObjectSubclass>::Class;
    #[inline]
    fn as_object_ref(&self) -> &::glib::object::ObjectRef {
        &self.inner
    }
    #[inline]
    fn as_ptr(&self) -> *mut Self::GlibType {
        unsafe {
            *(self as *const Self
                as *const *const <imp::MyObject as ::glib::subclass::types::ObjectSubclass>::Instance)
                as *mut <imp::MyObject as ::glib::subclass::types::ObjectSubclass>::Instance
        }
    }
    #[inline]
    unsafe fn from_glib_ptr_borrow<'a>(ptr: *const *const Self::GlibType) -> &'a Self {
        &*(ptr as *const Self)
    }
}
#[doc(hidden)]
impl AsRef<::glib::object::ObjectRef> for MyObject {
    #[inline]
    fn as_ref(&self) -> &::glib::object::ObjectRef {
        &self.inner
    }
}
#[doc(hidden)]
impl AsRef<Self> for MyObject {
    #[inline]
    fn as_ref(&self) -> &Self {
        self
    }
}
#[doc(hidden)]
unsafe impl ::glib::object::IsA<Self> for MyObject {}
#[doc(hidden)]
impl ::glib::subclass::types::FromObject for MyObject {
    type FromObjectType = Self;
    #[inline]
    fn from_object(obj: &Self::FromObjectType) -> &Self {
        obj
    }
}
#[doc(hidden)]
impl<
    'a,
> ::glib::translate::ToGlibPtr<
    'a,
    *const <imp::MyObject as ::glib::subclass::types::ObjectSubclass>::Instance,
> for MyObject {
    type Storage = <::glib::object::ObjectRef as ::glib::translate::ToGlibPtr<
        'a,
        *mut ::glib::gobject_ffi::GObject,
    >>::Storage;
    #[inline]
    fn to_glib_none(
        &'a self,
    ) -> ::glib::translate::Stash<
        'a,
        *const <imp::MyObject as ::glib::subclass::types::ObjectSubclass>::Instance,
        Self,
    > {
        let stash = ::glib::translate::ToGlibPtr::to_glib_none(&*self.inner);
        ::glib::translate::Stash(stash.0 as *const _, stash.1)
    }
    #[inline]
    fn to_glib_full(
        &self,
    ) -> *const <imp::MyObject as ::glib::subclass::types::ObjectSubclass>::Instance {
        ::glib::translate::ToGlibPtr::to_glib_full(&*self.inner) as *const _
    }
}
#[doc(hidden)]
impl<
    'a,
> ::glib::translate::ToGlibPtr<
    'a,
    *mut <imp::MyObject as ::glib::subclass::types::ObjectSubclass>::Instance,
> for MyObject {
    type Storage = <::glib::object::ObjectRef as ::glib::translate::ToGlibPtr<
        'a,
        *mut ::glib::gobject_ffi::GObject,
    >>::Storage;
    #[inline]
    fn to_glib_none(
        &'a self,
    ) -> ::glib::translate::Stash<
        'a,
        *mut <imp::MyObject as ::glib::subclass::types::ObjectSubclass>::Instance,
        Self,
    > {
        let stash = ::glib::translate::ToGlibPtr::to_glib_none(&*self.inner);
        ::glib::translate::Stash(stash.0 as *mut _, stash.1)
    }
    #[inline]
    fn to_glib_full(
        &self,
    ) -> *mut <imp::MyObject as ::glib::subclass::types::ObjectSubclass>::Instance {
        ::glib::translate::ToGlibPtr::to_glib_full(&*self.inner) as *mut _
    }
}
#[doc(hidden)]
impl ::glib::translate::IntoGlibPtr<
    *mut <imp::MyObject as ::glib::subclass::types::ObjectSubclass>::Instance,
> for MyObject {
    #[inline]
    unsafe fn into_glib_ptr(
        self,
    ) -> *mut <imp::MyObject as ::glib::subclass::types::ObjectSubclass>::Instance {
        let s = std::mem::ManuallyDrop::new(self);
        ::glib::translate::ToGlibPtr::<
            *const <imp::MyObject as ::glib::subclass::types::ObjectSubclass>::Instance,
        >::to_glib_none(&*s)
            .0 as *mut _
    }
}
#[doc(hidden)]
impl ::glib::translate::IntoGlibPtr<
    *const <imp::MyObject as ::glib::subclass::types::ObjectSubclass>::Instance,
> for MyObject {
    #[inline]
    unsafe fn into_glib_ptr(
        self,
    ) -> *const <imp::MyObject as ::glib::subclass::types::ObjectSubclass>::Instance {
        let s = std::mem::ManuallyDrop::new(self);
        ::glib::translate::ToGlibPtr::<
            *const <imp::MyObject as ::glib::subclass::types::ObjectSubclass>::Instance,
        >::to_glib_none(&*s)
            .0 as *const _
    }
}
#[doc(hidden)]
impl<
    'a,
> ::glib::translate::ToGlibContainerFromSlice<
    'a,
    *mut *mut <imp::MyObject as ::glib::subclass::types::ObjectSubclass>::Instance,
> for MyObject {
    type Storage = (
        std::marker::PhantomData<&'a [Self]>,
        Option<
            Vec<
                *mut <imp::MyObject as ::glib::subclass::types::ObjectSubclass>::Instance,
            >,
        >,
    );
    fn to_glib_none_from_slice(
        t: &'a [Self],
    ) -> (
        *mut *mut <imp::MyObject as ::glib::subclass::types::ObjectSubclass>::Instance,
        Self::Storage,
    ) {
        let mut v_ptr = Vec::with_capacity(t.len() + 1);
        unsafe {
            let ptr = v_ptr.as_mut_ptr();
            std::ptr::copy_nonoverlapping(
                t.as_ptr()
                    as *mut *mut <imp::MyObject as ::glib::subclass::types::ObjectSubclass>::Instance,
                ptr,
                t.len(),
            );
            std::ptr::write(ptr.add(t.len()), std::ptr::null_mut());
            v_ptr.set_len(t.len() + 1);
        }
        (
            v_ptr.as_ptr()
                as *mut *mut <imp::MyObject as ::glib::subclass::types::ObjectSubclass>::Instance,
            (std::marker::PhantomData, Some(v_ptr)),
        )
    }
    fn to_glib_container_from_slice(
        t: &'a [Self],
    ) -> (
        *mut *mut <imp::MyObject as ::glib::subclass::types::ObjectSubclass>::Instance,
        Self::Storage,
    ) {
        let v_ptr = unsafe {
            let v_ptr = ::glib::ffi::g_malloc(
                std::mem::size_of::<
                    *mut <imp::MyObject as ::glib::subclass::types::ObjectSubclass>::Instance,
                >() * (t.len() + 1),
            )
                as *mut *mut <imp::MyObject as ::glib::subclass::types::ObjectSubclass>::Instance;
            std::ptr::copy_nonoverlapping(
                t.as_ptr()
                    as *mut *mut <imp::MyObject as ::glib::subclass::types::ObjectSubclass>::Instance,
                v_ptr,
                t.len(),
            );
            std::ptr::write(v_ptr.add(t.len()), std::ptr::null_mut());
            v_ptr
        };
        (v_ptr, (std::marker::PhantomData, None))
    }
    fn to_glib_full_from_slice(
        t: &[Self],
    ) -> *mut *mut <imp::MyObject as ::glib::subclass::types::ObjectSubclass>::Instance {
        unsafe {
            let v_ptr = ::glib::ffi::g_malloc(
                std::mem::size_of::<
                    *mut <imp::MyObject as ::glib::subclass::types::ObjectSubclass>::Instance,
                >() * (t.len() + 1),
            )
                as *mut *mut <imp::MyObject as ::glib::subclass::types::ObjectSubclass>::Instance;
            for (i, s) in t.iter().enumerate() {
                std::ptr::write(
                    v_ptr.add(i),
                    ::glib::translate::ToGlibPtr::to_glib_full(s),
                );
            }
            std::ptr::write(v_ptr.add(t.len()), std::ptr::null_mut());
            v_ptr
        }
    }
}
#[doc(hidden)]
impl<
    'a,
> ::glib::translate::ToGlibContainerFromSlice<
    'a,
    *const *mut <imp::MyObject as ::glib::subclass::types::ObjectSubclass>::Instance,
> for MyObject {
    type Storage = (
        std::marker::PhantomData<&'a [Self]>,
        Option<
            Vec<
                *mut <imp::MyObject as ::glib::subclass::types::ObjectSubclass>::Instance,
            >,
        >,
    );
    fn to_glib_none_from_slice(
        t: &'a [Self],
    ) -> (
        *const *mut <imp::MyObject as ::glib::subclass::types::ObjectSubclass>::Instance,
        Self::Storage,
    ) {
        let (ptr, stash) = ::glib::translate::ToGlibContainerFromSlice::<
            'a,
            *mut *mut <imp::MyObject as ::glib::subclass::types::ObjectSubclass>::Instance,
        >::to_glib_none_from_slice(t);
        (
            ptr
                as *const *mut <imp::MyObject as ::glib::subclass::types::ObjectSubclass>::Instance,
            stash,
        )
    }
    fn to_glib_container_from_slice(
        _: &'a [Self],
    ) -> (
        *const *mut <imp::MyObject as ::glib::subclass::types::ObjectSubclass>::Instance,
        Self::Storage,
    ) {
        ::core::panicking::panic("not implemented")
    }
    fn to_glib_full_from_slice(
        _: &[Self],
    ) -> *const *mut <imp::MyObject as ::glib::subclass::types::ObjectSubclass>::Instance {
        ::core::panicking::panic("not implemented")
    }
}
#[doc(hidden)]
impl ::glib::translate::FromGlibPtrNone<
    *mut <imp::MyObject as ::glib::subclass::types::ObjectSubclass>::Instance,
> for MyObject {
    #[inline]
    #[allow(clippy::cast_ptr_alignment)]
    unsafe fn from_glib_none(
        ptr: *mut <imp::MyObject as ::glib::subclass::types::ObjectSubclass>::Instance,
    ) -> Self {
        if true {
            if !!ptr.is_null() {
                ::core::panicking::panic("assertion failed: !ptr.is_null()")
            }
        }
        if true {
            if !::glib::types::instance_of::<Self>(ptr as *const _) {
                ::core::panicking::panic(
                    "assertion failed: ::glib::types::instance_of::<Self>(ptr as *const _)",
                )
            }
        }
        MyObject {
            inner: ::glib::object::TypedObjectRef::new(
                ::glib::translate::from_glib_none(ptr as *mut _),
            ),
            phantom: std::marker::PhantomData,
        }
    }
}
#[doc(hidden)]
impl ::glib::translate::FromGlibPtrNone<
    *const <imp::MyObject as ::glib::subclass::types::ObjectSubclass>::Instance,
> for MyObject {
    #[inline]
    #[allow(clippy::cast_ptr_alignment)]
    unsafe fn from_glib_none(
        ptr: *const <imp::MyObject as ::glib::subclass::types::ObjectSubclass>::Instance,
    ) -> Self {
        if true {
            if !!ptr.is_null() {
                ::core::panicking::panic("assertion failed: !ptr.is_null()")
            }
        }
        if true {
            if !::glib::types::instance_of::<Self>(ptr as *const _) {
                ::core::panicking::panic(
                    "assertion failed: ::glib::types::instance_of::<Self>(ptr as *const _)",
                )
            }
        }
        MyObject {
            inner: ::glib::object::TypedObjectRef::new(
                ::glib::translate::from_glib_none(ptr as *mut _),
            ),
            phantom: std::marker::PhantomData,
        }
    }
}
#[doc(hidden)]
impl ::glib::translate::FromGlibPtrFull<
    *mut <imp::MyObject as ::glib::subclass::types::ObjectSubclass>::Instance,
> for MyObject {
    #[inline]
    #[allow(clippy::cast_ptr_alignment)]
    unsafe fn from_glib_full(
        ptr: *mut <imp::MyObject as ::glib::subclass::types::ObjectSubclass>::Instance,
    ) -> Self {
        if true {
            if !!ptr.is_null() {
                ::core::panicking::panic("assertion failed: !ptr.is_null()")
            }
        }
        if true {
            if !::glib::types::instance_of::<Self>(ptr as *const _) {
                ::core::panicking::panic(
                    "assertion failed: ::glib::types::instance_of::<Self>(ptr as *const _)",
                )
            }
        }
        MyObject {
            inner: ::glib::object::TypedObjectRef::new(
                ::glib::translate::from_glib_full(ptr as *mut _),
            ),
            phantom: std::marker::PhantomData,
        }
    }
}
#[doc(hidden)]
impl ::glib::translate::FromGlibPtrBorrow<
    *mut <imp::MyObject as ::glib::subclass::types::ObjectSubclass>::Instance,
> for MyObject {
    #[inline]
    #[allow(clippy::cast_ptr_alignment)]
    unsafe fn from_glib_borrow(
        ptr: *mut <imp::MyObject as ::glib::subclass::types::ObjectSubclass>::Instance,
    ) -> ::glib::translate::Borrowed<Self> {
        if true {
            if !!ptr.is_null() {
                ::core::panicking::panic("assertion failed: !ptr.is_null()")
            }
        }
        if true {
            if !::glib::types::instance_of::<Self>(ptr as *const _) {
                ::core::panicking::panic(
                    "assertion failed: ::glib::types::instance_of::<Self>(ptr as *const _)",
                )
            }
        }
        ::glib::translate::Borrowed::new(MyObject {
            inner: ::glib::object::TypedObjectRef::new(
                ::glib::translate::from_glib_borrow::<
                    _,
                    ::glib::object::ObjectRef,
                >(ptr as *mut _)
                    .into_inner(),
            ),
            phantom: std::marker::PhantomData,
        })
    }
}
#[doc(hidden)]
impl ::glib::translate::FromGlibPtrBorrow<
    *const <imp::MyObject as ::glib::subclass::types::ObjectSubclass>::Instance,
> for MyObject {
    #[inline]
    #[allow(clippy::cast_ptr_alignment)]
    unsafe fn from_glib_borrow(
        ptr: *const <imp::MyObject as ::glib::subclass::types::ObjectSubclass>::Instance,
    ) -> ::glib::translate::Borrowed<Self> {
        ::glib::translate::from_glib_borrow::<
            _,
            Self,
        >(
            ptr
                as *mut <imp::MyObject as ::glib::subclass::types::ObjectSubclass>::Instance,
        )
    }
}
#[doc(hidden)]
impl ::glib::translate::FromGlibContainerAsVec<
    *mut <imp::MyObject as ::glib::subclass::types::ObjectSubclass>::Instance,
    *mut *mut <imp::MyObject as ::glib::subclass::types::ObjectSubclass>::Instance,
> for MyObject {
    unsafe fn from_glib_none_num_as_vec(
        ptr: *mut *mut <imp::MyObject as ::glib::subclass::types::ObjectSubclass>::Instance,
        num: usize,
    ) -> Vec<Self> {
        if num == 0 || ptr.is_null() {
            return Vec::new();
        }
        let mut res = Vec::<Self>::with_capacity(num);
        let res_ptr = res.as_mut_ptr();
        for i in 0..num {
            ::std::ptr::write(
                res_ptr.add(i),
                ::glib::translate::from_glib_none(std::ptr::read(ptr.add(i))),
            );
        }
        res.set_len(num);
        res
    }
    unsafe fn from_glib_container_num_as_vec(
        ptr: *mut *mut <imp::MyObject as ::glib::subclass::types::ObjectSubclass>::Instance,
        num: usize,
    ) -> Vec<Self> {
        let res = ::glib::translate::FromGlibContainerAsVec::from_glib_none_num_as_vec(
            ptr,
            num,
        );
        ::glib::ffi::g_free(ptr as *mut _);
        res
    }
    unsafe fn from_glib_full_num_as_vec(
        ptr: *mut *mut <imp::MyObject as ::glib::subclass::types::ObjectSubclass>::Instance,
        num: usize,
    ) -> Vec<Self> {
        if num == 0 || ptr.is_null() {
            ::glib::ffi::g_free(ptr as *mut _);
            return Vec::new();
        }
        let mut res = Vec::with_capacity(num);
        let res_ptr = res.as_mut_ptr();
        ::std::ptr::copy_nonoverlapping(ptr as *mut Self, res_ptr, num);
        res.set_len(num);
        ::glib::ffi::g_free(ptr as *mut _);
        res
    }
}
#[doc(hidden)]
impl ::glib::translate::FromGlibPtrArrayContainerAsVec<
    *mut <imp::MyObject as ::glib::subclass::types::ObjectSubclass>::Instance,
    *mut *mut <imp::MyObject as ::glib::subclass::types::ObjectSubclass>::Instance,
> for MyObject {
    unsafe fn from_glib_none_as_vec(
        ptr: *mut *mut <imp::MyObject as ::glib::subclass::types::ObjectSubclass>::Instance,
    ) -> Vec<Self> {
        ::glib::translate::FromGlibContainerAsVec::from_glib_none_num_as_vec(
            ptr,
            ::glib::translate::c_ptr_array_len(ptr),
        )
    }
    unsafe fn from_glib_container_as_vec(
        ptr: *mut *mut <imp::MyObject as ::glib::subclass::types::ObjectSubclass>::Instance,
    ) -> Vec<Self> {
        ::glib::translate::FromGlibContainerAsVec::from_glib_container_num_as_vec(
            ptr,
            ::glib::translate::c_ptr_array_len(ptr),
        )
    }
    unsafe fn from_glib_full_as_vec(
        ptr: *mut *mut <imp::MyObject as ::glib::subclass::types::ObjectSubclass>::Instance,
    ) -> Vec<Self> {
        ::glib::translate::FromGlibContainerAsVec::from_glib_full_num_as_vec(
            ptr,
            ::glib::translate::c_ptr_array_len(ptr),
        )
    }
}
#[doc(hidden)]
impl ::glib::translate::FromGlibContainerAsVec<
    *mut <imp::MyObject as ::glib::subclass::types::ObjectSubclass>::Instance,
    *const *mut <imp::MyObject as ::glib::subclass::types::ObjectSubclass>::Instance,
> for MyObject {
    unsafe fn from_glib_none_num_as_vec(
        ptr: *const *mut <imp::MyObject as ::glib::subclass::types::ObjectSubclass>::Instance,
        num: usize,
    ) -> Vec<Self> {
        ::glib::translate::FromGlibContainerAsVec::from_glib_none_num_as_vec(
            ptr as *mut *mut _,
            num,
        )
    }
    unsafe fn from_glib_container_num_as_vec(
        _: *const *mut <imp::MyObject as ::glib::subclass::types::ObjectSubclass>::Instance,
        _: usize,
    ) -> Vec<Self> {
        ::core::panicking::panic("not implemented")
    }
    unsafe fn from_glib_full_num_as_vec(
        _: *const *mut <imp::MyObject as ::glib::subclass::types::ObjectSubclass>::Instance,
        _: usize,
    ) -> Vec<Self> {
        ::core::panicking::panic("not implemented")
    }
}
#[doc(hidden)]
impl ::glib::translate::FromGlibPtrArrayContainerAsVec<
    *mut <imp::MyObject as ::glib::subclass::types::ObjectSubclass>::Instance,
    *const *mut <imp::MyObject as ::glib::subclass::types::ObjectSubclass>::Instance,
> for MyObject {
    unsafe fn from_glib_none_as_vec(
        ptr: *const *mut <imp::MyObject as ::glib::subclass::types::ObjectSubclass>::Instance,
    ) -> Vec<Self> {
        ::glib::translate::FromGlibPtrArrayContainerAsVec::from_glib_none_as_vec(
            ptr as *mut *mut _,
        )
    }
    unsafe fn from_glib_container_as_vec(
        _: *const *mut <imp::MyObject as ::glib::subclass::types::ObjectSubclass>::Instance,
    ) -> Vec<Self> {
        ::core::panicking::panic("not implemented")
    }
    unsafe fn from_glib_full_as_vec(
        _: *const *mut <imp::MyObject as ::glib::subclass::types::ObjectSubclass>::Instance,
    ) -> Vec<Self> {
        ::core::panicking::panic("not implemented")
    }
}
impl ::glib::types::StaticType for MyObject {
    #[inline]
    fn static_type() -> ::glib::types::Type {
        #[allow(unused_unsafe)]
        unsafe {
            ::glib::translate::from_glib(
                ::glib::translate::IntoGlib::into_glib(
                    <imp::MyObject as ::glib::subclass::types::ObjectSubclassType>::type_(),
                ),
            )
        }
    }
}
#[doc(hidden)]
impl ::glib::value::ValueType for MyObject {
    type Type = MyObject;
}
#[doc(hidden)]
impl ::glib::value::ValueTypeOptional for MyObject {}
#[doc(hidden)]
unsafe impl<'a> ::glib::value::FromValue<'a> for MyObject {
    type Checker = ::glib::object::ObjectValueTypeChecker<Self>;
    #[inline]
    unsafe fn from_value(value: &'a ::glib::Value) -> Self {
        let ptr = ::glib::gobject_ffi::g_value_dup_object(
            ::glib::translate::ToGlibPtr::to_glib_none(value).0,
        );
        if true {
            if !!ptr.is_null() {
                ::core::panicking::panic("assertion failed: !ptr.is_null()")
            }
        }
        if true {
            match (&(*ptr).ref_count, &0) {
                (left_val, right_val) => {
                    if *left_val == *right_val {
                        let kind = ::core::panicking::AssertKind::Ne;
                        ::core::panicking::assert_failed(
                            kind,
                            &*left_val,
                            &*right_val,
                            ::core::option::Option::None,
                        );
                    }
                }
            };
        }
        <Self as ::glib::translate::FromGlibPtrFull<
            *mut <imp::MyObject as ::glib::subclass::types::ObjectSubclass>::Instance,
        >>::from_glib_full(
            ptr
                as *mut <imp::MyObject as ::glib::subclass::types::ObjectSubclass>::Instance,
        )
    }
}
#[doc(hidden)]
unsafe impl<'a> ::glib::value::FromValue<'a> for &'a MyObject {
    type Checker = ::glib::object::ObjectValueTypeChecker<Self>;
    #[inline]
    unsafe fn from_value(value: &'a ::glib::Value) -> Self {
        if true {
            match (
                &std::mem::size_of::<Self>(),
                &std::mem::size_of::<::glib::ffi::gpointer>(),
            ) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        let kind = ::core::panicking::AssertKind::Eq;
                        ::core::panicking::assert_failed(
                            kind,
                            &*left_val,
                            &*right_val,
                            ::core::option::Option::None,
                        );
                    }
                }
            };
        }
        let value = &*(value as *const ::glib::Value
            as *const ::glib::gobject_ffi::GValue);
        if true {
            if !!value.data[0].v_pointer.is_null() {
                ::core::panicking::panic(
                    "assertion failed: !value.data[0].v_pointer.is_null()",
                )
            }
        }
        if true {
            match (
                &(*(value.data[0].v_pointer as *const ::glib::gobject_ffi::GObject))
                    .ref_count,
                &0,
            ) {
                (left_val, right_val) => {
                    if *left_val == *right_val {
                        let kind = ::core::panicking::AssertKind::Ne;
                        ::core::panicking::assert_failed(
                            kind,
                            &*left_val,
                            &*right_val,
                            ::core::option::Option::None,
                        );
                    }
                }
            };
        }
        <MyObject as ::glib::object::ObjectType>::from_glib_ptr_borrow(
            &value.data[0].v_pointer as *const ::glib::ffi::gpointer
                as *const *const <imp::MyObject as ::glib::subclass::types::ObjectSubclass>::Instance,
        )
    }
}
#[doc(hidden)]
impl ::glib::value::ToValue for MyObject {
    #[inline]
    fn to_value(&self) -> ::glib::Value {
        unsafe {
            let mut value = ::glib::Value::from_type_unchecked(
                <Self as ::glib::StaticType>::static_type(),
            );
            ::glib::gobject_ffi::g_value_take_object(
                ::glib::translate::ToGlibPtrMut::to_glib_none_mut(&mut value).0,
                ::glib::translate::ToGlibPtr::<
                    *mut <imp::MyObject as ::glib::subclass::types::ObjectSubclass>::Instance,
                >::to_glib_full(self) as *mut _,
            );
            value
        }
    }
    #[inline]
    fn value_type(&self) -> ::glib::Type {
        <Self as ::glib::StaticType>::static_type()
    }
}
#[doc(hidden)]
impl ::std::convert::From<MyObject> for ::glib::Value {
    #[inline]
    fn from(o: MyObject) -> Self {
        unsafe {
            let mut value = ::glib::Value::from_type_unchecked(
                <MyObject as ::glib::StaticType>::static_type(),
            );
            ::glib::gobject_ffi::g_value_take_object(
                ::glib::translate::ToGlibPtrMut::to_glib_none_mut(&mut value).0,
                ::glib::translate::IntoGlibPtr::<
                    *mut <imp::MyObject as ::glib::subclass::types::ObjectSubclass>::Instance,
                >::into_glib_ptr(o) as *mut _,
            );
            value
        }
    }
}
#[doc(hidden)]
impl ::glib::value::ToValueOptional for MyObject {
    #[inline]
    fn to_value_optional(s: Option<&Self>) -> ::glib::Value {
        let mut value = ::glib::Value::for_value_type::<Self>();
        unsafe {
            ::glib::gobject_ffi::g_value_take_object(
                ::glib::translate::ToGlibPtrMut::to_glib_none_mut(&mut value).0,
                ::glib::translate::ToGlibPtr::<
                    *mut <imp::MyObject as ::glib::subclass::types::ObjectSubclass>::Instance,
                >::to_glib_full(&s) as *mut _,
            );
        }
        value
    }
}
#[doc(hidden)]
impl ::glib::clone::Downgrade for MyObject {
    type Weak = ::glib::object::WeakRef<Self>;
    #[inline]
    fn downgrade(&self) -> Self::Weak {
        <Self as ::glib::object::ObjectExt>::downgrade(&self)
    }
}
impl ::glib::HasParamSpec for MyObject {
    type ParamSpec = ::glib::ParamSpecObject;
    type SetValue = Self;
    type BuilderFn = fn(&str) -> ::glib::ParamSpecObjectBuilder<Self>;
    fn param_spec_builder() -> Self::BuilderFn {
        |name| Self::ParamSpec::builder(name)
    }
}
unsafe impl ::glib::object::ParentClassIs for MyObject {
    type Parent = ::glib::object::Object;
}
#[doc(hidden)]
impl AsRef<::glib::object::Object> for MyObject {
    #[inline]
    fn as_ref(&self) -> &::glib::object::Object {
        ::glib::object::Cast::upcast_ref(self)
    }
}
#[doc(hidden)]
impl std::borrow::Borrow<::glib::object::Object> for MyObject {
    #[inline]
    fn borrow(&self) -> &::glib::object::Object {
        ::glib::object::Cast::upcast_ref(self)
    }
}
#[doc(hidden)]
impl From<MyObject> for ::glib::object::Object {
    #[inline]
    fn from(v: MyObject) -> Self {
        <MyObject as ::glib::Cast>::upcast(v)
    }
}
#[doc(hidden)]
unsafe impl ::glib::object::IsA<::glib::object::Object> for MyObject {}
#[doc(hidden)]
unsafe impl ::glib::object::IsClass for MyObject {}
unsafe impl ::glib::object::ObjectSubclassIs for MyObject {
    type Subclass = imp::MyObject;
}
impl MyObject {
    pub fn new(enum_value: MyEnum) -> Self {
        Object::builder().property("enum_value", enum_value).build()
    }
}
fn main() {
    {
        ::std::io::_print(format_args!("Hello, world!\n"));
    };
}
