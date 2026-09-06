//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/sti/uniperif.h
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
// Copyright (C) STMicroelectronics SA 2015
// Authors: Arnaud Pouliquen <arnaud.pouliquen@st.com>
// for STMicroelectronics.
//

//
// Register access macros
//

//
// UNIPERIF_SOFT_RST reg
//
pub const UNIPERIF_SOFT_RST_OFFSET(ip): c_uint = 0x0000;

// SOFT_RST
pub const UNIPERIF_SOFT_RST_SOFT_RST_SHIFT(ip): c_uint = 0x0;
pub const UNIPERIF_SOFT_RST_SOFT_RST_MASK(ip): c_uint = 0x1;

//
// UNIPERIF_FIFO_DATA reg
//
pub const UNIPERIF_FIFO_DATA_OFFSET(ip): c_uint = 0x0004;

//
// UNIPERIF_CHANNEL_STA_REGN reg
//

pub const UNIPERIF_CHANNEL_STA_REG0_OFFSET(ip): c_uint = 0x0060;

pub const UNIPERIF_CHANNEL_STA_REG1_OFFSET(ip): c_uint = 0x0064;

pub const UNIPERIF_CHANNEL_STA_REG2_OFFSET(ip): c_uint = 0x0068;

pub const UNIPERIF_CHANNEL_STA_REG3_OFFSET(ip): c_uint = 0x006C;

pub const UNIPERIF_CHANNEL_STA_REG4_OFFSET(ip): c_uint = 0x0070;

pub const UNIPERIF_CHANNEL_STA_REG5_OFFSET(ip): c_uint = 0x0074;

//
// UNIPERIF_ITS reg
//
pub const UNIPERIF_ITS_OFFSET(ip): c_uint = 0x000C;

// MEM_BLK_READ
pub const UNIPERIF_ITS_MEM_BLK_READ_SHIFT(ip): c_int = 5;

// FIFO_ERROR

// DMA_ERROR
pub const UNIPERIF_ITS_DMA_ERROR_SHIFT(ip): c_int = 9;

// UNDERFLOW_REC_DONE

// UNDERFLOW_REC_FAILED

//
// UNIPERIF_ITS_BCLR reg
//
// FIFO_ERROR

pub const UNIPERIF_ITS_BCLR_OFFSET(ip): c_uint = 0x0010;

//
// UNIPERIF_ITM reg
//
pub const UNIPERIF_ITM_OFFSET(ip): c_uint = 0x0018;

// FIFO_ERROR

// UNDERFLOW_REC_DONE

// UNDERFLOW_REC_FAILED

//
// UNIPERIF_ITM_BCLR reg
//
pub const UNIPERIF_ITM_BCLR_OFFSET(ip): c_uint = 0x001c;

// FIFO_ERROR

// DMA_ERROR
pub const UNIPERIF_ITM_BCLR_DMA_ERROR_SHIFT(ip): c_int = 9;

//
// UNIPERIF_ITM_BSET reg
//
pub const UNIPERIF_ITM_BSET_OFFSET(ip): c_uint = 0x0020;

// FIFO_ERROR

// MEM_BLK_READ
pub const UNIPERIF_ITM_BSET_MEM_BLK_READ_SHIFT(ip): c_int = 5;

// DMA_ERROR
pub const UNIPERIF_ITM_BSET_DMA_ERROR_SHIFT(ip): c_int = 9;

// UNDERFLOW_REC_DONE

// UNDERFLOW_REC_FAILED

//
// UNIPERIF_CONFIG reg
//
pub const UNIPERIF_CONFIG_OFFSET(ip): c_uint = 0x0040;

// PARITY_CNTR
pub const UNIPERIF_CONFIG_PARITY_CNTR_SHIFT(ip): c_int = 0;
pub const UNIPERIF_CONFIG_PARITY_CNTR_MASK(ip): c_uint = 0x1;

// CHANNEL_STA_CNTR
pub const UNIPERIF_CONFIG_CHANNEL_STA_CNTR_SHIFT(ip): c_int = 1;
pub const UNIPERIF_CONFIG_CHANNEL_STA_CNTR_MASK(ip): c_uint = 0x1;

// USER_DAT_CNTR
pub const UNIPERIF_CONFIG_USER_DAT_CNTR_SHIFT(ip): c_int = 2;
pub const UNIPERIF_CONFIG_USER_DAT_CNTR_MASK(ip): c_uint = 0x1;

// VALIDITY_DAT_CNTR
pub const UNIPERIF_CONFIG_VALIDITY_DAT_CNTR_SHIFT(ip): c_int = 3;
pub const UNIPERIF_CONFIG_VALIDITY_DAT_CNTR_MASK(ip): c_uint = 0x1;

// ONE_BIT_AUD_SUPPORT
pub const UNIPERIF_CONFIG_ONE_BIT_AUD_SHIFT(ip): c_int = 4;
pub const UNIPERIF_CONFIG_ONE_BIT_AUD_MASK(ip): c_uint = 0x1;

// MEMORY_FMT
pub const UNIPERIF_CONFIG_MEM_FMT_SHIFT(ip): c_int = 5;
pub const UNIPERIF_CONFIG_MEM_FMT_MASK(ip): c_uint = 0x1;
pub const VALUE_UNIPERIF_CONFIG_MEM_FMT_16_0(ip): c_int = 0;
pub const VALUE_UNIPERIF_CONFIG_MEM_FMT_16_16(ip): c_int = 1;

// REPEAT_CHL_STS
pub const UNIPERIF_CONFIG_REPEAT_CHL_STS_SHIFT(ip): c_int = 6;
pub const UNIPERIF_CONFIG_REPEAT_CHL_STS_MASK(ip): c_uint = 0x1;

// BACK_STALL_REQ

pub const UNIPERIF_CONFIG_BACK_STALL_REQ_MASK(ip): c_uint = 0x1;

// FDMA_TRIGGER_LIMIT
pub const UNIPERIF_CONFIG_DMA_TRIG_LIMIT_SHIFT(ip): c_int = 8;
pub const UNIPERIF_CONFIG_DMA_TRIG_LIMIT_MASK(ip): c_uint = 0x7F;

// CHL_STS_UPDATE

pub const UNIPERIF_CONFIG_CHL_STS_UPDATE_MASK(ip): c_uint = 0x1;

// IDLE_MOD
pub const UNIPERIF_CONFIG_IDLE_MOD_SHIFT(ip): c_int = 18;
pub const UNIPERIF_CONFIG_IDLE_MOD_MASK(ip): c_uint = 0x1;

// SUBFRAME_SELECTION
pub const UNIPERIF_CONFIG_SUBFRAME_SEL_SHIFT(ip): c_int = 19;
pub const UNIPERIF_CONFIG_SUBFRAME_SEL_MASK(ip): c_uint = 0x1;

// FULL_SW_CONTROL
pub const UNIPERIF_CONFIG_SPDIF_SW_CTRL_SHIFT(ip): c_int = 20;
pub const UNIPERIF_CONFIG_SPDIF_SW_CTRL_MASK(ip): c_uint = 0x1;

// MASTER_CLKEDGE

pub const UNIPERIF_CONFIG_MSTR_CLKEDGE_MASK(ip): c_uint = 0x1;

//
// UNIPERIF_CTRL reg
//
pub const UNIPERIF_CTRL_OFFSET(ip): c_uint = 0x0044;

// OPERATION
pub const UNIPERIF_CTRL_OPERATION_SHIFT(ip): c_int = 0;
pub const UNIPERIF_CTRL_OPERATION_MASK(ip): c_uint = 0x7;

pub const VALUE_UNIPERIF_CTRL_OPERATION_OFF(ip): c_int = 0;

pub const VALUE_UNIPERIF_CTRL_OPERATION_PCM_DATA(ip): c_int = 3;

// This is the same as above!
pub const VALUE_UNIPERIF_CTRL_OPERATION_AUDIO_DATA(ip): c_int = 3;

pub const VALUE_UNIPERIF_CTRL_OPERATION_ENC_DATA(ip): c_int = 4;

// EXIT_STBY_ON_EOBLOCK

pub const UNIPERIF_CTRL_EXIT_STBY_ON_EOBLOCK_MASK(ip): c_uint = 0x1;

// ROUNDING
pub const UNIPERIF_CTRL_ROUNDING_SHIFT(ip): c_int = 4;
pub const UNIPERIF_CTRL_ROUNDING_MASK(ip): c_uint = 0x1;

// DIVIDER
pub const UNIPERIF_CTRL_DIVIDER_SHIFT(ip): c_int = 5;
pub const UNIPERIF_CTRL_DIVIDER_MASK(ip): c_uint = 0xff;

// BYTE_SWAP

pub const UNIPERIF_CTRL_BYTE_SWP_MASK(ip): c_uint = 0x1;

// ZERO_STUFFING_HW_SW

pub const UNIPERIF_CTRL_ZERO_STUFF_MASK(ip): c_uint = 0x1;

// SPDIF_LAT

pub const UNIPERIF_CTRL_SPDIF_LAT_MASK(ip): c_uint = 0x1;

// EN_SPDIF_FORMATTING
pub const UNIPERIF_CTRL_SPDIF_FMT_SHIFT(ip): c_int = 17;
pub const UNIPERIF_CTRL_SPDIF_FMT_MASK(ip): c_uint = 0x1;

// READER_OUT_SELECT

pub const UNIPERIF_CTRL_READER_OUT_SEL_MASK(ip): c_uint = 0x1;

// UNDERFLOW_REC_WINDOW
pub const UNIPERIF_CTRL_UNDERFLOW_REC_WINDOW_SHIFT(ip): c_int = 20;
pub const UNIPERIF_CTRL_UNDERFLOW_REC_WINDOW_MASK(ip): c_uint = 0xff;

//
// UNIPERIF_I2S_FMT a.k.a UNIPERIF_FORMAT reg
//
pub const UNIPERIF_I2S_FMT_OFFSET(ip): c_uint = 0x0048;

// NBIT
pub const UNIPERIF_I2S_FMT_NBIT_SHIFT(ip): c_int = 0;
pub const UNIPERIF_I2S_FMT_NBIT_MASK(ip): c_uint = 0x1;

// DATA_SIZE
pub const UNIPERIF_I2S_FMT_DATA_SIZE_SHIFT(ip): c_int = 1;
pub const UNIPERIF_I2S_FMT_DATA_SIZE_MASK(ip): c_uint = 0x7;

// LR_POL
pub const UNIPERIF_I2S_FMT_LR_POL_SHIFT(ip): c_int = 4;
pub const UNIPERIF_I2S_FMT_LR_POL_MASK(ip): c_uint = 0x1;
pub const VALUE_UNIPERIF_I2S_FMT_LR_POL_LOW(ip): c_uint = 0x0;
pub const VALUE_UNIPERIF_I2S_FMT_LR_POL_HIG(ip): c_uint = 0x1;

// SCLK_EDGE
pub const UNIPERIF_I2S_FMT_SCLK_EDGE_SHIFT(ip): c_int = 5;
pub const UNIPERIF_I2S_FMT_SCLK_EDGE_MASK(ip): c_uint = 0x1;

// PADDING
pub const UNIPERIF_I2S_FMT_PADDING_SHIFT(ip): c_int = 6;
pub const UNIPERIF_I2S_FMT_PADDING_MASK(ip): c_uint = 0x1;
pub const UNIPERIF_I2S_FMT_PADDING_MASK(ip): c_uint = 0x1;
pub const VALUE_UNIPERIF_I2S_FMT_PADDING_I2S_MODE(ip): c_uint = 0x0;
pub const VALUE_UNIPERIF_I2S_FMT_PADDING_SONY_MODE(ip): c_uint = 0x1;

// ALIGN
pub const UNIPERIF_I2S_FMT_ALIGN_SHIFT(ip): c_int = 7;
pub const UNIPERIF_I2S_FMT_ALIGN_MASK(ip): c_uint = 0x1;

// ORDER
pub const UNIPERIF_I2S_FMT_ORDER_SHIFT(ip): c_int = 8;
pub const UNIPERIF_I2S_FMT_ORDER_MASK(ip): c_uint = 0x1;

// NUM_CH
pub const UNIPERIF_I2S_FMT_NUM_CH_SHIFT(ip): c_int = 9;
pub const UNIPERIF_I2S_FMT_NUM_CH_MASK(ip): c_uint = 0x7;

// NO_OF_SAMPLES_TO_READ
pub const UNIPERIF_I2S_FMT_NO_OF_SAMPLES_TO_READ_SHIFT(ip): c_int = 12;
pub const UNIPERIF_I2S_FMT_NO_OF_SAMPLES_TO_READ_MASK(ip): c_uint = 0xfffff;

//
// UNIPERIF_BIT_CONTROL reg
//

// CLR_UNDERFLOW_DURATION
pub const UNIPERIF_BIT_CONTROL_CLR_UNDERFLOW_DURATION_SHIFT(ip): c_int = 0;
pub const UNIPERIF_BIT_CONTROL_CLR_UNDERFLOW_DURATION_MASK(ip): c_uint = 0x1;

// CHL_STS_UPDATE
pub const UNIPERIF_BIT_CONTROL_CHL_STS_UPDATE_SHIFT(ip): c_int = 1;
pub const UNIPERIF_BIT_CONTROL_CHL_STS_UPDATE_MASK(ip): c_uint = 0x1;

//
// UNIPERIF_STATUS_1 reg
//
pub const UNIPERIF_STATUS_1_OFFSET(ip): c_uint = 0x0050;

// UNDERFLOW_DURATION

pub const UNIPERIF_STATUS_1_UNDERFLOW_DURATION_MASK(ip): c_uint = 0xff;

//
// UNIPERIF_CHANNEL_STA_REGN reg
//

//
// UNIPERIF_USER_VALIDITY reg
//
pub const UNIPERIF_USER_VALIDITY_OFFSET(ip): c_uint = 0x0090;

// VALIDITY_LEFT_AND_RIGHT
pub const UNIPERIF_USER_VALIDITY_VALIDITY_LR_SHIFT(ip): c_int = 0;
pub const UNIPERIF_USER_VALIDITY_VALIDITY_LR_MASK(ip): c_uint = 0x3;

//
// UNIPERIF_DBG_STANDBY_LEFT_SP reg
//
pub const UNIPERIF_DBG_STANDBY_LEFT_SP_OFFSET(ip): c_uint = 0x0150;

//
// UNIPERIF_TDM_ENABLE
//
pub const UNIPERIF_TDM_ENABLE_OFFSET(ip): c_uint = 0x0118;

// TDM_ENABLE
pub const UNIPERIF_TDM_ENABLE_EN_TDM_SHIFT(ip): c_uint = 0x0;
pub const UNIPERIF_TDM_ENABLE_EN_TDM_MASK(ip): c_uint = 0x1;

//
// UNIPERIF_TDM_FS_REF_FREQ
//
pub const UNIPERIF_TDM_FS_REF_FREQ_OFFSET(ip): c_uint = 0x011c;

// REF_FREQ
pub const UNIPERIF_TDM_FS_REF_FREQ_REF_FREQ_SHIFT(ip): c_uint = 0x0;
pub const VALUE_UNIPERIF_TDM_FS_REF_FREQ_8KHZ(ip): c_int = 0;
pub const VALUE_UNIPERIF_TDM_FS_REF_FREQ_16KHZ(ip): c_int = 1;
pub const VALUE_UNIPERIF_TDM_FS_REF_FREQ_32KHZ(ip): c_int = 2;
pub const VALUE_UNIPERIF_TDM_FS_REF_FREQ_48KHZ(ip): c_int = 3;
pub const UNIPERIF_TDM_FS_REF_FREQ_REF_FREQ_MASK(ip): c_uint = 0x3;

//
// UNIPERIF_TDM_FS_REF_DIV
//
pub const UNIPERIF_TDM_FS_REF_DIV_OFFSET(ip): c_uint = 0x0120;

// NUM_TIMESLOT
pub const UNIPERIF_TDM_FS_REF_DIV_NUM_TIMESLOT_SHIFT(ip): c_uint = 0x0;
pub const UNIPERIF_TDM_FS_REF_DIV_NUM_TIMESLOT_MASK(ip): c_uint = 0xff;

//
// UNIPERIF_TDM_WORD_POS_X_Y
// 32 bits of UNIPERIF_TDM_WORD_POS_X_Y register shall be set in 1 shot
//
pub const UNIPERIF_TDM_WORD_POS_1_2_OFFSET(ip): c_uint = 0x013c;
pub const UNIPERIF_TDM_WORD_POS_3_4_OFFSET(ip): c_uint = 0x0140;
pub const UNIPERIF_TDM_WORD_POS_5_6_OFFSET(ip): c_uint = 0x0144;
pub const UNIPERIF_TDM_WORD_POS_7_8_OFFSET(ip): c_uint = 0x0148;

//
// uniperipheral IP capabilities
//

//
// Uniperipheral IP revisions
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum uniperif_version {
    SND_ST_UNIPERIF_VERSION_UNKNOWN,
// SASG1 (Orly), Newman
    SND_ST_UNIPERIF_VERSION_C6AUD0_UNI_1_0,
// SASC1, SASG2 (Orly2)
    SND_ST_UNIPERIF_VERSION_UNI_PLR_1_0,
// SASC1, SASG2 (Orly2), TELSS, Cannes
    SND_ST_UNIPERIF_VERSION_UNI_RDR_1_0,
// TELSS (SASC1)
    SND_ST_UNIPERIF_VERSION_TDM_PLR_1_0,
// Cannes/Monaco
    SND_ST_UNIPERIF_VERSION_UNI_PLR_TOP_1_0
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum uniperif_type {
    SND_ST_UNIPERIF_TYPE_NONE	= 0x00,
    SND_ST_UNIPERIF_TYPE_HDMI	= 0x01,
    SND_ST_UNIPERIF_TYPE_PCM	= 0x02,
    SND_ST_UNIPERIF_TYPE_SPDIF	= 0x04,
    SND_ST_UNIPERIF_TYPE_TDM	= 0x08
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum uniperif_state {
    UNIPERIF_STATE_STOPPED,
    UNIPERIF_STATE_STARTED,
    UNIPERIF_STATE_STANDBY,
    UNIPERIF_STATE_UNDERFLOW,
    UNIPERIF_STATE_OVERFLOW = UNIPERIF_STATE_UNDERFLOW,
    UNIPERIF_STATE_XRUN
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum uniperif_iec958_encoding_mode {
    UNIPERIF_IEC958_ENCODING_MODE_PCM,
    UNIPERIF_IEC958_ENCODING_MODE_ENCODED
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum uniperif_word_pos {
    WORD_1_2,
    WORD_3_4,
    WORD_5_6,
    WORD_7_8,
    WORD_MAX
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uniperif_iec958_settings {
    pub encoding_mode: uniperif_iec958_encoding_mode,
    pub iec958: snd_aes_iec958,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dai_tdm_slot {
    pub mask: c_uint,
    pub slots: c_int,
    pub slot_width: c_int,
    pub avail_slots: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uniperif {
// System information
    pub type: uniperif_type,
    pub /: *mut *mut int underflow_enabled; / Underflow recovery mode,
    pub dev: *mut device,
    pub /: *mut *mut int id; / instance value of the uniperipheral IP,
    pub /: *mut *mut int ver; / IP version, used by register access macros,
    pub clk_sel: *mut regmap_field,
    pub valid_sel: *mut regmap_field,
    pub /: *mut *mut spinlock_t irq_lock; / use to prevent race condition with IRQ,
// capabilities
    pub hw: *const snd_pcm_hardware,
// Resources
    pub mem_region: *mut resource,
    pub base: *mut void __iomem,
    pub fifo_phys_address: c_ulong,
    pub irq: c_int,
// Clocks
    pub clk: *mut clk,
    pub mclk: c_int,
    pub clk_adj: c_int,
// Runtime data
    pub state: uniperif_state,
    pub substream: *mut snd_pcm_substream,
// Specific to IEC958 player
    pub stream_settings: uniperif_iec958_settings,
    pub controls*/: *mut *mut mutex ctrl_lock; / For resource updated by stream and,
// alsa ctrl
    pub snd_ctrls: *mut snd_kcontrol_new,
    pub num_ctrls: c_int,
// dai properties
    pub daifmt: c_uint,
    pub tdm_slot: dai_tdm_slot,
// DAI callbacks
    pub dai_ops: *const snd_soc_dai_ops,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sti_uniperiph_dai {
    pub stream: c_int,
    pub uni: *mut uniperif,
    pub dma_data: snd_dmaengine_dai_dma_data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sti_uniperiph_data {
    pub pdev: *mut platform_device,
    pub dai: *mut snd_soc_dai_driver,
    pub dai_data: sti_uniperiph_dai,
}

// uniperiph player
extern "C" {
    pub fn uni_player_resume(player: *mut uniperif) -> c_int;
}
// uniperiph reader
// common
extern "C" {
    pub fn sti_uniperiph_dai_probe(dai: *mut snd_soc_dai) -> c_int;
}
extern "C" {
    pub fn sti_uniperiph_reset(uni: *mut uniperif) -> c_int;
}
