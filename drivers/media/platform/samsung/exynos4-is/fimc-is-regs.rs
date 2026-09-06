//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/samsung/exynos4-is/fimc-is-regs.h
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
// Authors: Sylwester Nawrocki <s.nawrocki@samsung.com>
// Younghwan Joo <yhwan.joo@samsung.com>
//
// WDT_ISP register
pub const REG_WDT_ISP: c_uint = 0x00170000;
// MCUCTL registers base offset
pub const MCUCTL_BASE: c_uint = 0x00180000;
// MCU Controller Register

// Boot Base Offset Address Register

// Interrupt Generation Register 0 from Host CPU to VIC

// __n = 0...9

// __n = 0...5

// Interrupt Clear Register 0 from Host CPU to VIC

// __n = 0...9

// __n = 0...5

// Interrupt Mask Register 0 from Host CPU to VIC

// __n = 0...9

// __n = 0...5

// Interrupt Status Register 0 from Host CPU to VIC

// __n (bit number) = 0...4

// __n (bit number) = 0...9

// Interrupt Mask Status Register 0 from Host CPU to VIC

// __n (bit number) = 0...4

// __n (bit number) = 0...9

// Interrupt Generation Register 1 from ISP CPU to Host IC

// __n = 0...9

// Interrupt Clear Register 1 from ISP CPU to Host IC

// __n = 0...9

// Interrupt Mask Register 1 from ISP CPU to Host IC

// __n = 0...9

// Interrupt Status Register 1 from ISP CPU to Host IC

// Interrupt Mask Status Register 1 from ISP CPU to Host IC

// Interrupt Clear Register 2 from ISP BLK's interrupts to Host IC

// __n = 0...5

// Interrupt Mask Register 2 from ISP BLK's interrupts to Host IC

// __n = 0...25

// Interrupt Status Register 2 from ISP BLK's interrupts to Host IC

// Interrupt Mask Status Register 2 from ISP BLK's interrupts to Host IC

// General Purpose Output Control Register (0~17)

// __n = 0...17

// General Purpose Pad Output Enable Register (0~17)

// __n = 0...17

// General Purpose Input Control Register (0~17)

// Shared registers between ISP CPU and the host CPU - ISSRxx
// ISSR(1): Command Host -> IS
// ISSR(1): Sensor ID for Command, ISSR2...5 = Parameter 1...4
// ISSR(10): Reply IS -> Host
// ISSR(11): Sensor ID for Reply, ISSR12...15 = Parameter 1...4
// ISSR(20): ISP_FRAME_DONE : SENSOR ID
// ISSR(21): ISP_FRAME_DONE : PARAMETER 1
// ISSR(24): SCALERC_FRAME_DONE : SENSOR ID
// ISSR(25): SCALERC_FRAME_DONE : PARAMETER 1
// ISSR(28): 3DNR_FRAME_DONE : SENSOR ID
// ISSR(29): 3DNR_FRAME_DONE : PARAMETER 1
// ISSR(32): SCALERP_FRAME_DONE : SENSOR ID
// ISSR(33): SCALERP_FRAME_DONE : PARAMETER 1
// __n = 0...63

// PMU ISP register offsets
pub const REG_CMU_RESET_ISP_SYS_PWR_REG: c_uint = 0x1174;
pub const REG_CMU_SYSCLK_ISP_SYS_PWR_REG: c_uint = 0x13b8;
pub const REG_PMU_ISP_ARM_SYS: c_uint = 0x1050;
pub const REG_PMU_ISP_ARM_CONFIGURATION: c_uint = 0x2280;
pub const REG_PMU_ISP_ARM_STATUS: c_uint = 0x2284;
pub const REG_PMU_ISP_ARM_OPTION: c_uint = 0x2288;
extern "C" {
    pub fn fimc_is_fw_clear_irq1(is: *mut fimc_is, bit: c_uint);
}
extern "C" {
    pub fn fimc_is_fw_clear_irq2(is: *mut fimc_is);
}
extern "C" {
    pub fn fimc_is_hw_get_params(is: *mut fimc_is, num: c_uint) -> c_int;
}
extern "C" {
    pub fn fimc_is_hw_set_intgr0_gd0(is: *mut fimc_is);
}
extern "C" {
    pub fn fimc_is_hw_wait_intmsr0_intmsd0(is: *mut fimc_is) -> c_int;
}
extern "C" {
    pub fn fimc_is_hw_set_sensor_num(is: *mut fimc_is);
}
extern "C" {
    pub fn fimc_is_hw_set_isp_buf_mask(is: *mut fimc_is, mask: c_uint);
}
extern "C" {
    pub fn fimc_is_hw_stream_on(is: *mut fimc_is);
}
extern "C" {
    pub fn fimc_is_hw_stream_off(is: *mut fimc_is);
}
extern "C" {
    pub fn fimc_is_hw_set_param(is: *mut fimc_is) -> c_int;
}
extern "C" {
    pub fn fimc_is_hw_change_mode(is: *mut fimc_is) -> c_int;
}
extern "C" {
    pub fn fimc_is_hw_close_sensor(is: *mut fimc_is, index: c_uint);
}
extern "C" {
    pub fn fimc_is_hw_get_setfile_addr(is: *mut fimc_is);
}
extern "C" {
    pub fn fimc_is_hw_load_setfile(is: *mut fimc_is);
}
extern "C" {
    pub fn fimc_is_hw_subip_power_off(is: *mut fimc_is);
}
extern "C" {
    pub fn fimc_is_itf_s_param(is: *mut fimc_is, update: bool) -> c_int;
}
extern "C" {
    pub fn fimc_is_itf_mode_change(is: *mut fimc_is) -> c_int;
}
