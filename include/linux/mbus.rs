//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mbus.h
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


//
// Marvell MBUS common definitions.
//
// Copyright (C) 2008 Marvell Semiconductor
//
// This file is licensed under the terms of the GNU General Public
// License version 2.  This program is licensed "as is" without any
// warranty of any kind, whether express or implied.
//

//
// The 4-bit MBUS target ID of the DRAM controller.
//
// The base address, size, and MBUS attribute ID for each
// of the possible DRAM chip selects.  Peripherals are
// required to support at least 4 decode windows.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mbus_dram_window {
    pub cs_index: u8,
    pub mbus_attr: u8,
    pub base: u64,
    pub size: u64,
    pub cs: [}; 4],
}

// Flags for PCI/PCIe address decoding regions
pub const MVEBU_MBUS_PCI_IO: c_uint = 0x1;
pub const MVEBU_MBUS_PCI_MEM: c_uint = 0x2;
pub const MVEBU_MBUS_PCI_WA: c_uint = 0x3;
//
// Magic value that explicits that we don't need a remapping-capable
// address decoding window.
//

// Maximum size of a mbus window name
pub const MVEBU_MBUS_MAX_WINNAME_SZ: c_int = 32;
//
// The Marvell mbus is to be found only on SOCs from the Orion family
// at the moment.  Provide a dummy stub for other architectures.
//

//
// On all ARM32 MVEBU platforms with MBus support, this stub
// function will not get called. The real function from the
// MBus driver is called instead. ARM64 MVEBU platforms like
// the Armada 3700 could use the mv_xor device driver which calls
// into this function
//

extern "C" {
    pub fn mvebu_mbus_save_cpu_target(store_addr: *mut u32 __iomem) -> c_int;
}
extern "C" {
    pub fn mvebu_mbus_get_pcie_mem_aperture(res: *mut resource);
}
extern "C" {
    pub fn mvebu_mbus_get_pcie_io_aperture(res: *mut resource);
}
extern "C" {
    pub fn mvebu_mbus_get_dram_win_info(phyaddr: phys_addr_t, target: *mut u8, attr: *mut u8) -> c_int;
}
extern "C" {
    pub fn mvebu_mbus_del_window(base: phys_addr_t, size: usize) -> c_int;
}
extern "C" {
    pub fn mvebu_mbus_dt_init(is_coherent: bool) -> c_int;
}

