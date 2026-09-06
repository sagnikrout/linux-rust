//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/inside-secure/eip93/eip93-main.h
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
// Copyright (C) 2019 - 2021
//
// Richard van Schagen <vschagen@icloud.com>
// Christian Marangi <ansuelsmth@gmail.com>
//

pub const EIP93_RING_BUSY_DELAY: c_int = 500;
pub const EIP93_RING_NUM: c_int = 512;
pub const EIP93_RING_BUSY: c_int = 32;
pub const EIP93_CRA_PRIORITY: c_int = 1500;

// cipher algorithms

// hash and hmac algorithms

// cipher modes

// cipher encryption/decryption operations

// descriptor flags

#[repr(C)]
#[derive(Copy, Clone)]
pub struct eip93_desc_ring {
    pub base: *mut c_void,
    pub base_end: *mut c_void,
    pub base_dma: dma_addr_t,
// write and read pointers
    pub read: *mut c_void,
    pub write: *mut c_void,
// descriptor element offset
    pub offset: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct eip93_state_pool {
    pub base: *mut c_void,
    pub base_dma: dma_addr_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct eip93_ring {
    pub done_task: tasklet_struct,
// command/result rings
    pub cdr: eip93_desc_ring,
    pub rdr: eip93_desc_ring,
    pub write_lock: spinlock_t,
    pub read_lock: spinlock_t,
// aync idr
    pub idr_lock: spinlock_t,
    pub crypto_async_idr: idr,
}

//
// struct eip93_device - crypto engine device structure
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct eip93_device {
    pub base: *mut void __iomem,
    pub dev: *mut device,
    pub clk: *mut clk,
    pub irq: c_int,
    pub ring: [eip93_ring; ],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum eip93_alg_type {
    EIP93_ALG_TYPE_AEAD,
    EIP93_ALG_TYPE_SKCIPHER,
    EIP93_ALG_TYPE_HASH,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct eip93_alg_template {
    pub eip93: *mut eip93_device,
    pub type: eip93_alg_type,
    pub flags: u32,
    pub aead: aead_alg,
    pub skcipher: skcipher_alg,
    pub ahash: ahash_alg,
    pub alg: },
}
