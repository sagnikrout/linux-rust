//! Automatically rewritten from C Header to Rust Module
//! Source: sound/pci/oxygen/cs2000.h
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

// Macro flag: #define CS2000_H_INCLUDED
pub const CS2000_DEV_ID: c_uint = 0x01;
pub const CS2000_DEV_CTRL: c_uint = 0x02;
pub const CS2000_DEV_CFG_1: c_uint = 0x03;
pub const CS2000_DEV_CFG_2: c_uint = 0x04;
pub const CS2000_GLOBAL_CFG: c_uint = 0x05;
pub const CS2000_RATIO_0: c_uint = 0x06 /* 32 bits, big endian */;
pub const CS2000_RATIO_1: c_uint = 0x0a;
pub const CS2000_RATIO_2: c_uint = 0x0e;
pub const CS2000_RATIO_3: c_uint = 0x12;
pub const CS2000_FUN_CFG_1: c_uint = 0x16;
pub const CS2000_FUN_CFG_2: c_uint = 0x17;
pub const CS2000_FUN_CFG_3: c_uint = 0x1e;
// DEV_ID
pub const CS2000_DEVICE_MASK: c_uint = 0xf8;
pub const CS2000_REVISION_MASK: c_uint = 0x07;
// DEV_CTRL
pub const CS2000_UNLOCK: c_uint = 0x80;
pub const CS2000_AUX_OUT_DIS: c_uint = 0x02;
pub const CS2000_CLK_OUT_DIS: c_uint = 0x01;
// DEV_CFG_1
pub const CS2000_R_MOD_SEL_MASK: c_uint = 0xe0;
pub const CS2000_R_MOD_SEL_1: c_uint = 0x00;
pub const CS2000_R_MOD_SEL_2: c_uint = 0x20;
pub const CS2000_R_MOD_SEL_4: c_uint = 0x40;
pub const CS2000_R_MOD_SEL_8: c_uint = 0x60;
pub const CS2000_R_MOD_SEL_1_2: c_uint = 0x80;
pub const CS2000_R_MOD_SEL_1_4: c_uint = 0xa0;
pub const CS2000_R_MOD_SEL_1_8: c_uint = 0xc0;
pub const CS2000_R_MOD_SEL_1_16: c_uint = 0xe0;
pub const CS2000_R_SEL_MASK: c_uint = 0x18;
pub const CS2000_R_SEL_SHIFT: c_int = 3;
pub const CS2000_AUX_OUT_SRC_MASK: c_uint = 0x06;
pub const CS2000_AUX_OUT_SRC_REF_CLK: c_uint = 0x00;
pub const CS2000_AUX_OUT_SRC_CLK_IN: c_uint = 0x02;
pub const CS2000_AUX_OUT_SRC_CLK_OUT: c_uint = 0x04;
pub const CS2000_AUX_OUT_SRC_PLL_LOCK: c_uint = 0x06;
pub const CS2000_EN_DEV_CFG_1: c_uint = 0x01;
// DEV_CFG_2
pub const CS2000_LOCK_CLK_MASK: c_uint = 0x06;
pub const CS2000_LOCK_CLK_SHIFT: c_int = 1;
pub const CS2000_FRAC_N_SRC_MASK: c_uint = 0x01;
pub const CS2000_FRAC_N_SRC_STATIC: c_uint = 0x00;
pub const CS2000_FRAC_N_SRC_DYNAMIC: c_uint = 0x01;
// GLOBAL_CFG
pub const CS2000_FREEZE: c_uint = 0x08;
pub const CS2000_EN_DEV_CFG_2: c_uint = 0x01;
// FUN_CFG_1
pub const CS2000_CLK_SKIP_EN: c_uint = 0x80;
pub const CS2000_AUX_LOCK_CFG_MASK: c_uint = 0x40;
pub const CS2000_AUX_LOCK_CFG_PP_HIGH: c_uint = 0x00;
pub const CS2000_AUX_LOCK_CFG_OD_LOW: c_uint = 0x40;
pub const CS2000_REF_CLK_DIV_MASK: c_uint = 0x18;
pub const CS2000_REF_CLK_DIV_4: c_uint = 0x00;
pub const CS2000_REF_CLK_DIV_2: c_uint = 0x08;
pub const CS2000_REF_CLK_DIV_1: c_uint = 0x10;
// FUN_CFG_2
pub const CS2000_CLK_OUT_UNL: c_uint = 0x10;
pub const CS2000_L_F_RATIO_CFG_MASK: c_uint = 0x08;
pub const CS2000_L_F_RATIO_CFG_20_12: c_uint = 0x00;
pub const CS2000_L_F_RATIO_CFG_12_20: c_uint = 0x08;
// FUN_CFG_3
pub const CS2000_CLK_IN_BW_MASK: c_uint = 0x70;
pub const CS2000_CLK_IN_BW_1: c_uint = 0x00;
pub const CS2000_CLK_IN_BW_2: c_uint = 0x10;
pub const CS2000_CLK_IN_BW_4: c_uint = 0x20;
pub const CS2000_CLK_IN_BW_8: c_uint = 0x30;
pub const CS2000_CLK_IN_BW_16: c_uint = 0x40;
pub const CS2000_CLK_IN_BW_32: c_uint = 0x50;
pub const CS2000_CLK_IN_BW_64: c_uint = 0x60;
pub const CS2000_CLK_IN_BW_128: c_uint = 0x70;
