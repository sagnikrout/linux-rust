//! Automatically rewritten from C Header to Rust Module
//! Source: lib/test_hmm_uapi.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note
//
// This is a module to test the HMM (Heterogeneous Memory Management) API
// of the kernel. It allows a userspace program to expose its entire address
// space through the HMM test module device file.
//

//
// Structure to pass to the HMM test driver to mimic a device accessing
// system memory and ZONE_DEVICE private memory through device page tables.
//
// @addr: (in) user address the device will read/write
// @ptr: (in) user address where device data is copied to/from
// @npages: (in) number of pages to read/write
// @cpages: (out) number of pages copied
// @faults: (out) number of device page faults seen
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hmm_dmirror_cmd {
    pub addr: __u64,
    pub ptr: __u64,
    pub npages: __u64,
    pub cpages: __u64,
    pub faults: __u64,
}

// Expose the address space of the calling process through hmm device file

//
// Values returned in hmm_dmirror_cmd.ptr for HMM_DMIRROR_SNAPSHOT.
// HMM_DMIRROR_PROT_ERROR: no valid mirror PTE for this page
// HMM_DMIRROR_PROT_NONE: unpopulated PTE or PTE with no access
// HMM_DMIRROR_PROT_READ: read-only PTE
// HMM_DMIRROR_PROT_WRITE: read/write PTE
// HMM_DMIRROR_PROT_PMD: PMD sized page is fully mapped by same permissions
// HMM_DMIRROR_PROT_PUD: PUD sized page is fully mapped by same permissions
// HMM_DMIRROR_PROT_ZERO: special read-only zero page
// HMM_DMIRROR_PROT_DEV_PRIVATE_LOCAL: Migrated device private page on the
// device the ioctl() is made
// HMM_DMIRROR_PROT_DEV_PRIVATE_REMOTE: Migrated device private page on some
// other device
// HMM_DMIRROR_PROT_DEV_COHERENT: Migrate device coherent page on the device
// the ioctl() is made
//
// 0 is reserved to catch uninitialized type fields
