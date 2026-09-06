//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/xe_pt_types.h
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


// SPDX-License-Identifier: MIT
//
// Copyright © 2022 Intel Corporation
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xe_cache_level {
    XE_CACHE_NONE,
    XE_CACHE_WT,
    XE_CACHE_WB,
    XE_CACHE_NONE_COMPRESSION, /*UC + COH_NONE + COMPRESSION */
    XE_CACHE_WB_COMPRESSION,
    __XE_CACHE_LEVEL_COUNT,
}

pub const XE_VM_MAX_LEVEL: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_pt {
    pub base: xe_ptw,
    pub bo: *mut xe_bo,
    pub level: c_uint,
    pub num_live: c_uint,
    pub rebind: bool,
    pub is_compact: bool,

// @addr: Virtual address start address of the PT.
    pub addr: u64,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_pt_ops {
    pub pt_level): u16 pat_index, u32,
    pub pt_level): u16 pat_index, u32,
    pub flags): u32 pt_level, bool devmem, u64,
    pub bo_offset): *mut *mut *mut u64 (pde_encode_bo)(struct xe_bo bo, u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_pt_entry {
    pub pt: *mut xe_pt,
    pub pte: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_vm_pgtable_update {
// @bo: page table bo to write to
    pub pt_bo: *mut xe_bo,
// @ofs: offset inside this PTE to begin writing to (in qwords)
    pub ofs: u32,
// @qwords: number of PTE's to write
    pub qwords: u32,
// @pt: opaque pointer useful for the caller of xe_migrate_update_pgtables
    pub pt: *mut xe_pt,
// @pt_entries: Newly added pagetable entries
    pub pt_entries: *mut xe_pt_entry,
// @flags: Target flags
    pub flags: u32,
}

// struct xe_vm_pgtable_update_op - Page table update operation
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_vm_pgtable_update_op {
// @entries: entries to update for this operation
    pub 1]: *mut *mut xe_vm_pgtable_update entries[XE_VM_MAX_LEVEL  2 +,
// @vma: VMA for operation, operation not valid if NULL
    pub vma: *mut xe_vma,
// @prl: Backing pointer to page reclaim list of pt_update_ops
    pub prl: *mut xe_page_reclaim_list,
// @num_entries: number of entries for this update operation
    pub num_entries: u32,
// @bind: is a bind
    pub bind: bool,
// @rebind: is a rebind
    pub rebind: bool,
}

// struct xe_vm_pgtable_update_ops: page table update operations
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_vm_pgtable_update_ops {
// @ops: operations
    pub ops: *mut xe_vm_pgtable_update_op,
// @deferred: deferred list to destroy PT entries
    pub deferred: llist_head,
// @q: exec queue for PT operations
    pub q: *mut xe_exec_queue,
// @prl: embedded page reclaim list
    pub prl: xe_page_reclaim_list,
// @start: start address of ops
    pub start: u64,
// @last: last address of ops
    pub last: u64,
// @num_ops: number of operations
    pub num_ops: u32,
// @current_op: current operations
    pub current_op: u32,
// @needs_svm_lock: Needs SVM lock
    pub needs_svm_lock: bool,
// @needs_invalidation: Needs invalidation
    pub needs_invalidation: bool,
//
// @wait_vm_bookkeep: PT operations need to wait until VM is idle
// (bookkeep dma-resv slots are idle) and stage all future VM activity
// behind these operations (install PT operations into VM kernel
// dma-resv slot).
//
    pub wait_vm_bookkeep: bool,
//
// @wait_vm_kernel: PT operations need to wait until VM kernel dma-resv
// slots are idle.
//
    pub wait_vm_kernel: bool,
}
