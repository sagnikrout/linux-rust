//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/dma/k3-psil.h
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
// Copyright (C) 2019 Texas Instruments Incorporated - https://www.ti.com
//

pub const K3_PSIL_DST_THREAD_ID_OFFSET: c_uint = 0x8000;
//
// enum udma_tp_level - Channel Throughput Levels
// @UDMA_TP_NORMAL:	Normal channel
// @UDMA_TP_HIGH:	High Throughput channel
// @UDMA_TP_ULTRAHIGH:	Ultra High Throughput channel
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum udma_tp_level {
    UDMA_TP_NORMAL = 0,
    UDMA_TP_HIGH,
    UDMA_TP_ULTRAHIGH,
    UDMA_TP_LAST,
}

//
// enum psil_endpoint_type - PSI-L Endpoint type
// @PSIL_EP_NATIVE:	Normal channel
// @PSIL_EP_PDMA_XY:	XY mode PDMA
// @PSIL_EP_PDMA_MCAN:	MCAN mode PDMA
// @PSIL_EP_PDMA_AASRC: AASRC mode PDMA
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum psil_endpoint_type {
    PSIL_EP_NATIVE = 0,
    PSIL_EP_PDMA_XY,
    PSIL_EP_PDMA_MCAN,
    PSIL_EP_PDMA_AASRC,
}

//
// struct psil_endpoint_config - PSI-L Endpoint configuration
// @ep_type:		PSI-L endpoint type
// @channel_tpl:	Desired throughput level for the channel
// @pkt_mode:		If set, the channel must be in Packet mode, otherwise in
// TR mode
// @notdpkt:		TDCM must be suppressed on the TX channel
// @needs_epib:		Endpoint needs EPIB
// @pdma_acc32:		ACC32 must be enabled on the PDMA side
// @pdma_burst:		BURST must be enabled on the PDMA side
// @psd_size:		If set, PSdata is used by the endpoint
// @mapped_channel_id:	PKTDMA thread to channel mapping for mapped channels.
// The thread must be serviced by the specified channel if
// mapped_channel_id is >= 0 in case of PKTDMA
// @flow_start:		PKDMA flow range start of mapped channel. Unmapped
// channels use flow_id == chan_id
// @flow_num:		PKDMA flow count of mapped channel. Unmapped channels
// use flow_id == chan_id
// @default_flow_id:	PKDMA default (r)flow index of mapped channel.
// Must be within the flow range of the mapped channel.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct psil_endpoint_config {
    pub ep_type: psil_endpoint_type,
    pub channel_tpl: udma_tp_level,
    pub pkt_mode:1: unsigned,
    pub notdpkt:1: unsigned,
    pub needs_epib:1: unsigned,
// PDMA properties, valid for PSIL_EP_PDMA_*
    pub pdma_acc32:1: unsigned,
    pub pdma_burst:1: unsigned,
    pub psd_size: u32,
// PKDMA mapped channel
    pub mapped_channel_id: i16,
// PKTDMA tflow and rflow ranges for mapped channel
    pub flow_start: u16,
    pub flow_num: u16,
    pub default_flow_id: i16,
}
