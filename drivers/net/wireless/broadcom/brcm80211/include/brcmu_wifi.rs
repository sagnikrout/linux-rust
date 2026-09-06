//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/broadcom/brcm80211/include/brcmu_wifi.h
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


// SPDX-License-Identifier: ISC
//
// Copyright (c) 2010 Broadcom Corporation
//

//
// A chanspec (u16) holds the channel number, band, bandwidth and control
// sideband
//
// channel defines
pub const CH_UPPER_SB: c_uint = 0x01;
pub const CH_LOWER_SB: c_uint = 0x02;
pub const CH_EWA_VALID: c_uint = 0x04;
pub const CH_70MHZ_APART: c_int = 14;
pub const CH_50MHZ_APART: c_int = 10;
pub const CH_30MHZ_APART: c_int = 6;
pub const CH_20MHZ_APART: c_int = 4;
pub const CH_10MHZ_APART: c_int = 2;

pub const CH_MIN_2G_CHANNEL: c_int = 1;

pub const CH_MIN_5G_CHANNEL: c_int = 34;
// bandstate array indices

//
// max # supported channels. The max channel no is 216, this is that + 1
// rounded up to a multiple of NBBY (8). DO NOT MAKE it > 255: channels are
// u8's all over
//
pub const MAXCHANNEL: c_int = 224;
pub const WL_CHANSPEC_CHAN_MASK: c_uint = 0x00ff;
pub const WL_CHANSPEC_CHAN_SHIFT: c_int = 0;
pub const WL_CHANSPEC_CTL_SB_MASK: c_uint = 0x0300;
pub const WL_CHANSPEC_CTL_SB_SHIFT: c_int = 8;
pub const WL_CHANSPEC_CTL_SB_LOWER: c_uint = 0x0100;
pub const WL_CHANSPEC_CTL_SB_UPPER: c_uint = 0x0200;
pub const WL_CHANSPEC_CTL_SB_NONE: c_uint = 0x0300;
pub const WL_CHANSPEC_BW_MASK: c_uint = 0x0C00;
pub const WL_CHANSPEC_BW_SHIFT: c_int = 10;
pub const WL_CHANSPEC_BW_10: c_uint = 0x0400;
pub const WL_CHANSPEC_BW_20: c_uint = 0x0800;
pub const WL_CHANSPEC_BW_40: c_uint = 0x0C00;
pub const WL_CHANSPEC_BW_80: c_uint = 0x2000;
pub const WL_CHANSPEC_BAND_MASK: c_uint = 0xf000;
pub const WL_CHANSPEC_BAND_SHIFT: c_int = 12;
pub const WL_CHANSPEC_BAND_5G: c_uint = 0x1000;
pub const WL_CHANSPEC_BAND_2G: c_uint = 0x2000;
pub const INVCHANSPEC: c_int = 255;

// values for band specific 40MHz capabilities
pub const WLC_N_BW_20ALL: c_int = 0;
pub const WLC_N_BW_40ALL: c_int = 1;
pub const WLC_N_BW_20IN2G_40IN5G: c_int = 2;

// Bandwidth capabilities

pub const WLC_BW_CAP_UNRESTRICTED: c_uint = 0xFF;
// band types

pub const CHANSPEC_STR_LEN: c_int = 8;
// defined rate in 500kbps

pub const MCSSET_LEN: c_int = 16;
// Enumerate crypto algorithms
pub const CRYPTO_ALGO_OFF: c_int = 0;
pub const CRYPTO_ALGO_WEP1: c_int = 1;
pub const CRYPTO_ALGO_TKIP: c_int = 2;
pub const CRYPTO_ALGO_WEP128: c_int = 3;
pub const CRYPTO_ALGO_AES_CCM: c_int = 4;
pub const CRYPTO_ALGO_AES_RESERVED1: c_int = 5;
pub const CRYPTO_ALGO_AES_RESERVED2: c_int = 6;
pub const CRYPTO_ALGO_NALG: c_int = 7;
// wireless security bitvec
pub const WEP_ENABLED: c_uint = 0x0001;
pub const TKIP_ENABLED: c_uint = 0x0002;
pub const AES_ENABLED: c_uint = 0x0004;
pub const WSEC_SWFLAG: c_uint = 0x0008;
// to go into transition mode without setting wep
pub const SES_OW_ENABLED: c_uint = 0x0040;
// MFP
pub const MFP_CAPABLE: c_uint = 0x0200;
pub const MFP_REQUIRED: c_uint = 0x0400;
// WPA authentication mode bitvec
pub const WPA_AUTH_DISABLED: c_uint = 0x0000	/* Legacy (i.e., non-WPA) */;
pub const WPA_AUTH_NONE: c_uint = 0x0001	/* none (IBSS) */;
pub const WPA_AUTH_UNSPECIFIED: c_uint = 0x0002	/* over 802.1x */;
pub const WPA_AUTH_PSK: c_uint = 0x0004	/* Pre-shared key */;
pub const WPA_AUTH_RESERVED1: c_uint = 0x0008;
pub const WPA_AUTH_RESERVED2: c_uint = 0x0010;
pub const WPA2_AUTH_RESERVED1: c_uint = 0x0020;
pub const WPA2_AUTH_UNSPECIFIED: c_uint = 0x0040	/* over 802.1x */;
pub const WPA2_AUTH_PSK: c_uint = 0x0080	/* Pre-shared key */;
pub const WPA2_AUTH_RESERVED3: c_uint = 0x0200;
pub const WPA2_AUTH_RESERVED4: c_uint = 0x0400;
pub const WPA2_AUTH_RESERVED5: c_uint = 0x0800;
pub const WPA2_AUTH_1X_SHA256: c_uint = 0x1000  /* 1X with SHA256 key derivation */;
pub const WPA2_AUTH_FT: c_uint = 0x4000	/* Fast BSS Transition */;
pub const WPA2_AUTH_PSK_SHA256: c_uint = 0x8000	/* PSK with SHA256 key derivation */;
pub const WPA3_AUTH_SAE_PSK: c_uint = 0x40000	/* SAE with 4-way handshake */;
pub const WFA_AUTH_DPP: c_uint = 0x200000 /* WFA DPP AUTH */;
pub const DOT11_DEFAULT_RTS_LEN: c_int = 2347;
pub const DOT11_DEFAULT_FRAG_LEN: c_int = 2346;
pub const DOT11_ICV_AES_LEN: c_int = 8;
pub const DOT11_QOS_LEN: c_int = 2;
pub const DOT11_IV_MAX_LEN: c_int = 8;
pub const DOT11_A4_HDR_LEN: c_int = 30;
pub const HT_CAP_RX_STBC_NO: c_uint = 0x0;
pub const HT_CAP_RX_STBC_ONE_STREAM: c_uint = 0x1;
