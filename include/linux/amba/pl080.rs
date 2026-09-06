//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/amba/pl080.h
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
// include/linux/amba/pl080.h
//
// Copyright 2008 Openmoko, Inc.
// Copyright 2008 Simtec Electronics
// http://armlinux.simtec.co.uk
// Ben Dooks <ben@simtec.co.uk>
//
// ARM PrimeCell PL080 DMA controller
//
// Note, there are some Samsung updates to this controller block which
// make it not entierly compatible with the PL080 specification from
// ARM. When in doubt, check the Samsung documentation first.
//
// The Samsung defines are PL080S, and add an extra control register,
// the ability to move more than 2^11 counts of data and some extra
// OneNAND features.
//

// The Faraday Technology FTDMAC020 variant registers

// Identical to PL080_CONFIG

// Identical to PL080_SYNC

// Per channel configuration registers

// The Faraday FTDMAC020 derivative shuffles the registers around

// Later versions have a threshold in bits 24..26,

// 00 = increase, 01 = decrease, 10 = fix

// FIFO threshold setting

// The FTDMAC020 supports 64bit wide transfers

// Address can be increased, decreased or fixed

// Inside the LLIs, the applicable CSR fields are mapped differently

// DMA linked list chain structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pl080_lli {
    pub src_addr: u32,
    pub dst_addr: u32,
    pub next_lli: u32,
    pub control0: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pl080s_lli {
    pub src_addr: u32,
    pub dst_addr: u32,
    pub next_lli: u32,
    pub control0: u32,
    pub control1: u32,
}
