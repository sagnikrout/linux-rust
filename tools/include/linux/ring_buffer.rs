//! Automatically rewritten from C Header to Rust Module
//! Source: tools/include/linux/ring_buffer.h
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
// Contract with kernel for walking the perf ring buffer from
// user space requires the following barrier pairing (quote
// from kernel/events/ring_buffer.c):
//
// Since the mmap() consumer (userspace) can run on a
// different CPU:
//
// kernel                             user
//
// if (LOAD ->data_tail) {            LOAD ->data_head
// (A)             smp_rmb()       (C)
// STORE $data                     LOAD $data
// smp_wmb()       (B)             smp_mb()        (D)
// STORE ->data_head               STORE ->data_tail
// }
//
// Where A pairs with D, and B pairs with C.
//
// In our case A is a control dependency that separates the
// load of the ->data_tail and the stores of $data. In case
// ->data_tail indicates there is no room in the buffer to
// store $data we do not.
//
// D needs to be a full barrier since it separates the data
// READ from the tail WRITE.
//
// For B a WMB is sufficient since it separates two WRITEs,
// and for C an RMB is sufficient since it separates two READs.
//
// Note, instead of B, C, D we could also use smp_store_release()
// in B and D as well as smp_load_acquire() in C.
//
// However, this optimization does not make sense for all kernel
// supported architectures since for a fair number it would
// resolve into READ_ONCE() + smp_mb() pair for smp_load_acquire(),
// and smp_mb() + WRITE_ONCE() pair for smp_store_release().
//
// Thus for those smp_wmb() in B and smp_rmb() in C would still
// be less expensive. For the case of D this has either the same
// cost or is less expensive, for example, due to TSO x86 can
// avoid the CPU barrier entirely.
//
// Architectures where smp_load_acquire() does not fallback to
// READ_ONCE() + smp_mb() pair.
//

extern "C" {
    pub fn smp_load_acquire(_arg: &base->data_head) -> return;
}

