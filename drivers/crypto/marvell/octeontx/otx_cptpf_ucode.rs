//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/marvell/octeontx/otx_cptpf_ucode.h
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

// CPT ucode name maximum length
pub const OTX_CPT_UCODE_NAME_LENGTH: c_int = 64;
//
// On OcteonTX 83xx platform, only one type of engines is allowed to be
// attached to an engine group.
//
pub const OTX_CPT_MAX_ETYPES_PER_GRP: c_int = 1;
// Default tar archive file names

// CPT ucode alignment
pub const OTX_CPT_UCODE_ALIGNMENT: c_int = 128;
// CPT ucode signature size
pub const OTX_CPT_UCODE_SIGN_LEN: c_int = 256;
// Microcode version string length
pub const OTX_CPT_UCODE_VER_STR_SZ: c_int = 44;
// Maximum number of supported engines/cores on OcteonTX 83XX platform
pub const OTX_CPT_MAX_ENGINES: c_int = 64;

// Microcode types
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum otx_cpt_ucode_type {
    OTX_CPT_AE_UC_TYPE =	1,  /* AE-MAIN */
    OTX_CPT_SE_UC_TYPE1 =	20, /* SE-MAIN - combination of 21 and 22 */
    OTX_CPT_SE_UC_TYPE2 =	21, /* Fast Path IPSec + AirCrypto */
    OTX_CPT_SE_UC_TYPE3 =	22, /*
// Hash + HMAC + FlexiCrypto + RNG + Full
// Feature IPSec + AirCrypto + Kasumi
//
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct otx_cpt_bitmap {
    pub bits: [c_ulong; OTX_CPT_ENGS_BITMASK_LEN],
    pub size: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct otx_cpt_engines {
    pub type: c_int,
    pub count: c_int,
}

// Microcode version number
#[repr(C)]
#[derive(Copy, Clone)]
pub struct otx_cpt_ucode_ver_num {
    pub nn: u8,
    pub xx: u8,
    pub yy: u8,
    pub zz: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct otx_cpt_ucode_hdr {
    pub ver_num: otx_cpt_ucode_ver_num,
    pub ver_str: [u8; OTX_CPT_UCODE_VER_STR_SZ],
    pub code_length: __be32,
    pub padding: [u32; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct otx_cpt_ucode {
    pub ver_str[OTX_CPT_UCODE_VER_STR_SZ];/*: *mut u8,
// ucode version in readable format
//
    pub /: *mut *mut otx_cpt_ucode_ver_num ver_num;/ ucode version number,
    pub /: *mut *mut char filename[OTX_CPT_UCODE_NAME_LENGTH]; / ucode filename,
    pub /: *mut *mut dma_addr_t dma; / phys address of ucode image,
    pub /: *mut *mut dma_addr_t align_dma; / aligned phys address of ucode image,
    pub /: *mut *mut *mut void va; / virt address of ucode image,
    pub /: *mut *mut *mut void align_va; / aligned virt address of ucode image,
    pub /: *mut *mut u32 size; / ucode image size,
    pub /: *mut *mut int type; / ucode image type SE or AE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tar_ucode_info_t {
    pub list: list_head,
    pub /: *mut *mut otx_cpt_ucode ucode;/ microcode information,
    pub /: *const *const *const u8 ucode_ptr; / pointer to microcode in tar archive,
}

// Maximum and current number of engines available for all engine groups
#[repr(C)]
#[derive(Copy, Clone)]
pub struct otx_cpt_engs_available {
    pub max_se_cnt: c_int,
    pub max_ae_cnt: c_int,
    pub se_cnt: c_int,
    pub ae_cnt: c_int,
}

// Engines reserved to an engine group
#[repr(C)]
#[derive(Copy, Clone)]
pub struct otx_cpt_engs_rsvd {
    pub /: *mut *mut int type; / engine type,
    pub /: *mut *mut int count; / number of engines attached,
    pub /: *mut *mut int offset; / constant offset of engine type in the bitmap,
    pub /: *mut *mut *mut unsigned long bmap; / attached engines bitmap,
    pub /: *mut *mut *mut otx_cpt_ucode ucode; / ucode used by these engines,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct otx_cpt_mirror_info {
    pub /*: *mut int is_ena;,
// is mirroring enabled, it is set only for engine
// group which mirrors another engine group
//
    pub /*: *mut int idx;,
// index of engine group which is mirrored by this
// group, set only for engine group which mirrors
// another group
//
    pub /*: *mut int ref_count;,
// number of times this engine group is mirrored by
// other groups, this is set only for engine group
// which is mirrored by other group(s)
//
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct otx_cpt_eng_grp_info {
    pub /: *mut *mut *mut otx_cpt_eng_grps g; / pointer to engine_groups structure,
    pub /: *mut *mut device_attribute info_attr; / group info entry attr,
// engines attached
    pub engs: [otx_cpt_engs_rsvd; OTX_CPT_MAX_ETYPES_PER_GRP],
// Microcode information
    pub ucode: [otx_cpt_ucode; OTX_CPT_MAX_ETYPES_PER_GRP],
// sysfs info entry name
    pub sysfs_info_name: [c_char; OTX_CPT_UCODE_NAME_LENGTH],
// engine group mirroring information
    pub mirror: otx_cpt_mirror_info,
    pub /: *mut *mut int idx; / engine group index,
    pub /*: *mut bool is_enabled;,
// is engine group enabled, engine group is enabled
// when it has engines attached and ucode loaded
//
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct otx_cpt_eng_grps {
    pub grp: [otx_cpt_eng_grp_info; OTX_CPT_MAX_ENGINE_GROUPS],
    pub /: *mut *mut device_attribute ucode_load_attr;/ ucode load attr,
    pub avail: otx_cpt_engs_available,
    pub lock: mutex,
    pub obj: *mut c_void,
    pub /: *mut *mut int engs_num; / total number of engines supported,
    pub /: *mut *mut int eng_types_supported; / engine types supported SE, AE,
    pub /: *mut *mut u8 eng_ref_cnt[OTX_CPT_MAX_ENGINES];/ engines reference count,
    pub /: *mut *mut bool is_ucode_load_created; / is ucode_load sysfs entry created,
    pub /: *mut *mut bool is_first_try; / is this first try to create kcrypto engine grp,
    pub /: *mut *mut bool is_rdonly; / do engine groups configuration can be modified,
}

extern "C" {
    pub fn otx_cpt_uc_supports_eng_type(ucode: *mut otx_cpt_ucode, eng_type: c_int) -> c_int;
}
