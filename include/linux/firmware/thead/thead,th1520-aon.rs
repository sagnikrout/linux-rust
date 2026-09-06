//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/firmware/thead/thead,th1520-aon.h
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


// SPDX-License-Identifier: GPL-2.0
//
// Copyright (C) 2021 Alibaba Group Holding Limited.
//

pub const TH1520_AON_RPC_VERSION: c_int = 2;
pub const TH1520_AON_RPC_MSG_NUM: c_int = 7;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum th1520_aon_rpc_svc {
    TH1520_AON_RPC_SVC_UNKNOWN = 0,
    TH1520_AON_RPC_SVC_PM = 1,
    TH1520_AON_RPC_SVC_MISC = 2,
    TH1520_AON_RPC_SVC_AVFS = 3,
    TH1520_AON_RPC_SVC_SYS = 4,
    TH1520_AON_RPC_SVC_WDG = 5,
    TH1520_AON_RPC_SVC_LPM = 6,
    TH1520_AON_RPC_SVC_MAX = 0x3F,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum th1520_aon_misc_func {
    TH1520_AON_MISC_FUNC_UNKNOWN = 0,
    TH1520_AON_MISC_FUNC_SET_CONTROL = 1,
    TH1520_AON_MISC_FUNC_GET_CONTROL = 2,
    TH1520_AON_MISC_FUNC_REGDUMP_CFG = 3,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum th1520_aon_wdg_func {
    TH1520_AON_WDG_FUNC_UNKNOWN = 0,
    TH1520_AON_WDG_FUNC_START = 1,
    TH1520_AON_WDG_FUNC_STOP = 2,
    TH1520_AON_WDG_FUNC_PING = 3,
    TH1520_AON_WDG_FUNC_TIMEOUTSET = 4,
    TH1520_AON_WDG_FUNC_RESTART = 5,
    TH1520_AON_WDG_FUNC_GET_STATE = 6,
    TH1520_AON_WDG_FUNC_POWER_OFF = 7,
    TH1520_AON_WDG_FUNC_AON_WDT_ON = 8,
    TH1520_AON_WDG_FUNC_AON_WDT_OFF = 9,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum th1520_aon_sys_func {
    TH1520_AON_SYS_FUNC_UNKNOWN = 0,
    TH1520_AON_SYS_FUNC_AON_RESERVE_MEM = 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum th1520_aon_lpm_func {
    TH1520_AON_LPM_FUNC_UNKNOWN = 0,
    TH1520_AON_LPM_FUNC_REQUIRE_STR = 1,
    TH1520_AON_LPM_FUNC_RESUME_STR = 2,
    TH1520_AON_LPM_FUNC_REQUIRE_STD = 3,
    TH1520_AON_LPM_FUNC_CPUHP = 4,
    TH1520_AON_LPM_FUNC_REGDUMP_CFG = 5,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum th1520_aon_pm_func {
    TH1520_AON_PM_FUNC_UNKNOWN = 0,
    TH1520_AON_PM_FUNC_SET_RESOURCE_REGULATOR = 1,
    TH1520_AON_PM_FUNC_GET_RESOURCE_REGULATOR = 2,
    TH1520_AON_PM_FUNC_SET_RESOURCE_POWER_MODE = 3,
    TH1520_AON_PM_FUNC_PWR_SET = 4,
    TH1520_AON_PM_FUNC_PWR_GET = 5,
    TH1520_AON_PM_FUNC_CHECK_FAULT = 6,
    TH1520_AON_PM_FUNC_GET_TEMPERATURE = 7,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct th1520_aon_rpc_msg_hdr {
    pub /: *mut *mut u8 ver; / version of msg hdr,
    pub /: *mut *mut u8 size; / msg size ,uinit in bytes,the size includes rpc msg header self,
    pub /: *mut *mut u8 svc; / rpc main service id,
    pub /: *mut *mut u8 func; / rpc sub func id of specific service, sent by caller,
    pub __aligned(1): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct th1520_aon_rpc_ack_common {
    pub hdr: th1520_aon_rpc_msg_hdr,
    pub err_code: u8,
    pub __aligned(1): } __packed,
pub const RPC_SVC_MSG_TYPE_DATA: c_int = 0;
pub const RPC_SVC_MSG_TYPE_ACK: c_int = 1;
pub const RPC_SVC_MSG_NEED_ACK: c_int = 0;
pub const RPC_SVC_MSG_NO_NEED_ACK: c_int = 1;

//
// Defines for SC PM Power Mode
//

//
// Defines for AON power islands
//
pub const TH1520_AON_AUDIO_PD: c_int = 0;
pub const TH1520_AON_VDEC_PD: c_int = 1;
pub const TH1520_AON_NPU_PD: c_int = 2;
pub const TH1520_AON_VENC_PD: c_int = 3;
pub const TH1520_AON_GPU_PD: c_int = 4;
pub const TH1520_AON_DSP0_PD: c_int = 5;
pub const TH1520_AON_DSP1_PD: c_int = 6;
    pub dev): *mut *mut th1520_aon_chan th1520_aon_init(device,
    pub aon_chan): *mut void th1520_aon_deinit(struct th1520_aon_chan,
    pub msg): *mut *mut int th1520_aon_call_rpc(struct th1520_aon_chan aon_chan, void,
    pub power_on): bool,
