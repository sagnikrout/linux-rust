//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/pci.h
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
// pci.h
//
// PCI defines and function prototypes
// Copyright 1994, Drew Eckhardt
// Copyright 1997--1999 Martin Mares <mj@ucw.cz>
//
// For more information, please consult the following manuals (look at
// http://www.pcisig.com/ for how to get them):
//
// PCI BIOS Specification
// PCI Local Bus Specification
// PCI to PCI Bridge Specification
// PCI System Design Guide
//

//
// The PCI interface treats multi-function devices as independent
// devices.  The slot/function address of each device is encoded
// in a single byte as follows:
//
// 7:3 = slot
// 2:0 = function
//

// Ioctls for /proc/bus/pci/X/Y nodes.

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pci_hotplug_event {
    PCI_HOTPLUG_LINK_UP,
    PCI_HOTPLUG_LINK_DOWN,
    PCI_HOTPLUG_CARD_PRESENT,
    PCI_HOTPLUG_CARD_NOT_PRESENT,
}
