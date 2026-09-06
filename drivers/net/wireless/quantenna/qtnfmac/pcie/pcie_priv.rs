//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/quantenna/qtnfmac/pcie/pcie_priv.h
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
// Copyright (c) 2018 Quantenna Communications, Inc. All rights reserved.

pub const SKB_BUF_SIZE: c_int = 2048;
pub const QTN_FW_DL_TIMEOUT_MS: c_int = 3000;
pub const QTN_FW_QLINK_TIMEOUT_MS: c_int = 30000;
pub const QTN_EP_RESET_WAIT_MS: c_int = 1000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qtnf_pcie_bus_priv {
    pub pdev: *mut pci_dev,
    pub rx_bd_size): c_uint,
    pub bus): *mut *mut void (remove_cb)(struct qtnf_bus,
    pub bus): *mut *mut int (suspend_cb)(struct qtnf_bus,
    pub bus): *mut *mut int (resume_cb)(struct qtnf_bus,
    pub (*dma_mask_get_cb)(void): *mut u64,
    pub tx_reclaim_lock: spinlock_t,
    pub tx_lock: spinlock_t,
    pub workqueue: *mut workqueue_struct,
    pub reclaim_tq: tasklet_struct,
    pub sysctl_bar: *mut void __iomem,
    pub epmem_bar: *mut void __iomem,
    pub dmareg_bar: *mut void __iomem,
    pub shm_ipc_ep_in: qtnf_shm_ipc,
    pub shm_ipc_ep_out: qtnf_shm_ipc,
    pub tx_bd_num: u16,
    pub rx_bd_num: u16,
    pub tx_skb: *mut sk_buff,
    pub rx_skb: *mut sk_buff,
    pub fw_blksize: c_uint,
    pub rx_bd_w_index: u32,
    pub rx_bd_r_index: u32,
    pub tx_bd_w_index: u32,
    pub tx_bd_r_index: u32,
// diagnostics stats
    pub pcie_irq_count: u32,
    pub tx_full_count: u32,
    pub tx_done_count: u32,
    pub tx_reclaim_done: u32,
    pub tx_reclaim_req: u32,
    pub msi_enabled: u8,
    pub tx_stopped: u8,
    pub flashboot: bool,
}

extern "C" {
    pub fn qtnf_pcie_control_tx(bus: *mut qtnf_bus, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn qtnf_pcie_alloc_skb_array(priv: *mut qtnf_pcie_bus_priv) -> c_int;
}
extern "C" {
    pub fn qtnf_pcie_fw_boot_done(bus: *mut qtnf_bus) -> c_int;
}
// flush posted write
