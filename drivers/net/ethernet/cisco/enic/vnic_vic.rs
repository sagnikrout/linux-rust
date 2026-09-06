//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/cisco/enic/vnic_vic.h
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
// Copyright 2010 Cisco Systems, Inc.  All rights reserved.
// Note: All integer fields in NETWORK byte order
// Note: String field lengths include null char

pub const VIC_PROVINFO_GENERIC_TYPE: c_uint = 0x4;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vic_generic_prov_tlv_type {
    VIC_GENERIC_PROV_TLV_PORT_PROFILE_NAME_STR = 0,
    VIC_GENERIC_PROV_TLV_CLIENT_MAC_ADDR = 1,
    VIC_GENERIC_PROV_TLV_CLIENT_NAME_STR = 2,
    VIC_GENERIC_PROV_TLV_CLUSTER_PORT_NAME_STR = 3,
    VIC_GENERIC_PROV_TLV_CLUSTER_PORT_UUID_STR = 4,
    VIC_GENERIC_PROV_TLV_CLUSTER_UUID_STR = 5,
    VIC_GENERIC_PROV_TLV_CLUSTER_NAME_STR = 7,
    VIC_GENERIC_PROV_TLV_HOST_UUID_STR = 8,
    VIC_GENERIC_PROV_TLV_CLIENT_UUID_STR = 9,
    VIC_GENERIC_PROV_TLV_INCARNATION_NUMBER = 10,
    VIC_GENERIC_PROV_TLV_OS_TYPE = 11,
    VIC_GENERIC_PROV_TLV_OS_VENDOR = 12,
    VIC_GENERIC_PROV_TLV_CLIENT_TYPE = 15,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vic_generic_prov_os_type {
    VIC_GENERIC_PROV_OS_TYPE_UNKNOWN = 0,
    VIC_GENERIC_PROV_OS_TYPE_ESX = 1,
    VIC_GENERIC_PROV_OS_TYPE_LINUX = 2,
    VIC_GENERIC_PROV_OS_TYPE_WINDOWS = 3,
    VIC_GENERIC_PROV_OS_TYPE_SOLARIS = 4,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vic_provinfo {
    pub /: *mut *mut u8 oui[3]; / OUI of data provider,
    pub /: *mut *mut u8 type; / provider-specific type,
    pub /: *mut *mut u32 length; / length of data below,
    pub /: *mut *mut u32 num_tlvs; / number of tlvs,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vic_provinfo_tlv {
    pub type: u16,
    pub length: u16,
    pub value: [u8; 0],
    pub tlv: [}; ],
    pub __packed: },

    pub \: err = vic_provinfo_add_tlv(vp, tlvtype, tlvlen, data);,
    pub \: goto add_tlv_failure;,
pub const VIC_PROVINFO_MAX_DATA: c_int = 1385;

    pub type): u8,
    pub vp): *mut void vic_provinfo_free(struct vic_provinfo,
    pub value): *const c_void,
    pub vp): *mut size_t vic_provinfo_size(struct vic_provinfo,
