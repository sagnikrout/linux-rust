//! Automatically rewritten from C to Rust
//! Source: drivers/accel/amdxdna/npu3_regs.c
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
// Copyright (C) 2026, Advanced Micro Devices, Inc.
//

pub const NPU3_MBOX_BAR: c_int = 0;
pub const NPU3_MBOX_BUFFER_BAR: c_int = 2;
pub const NPU3_MBOX_INFO_OFF: c_uint = 0x0;
pub const NPU3_DOORBELL_BAR: c_int = 2;
pub const NPU3_DOORBELL_OFF: c_uint = 0x0;
// PCIe BAR Index for NPU3
pub const NPU3_REG_BAR_INDEX: c_int = 0;
pub const NPU3_PSP_BAR_INDEX: c_int = 4;
pub const NPU3_SMU_BAR_INDEX: c_int = 5;
pub const MMNPU_APERTURE3_BASE: c_uint = 0x3810000;
pub const MMNPU_APERTURE4_BASE: c_uint = 0x3B10000;

pub const MPASP_C2PMSG_123_ALT_1: c_uint = 0x3810AEC;
pub const MPASP_C2PMSG_156_ALT_1: c_uint = 0x3810B70;
pub const MPASP_C2PMSG_157_ALT_1: c_uint = 0x3810B74;
pub const MPASP_C2PMSG_73_ALT_1: c_uint = 0x3810A24;
pub const MP1_C2PMSG_59_ALT_1: c_uint = 0x3B109EC;
pub const MP1_C2PMSG_61_ALT_1: c_uint = 0x3B109F4;
pub const MP1_C2PMSG_60_ALT_1: c_uint = 0x3B109F0;
    static const struct amdxdna_fw_feature_tbl npu3_fw_feature_table[] = {
    { .major = 5, .min_minor = 10 },
    { 0 }
    };
    static const struct amdxdna_dev_priv npu3_dev_priv = {
    .npufw_path             = "npu.dev.sbin",
    .certfw_path            = "cert.dev.sbin",
    .mbox_bar		= NPU3_MBOX_BAR,
    .mbox_rbuf_bar		= NPU3_MBOX_BUFFER_BAR,
    .mbox_info_off		= NPU3_MBOX_INFO_OFF,
    .doorbell_off		= NPU3_DOORBELL_OFF,
    .psp_regs_off   = {
    DEFINE_BAR_OFFSET(PSP_CMD_REG,    NPU3_PSP, MPASP_C2PMSG_123_ALT_1),
    DEFINE_BAR_OFFSET(PSP_ARG0_REG,   NPU3_PSP, MPASP_C2PMSG_156_ALT_1),
    DEFINE_BAR_OFFSET(PSP_ARG1_REG,   NPU3_PSP, MPASP_C2PMSG_157_ALT_1),
    DEFINE_BAR_OFFSET(PSP_ARG2_REG,   NPU3_PSP, MPASP_C2PMSG_123_ALT_1),
    DEFINE_BAR_OFFSET(PSP_INTR_REG,   NPU3_PSP, MPASP_C2PMSG_73_ALT_1),
    DEFINE_BAR_OFFSET(PSP_STATUS_REG, NPU3_PSP, MPASP_C2PMSG_123_ALT_1),
    DEFINE_BAR_OFFSET(PSP_RESP_REG,   NPU3_PSP, MPASP_C2PMSG_156_ALT_1),
// npu3 doesn't use 8th pwaitmode register
    },
    .smu_regs_off   = {
    DEFINE_BAR_OFFSET(SMU_CMD_REG,  NPU3_SMU, MP1_C2PMSG_59_ALT_1),
    DEFINE_BAR_OFFSET(SMU_ARG_REG,  NPU3_SMU, MP1_C2PMSG_61_ALT_1),
    DEFINE_BAR_OFFSET(SMU_INTR_REG, NPU3_SMU, MMNPU_APERTURE4_BASE),
    DEFINE_BAR_OFFSET(SMU_RESP_REG, NPU3_SMU, MP1_C2PMSG_60_ALT_1),
    DEFINE_BAR_OFFSET(SMU_OUT_REG,  NPU3_SMU, MP1_C2PMSG_61_ALT_1),
    },
    };
    static const struct amdxdna_dev_priv npu3_dev_vf_priv = {
// vf device does not load firmware
    .mbox_bar		= NPU3_MBOX_BAR,
    .mbox_rbuf_bar		= NPU3_MBOX_BUFFER_BAR,
    .mbox_info_off		= NPU3_MBOX_INFO_OFF,
// vf device does not have smu and psp
    };
    const struct amdxdna_dev_info dev_npu3_pf_info = {
    .mbox_bar		= NPU3_MBOX_BAR,
    .sram_bar		= NPU3_MBOX_BUFFER_BAR,
    .psp_bar                = NPU3_PSP_BAR_INDEX,
    .smu_bar		= NPU3_SMU_BAR_INDEX,
    .default_vbnv		= "RyzenAI-npu3-pf",
    .device_type		= AMDXDNA_DEV_TYPE_PF,
    .dev_priv		= &npu3_dev_priv,
    .fw_feature_tbl		= npu3_fw_feature_table,
    .ops			= &aie4_pf_ops,
    };
    const struct amdxdna_dev_info dev_npu3_vf_info = {
    .mbox_bar		= NPU3_MBOX_BAR,
    .sram_bar		= NPU3_MBOX_BUFFER_BAR,
    .doorbell_bar		= NPU3_DOORBELL_BAR,
    .default_vbnv		= "RyzenAI-npu3-vf",
    .device_type		= AMDXDNA_DEV_TYPE_UMQ,
    .dev_priv		= &npu3_dev_vf_priv,
    .fw_feature_tbl		= npu3_fw_feature_table,
    .ops			= &aie4_vf_ops,
    };
