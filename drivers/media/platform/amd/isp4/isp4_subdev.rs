//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/amd/isp4/isp4_subdev.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// Copyright (C) 2025 Advanced Micro Devices, Inc.
//

//
// One is for none sensor specific response which is not used now.
// Another is for sensor specific response
//
pub const ISP4SD_MAX_FW_RESP_STREAM_NUM: c_int = 2;
// Indicates the ISP status
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum isp4sd_status {
    ISP4SD_STATUS_PWR_OFF,
    ISP4SD_STATUS_PWR_ON,
    ISP4SD_STATUS_FW_RUNNING,
    ISP4SD_STATUS_MAX
}

// Indicates sensor and output stream status
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum isp4sd_start_status {
    ISP4SD_START_STATUS_OFF,
    ISP4SD_START_STATUS_STARTED,
    ISP4SD_START_STATUS_START_FAIL,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct isp4sd_img_buf_node {
    pub node: list_head,
    pub buf_info: isp4if_img_buf_info,
}

// This is ISP output after processing Bayer raw sensor input
#[repr(C)]
#[derive(Copy, Clone)]
pub struct isp4sd_output_info {
    pub start_status: isp4sd_start_status,
    pub image_size: u32,
}

//
// Struct for sensor info used as ISP input or source.
// status: sensor status.
// output_info: ISP output after processing the sensor input.
// start_stream_cmd_sent: indicates if ISP4FW_CMD_ID_START_STREAM was sent
// to firmware.
// buf_sent_cnt: number of buffers sent to receive images.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct isp4sd_sensor_info {
    pub output_info: isp4sd_output_info,
    pub status: isp4sd_start_status,
    pub start_stream_cmd_sent: bool,
    pub buf_sent_cnt: u32,
}

//
// The thread is created by the driver to handle firmware responses which will
// be waken up when a firmware-to-driver response interrupt occurs.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct isp4sd_thread_handler {
    pub thread: *mut task_struct,
    pub waitq: wait_queue_head_t,
    pub resp_ready: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct isp4_subdev_thread_param {
    pub idx: u32,
    pub isp_subdev: *mut isp4_subdev,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct isp4_subdev {
    pub sdev: v4l2_subdev,
    pub ispif: isp4_interface,
    pub isp_vdev: isp4vid_dev,
    pub sdev_pad: media_pad,
    pub isp_status: isp4sd_status,
// mutex used to synchronize the operation with firmware
    pub ops_mutex: mutex,
    pub host2fw_seq_num: u32,
    pub sensor_info: isp4sd_sensor_info,
// gpio descriptor
    pub enable_gpio: *mut gpio_desc,
    pub dev: *mut device,
    pub mmio: *mut void __iomem,
    pub irq: [c_int; ISP4SD_MAX_FW_RESP_STREAM_NUM],
    pub irq_enabled: bool,
// spin lock to access ISP_SYS_INT0_EN exclusively
    pub irq_lock: spinlock_t,

    pub enable_fw_log: bool,
    pub debugfs_dir: *mut dentry,
    pub fw_log_output: *mut c_char,

}

extern "C" {
    pub fn isp4sd_deinit(isp_subdev: *mut isp4_subdev);
}
extern "C" {
    pub fn isp4sd_pwron_and_init(sd: *mut v4l2_subdev) -> c_int;
}
extern "C" {
    pub fn isp4sd_pwroff_and_deinit(sd: *mut v4l2_subdev) -> c_int;
}
