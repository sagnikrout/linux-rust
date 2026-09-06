//! Automatically rewritten from C Header to Rust Module
//! Source: arch/s390/include/asm/tpi.h
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

// I/O-Interruption Code as stored by TEST PENDING INTERRUPTION (TPI).
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tpi_info {
    pub schid: subchannel_id,
    pub intparm: u32,
    pub adapter_IO:1: u32,
    pub directed_irq:1: u32,
    pub isc:3: u32,
    pub :12: u32,
    pub type:3: u32,
    pub :12: u32,
    pub __aligned(4): } __packed,
// I/O-Interruption Code as stored by TPI for an Adapter I/O
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tpi_adapter_info {
    pub aism:8: u32,
    pub :22: u32,
    pub error:1: u32,
    pub forward:1: u32,
    pub reserved: u32,
    pub adapter_IO:1: u32,
    pub directed_irq:1: u32,
    pub isc:3: u32,
    pub :27: u32,
    pub __aligned(4): } __packed,

