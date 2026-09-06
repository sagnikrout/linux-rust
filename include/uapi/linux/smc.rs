//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/smc.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note
//
// Shared Memory Communications over RDMA (SMC-R) and RoCE
//
// Definitions for generic netlink based configuration of an SMC-R PNET table
//
// Copyright IBM Corp. 2016
//
// Author(s):  Thomas Richter <tmricht@linux.vnet.ibm.com>
//
// Netlink SMC_PNETID attributes

pub const SMCR_GENL_FAMILY_VERSION: c_int = 1;
// gennetlink interface to access non-socket information from SMC module

pub const SMC_GENL_FAMILY_VERSION: c_int = 1;

// SMC_GENL_FAMILY commands
// SMC_GENL_FAMILY top level attributes
// SMC_GEN_SYS_INFO attributes
// SMC_NLA_LGR_D_V2_COMMON and SMC_NLA_LGR_R_V2_COMMON nested attributes
// SMC_NLA_LGR_R_V2 nested attributes
// SMC_GEN_LGR_SMCR attributes
// SMC_GEN_LINK_SMCR attributes
// SMC_GEN_LGR_SMCD attributes
// SMC_NLA_DEV_PORT nested attributes
// SMC_GEN_DEV_SMCD and SMC_GEN_DEV_SMCR attributes
// SMC_NLA_STATS_T_TX(RX)_RMB_SIZE nested attributes
// SMC_NLA_STATS_TX(RX)PLOAD_SIZE nested attributes
// SMC_NLA_STATS_T_TX(RX)_RMB_STATS nested attributes
// SMC_NLA_STATS_SMCD_TECH and _SMCR_TECH nested attributes
// SMC_GEN_STATS attributes
// SMC_GEN_FBACK_STATS attributes
// SMC_NETLINK_UEID attributes
// SMC_NETLINK_SEID attributes
// SMC_NETLINK_HS_LIMITATION attributes
// SMC socket options

