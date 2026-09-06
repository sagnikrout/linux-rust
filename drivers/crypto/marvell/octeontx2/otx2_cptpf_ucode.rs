//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/marvell/octeontx2/otx2_cptpf_ucode.h
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

//
// On OcteonTX2 platform IPSec ucode can use both IE and SE engines therefore
// IE and SE engines can be attached to the same engine group.
//
pub const OTX2_CPT_MAX_ETYPES_PER_GRP: c_int = 2;
// CPT ucode signature size
pub const OTX2_CPT_UCODE_SIGN_LEN: c_int = 256;
// Microcode version string length
pub const OTX2_CPT_UCODE_VER_STR_SZ: c_int = 44;
// Maximum number of supported engines/cores on OcteonTX2/CN10K platform
pub const OTX2_CPT_MAX_ENGINES: c_int = 144;

// Microcode types
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum otx2_cpt_ucode_type {
    OTX2_CPT_AE_UC_TYPE = 1,  /* AE-MAIN */
    OTX2_CPT_SE_UC_TYPE1 = 20,/* SE-MAIN - combination of 21 and 22 */
    OTX2_CPT_SE_UC_TYPE2 = 21,/* Fast Path IPSec + AirCrypto */
    OTX2_CPT_SE_UC_TYPE3 = 22,/*
// Hash + HMAC + FlexiCrypto + RNG +
// Full Feature IPSec + AirCrypto + Kasumi
//
    OTX2_CPT_IE_UC_TYPE1 = 30, /* IE-MAIN - combination of 31 and 32 */
    OTX2_CPT_IE_UC_TYPE2 = 31, /* Fast Path IPSec */
    OTX2_CPT_IE_UC_TYPE3 = 32, /*
// Hash + HMAC + FlexiCrypto + RNG +
// Full Future IPSec
//
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct otx2_cpt_bitmap {
    pub bits: [c_ulong; OTX2_CPT_ENGS_BITMASK_LEN],
    pub size: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct otx2_cpt_engines {
    pub type: c_int,
    pub count: c_int,
}

// Microcode version number
#[repr(C)]
#[derive(Copy, Clone)]
pub struct otx2_cpt_ucode_ver_num {
    pub nn: u8,
    pub xx: u8,
    pub yy: u8,
    pub zz: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct otx2_cpt_ucode_hdr {
    pub ver_num: otx2_cpt_ucode_ver_num,
    pub ver_str: [u8; OTX2_CPT_UCODE_VER_STR_SZ],
    pub code_length: __be32,
    pub padding: [u32; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct otx2_cpt_ucode {
    pub 1];/*: *mut u8 ver_str[OTX2_CPT_UCODE_VER_STR_SZ +,
// ucode version in readable
// format
//
    pub /: *mut *mut otx2_cpt_ucode_ver_num ver_num;/ ucode version number,
    pub /: *mut *mut char filename[OTX2_CPT_NAME_LENGTH];/ ucode filename,
    pub /: *mut *mut dma_addr_t dma; / phys address of ucode image,
    pub /: *mut *mut *mut void va; / virt address of ucode image,
    pub /: *mut *mut u32 size; / ucode image size,
    pub /: *mut *mut int type; / ucode image type SE, IE, AE or SE+IE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct otx2_cpt_uc_info_t {
    pub list: list_head,
    pub /: *mut *mut otx2_cpt_ucode ucode;/ microcode information,
    pub fw: *const firmware,
}

// Maximum and current number of engines available for all engine groups
#[repr(C)]
#[derive(Copy, Clone)]
pub struct otx2_cpt_engs_available {
    pub max_se_cnt: c_int,
    pub max_ie_cnt: c_int,
    pub max_ae_cnt: c_int,
    pub se_cnt: c_int,
    pub ie_cnt: c_int,
    pub ae_cnt: c_int,
}

// Engines reserved to an engine group
#[repr(C)]
#[derive(Copy, Clone)]
pub struct otx2_cpt_engs_rsvd {
    pub /: *mut *mut int type; / engine type,
    pub /: *mut *mut int count; / number of engines attached,
    pub /: *mut *mut int offset; / constant offset of engine type in the bitmap,
    pub /: *mut *mut *mut unsigned long bmap; / attached engines bitmap,
    pub /: *mut *mut *mut otx2_cpt_ucode ucode; / ucode used by these engines,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct otx2_cpt_mirror_info {
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
pub struct otx2_cpt_eng_grp_info {
    pub /: *mut *mut *mut otx2_cpt_eng_grps g; / pointer to engine_groups structure,
// engines attached
    pub engs: [otx2_cpt_engs_rsvd; OTX2_CPT_MAX_ETYPES_PER_GRP],
// ucodes information
    pub ucode: [otx2_cpt_ucode; OTX2_CPT_MAX_ETYPES_PER_GRP],
// engine group mirroring information
    pub mirror: otx2_cpt_mirror_info,
    pub /: *mut *mut int idx; / engine group index,
    pub /*: *mut bool is_enabled;,
// is engine group enabled, engine group is enabled
// when it has engines attached and ucode loaded
//
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct otx2_cpt_eng_grps {
    pub lock: mutex,
    pub grp: [otx2_cpt_eng_grp_info; OTX2_CPT_MAX_ENGINE_GROUPS],
    pub avail: otx2_cpt_engs_available,
    pub /: *mut *mut *mut void obj; / device specific data,
    pub /: *mut *mut int engs_num; / total number of engines supported,
    pub /: *mut *mut u8 eng_ref_cnt[OTX2_CPT_MAX_ENGINES];/ engines reference count,
    pub /: *mut *mut bool is_grps_created; / Is the engine groups are already created,
    pub rid: u16,
}

extern "C" {
    pub fn otx2_cpt_disable_all_cores(cptpf: *mut otx2_cptpf_dev) -> c_int;
}
extern "C" {
    pub fn otx2_cpt_get_eng_grp(eng_grps: *mut otx2_cpt_eng_grps, eng_type: c_int) -> c_int;
}
extern "C" {
    pub fn otx2_cpt_discover_eng_capabilities(cptpf: *mut otx2_cptpf_dev) -> c_int;
}
