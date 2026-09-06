//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/fnic/fnic_stats.h
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
// Copyright 2013 Cisco Systems, Inc.  All rights reserved.
pub const FNIC_MQ_MAX_QUEUES: c_int = 64;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stats_timestamps {
    pub last_reset_time: timespec64,
    pub last_read_time: timespec64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct io_path_stats {
    pub active_ios: core::sync::atomic::AtomicI64,
    pub max_active_ios: core::sync::atomic::AtomicI64,
    pub io_completions: core::sync::atomic::AtomicI64,
    pub io_failures: core::sync::atomic::AtomicI64,
    pub ioreq_null: core::sync::atomic::AtomicI64,
    pub alloc_failures: core::sync::atomic::AtomicI64,
    pub sc_null: core::sync::atomic::AtomicI64,
    pub io_not_found: core::sync::atomic::AtomicI64,
    pub num_ios: core::sync::atomic::AtomicI64,
    pub io_btw_0_to_10_msec: core::sync::atomic::AtomicI64,
    pub io_btw_10_to_100_msec: core::sync::atomic::AtomicI64,
    pub io_btw_100_to_500_msec: core::sync::atomic::AtomicI64,
    pub io_btw_500_to_5000_msec: core::sync::atomic::AtomicI64,
    pub io_btw_5000_to_10000_msec: core::sync::atomic::AtomicI64,
    pub io_btw_10000_to_30000_msec: core::sync::atomic::AtomicI64,
    pub io_greater_than_30000_msec: core::sync::atomic::AtomicI64,
    pub current_max_io_time: core::sync::atomic::AtomicI64,
    pub ios: [core::sync::atomic::AtomicI64; FNIC_MQ_MAX_QUEUES],
    pub nvme_io_reqs_rcvd: core::sync::atomic::AtomicI64,
    pub nvme_ios_queued_for_rsp: core::sync::atomic::AtomicI64,
    pub nvme_io_rsps_unqueued: core::sync::atomic::AtomicI64,
    pub nvme_io_rsps_sending: core::sync::atomic::AtomicI64,
    pub nvme_io_rsps_sent: core::sync::atomic::AtomicI64,
    pub nvme_num_ios_in_waitq: core::sync::atomic::AtomicI64,
    pub nvme_ios_in_waitq_3000_msec: core::sync::atomic::AtomicI64,
    pub nvme_ios_in_waitq_max_time: core::sync::atomic::AtomicI64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct abort_stats {
    pub aborts: core::sync::atomic::AtomicI64,
    pub abort_failures: core::sync::atomic::AtomicI64,
    pub abort_drv_timeouts: core::sync::atomic::AtomicI64,
    pub abort_fw_timeouts: core::sync::atomic::AtomicI64,
    pub abort_io_not_found: core::sync::atomic::AtomicI64,
    pub abort_issued_btw_0_to_6_sec: core::sync::atomic::AtomicI64,
    pub abort_issued_btw_6_to_20_sec: core::sync::atomic::AtomicI64,
    pub abort_issued_btw_20_to_30_sec: core::sync::atomic::AtomicI64,
    pub abort_issued_btw_30_to_40_sec: core::sync::atomic::AtomicI64,
    pub abort_issued_btw_40_to_50_sec: core::sync::atomic::AtomicI64,
    pub abort_issued_btw_50_to_60_sec: core::sync::atomic::AtomicI64,
    pub abort_issued_greater_than_60_sec: core::sync::atomic::AtomicI64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct terminate_stats {
    pub terminates: core::sync::atomic::AtomicI64,
    pub max_terminates: core::sync::atomic::AtomicI64,
    pub terminate_drv_timeouts: core::sync::atomic::AtomicI64,
    pub terminate_fw_timeouts: core::sync::atomic::AtomicI64,
    pub terminate_io_not_found: core::sync::atomic::AtomicI64,
    pub terminate_failures: core::sync::atomic::AtomicI64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct reset_stats {
    pub device_resets: core::sync::atomic::AtomicI64,
    pub device_reset_failures: core::sync::atomic::AtomicI64,
    pub device_reset_aborts: core::sync::atomic::AtomicI64,
    pub device_reset_timeouts: core::sync::atomic::AtomicI64,
    pub device_reset_terminates: core::sync::atomic::AtomicI64,
    pub fw_resets: core::sync::atomic::AtomicI64,
    pub fw_reset_completions: core::sync::atomic::AtomicI64,
    pub fw_reset_failures: core::sync::atomic::AtomicI64,
    pub fw_reset_timeouts: core::sync::atomic::AtomicI64,
    pub fnic_resets: core::sync::atomic::AtomicI64,
    pub fnic_reset_completions: core::sync::atomic::AtomicI64,
    pub fnic_reset_failures: core::sync::atomic::AtomicI64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_stats {
    pub active_fw_reqs: core::sync::atomic::AtomicI64,
    pub max_fw_reqs: core::sync::atomic::AtomicI64,
    pub fw_out_of_resources: core::sync::atomic::AtomicI64,
    pub io_fw_errs: core::sync::atomic::AtomicI64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vlan_stats {
    pub vlan_disc_reqs: core::sync::atomic::AtomicI64,
    pub resp_withno_vlanID: core::sync::atomic::AtomicI64,
    pub sol_expiry_count: core::sync::atomic::AtomicI64,
    pub flogi_rejects: core::sync::atomic::AtomicI64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct misc_stats {
    pub last_isr_time: u64,
    pub last_ack_time: u64,
    pub max_isr_jiffies: core::sync::atomic::AtomicI64,
    pub max_isr_time_ms: core::sync::atomic::AtomicI64,
    pub corr_work_done: core::sync::atomic::AtomicI64,
    pub isr_count: core::sync::atomic::AtomicI64,
    pub max_cq_entries: core::sync::atomic::AtomicI64,
    pub ack_index_out_of_range: core::sync::atomic::AtomicI64,
    pub data_count_mismatch: core::sync::atomic::AtomicI64,
    pub fcpio_timeout: core::sync::atomic::AtomicI64,
    pub fcpio_aborted: core::sync::atomic::AtomicI64,
    pub sgl_invalid: core::sync::atomic::AtomicI64,
    pub mss_invalid: core::sync::atomic::AtomicI64,
    pub abts_cpwq_alloc_failures: core::sync::atomic::AtomicI64,
    pub devrst_cpwq_alloc_failures: core::sync::atomic::AtomicI64,
    pub io_cpwq_alloc_failures: core::sync::atomic::AtomicI64,
    pub no_icmnd_itmf_cmpls: core::sync::atomic::AtomicI64,
    pub check_condition: core::sync::atomic::AtomicI64,
    pub queue_fulls: core::sync::atomic::AtomicI64,
    pub tport_not_ready: core::sync::atomic::AtomicI64,
    pub iport_not_ready: core::sync::atomic::AtomicI64,
    pub frame_errors: core::sync::atomic::AtomicI64,
    pub current_port_speed: core::sync::atomic::AtomicI64,
    pub intx_dummy: core::sync::atomic::AtomicI64,
    pub port_speed_in_mbps: core::sync::atomic::AtomicI64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fnic_iport_stats {
    pub num_linkdn: core::sync::atomic::AtomicI64,
    pub num_linkup: core::sync::atomic::AtomicI64,
    pub link_failure_count: core::sync::atomic::AtomicI64,
    pub num_rscns: core::sync::atomic::AtomicI64,
    pub rscn_redisc: core::sync::atomic::AtomicI64,
    pub rscn_not_redisc: core::sync::atomic::AtomicI64,
    pub frame_err: core::sync::atomic::AtomicI64,
    pub num_rnid: core::sync::atomic::AtomicI64,
    pub fabric_flogi_sent: core::sync::atomic::AtomicI64,
    pub fabric_flogi_ls_accepts: core::sync::atomic::AtomicI64,
    pub fabric_flogi_ls_rejects: core::sync::atomic::AtomicI64,
    pub fabric_flogi_misc_rejects: core::sync::atomic::AtomicI64,
    pub fabric_plogi_sent: core::sync::atomic::AtomicI64,
    pub fabric_plogi_ls_accepts: core::sync::atomic::AtomicI64,
    pub fabric_plogi_ls_rejects: core::sync::atomic::AtomicI64,
    pub fabric_plogi_misc_rejects: core::sync::atomic::AtomicI64,
    pub fabric_scr_sent: core::sync::atomic::AtomicI64,
    pub fabric_scr_ls_accepts: core::sync::atomic::AtomicI64,
    pub fabric_scr_ls_rejects: core::sync::atomic::AtomicI64,
    pub fabric_scr_misc_rejects: core::sync::atomic::AtomicI64,
    pub fabric_logo_sent: core::sync::atomic::AtomicI64,
    pub tport_alive: core::sync::atomic::AtomicI64,
    pub tport_plogi_sent: core::sync::atomic::AtomicI64,
    pub tport_plogi_ls_accepts: core::sync::atomic::AtomicI64,
    pub tport_plogi_ls_rejects: core::sync::atomic::AtomicI64,
    pub tport_plogi_misc_rejects: core::sync::atomic::AtomicI64,
    pub tport_prli_sent: core::sync::atomic::AtomicI64,
    pub tport_prli_ls_accepts: core::sync::atomic::AtomicI64,
    pub tport_prli_ls_rejects: core::sync::atomic::AtomicI64,
    pub tport_prli_misc_rejects: core::sync::atomic::AtomicI64,
    pub tport_adisc_sent: core::sync::atomic::AtomicI64,
    pub tport_adisc_ls_accepts: core::sync::atomic::AtomicI64,
    pub tport_adisc_ls_rejects: core::sync::atomic::AtomicI64,
    pub tport_logo_sent: core::sync::atomic::AtomicI64,
    pub unsupported_frames_ls_rejects: core::sync::atomic::AtomicI64,
    pub unsupported_frames_dropped: core::sync::atomic::AtomicI64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvme_host_statistics {
    pub nvme_input_requests: core::sync::atomic::AtomicI64,
    pub nvme_output_requests: core::sync::atomic::AtomicI64,
    pub nvme_control_requests: core::sync::atomic::AtomicI64,
    pub nvme_ersps: core::sync::atomic::AtomicI64,
    pub nvme_ls_requests: core::sync::atomic::AtomicI64,
    pub nvme_ls_responses: core::sync::atomic::AtomicI64,
    pub nvme_ls_aborts: core::sync::atomic::AtomicI64,
    pub nvme_ls_abort_responses: core::sync::atomic::AtomicI64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fnic_stats {
    pub stats_timestamps: stats_timestamps,
    pub io_stats: io_path_stats,
    pub abts_stats: abort_stats,
    pub term_stats: terminate_stats,
    pub reset_stats: reset_stats,
    pub fw_stats: fw_stats,
    pub vlan_stats: vlan_stats,
    pub host_stats: fc_host_statistics,
    pub misc_stats: misc_stats,
    pub nvme_stats: nvme_host_statistics,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stats_debug_info {
    pub debug_buffer: *mut c_char,
    pub i_private: *mut c_void,
    pub buf_size: c_int,
    pub buffer_len: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fnic_nvmef_info {
    pub info_buffer: *mut c_char,
    pub i_private: *mut c_void,
    pub buf_size: c_int,
    pub buffer_len: c_int,
}

extern "C" {
    pub fn fnic_get_stats_data(: *mut stats_debug_info, : *mut fnic_stats) -> c_int;
}
