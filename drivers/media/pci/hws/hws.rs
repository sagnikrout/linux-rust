//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/pci/hws/hws.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hwsmem_param {
    pub index: u32,
    pub type: u32,
    pub status: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hws_pix_state {
    pub width: u32,
    pub height: u32,
    pub /: *mut *mut *mut u32 fourcc; / V4L2_PIX_FMT_ (YUYV only here),
    pub /: *mut *mut u32 bytesperline; / stride,
    pub /: *mut *mut u32 sizeimage; / full frame,
    pub /: *mut *mut v4l2_field field; / V4L2_FIELD_NONE or INTERLACED,
    pub /: *mut *mut v4l2_colorspace colorspace; / e.g., REC709,
    pub /: *mut *mut v4l2_ycbcr_encoding ycbcr_enc; / V4L2_YCBCR_ENC_DEFAULT,
    pub /: *mut *mut v4l2_quantization quantization; / V4L2_QUANTIZATION_LIM_RANGE,
    pub /: *mut *mut v4l2_xfer_func xfer_func; / V4L2_XFER_FUNC_DEFAULT,
    pub /: *mut *mut bool interlaced; / cached hardware state,
    pub /: *mut *mut u32 half_size; / hardware half-frame size,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hwsvideo_buffer {
    pub vb: vb2_v4l2_buffer,
    pub list: list_head,
    pub slot: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hws_video {
// Linkage
    pub parent: *mut hws_pcie_dev,
    pub video_device: *mut video_device,
    pub buffer_queue: vb2_queue,
    pub capture_queue: list_head,
    pub active: *mut hwsvideo_buffer,
    pub next_prepared: *mut hwsvideo_buffer,
// Locking
    pub state_lock: mutex,
    pub /: *mut *mut spinlock_t irq_lock; / Protects capture_queue and active buffers.,
// Indices
    pub channel_index: c_int,
// Color controls
    pub current_brightness: c_int,
    pub current_contrast: c_int,
    pub current_saturation: c_int,
    pub current_hue: c_int,
// V4L2 controls
    pub control_handler: v4l2_ctrl_handler,
    pub ctrl_brightness: *mut v4l2_ctrl,
    pub ctrl_contrast: *mut v4l2_ctrl,
    pub ctrl_saturation: *mut v4l2_ctrl,
    pub ctrl_hue: *mut v4l2_ctrl,
// Capture queue status
    pub pix: hws_pix_state,
    pub /: *mut *mut v4l2_dv_timings cur_dv_timings; / last configured/notified DV timings,
    pub /: *mut *mut u32 current_fps; / Hz, updated by mode changes, not by read-only queries,
// Per-channel capture state
    pub cap_active: bool,
    pub stop_requested: bool,
    pub last_buf_half_toggle: u8,
    pub half_seen: bool,
    pub sequence_number: core::sync::atomic::AtomicI32,
    pub queued_count: u32,
// Timeout and error handling
    pub timeout_count: u32,
    pub error_count: u32,
    pub window_valid: bool,
    pub last_dma_hi: u32,
    pub last_dma_page: u32,
    pub last_pci_addr: u32,
    pub last_half16: u32,
// Misc counters
    pub signal_loss_cnt: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hws_scratch_dma {
    pub cpu: *mut c_void,
    pub dma: dma_addr_t,
    pub size: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hws_pcie_dev {
// Core objects
    pub pdev: *mut pci_dev,
    pub video: [hws_video; MAX_VID_CHANNELS],
// BAR and workqueues
    pub bar0_base: *mut void __iomem,
// Device identity and capabilities
    pub vendor_id: u16,
    pub device_id: u16,
    pub device_ver: u16,
    pub hw_ver: u16,
    pub sub_ver: u32,
    pub port_id: u32,
// Tri-state support flag used by set_video_format_size().
    pub support_yv12: u32,
    pub max_hw_video_buf_sz: u32,
    pub max_channels: u8,
    pub cur_max_video_ch: u8,
    pub start_run: bool,
    pub buf_allocated: bool,
// V4L2 framework objects
    pub v4l2_device: v4l2_device,
// Kernel thread
    pub main_task: *mut task_struct,
    pub scratch_vid: [hws_scratch_dma; MAX_VID_CHANNELS],
    pub suspended: bool,
    pub irq: c_int,
// Error flags
    pub pci_lost: c_int,
}
