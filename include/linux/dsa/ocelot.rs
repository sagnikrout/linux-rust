//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/dsa/ocelot.h
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
// Copyright 2019-2021 NXP
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocelot_skb_cb {
    pub clone: *mut sk_buff,
    pub /: *mut *mut unsigned int ptp_class; / valid only for clones,
    pub /: *mut *mut unsigned long ptp_tx_time; / valid only for clones,
    pub tstamp_lo: u32,
    pub ptp_cmd: u8,
    pub ts_id: u8,
}

pub const IFH_TAG_TYPE_C: c_int = 0;
pub const IFH_TAG_TYPE_S: c_int = 1;
pub const IFH_REW_OP_NOOP: c_uint = 0x0;
pub const IFH_REW_OP_DSCP: c_uint = 0x1;
pub const IFH_REW_OP_ONE_STEP_PTP: c_uint = 0x2;
pub const IFH_REW_OP_TWO_STEP_PTP: c_uint = 0x3;
pub const IFH_REW_OP_ORIGIN_PTP: c_uint = 0x5;
pub const OCELOT_TAG_LEN: c_int = 16;
pub const OCELOT_SHORT_PREFIX_LEN: c_int = 4;
pub const OCELOT_LONG_PREFIX_LEN: c_int = 16;

// The CPU injection header and the CPU extraction header can have 3 types of
// prefixes: long, short and no prefix. The format of the header itself is the
// same in all 3 cases.
//
// Extraction with long prefix:
//
// +-------------------+-------------------+------+------+------------+-------+
// | ff:ff:ff:ff:ff:ff | fe:ff:ff:ff:ff:ff | 8880 | 000a | extraction | frame |
// |                   |                   |      |      |   header   |       |
// +-------------------+-------------------+------+------+------------+-------+
// 48 bits             48 bits      16 bits 16 bits  128 bits
//
// Extraction with short prefix:
//
// +------+------+------------+-------+
// | 8880 | 000a | extraction | frame |
// |      |      |   header   |       |
// +------+------+------------+-------+
// 16 bits 16 bits  128 bits
//
// Extraction with no prefix:
//
// +------------+-------+
// | extraction | frame |
// |   header   |       |
// +------------+-------+
// 128 bits
//
// Injection with long prefix:
//
// +-------------------+-------------------+------+------+------------+-------+
// |      any dmac     |      any smac     | 8880 | 000a | injection  | frame |
// |                   |                   |      |      |   header   |       |
// +-------------------+-------------------+------+------+------------+-------+
// 48 bits             48 bits      16 bits 16 bits  128 bits
//
// Injection with short prefix:
//
// +------+------+------------+-------+
// | 8880 | 000a | injection  | frame |
// |      |      |   header   |       |
// +------+------+------------+-------+
// 16 bits 16 bits  128 bits
//
// Injection with no prefix:
//
// +------------+-------+
// | injection  | frame |
// |   header   |       |
// +------------+-------+
// 128 bits
//
// The injection header looks like this (network byte order, bit 127
// is part of lowest address byte in memory, bit 0 is part of highest
// address byte):
//
// +------+------+------+------+------+------+------+------+
// 127:120 |BYPASS| MASQ |          MASQ_PORT        |REW_OP|REW_OP|
// +------+------+------+------+------+------+------+------+
// 119:112 |                         REW_OP                        |
// +------+------+------+------+------+------+------+------+
// 111:104 |                         REW_VAL                       |
// +------+------+------+------+------+------+------+------+
// 103: 96 |                         REW_VAL                       |
// +------+------+------+------+------+------+------+------+
// 95: 88 |                         REW_VAL                       |
// +------+------+------+------+------+------+------+------+
// 87: 80 |                         REW_VAL                       |
// +------+------+------+------+------+------+------+------+
// 79: 72 |                          RSV                          |
// +------+------+------+------+------+------+------+------+
// 71: 64 |            RSV            |           DEST            |
// +------+------+------+------+------+------+------+------+
// 63: 56 |                         DEST                          |
// +------+------+------+------+------+------+------+------+
// 55: 48 |                          RSV                          |
// +------+------+------+------+------+------+------+------+
// 47: 40 |  RSV |         SRC_PORT          |     RSV     |TFRM_TIMER|
// +------+------+------+------+------+------+------+------+
// 39: 32 |     TFRM_TIMER     |               RSV                |
// +------+------+------+------+------+------+------+------+
// 31: 24 |  RSV |  DP  |   POP_CNT   |           CPUQ            |
// +------+------+------+------+------+------+------+------+
// 23: 16 |           CPUQ            |      QOS_CLASS     |TAG_TYPE|
// +------+------+------+------+------+------+------+------+
// 15:  8 |         PCP        |  DEI |            VID            |
// +------+------+------+------+------+------+------+------+
// 7:  0 |                          VID                          |
// +------+------+------+------+------+------+------+------+
//
// And the extraction header looks like this:
//
// +------+------+------+------+------+------+------+------+
// 127:120 |  RSV |                  REW_OP                        |
// +------+------+------+------+------+------+------+------+
// 119:112 |       REW_OP       |              REW_VAL             |
// +------+------+------+------+------+------+------+------+
// 111:104 |                         REW_VAL                       |
// +------+------+------+------+------+------+------+------+
// 103: 96 |                         REW_VAL                       |
// +------+------+------+------+------+------+------+------+
// 95: 88 |                         REW_VAL                       |
// +------+------+------+------+------+------+------+------+
// 87: 80 |       REW_VAL      |               LLEN               |
// +------+------+------+------+------+------+------+------+
// 79: 72 | LLEN |                      WLEN                      |
// +------+------+------+------+------+------+------+------+
// 71: 64 | WLEN |                      RSV                       |
// +------+------+------+------+------+------+------+------+
// 63: 56 |                          RSV                          |
// +------+------+------+------+------+------+------+------+
// 55: 48 |                          RSV                          |
// +------+------+------+------+------+------+------+------+
// 47: 40 | RSV  |          SRC_PORT         |       ACL_ID       |
// +------+------+------+------+------+------+------+------+
// 39: 32 |       ACL_ID       |  RSV |         SFLOW_ID          |
// +------+------+------+------+------+------+------+------+
// 31: 24 |ACL_HIT| DP  |  LRN_FLAGS  |           CPUQ            |
// +------+------+------+------+------+------+------+------+
// 23: 16 |           CPUQ            |      QOS_CLASS     |TAG_TYPE|
// +------+------+------+------+------+------+------+------+
// 15:  8 |         PCP        |  DEI |            VID            |
// +------+------+------+------+------+------+------+------+
// 7:  0 |                          VID                          |
// +------+------+------+------+------+------+------+------+
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct felix_deferred_xmit_work {
    pub dp: *mut dsa_port,
    pub skb: *mut sk_buff,
    pub work: kthread_work,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocelot_8021q_tagger_data {
    pub work): *mut *mut void (xmit_work_fn)(struct kthread_work,
}

// len = 60 * wlen + llen - 80;
// Determine the PTP REW_OP to use for injecting the given skb
//
// ocelot_xmit_get_vlan_info: Determine VLAN_TCI and TAG_TYPE for injected frame
// @skb: Pointer to socket buffer
// @br: Pointer to bridge device that the port is under, if any
// @vlan_tci:
// @tag_type:
//
// If the port is under a VLAN-aware bridge, remove the VLAN header from the
// payload and move it into the DSA tag, which will make the switch classify
// the packet to the bridge VLAN. Otherwise, leave the classified VLAN at zero,
// which is the pvid of standalone ports (OCELOT_STANDALONE_PVID), although not
// of VLAN-unaware bridge ports (that would be ocelot_vlan_unaware_pvid()).
// Anyway, VID 0 is fine because it is stripped on egress for these port modes,
// and source address learning is not performed for packets injected from the
// CPU anyway, so it doesn't matter that the VID is "wrong".
//
// vlan_tci = 0;
// tag_type = IFH_TAG_TYPE_C;
// vlan_tci = tci;
// tag_type = (proto != ETH_P_8021Q) ? IFH_TAG_TYPE_S : IFH_TAG_TYPE_C;
