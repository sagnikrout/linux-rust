//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/fsl/bestcomm/ata.h
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
// Header for Bestcomm ATA task driver
//
// Copyright (C) 2006 Freescale - John Rigby
// Copyright (C) 2006 Sylvain Munaut <tnt@246tNt.com>
//
// This file is licensed under the terms of the GNU General Public License
// version 2. This program is licensed "as is" without any warranty of any
// kind, whether express or implied.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcom_ata_bd {
    pub status: u32,
    pub src_pa: u32,
    pub dst_pa: u32,
}

extern "C" {
    pub fn bcom_ata_init(queue_len: c_int, maxbufsize: c_int) -> *mut bcom_task;
}
extern "C" {
    pub fn bcom_ata_rx_prepare(tsk: *mut bcom_task);
}
extern "C" {
    pub fn bcom_ata_tx_prepare(tsk: *mut bcom_task);
}
extern "C" {
    pub fn bcom_ata_reset_bd(tsk: *mut bcom_task);
}
extern "C" {
    pub fn bcom_ata_release(tsk: *mut bcom_task);
}
