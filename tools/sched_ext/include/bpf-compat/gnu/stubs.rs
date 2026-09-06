//! Automatically rewritten from C Header to Rust Module
//! Source: tools/sched_ext/include/bpf-compat/gnu/stubs.h
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
// Dummy gnu/stubs.h. clang can end up including /usr/include/gnu/stubs.h when
// compiling BPF files although its content doesn't play any role. The file in
// turn includes stubs-64.h or stubs-32.h depending on whether __x86_64__ is
// defined. When compiling a BPF source, __x86_64__ isn't set and thus
// stubs-32.h is selected. However, the file is not there if the system doesn't
// have 32bit glibc devel package installed leading to a build failure.
//
// The problem is worked around by making this file available in the include
// search paths before the system one when building BPF.
//
