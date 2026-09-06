//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/rockchip/rk3066_hdmi.h
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
// Copyright (C) Rockchip Electronics Co., Ltd.
// Zheng Yang <zhengyang@rock-chips.com>
//
pub const GRF_SOC_CON0: c_uint = 0x150;

pub const DDC_SEGMENT_ADDR: c_uint = 0x30;

pub const HDMI_MAXIMUM_INFO_FRAME_SIZE: c_uint = 0x11;
pub const N_32K: c_uint = 0x1000;
pub const N_441K: c_uint = 0x1880;
pub const N_882K: c_uint = 0x3100;
pub const N_1764K: c_uint = 0x6200;
pub const N_48K: c_uint = 0x1800;
pub const N_96K: c_uint = 0x3000;
pub const N_192K: c_uint = 0x6000;
pub const HDMI_SYS_CTRL: c_uint = 0x000;
pub const HDMI_LR_SWAP_N3: c_uint = 0x004;
pub const HDMI_N2: c_uint = 0x008;
pub const HDMI_N1: c_uint = 0x00c;
pub const HDMI_SPDIF_FS_CTS_INT3: c_uint = 0x010;
pub const HDMI_CTS_INT2: c_uint = 0x014;
pub const HDMI_CTS_INT1: c_uint = 0x018;
pub const HDMI_CTS_EXT3: c_uint = 0x01c;
pub const HDMI_CTS_EXT2: c_uint = 0x020;
pub const HDMI_CTS_EXT1: c_uint = 0x024;
pub const HDMI_AUDIO_CTRL1: c_uint = 0x028;
pub const HDMI_AUDIO_CTRL2: c_uint = 0x02c;
pub const HDMI_I2S_AUDIO_CTRL: c_uint = 0x030;
pub const HDMI_I2S_SWAP: c_uint = 0x040;
pub const HDMI_AUDIO_STA_BIT_CTRL1: c_uint = 0x044;
pub const HDMI_AUDIO_STA_BIT_CTRL2: c_uint = 0x048;
pub const HDMI_AUDIO_SRC_NUM_AND_LENGTH: c_uint = 0x050;
pub const HDMI_AV_CTRL1: c_uint = 0x054;
pub const HDMI_VIDEO_CTRL1: c_uint = 0x058;
pub const HDMI_DEEP_COLOR_MODE: c_uint = 0x05c;
pub const HDMI_EXT_VIDEO_PARA: c_uint = 0x0c0;
pub const HDMI_EXT_HTOTAL_L: c_uint = 0x0c4;
pub const HDMI_EXT_HTOTAL_H: c_uint = 0x0c8;
pub const HDMI_EXT_HBLANK_L: c_uint = 0x0cc;
pub const HDMI_EXT_HBLANK_H: c_uint = 0x0d0;
pub const HDMI_EXT_HDELAY_L: c_uint = 0x0d4;
pub const HDMI_EXT_HDELAY_H: c_uint = 0x0d8;
pub const HDMI_EXT_HDURATION_L: c_uint = 0x0dc;
pub const HDMI_EXT_HDURATION_H: c_uint = 0x0e0;
pub const HDMI_EXT_VTOTAL_L: c_uint = 0x0e4;
pub const HDMI_EXT_VTOTAL_H: c_uint = 0x0e8;
pub const HDMI_AV_CTRL2: c_uint = 0x0ec;
pub const HDMI_EXT_VBLANK_L: c_uint = 0x0f4;
pub const HDMI_EXT_VBLANK_H: c_uint = 0x10c;
pub const HDMI_EXT_VDELAY: c_uint = 0x0f8;
pub const HDMI_EXT_VDURATION: c_uint = 0x0fc;
pub const HDMI_CP_MANU_SEND_CTRL: c_uint = 0x100;
pub const HDMI_CP_AUTO_SEND_CTRL: c_uint = 0x104;
pub const HDMI_AUTO_CHECKSUM_OPT: c_uint = 0x108;
pub const HDMI_VIDEO_CTRL2: c_uint = 0x114;
pub const HDMI_PHY_OPTION: c_uint = 0x144;
pub const HDMI_CP_BUF_INDEX: c_uint = 0x17c;
pub const HDMI_CP_BUF_ACC_HB0: c_uint = 0x180;
pub const HDMI_CP_BUF_ACC_HB1: c_uint = 0x184;
pub const HDMI_CP_BUF_ACC_HB2: c_uint = 0x188;
pub const HDMI_CP_BUF_ACC_PB0: c_uint = 0x18c;
pub const HDMI_DDC_READ_FIFO_ADDR: c_uint = 0x200;
pub const HDMI_DDC_BUS_FREQ_L: c_uint = 0x204;
pub const HDMI_DDC_BUS_FREQ_H: c_uint = 0x208;
pub const HDMI_DDC_BUS_CTRL: c_uint = 0x2dc;
pub const HDMI_DDC_I2C_LEN: c_uint = 0x278;
pub const HDMI_DDC_I2C_OFFSET: c_uint = 0x280;
pub const HDMI_DDC_I2C_CTRL: c_uint = 0x284;
pub const HDMI_DDC_I2C_READ_BUF0: c_uint = 0x288;
pub const HDMI_DDC_I2C_READ_BUF1: c_uint = 0x28c;
pub const HDMI_DDC_I2C_READ_BUF2: c_uint = 0x290;
pub const HDMI_DDC_I2C_READ_BUF3: c_uint = 0x294;
pub const HDMI_DDC_I2C_WRITE_BUF0: c_uint = 0x298;
pub const HDMI_DDC_I2C_WRITE_BUF1: c_uint = 0x29c;
pub const HDMI_DDC_I2C_WRITE_BUF2: c_uint = 0x2a0;
pub const HDMI_DDC_I2C_WRITE_BUF3: c_uint = 0x2a4;
pub const HDMI_DDC_I2C_WRITE_BUF4: c_uint = 0x2ac;
pub const HDMI_DDC_I2C_WRITE_BUF5: c_uint = 0x2b0;
pub const HDMI_DDC_I2C_WRITE_BUF6: c_uint = 0x2b4;
pub const HDMI_INTR_MASK1: c_uint = 0x248;
pub const HDMI_INTR_MASK2: c_uint = 0x24c;
pub const HDMI_INTR_STATUS1: c_uint = 0x250;
pub const HDMI_INTR_STATUS2: c_uint = 0x254;
pub const HDMI_INTR_MASK3: c_uint = 0x258;
pub const HDMI_INTR_MASK4: c_uint = 0x25c;
pub const HDMI_INTR_STATUS3: c_uint = 0x260;
pub const HDMI_INTR_STATUS4: c_uint = 0x264;
pub const HDMI_HDCP_CTRL: c_uint = 0x2bc;
pub const HDMI_EDID_SEGMENT_POINTER: c_uint = 0x310;
pub const HDMI_EDID_WORD_ADDR: c_uint = 0x314;
pub const HDMI_EDID_FIFO_ADDR: c_uint = 0x318;
pub const HDMI_HPG_MENS_STA: c_uint = 0x37c;
pub const HDMI_INTERNAL_CLK_DIVIDER: c_uint = 0x800;
// HDMI_SYS_CTRL
// HDMI_LR_SWAP_N3
// HDMI_AUDIO_CTRL1
// HDMI_I2S_AUDIO_CTRL
// HDMI_AV_CTRL1
// HDMI_VIDEO_CTRL1
// HDMI_EXT_VIDEO_PARA
// HDMI_CP_AUTO_SEND_CTRL
// HDMI_VIDEO_CTRL2
// HDMI_CP_BUF_INDEX
// HDMI_INTR_MASK1
// HDMI_INTR_STATUS1
// HDMI_HDCP_CTRL
// HDMI_HPG_MENS_STA
