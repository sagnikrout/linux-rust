//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/broadcom/b43/xmit.h
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

// struct b43_plcp_hdr4
// struct b43_plcp_hdr6

// TX header for v4 firmware
#[repr(C)]
#[derive(Copy, Clone)]
pub struct b43_txhdr {
    pub /: *mut *mut __le32 mac_ctl; / MAC TX control,
    pub /: *mut *mut __le16 mac_frame_ctl; / Copy of the FrameControl field,
    pub /: *mut *mut __le16 tx_fes_time_norm; / TX FES Time Normal,
    pub /: *mut *mut __le16 phy_ctl; / PHY TX control,
    pub /: *mut *mut __le16 phy_ctl1; / PHY TX control word 1,
    pub /: *mut *mut __le16 phy_ctl1_fb; / PHY TX control word 1 for fallback rates,
    pub /: *mut *mut __le16 phy_ctl1_rts; / PHY TX control word 1 RTS,
    pub /: *mut *mut __le16 phy_ctl1_rts_fb; / PHY TX control word 1 RTS for fallback rates,
    pub /: *mut *mut __u8 phy_rate; / PHY rate,
    pub /: *mut *mut __u8 phy_rate_rts; / PHY rate for RTS/CTS,
    pub /: *mut *mut __u8 extra_ft; / Extra Frame Types,
    pub /: *mut *mut __u8 chan_radio_code; / Channel Radio Code,
    pub /: *mut *mut __u8 iv[16]; / Encryption IV,
    pub /: *mut *mut __u8 tx_receiver[6]; / TX Frame Receiver address,
    pub /: *mut *mut __le16 tx_fes_time_fb; / TX FES Time Fallback,
    pub /: *mut *mut b43_plcp_hdr6 rts_plcp_fb; / RTS fallback PLCP header,
    pub /: *mut *mut __le16 rts_dur_fb; / RTS fallback duration,
    pub /: *mut *mut b43_plcp_hdr6 plcp_fb; / Fallback PLCP header,
    pub /: *mut *mut __le16 dur_fb; / Fallback duration,
    pub /: *mut *mut __le16 mimo_modelen; / MIMO mode length,
    pub /: *mut *mut __le16 mimo_ratelen_fb; / MIMO fallback rate length,
    pub /: *mut *mut __le32 timeout; / Timeout,
// Tested with 598.314, 644.1001 and 666.2
    pub /: *mut *mut __le16 mimo_antenna; / MIMO antenna select,
    pub /: *mut *mut __le16 preload_size; / Preload size,
    pub /: *mut *mut __le16 cookie; / TX frame cookie,
    pub /: *mut *mut __le16 tx_status; / TX status,
    pub max_n_mpdus: __le16,
    pub max_a_bytes_mrt: __le16,
    pub max_a_bytes_fbr: __le16,
    pub min_m_bytes: __le16,
    pub /: *mut *mut b43_plcp_hdr6 rts_plcp; / RTS PLCP header,
    pub /: *mut *mut __u8 rts_frame[16]; / The RTS frame (if used),
    pub /: *mut *mut b43_plcp_hdr6 plcp; / Main PLCP header,
    pub __packed: } format_598,
// Tested with 410.2160, 478.104 and 508.*
    pub /: *mut *mut __le16 mimo_antenna; / MIMO antenna select,
    pub /: *mut *mut __le16 preload_size; / Preload size,
    pub /: *mut *mut __le16 cookie; / TX frame cookie,
    pub /: *mut *mut __le16 tx_status; / TX status,
    pub /: *mut *mut b43_plcp_hdr6 rts_plcp; / RTS PLCP header,
    pub /: *mut *mut __u8 rts_frame[16]; / The RTS frame (if used),
    pub /: *mut *mut b43_plcp_hdr6 plcp; / Main PLCP header,
    pub __packed: } format_410,
// Tested with 351.126
    pub /: *mut *mut __le16 cookie; / TX frame cookie,
    pub /: *mut *mut __le16 tx_status; / TX status,
    pub /: *mut *mut b43_plcp_hdr6 rts_plcp; / RTS PLCP header,
    pub /: *mut *mut __u8 rts_frame[16]; / The RTS frame (if used),
    pub /: *mut *mut b43_plcp_hdr6 plcp; / Main PLCP header,
    pub __packed: } format_351,
    pub __packed: },
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct b43_tx_legacy_rate_phy_ctl_entry {
    pub bitrate: u8,
    pub coding_rate: u16,
    pub modulation: u16,
}

// MAC TX control
pub const B43_TXH_MAC_RTS_FB_SHORTPRMBL: c_uint = 0x80000000 /* RTS fallback preamble */;
pub const B43_TXH_MAC_RTS_SHORTPRMBL: c_uint = 0x40000000 /* RTS main rate preamble */;
pub const B43_TXH_MAC_FB_SHORTPRMBL: c_uint = 0x20000000 /* Main fallback preamble */;
pub const B43_TXH_MAC_USEFBR: c_uint = 0x10000000 /* Use fallback rate for this AMPDU */;
pub const B43_TXH_MAC_KEYIDX: c_uint = 0x0FF00000 /* Security key index */;
pub const B43_TXH_MAC_KEYIDX_SHIFT: c_int = 20;
pub const B43_TXH_MAC_ALT_TXPWR: c_uint = 0x00080000 /* Use alternate txpwr defined at loc. M_ALT_TXPWR_IDX */;
pub const B43_TXH_MAC_KEYALG: c_uint = 0x00070000 /* Security key algorithm */;
pub const B43_TXH_MAC_KEYALG_SHIFT: c_int = 16;
pub const B43_TXH_MAC_AMIC: c_uint = 0x00008000 /* AMIC */;
pub const B43_TXH_MAC_RIFS: c_uint = 0x00004000 /* Use RIFS */;
pub const B43_TXH_MAC_LIFETIME: c_uint = 0x00002000 /* Lifetime */;
pub const B43_TXH_MAC_FRAMEBURST: c_uint = 0x00001000 /* Frameburst */;
pub const B43_TXH_MAC_SENDCTS: c_uint = 0x00000800 /* Send CTS-to-self */;
pub const B43_TXH_MAC_AMPDU: c_uint = 0x00000600 /* AMPDU status */;
pub const B43_TXH_MAC_AMPDU_MPDU: c_uint = 0x00000000 /* Regular MPDU, not an AMPDU */;
pub const B43_TXH_MAC_AMPDU_FIRST: c_uint = 0x00000200 /* First MPDU or AMPDU */;
pub const B43_TXH_MAC_AMPDU_INTER: c_uint = 0x00000400 /* Intermediate MPDU or AMPDU */;
pub const B43_TXH_MAC_AMPDU_LAST: c_uint = 0x00000600 /* Last (or only) MPDU of AMPDU */;
pub const B43_TXH_MAC_40MHZ: c_uint = 0x00000100 /* Use 40 MHz bandwidth */;
pub const B43_TXH_MAC_5GHZ: c_uint = 0x00000080 /* 5GHz band */;
pub const B43_TXH_MAC_DFCS: c_uint = 0x00000040 /* DFCS */;
pub const B43_TXH_MAC_IGNPMQ: c_uint = 0x00000020 /* Ignore PMQ */;
pub const B43_TXH_MAC_HWSEQ: c_uint = 0x00000010 /* Use Hardware Sequence Number */;
pub const B43_TXH_MAC_STMSDU: c_uint = 0x00000008 /* Start MSDU */;
pub const B43_TXH_MAC_SENDRTS: c_uint = 0x00000004 /* Send RTS */;
pub const B43_TXH_MAC_LONGFRAME: c_uint = 0x00000002 /* Long frame */;
pub const B43_TXH_MAC_ACK: c_uint = 0x00000001 /* Immediate ACK */;
// Extra Frame Types
pub const B43_TXH_EFT_FB: c_uint = 0x03 /* Data frame fallback encoding */;
pub const B43_TXH_EFT_FB_CCK: c_uint = 0x00 /* CCK */;
pub const B43_TXH_EFT_FB_OFDM: c_uint = 0x01 /* OFDM */;
pub const B43_TXH_EFT_FB_HT: c_uint = 0x02 /* HT */;
pub const B43_TXH_EFT_FB_VHT: c_uint = 0x03 /* VHT */;
pub const B43_TXH_EFT_RTS: c_uint = 0x0C /* RTS/CTS encoding */;
pub const B43_TXH_EFT_RTS_CCK: c_uint = 0x00 /* CCK */;
pub const B43_TXH_EFT_RTS_OFDM: c_uint = 0x04 /* OFDM */;
pub const B43_TXH_EFT_RTS_HT: c_uint = 0x08 /* HT */;
pub const B43_TXH_EFT_RTS_VHT: c_uint = 0x0C /* VHT */;
pub const B43_TXH_EFT_RTSFB: c_uint = 0x30 /* RTS/CTS fallback encoding */;
pub const B43_TXH_EFT_RTSFB_CCK: c_uint = 0x00 /* CCK */;
pub const B43_TXH_EFT_RTSFB_OFDM: c_uint = 0x10 /* OFDM */;
pub const B43_TXH_EFT_RTSFB_HT: c_uint = 0x20 /* HT */;
pub const B43_TXH_EFT_RTSFB_VHT: c_uint = 0x30 /* VHT */;
// PHY TX control word
pub const B43_TXH_PHY_ENC: c_uint = 0x0003 /* Data frame encoding */;
pub const B43_TXH_PHY_ENC_CCK: c_uint = 0x0000 /* CCK */;
pub const B43_TXH_PHY_ENC_OFDM: c_uint = 0x0001 /* OFDM */;
pub const B43_TXH_PHY_ENC_HT: c_uint = 0x0002 /* HT */;
pub const B43_TXH_PHY_ENC_VHT: c_uint = 0x0003 /* VHT */;
pub const B43_TXH_PHY_SHORTPRMBL: c_uint = 0x0010 /* Use short preamble */;
pub const B43_TXH_PHY_ANT: c_uint = 0x03C0 /* Antenna selection */;
pub const B43_TXH_PHY_ANT0: c_uint = 0x0000 /* Use antenna 0 */;
pub const B43_TXH_PHY_ANT1: c_uint = 0x0040 /* Use antenna 1 */;
pub const B43_TXH_PHY_ANT01AUTO: c_uint = 0x00C0 /* Use antenna 0/1 auto */;
pub const B43_TXH_PHY_ANT2: c_uint = 0x0100 /* Use antenna 2 */;
pub const B43_TXH_PHY_ANT3: c_uint = 0x0200 /* Use antenna 3 */;
pub const B43_TXH_PHY_TXPWR: c_uint = 0xFC00 /* TX power */;
pub const B43_TXH_PHY_TXPWR_SHIFT: c_int = 10;
// PHY TX control word 1
pub const B43_TXH_PHY1_BW: c_uint = 0x0007 /* Bandwidth */;
pub const B43_TXH_PHY1_BW_10: c_uint = 0x0000 /* 10 MHz */;
pub const B43_TXH_PHY1_BW_10U: c_uint = 0x0001 /* 10 MHz upper */;
pub const B43_TXH_PHY1_BW_20: c_uint = 0x0002 /* 20 MHz */;
pub const B43_TXH_PHY1_BW_20U: c_uint = 0x0003 /* 20 MHz upper */;
pub const B43_TXH_PHY1_BW_40: c_uint = 0x0004 /* 40 MHz */;
pub const B43_TXH_PHY1_BW_40DUP: c_uint = 0x0005 /* 40 MHz duplicate */;
pub const B43_TXH_PHY1_MODE: c_uint = 0x0038 /* Mode */;
pub const B43_TXH_PHY1_MODE_SISO: c_uint = 0x0000 /* SISO */;
pub const B43_TXH_PHY1_MODE_CDD: c_uint = 0x0008 /* CDD */;
pub const B43_TXH_PHY1_MODE_STBC: c_uint = 0x0010 /* STBC */;
pub const B43_TXH_PHY1_MODE_SDM: c_uint = 0x0018 /* SDM */;
pub const B43_TXH_PHY1_CRATE: c_uint = 0x0700 /* Coding rate */;
pub const B43_TXH_PHY1_CRATE_1_2: c_uint = 0x0000 /* 1/2 */;
pub const B43_TXH_PHY1_CRATE_2_3: c_uint = 0x0100 /* 2/3 */;
pub const B43_TXH_PHY1_CRATE_3_4: c_uint = 0x0200 /* 3/4 */;
pub const B43_TXH_PHY1_CRATE_4_5: c_uint = 0x0300 /* 4/5 */;
pub const B43_TXH_PHY1_CRATE_5_6: c_uint = 0x0400 /* 5/6 */;
pub const B43_TXH_PHY1_CRATE_7_8: c_uint = 0x0600 /* 7/8 */;
pub const B43_TXH_PHY1_MODUL: c_uint = 0x3800 /* Modulation scheme */;
pub const B43_TXH_PHY1_MODUL_BPSK: c_uint = 0x0000 /* BPSK */;
pub const B43_TXH_PHY1_MODUL_QPSK: c_uint = 0x0800 /* QPSK */;
pub const B43_TXH_PHY1_MODUL_QAM16: c_uint = 0x1000 /* QAM16 */;
pub const B43_TXH_PHY1_MODUL_QAM64: c_uint = 0x1800 /* QAM64 */;
pub const B43_TXH_PHY1_MODUL_QAM256: c_uint = 0x2000 /* QAM256 */;
// Transmit Status
#[repr(C)]
#[derive(Copy, Clone)]
pub struct b43_txstatus {
    pub /: *mut *mut u16 cookie; / The cookie from the txhdr,
    pub /: *mut *mut u16 seq; / Sequence number,
    pub /: *mut *mut u8 phy_stat; / PHY TX status,
    pub /: *mut *mut u8 frame_count; / Frame transmit count,
    pub /: *mut *mut u8 rts_count; / RTS transmit count,
    pub /: *mut *mut u8 supp_reason; / Suppression reason,
// flags
    pub /: *mut *mut u8 pm_indicated; / PM mode indicated to AP,
    pub /: *mut *mut u8 intermediate; / Intermediate status notification (not final),
    pub /: *mut *mut u8 for_ampdu; / Status is for an AMPDU (afterburner),
    pub /: *mut *mut u8 acked; / Wireless ACK received,
}

// txstatus supp_reason values
// Receive header for v4 firmware.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct b43_rxhdr_fw4 {
    pub /: *mut *mut __le16 frame_len; / Frame length,
    pub /: *mut *mut __le16 phy_status0; / PHY RX Status 0,
// RSSI for A/B/G-PHYs
    pub /: *mut *mut __u8 jssi; / PHY RX Status 1: JSSI,
    pub /: *mut *mut __u8 sig_qual; / PHY RX Status 1: Signal Quality,
    pub __packed: },
// RSSI for N-PHYs
    pub /: *mut *mut __s8 power0; / PHY RX Status 1: Power 0,
    pub /: *mut *mut __s8 power1; / PHY RX Status 1: Power 1,
    pub __packed: },
    pub __packed: },
// HT-PHY
    pub phy_ht_power0: __s8,
    pub __packed: },
// RSSI for N-PHYs
    pub power2: __s8,
    pub __packed: },
    pub /: *mut *mut __le16 phy_status2; / PHY RX Status 2,
    pub __packed: },
// HT-PHY
    pub phy_ht_power1: __s8,
    pub phy_ht_power2: __s8,
    pub __packed: },
    pub /: *mut *mut __le16 phy_status3; / PHY RX Status 3,
    pub __packed: },
// Tested with 598.314, 644.1001 and 666.2
    pub /: *mut *mut __le16 phy_status4; / PHY RX Status 4,
    pub /: *mut *mut __le16 phy_status5; / PHY RX Status 5,
    pub /: *mut *mut __le32 mac_status; / MAC RX status,
    pub mac_time: __le16,
    pub channel: __le16,
    pub __packed: } format_598,
// Tested with 351.126, 410.2160, 478.104 and 508.*
    pub /: *mut *mut __le32 mac_status; / MAC RX status,
    pub mac_time: __le16,
    pub channel: __le16,
    pub __packed: } format_351,
    pub __packed: },
    pub __packed: },
// PHY RX Status 0
pub const B43_RX_PHYST0_GAINCTL: c_uint = 0x4000 /* Gain Control */;
pub const B43_RX_PHYST0_PLCPHCF: c_uint = 0x0200;
pub const B43_RX_PHYST0_PLCPFV: c_uint = 0x0100;
pub const B43_RX_PHYST0_SHORTPRMBL: c_uint = 0x0080 /* Received with Short Preamble */;
pub const B43_RX_PHYST0_LCRS: c_uint = 0x0040;
pub const B43_RX_PHYST0_ANT: c_uint = 0x0020 /* Antenna */;
pub const B43_RX_PHYST0_UNSRATE: c_uint = 0x0010;
pub const B43_RX_PHYST0_CLIP: c_uint = 0x000C;
pub const B43_RX_PHYST0_CLIP_SHIFT: c_int = 2;
pub const B43_RX_PHYST0_FTYPE: c_uint = 0x0003 /* Frame type */;
pub const B43_RX_PHYST0_CCK: c_uint = 0x0000 /* Frame type: CCK */;
pub const B43_RX_PHYST0_OFDM: c_uint = 0x0001 /* Frame type: OFDM */;
pub const B43_RX_PHYST0_PRE_N: c_uint = 0x0002 /* Pre-standard N-PHY frame */;
pub const B43_RX_PHYST0_STD_N: c_uint = 0x0003 /* Standard N-PHY frame */;
// PHY RX Status 2
pub const B43_RX_PHYST2_LNAG: c_uint = 0xC000 /* LNA Gain */;
pub const B43_RX_PHYST2_LNAG_SHIFT: c_int = 14;
pub const B43_RX_PHYST2_PNAG: c_uint = 0x3C00 /* PNA Gain */;
pub const B43_RX_PHYST2_PNAG_SHIFT: c_int = 10;
pub const B43_RX_PHYST2_FOFF: c_uint = 0x03FF /* F offset */;
// PHY RX Status 3
pub const B43_RX_PHYST3_DIGG: c_uint = 0x1800 /* DIG Gain */;
pub const B43_RX_PHYST3_DIGG_SHIFT: c_int = 11;
pub const B43_RX_PHYST3_TRSTATE: c_uint = 0x0400 /* TR state */;
// MAC RX Status
pub const B43_RX_MAC_RXST_VALID: c_uint = 0x01000000 /* PHY RXST valid */;
pub const B43_RX_MAC_TKIP_MICERR: c_uint = 0x00100000 /* TKIP MIC error */;
pub const B43_RX_MAC_TKIP_MICATT: c_uint = 0x00080000 /* TKIP MIC attempted */;
pub const B43_RX_MAC_AGGTYPE: c_uint = 0x00060000 /* Aggregation type */;
pub const B43_RX_MAC_AGGTYPE_SHIFT: c_int = 17;
pub const B43_RX_MAC_AMSDU: c_uint = 0x00010000 /* A-MSDU mask */;
pub const B43_RX_MAC_BEACONSENT: c_uint = 0x00008000 /* Beacon sent flag */;
pub const B43_RX_MAC_KEYIDX: c_uint = 0x000007E0 /* Key index */;
pub const B43_RX_MAC_KEYIDX_SHIFT: c_int = 5;
pub const B43_RX_MAC_DECERR: c_uint = 0x00000010 /* Decrypt error */;
pub const B43_RX_MAC_DEC: c_uint = 0x00000008 /* Decryption attempted */;
pub const B43_RX_MAC_PADDING: c_uint = 0x00000004 /* Pad bytes present */;
pub const B43_RX_MAC_RESP: c_uint = 0x00000002 /* Response frame transmitted */;
pub const B43_RX_MAC_FCSERR: c_uint = 0x00000001 /* FCS error */;
// RX channel
pub const B43_RX_CHAN_40MHZ: c_uint = 0x1000 /* 40 Mhz channel width */;
pub const B43_RX_CHAN_5GHZ: c_uint = 0x0800 /* 5 Ghz band */;
pub const B43_RX_CHAN_ID: c_uint = 0x07F8 /* Channel ID */;
pub const B43_RX_CHAN_ID_SHIFT: c_int = 3;
pub const B43_RX_CHAN_PHYTYPE: c_uint = 0x0007 /* PHY type */;
    pub bitrate): u8 b43_plcp_get_ratecode_cck(u8,
    pub bitrate): u8 b43_plcp_get_ratecode_ofdm(u8,
    pub bitrate): u16 octets, u8,
    pub _rxhdr): *const *const *const void b43_rx(struct b43_wldev dev, struct sk_buff skb, void,
    pub status): *const b43_txstatus,
    pub status): *const b43_txstatus,
    pub dev): *mut void b43_tx_suspend(struct b43_wldev,
    pub dev): *mut void b43_tx_resume(struct b43_wldev,
// Helper functions for converting the key-table index from "firmware-format"
// to "raw-format" and back. The firmware API changed for this at some revision.
// We need to account for that here.
// FIXME: Not sure the change was at rev 351
    pub 351): return (dev->fw.rev >=,
    pub firmware_kidx: u8,
    pub raw_kidx: firmware_kidx =,
    pub 4: firmware_kidx = raw_kidx -,
    pub /: *mut *mut firmware_kidx = raw_kidx; / TX default key,
    pub firmware_kidx: return,
    pub raw_kidx: u8,
    pub firmware_kidx: raw_kidx =,
    pub /: *mut *mut raw_kidx = firmware_kidx + 4; / RX default keys or per STA keys,
    pub raw_kidx: return,
// struct b43_private_tx_info - TX info private to b43.
// The structure is placed in (struct ieee80211_tx_info *)->rate_driver_data
//
// @bouncebuffer: DMA Bouncebuffer (if used)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct b43_private_tx_info {
    pub bouncebuffer: *mut c_void,
}
