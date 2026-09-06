//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/irqchip/riscv-aplic.h
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
//
// Copyright (C) 2021 Western Digital Corporation or its affiliates.
// Copyright (C) 2022 Ventana Micro Systems Inc.
//

pub const APLIC_MAX_SOURCE: c_int = 1024;
pub const APLIC_DOMAINCFG: c_uint = 0x0000;
pub const APLIC_DOMAINCFG_RDONLY: c_uint = 0x80000000;

pub const APLIC_SOURCECFG_BASE: c_uint = 0x0004;

pub const APLIC_SOURCECFG_CHILDIDX_MASK: c_uint = 0x000003ff;
pub const APLIC_SOURCECFG_SM_MASK: c_uint = 0x00000007;
pub const APLIC_SOURCECFG_SM_INACTIVE: c_uint = 0x0;
pub const APLIC_SOURCECFG_SM_DETACH: c_uint = 0x1;
pub const APLIC_SOURCECFG_SM_EDGE_RISE: c_uint = 0x4;
pub const APLIC_SOURCECFG_SM_EDGE_FALL: c_uint = 0x5;
pub const APLIC_SOURCECFG_SM_LEVEL_HIGH: c_uint = 0x6;
pub const APLIC_SOURCECFG_SM_LEVEL_LOW: c_uint = 0x7;
pub const APLIC_MMSICFGADDR: c_uint = 0x1bc0;
pub const APLIC_MMSICFGADDRH: c_uint = 0x1bc4;
pub const APLIC_SMSICFGADDR: c_uint = 0x1bc8;
pub const APLIC_SMSICFGADDRH: c_uint = 0x1bcc;

pub const APLIC_xMSICFGADDRH_HHXS_MASK: c_uint = 0x1f;
pub const APLIC_xMSICFGADDRH_HHXS_SHIFT: c_int = 24;

pub const APLIC_xMSICFGADDRH_LHXS_MASK: c_uint = 0x7;
pub const APLIC_xMSICFGADDRH_LHXS_SHIFT: c_int = 20;

pub const APLIC_xMSICFGADDRH_HHXW_MASK: c_uint = 0x7;
pub const APLIC_xMSICFGADDRH_HHXW_SHIFT: c_int = 16;

pub const APLIC_xMSICFGADDRH_LHXW_MASK: c_uint = 0xf;
pub const APLIC_xMSICFGADDRH_LHXW_SHIFT: c_int = 12;

pub const APLIC_xMSICFGADDRH_BAPPN_MASK: c_uint = 0xfff;
pub const APLIC_xMSICFGADDRH_BAPPN_SHIFT: c_int = 0;

pub const APLIC_xMSICFGADDR_PPN_SHIFT: c_int = 12;

pub const APLIC_IRQBITS_PER_REG: c_int = 32;
pub const APLIC_SETIP_BASE: c_uint = 0x1c00;
pub const APLIC_SETIPNUM: c_uint = 0x1cdc;
pub const APLIC_CLRIP_BASE: c_uint = 0x1d00;
pub const APLIC_CLRIPNUM: c_uint = 0x1ddc;
pub const APLIC_SETIE_BASE: c_uint = 0x1e00;
pub const APLIC_SETIENUM: c_uint = 0x1edc;
pub const APLIC_CLRIE_BASE: c_uint = 0x1f00;
pub const APLIC_CLRIENUM: c_uint = 0x1fdc;
pub const APLIC_SETIPNUM_LE: c_uint = 0x2000;
pub const APLIC_SETIPNUM_BE: c_uint = 0x2004;
pub const APLIC_GENMSI: c_uint = 0x3000;
pub const APLIC_TARGET_BASE: c_uint = 0x3004;
pub const APLIC_TARGET_HART_IDX_SHIFT: c_int = 18;
pub const APLIC_TARGET_HART_IDX_MASK: c_uint = 0x3fff;

pub const APLIC_TARGET_GUEST_IDX_SHIFT: c_int = 12;
pub const APLIC_TARGET_GUEST_IDX_MASK: c_uint = 0x3f;

pub const APLIC_TARGET_IPRIO_SHIFT: c_int = 0;
pub const APLIC_TARGET_IPRIO_MASK: c_uint = 0xff;

pub const APLIC_TARGET_EIID_SHIFT: c_int = 0;
pub const APLIC_TARGET_EIID_MASK: c_uint = 0x7ff;

pub const APLIC_IDC_BASE: c_uint = 0x4000;
pub const APLIC_IDC_SIZE: c_int = 32;
pub const APLIC_IDC_IDELIVERY: c_uint = 0x00;
pub const APLIC_IDC_IFORCE: c_uint = 0x04;
pub const APLIC_IDC_ITHRESHOLD: c_uint = 0x08;
pub const APLIC_IDC_TOPI: c_uint = 0x18;
pub const APLIC_IDC_TOPI_ID_SHIFT: c_int = 16;
pub const APLIC_IDC_TOPI_ID_MASK: c_uint = 0x3ff;

pub const APLIC_IDC_TOPI_PRIO_SHIFT: c_int = 0;
pub const APLIC_IDC_TOPI_PRIO_MASK: c_uint = 0xff;

pub const APLIC_IDC_CLAIMI: c_uint = 0x1c;
