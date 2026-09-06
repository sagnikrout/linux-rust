//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/pci/cx18/cx18-av-core.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// cx18 ADEC header
//
// Derived from cx25840-core.h
//
// Copyright (C) 2007  Hans Verkuil <hverkuil@kernel.org>
// Copyright (C) 2008  Andy Walls <awalls@md.metrocast.net>
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cx18_av_video_input {
// Composite video inputs In1-In8
    CX18_AV_COMPOSITE1 = 1,
    CX18_AV_COMPOSITE2,
    CX18_AV_COMPOSITE3,
    CX18_AV_COMPOSITE4,
    CX18_AV_COMPOSITE5,
    CX18_AV_COMPOSITE6,
    CX18_AV_COMPOSITE7,
    CX18_AV_COMPOSITE8,

// S-Video inputs consist of one luma input (In1-In8) ORed with one
    chroma input (In5-In8) */
    CX18_AV_SVIDEO_LUMA1 = 0x10,
    CX18_AV_SVIDEO_LUMA2 = 0x20,
    CX18_AV_SVIDEO_LUMA3 = 0x30,
    CX18_AV_SVIDEO_LUMA4 = 0x40,
    CX18_AV_SVIDEO_LUMA5 = 0x50,
    CX18_AV_SVIDEO_LUMA6 = 0x60,
    CX18_AV_SVIDEO_LUMA7 = 0x70,
    CX18_AV_SVIDEO_LUMA8 = 0x80,
    CX18_AV_SVIDEO_CHROMA4 = 0x400,
    CX18_AV_SVIDEO_CHROMA5 = 0x500,
    CX18_AV_SVIDEO_CHROMA6 = 0x600,
    CX18_AV_SVIDEO_CHROMA7 = 0x700,
    CX18_AV_SVIDEO_CHROMA8 = 0x800,

// S-Video aliases for common luma/chroma combinations
    CX18_AV_SVIDEO1 = 0x510,
    CX18_AV_SVIDEO2 = 0x620,
    CX18_AV_SVIDEO3 = 0x730,
    CX18_AV_SVIDEO4 = 0x840,

// Component Video inputs consist of one luma input (In1-In8) ORed
    with a red chroma (In4-In6) and blue chroma input (In7-In8) */
    CX18_AV_COMPONENT_LUMA1 = 0x1000,
    CX18_AV_COMPONENT_LUMA2 = 0x2000,
    CX18_AV_COMPONENT_LUMA3 = 0x3000,
    CX18_AV_COMPONENT_LUMA4 = 0x4000,
    CX18_AV_COMPONENT_LUMA5 = 0x5000,
    CX18_AV_COMPONENT_LUMA6 = 0x6000,
    CX18_AV_COMPONENT_LUMA7 = 0x7000,
    CX18_AV_COMPONENT_LUMA8 = 0x8000,
    CX18_AV_COMPONENT_R_CHROMA4 = 0x40000,
    CX18_AV_COMPONENT_R_CHROMA5 = 0x50000,
    CX18_AV_COMPONENT_R_CHROMA6 = 0x60000,
    CX18_AV_COMPONENT_B_CHROMA7 = 0x700000,
    CX18_AV_COMPONENT_B_CHROMA8 = 0x800000,

// Component Video aliases for common combinations
    CX18_AV_COMPONENT1 = 0x861000,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cx18_av_audio_input {
// Audio inputs: serial or In4-In8
    CX18_AV_AUDIO_SERIAL1,
    CX18_AV_AUDIO_SERIAL2,
    CX18_AV_AUDIO4 = 4,
    CX18_AV_AUDIO5,
    CX18_AV_AUDIO6,
    CX18_AV_AUDIO7,
    CX18_AV_AUDIO8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cx18_av_state {
    pub sd: v4l2_subdev,
    pub hdl: v4l2_ctrl_handler,
    pub volume: *mut v4l2_ctrl,
    pub radio: c_int,
    pub std: v4l2_std_id,
    pub vid_input: cx18_av_video_input,
    pub aud_input: cx18_av_audio_input,
    pub audclk_freq: u32,
    pub audmode: c_int,
    pub rev: u32,
    pub is_initialized: c_int,
//
// The VBI slicer starts operating and counting lines, beginning at
// slicer line count of 1, at D lines after the deassertion of VRESET.
// This staring field line, S, is 6 (& 319) or 10 (& 273) for 625 or 525
// line systems respectively.  Sliced ancillary data captured on VBI
// slicer line M is inserted after the VBI slicer is done with line M,
// when VBI slicer line count is N = M+1.  Thus when the VBI slicer
// reports a VBI slicer line number with ancillary data, the IDID0 byte
// indicates VBI slicer line N.  The actual field line that the captured
// data comes from is
//
// L = M+(S+D-1) = N-1+(S+D-1) = N + (S+D-2).
//
// L is the line in the field, not frame, from which the VBI data came.
// N is the line reported by the slicer in the ancillary data.
// D is the slicer_line_delay value programmed into register 0x47f.
// S is 6 for 625 line systems or 10 for 525 line systems
// (S+D-2) is the slicer_line_offset used to convert slicer reported
// line counts to actual field lines.
//
    pub slicer_line_delay: c_int,
    pub slicer_line_offset: c_int,
}

// Registers
pub const CXADEC_CHIP_TYPE_TIGER: c_uint = 0x837;
pub const CXADEC_CHIP_TYPE_MAKO: c_uint = 0x843;
pub const CXADEC_HOST_REG1: c_uint = 0x000;
pub const CXADEC_HOST_REG2: c_uint = 0x001;
pub const CXADEC_CHIP_CTRL: c_uint = 0x100;
pub const CXADEC_AFE_CTRL: c_uint = 0x104;
pub const CXADEC_PLL_CTRL1: c_uint = 0x108;
pub const CXADEC_VID_PLL_FRAC: c_uint = 0x10C;
pub const CXADEC_AUX_PLL_FRAC: c_uint = 0x110;
pub const CXADEC_PIN_CTRL1: c_uint = 0x114;
pub const CXADEC_PIN_CTRL2: c_uint = 0x118;
pub const CXADEC_PIN_CFG1: c_uint = 0x11C;
pub const CXADEC_PIN_CFG2: c_uint = 0x120;
pub const CXADEC_PIN_CFG3: c_uint = 0x124;
pub const CXADEC_I2S_MCLK: c_uint = 0x127;
pub const CXADEC_AUD_LOCK1: c_uint = 0x128;
pub const CXADEC_AUD_LOCK2: c_uint = 0x12C;
pub const CXADEC_POWER_CTRL: c_uint = 0x130;
pub const CXADEC_AFE_DIAG_CTRL1: c_uint = 0x134;
pub const CXADEC_AFE_DIAG_CTRL2: c_uint = 0x138;
pub const CXADEC_AFE_DIAG_CTRL3: c_uint = 0x13C;
pub const CXADEC_PLL_DIAG_CTRL: c_uint = 0x140;
pub const CXADEC_TEST_CTRL1: c_uint = 0x144;
pub const CXADEC_TEST_CTRL2: c_uint = 0x148;
pub const CXADEC_BIST_STAT: c_uint = 0x14C;
pub const CXADEC_DLL1_DIAG_CTRL: c_uint = 0x158;
pub const CXADEC_DLL2_DIAG_CTRL: c_uint = 0x15C;
// IR registers
pub const CXADEC_IR_CTRL_REG: c_uint = 0x200;
pub const CXADEC_IR_TXCLK_REG: c_uint = 0x204;
pub const CXADEC_IR_RXCLK_REG: c_uint = 0x208;
pub const CXADEC_IR_CDUTY_REG: c_uint = 0x20C;
pub const CXADEC_IR_STAT_REG: c_uint = 0x210;
pub const CXADEC_IR_IRQEN_REG: c_uint = 0x214;
pub const CXADEC_IR_FILTER_REG: c_uint = 0x218;
pub const CXADEC_IR_FIFO_REG: c_uint = 0x21C;
// Video Registers
pub const CXADEC_MODE_CTRL: c_uint = 0x400;
pub const CXADEC_OUT_CTRL1: c_uint = 0x404;
pub const CXADEC_OUT_CTRL2: c_uint = 0x408;
pub const CXADEC_GEN_STAT: c_uint = 0x40C;
pub const CXADEC_INT_STAT_MASK: c_uint = 0x410;
pub const CXADEC_LUMA_CTRL: c_uint = 0x414;
pub const CXADEC_BRIGHTNESS_CTRL_BYTE: c_uint = 0x414;
pub const CXADEC_CONTRAST_CTRL_BYTE: c_uint = 0x415;
pub const CXADEC_LUMA_CTRL_BYTE_3: c_uint = 0x416;
pub const CXADEC_HSCALE_CTRL: c_uint = 0x418;
pub const CXADEC_VSCALE_CTRL: c_uint = 0x41C;
pub const CXADEC_CHROMA_CTRL: c_uint = 0x420;
pub const CXADEC_USAT_CTRL_BYTE: c_uint = 0x420;
pub const CXADEC_VSAT_CTRL_BYTE: c_uint = 0x421;
pub const CXADEC_HUE_CTRL_BYTE: c_uint = 0x422;
pub const CXADEC_VBI_LINE_CTRL1: c_uint = 0x424;
pub const CXADEC_VBI_LINE_CTRL2: c_uint = 0x428;
pub const CXADEC_VBI_LINE_CTRL3: c_uint = 0x42C;
pub const CXADEC_VBI_LINE_CTRL4: c_uint = 0x430;
pub const CXADEC_VBI_LINE_CTRL5: c_uint = 0x434;
pub const CXADEC_VBI_FC_CFG: c_uint = 0x438;
pub const CXADEC_VBI_MISC_CFG1: c_uint = 0x43C;
pub const CXADEC_VBI_MISC_CFG2: c_uint = 0x440;
pub const CXADEC_VBI_PAY1: c_uint = 0x444;
pub const CXADEC_VBI_PAY2: c_uint = 0x448;
pub const CXADEC_VBI_CUST1_CFG1: c_uint = 0x44C;
pub const CXADEC_VBI_CUST1_CFG2: c_uint = 0x450;
pub const CXADEC_VBI_CUST1_CFG3: c_uint = 0x454;
pub const CXADEC_VBI_CUST2_CFG1: c_uint = 0x458;
pub const CXADEC_VBI_CUST2_CFG2: c_uint = 0x45C;
pub const CXADEC_VBI_CUST2_CFG3: c_uint = 0x460;
pub const CXADEC_VBI_CUST3_CFG1: c_uint = 0x464;
pub const CXADEC_VBI_CUST3_CFG2: c_uint = 0x468;
pub const CXADEC_VBI_CUST3_CFG3: c_uint = 0x46C;
pub const CXADEC_HORIZ_TIM_CTRL: c_uint = 0x470;
pub const CXADEC_VERT_TIM_CTRL: c_uint = 0x474;
pub const CXADEC_SRC_COMB_CFG: c_uint = 0x478;
pub const CXADEC_CHROMA_VBIOFF_CFG: c_uint = 0x47C;
pub const CXADEC_FIELD_COUNT: c_uint = 0x480;
pub const CXADEC_MISC_TIM_CTRL: c_uint = 0x484;
pub const CXADEC_DFE_CTRL1: c_uint = 0x488;
pub const CXADEC_DFE_CTRL2: c_uint = 0x48C;
pub const CXADEC_DFE_CTRL3: c_uint = 0x490;
pub const CXADEC_PLL_CTRL2: c_uint = 0x494;
pub const CXADEC_HTL_CTRL: c_uint = 0x498;
pub const CXADEC_COMB_CTRL: c_uint = 0x49C;
pub const CXADEC_CRUSH_CTRL: c_uint = 0x4A0;
pub const CXADEC_SOFT_RST_CTRL: c_uint = 0x4A4;
pub const CXADEC_MV_DT_CTRL2: c_uint = 0x4A8;
pub const CXADEC_MV_DT_CTRL3: c_uint = 0x4AC;
pub const CXADEC_MISC_DIAG_CTRL: c_uint = 0x4B8;
pub const CXADEC_DL_CTL: c_uint = 0x800;
pub const CXADEC_DL_CTL_ADDRESS_LOW: c_uint = 0x800   /* Byte 1 in DL_CTL */;
pub const CXADEC_DL_CTL_ADDRESS_HIGH: c_uint = 0x801   /* Byte 2 in DL_CTL */;
pub const CXADEC_DL_CTL_DATA: c_uint = 0x802   /* Byte 3 in DL_CTL */;
pub const CXADEC_DL_CTL_CONTROL: c_uint = 0x803   /* Byte 4 in DL_CTL */;
pub const CXADEC_STD_DET_STATUS: c_uint = 0x804;
pub const CXADEC_STD_DET_CTL: c_uint = 0x808;
pub const CXADEC_STD_DET_CTL_AUD_CTL: c_uint = 0x808 /* Byte 1 in STD_DET_CTL */;
pub const CXADEC_STD_DET_CTL_PREF_MODE: c_uint = 0x809 /* Byte 2 in STD_DET_CTL */;
pub const CXADEC_DW8051_INT: c_uint = 0x80C;
pub const CXADEC_GENERAL_CTL: c_uint = 0x810;
pub const CXADEC_AAGC_CTL: c_uint = 0x814;
pub const CXADEC_IF_SRC_CTL: c_uint = 0x818;
pub const CXADEC_ANLOG_DEMOD_CTL: c_uint = 0x81C;
pub const CXADEC_ROT_FREQ_CTL: c_uint = 0x820;
pub const CXADEC_FM1_CTL: c_uint = 0x824;
pub const CXADEC_PDF_CTL: c_uint = 0x828;
pub const CXADEC_DFT1_CTL1: c_uint = 0x82C;
pub const CXADEC_DFT1_CTL2: c_uint = 0x830;
pub const CXADEC_DFT_STATUS: c_uint = 0x834;
pub const CXADEC_DFT2_CTL1: c_uint = 0x838;
pub const CXADEC_DFT2_CTL2: c_uint = 0x83C;
pub const CXADEC_DFT2_STATUS: c_uint = 0x840;
pub const CXADEC_DFT3_CTL1: c_uint = 0x844;
pub const CXADEC_DFT3_CTL2: c_uint = 0x848;
pub const CXADEC_DFT3_STATUS: c_uint = 0x84C;
pub const CXADEC_DFT4_CTL1: c_uint = 0x850;
pub const CXADEC_DFT4_CTL2: c_uint = 0x854;
pub const CXADEC_DFT4_STATUS: c_uint = 0x858;
pub const CXADEC_AM_MTS_DET: c_uint = 0x85C;
pub const CXADEC_ANALOG_MUX_CTL: c_uint = 0x860;
pub const CXADEC_DIG_PLL_CTL1: c_uint = 0x864;
pub const CXADEC_DIG_PLL_CTL2: c_uint = 0x868;
pub const CXADEC_DIG_PLL_CTL3: c_uint = 0x86C;
pub const CXADEC_DIG_PLL_CTL4: c_uint = 0x870;
pub const CXADEC_DIG_PLL_CTL5: c_uint = 0x874;
pub const CXADEC_DEEMPH_GAIN_CTL: c_uint = 0x878;
pub const CXADEC_DEEMPH_COEF1: c_uint = 0x87C;
pub const CXADEC_DEEMPH_COEF2: c_uint = 0x880;
pub const CXADEC_DBX1_CTL1: c_uint = 0x884;
pub const CXADEC_DBX1_CTL2: c_uint = 0x888;
pub const CXADEC_DBX1_STATUS: c_uint = 0x88C;
pub const CXADEC_DBX2_CTL1: c_uint = 0x890;
pub const CXADEC_DBX2_CTL2: c_uint = 0x894;
pub const CXADEC_DBX2_STATUS: c_uint = 0x898;
pub const CXADEC_AM_FM_DIFF: c_uint = 0x89C;
// NICAM registers go here
pub const CXADEC_NICAM_STATUS: c_uint = 0x8C8;
pub const CXADEC_DEMATRIX_CTL: c_uint = 0x8CC;
pub const CXADEC_PATH1_CTL1: c_uint = 0x8D0;
pub const CXADEC_PATH1_VOL_CTL: c_uint = 0x8D4;
pub const CXADEC_PATH1_EQ_CTL: c_uint = 0x8D8;
pub const CXADEC_PATH1_SC_CTL: c_uint = 0x8DC;
pub const CXADEC_PATH2_CTL1: c_uint = 0x8E0;
pub const CXADEC_PATH2_VOL_CTL: c_uint = 0x8E4;
pub const CXADEC_PATH2_EQ_CTL: c_uint = 0x8E8;
pub const CXADEC_PATH2_SC_CTL: c_uint = 0x8EC;
pub const CXADEC_SRC_CTL: c_uint = 0x8F0;
pub const CXADEC_SRC_LF_COEF: c_uint = 0x8F4;
pub const CXADEC_SRC1_CTL: c_uint = 0x8F8;
pub const CXADEC_SRC2_CTL: c_uint = 0x8FC;
pub const CXADEC_SRC3_CTL: c_uint = 0x900;
pub const CXADEC_SRC4_CTL: c_uint = 0x904;
pub const CXADEC_SRC5_CTL: c_uint = 0x908;
pub const CXADEC_SRC6_CTL: c_uint = 0x90C;
pub const CXADEC_BASEBAND_OUT_SEL: c_uint = 0x910;
pub const CXADEC_I2S_IN_CTL: c_uint = 0x914;
pub const CXADEC_I2S_OUT_CTL: c_uint = 0x918;
pub const CXADEC_AC97_CTL: c_uint = 0x91C;
pub const CXADEC_QAM_PDF: c_uint = 0x920;
pub const CXADEC_QAM_CONST_DEC: c_uint = 0x924;
pub const CXADEC_QAM_ROTATOR_FREQ: c_uint = 0x948;
// Bit definitions / settings used in Mako Audio
pub const CXADEC_PREF_MODE_MONO_LANGA: c_int = 0;
pub const CXADEC_PREF_MODE_MONO_LANGB: c_int = 1;
pub const CXADEC_PREF_MODE_MONO_LANGC: c_int = 2;
pub const CXADEC_PREF_MODE_FALLBACK: c_int = 3;
pub const CXADEC_PREF_MODE_STEREO: c_int = 4;
pub const CXADEC_PREF_MODE_DUAL_LANG_AC: c_int = 5;
pub const CXADEC_PREF_MODE_DUAL_LANG_BC: c_int = 6;
pub const CXADEC_PREF_MODE_DUAL_LANG_AB: c_int = 7;
pub const CXADEC_DETECT_STEREO: c_int = 1;
pub const CXADEC_DETECT_DUAL: c_int = 2;
pub const CXADEC_DETECT_TRI: c_int = 4;
pub const CXADEC_DETECT_SAP: c_uint = 0x10;
pub const CXADEC_DETECT_NO_SIGNAL: c_uint = 0xFF;
pub const CXADEC_SELECT_AUDIO_STANDARD_BG: c_uint = 0xF0  /* NICAM BG and A2 BG */;
pub const CXADEC_SELECT_AUDIO_STANDARD_DK1: c_uint = 0xF1  /* NICAM DK and A2 DK */;
pub const CXADEC_SELECT_AUDIO_STANDARD_DK2: c_uint = 0xF2;
pub const CXADEC_SELECT_AUDIO_STANDARD_DK3: c_uint = 0xF3;
pub const CXADEC_SELECT_AUDIO_STANDARD_I: c_uint = 0xF4  /* NICAM I and A1 */;
pub const CXADEC_SELECT_AUDIO_STANDARD_L: c_uint = 0xF5  /* NICAM L and System L AM */;
pub const CXADEC_SELECT_AUDIO_STANDARD_BTSC: c_uint = 0xF6;
pub const CXADEC_SELECT_AUDIO_STANDARD_EIAJ: c_uint = 0xF7;
pub const CXADEC_SELECT_AUDIO_STANDARD_A2_M: c_uint = 0xF8  /* A2 M */;
pub const CXADEC_SELECT_AUDIO_STANDARD_FM: c_uint = 0xF9  /* FM radio */;
pub const CXADEC_SELECT_AUDIO_STANDARD_AUTO: c_uint = 0xFF  /* Auto detect */;
extern "C" {
    pub fn container_of(_arg: sd, cx18_av_state: struct, _arg: sd) -> return;
}
// -----------------------------------------------------------------------
// cx18_av-core.c
extern "C" {
    pub fn cx18_av_write(cx: *mut cx18, addr: u16, value: u8) -> c_int;
}
extern "C" {
    pub fn cx18_av_write4(cx: *mut cx18, addr: u16, value: u32) -> c_int;
}
extern "C" {
    pub fn cx18_av_write4_noretry(cx: *mut cx18, addr: u16, value: u32) -> c_int;
}
extern "C" {
    pub fn cx18_av_write_expect(cx: *mut cx18, addr: u16, value: u8, eval: u8, mask: u8) -> c_int;
}
extern "C" {
    pub fn cx18_av_read(cx: *mut cx18, addr: u16) -> u8;
}
extern "C" {
    pub fn cx18_av_read4(cx: *mut cx18, addr: u16) -> u32;
}
extern "C" {
    pub fn cx18_av_and_or(cx: *mut cx18, addr: u16, mask: unsigned, value: u8) -> c_int;
}
extern "C" {
    pub fn cx18_av_and_or4(cx: *mut cx18, addr: u16, mask: u32, value: u32) -> c_int;
}
extern "C" {
    pub fn cx18_av_std_setup(cx: *mut cx18);
}
extern "C" {
    pub fn cx18_av_probe(cx: *mut cx18) -> c_int;
}
// -----------------------------------------------------------------------
// cx18_av-firmware.c
extern "C" {
    pub fn cx18_av_loadfw(cx: *mut cx18) -> c_int;
}
// -----------------------------------------------------------------------
// cx18_av-audio.c
extern "C" {
    pub fn cx18_av_s_clock_freq(sd: *mut v4l2_subdev, freq: u32) -> c_int;
}
extern "C" {
    pub fn cx18_av_audio_set_path(cx: *mut cx18);
}
// -----------------------------------------------------------------------
// cx18_av-vbi.c
extern "C" {
    pub fn cx18_av_s_raw_fmt(sd: *mut v4l2_subdev, fmt: *mut v4l2_vbi_format) -> c_int;
}
extern "C" {
    pub fn cx18_av_g_sliced_fmt(sd: *mut v4l2_subdev, fmt: *mut v4l2_sliced_vbi_format) -> c_int;
}
extern "C" {
    pub fn cx18_av_s_sliced_fmt(sd: *mut v4l2_subdev, fmt: *mut v4l2_sliced_vbi_format) -> c_int;
}
