//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/uniphier/aio.h
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
// Socionext UniPhier AIO ALSA driver.
//
// Copyright (c) 2016-2018 Socionext Inc.
//

// Macro flag: #define SND_UNIPHIER_AIO_H__

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ID_PORT_TYPE {
    PORT_TYPE_UNKNOWN,
    PORT_TYPE_I2S,
    PORT_TYPE_SPDIF,
    PORT_TYPE_EVE,
    PORT_TYPE_CONV,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ID_PORT_DIR {
    PORT_DIR_OUTPUT,
    PORT_DIR_INPUT,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum IEC61937_PC {
    IEC61937_PC_AC3   = 0x0001,
    IEC61937_PC_PAUSE = 0x0003,
    IEC61937_PC_MPA   = 0x0004,
    IEC61937_PC_MP3   = 0x0005,
    IEC61937_PC_DTS1  = 0x000b,
    IEC61937_PC_DTS2  = 0x000c,
    IEC61937_PC_DTS3  = 0x000d,
    IEC61937_PC_AAC   = 0x0007,
}

// IEC61937 Repetition period of data-burst in IEC60958 frames
pub const IEC61937_FRM_STR_AC3: c_int = 1536;
pub const IEC61937_FRM_STR_MPA: c_int = 1152;
pub const IEC61937_FRM_STR_MP3: c_int = 1152;
pub const IEC61937_FRM_STR_DTS1: c_int = 512;
pub const IEC61937_FRM_STR_DTS2: c_int = 1024;
pub const IEC61937_FRM_STR_DTS3: c_int = 2048;
pub const IEC61937_FRM_STR_AAC: c_int = 1024;
// IEC61937 Repetition period of Pause data-burst in IEC60958 frames
pub const IEC61937_FRM_PAU_AC3: c_int = 3;
pub const IEC61937_FRM_PAU_MPA: c_int = 32;
pub const IEC61937_FRM_PAU_MP3: c_int = 32;
pub const IEC61937_FRM_PAU_DTS1: c_int = 3;
pub const IEC61937_FRM_PAU_DTS2: c_int = 3;
pub const IEC61937_FRM_PAU_DTS3: c_int = 3;
pub const IEC61937_FRM_PAU_AAC: c_int = 32;
// IEC61937 Pa and Pb
pub const IEC61937_HEADER_SIGN: c_uint = 0x1f4e72f8;
pub const AUD_HW_PCMIN1: c_int = 0;
pub const AUD_HW_PCMIN2: c_int = 1;
pub const AUD_HW_PCMIN3: c_int = 2;
pub const AUD_HW_IECIN1: c_int = 3;
pub const AUD_HW_DIECIN1: c_int = 4;

pub const AUD_HW_HPCMOUT1: c_int = 0;
pub const AUD_HW_PCMOUT1: c_int = 1;
pub const AUD_HW_PCMOUT2: c_int = 2;
pub const AUD_HW_PCMOUT3: c_int = 3;
pub const AUD_HW_EPCMOUT1: c_int = 4;
pub const AUD_HW_EPCMOUT2: c_int = 5;
pub const AUD_HW_EPCMOUT3: c_int = 6;
pub const AUD_HW_EPCMOUT6: c_int = 9;
pub const AUD_HW_HIECOUT1: c_int = 10;
pub const AUD_HW_IECOUT1: c_int = 11;
pub const AUD_HW_CMASTER: c_int = 31;

pub const AUD_CLK_IO: c_int = 0;
pub const AUD_CLK_A1: c_int = 1;
pub const AUD_CLK_F1: c_int = 2;
pub const AUD_CLK_A2: c_int = 3;
pub const AUD_CLK_F2: c_int = 4;
pub const AUD_CLK_A: c_int = 5;
pub const AUD_CLK_F: c_int = 6;
pub const AUD_CLK_APLL: c_int = 7;
pub const AUD_CLK_RX0: c_int = 8;
pub const AUD_CLK_USB0: c_int = 9;
pub const AUD_CLK_HSC0: c_int = 10;
pub const AUD_PLL_A1: c_int = 0;
pub const AUD_PLL_F1: c_int = 1;
pub const AUD_PLL_A2: c_int = 2;
pub const AUD_PLL_F2: c_int = 3;
pub const AUD_PLL_APLL: c_int = 4;
pub const AUD_PLL_RX0: c_int = 5;
pub const AUD_PLL_USB0: c_int = 6;
pub const AUD_PLL_HSC0: c_int = 7;
pub const AUD_PLLDIV_1_2: c_int = 0;
pub const AUD_PLLDIV_1_3: c_int = 1;
pub const AUD_PLLDIV_1_1: c_int = 2;
pub const AUD_PLLDIV_2_3: c_int = 3;
pub const AUD_VOL_INIT: c_uint = 0x4000 /* +0dB */;
pub const AUD_VOL_MAX: c_uint = 0xffff /* +6dB */;

pub const AUD_MIN_FRAGMENT: c_int = 4;
pub const AUD_MAX_FRAGMENT: c_int = 8;

// max 5 slots, 10 channels, 2 channel in 1 slot
pub const AUD_MAX_SLOTSEL: c_int = 5;
//
// This is a selector for virtual register map of AIO.
//
// map:  Specify the index of virtual register map.
// hw :  Specify the ID of real register map, selector uses this value.
// A meaning of this value depends specification of SoC.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uniphier_aio_selector {
    pub map: c_int,
    pub hw: c_int,
}

//
// 'SoftWare MAPping' setting of UniPhier AIO registers.
//
// We have to setup 'virtual' register maps to access 'real' registers of AIO.
// This feature is legacy and meaningless but AIO needs this to work.
//
// Each hardware blocks have own virtual register maps as following:
//
// Address Virtual                      Real
// ------- ---------                    ---------------
// 0x12000 DMAC map0 --> [selector] --> DMAC hardware 3
// 0x12080 DMAC map1 --> [selector] --> DMAC hardware 1
// ...
// 0x42000 Port map0 --> [selector] --> Port hardware 1
// 0x42400 Port map1 --> [selector] --> Port hardware 2
// ...
//
// ch   : Input or output channel of DMAC
// rb   : Ring buffer
// iport: PCM input port
// iif  : Input interface
// oport: PCM output port
// oif  : Output interface
// och  : Output channel of DMAC for sampling rate converter
//
// These are examples for sound data paths:
//
// For caputure device:
// (outer of AIO) -> iport -> iif -> ch -> rb -> (CPU)
// For playback device:
// (CPU) -> rb -> ch -> oif -> oport -> (outer of AIO)
// For sampling rate converter device:
// (CPU) -> rb -> ch -> oif -> (HW SRC) -> iif -> och -> orb -> (CPU)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uniphier_aio_swmap {
    pub type: c_int,
    pub dir: c_int,
    pub ch: uniphier_aio_selector,
    pub rb: uniphier_aio_selector,
    pub iport: uniphier_aio_selector,
    pub iif: uniphier_aio_selector,
    pub oport: uniphier_aio_selector,
    pub oif: uniphier_aio_selector,
    pub och: uniphier_aio_selector,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uniphier_aio_spec {
    pub name: *const c_char,
    pub gname: *const c_char,
    pub swm: uniphier_aio_swmap,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uniphier_aio_pll {
    pub enable: bool,
    pub freq: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uniphier_aio_chip_spec {
    pub specs: *const uniphier_aio_spec,
    pub num_specs: c_int,
    pub plls: *const uniphier_aio_pll,
    pub num_plls: c_int,
    pub dais: *mut snd_soc_dai_driver,
    pub num_dais: c_int,
// DMA access mode, this is workaround for DMA hungup
    pub addr_ext: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uniphier_aio_sub {
    pub aio: *mut uniphier_aio,
// Guard sub->rd_offs and wr_offs from IRQ handler.
    pub lock: spinlock_t,
    pub swm: *const uniphier_aio_swmap,
    pub spec: *const uniphier_aio_spec,
// For PCM audio
    pub substream: *mut snd_pcm_substream,
    pub params: snd_pcm_hw_params,
    pub vol: c_int,
// For compress audio
    pub cstream: *mut snd_compr_stream,
    pub cparams: snd_compr_params,
    pub compr_area: *mut c_uchar,
    pub compr_addr: dma_addr_t,
    pub compr_bytes: usize,
    pub pass_through: c_int,
    pub iec_pc: IEC61937_PC,
    pub iec_header: bool,
// Both PCM and compress audio
    pub use_mmap: bool,
    pub setting: c_int,
    pub running: c_int,
    pub rd_offs: u64,
    pub wr_offs: u64,
    pub threshold: u32,
    pub rd_org: u64,
    pub wr_org: u64,
    pub rd_total: u64,
    pub wr_total: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uniphier_aio {
    pub chip: *mut uniphier_aio_chip,
    pub sub: [uniphier_aio_sub; 2],
    pub fmt: c_uint,
// Set one of AUD_CLK_X
    pub clk_in: c_int,
    pub clk_out: c_int,
// Set one of AUD_PLL_X
    pub pll_in: c_int,
    pub pll_out: c_int,
// Set one of AUD_PLLDIV_X
    pub plldiv: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uniphier_aio_chip {
    pub pdev: *mut platform_device,
    pub chip_spec: *const uniphier_aio_chip_spec,
    pub aios: *mut uniphier_aio,
    pub num_aios: c_int,
    pub num_wup_aios: c_int,
    pub plls: *mut uniphier_aio_pll,
    pub num_plls: c_int,
    pub clk: *mut clk,
    pub rst: *mut reset_control,
    pub regmap: *mut regmap,
    pub regmap_sg: *mut regmap,
    pub active: c_int,
}

extern "C" {
    pub fn uniphier_aiodma_soc_register_platform(pdev: *mut platform_device) -> c_int;
}
extern "C" {
    pub fn uniphier_aio_probe(pdev: *mut platform_device) -> c_int;
}
extern "C" {
    pub fn uniphier_aio_remove(pdev: *mut platform_device);
}
extern "C" {
    pub fn aio_rb_cnt(sub: *mut uniphier_aio_sub) -> u64;
}
extern "C" {
    pub fn aio_rbt_cnt_to_end(sub: *mut uniphier_aio_sub) -> u64;
}
extern "C" {
    pub fn aio_rb_space(sub: *mut uniphier_aio_sub) -> u64;
}
extern "C" {
    pub fn aio_rb_space_to_end(sub: *mut uniphier_aio_sub) -> u64;
}
extern "C" {
    pub fn aio_iecout_set_enable(chip: *mut uniphier_aio_chip, enable: bool);
}
extern "C" {
    pub fn aio_chip_init(chip: *mut uniphier_aio_chip);
}
extern "C" {
    pub fn aio_init(sub: *mut uniphier_aio_sub) -> c_int;
}
extern "C" {
    pub fn aio_port_reset(sub: *mut uniphier_aio_sub);
}
extern "C" {
    pub fn aio_port_set_enable(sub: *mut uniphier_aio_sub, enable: c_int);
}
extern "C" {
    pub fn aio_port_get_volume(sub: *mut uniphier_aio_sub) -> c_int;
}
extern "C" {
    pub fn aio_port_set_volume(sub: *mut uniphier_aio_sub, vol: c_int);
}
extern "C" {
    pub fn aio_if_set_param(sub: *mut uniphier_aio_sub, pass_through: c_int) -> c_int;
}
extern "C" {
    pub fn aio_src_reset(sub: *mut uniphier_aio_sub);
}
extern "C" {
    pub fn aio_srcif_set_param(sub: *mut uniphier_aio_sub) -> c_int;
}
extern "C" {
    pub fn aio_srcch_set_param(sub: *mut uniphier_aio_sub) -> c_int;
}
extern "C" {
    pub fn aio_srcch_set_enable(sub: *mut uniphier_aio_sub, enable: c_int);
}
extern "C" {
    pub fn aiodma_ch_set_param(sub: *mut uniphier_aio_sub) -> c_int;
}
extern "C" {
    pub fn aiodma_ch_set_enable(sub: *mut uniphier_aio_sub, enable: c_int);
}
extern "C" {
    pub fn aiodma_rb_set_threshold(sub: *mut uniphier_aio_sub, size: u64, th: u32) -> c_int;
}
extern "C" {
    pub fn aiodma_rb_is_irq(sub: *mut uniphier_aio_sub) -> bool;
}
extern "C" {
    pub fn aiodma_rb_clear_irq(sub: *mut uniphier_aio_sub);
}
