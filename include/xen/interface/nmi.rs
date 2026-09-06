//! Automatically rewritten from C Header to Rust Module
//! Source: include/xen/interface/nmi.h
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
// nmi.h
//
// NMI callback registration and reason codes.
//
// Copyright (c) 2005, Keir Fraser <keir@xensource.com>
//

//
// NMI reason codes:
// Currently these are x86-specific, stored in arch_shared_info.nmi_reason.
//
// I/O-check error reported via ISA port 0x61, bit 6.
pub const _XEN_NMIREASON_io_error: c_int = 0;

// PCI SERR reported via ISA port 0x61, bit 7.
pub const _XEN_NMIREASON_pci_serr: c_int = 1;

// Unknown hardware-generated NMI.
pub const _XEN_NMIREASON_unknown: c_int = 2;

//
// long nmi_op(unsigned int cmd, void *arg)
// NB. All ops return zero on success, else a negative error code.
//
// Register NMI callback for this (calling) VCPU. Currently this only makes
// sense for domain 0, vcpu 0. All other callers will be returned EINVAL.
// arg == pointer to xennmi_callback structure.
//
pub const XENNMI_register_callback: c_int = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xennmi_callback {
    pub handler_address: c_ulong,
    pub pad: c_ulong,
}

//
// Deregister NMI callback for this (calling) VCPU.
// arg == NULL.
//
pub const XENNMI_unregister_callback: c_int = 1;
