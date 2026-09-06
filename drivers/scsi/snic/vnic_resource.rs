//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/snic/vnic_resource.h
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
// Copyright 2014 Cisco Systems, Inc.  All rights reserved.
pub const VNIC_RES_MAGIC: c_uint = 0x766E6963L	/* 'vnic' */;
pub const VNIC_RES_VERSION: c_uint = 0x00000000L;
// vNIC resource types
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vnic_res_type {
    RES_TYPE_EOL,			/* End-of-list */
    RES_TYPE_WQ,			/* Work queues */
    RES_TYPE_RQ,			/* Receive queues */
    RES_TYPE_CQ,			/* Completion queues */
    RES_TYPE_RSVD1,
    RES_TYPE_NIC_CFG,		/* Enet NIC config registers */
    RES_TYPE_RSVD2,
    RES_TYPE_RSVD3,
    RES_TYPE_RSVD4,
    RES_TYPE_RSVD5,
    RES_TYPE_INTR_CTRL,		/* Interrupt ctrl table */
    RES_TYPE_INTR_TABLE,		/* MSI/MSI-X Interrupt table */
    RES_TYPE_INTR_PBA,		/* MSI/MSI-X PBA table */
    RES_TYPE_INTR_PBA_LEGACY,	/* Legacy intr status */
    RES_TYPE_RSVD6,
    RES_TYPE_RSVD7,
    RES_TYPE_DEVCMD,		/* Device command region */
    RES_TYPE_PASS_THRU_PAGE,	/* Pass-thru page */
    RES_TYPE_SUBVNIC,		/* subvnic resource type */
    RES_TYPE_MQ_WQ,			/* MQ Work queues */
    RES_TYPE_MQ_RQ,			/* MQ Receive queues */
    RES_TYPE_MQ_CQ,			/* MQ Completion queues */
    RES_TYPE_DEPRECATED1,		/* Old version of devcmd 2 */
    RES_TYPE_DEPRECATED2,		/* Old version of devcmd 2 */
    RES_TYPE_DEVCMD2,		/* Device control region */

    RES_TYPE_MAX,			/* Count of resource types */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vnic_resource_header {
    pub magic: u32,
    pub version: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vnic_resource {
    pub type: u8,
    pub bar: u8,
    pub pad: [u8; 2],
    pub bar_offset: u32,
    pub count: u32,
}
