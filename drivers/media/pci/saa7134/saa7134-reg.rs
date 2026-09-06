//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/pci/saa7134/saa7134-reg.h
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
// philips saa7134 registers
//
// ------------------------------------------------------------------
//
// PCI ID's
//

// ------------------------------------------------------------------
//
// registers -- 32 bit
//
// DMA channels, n = 0 ... 6

// main control

// DMA status

// audio / video status

// interrupt

// ------------------------------------------------------------------
//
// registers -- 8 bit
//
// video decoder
pub const SAA7134_INCR_DELAY: c_uint = 0x101;
pub const SAA7134_ANALOG_IN_CTRL1: c_uint = 0x102;
pub const SAA7134_ANALOG_IN_CTRL2: c_uint = 0x103;
pub const SAA7134_ANALOG_IN_CTRL3: c_uint = 0x104;
pub const SAA7134_ANALOG_IN_CTRL4: c_uint = 0x105;
pub const SAA7134_HSYNC_START: c_uint = 0x106;
pub const SAA7134_HSYNC_STOP: c_uint = 0x107;
pub const SAA7134_SYNC_CTRL: c_uint = 0x108;

pub const SAA7134_LUMA_CTRL: c_uint = 0x109;

pub const SAA7134_DEC_LUMA_BRIGHT: c_uint = 0x10a;
pub const SAA7134_DEC_LUMA_CONTRAST: c_uint = 0x10b;
pub const SAA7134_DEC_CHROMA_SATURATION: c_uint = 0x10c;
pub const SAA7134_DEC_CHROMA_HUE: c_uint = 0x10d;
pub const SAA7134_CHROMA_CTRL1: c_uint = 0x10e;

pub const SAA7134_CHROMA_GAIN: c_uint = 0x10f;
pub const SAA7134_CHROMA_CTRL2: c_uint = 0x110;
pub const SAA7134_MODE_DELAY_CTRL: c_uint = 0x111;
pub const SAA7134_ANALOG_ADC: c_uint = 0x114;

pub const SAA7134_VGATE_START: c_uint = 0x115;
pub const SAA7134_VGATE_STOP: c_uint = 0x116;
pub const SAA7134_MISC_VGATE_MSB: c_uint = 0x117;
pub const SAA7134_RAW_DATA_GAIN: c_uint = 0x118;
pub const SAA7134_RAW_DATA_OFFSET: c_uint = 0x119;
pub const SAA7134_STATUS_VIDEO1: c_uint = 0x11e;
pub const SAA7134_STATUS_VIDEO2: c_uint = 0x11f;
// video scaler
pub const SAA7134_SOURCE_TIMING1: c_uint = 0x000;
pub const SAA7134_SOURCE_TIMING2: c_uint = 0x001;
pub const SAA7134_REGION_ENABLE: c_uint = 0x004;
pub const SAA7134_SCALER_STATUS0: c_uint = 0x006;
pub const SAA7134_SCALER_STATUS1: c_uint = 0x007;
pub const SAA7134_START_GREEN: c_uint = 0x00c;
pub const SAA7134_START_BLUE: c_uint = 0x00d;
pub const SAA7134_START_RED: c_uint = 0x00e;

pub const TASK_A: c_uint = 0x040;
pub const TASK_B: c_uint = 0x080;

// clipping & dma
pub const SAA7134_OFMT_VIDEO_A: c_uint = 0x300;
pub const SAA7134_OFMT_DATA_A: c_uint = 0x301;
pub const SAA7134_OFMT_VIDEO_B: c_uint = 0x302;
pub const SAA7134_OFMT_DATA_B: c_uint = 0x303;
pub const SAA7134_ALPHA_NOCLIP: c_uint = 0x304;
pub const SAA7134_ALPHA_CLIP: c_uint = 0x305;
pub const SAA7134_UV_PIXEL: c_uint = 0x308;
pub const SAA7134_CLIP_RED: c_uint = 0x309;
pub const SAA7134_CLIP_GREEN: c_uint = 0x30a;
pub const SAA7134_CLIP_BLUE: c_uint = 0x30b;
// i2c bus
pub const SAA7134_I2C_ATTR_STATUS: c_uint = 0x180;
pub const SAA7134_I2C_DATA: c_uint = 0x181;
pub const SAA7134_I2C_CLOCK_SELECT: c_uint = 0x182;
pub const SAA7134_I2C_TIMER: c_uint = 0x183;
// audio
pub const SAA7134_NICAM_ADD_DATA1: c_uint = 0x140;
pub const SAA7134_NICAM_ADD_DATA2: c_uint = 0x141;
pub const SAA7134_NICAM_STATUS: c_uint = 0x142;
pub const SAA7134_AUDIO_STATUS: c_uint = 0x143;
pub const SAA7134_NICAM_ERROR_COUNT: c_uint = 0x144;
pub const SAA7134_IDENT_SIF: c_uint = 0x145;
pub const SAA7134_LEVEL_READOUT1: c_uint = 0x146;
pub const SAA7134_LEVEL_READOUT2: c_uint = 0x147;
pub const SAA7134_NICAM_ERROR_LOW: c_uint = 0x148;
pub const SAA7134_NICAM_ERROR_HIGH: c_uint = 0x149;
pub const SAA7134_DCXO_IDENT_CTRL: c_uint = 0x14a;
pub const SAA7134_DEMODULATOR: c_uint = 0x14b;
pub const SAA7134_AGC_GAIN_SELECT: c_uint = 0x14c;
pub const SAA7134_CARRIER1_FREQ0: c_uint = 0x150;
pub const SAA7134_CARRIER1_FREQ1: c_uint = 0x151;
pub const SAA7134_CARRIER1_FREQ2: c_uint = 0x152;
pub const SAA7134_CARRIER2_FREQ0: c_uint = 0x154;
pub const SAA7134_CARRIER2_FREQ1: c_uint = 0x155;
pub const SAA7134_CARRIER2_FREQ2: c_uint = 0x156;
pub const SAA7134_NUM_SAMPLES0: c_uint = 0x158;
pub const SAA7134_NUM_SAMPLES1: c_uint = 0x159;
pub const SAA7134_NUM_SAMPLES2: c_uint = 0x15a;
pub const SAA7134_AUDIO_FORMAT_CTRL: c_uint = 0x15b;
pub const SAA7134_MONITOR_SELECT: c_uint = 0x160;
pub const SAA7134_FM_DEEMPHASIS: c_uint = 0x161;
pub const SAA7134_FM_DEMATRIX: c_uint = 0x162;
pub const SAA7134_CHANNEL1_LEVEL: c_uint = 0x163;
pub const SAA7134_CHANNEL2_LEVEL: c_uint = 0x164;
pub const SAA7134_NICAM_CONFIG: c_uint = 0x165;
pub const SAA7134_NICAM_LEVEL_ADJUST: c_uint = 0x166;
pub const SAA7134_STEREO_DAC_OUTPUT_SELECT: c_uint = 0x167;
pub const SAA7134_I2S_OUTPUT_FORMAT: c_uint = 0x168;
pub const SAA7134_I2S_OUTPUT_SELECT: c_uint = 0x169;
pub const SAA7134_I2S_OUTPUT_LEVEL: c_uint = 0x16a;
pub const SAA7134_DSP_OUTPUT_SELECT: c_uint = 0x16b;
pub const SAA7134_AUDIO_MUTE_CTRL: c_uint = 0x16c;
pub const SAA7134_SIF_SAMPLE_FREQ: c_uint = 0x16d;
pub const SAA7134_ANALOG_IO_SELECT: c_uint = 0x16e;
pub const SAA7134_AUDIO_CLOCK0: c_uint = 0x170;
pub const SAA7134_AUDIO_CLOCK1: c_uint = 0x171;
pub const SAA7134_AUDIO_CLOCK2: c_uint = 0x172;
pub const SAA7134_AUDIO_PLL_CTRL: c_uint = 0x173;
pub const SAA7134_AUDIO_CLOCKS_PER_FIELD0: c_uint = 0x174;
pub const SAA7134_AUDIO_CLOCKS_PER_FIELD1: c_uint = 0x175;
pub const SAA7134_AUDIO_CLOCKS_PER_FIELD2: c_uint = 0x176;
// video port output
pub const SAA7134_VIDEO_PORT_CTRL0: c_uint = 0x190;
pub const SAA7134_VIDEO_PORT_CTRL1: c_uint = 0x191;
pub const SAA7134_VIDEO_PORT_CTRL2: c_uint = 0x192;
pub const SAA7134_VIDEO_PORT_CTRL3: c_uint = 0x193;
pub const SAA7134_VIDEO_PORT_CTRL4: c_uint = 0x194;
pub const SAA7134_VIDEO_PORT_CTRL5: c_uint = 0x195;
pub const SAA7134_VIDEO_PORT_CTRL6: c_uint = 0x196;
pub const SAA7134_VIDEO_PORT_CTRL7: c_uint = 0x197;
pub const SAA7134_VIDEO_PORT_CTRL8: c_uint = 0x198;
// transport stream interface
pub const SAA7134_TS_PARALLEL: c_uint = 0x1a0;
pub const SAA7134_TS_PARALLEL_SERIAL: c_uint = 0x1a1;
pub const SAA7134_TS_SERIAL0: c_uint = 0x1a2;
pub const SAA7134_TS_SERIAL1: c_uint = 0x1a3;
pub const SAA7134_TS_DMA0: c_uint = 0x1a4;
pub const SAA7134_TS_DMA1: c_uint = 0x1a5;
pub const SAA7134_TS_DMA2: c_uint = 0x1a6;
// GPIO Controls
pub const SAA7134_GPIO_GPRESCAN: c_uint = 0x80;
pub const SAA7134_GPIO_27_25: c_uint = 0x0E;
pub const SAA7134_GPIO_GPMODE0: c_uint = 0x1B0;
pub const SAA7134_GPIO_GPMODE1: c_uint = 0x1B1;
pub const SAA7134_GPIO_GPMODE2: c_uint = 0x1B2;
pub const SAA7134_GPIO_GPMODE3: c_uint = 0x1B3;
pub const SAA7134_GPIO_GPSTATUS0: c_uint = 0x1B4;
pub const SAA7134_GPIO_GPSTATUS1: c_uint = 0x1B5;
pub const SAA7134_GPIO_GPSTATUS2: c_uint = 0x1B6;
pub const SAA7134_GPIO_GPSTATUS3: c_uint = 0x1B7;
// I2S output
pub const SAA7134_I2S_AUDIO_OUTPUT: c_uint = 0x1c0;
// test modes
pub const SAA7134_SPECIAL_MODE: c_uint = 0x1d0;
pub const SAA7134_PRODUCTION_TEST_MODE: c_uint = 0x1d1;
// audio -- saa7133 + saa7135 only
pub const SAA7135_DSP_RWSTATE: c_uint = 0x580;

pub const SAA7135_DSP_RWCLEAR: c_uint = 0x586;
pub const SAA7135_DSP_RWCLEAR_RERR: c_int = 1;
pub const SAA7133_I2S_AUDIO_CONTROL: c_uint = 0x591;
