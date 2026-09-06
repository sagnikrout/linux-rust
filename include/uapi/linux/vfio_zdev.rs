//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/vfio_zdev.h
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
// VFIO Region definitions for ZPCI devices
//
// Copyright IBM Corp. 2020
//
// Author(s): Pierre Morel <pmorel@linux.ibm.com>
// Matthew Rosato <mjrosato@linux.ibm.com>
//

//
// VFIO_DEVICE_INFO_CAP_ZPCI_BASE - Base PCI Function information
//
// This capability provides a set of descriptive information about the
// associated PCI function.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vfio_device_info_cap_zpci_base {
    pub header: vfio_info_cap_header,
    pub /: *mut *mut __u64 start_dma; / Start of available DMA addresses,
    pub /: *mut *mut __u64 end_dma; / End of available DMA addresses,
    pub /: *mut *mut __u16 pchid; / Physical Channel ID,
    pub /: *mut *mut __u16 vfn; / Virtual function number,
    pub /: *mut *mut __u16 fmb_length; / Measurement Block Length (in bytes),
    pub /: *mut *mut __u8 pft; / PCI Function Type,
    pub /: *mut *mut __u8 gid; / PCI function group ID,
// End of version 1
    pub /: *mut *mut __u32 fh; / PCI function handle,
// End of version 2
    pub /: *mut *mut __u32 ccdf_err_length; / PCI CCDF length,
// End of version 3
}

//
// VFIO_DEVICE_INFO_CAP_ZPCI_GROUP - Base PCI Function Group information
//
// This capability provides a set of descriptive information about the group of
// PCI functions that the associated device belongs to.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vfio_device_info_cap_zpci_group {
    pub header: vfio_info_cap_header,
    pub /: *mut *mut __u64 dasm; / DMA Address space mask,
    pub /: *mut *mut __u64 msi_addr; / MSI address,
    pub flags: __u64,

    pub /: *mut *mut __u16 mui; / Measurement Block Update Interval,
    pub /: *mut *mut __u16 noi; / Maximum number of MSIs,
    pub /: *mut *mut __u16 maxstbl; / Maximum Store Block Length,
    pub /: *mut *mut __u8 version; / Supported PCI Version,
// End of version 1
    pub reserved: __u8,
    pub /: *mut *mut __u16 imaxstbl; / Maximum Interpreted Store Block Length,
// End of version 2
}

//
// VFIO_DEVICE_INFO_CAP_ZPCI_UTIL - Utility String
//
// This capability provides the utility string for the associated device, which
// is a device identifier string made up of EBCDID characters.  'size' specifies
// the length of 'util_str'.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vfio_device_info_cap_zpci_util {
    pub header: vfio_info_cap_header,
    pub size: __u32,
    pub util_str: [__u8; ],
}

//
// VFIO_DEVICE_INFO_CAP_ZPCI_PFIP - PCI Function Path
//
// This capability provides the PCI function path string, which is an identifier
// that describes the internal hardware path of the device. 'size' specifies
// the length of 'pfip'.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vfio_device_info_cap_zpci_pfip {
    pub header: vfio_info_cap_header,
    pub size: __u32,
    pub pfip: [__u8; ],
}
