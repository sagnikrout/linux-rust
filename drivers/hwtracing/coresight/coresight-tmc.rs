//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/hwtracing/coresight/coresight-tmc.h
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
// Copyright(C) 2015 Linaro Limited. All rights reserved.
// Author: Mathieu Poirier <mathieu.poirier@linaro.org>
//

pub const TMC_RSZ: c_uint = 0x004;
pub const TMC_STS: c_uint = 0x00c;
pub const TMC_RRD: c_uint = 0x010;
pub const TMC_RRP: c_uint = 0x014;
pub const TMC_RWP: c_uint = 0x018;
pub const TMC_TRG: c_uint = 0x01c;
pub const TMC_CTL: c_uint = 0x020;
pub const TMC_RWD: c_uint = 0x024;
pub const TMC_MODE: c_uint = 0x028;
pub const TMC_LBUFLEVEL: c_uint = 0x02c;
pub const TMC_CBUFLEVEL: c_uint = 0x030;
pub const TMC_BUFWM: c_uint = 0x034;
pub const TMC_RRPHI: c_uint = 0x038;
pub const TMC_RWPHI: c_uint = 0x03c;
pub const TMC_AXICTL: c_uint = 0x110;
pub const TMC_DBALO: c_uint = 0x118;
pub const TMC_DBAHI: c_uint = 0x11c;
pub const TMC_FFSR: c_uint = 0x300;
pub const TMC_FFCR: c_uint = 0x304;
pub const TMC_PSCR: c_uint = 0x308;
pub const TMC_ITMISCOP0: c_uint = 0xee0;
pub const TMC_ITTRFLIN: c_uint = 0xee8;
pub const TMC_ITATBDATA0: c_uint = 0xeec;
pub const TMC_ITATBCTR2: c_uint = 0xef0;
pub const TMC_ITATBCTR1: c_uint = 0xef4;
pub const TMC_ITATBCTR0: c_uint = 0xef8;
pub const TMC_AUTHSTATUS: c_uint = 0xfb8;
// register description
// TMC_CTL - 0x020

// TMC_STS - 0x00C
pub const TMC_STS_TMCREADY_BIT: c_int = 2;

//
// TMC_AXICTL - 0x110
//
// TMC AXICTL format for SoC-400
// Bits [0-1]	: ProtCtrlBit0-1
// Bits [2-5]	: CacheCtrlBits 0-3 (AXCACHE)
// Bit  6		: Reserved
// Bit  7		: ScatterGatherMode
// Bits [8-11]	: WrBurstLen
// Bits [12-31]	: Reserved.
// TMC AXICTL format for SoC-600, as above except:
// Bits [2-5]	: AXI WCACHE
// Bits [16-19]	: AXI RCACHE
// Bits [20-31]	: Reserved
//
pub const TMC_AXICTL_CLEAR_MASK: c_uint = 0xfbf;

pub const TMC_AXICTL_WR_BURST_16: c_uint = 0xf;
// Write-back Read and Write-allocate

// TMC_FFSR - 0x300

// TMC_FFCR - 0x304
pub const TMC_FFCR_FLUSHMAN_BIT: c_int = 6;

pub const TMC_DEVID_AXIAW_SHIFT: c_int = 17;
pub const TMC_DEVID_AXIAW_MASK: c_uint = 0x7f;

// Major version 1 Minor version 0

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tmc_config_type {
    TMC_CONFIG_TYPE_ETB,
    TMC_CONFIG_TYPE_ETR,
    TMC_CONFIG_TYPE_ETF,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tmc_mode {
    TMC_MODE_CIRCULAR_BUFFER,
    TMC_MODE_SOFTWARE_FIFO,
    TMC_MODE_HARDWARE_FIFO,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tmc_mem_intf_width {
    TMC_MEM_INTF_WIDTH_32BITS	= 1,
    TMC_MEM_INTF_WIDTH_64BITS	= 2,
    TMC_MEM_INTF_WIDTH_128BITS	= 4,
    TMC_MEM_INTF_WIDTH_256BITS	= 8,
}

// TMC ETR Capability bit definitions

// ETR has separate read/write cache encodings

//
// TMC_ETR_SAVE_RESTORE - Values of RRP/RWP/STS.Full are
// retained when TMC leaves Disabled state, allowing us to continue
// the tracing from a point where we stopped. This also implies that
// the RRP/RWP/STS.Full should always be programmed to the correct
// value. Unfortunately this is not advertised by the hardware,
// so we have to rely on PID of the IP to detect the functionality.
//

// Coresight SoC-600 TMC-ETR unadvertised capabilities

// TMC metadata region for ETR and ETF configurations
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tmc_crash_metadata {
    pub /: *mut *mut uint32_t crc32_mdata; / crc of metadata,
    pub /: *mut *mut uint32_t crc32_tdata; / crc of tracedata,
    pub /: *mut *mut uint32_t version; / 31:16 Major version, 15:0 Minor version,
    pub /: *mut *mut uint32_t valid; / Indicate if this ETF/ETR was enabled,
    pub /: *mut *mut uint32_t tmc_ram_size; / Ram Size register,
    pub /: *mut *mut uint32_t tmc_sts; / Status register,
    pub /: *mut *mut uint32_t tmc_mode; / Mode register,
    pub /: *mut *mut uint32_t tmc_ffcr; / Formatter and flush control register,
    pub /: *mut *mut uint32_t tmc_ffsr; / Formatter and flush status register,
    pub reserved32: u32,
    pub /: *mut *mut uint64_t tmc_rrp; / Ram Read pointer register,
    pub /: *mut *mut uint64_t tmc_rwp; / Ram Write pointer register,
    pub /: *mut *mut uint64_t tmc_dba; / Data buffer address register,
    pub /: *mut *mut uint64_t trace_paddr; / Phys address of trace buffer,
    pub reserved64: [u64; 3],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum etr_mode {
    ETR_MODE_FLAT,		/* Uses contiguous flat buffer */
    ETR_MODE_ETR_SG,	/* Uses in-built TMC ETR SG mechanism */
    ETR_MODE_CATU,		/* Use SG mechanism in CATU */
    ETR_MODE_RESRV,		/* Use reserved region contiguous buffer */
    ETR_MODE_AUTO,		/* Use the default mechanism */
}

//
// struct etr_buf - Details of the buffer used by ETR
// refcount	; Number of sources currently using this etr_buf.
// @mode	: Mode of the ETR buffer, contiguous, Scatter Gather etc.
// @full	: Trace data overflow
// @size	: Size of the buffer.
// @hwaddr	: Address to be programmed in the TMC:DBA{LO,HI}
// @offset	: Offset of the trace data in the buffer for consumption.
// @len		: Available trace data @buf (may round up to the beginning).
// @ops		: ETR buffer operations for the mode.
// @private	: Backend specific information for the buf
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct etr_buf {
    pub refcount: refcount_t,
    pub mode: etr_mode,
    pub full: bool,
    pub size: isize,
    pub hwaddr: dma_addr_t,
    pub offset: c_ulong,
    pub len: i64,
    pub ops: *const etr_buf_operations,
    pub private: *mut c_void,
}

//
// @paddr	: Start address of reserved memory region.
// @vaddr	: Corresponding CPU virtual address.
// @size	: Size of reserved memory region.
// @offset	: Offset of the trace data in the buffer for consumption.
// @reading	: Flag to indicate if reading is active
// @len	: Available trace data @buf (may round up to the beginning).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tmc_resrv_buf {
    pub paddr: phys_addr_t,
    pub vaddr: *mut c_void,
    pub size: usize,
    pub offset: c_ulong,
    pub reading: bool,
    pub len: i64,
}

//
// struct tmc_drvdata - specifics associated to an TMC component
// @atclk:	optional clock for the core parts of the TMC.
// @pclk:	APB clock if present, otherwise NULL
// @base:	memory mapped base address for this component.
// @csdev:	component vitals needed by the framework.
// @miscdev:	specifics to handle "/dev/xyz.tmc" entry.
// @crashdev:	specifics to handle "/dev/crash_tmc_xyz" entry for reading
// crash tracedata.
// @spinlock:	only one at a time pls.
// @pid:	Process ID of the process that owns the session that is using
// this component. For example this would be the pid of the Perf
// process.
// @reading:	buffer's in the reading through "/dev/xyz.tmc" entry
// @stop_on_flush: Stop on flush trigger user configuration.
// @buf:	Snapshot of the trace data for ETF/ETB.
// @etr_buf:	details of buffer used in TMC-ETR
// @len:	size of the available trace for ETF/ETB.
// @size:	trace buffer size for this TMC (common for all modes).
// @max_burst_size: The maximum burst size that can be initiated by
// TMC-ETR on AXI bus.
// @config_type: TMC variant, must be of type @tmc_config_type.
// @memwidth:	width of the memory interface databus, in bytes.
// @trigger_cntr: amount of words to store after a trigger.
// @etr_caps:	Bitmask of capabilities of the TMC ETR, inferred from the
// device configuration register (DEVID)
// @etr_mode:	User preferred mode of the ETR device, default auto mode.
// @idr:	Holds etr_bufs allocated for this ETR.
// @idr_mutex:	Access serialisation for idr.
// @sysfs_buf:	SYSFS buffer for ETR.
// @perf_buf:	PERF buffer for ETR.
// @resrv_buf:  Used by ETR as hardware trace buffer and for trace data
// retention (after crash) only when ETR_MODE_RESRV buffer
// mode is enabled. Used by ETF for trace data retention
// (after crash) by default.
// @crash_mdata: Reserved memory for storing tmc crash metadata.
// Used by ETR/ETF.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tmc_drvdata {
    pub atclk: *mut clk,
    pub pclk: *mut clk,
    pub base: *mut void __iomem,
    pub csdev: *mut coresight_device,
    pub miscdev: miscdevice,
    pub crashdev: miscdevice,
    pub spinlock: raw_spinlock_t,
    pub pid: pid_t,
    pub reading: bool,
    pub stop_on_flush: bool,
    pub /: *mut *mut *mut char buf; / TMC ETB,
    pub /: *mut *mut *mut etr_buf etr_buf; / TMC ETR,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct etr_buf_operations {
    pub pages): *mut int node, void,
    pub rwp): *mut *mut *mut void (sync)(struct etr_buf etr_buf, u64 rrp, u64,
    pub bufpp): *mut c_char,
    pub etr_buf): *mut *mut void (free)(struct etr_buf,
}

//
// struct tmc_pages - Collection of pages used for SG.
// @nr_pages:		Number of pages in the list.
// @daddrs:		Array of DMA'able page address.
// @pages:		Array pages for the buffer.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tmc_pages {
    pub nr_pages: c_int,
    pub daddrs: *mut dma_addr_t,
    pub pages: *mut page,
}

//
// struct tmc_sg_table - Generic SG table for TMC
// @dev:		Device for DMA allocations
// @table_vaddr:	Contiguous Virtual address for PageTable
// @data_vaddr:		Contiguous Virtual address for Data Buffer
// @table_daddr:	DMA address of the PageTable base
// @node:		Node for Page allocations
// @table_pages:	List of pages & dma address for Table
// @data_pages:		List of pages & dma address for Data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tmc_sg_table {
    pub dev: *mut device,
    pub table_vaddr: *mut c_void,
    pub data_vaddr: *mut c_void,
    pub table_daddr: dma_addr_t,
    pub node: c_int,
    pub table_pages: tmc_pages,
    pub data_pages: tmc_pages,
}

// Generic functions
extern "C" {
    pub fn tmc_wait_for_tmcready(drvdata: *mut tmc_drvdata) -> c_int;
}
extern "C" {
    pub fn tmc_flush_and_stop(drvdata: *mut tmc_drvdata);
}
extern "C" {
    pub fn tmc_enable_hw(drvdata: *mut tmc_drvdata);
}
extern "C" {
    pub fn tmc_disable_hw(drvdata: *mut tmc_drvdata);
}
extern "C" {
    pub fn tmc_get_memwidth_mask(drvdata: *mut tmc_drvdata) -> u32;
}
extern "C" {
    pub fn tmc_read_prepare_crashdata(drvdata: *mut tmc_drvdata) -> c_int;
}
// ETB/ETF functions
extern "C" {
    pub fn tmc_read_prepare_etb(drvdata: *mut tmc_drvdata) -> c_int;
}
extern "C" {
    pub fn tmc_read_unprepare_etb(drvdata: *mut tmc_drvdata) -> c_int;
}
// ETR functions
extern "C" {
    pub fn tmc_read_prepare_etr(drvdata: *mut tmc_drvdata) -> c_int;
}
extern "C" {
    pub fn tmc_read_unprepare_etr(drvdata: *mut tmc_drvdata) -> c_int;
}
extern "C" {
    pub fn tmc_etr_disable_hw(drvdata: *mut tmc_drvdata);
}

// Initialise the caps from unadvertised static capabilities of the device
extern "C" {
    pub fn tmc_free_sg_table(sg_table: *mut tmc_sg_table);
}
extern "C" {
    pub fn tmc_sg_table_sync_table(sg_table: *mut tmc_sg_table);
}
extern "C" {
    pub fn crc32_le(_arg: 0, )&md->crc32_tdata: *mut (void, _arg: crc_size) -> return;
}
// Take CRC of configured buffer size to keep it simple
extern "C" {
    pub fn crc32_le(_arg: 0, )drvdata->resrv_buf.vaddr: *mut (void, _arg: crc_size) -> return;
}
extern "C" {
    pub fn tmc_etr_set_catu_ops(catu: *const etr_buf_operations);
}
extern "C" {
    pub fn tmc_etr_remove_catu_ops();
}
