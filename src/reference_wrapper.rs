// Copyright 2022 Google LLC
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::marker::PhantomData;

pub trait CppRefMutability {}

pub struct Const;

impl CppRefMutability for Const {}

pub struct Mut;

impl CppRefMutability for Mut {}

/// A C++ non-const reference. These are different from Rust's `&mut T` in that
/// several C++ references can exist to the same underlying data ("aliasing")
/// and that's not permitted in Rust.
///
/// This type
#[repr(transparent)]
pub struct CppRef<'a, M: CppRefMutability, T>(*mut T, PhantomData<&'a T>, PhantomData<M>);

// Implement manually so that there's no need for the inner type to implement Clone
impl<'a, M: CppRefMutability, T> Clone for CppRef<'a, M, T> {
    fn clone(&self) -> Self {
        Self(self.0.clone(), PhantomData, PhantomData)
    }
}

impl<'a, M: CppRefMutability, T> Copy for CppRef<'a, M, T> {}

impl<'a, M: CppRefMutability, T> CppRef<'a, M, T> {
    #[doc(hidden)]
    pub fn new(ptr: *mut T) -> Self {
        Self(ptr, PhantomData, PhantomData)
    }

    pub unsafe fn as_ref(&self) -> &T {
        &*self.0
    }
}

impl<'a, T> CppRef<'a, Mut, T> {
    pub unsafe fn as_mut(&mut self) -> &mut T {
        &mut *self.0
    }
}
