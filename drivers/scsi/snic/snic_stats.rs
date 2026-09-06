//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/snic/snic_stats.h
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
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snic_io_stats {
    pub /: *mut *mut atomic64_t active; / Active IOs,
    pub /: *mut *mut atomic64_t max_active; / Max # active IOs,
    pub /: *mut *mut atomic64_t max_sgl; / Max # SGLs for any IO,
    pub /: *mut *mut atomic64_t max_time; / Max time to process IO,
    pub /: *mut *mut atomic64_t max_qtime; / Max time to Queue the IO,
    pub /: *mut *mut atomic64_t max_cmpl_time; / Max time to complete the IO,
    pub /: *mut *mut atomic64_t sgl_cnt[SNIC_MAX_SG_DESC_CNT]; / SGL Counters,
    pub /: *mut *mut atomic64_t max_io_sz; / Max IO Size,
    pub /: *mut *mut atomic64_t compl; / IO Completions,
    pub /: *mut *mut atomic64_t fail; / IO Failures,
    pub /: *mut *mut atomic64_t req_null; / req or req info is NULL,
    pub /: *mut *mut atomic64_t alloc_fail; / Alloc Failures,
    pub sc_null: core::sync::atomic::AtomicI64,
    pub /: *mut *mut atomic64_t io_not_found; / IO Not Found,
    pub /: *mut *mut atomic64_t num_ios; / Number of IOs,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snic_abort_stats {
    pub /: *mut *mut atomic64_t num; / Abort counter,
    pub /: *mut *mut atomic64_t fail; / Abort Failure Counter,
    pub /: *mut *mut atomic64_t drv_tmo; / Abort Driver Timeouts,
    pub /: *mut *mut atomic64_t fw_tmo; / Abort Firmware Timeouts,
    pub /: *mut *mut atomic64_t io_not_found;/ Abort IO Not Found,
    pub /: *mut *mut atomic64_t q_fail; / Abort Queuing Failed,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snic_reset_stats {
    pub /: *mut *mut atomic64_t dev_resets; / Device Reset Counter,
    pub /: *mut *mut atomic64_t dev_reset_fail; / Device Reset Failures,
    pub /: *mut *mut atomic64_t dev_reset_aborts; / Device Reset Aborts,
    pub /: *mut *mut atomic64_t dev_reset_tmo; / Device Reset Timeout,
    pub /: *mut *mut atomic64_t dev_reset_terms; / Device Reset terminate,
    pub /: *mut *mut atomic64_t hba_resets; / hba/firmware resets,
    pub /: *mut *mut atomic64_t hba_reset_cmpl; / hba/firmware reset completions,
    pub /: *mut *mut atomic64_t hba_reset_fail; / hba/firmware failures,
    pub /: *mut *mut atomic64_t snic_resets; / snic resets,
    pub /: *mut *mut atomic64_t snic_reset_compl; / snic reset completions,
    pub /: *mut *mut atomic64_t snic_reset_fail; / snic reset failures,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snic_fw_stats {
    pub /: *mut *mut atomic64_t actv_reqs; / Active Requests,
    pub /: *mut *mut atomic64_t max_actv_reqs; / Max Active Requests,
    pub /: *mut *mut atomic64_t out_of_res; / Firmware Out Of Resources,
    pub /: *mut *mut atomic64_t io_errs; / Firmware IO Firmware Errors,
    pub /: *mut *mut atomic64_t scsi_errs; / Target hits check condition,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snic_misc_stats {
    pub last_isr_time: u64,
    pub last_ack_time: u64,
    pub ack_isr_cnt: core::sync::atomic::AtomicI64,
    pub cmpl_isr_cnt: core::sync::atomic::AtomicI64,
    pub errnotify_isr_cnt: core::sync::atomic::AtomicI64,
    pub /: *mut *mut atomic64_t max_cq_ents; / Max CQ Entries,
    pub /: *mut *mut atomic64_t data_cnt_mismat; / Data Count Mismatch,
    pub io_tmo: core::sync::atomic::AtomicI64,
    pub io_aborted: core::sync::atomic::AtomicI64,
    pub /: *mut *mut atomic64_t sgl_inval; / SGL Invalid,
    pub /: *mut *mut atomic64_t abts_wq_alloc_fail; / Abort Path WQ desc alloc failure,
    pub /: *mut *mut atomic64_t devrst_wq_alloc_fail;/ Device Reset - WQ desc alloc fail,
    pub /: *mut *mut atomic64_t wq_alloc_fail; / IO WQ desc alloc failure,
    pub no_icmnd_itmf_cmpls: core::sync::atomic::AtomicI64,
    pub io_under_run: core::sync::atomic::AtomicI64,
    pub qfull: core::sync::atomic::AtomicI64,
    pub qsz_rampup: core::sync::atomic::AtomicI64,
    pub qsz_rampdown: core::sync::atomic::AtomicI64,
    pub last_qsz: core::sync::atomic::AtomicI64,
    pub tgt_not_rdy: core::sync::atomic::AtomicI64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snic_stats {
    pub io: snic_io_stats,
    pub abts: snic_abort_stats,
    pub reset: snic_reset_stats,
    pub fw: snic_fw_stats,
    pub misc: snic_misc_stats,
    pub io_cmpl_skip: core::sync::atomic::AtomicI64,
}

extern "C" {
    pub fn snic_stats_debugfs_init(: *mut snic);
}
extern "C" {
    pub fn snic_stats_debugfs_remove(: *mut snic);
}
// Auxillary function to update active IO counter
// Auxillary function to update IO completion counter
