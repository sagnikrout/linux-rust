//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/mellanox/mlxfw/mlxfw_mfa2_format.h
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


// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
// Copyright (c) 2017-2019 Mellanox Technologies. All rights reserved

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlxfw_mfa2_tlv_type {
    MLXFW_MFA2_TLV_MULTI_PART = 0x01,
    MLXFW_MFA2_TLV_PACKAGE_DESCRIPTOR = 0x02,
    MLXFW_MFA2_TLV_COMPONENT_DESCRIPTOR = 0x04,
    MLXFW_MFA2_TLV_COMPONENT_PTR = 0x22,
    MLXFW_MFA2_TLV_PSID = 0x2A,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlxfw_mfa2_compression_type {
    MLXFW_MFA2_COMPRESSION_TYPE_NONE,
    MLXFW_MFA2_COMPRESSION_TYPE_XZ,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlxfw_mfa2_tlv_package_descriptor {
    pub num_components: __be16,
    pub num_devices: __be16,
    pub cb_offset: __be32,
    pub cb_archive_size: __be32,
    pub cb_size_h: __be32,
    pub cb_size_l: __be32,
    pub padding: [u8; 3],
    pub cv_compression: u8,
    pub user_data_offset: __be32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlxfw_mfa2_tlv_multi {
    pub num_extensions: __be16,
    pub total_len: __be16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlxfw_mfa2_tlv_psid {
    pub psid): DECLARE_FLEX_ARRAY(u8,,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlxfw_mfa2_tlv_component_ptr {
    pub storage_id: __be16,
    pub component_index: __be16,
    pub storage_address: __be32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlxfw_mfa2_tlv_component_descriptor {
    pub pldm_classification: __be16,
    pub identifier: __be16,
    pub cb_offset_h: __be32,
    pub cb_offset_l: __be32,
    pub size: __be32,
    pub __packed: },
