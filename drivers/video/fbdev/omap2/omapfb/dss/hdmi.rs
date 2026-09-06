//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/video/fbdev/omap2/omapfb/dss/hdmi.h
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
// HDMI driver definition for TI OMAP4 Processor.
//
// Copyright (C) 2010-2011 Texas Instruments Incorporated - https://www.ti.com
//

// HDMI Wrapper
pub const HDMI_WP_REVISION: c_uint = 0x0;
pub const HDMI_WP_SYSCONFIG: c_uint = 0x10;
pub const HDMI_WP_IRQSTATUS_RAW: c_uint = 0x24;
pub const HDMI_WP_IRQSTATUS: c_uint = 0x28;
pub const HDMI_WP_IRQENABLE_SET: c_uint = 0x2C;
pub const HDMI_WP_IRQENABLE_CLR: c_uint = 0x30;
pub const HDMI_WP_IRQWAKEEN: c_uint = 0x34;
pub const HDMI_WP_PWR_CTRL: c_uint = 0x40;
pub const HDMI_WP_DEBOUNCE: c_uint = 0x44;
pub const HDMI_WP_VIDEO_CFG: c_uint = 0x50;
pub const HDMI_WP_VIDEO_SIZE: c_uint = 0x60;
pub const HDMI_WP_VIDEO_TIMING_H: c_uint = 0x68;
pub const HDMI_WP_VIDEO_TIMING_V: c_uint = 0x6C;
pub const HDMI_WP_CLK: c_uint = 0x70;
pub const HDMI_WP_AUDIO_CFG: c_uint = 0x80;
pub const HDMI_WP_AUDIO_CFG2: c_uint = 0x84;
pub const HDMI_WP_AUDIO_CTRL: c_uint = 0x88;
pub const HDMI_WP_AUDIO_DATA: c_uint = 0x8C;
// HDMI WP IRQ flags

// HDMI PLL
pub const PLLCTRL_PLL_CONTROL: c_uint = 0x0;
pub const PLLCTRL_PLL_STATUS: c_uint = 0x4;
pub const PLLCTRL_PLL_GO: c_uint = 0x8;
pub const PLLCTRL_CFG1: c_uint = 0xC;
pub const PLLCTRL_CFG2: c_uint = 0x10;
pub const PLLCTRL_CFG3: c_uint = 0x14;
pub const PLLCTRL_SSC_CFG1: c_uint = 0x18;
pub const PLLCTRL_SSC_CFG2: c_uint = 0x1C;
pub const PLLCTRL_CFG4: c_uint = 0x20;
// HDMI PHY
pub const HDMI_TXPHY_TX_CTRL: c_uint = 0x0;
pub const HDMI_TXPHY_DIGITAL_CTRL: c_uint = 0x4;
pub const HDMI_TXPHY_POWER_CTRL: c_uint = 0x8;
pub const HDMI_TXPHY_PAD_CFG_CTRL: c_uint = 0xC;
pub const HDMI_TXPHY_BIST_CONTROL: c_uint = 0x1C;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hdmi_pll_pwr {
    HDMI_PLLPWRCMD_ALLOFF = 0,
    HDMI_PLLPWRCMD_PLLONLY = 1,
    HDMI_PLLPWRCMD_BOTHON_ALLCLKS = 2,
    HDMI_PLLPWRCMD_BOTHON_NOPHYCLK = 3
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hdmi_phy_pwr {
    HDMI_PHYPWRCMD_OFF = 0,
    HDMI_PHYPWRCMD_LDOON = 1,
    HDMI_PHYPWRCMD_TXON = 2
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hdmi_core_hdmi_dvi {
    HDMI_DVI = 0,
    HDMI_HDMI = 1
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hdmi_packing_mode {
    HDMI_PACK_10b_RGB_YUV444 = 0,
    HDMI_PACK_24b_RGB_YUV444_YUV422 = 1,
    HDMI_PACK_20b_YUV422 = 2,
    HDMI_PACK_ALREADYPACKED = 7
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hdmi_stereo_channels {
    HDMI_AUDIO_STEREO_NOCHANNELS = 0,
    HDMI_AUDIO_STEREO_ONECHANNEL = 1,
    HDMI_AUDIO_STEREO_TWOCHANNELS = 2,
    HDMI_AUDIO_STEREO_THREECHANNELS = 3,
    HDMI_AUDIO_STEREO_FOURCHANNELS = 4
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hdmi_audio_type {
    HDMI_AUDIO_TYPE_LPCM = 0,
    HDMI_AUDIO_TYPE_IEC = 1
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hdmi_audio_justify {
    HDMI_AUDIO_JUSTIFY_LEFT = 0,
    HDMI_AUDIO_JUSTIFY_RIGHT = 1
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hdmi_audio_sample_order {
    HDMI_AUDIO_SAMPLE_RIGHT_FIRST = 0,
    HDMI_AUDIO_SAMPLE_LEFT_FIRST = 1
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hdmi_audio_samples_perword {
    HDMI_AUDIO_ONEWORD_ONESAMPLE = 0,
    HDMI_AUDIO_ONEWORD_TWOSAMPLES = 1
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hdmi_audio_sample_size_omap {
    HDMI_AUDIO_SAMPLE_16BITS = 0,
    HDMI_AUDIO_SAMPLE_24BITS = 1
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hdmi_audio_transf_mode {
    HDMI_AUDIO_TRANSF_DMA = 0,
    HDMI_AUDIO_TRANSF_IRQ = 1
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hdmi_audio_blk_strt_end_sig {
    HDMI_AUDIO_BLOCK_SIG_STARTEND_ON = 0,
    HDMI_AUDIO_BLOCK_SIG_STARTEND_OFF = 1
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hdmi_core_audio_layout {
    HDMI_AUDIO_LAYOUT_2CH = 0,
    HDMI_AUDIO_LAYOUT_8CH = 1,
    HDMI_AUDIO_LAYOUT_6CH = 2
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hdmi_core_cts_mode {
    HDMI_AUDIO_CTS_MODE_HW = 0,
    HDMI_AUDIO_CTS_MODE_SW = 1
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hdmi_audio_mclk_mode {
    HDMI_AUDIO_MCLK_128FS = 0,
    HDMI_AUDIO_MCLK_256FS = 1,
    HDMI_AUDIO_MCLK_384FS = 2,
    HDMI_AUDIO_MCLK_512FS = 3,
    HDMI_AUDIO_MCLK_768FS = 4,
    HDMI_AUDIO_MCLK_1024FS = 5,
    HDMI_AUDIO_MCLK_1152FS = 6,
    HDMI_AUDIO_MCLK_192FS = 7
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hdmi_video_format {
    pub packing_mode: hdmi_packing_mode,
    pub /: *mut *mut u32 y_res; / Line per panel,
    pub /: *mut *mut u32 x_res; / pixel per line,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hdmi_config {
    pub timings: omap_video_timings,
    pub infoframe: hdmi_avi_infoframe,
    pub hdmi_dvi_mode: hdmi_core_hdmi_dvi,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hdmi_audio_format {
    pub stereo_channels: hdmi_stereo_channels,
    pub active_chnnls_msk: u8,
    pub type: hdmi_audio_type,
    pub justification: hdmi_audio_justify,
    pub sample_order: hdmi_audio_sample_order,
    pub samples_per_word: hdmi_audio_samples_perword,
    pub sample_size: hdmi_audio_sample_size_omap,
    pub en_sig_blk_strt_end: hdmi_audio_blk_strt_end_sig,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hdmi_audio_dma {
    pub transfer_size: u8,
    pub block_size: u8,
    pub mode: hdmi_audio_transf_mode,
    pub fifo_threshold: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hdmi_core_audio_i2s_config {
    pub in_length_bits: u8,
    pub justification: u8,
    pub sck_edge_mode: u8,
    pub vbit: u8,
    pub direction: u8,
    pub shift: u8,
    pub active_sds: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hdmi_core_audio_config {
    pub i2s_cfg: hdmi_core_audio_i2s_config,
    pub iec60958_cfg: *mut snd_aes_iec958,
    pub fs_override: bool,
    pub n: u32,
    pub cts: u32,
    pub aud_par_busclk: u32,
    pub layout: hdmi_core_audio_layout,
    pub cts_mode: hdmi_core_cts_mode,
    pub use_mclk: bool,
    pub mclk_mode: hdmi_audio_mclk_mode,
    pub en_acr_pkt: bool,
    pub en_dsd_audio: bool,
    pub en_parallel_aud_input: bool,
    pub en_spdif: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hdmi_wp_data {
    pub base: *mut void __iomem,
    pub phys_base: phys_addr_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hdmi_pll_data {
    pub pll: dss_pll,
    pub base: *mut void __iomem,
    pub wp: *mut hdmi_wp_data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hdmi_phy_data {
    pub base: *mut void __iomem,
    pub lane_function: [u8; 4],
    pub lane_polarity: [u8; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hdmi_core_data {
    pub base: *mut void __iomem,
}

extern "C" {
    pub fn __raw_readl(idx: base_addr +) -> return;
}

// HDMI wrapper funcs
extern "C" {
    pub fn hdmi_wp_video_start(wp: *mut hdmi_wp_data) -> c_int;
}
extern "C" {
    pub fn hdmi_wp_video_stop(wp: *mut hdmi_wp_data);
}
extern "C" {
    pub fn hdmi_wp_dump(wp: *mut hdmi_wp_data, s: *mut seq_file);
}
extern "C" {
    pub fn hdmi_wp_get_irqstatus(wp: *mut hdmi_wp_data) -> u32;
}
extern "C" {
    pub fn hdmi_wp_set_irqstatus(wp: *mut hdmi_wp_data, irqstatus: u32);
}
extern "C" {
    pub fn hdmi_wp_set_irqenable(wp: *mut hdmi_wp_data, mask: u32);
}
extern "C" {
    pub fn hdmi_wp_clear_irqenable(wp: *mut hdmi_wp_data, mask: u32);
}
extern "C" {
    pub fn hdmi_wp_set_phy_pwr(wp: *mut hdmi_wp_data, val: hdmi_phy_pwr) -> c_int;
}
extern "C" {
    pub fn hdmi_wp_set_pll_pwr(wp: *mut hdmi_wp_data, val: hdmi_pll_pwr) -> c_int;
}
extern "C" {
    pub fn hdmi_wp_init(pdev: *mut platform_device, wp: *mut hdmi_wp_data) -> c_int;
}
extern "C" {
    pub fn hdmi_wp_get_audio_dma_addr(wp: *mut hdmi_wp_data) -> phys_addr_t;
}
// HDMI PLL funcs
extern "C" {
    pub fn hdmi_pll_dump(pll: *mut hdmi_pll_data, s: *mut seq_file);
}
extern "C" {
    pub fn hdmi_pll_uninit(hpll: *mut hdmi_pll_data);
}
// HDMI PHY funcs
extern "C" {
    pub fn hdmi_phy_dump(phy: *mut hdmi_phy_data, s: *mut seq_file);
}
extern "C" {
    pub fn hdmi_phy_init(pdev: *mut platform_device, phy: *mut hdmi_phy_data) -> c_int;
}
extern "C" {
    pub fn hdmi_phy_parse_lanes(phy: *mut hdmi_phy_data, lanes: *const u32) -> c_int;
}
// HDMI common funcs
// Audio funcs
extern "C" {
    pub fn hdmi_compute_acr(pclk: u32, sample_freq: u32, n: *mut u32, cts: *mut u32) -> c_int;
}
extern "C" {
    pub fn hdmi_wp_audio_enable(wp: *mut hdmi_wp_data, enable: bool) -> c_int;
}
extern "C" {
    pub fn hdmi_wp_audio_core_req_enable(wp: *mut hdmi_wp_data, enable: bool) -> c_int;
}
// HDMI DRV data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct omap_hdmi {
    pub lock: mutex,
    pub pdev: *mut platform_device,
    pub wp: hdmi_wp_data,
    pub pll: hdmi_pll_data,
    pub phy: hdmi_phy_data,
    pub core: hdmi_core_data,
    pub cfg: hdmi_config,
    pub vdda_reg: *mut regulator,
    pub core_enabled: bool,
    pub output: omap_dss_device,
    pub audio_pdev: *mut platform_device,
    pub dev): *mut *mut void (audio_abort_cb)(struct device,
    pub wp_idlemode: c_int,
    pub audio_configured: bool,
    pub audio_config: omap_dss_audio,
// This lock should be taken when booleans below are touched.
    pub audio_playing_lock: spinlock_t,
    pub audio_playing: bool,
    pub display_enabled: bool,
}
