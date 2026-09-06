//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/mellanox/mlxbf_gige/mlxbf_gige.h
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


// SPDX-License-Identifier: GPL-2.0-only OR BSD-3-Clause
// Header file for Gigabit Ethernet driver for Mellanox BlueField SoC
// - this file contains software data structures and any chip-specific
// data structures (e.g. TX WQE format) that are memory resident.
//
// Copyright (C) 2020-2021 NVIDIA CORPORATION & AFFILIATES
//

// The silicon design supports a maximum RX ring size of
// 32K entries. Based on current testing this maximum size
// is not required to be supported.  Instead the RX ring
// will be capped at a realistic value of 1024 entries.
//
pub const MLXBF_GIGE_MIN_RXQ_SZ: c_int = 32;
pub const MLXBF_GIGE_MAX_RXQ_SZ: c_int = 1024;
pub const MLXBF_GIGE_DEFAULT_RXQ_SZ: c_int = 128;
pub const MLXBF_GIGE_MIN_TXQ_SZ: c_int = 4;
pub const MLXBF_GIGE_MAX_TXQ_SZ: c_int = 256;
pub const MLXBF_GIGE_DEFAULT_TXQ_SZ: c_int = 128;
pub const MLXBF_GIGE_DEFAULT_BUF_SZ: c_int = 2048;
pub const MLXBF_GIGE_DMA_PAGE_SZ: c_int = 4096;
pub const MLXBF_GIGE_DMA_PAGE_SHIFT: c_int = 12;
// There are four individual MAC RX filters. Currently
// two of them are being used: one for the broadcast MAC
// (index 0) and one for local MAC (index 1)
//
pub const MLXBF_GIGE_BCAST_MAC_FILTER_IDX: c_int = 0;
pub const MLXBF_GIGE_LOCAL_MAC_FILTER_IDX: c_int = 1;
pub const MLXBF_GIGE_MAX_FILTER_IDX: c_int = 3;
// Define for broadcast MAC literal
pub const BCAST_MAC_ADDR: c_uint = 0xFFFFFFFFFFFF;
// There are three individual interrupts:
// 1) Errors, "OOB" interrupt line
// 2) Receive Packet, "OOB_LLU" interrupt line
// 3) LLU and PLU Events, "OOB_PLU" interrupt line
//
pub const MLXBF_GIGE_ERROR_INTR_IDX: c_int = 0;
pub const MLXBF_GIGE_RECEIVE_PKT_INTR_IDX: c_int = 1;
pub const MLXBF_GIGE_LLU_PLU_INTR_IDX: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlxbf_gige_stats {
    pub hw_access_errors: u64,
    pub tx_invalid_checksums: u64,
    pub tx_small_frames: u64,
    pub tx_index_errors: u64,
    pub sw_config_errors: u64,
    pub sw_access_errors: u64,
    pub rx_truncate_errors: u64,
    pub rx_mac_errors: u64,
    pub rx_din_dropped_pkts: u64,
    pub tx_fifo_full: u64,
    pub rx_filter_passed_pkts: u64,
    pub rx_filter_discard_pkts: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlxbf_gige_reg_param {
    pub mask: u32,
    pub shift: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlxbf_gige_mdio_gw {
    pub gw_address: u32,
    pub read_data_address: u32,
    pub busy: mlxbf_gige_reg_param,
    pub write_data: mlxbf_gige_reg_param,
    pub read_data: mlxbf_gige_reg_param,
    pub devad: mlxbf_gige_reg_param,
    pub partad: mlxbf_gige_reg_param,
    pub opcode: mlxbf_gige_reg_param,
    pub st1: mlxbf_gige_reg_param,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlxbf_gige_link_cfg {
    pub phydev): *mut *mut void (set_phy_link_mode)(struct phy_device,
    pub netdev): *mut *mut void (adjust_link)(struct net_device,
    pub phy_mode: phy_interface_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlxbf_gige {
    pub base: *mut void __iomem,
    pub llu_base: *mut void __iomem,
    pub plu_base: *mut void __iomem,
    pub dev: *mut device,
    pub netdev: *mut net_device,
    pub pdev: *mut platform_device,
    pub mdio_io: *mut void __iomem,
    pub clk_io: *mut void __iomem,
    pub mdiobus: *mut mii_bus,
    pub /: *mut *mut spinlock_t lock; / for packet processing indices,
    pub rx_q_entries: u16,
    pub tx_q_entries: u16,
    pub tx_wqe_base: *mut u64,
    pub tx_wqe_base_dma: dma_addr_t,
    pub tx_wqe_next: *mut u64,
    pub tx_cc: *mut u64,
    pub tx_cc_dma: dma_addr_t,
    pub rx_wqe_base: *mut dma_addr_t,
    pub rx_wqe_base_dma: dma_addr_t,
    pub rx_cqe_base: *mut u64,
    pub rx_cqe_base_dma: dma_addr_t,
    pub tx_pi: u16,
    pub prev_tx_ci: u16,
    pub rx_skb: [*mut sk_buff; MLXBF_GIGE_MAX_RXQ_SZ],
    pub tx_skb: [*mut sk_buff; MLXBF_GIGE_MAX_TXQ_SZ],
    pub error_irq: c_int,
    pub rx_irq: c_int,
    pub llu_plu_irq: c_int,
    pub phy_irq: c_int,
    pub hw_phy_irq: c_int,
    pub promisc_enabled: bool,
    pub valid_polarity: u8,
    pub napi: napi_struct,
    pub stats: mlxbf_gige_stats,
    pub hw_version: u8,
    pub mdio_gw: *mut mlxbf_gige_mdio_gw,
    pub prev_speed: c_int,
}

// Rx Work Queue Element definitions
pub const MLXBF_GIGE_RX_WQE_SZ: c_int = 8;
// Rx Completion Queue Element definitions
pub const MLXBF_GIGE_RX_CQE_SZ: c_int = 8;

// Tx Work Queue Element definitions
pub const MLXBF_GIGE_TX_WQE_SZ_QWORDS: c_int = 2;
pub const MLXBF_GIGE_TX_WQE_SZ: c_int = 16;

// Macro to return packet length of specified TX WQE

// Tx Completion Count
pub const MLXBF_GIGE_TX_CC_SZ: c_int = 8;
// List of resources in ACPI table
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlxbf_gige_res {
    MLXBF_GIGE_RES_MAC,
    MLXBF_GIGE_RES_MDIO9,
    MLXBF_GIGE_RES_GPIO0,
    MLXBF_GIGE_RES_LLU,
    MLXBF_GIGE_RES_PLU,
    MLXBF_GIGE_RES_CLK
}

// Version of register data returned by mlxbf_gige_get_regs()
pub const MLXBF_GIGE_REGS_VERSION: c_int = 1;
extern "C" {
    pub fn mlxbf_gige_mdio_remove(priv: *mut mlxbf_gige);
}
extern "C" {
    pub fn mlxbf_gige_enable_multicast_rx(priv: *mut mlxbf_gige);
}
extern "C" {
    pub fn mlxbf_gige_disable_multicast_rx(priv: *mut mlxbf_gige);
}
extern "C" {
    pub fn mlxbf_gige_enable_promisc(priv: *mut mlxbf_gige);
}
extern "C" {
    pub fn mlxbf_gige_disable_promisc(priv: *mut mlxbf_gige);
}
extern "C" {
    pub fn mlxbf_gige_rx_init(priv: *mut mlxbf_gige) -> c_int;
}
extern "C" {
    pub fn mlxbf_gige_rx_deinit(priv: *mut mlxbf_gige);
}
extern "C" {
    pub fn mlxbf_gige_tx_init(priv: *mut mlxbf_gige) -> c_int;
}
extern "C" {
    pub fn mlxbf_gige_tx_deinit(priv: *mut mlxbf_gige);
}
extern "C" {
    pub fn mlxbf_gige_handle_tx_complete(priv: *mut mlxbf_gige) -> bool;
}
extern "C" {
    pub fn mlxbf_gige_request_irqs(priv: *mut mlxbf_gige) -> c_int;
}
extern "C" {
    pub fn mlxbf_gige_free_irqs(priv: *mut mlxbf_gige);
}
extern "C" {
    pub fn mlxbf_gige_poll(napi: *mut napi_struct, budget: c_int) -> c_int;
}
extern "C" {
    pub fn mlxbf_gige_update_tx_wqe_next(priv: *mut mlxbf_gige);
}
