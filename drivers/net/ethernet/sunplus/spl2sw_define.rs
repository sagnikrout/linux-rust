//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/sunplus/spl2sw_define.h
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
// Copyright Sunplus Technology Co., Ltd.
// All rights reserved.
//

// Interrupt status

// Address table search

// Address table status

// Wt mac ad0

// W mac 15_0 bus

// W mac 47_16 bus

// PVID config 0

// VLAN member config 0

// VLAN member config 1

// Port ability

// CPU control

// Port control 0

// Port control 1

// Port control 2

// Switch Global control

// LED port 0

// PHY control register 0

// PHY control register 1

// MAC force mode

// CPU transmit trigger

// Config descriptor queue

// Tx descriptor
// cmd1

// cmd2

// Rx descriptor
// cmd1

// cmd2

// structure of descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct spl2sw_mac_desc {
    pub cmd1: u32,
    pub cmd2: u32,
    pub addr1: u32,
    pub addr2: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct spl2sw_skb_info {
    pub skb: *mut sk_buff,
    pub mapping: u32,
    pub len: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct spl2sw_common {
    pub l2sw_reg_base: *mut void __iomem,
    pub pdev: *mut platform_device,
    pub rstc: *mut reset_control,
    pub clk: *mut clk,
    pub desc_base: *mut c_void,
    pub desc_dma: dma_addr_t,
    pub desc_size: i32,
    pub rx_desc: [*mut spl2sw_mac_desc; RX_DESC_QUEUE_NUM],
    pub rx_skb_info: [*mut spl2sw_skb_info; RX_DESC_QUEUE_NUM],
    pub rx_pos: [u32; RX_DESC_QUEUE_NUM],
    pub rx_desc_num: [u32; RX_DESC_QUEUE_NUM],
    pub rx_desc_buff_size: u32,
    pub tx_desc: *mut spl2sw_mac_desc,
    pub tx_temp_skb_info: [spl2sw_skb_info; TX_DESC_NUM],
    pub tx_done_pos: u32,
    pub tx_pos: u32,
    pub tx_desc_full: u32,
    pub ndev: [*mut net_device; MAX_NETDEV_NUM],
    pub mii_bus: *mut mii_bus,
    pub rx_napi: napi_struct,
    pub tx_napi: napi_struct,
    pub /: *mut *mut spinlock_t tx_lock; / spinlock for accessing tx buffer,
    pub /: *mut *mut spinlock_t mdio_lock; / spinlock for mdio commands,
    pub /: *mut *mut spinlock_t int_mask_lock; / spinlock for accessing int mask reg.,
    pub enable: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct spl2sw_mac {
    pub ndev: *mut net_device,
    pub comm: *mut spl2sw_common,
    pub mac_addr: [u8; ETH_ALEN],
    pub phy_mode: phy_interface_t,
    pub phy_node: *mut device_node,
    pub lan_port: u8,
    pub to_vlan: u8,
    pub vlan_id: u8,
}
