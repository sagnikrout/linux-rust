//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/pci/pt3/pt3.h
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
// Earthsoft PT3 driver
//
// Copyright (C) 2014 Akihiro Tsukada <tskd08@gmail.com>
//

pub const PT3_NUM_FE: c_int = 4;
//
// register index of the FPGA chip
//
pub const REG_VERSION: c_uint = 0x00;
pub const REG_BUS: c_uint = 0x04;
pub const REG_SYSTEM_W: c_uint = 0x08;
pub const REG_SYSTEM_R: c_uint = 0x0c;
pub const REG_I2C_W: c_uint = 0x10;
pub const REG_I2C_R: c_uint = 0x14;
pub const REG_RAM_W: c_uint = 0x18;
pub const REG_RAM_R: c_uint = 0x1c;
pub const REG_DMA_BASE: c_uint = 0x40	/* regs for FE[i] = REG_DMA_BASE + 0x18 * i */;
pub const OFST_DMA_DESC_L: c_uint = 0x00;
pub const OFST_DMA_DESC_H: c_uint = 0x04;
pub const OFST_DMA_CTL: c_uint = 0x08;
pub const OFST_TS_CTL: c_uint = 0x0c;
pub const OFST_STATUS: c_uint = 0x10;
pub const OFST_TS_ERR: c_uint = 0x14;
//
// internal buffer for I2C
//
pub const PT3_I2C_MAX: c_int = 4091;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pt3_i2cbuf {
    pub data: [u8; PT3_I2C_MAX],
    pub tmp: u8,
    pub num_cmds: u32,
}

//
// DMA things
//
pub const TS_PACKET_SZ: c_int = 188;
// DMA transfers must not cross 4GiB, so use one page / transfer
pub const DATA_XFER_SZ: c_int = 4096;
pub const DATA_BUF_XFERS: c_int = 47;
// (num_bufs * DATA_BUF_SZ) % TS_PACKET_SZ must be 0

pub const MAX_DATA_BUFS: c_int = 16;
pub const MIN_DATA_BUFS: c_int = 2;

// DMA transfer description.
// device is passed a pointer to this struct, dma-reads it,
// and gets the DMA buffer ring for storing TS data.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfer_desc {
    pub /: *mut *mut u32 addr_l; / bus address of target data buffer,
    pub addr_h: u32,
    pub size: u32,
    pub /: *mut *mut u32 next_l; / bus address of the next xfer_desc,
    pub next_h: u32,
}

// A DMA mapping of a page containing xfer_desc's
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfer_desc_buffer {
    pub b_addr: dma_addr_t,
    pub /: *mut *mut *mut xfer_desc descs; / PAGE_SIZE (xfer_desc[DESCS_IN_PAGE]),
}

// A DMA mapping of a data buffer
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dma_data_buffer {
    pub b_addr: dma_addr_t,
    pub /: *mut *mut *mut u8 data; / size: u8[PAGE_SIZE],
}

//
// device things
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pt3_adap_config {
    pub demod_info: i2c_board_info,
    pub demod_cfg: tc90522_config,
    pub tuner_info: i2c_board_info,
#[repr(C)]
#[derive(Copy, Clone)]
pub union tuner_config {
    pub qm1d1c0042: qm1d1c0042_config,
    pub mxl301rf: mxl301rf_config,
    pub tuner_cfg: },
    pub init_freq: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pt3_adapter {
    pub /: *mut *mut dvb_adapter dvb_adap; / dvb_adap.priv => pt3_board,
    pub adap_idx: c_int,
    pub demux: dvb_demux,
    pub dmxdev: dmxdev,
    pub fe: *mut dvb_frontend,
    pub i2c_demod: *mut i2c_client,
    pub i2c_tuner: *mut i2c_client,
// data fetch thread
    pub thread: *mut task_struct,
    pub num_feeds: c_int,
    pub cur_lna: bool,
    pub /: *mut *mut bool cur_lnb; / current LNB power status (on/off),
// items below are for DMA
    pub buffer: [dma_data_buffer; MAX_DATA_BUFS],
    pub buf_idx: c_int,
    pub buf_ofs: c_int,
    pub /: *mut *mut int num_bufs; / == pt3_board->num_bufs,
    pub /: *mut *mut int num_discard; / how many access units to discard initially,
    pub desc_buf: [xfer_desc_buffer; MAX_DESC_BUFS],
    pub /: *mut *mut *mut int num_desc_bufs; / == num_bufs  DATA_BUF_XFERS / DESCS_IN_PAGE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pt3_board {
    pub pdev: *mut pci_dev,
    pub regs: [*mut void __iomem; 2],
// regs[0]: registers, regs[1]: internal memory, used for I2C
    pub lock: mutex,
// LNB power shared among sat-FEs
    pub /: *mut *mut int lnb_on_cnt; / LNB power on count,
// LNA shared among terr-FEs
    pub /: *mut *mut int lna_on_cnt; / booster enabled count,
    pub /: *mut *mut int num_bufs; / number of DMA buffers allocated/mapped per FE,
    pub i2c_adap: i2c_adapter,
    pub i2c_buf: *mut pt3_i2cbuf,
    pub adaps: [*mut pt3_adapter; PT3_NUM_FE],
}

//
// prototypes
//
extern "C" {
    pub fn pt3_alloc_dmabuf(adap: *mut pt3_adapter) -> c_int;
}
extern "C" {
    pub fn pt3_init_dmabuf(adap: *mut pt3_adapter);
}
extern "C" {
    pub fn pt3_free_dmabuf(adap: *mut pt3_adapter);
}
extern "C" {
    pub fn pt3_start_dma(adap: *mut pt3_adapter) -> c_int;
}
extern "C" {
    pub fn pt3_stop_dma(adap: *mut pt3_adapter) -> c_int;
}
extern "C" {
    pub fn pt3_proc_dma(adap: *mut pt3_adapter) -> c_int;
}
extern "C" {
    pub fn pt3_i2c_functionality(adap: *mut i2c_adapter) -> u32;
}
extern "C" {
    pub fn pt3_i2c_reset(pt3: *mut pt3_board);
}
extern "C" {
    pub fn pt3_init_all_demods(pt3: *mut pt3_board) -> c_int;
}
extern "C" {
    pub fn pt3_init_all_mxl301rf(pt3: *mut pt3_board) -> c_int;
}
