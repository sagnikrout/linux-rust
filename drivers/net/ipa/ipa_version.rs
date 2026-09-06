//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ipa/ipa_version.h
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
// Copyright (c) 2012-2018, The Linux Foundation. All rights reserved.
// Copyright (C) 2019-2024 Linaro Ltd.
//

//
// enum ipa_version
// @IPA_VERSION_3_0:	IPA version 3.0/GSI version 1.0
// @IPA_VERSION_3_1:	IPA version 3.1/GSI version 1.0
// @IPA_VERSION_3_5:	IPA version 3.5/GSI version 1.2
// @IPA_VERSION_3_5_1:	IPA version 3.5.1/GSI version 1.3
// @IPA_VERSION_4_0:	IPA version 4.0/GSI version 2.0
// @IPA_VERSION_4_1:	IPA version 4.1/GSI version 2.0
// @IPA_VERSION_4_2:	IPA version 4.2/GSI version 2.2
// @IPA_VERSION_4_5:	IPA version 4.5/GSI version 2.5
// @IPA_VERSION_4_7:	IPA version 4.7/GSI version 2.7
// @IPA_VERSION_4_9:	IPA version 4.9/GSI version 2.9
// @IPA_VERSION_4_11:	IPA version 4.11/GSI version 2.11 (2.1.1)
// @IPA_VERSION_5_0:	IPA version 5.0/GSI version 3.0
// @IPA_VERSION_5_1:	IPA version 5.1/GSI version 3.0
// @IPA_VERSION_5_2:	IPA version 5.2/GSI version 5.2
// @IPA_VERSION_5_5:	IPA version 5.5/GSI version 5.5
// @IPA_VERSION_COUNT:	Number of defined IPA versions
//
// Defines the version of IPA (and GSI) hardware present on the platform.
// Please update ipa_version_string() whenever a new version is added.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ipa_version {
    IPA_VERSION_3_0,
    IPA_VERSION_3_1,
    IPA_VERSION_3_5,
    IPA_VERSION_3_5_1,
    IPA_VERSION_4_0,
    IPA_VERSION_4_1,
    IPA_VERSION_4_2,
    IPA_VERSION_4_5,
    IPA_VERSION_4_7,
    IPA_VERSION_4_9,
    IPA_VERSION_4_11,
    IPA_VERSION_5_0,
    IPA_VERSION_5_1,
    IPA_VERSION_5_2,
    IPA_VERSION_5_5,
    IPA_VERSION_COUNT,			/* Last; not a version */
}

// Execution environment IDs
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gsi_ee_id {
    GSI_EE_AP		= 0x0,
    GSI_EE_MODEM		= 0x1,
    GSI_EE_UC		= 0x2,
    GSI_EE_TZ		= 0x3,
}
