//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/video/fbdev/via/viafbdev.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Copyright 1998-2008 VIA Technologies, Inc. All Rights Reserved.
// Copyright 2001-2008 S3 Graphics, Inc. All Rights Reserved.
//

pub const VERSION_MAJOR: c_int = 2;

pub const VERSION_MINOR: c_int = 4;
pub const VIAFB_NUM_I2C: c_int = 5;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct viafb_shared {
    pub iga1_devices: u32,
    pub iga2_devices: u32,
    pub /: *mut *mut *mut proc_dir_entry proc_entry; /viafb proc entry,
    pub iga1_proc_entry: *mut proc_dir_entry,
    pub iga2_proc_entry: *mut proc_dir_entry,
    pub /: *mut *mut *mut viafb_dev vdev; / Global dev info,
// I2C busses that may have auxiliary devices
    pub i2c_26: *mut via_aux_bus,
    pub i2c_31: *mut via_aux_bus,
    pub i2c_2C: *mut via_aux_bus,
// All the information will be needed to set engine
    pub tmds_setting_info: tmds_setting_information,
    pub lvds_setting_info: lvds_setting_information,
    pub lvds_setting_info2: lvds_setting_information,
    pub chip_info: chip_information,
// hardware acceleration stuff
    pub cursor_vram_addr: u32,
    pub /: *mut *mut u32 vq_vram_addr; / virtual queue address in video ram,
    pub fill_rop): u32 fg_color, u32 bg_color, u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct viafb_par {
    pub depth: u8,
    pub vram_addr: u32,
    pub /: *mut *mut unsigned int fbmem; /framebuffer physical memory address,
    pub /: *mut *mut unsigned int memsize; /size of fbmem,
    pub /: *mut *mut u32 fbmem_free; / Free FB memory,
    pub /: *mut *mut u32 fbmem_used; / Use FB memory size,
    pub iga_path: u32,
    pub shared: *mut viafb_shared,
// All the information will be needed to set engine
// depreciated, use the ones in shared directly
    pub tmds_setting_info: *mut tmds_setting_information,
    pub lvds_setting_info: *mut lvds_setting_information,
    pub lvds_setting_info2: *mut lvds_setting_information,
    pub chip_info: *mut chip_information,
}

// plvds_setting_info, struct lvds_chip_information
// plvds_chip_info, u8 index);
// plvds_setting_info, struct lvds_chip_information
// plvds_chip_info, struct IODATA io_data);
extern "C" {
    pub fn via_fb_pci_probe(vdev: *mut viafb_dev) -> c_int;
}
extern "C" {
    pub fn via_fb_pci_remove(pdev: *mut pci_dev);
}
// Temporary
extern "C" {
    pub fn viafb_init() -> c_int;
}
extern "C" {
    pub fn viafb_exit();
}
