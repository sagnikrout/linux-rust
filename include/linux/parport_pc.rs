//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/parport_pc.h
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

// --- register definitions -------------------------------

#[repr(C)]
#[derive(Copy, Clone)]
pub struct parport_pc_private {
// Contents of CTR.
    pub ctr: c_uchar,
// Bitmask of writable CTR bits.
    pub ctr_writable: c_uchar,
// Whether or not there's an ECR.
    pub ecr: c_int,
// Bitmask of writable ECR bits.
    pub ecr_writable: c_uchar,
// Number of PWords that FIFO will hold.
    pub fifo_depth: c_int,
// Number of bytes per portword.
    pub pword: c_int,
// Not used yet.
    pub readIntrThreshold: c_int,
    pub writeIntrThreshold: c_int,
// buffer suitable for DMA, if DMA enabled
    pub dma_buf: *mut c_char,
    pub dma_handle: dma_addr_t,
    pub list: list_head,
    pub port: *mut parport,
}

// ISA PnP IRQ routing register 1
// ISA PnP DMA request routing register
// Register and value to enable SuperIO configuration access
// SuperIO function register number
// parallel port control register number
// Parallel port base address register

// here's hoping that reading these ports won't side-effect anything underneath

// Macro flag: #define dump_parport_state(args...)

// __parport_pc_frob_control differs from parport_pc_frob_control in that
// it doesn't do any extra masking.

// Take this out when drivers have adapted to newer interface.
// Restrict mask and val to control lines.
extern "C" {
    pub fn __parport_pc_frob_control(_arg: p, _arg: mask, _arg: val) -> return;
}
extern "C" {
    pub fn inb(_arg: STATUS(p)) -> return;
}
extern "C" {
    pub fn parport_pc_release_resources(p: *mut parport);
}
extern "C" {
    pub fn parport_pc_claim_resources(p: *mut parport) -> c_int;
}
// PCMCIA code will want to get us to look at a port.  Provide a mechanism.
extern "C" {
    pub fn parport_pc_unregister_port(p: *mut parport);
}
