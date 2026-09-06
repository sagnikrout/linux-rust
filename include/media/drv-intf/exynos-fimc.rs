//! Automatically rewritten from C Header to Rust Module
//! Source: include/media/drv-intf/exynos-fimc.h
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
// Samsung S5P/Exynos4 SoC series camera interface driver header
//
// Copyright (C) 2010 - 2013 Samsung Electronics Co., Ltd.
// Sylwester Nawrocki <s.nawrocki@samsung.com>
//

//
// Enumeration of data inputs to the camera subsystem.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fimc_input {
    FIMC_INPUT_PARALLEL_0	= 1,
    FIMC_INPUT_PARALLEL_1,
    FIMC_INPUT_MIPI_CSI2_0	= 3,
    FIMC_INPUT_MIPI_CSI2_1,
    FIMC_INPUT_WRITEBACK_A	= 5,
    FIMC_INPUT_WRITEBACK_B,
    FIMC_INPUT_WRITEBACK_ISP = 5,
}

//
// Enumeration of the FIMC data bus types.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fimc_bus_type {
// Camera parallel bus
    FIMC_BUS_TYPE_ITU_601 = 1,
// Camera parallel bus with embedded synchronization
    FIMC_BUS_TYPE_ITU_656,
// Camera MIPI-CSI2 serial bus
    FIMC_BUS_TYPE_MIPI_CSI2,
// FIFO link from LCD controller (WriteBack A)
    FIMC_BUS_TYPE_LCD_WRITEBACK_A,
// FIFO link from LCD controller (WriteBack B)
    FIMC_BUS_TYPE_LCD_WRITEBACK_B,
// FIFO link from FIMC-IS
    FIMC_BUS_TYPE_ISP_WRITEBACK = FIMC_BUS_TYPE_LCD_WRITEBACK_B,
}

//
// The subdevices' group IDs.
//

//
// struct fimc_source_info - video source description required for the host
// interface configuration
//
// @fimc_bus_type: FIMC camera input type
// @sensor_bus_type: image sensor bus type, MIPI, ITU-R BT.601 etc.
// @flags: the parallel sensor bus flags defining signals polarity (V4L2_MBUS_*)
// @mux_id: FIMC camera interface multiplexer index (separate for MIPI and ITU)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fimc_source_info {
    pub fimc_bus_type: fimc_bus_type,
    pub sensor_bus_type: fimc_bus_type,
    pub flags: u16,
    pub mux_id: u16,
}

//
// v4l2_device notification id. This is only for internal use in the kernel.
// Sensor subdevs should issue S5P_FIMC_TX_END_NOTIFY notification in single
// frame capture mode when there is only one VSYNC pulse issued by the sensor
// at beginning of the frame transmission.
//

pub const FIMC_MAX_PLANES: c_int = 3;
//
// struct fimc_fmt - color format data structure
// @mbus_code: media bus pixel code, -1 if not applicable
// @fourcc: fourcc code for this format, 0 if not applicable
// @color: the driver's private color format id
// @memplanes: number of physically non-contiguous data planes
// @colplanes: number of physically contiguous data planes
// @colorspace: v4l2 colorspace (V4L2_COLORSPACE_*)
// @depth: per plane driver's private 'number of bits per pixel'
// @mdataplanes: bitmask indicating meta data plane(s), (1 << plane_no)
// @flags: flags indicating which operation mode format applies to
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fimc_fmt {
    pub mbus_code: u32,
    pub fourcc: u32,
    pub color: u32,
    pub memplanes: u16,
    pub colplanes: u16,
    pub colorspace: u8,
    pub depth: [u8; FIMC_MAX_PLANES],
    pub mdataplanes: u16,
    pub flags: u16,

}

//
// Media pipeline operations to be called from within a video node,  i.e. the
// last entity within the pipeline. Implemented by related media device driver.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct exynos_media_pipeline_ops {
    pub me): *mut media_entity,
    pub p): *mut *mut int (unprepare)(struct exynos_media_pipeline,
    pub resume): bool,
    pub p): *mut *mut int (close)(struct exynos_media_pipeline,
    pub state): *mut *mut *mut int (set_stream)(struct exynos_media_pipeline p, bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct exynos_video_entity {
    pub vdev: video_device,
    pub pipe: *mut exynos_media_pipeline,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct exynos_media_pipeline {
    pub mp: media_pipeline,
    pub ops: *const exynos_media_pipeline_ops,
}

extern "C" {
    pub fn container_of(_arg: vdev, exynos_video_entity: struct, _arg: vdev) -> return;
}

