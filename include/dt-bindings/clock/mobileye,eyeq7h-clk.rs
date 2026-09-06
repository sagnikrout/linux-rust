//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/mobileye,eyeq7h-clk.h
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


// SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause)
//
// Copyright (C) 2025 Mobileye Vision Technologies Ltd.
//
// ACC0 and ACC1 OLBs PLL and dividers
pub const EQ7HC_ACC_PLL_VMP: c_int = 0;
pub const EQ7HC_ACC_PLL_MPC: c_int = 1;
pub const EQ7HC_ACC_PLL_PMA: c_int = 2;
pub const EQ7HC_ACC_PLL_NOC: c_int = 3;
pub const EQ7HC_ACC_DIV_PMA: c_int = 4;
pub const EQ7HC_ACC_DIV_NCORE: c_int = 5;
pub const EQ7HC_ACC_DIV_CFG: c_int = 6;
// DDR0 and DDR1 OLBs PLL and dividers
pub const EQ7HC_DDR_PLL: c_int = 0;
pub const EQ7HC_DDR_DIV_APB: c_int = 1;
pub const EQ7HC_DDR_DIV_PLLREF: c_int = 2;
pub const EQ7HC_DDR_DIV_DFI: c_int = 3;
// east OLB PLL and dividers
pub const EQ7HC_EAST_PLL_106P6: c_int = 0;
pub const EQ7HC_EAST_DIV_REF_106P6: c_int = 1;
pub const EQ7HC_EAST_PLL_NOC: c_int = 2;
pub const EQ7HC_EAST_PLL_ISP: c_int = 3;
pub const EQ7HC_EAST_PLL_VEU: c_int = 4;
pub const EQ7HC_EAST_DIV_REF_DDR_PHY: c_int = 5;
pub const EQ7HC_EAST_DIV_CORE: c_int = 6;
pub const EQ7HC_EAST_DIV_CORE_MBIST: c_int = 7;
pub const EQ7HC_EAST_DIV_ISRAM_MBIST: c_int = 8;
pub const EQ7HC_EAST_DIV_CFG: c_int = 9;
pub const EQ7HC_EAST_DIV_VEU_CORE: c_int = 10;
pub const EQ7HC_EAST_DIV_VEU_MBIST: c_int = 11;
pub const EQ7HC_EAST_DIV_VEU_OCP: c_int = 12;
pub const EQ7HC_EAST_DIV_LBITS: c_int = 13;
pub const EQ7HC_EAST_DIV_ISP0_CORE: c_int = 14;
// MIPS0, MIPS1 and MIPS2 OLBs PLL and dividers
pub const EQ7HC_MIPS_PLL_CPU: c_int = 0;
pub const EQ7HC_MIPS_DIV_CM: c_int = 1;
// periph east OLB PLL and dividers
pub const EQ7HC_PERIPH_EAST_PLL_PER: c_int = 0;
pub const EQ7HC_PERIPH_EAST_DIV_PER: c_int = 1;
// periph west OLB PLL and dividers
pub const EQ7HC_PERIPH_WEST_PLL_PER: c_int = 0;
pub const EQ7HC_PERIPH_WEST_PLL_I2S: c_int = 1;
pub const EQ7HC_PERIPH_WEST_DIV_PER: c_int = 2;
pub const EQ7HC_PERIPH_WEST_DIV_I2S: c_int = 3;
// south OLB PLL and dividers
pub const EQ7HC_SOUTH_PLL_100P0: c_int = 0;
pub const EQ7HC_SOUTH_DIV_REF_100P0: c_int = 1;
pub const EQ7HC_SOUTH_PLL_XSPI: c_int = 2;
pub const EQ7HC_SOUTH_PLL_VDIO: c_int = 3;
pub const EQ7HC_SOUTH_PLL_PER: c_int = 4;
pub const EQ7HC_SOUTH_DIV_VDO_DSI_SYS: c_int = 5;
pub const EQ7HC_SOUTH_DIV_PMA_CMN_REF: c_int = 6;
pub const EQ7HC_SOUTH_DIV_REF_UFS: c_int = 7;
pub const EQ7HC_SOUTH_DIV_XSPI_SYS: c_int = 8;
pub const EQ7HC_SOUTH_DIV_XSPI_MBIST: c_int = 9;
pub const EQ7HC_SOUTH_DIV_NOC_S: c_int = 10;
pub const EQ7HC_SOUTH_DIV_PCIE_SYS: c_int = 11;
pub const EQ7HC_SOUTH_DIV_PCIE_SYS_MBIST: c_int = 12;
pub const EQ7HC_SOUTH_DIV_PCIE_GBE_PHY: c_int = 13;
pub const EQ7HC_SOUTH_DIV_UFS_CORE: c_int = 14;
pub const EQ7HC_SOUTH_DIV_UFS_SMS: c_int = 15;
pub const EQ7HC_SOUTH_DIV_UFS_ROM_SMS: c_int = 16;
pub const EQ7HC_SOUTH_DIV_ETH_SYS: c_int = 17;
pub const EQ7HC_SOUTH_DIV_ETH_MBIST: c_int = 18;
pub const EQ7HC_SOUTH_DIV_CFG_S: c_int = 19;
pub const EQ7HC_SOUTH_DIV_TSU: c_int = 20;
pub const EQ7HC_SOUTH_DIV_VDIO: c_int = 21;
pub const EQ7HC_SOUTH_DIV_VDIO_CORE: c_int = 22;
pub const EQ7HC_SOUTH_DIV_VDIO_CORE_MBIST: c_int = 23;
pub const EQ7HC_SOUTH_DIV_VDO_CORE_MBIST: c_int = 24;
pub const EQ7HC_SOUTH_DIV_VDO_P: c_int = 25;
pub const EQ7HC_SOUTH_DIV_VDIO_CFG: c_int = 26;
pub const EQ7HC_SOUTH_DIV_VDIO_TXCLKESC: c_int = 27;
// west OLB PLL and dividers
pub const EQ7HC_WEST_PLL_106P6: c_int = 0;
pub const EQ7HC_WEST_DIV_REF_106P6: c_int = 1;
pub const EQ7HC_WEST_PLL_NOC: c_int = 2;
pub const EQ7HC_WEST_PLL_GPU: c_int = 3;
pub const EQ7HC_WEST_PLL_SSI: c_int = 4;
pub const EQ7HC_WEST_DIV_GPU: c_int = 5;
pub const EQ7HC_WEST_DIV_GPU_MBIST: c_int = 6;
pub const EQ7HC_WEST_DIV_LBITS: c_int = 7;
pub const EQ7HC_WEST_DIV_MIPS_TIMER: c_int = 8;
pub const EQ7HC_WEST_DIV_SSI_CORE: c_int = 9;
pub const EQ7HC_WEST_DIV_SSI_CORE_MBIST: c_int = 10;
pub const EQ7HC_WEST_DIV_SSI_ROM: c_int = 11;
pub const EQ7HC_WEST_DIV_SSI_ROM_MBIST: c_int = 12;
pub const EQ7HC_WEST_DIV_REF_DDR_PHY: c_int = 13;
pub const EQ7HC_WEST_DIV_CORE: c_int = 14;
pub const EQ7HC_WEST_DIV_CORE_MBIST: c_int = 15;
pub const EQ7HC_WEST_DIV_CFG: c_int = 16;
pub const EQ7HC_WEST_DIV_CAU: c_int = 17;
pub const EQ7HC_WEST_DIV_CAU_MBIST: c_int = 18;
// XNN0 and XNN1 OLBs PLL and dividers
pub const EQ7HC_XNN_PLL_XNN0: c_int = 0;
pub const EQ7HC_XNN_PLL_XNN1: c_int = 1;
pub const EQ7HC_XNN_PLL_XNN2: c_int = 2;
pub const EQ7HC_XNN_PLL_CLSTR: c_int = 3;
pub const EQ7HC_XNN_DIV_XNN0: c_int = 4;
pub const EQ7HC_XNN_DIV_XNN1: c_int = 5;
pub const EQ7HC_XNN_DIV_XNN2: c_int = 6;
pub const EQ7HC_XNN_DIV_CLSTR: c_int = 7;
pub const EQ7HC_XNN_DIV_I2: c_int = 8;
pub const EQ7HC_XNN_DIV_I2_SMS: c_int = 9;
pub const EQ7HC_XNN_DIV_CFG: c_int = 10;
