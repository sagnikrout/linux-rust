//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/fsl/fsl_asrc.h
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
// fsl_asrc.h - Freescale ASRC ALSA SoC header file
//
// Copyright (C) 2014 Freescale Semiconductor, Inc.
//
// Author: Nicolin Chen <nicoleotsuka@gmail.com>
//

pub const ASRC_M2M_INPUTFIFO_WML: c_uint = 0x4;
pub const ASRC_M2M_OUTPUTFIFO_WML: c_uint = 0x2;
pub const ASRC_DMA_BUFFER_NUM: c_int = 2;
pub const ASRC_INPUTFIFO_THRESHOLD: c_int = 32;
pub const ASRC_OUTPUTFIFO_THRESHOLD: c_int = 32;
pub const ASRC_FIFO_THRESHOLD_MIN: c_int = 0;
pub const ASRC_FIFO_THRESHOLD_MAX: c_int = 63;

pub const ASRC_OUTPUT_LAST_SAMPLE: c_int = 8;
pub const IDEAL_RATIO_RATE: c_int = 1000000;
pub const REG_ASRCTR: c_uint = 0x00;
pub const REG_ASRIER: c_uint = 0x04;
pub const REG_ASRCNCR: c_uint = 0x0C;
pub const REG_ASRCFG: c_uint = 0x10;
pub const REG_ASRCSR: c_uint = 0x14;
pub const REG_ASRCDR1: c_uint = 0x18;
pub const REG_ASRCDR2: c_uint = 0x1C;

pub const REG_ASRSTR: c_uint = 0x20;
pub const REG_ASRRA: c_uint = 0x24;
pub const REG_ASRRB: c_uint = 0x28;
pub const REG_ASRRC: c_uint = 0x2C;
pub const REG_ASRPM1: c_uint = 0x40;
pub const REG_ASRPM2: c_uint = 0x44;
pub const REG_ASRPM3: c_uint = 0x48;
pub const REG_ASRPM4: c_uint = 0x4C;
pub const REG_ASRPM5: c_uint = 0x50;
pub const REG_ASRTFR1: c_uint = 0x54;
pub const REG_ASRCCR: c_uint = 0x5C;
pub const REG_ASRDIA: c_uint = 0x60;
pub const REG_ASRDOA: c_uint = 0x64;
pub const REG_ASRDIB: c_uint = 0x68;
pub const REG_ASRDOB: c_uint = 0x6C;
pub const REG_ASRDIC: c_uint = 0x70;
pub const REG_ASRDOC: c_uint = 0x74;

pub const REG_ASRIDRHA: c_uint = 0x80;
pub const REG_ASRIDRLA: c_uint = 0x84;
pub const REG_ASRIDRHB: c_uint = 0x88;
pub const REG_ASRIDRLB: c_uint = 0x8C;
pub const REG_ASRIDRHC: c_uint = 0x90;
pub const REG_ASRIDRLC: c_uint = 0x94;

pub const REG_ASR76K: c_uint = 0x98;
pub const REG_ASR56K: c_uint = 0x9C;
pub const REG_ASRMCRA: c_uint = 0xA0;
pub const REG_ASRFSTA: c_uint = 0xA4;
pub const REG_ASRMCRB: c_uint = 0xA8;
pub const REG_ASRFSTB: c_uint = 0xAC;
pub const REG_ASRMCRC: c_uint = 0xB0;
pub const REG_ASRFSTC: c_uint = 0xB4;

pub const REG_ASRMCR1A: c_uint = 0xC0;
pub const REG_ASRMCR1B: c_uint = 0xC4;
pub const REG_ASRMCR1C: c_uint = 0xC8;

// REG0 0x00 REG_ASRCTR

pub const ASRCTR_SRST_SHIFT: c_int = 4;

pub const ASRCTR_ASRCEN_SHIFT: c_int = 0;

// REG1 0x04 REG_ASRIER
pub const ASRIER_AFPWE_SHIFT: c_int = 7;

pub const ASRIER_AOLIE_SHIFT: c_int = 6;

// REG2 0x0C REG_ASRCNCR

// REG3 0x10 REG_ASRCFG

pub const ASRCFG_NDPRi_ALL_SHIFT: c_int = 18;

pub const ASRCFG_POSTMODi_WIDTH: c_int = 2;

pub const ASRCFG_PREMODi_WIDTH: c_int = 2;

// REG4 0x14 REG_ASRCSR
pub const ASRCSR_AxCSi_WIDTH: c_int = 4;

// REG5&6 0x18 & 0x1C REG_ASRCDR1 & ASRCDR2
pub const ASRCDRi_AxCPi_WIDTH: c_int = 3;

// REG7 0x20 REG_ASRSTR
pub const ASRSTR_DSLCNT_SHIFT: c_int = 21;

pub const ASRSTR_ATQOL_SHIFT: c_int = 20;

pub const ASRSTR_FPWT_SHIFT: c_int = 7;

pub const ASRSTR_AOLE_SHIFT: c_int = 6;

// REG10 0x54 REG_ASRTFR1
pub const ASRTFR1_TF_BASE_WIDTH: c_int = 7;
pub const ASRTFR1_TF_BASE_SHIFT: c_int = 6;

//
// REG22 0xA0 REG_ASRMCRA
// REG24 0xA8 REG_ASRMCRB
// REG26 0xB0 REG_ASRMCRC
//
pub const ASRMCRi_ZEROBUFi_SHIFT: c_int = 23;

pub const ASRMCRi_EXTTHRSHi_SHIFT: c_int = 22;

pub const ASRMCRi_BUFSTALLi_SHIFT: c_int = 21;

pub const ASRMCRi_BYPASSPOLYi_SHIFT: c_int = 20;

pub const ASRMCRi_OUTFIFO_THRESHOLD_WIDTH: c_int = 6;
pub const ASRMCRi_OUTFIFO_THRESHOLD_SHIFT: c_int = 12;

pub const ASRMCRi_RSYNIFi_SHIFT: c_int = 11;

pub const ASRMCRi_RSYNOFi_SHIFT: c_int = 10;

pub const ASRMCRi_INFIFO_THRESHOLD_WIDTH: c_int = 6;
pub const ASRMCRi_INFIFO_THRESHOLD_SHIFT: c_int = 0;

//
// REG23 0xA4 REG_ASRFSTA
// REG25 0xAC REG_ASRFSTB
// REG27 0xB4 REG_ASRFSTC
//
pub const ASRFSTi_OAFi_SHIFT: c_int = 23;

pub const ASRFSTi_OUTPUT_FIFO_WIDTH: c_int = 7;
pub const ASRFSTi_OUTPUT_FIFO_SHIFT: c_int = 12;

pub const ASRFSTi_IAEi_SHIFT: c_int = 11;

pub const ASRFSTi_INPUT_FIFO_WIDTH: c_int = 7;
pub const ASRFSTi_INPUT_FIFO_SHIFT: c_int = 0;

// REG28 0xC0 & 0xC4 & 0xC8 REG_ASRMCR1i
pub const ASRMCR1i_IWD_WIDTH: c_int = 3;
pub const ASRMCR1i_IWD_SHIFT: c_int = 9;

pub const ASRMCR1i_IMSB_SHIFT: c_int = 8;

pub const ASRMCR1i_OMSB_SHIFT: c_int = 2;

pub const ASRMCR1i_OSGN_SHIFT: c_int = 1;

pub const ASRMCR1i_OW16_SHIFT: c_int = 0;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum asrc_inclk {
    INCLK_NONE = 0x03,
    INCLK_ESAI_RX = 0x00,
    INCLK_SSI1_RX = 0x01,
    INCLK_SSI2_RX = 0x02,
    INCLK_SSI3_RX = 0x07,
    INCLK_SPDIF_RX = 0x04,
    INCLK_MLB_CLK = 0x05,
    INCLK_PAD = 0x06,
    INCLK_ESAI_TX = 0x08,
    INCLK_SSI1_TX = 0x09,
    INCLK_SSI2_TX = 0x0a,
    INCLK_SSI3_TX = 0x0b,
    INCLK_SPDIF_TX = 0x0c,
    INCLK_ASRCK1_CLK = 0x0f,

// clocks for imx8
    INCLK_AUD_PLL_DIV_CLK0 = 0x10,
    INCLK_AUD_PLL_DIV_CLK1 = 0x11,
    INCLK_AUD_CLK0         = 0x12,
    INCLK_AUD_CLK1         = 0x13,
    INCLK_ESAI0_RX_CLK     = 0x14,
    INCLK_ESAI0_TX_CLK     = 0x15,
    INCLK_SPDIF0_RX        = 0x16,
    INCLK_SPDIF1_RX        = 0x17,
    INCLK_SAI0_RX_BCLK     = 0x18,
    INCLK_SAI0_TX_BCLK     = 0x19,
    INCLK_SAI1_RX_BCLK     = 0x1a,
    INCLK_SAI1_TX_BCLK     = 0x1b,
    INCLK_SAI2_RX_BCLK     = 0x1c,
    INCLK_SAI3_RX_BCLK     = 0x1d,
    INCLK_ASRC0_MUX_CLK    = 0x1e,

    INCLK_ESAI1_RX_CLK     = 0x20,
    INCLK_ESAI1_TX_CLK     = 0x21,
    INCLK_SAI6_TX_BCLK     = 0x22,
    INCLK_HDMI_RX_SAI0_RX_BCLK     = 0x24,
    INCLK_HDMI_TX_SAI0_TX_BCLK     = 0x25,

    INCLK_SAI2_TX_BCLK	= 0x26,
    INCLK_SAI3_TX_BCLK	= 0x27,
    INCLK_SAI4_RX_BCLK	= 0x28,
    INCLK_SAI4_TX_BCLK	= 0x29,
    INCLK_SAI5_RX_BCLK	= 0x2a,
    INCLK_SAI5_TX_BCLK	= 0x2b,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum asrc_outclk {
    OUTCLK_NONE = 0x03,
    OUTCLK_ESAI_TX = 0x00,
    OUTCLK_SSI1_TX = 0x01,
    OUTCLK_SSI2_TX = 0x02,
    OUTCLK_SSI3_TX = 0x07,
    OUTCLK_SPDIF_TX = 0x04,
    OUTCLK_MLB_CLK = 0x05,
    OUTCLK_PAD = 0x06,
    OUTCLK_ESAI_RX = 0x08,
    OUTCLK_SSI1_RX = 0x09,
    OUTCLK_SSI2_RX = 0x0a,
    OUTCLK_SSI3_RX = 0x0b,
    OUTCLK_SPDIF_RX = 0x0c,
    OUTCLK_ASRCK1_CLK = 0x0f,

// clocks for imx8
    OUTCLK_AUD_PLL_DIV_CLK0 = 0x10,
    OUTCLK_AUD_PLL_DIV_CLK1 = 0x11,
    OUTCLK_AUD_CLK0         = 0x12,
    OUTCLK_AUD_CLK1         = 0x13,
    OUTCLK_ESAI0_RX_CLK     = 0x14,
    OUTCLK_ESAI0_TX_CLK     = 0x15,
    OUTCLK_SPDIF0_RX        = 0x16,
    OUTCLK_SPDIF1_RX        = 0x17,
    OUTCLK_SAI0_RX_BCLK     = 0x18,
    OUTCLK_SAI0_TX_BCLK     = 0x19,
    OUTCLK_SAI1_RX_BCLK     = 0x1a,
    OUTCLK_SAI1_TX_BCLK     = 0x1b,
    OUTCLK_SAI2_RX_BCLK     = 0x1c,
    OUTCLK_SAI3_RX_BCLK     = 0x1d,
    OUTCLK_ASRCO_MUX_CLK    = 0x1e,

    OUTCLK_ESAI1_RX_CLK     = 0x20,
    OUTCLK_ESAI1_TX_CLK     = 0x21,
    OUTCLK_SAI6_TX_BCLK     = 0x22,
    OUTCLK_HDMI_RX_SAI0_RX_BCLK     = 0x24,
    OUTCLK_HDMI_TX_SAI0_TX_BCLK     = 0x25,

    OUTCLK_SAI2_TX_BCLK	= 0x26,
    OUTCLK_SAI3_TX_BCLK	= 0x27,
    OUTCLK_SAI4_RX_BCLK	= 0x28,
    OUTCLK_SAI4_TX_BCLK	= 0x29,
    OUTCLK_SAI5_RX_BCLK	= 0x2a,
    OUTCLK_SAI5_TX_BCLK	= 0x2b,
}

pub const ASRC_CLK_MAX_NUM: c_int = 16;
pub const ASRC_CLK_MAP_LEN: c_uint = 0x30;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum asrc_word_width {
    ASRC_WIDTH_24_BIT = 0,
    ASRC_WIDTH_16_BIT = 1,
    ASRC_WIDTH_8_BIT = 2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct asrc_config {
    pub pair: asrc_pair_index,
    pub channel_num: c_uint,
    pub buffer_num: c_uint,
    pub dma_buffer_size: c_uint,
    pub input_sample_rate: c_uint,
    pub output_sample_rate: c_uint,
    pub input_format: snd_pcm_format_t,
    pub output_format: snd_pcm_format_t,
    pub inclk: asrc_inclk,
    pub outclk: asrc_outclk,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct asrc_req {
    pub chn_num: c_uint,
    pub index: asrc_pair_index,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct asrc_querybuf {
    pub buffer_index: c_uint,
    pub input_length: c_uint,
    pub output_length: c_uint,
    pub input_offset: c_ulong,
    pub output_offset: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct asrc_convert_buffer {
    pub input_buffer_vaddr: *mut c_void,
    pub output_buffer_vaddr: *mut c_void,
    pub input_buffer_length: c_uint,
    pub output_buffer_length: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct asrc_status_flags {
    pub index: asrc_pair_index,
    pub overload_error: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum asrc_error_status {
    ASRC_TASK_Q_OVERLOAD		= 0x01,
    ASRC_OUTPUT_TASK_OVERLOAD	= 0x02,
    ASRC_INPUT_TASK_OVERLOAD	= 0x04,
    ASRC_OUTPUT_BUFFER_OVERFLOW	= 0x08,
    ASRC_INPUT_BUFFER_UNDERRUN	= 0x10,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dma_block {
    pub dma_paddr: dma_addr_t,
    pub dma_vaddr: *mut c_void,
    pub length: c_uint,
}

//
// struct fsl_asrc_soc_data - soc specific data
//
// @use_edma: using edma as dma device or not
// @channel_bits: width of ASRCNCR register for each pair
// @start_before_dma: start asrc before dma
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsl_asrc_soc_data {
    pub use_edma: bool,
    pub channel_bits: c_uint,
    pub start_before_dma: bool,
}

//
// struct fsl_asrc_pair_priv - ASRC Pair private data
//
// @config: configuration profile
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsl_asrc_pair_priv {
    pub config: *mut asrc_config,
}

//
// struct fsl_asrc_priv - ASRC private data
//
// @asrck_clk: clock sources to driver ASRC internal logic
// @soc: soc specific data
// @clk_map: clock map for input/output clock
// @regcache_cfg: store register value of REG_ASRCFG
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsl_asrc_priv {
    pub asrck_clk: [*mut clk; ASRC_CLK_MAX_NUM],
    pub soc: *const fsl_asrc_soc_data,
    pub clk_map: [*mut c_uchar; 2],
    pub regcache_cfg: u32,
}
