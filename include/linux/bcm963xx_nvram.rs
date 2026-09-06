//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/bcm963xx_nvram.h
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
// Broadcom BCM963xx SoC board nvram data structure.
//
// The nvram structure varies in size depending on the SoC board version. Use
// the appropriate minimum BCM963XX_NVRAM_*_SIZE define for the information
// you need instead of sizeof(struct bcm963xx_nvram) as this may change.
//
pub const BCM963XX_NVRAM_V4_SIZE: c_int = 300;

pub const BCM963XX_DEFAULT_PSI_SIZE: c_int = 64;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bcm963xx_nvram_nand_part {
    BCM963XX_NVRAM_NAND_PART_BOOT = 0,
    BCM963XX_NVRAM_NAND_PART_ROOTFS_1,
    BCM963XX_NVRAM_NAND_PART_ROOTFS_2,
    BCM963XX_NVRAM_NAND_PART_DATA,
    BCM963XX_NVRAM_NAND_PART_BBT,

    __BCM963XX_NVRAM_NAND_NR_PARTS
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcm963xx_nvram {
    pub version: u32,
    pub bootline: [c_char; 256],
    pub name: [c_char; 16],
    pub main_tp_number: u32,
    pub psi_size: u32,
    pub mac_addr_count: u32,
    pub mac_addr_base: [u8; ETH_ALEN],
    pub __reserved1: [u8; 2],
    pub checksum_v4: u32,
    pub __reserved2: [u8; 292],
    pub nand_part_offset: [u32; __BCM963XX_NVRAM_NAND_NR_PARTS],
    pub nand_part_size: [u32; __BCM963XX_NVRAM_NAND_NR_PARTS],
    pub __reserved3: [u8; 388],
    pub checksum_v5: u32,
}

//
// bcm963xx_nvram_checksum - Verify nvram checksum
//
// @nvram: pointer to full size nvram data structure
// @expected_out: optional pointer to store expected checksum value
// @actual_out: optional pointer to store actual checksum value
//
// Return: 0 if the checksum is valid, otherwise -EINVAL
//
// Calculate the CRC32 of the nvram with the checksum field set to 0.
// expected_out = expected;
// actual_out = actual;
