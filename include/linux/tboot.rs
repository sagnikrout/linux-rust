//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/tboot.h
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
// tboot.h: shared data structure with tboot and kernel and functions
// used by kernel for runtime support of Intel(R) Trusted
// Execution Technology
//
// Copyright (c) 2006-2009, Intel Corporation
//
// these must have the values from 0-5 in this order

// used to communicate between tboot and the launched kernel

pub const MAX_TB_MAC_REGIONS: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tboot_mac_region {
    pub /: *mut *mut u64 start; / must be 64 byte -aligned,
    pub /: *mut *mut u32 size; / must be 64 byte -granular,
    pub __packed: },
// GAS - Generic Address Structure (ACPI 2.0+)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tboot_acpi_generic_address {
    pub space_id: u8,
    pub bit_width: u8,
    pub bit_offset: u8,
    pub access_width: u8,
    pub address: u64,
    pub __packed: },
//
// combines Sx info from FADT and FACS tables per ACPI 2.0+ spec
// (https://uefi.org/specifications)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tboot_acpi_sleep_info {
    pub pm1a_cnt_blk: tboot_acpi_generic_address,
    pub pm1b_cnt_blk: tboot_acpi_generic_address,
    pub pm1a_evt_blk: tboot_acpi_generic_address,
    pub pm1b_evt_blk: tboot_acpi_generic_address,
    pub pm1a_cnt_val: u16,
    pub pm1b_cnt_val: u16,
    pub wakeup_vector: u64,
    pub vector_width: u32,
    pub kernel_s3_resume_vector: u64,
    pub __packed: },
//
// shared memory page used for communication between tboot and kernel
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tboot {
//
// version 3+ fields:
//
// TBOOT_UUID
    pub uuid: [u8; 16],
// version number: 5 is current
    pub version: u32,
// physical addr of tb_log_t log
    pub log_addr: u32,
//
// physical addr of entry point for tboot shutdown and
// type of shutdown (TB_SHUTDOWN_*) being requested
//
    pub shutdown_entry: u32,
    pub shutdown_type: u32,
// kernel-specified ACPI info for Sx shutdown
    pub acpi_sinfo: tboot_acpi_sleep_info,
// tboot location in memory (physical)
    pub tboot_base: u32,
    pub tboot_size: u32,
// memory regions (phys addrs) for tboot to MAC on S3
    pub num_mac_regions: u8,
    pub mac_regions: [tboot_mac_region; MAX_TB_MAC_REGIONS],
//
// version 4+ fields:
//
// symmetric key for use by kernel; will be encrypted on S3
    pub s3_key: [u8; TB_KEY_SIZE],
//
// version 5+ fields:
//
// used to 4byte-align num_in_wfs
    pub reserved_align: [u8; 3],
// number of processors in wait-for-SIPI
    pub num_in_wfs: u32,
    pub __packed: },
//
// UUID for tboot data struct to facilitate matching
// defined as {663C8DFF-E8B3-4b82-AABF-19EA4D057A08} by tboot, which is
// represented as {} in the char array used here
//

    pub tboot_enabled(void): bool,
    pub tboot_probe(void): extern void,
    pub shutdown_type): extern void tboot_shutdown(u32,
    pub dmar_tbl): *mut acpi_table_header,

pub const tboot_enabled(): c_int = 0;

