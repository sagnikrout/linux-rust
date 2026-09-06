//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/adfs_fs.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note

//
// Disc Record at disc address 0xc00
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct adfs_discrecord {
    pub log2secsize: __u8,
    pub secspertrack: __u8,
    pub heads: __u8,
    pub density: __u8,
    pub idlen: __u8,
    pub log2bpmb: __u8,
    pub skew: __u8,
    pub bootoption: __u8,
    pub lowsector: __u8,
    pub nzones: __u8,
    pub zone_spare: __le16,
    pub root: __le32,
    pub disc_size: __le32,
    pub disc_id: __le16,
    pub disc_name: [__u8; 10],
    pub disc_type: __le32,
    pub disc_size_high: __le32,
    pub log2sharesize:4: __u8,
    pub unused40:4: __u8,
    pub big_flag:1: __u8,
    pub unused41:7: __u8,
    pub nzones_high: __u8,
    pub reserved43: __u8,
    pub format_version: __le32,
    pub root_size: __le32,
    pub 52]: __u8 unused52[60 -,
// C attribute field omitted

pub const ADFS_DR_SIZE: c_int = 60;

