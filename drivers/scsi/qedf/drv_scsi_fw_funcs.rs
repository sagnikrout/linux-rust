//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/qedf/drv_scsi_fw_funcs.h
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
// QLogic FCoE Offload Driver
// Copyright (c) 2016-2018 Cavium Inc.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct scsi_sgl_task_params {
    pub sgl: *mut scsi_sge,
    pub sgl_phys_addr: regpair,
    pub total_buffer_size: u32,
    pub num_sges: u16,
// true if SGL contains a small (< 4KB) SGE in middle(not 1st or last)
// -> relevant for tx only
//
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
// Enable Connection error upon DIF error (segments with DIF errors are
// dropped)
//
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
// for cdb_size > default CDB size (extended CDB > 16 bytes) ->
// pointer to the CDB buffer SGE
//
    pub extended_cdb_sge: scsi_sge,
// Physical address of sense data buffer for sense data - 256B buffer
    pub sense_data_buffer_phys_addr: regpair,
}

//
// @brief scsi_is_slow_sgl - checks for slow SGL
//
// @param num_sges - number of sges in SGL
// @param small_mid_sge - True is the SGL contains an SGE which is smaller than
// 4KB and its not the 1st or last SGE in the SGL
//
extern "C" {
    pub fn scsi_is_slow_sgl(num_sges: u16, small_mid_sge: bool) -> bool;
}
//
// @brief init_scsi_sgl_context - initializes SGL task context
//
// @param sgl_params - SGL context parameters to initialize (output parameter)
// @param data_desc - context struct containing SGEs array to set (output
// parameter)
// @param sgl_task_params - SGL parameters (input)
//
