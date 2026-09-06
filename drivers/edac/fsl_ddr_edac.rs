//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/edac/fsl_ddr_edac.h
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
// Freescale Memory Controller kernel module
//
// Support  Power-based SoCs including MPC85xx, MPC86xx, MPC83xx and
// ARM-based Layerscape SoCs including LS2xxx and LS1021A. Originally
// split out from mpc85xx_edac EDAC driver.
//
// Author: Dave Jiang <djiang@mvista.com>
//
// 2006-2007 (c) MontaVista Software, Inc.
//

//
// DRAM error defines
//
// DDR_SDRAM_CFG
pub const FSL_MC_DDR_SDRAM_CFG: c_uint = 0x0110;
pub const FSL_MC_CS_BNDS_0: c_uint = 0x0000;
pub const FSL_MC_CS_BNDS_OFS: c_uint = 0x0008;
pub const FSL_MC_DATA_ERR_INJECT_HI: c_uint = 0x0e00;
pub const FSL_MC_DATA_ERR_INJECT_LO: c_uint = 0x0e04;
pub const FSL_MC_ECC_ERR_INJECT: c_uint = 0x0e08;
pub const FSL_MC_CAPTURE_DATA_HI: c_uint = 0x0e20;
pub const FSL_MC_CAPTURE_DATA_LO: c_uint = 0x0e24;
pub const FSL_MC_CAPTURE_ECC: c_uint = 0x0e28;
pub const FSL_MC_ERR_DETECT: c_uint = 0x0e40;
pub const FSL_MC_ERR_DISABLE: c_uint = 0x0e44;
pub const FSL_MC_ERR_INT_EN: c_uint = 0x0e48;
pub const FSL_MC_CAPTURE_ATRIBUTES: c_uint = 0x0e4c;
pub const FSL_MC_CAPTURE_ADDRESS: c_uint = 0x0e50;
pub const FSL_MC_CAPTURE_EXT_ADDRESS: c_uint = 0x0e54;
pub const FSL_MC_ERR_SBE: c_uint = 0x0e58;
pub const IMX9_MC_ERR_EN: c_uint = 0x1000;
pub const IMX9_MC_DATA_ERR_INJECT_OFF: c_uint = 0x100;
pub const DSC_MEM_EN: c_uint = 0x80000000;
pub const DSC_ECC_EN: c_uint = 0x20000000;
pub const DSC_RD_EN: c_uint = 0x10000000;
pub const DSC_DBW_MASK: c_uint = 0x00180000;
pub const DSC_DBW_32: c_uint = 0x00080000;
pub const DSC_DBW_64: c_uint = 0x00000000;
pub const ERR_ECC_EN: c_uint = 0x80000000;
pub const ERR_INLINE_ECC: c_uint = 0x40000000;
pub const DSC_SDTYPE_MASK: c_uint = 0x07000000;
pub const DSC_X32_EN: c_uint = 0x00000020;
// Err_Int_En
pub const DDR_EIE_MSEE: c_uint = 0x1	/* memory select */;
pub const DDR_EIE_SBEE: c_uint = 0x4	/* single-bit ECC error */;
pub const DDR_EIE_MBEE: c_uint = 0x8	/* multi-bit ECC error */;
// Err_Detect
pub const DDR_EDE_MSE: c_uint = 0x1	/* memory select */;
pub const DDR_EDE_SBE: c_uint = 0x4	/* single-bit ECC error */;
pub const DDR_EDE_MBE: c_uint = 0x8	/* multi-bit ECC error */;
pub const DDR_EDE_MME: c_uint = 0x80000000	/* multiple memory errors */;
// Err_Disable
pub const DDR_EDI_MSED: c_uint = 0x1	/* memory select disable */;
pub const DDR_EDI_SBED: c_uint = 0x4	/* single-bit ECC error disable */;
pub const DDR_EDI_MBED: c_uint = 0x8	/* multi-bit ECC error disable */;
pub const TYPE_IMX9: c_uint = 0x1	/* MC used by iMX9 having registers changed */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsl_mc_pdata {
    pub name: *mut c_char,
    pub edac_idx: c_int,
    pub mc_vbase: *mut void __iomem,
    pub inject_vbase: *mut void __iomem,
    pub irq: c_int,
    pub orig_ddr_err_disable: u32,
    pub orig_ddr_err_sbe: u32,
    pub little_endian: bool,
    pub flag: c_ulong,
}

extern "C" {
    pub fn fsl_mc_err_probe(op: *mut platform_device) -> c_int;
}
extern "C" {
    pub fn fsl_mc_err_remove(op: *mut platform_device);
}
