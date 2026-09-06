//! Automatically rewritten from C Header to Rust Module
//! Source: kernel/bpf/mmap_unlock_work.h
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


// SPDX-License-Identifier: GPL-2.0-only
// Copyright (c) 2021 Facebook
//

// irq_work to run mmap_read_unlock() in irq_work
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmap_unlock_irq_work {
    pub irq_work: irq_work,
    pub mm: *mut mm_struct,
    pub active: core::sync::atomic::AtomicI32,
}

//
// We cannot do mmap_read_unlock() when the irq is disabled, because of
// risk to deadlock with rq_lock. To look up vma when the irqs are
// disabled, we need to run mmap_read_unlock() in irq_work. We use a
// percpu variable to do the irq_work. The active flag reserves the slot
// before mmap_read_trylock() and until the irq_work callback consumes mm.
//
// PREEMPT_RT does not allow to trylock mmap sem in interrupt
// disabled context. Force the fallback code.
//
extern "C" {
    pub fn ERR_PTR(_arg: -EBUSY) -> return;
}
extern "C" {
    pub fn ERR_PTR(_arg: -EBUSY) -> return;
}
// The lock will be released once we're out of interrupt
// context. Tell lockdep that we've released it now so
// it doesn't complain that we forgot to release it.
//
