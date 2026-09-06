//! Automatically rewritten from C Header to Rust Module
//! Source: include/soc/fsl/dpaa2-io.h
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


// SPDX-License-Identifier: (GPL-2.0+ OR BSD-3-Clause)
//
// Copyright 2014-2016 Freescale Semiconductor Inc.
// Copyright 2017-2019 NXP
//

//
// DOC: DPIO Service
//
// The DPIO service provides APIs for users to interact with the datapath
// by enqueueing and dequeueing frame descriptors.
//
// The following set of APIs can be used to enqueue and dequeue frames
// as well as producing notification callbacks when data is available
// for dequeue.
//

//
// struct dpaa2_io_desc - The DPIO descriptor
// @receives_notifications: Use notification mode. Non-zero if the DPIO
// has a channel.
// @has_8prio:      Set to non-zero for channel with 8 priority WQs.  Ignored
// unless receives_notification is TRUE.
// @cpu:            The cpu index that at least interrupt handlers will
// execute on.
// @stash_affinity: The stash affinity for this portal favour 'cpu'
// @regs_cena:      The cache enabled regs.
// @regs_cinh:      The cache inhibited regs
// @dpio_id:        The dpio index
// @qman_version:   The qman version
// @qman_clk:       The qman clock frequency in Hz
//
// Describes the attributes and features of the DPIO object.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpaa2_io_desc {
    pub receives_notifications: c_int,
    pub has_8prio: c_int,
    pub cpu: c_int,
    pub regs_cena: *mut c_void,
    pub regs_cinh: *mut void __iomem,
    pub dpio_id: c_int,
    pub qman_version: u32,
    pub qman_clk: u32,
}

extern "C" {
    pub fn dpaa2_io_down(d: *mut dpaa2_io);
}
extern "C" {
    pub fn dpaa2_io_irq(obj: *mut dpaa2_io) -> irqreturn_t;
}
//
// struct dpaa2_io_notification_ctx - The DPIO notification context structure
// @cb:           The callback to be invoked when the notification arrives
// @is_cdan:      Zero for FQDAN, non-zero for CDAN
// @id:           FQID or channel ID, needed for rearm
// @desired_cpu:  The cpu on which the notifications will show up. Use
// DPAA2_IO_ANY_CPU if don't care
// @dpio_id:      The dpio index
// @qman64:       The 64-bit context value shows up in the FQDAN/CDAN.
// @node:         The list node
// @dpio_private: The dpio object internal to dpio_service
//
// Used when a FQDAN/CDAN registration is made by drivers.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpaa2_io_notification_ctx {
    pub ctx): *mut *mut void (cb)(struct dpaa2_io_notification_ctx,
    pub is_cdan: c_int,
    pub id: u32,
    pub desired_cpu: c_int,
    pub dpio_id: c_int,
    pub qman64: u64,
    pub node: list_head,
    pub dpio_private: *mut c_void,
}

extern "C" {
    pub fn dpaa2_io_get_cpu(d: *mut dpaa2_io) -> c_int;
}
extern "C" {
    pub fn dpaa2_io_store_destroy(s: *mut dpaa2_io_store);
}
extern "C" {
    pub fn dpaa2_io_set_irq_coalescing(d: *mut dpaa2_io, irq_holdoff: u32) -> c_int;
}
extern "C" {
    pub fn dpaa2_io_get_irq_coalescing(d: *mut dpaa2_io, irq_holdoff: *mut u32);
}
extern "C" {
    pub fn dpaa2_io_get_adaptive_coalescing(d: *mut dpaa2_io) -> c_int;
}
extern "C" {
    pub fn dpaa2_io_update_net_dim(d: *mut dpaa2_io, frames: __u64, bytes: __u64);
}
