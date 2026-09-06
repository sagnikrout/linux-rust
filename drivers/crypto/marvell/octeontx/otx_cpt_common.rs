//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/marvell/octeontx/otx_cpt_common.h
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
// Marvell OcteonTX CPT driver
//
// Copyright (C) 2019 Marvell International Ltd.
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License version 2 as
// published by the Free Software Foundation.
//

pub const OTX_CPT_MAX_MBOX_DATA_STR_SIZE: c_int = 64;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum otx_cptpf_type {
    OTX_CPT_AE = 2,
    OTX_CPT_SE = 3,
    BAD_OTX_CPTPF_TYPE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum otx_cptvf_type {
    OTX_CPT_AE_TYPES = 1,
    OTX_CPT_SE_TYPES = 2,
    BAD_OTX_CPTVF_TYPE,
}

// VF-PF message opcodes
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum otx_cpt_mbox_opcode {
    OTX_CPT_MSG_VF_UP = 1,
    OTX_CPT_MSG_VF_DOWN,
    OTX_CPT_MSG_READY,
    OTX_CPT_MSG_QLEN,
    OTX_CPT_MSG_QBIND_GRP,
    OTX_CPT_MSG_VQ_PRIORITY,
    OTX_CPT_MSG_PF_TYPE,
    OTX_CPT_MSG_ACK,
    OTX_CPT_MSG_NACK
}

// OcteonTX CPT mailbox structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct otx_cpt_mbox {
    pub /: *mut *mut u64 msg; / Message type MBOX[0],
    pub /: *mut *mut u64 data;/ Data MBOX[1],
}
