//! Automatically rewritten from C Header to Rust Module
//! Source: include/asm-generic/rwonce.h
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
// Prevent the compiler from merging or refetching reads or writes. The
// compiler is also forbidden from reordering successive instances of
// READ_ONCE and WRITE_ONCE, but only when the compiler is aware of some
// particular ordering. One way to make the compiler aware of ordering is to
// put the two invocations of READ_ONCE or WRITE_ONCE in different C
// statements.
//
// These two macros will also work on aggregate data types like structs or
// unions.
//
// Their two major use cases are: (1) Mediating communication between
// process-level code and irq/NMI handlers, all running on the same CPU,
// and (2) Ensuring that the compiler does not fold, spindle, or otherwise
// mutilate accesses that either do not require ordering or that interact
// with an explicit memory barrier or atomic instruction that provides the
// required ordering.
//

//
// Yes, this permits 64-bit accesses on 32-bit architectures. These will
// actually be atomic in some cases (namely Armv7 + LPAE), but for others we
// rely on the access being split into 2x32-bit accesses for a 32-bit quantity
// (e.g. a virtual address) and a strong prevailing wind.
//

//
// Use __READ_ONCE() instead of READ_ONCE() if you do not require any
// atomicity. Note that this may result in tears!
//

// (volatile typeof(x) *)&(x) = (val);				\

extern "C" {
    pub fn __READ_ONCE()addr: *mut *mut (unsigned long) -> return;
}
//
// Use READ_ONCE_NOCHECK() instead of READ_ONCE() if you need to load a
// word from memory atomically but without telling KASAN/KCSAN. This is
// usually used by unwinding code when walking the stack of a running process.
//

// open-coded instrument_read(addr, 1)
//
// This load can race with concurrent stores to out-of-bounds memory,
// but READ_ONCE() can't be used because it requires higher alignment
// than plain loads in arm64 builds with LTO.
//

