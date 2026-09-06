//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/mediatek/mtk_ppe_regs.h
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
// Copyright (C) 2020 Felix Fietkau <nbd@nbd.name>
pub const MTK_PPE_GLO_CFG: c_uint = 0x200;

pub const MTK_PPE_FLOW_CFG: c_uint = 0x204;

pub const MTK_PPE_IP_PROTO_CHK: c_uint = 0x208;

pub const MTK_PPE_TB_CFG: c_uint = 0x21c;

pub const MTK_PPE_BIND_LMT1: c_uint = 0x230;

pub const MTK_PPE_KEEPALIVE: c_uint = 0x234;
pub const MTK_PPE_TB_BASE: c_uint = 0x220;
pub const MTK_PPE_TB_USED: c_uint = 0x224;

pub const MTK_PPE_BIND_RATE: c_uint = 0x228;

pub const MTK_PPE_BIND_LIMIT0: c_uint = 0x22c;

pub const MTK_PPE_BIND_LIMIT1: c_uint = 0x230;

pub const MTK_PPE_KEEPALIVE: c_uint = 0x234;

pub const MTK_PPE_UNBIND_AGE: c_uint = 0x238;

pub const MTK_PPE_BIND_AGE0: c_uint = 0x23c;

pub const MTK_PPE_BIND_AGE1: c_uint = 0x240;

pub const MTK_PPE_HASH_SEED: c_uint = 0x244;
pub const MTK_PPE_DEFAULT_CPU_PORT: c_uint = 0x248;

pub const MTK_PPE_DEFAULT_CPU_PORT1: c_uint = 0x24c;
pub const MTK_PPE_MTU_DROP: c_uint = 0x308;
pub const MTK_PPE_VLAN_MTU0: c_uint = 0x30c;

pub const MTK_PPE_VLAN_MTU1: c_uint = 0x310;

pub const MTK_PPE_VPM_TPID: c_uint = 0x318;
pub const MTK_PPE_CACHE_CTL: c_uint = 0x320;

pub const MTK_PPE_MIB_CFG: c_uint = 0x334;

pub const MTK_PPE_MIB_TB_BASE: c_uint = 0x338;
pub const MTK_PPE_MIB_SER_CR: c_uint = 0x33C;

pub const MTK_PPE_MIB_SER_R0: c_uint = 0x340;

pub const MTK_PPE_MIB_SER_R1: c_uint = 0x344;

pub const MTK_PPE_MIB_SER_R2: c_uint = 0x348;

pub const MTK_PPE_MIB_SER_R3: c_uint = 0x34c;
pub const MTK_PPE_MIB_CACHE_CTL: c_uint = 0x350;

pub const MTK_PPE_SBW_CTRL: c_uint = 0x374;
