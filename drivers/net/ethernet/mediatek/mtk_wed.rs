//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/mediatek/mtk_wed.h
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
// Copyright (C) 2021 Felix Fietkau <nbd@nbd.name>

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_wed_soc_data {
    pub tx_bm_tkid: u32,
    pub wpdma_rx_ring: [u32; MTK_WED_RX_QUEUES],
    pub reset_idx_tx_mask: u32,
    pub reset_idx_rx_mask: u32,
    pub regmap: },
    pub tx_ring_desc_size: u32,
    pub wdma_desc_size: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_wed_amsdu {
    pub txd: *mut c_void,
    pub txd_phy: dma_addr_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_wed_hw {
    pub soc: *const mtk_wed_soc_data,
    pub node: *mut device_node,
    pub eth: *mut mtk_eth,
    pub regs: *mut regmap,
    pub hifsys: *mut regmap,
    pub dev: *mut device,
    pub wdma: *mut void __iomem,
    pub wdma_phy: phys_addr_t,
    pub mirror: *mut regmap,
    pub debugfs_dir: *mut dentry,
    pub wed_dev: *mut mtk_wed_device,
    pub wed_wo: *mut mtk_wed_wo,
    pub wed_amsdu: *mut mtk_wed_amsdu,
    pub pcie_base: u32,
    pub debugfs_reg: u32,
    pub num_flows: u32,
    pub version: u8,
    pub dirname: [c_char; 5],
    pub irq: c_int,
    pub index: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_wdma_info {
    pub wdma_idx: u8,
    pub queue: u8,
    pub wcid: u16,
    pub bss: u8,
    pub amsdu: u8,
}

extern "C" {
    pub fn readl(reg: dev->hw->wdma +) -> return;
}
extern "C" {
    pub fn readl(reg: dev->tx_ring[ring].wpdma +) -> return;
}
extern "C" {
    pub fn readl(reg: dev->rx_ring[ring].wpdma +) -> return;
}
extern "C" {
    pub fn readl(reg: dev->txfree_ring.wpdma +) -> return;
}
extern "C" {
    pub fn mtk_wed_exit();
}
extern "C" {
    pub fn mtk_wed_flow_add(index: c_int) -> c_int;
}
extern "C" {
    pub fn mtk_wed_flow_remove(index: c_int);
}
extern "C" {
    pub fn mtk_wed_fe_reset();
}
extern "C" {
    pub fn mtk_wed_fe_reset_complete();
}

extern "C" {
    pub fn mtk_wed_hw_add_debugfs(hw: *mut mtk_wed_hw);
}

