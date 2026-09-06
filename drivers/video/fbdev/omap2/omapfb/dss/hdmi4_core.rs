//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/video/fbdev/omap2/omapfb/dss/hdmi4_core.h
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
// HDMI header definition for OMAP4 HDMI core IP
//
// Copyright (C) 2010-2011 Texas Instruments Incorporated - https://www.ti.com
//

// OMAP4 HDMI IP Core System
pub const HDMI_CORE_SYS_VND_IDL: c_uint = 0x0;
pub const HDMI_CORE_SYS_DEV_IDL: c_uint = 0x8;
pub const HDMI_CORE_SYS_DEV_IDH: c_uint = 0xC;
pub const HDMI_CORE_SYS_DEV_REV: c_uint = 0x10;
pub const HDMI_CORE_SYS_SRST: c_uint = 0x14;
pub const HDMI_CORE_SYS_SYS_CTRL1: c_uint = 0x20;
pub const HDMI_CORE_SYS_SYS_STAT: c_uint = 0x24;
pub const HDMI_CORE_SYS_SYS_CTRL3: c_uint = 0x28;
pub const HDMI_CORE_SYS_DCTL: c_uint = 0x34;
pub const HDMI_CORE_SYS_DE_DLY: c_uint = 0xC8;
pub const HDMI_CORE_SYS_DE_CTRL: c_uint = 0xCC;
pub const HDMI_CORE_SYS_DE_TOP: c_uint = 0xD0;
pub const HDMI_CORE_SYS_DE_CNTL: c_uint = 0xD8;
pub const HDMI_CORE_SYS_DE_CNTH: c_uint = 0xDC;
pub const HDMI_CORE_SYS_DE_LINL: c_uint = 0xE0;
pub const HDMI_CORE_SYS_DE_LINH_1: c_uint = 0xE4;
pub const HDMI_CORE_SYS_HRES_L: c_uint = 0xE8;
pub const HDMI_CORE_SYS_HRES_H: c_uint = 0xEC;
pub const HDMI_CORE_SYS_VRES_L: c_uint = 0xF0;
pub const HDMI_CORE_SYS_VRES_H: c_uint = 0xF4;
pub const HDMI_CORE_SYS_IADJUST: c_uint = 0xF8;
pub const HDMI_CORE_SYS_POLDETECT: c_uint = 0xFC;
pub const HDMI_CORE_SYS_HWIDTH1: c_uint = 0x110;
pub const HDMI_CORE_SYS_HWIDTH2: c_uint = 0x114;
pub const HDMI_CORE_SYS_VWIDTH: c_uint = 0x11C;
pub const HDMI_CORE_SYS_VID_CTRL: c_uint = 0x120;
pub const HDMI_CORE_SYS_VID_ACEN: c_uint = 0x124;
pub const HDMI_CORE_SYS_VID_MODE: c_uint = 0x128;
pub const HDMI_CORE_SYS_VID_BLANK1: c_uint = 0x12C;
pub const HDMI_CORE_SYS_VID_BLANK2: c_uint = 0x130;
pub const HDMI_CORE_SYS_VID_BLANK3: c_uint = 0x134;
pub const HDMI_CORE_SYS_DC_HEADER: c_uint = 0x138;
pub const HDMI_CORE_SYS_VID_DITHER: c_uint = 0x13C;
pub const HDMI_CORE_SYS_RGB2XVYCC_CT: c_uint = 0x140;
pub const HDMI_CORE_SYS_R2Y_COEFF_LOW: c_uint = 0x144;
pub const HDMI_CORE_SYS_R2Y_COEFF_UP: c_uint = 0x148;
pub const HDMI_CORE_SYS_G2Y_COEFF_LOW: c_uint = 0x14C;
pub const HDMI_CORE_SYS_G2Y_COEFF_UP: c_uint = 0x150;
pub const HDMI_CORE_SYS_B2Y_COEFF_LOW: c_uint = 0x154;
pub const HDMI_CORE_SYS_B2Y_COEFF_UP: c_uint = 0x158;
pub const HDMI_CORE_SYS_R2CB_COEFF_LOW: c_uint = 0x15C;
pub const HDMI_CORE_SYS_R2CB_COEFF_UP: c_uint = 0x160;
pub const HDMI_CORE_SYS_G2CB_COEFF_LOW: c_uint = 0x164;
pub const HDMI_CORE_SYS_G2CB_COEFF_UP: c_uint = 0x168;
pub const HDMI_CORE_SYS_B2CB_COEFF_LOW: c_uint = 0x16C;
pub const HDMI_CORE_SYS_B2CB_COEFF_UP: c_uint = 0x170;
pub const HDMI_CORE_SYS_R2CR_COEFF_LOW: c_uint = 0x174;
pub const HDMI_CORE_SYS_R2CR_COEFF_UP: c_uint = 0x178;
pub const HDMI_CORE_SYS_G2CR_COEFF_LOW: c_uint = 0x17C;
pub const HDMI_CORE_SYS_G2CR_COEFF_UP: c_uint = 0x180;
pub const HDMI_CORE_SYS_B2CR_COEFF_LOW: c_uint = 0x184;
pub const HDMI_CORE_SYS_B2CR_COEFF_UP: c_uint = 0x188;
pub const HDMI_CORE_SYS_RGB_OFFSET_LOW: c_uint = 0x18C;
pub const HDMI_CORE_SYS_RGB_OFFSET_UP: c_uint = 0x190;
pub const HDMI_CORE_SYS_Y_OFFSET_LOW: c_uint = 0x194;
pub const HDMI_CORE_SYS_Y_OFFSET_UP: c_uint = 0x198;
pub const HDMI_CORE_SYS_CBCR_OFFSET_LOW: c_uint = 0x19C;
pub const HDMI_CORE_SYS_CBCR_OFFSET_UP: c_uint = 0x1A0;
pub const HDMI_CORE_SYS_INTR_STATE: c_uint = 0x1C0;
pub const HDMI_CORE_SYS_INTR1: c_uint = 0x1C4;
pub const HDMI_CORE_SYS_INTR2: c_uint = 0x1C8;
pub const HDMI_CORE_SYS_INTR3: c_uint = 0x1CC;
pub const HDMI_CORE_SYS_INTR4: c_uint = 0x1D0;
pub const HDMI_CORE_SYS_INTR_UNMASK1: c_uint = 0x1D4;
pub const HDMI_CORE_SYS_INTR_UNMASK2: c_uint = 0x1D8;
pub const HDMI_CORE_SYS_INTR_UNMASK3: c_uint = 0x1DC;
pub const HDMI_CORE_SYS_INTR_UNMASK4: c_uint = 0x1E0;
pub const HDMI_CORE_SYS_INTR_CTRL: c_uint = 0x1E4;
pub const HDMI_CORE_SYS_TMDS_CTRL: c_uint = 0x208;
// value definitions for HDMI_CORE_SYS_SYS_CTRL1 fields
pub const HDMI_CORE_SYS_SYS_CTRL1_VEN_FOLLOWVSYNC: c_uint = 0x1;
pub const HDMI_CORE_SYS_SYS_CTRL1_HEN_FOLLOWHSYNC: c_uint = 0x1;
pub const HDMI_CORE_SYS_SYS_CTRL1_BSEL_24BITBUS: c_uint = 0x1;
pub const HDMI_CORE_SYS_SYS_CTRL1_EDGE_RISINGEDGE: c_uint = 0x1;
// HDMI DDC E-DID
pub const HDMI_CORE_DDC_ADDR: c_uint = 0x3B4;
pub const HDMI_CORE_DDC_SEGM: c_uint = 0x3B8;
pub const HDMI_CORE_DDC_OFFSET: c_uint = 0x3BC;
pub const HDMI_CORE_DDC_COUNT1: c_uint = 0x3C0;
pub const HDMI_CORE_DDC_COUNT2: c_uint = 0x3C4;
pub const HDMI_CORE_DDC_STATUS: c_uint = 0x3C8;
pub const HDMI_CORE_DDC_CMD: c_uint = 0x3CC;
pub const HDMI_CORE_DDC_DATA: c_uint = 0x3D0;
// HDMI IP Core Audio Video
pub const HDMI_CORE_AV_ACR_CTRL: c_uint = 0x4;
pub const HDMI_CORE_AV_FREQ_SVAL: c_uint = 0x8;
pub const HDMI_CORE_AV_N_SVAL1: c_uint = 0xC;
pub const HDMI_CORE_AV_N_SVAL2: c_uint = 0x10;
pub const HDMI_CORE_AV_N_SVAL3: c_uint = 0x14;
pub const HDMI_CORE_AV_CTS_SVAL1: c_uint = 0x18;
pub const HDMI_CORE_AV_CTS_SVAL2: c_uint = 0x1C;
pub const HDMI_CORE_AV_CTS_SVAL3: c_uint = 0x20;
pub const HDMI_CORE_AV_CTS_HVAL1: c_uint = 0x24;
pub const HDMI_CORE_AV_CTS_HVAL2: c_uint = 0x28;
pub const HDMI_CORE_AV_CTS_HVAL3: c_uint = 0x2C;
pub const HDMI_CORE_AV_AUD_MODE: c_uint = 0x50;
pub const HDMI_CORE_AV_SPDIF_CTRL: c_uint = 0x54;
pub const HDMI_CORE_AV_HW_SPDIF_FS: c_uint = 0x60;
pub const HDMI_CORE_AV_SWAP_I2S: c_uint = 0x64;
pub const HDMI_CORE_AV_SPDIF_ERTH: c_uint = 0x6C;
pub const HDMI_CORE_AV_I2S_IN_MAP: c_uint = 0x70;
pub const HDMI_CORE_AV_I2S_IN_CTRL: c_uint = 0x74;
pub const HDMI_CORE_AV_I2S_CHST0: c_uint = 0x78;
pub const HDMI_CORE_AV_I2S_CHST1: c_uint = 0x7C;
pub const HDMI_CORE_AV_I2S_CHST2: c_uint = 0x80;
pub const HDMI_CORE_AV_I2S_CHST4: c_uint = 0x84;
pub const HDMI_CORE_AV_I2S_CHST5: c_uint = 0x88;
pub const HDMI_CORE_AV_ASRC: c_uint = 0x8C;
pub const HDMI_CORE_AV_I2S_IN_LEN: c_uint = 0x90;
pub const HDMI_CORE_AV_HDMI_CTRL: c_uint = 0xBC;
pub const HDMI_CORE_AV_AUDO_TXSTAT: c_uint = 0xC0;
pub const HDMI_CORE_AV_AUD_PAR_BUSCLK_1: c_uint = 0xCC;
pub const HDMI_CORE_AV_AUD_PAR_BUSCLK_2: c_uint = 0xD0;
pub const HDMI_CORE_AV_AUD_PAR_BUSCLK_3: c_uint = 0xD4;
pub const HDMI_CORE_AV_TEST_TXCTRL: c_uint = 0xF0;
pub const HDMI_CORE_AV_DPD: c_uint = 0xF4;
pub const HDMI_CORE_AV_PB_CTRL1: c_uint = 0xF8;
pub const HDMI_CORE_AV_PB_CTRL2: c_uint = 0xFC;
pub const HDMI_CORE_AV_AVI_BASE: c_uint = 0x100;
pub const HDMI_CORE_AV_AVI_TYPE: c_uint = 0x100;
pub const HDMI_CORE_AV_AVI_VERS: c_uint = 0x104;
pub const HDMI_CORE_AV_AVI_LEN: c_uint = 0x108;
pub const HDMI_CORE_AV_AVI_CHSUM: c_uint = 0x10C;

pub const HDMI_CORE_AV_SPD_TYPE: c_uint = 0x180;
pub const HDMI_CORE_AV_SPD_VERS: c_uint = 0x184;
pub const HDMI_CORE_AV_SPD_LEN: c_uint = 0x188;
pub const HDMI_CORE_AV_SPD_CHSUM: c_uint = 0x18C;

pub const HDMI_CORE_AV_AUDIO_TYPE: c_uint = 0x200;
pub const HDMI_CORE_AV_AUDIO_VERS: c_uint = 0x204;
pub const HDMI_CORE_AV_AUDIO_LEN: c_uint = 0x208;
pub const HDMI_CORE_AV_AUDIO_CHSUM: c_uint = 0x20C;

pub const HDMI_CORE_AV_MPEG_TYPE: c_uint = 0x280;
pub const HDMI_CORE_AV_MPEG_VERS: c_uint = 0x284;
pub const HDMI_CORE_AV_MPEG_LEN: c_uint = 0x288;
pub const HDMI_CORE_AV_MPEG_CHSUM: c_uint = 0x28C;

pub const HDMI_CORE_AV_CP_BYTE1: c_uint = 0x37C;

pub const HDMI_CORE_AV_CEC_ADDR_ID: c_uint = 0x3FC;
pub const HDMI_CORE_AV_SPD_DBYTE_ELSIZE: c_uint = 0x4;
pub const HDMI_CORE_AV_GEN2_DBYTE_ELSIZE: c_uint = 0x4;
pub const HDMI_CORE_AV_MPEG_DBYTE_ELSIZE: c_uint = 0x4;
pub const HDMI_CORE_AV_GEN_DBYTE_ELSIZE: c_uint = 0x4;
pub const HDMI_CORE_AV_AVI_DBYTE_NELEMS: c_int = 15;
pub const HDMI_CORE_AV_SPD_DBYTE_NELEMS: c_int = 27;
pub const HDMI_CORE_AV_AUD_DBYTE_NELEMS: c_int = 10;
pub const HDMI_CORE_AV_MPEG_DBYTE_NELEMS: c_int = 27;
pub const HDMI_CORE_AV_GEN_DBYTE_NELEMS: c_int = 31;
pub const HDMI_CORE_AV_GEN2_DBYTE_NELEMS: c_int = 31;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hdmi_core_inputbus_width {
    HDMI_INPUT_8BIT = 0,
    HDMI_INPUT_10BIT = 1,
    HDMI_INPUT_12BIT = 2
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hdmi_core_dither_trunc {
    HDMI_OUTPUTTRUNCATION_8BIT = 0,
    HDMI_OUTPUTTRUNCATION_10BIT = 1,
    HDMI_OUTPUTTRUNCATION_12BIT = 2,
    HDMI_OUTPUTDITHER_8BIT = 3,
    HDMI_OUTPUTDITHER_10BIT = 4,
    HDMI_OUTPUTDITHER_12BIT = 5
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hdmi_core_deepcolor_ed {
    HDMI_DEEPCOLORPACKECTDISABLE = 0,
    HDMI_DEEPCOLORPACKECTENABLE = 1
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hdmi_core_packet_mode {
    HDMI_PACKETMODERESERVEDVALUE = 0,
    HDMI_PACKETMODE24BITPERPIXEL = 4,
    HDMI_PACKETMODE30BITPERPIXEL = 5,
    HDMI_PACKETMODE36BITPERPIXEL = 6,
    HDMI_PACKETMODE48BITPERPIXEL = 7
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hdmi_core_tclkselclkmult {
    HDMI_FPLL05IDCK = 0,
    HDMI_FPLL10IDCK = 1,
    HDMI_FPLL20IDCK = 2,
    HDMI_FPLL40IDCK = 3
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hdmi_core_packet_ctrl {
    HDMI_PACKETENABLE = 1,
    HDMI_PACKETDISABLE = 0,
    HDMI_PACKETREPEATON = 1,
    HDMI_PACKETREPEATOFF = 0
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hdmi_audio_i2s_config {
    HDMI_AUDIO_I2S_MSB_SHIFTED_FIRST = 0,
    HDMI_AUDIO_I2S_LSB_SHIFTED_FIRST = 1,
    HDMI_AUDIO_I2S_SCK_EDGE_FALLING = 0,
    HDMI_AUDIO_I2S_SCK_EDGE_RISING = 1,
    HDMI_AUDIO_I2S_VBIT_FOR_PCM = 0,
    HDMI_AUDIO_I2S_VBIT_FOR_COMPRESSED = 1,
    HDMI_AUDIO_I2S_FIRST_BIT_SHIFT = 0,
    HDMI_AUDIO_I2S_FIRST_BIT_NO_SHIFT = 1,
    HDMI_AUDIO_I2S_SD0_EN = 1,
    HDMI_AUDIO_I2S_SD1_EN = 1 << 1,
    HDMI_AUDIO_I2S_SD2_EN = 1 << 2,
    HDMI_AUDIO_I2S_SD3_EN = 1 << 3,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hdmi_core_video_config {
    pub ip_bus_width: hdmi_core_inputbus_width,
    pub op_dither_truc: hdmi_core_dither_trunc,
    pub deep_color_pkt: hdmi_core_deepcolor_ed,
    pub pkt_mode: hdmi_core_packet_mode,
    pub hdmi_dvi: hdmi_core_hdmi_dvi,
    pub tclk_sel_clkmult: hdmi_core_tclkselclkmult,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hdmi_core_packet_enable_repeat {
    pub audio_pkt: u32,
    pub audio_pkt_repeat: u32,
    pub avi_infoframe: u32,
    pub avi_infoframe_repeat: u32,
    pub gen_cntrl_pkt: u32,
    pub gen_cntrl_pkt_repeat: u32,
    pub generic_pkt: u32,
    pub generic_pkt_repeat: u32,
}

extern "C" {
    pub fn hdmi4_read_edid(core: *mut hdmi_core_data, edid: *mut u8, len: c_int) -> c_int;
}
extern "C" {
    pub fn hdmi4_core_dump(core: *mut hdmi_core_data, s: *mut seq_file);
}
extern "C" {
    pub fn hdmi4_core_init(pdev: *mut platform_device, core: *mut hdmi_core_data) -> c_int;
}
extern "C" {
    pub fn hdmi4_audio_start(core: *mut hdmi_core_data, wp: *mut hdmi_wp_data) -> c_int;
}
extern "C" {
    pub fn hdmi4_audio_stop(core: *mut hdmi_core_data, wp: *mut hdmi_wp_data);
}
