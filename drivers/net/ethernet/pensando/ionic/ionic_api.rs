//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/pensando/ionic/ionic_api.h
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
// Copyright (C) 2018-2025, Advanced Micro Devices, Inc.

//
// struct ionic_aux_dev - Auxiliary device information
// @lif:        Logical interface
// @idx:        Index identifier
// @adev:       Auxiliary device
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_aux_dev {
    pub lif: *mut ionic_lif,
    pub idx: c_int,
    pub adev: auxiliary_device,
}

//
// struct ionic_admin_ctx - Admin command context
// @work:       Work completion wait queue element
// @cmd:        Admin command (64B) to be copied to the queue
// @comp:       Admin completion (16B) copied from the queue
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_admin_ctx {
    pub work: completion,
    pub cmd: ionic_adminq_cmd,
    pub comp: ionic_adminq_comp,
}

pub const IONIC_INTR_NAME_MAX_SZ: c_int = 32;
//
// struct ionic_intr_info - Interrupt information
// @name:          Name identifier
// @rearm_count:   Interrupt rearm count
// @index:         Interrupt index position
// @vector:        Interrupt number
// @dim_coal_hw:   Interrupt coalesce value in hardware units
// @affinity_mask: CPU affinity mask
// @aff_notify:    context for notification of IRQ affinity changes
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_intr_info {
    pub name: [c_char; IONIC_INTR_NAME_MAX_SZ],
    pub rearm_count: u64,
    pub index: c_uint,
    pub vector: c_uint,
    pub dim_coal_hw: u32,
    pub affinity_mask: *mut cpumask_var_t,
    pub aff_notify: irq_affinity_notify,
}

//
// ionic_adminq_post_wait - Post an admin command and wait for response
// @lif:        Logical interface
// @ctx:        API admin command context
//
// Post the command to an admin queue in the ethernet driver.  If this command
// succeeds, then the command has been posted, but that does not indicate a
// completion.  If this command returns success, then the completion callback
// will eventually be called.
//
// Return: zero or negative error status
//
extern "C" {
    pub fn ionic_adminq_post_wait(lif: *mut ionic_lif, ctx: *mut ionic_admin_ctx) -> c_int;
}
//
// ionic_error_to_errno - Transform ionic_if errors to os errno
// @code:       Ionic error number
//
// Return:      Negative OS error number or zero
//
extern "C" {
    pub fn ionic_error_to_errno(code: ionic_status_code) -> c_int;
}
//
// ionic_request_rdma_reset - request reset or disable the device or lif
// @lif:        Logical interface
//
// The reset is triggered asynchronously. It will wait until reset request
// completes or times out.
//
extern "C" {
    pub fn ionic_request_rdma_reset(lif: *mut ionic_lif);
}
//
// ionic_intr_alloc - Reserve a device interrupt
// @lif:        Logical interface
// @intr:       Reserved ionic interrupt structure
//
// Reserve an interrupt index and get irq number for that index.
//
// Return: zero or negative error status
//
extern "C" {
    pub fn ionic_intr_alloc(lif: *mut ionic_lif, intr: *mut ionic_intr_info) -> c_int;
}
//
// ionic_intr_free - Release a device interrupt index
// @lif:        Logical interface
// @intr:       Interrupt index
//
// Mark the interrupt index unused so that it can be reserved again.
//
extern "C" {
    pub fn ionic_intr_free(lif: *mut ionic_lif, intr: c_int);
}
//
// ionic_get_cmb - Reserve cmb pages
// @lif:         Logical interface
// @pgid:        First page index
// @pgaddr:      First page bus addr (contiguous)
// @order:       Log base two number of pages (PAGE_SIZE)
// @stride_log2: Size of stride to determine CMB pool
// @expdb:       Will be set to true if this CMB region has expdb enabled
//
// Return: zero or negative error status
//
// ionic_put_cmb - Release cmb pages
// @lif:        Logical interface
// @pgid:       First page index
// @order:      Log base two number of pages (PAGE_SIZE)
//
extern "C" {
    pub fn ionic_put_cmb(lif: *mut ionic_lif, pgid: u32, order: c_int);
}
