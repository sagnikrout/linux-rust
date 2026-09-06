//! Automatically rewritten from C Header to Rust Module
//! Source: scripts/gendwarfksyms/examples/kabi.h
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
// Copyright (C) 2024 Google LLC
//
// Example macros for maintaining kABI stability.
//
// This file is based on android_kabi.h, which has the following notice:
//
// Heavily influenced by rh_kabi.h which came from the RHEL/CENTOS kernel
// and was:
// Copyright (c) 2014 Don Zickus
// Copyright (c) 2015-2018 Jiri Benc
// Copyright (c) 2015 Sabrina Dubroca, Hannes Frederic Sowa
// Copyright (c) 2016-2018 Prarit Bhargava
// Copyright (c) 2017 Paolo Abeni, Larry Woodman
//
// Kernel macros for userspace testing.

//
// KABI_DECLONLY(fqn)
// Treat the struct/union/enum fqn as a declaration, i.e. even if
// a definition is available, don't expand the contents.
//

//
// KABI_ENUMERATOR_IGNORE(fqn, field)
// When expanding enum fqn, skip the provided field. This makes it
// possible to hide added enum fields from versioning.
//

//
// KABI_ENUMERATOR_VALUE(fqn, field, value)
// When expanding enum fqn, use the provided value for the
// specified field. This makes it possible to override enumerator
// values when calculating versions.
//

//
// KABI_BYTE_SIZE(fqn, value)
// Set the byte_size attribute for the struct/union/enum fqn to
// value bytes.
//

//
// KABI_TYPE_STRING(type, str)
// For the given type, override the type string used in symtypes
// output and version calculation with str.
//

//
// KABI_RESERVE
// Reserve some "padding" in a structure for use by LTS backports.
// This is normally placed at the end of a structure.
// number: the "number" of the padding variable in the structure.  Start with
// 1 and go up.
//

//
// KABI_RESERVE_ARRAY
// Same as _BACKPORT_RESERVE but allocates an array with the specified
// size in bytes.
//

//
// KABI_IGNORE
// Add a new field that's ignored in versioning.
//

//
// KABI_REPLACE
// Replace a field with a compatible new field.
//

//
// KABI_USE(number, _new)
// Use a previous padding entry that was defined with KABI_RESERVE
// number: the previous "number" of the padding variable
// _new: the variable to use now instead of the padding variable
//

//
// KABI_USE2(number, _new1, _new2)
// Use a previous padding entry that was defined with KABI_RESERVE for
// two new variables that fit into 64 bits.  This is good for when you do not
// want to "burn" a 64bit padding variable for a smaller variable size if not
// needed.
//

//
// KABI_USE_ARRAY(number, bytes, _new)
// Use a previous padding entry that was defined with KABI_RESERVE_ARRAY
// number: the previous "number" of the padding variable
// bytes: the size in bytes reserved for the array
// _new: the variable to use now instead of the padding variable
//

