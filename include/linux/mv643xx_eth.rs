//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mv643xx_eth.h
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
//
// MV-643XX ethernet platform device data definition file.
//

pub const MV643XX_ETH_SHARED_REGS: c_uint = 0x2000;
pub const MV643XX_ETH_SHARED_REGS_SIZE: c_uint = 0x2000;
pub const MV643XX_ETH_BAR_4: c_uint = 0x2220;
pub const MV643XX_ETH_SIZE_REG_4: c_uint = 0x2224;
pub const MV643XX_ETH_BASE_ADDR_ENABLE_REG: c_uint = 0x2290;
pub const MV643XX_TX_CSUM_DEFAULT_LIMIT: c_int = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mv643xx_eth_shared_platform_data {
    pub dram: *mut mbus_dram_target_info,
//
// Max packet size for Tx IP/Layer 4 checksum, when set to 0, default
// limit of 9KiB will be used.
//
    pub tx_csum_limit: c_int,
}

pub const MV643XX_ETH_PHY_ADDR_DEFAULT: c_int = 0;

pub const MV643XX_ETH_PHY_NONE: c_uint = 0xff;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mv643xx_eth_platform_data {
//
// Pointer back to our parent instance, and our port number.
//
    pub shared: *mut platform_device,
    pub port_number: c_int,
//
// Whether a PHY is present, and if yes, at which address.
//
    pub phy_addr: c_int,
    pub phy_node: *mut device_node,
//
// Use this MAC address if it is valid, overriding the
// address that is already in the hardware.
//
    pub mac_addr: [u8; ETH_ALEN],
//
// If speed is 0, autonegotiation is enabled.
// Valid values for speed: 0, SPEED_10, SPEED_100, SPEED_1000.
// Valid values for duplex: DUPLEX_HALF, DUPLEX_FULL.
//
    pub speed: c_int,
    pub duplex: c_int,
    pub interface: phy_interface_t,
//
// How many RX/TX queues to use.
//
    pub rx_queue_count: c_int,
    pub tx_queue_count: c_int,
//
// Override default RX/TX queue sizes if nonzero.
//
    pub rx_queue_size: c_int,
    pub tx_queue_size: c_int,
//
// Use on-chip SRAM for RX/TX descriptors if size is nonzero
// and sufficient to contain all descriptors for the requested
// ring sizes.
//
    pub rx_sram_addr: c_ulong,
    pub rx_sram_size: c_int,
    pub tx_sram_addr: c_ulong,
    pub tx_sram_size: c_int,
}
