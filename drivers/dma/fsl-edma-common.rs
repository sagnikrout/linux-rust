//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/dma/fsl-edma-common.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// Copyright 2013-2014 Freescale Semiconductor, Inc.
// Copyright 2018 Angelo Dureghello <angelo@sysam.it>
//

pub const EDMAMUX_CHCFG_DIS: c_uint = 0x0;
pub const EDMAMUX_CHCFG_ENBL: c_uint = 0x80;

pub const DMAMUX_NR: c_int = 2;
pub const EDMA_TCD: c_uint = 0x1000;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fsl_edma_pm_state {
    RUNNING = 0,
    SUSPENDED,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsl_edma_hw_tcd {
    pub saddr: __le32,
    pub soff: __le16,
    pub attr: __le16,
    pub nbytes: __le32,
    pub slast: __le32,
    pub daddr: __le32,
    pub doff: __le16,
    pub citer: __le16,
    pub dlast_sga: __le32,
    pub csr: __le16,
    pub biter: __le16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsl_edma_hw_tcd64 {
    pub saddr: __le64,
    pub soff: __le16,
    pub attr: __le16,
    pub nbytes: __le32,
    pub slast: __le64,
    pub daddr: __le64,
    pub dlast_sga: __le64,
    pub doff: __le16,
    pub citer: __le16,
    pub csr: __le16,
    pub biter: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsl_edma3_ch_reg {
    pub ch_csr: __le32,
    pub ch_es: __le32,
    pub ch_int: __le32,
    pub ch_sbr: __le32,
    pub ch_pri: __le32,
    pub ch_mux: __le32,
    pub /: *mut *mut __le32 ch_mattr; / edma4, reserved for edma3,
    pub ch_reserved: __le32,
    pub tcd: fsl_edma_hw_tcd,
    pub tcd64: fsl_edma_hw_tcd64,
}

//
// These are iomem pointers, for both v32 and v64.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct edma_regs {
    pub cr: *mut void __iomem,
    pub es: *mut void __iomem,
    pub erqh: *mut void __iomem,
    pub /: *mut *mut *mut void __iomem erql; / aka erq on v32,
    pub eeih: *mut void __iomem,
    pub /: *mut *mut *mut void __iomem eeil; / aka eei on v32,
    pub seei: *mut void __iomem,
    pub ceei: *mut void __iomem,
    pub serq: *mut void __iomem,
    pub cerq: *mut void __iomem,
    pub cint: *mut void __iomem,
    pub cerr: *mut void __iomem,
    pub ssrt: *mut void __iomem,
    pub cdne: *mut void __iomem,
    pub inth: *mut void __iomem,
    pub intl: *mut void __iomem,
    pub errh: *mut void __iomem,
    pub errl: *mut void __iomem,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsl_edma_sw_tcd {
    pub ptcd: dma_addr_t,
    pub vtcd: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsl_edma_chan {
    pub vchan: virt_dma_chan,
    pub status: dma_status,
    pub pm_state: fsl_edma_pm_state,
    pub edma: *mut fsl_edma_engine,
    pub edesc: *mut fsl_edma_desc,
    pub cfg: dma_slave_config,
    pub attr: u32,
    pub is_sw: bool,
    pub tcd_pool: *mut dma_pool,
    pub dma_dev_addr: dma_addr_t,
    pub dma_dev_size: u32,
    pub dma_dir: dma_data_direction,
    pub chan_name: [c_char; 32],
    pub errirq_name: [c_char; 36],
    pub tcd: *mut void __iomem,
    pub mux_addr: *mut void __iomem,
    pub real_count: u32,
    pub issue_worker: work_struct,
    pub pdev: *mut platform_device,
    pub pd_dev: *mut device,
    pub pd_dev_link: *mut device_link,
    pub srcid: u32,
    pub clk: *mut clk,
    pub priority: c_int,
    pub hw_chanid: c_int,
    pub txirq: c_int,
    pub errirq: c_int,
    pub dev_id): *mut *mut irqreturn_t (irq_handler)(int irq, void,
    pub dev_id): *mut *mut irqreturn_t (errirq_handler)(int irq, void,
    pub is_rxchan: bool,
    pub is_remote: bool,
    pub is_multi_fifo: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsl_edma_desc {
    pub vdesc: virt_dma_desc,
    pub echan: *mut fsl_edma_chan,
    pub iscyclic: bool,
    pub dirn: dma_transfer_direction,
    pub n_tcds: c_uint,
    pub tcd: [fsl_edma_sw_tcd; ],
}

// control and status register is in tcd address space, edma3 reg layout

// Need clean CHn_CSR DONE before enable TCD's ESG

// Need clean CHn_CSR DONE before enable TCD's MAJORELINK

// All channel ERR IRQ share one IRQ line

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsl_edma_drvdata {
    pub /: *mut *mut u32 dmamuxs; / only used before v3,
    pub chreg_off: u32,
    pub chreg_space_sz: u32,
    pub flags: u32,
    pub /: *mut *mut u32 mux_off; / channel mux register offset,
    pub /: *mut *mut u32 mux_skip; / how much skip for each channel,
    pub fsl_edma): *mut fsl_edma_engine,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsl_edma_engine {
    pub dma_dev: dma_device,
    pub membase: *mut void __iomem,
    pub muxbase: [*mut void __iomem; DMAMUX_NR],
    pub muxclk: [*mut clk; DMAMUX_NR],
    pub dmaclk: *mut clk,
    pub fsl_edma_mutex: mutex,
    pub drvdata: *const fsl_edma_drvdata,
    pub n_chans: u32,
    pub txirq: c_int,
    pub txirq_16_31: c_int,
    pub errirq: c_int,
    pub big_endian: bool,
    pub regs: edma_regs,
    pub chan_masked: u64,
    pub __counted_by(n_chans): fsl_edma_chan chans[],
}

// Need after struct defination

//
// R/W functions for big- or little-endian registers:
// The eDMA controller's endian is independent of the CPU core's endian.
// For the big-endian IP module, the offset for 8-bit or 16-bit registers
// should also be swapped opposite to that in little-endian IP.
//
// swap the reg offset for these in big-endian mode
extern "C" {
    pub fn container_of(_arg: chan, fsl_edma_chan: struct, _arg: vchan.chan) -> return;
}
extern "C" {
    pub fn container_of(_arg: vd, fsl_edma_desc: struct, _arg: vdesc) -> return;
}
extern "C" {
    pub fn fsl_edma_tx_chan_handler(fsl_chan: *mut fsl_edma_chan);
}
extern "C" {
    pub fn fsl_edma_disable_request(fsl_chan: *mut fsl_edma_chan);
}
extern "C" {
    pub fn fsl_edma_free_desc(vdesc: *mut virt_dma_desc);
}
extern "C" {
    pub fn fsl_edma_terminate_all(chan: *mut dma_chan) -> c_int;
}
extern "C" {
    pub fn fsl_edma_pause(chan: *mut dma_chan) -> c_int;
}
extern "C" {
    pub fn fsl_edma_resume(chan: *mut dma_chan) -> c_int;
}
extern "C" {
    pub fn fsl_edma_xfer_desc(fsl_chan: *mut fsl_edma_chan);
}
extern "C" {
    pub fn fsl_edma_issue_pending(chan: *mut dma_chan);
}
extern "C" {
    pub fn fsl_edma_alloc_chan_resources(chan: *mut dma_chan) -> c_int;
}
extern "C" {
    pub fn fsl_edma_free_chan_resources(chan: *mut dma_chan);
}
extern "C" {
    pub fn fsl_edma_cleanup_vchan(dmadev: *mut dma_device);
}
extern "C" {
    pub fn fsl_edma_setup_regs(edma: *mut fsl_edma_engine);
}
