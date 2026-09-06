//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/quantenna/qtnfmac/shm_ipc.h
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


// SPDX-License-Identifier: GPL-2.0+
// Copyright (c) 2015-2016 Quantenna Communications. All rights reserved.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qtnf_shm_ipc_int {
    pub arg): *mut *mut void (fn)(void,
    pub arg: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qtnf_shm_ipc_rx_callback {
    pub len): *const *const *const *const void (fn)(void arg, u8 __iomem buf, size_t,
    pub arg: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qtnf_shm_ipc_direction {
    QTNF_SHM_IPC_OUTBOUND		= BIT(0),
    QTNF_SHM_IPC_INBOUND		= BIT(1),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qtnf_shm_ipc {
    pub shm_region: *mut qtnf_shm_ipc_region __iomem,
    pub direction: qtnf_shm_ipc_direction,
    pub tx_packet_count: usize,
    pub rx_packet_count: usize,
    pub tx_timeout_count: usize,
    pub waiting_for_ack: u8,
    pub interrupt: qtnf_shm_ipc_int,
    pub rx_callback: qtnf_shm_ipc_rx_callback,
    pub ipc): *mut *mut void (irq_handler)(struct qtnf_shm_ipc,
    pub workqueue: *mut workqueue_struct,
    pub irq_work: work_struct,
    pub tx_completion: completion,
}

extern "C" {
    pub fn qtnf_shm_ipc_free(ipc: *mut qtnf_shm_ipc);
}
extern "C" {
    pub fn qtnf_shm_ipc_send(ipc: *mut qtnf_shm_ipc, buf: *const u8, size: usize) -> c_int;
}
