//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/can/kvaser_pciefd/kvaser_pciefd.h
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


// SPDX-License-Identifier: GPL-2.0 OR BSD-2-Clause
// kvaser_pciefd common definitions and declarations
//
// Copyright (C) 2025 KVASER AB, Sweden. All rights reserved.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvaser_pciefd_address_offset {
    pub serdes: u32,
    pub pci_ien: u32,
    pub pci_irq: u32,
    pub sysid: u32,
    pub loopback: u32,
    pub kcan_srb_fifo: u32,
    pub kcan_srb: u32,
    pub kcan_ch0: u32,
    pub kcan_ch1: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvaser_pciefd_irq_mask {
    pub kcan_rx0: u32,
    pub kcan_tx: [u32; KVASER_PCIEFD_MAX_CAN_CHANNELS],
    pub all: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvaser_pciefd_dev_ops {
    pub index): dma_addr_t addr, int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvaser_pciefd_driver_data {
    pub address_offset: *const kvaser_pciefd_address_offset,
    pub irq_mask: *const kvaser_pciefd_irq_mask,
    pub ops: *const kvaser_pciefd_dev_ops,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvaser_pciefd_fw_version {
    pub major: u8,
    pub minor: u8,
    pub build: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvaser_pciefd_can {
    pub can: can_priv,
    pub devlink_port: devlink_port,
    pub kv_pcie: *mut kvaser_pciefd,
    pub reg_base: *mut void __iomem,
    pub bec: can_berr_counter,
    pub ioc: u32,
    pub cmd_seq: u8,
    pub tx_max_count: u8,
    pub tx_idx: u8,
    pub ack_idx: u8,
    pub err_rep_cnt: c_int,
    pub completed_tx_pkts: c_uint,
    pub completed_tx_bytes: c_uint,
    pub /: *mut *mut spinlock_t lock; / Locks sensitive registers (e.g. MODE),
    pub bec_poll_timer: timer_list,
    pub flush_comp: completion start_comp,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvaser_pciefd {
    pub pci: *mut pci_dev,
    pub reg_base: *mut void __iomem,
    pub can: [*mut kvaser_pciefd_can; KVASER_PCIEFD_MAX_CAN_CHANNELS],
    pub driver_data: *const kvaser_pciefd_driver_data,
    pub dma_data: [*mut c_void; KVASER_PCIEFD_DMA_COUNT],
    pub nr_channels: u8,
    pub bus_freq: u32,
    pub freq: u32,
    pub freq_to_ticks_div: u32,
    pub fw_version: kvaser_pciefd_fw_version,
}

extern "C" {
    pub fn kvaser_pciefd_devlink_port_register(can: *mut kvaser_pciefd_can) -> c_int;
}
extern "C" {
    pub fn kvaser_pciefd_devlink_port_unregister(can: *mut kvaser_pciefd_can);
}
