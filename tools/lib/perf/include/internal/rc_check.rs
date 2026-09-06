//! Automatically rewritten from C Header to Rust Module
//! Source: tools/lib/perf/include/internal/rc_check.h
#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use core::ffi::*;

// --- Linux Kernel Primitives Prelude ---
pub type uid_t = u32;
pub type gid_t = u32;
pub type uid16_t = u16;
pub type gid16_t = u16;
pub type pid_t = i32;
pub type mode_t = u32;
pub type umode_t = u16;
pub type nlink_t = u32;
pub type off_t = i64;
pub type loff_t = i64;
pub type dev_t = u32;
pub type ino_t = u64;
pub type size_t = usize;
pub type ssize_t = isize;
pub type uintptr_t = usize;
pub type intptr_t = isize;
pub type ptrdiff_t = isize;
pub type clockid_t = i32;
pub type timer_t = i32;
pub type time64_t = i64;
pub type atomic_t = core::sync::atomic::AtomicI32;
pub type atomic64_t = core::sync::atomic::AtomicI64;
// ---------------------------------------


// SPDX-License-Identifier: (LGPL-2.1 OR BSD-2-Clause)

//
// Enable reference count checking implicitly with leak checking, which is
// integrated into address sanitizer.
//

pub const REFCNT_CHECKING: c_int = 1;

pub const REFCNT_CHECKING: c_int = 1;

//
// Shared reference count checking macros.
//
// Reference count checking is an approach to sanitizing the use of reference
// counted structs. It leverages address and leak sanitizers to make sure gets
// are paired with a put. Reference count checking adds a malloc-ed layer of
// indirection on a get, and frees it on a put. A missed put will be reported as
// a memory leak. A double put will be reported as a double free. Accessing
// after a put will cause a use-after-free and/or a segfault.
//

// Replaces "struct foo" so that the pointer may be interposed.

// Declare a reference counted struct variable.

//
// Interpose the indirection. Result will hold the indirection and object is the
// reference counted struct.
//

// Strip the indirection layer.

// Frees the object and the indirection layer.

// A get operation adding the indirection layer.

// A put operation removing the indirection layer.

// Pointer equality when the indirection may or may not be there.

// Replaces "struct foo" so that the pointer may be interposed.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct struct_name {
    pub \: *mut *mut original_##struct_name orig;,
}

// Declare a reference counted struct variable.

//
// Interpose the indirection. Result will hold the indirection and object is the
// reference counted struct.
//

// Strip the indirection layer.

// Frees the object and the indirection layer.

// A get operation adding the indirection layer.

// A put operation removing the indirection layer.

// Pointer equality when the indirection may or may not be there.

