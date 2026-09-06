//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/pci/intel/ipu6/ipu6-isys.h
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
// Copyright (C) 2013--2024 Intel Corporation

// FW support max 16 streams
pub const IPU6_ISYS_MAX_STREAMS: c_int = 16;

//
// Current message queue configuration. These must be big enough
// so that they never gets full. Queues are located in system memory
//
pub const IPU6_ISYS_SIZE_RECV_QUEUE: c_int = 40;
pub const IPU6_ISYS_SIZE_SEND_QUEUE: c_int = 40;
pub const IPU6_ISYS_SIZE_PROXY_RECV_QUEUE: c_int = 5;
pub const IPU6_ISYS_SIZE_PROXY_SEND_QUEUE: c_int = 5;
pub const IPU6_ISYS_NUM_RECV_QUEUE: c_int = 1;

// the threshold granularity is 2KB on IPU6
pub const IPU6_SRAM_GRANULARITY_SHIFT: c_int = 11;
pub const IPU6_SRAM_GRANULARITY_SIZE: c_int = 2048;
// the threshold granularity is 1KB on IPU6SE
pub const IPU6SE_SRAM_GRANULARITY_SHIFT: c_int = 10;
pub const IPU6SE_SRAM_GRANULARITY_SIZE: c_int = 1024;
// IS pixel buffer is 256KB, MaxSRAMSize is 200KB on IPU6

// IS pixel buffer is 128KB, MaxSRAMSize is 96KB on IPU6SE

pub const IPU6EP_LTR_VALUE: c_int = 200;
pub const IPU6EP_MIN_MEMOPEN_TH: c_uint = 0x4;
pub const IPU6EP_MTL_LTR_VALUE: c_int = 1023;
pub const IPU6EP_MTL_MIN_MEMOPEN_TH: c_uint = 0xc;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ltr_did {
    pub value: u32,
    pub val0: u8,
    pub val1: u8,
    pub val2: u8,
    pub val3: u8,
    pub bits: },
    pub lut_ltr: },
    pub value: u32,
    pub th0: u8,
    pub th1: u8,
    pub th2: u8,
    pub th3: u8,
    pub bits: },
    pub lut_fill_time: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct isys_iwake_watermark {
    pub iwake_enabled: bool,
    pub force_iwake_disable: bool,
    pub iwake_threshold: u32,
    pub isys_pixelbuffer_datarate: u64,
    pub ltrdid: ltr_did,
    pub /: *mut *mut mutex mutex; / protect whole struct,
    pub isys: *mut ipu6_isys,
    pub video_list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipu6_isys_csi2_config {
    pub nlanes: u32,
    pub port: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sensor_async_sd {
    pub asc: v4l2_async_connection,
    pub csi2: ipu6_isys_csi2_config,
}

//
// struct ipu6_isys
//
// @media_dev: Media device
// @v4l2_dev: V4L2 device
// @adev: ISYS bus device
// @power: Is ISYS powered on or not?
// @isr_bits: Which bits does the ISR handle?
// @power_lock: Serialise access to power (power state in general)
// @csi2_rx_ctrl_cached: cached shared value between all CSI2 receivers
// @streams_lock: serialise access to streams
// @streams: streams per firmware stream ID
// @fwcom: fw communication layer private pointer
// or optional external library private pointer
// @phy_termcal_val: the termination calibration value, only used for DWC PHY
// @need_reset: Isys requires d0i0->i3 transition
// @ref_count: total number of callers fw open
// @mutex: serialise access isys video open/release related operations
// @stream_mutex: serialise stream start and stop, queueing requests
// @pdata: platform data pointer
// @csi2: CSI-2 receivers
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipu6_isys {
    pub media_dev: media_device,
    pub v4l2_dev: v4l2_device,
    pub adev: *mut ipu6_bus_device,
    pub power: c_int,
    pub power_lock: spinlock_t,
    pub isr_csi2_bits: u32,
    pub csi2_rx_ctrl_cached: u32,
    pub streams_lock: spinlock_t,
    pub streams: [ipu6_isys_stream; IPU6_ISYS_MAX_STREAMS],
    pub streams_ref_count: [c_int; IPU6_ISYS_MAX_STREAMS],
    pub fwcom: *mut c_void,
    pub phy_termcal_val: u32,
    pub need_reset: bool,
    pub icache_prefetch: bool,
    pub csi2_cse_ipc_not_supported: bool,
    pub ref_count: c_uint,
    pub stream_opened: c_uint,
    pub sensor_type: c_uint,
    pub mutex: mutex,
    pub stream_mutex: mutex,
    pub pdata: *mut ipu6_isys_pdata,
    pub on): bool,
    pub csi2: *mut ipu6_isys_csi2,
    pub pm_qos: pm_qos_request,
    pub /: *mut *mut spinlock_t listlock; / Protect framebuflist,
    pub framebuflist: list_head,
    pub framebuflist_fw: list_head,
    pub notifier: v4l2_async_notifier,
    pub iwake_watermark: isys_iwake_watermark,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct isys_fw_msgs {
    pub dummy: u64,
    pub frame: ipu6_fw_isys_frame_buff_set_abi,
    pub stream: ipu6_fw_isys_stream_cfg_data_abi,
    pub fw_msg: },
    pub head: list_head,
    pub dma_addr: dma_addr_t,
}

extern "C" {
    pub fn ipu6_put_fw_msg_buf(isys: *mut ipu6_isys, data: uintptr_t);
}
extern "C" {
    pub fn ipu6_cleanup_fw_msg_bufs(isys: *mut ipu6_isys);
}
extern "C" {
    pub fn update_watermark_setting(isys: *mut ipu6_isys);
}
