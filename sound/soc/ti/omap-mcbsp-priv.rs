//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/ti/omap-mcbsp-priv.h
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
//
// OMAP Multi-Channel Buffered Serial Port
//
// Contact: Jarkko Nikula <jarkko.nikula@bitmer.com>
// Peter Ujfalusi <peter.ujfalusi@ti.com>
//

pub const mcbsp_omap1(): c_int = 1;

pub const mcbsp_omap1(): c_int = 0;

// McBSP register numbers. Register address offset = num * reg_step
// Common registers
// OMAP1-OMAP2420 registers
// OMAP2430 and onwards
// McBSP SPCR1 bit definitions

// McBSP SPCR2 bit definitions

// McBSP PCR bit definitions

// McBSP RCR1 bit definitions

// McBSP XCR1 bit definitions

// McBSP RCR2 bit definitions

// McBSP XCR2 bit definitions

// McBSP SRGR1 bit definitions

// McBSP SRGR2 bit definitions

// McBSP MCR1 bit definitions

// McBSP MCR2 bit definitions

// McBSP XCCR bit definitions

// McBSP RCCR bit definitions

// McBSP SYSCONFIG bit definitions

// McBSP DMA operating modes
pub const MCBSP_DMA_MODE_ELEMENT: c_int = 0;
pub const MCBSP_DMA_MODE_THRESHOLD: c_int = 1;
// McBSP WAKEUPEN/IRQST/IRQEN bit definitions

// Clock signal muxing options

// McBSP functional clock sources
pub const MCBSP_CLKS_PRCM_SRC: c_int = 0;
pub const MCBSP_CLKS_PAD_SRC: c_int = 1;
// we don't do multichannel for now
#[repr(C)]
#[derive(Copy, Clone)]
pub struct omap_mcbsp_reg_cfg {
    pub spcr2: u16,
    pub spcr1: u16,
    pub rcr2: u16,
    pub rcr1: u16,
    pub xcr2: u16,
    pub xcr1: u16,
    pub srgr2: u16,
    pub srgr1: u16,
    pub mcr2: u16,
    pub mcr1: u16,
    pub pcr0: u16,
    pub rcerc: u16,
    pub rcerd: u16,
    pub xcerc: u16,
    pub xcerd: u16,
    pub rcere: u16,
    pub rcerf: u16,
    pub xcere: u16,
    pub xcerf: u16,
    pub rcerg: u16,
    pub rcerh: u16,
    pub xcerg: u16,
    pub xcerh: u16,
    pub xccr: u16,
    pub rccr: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct omap_mcbsp {
    pub dev: *mut device,
    pub fclk: *mut clk,
    pub lock: spinlock_t,
    pub phys_base: c_ulong,
    pub phys_dma_base: c_ulong,
    pub io_base: *mut void __iomem,
    pub id: u8,
//
// Flags indicating is the bus already activated and configured by
// another substream
//
    pub active: c_int,
    pub configured: c_int,
    pub free: u8,
    pub irq: c_int,
    pub rx_irq: c_int,
    pub tx_irq: c_int,
// Protect the field .free, while checking if the mcbsp is in use
    pub pdata: *mut omap_mcbsp_platform_data,
    pub st_data: *mut omap_mcbsp_st_data,
    pub cfg_regs: omap_mcbsp_reg_cfg,
    pub dma_data: [snd_dmaengine_dai_dma_data; 2],
    pub dma_req: [c_uint; 2],
    pub dma_op_mode: c_int,
    pub max_tx_thres: u16,
    pub max_rx_thres: u16,
    pub reg_cache: *mut c_void,
    pub reg_cache_size: c_int,
    pub fmt: c_uint,
    pub in_freq: c_uint,
    pub latency: [c_uint; 2],
    pub clk_div: c_int,
    pub wlen: c_int,
    pub pm_qos_req: pm_qos_request,
}

// Sidetone specific API
extern "C" {
    pub fn omap_mcbsp_st_init(pdev: *mut platform_device) -> c_int;
}
extern "C" {
    pub fn omap_mcbsp_st_start(mcbsp: *mut omap_mcbsp) -> c_int;
}
extern "C" {
    pub fn omap_mcbsp_st_stop(mcbsp: *mut omap_mcbsp) -> c_int;
}
