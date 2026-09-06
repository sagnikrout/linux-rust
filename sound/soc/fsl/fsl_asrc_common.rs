//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/fsl/fsl_asrc_common.h
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
// Copyright 2019 NXP
//
// directions
pub const IN: c_int = 0;
pub const OUT: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum asrc_pair_index {
    ASRC_INVALID_PAIR = -1,
    ASRC_PAIR_A = 0,
    ASRC_PAIR_B = 1,
    ASRC_PAIR_C = 2,
    ASRC_PAIR_D = 3,
}

pub const PAIR_CTX_NUM: c_uint = 0x4;
//
// struct fsl_asrc_m2m_cap - capability data
// @fmt_in: input sample format
// @fmt_out: output sample format
// @chan_min: minimum channel number
// @chan_max: maximum channel number
// @rate_in: minimum rate
// @rate_out: maximum rete
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsl_asrc_m2m_cap {
    pub fmt_in: u64,
    pub fmt_out: u64,
    pub chan_min: c_int,
    pub chan_max: c_int,
    pub rate_in: *const c_uint,
    pub rate_in_count: c_int,
    pub rate_out: *const c_uint,
    pub rate_out_count: c_int,
}

//
// fsl_asrc_pair: ASRC Pair common data
//
// @asrc: pointer to its parent module
// @error: error record
// @index: pair index (ASRC_PAIR_A, ASRC_PAIR_B, ASRC_PAIR_C)
// @channels: occupied channel number
// @desc: input and output dma descriptors
// @dma_chan: inputer and output DMA channels
// @dma_data: private dma data
// @pos: hardware pointer position
// @req_dma_chan: flag to release dev_to_dev chan
// @private: pair private area
// @complete: dma task complete
// @sample_format: format of m2m
// @rate: rate of m2m
// @buf_len: buffer length of m2m
// @dma_buffer: buffer pointers
// @first_convert: start of conversion
// @ratio_mod_flag: flag for new ratio modifier
// @ratio_mod: ratio modification
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsl_asrc_pair {
    pub asrc: *mut fsl_asrc,
    pub error: c_uint,
    pub index: asrc_pair_index,
    pub channels: c_uint,
    pub desc: [*mut dma_async_tx_descriptor; 2],
    pub dma_chan: [*mut dma_chan; 2],
    pub dma_data: imx_dma_data,
    pub pos: c_uint,
    pub req_dma_chan: bool,
    pub private: *mut c_void,
// used for m2m
    pub complete: [completion; 2],
    pub sample_format: [snd_pcm_format_t; 2],
    pub rate: [c_uint; 2],
    pub buf_len: [c_uint; 2],
    pub dma_buffer: [snd_dma_buffer; 2],
    pub first_convert: c_uint,
    pub ratio_mod_flag: bool,
    pub ratio_mod: c_uint,
}

//
// fsl_asrc: ASRC common data
//
// @dma_params_rx: DMA parameters for receive channel
// @dma_params_tx: DMA parameters for transmit channel
// @pdev: platform device pointer
// @regmap: regmap handler
// @paddr: physical address to the base address of registers
// @mem_clk: clock source to access register
// @ipg_clk: clock source to drive peripheral
// @spba_clk: SPBA clock (optional, depending on SoC design)
// @card: compress sound card
// @lock: spin lock for resource protection
// @pair: pair pointers
// @channel_avail: non-occupied channel numbers
// @asrc_rate: default sample rate for ASoC Back-Ends
// @asrc_format: default sample format for ASoC Back-Ends
// @use_edma: edma is used
// @start_before_dma: start asrc before dma
// @get_dma_channel: function pointer
// @request_pair: function pointer
// @release_pair: function pointer
// @get_fifo_addr: function pointer
// @m2m_get_cap: function pointer
// @m2m_prepare: function pointer
// @m2m_start: function pointer
// @m2m_unprepare: function pointer
// @m2m_stop: function pointer
// @m2m_output_ready: function pointer, check output fifo ready or not
// @m2m_calc_out_len: function pointer
// @m2m_get_maxburst: function pointer
// @m2m_pair_suspend: function pointer
// @m2m_pair_resume: function pointer
// @m2m_set_ratio_mod: function pointer
// @get_output_fifo_size: function pointer
// @pair_priv_size: size of pair private struct.
// @private: private data structure
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsl_asrc {
    pub dma_params_rx: snd_dmaengine_dai_dma_data,
    pub dma_params_tx: snd_dmaengine_dai_dma_data,
    pub pdev: *mut platform_device,
    pub regmap: *mut regmap,
    pub paddr: c_ulong,
    pub mem_clk: *mut clk,
    pub ipg_clk: *mut clk,
    pub spba_clk: *mut clk,
    pub card: *mut snd_card,
    pub /: *mut *mut spinlock_t lock; / spin lock for resource protection,
    pub pair: [*mut fsl_asrc_pair; PAIR_CTX_NUM],
    pub channel_avail: c_uint,
    pub asrc_rate: c_int,
    pub asrc_format: snd_pcm_format_t,
    pub use_edma: bool,
    pub start_before_dma: bool,
    pub dir): *mut *mut *mut *mut dma_chan (get_dma_channel)(fsl_asrc_pair pair, bool,
    pub pair): *mut *mut int (request_pair)(int channels, struct fsl_asrc_pair,
    pub pair): *mut *mut void (release_pair)(struct fsl_asrc_pair,
    pub index): *mut *mut int (get_fifo_addr)(u8 dir, enum asrc_pair_index,
    pub cap): *mut *mut int (m2m_get_cap)(struct fsl_asrc_m2m_cap,
    pub pair): *mut *mut int (m2m_prepare)(struct fsl_asrc_pair,
    pub pair): *mut *mut int (m2m_start)(struct fsl_asrc_pair,
    pub pair): *mut *mut int (m2m_unprepare)(struct fsl_asrc_pair,
    pub pair): *mut *mut int (m2m_stop)(struct fsl_asrc_pair,
    pub pair): *mut *mut bool (m2m_output_ready)(struct fsl_asrc_pair,
    pub input_buffer_length): *mut *mut *mut int (m2m_calc_out_len)(struct fsl_asrc_pair pair, int,
    pub pair): *mut *mut int (m2m_get_maxburst)(u8 dir, struct fsl_asrc_pair,
    pub pair): *mut *mut int (m2m_pair_suspend)(struct fsl_asrc_pair,
    pub pair): *mut *mut int (m2m_pair_resume)(struct fsl_asrc_pair,
    pub val): *mut *mut *mut int (m2m_set_ratio_mod)(struct fsl_asrc_pair pair, int,
    pub pair): *mut *mut unsigned int (get_output_fifo_size)(struct fsl_asrc_pair,
    pub pair_priv_size: usize,
    pub private: *mut c_void,
}

extern "C" {
    pub fn fsl_asrc_m2m_init(asrc: *mut fsl_asrc) -> c_int;
}
extern "C" {
    pub fn fsl_asrc_m2m_exit(asrc: *mut fsl_asrc);
}
extern "C" {
    pub fn fsl_asrc_m2m_resume(asrc: *mut fsl_asrc) -> c_int;
}
extern "C" {
    pub fn fsl_asrc_m2m_suspend(asrc: *mut fsl_asrc) -> c_int;
}
