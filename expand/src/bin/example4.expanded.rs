#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
struct I32 {
    #[doc(hidden)]
    __private_inner: [u8; I32::__SIZE],
}
const _: () = {
    type __StackloverWrappedType<__Inner__> = __Inner__;
    #[inline(always)]
    fn __stacklover_create(dep2: i32) -> i32 {
        dep2
    }
    #[allow(unused)]
    #[allow(unreachable_code)]
    fn __stacklover_inner_unreachable() -> i32 {
        let __stacklover_inner_to_struct_fn_unreachable = |inner| -> I32 {
            ::core::panicking::panic("internal error: entered unreachable code")
        };
        let _ = {
            let created_value = __stacklover_create(
                ::core::panicking::panic("internal error: entered unreachable code"),
            );
            let inner_to_struct = __stacklover_inner_to_struct_fn_unreachable;
            inner_to_struct(created_value)
        };
        fn __stacklover_fn_param_unreachable<T, R>(_: impl Fn(T) -> R) -> T {
            ::core::panicking::panic("internal error: entered unreachable code")
        }
        __stacklover_fn_param_unreachable(__stacklover_inner_to_struct_fn_unreachable)
    }
    const _: () = {
        fn _unused() {
            fn assert_traits<
                T: ::core::marker::Send + ::core::marker::Sync + ::core::marker::Unpin
                    + ::core::panic::UnwindSafe + ::core::panic::RefUnwindSafe + 'static,
            >(_: T) {}
            assert_traits(__stacklover_inner_unreachable());
        }
    };
    impl I32 {
        #[doc(hidden)]
        const __SIZE: usize = {
            #[allow(non_camel_case_types)]
            const fn size_of_return_value<dep2, __StackloverR>(
                _: &(impl ::core::ops::Fn(
                    dep2,
                ) -> __StackloverWrappedType<__StackloverR>),
            ) -> usize {
                ::core::mem::size_of::<__StackloverR>()
            }
            size_of_return_value(&__stacklover_create)
        };
        #[inline(always)]
        pub fn new(dep2: i32) -> __StackloverWrappedType<Self> {
            let __stacklover_inner_to_struct_fn = |inner| Self {
                __private_inner: unsafe {
                    ::core::mem::transmute::<_, [u8; Self::__SIZE]>(inner)
                },
            };
            {
                let created_value = __stacklover_create(dep2);
                let inner_to_struct = __stacklover_inner_to_struct_fn;
                inner_to_struct(created_value)
            }
        }
        #[inline(always)]
        pub fn as_ref(&self) -> &(i32) {
            if true {
                unsafe {
                    ::core::mem::transmute::<
                        &[u8; Self::__SIZE],
                        _,
                    >(&self.__private_inner)
                }
            } else {
                fn ref_unreachable<S, T>(_self: &S, _: T) -> &T {
                    ::core::panicking::panic("internal error: entered unreachable code")
                }
                #[allow(unreachable_code)]
                ref_unreachable(self, __stacklover_inner_unreachable())
            }
        }
        #[inline(always)]
        pub fn as_mut(&mut self) -> &mut (i32) {
            if true {
                unsafe {
                    ::core::mem::transmute::<
                        &mut [u8; Self::__SIZE],
                        _,
                    >(&mut self.__private_inner)
                }
            } else {
                fn mut_unreachable<S, T>(_self: &S, _: T) -> &mut T {
                    ::core::panicking::panic("internal error: entered unreachable code")
                }
                #[allow(unreachable_code)]
                mut_unreachable(self, __stacklover_inner_unreachable())
            }
        }
        #[inline(always)]
        pub fn into_inner(self) -> i32 {
            let inner = if true {
                unsafe {
                    ::core::mem::transmute::<[u8; Self::__SIZE], _>(self.__private_inner)
                }
            } else {
                #[allow(unreachable_code)] __stacklover_inner_unreachable()
            };
            ::core::mem::forget(self);
            inner
        }
    }
    impl ::core::ops::Drop for I32 {
        #[inline(always)]
        fn drop(&mut self) {
            let _ = if true {
                unsafe {
                    ::core::mem::transmute::<[u8; Self::__SIZE], _>(self.__private_inner)
                }
            } else {
                #[allow(unreachable_code)] __stacklover_inner_unreachable()
            };
        }
    }
    impl ::core::cmp::PartialEq for I32 {
        fn eq(&self, other: &Self) -> bool {
            ::core::cmp::PartialEq::eq(I32::as_ref(self), I32::as_ref(other))
        }
        fn ne(&self, other: &Self) -> bool {
            ::core::cmp::PartialEq::ne(I32::as_ref(self), I32::as_ref(other))
        }
    }
    impl ::core::cmp::Eq for I32 {}
    impl ::core::cmp::PartialOrd for I32 {
        fn partial_cmp(
            &self,
            other: &Self,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            ::core::cmp::PartialOrd::partial_cmp(I32::as_ref(self), I32::as_ref(other))
        }
        fn lt(&self, other: &Self) -> bool {
            ::core::cmp::PartialOrd::lt(I32::as_ref(self), I32::as_ref(other))
        }
        fn le(&self, other: &Self) -> bool {
            ::core::cmp::PartialOrd::le(I32::as_ref(self), I32::as_ref(other))
        }
        fn gt(&self, other: &Self) -> bool {
            ::core::cmp::PartialOrd::gt(I32::as_ref(self), I32::as_ref(other))
        }
        fn ge(&self, other: &Self) -> bool {
            ::core::cmp::PartialOrd::ge(I32::as_ref(self), I32::as_ref(other))
        }
    }
    impl ::core::cmp::Ord for I32 {
        fn cmp(&self, other: &Self) -> ::core::cmp::Ordering {
            ::core::cmp::Ord::cmp(I32::as_ref(self), I32::as_ref(other))
        }
    }
    impl ::core::clone::Clone for I32 {
        fn clone(&self) -> Self {
            let cloned = ::core::clone::Clone::clone(I32::as_ref(self));
            Self {
                __private_inner: unsafe {
                    ::core::mem::transmute::<_, [u8; Self::__SIZE]>(cloned)
                },
            }
        }
    }
    impl ::core::hash::Hash for I32 {
        fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
            ::core::hash::Hash::hash(I32::as_ref(self), state)
        }
    }
    impl ::core::fmt::Debug for I32 {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            ::core::fmt::Debug::fmt(I32::as_ref(self), f)
        }
    }
};
fn main() {}
