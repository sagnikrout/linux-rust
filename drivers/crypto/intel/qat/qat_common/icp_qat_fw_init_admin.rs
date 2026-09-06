//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/intel/qat/qat_common/icp_qat_fw_init_admin.h
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


// SPDX-License-Identifier: (BSD-3-Clause OR GPL-2.0-only)
// Copyright(c) 2014 - 2020 Intel Corporation

pub const RL_MAX_RP_IDS: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum icp_qat_fw_init_admin_cmd_id {
    ICP_QAT_FW_INIT_AE = 0,
    ICP_QAT_FW_TRNG_ENABLE = 1,
    ICP_QAT_FW_TRNG_DISABLE = 2,
    ICP_QAT_FW_CONSTANTS_CFG = 3,
    ICP_QAT_FW_STATUS_GET = 4,
    ICP_QAT_FW_COUNTERS_GET = 5,
    ICP_QAT_FW_LOOPBACK = 6,
    ICP_QAT_FW_HEARTBEAT_SYNC = 7,
    ICP_QAT_FW_HEARTBEAT_GET = 8,
    ICP_QAT_FW_COMP_CAPABILITY_GET = 9,
    ICP_QAT_FW_CRYPTO_CAPABILITY_GET = 10,
    ICP_QAT_FW_DC_CHAIN_INIT = 11,
    ICP_QAT_FW_HEARTBEAT_TIMER_SET = 13,
    ICP_QAT_FW_RL_INIT = 15,
    ICP_QAT_FW_TIMER_GET = 19,
    ICP_QAT_FW_CNV_STATS_GET = 20,
    ICP_QAT_FW_PM_STATE_CONFIG = 128,
    ICP_QAT_FW_PM_INFO = 129,
    ICP_QAT_FW_RL_ADD = 134,
    ICP_QAT_FW_RL_UPDATE = 135,
    ICP_QAT_FW_RL_REMOVE = 136,
    ICP_QAT_FW_TL_START = 137,
    ICP_QAT_FW_TL_STOP = 138,
    ICP_QAT_FW_KPT_ENABLE = 144,
    ICP_QAT_FW_SVN_READ = 146,
    ICP_QAT_FW_SVN_COMMIT = 147,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum icp_qat_fw_init_admin_resp_status {
    ICP_QAT_FW_INIT_RESP_STATUS_SUCCESS = 0,
    ICP_QAT_FW_INIT_RESP_STATUS_FAIL = 1,
    ICP_QAT_FW_INIT_RESP_STATUS_RETRY = 2,
    ICP_QAT_FW_INIT_RESP_STATUS_UNSUPPORTED = 4,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icp_qat_fw_init_admin_tl_rp_indexes {
    pub rp_num_index_0: __u8,
    pub rp_num_index_1: __u8,
    pub rp_num_index_2: __u8,
    pub rp_num_index_3: __u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icp_qat_fw_init_admin_slice_cnt {
    pub cpr_cnt: __u8,
    pub xlt_cnt: __u8,
    pub dcpr_cnt: __u8,
    pub pke_cnt: __u8,
    pub wat_cnt: __u8,
    pub wcp_cnt: __u8,
    pub ucs_cnt: __u8,
    pub cph_cnt: __u8,
    pub ath_cnt: __u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icp_qat_fw_init_admin_sla_config_params {
    pub pcie_in_cir: __u32,
    pub pcie_in_pir: __u32,
    pub pcie_out_cir: __u32,
    pub pcie_out_pir: __u32,
    pub slice_util_cir: __u32,
    pub slice_util_pir: __u32,
    pub ae_util_cir: __u32,
    pub ae_util_pir: __u32,
    pub rp_ids: [__u16; RL_MAX_RP_IDS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icp_qat_fw_init_admin_req {
    pub init_cfg_sz: __u16,
    pub resrvd1: __u8,
    pub cmd_id: __u8,
    pub resrvd2: __u32,
    pub opaque_data: __u64,
    pub init_cfg_ptr: __u64,
    pub ibuf_size_in_kb: __u16,
    pub resrvd3: __u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icp_qat_fw_init_admin_resp {
    pub flags: __u8,
    pub resrvd1: __u8,
    pub status: __u8,
    pub cmd_id: __u8,
    pub resrvd2: __u32,
    pub version_minor_num: __u16,
    pub version_major_num: __u16,
}

pub const ICP_QAT_NUMBER_OF_PM_EVENTS: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct icp_qat_fw_init_admin_pm_info {
    pub max_pwrreq: __u16,
    pub min_pwrreq: __u16,
    pub resvrd1: __u16,
    pub pwr_state: __u8,
    pub resvrd2: __u8,
    pub fusectl0: __u32,
    pub sys_pm: __u32,
    pub host_msg: __u32,
    pub unknown: __u32,
    pub local_ssm: __u32,
    pub timer: __u32,
    pub event_log: [__u32; ICP_QAT_NUMBER_OF_PM_EVENTS],
    pub fw_init: __u32,
    pub pwrreq: __u32,
    pub status: __u32,
    pub main: __u32,
    pub thread: __u32,
    pub pm_enable: __u32,
    pub pm_active_status: __u32,
    pub pm_managed_status: __u32,
    pub pm_domain_status: __u32,
    pub active_constraint: __u32,
    pub resvrd3: [__u32; 6],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icp_qat_fw_init_admin_kpt_cfg {
    pub swk_cnt_per_fn: __u32,
    pub swk_cnt_per_pasid: __u32,
    pub swk_ttl_in_secs: __u32,
    pub swk_shared_disable: __u32,
}
