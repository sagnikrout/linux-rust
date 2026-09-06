//! Automatically rewritten from C Header to Rust Module
//! Source: tools/testing/vma/include/stubs.h
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


// SPDX-License-Identifier: GPL-2.0+

//
// Contains declarations that are STUBBED, that is that are rendered no-ops, in
// order to faciliate userland VMA testing.
//
// Forward declarations.
// Macro flag: #define 
// Macro flag: #define __randomize_layout

// Macro flag: #define ASSERT_EXCLUSIVE_WRITER(x)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vm_userfaultfd_ctx {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mempolicy {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmu_gather {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mutex {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vm_fault {
    pub 0: return,
    pub 0: return,
    pub 0: return,
    pub 0: return,
    pub false: return,
    pub vma_flags: return,
    pub 0: return,
    pub 0: return,
// Currently stubbed but we may later wish to un-stub.
    pub pages): static inline void vm_acct_memory(long,
    pub 0: return,
    pub 0: return,
    pub true: return,
    pub true: return,
    pub true: return,
    pub false: return,
    pub false: return,
    pub false: return,
    pub true: return,
    pub false: return,
    pub false: return,
    pub 0: return,
    pub true: return,
    pub 0: return,
    pub false: return,
    pub NULL: return,
    pub true: return,
    pub 0: return,
    pub 0: return,
    pub true: return,
    pub NULL: return,
    pub true: return,
    pub true: return,
    pub f: return,
    pub 0: return,
    pub false: return,
