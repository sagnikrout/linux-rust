//! Automatically rewritten from C Header to Rust Module
//! Source: sound/x86/intel_hdmi_lpe_audio.h
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
// intel_hdmi_lpe_audio.h - Intel HDMI LPE audio driver
//
// Copyright (C) 2016 Intel Corp
// Authors:	Sailaja Bandarupalli <sailaja.bandarupalli@intel.com>
// Ramesh Babu K V <ramesh.babu@intel.com>
// Vaibhav Agarwal <vaibhav.agarwal@intel.com>
// Jerome Anand <jerome.anand@intel.com>
// Aravind Siddappaji <aravindx.siddappaji@intel.com>
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
//
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
//
pub const HAD_MIN_CHANNEL: c_int = 2;
pub const HAD_MAX_CHANNEL: c_int = 8;
pub const HAD_NUM_OF_RING_BUFS: c_int = 4;
// max 20bit address, aligned to 64

pub const HAD_MIN_PERIODS: c_int = 1;

pub const MAX_SPEAKERS: c_int = 8;
pub const AUD_SAMPLE_RATE_32: c_int = 32000;
pub const AUD_SAMPLE_RATE_44_1: c_int = 44100;
pub const AUD_SAMPLE_RATE_48: c_int = 48000;
pub const AUD_SAMPLE_RATE_88_2: c_int = 88200;
pub const AUD_SAMPLE_RATE_96: c_int = 96000;
pub const AUD_SAMPLE_RATE_176_4: c_int = 176400;
pub const AUD_SAMPLE_RATE_192: c_int = 192000;

pub const DIS_SAMPLE_RATE_25_2: c_int = 25200;
pub const DIS_SAMPLE_RATE_27: c_int = 27000;
pub const DIS_SAMPLE_RATE_54: c_int = 54000;
pub const DIS_SAMPLE_RATE_74_25: c_int = 74250;
pub const DIS_SAMPLE_RATE_148_5: c_int = 148500;
pub const HAD_REG_WIDTH: c_uint = 0x08;
pub const HAD_MAX_DIP_WORDS: c_int = 16;
// DP Link Rates
pub const DP_2_7_GHZ: c_int = 270000;
pub const DP_1_62_GHZ: c_int = 162000;
// Maud Values
pub const AUD_SAMPLE_RATE_32_DP_2_7_MAUD_VAL: c_int = 1988;
pub const AUD_SAMPLE_RATE_44_1_DP_2_7_MAUD_VAL: c_int = 2740;
pub const AUD_SAMPLE_RATE_48_DP_2_7_MAUD_VAL: c_int = 2982;
pub const AUD_SAMPLE_RATE_88_2_DP_2_7_MAUD_VAL: c_int = 5480;
pub const AUD_SAMPLE_RATE_96_DP_2_7_MAUD_VAL: c_int = 5965;
pub const AUD_SAMPLE_RATE_176_4_DP_2_7_MAUD_VAL: c_int = 10961;
pub const HAD_MAX_RATE_DP_2_7_MAUD_VAL: c_int = 11930;
pub const AUD_SAMPLE_RATE_32_DP_1_62_MAUD_VAL: c_int = 3314;
pub const AUD_SAMPLE_RATE_44_1_DP_1_62_MAUD_VAL: c_int = 4567;
pub const AUD_SAMPLE_RATE_48_DP_1_62_MAUD_VAL: c_int = 4971;
pub const AUD_SAMPLE_RATE_88_2_DP_1_62_MAUD_VAL: c_int = 9134;
pub const AUD_SAMPLE_RATE_96_DP_1_62_MAUD_VAL: c_int = 9942;
pub const AUD_SAMPLE_RATE_176_4_DP_1_62_MAUD_VAL: c_int = 18268;
pub const HAD_MAX_RATE_DP_1_62_MAUD_VAL: c_int = 19884;
// Naud Value
pub const DP_NAUD_VAL: c_int = 32768;
// HDMI Controller register offsets - audio domain common
// Base address for below regs = 0x65000
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hdmi_ctrl_reg_offset_common {
    AUDIO_HDMI_CONFIG_A = 0x000,
    AUDIO_HDMI_CONFIG_B = 0x800,
    AUDIO_HDMI_CONFIG_C = 0x900,
}

// HDMI controller register offsets
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hdmi_ctrl_reg_offset {
    AUD_CONFIG		= 0x0,
    AUD_CH_STATUS_0		= 0x08,
    AUD_CH_STATUS_1		= 0x0C,
    AUD_HDMI_CTS		= 0x10,
    AUD_N_ENABLE		= 0x14,
    AUD_SAMPLE_RATE		= 0x18,
    AUD_BUF_CONFIG		= 0x20,
    AUD_BUF_CH_SWAP		= 0x24,
    AUD_BUF_A_ADDR		= 0x40,
    AUD_BUF_A_LENGTH	= 0x44,
    AUD_BUF_B_ADDR		= 0x48,
    AUD_BUF_B_LENGTH	= 0x4c,
    AUD_BUF_C_ADDR		= 0x50,
    AUD_BUF_C_LENGTH	= 0x54,
    AUD_BUF_D_ADDR		= 0x58,
    AUD_BUF_D_LENGTH	= 0x5c,
    AUD_CNTL_ST		= 0x60,
    AUD_HDMI_STATUS		= 0x64, /* v2 */
    AUD_HDMIW_INFOFR	= 0x68, /* v2 */
}

// Audio configuration
#[repr(C)]
#[derive(Copy, Clone)]
pub union aud_cfg {
    pub aud_en:1: u32,
    pub /: *mut *mut u32 layout:1; / LAYOUT[01], see below,
    pub fmt:2: u32,
    pub num_ch:3: u32,
    pub set:1: u32,
    pub flat:1: u32,
    pub val_bit:1: u32,
    pub user_bit:1: u32,
    pub packets,: *mut *mut u32 underrun:1; / 0: send null,
// 1: send silence stream
//
    pub /: *mut *mut u32 packet_mode:1; / 0: 32bit container, 1: 16bit,
    pub /: *mut *mut u32 left_align:1; / 0: MSB bits 0-23, 1: bits 8-31,
    pub /: *mut *mut u32 bogus_sample:1; / bogus sample for odd channels,
    pub /: *mut *mut u32 dp_modei:1; / 0: HDMI, 1: DP,
    pub rsvd:16: u32,
    pub regx: },
    pub regval: u32,
}

pub const AUD_CONFIG_CH_MASK: c_uint = 0x70;

// Audio Channel Status 0 Attributes
#[repr(C)]
#[derive(Copy, Clone)]
pub union aud_ch_status_0 {
    pub ch_status:1: u32,
    pub lpcm_id:1: u32,
    pub cp_info:1: u32,
    pub format:3: u32,
    pub mode:2: u32,
    pub ctg_code:8: u32,
    pub src_num:4: u32,
    pub ch_num:4: u32,
    pub /: *mut *mut u32 samp_freq:4; / CH_STATUS_MAP_XXX,
    pub clk_acc:2: u32,
    pub rsvd:2: u32,
    pub regx: },
    pub regval: u32,
}

// samp_freq values - Sampling rate as per IEC60958 Ver 3
pub const CH_STATUS_MAP_32KHZ: c_uint = 0x3;
pub const CH_STATUS_MAP_44KHZ: c_uint = 0x0;
pub const CH_STATUS_MAP_48KHZ: c_uint = 0x2;
pub const CH_STATUS_MAP_88KHZ: c_uint = 0x8;
pub const CH_STATUS_MAP_96KHZ: c_uint = 0xA;
pub const CH_STATUS_MAP_176KHZ: c_uint = 0xC;
pub const CH_STATUS_MAP_192KHZ: c_uint = 0xE;
// Audio Channel Status 1 Attributes
#[repr(C)]
#[derive(Copy, Clone)]
pub union aud_ch_status_1 {
    pub max_wrd_len:1: u32,
    pub wrd_len:3: u32,
    pub rsvd:28: u32,
    pub regx: },
    pub regval: u32,
}

pub const MAX_SMPL_WIDTH_20: c_uint = 0x0;
pub const MAX_SMPL_WIDTH_24: c_uint = 0x1;
pub const SMPL_WIDTH_16BITS: c_uint = 0x1;
pub const SMPL_WIDTH_24BITS: c_uint = 0x5;
// CTS register
#[repr(C)]
#[derive(Copy, Clone)]
pub union aud_hdmi_cts {
    pub cts_val:24: u32,
    pub en_cts_prog:1: u32,
    pub rsvd:7: u32,
    pub regx: },
    pub regval: u32,
}

// N register
#[repr(C)]
#[derive(Copy, Clone)]
pub union aud_hdmi_n_enable {
    pub n_val:24: u32,
    pub en_n_prog:1: u32,
    pub rsvd:7: u32,
    pub regx: },
    pub regval: u32,
}

// Audio Buffer configurations
#[repr(C)]
#[derive(Copy, Clone)]
pub union aud_buf_config {
    pub audio_fifo_watermark:8: u32,
    pub dma_fifo_watermark:3: u32,
    pub rsvd0:5: u32,
    pub aud_delay:8: u32,
    pub rsvd1:8: u32,
    pub regx: },
    pub regval: u32,
}

pub const FIFO_THRESHOLD: c_uint = 0xFE;
pub const DMA_FIFO_THRESHOLD: c_uint = 0x7;
// Audio Sample Swapping offset
#[repr(C)]
#[derive(Copy, Clone)]
pub union aud_buf_ch_swap {
    pub first_0:3: u32,
    pub second_0:3: u32,
    pub first_1:3: u32,
    pub second_1:3: u32,
    pub first_2:3: u32,
    pub second_2:3: u32,
    pub first_3:3: u32,
    pub second_3:3: u32,
    pub rsvd:8: u32,
    pub regx: },
    pub regval: u32,
}

pub const SWAP_LFE_CENTER: c_uint = 0x00fac4c8	/* octal 76543210 */;
// Address for Audio Buffer
#[repr(C)]
#[derive(Copy, Clone)]
pub union aud_buf_addr {
    pub valid:1: u32,
    pub intr_en:1: u32,
    pub rsvd:4: u32,
    pub addr:26: u32,
    pub regx: },
    pub regval: u32,
}

// Length of Audio Buffer
#[repr(C)]
#[derive(Copy, Clone)]
pub union aud_buf_len {
    pub buf_len:20: u32,
    pub rsvd:12: u32,
    pub regx: },
    pub regval: u32,
}

// Audio Control State Register offset
#[repr(C)]
#[derive(Copy, Clone)]
pub union aud_ctrl_st {
    pub ram_addr:4: u32,
    pub eld_ack:1: u32,
    pub eld_addr:4: u32,
    pub eld_buf_size:5: u32,
    pub eld_valid:1: u32,
    pub cp_ready:1: u32,
    pub dip_freq:2: u32,
    pub dip_idx:3: u32,
    pub dip_en_sta:4: u32,
    pub rsvd:7: u32,
    pub regx: },
    pub regval: u32,
}

// Audio HDMI Widget Data Island Packet offset
#[repr(C)]
#[derive(Copy, Clone)]
pub union aud_info_frame1 {
    pub pkt_type:8: u32,
    pub ver_num:8: u32,
    pub len:5: u32,
    pub rsvd:11: u32,
    pub regx: },
    pub regval: u32,
}

pub const HDMI_INFO_FRAME_WORD1: c_uint = 0x000a0184;
pub const DP_INFO_FRAME_WORD1: c_uint = 0x00441b84;
// DIP frame 2
#[repr(C)]
#[derive(Copy, Clone)]
pub union aud_info_frame2 {
    pub chksum:8: u32,
    pub chnl_cnt:3: u32,
    pub rsvd0:1: u32,
    pub coding_type:4: u32,
    pub smpl_size:2: u32,
    pub smpl_freq:3: u32,
    pub rsvd1:3: u32,
    pub format:8: u32,
    pub regx: },
    pub regval: u32,
}

// DIP frame 3
#[repr(C)]
#[derive(Copy, Clone)]
pub union aud_info_frame3 {
    pub chnl_alloc:8: u32,
    pub rsvd0:3: u32,
    pub lsv:4: u32,
    pub dm_inh:1: u32,
    pub rsvd1:16: u32,
    pub regx: },
    pub regval: u32,
}

pub const VALID_DIP_WORDS: c_int = 3;
// AUD_HDMI_STATUS bits

// AUD_HDMI_STATUS register mask
pub const AUD_HDMI_STATUS_MASK_UNDERRUN: c_uint = 0xC0000000;
pub const AUD_HDMI_STATUS_MASK_SRDBG: c_uint = 0x00000002;
pub const AUD_HDMI_STATUSG_MASK_FUNCRST: c_uint = 0x00000001;
