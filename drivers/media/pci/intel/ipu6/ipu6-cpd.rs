//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/pci/intel/ipu6/ipu6-cpd.h
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
// Copyright (C) 2015--2024 Intel Corporation
pub const IPU6_CPD_SIZE_OF_FW_ARCH_VERSION: c_int = 7;
pub const IPU6_CPD_SIZE_OF_SYSTEM_VERSION: c_int = 11;
pub const IPU6_CPD_SIZE_OF_COMPONENT_NAME: c_int = 12;
pub const IPU6_CPD_METADATA_EXTN_TYPE_IUNIT: c_uint = 0x10;
pub const IPU6_CPD_METADATA_IMAGE_TYPE_RESERVED: c_int = 0;
pub const IPU6_CPD_METADATA_IMAGE_TYPE_BOOTLOADER: c_int = 1;
pub const IPU6_CPD_METADATA_IMAGE_TYPE_MAIN_FIRMWARE: c_int = 2;
pub const IPU6_CPD_PKG_DIR_PSYS_SERVER_IDX: c_int = 0;
pub const IPU6_CPD_PKG_DIR_ISYS_SERVER_IDX: c_int = 1;
pub const IPU6_CPD_PKG_DIR_CLIENT_PG_TYPE: c_int = 3;
pub const IPU6_CPD_METADATA_HASH_KEY_SIZE: c_int = 48;
pub const IPU6SE_CPD_METADATA_HASH_KEY_SIZE: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipu6_cpd_module_data_hdr {
    pub hdr_len: u32,
    pub endian: u32,
    pub fw_pkg_date: u32,
    pub hive_sdk_date: u32,
    pub compiler_date: u32,
    pub target_platform_type: u32,
    pub sys_ver: [u8; IPU6_CPD_SIZE_OF_SYSTEM_VERSION],
    pub fw_arch_ver: [u8; IPU6_CPD_SIZE_OF_FW_ARCH_VERSION],
    pub rsvd: [u8; 2],
    pub __packed: },
//
// ipu6_cpd_hdr structure updated as the chksum and
// sub_partition_name is unused on host side
// CSE layout version 1.6 for IPU6SE (hdr_len = 0x10)
// CSE layout version 1.7 for IPU6 (hdr_len = 0x14)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipu6_cpd_hdr {
    pub hdr_mark: u32,
    pub ent_cnt: u32,
    pub hdr_ver: u8,
    pub ent_ver: u8,
    pub hdr_len: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipu6_cpd_ent {
    pub name: [u8; IPU6_CPD_SIZE_OF_COMPONENT_NAME],
    pub offset: u32,
    pub len: u32,
    pub rsvd: [u8; 4],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipu6_cpd_metadata_cmpnt_hdr {
    pub id: u32,
    pub size: u32,
    pub ver: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipu6_cpd_metadata_cmpnt {
    pub hdr: ipu6_cpd_metadata_cmpnt_hdr,
    pub sha2_hash: [u8; IPU6_CPD_METADATA_HASH_KEY_SIZE],
    pub entry_point: u32,
    pub icache_base_offs: u32,
    pub attrs: [u8; 16],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipu6se_cpd_metadata_cmpnt {
    pub hdr: ipu6_cpd_metadata_cmpnt_hdr,
    pub sha2_hash: [u8; IPU6SE_CPD_METADATA_HASH_KEY_SIZE],
    pub entry_point: u32,
    pub icache_base_offs: u32,
    pub attrs: [u8; 16],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipu6_cpd_metadata_extn {
    pub extn_type: u32,
    pub len: u32,
    pub img_type: u32,
    pub rsvd: [u8; 16],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipu6_cpd_client_pkg_hdr {
    pub prog_list_offs: u32,
    pub prog_list_size: u32,
    pub prog_desc_offs: u32,
    pub prog_desc_size: u32,
    pub pg_manifest_offs: u32,
    pub pg_manifest_size: u32,
    pub prog_bin_offs: u32,
    pub prog_bin_size: u32,
    pub __packed: },
    pub src): *const *const int ipu6_cpd_create_pkg_dir(struct ipu6_bus_device adev, void,
    pub adev): *mut void ipu6_cpd_free_pkg_dir(struct ipu6_bus_device,
    pub cpd_file_size): c_ulong,
