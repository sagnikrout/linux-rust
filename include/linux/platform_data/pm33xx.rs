//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/platform_data/pm33xx.h
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
// TI pm33xx platform data
//
// Copyright (C) 2016-2018 Texas Instruments, Inc.
// Dave Gerlach <d-gerlach@ti.com>
//

//
// WFI Flags for sleep code control
//
// These flags allow PM code to exclude certain operations from happening
// in the low level ASM code found in sleep33xx.S and sleep43xx.S
//
// WFI_FLAG_FLUSH_CACHE: Flush the ARM caches and disable caching. Only
// needed when MPU will lose context.
// WFI_FLAG_SELF_REFRESH: Let EMIF place DDR memory into self-refresh and
// disable EMIF.
// WFI_FLAG_SAVE_EMIF: Save context of all EMIF registers and restore in
// resume path. Only needed if PER domain loses context
// and must also have WFI_FLAG_SELF_REFRESH set.
// WFI_FLAG_WAKE_M3: Disable MPU clock or clockdomain to cause wkup_m3 to
// execute when WFI instruction executes.
// WFI_FLAG_RTC_ONLY: Configure the RTC to enter RTC+DDR mode.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct am33xx_pm_sram_addr {
    pub (*do_wfi)(void): *mut c_void,
    pub do_wfi_sz: *mut c_ulong,
    pub resume_offset: *mut c_ulong,
    pub emif_sram_table: *mut c_ulong,
    pub ro_sram_data: *mut c_ulong,
    pub resume_address: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct am33xx_pm_platform_data {
    pub wfi_flags)): *mut *mut *mut int (init)(int (idle)(u32,
    pub (*deinit)(void): *mut c_int,
    pub args): c_ulong,
    pub args): *mut *mut *mut int (cpu_suspend)(int (fn)(unsigned long), unsigned long,
    pub (*begin_suspend)(void): *mut c_void,
    pub (*finish_suspend)(void): *mut c_void,
    pub (*get_sram_addrs)(void): *mut am33xx_pm_sram_addr,
    pub (*save_context)(void): *mut c_void,
    pub (*restore_context)(void): *mut c_void,
    pub (*check_off_mode_enable)(void): *mut c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct am33xx_pm_sram_data {
    pub wfi_flags: u32,
    pub l2_aux_ctrl_val: u32,
    pub l2_prefetch_ctrl_val: u32,
    pub __aligned(8): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct am33xx_pm_ro_sram_data {
    pub amx3_pm_sram_data_virt: u32,
    pub amx3_pm_sram_data_phys: u32,
    pub rtc_base_virt: *mut void __iomem,
    pub __aligned(8): } __packed,

