//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/sof/imx/imx-common.h
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


// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)

pub const EXCEPT_MAX_HDR_SIZE: c_uint = 0x400;
pub const IMX8_STACK_DUMP_SIZE: c_int = 32;
// chip_info refers to the data stored in struct sof_dev_desc's chip_info
// Macro flag: #define get_chip_info(sdev)\
// chip_pdata refers to the data stored in struct imx_common_data's chip_pdata
// Macro flag: #define get_chip_pdata(sdev)\
// can be used if:
// 1) The only supported IPC version is IPC3.
// 2) The default paths/FW name match values below.
//
// otherwise, just explicitly declare the structure
//

// to be used alongside IMX_SOF_DEV_DESC()

// dai driver entry w/ playback and capture caps. If one direction is missing
// then set the channels to 0.
//

// use if playback and capture have the same min/max channel count

#[repr(C)]
#[derive(Copy, Clone)]
pub struct imx_ipc_info {
// true if core is able to write a panic code to the debug box
    pub has_panic_code: bool,
// offset to mailbox in which firmware initially writes FW_READY
    pub boot_mbox_offset: c_int,
// offset to region at which the mailboxes start
    pub window_offset: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct imx_chip_ops {
// called after clocks and PDs are enabled
    pub sdev): *mut *mut int (probe)(struct snd_sof_dev,
// used directly by the SOF core
    pub sdev): *mut *mut int (core_kick)(struct snd_sof_dev,
// called during suspend()/remove() before clocks are disabled
    pub sdev): *mut *mut int (core_shutdown)(struct snd_sof_dev,
// used directly by the SOF core
    pub sdev): *mut *mut int (core_reset)(struct snd_sof_dev,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct imx_memory_info {
    pub name: *const c_char,
    pub reserved: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct imx_chip_info {
    pub ipc_info: imx_ipc_info,
// does the chip have a reserved memory region for DMA?
    pub has_dma_reserved: bool,
    pub memory: *mut imx_memory_info,
    pub drv: *mut snd_soc_dai_driver,
    pub num_drv: c_int,
// optional
    pub ops: *const imx_chip_ops,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct imx_common_data {
    pub ipc_dev: *mut platform_device,
    pub ipc_handle: *mut imx_dsp_ipc,
// core may have no clocks
    pub clks: *mut clk_bulk_data,
    pub clk_num: c_int,
// core may have no PDs
    pub pd_list: *mut dev_pm_domain_list,
    pub chip_pdata: *mut c_void,
}

extern "C" {
    pub fn imx8_dump(sdev: *mut snd_sof_dev, flags: u32);
}
