//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mtd/spear_smi.h
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
// Copyright © 2010 ST Microelectronics
// Shiraz Hashim <shiraz.linux.kernel@gmail.com>
//
// This file is licensed under the terms of the GNU General Public
// License version 2. This program is licensed "as is" without any
// warranty of any kind, whether express or implied.
//

// max possible slots for serial-nor flash chip in the SMI controller
pub const MAX_NUM_FLASH_CHIP: c_int = 4;
// macro to define partitions for flash devices

//
// struct spear_smi_flash_info - platform structure for passing flash
// information
//
// @name: name of the serial nor flash for identification
// @mem_base: the memory base on which the flash is mapped
// @size: size of the flash in bytes
// @partitions: parition details
// @nr_partitions: number of partitions
// @fast_mode: whether flash supports fast mode
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct spear_smi_flash_info {
    pub name: *mut c_char,
    pub mem_base: c_ulong,
    pub size: c_ulong,
    pub partitions: *mut mtd_partition,
    pub nr_partitions: c_int,
    pub fast_mode: u8,
}

//
// struct spear_smi_plat_data - platform structure for configuring smi
//
// @clk_rate: clk rate at which SMI must operate
// @num_flashes: number of flashes present on board
// @board_flash_info: specific details of each flash present on board
// @np: array of DT node pointers for all possible flash chip devices
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct spear_smi_plat_data {
    pub clk_rate: c_ulong,
    pub num_flashes: c_int,
    pub board_flash_info: *mut spear_smi_flash_info,
    pub np: [*mut device_node; MAX_NUM_FLASH_CHIP],
}
