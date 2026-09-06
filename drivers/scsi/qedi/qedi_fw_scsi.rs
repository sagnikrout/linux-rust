//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/qedi/qedi_fw_scsi.h
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
// QLogic iSCSI Offload Driver
// Copyright (c) 2016 Cavium Inc.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct scsi_sgl_task_params {
    pub sgl: *mut scsi_sge,
    pub sgl_phys_addr: regpair,
    pub total_buffer_size: u32,
    pub num_sges: u16,
    pub small_mid_sge: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct scsi_dif_task_params {
    pub initial_ref_tag: u32,
    pub initial_ref_tag_is_valid: bool,
    pub application_tag: u16,
    pub application_tag_mask: u16,
    pub dif_block_size_log: u16,
    pub dif_on_network: bool,
    pub dif_on_host: bool,
    pub host_guard_type: u8,
    pub protection_type: u8,
    pub ref_tag_mask: u8,
    pub crc_seed: bool,
    pub tx_dif_conn_err_en: bool,
    pub ignore_app_tag: bool,
    pub keep_ref_tag_const: bool,
    pub validate_guard: bool,
    pub validate_app_tag: bool,
    pub validate_ref_tag: bool,
    pub forward_guard: bool,
    pub forward_app_tag: bool,
    pub forward_ref_tag: bool,
    pub forward_app_tag_with_mask: bool,
    pub forward_ref_tag_with_mask: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct scsi_initiator_cmd_params {
    pub extended_cdb_sge: scsi_sge,
    pub sense_data_buffer_phys_addr: regpair,
}
