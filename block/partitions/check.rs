//! Automatically rewritten from C Header to Rust Module
//! Source: block/partitions/check.h
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
// add_gd_partition adds a partitions details to the devices partition
// description.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct parsed_partitions {
    pub disk: *mut gendisk,
    pub name: [c_char; BDEVNAME_SIZE],
    pub from: sector_t,
    pub size: sector_t,
    pub flags: c_int,
    pub has_info: bool,
    pub info: partition_meta_info,
    pub parts: *mut },
    pub next: c_int,
    pub limit: c_int,
    pub access_beyond_eod: bool,
    pub pp_buf: seq_buf,
}

// detection routines go here in alphabetical order:
extern "C" {
    pub fn adfspart_check_ADFS(state: *mut parsed_partitions) -> c_int;
}
extern "C" {
    pub fn adfspart_check_CUMANA(state: *mut parsed_partitions) -> c_int;
}
extern "C" {
    pub fn adfspart_check_EESOX(state: *mut parsed_partitions) -> c_int;
}
extern "C" {
    pub fn adfspart_check_ICS(state: *mut parsed_partitions) -> c_int;
}
extern "C" {
    pub fn adfspart_check_POWERTEC(state: *mut parsed_partitions) -> c_int;
}
extern "C" {
    pub fn aix_partition(state: *mut parsed_partitions) -> c_int;
}
extern "C" {
    pub fn amiga_partition(state: *mut parsed_partitions) -> c_int;
}
extern "C" {
    pub fn atari_partition(state: *mut parsed_partitions) -> c_int;
}
extern "C" {
    pub fn cmdline_partition(state: *mut parsed_partitions) -> c_int;
}
extern "C" {
    pub fn efi_partition(state: *mut parsed_partitions) -> c_int;
}
extern "C" {
    pub fn ibm_partition(: *mut parsed_partitions) -> c_int;
}
extern "C" {
    pub fn karma_partition(state: *mut parsed_partitions) -> c_int;
}
extern "C" {
    pub fn ldm_partition(state: *mut parsed_partitions) -> c_int;
}
extern "C" {
    pub fn mac_partition(state: *mut parsed_partitions) -> c_int;
}
extern "C" {
    pub fn msdos_partition(state: *mut parsed_partitions) -> c_int;
}
extern "C" {
    pub fn of_partition(state: *mut parsed_partitions) -> c_int;
}
extern "C" {
    pub fn osf_partition(state: *mut parsed_partitions) -> c_int;
}
extern "C" {
    pub fn sgi_partition(state: *mut parsed_partitions) -> c_int;
}
extern "C" {
    pub fn sun_partition(state: *mut parsed_partitions) -> c_int;
}
extern "C" {
    pub fn sysv68_partition(state: *mut parsed_partitions) -> c_int;
}
extern "C" {
    pub fn ultrix_partition(state: *mut parsed_partitions) -> c_int;
}
