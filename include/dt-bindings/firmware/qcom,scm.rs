//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/firmware/qcom,scm.h
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


// SPDX-License-Identifier: GPL-2.0-only OR BSD-2-Clause
//
// Copyright (c) 2010-2015, 2018-2019 The Linux Foundation. All rights reserved.
// Copyright (C) 2015 Linaro Ltd.
// Copyright (c) 2023 Qualcomm Innovation Center, Inc. All rights reserved.
//
pub const QCOM_SCM_VMID_TZ: c_uint = 0x1;
pub const QCOM_SCM_VMID_HLOS: c_uint = 0x3;
pub const QCOM_SCM_VMID_SSC_Q6: c_uint = 0x5;
pub const QCOM_SCM_VMID_ADSP_Q6: c_uint = 0x6;
pub const QCOM_SCM_VMID_CP_TOUCH: c_uint = 0x8;
pub const QCOM_SCM_VMID_CP_BITSTREAM: c_uint = 0x9;
pub const QCOM_SCM_VMID_CP_PIXEL: c_uint = 0xA;
pub const QCOM_SCM_VMID_CP_NON_PIXEL: c_uint = 0xB;
pub const QCOM_SCM_VMID_CP_CAMERA: c_uint = 0xD;
pub const QCOM_SCM_VMID_HLOS_FREE: c_uint = 0xE;
pub const QCOM_SCM_VMID_MSS_MSA: c_uint = 0xF;
pub const QCOM_SCM_VMID_MSS_NONMSA: c_uint = 0x10;
pub const QCOM_SCM_VMID_CP_SEC_DISPLAY: c_uint = 0x11;
pub const QCOM_SCM_VMID_CP_APP: c_uint = 0x12;
pub const QCOM_SCM_VMID_LPASS: c_uint = 0x16;
pub const QCOM_SCM_VMID_WLAN: c_uint = 0x18;
pub const QCOM_SCM_VMID_WLAN_CE: c_uint = 0x19;
pub const QCOM_SCM_VMID_CP_SPSS_SP: c_uint = 0x1A;
pub const QCOM_SCM_VMID_CP_CAMERA_PREVIEW: c_uint = 0x1D;
pub const QCOM_SCM_VMID_CDSP: c_uint = 0x1E;
pub const QCOM_SCM_VMID_CP_SPSS_SP_SHARED: c_uint = 0x22;
pub const QCOM_SCM_VMID_CP_SPSS_HLOS_SHARED: c_uint = 0x24;
pub const QCOM_SCM_VMID_ADSP_HEAP: c_uint = 0x25;
pub const QCOM_SCM_VMID_CP_CDSP: c_uint = 0x2A;
pub const QCOM_SCM_VMID_NAV: c_uint = 0x2B;
pub const QCOM_SCM_VMID_TVM: c_uint = 0x2D;
pub const QCOM_SCM_VMID_OEMVM: c_uint = 0x31;
pub const QCOM_SCM_VMID_CP_ADSP_SHARED: c_uint = 0x33;
