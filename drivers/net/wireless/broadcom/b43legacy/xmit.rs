//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/broadcom/b43legacy/xmit.h
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

// Macro flag: #define B43legacy_XMIT_H_

// struct b43legacy_plcp_hdr4
// struct b43legacy_plcp_hdr6

// TX header for v3 firmware
#[repr(C)]
#[derive(Copy, Clone)]
pub struct b43legacy_txhdr_fw3 {
    pub /: *mut *mut __le32 mac_ctl; / MAC TX control,
    pub /: *mut *mut __le16 mac_frame_ctl; / Copy of the FrameControl,
    pub /: *mut *mut __le16 tx_fes_time_norm; / TX FES Time Normal,
    pub /: *mut *mut __le16 phy_ctl; / PHY TX control,
    pub /: *mut *mut __u8 iv[16]; / Encryption IV,
    pub /: *mut *mut __u8 tx_receiver[6]; / TX Frame Receiver address,
    pub /: *mut *mut __le16 tx_fes_time_fb; / TX FES Time Fallback,
    pub /: *mut *mut b43legacy_plcp_hdr4 rts_plcp_fb; / RTS fallback PLCP,
    pub /: *mut *mut __le16 rts_dur_fb; / RTS fallback duration,
    pub /: *mut *mut b43legacy_plcp_hdr4 plcp_fb; / Fallback PLCP,
    pub /: *mut *mut __le16 dur_fb; / Fallback duration,
    pub cookie: __le16,
    pub unknown_scb_stuff: __le16,
    pub /: *mut *mut b43legacy_plcp_hdr6 rts_plcp; / RTS PLCP,
    pub /: *mut *mut __u8 rts_frame[18]; / The RTS frame (if used),
    pub plcp: b43legacy_plcp_hdr6,
    pub __packed: },
// MAC TX control
pub const B43legacy_TX4_MAC_KEYIDX: c_uint = 0x0FF00000 /* Security key index */;
pub const B43legacy_TX4_MAC_KEYIDX_SHIFT: c_int = 20;
pub const B43legacy_TX4_MAC_KEYALG: c_uint = 0x00070000 /* Security key algorithm */;
pub const B43legacy_TX4_MAC_KEYALG_SHIFT: c_int = 16;
pub const B43legacy_TX4_MAC_LIFETIME: c_uint = 0x00001000;
pub const B43legacy_TX4_MAC_FRAMEBURST: c_uint = 0x00000800;
pub const B43legacy_TX4_MAC_SENDCTS: c_uint = 0x00000400;
pub const B43legacy_TX4_MAC_AMPDU: c_uint = 0x00000300;
pub const B43legacy_TX4_MAC_AMPDU_SHIFT: c_int = 8;
pub const B43legacy_TX4_MAC_CTSFALLBACKOFDM: c_uint = 0x00000200;
pub const B43legacy_TX4_MAC_FALLBACKOFDM: c_uint = 0x00000100;
pub const B43legacy_TX4_MAC_5GHZ: c_uint = 0x00000080;
pub const B43legacy_TX4_MAC_IGNPMQ: c_uint = 0x00000020;
pub const B43legacy_TX4_MAC_HWSEQ: c_uint = 0x00000010 /* Use Hardware Seq No */;
pub const B43legacy_TX4_MAC_STMSDU: c_uint = 0x00000008 /* Start MSDU */;
pub const B43legacy_TX4_MAC_SENDRTS: c_uint = 0x00000004;
pub const B43legacy_TX4_MAC_LONGFRAME: c_uint = 0x00000002;
pub const B43legacy_TX4_MAC_ACK: c_uint = 0x00000001;
// Extra Frame Types
pub const B43legacy_TX4_EFT_FBOFDM: c_uint = 0x0001 /* Data frame fb rate type */;
pub const B43legacy_TX4_EFT_RTSOFDM: c_uint = 0x0004 /* RTS/CTS rate type */;
pub const B43legacy_TX4_EFT_RTSFBOFDM: c_uint = 0x0010 /* RTS/CTS fallback rate type */;
// PHY TX control word
pub const B43legacy_TX4_PHY_ENC: c_uint = 0x0003 /* Data frame encoding */;
pub const B43legacy_TX4_PHY_ENC_CCK: c_uint = 0x0000 /* CCK */;
pub const B43legacy_TX4_PHY_ENC_OFDM: c_uint = 0x0001 /* Data frame rate type */;
pub const B43legacy_TX4_PHY_SHORTPRMBL: c_uint = 0x0010 /* Use short preamble */;
pub const B43legacy_TX4_PHY_ANT: c_uint = 0x03C0 /* Antenna selection */;
pub const B43legacy_TX4_PHY_ANT0: c_uint = 0x0000 /* Use antenna 0 */;
pub const B43legacy_TX4_PHY_ANT1: c_uint = 0x0100 /* Use antenna 1 */;
pub const B43legacy_TX4_PHY_ANTLAST: c_uint = 0x0300 /* Use last used antenna */;
    pub cookie): u16,
// Transmit Status
#[repr(C)]
#[derive(Copy, Clone)]
pub struct b43legacy_txstatus {
    pub /: *mut *mut u16 cookie; / The cookie from the txhdr,
    pub /: *mut *mut u16 seq; / Sequence number,
    pub /: *mut *mut u8 phy_stat; / PHY TX status,
    pub /: *mut *mut u8 frame_count; / Frame transmit count,
    pub /: *mut *mut u8 rts_count; / RTS transmit count,
    pub /: *mut *mut u8 supp_reason; / Suppression reason,
// flags
    pub /: *mut *mut u8 pm_indicated;/ PM mode indicated to AP,
    pub /: *mut *mut u8 intermediate;/ Intermediate status notification,
    pub /: *mut *mut u8 for_ampdu; / Status is for an AMPDU (afterburner),
    pub /: *mut *mut u8 acked; / Wireless ACK received,
}

// txstatus supp_reason values
// Transmit Status as received through DMA/PIO on old chips
#[repr(C)]
#[derive(Copy, Clone)]
pub struct b43legacy_hwtxstatus {
    pub cookie: __le16,
    pub flags: u8,
    pub count: u8,
    pub seq: __le16,
    pub phy_stat: u8,
    pub __packed: },
// Receive header for v3 firmware.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct b43legacy_rxhdr_fw3 {
    pub /: *mut *mut __le16 frame_len; / Frame length,
    pub /: *mut *mut __le16 phy_status0; / PHY RX Status 0,
    pub /: *mut *mut __u8 jssi; / PHY RX Status 1: JSSI,
    pub /: *mut *mut __u8 sig_qual; / PHY RX Status 1: Signal Quality,
    pub /: *mut *mut PAD_BYTES(2); / PHY RX Status 2,
    pub /: *mut *mut __le16 phy_status3; / PHY RX Status 3,
    pub /: *mut *mut __le16 mac_status; / MAC RX status,
    pub mac_time: __le16,
    pub channel: __le16,
    pub __packed: },
// PHY RX Status 0
pub const B43legacy_RX_PHYST0_GAINCTL: c_uint = 0x4000 /* Gain Control */;
pub const B43legacy_RX_PHYST0_PLCPHCF: c_uint = 0x0200;
pub const B43legacy_RX_PHYST0_PLCPFV: c_uint = 0x0100;
pub const B43legacy_RX_PHYST0_SHORTPRMBL: c_uint = 0x0080 /* Recvd with Short Preamble */;
pub const B43legacy_RX_PHYST0_LCRS: c_uint = 0x0040;
pub const B43legacy_RX_PHYST0_ANT: c_uint = 0x0020 /* Antenna */;
pub const B43legacy_RX_PHYST0_UNSRATE: c_uint = 0x0010;
pub const B43legacy_RX_PHYST0_CLIP: c_uint = 0x000C;
pub const B43legacy_RX_PHYST0_CLIP_SHIFT: c_int = 2;
pub const B43legacy_RX_PHYST0_FTYPE: c_uint = 0x0003 /* Frame type */;
pub const B43legacy_RX_PHYST0_CCK: c_uint = 0x0000 /* Frame type: CCK */;
pub const B43legacy_RX_PHYST0_OFDM: c_uint = 0x0001 /* Frame type: OFDM */;
pub const B43legacy_RX_PHYST0_PRE_N: c_uint = 0x0002 /* Pre-standard N-PHY frame */;
pub const B43legacy_RX_PHYST0_STD_N: c_uint = 0x0003 /* Standard N-PHY frame */;
// PHY RX Status 2
pub const B43legacy_RX_PHYST2_LNAG: c_uint = 0xC000 /* LNA Gain */;
pub const B43legacy_RX_PHYST2_LNAG_SHIFT: c_int = 14;
pub const B43legacy_RX_PHYST2_PNAG: c_uint = 0x3C00 /* PNA Gain */;
pub const B43legacy_RX_PHYST2_PNAG_SHIFT: c_int = 10;
pub const B43legacy_RX_PHYST2_FOFF: c_uint = 0x03FF /* F offset */;
// PHY RX Status 3
pub const B43legacy_RX_PHYST3_DIGG: c_uint = 0x1800 /* DIG Gain */;
pub const B43legacy_RX_PHYST3_DIGG_SHIFT: c_int = 11;
pub const B43legacy_RX_PHYST3_TRSTATE: c_uint = 0x0400 /* TR state */;
// MAC RX Status
pub const B43legacy_RX_MAC_BEACONSENT: c_uint = 0x00008000 /* Beacon send flag */;
pub const B43legacy_RX_MAC_KEYIDX: c_uint = 0x000007E0 /* Key index */;
pub const B43legacy_RX_MAC_KEYIDX_SHIFT: c_int = 5;
pub const B43legacy_RX_MAC_DECERR: c_uint = 0x00000010 /* Decrypt error */;
pub const B43legacy_RX_MAC_DEC: c_uint = 0x00000008 /* Decryption attempted */;
pub const B43legacy_RX_MAC_PADDING: c_uint = 0x00000004 /* Pad bytes present */;
pub const B43legacy_RX_MAC_RESP: c_uint = 0x00000002 /* Response frame xmitted */;
pub const B43legacy_RX_MAC_FCSERR: c_uint = 0x00000001 /* FCS error */;
// RX channel
pub const B43legacy_RX_CHAN_GAIN: c_uint = 0xFC00 /* Gain */;
pub const B43legacy_RX_CHAN_GAIN_SHIFT: c_int = 10;
pub const B43legacy_RX_CHAN_ID: c_uint = 0x03FC /* Channel ID */;
pub const B43legacy_RX_CHAN_ID_SHIFT: c_int = 2;
pub const B43legacy_RX_CHAN_PHYTYPE: c_uint = 0x0003 /* PHY type */;
    pub bitrate): u8 b43legacy_plcp_get_ratecode_cck(u8,
    pub bitrate): u8 b43legacy_plcp_get_ratecode_ofdm(u8,
    pub bitrate): u16 octets, u8,
    pub _rxhdr): *const c_void,
    pub status): *const b43legacy_txstatus,
    pub hw): *const b43legacy_hwtxstatus,
    pub dev): *mut void b43legacy_tx_suspend(struct b43legacy_wldev,
    pub dev): *mut void b43legacy_tx_resume(struct b43legacy_wldev,
pub const B43legacy_NR_QOSPARMS: c_int = 22;
}

extern "C" {
    pub fn b43legacy_qos_init(dev: *mut b43legacy_wldev);
}
// Helper functions for converting the key-table index from "firmware-format"
// to "raw-format" and back. The firmware API changed for this at some revision.
// We need to account for that here.
// FIXME: Not sure the change was at rev 351
// RX default keys or per STA keys
