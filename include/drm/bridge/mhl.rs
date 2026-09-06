//! Automatically rewritten from C Header to Rust Module
//! Source: include/drm/bridge/mhl.h
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
// Defines for Mobile High-Definition Link (MHL) interface
//
// Copyright (C) 2015, Samsung Electronics, Co., Ltd.
// Andrzej Hajda <a.hajda@samsung.com>
//
// Based on MHL driver for Android devices.
// Copyright (C) 2013-2014 Silicon Image, Inc.
//

// Device Capabilities Registers
pub const MHL_DCAP_CAT_SINK: c_uint = 0x01;
pub const MHL_DCAP_CAT_SOURCE: c_uint = 0x02;
pub const MHL_DCAP_CAT_POWER: c_uint = 0x10;

pub const MHL_DCAP_VID_LINK_RGB444: c_uint = 0x01;
pub const MHL_DCAP_VID_LINK_YCBCR444: c_uint = 0x02;
pub const MHL_DCAP_VID_LINK_YCBCR422: c_uint = 0x04;
pub const MHL_DCAP_VID_LINK_PPIXEL: c_uint = 0x08;
pub const MHL_DCAP_VID_LINK_ISLANDS: c_uint = 0x10;
pub const MHL_DCAP_VID_LINK_VGA: c_uint = 0x20;
pub const MHL_DCAP_VID_LINK_16BPP: c_uint = 0x40;
pub const MHL_DCAP_AUD_LINK_2CH: c_uint = 0x01;
pub const MHL_DCAP_AUD_LINK_8CH: c_uint = 0x02;
pub const MHL_DCAP_VT_GRAPHICS: c_uint = 0x00;
pub const MHL_DCAP_VT_PHOTO: c_uint = 0x02;
pub const MHL_DCAP_VT_CINEMA: c_uint = 0x04;
pub const MHL_DCAP_VT_GAMES: c_uint = 0x08;
pub const MHL_DCAP_SUPP_VT: c_uint = 0x80;
pub const MHL_DCAP_LD_DISPLAY: c_uint = 0x01;
pub const MHL_DCAP_LD_VIDEO: c_uint = 0x02;
pub const MHL_DCAP_LD_AUDIO: c_uint = 0x04;
pub const MHL_DCAP_LD_MEDIA: c_uint = 0x08;
pub const MHL_DCAP_LD_TUNER: c_uint = 0x10;
pub const MHL_DCAP_LD_RECORD: c_uint = 0x20;
pub const MHL_DCAP_LD_SPEAKER: c_uint = 0x40;
pub const MHL_DCAP_LD_GUI: c_uint = 0x80;
pub const MHL_DCAP_LD_ALL: c_uint = 0xFF;
pub const MHL_DCAP_FEATURE_RCP_SUPPORT: c_uint = 0x01;
pub const MHL_DCAP_FEATURE_RAP_SUPPORT: c_uint = 0x02;
pub const MHL_DCAP_FEATURE_SP_SUPPORT: c_uint = 0x04;
pub const MHL_DCAP_FEATURE_UCP_SEND_SUPPOR: c_uint = 0x08;
pub const MHL_DCAP_FEATURE_UCP_RECV_SUPPORT: c_uint = 0x10;
pub const MHL_DCAP_FEATURE_RBP_SUPPORT: c_uint = 0x40;
// Extended Device Capabilities Registers
pub const MHL_XDC_ECBUS_S_075: c_uint = 0x01;
pub const MHL_XDC_ECBUS_S_8BIT: c_uint = 0x02;
pub const MHL_XDC_ECBUS_S_12BIT: c_uint = 0x04;
pub const MHL_XDC_ECBUS_D_150: c_uint = 0x10;
pub const MHL_XDC_ECBUS_D_8BIT: c_uint = 0x20;
pub const MHL_XDC_TMDS_000: c_uint = 0x00;
pub const MHL_XDC_TMDS_150: c_uint = 0x01;
pub const MHL_XDC_TMDS_300: c_uint = 0x02;
pub const MHL_XDC_TMDS_600: c_uint = 0x04;
// MHL_XDC_ECBUS_ROLES flags
pub const MHL_XDC_DEV_HOST: c_uint = 0x01;
pub const MHL_XDC_DEV_DEVICE: c_uint = 0x02;
pub const MHL_XDC_DEV_CHARGER: c_uint = 0x04;
pub const MHL_XDC_HID_HOST: c_uint = 0x08;
pub const MHL_XDC_HID_DEVICE: c_uint = 0x10;
// MHL_XDC_LOG_DEV_MAPX flags
pub const MHL_XDC_LD_PHONE: c_uint = 0x01;
// Device Status Registers
// Offset of DEVSTAT registers
pub const MHL_DST_OFFSET: c_uint = 0x30;

pub const MHL_DST_CONN_DCAP_RDY: c_uint = 0x01;
pub const MHL_DST_CONN_XDEVCAPP_SUPP: c_uint = 0x02;
pub const MHL_DST_CONN_POW_STAT: c_uint = 0x04;
pub const MHL_DST_CONN_PLIM_STAT_MASK: c_uint = 0x38;
pub const MHL_DST_LM_CLK_MODE_MASK: c_uint = 0x07;
pub const MHL_DST_LM_CLK_MODE_PACKED_PIXEL: c_uint = 0x02;
pub const MHL_DST_LM_CLK_MODE_NORMAL: c_uint = 0x03;
pub const MHL_DST_LM_PATH_EN_MASK: c_uint = 0x08;
pub const MHL_DST_LM_PATH_ENABLED: c_uint = 0x08;
pub const MHL_DST_LM_PATH_DISABLED: c_uint = 0x00;
pub const MHL_DST_LM_MUTED_MASK: c_uint = 0x10;
// Extended Device Status Registers
// Offset of XDEVSTAT registers
pub const MHL_XDS_OFFSET: c_uint = 0x90;

// MHL_XDS_REG_CURR_ECBUS_MODE flags
pub const MHL_XDS_SLOT_MODE_8BIT: c_uint = 0x00;
pub const MHL_XDS_SLOT_MODE_6BIT: c_uint = 0x01;
pub const MHL_XDS_ECBUS_S: c_uint = 0x04;
pub const MHL_XDS_ECBUS_D: c_uint = 0x08;
pub const MHL_XDS_LINK_CLOCK_75MHZ: c_uint = 0x00;
pub const MHL_XDS_LINK_CLOCK_150MHZ: c_uint = 0x10;
pub const MHL_XDS_LINK_CLOCK_300MHZ: c_uint = 0x20;
pub const MHL_XDS_LINK_CLOCK_600MHZ: c_uint = 0x30;
pub const MHL_XDS_LINK_STATUS_NO_SIGNAL: c_uint = 0x00;
pub const MHL_XDS_LINK_STATUS_CRU_LOCKED: c_uint = 0x01;
pub const MHL_XDS_LINK_STATUS_TMDS_NORMAL: c_uint = 0x02;
pub const MHL_XDS_LINK_STATUS_TMDS_RESERVED: c_uint = 0x03;
pub const MHL_XDS_LINK_RATE_1_5_GBPS: c_uint = 0x00;
pub const MHL_XDS_LINK_RATE_3_0_GBPS: c_uint = 0x01;
pub const MHL_XDS_LINK_RATE_6_0_GBPS: c_uint = 0x02;
pub const MHL_XDS_ATT_CAPABLE: c_uint = 0x08;
pub const MHL_XDS_SINK_STATUS_1_HPD_LOW: c_uint = 0x00;
pub const MHL_XDS_SINK_STATUS_1_HPD_HIGH: c_uint = 0x01;
pub const MHL_XDS_SINK_STATUS_2_HPD_LOW: c_uint = 0x00;
pub const MHL_XDS_SINK_STATUS_2_HPD_HIGH: c_uint = 0x04;
pub const MHL_XDS_SINK_STATUS_3_HPD_LOW: c_uint = 0x00;
pub const MHL_XDS_SINK_STATUS_3_HPD_HIGH: c_uint = 0x10;
pub const MHL_XDS_SINK_STATUS_4_HPD_LOW: c_uint = 0x00;
pub const MHL_XDS_SINK_STATUS_4_HPD_HIGH: c_uint = 0x40;
// Interrupt Registers
// Offset of DEVSTAT registers
pub const MHL_INT_OFFSET: c_uint = 0x20;

pub const MHL_INT_RC_DCAP_CHG: c_uint = 0x01;
pub const MHL_INT_RC_DSCR_CHG: c_uint = 0x02;
pub const MHL_INT_RC_REQ_WRT: c_uint = 0x04;
pub const MHL_INT_RC_GRT_WRT: c_uint = 0x08;
pub const MHL_INT_RC_3D_REQ: c_uint = 0x10;
pub const MHL_INT_RC_FEAT_REQ: c_uint = 0x20;
pub const MHL_INT_RC_FEAT_COMPLETE: c_uint = 0x40;
pub const MHL_INT_DC_EDID_CHG: c_uint = 0x02;
// let the rest of these float, they are software specific
// MSC message types
// RAP action codes
pub const MHL_RAP_POLL: c_uint = 0x00	/* Just do an ack */;
pub const MHL_RAP_CONTENT_ON: c_uint = 0x10	/* Turn content stream ON */;
pub const MHL_RAP_CONTENT_OFF: c_uint = 0x11	/* Turn content stream OFF */;
pub const MHL_RAP_CBUS_MODE_DOWN: c_uint = 0x20;
pub const MHL_RAP_CBUS_MODE_UP: c_uint = 0x21;
// RAPK status codes
pub const MHL_RAPK_NO_ERR: c_uint = 0x00	/* RAP action recognized & supported */;
pub const MHL_RAPK_UNRECOGNIZED: c_uint = 0x01	/* Unknown RAP action code received */;
pub const MHL_RAPK_UNSUPPORTED: c_uint = 0x02	/* Rcvd RAP action code not supported */;
pub const MHL_RAPK_BUSY: c_uint = 0x03	/* Responder too busy to respond */;
// Bit masks for RCP messages
pub const MHL_RCP_KEY_RELEASED_MASK: c_uint = 0x80;
pub const MHL_RCP_KEY_ID_MASK: c_uint = 0x7F;
//
// Error status codes for RCPE messages
//
// No error. (Not allowed in RCPE messages)
pub const MHL_RCPE_STATUS_NO_ERROR: c_uint = 0x00;
// Unsupported/unrecognized key code
pub const MHL_RCPE_STATUS_INEFFECTIVE_KEY_CODE: c_uint = 0x01;
// Responder busy. Initiator may retry message
pub const MHL_RCPE_STATUS_BUSY: c_uint = 0x02;
//
// Error status codes for RBPE messages
//
// No error. (Not allowed in RBPE messages)
pub const MHL_RBPE_STATUS_NO_ERROR: c_uint = 0x00;
// Unsupported/unrecognized button code
pub const MHL_RBPE_STATUS_INEFFECTIVE_BUTTON_CODE: c_uint = 0x01;
// Responder busy. Initiator may retry message
pub const MHL_RBPE_STATUS_BUSY: c_uint = 0x02;
//
// Error status codes for UCPE messages
//
// No error. (Not allowed in UCPE messages)
pub const MHL_UCPE_STATUS_NO_ERROR: c_uint = 0x00;
// Unsupported/unrecognized key code
pub const MHL_UCPE_STATUS_INEFFECTIVE_KEY_CODE: c_uint = 0x01;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mhl_burst_id {
    MHL_BURST_ID_3D_VIC = 0x10,
    MHL_BURST_ID_3D_DTD = 0x11,
    MHL_BURST_ID_HEV_VIC = 0x20,
    MHL_BURST_ID_HEV_DTDA = 0x21,
    MHL_BURST_ID_HEV_DTDB = 0x22,
    MHL_BURST_ID_VC_ASSIGN = 0x38,
    MHL_BURST_ID_VC_CONFIRM = 0x39,
    MHL_BURST_ID_AUD_DELAY = 0x40,
    MHL_BURST_ID_ADT_BURSTID = 0x41,
    MHL_BURST_ID_BIST_SETUP = 0x51,
    MHL_BURST_ID_BIST_RETURN_STAT = 0x52,
    MHL_BURST_ID_EMSC_SUPPORT = 0x61,
    MHL_BURST_ID_HID_PAYLOAD = 0x62,
    MHL_BURST_ID_BLK_RCV_BUFFER_INFO = 0x63,
    MHL_BURST_ID_BITS_PER_PIXEL_FMT = 0x64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mhl_burst_blk_rcv_buffer_info {
    pub id: __be16,
    pub size: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mhl3_burst_header {
    pub id: __be16,
    pub checksum: u8,
    pub total_entries: u8,
    pub sequence_index: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mhl_burst_bits_per_pixel_fmt {
    pub hdr: mhl3_burst_header,
    pub num_entries: u8,
    pub stream_id: u8,
    pub pixel_format: u8,
    pub desc: [} __packed; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mhl_burst_emsc_support {
    pub hdr: mhl3_burst_header,
    pub num_entries: u8,
    pub burst_id: [__be16; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mhl_burst_audio_descr {
    pub hdr: mhl3_burst_header,
    pub flags: u8,
    pub short_desc: [u8; 9],
    pub __packed: },
//
// MHL3 infoframe related definitions
//
pub const MHL3_IEEE_OUI: c_uint = 0x7ca61d;
pub const MHL3_INFOFRAME_SIZE: c_int = 15;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mhl3_video_format {
    MHL3_VIDEO_FORMAT_NONE,
    MHL3_VIDEO_FORMAT_3D,
    MHL3_VIDEO_FORMAT_MULTI_VIEW,
    MHL3_VIDEO_FORMAT_DUAL_3D
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mhl3_3d_format_type {
    MHL3_3D_FORMAT_TYPE_FS, /* frame sequential */
    MHL3_3D_FORMAT_TYPE_TB, /* top-bottom */
    MHL3_3D_FORMAT_TYPE_LR, /* left-right */
    MHL3_3D_FORMAT_TYPE_FS_TB, /* frame sequential, top-bottom */
    MHL3_3D_FORMAT_TYPE_FS_LR, /* frame sequential, left-right */
    MHL3_3D_FORMAT_TYPE_TB_LR /* top-bottom, left-right */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mhl3_infoframe {
    pub version: c_uchar,
    pub video_format: mhl3_video_format,
    pub format_type: mhl3_3d_format_type,
    pub sep_audio: bool,
    pub hev_format: c_int,
    pub av_delay: c_int,
}
