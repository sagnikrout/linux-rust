//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/stddef.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note

// Not all C++ standards support type declarations inside an anonymous union

// Macro flag: #define __struct_group_tag(TAG)

//
// __struct_group() - Create a mirrored named and anonyomous struct
//
// @TAG: The tag name for the named sub-struct (usually empty)
// @NAME: The identifier name of the mirrored sub-struct
// @ATTRS: Any struct attributes (usually empty)
// @MEMBERS: The member declarations for the mirrored structs
//
// Used to create an anonymous union of two structs with identical layout
// and size: one anonymous and one named. The former's members can be used
// normally without sub-struct naming, and the latter can be used to
// reason about the start, end, and size of the group of struct members.
// The named struct can also be explicitly tagged for layer reuse (C only),
// as well as both having struct attributes appended.
//

// sizeof(struct{}) is 1 in C++, not 0, can't use C version of the macro.

//
// __DECLARE_FLEX_ARRAY() - Declare a flexible array usable in a union
//
// @TYPE: The type of each flexible array element
// @NAME: The name of the flexible array member
//
// In order to have a flexible array member in a union or alone in a
// struct, it needs to be wrapped in an anonymous struct with at least 1
// named member, but that member can be empty.
//

// Macro flag: #define __counted_by(m)

// Macro flag: #define __counted_by_le(m)

// Macro flag: #define __counted_by_be(m)

// Macro flag: #define __counted_by_ptr(m)

// Macro flag: #define __kernel_nonstring

