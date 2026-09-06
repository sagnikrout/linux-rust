//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/samsung/exynos4-is/media-dev.h
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
// Copyright (C) 2011 - 2012 Samsung Electronics Co., Ltd.
//

pub const FIMC_MAX_SENSORS: c_int = 4;
pub const FIMC_MAX_CAMCLKS: c_int = 2;

// LCD/ISP Writeback clocks (PIXELASYNCMx)
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fimc_subdev_index {
    IDX_SENSOR,
    IDX_CSIS,
    IDX_FLITE,
    IDX_IS_ISP,
    IDX_FIMC,
    IDX_MAX,
}

//
// This structure represents a chain of media entities, including a data
// source entity (e.g. an image sensor subdevice), a data capture entity
// - a video capture device node and any remaining entities.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fimc_pipeline {
    pub ep: exynos_media_pipeline,
    pub list: list_head,
    pub vdev_entity: *mut media_entity,
    pub subdevs: [*mut v4l2_subdev; IDX_MAX],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fimc_csis_info {
    pub sd: *mut v4l2_subdev,
    pub id: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fimc_camclk_info {
    pub clock: *mut clk,
    pub use_count: c_int,
    pub frequency: c_ulong,
}

//
// struct fimc_sensor_info - image data source subdev information
// @pdata: sensor's attributes passed as media device's platform data
// @asd: asynchronous subdev registration data structure
// @subdev: image sensor v4l2 subdev
// @host: fimc device the sensor is currently linked to
//
// This data structure applies to image sensor and the writeback subdevs.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fimc_sensor_info {
    pub pdata: fimc_source_info,
    pub asd: *mut v4l2_async_connection,
    pub subdev: *mut v4l2_subdev,
    pub host: *mut fimc_dev,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cam_clk {
    pub hw: clk_hw,
    pub fmd: *mut fimc_md,
}

//
// struct fimc_md - fimc media device information
// @csis: MIPI CSIS subdevs data
// @sensor: array of registered sensor subdevs
// @num_sensors: actual number of registered sensors
// @camclk: external sensor clock information
// @wbclk: external writeback clock information
// @fimc_lite: array of registered fimc-lite devices
// @fimc: array of registered fimc devices
// @fimc_is: fimc-is data structure
// @use_isp: set to true when FIMC-IS subsystem is used
// @pmf: handle to the CAMCLK clock control FIMC helper device
// @media_dev: top level media device
// @v4l2_dev: top level v4l2_device holding up the subdevs
// @pdev: platform device this media device is hooked up into
// @clk_provider: CAMCLK clock provider structure
// @subdev_notifier: notifier for the subdevs
// @user_subdev_api: true if subdevs are not configured by the host driver
// @slock: spinlock protecting @sensor array
// @pipelines: list of pipelines
// @link_setup_graph: graph iterator
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fimc_md {
    pub csis: [fimc_csis_info; CSIS_MAX_ENTITIES],
    pub sensor: [fimc_sensor_info; FIMC_MAX_SENSORS],
    pub num_sensors: c_int,
    pub camclk: [fimc_camclk_info; FIMC_MAX_CAMCLKS],
    pub wbclk: [*mut clk; FIMC_MAX_WBCLKS],
    pub fimc_lite: [*mut fimc_lite; FIMC_LITE_MAX_DEVS],
    pub fimc: [*mut fimc_dev; FIMC_MAX_DEVS],
    pub fimc_is: *mut fimc_is,
    pub use_isp: bool,
    pub pmf: *mut device,
    pub media_dev: media_device,
    pub v4l2_dev: v4l2_device,
    pub pdev: *mut platform_device,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cam_clk_provider {
    pub clks: [*mut clk; FIMC_MAX_CAMCLKS],
    pub clk_data: clk_onecell_data,
    pub of_node: *mut device_node,
    pub camclk: [cam_clk; FIMC_MAX_CAMCLKS],
    pub num_clocks: c_int,
    pub clk_provider: },
    pub subdev_notifier: v4l2_async_notifier,
    pub user_subdev_api: bool,
    pub slock: spinlock_t,
    pub pipelines: list_head,
    pub link_setup_graph: media_graph,
}

extern "C" {
    pub fn container_of(_arg: si, fimc_sensor_info: struct, _arg: pdata) -> return;
}
extern "C" {
    pub fn container_of(_arg: n, fimc_md: struct, _arg: subdev_notifier) -> return;
}
extern "C" {
    pub fn fimc_md_set_camclk(sd: *mut v4l2_subdev, on: bool) -> c_int;
}

