//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/marvell/octeontx2/otx2_cpt_common.h
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
// Copyright (C) 2020 Marvell.
//

pub const OTX2_CPT_MAX_VFS_NUM: c_int = 128;

pub const OTX2_CPT_INVALID_CRYPTO_ENG_GRP: c_uint = 0xFF;
pub const OTX2_CPT_NAME_LENGTH: c_int = 64;
pub const OTX2_CPT_DMA_MINALIGN: c_int = 128;
// HW capability flags
pub const CN10K_MBOX: c_int = 0;
pub const CN10K_LMTST: c_int = 1;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum otx2_cpt_eng_type {
    OTX2_CPT_AE_TYPES = 1,
    OTX2_CPT_SE_TYPES = 2,
    OTX2_CPT_IE_TYPES = 3,
    OTX2_CPT_MAX_ENG_TYPES,
}

// Take mbox id from end of CPT mbox range in AF (range 0xA00 - 0xBFF)
pub const MBOX_MSG_RX_INLINE_IPSEC_LF_CFG: c_uint = 0xBFE;
pub const MBOX_MSG_GET_ENG_GRP_NUM: c_uint = 0xBFF;
pub const MBOX_MSG_GET_CAPS: c_uint = 0xBFD;
pub const MBOX_MSG_GET_KVF_LIMITS: c_uint = 0xBFC;
//
// Message request to config cpt lf for inline inbound ipsec.
// This message is only used between CPT PF <-> CPT VF
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct otx2_cpt_rx_inline_lf_cfg {
    pub hdr: mbox_msghdr,
    pub sso_pf_func: u16,
    pub param1: u16,
    pub param2: u16,
    pub opcode: u16,
    pub credit: u32,
    pub credit_th: u32,
    pub bpid: u16,
    pub reserved: u32,
    pub 1: u8 ctx_ilen_valid :,
    pub 7: u8 ctx_ilen :,
}

//
// Message request and response to get engine group number
// which has attached a given type of engines (SE, AE, IE)
// This messages are only used between CPT PF <=> CPT VF
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct otx2_cpt_egrp_num_msg {
    pub hdr: mbox_msghdr,
    pub eng_type: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct otx2_cpt_egrp_num_rsp {
    pub hdr: mbox_msghdr,
    pub eng_type: u8,
    pub eng_grp_num: u8,
}

//
// Message request and response to get kernel crypto limits
// This messages are only used between CPT PF <-> CPT VF
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct otx2_cpt_kvf_limits_msg {
    pub hdr: mbox_msghdr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct otx2_cpt_kvf_limits_rsp {
    pub hdr: mbox_msghdr,
    pub kvf_limits: u8,
}

// CPT HW capabilities
#[repr(C)]
#[derive(Copy, Clone)]
pub union otx2_cpt_eng_caps {
    pub u: u64,
    pub reserved_0_4:5: u64,
    pub mul:1: u64,
    pub sha1_sha2:1: u64,
    pub chacha20:1: u64,
    pub zuc_snow3g:1: u64,
    pub sha3:1: u64,
    pub aes:1: u64,
    pub kasumi:1: u64,
    pub des:1: u64,
    pub crc:1: u64,
    pub mmul:1: u64,
    pub reserved_15_33:19: u64,
    pub pdcp_chain:1: u64,
    pub reserved_35_63:29: u64,
}

//
// Message request and response to get HW capabilities for each
// engine type (SE, IE, AE).
// This messages are only used between CPT PF <=> CPT VF
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct otx2_cpt_caps_msg {
    pub hdr: mbox_msghdr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct otx2_cpt_caps_rsp {
    pub hdr: mbox_msghdr,
    pub cpt_pf_drv_version: u16,
    pub cpt_revision: u8,
    pub eng_caps: [otx2_cpt_eng_caps; OTX2_CPT_MAX_ENG_TYPES],
}

extern "C" {
    pub fn is_dev_otx2(is_dev_cn10ka_ax(pdev: pdev) ||) -> return;
}
extern "C" {
    pub fn otx2_cpt_send_ready_msg(mbox: *mut otx2_mbox, pdev: *mut pci_dev) -> c_int;
}
extern "C" {
    pub fn otx2_cpt_send_mbox_msg(mbox: *mut otx2_mbox, pdev: *mut pci_dev) -> c_int;
}
extern "C" {
    pub fn otx2_cpt_attach_rscrs_msg(lfs: *mut otx2_cptlfs_info) -> c_int;
}
extern "C" {
    pub fn otx2_cpt_detach_rsrcs_msg(lfs: *mut otx2_cptlfs_info) -> c_int;
}
extern "C" {
    pub fn otx2_cpt_msix_offset_msg(lfs: *mut otx2_cptlfs_info) -> c_int;
}
extern "C" {
    pub fn otx2_cpt_sync_mbox_msg(mbox: *mut otx2_mbox) -> c_int;
}
extern "C" {
    pub fn otx2_cpt_lf_reset_msg(lfs: *mut otx2_cptlfs_info, slot: c_int) -> c_int;
}
extern "C" {
    pub fn otx2_cpt_lmtst_tbl_setup_msg(lfs: *mut otx2_cptlfs_info) -> c_int;
}
