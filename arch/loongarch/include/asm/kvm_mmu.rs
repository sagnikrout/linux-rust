//! Automatically rewritten from C Header to Rust Module
//! Source: arch/loongarch/include/asm/kvm_mmu.h
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
// Copyright (C) 2020-2023 Loongson Technology Corporation Limited
//

//
// KVM_MMU_CACHE_MIN_PAGES is the number of GPA page table translation levels
// for which pages need to be cached.
//

//
// _PAGE_MODIFIED is a SW pte bit, it records page ever written on host
// kernel, on secondary MMU it records the page writeable attribute, in
// order for fast path handling.
//

pub const _KVM_FLUSH_PGTABLE: c_uint = 0x1;
pub const _KVM_HAS_PGMASK: c_uint = 0x2;

pub type kvm_pte_t = c_ulong;
pub type kvm_ptw_ctx = kvm_ptw_ctx;
extern "C" {
    pub fn int(pte: *mut *mut kvm_pte_ops)(kvm_pte_t, addr: phys_addr_t, ctx: *mut kvm_ptw_ctx) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_ptw_ctx {
    pub ops: kvm_pte_ops,
    pub flag: c_ulong,
// for kvm_arch_mmu_enable_log_dirty_pt_masked use
    pub mask: c_ulong,
    pub gfn: c_ulong,
// page walk mmu info
    pub level: c_uint,
    pub pgtable_shift: c_ulong,
    pub invalid_entry: c_ulong,
    pub invalid_ptes: *mut c_ulong,
    pub pte_shifts: *mut c_uint,
    pub opaque: *mut c_void,
// free pte table page list
    pub list: list_head,
}
