//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/xe_userptr.h
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
// Copyright © 2025 Intel Corporation
//

// struct xe_userptr_vm - User pointer VM level state
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_userptr_vm {
//
// @userptr.repin_list: list of VMAs which are user pointers,
// and needs repinning. Protected by @lock.
//
    pub repin_list: list_head,
//
// @userptr.invalidated_lock: Protects the
// @userptr.invalidated list.
//
    pub invalidated_lock: spinlock_t,
//
// @userptr.invalidated: List of invalidated userptrs, not yet
// picked
// up for revalidation. Protected from access with the
// @invalidated_lock. Removing items from the list
// additionally requires @lock in write mode, and adding
// items to the list requires either the @svm.gpusvm.notifier_lock in
// write mode, OR @lock in write mode.
//
    pub invalidated: list_head,
}

// struct xe_userptr - User pointer
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_userptr {
// @invalidate_link: Link for the vm::userptr.invalidated list
    pub invalidate_link: list_head,
// @userptr: link into VM repin list if userptr.
    pub repin_link: list_head,
//
// @pages: gpusvm pages for this user pointer.
//
    pub pages: drm_gpusvm_pages,
//
// @notifier: MMU notifier for user pointer (invalidation call back)
//
    pub notifier: mmu_interval_notifier,
//
// @finish: MMU notifier finish structure for two-pass invalidation.
// Embedded here to avoid allocation in the notifier callback.
// Protected by struct xe_vm::svm.gpusvm.notifier_lock in write mode
// alternatively by the same lock in read mode *and* the vm resv held.
//
    pub finish: mmu_interval_notifier_finish,
//
// @inval_batch: TLB invalidation batch for deferred completion.
// Stores an in-flight TLB invalidation submitted during a two-pass
// notifier so the wait can be deferred to a subsequent pass, allowing
// multiple GPUs to be signalled before any of them are waited on.
// Protected using the same locking as @finish.
//
    pub inval_batch: xe_tlb_inval_batch,
//
// @finish_inuse: Whether @finish is currently in use by an in-progress
// two-pass invalidation.
// Protected using the same locking as @finish.
//
    pub finish_inuse: bool,
//
// @tlb_inval_submitted: Whether a TLB invalidation has been submitted
// via @inval_batch and is pending completion.  When set, the next pass
// must call xe_tlb_inval_batch_wait() before reusing @inval_batch.
// Protected using the same locking as @finish.
//
    pub tlb_inval_submitted: bool,
//
// @initial_bind: user pointer has been bound at least once.
// write: vm->svm.gpusvm.notifier_lock in read mode and vm->resv held.
// read: vm->svm.gpusvm.notifier_lock in write mode or vm->resv held.
//
    pub initial_bind: bool,

    pub divisor: u32,

}

extern "C" {
    pub fn xe_userptr_remove(uvma: *mut xe_userptr_vma);
}
extern "C" {
    pub fn xe_userptr_destroy(uvma: *mut xe_userptr_vma);
}
extern "C" {
    pub fn xe_vm_userptr_pin(vm: *mut xe_vm) -> c_int;
}
extern "C" {
    pub fn __xe_vm_userptr_needs_repin(vm: *mut xe_vm) -> c_int;
}
extern "C" {
    pub fn xe_vm_userptr_check_repin(vm: *mut xe_vm) -> c_int;
}
extern "C" {
    pub fn xe_vma_userptr_pin_pages(uvma: *mut xe_userptr_vma) -> c_int;
}
extern "C" {
    pub fn xe_vma_userptr_check_repin(uvma: *mut xe_userptr_vma) -> c_int;
}

extern "C" {
    pub fn xe_vma_userptr_force_invalidate(uvma: *mut xe_userptr_vma);
}

