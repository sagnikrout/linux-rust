//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/sgi/meth.h
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


// version dependencies have been confined to a separate file
// Tunable parameters

// Internal constants

pub const METH_RX_BUFF_SIZE: c_int = 4096;

pub const RX_BUCKET_SIZE: c_int = 256;
// For more detailed explanations of what each field menas,
// tx status vector is written over tx command header upon
//
// Each packet is 128 bytes long.
// It consists of header, 0-3 concatination
// buffer pointers and up to 120 data bytes.
//
pub const TX_INFO_RPTR: c_uint = 0x00FF0000;
pub const TX_INFO_WPTR: c_uint = 0x000000FF;
// Bits in METH_MAC

// selects ignored

// Note: when loopback is set this bit becomes collision control.  Setting this bit will
// cause a collision to be reported.
// Bits 5 and 6 are used to determine the Destination address filter mode

pub const METH_ACCEPT_MCAST: c_uint = 0x20	/* 01: Accept physical, broadcast, and multicast filter matches only */;
pub const METH_ACCEPT_AMCAST: c_uint = 0x40	/* 10: Accept physical, broadcast, and all multicast packets */;
pub const METH_PROMISC: c_uint = 0x60		/* 11: Promiscious mode */;

pub const METH_MAC_IPG: c_uint = 0x1ffff00;

// 0x172e5c00 */ /* 23, 23, 23 */ /*0x54A9500 *//*21,21,21
// Bits 8 through 14 are used to determine Inter-Packet Gap between "Back to Back" packets
// The gap depends on the clock speed of the link, 80ns per increment for 100baseT, 800ns
// per increment for 10BaseT
// Bits 15 through 21 are used to determine IPGR1
// Bits 22 through 28 are used to determine IPGR2

// 000: Initial revision
// 001: First revision, Improved TX concatenation
// DMA control bits

// RX FIFO MCL Info bits

// RX status bits

// Bits in METH_INT
// Write _1_ to corresponding bit to clear

// 1: A TX message had the INT request bit set, the packet has been sent.

// 1: A memory error occurred during DMA, DMA stopped, Fatal

// #define METH_INT_RX_RPTR_MASK 0x0001F00*/		/* Bits 8 through 12 alias of RX read-pointer
pub const METH_INT_RX_RPTR_MASK: c_uint = 0x0000F00		/* Bits 8 through 11 alias of RX read-pointer - so, is Rx FIFO 16 or 32 entry?*/;
// Bits 13 through 15 are always 0.
pub const METH_INT_TX_RPTR_MASK: c_uint = 0x1FF0000        /* Bits 16 through 24 alias of TX read-pointer */;
pub const METH_INT_RX_SEQ_MASK: c_uint = 0x2E000000	/* Bits 25 through 29 are the starting seq number for the message at the */;
// top of the queue

// TX status bits

// Tx command header bits

// Phy MDIO interface busy flag

pub const MDIO_DATA_MASK: c_uint = 0xFFFF;
// PHY defines
pub const PHY_QS6612X: c_uint = 0x0181441    /* Quality TX */;
pub const PHY_ICS1889: c_uint = 0x0015F41    /* ICS FX */;
pub const PHY_ICS1890: c_uint = 0x0015F42    /* ICS TX */;
pub const PHY_DP83840: c_uint = 0x20005C0    /* National TX */;
