use std::any::TypeId;

use crate::{caster, CastFrom, Caster, CASTER_MAP};

/// A trait that is blanket-implemented for traits extending `CastFrom` to allow for casting
/// of a trait object for it behind an immutable reference to a trait object for another trait
/// implemented by the underlying value.
///
/// # Examples
/// ## Casting an immutable reference
/// ```
/// # use portable_intertrait::*;
/// use portable_intertrait::cast::*;
///
/// # #[cast_to(Greet)]
/// # struct Data;
/// # trait Source: CastFrom {}
/// # trait Greet {
/// #     fn greet(&self);
/// # }
/// # impl Greet for Data {
/// #    fn greet(&self) {
/// #        println!("Hello");
/// #    }
/// # }
/// impl Source for Data {}
/// let data = Data;
/// let source: &dyn Source = &data;
/// let greet = source.cast::<dyn Greet>();
/// greet.unwrap().greet();
/// ```
///
/// ## Testing if a cast is possible
/// ```
/// # use portable_intertrait::*;
/// use portable_intertrait::cast::*;
///
/// # #[cast_to(Greet)]
/// # struct Data;
/// # trait Source: CastFrom {}
/// # trait Greet {
/// #     fn greet(&self);
/// # }
/// # impl Greet for Data {
/// #    fn greet(&self) {
/// #        println!("Hello");
/// #    }
/// # }
/// impl Source for Data {}
/// let data = Data;
/// let source: &dyn Source = &data;
/// assert!(source.impls::<dyn Greet>());
/// assert!(!source.impls::<dyn std::fmt::Debug>());
/// ```
pub trait CastRef {
    /// Casts a reference to this trait into that of type `T`.
    fn cast<T: ?Sized + 'static>(&self) -> Option<&T>;

    /// Tests if this trait object can be cast into `T`.
    fn impls<T: ?Sized + 'static>(&self) -> bool;
}

/// A blanket implementation of `CastRef` for traits extending `CastFrom`.
impl<S: ?Sized + CastFrom> CastRef for S {
    fn cast<T: ?Sized + 'static>(&self) -> Option<&T> {
        let any = self.ref_any();
        let caster = caster::<T>(any.type_id())?;
        (caster.cast_ref)(any).into()
    }

    fn impls<T: ?Sized + 'static>(&self) -> bool {
        CASTER_MAP.contains_key(&(self.type_id(), TypeId::of::<Caster<T>>()))
    }
}
