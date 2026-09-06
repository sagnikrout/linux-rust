//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/ath10k/hif.h
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


// SPDX-License-Identifier: ISC
//
// Copyright (c) 2005-2011 Atheros Communications Inc.
// Copyright (c) 2011-2015,2017 Qualcomm Atheros, Inc.
//

// Types of fw logging mode
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath_dbg_mode {
    ATH10K_ENABLE_FW_LOG_DIAG,
    ATH10K_ENABLE_FW_LOG_CE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath10k_hif_sg_item {
    pub transfer_id: u16,
    pub /: *mut *mut *mut void transfer_context; / NULL = tx completion callback not called,
    pub /: *mut *mut *mut void vaddr; / for debugging mostly,
    pub paddr: dma_addr_t,
    pub len: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath10k_hif_ops {
// send a scatter-gather list to the target
    pub n_items): *mut *mut ath10k_hif_sg_item items, int,
// read firmware memory through the diagnose interface
    pub buf_len): usize,
    pub nbytes): c_int,
//
// API to handle HIF-specific BMI message exchanges, this API is
// synchronous and only allowed to be called from a context that
// can block (sleep)
//
    pub response_len): *mut *mut void response, u32,
// Post BMI phase, after FW is loaded. Starts regular operation
    pub ar): *mut *mut int (start)(struct ath10k,
// Clean up what start() did. This does not revert to BMI phase. If
// desired so, call power_down() and power_up()
//
    pub ar): *mut *mut void (stop)(struct ath10k,
    pub ar): *mut *mut int (start_post)(struct ath10k,
    pub ar): *mut *mut int (get_htt_tx_complete)(struct ath10k,
    pub dl_pipe): *mut *mut u8 ul_pipe, u8,
    pub dl_pipe): *mut *mut *mut *mut void (get_default_pipe)(struct ath10k ar, u8 ul_pipe, u8,
//
// Check if prior sends have completed.
//
// Check whether the pipe in question has any completed
// sends that have not yet been processed.
// This function is only relevant for HIF pipes that are configured
// to be polled rather than interrupt-driven.
//
    pub force): *mut *mut *mut void (send_complete_check)(struct ath10k ar, u8 pipe_id, int,
    pub pipe_id): *mut *mut *mut u16 (get_free_queue_number)(struct ath10k ar, u8,
    pub address): *mut *mut *mut u32 (read32)(struct ath10k ar, u32,
    pub value): *mut *mut *mut void (write32)(struct ath10k ar, u32 address, u32,
// Power up the device and enter BMI transfer mode for FW download
    pub fw_mode): *mut *mut *mut int (power_up)(struct ath10k ar, enum ath10k_firmware_mode,
// Power down the device and free up resources. stop() must be called
// before this if start() was called earlier
//
    pub ar): *mut *mut void (power_down)(struct ath10k,
    pub ar): *mut *mut int (suspend)(struct ath10k,
    pub ar): *mut *mut int (resume)(struct ath10k,
// fetch calibration data from target eeprom
    pub data_len): *mut usize,
    pub target_info): *mut bmi_target_info,
    pub fw_log_mode): *mut *mut *mut int (set_target_log_mode)(struct ath10k ar, u8,
}
