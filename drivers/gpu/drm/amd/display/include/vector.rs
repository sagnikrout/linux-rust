//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/include/vector.h
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


//
// Copyright 2012-15 Advanced Micro Devices, Inc.
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
// THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
// OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
// ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
// OTHER DEALINGS IN THE SOFTWARE.
//
// Authors: AMD
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vector {
    pub container: *mut u8,
    pub struct_size: u32,
    pub count: u32,
    pub capacity: u32,
    pub ctx: *mut dc_context,
}

// 'initial_value' is optional. If initial_value not supplied,
// each "structure" in the vector will contain zeros by default.
// dal_vector_insert_at
// reallocate container if necessary
// then shell items at right and insert
// return if the container modified
// do not check that index belongs to container
// since the function is private and index is going to be calculated
// either with by function or as get_count+1
// operator[]
// create a clone (copy) of a vector
// dal_vector_remove_at_index
// Shifts elements on the right from remove position to the left,
// removing an element at position by overwrite means
extern "C" {
    pub fn dal_vector_capacity(vector: *const vector) -> u32;
}
extern "C" {
    pub fn dal_vector_reserve(vector: *mut vector, capacity: u32) -> bool;
}
extern "C" {
    pub fn dal_vector_clear(vector: *mut vector);
}
//
// Macro definitions of TYPE-SAFE versions of vector set/get functions.
//

// Note: "type_t" is the ONLY token accepted by "checkpatch.pl" and by
// "checkcommit" as *return type*.
// For uniformity reasons "type_t" is used for all type-safe macro
// definitions here.

