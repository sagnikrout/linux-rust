//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/fsl/fsl_easrc.h
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
// Copyright (C) 2019 NXP
//

// EASRC Register Map
// ASRC Input Write FIFO

// ASRC Output Read FIFO

// ASRC Context Control

// ASRC Context Control Extended 1

// ASRC Context Control Extended 2

// ASRC Control Input Access

// ASRC Datapath Processor Control Slot0

// ASRC Datapath Processor Control Slot1

// ASRC Context Output Control

// ASRC Control Output Access

// ASRC Sample FIFO Status

// ASRC Resampling Ratio Low

// ASRC Resampling Ratio High

// ASRC Resampling Ratio Update Control

// ASRC Resampling Ratio Update Rate

// ASRC Resampling Center Tap Coefficient Low

// ASRC Resampling Center Tap Coefficient High

// ASRC Prefilter Coefficient FIFO

// ASRC Context Resampling Coefficient Memory
pub const REG_EASRC_CRCM: c_uint = 0x170;
// ASRC Context Resampling Coefficient Control
pub const REG_EASRC_CRCC: c_uint = 0x174;
// ASRC Interrupt Control
pub const REG_EASRC_IRQC: c_uint = 0x178;
// ASRC Interrupt Status Flags
pub const REG_EASRC_IRQF: c_uint = 0x17C;
// ASRC Channel Status 0

// ASRC Channel Status 1

// ASRC Channel Status 2

// ASRC Channel Status 3

// ASRC Channel Status 4

// ASRC Channel Status 5

// ASRC Debug Control Register
pub const REG_EASRC_DBGC: c_uint = 0x1E0;
// ASRC Debug Status Register
pub const REG_EASRC_DBGS: c_uint = 0x1E4;

// ASRC Context Control (CC)
pub const EASRC_CC_EN_SHIFT: c_int = 31;

pub const EASRC_CC_STOP_SHIFT: c_int = 29;

pub const EASRC_CC_FWMDE_SHIFT: c_int = 28;

pub const EASRC_CC_FIFO_WTMK_SHIFT: c_int = 16;
pub const EASRC_CC_FIFO_WTMK_WIDTH: c_int = 7;

pub const EASRC_CC_SAMPLE_POS_SHIFT: c_int = 11;
pub const EASRC_CC_SAMPLE_POS_WIDTH: c_int = 5;

pub const EASRC_CC_ENDIANNESS_SHIFT: c_int = 10;

pub const EASRC_CC_BPS_SHIFT: c_int = 8;
pub const EASRC_CC_BPS_WIDTH: c_int = 2;

pub const EASRC_CC_FMT_SHIFT: c_int = 7;

pub const EASRC_CC_INSIGN_SHIFT: c_int = 6;

pub const EASRC_CC_CHEN_SHIFT: c_int = 0;
pub const EASRC_CC_CHEN_WIDTH: c_int = 5;

// ASRC Context Control Extended 1 (CCE1)
pub const EASRC_CCE1_COEF_WS_SHIFT: c_int = 25;

pub const EASRC_CCE1_COEF_MEM_RST_SHIFT: c_int = 24;

pub const EASRC_CCE1_PF_EXP_SHIFT: c_int = 16;
pub const EASRC_CCE1_PF_EXP_WIDTH: c_int = 8;

pub const EASRC_CCE1_PF_ST1_WBFP_SHIFT: c_int = 9;

pub const EASRC_CCE1_PF_TSEN_SHIFT: c_int = 8;

pub const EASRC_CCE1_RS_BYPASS_SHIFT: c_int = 7;

pub const EASRC_CCE1_PF_BYPASS_SHIFT: c_int = 6;

pub const EASRC_CCE1_RS_STOP_SHIFT: c_int = 5;

pub const EASRC_CCE1_PF_STOP_SHIFT: c_int = 4;

pub const EASRC_CCE1_RS_INIT_SHIFT: c_int = 2;
pub const EASRC_CCE1_RS_INIT_WIDTH: c_int = 2;

pub const EASRC_CCE1_PF_INIT_SHIFT: c_int = 0;
pub const EASRC_CCE1_PF_INIT_WIDTH: c_int = 2;

// ASRC Context Control Extended 2 (CCE2)
pub const EASRC_CCE2_ST2_TAPS_SHIFT: c_int = 16;
pub const EASRC_CCE2_ST2_TAPS_WIDTH: c_int = 9;

pub const EASRC_CCE2_ST1_TAPS_SHIFT: c_int = 0;
pub const EASRC_CCE2_ST1_TAPS_WIDTH: c_int = 9;

// ASRC Control Input Access (CIA)
pub const EASRC_CIA_ITER_SHIFT: c_int = 16;
pub const EASRC_CIA_ITER_WIDTH: c_int = 6;

pub const EASRC_CIA_GRLEN_SHIFT: c_int = 8;
pub const EASRC_CIA_GRLEN_WIDTH: c_int = 6;

pub const EASRC_CIA_ACCLEN_SHIFT: c_int = 0;
pub const EASRC_CIA_ACCLEN_WIDTH: c_int = 6;

// ASRC Datapath Processor Control Slot0 Register0 (DPCS0R0)
pub const EASRC_DPCS0R0_MAXCH_SHIFT: c_int = 24;
pub const EASRC_DPCS0R0_MAXCH_WIDTH: c_int = 5;

pub const EASRC_DPCS0R0_MINCH_SHIFT: c_int = 16;
pub const EASRC_DPCS0R0_MINCH_WIDTH: c_int = 5;

pub const EASRC_DPCS0R0_NUMCH_SHIFT: c_int = 8;
pub const EASRC_DPCS0R0_NUMCH_WIDTH: c_int = 5;

pub const EASRC_DPCS0R0_CTXNUM_SHIFT: c_int = 1;
pub const EASRC_DPCS0R0_CTXNUM_WIDTH: c_int = 2;

pub const EASRC_DPCS0R0_EN_SHIFT: c_int = 0;

// ASRC Datapath Processor Control Slot0 Register1 (DPCS0R1)
pub const EASRC_DPCS0R1_ST1_EXP_SHIFT: c_int = 0;
pub const EASRC_DPCS0R1_ST1_EXP_WIDTH: c_int = 13;

// ASRC Datapath Processor Control Slot0 Register2 (DPCS0R2)
pub const EASRC_DPCS0R2_ST1_MA_SHIFT: c_int = 16;
pub const EASRC_DPCS0R2_ST1_MA_WIDTH: c_int = 13;

pub const EASRC_DPCS0R2_ST1_SA_SHIFT: c_int = 0;
pub const EASRC_DPCS0R2_ST1_SA_WIDTH: c_int = 13;

// ASRC Datapath Processor Control Slot0 Register3 (DPCS0R3)
pub const EASRC_DPCS0R3_ST2_MA_SHIFT: c_int = 16;
pub const EASRC_DPCS0R3_ST2_MA_WIDTH: c_int = 13;

pub const EASRC_DPCS0R3_ST2_SA_SHIFT: c_int = 0;
pub const EASRC_DPCS0R3_ST2_SA_WIDTH: c_int = 13;

// ASRC Context Output Control (COC)
pub const EASRC_COC_FWMDE_SHIFT: c_int = 28;

pub const EASRC_COC_FIFO_WTMK_SHIFT: c_int = 16;
pub const EASRC_COC_FIFO_WTMK_WIDTH: c_int = 7;

pub const EASRC_COC_SAMPLE_POS_SHIFT: c_int = 11;
pub const EASRC_COC_SAMPLE_POS_WIDTH: c_int = 5;

pub const EASRC_COC_ENDIANNESS_SHIFT: c_int = 10;

pub const EASRC_COC_BPS_SHIFT: c_int = 8;
pub const EASRC_COC_BPS_WIDTH: c_int = 2;

pub const EASRC_COC_FMT_SHIFT: c_int = 7;

pub const EASRC_COC_OUTSIGN_SHIFT: c_int = 6;

pub const EASRC_COC_IEC_VDATA_SHIFT: c_int = 2;

pub const EASRC_COC_IEC_EN_SHIFT: c_int = 1;

pub const EASRC_COC_DITHER_EN_SHIFT: c_int = 0;

// ASRC Control Output Access (COA)
pub const EASRC_COA_ITER_SHIFT: c_int = 16;
pub const EASRC_COA_ITER_WIDTH: c_int = 6;

pub const EASRC_COA_GRLEN_SHIFT: c_int = 8;
pub const EASRC_COA_GRLEN_WIDTH: c_int = 6;

pub const EASRC_COA_ACCLEN_SHIFT: c_int = 0;
pub const EASRC_COA_ACCLEN_WIDTH: c_int = 6;

// ASRC Sample FIFO Status (SFS)
pub const EASRC_SFS_IWTMK_SHIFT: c_int = 23;

pub const EASRC_SFS_NSGI_SHIFT: c_int = 16;
pub const EASRC_SFS_NSGI_WIDTH: c_int = 7;

pub const EASRC_SFS_OWTMK_SHIFT: c_int = 7;

pub const EASRC_SFS_NSGO_SHIFT: c_int = 0;
pub const EASRC_SFS_NSGO_WIDTH: c_int = 7;

// ASRC Resampling Ratio Low (RRL)
pub const EASRC_RRL_RS_RL_SHIFT: c_int = 0;
pub const EASRC_RRL_RS_RL_WIDTH: c_int = 32;

// ASRC Resampling Ratio High (RRH)
pub const EASRC_RRH_RS_VLD_SHIFT: c_int = 31;

pub const EASRC_RRH_RS_RH_SHIFT: c_int = 0;
pub const EASRC_RRH_RS_RH_WIDTH: c_int = 12;

// ASRC Resampling Ratio Update Control (RSUC)
pub const EASRC_RSUC_RS_RM_SHIFT: c_int = 0;
pub const EASRC_RSUC_RS_RM_WIDTH: c_int = 32;

// ASRC Resampling Ratio Update Rate (RRUR)
pub const EASRC_RRUR_RRR_SHIFT: c_int = 0;
pub const EASRC_RRUR_RRR_WIDTH: c_int = 31;

// ASRC Resampling Center Tap Coefficient Low (RCTCL)
pub const EASRC_RCTCL_RS_CL_SHIFT: c_int = 0;
pub const EASRC_RCTCL_RS_CL_WIDTH: c_int = 32;

// ASRC Resampling Center Tap Coefficient High (RCTCH)
pub const EASRC_RCTCH_RS_CH_SHIFT: c_int = 0;
pub const EASRC_RCTCH_RS_CH_WIDTH: c_int = 32;

// ASRC Prefilter Coefficient FIFO (PCF)
pub const EASRC_PCF_CD_SHIFT: c_int = 0;
pub const EASRC_PCF_CD_WIDTH: c_int = 32;

// ASRC Context Resampling Coefficient Memory (CRCM)
pub const EASRC_CRCM_RS_CWD_SHIFT: c_int = 0;
pub const EASRC_CRCM_RS_CWD_WIDTH: c_int = 32;

// ASRC Context Resampling Coefficient Control (CRCC)
pub const EASRC_CRCC_RS_CA_SHIFT: c_int = 16;
pub const EASRC_CRCC_RS_CA_WIDTH: c_int = 11;

pub const EASRC_CRCC_RS_TAPS_SHIFT: c_int = 1;
pub const EASRC_CRCC_RS_TAPS_WIDTH: c_int = 2;

pub const EASRC_CRCC_RS_CPR_SHIFT: c_int = 0;

// ASRC Interrupt_Control (IC)
pub const EASRC_IRQC_RSDM_SHIFT: c_int = 8;
pub const EASRC_IRQC_RSDM_WIDTH: c_int = 4;

pub const EASRC_IRQC_OERM_SHIFT: c_int = 4;
pub const EASRC_IRQC_OERM_WIDTH: c_int = 4;

pub const EASRC_IRQC_IOM_SHIFT: c_int = 0;
pub const EASRC_IRQC_IOM_WIDTH: c_int = 4;

// ASRC Interrupt Status Flags (ISF)
pub const EASRC_IRQF_RSD_SHIFT: c_int = 8;
pub const EASRC_IRQF_RSD_WIDTH: c_int = 4;

pub const EASRC_IRQF_OER_SHIFT: c_int = 4;
pub const EASRC_IRQF_OER_WIDTH: c_int = 4;

pub const EASRC_IRQF_IFO_SHIFT: c_int = 0;
pub const EASRC_IRQF_IFO_WIDTH: c_int = 4;

// ASRC Context Channel STAT
pub const EASRC_CSx_CSx_SHIFT: c_int = 0;
pub const EASRC_CSx_CSx_WIDTH: c_int = 32;

// ASRC Debug Control Register
pub const EASRC_DBGC_DMS_SHIFT: c_int = 0;
pub const EASRC_DBGC_DMS_WIDTH: c_int = 6;

// ASRC Debug Status Register
pub const EASRC_DBGS_DS_SHIFT: c_int = 0;
pub const EASRC_DBGS_DS_WIDTH: c_int = 32;

// General Constants
pub const EASRC_CTX_MAX_NUM: c_int = 4;
pub const EASRC_RS_COEFF_MEM: c_int = 0;
pub const EASRC_PF_COEFF_MEM: c_int = 1;
// Prefilter constants
pub const EASRC_PF_ST1_ONLY: c_int = 0;
pub const EASRC_PF_TWO_STAGE_MODE: c_int = 1;
pub const EASRC_PF_ST1_COEFF_WR: c_int = 0;
pub const EASRC_PF_ST2_COEFF_WR: c_int = 1;
pub const EASRC_MAX_PF_TAPS: c_int = 384;
// Resampling constants
pub const EASRC_RS_32_TAPS: c_int = 0;
pub const EASRC_RS_64_TAPS: c_int = 1;
pub const EASRC_RS_128_TAPS: c_int = 2;
// Initialization mode
pub const EASRC_INIT_MODE_SW_CONTROL: c_int = 0;
pub const EASRC_INIT_MODE_REPLICATE: c_int = 1;
pub const EASRC_INIT_MODE_ZERO_FILL: c_int = 2;
// FIFO watermarks
pub const FSL_EASRC_INPUTFIFO_WML: c_uint = 0x4;
pub const FSL_EASRC_OUTPUTFIFO_WML: c_uint = 0x1;
pub const EASRC_INPUTFIFO_THRESHOLD_MIN: c_int = 0;
pub const EASRC_INPUTFIFO_THRESHOLD_MAX: c_int = 127;
pub const EASRC_OUTPUTFIFO_THRESHOLD_MIN: c_int = 0;
pub const EASRC_OUTPUTFIFO_THRESHOLD_MAX: c_int = 63;

pub const FIRMWARE_MAGIC: c_uint = 0xDEAD;
pub const FIRMWARE_VERSION: c_int = 1;
pub const PREFILTER_MEM_LEN: c_uint = 0x1800;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum easrc_word_width {
    EASRC_WIDTH_16_BIT = 0,
    EASRC_WIDTH_20_BIT = 1,
    EASRC_WIDTH_24_BIT = 2,
    EASRC_WIDTH_32_BIT = 3,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dma_block {
    pub dma_vaddr: *mut c_void,
    pub length: c_uint,
    pub max_buf_size: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsl_easrc_data_fmt {
    pub 2: unsigned int width :,
    pub 1: unsigned int endianness :,
    pub 1: unsigned int unsign :,
    pub 1: unsigned int floating_point :,
    pub 1: unsigned int iec958:,
    pub 5: unsigned int sample_pos:,
    pub addexp: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsl_easrc_io_params {
    pub fmt: fsl_easrc_data_fmt,
    pub group_len: c_uint,
    pub iterations: c_uint,
    pub access_len: c_uint,
    pub fifo_wtmk: c_uint,
    pub sample_rate: c_uint,
    pub sample_format: snd_pcm_format_t,
    pub norm_rate: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsl_easrc_slot {
    pub busy: bool,
    pub ctx_index: c_int,
    pub slot_index: c_int,
    pub /: *mut *mut int num_channel; / maximum is 8,
    pub min_channel: c_int,
    pub max_channel: c_int,
    pub pf_mem_used: c_int,
}

//
// struct fsl_easrc_ctx_priv - EASRC context private data
//
// @in_params: input parameter
// @out_params:  output parameter
// @st1_num_taps: tap number of stage 1
// @st2_num_taps: tap number of stage 2
// @st1_num_exp: exponent number of stage 1
// @pf_init_mode: prefilter init mode
// @rs_init_mode:  resample filter init mode
// @ctx_streams: stream flag of ctx
// @rs_ratio: resampler ratio
// @st1_coeff: pointer of stage 1 coeff
// @st2_coeff: pointer of stage 2 coeff
// @in_filled_sample: input filled sample
// @out_missed_sample: sample missed in output
// @st1_addexp: exponent added for stage1
// @st2_addexp: exponent added for stage2
// @ratio_mod: update ratio
// @in_filled_len: input filled length
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsl_easrc_ctx_priv {
    pub in_params: fsl_easrc_io_params,
    pub out_params: fsl_easrc_io_params,
    pub st1_num_taps: c_uint,
    pub st2_num_taps: c_uint,
    pub st1_num_exp: c_uint,
    pub pf_init_mode: c_uint,
    pub rs_init_mode: c_uint,
    pub ctx_streams: c_uint,
    pub rs_ratio: u64,
    pub st1_coeff: *mut u64,
    pub st2_coeff: *mut u64,
    pub in_filled_sample: c_int,
    pub out_missed_sample: c_int,
    pub st1_addexp: c_int,
    pub st2_addexp: c_int,
    pub ratio_mod: c_int,
    pub in_filled_len: c_uint,
}

//
// struct fsl_easrc_priv - EASRC private data
//
// @slot: slot setting
// @firmware_hdr:  the header of firmware
// @interp: pointer to interpolation filter coeff
// @prefil: pointer to prefilter coeff
// @fw: firmware of coeff table
// @fw_name: firmware name
// @rs_num_taps:  resample filter taps, 32, 64, or 128
// @bps_iec958: bits per sample of iec958
// @rs_coeff: resampler coefficient
// @const_coeff: one tap prefilter coefficient
// @firmware_loaded: firmware is loaded
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsl_easrc_priv {
    pub slot: [fsl_easrc_slot; EASRC_CTX_MAX_NUM][2],
    pub firmware_hdr: *mut asrc_firmware_hdr,
    pub interp: *mut interp_params,
    pub prefil: *mut prefil_params,
    pub fw: *const firmware,
    pub fw_name: *const c_char,
    pub rs_num_taps: c_uint,
    pub bps_iec958: [c_uint; EASRC_CTX_MAX_NUM],
    pub rs_coeff: *mut u64,
    pub const_coeff: u64,
    pub firmware_loaded: c_int,
}
