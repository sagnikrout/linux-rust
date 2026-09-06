//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/aic94xx/aic94xx_sds.h
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
// Aic94xx SAS/SATA driver hardware interface header file.
//
// Copyright (C) 2005 Adaptec, Inc.  All rights reserved.
// Copyright (C) 2005 Gilbert Wu <gilbert_wu@adaptec.com>
//
pub const FLASH_MANUF_ID_AMD: c_uint = 0x01;
pub const FLASH_MANUF_ID_ST: c_uint = 0x20;
pub const FLASH_MANUF_ID_FUJITSU: c_uint = 0x04;
pub const FLASH_MANUF_ID_MACRONIX: c_uint = 0xC2;
pub const FLASH_MANUF_ID_INTEL: c_uint = 0x89;
pub const FLASH_MANUF_ID_UNKNOWN: c_uint = 0xFF;
pub const FLASH_DEV_ID_AM29LV008BT: c_uint = 0x3E;
pub const FLASH_DEV_ID_AM29LV800DT: c_uint = 0xDA;
pub const FLASH_DEV_ID_STM29W800DT: c_uint = 0xD7;
pub const FLASH_DEV_ID_STM29LV640: c_uint = 0xDE;
pub const FLASH_DEV_ID_STM29008: c_uint = 0xEA;
pub const FLASH_DEV_ID_MBM29LV800TE: c_uint = 0xDA;
pub const FLASH_DEV_ID_MBM29DL800TA: c_uint = 0x4A;
pub const FLASH_DEV_ID_MBM29LV008TA: c_uint = 0x3E;
pub const FLASH_DEV_ID_AM29LV640MT: c_uint = 0x7E;
pub const FLASH_DEV_ID_AM29F800B: c_uint = 0xD6;
pub const FLASH_DEV_ID_MX29LV800BT: c_uint = 0xDA;
pub const FLASH_DEV_ID_MX29LV008CT: c_uint = 0xDA;
pub const FLASH_DEV_ID_I28LV00TAT: c_uint = 0x3E;
pub const FLASH_DEV_ID_UNKNOWN: c_uint = 0xFF;
// status bit mask values
pub const FLASH_STATUS_BIT_MASK_DQ6: c_uint = 0x40;
pub const FLASH_STATUS_BIT_MASK_DQ5: c_uint = 0x20;
pub const FLASH_STATUS_BIT_MASK_DQ2: c_uint = 0x04;
// minimum value in micro seconds needed for checking status
pub const FLASH_STATUS_ERASE_DELAY_COUNT: c_int = 50;
pub const FLASH_STATUS_WRITE_DELAY_COUNT: c_int = 25;
pub const FLASH_SECTOR_SIZE: c_uint = 0x010000;
pub const FLASH_SECTOR_SIZE_MASK: c_uint = 0xffff0000;
pub const FLASH_OK: c_uint = 0x000000;
pub const FAIL_OPEN_BIOS_FILE: c_uint = 0x000100;
pub const FAIL_CHECK_PCI_ID: c_uint = 0x000200;
pub const FAIL_CHECK_SUM: c_uint = 0x000300;
pub const FAIL_UNKNOWN: c_uint = 0x000400;
pub const FAIL_VERIFY: c_uint = 0x000500;
pub const FAIL_RESET_FLASH: c_uint = 0x000600;
pub const FAIL_FIND_FLASH_ID: c_uint = 0x000700;
pub const FAIL_ERASE_FLASH: c_uint = 0x000800;
pub const FAIL_WRITE_FLASH: c_uint = 0x000900;
pub const FAIL_FILE_SIZE: c_uint = 0x000a00;
pub const FAIL_PARAMETERS: c_uint = 0x000b00;
pub const FAIL_OUT_MEMORY: c_uint = 0x000c00;
pub const FLASH_IN_PROGRESS: c_uint = 0x001000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct controller_id {
    pub /: *mut *mut u32 vendor; / PCI Vendor ID,
    pub /: *mut *mut u32 device; / PCI Device ID,
    pub /: *mut *mut u32 sub_vendor; / PCI Subvendor ID,
    pub /: *mut *mut u32 sub_device; / PCI Subdevice ID,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct image_info {
    pub /: *mut *mut u32 ImageId; / Identifies the image,
    pub /: *mut *mut u32 ImageOffset; / Offset the beginning of the file,
    pub /: *mut *mut u32 ImageLength; / length of the image,
    pub /: *mut *mut u32 ImageChecksum; / Image checksum,
    pub /: *mut *mut u32 ImageVersion; / Version of the image, could be build number,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bios_file_header {
    pub /: *mut *mut u8 signature[32]; / Signature/Cookie to identify the file,
    pub /: *mut *mut u32 checksum; /Entire file checksum with this field zero,
    pub /: *mut *mut u32 antidote; / Entire file checksum with this field 0xFFFFFFFF,
    pub /: *mut *mut controller_id contrl_id; /PCI id to identify the controller,
    pub file*/: *mut *mut u32 filelen; /Length of the entire,
    pub /: *mut *mut u32 chunk_num; /The chunk/part number for multiple Image files,
    pub /: *mut *mut u32 total_chunks; /Total number of chunks/parts in the image file,
    pub /: *mut *mut u32 num_images; / Number of images in the file,
    pub /: *mut *mut u32 build_num; / Build number of this image,
    pub image_header: image_info,
}

extern "C" {
    pub fn asd_check_flash_type(asd_ha: *mut asd_ha_struct) -> c_int;
}
