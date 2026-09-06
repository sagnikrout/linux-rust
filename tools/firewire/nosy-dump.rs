//! Automatically rewritten from C Header to Rust Module
//! Source: tools/firewire/nosy-dump.h
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

// Macro flag: #define __nosy_dump_h__

pub const ACK_NO_ACK: c_uint = 0x0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct phy_packet {
    pub timestamp: u32,
    pub zero:24: u32,
    pub phy_id:6: u32,
    pub identifier:2: u32,
    pub link_on: } common,,
    pub zero:16: u32,
    pub gap_count:6: u32,
    pub set_gap_count:1: u32,
    pub set_root:1: u32,
    pub root_id:6: u32,
    pub identifier:2: u32,
    pub phy_config: },
    pub more_packets:1: u32,
    pub initiated_reset:1: u32,
    pub port2:2: u32,
    pub port1:2: u32,
    pub port0:2: u32,
    pub power_class:3: u32,
    pub contender:1: u32,
    pub phy_delay:2: u32,
    pub phy_speed:2: u32,
    pub gap_count:6: u32,
    pub link_active:1: u32,
    pub extended:1: u32,
    pub phy_id:6: u32,
    pub identifier:2: u32,
    pub self_id: },
    pub more_packets:1: u32,
    pub reserved1:1: u32,
    pub porth:2: u32,
    pub portg:2: u32,
    pub portf:2: u32,
    pub porte:2: u32,
    pub portd:2: u32,
    pub portc:2: u32,
    pub portb:2: u32,
    pub porta:2: u32,
    pub reserved0:2: u32,
    pub sequence:3: u32,
    pub extended:1: u32,
    pub phy_id:6: u32,
    pub identifier:2: u32,
    pub ext_self_id: },
}

pub const TCODE_PHY_PACKET: c_uint = 0x10;
pub const PHY_PACKET_CONFIGURATION: c_uint = 0x00;
pub const PHY_PACKET_LINK_ON: c_uint = 0x01;
pub const PHY_PACKET_SELF_ID: c_uint = 0x02;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct link_packet {
    pub timestamp: u32,
    pub priority:4: u32,
    pub tcode:4: u32,
    pub rt:2: u32,
    pub tlabel:6: u32,
    pub destination:16: u32,
    pub offset_high:16: u32,
    pub source:16: u32,
    pub offset_low: u32,
    pub common: },
    pub common: [u32; 3],
    pub crc: u32,
    pub read_quadlet: },
    pub common: [u32; 3],
    pub data: u32,
    pub crc: u32,
    pub read_quadlet_response: },
    pub common: [u32; 3],
    pub extended_tcode:16: u32,
    pub data_length:16: u32,
    pub crc: u32,
    pub read_block: },
    pub common: [u32; 3],
    pub extended_tcode:16: u32,
    pub data_length:16: u32,
    pub crc: u32,
    pub data: [u32; 0],
// crc and ack follows.
    pub read_block_response: },
    pub common: [u32; 3],
    pub data: u32,
    pub crc: u32,
    pub write_quadlet: },
    pub common: [u32; 3],
    pub extended_tcode:16: u32,
    pub data_length:16: u32,
    pub crc: u32,
    pub data: [u32; 0],
// crc and ack follows.
    pub write_block: },
    pub common: [u32; 3],
    pub crc: u32,
    pub write_response: },
    pub common: [u32; 3],
    pub data: u32,
    pub crc: u32,
    pub cycle_start: },
    pub sy:4: u32,
    pub tcode:4: u32,
    pub channel:6: u32,
    pub tag:2: u32,
    pub data_length:16: u32,
    pub crc: u32,
    pub iso_data: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct subaction {
    pub ack: u32,
    pub length: usize,
    pub link: list,
    pub packet: link_packet,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct link_transaction {
    pub tlabel: int request_node, response_node,,
    pub response: *mut *mut subaction request,,
    pub response_list: list request_list,,
    pub link: list,
}

extern "C" {
    pub fn decode_fcp(t: *mut link_transaction) -> c_int;
}
