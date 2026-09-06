//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/ti/omap3isp/isp.h
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
// isp.h
//
// TI OMAP3 ISP - Core
//
// Copyright (C) 2009-2010 Nokia Corporation
// Copyright (C) 2009 Texas Instruments, Inc.
//
// Contacts: Laurent Pinchart <laurent.pinchart@ideasonboard.com>
// Sakari Ailus <sakari.ailus@iki.fi>
//

pub const ISP_TOK_TERM: c_uint = 0xFFFFFFFF	/*;
// terminating token for ISP
// modules reg list
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum isp_mem_resources {
    OMAP3_ISP_IOMEM_MAIN,
    OMAP3_ISP_IOMEM_CCP2,
    OMAP3_ISP_IOMEM_CCDC,
    OMAP3_ISP_IOMEM_HIST,
    OMAP3_ISP_IOMEM_H3A,
    OMAP3_ISP_IOMEM_PREV,
    OMAP3_ISP_IOMEM_RESZ,
    OMAP3_ISP_IOMEM_SBL,
    OMAP3_ISP_IOMEM_CSI2A_REGS1,
    OMAP3_ISP_IOMEM_CSIPHY2,
    OMAP3_ISP_IOMEM_CSI2A_REGS2,
    OMAP3_ISP_IOMEM_CSI2C_REGS1,
    OMAP3_ISP_IOMEM_CSIPHY1,
    OMAP3_ISP_IOMEM_CSI2C_REGS2,
    OMAP3_ISP_IOMEM_LAST
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum isp_sbl_resource {
    OMAP3_ISP_SBL_CSI1_READ		= 0x1,
    OMAP3_ISP_SBL_CSI1_WRITE	= 0x2,
    OMAP3_ISP_SBL_CSI2A_WRITE	= 0x4,
    OMAP3_ISP_SBL_CSI2C_WRITE	= 0x8,
    OMAP3_ISP_SBL_CCDC_LSC_READ	= 0x10,
    OMAP3_ISP_SBL_CCDC_WRITE	= 0x20,
    OMAP3_ISP_SBL_PREVIEW_READ	= 0x40,
    OMAP3_ISP_SBL_PREVIEW_WRITE	= 0x80,
    OMAP3_ISP_SBL_RESIZER_READ	= 0x100,
    OMAP3_ISP_SBL_RESIZER_WRITE	= 0x200,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum isp_subclk_resource {
    OMAP3_ISP_SUBCLK_CCDC		= (1 << 0),
    OMAP3_ISP_SUBCLK_AEWB		= (1 << 1),
    OMAP3_ISP_SUBCLK_AF		= (1 << 2),
    OMAP3_ISP_SUBCLK_HIST		= (1 << 3),
    OMAP3_ISP_SUBCLK_PREVIEW	= (1 << 4),
    OMAP3_ISP_SUBCLK_RESIZER	= (1 << 5),
}

// ISP: OMAP 34xx ES 1.0
pub const ISP_REVISION_1_0: c_uint = 0x10;
// ISP2: OMAP 34xx ES 2.0, 2.1 and 3.0
pub const ISP_REVISION_2_0: c_uint = 0x20;
// ISP2P: OMAP 36xx
pub const ISP_REVISION_15_0: c_uint = 0xF0;
pub const ISP_PHY_TYPE_3430: c_int = 0;
pub const ISP_PHY_TYPE_3630: c_int = 1;
//
// struct isp_res_mapping - Map ISP io resources to ISP revision.
// @isp_rev: ISP_REVISION_x_x
// @offset: register offsets of various ISP sub-blocks
// @phy_type: ISP_PHY_TYPE_{3430,3630}
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct isp_res_mapping {
    pub isp_rev: u32,
    pub offset: [u32; OMAP3_ISP_IOMEM_LAST],
    pub phy_type: u32,
}

//
// struct isp_reg - Structure for ISP register values.
// @reg: 32-bit Register address.
// @val: 32-bit Register value.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct isp_reg {
    pub mmio_range: isp_mem_resources,
    pub reg: u32,
    pub val: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum isp_xclk_id {
    ISP_XCLK_A,
    ISP_XCLK_B,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct isp_xclk {
    pub isp: *mut isp_device,
    pub hw: clk_hw,
    pub clk: *mut clk,
    pub id: isp_xclk_id,
    pub /: *mut *mut spinlock_t lock; / Protects enabled and divider,
    pub enabled: bool,
    pub divider: c_uint,
}

//
// struct isp_device - ISP device structure.
// @dev: Device pointer specific to the OMAP3 ISP.
// @revision: Stores current ISP module revision.
// @irq_num: Currently used IRQ number.
// @mmio_base: Array with kernel base addresses for ioremapped ISP register
// regions.
// @mmio_hist_base_phys: Physical L4 bus address for ISP hist block register
// region.
// @syscon: Regmap for the syscon register space
// @syscon_offset: Offset of the CSIPHY control register in syscon
// @phy_type: ISP_PHY_TYPE_{3430,3630}
// @mapping: IOMMU mapping
// @stat_lock: Spinlock for handling statistics
// @isp_mutex: Mutex for serializing requests to ISP.
// @stop_failure: Indicates that an entity failed to stop.
// @crashed: Crashed ent_enum
// @has_context: Context has been saved at least once and can be restored.
// @ref_count: Reference count for handling multiple ISP requests.
// @cam_ick: Pointer to camera interface clock structure.
// @cam_mclk: Pointer to camera functional clock structure.
// @csi2_fck: Pointer to camera CSI2 complexIO clock structure.
// @l3_ick: Pointer to OMAP3 L3 bus interface clock.
// @xclks: External clocks provided by the ISP
// @irq: Currently attached ISP ISR callbacks information structure.
// @isp_af: Pointer to current settings for ISP AutoFocus SCM.
// @isp_hist: Pointer to current settings for ISP Histogram SCM.
// @isp_h3a: Pointer to current settings for ISP Auto Exposure and
// White Balance SCM.
// @isp_res: Pointer to current settings for ISP Resizer.
// @isp_prev: Pointer to current settings for ISP Preview.
// @isp_ccdc: Pointer to current settings for ISP CCDC.
// @platform_cb: ISP driver callback function pointers for platform code
//
// This structure is used to store the OMAP ISP Information.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct isp_device {
    pub v4l2_dev: v4l2_device,
    pub notifier: v4l2_async_notifier,
    pub media_dev: media_device,
    pub dev: *mut device,
    pub revision: u32,
// platform HW resources
    pub irq_num: c_uint,
    pub mmio_base: [*mut void __iomem; OMAP3_ISP_IOMEM_LAST],
    pub mmio_hist_base_phys: c_ulong,
    pub syscon: *mut regmap,
    pub syscon_offset: u32,
    pub phy_type: u32,
    pub mapping: *mut dma_iommu_mapping,
// ISP Obj
    pub /: *mut *mut spinlock_t stat_lock; / common lock for statistic drivers,
    pub /: *mut *mut mutex isp_mutex; / For handling ref_count field,
    pub stop_failure: bool,
    pub crashed: media_entity_enum,
    pub has_context: c_int,
    pub ref_count: c_int,
    pub autoidle: c_uint,
pub const ISP_CLK_CAM_ICK: c_int = 0;
pub const ISP_CLK_CAM_MCLK: c_int = 1;
pub const ISP_CLK_CSI2_FCK: c_int = 2;
pub const ISP_CLK_L3_ICK: c_int = 3;
    pub clock: [*mut clk; 4],
    pub xclks: [isp_xclk; 2],
// ISP modules
    pub isp_af: ispstat,
    pub isp_aewb: ispstat,
    pub isp_hist: ispstat,
    pub isp_res: isp_res_device,
    pub isp_prev: isp_prev_device,
    pub isp_ccdc: isp_ccdc_device,
    pub isp_csi2a: isp_csi2_device,
    pub isp_csi2c: isp_csi2_device,
    pub isp_ccp2: isp_ccp2_device,
    pub isp_csiphy1: isp_csiphy,
    pub isp_csiphy2: isp_csiphy,
    pub sbl_resources: c_uint,
    pub subclk_resources: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct isp_async_subdev {
    pub asd: v4l2_async_connection,
    pub bus: isp_bus_cfg,
}

extern "C" {
    pub fn omap3isp_hist_dma_done(isp: *mut isp_device);
}
extern "C" {
    pub fn omap3isp_flush(isp: *mut isp_device);
}
extern "C" {
    pub fn omap3isp_pipeline_cancel_stream(pipe: *mut isp_pipeline);
}
extern "C" {
    pub fn omap3isp_put(isp: *mut isp_device);
}
extern "C" {
    pub fn omap3isp_sbl_enable(isp: *mut isp_device, res: isp_sbl_resource);
}
extern "C" {
    pub fn omap3isp_sbl_disable(isp: *mut isp_device, res: isp_sbl_resource);
}
extern "C" {
    pub fn omap3isp_unregister_entities(pdev: *mut platform_device);
}
//
// isp_reg_readl - Read value of an OMAP3 ISP register
// @isp: Device pointer specific to the OMAP3 ISP.
// @isp_mmio_range: Range to which the register offset refers to.
// @reg_offset: Register offset to read from.
//
// Returns an unsigned 32 bit value with the required register contents.
//
extern "C" {
    pub fn __raw_readl(reg_offset: isp->mmio_base[isp_mmio_range] +) -> return;
}
//
// isp_reg_writel - Write value to an OMAP3 ISP register
// @isp: Device pointer specific to the OMAP3 ISP.
// @reg_value: 32 bit value to write to the register.
// @isp_mmio_range: Range to which the register offset refers to.
// @reg_offset: Register offset to write into.
//
// isp_reg_clr - Clear individual bits in an OMAP3 ISP register
// @isp: Device pointer specific to the OMAP3 ISP.
// @mmio_range: Range to which the register offset refers to.
// @reg: Register offset to work on.
// @clr_bits: 32 bit value which would be cleared in the register.
//
// isp_reg_set - Set individual bits in an OMAP3 ISP register
// @isp: Device pointer specific to the OMAP3 ISP.
// @mmio_range: Range to which the register offset refers to.
// @reg: Register offset to work on.
// @set_bits: 32 bit value which would be set in the register.
//
// isp_reg_clr_set - Clear and set invidial bits in an OMAP3 ISP register
// @isp: Device pointer specific to the OMAP3 ISP.
// @mmio_range: Range to which the register offset refers to.
// @reg: Register offset to work on.
// @clr_bits: 32 bit value which would be cleared in the register.
// @set_bits: 32 bit value which would be set in the register.
//
// The clear operation is done first, and then the set operation.
//
