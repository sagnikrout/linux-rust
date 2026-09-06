//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/fnic/vnic_scsi.h
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
// Copyright 2008 Cisco Systems, Inc.  All rights reserved.
// Copyright 2007 Nuova Systems, Inc.  All rights reserved.
//
pub const VNIC_FNIC_WQ_COPY_COUNT_MIN: c_int = 1;
pub const VNIC_FNIC_WQ_COPY_COUNT_MAX: c_int = 1;
pub const VNIC_FNIC_WQ_DESCS_MIN: c_int = 64;
pub const VNIC_FNIC_WQ_DESCS_MAX: c_int = 128;
pub const VNIC_FNIC_WQ_COPY_DESCS_MIN: c_int = 64;
pub const VNIC_FNIC_WQ_COPY_DESCS_MAX: c_int = 512;
pub const VNIC_FNIC_RQ_DESCS_MIN: c_int = 64;
pub const VNIC_FNIC_RQ_DESCS_MAX: c_int = 128;
pub const VNIC_FNIC_EDTOV_MIN: c_int = 1000;
pub const VNIC_FNIC_EDTOV_MAX: c_int = 255000;
pub const VNIC_FNIC_EDTOV_DEF: c_int = 2000;
pub const VNIC_FNIC_RATOV_MIN: c_int = 1000;
pub const VNIC_FNIC_RATOV_MAX: c_int = 255000;
pub const VNIC_FNIC_MAXDATAFIELDSIZE_MIN: c_int = 256;
pub const VNIC_FNIC_MAXDATAFIELDSIZE_MAX: c_int = 2048;
pub const VNIC_FNIC_FLOGI_RETRIES_MIN: c_int = 0;
pub const VNIC_FNIC_FLOGI_RETRIES_MAX: c_uint = 0xffffffff;
pub const VNIC_FNIC_FLOGI_RETRIES_DEF: c_uint = 0xffffffff;
pub const VNIC_FNIC_FLOGI_TIMEOUT_MIN: c_int = 1000;
pub const VNIC_FNIC_FLOGI_TIMEOUT_MAX: c_int = 255000;
pub const VNIC_FNIC_PLOGI_RETRIES_MIN: c_int = 0;
pub const VNIC_FNIC_PLOGI_RETRIES_MAX: c_int = 255;
pub const VNIC_FNIC_PLOGI_RETRIES_DEF: c_int = 8;
pub const VNIC_FNIC_PLOGI_TIMEOUT_MIN: c_int = 1000;
pub const VNIC_FNIC_PLOGI_TIMEOUT_MAX: c_int = 255000;
pub const VNIC_FNIC_IO_THROTTLE_COUNT_MIN: c_int = 1;
pub const VNIC_FNIC_IO_THROTTLE_COUNT_MAX: c_int = 2048;
pub const VNIC_FNIC_LINK_DOWN_TIMEOUT_MIN: c_int = 0;
pub const VNIC_FNIC_LINK_DOWN_TIMEOUT_MAX: c_int = 240000;
pub const VNIC_FNIC_PORT_DOWN_TIMEOUT_MIN: c_int = 0;
pub const VNIC_FNIC_PORT_DOWN_TIMEOUT_MAX: c_int = 240000;
pub const VNIC_FNIC_PORT_DOWN_IO_RETRIES_MIN: c_int = 0;
pub const VNIC_FNIC_PORT_DOWN_IO_RETRIES_MAX: c_int = 255;
pub const VNIC_FNIC_LUNS_PER_TARGET_MIN: c_int = 1;
pub const VNIC_FNIC_LUNS_PER_TARGET_MAX: c_int = 4096;
// Device-specific region: scsi configuration
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vnic_fc_config {
    pub node_wwn: u64,
    pub port_wwn: u64,
    pub flags: u32,
    pub wq_enet_desc_count: u32,
    pub wq_copy_desc_count: u32,
    pub rq_desc_count: u32,
    pub flogi_retries: u32,
    pub flogi_timeout: u32,
    pub plogi_retries: u32,
    pub plogi_timeout: u32,
    pub io_throttle_count: u32,
    pub link_down_timeout: u32,
    pub port_down_timeout: u32,
    pub port_down_io_retries: u32,
    pub luns_per_tgt: u32,
    pub maxdatafieldsize: u16,
    pub ed_tov: u16,
    pub ra_tov: u16,
    pub intr_timer: u16,
    pub intr_timer_type: u8,
    pub intr_mode: u8,
    pub lun_queue_depth: u8,
    pub io_timeout_retry: u8,
    pub wq_copy_count: u16,
}

pub const VFCF_FCP_SEQ_LVL_ERR: c_uint = 0x1	/* Enable FCP-2 Error Recovery */;
pub const VFCF_PERBI: c_uint = 0x2	/* persistent binding info available */;
pub const VFCF_FIP_CAPABLE: c_uint = 0x4	/* firmware can handle FIP */;
pub const VFCF_FC_INITIATOR: c_uint = 0x20    /* FC Initiator Mode */;
pub const VFCF_FC_TARGET: c_uint = 0x40    /* FC Target Mode */;
pub const VFCF_FC_NVME_INITIATOR: c_uint = 0x80    /* FC-NVMe Initiator Mode */;
pub const VFCF_FC_NVME_TARGET: c_uint = 0x100   /* FC-NVMe Target Mode */;
