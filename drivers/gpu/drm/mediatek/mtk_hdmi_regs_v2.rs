//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/mediatek/mtk_hdmi_regs_v2.h
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
// Copyright (c) 2021 MediaTek Inc.
// Copyright (c) 2021 BayLibre, SAS
// Copyright (c) 2024 Collabora Ltd.
// AngeloGioacchino Del Regno <angelogioacchino.delregno@collabora.com>
//
// HDMI_TOP Config
pub const TOP_CFG00: c_uint = 0x000;

pub const TMDS_PACK_MODE_8BPP: c_int = 0;
pub const TMDS_PACK_MODE_10BPP: c_int = 1;
pub const TMDS_PACK_MODE_12BPP: c_int = 2;
pub const TMDS_PACK_MODE_16BPP: c_int = 3;

pub const TOP_CFG01: c_uint = 0x004;

// HDMI_TOP Audio: Channel Mapping
pub const TOP_AUD_MAP: c_uint = 0x00c;

// Auxiliary Video Information (AVI) Infoframe
pub const TOP_AVI_HEADER: c_uint = 0x024;
pub const TOP_AVI_PKT00: c_uint = 0x028;
pub const TOP_AVI_PKT01: c_uint = 0x02C;
pub const TOP_AVI_PKT02: c_uint = 0x030;
pub const TOP_AVI_PKT03: c_uint = 0x034;
pub const TOP_AVI_PKT04: c_uint = 0x038;
pub const TOP_AVI_PKT05: c_uint = 0x03C;
// Audio Interface Infoframe
pub const TOP_AIF_HEADER: c_uint = 0x040;
pub const TOP_AIF_PKT00: c_uint = 0x044;
pub const TOP_AIF_PKT01: c_uint = 0x048;
pub const TOP_AIF_PKT02: c_uint = 0x04c;
pub const TOP_AIF_PKT03: c_uint = 0x050;
// Audio SPDIF Infoframe
pub const TOP_SPDIF_HEADER: c_uint = 0x054;
pub const TOP_SPDIF_PKT00: c_uint = 0x058;
pub const TOP_SPDIF_PKT01: c_uint = 0x05c;
pub const TOP_SPDIF_PKT02: c_uint = 0x060;
pub const TOP_SPDIF_PKT03: c_uint = 0x064;
pub const TOP_SPDIF_PKT04: c_uint = 0x068;
pub const TOP_SPDIF_PKT05: c_uint = 0x06c;
pub const TOP_SPDIF_PKT06: c_uint = 0x070;
pub const TOP_SPDIF_PKT07: c_uint = 0x074;
// Infoframes Configuration
pub const TOP_INFO_EN: c_uint = 0x01c;

pub const TOP_INFO_RPT: c_uint = 0x020;

// Vendor Specific Infoframe
pub const TOP_VSIF_HEADER: c_uint = 0x174;
pub const TOP_VSIF_PKT00: c_uint = 0x178;
pub const TOP_VSIF_PKT01: c_uint = 0x17c;
pub const TOP_VSIF_PKT02: c_uint = 0x180;
pub const TOP_VSIF_PKT03: c_uint = 0x184;
pub const TOP_VSIF_PKT04: c_uint = 0x188;
pub const TOP_VSIF_PKT05: c_uint = 0x18c;
pub const TOP_VSIF_PKT06: c_uint = 0x190;
pub const TOP_VSIF_PKT07: c_uint = 0x194;
// HDMI_TOP Misc
pub const TOP_MISC_CTLR: c_uint = 0x1a4;

// Hardware interrupts
pub const TOP_INT_STA00: c_uint = 0x1a8;
pub const TOP_INT_ENABLE00: c_uint = 0x1b0;

pub const TOP_INT_ENABLE01: c_uint = 0x1b4;
pub const TOP_INT_CLR00: c_uint = 0x1b8;
pub const TOP_INT_CLR01: c_uint = 0x1bc;
// Video Mute
pub const TOP_VMUTE_CFG1: c_uint = 0x1c8;

// HDMI Audio IP
pub const AIP_CTRL: c_uint = 0x400;

pub const AIP_N_VAL: c_uint = 0x404;
pub const AIP_CTS_SVAL: c_uint = 0x408;
pub const AIP_SPDIF_CTRL: c_uint = 0x40c;

pub const AIP_I2S_CTRL: c_uint = 0x410;

pub const AIP_I2S_CHST0: c_uint = 0x414;
pub const AIP_I2S_CHST1: c_uint = 0x418;
pub const AIP_TXCTRL: c_uint = 0x424;

pub const AIP_TPI_CTRL: c_uint = 0x428;

// Video downsampling configuration
pub const VID_DOWNSAMPLE_CONFIG: c_uint = 0x8d0;

pub const VID_OUT_FORMAT: c_uint = 0x8fc;

// HDCP registers
pub const HDCP_TOP_CTRL: c_uint = 0xc00;
pub const HDCP2X_CTRL_0: c_uint = 0xc20;

pub const HDCP2X_POL_CTRL: c_uint = 0xc54;

pub const HDCP1X_CTRL: c_uint = 0xcd0;

// HDMI DDC registers
pub const HPD_DDC_CTRL: c_uint = 0xc08;

pub const DDC_CTRL: c_uint = 0xc10;

pub const SCDC_CTRL: c_uint = 0xc18;

pub const HPD_DDC_STATUS: c_uint = 0xc60;

pub const HPD_STATE_CONNECTED: c_int = 2;

pub const SI2C_CTRL: c_uint = 0xcac;

// HDCP DDC registers
pub const HDCP2X_DDCM_STATUS: c_uint = 0xc68;

// HDMI TX registers
pub const HDMITX_CONFIG_MT8188: c_uint = 0xea0;
pub const HDMITX_CONFIG_MT8195: c_uint = 0x900;

//
// enum mtk_hdmi_ddc_v2_cmds - DDC_CMD register commands
// @DDC_CMD_READ_NOACK:      Current address read with no ACK on last byte
// @DDC_CMD_READ:            Current address read with ACK on last byte
// @DDC_CMD_SEQ_READ_NOACK:  Sequential read with no ACK on last byte
// @DDC_CMD_SEQ_READ:        Sequential read with ACK on last byte
// @DDC_CMD_ENH_READ_NOACK:  Enhanced read with no ACK on last byte
// @DDC_CMD_ENH_READ:        Enhanced read with ACK on last byte
// @DDC_CMD_SEQ_WRITE_NOACK: Sequential write ignoring ACK on last byte
// @DDC_CMD_SEQ_WRITE:       Sequential write requiring ACK on last byte
// @DDC_CMD_RSVD:            Reserved for future use
// @DDC_CMD_CLEAR_FIFO:      Clear DDC I2C FIFO
// @DDC_CMD_CLOCK_SCL:       Start clocking DDC I2C SCL
// @DDC_CMD_ABORT_XFER:      Abort DDC I2C transaction
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mtk_hdmi_ddc_v2_cmds {
    DDC_CMD_READ_NOACK = 0x0,
    DDC_CMD_READ,
    DDC_CMD_SEQ_READ_NOACK,
    DDC_CMD_SEQ_READ,
    DDC_CMD_ENH_READ_NOACK,
    DDC_CMD_ENH_READ,
    DDC_CMD_SEQ_WRITE_NOACK,
    DDC_CMD_SEQ_WRITE = 0x07,
    DDC_CMD_CLEAR_FIFO = 0x09,
    DDC_CMD_CLOCK_SCL = 0x0a,
    DDC_CMD_ABORT_XFER = 0x0f
}
