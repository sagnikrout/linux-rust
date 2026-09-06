//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/mellanox/mlx4/en_port.h
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


//
// Copyright (c) 2007 Mellanox Technologies. All rights reserved.
//
// This software is available to you under a choice of one of two
// licenses.  You may choose to be licensed under the terms of the GNU
// General Public License (GPL) Version 2, available from the file
// COPYING in the main directory of this source tree, or the
// OpenIB.org BSD license below:
//
// Redistribution and use in source and binary forms, with or
// without modification, are permitted provided that the following
// conditions are met:
//
// - Redistributions of source code must retain the above
// copyright notice, this list of conditions and the following
// disclaimer.
//
// - Redistributions in binary form must reproduce the above
// copyright notice, this list of conditions and the following
// disclaimer in the documentation and/or other materials
// provided with the distribution.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS
// BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN
// ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//
pub const SET_PORT_PROMISC_SHIFT: c_int = 31;
pub const SET_PORT_MC_PROMISC_SHIFT: c_int = 30;
pub const MLX4_EN_NUM_TC: c_int = 8;
pub const VLAN_FLTR_SIZE: c_int = 128;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_set_vlan_fltr_mbox {
    pub entry: [__be32; VLAN_FLTR_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx4_link_mode {
    MLX4_1000BASE_CX_SGMII	 = 0,
    MLX4_1000BASE_KX	 = 1,
    MLX4_10GBASE_CX4	 = 2,
    MLX4_10GBASE_KX4	 = 3,
    MLX4_10GBASE_KR		 = 4,
    MLX4_20GBASE_KR2	 = 5,
    MLX4_40GBASE_CR4	 = 6,
    MLX4_40GBASE_KR4	 = 7,
    MLX4_56GBASE_KR4	 = 8,
    MLX4_10GBASE_CR		 = 12,
    MLX4_10GBASE_SR		 = 13,
    MLX4_40GBASE_SR4	 = 15,
    MLX4_56GBASE_CR4	 = 17,
    MLX4_56GBASE_SR4	 = 18,
    MLX4_100BASE_TX		 = 24,
    MLX4_1000BASE_T		 = 25,
    MLX4_10GBASE_T		 = 26,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_en_query_port_context {
    pub link_up: u8,
pub const MLX4_EN_LINK_UP_MASK: c_uint = 0x80;
pub const MLX4_EN_ANC_MASK: c_uint = 0x40;
    pub autoneg: u8,
pub const MLX4_EN_AUTONEG_MASK: c_uint = 0x80;
    pub mtu: __be16,
    pub reserved2: u8,
    pub link_speed: u8,
pub const MLX4_EN_SPEED_MASK: c_uint = 0x6f;
    pub reserved3: [u16; 5],
    pub mac: __be64,
    pub transceiver: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_en_stat_out_mbox {
// Received frames with a length of 64 octets
    pub R64_prio_0: __be64,
    pub R64_prio_1: __be64,
    pub R64_prio_2: __be64,
    pub R64_prio_3: __be64,
    pub R64_prio_4: __be64,
    pub R64_prio_5: __be64,
    pub R64_prio_6: __be64,
    pub R64_prio_7: __be64,
    pub R64_novlan: __be64,
// Received frames with a length of 127 octets
    pub R127_prio_0: __be64,
    pub R127_prio_1: __be64,
    pub R127_prio_2: __be64,
    pub R127_prio_3: __be64,
    pub R127_prio_4: __be64,
    pub R127_prio_5: __be64,
    pub R127_prio_6: __be64,
    pub R127_prio_7: __be64,
    pub R127_novlan: __be64,
// Received frames with a length of 255 octets
    pub R255_prio_0: __be64,
    pub R255_prio_1: __be64,
    pub R255_prio_2: __be64,
    pub R255_prio_3: __be64,
    pub R255_prio_4: __be64,
    pub R255_prio_5: __be64,
    pub R255_prio_6: __be64,
    pub R255_prio_7: __be64,
    pub R255_novlan: __be64,
// Received frames with a length of 511 octets
    pub R511_prio_0: __be64,
    pub R511_prio_1: __be64,
    pub R511_prio_2: __be64,
    pub R511_prio_3: __be64,
    pub R511_prio_4: __be64,
    pub R511_prio_5: __be64,
    pub R511_prio_6: __be64,
    pub R511_prio_7: __be64,
    pub R511_novlan: __be64,
// Received frames with a length of 1023 octets
    pub R1023_prio_0: __be64,
    pub R1023_prio_1: __be64,
    pub R1023_prio_2: __be64,
    pub R1023_prio_3: __be64,
    pub R1023_prio_4: __be64,
    pub R1023_prio_5: __be64,
    pub R1023_prio_6: __be64,
    pub R1023_prio_7: __be64,
    pub R1023_novlan: __be64,
// Received frames with a length of 1518 octets
    pub R1518_prio_0: __be64,
    pub R1518_prio_1: __be64,
    pub R1518_prio_2: __be64,
    pub R1518_prio_3: __be64,
    pub R1518_prio_4: __be64,
    pub R1518_prio_5: __be64,
    pub R1518_prio_6: __be64,
    pub R1518_prio_7: __be64,
    pub R1518_novlan: __be64,
// Received frames with a length of 1522 octets
    pub R1522_prio_0: __be64,
    pub R1522_prio_1: __be64,
    pub R1522_prio_2: __be64,
    pub R1522_prio_3: __be64,
    pub R1522_prio_4: __be64,
    pub R1522_prio_5: __be64,
    pub R1522_prio_6: __be64,
    pub R1522_prio_7: __be64,
    pub R1522_novlan: __be64,
// Received frames with a length of 1548 octets
    pub R1548_prio_0: __be64,
    pub R1548_prio_1: __be64,
    pub R1548_prio_2: __be64,
    pub R1548_prio_3: __be64,
    pub R1548_prio_4: __be64,
    pub R1548_prio_5: __be64,
    pub R1548_prio_6: __be64,
    pub R1548_prio_7: __be64,
    pub R1548_novlan: __be64,
// Received frames with a length of 1548 < octets < MTU
    pub R2MTU_prio_0: __be64,
    pub R2MTU_prio_1: __be64,
    pub R2MTU_prio_2: __be64,
    pub R2MTU_prio_3: __be64,
    pub R2MTU_prio_4: __be64,
    pub R2MTU_prio_5: __be64,
    pub R2MTU_prio_6: __be64,
    pub R2MTU_prio_7: __be64,
    pub R2MTU_novlan: __be64,
// Received frames with a length of MTU< octets and good CRC
    pub RGIANT_prio_0: __be64,
    pub RGIANT_prio_1: __be64,
    pub RGIANT_prio_2: __be64,
    pub RGIANT_prio_3: __be64,
    pub RGIANT_prio_4: __be64,
    pub RGIANT_prio_5: __be64,
    pub RGIANT_prio_6: __be64,
    pub RGIANT_prio_7: __be64,
    pub RGIANT_novlan: __be64,
// Received broadcast frames with good CRC
    pub RBCAST_prio_0: __be64,
    pub RBCAST_prio_1: __be64,
    pub RBCAST_prio_2: __be64,
    pub RBCAST_prio_3: __be64,
    pub RBCAST_prio_4: __be64,
    pub RBCAST_prio_5: __be64,
    pub RBCAST_prio_6: __be64,
    pub RBCAST_prio_7: __be64,
    pub RBCAST_novlan: __be64,
// Received multicast frames with good CRC
    pub MCAST_prio_0: __be64,
    pub MCAST_prio_1: __be64,
    pub MCAST_prio_2: __be64,
    pub MCAST_prio_3: __be64,
    pub MCAST_prio_4: __be64,
    pub MCAST_prio_5: __be64,
    pub MCAST_prio_6: __be64,
    pub MCAST_prio_7: __be64,
    pub MCAST_novlan: __be64,
// Received unicast not short or GIANT frames with good CRC
    pub RTOTG_prio_0: __be64,
    pub RTOTG_prio_1: __be64,
    pub RTOTG_prio_2: __be64,
    pub RTOTG_prio_3: __be64,
    pub RTOTG_prio_4: __be64,
    pub RTOTG_prio_5: __be64,
    pub RTOTG_prio_6: __be64,
    pub RTOTG_prio_7: __be64,
    pub RTOTG_novlan: __be64,
// Count of total octets of received frames, includes framing characters
    pub RTTLOCT_prio_0: __be64,
// Count of total octets of received frames, not including framing
    pub RTTLOCT_NOFRM_prio_0: __be64,
// Count of Total number of octets received
    pub ROCT_prio_0: __be64,
    pub RTTLOCT_prio_1: __be64,
    pub RTTLOCT_NOFRM_prio_1: __be64,
    pub ROCT_prio_1: __be64,
    pub RTTLOCT_prio_2: __be64,
    pub RTTLOCT_NOFRM_prio_2: __be64,
    pub ROCT_prio_2: __be64,
    pub RTTLOCT_prio_3: __be64,
    pub RTTLOCT_NOFRM_prio_3: __be64,
    pub ROCT_prio_3: __be64,
    pub RTTLOCT_prio_4: __be64,
    pub RTTLOCT_NOFRM_prio_4: __be64,
    pub ROCT_prio_4: __be64,
    pub RTTLOCT_prio_5: __be64,
    pub RTTLOCT_NOFRM_prio_5: __be64,
    pub ROCT_prio_5: __be64,
    pub RTTLOCT_prio_6: __be64,
    pub RTTLOCT_NOFRM_prio_6: __be64,
    pub ROCT_prio_6: __be64,
    pub RTTLOCT_prio_7: __be64,
    pub RTTLOCT_NOFRM_prio_7: __be64,
    pub ROCT_prio_7: __be64,
    pub RTTLOCT_novlan: __be64,
    pub RTTLOCT_NOFRM_novlan: __be64,
    pub ROCT_novlan: __be64,
// Count of Total received frames including bad frames
    pub RTOT_prio_0: __be64,
// Count of  Total number of received frames with 802.1Q encapsulation
    pub R1Q_prio_0: __be64,
    pub reserved1: __be64,
    pub RTOT_prio_1: __be64,
    pub R1Q_prio_1: __be64,
    pub reserved2: __be64,
    pub RTOT_prio_2: __be64,
    pub R1Q_prio_2: __be64,
    pub reserved3: __be64,
    pub RTOT_prio_3: __be64,
    pub R1Q_prio_3: __be64,
    pub reserved4: __be64,
    pub RTOT_prio_4: __be64,
    pub R1Q_prio_4: __be64,
    pub reserved5: __be64,
    pub RTOT_prio_5: __be64,
    pub R1Q_prio_5: __be64,
    pub reserved6: __be64,
    pub RTOT_prio_6: __be64,
    pub R1Q_prio_6: __be64,
    pub reserved7: __be64,
    pub RTOT_prio_7: __be64,
    pub R1Q_prio_7: __be64,
    pub reserved8: __be64,
    pub RTOT_novlan: __be64,
    pub R1Q_novlan: __be64,
    pub reserved9: __be64,
// Total number of Successfully Received Control Frames
    pub RCNTL: __be64,
    pub reserved10: __be64,
    pub reserved11: __be64,
    pub reserved12: __be64,
// Count of received frames with a length/type field  value between 46
    pub RInRangeLengthErr: __be64,
// Count of received frames with length/type field between 1501 and 1535
    pub ROutRangeLengthErr: __be64,
// Count of received frames that are longer than max allowed size for
    pub RFrmTooLong: __be64,
// Count frames received with PCS error
    pub PCS: __be64,
// Transmit frames with a length of 64 octets
    pub T64_prio_0: __be64,
    pub T64_prio_1: __be64,
    pub T64_prio_2: __be64,
    pub T64_prio_3: __be64,
    pub T64_prio_4: __be64,
    pub T64_prio_5: __be64,
    pub T64_prio_6: __be64,
    pub T64_prio_7: __be64,
    pub T64_novlan: __be64,
    pub T64_loopbk: __be64,
// Transmit frames with a length of 65 to 127 octets.
    pub T127_prio_0: __be64,
    pub T127_prio_1: __be64,
    pub T127_prio_2: __be64,
    pub T127_prio_3: __be64,
    pub T127_prio_4: __be64,
    pub T127_prio_5: __be64,
    pub T127_prio_6: __be64,
    pub T127_prio_7: __be64,
    pub T127_novlan: __be64,
    pub T127_loopbk: __be64,
// Transmit frames with a length of 128 to 255 octets
    pub T255_prio_0: __be64,
    pub T255_prio_1: __be64,
    pub T255_prio_2: __be64,
    pub T255_prio_3: __be64,
    pub T255_prio_4: __be64,
    pub T255_prio_5: __be64,
    pub T255_prio_6: __be64,
    pub T255_prio_7: __be64,
    pub T255_novlan: __be64,
    pub T255_loopbk: __be64,
// Transmit frames with a length of 256 to 511 octets
    pub T511_prio_0: __be64,
    pub T511_prio_1: __be64,
    pub T511_prio_2: __be64,
    pub T511_prio_3: __be64,
    pub T511_prio_4: __be64,
    pub T511_prio_5: __be64,
    pub T511_prio_6: __be64,
    pub T511_prio_7: __be64,
    pub T511_novlan: __be64,
    pub T511_loopbk: __be64,
// Transmit frames with a length of 512 to 1023 octets
    pub T1023_prio_0: __be64,
    pub T1023_prio_1: __be64,
    pub T1023_prio_2: __be64,
    pub T1023_prio_3: __be64,
    pub T1023_prio_4: __be64,
    pub T1023_prio_5: __be64,
    pub T1023_prio_6: __be64,
    pub T1023_prio_7: __be64,
    pub T1023_novlan: __be64,
    pub T1023_loopbk: __be64,
// Transmit frames with a length of 1024 to 1518 octets
    pub T1518_prio_0: __be64,
    pub T1518_prio_1: __be64,
    pub T1518_prio_2: __be64,
    pub T1518_prio_3: __be64,
    pub T1518_prio_4: __be64,
    pub T1518_prio_5: __be64,
    pub T1518_prio_6: __be64,
    pub T1518_prio_7: __be64,
    pub T1518_novlan: __be64,
    pub T1518_loopbk: __be64,
// Counts transmit frames with a length of 1519 to 1522 bytes
    pub T1522_prio_0: __be64,
    pub T1522_prio_1: __be64,
    pub T1522_prio_2: __be64,
    pub T1522_prio_3: __be64,
    pub T1522_prio_4: __be64,
    pub T1522_prio_5: __be64,
    pub T1522_prio_6: __be64,
    pub T1522_prio_7: __be64,
    pub T1522_novlan: __be64,
    pub T1522_loopbk: __be64,
// Transmit frames with a length of 1523 to 1548 octets
    pub T1548_prio_0: __be64,
    pub T1548_prio_1: __be64,
    pub T1548_prio_2: __be64,
    pub T1548_prio_3: __be64,
    pub T1548_prio_4: __be64,
    pub T1548_prio_5: __be64,
    pub T1548_prio_6: __be64,
    pub T1548_prio_7: __be64,
    pub T1548_novlan: __be64,
    pub T1548_loopbk: __be64,
// Counts transmit frames with a length of 1549 to MTU bytes
    pub T2MTU_prio_0: __be64,
    pub T2MTU_prio_1: __be64,
    pub T2MTU_prio_2: __be64,
    pub T2MTU_prio_3: __be64,
    pub T2MTU_prio_4: __be64,
    pub T2MTU_prio_5: __be64,
    pub T2MTU_prio_6: __be64,
    pub T2MTU_prio_7: __be64,
    pub T2MTU_novlan: __be64,
    pub T2MTU_loopbk: __be64,
// Transmit frames with a length greater than MTU octets and a good CRC.
    pub TGIANT_prio_0: __be64,
    pub TGIANT_prio_1: __be64,
    pub TGIANT_prio_2: __be64,
    pub TGIANT_prio_3: __be64,
    pub TGIANT_prio_4: __be64,
    pub TGIANT_prio_5: __be64,
    pub TGIANT_prio_6: __be64,
    pub TGIANT_prio_7: __be64,
    pub TGIANT_novlan: __be64,
    pub TGIANT_loopbk: __be64,
// Transmit broadcast frames with a good CRC
    pub TBCAST_prio_0: __be64,
    pub TBCAST_prio_1: __be64,
    pub TBCAST_prio_2: __be64,
    pub TBCAST_prio_3: __be64,
    pub TBCAST_prio_4: __be64,
    pub TBCAST_prio_5: __be64,
    pub TBCAST_prio_6: __be64,
    pub TBCAST_prio_7: __be64,
    pub TBCAST_novlan: __be64,
    pub TBCAST_loopbk: __be64,
// Transmit multicast frames with a good CRC
    pub TMCAST_prio_0: __be64,
    pub TMCAST_prio_1: __be64,
    pub TMCAST_prio_2: __be64,
    pub TMCAST_prio_3: __be64,
    pub TMCAST_prio_4: __be64,
    pub TMCAST_prio_5: __be64,
    pub TMCAST_prio_6: __be64,
    pub TMCAST_prio_7: __be64,
    pub TMCAST_novlan: __be64,
    pub TMCAST_loopbk: __be64,
// Transmit good frames that are neither broadcast nor multicast
    pub TTOTG_prio_0: __be64,
    pub TTOTG_prio_1: __be64,
    pub TTOTG_prio_2: __be64,
    pub TTOTG_prio_3: __be64,
    pub TTOTG_prio_4: __be64,
    pub TTOTG_prio_5: __be64,
    pub TTOTG_prio_6: __be64,
    pub TTOTG_prio_7: __be64,
    pub TTOTG_novlan: __be64,
    pub TTOTG_loopbk: __be64,
// total octets of transmitted frames, including framing characters
    pub TTTLOCT_prio_0: __be64,
// total octets of transmitted frames, not including framing characters
    pub TTTLOCT_NOFRM_prio_0: __be64,
// ifOutOctets
    pub TOCT_prio_0: __be64,
    pub TTTLOCT_prio_1: __be64,
    pub TTTLOCT_NOFRM_prio_1: __be64,
    pub TOCT_prio_1: __be64,
    pub TTTLOCT_prio_2: __be64,
    pub TTTLOCT_NOFRM_prio_2: __be64,
    pub TOCT_prio_2: __be64,
    pub TTTLOCT_prio_3: __be64,
    pub TTTLOCT_NOFRM_prio_3: __be64,
    pub TOCT_prio_3: __be64,
    pub TTTLOCT_prio_4: __be64,
    pub TTTLOCT_NOFRM_prio_4: __be64,
    pub TOCT_prio_4: __be64,
    pub TTTLOCT_prio_5: __be64,
    pub TTTLOCT_NOFRM_prio_5: __be64,
    pub TOCT_prio_5: __be64,
    pub TTTLOCT_prio_6: __be64,
    pub TTTLOCT_NOFRM_prio_6: __be64,
    pub TOCT_prio_6: __be64,
    pub TTTLOCT_prio_7: __be64,
    pub TTTLOCT_NOFRM_prio_7: __be64,
    pub TOCT_prio_7: __be64,
    pub TTTLOCT_novlan: __be64,
    pub TTTLOCT_NOFRM_novlan: __be64,
    pub TOCT_novlan: __be64,
    pub TTTLOCT_loopbk: __be64,
    pub TTTLOCT_NOFRM_loopbk: __be64,
    pub TOCT_loopbk: __be64,
// Total frames transmitted with a good CRC that are not aborted
    pub TTOT_prio_0: __be64,
// Total number of frames transmitted with 802.1Q encapsulation
    pub T1Q_prio_0: __be64,
    pub reserved13: __be64,
    pub TTOT_prio_1: __be64,
    pub T1Q_prio_1: __be64,
    pub reserved14: __be64,
    pub TTOT_prio_2: __be64,
    pub T1Q_prio_2: __be64,
    pub reserved15: __be64,
    pub TTOT_prio_3: __be64,
    pub T1Q_prio_3: __be64,
    pub reserved16: __be64,
    pub TTOT_prio_4: __be64,
    pub T1Q_prio_4: __be64,
    pub reserved17: __be64,
    pub TTOT_prio_5: __be64,
    pub T1Q_prio_5: __be64,
    pub reserved18: __be64,
    pub TTOT_prio_6: __be64,
    pub T1Q_prio_6: __be64,
    pub reserved19: __be64,
    pub TTOT_prio_7: __be64,
    pub T1Q_prio_7: __be64,
    pub reserved20: __be64,
    pub TTOT_novlan: __be64,
    pub T1Q_novlan: __be64,
    pub reserved21: __be64,
    pub TTOT_loopbk: __be64,
    pub T1Q_loopbk: __be64,
    pub reserved22: __be64,
// Received frames with a length greater than MTU octets and a bad CRC
    pub RJBBR: __be32,
// Received frames with a bad CRC that are not runts, jabbers,
    pub RCRC: __be32,
// Received frames with SFD with a length of less than 64 octets and a
    pub RRUNT: __be32,
// Received frames with a length less than 64 octets and a good CRC
    pub RSHORT: __be32,
// Total Number of Received Packets Dropped
    pub RDROP: __be32,
// Drop due to overflow
    pub RdropOvflw: __be32,
// Drop due to overflow
    pub RdropLength: __be32,
// Total of good frames. Does not include frames received with
    pub RTOTFRMS: __be32,
// Total dropped Xmited packets
    pub TDROP: __be32,
}
