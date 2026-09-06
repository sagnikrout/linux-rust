//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/realtek/rtl818x/rtl818x.h
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
// Definitions for RTL818x hardware
//
// Copyright 2007 Michael Wu <flamingice@sourmilk.net>
// Copyright 2007 Andrea Merello <andrea.merello@gmail.com>
//
// Based on the r8187 driver, which is:
// Copyright 2005 Andrea Merello <andrea.merello@gmail.com>, et al.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtl818x_csr {
    pub MAC: [u8; 6],
    pub reserved_0: [u8; 2],
    pub /: *mut *mut __le32 MAR[2]; / 0x8,
    pub /: *mut *mut u8 rf_sw_config; / 0x8,
    pub reserved_01: [u8; 3],
    pub /: *mut *mut __le32 TMGDA; / 0xc,
    pub __packed: },
    pub __packed: },
    pub RX_FIFO_COUNT: u8,
    pub reserved_1: u8,
    pub TX_FIFO_COUNT: u8,
    pub BQREQ: u8,
    pub __packed: },
    pub /: *mut *mut __le32 TBKDA; / for 8187se,
    pub __packed: },
    pub /: *mut *mut __le32 TBEDA; / 0x14 - for rtl8187se,
    pub TSFT: [__le32; 2],
    pub TLPDA: __le32,
    pub /: *mut *mut __le32 TVIDA; / for 8187se,
    pub __packed: },
    pub TNPDA: __le32,
    pub /: *mut *mut __le32 TVODA; / for 8187se,
    pub __packed: },
// hi pri ring for all cards
    pub /: *mut *mut __le32 THPDA; / 0x28,
    pub reserved_2a: u8,
    pub EIFS_8187SE: u8,
    pub __packed: },
    pub BRSR: __le16,
    pub __packed: },
    pub /: *mut *mut u8 BSSID[6]; / 0x2e,
    pub RESP_RATE: u8,
    pub EIFS: u8,
    pub __packed: },
    pub BRSR_8187SE: __le16,
    pub __packed: },
    pub /: *mut *mut u8 reserved_3[1]; / 0x36,
    pub /: *mut *mut u8 CMD; / 0x37,

    pub /: *mut *mut u8 reserved_4[4]; / 0x38,
    pub INT_MASK: __le16,
    pub INT_STATUS: __le16,
    pub __packed: },
    pub /: *mut *mut __le32 INT_STATUS_SE; / 0x3c,
    pub __packed: },
// status bits for rtl8187 and rtl8180/8185

// status bits for rtl8187se

    pub /: *mut *mut __le32 TX_CONF; / 0x40,

    pub RX_CONF: __le32,

    pub INT_TIMEOUT: __le32,
    pub TBDA: __le32,
    pub EEPROM_CMD: u8,

    pub CONFIG0: u8,
    pub CONFIG1: u8,
    pub CONFIG2: u8,

    pub ANAPARAM: __le32,
    pub MSR: u8,

    pub CONFIG3: u8,

    pub CONFIG4: u8,

    pub TESTR: u8,
    pub reserved_9: [u8; 2],
    pub PGSELECT: u8,
    pub SECURITY: u8,
    pub ANAPARAM2: __le32,
    pub reserved_10: [u8; 8],
    pub /: *mut *mut __le32 IMR; / 0x6c - Interrupt mask reg for 8187se,

    pub /: *mut *mut __le16 BEACON_INTERVAL; / 0x70,
    pub /: *mut *mut __le16 ATIM_WND; / 0x72,
    pub /: *mut *mut __le16 BEACON_INTERVAL_TIME; / 0x74,
    pub /: *mut *mut __le16 ATIMTR_INTERVAL; / 0x76,
    pub /: *mut *mut u8 PHY_DELAY; / 0x78,
    pub /: *mut *mut u8 CARRIER_SENSE_COUNTER; / 0x79,
    pub /: *mut *mut u8 reserved_11[2]; / 0x7a,
    pub /: *mut *mut u8 PHY[4]; / 0x7c,
    pub /: *mut *mut __le16 RFPinsOutput; / 0x80,
    pub /: *mut *mut __le16 RFPinsEnable; / 0x82,
    pub /: *mut *mut __le16 RFPinsSelect; / 0x84,
    pub /: *mut *mut __le16 RFPinsInput; / 0x86,
    pub /: *mut *mut __le32 RF_PARA; / 0x88,
    pub /: *mut *mut __le32 RF_TIMING; / 0x8c,
    pub /: *mut *mut u8 GP_ENABLE; / 0x90,
    pub /: *mut *mut u8 GPIO0; / 0x91,
    pub /: *mut *mut u8 GPIO1; / 0x92,
    pub /: *mut *mut u8 TPPOLL_STOP; / 0x93 - rtl8187se only,

    pub /: *mut *mut __le32 HSSI_PARA; / 0x94,
    pub /: *mut *mut u8 reserved_13[4]; / 0x98,
    pub /: *mut *mut u8 TX_AGC_CTL; / 0x9c,

    pub TX_GAIN_CCK: u8,
    pub TX_GAIN_OFDM: u8,
    pub TX_ANTENNA: u8,
    pub reserved_14: [u8; 16],
    pub WPA_CONF: u8,
    pub reserved_15: [u8; 3],
    pub SIFS: u8,
    pub DIFS: u8,
    pub SLOT: u8,
    pub reserved_16: [u8; 5],
    pub CW_CONF: u8,

    pub CW_VAL: u8,
    pub RATE_FALLBACK: u8,

    pub ACM_CONTROL: u8,
    pub reserved_17: [u8; 24],
    pub CONFIG5: u8,
    pub TX_DMA_POLLING: u8,
    pub PHY_PR: u8,
    pub reserved_18: u8,
    pub CWR: __le16,
    pub RETRY_CTR: u8,
    pub reserved_19: [u8; 3],
    pub INT_MIG: __le16,
// RTL818X_R8187B_*: magic numbers from ioregisters
pub const RTL818X_R8187B_B: c_int = 0;
pub const RTL818X_R8187B_D: c_int = 1;
pub const RTL818X_R8187B_E: c_int = 2;
    pub RDSAR: __le32,
    pub TID_AC_MAP: __le16,
    pub reserved_20: [u8; 4],
    pub /: *mut *mut __le16 ANAPARAM3; / 0xee,
    pub /: *mut *mut u8 ANAPARAM3A; / for rtl8187,
}

pub const AC_PARAM_TXOP_LIMIT_SHIFT: c_int = 16;
pub const AC_PARAM_ECW_MAX_SHIFT: c_int = 12;
pub const AC_PARAM_ECW_MIN_SHIFT: c_int = 8;
pub const AC_PARAM_AIFS_SHIFT: c_int = 0;
// These are addresses with NON-standard usage.
// They have offsets very far from this struct.
// I don't like to introduce a ton of "reserved"..
// They are for RTL8187SE
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtl818x_rf_ops {
    pub name: *mut c_char,
    pub ): *mut *mut void (init)(struct ieee80211_hw,
    pub ): *mut *mut void (stop)(struct ieee80211_hw,
    pub ): *mut *mut *mut void (set_chan)(struct ieee80211_hw , struct ieee80211_conf,
    pub sq): *mut *mut u8 (calc_rssi)(u8 agc, u8,
}

//
// enum rtl818x_tx_desc_flags - Tx/Rx flags are common between RTL818X chips
//
// @RTL818X_TX_DESC_FLAG_NO_ENC: Disable hardware based encryption.
// @RTL818X_TX_DESC_FLAG_TX_OK: TX frame was ACKed.
// @RTL818X_TX_DESC_FLAG_SPLCP: Use short preamble.
// @RTL818X_TX_DESC_FLAG_MOREFRAG: More fragments follow.
// @RTL818X_TX_DESC_FLAG_CTS: Use CTS-to-self protection.
// @RTL818X_TX_DESC_FLAG_RTS: Use RTS/CTS protection.
// @RTL818X_TX_DESC_FLAG_LS: Last segment of the frame.
// @RTL818X_TX_DESC_FLAG_FS: First segment of the frame.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtl818x_tx_desc_flags {
    RTL818X_TX_DESC_FLAG_NO_ENC	= (1 << 15),
    RTL818X_TX_DESC_FLAG_TX_OK	= (1 << 15),
    RTL818X_TX_DESC_FLAG_SPLCP	= (1 << 16),
    RTL818X_TX_DESC_FLAG_RX_UNDER	= (1 << 16),
    RTL818X_TX_DESC_FLAG_MOREFRAG	= (1 << 17),
    RTL818X_TX_DESC_FLAG_CTS	= (1 << 18),
    RTL818X_TX_DESC_FLAG_RTS	= (1 << 23),
    RTL818X_TX_DESC_FLAG_LS		= (1 << 28),
    RTL818X_TX_DESC_FLAG_FS		= (1 << 29),
    RTL818X_TX_DESC_FLAG_DMA	= (1 << 30),
    RTL818X_TX_DESC_FLAG_OWN	= (1 << 31)
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtl818x_rx_desc_flags {
    RTL818X_RX_DESC_FLAG_ICV_ERR	= (1 << 12),
    RTL818X_RX_DESC_FLAG_CRC32_ERR	= (1 << 13),
    RTL818X_RX_DESC_FLAG_PM		= (1 << 14),
    RTL818X_RX_DESC_FLAG_RX_ERR	= (1 << 15),
    RTL818X_RX_DESC_FLAG_BCAST	= (1 << 16),
    RTL818X_RX_DESC_FLAG_PAM	= (1 << 17),
    RTL818X_RX_DESC_FLAG_MCAST	= (1 << 18),
    RTL818X_RX_DESC_FLAG_QOS	= (1 << 19), /* RTL8187(B) only */
    RTL818X_RX_DESC_FLAG_TRSW	= (1 << 24), /* RTL8187(B) only */
    RTL818X_RX_DESC_FLAG_SPLCP	= (1 << 25),
    RTL818X_RX_DESC_FLAG_FOF	= (1 << 26),
    RTL818X_RX_DESC_FLAG_DMA_FAIL	= (1 << 27),
    RTL818X_RX_DESC_FLAG_LS		= (1 << 28),
    RTL818X_RX_DESC_FLAG_FS		= (1 << 29),
    RTL818X_RX_DESC_FLAG_EOR	= (1 << 30),
    RTL818X_RX_DESC_FLAG_OWN	= (1 << 31)
}
