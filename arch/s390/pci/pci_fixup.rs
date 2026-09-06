//! Automatically rewritten from C to Rust
//! Source: arch/s390/pci/pci_fixup.c
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
// Exceptions for specific devices,
//
// Copyright IBM Corp. 2025
//
// Author(s):
// Niklas Schnelle <schnelle@linux.ibm.com>
//

#[no_mangle]
unsafe extern "C" fn zpci_ism_bar_no_mmap(pdev: *mut pci_dev) {
    static void zpci_ism_bar_no_mmap(struct pci_dev *pdev)
    {
//
// ISM's BAR is special. Drivers written for ISM know
// how to handle this but others need to be aware of their
// special nature e.g. to prevent attempts to mmap() it.
//
    pdev.non_mappable_bars = 1;
    }
    DECLARE_PCI_FIXUP_EARLY(PCI_VENDOR_ID_IBM,
    PCI_DEVICE_ID_IBM_ISM,
    zpci_ism_bar_no_mmap);
