//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/err.h
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

//
// Kernel pointers have redundant information, so we can use a
// scheme where we can return either an error code or a normal
// pointer with the same return value.
//
// This should be a per-architecture thing, to allow different
// error and pointer decisions.
//
pub const MAX_ERRNO: c_int = 4095;
//
// IS_ERR_VALUE - Detect an error pointer.
// @x: The pointer to check.
//
// Like IS_ERR(), but does not generate a compiler warning if result is unused.
//

//
// ERR_PTR - Create an error pointer.
// @error: A negative error code.
//
// Encodes @error into a pointer value. Users should consider the result
// opaque and not assume anything about how the error is encoded.
//
// Return: A pointer with @error encoded within its value.
//
// INIT_ERR_PTR - Init a const error pointer.
// @error: A negative error code.
//
// Like ERR_PTR(), but usable to initialize static variables.
//

// Return the pointer in the percpu address space.

// Cast an error pointer to __iomem.

//
// PTR_ERR - Extract the error code from an error pointer.
// @ptr: An error pointer.
// Return: The error code within @ptr.
//
// Read an error pointer from the percpu address space.

//
// IS_ERR - Detect an error pointer.
// @ptr: The pointer to check.
// Return: true if @ptr is an error pointer, false otherwise.
//
extern "C" {
    pub fn IS_ERR_VALUE(long)ptr: (unsigned) -> return;
}
// Read an error pointer from the percpu address space.

//
// IS_ERR_OR_NULL - Detect an error pointer or a null pointer.
// @ptr: The pointer to check.
//
// Like IS_ERR(), but also returns true for a null pointer.
//
extern "C" {
    pub fn unlikely(long)ptr: !ptr) || IS_ERR_VALUE((unsigned) -> return;
}
//
// ERR_CAST - Explicitly cast an error-valued pointer to another pointer type
// @ptr: The pointer to cast.
//
// Explicitly cast an error-valued pointer to another pointer type in such a
// way as to make it clear that's what's going on.
//
// cast away the const
//
// PTR_ERR_OR_ZERO - Extract the error code from a pointer if it has one.
// @ptr: A potential error pointer.
//
// Convenience function that can be used inside a function that returns
// an error code to propagate errors received as error pointers.
// For example, ``return PTR_ERR_OR_ZERO(ptr);`` replaces:
//
// .. code-block:: c
//
// if (IS_ERR(ptr))
// return PTR_ERR(ptr);
// else
// return 0;
//
// Return: The error code within @ptr if it is an error pointer; 0 otherwise.
//
extern "C" {
    pub fn PTR_ERR(_arg: ptr) -> return;
}

