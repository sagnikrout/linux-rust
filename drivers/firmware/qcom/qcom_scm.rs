//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/firmware/qcom/qcom_scm.h
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
// Copyright (c) 2010-2015,2019 The Linux Foundation. All rights reserved.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qcom_scm_convention {
    SMC_CONVENTION_UNKNOWN,
    SMC_CONVENTION_LEGACY,
    SMC_CONVENTION_ARM_32,
    SMC_CONVENTION_ARM_64,
}

pub const MAX_QCOM_SCM_ARGS: c_int = 10;
pub const MAX_QCOM_SCM_RETS: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qcom_scm_arg_types {
    QCOM_SCM_VAL,
    QCOM_SCM_RO,
    QCOM_SCM_RW,
    QCOM_SCM_BUFVAL,
}

//
// struct qcom_scm_desc
// @svc: Service identifier
// @cmd: Command identifier
// @arginfo:	Metadata describing the arguments in args[]
// @args:	The array of arguments for the secure syscall
// @owner: Owner identifier
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qcom_scm_desc {
    pub svc: u32,
    pub cmd: u32,
    pub arginfo: u32,
    pub args: [u64; MAX_QCOM_SCM_ARGS],
    pub owner: u32,
}

//
// struct qcom_scm_res
// @result:	The values returned by the secure syscall
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qcom_scm_res {
    pub result: [u64; MAX_QCOM_SCM_RETS],
}

extern "C" {
    pub fn qcom_scm_wait_for_wq_completion(dev: *mut device, wq_ctx: u32) -> c_int;
}
extern "C" {
    pub fn scm_get_wq_ctx(wq_ctx: *mut u32, flags: *mut u32, more_pending: *mut u32) -> c_int;
}

extern "C" {
    pub fn qcom_scm_shm_bridge_enable(scm_dev: *mut device) -> c_int;
}
pub const QCOM_SCM_SVC_BOOT: c_uint = 0x01;
pub const QCOM_SCM_BOOT_SET_ADDR: c_uint = 0x01;
pub const QCOM_SCM_BOOT_TERMINATE_PC: c_uint = 0x02;
pub const QCOM_SCM_BOOT_SDI_CONFIG: c_uint = 0x09;
pub const QCOM_SCM_BOOT_SET_DLOAD_MODE: c_uint = 0x10;
pub const QCOM_SCM_BOOT_SET_ADDR_MC: c_uint = 0x11;
pub const QCOM_SCM_BOOT_SET_REMOTE_STATE: c_uint = 0x0a;
pub const QCOM_SCM_FLUSH_FLAG_MASK: c_uint = 0x3;
pub const QCOM_SCM_BOOT_MAX_CPUS: c_int = 4;

pub const QCOM_SCM_SVC_PIL: c_uint = 0x02;
pub const QCOM_SCM_PIL_PAS_INIT_IMAGE: c_uint = 0x01;
pub const QCOM_SCM_PIL_PAS_MEM_SETUP: c_uint = 0x02;
pub const QCOM_SCM_PIL_PAS_AUTH_AND_RESET: c_uint = 0x05;
pub const QCOM_SCM_PIL_PAS_SHUTDOWN: c_uint = 0x06;
pub const QCOM_SCM_PIL_PAS_IS_SUPPORTED: c_uint = 0x07;
pub const QCOM_SCM_PIL_PAS_MSS_RESET: c_uint = 0x0a;
pub const QCOM_SCM_PIL_PAS_GET_RSCTABLE: c_uint = 0x21;
pub const QCOM_SCM_SVC_IO: c_uint = 0x05;
pub const QCOM_SCM_IO_READ: c_uint = 0x01;
pub const QCOM_SCM_IO_WRITE: c_uint = 0x02;
pub const QCOM_SCM_SVC_INFO: c_uint = 0x06;
pub const QCOM_SCM_INFO_IS_CALL_AVAIL: c_uint = 0x01;
pub const QCOM_SCM_SVC_MP: c_uint = 0x0c;
pub const QCOM_SCM_MP_RESTORE_SEC_CFG: c_uint = 0x02;
pub const QCOM_SCM_MP_IOMMU_SECURE_PTBL_SIZE: c_uint = 0x03;
pub const QCOM_SCM_MP_IOMMU_SECURE_PTBL_INIT: c_uint = 0x04;
pub const QCOM_SCM_MP_IOMMU_SET_CP_POOL_SIZE: c_uint = 0x05;
pub const QCOM_SCM_MP_VIDEO_VAR: c_uint = 0x08;
pub const QCOM_SCM_MP_ASSIGN: c_uint = 0x16;
pub const QCOM_SCM_MP_CP_SMMU_APERTURE_ID: c_uint = 0x1b;
pub const QCOM_SCM_MP_SHM_BRIDGE_ENABLE: c_uint = 0x1c;
pub const QCOM_SCM_MP_SHM_BRIDGE_DELETE: c_uint = 0x1d;
pub const QCOM_SCM_MP_SHM_BRIDGE_CREATE: c_uint = 0x1e;
pub const QCOM_SCM_SVC_OCMEM: c_uint = 0x0f;
pub const QCOM_SCM_OCMEM_LOCK_CMD: c_uint = 0x01;
pub const QCOM_SCM_OCMEM_UNLOCK_CMD: c_uint = 0x02;
pub const QCOM_SCM_SVC_ES: c_uint = 0x10	/* Enterprise Security */;
pub const QCOM_SCM_ES_INVALIDATE_ICE_KEY: c_uint = 0x03;
pub const QCOM_SCM_ES_CONFIG_SET_ICE_KEY: c_uint = 0x04;
pub const QCOM_SCM_ES_DERIVE_SW_SECRET: c_uint = 0x07;
pub const QCOM_SCM_ES_GENERATE_ICE_KEY: c_uint = 0x08;
pub const QCOM_SCM_ES_PREPARE_ICE_KEY: c_uint = 0x09;
pub const QCOM_SCM_ES_IMPORT_ICE_KEY: c_uint = 0x0a;
pub const QCOM_SCM_SVC_HDCP: c_uint = 0x11;
pub const QCOM_SCM_HDCP_INVOKE: c_uint = 0x01;
pub const QCOM_SCM_SVC_LMH: c_uint = 0x13;
pub const QCOM_SCM_LMH_LIMIT_PROFILE_CHANGE: c_uint = 0x01;
pub const QCOM_SCM_LMH_LIMIT_DCVSH: c_uint = 0x10;
pub const QCOM_SCM_SVC_SMMU_PROGRAM: c_uint = 0x15;
pub const QCOM_SCM_SMMU_PT_FORMAT: c_uint = 0x01;
pub const QCOM_SCM_SMMU_CONFIG_ERRATA1: c_uint = 0x03;
pub const QCOM_SCM_SMMU_CONFIG_ERRATA1_CLIENT_ALL: c_uint = 0x02;
pub const QCOM_SCM_SVC_WAITQ: c_uint = 0x24;
pub const QCOM_SCM_WAITQ_RESUME: c_uint = 0x02;
pub const QCOM_SCM_WAITQ_GET_WQ_CTX: c_uint = 0x03;
pub const QCOM_SCM_WAITQ_GET_INFO: c_uint = 0x04;
pub const QCOM_SCM_SVC_GPU: c_uint = 0x28;
pub const QCOM_SCM_SVC_GPU_INIT_REGS: c_uint = 0x01;
// ARM_SMCCC_OWNER_TRUSTED_OS calls
pub const QCOM_SCM_SVC_SMCINVOKE: c_uint = 0x06;
pub const QCOM_SCM_SMCINVOKE_INVOKE_LEGACY: c_uint = 0x00;
pub const QCOM_SCM_SMCINVOKE_CB_RSP: c_uint = 0x01;
pub const QCOM_SCM_SMCINVOKE_INVOKE: c_uint = 0x02;
// common error codes

pub const QCOM_SCM_INTERRUPTED: c_int = 1;
pub const QCOM_SCM_WAITQ_SLEEP: c_int = 2;
