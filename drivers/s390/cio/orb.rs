//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/s390/cio/orb.h
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
// Orb related data structures.
//
// Copyright IBM Corp. 2007, 2011
//
// Author(s): Cornelia Huck <cornelia.huck@de.ibm.com>
// Peter Oberparleiter <peter.oberparleiter@de.ibm.com>
// Sebastian Ott <sebott@linux.vnet.ibm.com>
//

//
// Command-mode operation request block
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmd_orb {
    pub /: *mut *mut u32 intparm; / interruption parameter,
    pub /: *mut *mut u32 key:4; / flags, like key, suspend control, etc.,
    pub /: *mut *mut u32 spnd:1; / suspend control,
    pub /: *mut *mut u32 res1:1; / reserved,
    pub /: *mut *mut u32 mod:1; / modification control,
    pub /: *mut *mut u32 sync:1; / synchronize control,
    pub /: *mut *mut u32 fmt:1; / format control,
    pub /: *mut *mut u32 pfch:1; / prefetch control,
    pub /: *mut *mut u32 isic:1; / initial-status-interruption control,
    pub /: *mut *mut u32 alcc:1; / address-limit-checking control,
    pub /: *mut *mut u32 ssic:1; / suppress-suspended-interr. control,
    pub /: *mut *mut u32 res2:1; / reserved,
    pub /: *mut *mut u32 c64:1; / IDAW/QDIO 64 bit control,
    pub /: *mut *mut u32 i2k:1; / IDAW 2/4kB block size control,
    pub /: *mut *mut u32 lpm:8; / logical path mask,
    pub /: *mut *mut u32 ils:1; / incorrect length,
    pub /: *mut *mut u32 zero:6; / reserved zeros,
    pub /: *mut *mut u32 orbx:1; / ORB extension control,
    pub /: *mut *mut dma32_t cpa; / channel program address,
    pub __aligned(4): } __packed,
//
// Transport-mode operation request block
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tm_orb {
    pub intparm: u32,
    pub key:4: u32,
    pub b:1: u32,
    pub lpm:8: u32,
    pub x:1: u32,
    pub tcw: dma32_t,
    pub prio:8: u32,
    pub rsvpgm:8: u32,
    pub __aligned(4): } __packed,
//
// eadm operation request block
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct eadm_orb {
    pub intparm: u32,
    pub key:4: u32,
    pub compat1:1: u32,
    pub compat2:1: u32,
    pub x:1: u32,
    pub aob: dma32_t,
    pub css_prio:8: u32,
    pub scm_prio:8: u32,
    pub fmt:3: u32,
    pub __aligned(4): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub union orb {
    pub cmd: cmd_orb,
    pub tm: tm_orb,
    pub eadm: eadm_orb,
    pub __aligned(4): } __packed,
