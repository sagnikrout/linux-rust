//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/platform_data/xilinx-ll-temac.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ll_temac_platform_data {
    pub /: *mut *mut bool txcsum; / Enable/disable TX checksum,
    pub /: *mut *mut bool rxcsum; / Enable/disable RX checksum,
    pub /: *mut *mut u8 mac_addr[ETH_ALEN]; / MAC address (6 bytes),
// Clock frequency for input to MDIO clock generator
    pub mdio_clk_freq: u32,
    pub /: *mut *mut unsigned long long mdio_bus_id; / Unique id for MDIO bus,
    pub /: *mut *mut int phy_addr; / Address of the PHY to connect to,
    pub /: *mut *mut phy_interface_t phy_interface; / PHY interface mode,
    pub /: *mut *mut bool reg_little_endian; / Little endian TEMAC register access,
    pub /: *mut *mut bool dma_little_endian; / Little endian DMA register access,
// Pre-initialized mutex to use for synchronizing indirect
// register access.  When using both interfaces of a single
// TEMAC IP block, the same mutex should be passed here, as
// they share the same DCR bus bridge.
//
    pub indirect_lock: *mut spinlock_t,
// DMA channel control setup
    pub /: *mut *mut u8 tx_irq_timeout; / TX Interrupt Delay Time-out,
    pub /: *mut *mut u8 tx_irq_count; / TX Interrupt Coalescing Threshold Count,
    pub /: *mut *mut u8 rx_irq_timeout; / RX Interrupt Delay Time-out,
    pub /: *mut *mut u8 rx_irq_count; / RX Interrupt Coalescing Threshold Count,
}
