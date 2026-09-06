//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/tegra/falcon.h
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
// Copyright (c) 2015, NVIDIA Corporation.
//

pub const FALCON_UCLASS_METHOD_OFFSET: c_uint = 0x00000040;
pub const FALCON_UCLASS_METHOD_DATA: c_uint = 0x00000044;
pub const FALCON_IRQMSET: c_uint = 0x00001010;

pub const FALCON_IRQDEST: c_uint = 0x0000101c;

pub const FALCON_ITFEN: c_uint = 0x00001048;

pub const FALCON_IDLESTATE: c_uint = 0x0000104c;
pub const FALCON_CPUCTL: c_uint = 0x00001100;

pub const FALCON_BOOTVEC: c_uint = 0x00001104;
pub const FALCON_DMACTL: c_uint = 0x0000110c;

pub const FALCON_DMATRFBASE: c_uint = 0x00001110;
pub const FALCON_DMATRFMOFFS: c_uint = 0x00001114;
pub const FALCON_DMATRFCMD: c_uint = 0x00001118;

pub const FALCON_DMATRFFBOFFS: c_uint = 0x0000111c;
pub const RISCV_BOOT_VECTOR_LO: c_uint = 0x00001780;
pub const RISCV_BOOT_VECTOR_HI: c_uint = 0x00001784;
pub const RISCV_CPUCTL: c_uint = 0x00001788;

pub const RISCV_BCR_CTRL: c_uint = 0x00001a68;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct falcon_fw_bin_header_v1 {
    pub /: *mut *mut u32 magic; / 0x10de,
    pub /: *mut *mut u32 version; / version of bin format (1),
    pub /: *mut *mut u32 size; / entire image size including this header,
    pub os_header_offset: u32,
    pub os_data_offset: u32,
    pub os_size: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct falcon_fw_os_app_v1 {
    pub offset: u32,
    pub size: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct falcon_fw_os_header_v1 {
    pub code_offset: u32,
    pub code_size: u32,
    pub data_offset: u32,
    pub data_size: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct falcon_fw_riscv_desc {
    pub reserved: [u32; 74],
    pub data_offset: u32,
    pub data_size: u32,
    pub code_offset: u32,
    pub code_size: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct falcon_firmware_section {
    pub offset: c_ulong,
    pub size: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct falcon_firmware {
// Firmware after it is read but not loaded
    pub firmware: *const firmware,
// RISC-V firmware descriptor
    pub desc_firmware: *const firmware,
// Raw firmware data
    pub iova: dma_addr_t,
    pub phys: dma_addr_t,
    pub virt: *mut c_void,
    pub size: usize,
// Parsed firmware information
    pub bin_data: falcon_firmware_section,
    pub data: falcon_firmware_section,
    pub code: falcon_firmware_section,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct falcon {
// Set by falcon client
    pub dev: *mut device,
    pub regs: *mut void __iomem,
// Peregrine falcon, external boot
    pub riscv: bool,
    pub firmware: falcon_firmware,
}

extern "C" {
    pub fn falcon_init(falcon: *mut falcon) -> c_int;
}
extern "C" {
    pub fn falcon_exit(falcon: *mut falcon);
}
extern "C" {
    pub fn falcon_read_firmware(falcon: *mut falcon, firmware_name: *const c_char) -> c_int;
}
extern "C" {
    pub fn falcon_load_firmware(falcon: *mut falcon) -> c_int;
}
extern "C" {
    pub fn falcon_boot(falcon: *mut falcon) -> c_int;
}
extern "C" {
    pub fn falcon_execute_method(falcon: *mut falcon, method: u32, data: u32);
}
extern "C" {
    pub fn falcon_wait_idle(falcon: *mut falcon) -> c_int;
}
