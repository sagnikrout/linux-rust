//! Automatically rewritten from C Header to Rust Module
//! Source: arch/s390/include/asm/kmsan.h
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
// Different lowcores accessed via S390_lowcore are described
// by the same struct page. Resolve the prefix manually in
// order to get a distinct struct page.
//
extern "C" {
    pub fn kmsan_get_metadata(_arg: addr, _arg: is_origin) -> return;
}
//
// pfn_valid() relies on RCU, and may call into the scheduler on exiting
// the critical section. However, this would result in recursion with
// KMSAN. Therefore, disable preemption here, and re-enable preemption
// below while suppressing reschedules to avoid recursion.
//
// Note, this sacrifices occasionally breaking scheduling guarantees.
// Although, a kernel compiled with KMSAN has already given up on any
// performance guarantees due to being heavily instrumented.
//

