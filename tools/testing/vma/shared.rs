//! Automatically rewritten from C Header to Rust Module
//! Source: tools/testing/vma/shared.h
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


// SPDX-License-Identifier: GPL-2.0-or-later

// Simple test runner. Assumes local num_[fail, tests] counters.

// Override vma_iter_prealloc() so we can choose to fail it.

pub const CONFIG_DEFAULT_MMAP_MIN_ADDR: c_int = 65536;
//
// Helper function which provides a wrapper around a merge existing VMA
// operation.
//
// Declared in main.c as uses static VMA function.
//
// Helper function to allocate a VMA and link it to the tree.
//
// Declared in main.c as uses static VMA function.
//
extern "C" {
    pub fn attach_vma(mm: *mut mm_struct, vma: *mut vm_area_struct) -> c_int;
}
// Helper function providing a dummy vm_ops->close() method.
// Helper function to simply allocate a VMA.
// Helper function to detach and free a VMA.
extern "C" {
    pub fn detach_free_vma(vma: *mut vm_area_struct);
}
// Helper function to allocate a VMA and link it to the tree.
//
// Helper function to reset the dummy anon_vma to indicate it has not been
// duplicated.
//
extern "C" {
    pub fn reset_dummy_anon_vma();
}
//
// Helper function to remove all VMAs and destroy the maple tree associated with
// a virtual address space. Returns a count of VMAs in the tree.
//
extern "C" {
    pub fn cleanup_mm(mm: *mut mm_struct, vmi: *mut vma_iterator) -> c_int;
}
// Helper function to determine if VMA has had vma_start_write() performed.
extern "C" {
    pub fn vma_write_started(vma: *mut vm_area_struct) -> bool;
}
// Provide a simple dummy VMA/anon_vma dummy setup for testing.
