//! Automatically rewritten from C Header to Rust Module
//! Source: arch/um/include/shared/init.h
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


// SPDX-License-Identifier: GPL-2.0
// These macros are used to mark some functions or
// initialized data (doesn't apply to uninitialized data)
// as `initialization' functions. The kernel can take this
// as hint that the function is used only during the initialization
// phase and free up used memory resources after
//
// Usage:
// For functions:
//
// You should add __init immediately before the function name, like:
//
// static void __init initme(int x, int y)
// {
// extern int z; z = x * y;
// }
//
// If the function has a prototype somewhere, you can also add
// __init between closing brace of the prototype and semicolon:
//
// extern int initialize_foobar_device(int, int, int) __init;
//
// For initialized data:
// You should insert __initdata between the variable name and equal
// sign followed by value, e.g.:
//
// static int init_variable __initdata = 0;
// static const char linux_logo[] __initconst = { 0x32, 0x36, ... };
//
// Don't forget to initialize data not at file scope, i.e. within a function,
// as gcc otherwise puts the data into the bss section and not into the init
// section.
//
// Also note, that this data cannot be "const".
//
extern "C" {
    pub fn int(_arg: *mut initcall_t)(void) -> typedef;
}
extern "C" {
    pub fn void(_arg: *mut exitcall_t)(void) -> typedef;
}

// These are for everybody (although not all archs will actually

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uml_param {
    pub str: *const c_char,
    pub ): *mut *mut *mut int (setup_func)(char , int,
}

//
// Mark functions and data as being only used at initialization
// or exit time.
//

// Userspace initcalls shouldn't depend on anything in the kernel, so we'll
// make them run first.
//

