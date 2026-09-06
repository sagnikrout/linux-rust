//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/display/bxt_dpio_phy_regs.h
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
// Copyright © 2023 Intel Corporation
//

// BXT PHY registers
pub const _BXT_PHY0_BASE: c_uint = 0x6C000;
pub const _BXT_PHY1_BASE: c_uint = 0x162000;
pub const _BXT_PHY2_BASE: c_uint = 0x163000;

// BXT PHY PLL registers
pub const _PORT_PLL_A: c_uint = 0x46074;
pub const _PORT_PLL_B: c_uint = 0x46078;
pub const _PORT_PLL_C: c_uint = 0x4607c;

pub const _PORT_PLL_EBB_0_A: c_uint = 0x162034;
pub const _PORT_PLL_EBB_0_B: c_uint = 0x6C034;
pub const _PORT_PLL_EBB_0_C: c_uint = 0x6C340;

pub const _PORT_PLL_EBB_4_A: c_uint = 0x162038;
pub const _PORT_PLL_EBB_4_B: c_uint = 0x6C038;
pub const _PORT_PLL_EBB_4_C: c_uint = 0x6C344;

pub const _PORT_PLL_0_A: c_uint = 0x162100;
pub const _PORT_PLL_0_B: c_uint = 0x6C100;
pub const _PORT_PLL_0_C: c_uint = 0x6C380;
// PORT_PLL_0_A

// PORT_PLL_1_A

// PORT_PLL_2_A

// PORT_PLL_3_A

// PORT_PLL_6_A

// PORT_PLL_8_A

// PORT_PLL_9_A

// PORT_PLL_10_A

// BXT PHY common lane registers
pub const _PORT_CL1CM_DW0_A: c_uint = 0x162000;
pub const _PORT_CL1CM_DW0_BC: c_uint = 0x6C000;

pub const _PORT_CL1CM_DW9_A: c_uint = 0x162024;
pub const _PORT_CL1CM_DW9_BC: c_uint = 0x6C024;

pub const _PORT_CL1CM_DW10_A: c_uint = 0x162028;
pub const _PORT_CL1CM_DW10_BC: c_uint = 0x6C028;

pub const _PORT_CL1CM_DW28_A: c_uint = 0x162070;
pub const _PORT_CL1CM_DW28_BC: c_uint = 0x6C070;

pub const _PORT_CL1CM_DW30_A: c_uint = 0x162078;
pub const _PORT_CL1CM_DW30_BC: c_uint = 0x6C078;

// The spec defines this only for BXT PHY0, but lets assume that this
// would exist for PHY1 too if it had a second channel.
//
pub const _PORT_CL2CM_DW6_A: c_uint = 0x162358;
pub const _PORT_CL2CM_DW6_BC: c_uint = 0x6C358;

// BXT PHY Ref registers
pub const _PORT_REF_DW3_A: c_uint = 0x16218C;
pub const _PORT_REF_DW3_BC: c_uint = 0x6C18C;

pub const _PORT_REF_DW6_A: c_uint = 0x162198;
pub const _PORT_REF_DW6_BC: c_uint = 0x6C198;

pub const _PORT_REF_DW8_A: c_uint = 0x1621A0;
pub const _PORT_REF_DW8_BC: c_uint = 0x6C1A0;

// BXT PHY PCS registers
pub const _PORT_PCS_DW10_LN01_A: c_uint = 0x162428;
pub const _PORT_PCS_DW10_LN01_B: c_uint = 0x6C428;
pub const _PORT_PCS_DW10_LN01_C: c_uint = 0x6C828;
pub const _PORT_PCS_DW10_GRP_A: c_uint = 0x162C28;
pub const _PORT_PCS_DW10_GRP_B: c_uint = 0x6CC28;
pub const _PORT_PCS_DW10_GRP_C: c_uint = 0x6CE28;

pub const _PORT_PCS_DW12_LN01_A: c_uint = 0x162430;
pub const _PORT_PCS_DW12_LN01_B: c_uint = 0x6C430;
pub const _PORT_PCS_DW12_LN01_C: c_uint = 0x6C830;
pub const _PORT_PCS_DW12_LN23_A: c_uint = 0x162630;
pub const _PORT_PCS_DW12_LN23_B: c_uint = 0x6C630;
pub const _PORT_PCS_DW12_LN23_C: c_uint = 0x6CA30;
pub const _PORT_PCS_DW12_GRP_A: c_uint = 0x162c30;
pub const _PORT_PCS_DW12_GRP_B: c_uint = 0x6CC30;
pub const _PORT_PCS_DW12_GRP_C: c_uint = 0x6CE30;

// BXT PHY TX registers
pub const _PORT_TX_DW2_LN0_A: c_uint = 0x162508;
pub const _PORT_TX_DW2_LN0_B: c_uint = 0x6C508;
pub const _PORT_TX_DW2_LN0_C: c_uint = 0x6C908;
pub const _PORT_TX_DW2_GRP_A: c_uint = 0x162D08;
pub const _PORT_TX_DW2_GRP_B: c_uint = 0x6CD08;
pub const _PORT_TX_DW2_GRP_C: c_uint = 0x6CF08;

pub const _PORT_TX_DW3_LN0_A: c_uint = 0x16250C;
pub const _PORT_TX_DW3_LN0_B: c_uint = 0x6C50C;
pub const _PORT_TX_DW3_LN0_C: c_uint = 0x6C90C;
pub const _PORT_TX_DW3_GRP_A: c_uint = 0x162D0C;
pub const _PORT_TX_DW3_GRP_B: c_uint = 0x6CD0C;
pub const _PORT_TX_DW3_GRP_C: c_uint = 0x6CF0C;

pub const _PORT_TX_DW4_LN0_A: c_uint = 0x162510;
pub const _PORT_TX_DW4_LN0_B: c_uint = 0x6C510;
pub const _PORT_TX_DW4_LN0_C: c_uint = 0x6C910;
pub const _PORT_TX_DW4_GRP_A: c_uint = 0x162D10;
pub const _PORT_TX_DW4_GRP_B: c_uint = 0x6CD10;
pub const _PORT_TX_DW4_GRP_C: c_uint = 0x6CF10;

pub const _PORT_TX_DW5_LN0_A: c_uint = 0x162514;
pub const _PORT_TX_DW5_LN0_B: c_uint = 0x6C514;
pub const _PORT_TX_DW5_LN0_C: c_uint = 0x6C914;
pub const _PORT_TX_DW5_GRP_A: c_uint = 0x162D14;
pub const _PORT_TX_DW5_GRP_B: c_uint = 0x6CD14;
pub const _PORT_TX_DW5_GRP_C: c_uint = 0x6CF14;

pub const _PORT_TX_DW14_LN0_A: c_uint = 0x162538;
pub const _PORT_TX_DW14_LN0_B: c_uint = 0x6C538;
pub const _PORT_TX_DW14_LN0_C: c_uint = 0x6C938;

