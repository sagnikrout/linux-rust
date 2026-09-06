//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/fsl/fsl_sai.h
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
// Copyright 2012-2013 Freescale Semiconductor, Inc.
//

pub const FAL_SAI_NUM_RATES: c_int = 20;

// SAI Register Map Register
pub const FSL_SAI_VERID: c_uint = 0x00 /* SAI Version ID Register */;
pub const FSL_SAI_PARAM: c_uint = 0x04 /* SAI Parameter Register */;

pub const FSL_SAI_TDR0: c_uint = 0x20 /* SAI Transmit Data 0 */;
pub const FSL_SAI_TDR1: c_uint = 0x24 /* SAI Transmit Data 1 */;
pub const FSL_SAI_TDR2: c_uint = 0x28 /* SAI Transmit Data 2 */;
pub const FSL_SAI_TDR3: c_uint = 0x2C /* SAI Transmit Data 3 */;
pub const FSL_SAI_TDR4: c_uint = 0x30 /* SAI Transmit Data 4 */;
pub const FSL_SAI_TDR5: c_uint = 0x34 /* SAI Transmit Data 5 */;
pub const FSL_SAI_TDR6: c_uint = 0x38 /* SAI Transmit Data 6 */;
pub const FSL_SAI_TDR7: c_uint = 0x3C /* SAI Transmit Data 7 */;
pub const FSL_SAI_TFR0: c_uint = 0x40 /* SAI Transmit FIFO 0 */;
pub const FSL_SAI_TFR1: c_uint = 0x44 /* SAI Transmit FIFO 1 */;
pub const FSL_SAI_TFR2: c_uint = 0x48 /* SAI Transmit FIFO 2 */;
pub const FSL_SAI_TFR3: c_uint = 0x4C /* SAI Transmit FIFO 3 */;
pub const FSL_SAI_TFR4: c_uint = 0x50 /* SAI Transmit FIFO 4 */;
pub const FSL_SAI_TFR5: c_uint = 0x54 /* SAI Transmit FIFO 5 */;
pub const FSL_SAI_TFR6: c_uint = 0x58 /* SAI Transmit FIFO 6 */;
pub const FSL_SAI_TFR7: c_uint = 0x5C /* SAI Transmit FIFO 7 */;
pub const FSL_SAI_TMR: c_uint = 0x60 /* SAI Transmit Mask */;
pub const FSL_SAI_TTCTL: c_uint = 0x70 /* SAI Transmit Timestamp Control Register */;
pub const FSL_SAI_TTCTN: c_uint = 0x74 /* SAI Transmit Timestamp Counter Register */;
pub const FSL_SAI_TBCTN: c_uint = 0x78 /* SAI Transmit Bit Counter Register */;
pub const FSL_SAI_TTCAP: c_uint = 0x7C /* SAI Transmit Timestamp Capture */;

pub const FSL_SAI_RDR0: c_uint = 0xa0 /* SAI Receive Data 0 */;
pub const FSL_SAI_RDR1: c_uint = 0xa4 /* SAI Receive Data 1 */;
pub const FSL_SAI_RDR2: c_uint = 0xa8 /* SAI Receive Data 2 */;
pub const FSL_SAI_RDR3: c_uint = 0xac /* SAI Receive Data 3 */;
pub const FSL_SAI_RDR4: c_uint = 0xb0 /* SAI Receive Data 4 */;
pub const FSL_SAI_RDR5: c_uint = 0xb4 /* SAI Receive Data 5 */;
pub const FSL_SAI_RDR6: c_uint = 0xb8 /* SAI Receive Data 6 */;
pub const FSL_SAI_RDR7: c_uint = 0xbc /* SAI Receive Data 7 */;
pub const FSL_SAI_RFR0: c_uint = 0xc0 /* SAI Receive FIFO 0 */;
pub const FSL_SAI_RFR1: c_uint = 0xc4 /* SAI Receive FIFO 1 */;
pub const FSL_SAI_RFR2: c_uint = 0xc8 /* SAI Receive FIFO 2 */;
pub const FSL_SAI_RFR3: c_uint = 0xcc /* SAI Receive FIFO 3 */;
pub const FSL_SAI_RFR4: c_uint = 0xd0 /* SAI Receive FIFO 4 */;
pub const FSL_SAI_RFR5: c_uint = 0xd4 /* SAI Receive FIFO 5 */;
pub const FSL_SAI_RFR6: c_uint = 0xd8 /* SAI Receive FIFO 6 */;
pub const FSL_SAI_RFR7: c_uint = 0xdc /* SAI Receive FIFO 7 */;
pub const FSL_SAI_RMR: c_uint = 0xe0 /* SAI Receive Mask */;
pub const FSL_SAI_RTCTL: c_uint = 0xf0 /* SAI Receive Timestamp Control Register */;
pub const FSL_SAI_RTCTN: c_uint = 0xf4 /* SAI Receive Timestamp Counter Register */;
pub const FSL_SAI_RBCTN: c_uint = 0xf8 /* SAI Receive Bit Counter Register */;
pub const FSL_SAI_RTCAP: c_uint = 0xfc /* SAI Receive Timestamp Capture */;
pub const FSL_SAI_MCTL: c_uint = 0x100 /* SAI MCLK Control Register */;
pub const FSL_SAI_MDIV: c_uint = 0x104 /* SAI MCLK Divide Register */;

// SAI Transmit/Receive Control Register

pub const FSL_SAI_CSR_xF_SHIFT: c_int = 16;
pub const FSL_SAI_CSR_xF_W_SHIFT: c_int = 18;

pub const FSL_SAI_CSR_xIE_SHIFT: c_int = 8;

// SAI Transmit and Receive Configuration 1 Register

// SAI Transmit and Receive Configuration 2 Register

pub const FSL_SAI_CR2_MSEL_BUS: c_int = 0;

pub const FSL_SAI_CR2_DIV_MASK: c_uint = 0xff;
// SAI Transmit and Receive Configuration 3 Register

pub const FSL_SAI_CR3_WDFL_MASK: c_uint = 0x1f;
// SAI Transmit and Receive Configuration 4 Register

// SAI Transmit and Receive Configuration 5 Register

// SAI MCLK Control Register

pub const FSL_SAI_MCTL_MSEL_BUS: c_int = 0;

pub const FSL_SAI_MCTL_DIV_MASK: c_uint = 0xFF;
// SAI VERID Register
pub const FSL_SAI_VERID_MAJOR_SHIFT: c_int = 24;

pub const FSL_SAI_VERID_MINOR_SHIFT: c_int = 16;

pub const FSL_SAI_VERID_FEATURE_SHIFT: c_int = 0;

// SAI PARAM Register
pub const FSL_SAI_PARAM_SPF_SHIFT: c_int = 16;

pub const FSL_SAI_PARAM_WPF_SHIFT: c_int = 8;

// SAI MCLK Divide Register
pub const FSL_SAI_MDIV_MASK: c_uint = 0xFFFFF;
// SAI timestamp and bitcounter
pub const FSL_SAI_xTCTL_TSEN_SHIFT: c_int = 0;

pub const FSL_SAI_xTCTL_TSINC_SHIFT: c_int = 1;

pub const FSL_SAI_xTCTL_RTSC_SHIFT: c_int = 8;

pub const FSL_SAI_xTCTL_RBC_SHIFT: c_int = 9;

// SAI type

// SAI clock sources
pub const FSL_SAI_CLK_BUS: c_int = 0;
pub const FSL_SAI_CLK_MAST1: c_int = 1;
pub const FSL_SAI_CLK_MAST2: c_int = 2;
pub const FSL_SAI_CLK_MAST3: c_int = 3;
pub const FSL_SAI_MCLK_MAX: c_int = 4;
// SAI data transfer numbers per DMA request
pub const FSL_SAI_MAXBURST_TX: c_int = 6;
pub const FSL_SAI_MAXBURST_RX: c_int = 6;

// Max number of dataline

// default dataline type is zero

pub const FSL_SAI_AMIX_BYPASS: c_int = 0;
pub const FSL_SAI_AMIX_AUDMIX: c_int = 1;
pub const FSL_SAI_AMIX_NONE: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsl_sai_soc_data {
    pub use_imx_pcm: bool,
    pub use_edma: bool,
    pub mclk0_is_mclk1: bool,
    pub mclk_with_tere: bool,
    pub fifo_depth: c_uint,
    pub pins: c_uint,
    pub reg_offset: c_uint,
    pub flags: c_uint,
    pub max_register: c_uint,
    pub max_burst: [c_uint; 2],
}

//
// struct fsl_sai_verid - version id data
// @version: version number
// @feature: feature specification number
// 0000000000000000b - Standard feature set
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsl_sai_verid {
    pub version: u32,
    pub feature: u32,
}

//
// struct fsl_sai_param - parameter data
// @slot_num: The maximum number of slots per frame
// @fifo_depth: The number of words in each FIFO (depth)
// @dataline: The number of datalines implemented
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsl_sai_param {
    pub slot_num: u32,
    pub fifo_depth: u32,
    pub dataline: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsl_sai_dl_cfg {
    pub type: c_uint,
    pub pins: [c_uint; 2],
    pub mask: [c_uint; 2],
    pub start_off: [c_uint; 2],
    pub next_off: [c_uint; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsl_sai {
    pub pdev: *mut platform_device,
    pub regmap: *mut regmap,
    pub bus_clk: *mut clk,
    pub mclk_clk: [*mut clk; FSL_SAI_MCLK_MAX],
    pub pll8k_clk: *mut clk,
    pub pll11k_clk: *mut clk,
    pub res: *mut resource,
    pub is_consumer_mode: [bool; 2],
    pub is_lsb_first: bool,
    pub is_dsp_mode: [bool; 2],
    pub is_pdm_mode: bool,
    pub is_multi_fifo_dma: bool,
    pub synchronous: [bool; 2],
    pub dl_cfg: *mut fsl_sai_dl_cfg,
    pub dl_cfg_cnt: c_uint,
    pub mclk_direction_output: bool,
    pub is_bit_clock_swap: bool,
    pub mclk_id: [c_uint; 2],
    pub mclk_streams: c_uint,
    pub slots: [c_uint; 2],
    pub slot_width: [c_uint; 2],
    pub bclk_ratio: c_uint,
    pub soc_data: *const fsl_sai_soc_data,
    pub cpu_dai_drv: [snd_soc_dai_driver; 3],
    pub dma_params_rx: snd_dmaengine_dai_dma_data,
    pub dma_params_tx: snd_dmaengine_dai_dma_data,
    pub verid: fsl_sai_verid,
    pub param: fsl_sai_param,
    pub pm_qos_req: pm_qos_request,
    pub pinctrl: *mut pinctrl,
    pub pins_state: *mut pinctrl_state,
    pub audio_config: [sdma_peripheral_config; 2],
    pub constraint_rates: snd_pcm_hw_constraint_list,
    pub constraint_rates_list: [c_uint; FAL_SAI_NUM_RATES],
}

pub const TX: c_int = 1;
pub const RX: c_int = 0;
