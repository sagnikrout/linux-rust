//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/ti/vpe/vpdma_priv.h
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
// Copyright (c) 2013 Texas Instruments Inc.
//
// David Griego, <dagriego@biglakesoftware.com>
// Dale Farnsworth, <dale@farnsworth.org>
// Archit Taneja, <archit@ti.com>
//
// VPDMA Register offsets
//
// Top level
pub const VPDMA_PID: c_uint = 0x00;
pub const VPDMA_LIST_ADDR: c_uint = 0x04;
pub const VPDMA_LIST_ATTR: c_uint = 0x08;
pub const VPDMA_LIST_STAT_SYNC: c_uint = 0x0c;
pub const VPDMA_BG_RGB: c_uint = 0x18;
pub const VPDMA_BG_YUV: c_uint = 0x1c;
pub const VPDMA_SETUP: c_uint = 0x30;
pub const VPDMA_MAX_SIZE1: c_uint = 0x34;
pub const VPDMA_MAX_SIZE2: c_uint = 0x38;
pub const VPDMA_MAX_SIZE3: c_uint = 0x3c;
pub const VPDMA_MAX_SIZE_WIDTH_MASK: c_uint = 0xffff;
pub const VPDMA_MAX_SIZE_WIDTH_SHFT: c_int = 16;
pub const VPDMA_MAX_SIZE_HEIGHT_MASK: c_uint = 0xffff;
pub const VPDMA_MAX_SIZE_HEIGHT_SHFT: c_int = 0;
// Interrupts

pub const VPDMA_INT_CLIENT0_STAT: c_uint = 0x78;
pub const VPDMA_INT_CLIENT0_MASK: c_uint = 0x7c;
pub const VPDMA_INT_CLIENT1_STAT: c_uint = 0x80;
pub const VPDMA_INT_CLIENT1_MASK: c_uint = 0x84;
pub const VPDMA_INT_LIST0_STAT: c_uint = 0x88;
pub const VPDMA_INT_LIST0_MASK: c_uint = 0x8c;
pub const VPDMA_INTX_OFFSET: c_uint = 0x50;

// VIP/VPE client registers
pub const VPDMA_DEI_CHROMA1_CSTAT: c_uint = 0x0300;
pub const VPDMA_DEI_LUMA1_CSTAT: c_uint = 0x0304;
pub const VPDMA_DEI_LUMA2_CSTAT: c_uint = 0x0308;
pub const VPDMA_DEI_CHROMA2_CSTAT: c_uint = 0x030c;
pub const VPDMA_DEI_LUMA3_CSTAT: c_uint = 0x0310;
pub const VPDMA_DEI_CHROMA3_CSTAT: c_uint = 0x0314;
pub const VPDMA_DEI_MV_IN_CSTAT: c_uint = 0x0330;
pub const VPDMA_DEI_MV_OUT_CSTAT: c_uint = 0x033c;
pub const VPDMA_VIP_LO_Y_CSTAT: c_uint = 0x0388;
pub const VPDMA_VIP_LO_UV_CSTAT: c_uint = 0x038c;
pub const VPDMA_VIP_UP_Y_CSTAT: c_uint = 0x0390;
pub const VPDMA_VIP_UP_UV_CSTAT: c_uint = 0x0394;
pub const VPDMA_VPI_CTL_CSTAT: c_uint = 0x03d0;
// Reg field info for VPDMA_CLIENT_CSTAT registers
pub const VPDMA_CSTAT_LINE_MODE_MASK: c_uint = 0x03;
pub const VPDMA_CSTAT_LINE_MODE_SHIFT: c_int = 8;
pub const VPDMA_CSTAT_FRAME_START_MASK: c_uint = 0xf;
pub const VPDMA_CSTAT_FRAME_START_SHIFT: c_int = 10;
pub const VPDMA_LIST_NUM_MASK: c_uint = 0x07;
pub const VPDMA_LIST_NUM_SHFT: c_int = 24;
pub const VPDMA_LIST_STOP_SHFT: c_int = 20;
pub const VPDMA_LIST_RDY_MASK: c_uint = 0x01;
pub const VPDMA_LIST_RDY_SHFT: c_int = 19;
pub const VPDMA_LIST_TYPE_MASK: c_uint = 0x03;
pub const VPDMA_LIST_TYPE_SHFT: c_int = 16;
pub const VPDMA_LIST_SIZE_MASK: c_uint = 0xffff;
//
// The YUV data type definition below are taken from
// both the TRM and i839 Errata information.
// Use the correct data type considering byte
// reordering of components.
//
// Also since the single use of "C" in the 422 case
// to mean "Cr" (i.e. V component). It was decided
// to explicitly label them CR to remove any confusion.
// Bear in mind that the type label refer to the memory
// packed order (LSB - MSB).
//
pub const DATA_TYPE_Y444: c_uint = 0x0;
pub const DATA_TYPE_Y422: c_uint = 0x1;
pub const DATA_TYPE_Y420: c_uint = 0x2;
pub const DATA_TYPE_C444: c_uint = 0x4;
pub const DATA_TYPE_C422: c_uint = 0x5;
pub const DATA_TYPE_C420: c_uint = 0x6;
pub const DATA_TYPE_CB420: c_uint = 0x16;
pub const DATA_TYPE_YC444: c_uint = 0x8;
pub const DATA_TYPE_YCB422: c_uint = 0x7;
pub const DATA_TYPE_YCR422: c_uint = 0x17;
pub const DATA_TYPE_CBY422: c_uint = 0x27;
pub const DATA_TYPE_CRY422: c_uint = 0x37;
//
// The RGB data type definition below are defined
// to follow Errata i819.
// The initial values were taken from:
// VPDMA_data_type_mapping_v0.2vayu_c.pdf
// But some of the ARGB definition appeared to be wrong
// in the document also. As they would yield RGBA instead.
// They have been corrected based on experimentation.
//
pub const DATA_TYPE_RGB16_565: c_uint = 0x10;
pub const DATA_TYPE_ARGB_1555: c_uint = 0x13;
pub const DATA_TYPE_ARGB_4444: c_uint = 0x14;
pub const DATA_TYPE_RGBA_5551: c_uint = 0x11;
pub const DATA_TYPE_RGBA_4444: c_uint = 0x12;
pub const DATA_TYPE_ARGB24_6666: c_uint = 0x18;
pub const DATA_TYPE_RGB24_888: c_uint = 0x16;
pub const DATA_TYPE_ARGB32_8888: c_uint = 0x17;
pub const DATA_TYPE_RGBA24_6666: c_uint = 0x15;
pub const DATA_TYPE_RGBA32_8888: c_uint = 0x19;
pub const DATA_TYPE_BGR16_565: c_uint = 0x0;
pub const DATA_TYPE_ABGR_1555: c_uint = 0x3;
pub const DATA_TYPE_ABGR_4444: c_uint = 0x4;
pub const DATA_TYPE_BGRA_5551: c_uint = 0x1;
pub const DATA_TYPE_BGRA_4444: c_uint = 0x2;
pub const DATA_TYPE_ABGR24_6666: c_uint = 0x8;
pub const DATA_TYPE_BGR24_888: c_uint = 0x6;
pub const DATA_TYPE_ABGR32_8888: c_uint = 0x7;
pub const DATA_TYPE_BGRA24_6666: c_uint = 0x5;
pub const DATA_TYPE_BGRA32_8888: c_uint = 0x9;
pub const DATA_TYPE_MV: c_uint = 0x3;
// VPDMA channel numbers, some are common between VIP/VPE and appear twice
pub const VPE_CHAN_NUM_LUMA1_IN: c_int = 0;
pub const VPE_CHAN_NUM_CHROMA1_IN: c_int = 1;
pub const VPE_CHAN_NUM_LUMA2_IN: c_int = 2;
pub const VPE_CHAN_NUM_CHROMA2_IN: c_int = 3;
pub const VPE_CHAN_NUM_LUMA3_IN: c_int = 4;
pub const VPE_CHAN_NUM_CHROMA3_IN: c_int = 5;
pub const VPE_CHAN_NUM_MV_IN: c_int = 12;
pub const VPE_CHAN_NUM_MV_OUT: c_int = 15;
pub const VIP1_CHAN_NUM_MULT_PORT_A_SRC0: c_int = 38;
pub const VIP1_CHAN_NUM_MULT_ANC_A_SRC0: c_int = 70;
pub const VPE_CHAN_NUM_LUMA_OUT: c_int = 102;
pub const VPE_CHAN_NUM_CHROMA_OUT: c_int = 103;
pub const VIP1_CHAN_NUM_PORT_A_LUMA: c_int = 102;
pub const VIP1_CHAN_NUM_PORT_A_CHROMA: c_int = 103;
pub const VPE_CHAN_NUM_RGB_OUT: c_int = 106;
pub const VIP1_CHAN_NUM_PORT_A_RGB: c_int = 106;
pub const VIP1_CHAN_NUM_PORT_B_RGB: c_int = 107;
//
// a VPDMA address data block payload for a configuration descriptor needs to
// have each sub block length as a multiple of 16 bytes. Therefore, the overall
// size of the payload also needs to be a multiple of 16 bytes. The sub block
// lengths should be ensured to be aligned by the VPDMA user.
//
pub const VPDMA_ADB_SIZE_ALIGN: c_uint = 0x0f;
//
// data transfer descriptor
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vpdma_dtd {
    pub type_ctl_stride: u32,
    pub xfer_length_height: u32,
    pub w1: u32,
}

// Data Transfer Descriptor specifics
pub const DTD_NO_NOTIFY: c_int = 0;
pub const DTD_NOTIFY: c_int = 1;
pub const DTD_PKT_TYPE: c_uint = 0xa;
pub const DTD_DIR_IN: c_int = 0;
pub const DTD_DIR_OUT: c_int = 1;
// type_ctl_stride
pub const DTD_DATA_TYPE_MASK: c_uint = 0x3f;
pub const DTD_DATA_TYPE_SHFT: c_int = 26;
pub const DTD_NOTIFY_MASK: c_uint = 0x01;
pub const DTD_NOTIFY_SHFT: c_int = 25;
pub const DTD_FIELD_MASK: c_uint = 0x01;
pub const DTD_FIELD_SHFT: c_int = 24;
pub const DTD_1D_MASK: c_uint = 0x01;
pub const DTD_1D_SHFT: c_int = 23;
pub const DTD_EVEN_LINE_SKIP_MASK: c_uint = 0x01;
pub const DTD_EVEN_LINE_SKIP_SHFT: c_int = 20;
pub const DTD_ODD_LINE_SKIP_MASK: c_uint = 0x01;
pub const DTD_ODD_LINE_SKIP_SHFT: c_int = 16;
pub const DTD_LINE_STRIDE_MASK: c_uint = 0xffff;
pub const DTD_LINE_STRIDE_SHFT: c_int = 0;
// xfer_length_height
pub const DTD_LINE_LENGTH_MASK: c_uint = 0xffff;
pub const DTD_LINE_LENGTH_SHFT: c_int = 16;
pub const DTD_XFER_HEIGHT_MASK: c_uint = 0xffff;
pub const DTD_XFER_HEIGHT_SHFT: c_int = 0;
// pkt_ctl
pub const DTD_PKT_TYPE_MASK: c_uint = 0x1f;
pub const DTD_PKT_TYPE_SHFT: c_int = 27;
pub const DTD_MODE_MASK: c_uint = 0x01;
pub const DTD_MODE_SHFT: c_int = 26;
pub const DTD_DIR_MASK: c_uint = 0x01;
pub const DTD_DIR_SHFT: c_int = 25;
pub const DTD_CHAN_MASK: c_uint = 0x01ff;
pub const DTD_CHAN_SHFT: c_int = 16;
pub const DTD_PRI_MASK: c_uint = 0x0f;
pub const DTD_PRI_SHFT: c_int = 9;
pub const DTD_NEXT_CHAN_MASK: c_uint = 0x01ff;
pub const DTD_NEXT_CHAN_SHFT: c_int = 0;
// frame_width_height
pub const DTD_FRAME_WIDTH_MASK: c_uint = 0xffff;
pub const DTD_FRAME_WIDTH_SHFT: c_int = 16;
pub const DTD_FRAME_HEIGHT_MASK: c_uint = 0xffff;
pub const DTD_FRAME_HEIGHT_SHFT: c_int = 0;
// start_h_v
pub const DTD_H_START_MASK: c_uint = 0xffff;
pub const DTD_H_START_SHFT: c_int = 16;
pub const DTD_V_START_MASK: c_uint = 0xffff;
pub const DTD_V_START_SHFT: c_int = 0;
pub const DTD_DESC_START_MASK: c_uint = 0xffffffe0;
pub const DTD_DESC_START_SHIFT: c_int = 5;
pub const DTD_WRITE_DESC_MASK: c_uint = 0x01;
pub const DTD_WRITE_DESC_SHIFT: c_int = 2;
pub const DTD_DROP_DATA_MASK: c_uint = 0x01;
pub const DTD_DROP_DATA_SHIFT: c_int = 1;
pub const DTD_USE_DESC_MASK: c_uint = 0x01;
pub const DTD_USE_DESC_SHIFT: c_int = 0;
// max_width_height
pub const DTD_MAX_WIDTH_MASK: c_uint = 0x07;
pub const DTD_MAX_WIDTH_SHFT: c_int = 4;
pub const DTD_MAX_HEIGHT_MASK: c_uint = 0x07;
pub const DTD_MAX_HEIGHT_SHFT: c_int = 0;
//
// configuration descriptor
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vpdma_cfd {
    pub dest_addr_offset: u32,
    pub w0: u32,
}

// Configuration descriptor specifics
pub const CFD_PKT_TYPE: c_uint = 0xb;
pub const CFD_DIRECT: c_int = 1;
pub const CFD_INDIRECT: c_int = 0;
pub const CFD_CLS_ADB: c_int = 0;
pub const CFD_CLS_BLOCK: c_int = 1;
// block_len
pub const CFD__BLOCK_LEN_MASK: c_uint = 0xffff;
pub const CFD__BLOCK_LEN_SHFT: c_int = 0;
// ctl_payload_len
pub const CFD_PKT_TYPE_MASK: c_uint = 0x1f;
pub const CFD_PKT_TYPE_SHFT: c_int = 27;
pub const CFD_DIRECT_MASK: c_uint = 0x01;
pub const CFD_DIRECT_SHFT: c_int = 26;
pub const CFD_CLASS_MASK: c_uint = 0x03;
pub const CFD_CLASS_SHFT: c_int = 24;
pub const CFD_DEST_MASK: c_uint = 0xff;
pub const CFD_DEST_SHFT: c_int = 16;
pub const CFD_PAYLOAD_LEN_MASK: c_uint = 0xffff;
pub const CFD_PAYLOAD_LEN_SHFT: c_int = 0;
//
// control descriptor
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vpdma_ctd {
    pub timer_value: u32,
    pub list_addr: u32,
    pub w0: u32,
}

// control descriptor types
pub const CTD_TYPE_SYNC_ON_CLIENT: c_int = 0;
pub const CTD_TYPE_SYNC_ON_LIST: c_int = 1;
pub const CTD_TYPE_SYNC_ON_EXT: c_int = 2;
pub const CTD_TYPE_SYNC_ON_LM_TIMER: c_int = 3;
pub const CTD_TYPE_SYNC_ON_CHANNEL: c_int = 4;
pub const CTD_TYPE_CHNG_CLIENT_IRQ: c_int = 5;
pub const CTD_TYPE_SEND_IRQ: c_int = 6;
pub const CTD_TYPE_RELOAD_LIST: c_int = 7;
pub const CTD_TYPE_ABORT_CHANNEL: c_int = 8;
pub const CTD_PKT_TYPE: c_uint = 0xc;
// timer_value
pub const CTD_TIMER_VALUE_MASK: c_uint = 0xffff;
pub const CTD_TIMER_VALUE_SHFT: c_int = 0;
// pixel_line_count
pub const CTD_PIXEL_COUNT_MASK: c_uint = 0xffff;
pub const CTD_PIXEL_COUNT_SHFT: c_int = 16;
pub const CTD_LINE_COUNT_MASK: c_uint = 0xffff;
pub const CTD_LINE_COUNT_SHFT: c_int = 0;
// list_size
pub const CTD_LIST_SIZE_MASK: c_uint = 0xffff;
pub const CTD_LIST_SIZE_SHFT: c_int = 0;
// event
pub const CTD_EVENT_MASK: c_uint = 0x0f;
pub const CTD_EVENT_SHFT: c_int = 0;
// fid_ctl
pub const CTD_FID2_MASK: c_uint = 0x03;
pub const CTD_FID2_SHFT: c_int = 4;
pub const CTD_FID1_MASK: c_uint = 0x03;
pub const CTD_FID1_SHFT: c_int = 2;
pub const CTD_FID0_MASK: c_uint = 0x03;
pub const CTD_FID0_SHFT: c_int = 0;
// type_source_ctl
pub const CTD_PKT_TYPE_MASK: c_uint = 0x1f;
pub const CTD_PKT_TYPE_SHFT: c_int = 27;
pub const CTD_SOURCE_MASK: c_uint = 0xff;
pub const CTD_SOURCE_SHFT: c_int = 16;
pub const CTD_CONTROL_MASK: c_uint = 0x0f;
pub const CTD_CONTROL_SHFT: c_int = 0;
