//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/target/target_core_rd.h
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

// Largest piece of memory kmalloc can allocate
pub const RD_MAX_ALLOCATION_SIZE: c_int = 65536;
pub const RD_DEVICE_QUEUE_DEPTH: c_int = 32;
pub const RD_MAX_DEVICE_QUEUE_DEPTH: c_int = 128;
pub const RD_BLOCKSIZE: c_int = 512;
// Used in target_core_init_configfs() for virtual LUN 0 access
extern "C" {
    pub fn rd_module_init() -> int __init;
}
extern "C" {
    pub fn rd_module_exit();
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rd_dev_sg_table {
    pub page_start_offset: u32,
    pub page_end_offset: u32,
    pub rd_sg_count: u32,
    pub sg_table: *mut scatterlist,
    pub ____cacheline_aligned: },
pub const RDF_HAS_PAGE_COUNT: c_uint = 0x01;
pub const RDF_NULLIO: c_uint = 0x02;
pub const RDF_DUMMY: c_uint = 0x04;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rd_dev {
    pub dev: se_device,
    pub rd_flags: u32,
// Unique Ramdisk Device ID in Ramdisk HBA
    pub rd_dev_id: u32,
// Total page count for ramdisk device
    pub rd_page_count: u32,
// Number of SG tables in sg_table_array
    pub sg_table_count: u32,
// Number of SG tables in sg_prot_array
    pub sg_prot_count: u32,
// Array of rd_dev_sg_table_t containing scatterlists
    pub sg_table_array: *mut rd_dev_sg_table,
// Array of rd_dev_sg_table containing protection scatterlists
    pub sg_prot_array: *mut rd_dev_sg_table,
// Ramdisk HBA device is connected to
    pub rd_host: *mut rd_host,
    pub ____cacheline_aligned: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rd_host {
    pub rd_host_dev_id_count: u32,
    pub /: *mut *mut u32 rd_host_id; / Unique Ramdisk Host ID,
    pub ____cacheline_aligned: },
