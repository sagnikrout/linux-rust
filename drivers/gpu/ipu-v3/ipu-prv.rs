//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/ipu-v3/ipu-prv.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Copyright (c) 2010 Sascha Hauer <s.hauer@pengutronix.de>
// Copyright (C) 2005-2009 Freescale Semiconductor, Inc.
//

pub const IPU_MCU_T_DEFAULT: c_int = 8;
pub const IPU_CM_IDMAC_REG_OFS: c_uint = 0x00008000;
pub const IPU_CM_IC_REG_OFS: c_uint = 0x00020000;
pub const IPU_CM_IRT_REG_OFS: c_uint = 0x00028000;
pub const IPU_CM_CSI0_REG_OFS: c_uint = 0x00030000;
pub const IPU_CM_CSI1_REG_OFS: c_uint = 0x00038000;
pub const IPU_CM_SMFC_REG_OFS: c_uint = 0x00050000;
pub const IPU_CM_DC_REG_OFS: c_uint = 0x00058000;
pub const IPU_CM_DMFC_REG_OFS: c_uint = 0x00060000;
// Register addresses
// IPU Common registers

// SRM_PRI2

// FS_PROC_FLOW1

// FS_PROC_FLOW2

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ipu_modules {
    IPU_CONF_CSI0_EN		= (1 << 0),
    IPU_CONF_CSI1_EN		= (1 << 1),
    IPU_CONF_IC_EN			= (1 << 2),
    IPU_CONF_ROT_EN			= (1 << 3),
    IPU_CONF_ISP_EN			= (1 << 4),
    IPU_CONF_DP_EN			= (1 << 5),
    IPU_CONF_DI0_EN			= (1 << 6),
    IPU_CONF_DI1_EN			= (1 << 7),
    IPU_CONF_SMFC_EN		= (1 << 8),
    IPU_CONF_DC_EN			= (1 << 9),
    IPU_CONF_DMFC_EN		= (1 << 10),

    IPU_CONF_VDI_EN			= (1 << 12),

    IPU_CONF_IDMAC_DIS		= (1 << 22),

    IPU_CONF_IC_DMFC_SEL		= (1 << 25),
    IPU_CONF_IC_DMFC_SYNC		= (1 << 26),
    IPU_CONF_VDI_DMFC_SYNC		= (1 << 27),

    IPU_CONF_CSI0_DATA_SOURCE	= (1 << 28),
    IPU_CONF_CSI1_DATA_SOURCE	= (1 << 29),
    IPU_CONF_IC_INPUT		= (1 << 30),
    IPU_CONF_CSI_SEL		= (1 << 31),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipuv3_channel {
    pub num: c_uint,
    pub ipu: *mut ipu_soc,
    pub list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipu_soc {
    pub dev: *mut device,
    pub devtype: *const ipu_devtype,
    pub ipu_type: ipuv3_type,
    pub lock: spinlock_t,
    pub channel_lock: mutex,
    pub channels: list_head,
    pub cm_reg: *mut void __iomem,
    pub idmac_reg: *mut void __iomem,
    pub id: c_int,
    pub usecount: c_int,
    pub clk: *mut clk,
    pub irq_sync: c_int,
    pub irq_err: c_int,
    pub domain: *mut irq_domain,
    pub cpmem_priv: *mut ipu_cpmem,
    pub dc_priv: *mut ipu_dc_priv,
    pub dp_priv: *mut ipu_dp_priv,
    pub dmfc_priv: *mut ipu_dmfc_priv,
    pub di_priv: [*mut ipu_di; 2],
    pub csi_priv: [*mut ipu_csi; 2],
    pub ic_priv: *mut ipu_ic_priv,
    pub vdi_priv: *mut ipu_vdi,
    pub image_convert_priv: *mut ipu_image_convert_priv,
    pub smfc_priv: *mut ipu_smfc_priv,
    pub prg_priv: *mut ipu_prg,
}

extern "C" {
    pub fn readl(offset: ipu->idmac_reg +) -> return;
}
extern "C" {
    pub fn ipu_srm_dp_update(ipu: *mut ipu_soc, sync: bool);
}
extern "C" {
    pub fn ipu_module_enable(ipu: *mut ipu_soc, mask: u32) -> c_int;
}
extern "C" {
    pub fn ipu_module_disable(ipu: *mut ipu_soc, mask: u32) -> c_int;
}
extern "C" {
    pub fn ipu_csi_exit(ipu: *mut ipu_soc, id: c_int);
}
extern "C" {
    pub fn ipu_ic_exit(ipu: *mut ipu_soc);
}
extern "C" {
    pub fn ipu_vdi_exit(ipu: *mut ipu_soc);
}
extern "C" {
    pub fn ipu_image_convert_init(ipu: *mut ipu_soc, dev: *mut device) -> c_int;
}
extern "C" {
    pub fn ipu_image_convert_exit(ipu: *mut ipu_soc);
}
extern "C" {
    pub fn ipu_di_exit(ipu: *mut ipu_soc, id: c_int);
}
extern "C" {
    pub fn ipu_dmfc_exit(ipu: *mut ipu_soc);
}
extern "C" {
    pub fn ipu_dp_init(ipu: *mut ipu_soc, dev: *mut device, base: c_ulong) -> c_int;
}
extern "C" {
    pub fn ipu_dp_exit(ipu: *mut ipu_soc);
}
extern "C" {
    pub fn ipu_dc_exit(ipu: *mut ipu_soc);
}
extern "C" {
    pub fn ipu_cpmem_init(ipu: *mut ipu_soc, dev: *mut device, base: c_ulong) -> c_int;
}
extern "C" {
    pub fn ipu_cpmem_exit(ipu: *mut ipu_soc);
}
extern "C" {
    pub fn ipu_smfc_init(ipu: *mut ipu_soc, dev: *mut device, base: c_ulong) -> c_int;
}
extern "C" {
    pub fn ipu_smfc_exit(ipu: *mut ipu_soc);
}
extern "C" {
    pub fn ipu_pre_get_available_count() -> c_int;
}
extern "C" {
    pub fn ipu_pre_get(pre: *mut ipu_pre) -> c_int;
}
extern "C" {
    pub fn ipu_pre_put(pre: *mut ipu_pre);
}
extern "C" {
    pub fn ipu_pre_get_baddr(pre: *mut ipu_pre) -> u32;
}
extern "C" {
    pub fn ipu_pre_update(pre: *mut ipu_pre, modifier: u64, bufaddr: c_uint);
}
extern "C" {
    pub fn ipu_pre_update_pending(pre: *mut ipu_pre) -> bool;
}
