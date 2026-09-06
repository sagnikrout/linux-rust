//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/gt/uc/abi/guc_klvs_abi.h
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


// SPDX-License-Identifier: MIT
//
// Copyright © 2021 Intel Corporation
//

//
// DOC: GuC KLV
//
// +---+-------+--------------------------------------------------------------+
// |   | Bits  | Description                                                  |
// +===+=======+==============================================================+
// | 0 | 31:16 | **KEY** - KLV key identifier                                 |
// |   |       |   - `GuC Self Config KLVs`_                                  |
// |   |       |                                                              |
// |   +-------+--------------------------------------------------------------+
// |   |  15:0 | **LEN** - length of VALUE (in 32bit dwords)                  |
// +---+-------+--------------------------------------------------------------+
// | 1 |  31:0 | **VALUE** - actual value of the KLV (format depends on KEY)  |
// +---+-------+                                                              |
// |...|       |                                                              |
// +---+-------+                                                              |
// | n |  31:0 |                                                              |
// +---+-------+--------------------------------------------------------------+
//

//
// DOC: GuC Self Config KLVs
//
// `GuC KLV`_ keys available for use with HOST2GUC_SELF_CFG_.
//
// _`GUC_KLV_SELF_CFG_H2G_CTB_ADDR` : 0x0902
// Refers to 64 bit Global Gfx address of H2G `CT Buffer`_.
// Should be above WOPCM address but below APIC base address for native mode.
//
// _`GUC_KLV_SELF_CFG_H2G_CTB_DESCRIPTOR_ADDR` : 0x0903
// Refers to 64 bit Global Gfx address of H2G `CTB Descriptor`_.
// Should be above WOPCM address but below APIC base address for native mode.
//
// _`GUC_KLV_SELF_CFG_H2G_CTB_SIZE` : 0x0904
// Refers to size of H2G `CT Buffer`_ in bytes.
// Should be a multiple of 4K.
//
// _`GUC_KLV_SELF_CFG_G2H_CTB_ADDR` : 0x0905
// Refers to 64 bit Global Gfx address of G2H `CT Buffer`_.
// Should be above WOPCM address but below APIC base address for native mode.
//
// _`GUC_KLV_SELF_CFG_G2H_CTB_DESCRIPTOR_ADDR` : 0x0906
// Refers to 64 bit Global Gfx address of G2H `CTB Descriptor`_.
// Should be above WOPCM address but below APIC base address for native mode.
//
// _`GUC_KLV_SELF_CFG_G2H_CTB_SIZE` : 0x0907
// Refers to size of G2H `CT Buffer`_ in bytes.
// Should be a multiple of 4K.
//
pub const GUC_KLV_SELF_CFG_H2G_CTB_ADDR_KEY: c_uint = 0x0902;

pub const GUC_KLV_SELF_CFG_H2G_CTB_DESCRIPTOR_ADDR_KEY: c_uint = 0x0903;

pub const GUC_KLV_SELF_CFG_H2G_CTB_SIZE_KEY: c_uint = 0x0904;

pub const GUC_KLV_SELF_CFG_G2H_CTB_ADDR_KEY: c_uint = 0x0905;

pub const GUC_KLV_SELF_CFG_G2H_CTB_DESCRIPTOR_ADDR_KEY: c_uint = 0x0906;

pub const GUC_KLV_SELF_CFG_G2H_CTB_SIZE_KEY: c_uint = 0x0907;

//
// Global scheduling policy update keys.
//
// Per context scheduling policy update keys.
//
// Workaround keys:
//
