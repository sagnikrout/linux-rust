//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/md/raid0.h
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
#[repr(C)]
#[derive(Copy, Clone)]
pub struct strip_zone {
    pub /: *mut *mut sector_t zone_end; / Start of the next zone (in sectors),
    pub /: *mut *mut sector_t dev_start; / Zone offset in real dev (in sectors),
    pub /: *mut *mut int nb_dev; / # of devices attached to the zone,
    pub /: *mut *mut int disk_shift; / start disk for the original layout,
}

// Linux 3.14 (20d0189b101) made an unintended change to
// the RAID0 layout for multi-zone arrays (where devices aren't all
// the same size.
// RAID0_ORIG_LAYOUT restores the original layout
// RAID0_ALT_MULTIZONE_LAYOUT uses the altered layout
// The layouts are identical when there is only one zone (all
// devices the same size).
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum r0layout {
    RAID0_ORIG_LAYOUT = 1,
    RAID0_ALT_MULTIZONE_LAYOUT = 2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct r0conf {
    pub strip_zone: *mut strip_zone,
    pub to: *mut *mut *mut *mut md_rdev devlist; / lists of rdevs, pointed,
// by strip_zone->dev
    pub nr_strip_zones: c_int,
    pub layout: r0layout,
}
