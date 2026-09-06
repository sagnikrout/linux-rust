//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/soc/qcom/socinfo.h
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
// SMEM item id, used to acquire handles to respective
// SMEM region.
//
pub const SMEM_HW_SW_BUILD_ID: c_int = 137;
pub const SMEM_SOCINFO_BUILD_ID_LENGTH: c_int = 32;
pub const SMEM_SOCINFO_CHIP_ID_LENGTH: c_int = 32;
//
// SoC version type with major number in the upper 16 bits and minor
// number in the lower 16 bits.
//

// Socinfo SMEM item structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct socinfo {
    pub fmt: __le32,
    pub id: __le32,
    pub ver: __le32,
    pub build_id: [c_char; SMEM_SOCINFO_BUILD_ID_LENGTH],
// Version 2
    pub raw_id: __le32,
    pub raw_ver: __le32,
// Version 3
    pub hw_plat: __le32,
// Version 4
    pub plat_ver: __le32,
// Version 5
    pub accessory_chip: __le32,
// Version 6
    pub hw_plat_subtype: __le32,
// Version 7
    pub pmic_model: __le32,
    pub pmic_die_rev: __le32,
// Version 8
    pub pmic_model_1: __le32,
    pub pmic_die_rev_1: __le32,
    pub pmic_model_2: __le32,
    pub pmic_die_rev_2: __le32,
// Version 9
    pub foundry_id: __le32,
// Version 10
    pub serial_num: __le32,
// Version 11
    pub num_pmics: __le32,
    pub pmic_array_offset: __le32,
// Version 12
    pub chip_family: __le32,
    pub raw_device_family: __le32,
    pub raw_device_num: __le32,
// Version 13
    pub nproduct_id: __le32,
    pub chip_id: [c_char; SMEM_SOCINFO_CHIP_ID_LENGTH],
// Version 14
    pub num_clusters: __le32,
    pub ncluster_array_offset: __le32,
    pub num_subset_parts: __le32,
    pub nsubset_parts_array_offset: __le32,
// Version 15
    pub nmodem_supported: __le32,
// Version 16
    pub feature_code: __le32,
    pub pcode: __le32,
    pub npartnamemap_offset: __le32,
    pub nnum_partname_mapping: __le32,
// Version 17
    pub oem_variant: __le32,
// Version 18
    pub num_kvps: __le32,
    pub kvps_offset: __le32,
// Version 19
    pub num_func_clusters: __le32,
    pub boot_cluster: __le32,
    pub boot_core: __le32,
// Version 20
    pub raw_package_type: __le32,
// Version 21, 22, 23
    pub reserve1: [__le32; 4],
}

// Internal feature codes
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qcom_socinfo_feature_code {
// External feature codes
    SOCINFO_FC_UNKNOWN = 0x0,
    SOCINFO_FC_AA,
    SOCINFO_FC_AB,
    SOCINFO_FC_AC,
    SOCINFO_FC_AD,
    SOCINFO_FC_AE,
    SOCINFO_FC_AF,
    SOCINFO_FC_AG,
    SOCINFO_FC_AH,
}

// Internal feature codes
// Valid values: 0 <= n <= 0xf

// Product codes
pub const SOCINFO_PC_UNKNOWN: c_int = 0;

