//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/if_macsec.h
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


// SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note
//
// include/uapi/linux/if_macsec.h - MACsec device
//
// Copyright (c) 2015 Sabrina Dubroca <sd@queasysnail.net>
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation; either version 2 of the License, or
// (at your option) any later version.
//

pub const MACSEC_GENL_VERSION: c_int = 1;
pub const MACSEC_MAX_KEY_LEN: c_int = 128;
pub const MACSEC_KEYID_LEN: c_int = 16;
pub const MACSEC_SALT_LEN: c_int = 12;
// cipher IDs as per IEEE802.1AE-2018 (Table 14-1)
pub const MACSEC_CIPHER_ID_GCM_AES_128: c_uint = 0x0080C20001000001ULL;
pub const MACSEC_CIPHER_ID_GCM_AES_256: c_uint = 0x0080C20001000002ULL;
pub const MACSEC_CIPHER_ID_GCM_AES_XPN_128: c_uint = 0x0080C20001000003ULL;
pub const MACSEC_CIPHER_ID_GCM_AES_XPN_256: c_uint = 0x0080C20001000004ULL;
// deprecated cipher ID for GCM-AES-128
pub const MACSEC_DEFAULT_CIPHER_ID: c_uint = 0x0080020001000001ULL;

pub const MACSEC_MIN_ICV_LEN: c_int = 8;
pub const MACSEC_MAX_ICV_LEN: c_int = 32;
// upper limit for ICV length as recommended by IEEE802.1AE-2006
pub const MACSEC_STD_ICV_LEN: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum macsec_attrs {
    MACSEC_ATTR_UNSPEC,
    MACSEC_ATTR_IFINDEX,     /* u32, ifindex of the MACsec netdevice */
    MACSEC_ATTR_RXSC_CONFIG, /* config, nested macsec_rxsc_attrs */
    MACSEC_ATTR_SA_CONFIG,   /* config, nested macsec_sa_attrs */
    MACSEC_ATTR_SECY,        /* dump, nested macsec_secy_attrs */
    MACSEC_ATTR_TXSA_LIST,   /* dump, nested, macsec_sa_attrs for each TXSA */
    MACSEC_ATTR_RXSC_LIST,   /* dump, nested, macsec_rxsc_attrs for each RXSC */
    MACSEC_ATTR_TXSC_STATS,  /* dump, nested, macsec_txsc_stats_attr */
    MACSEC_ATTR_SECY_STATS,  /* dump, nested, macsec_secy_stats_attr */
    MACSEC_ATTR_OFFLOAD,     /* config, nested, macsec_offload_attrs */
    __MACSEC_ATTR_END,
    NUM_MACSEC_ATTR = __MACSEC_ATTR_END,
    MACSEC_ATTR_MAX = __MACSEC_ATTR_END - 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum macsec_secy_attrs {
    MACSEC_SECY_ATTR_UNSPEC,
    MACSEC_SECY_ATTR_SCI,
    MACSEC_SECY_ATTR_ENCODING_SA,
    MACSEC_SECY_ATTR_WINDOW,
    MACSEC_SECY_ATTR_CIPHER_SUITE,
    MACSEC_SECY_ATTR_ICV_LEN,
    MACSEC_SECY_ATTR_PROTECT,
    MACSEC_SECY_ATTR_REPLAY,
    MACSEC_SECY_ATTR_OPER,
    MACSEC_SECY_ATTR_VALIDATE,
    MACSEC_SECY_ATTR_ENCRYPT,
    MACSEC_SECY_ATTR_INC_SCI,
    MACSEC_SECY_ATTR_ES,
    MACSEC_SECY_ATTR_SCB,
    MACSEC_SECY_ATTR_PAD,
    __MACSEC_SECY_ATTR_END,
    NUM_MACSEC_SECY_ATTR = __MACSEC_SECY_ATTR_END,
    MACSEC_SECY_ATTR_MAX = __MACSEC_SECY_ATTR_END - 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum macsec_rxsc_attrs {
    MACSEC_RXSC_ATTR_UNSPEC,
    MACSEC_RXSC_ATTR_SCI,     /* config/dump, u64 */
    MACSEC_RXSC_ATTR_ACTIVE,  /* config/dump, u8 0..1 */
    MACSEC_RXSC_ATTR_SA_LIST, /* dump, nested */
    MACSEC_RXSC_ATTR_STATS,   /* dump, nested, macsec_rxsc_stats_attr */
    MACSEC_RXSC_ATTR_PAD,
    __MACSEC_RXSC_ATTR_END,
    NUM_MACSEC_RXSC_ATTR = __MACSEC_RXSC_ATTR_END,
    MACSEC_RXSC_ATTR_MAX = __MACSEC_RXSC_ATTR_END - 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum macsec_sa_attrs {
    MACSEC_SA_ATTR_UNSPEC,
    MACSEC_SA_ATTR_AN,     /* config/dump, u8 0..3 */
    MACSEC_SA_ATTR_ACTIVE, /* config/dump, u8 0..1 */
    MACSEC_SA_ATTR_PN,     /* config/dump, u32/u64 (u64 if XPN) */
    MACSEC_SA_ATTR_KEY,    /* config, data */
    MACSEC_SA_ATTR_KEYID,  /* config/dump, 128-bit */
    MACSEC_SA_ATTR_STATS,  /* dump, nested, macsec_sa_stats_attr */
    MACSEC_SA_ATTR_PAD,
    MACSEC_SA_ATTR_SSCI,   /* config/dump, u32 - XPN only */
    MACSEC_SA_ATTR_SALT,   /* config, 96-bit - XPN only */
    __MACSEC_SA_ATTR_END,
    NUM_MACSEC_SA_ATTR = __MACSEC_SA_ATTR_END,
    MACSEC_SA_ATTR_MAX = __MACSEC_SA_ATTR_END - 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum macsec_offload_attrs {
    MACSEC_OFFLOAD_ATTR_UNSPEC,
    MACSEC_OFFLOAD_ATTR_TYPE, /* config/dump, u8 0..2 */
    MACSEC_OFFLOAD_ATTR_PAD,
    __MACSEC_OFFLOAD_ATTR_END,
    NUM_MACSEC_OFFLOAD_ATTR = __MACSEC_OFFLOAD_ATTR_END,
    MACSEC_OFFLOAD_ATTR_MAX = __MACSEC_OFFLOAD_ATTR_END - 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum macsec_nl_commands {
    MACSEC_CMD_GET_TXSC,
    MACSEC_CMD_ADD_RXSC,
    MACSEC_CMD_DEL_RXSC,
    MACSEC_CMD_UPD_RXSC,
    MACSEC_CMD_ADD_TXSA,
    MACSEC_CMD_DEL_TXSA,
    MACSEC_CMD_UPD_TXSA,
    MACSEC_CMD_ADD_RXSA,
    MACSEC_CMD_DEL_RXSA,
    MACSEC_CMD_UPD_RXSA,
    MACSEC_CMD_UPD_OFFLOAD,
}

// u64 per-RXSC stats
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum macsec_rxsc_stats_attr {
    MACSEC_RXSC_STATS_ATTR_UNSPEC,
    MACSEC_RXSC_STATS_ATTR_IN_OCTETS_VALIDATED,
    MACSEC_RXSC_STATS_ATTR_IN_OCTETS_DECRYPTED,
    MACSEC_RXSC_STATS_ATTR_IN_PKTS_UNCHECKED,
    MACSEC_RXSC_STATS_ATTR_IN_PKTS_DELAYED,
    MACSEC_RXSC_STATS_ATTR_IN_PKTS_OK,
    MACSEC_RXSC_STATS_ATTR_IN_PKTS_INVALID,
    MACSEC_RXSC_STATS_ATTR_IN_PKTS_LATE,
    MACSEC_RXSC_STATS_ATTR_IN_PKTS_NOT_VALID,
    MACSEC_RXSC_STATS_ATTR_IN_PKTS_NOT_USING_SA,
    MACSEC_RXSC_STATS_ATTR_IN_PKTS_UNUSED_SA,
    MACSEC_RXSC_STATS_ATTR_PAD,
    __MACSEC_RXSC_STATS_ATTR_END,
    NUM_MACSEC_RXSC_STATS_ATTR = __MACSEC_RXSC_STATS_ATTR_END,
    MACSEC_RXSC_STATS_ATTR_MAX = __MACSEC_RXSC_STATS_ATTR_END - 1,
}

// u32 per-{RX,TX}SA stats
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum macsec_sa_stats_attr {
    MACSEC_SA_STATS_ATTR_UNSPEC,
    MACSEC_SA_STATS_ATTR_IN_PKTS_OK,
    MACSEC_SA_STATS_ATTR_IN_PKTS_INVALID,
    MACSEC_SA_STATS_ATTR_IN_PKTS_NOT_VALID,
    MACSEC_SA_STATS_ATTR_IN_PKTS_NOT_USING_SA,
    MACSEC_SA_STATS_ATTR_IN_PKTS_UNUSED_SA,
    MACSEC_SA_STATS_ATTR_OUT_PKTS_PROTECTED,
    MACSEC_SA_STATS_ATTR_OUT_PKTS_ENCRYPTED,
    __MACSEC_SA_STATS_ATTR_END,
    NUM_MACSEC_SA_STATS_ATTR = __MACSEC_SA_STATS_ATTR_END,
    MACSEC_SA_STATS_ATTR_MAX = __MACSEC_SA_STATS_ATTR_END - 1,
}

// u64 per-TXSC stats
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum macsec_txsc_stats_attr {
    MACSEC_TXSC_STATS_ATTR_UNSPEC,
    MACSEC_TXSC_STATS_ATTR_OUT_PKTS_PROTECTED,
    MACSEC_TXSC_STATS_ATTR_OUT_PKTS_ENCRYPTED,
    MACSEC_TXSC_STATS_ATTR_OUT_OCTETS_PROTECTED,
    MACSEC_TXSC_STATS_ATTR_OUT_OCTETS_ENCRYPTED,
    MACSEC_TXSC_STATS_ATTR_PAD,
    __MACSEC_TXSC_STATS_ATTR_END,
    NUM_MACSEC_TXSC_STATS_ATTR = __MACSEC_TXSC_STATS_ATTR_END,
    MACSEC_TXSC_STATS_ATTR_MAX = __MACSEC_TXSC_STATS_ATTR_END - 1,
}

// u64 per-SecY stats
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum macsec_secy_stats_attr {
    MACSEC_SECY_STATS_ATTR_UNSPEC,
    MACSEC_SECY_STATS_ATTR_OUT_PKTS_UNTAGGED,
    MACSEC_SECY_STATS_ATTR_IN_PKTS_UNTAGGED,
    MACSEC_SECY_STATS_ATTR_OUT_PKTS_TOO_LONG,
    MACSEC_SECY_STATS_ATTR_IN_PKTS_NO_TAG,
    MACSEC_SECY_STATS_ATTR_IN_PKTS_BAD_TAG,
    MACSEC_SECY_STATS_ATTR_IN_PKTS_UNKNOWN_SCI,
    MACSEC_SECY_STATS_ATTR_IN_PKTS_NO_SCI,
    MACSEC_SECY_STATS_ATTR_IN_PKTS_OVERRUN,
    MACSEC_SECY_STATS_ATTR_PAD,
    __MACSEC_SECY_STATS_ATTR_END,
    NUM_MACSEC_SECY_STATS_ATTR = __MACSEC_SECY_STATS_ATTR_END,
    MACSEC_SECY_STATS_ATTR_MAX = __MACSEC_SECY_STATS_ATTR_END - 1,
}
