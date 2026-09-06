//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/samsung/exynos4-is/fimc-is.h
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
// Samsung EXYNOS4x12 FIMC-IS (Imaging Subsystem) driver
//
// Copyright (C) 2013 Samsung Electronics Co., Ltd.
//
// Authors: Younghwan Joo <yhwan.joo@samsung.com>
// Sylwester Nawrocki <s.nawrocki@samsung.com>
//

pub const FIMC_IS_SENSORS_NUM: c_int = 2;
// Memory definitions

pub const FIMC_IS_REGION_SIZE: c_uint = 0x5000;
pub const FIMC_IS_DEBUG_REGION_OFFSET: c_uint = 0x0084b000;
pub const FIMC_IS_SHARED_REGION_OFFSET: c_uint = 0x008c0000;
pub const FIMC_IS_FW_INFO_LEN: c_int = 31;
pub const FIMC_IS_FW_VER_LEN: c_int = 7;

pub const FIMC_IS_SETFILE_INFO_LEN: c_int = 39;

pub const FIMC_IS_EXTRA_FW_SIZE: c_uint = 0x180000;
pub const FIMC_IS_EXTRA_SETFILE_SIZE: c_uint = 0x4b000;
// TODO: revisit

// The driver's internal state flags
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum af_state {
    FIMC_IS_AF_IDLE		= 0,
    FIMC_IS_AF_SETCONFIG	= 1,
    FIMC_IS_AF_RUNNING	= 2,
    FIMC_IS_AF_LOCK		= 3,
    FIMC_IS_AF_ABORT	= 4,
    FIMC_IS_AF_FAILED	= 5,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum af_lock_state {
    FIMC_IS_AF_UNLOCKED	= 0,
    FIMC_IS_AF_LOCKED	= 2
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ae_lock_state {
    FIMC_IS_AE_UNLOCKED	= 0,
    FIMC_IS_AE_LOCKED	= 1
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum awb_lock_state {
    FIMC_IS_AWB_UNLOCKED	= 0,
    FIMC_IS_AWB_LOCKED	= 1
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct is_setfile {
    pub info: *const firmware,
    pub state: c_int,
    pub sub_index: u32,
    pub base: u32,
    pub size: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct is_fd_result_header {
    pub offset: u32,
    pub count: u32,
    pub index: u32,
    pub curr_index: u32,
    pub width: u32,
    pub height: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct is_af_info {
    pub mode: u16,
    pub af_state: u32,
    pub af_lock_state: u32,
    pub ae_lock_state: u32,
    pub awb_lock_state: u32,
    pub pos_x: u16,
    pub pos_y: u16,
    pub prev_pos_x: u16,
    pub prev_pos_y: u16,
    pub use_af: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fimc_is_firmware {
    pub f_w: *const firmware,
    pub addr: dma_addr_t,
    pub vaddr: *mut c_void,
    pub size: c_uint,
    pub 1]: char info[FIMC_IS_FW_INFO_LEN +,
    pub 1]: char version[FIMC_IS_FW_VER_LEN +,
    pub 1]: char setfile_info[FIMC_IS_SETFILE_INFO_LEN +,
    pub state: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fimc_is_memory {
// DMA base address
    pub addr: dma_addr_t,
// virtual base address
    pub vaddr: *mut c_void,
// total length
    pub size: c_uint,
}

pub const FIMC_IS_I2H_MAX_ARGS: c_int = 12;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i2h_cmd {
    pub cmd: u32,
    pub sensor_id: u32,
    pub num_args: u16,
    pub args: [u32; FIMC_IS_I2H_MAX_ARGS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct h2i_cmd {
    pub cmd_type: u16,
    pub entry_id: u32,
}

pub const FIMC_IS_DEBUG_MSG: c_uint = 0x3f;
pub const FIMC_IS_DEBUG_LEVEL: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fimc_is_setfile {
    pub info: *const firmware,
    pub state: c_uint,
    pub size: c_uint,
    pub sub_index: u32,
    pub base: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct chain_config {
    pub global: global_param,
    pub sensor: sensor_param,
    pub isp: isp_param,
    pub drc: drc_param,
    pub fd: fd_param,
    pub p_region_index: [c_ulong; 2],
}

//
// struct fimc_is - fimc-is data structure
// @pdev: pointer to FIMC-IS platform device
// @v4l2_dev: pointer to the top level v4l2_device
// @fw: data structure describing the FIMC-IS firmware binary
// @memory: memory region assigned for the FIMC-IS (firmware)
// @isp: the ISP block data structure
// @sensor: fimc-is sensor subdevice array
// @setfile: descriptor of the imaging pipeline calibration data
// @ctrl_handler: the v4l2 controls handler
// @lock: mutex serializing video device and the subdev operations
// @slock: spinlock protecting this data structure and the hw registers
// @clocks: FIMC-LITE gate clock
// @regs: MCUCTL mmapped registers region
// @pmu_regs: PMU ISP mmapped registers region
// @irq: FIMC-IS interrupt
// @irq_queue: interrupt handling waitqueue
// @lpm: low power mode flag
// @state: internal driver's state flags
// @sensor_index: image sensor index for the firmware
// @i2h_cmd: FIMC-IS to the host (CPU) mailbox command data structure
// @h2i_cmd: the host (CPU) to FIMC-IS mailbox command data structure
// @fd_header: the face detection result data structure
// @config: shared HW pipeline configuration data
// @config_index: index to the @config entry currently in use
// @is_p_region: pointer to the shared parameter memory region
// @is_dma_p_region: DMA address of the shared parameter memory region
// @is_shared_region: pointer to the IS shared region data structure
// @af: auto focus data
// @debugfs_entry: debugfs entry for the firmware log
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fimc_is {
    pub pdev: *mut platform_device,
    pub v4l2_dev: *mut v4l2_device,
    pub fw: fimc_is_firmware,
    pub memory: fimc_is_memory,
    pub isp: fimc_isp,
    pub sensor: [fimc_is_sensor; FIMC_IS_SENSORS_NUM],
    pub setfile: fimc_is_setfile,
    pub ctrl_handler: v4l2_ctrl_handler,
    pub lock: mutex,
    pub slock: spinlock_t,
    pub clocks: [*mut clk; ISS_CLKS_MAX],
    pub regs: *mut void __iomem,
    pub pmu_regs: *mut void __iomem,
    pub irq: c_int,
    pub irq_queue: wait_queue_head_t,
    pub lpm: u8,
    pub state: c_ulong,
    pub sensor_index: c_uint,
    pub i2h_cmd: i2h_cmd,
    pub h2i_cmd: h2i_cmd,
    pub fd_header: is_fd_result_header,
    pub config: [chain_config; IS_SC_MAX],
    pub config_index: unsigned,
    pub is_p_region: *mut is_region,
    pub is_dma_p_region: dma_addr_t,
    pub is_shared_region: *mut is_share_region,
    pub af: is_af_info,
    pub debugfs_entry: *mut dentry,
}

extern "C" {
    pub fn container_of(_arg: isp, fimc_is: struct, _arg: isp) -> return;
}
extern "C" {
    pub fn readl(offset: is->regs +) -> return;
}
extern "C" {
    pub fn readl(offset: is->pmu_regs +) -> return;
}
extern "C" {
    pub fn fimc_is_cpu_set_power(is: *mut fimc_is, on: c_int) -> c_int;
}
extern "C" {
    pub fn fimc_is_start_firmware(is: *mut fimc_is) -> c_int;
}
extern "C" {
    pub fn fimc_is_hw_initialize(is: *mut fimc_is) -> c_int;
}
extern "C" {
    pub fn fimc_is_log_dump(level: *const c_char, buf: *const c_void, len: usize);
}
