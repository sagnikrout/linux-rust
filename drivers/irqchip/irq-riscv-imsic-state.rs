//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/irqchip/irq-riscv-imsic-state.h
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
// Copyright (C) 2021 Western Digital Corporation or its affiliates.
// Copyright (C) 2022 Ventana Micro Systems Inc.
//

pub const IMSIC_IPI_ID: c_int = 1;
pub const IMSIC_NR_IPI: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct imsic_vector {
// Fixed details of the vector
    pub cpu: c_uint,
    pub local_id: c_uint,
// Details saved by driver in the vector
    pub irq: c_uint,
// Details accessed using local lock held
    pub enable: bool,
    pub move_next: *mut imsic_vector,
    pub move_prev: *mut imsic_vector,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct imsic_local_priv {
// Local lock to protect vector enable/move variables and dirty bitmap
    pub lock: raw_spinlock_t,
// Local dirty bitmap for synchronization
    pub dirty_bitmap: *mut c_ulong,

// Local timer for synchronization
    pub timer: timer_list,

// Local vector table
    pub vectors: *mut imsic_vector,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct imsic_priv {
// Device details
    pub fwnode: *mut fwnode_handle,
// Global configuration common for all HARTs
    pub global: imsic_global_config,
// Per-CPU state
    pub lpriv: *mut imsic_local_priv __percpu,
// State of IRQ matrix allocator
    pub matrix_lock: raw_spinlock_t,
    pub matrix: *mut irq_matrix,
// IRQ domains (created by platform driver)
    pub base_domain: *mut irq_domain,
}

extern "C" {
    pub fn __imsic_eix_update(base_id: c_ulong, num_id: c_ulong, pend: bool, val: bool);
}
extern "C" {
    pub fn imsic_local_sync_all(force_all: bool);
}
extern "C" {
    pub fn imsic_local_delivery(enable: bool);
}
extern "C" {
    pub fn imsic_vector_mask(vec: *mut imsic_vector);
}
extern "C" {
    pub fn imsic_vector_unmask(vec: *mut imsic_vector);
}
extern "C" {
    pub fn READ_ONCE(_arg: vec->enable) -> return;
}
extern "C" {
    pub fn READ_ONCE(_arg: vec->move_prev) -> return;
}
extern "C" {
    pub fn imsic_vector_force_move_cleanup(vec: *mut imsic_vector);
}
extern "C" {
    pub fn imsic_vector_move(old_vec: *mut imsic_vector, new_vec: *mut imsic_vector);
}
extern "C" {
    pub fn imsic_vector_free(vector: *mut imsic_vector);
}
extern "C" {
    pub fn imsic_vector_debug_show(m: *mut seq_file, vec: *mut imsic_vector, ind: c_int);
}
extern "C" {
    pub fn imsic_vector_debug_show_summary(m: *mut seq_file, ind: c_int);
}
extern "C" {
    pub fn imsic_state_online();
}
extern "C" {
    pub fn imsic_state_offline();
}
extern "C" {
    pub fn imsic_setup_state(fwnode: *mut fwnode_handle, opaque: *mut c_void) -> c_int;
}
extern "C" {
    pub fn imsic_irqdomain_init() -> c_int;
}
