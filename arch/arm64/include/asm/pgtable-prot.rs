//! Automatically rewritten from C Header to Rust Module
//! Source: arch/arm64/include/asm/pgtable-prot.h
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
//
// Copyright (C) 2016 ARM Ltd.
//

//
// Software defined PTE bits definition.
//

//
// PTE_PRESENT_INVALID=1 & PTE_VALID=0 indicates that the pte's fields should be
// interpreted according to the HW layout by SW but any attempted HW access to
// the address will result in a fault. pte_present() returns true.
//

//
// Highest possible physical address supported.
//

//
// If we have userspace only BTI we don't want to mark kernel pages
// guarded even if the system does support BTI.
//

// shared+writable pages are clean by default, hence PTE_RDONLY|PTE_WRITE

//
// Page types used via Permission Indirection Extension (PIE). PIE uses
// the USER, DBM, PXN and UXN bits to to generate an index which is used
// to look up the actual permission in PIR_ELx and PIRE0_EL1. We define
// combinations we use on non-PIE systems with the same encoding, for
// convenience these are listed here as comments as are the unallocated
// encodings.
//
// 0: PAGE_DEFAULT
// 1:                                                      PTE_USER
// 2:                                          PTE_WRITE
// 3:                                          PTE_WRITE | PTE_USER
// 4: PAGE_EXECONLY                  PTE_PXN
// 5: PAGE_READONLY_EXEC             PTE_PXN |             PTE_USER
// 6:                                PTE_PXN | PTE_WRITE
// 7: PAGE_SHARED_EXEC               PTE_PXN | PTE_WRITE | PTE_USER
// 8: PAGE_KERNEL_ROX      PTE_UXN
// 9: PAGE_GCS_RO          PTE_UXN |                       PTE_USER
// a: PAGE_KERNEL_EXEC     PTE_UXN |           PTE_WRITE
// b: PAGE_GCS             PTE_UXN |           PTE_WRITE | PTE_USER
// c: PAGE_KERNEL_RO       PTE_UXN | PTE_PXN
// d: PAGE_READONLY        PTE_UXN | PTE_PXN |             PTE_USER
// e: PAGE_KERNEL          PTE_UXN | PTE_PXN | PTE_WRITE
// f: PAGE_SHARED          PTE_UXN | PTE_PXN | PTE_WRITE | PTE_USER

