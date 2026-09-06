//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/xe_vm.h
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
// Copyright © 2021 Intel Corporation
//

//
// MAX_FAULTS_SAVED_PER_VM - Maximum number of faults each vm can store before future
// faults are discarded to prevent memory overuse
//
pub const MAX_FAULTS_SAVED_PER_VM: c_int = 50;
extern "C" {
    pub fn xe_vma_cmp_vma_cb(key: *const c_void, node: *const rb_node) -> c_int;
}
extern "C" {
    pub fn xe_vm_lock(vm: *mut xe_vm, intr: bool) -> c_int;
}
extern "C" {
    pub fn xe_vm_unlock(vm: *mut xe_vm);
}
// Only guaranteed not to change when vm->lock is held
extern "C" {
    pub fn xe_vm_is_closed(xe_vm_is_banned(vm: vm) ||) -> return;
}
extern "C" {
    pub fn xe_vma_has_default_mem_attrs(vma: *mut xe_vma) -> bool;
}
//
// xe_vm_has_scratch() - Whether the vm is configured for scratch PTEs
// @vm: The vm
//
// Return: whether the vm populates unmapped areas with scratch PTEs
//
// gpuvm_to_vm() - Return the embedding xe_vm from a struct drm_gpuvm pointer
// @gpuvm: The struct drm_gpuvm pointer
//
// Return: Pointer to the embedding struct xe_vm.
//
extern "C" {
    pub fn container_of(_arg: gpuvm, xe_vm: struct, _arg: gpuvm) -> return;
}
extern "C" {
    pub fn gpuvm_to_vm(_arg: gpuva->vm) -> return;
}
extern "C" {
    pub fn container_of(_arg: gpuva, xe_vma: struct, _arg: gpuva) -> return;
}
extern "C" {
    pub fn container_of(_arg: op, xe_vma_op: struct, _arg: base) -> return;
}
//
// DOC: Provide accessors for vma members to facilitate easy change of
// implementation.
//
extern "C" {
    pub fn xe_vma_start(xe_vma_size(vma: vma) +) -> return;
}
extern "C" {
    pub fn container_of(_arg: vma->gpuva.vm, xe_vm: struct, _arg: gpuvm) -> return;
}
extern "C" {
    pub fn xe_vma_need_vram_for_atomic(xe: *mut xe_device, vma: *mut xe_vma, is_atomic: bool) -> c_int;
}
extern "C" {
    pub fn xe_vm_alloc_madvise_vma(vm: *mut xe_vm, addr: u64, size: u64) -> c_int;
}
extern "C" {
    pub fn xe_vm_alloc_cpu_addr_mirror_vma(vm: *mut xe_vm, addr: u64, size: u64) -> c_int;
}
//
// to_userptr_vma() - Return a pointer to an embedding userptr vma
// @vma: Pointer to the embedded struct xe_vma
//
// Return: Pointer to the embedding userptr vma
//
extern "C" {
    pub fn container_of(_arg: vma, xe_userptr_vma: struct, _arg: vma) -> return;
}
extern "C" {
    pub fn xe_vm_pdp4_descriptor(vm: *mut xe_vm, tile: *mut xe_tile) -> u64;
}
extern "C" {
    pub fn xe_vm_query_vmas_attrs_ioctl(dev: *mut drm_device, data: *mut c_void, file: *mut drm_file) -> c_int;
}
extern "C" {
    pub fn xe_vm_close_and_put(vm: *mut xe_vm);
}
extern "C" {
    pub fn xe_vm_in_lr_mode(!xe_vm_in_fault_mode(vm: vm) &&) -> return;
}
extern "C" {
    pub fn xe_vm_add_compute_exec_queue(vm: *mut xe_vm, q: *mut xe_exec_queue) -> c_int;
}
extern "C" {
    pub fn xe_vm_remove_compute_exec_queue(vm: *mut xe_vm, q: *mut xe_exec_queue);
}
extern "C" {
    pub fn xe_vm_rebind(vm: *mut xe_vm, rebind_worker: bool) -> c_int;
}
extern "C" {
    pub fn xe_vm_invalidate_vma(vma: *mut xe_vma) -> c_int;
}
extern "C" {
    pub fn xe_vm_invalidate_vma_submit(vma: *mut xe_vma, batch: *mut xe_tlb_inval_batch) -> c_int;
}
extern "C" {
    pub fn xe_vm_validate_protected(vm: *mut xe_vm) -> c_int;
}
//
// xe_vm_reactivate_rebind() - Reactivate the rebind functionality on compute
// vms.
// @vm: The vm.
//
// If the rebind functionality on a compute vm was disabled due
// to nothing to execute. Reactivate it and run the rebind worker.
// This function should be called after submitting a batch to a compute vm.
//
extern "C" {
    pub fn xe_vm_lock_vma(exec: *mut drm_exec, vma: *mut xe_vma) -> c_int;
}
extern "C" {
    pub fn xe_vm_resume_rebind_worker(vm: *mut xe_vm);
}
//
// xe_vm_resv() - Return's the vm's reservation object
// @vm: The vm
//
// Return: Pointer to the vm's reservation object.
//
extern "C" {
    pub fn drm_gpuvm_resv(_arg: &vm->gpuvm) -> return;
}
extern "C" {
    pub fn xe_vm_kill(vm: *mut xe_vm, unlocked: bool);
}
extern "C" {
    pub fn xe_vm_add_exec_queue(vm: *mut xe_vm, q: *mut xe_exec_queue);
}
extern "C" {
    pub fn xe_vm_remove_exec_queue(vm: *mut xe_vm, q: *mut xe_exec_queue);
}
//
// xe_vm_assert_held(vm) - Assert that the vm's reservation object is held.
// @vm: The vm
//

extern "C" {
    pub fn xe_vm_drm_exec_lock(vm: *mut xe_vm, exec: *mut drm_exec) -> c_int;
}

extern "C" {
    pub fn xe_vm_snapshot_capture_delayed(snap: *mut xe_vm_snapshot);
}
extern "C" {
    pub fn xe_vm_snapshot_print(snap: *mut xe_vm_snapshot, p: *mut drm_printer);
}
extern "C" {
    pub fn xe_vm_snapshot_free(snap: *mut xe_vm_snapshot);
}
extern "C" {
    pub fn xe_vm_add_fault_entry_pf(vm: *mut xe_vm, pf: *mut xe_pagefault);
}
//
// xe_vm_set_validating() - Register this task as currently making bos resident
// @allow_res_evict: Allow eviction of buffer objects bound to @vm when
// validating.
// @vm: Pointer to the vm or NULL.
//
// Register this task as currently making bos resident for the vm. Intended
// to avoid eviction by the same task of shared bos bound to the vm.
// Call with the vm's resv lock held.
//
// Pairs with READ_ONCE in xe_vm_is_validating()
//
// xe_vm_clear_validating() - Unregister this task as currently making bos resident
// @vm: Pointer to the vm or NULL
// @allow_res_evict: Eviction from @vm was allowed. Must be set to the same
// value as for xe_vm_set_validation().
//
// Register this task as currently making bos resident for the vm. Intended
// to avoid eviction by the same task of shared bos bound to the vm.
// Call with the vm's resv lock held.
//
// Pairs with READ_ONCE in xe_vm_is_validating()
//
// xe_vm_is_validating() - Whether bos bound to the vm are currently being made resident
// by the current task.
// @vm: Pointer to the vm.
//
// If this function returns %true, we should be in a vm resv locked region, since
// the current process is the same task that called xe_vm_set_validating().
// The function asserts that that's indeed the case.
//
// Return: %true if the task is currently making bos resident, %false otherwise.
//
// Pairs with WRITE_ONCE in xe_vm_is_validating()
//
// xe_vm_set_validation_exec() - Accessor to set the drm_exec object
// @vm: The vm we want to register a drm_exec object with.
// @exec: The exec object we want to register.
//
// Set the drm_exec object used to lock the vm's resv.
//
// xe_vm_validation_exec() - Accessor to read the drm_exec object
// @vm: The vm we want to register a drm_exec object with.
//
// Return: The drm_exec object used to lock the vm's resv. The value
// is a valid pointer, %NULL, or one of the special values defined in
// xe_validation.h.
//
// xe_vm_has_valid_gpu_mapping() - Advisory helper to check if VMA or SVM range has
// a valid GPU mapping
// @tile: The tile which the GPU mapping belongs to
// @tile_present: Tile present mask
// @tile_invalidated: Tile invalidated mask
//
// The READ_ONCEs pair with WRITE_ONCEs in either the TLB invalidation paths
// (xe_vm.c, xe_svm.c) or the binding paths (xe_pt.c). These are not reliable
// without the notifier lock in userptr or SVM cases, and not reliable without
// the BO dma-resv lock in the BO case. As such, they should only be used in
// opportunistic cases (e.g., skipping a page fault fix or not skipping a TLB
// invalidation) where it is harmless.
//
// Return: True is there are valid GPU pages, False otherwise
//

extern "C" {
    pub fn xe_vma_mem_attr_copy(to: *mut xe_vma_mem_attr, from: *mut xe_vma_mem_attr);
}
