//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/stmicro/stmmac/dwmac100.h
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
//

// ----------------------------------------------------------------------------
// MAC BLOCK defines
// ---------------------------------------------------------------------------
// MAC CSR offset
pub const MAC_CONTROL: c_uint = 0x00000000	/* MAC Control */;
pub const MAC_ADDR_HIGH: c_uint = 0x00000004	/* MAC Address High */;
pub const MAC_ADDR_LOW: c_uint = 0x00000008	/* MAC Address Low */;
pub const MAC_HASH_HIGH: c_uint = 0x0000000c	/* Multicast Hash Table High */;
pub const MAC_HASH_LOW: c_uint = 0x00000010	/* Multicast Hash Table Low */;
pub const MAC_MII_ADDR: c_uint = 0x00000014	/* MII Address */;
pub const MAC_MII_DATA: c_uint = 0x00000018	/* MII Data */;
pub const MAC_FLOW_CTRL: c_uint = 0x0000001c	/* Flow Control */;
pub const MAC_VLAN1: c_uint = 0x00000020	/* VLAN1 Tag */;
pub const MAC_VLAN2: c_uint = 0x00000024	/* VLAN2 Tag */;
// MAC CTRL defines
pub const MAC_CONTROL_HBD: c_uint = 0x10000000	/* Heartbeat Disable */;
pub const MAC_CONTROL_PS: c_uint = 0x08000000	/* Port Select */;
pub const MAC_CONTROL_OM: c_uint = 0x00200000	/* Loopback Operating Mode */;
pub const MAC_CONTROL_F: c_uint = 0x00100000	/* Full Duplex Mode */;
pub const MAC_CONTROL_PM: c_uint = 0x00080000	/* Pass All Multicast */;
pub const MAC_CONTROL_PR: c_uint = 0x00040000	/* Promiscuous Mode */;
pub const MAC_CONTROL_IF: c_uint = 0x00020000	/* Inverse Filtering */;
pub const MAC_CONTROL_HO: c_uint = 0x00008000	/* Hash Only Filtering Mode */;
pub const MAC_CONTROL_HP: c_uint = 0x00002000	/* Hash/Perfect Filtering Mode */;

// MAC FLOW CTRL defines

pub const MAC_FLOW_CTRL_ENABLE: c_uint = 0x00000002	/* Flow Control Enable */;
// ----------------------------------------------------------------------------
// DMA BLOCK defines
// ---------------------------------------------------------------------------
// DMA Bus Mode register defines

pub const DMA_BUS_MODE_DEFAULT: c_uint = 0x00000000;
// Transmit Threshold Control
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ttc_control {
    DMA_CONTROL_TTC_DEFAULT = 0x00000000,	/* Threshold is 32 DWORDS */
    DMA_CONTROL_TTC_64 = 0x00004000,	/* Threshold is 64 DWORDS */
    DMA_CONTROL_TTC_128 = 0x00008000,	/* Threshold is 128 DWORDS */
    DMA_CONTROL_TTC_256 = 0x0000c000,	/* Threshold is 256 DWORDS */
    DMA_CONTROL_TTC_18 = 0x00400000,	/* Threshold is 18 DWORDS */
    DMA_CONTROL_TTC_24 = 0x00404000,	/* Threshold is 24 DWORDS */
    DMA_CONTROL_TTC_32 = 0x00408000,	/* Threshold is 32 DWORDS */
    DMA_CONTROL_TTC_40 = 0x0040c000,	/* Threshold is 40 DWORDS */
    DMA_CONTROL_SE = 0x00000008,	/* Stop On Empty */
    DMA_CONTROL_OSF = 0x00000004,	/* Operate On 2nd Frame */
}

// STMAC110 DMA Missed Frame Counter register defines
pub const DMA_MISSED_FRAME_OVE: c_uint = 0x10000000	/* FIFO Overflow Overflow */;
pub const DMA_MISSED_FRAME_OVE_CNTR: c_uint = 0x0ffe0000	/* Overflow Frame Counter */;
pub const DMA_MISSED_FRAME_OVE_M: c_uint = 0x00010000	/* Missed Frame Overflow */;
pub const DMA_MISSED_FRAME_M_CNTR: c_uint = 0x0000ffff	/* Missed Frame Couinter */;
