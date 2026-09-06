//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/gt/uc/abi/guc_errors_abi.h
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


// SPDX-License-Identifier: MIT
//
// Copyright © 2014-2021 Intel Corporation
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum intel_guc_response_status {
    INTEL_GUC_RESPONSE_STATUS_SUCCESS = 0x0,
    INTEL_GUC_RESPONSE_NOT_SUPPORTED = 0x20,
    INTEL_GUC_RESPONSE_NO_ATTRIBUTE_TABLE = 0x201,
    INTEL_GUC_RESPONSE_NO_DECRYPTION_KEY = 0x202,
    INTEL_GUC_RESPONSE_DECRYPTION_FAILED = 0x204,
    INTEL_GUC_RESPONSE_STATUS_GENERIC_FAIL = 0xF000,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum intel_guc_load_status {
    INTEL_GUC_LOAD_STATUS_DEFAULT                          = 0x00,
    INTEL_GUC_LOAD_STATUS_START                            = 0x01,
    INTEL_GUC_LOAD_STATUS_ERROR_DEVID_BUILD_MISMATCH       = 0x02,
    INTEL_GUC_LOAD_STATUS_GUC_PREPROD_BUILD_MISMATCH       = 0x03,
    INTEL_GUC_LOAD_STATUS_ERROR_DEVID_INVALID_GUCTYPE      = 0x04,
    INTEL_GUC_LOAD_STATUS_HWCONFIG_START                   = 0x05,
    INTEL_GUC_LOAD_STATUS_HWCONFIG_DONE                    = 0x06,
    INTEL_GUC_LOAD_STATUS_HWCONFIG_ERROR                   = 0x07,
    INTEL_GUC_LOAD_STATUS_GDT_DONE                         = 0x10,
    INTEL_GUC_LOAD_STATUS_IDT_DONE                         = 0x20,
    INTEL_GUC_LOAD_STATUS_LAPIC_DONE                       = 0x30,
    INTEL_GUC_LOAD_STATUS_GUCINT_DONE                      = 0x40,
    INTEL_GUC_LOAD_STATUS_DPC_READY                        = 0x50,
    INTEL_GUC_LOAD_STATUS_DPC_ERROR                        = 0x60,
    INTEL_GUC_LOAD_STATUS_EXCEPTION                        = 0x70,
    INTEL_GUC_LOAD_STATUS_INIT_DATA_INVALID                = 0x71,
    INTEL_GUC_LOAD_STATUS_PXP_TEARDOWN_CTRL_ENABLED        = 0x72,
    INTEL_GUC_LOAD_STATUS_INVALID_INIT_DATA_RANGE_START,
    INTEL_GUC_LOAD_STATUS_MPU_DATA_INVALID                 = 0x73,
    INTEL_GUC_LOAD_STATUS_INIT_MMIO_SAVE_RESTORE_INVALID   = 0x74,
    INTEL_GUC_LOAD_STATUS_KLV_WORKAROUND_INIT_ERROR        = 0x75,
    INTEL_GUC_LOAD_STATUS_INVALID_INIT_DATA_RANGE_END,

    INTEL_GUC_LOAD_STATUS_READY                            = 0xF0,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum intel_bootrom_load_status {
    INTEL_BOOTROM_STATUS_NO_KEY_FOUND                 = 0x13,
    INTEL_BOOTROM_STATUS_AES_PROD_KEY_FOUND           = 0x1A,
    INTEL_BOOTROM_STATUS_PROD_KEY_CHECK_FAILURE       = 0x2B,
    INTEL_BOOTROM_STATUS_RSA_FAILED                   = 0x50,
    INTEL_BOOTROM_STATUS_PAVPC_FAILED                 = 0x73,
    INTEL_BOOTROM_STATUS_WOPCM_FAILED                 = 0x74,
    INTEL_BOOTROM_STATUS_LOADLOC_FAILED               = 0x75,
    INTEL_BOOTROM_STATUS_JUMP_PASSED                  = 0x76,
    INTEL_BOOTROM_STATUS_JUMP_FAILED                  = 0x77,
    INTEL_BOOTROM_STATUS_RC6CTXCONFIG_FAILED          = 0x79,
    INTEL_BOOTROM_STATUS_MPUMAP_INCORRECT             = 0x7A,
    INTEL_BOOTROM_STATUS_EXCEPTION                    = 0x7E,
}
