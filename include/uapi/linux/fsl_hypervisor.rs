//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/fsl_hypervisor.h
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


// SPDX-License-Identifier: ((GPL-2.0+ WITH Linux-syscall-note) OR BSD-3-Clause)
//
// Freescale hypervisor ioctl and kernel interface
//
// Copyright (C) 2008-2011 Freescale Semiconductor, Inc.
// Author: Timur Tabi <timur@freescale.com>
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are met:
// * Redistributions of source code must retain the above copyright
// notice, this list of conditions and the following disclaimer.
// * Redistributions in binary form must reproduce the above copyright
// notice, this list of conditions and the following disclaimer in the
// documentation and/or other materials provided with the distribution.
// * Neither the name of Freescale Semiconductor nor the
// names of its contributors may be used to endorse or promote products
// derived from this software without specific prior written permission.
//
// ALTERNATIVELY, this software may be distributed under the terms of the
// GNU General Public License ("GPL") as published by the Free Software
// Foundation, either version 2 of that License or (at your option) any
// later version.
//
// This software is provided by Freescale Semiconductor "as is" and any
// express or implied warranties, including, but not limited to, the implied
// warranties of merchantability and fitness for a particular purpose are
// disclaimed. In no event shall Freescale Semiconductor be liable for any
// direct, indirect, incidental, special, exemplary, or consequential damages
// (including, but not limited to, procurement of substitute goods or services;
// loss of use, data, or profits; or business interruption) however caused and
// on any theory of liability, whether in contract, strict liability, or tort
// (including negligence or otherwise) arising in any way out of the use of this
// software, even if advised of the possibility of such damage.
//
// This file is used by the Freescale hypervisor management driver.  It can
// also be included by applications that need to communicate with the driver
// via the ioctl interface.
//

//
// struct fsl_hv_ioctl_restart - restart a partition
// @ret: return error code from the hypervisor
// @partition: the ID of the partition to restart, or -1 for the
// calling partition
//
// Used by FSL_HV_IOCTL_PARTITION_RESTART
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsl_hv_ioctl_restart {
    pub ret: __u32,
    pub partition: __u32,
}

//
// struct fsl_hv_ioctl_status - get a partition's status
// @ret: return error code from the hypervisor
// @partition: the ID of the partition to query, or -1 for the
// calling partition
// @status: The returned status of the partition
//
// Used by FSL_HV_IOCTL_PARTITION_GET_STATUS
//
// Values of 'status':
// 0 = Stopped
// 1 = Running
// 2 = Starting
// 3 = Stopping
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsl_hv_ioctl_status {
    pub ret: __u32,
    pub partition: __u32,
    pub status: __u32,
}

//
// struct fsl_hv_ioctl_start - start a partition
// @ret: return error code from the hypervisor
// @partition: the ID of the partition to control
// @entry_point: The offset within the guest IMA to start execution
// @load: If non-zero, reload the partition's images before starting
//
// Used by FSL_HV_IOCTL_PARTITION_START
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsl_hv_ioctl_start {
    pub ret: __u32,
    pub partition: __u32,
    pub entry_point: __u32,
    pub load: __u32,
}

//
// struct fsl_hv_ioctl_stop - stop a partition
// @ret: return error code from the hypervisor
// @partition: the ID of the partition to stop, or -1 for the calling
// partition
//
// Used by FSL_HV_IOCTL_PARTITION_STOP
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsl_hv_ioctl_stop {
    pub ret: __u32,
    pub partition: __u32,
}

//
// struct fsl_hv_ioctl_memcpy - copy memory between partitions
// @ret: return error code from the hypervisor
// @source: the partition ID of the source partition, or -1 for this
// partition
// @target: the partition ID of the target partition, or -1 for this
// partition
// @reserved: reserved, must be set to 0
// @local_vaddr: user-space virtual address of a buffer in the local
// partition
// @remote_paddr: guest physical address of a buffer in the
// remote partition
// @count: the number of bytes to copy.  Both the local and remote
// buffers must be at least 'count' bytes long
//
// Used by FSL_HV_IOCTL_MEMCPY
//
// The 'local' partition is the partition that calls this ioctl.  The
// 'remote' partition is a different partition.  The data is copied from
// the 'source' paritition' to the 'target' partition.
//
// The buffer in the remote partition must be guest physically
// contiguous.
//
// This ioctl does not support copying memory between two remote
// partitions or within the same partition, so either 'source' or
// 'target' (but not both) must be -1.  In other words, either
//
// source == local and target == remote
// or
// source == remote and target == local
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsl_hv_ioctl_memcpy {
    pub ret: __u32,
    pub source: __u32,
    pub target: __u32,
    pub /: *mut *mut __u32 reserved; / padding to ensure local_vaddr is aligned,
    pub local_vaddr: __u64,
    pub remote_paddr: __u64,
    pub count: __u64,
}

//
// struct fsl_hv_ioctl_doorbell - ring a doorbell
// @ret: return error code from the hypervisor
// @doorbell: the handle of the doorbell to ring doorbell
//
// Used by FSL_HV_IOCTL_DOORBELL
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsl_hv_ioctl_doorbell {
    pub ret: __u32,
    pub doorbell: __u32,
}

//
// struct fsl_hv_ioctl_prop - get/set a device tree property
// @ret: return error code from the hypervisor
// @handle: handle of partition whose tree to access
// @path: virtual address of path name of node to access
// @propname: virtual address of name of property to access
// @propval: virtual address of property data buffer
// @proplen: Size of property data buffer
// @reserved: reserved, must be set to 0
//
// Used by FSL_HV_IOCTL_DOORBELL
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsl_hv_ioctl_prop {
    pub ret: __u32,
    pub handle: __u32,
    pub path: __u64,
    pub propname: __u64,
    pub propval: __u64,
    pub proplen: __u32,
    pub /: *mut *mut __u32 reserved; / padding to ensure structure is aligned,
}

// The ioctl type, documented in ioctl-number.txt
pub const FSL_HV_IOCTL_TYPE: c_uint = 0xAF;
// Restart another partition

// Get a partition's status

// Boot another partition

// Stop this or another partition

// Copy data from one partition to another

// Ring a doorbell

// Get a property from another guest's device tree

// Set a property in another guest's device tree

