//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/ath9k/mac.h
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
// Copyright (c) 2008-2011 Atheros Communications Inc.
//
// Permission to use, copy, modify, and/or distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
// ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
// ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
// OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
//

pub const CCK_SIFS_TIME: c_int = 10;
pub const CCK_PREAMBLE_BITS: c_int = 144;
pub const CCK_PLCP_BITS: c_int = 48;
pub const OFDM_SIFS_TIME: c_int = 16;
pub const OFDM_PREAMBLE_TIME: c_int = 20;
pub const OFDM_PLCP_BITS: c_int = 22;
pub const OFDM_SYMBOL_TIME: c_int = 4;
pub const OFDM_SIFS_TIME_HALF: c_int = 32;
pub const OFDM_PREAMBLE_TIME_HALF: c_int = 40;
pub const OFDM_PLCP_BITS_HALF: c_int = 22;
pub const OFDM_SYMBOL_TIME_HALF: c_int = 8;
pub const OFDM_SIFS_TIME_QUARTER: c_int = 64;
pub const OFDM_PREAMBLE_TIME_QUARTER: c_int = 80;
pub const OFDM_PLCP_BITS_QUARTER: c_int = 22;
pub const OFDM_SYMBOL_TIME_QUARTER: c_int = 16;
pub const INIT_AIFS: c_int = 2;
pub const INIT_CWMIN: c_int = 15;
pub const INIT_CWMIN_11B: c_int = 31;
pub const INIT_CWMAX: c_int = 1023;
pub const INIT_SH_RETRY: c_int = 10;
pub const INIT_LG_RETRY: c_int = 10;
pub const INIT_SSH_RETRY: c_int = 32;
pub const INIT_SLG_RETRY: c_int = 32;
pub const ATH9K_TXERR_XRETRY: c_uint = 0x01;
pub const ATH9K_TXERR_FILT: c_uint = 0x02;
pub const ATH9K_TXERR_FIFO: c_uint = 0x04;
pub const ATH9K_TXERR_XTXOP: c_uint = 0x08;
pub const ATH9K_TXERR_TIMER_EXPIRED: c_uint = 0x10;
pub const ATH9K_TX_ACKED: c_uint = 0x20;
pub const ATH9K_TX_FLUSH: c_uint = 0x40;

pub const ATH9K_TX_BA: c_uint = 0x01;
pub const ATH9K_TX_PWRMGMT: c_uint = 0x02;
pub const ATH9K_TX_DESC_CFG_ERR: c_uint = 0x04;
pub const ATH9K_TX_DATA_UNDERRUN: c_uint = 0x08;
pub const ATH9K_TX_DELIM_UNDERRUN: c_uint = 0x10;
pub const ATH9K_TX_SW_FILTERED: c_uint = 0x80;
// 64 bytes
pub const MIN_TX_FIFO_THRESHOLD: c_uint = 0x1;
//
// Single stream device AR9285 and AR9271 require 2 KB
// to work around a hardware issue, all other devices
// have can use the max 4 KB limit.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath_tx_status {
    pub ts_tstamp: u32,
    pub ts_seqnum: u16,
    pub ts_status: u8,
    pub ts_rateindex: u8,
    pub ts_rssi: i8,
    pub ts_shortretry: u8,
    pub ts_longretry: u8,
    pub ts_virtcol: u8,
    pub ts_flags: u8,
    pub ts_rssi_ctl0: i8,
    pub ts_rssi_ctl1: i8,
    pub ts_rssi_ctl2: i8,
    pub ts_rssi_ext0: i8,
    pub ts_rssi_ext1: i8,
    pub ts_rssi_ext2: i8,
    pub qid: u8,
    pub desc_id: u16,
    pub tid: u8,
    pub ba_low: u32,
    pub ba_high: u32,
    pub evm0: u32,
    pub evm1: u32,
    pub evm2: u32,
    pub duration: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath_rx_status {
    pub rs_tstamp: u32,
    pub rs_datalen: u16,
    pub rs_status: u8,
    pub rs_phyerr: u8,
    pub rs_rssi: i8,
    pub rs_keyix: u8,
    pub rs_rate: u8,
    pub rs_antenna: u8,
    pub rs_more: u8,
    pub rs_rssi_ctl: [i8; 3],
    pub rs_rssi_ext: [i8; 3],
    pub rs_isaggr: u8,
    pub rs_firstaggr: u8,
    pub rs_moreaggr: u8,
    pub rs_num_delims: u8,
    pub rs_flags: u8,
    pub is_mybeacon: bool,
    pub evm0: u32,
    pub evm1: u32,
    pub evm2: u32,
    pub evm3: u32,
    pub evm4: u32,
    pub enc_flags: u16,
    pub bw: rate_info_bw,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath_htc_rx_status {
    pub rs_tstamp: __be64,
    pub rs_datalen: __be16,
    pub rs_status: u8,
    pub rs_phyerr: u8,
    pub rs_rssi: i8,
    pub rs_rssi_ctl: [i8; 3],
    pub rs_rssi_ext: [i8; 3],
    pub rs_keyix: u8,
    pub rs_rate: u8,
    pub rs_antenna: u8,
    pub rs_more: u8,
    pub rs_isaggr: u8,
    pub rs_moreaggr: u8,
    pub rs_num_delims: u8,
    pub rs_flags: u8,
    pub rs_dummy: u8,
// FIXME: evm* never used?
    pub evm0: __be32,
    pub evm1: __be32,
    pub evm2: __be32,
}

pub const ATH9K_RXERR_CRC: c_uint = 0x01;
pub const ATH9K_RXERR_PHY: c_uint = 0x02;
pub const ATH9K_RXERR_FIFO: c_uint = 0x04;
pub const ATH9K_RXERR_DECRYPT: c_uint = 0x08;
pub const ATH9K_RXERR_MIC: c_uint = 0x10;
pub const ATH9K_RXERR_KEYMISS: c_uint = 0x20;
pub const ATH9K_RXERR_CORRUPT_DESC: c_uint = 0x40;
pub const ATH9K_RX_MORE: c_uint = 0x01;
pub const ATH9K_RX_MORE_AGGR: c_uint = 0x02;
pub const ATH9K_RX_GI: c_uint = 0x04;
pub const ATH9K_RX_2040: c_uint = 0x08;
pub const ATH9K_RX_DELIM_CRC_PRE: c_uint = 0x10;
pub const ATH9K_RX_DELIM_CRC_POST: c_uint = 0x20;
pub const ATH9K_RX_DECRYPT_BUSY: c_uint = 0x40;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath9k_phyerr {
    ATH9K_PHYERR_UNDERRUN             = 0,  /* Transmit underrun */
    ATH9K_PHYERR_TIMING               = 1,  /* Timing error */
    ATH9K_PHYERR_PARITY               = 2,  /* Illegal parity */
    ATH9K_PHYERR_RATE                 = 3,  /* Illegal rate */
    ATH9K_PHYERR_LENGTH               = 4,  /* Illegal length */
    ATH9K_PHYERR_RADAR                = 5,  /* Radar detect */
    ATH9K_PHYERR_SERVICE              = 6,  /* Illegal service */
    ATH9K_PHYERR_TOR                  = 7,  /* Transmit override receive */

    ATH9K_PHYERR_OFDM_TIMING          = 17,
    ATH9K_PHYERR_OFDM_SIGNAL_PARITY   = 18,
    ATH9K_PHYERR_OFDM_RATE_ILLEGAL    = 19,
    ATH9K_PHYERR_OFDM_LENGTH_ILLEGAL  = 20,
    ATH9K_PHYERR_OFDM_POWER_DROP      = 21,
    ATH9K_PHYERR_OFDM_SERVICE         = 22,
    ATH9K_PHYERR_OFDM_RESTART         = 23,

    ATH9K_PHYERR_CCK_BLOCKER          = 24,
    ATH9K_PHYERR_CCK_TIMING           = 25,
    ATH9K_PHYERR_CCK_HEADER_CRC       = 26,
    ATH9K_PHYERR_CCK_RATE_ILLEGAL     = 27,
    ATH9K_PHYERR_CCK_LENGTH_ILLEGAL   = 28,
    ATH9K_PHYERR_CCK_POWER_DROP       = 29,
    ATH9K_PHYERR_CCK_SERVICE          = 30,
    ATH9K_PHYERR_CCK_RESTART          = 31,

    ATH9K_PHYERR_HT_CRC_ERROR         = 32,
    ATH9K_PHYERR_HT_LENGTH_ILLEGAL    = 33,
    ATH9K_PHYERR_HT_RATE_ILLEGAL      = 34,
    ATH9K_PHYERR_HT_ZLF               = 35,

    ATH9K_PHYERR_FALSE_RADAR_EXT      = 36,
    ATH9K_PHYERR_GREEN_FIELD          = 37,
    ATH9K_PHYERR_SPECTRAL             = 38,

    ATH9K_PHYERR_MAX                  = 39,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath_desc {
    pub ds_link: u32,
    pub ds_data: u32,
    pub ds_ctl0: u32,
    pub ds_ctl1: u32,
    pub ds_hw: [u32; 20],
    pub ds_vdata: *mut c_void,
    pub __aligned(4): } __packed,
pub const ATH9K_TXDESC_NOACK: c_uint = 0x0002;
pub const ATH9K_TXDESC_RTSENA: c_uint = 0x0004;
pub const ATH9K_TXDESC_CTSENA: c_uint = 0x0008;
// ATH9K_TXDESC_INTREQ forces a tx interrupt to be generated for
// the descriptor its marked on.  We take a tx interrupt to reap
// descriptors when the h/w hits an EOL condition or
// when the descriptor is specifically marked to generate
// an interrupt with this flag. Descriptors should be
// marked periodically to insure timely replenishing of the
// supply needed for sending frames. Deferring interrupts
// reduces system load and potentially allows more concurrent
// work to be done but if done to aggressively can cause
// senders to backup. When the hardware queue is left too
// large rate control information may also be too out of
// date. An Alternative for this is TX interrupt mitigation
// but this needs more testing.
pub const ATH9K_TXDESC_INTREQ: c_uint = 0x0010;
pub const ATH9K_TXDESC_VEOL: c_uint = 0x0020;
pub const ATH9K_TXDESC_EXT_ONLY: c_uint = 0x0040;
pub const ATH9K_TXDESC_EXT_AND_CTL: c_uint = 0x0080;
pub const ATH9K_TXDESC_VMF: c_uint = 0x0100;
pub const ATH9K_TXDESC_FRAG_IS_ON: c_uint = 0x0200;
pub const ATH9K_TXDESC_LOWRXCHAIN: c_uint = 0x0400;
pub const ATH9K_TXDESC_LDPC: c_uint = 0x0800;
pub const ATH9K_TXDESC_CLRDMASK: c_uint = 0x1000;
pub const ATH9K_TXDESC_PAPRD: c_uint = 0x70000;
pub const ATH9K_TXDESC_PAPRD_S: c_int = 16;
pub const ATH9K_RXDESC_INTREQ: c_uint = 0x0020;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ar5416_desc {
    pub ds_link: u32,
    pub ds_data: u32,
    pub ds_ctl0: u32,
    pub ds_ctl1: u32,
    pub ctl2: u32,
    pub ctl3: u32,
    pub ctl4: u32,
    pub ctl5: u32,
    pub ctl6: u32,
    pub ctl7: u32,
    pub ctl8: u32,
    pub ctl9: u32,
    pub ctl10: u32,
    pub ctl11: u32,
    pub status0: u32,
    pub status1: u32,
    pub status2: u32,
    pub status3: u32,
    pub status4: u32,
    pub status5: u32,
    pub status6: u32,
    pub status7: u32,
    pub status8: u32,
    pub status9: u32,
    pub tx: },
    pub status0: u32,
    pub status1: u32,
    pub status2: u32,
    pub status3: u32,
    pub status4: u32,
    pub status5: u32,
    pub status6: u32,
    pub status7: u32,
    pub status8: u32,
    pub rx: },
    pub u: },
    pub __aligned(4): } __packed,

pub const AR_FrameLen: c_uint = 0x00000fff;
pub const AR_VirtMoreFrag: c_uint = 0x00001000;
pub const AR_TxCtlRsvd00: c_uint = 0x0000e000;
pub const AR_XmitPower0: c_uint = 0x003f0000;
pub const AR_XmitPower0_S: c_int = 16;
pub const AR_XmitPower1: c_uint = 0x3f000000;
pub const AR_XmitPower1_S: c_int = 24;
pub const AR_XmitPower2: c_uint = 0x3f000000;
pub const AR_XmitPower2_S: c_int = 24;
pub const AR_XmitPower3: c_uint = 0x3f000000;
pub const AR_XmitPower3_S: c_int = 24;
pub const AR_RTSEnable: c_uint = 0x00400000;
pub const AR_VEOL: c_uint = 0x00800000;
pub const AR_ClrDestMask: c_uint = 0x01000000;
pub const AR_TxCtlRsvd01: c_uint = 0x1e000000;
pub const AR_TxIntrReq: c_uint = 0x20000000;
pub const AR_DestIdxValid: c_uint = 0x40000000;
pub const AR_CTSEnable: c_uint = 0x80000000;
pub const AR_TxMore: c_uint = 0x00001000;
pub const AR_DestIdx: c_uint = 0x000fe000;
pub const AR_DestIdx_S: c_int = 13;
pub const AR_FrameType: c_uint = 0x00f00000;
pub const AR_FrameType_S: c_int = 20;
pub const AR_NoAck: c_uint = 0x01000000;
pub const AR_InsertTS: c_uint = 0x02000000;
pub const AR_CorruptFCS: c_uint = 0x04000000;
pub const AR_ExtOnly: c_uint = 0x08000000;
pub const AR_ExtAndCtl: c_uint = 0x10000000;
pub const AR_MoreAggr: c_uint = 0x20000000;
pub const AR_IsAggr: c_uint = 0x40000000;
pub const AR_BurstDur: c_uint = 0x00007fff;
pub const AR_BurstDur_S: c_int = 0;
pub const AR_DurUpdateEna: c_uint = 0x00008000;
pub const AR_XmitDataTries0: c_uint = 0x000f0000;
pub const AR_XmitDataTries0_S: c_int = 16;
pub const AR_XmitDataTries1: c_uint = 0x00f00000;
pub const AR_XmitDataTries1_S: c_int = 20;
pub const AR_XmitDataTries2: c_uint = 0x0f000000;
pub const AR_XmitDataTries2_S: c_int = 24;
pub const AR_XmitDataTries3: c_uint = 0xf0000000;
pub const AR_XmitDataTries3_S: c_int = 28;
pub const AR_XmitRate0: c_uint = 0x000000ff;
pub const AR_XmitRate0_S: c_int = 0;
pub const AR_XmitRate1: c_uint = 0x0000ff00;
pub const AR_XmitRate1_S: c_int = 8;
pub const AR_XmitRate2: c_uint = 0x00ff0000;
pub const AR_XmitRate2_S: c_int = 16;
pub const AR_XmitRate3: c_uint = 0xff000000;
pub const AR_XmitRate3_S: c_int = 24;
pub const AR_PacketDur0: c_uint = 0x00007fff;
pub const AR_PacketDur0_S: c_int = 0;
pub const AR_RTSCTSQual0: c_uint = 0x00008000;
pub const AR_PacketDur1: c_uint = 0x7fff0000;
pub const AR_PacketDur1_S: c_int = 16;
pub const AR_RTSCTSQual1: c_uint = 0x80000000;
pub const AR_PacketDur2: c_uint = 0x00007fff;
pub const AR_PacketDur2_S: c_int = 0;
pub const AR_RTSCTSQual2: c_uint = 0x00008000;
pub const AR_PacketDur3: c_uint = 0x7fff0000;
pub const AR_PacketDur3_S: c_int = 16;
pub const AR_RTSCTSQual3: c_uint = 0x80000000;
pub const AR_AggrLen: c_uint = 0x0000ffff;
pub const AR_AggrLen_S: c_int = 0;
pub const AR_TxCtlRsvd60: c_uint = 0x00030000;
pub const AR_PadDelim: c_uint = 0x03fc0000;
pub const AR_PadDelim_S: c_int = 18;
pub const AR_EncrType: c_uint = 0x0c000000;
pub const AR_EncrType_S: c_int = 26;
pub const AR_TxCtlRsvd61: c_uint = 0xf0000000;
pub const AR_LDPC: c_uint = 0x80000000;
pub const AR_2040_0: c_uint = 0x00000001;
pub const AR_GI0: c_uint = 0x00000002;
pub const AR_ChainSel0: c_uint = 0x0000001c;
pub const AR_ChainSel0_S: c_int = 2;
pub const AR_2040_1: c_uint = 0x00000020;
pub const AR_GI1: c_uint = 0x00000040;
pub const AR_ChainSel1: c_uint = 0x00000380;
pub const AR_ChainSel1_S: c_int = 7;
pub const AR_2040_2: c_uint = 0x00000400;
pub const AR_GI2: c_uint = 0x00000800;
pub const AR_ChainSel2: c_uint = 0x00007000;
pub const AR_ChainSel2_S: c_int = 12;
pub const AR_2040_3: c_uint = 0x00008000;
pub const AR_GI3: c_uint = 0x00010000;
pub const AR_ChainSel3: c_uint = 0x000e0000;
pub const AR_ChainSel3_S: c_int = 17;
pub const AR_RTSCTSRate: c_uint = 0x0ff00000;
pub const AR_RTSCTSRate_S: c_int = 20;
pub const AR_STBC0: c_uint = 0x10000000;
pub const AR_STBC1: c_uint = 0x20000000;
pub const AR_STBC2: c_uint = 0x40000000;
pub const AR_STBC3: c_uint = 0x80000000;
pub const AR_TxRSSIAnt00: c_uint = 0x000000ff;
pub const AR_TxRSSIAnt00_S: c_int = 0;
pub const AR_TxRSSIAnt01: c_uint = 0x0000ff00;
pub const AR_TxRSSIAnt01_S: c_int = 8;
pub const AR_TxRSSIAnt02: c_uint = 0x00ff0000;
pub const AR_TxRSSIAnt02_S: c_int = 16;
pub const AR_TxStatusRsvd00: c_uint = 0x3f000000;
pub const AR_TxBaStatus: c_uint = 0x40000000;
pub const AR_TxStatusRsvd01: c_uint = 0x80000000;
//
// AR_FrmXmitOK - Frame transmission success flag. If set, the frame was
// transmitted successfully. If clear, no ACK or BA was received to indicate
// successful transmission when we were expecting an ACK or BA.
//
pub const AR_FrmXmitOK: c_uint = 0x00000001;
pub const AR_ExcessiveRetries: c_uint = 0x00000002;
pub const AR_FIFOUnderrun: c_uint = 0x00000004;
pub const AR_Filtered: c_uint = 0x00000008;
pub const AR_RTSFailCnt: c_uint = 0x000000f0;
pub const AR_RTSFailCnt_S: c_int = 4;
pub const AR_DataFailCnt: c_uint = 0x00000f00;
pub const AR_DataFailCnt_S: c_int = 8;
pub const AR_VirtRetryCnt: c_uint = 0x0000f000;
pub const AR_VirtRetryCnt_S: c_int = 12;
pub const AR_TxDelimUnderrun: c_uint = 0x00010000;
pub const AR_TxDataUnderrun: c_uint = 0x00020000;
pub const AR_DescCfgErr: c_uint = 0x00040000;
pub const AR_TxTimerExpired: c_uint = 0x00080000;
pub const AR_TxStatusRsvd10: c_uint = 0xfff00000;

pub const AR_TxRSSIAnt10: c_uint = 0x000000ff;
pub const AR_TxRSSIAnt10_S: c_int = 0;
pub const AR_TxRSSIAnt11: c_uint = 0x0000ff00;
pub const AR_TxRSSIAnt11_S: c_int = 8;
pub const AR_TxRSSIAnt12: c_uint = 0x00ff0000;
pub const AR_TxRSSIAnt12_S: c_int = 16;
pub const AR_TxRSSICombined: c_uint = 0xff000000;
pub const AR_TxRSSICombined_S: c_int = 24;
pub const AR_TxTid: c_uint = 0xf0000000;
pub const AR_TxTid_S: c_int = 28;

pub const AR_TxDone: c_uint = 0x00000001;
pub const AR_SeqNum: c_uint = 0x00001ffe;
pub const AR_SeqNum_S: c_int = 1;
pub const AR_TxStatusRsvd80: c_uint = 0x0001e000;
pub const AR_TxOpExceeded: c_uint = 0x00020000;
pub const AR_TxStatusRsvd81: c_uint = 0x001c0000;
pub const AR_FinalTxIdx: c_uint = 0x00600000;
pub const AR_FinalTxIdx_S: c_int = 21;
pub const AR_TxStatusRsvd82: c_uint = 0x01800000;
pub const AR_PowerMgmt: c_uint = 0x02000000;
pub const AR_TxStatusRsvd83: c_uint = 0xfc000000;
pub const AR_RxCTLRsvd00: c_uint = 0xffffffff;
pub const AR_RxCtlRsvd00: c_uint = 0x00001000;
pub const AR_RxIntrReq: c_uint = 0x00002000;
pub const AR_RxCtlRsvd01: c_uint = 0xffffc000;
pub const AR_RxRSSIAnt00: c_uint = 0x000000ff;
pub const AR_RxRSSIAnt00_S: c_int = 0;
pub const AR_RxRSSIAnt01: c_uint = 0x0000ff00;
pub const AR_RxRSSIAnt01_S: c_int = 8;
pub const AR_RxRSSIAnt02: c_uint = 0x00ff0000;
pub const AR_RxRSSIAnt02_S: c_int = 16;
pub const AR_RxRate: c_uint = 0xff000000;
pub const AR_RxRate_S: c_int = 24;
pub const AR_RxStatusRsvd00: c_uint = 0xff000000;
pub const AR_DataLen: c_uint = 0x00000fff;
pub const AR_RxMore: c_uint = 0x00001000;
pub const AR_NumDelim: c_uint = 0x003fc000;
pub const AR_NumDelim_S: c_int = 14;
pub const AR_RxStatusRsvd10: c_uint = 0xff800000;

pub const AR_GI: c_uint = 0x00000001;
pub const AR_2040: c_uint = 0x00000002;
pub const AR_Parallel40: c_uint = 0x00000004;
pub const AR_Parallel40_S: c_int = 2;
pub const AR_STBC: c_uint = 0x00000008 /* on ar9280 and later */;
pub const AR_RxStatusRsvd30: c_uint = 0x000000f0;
pub const AR_RxAntenna: c_uint = 0xffffff00;
pub const AR_RxAntenna_S: c_int = 8;
pub const AR_RxRSSIAnt10: c_uint = 0x000000ff;
pub const AR_RxRSSIAnt10_S: c_int = 0;
pub const AR_RxRSSIAnt11: c_uint = 0x0000ff00;
pub const AR_RxRSSIAnt11_S: c_int = 8;
pub const AR_RxRSSIAnt12: c_uint = 0x00ff0000;
pub const AR_RxRSSIAnt12_S: c_int = 16;
pub const AR_RxRSSICombined: c_uint = 0xff000000;
pub const AR_RxRSSICombined_S: c_int = 24;

pub const AR_RxDone: c_uint = 0x00000001;
pub const AR_RxFrameOK: c_uint = 0x00000002;
pub const AR_CRCErr: c_uint = 0x00000004;
pub const AR_DecryptCRCErr: c_uint = 0x00000008;
pub const AR_PHYErr: c_uint = 0x00000010;
pub const AR_MichaelErr: c_uint = 0x00000020;
pub const AR_PreDelimCRCErr: c_uint = 0x00000040;
pub const AR_RxStatusRsvd70: c_uint = 0x00000080;
pub const AR_RxKeyIdxValid: c_uint = 0x00000100;
pub const AR_KeyIdx: c_uint = 0x0000fe00;
pub const AR_KeyIdx_S: c_int = 9;
pub const AR_PHYErrCode: c_uint = 0x0000ff00;
pub const AR_PHYErrCode_S: c_int = 8;
pub const AR_RxMoreAggr: c_uint = 0x00010000;
pub const AR_RxAggr: c_uint = 0x00020000;
pub const AR_PostDelimCRCErr: c_uint = 0x00040000;
pub const AR_RxStatusRsvd71: c_uint = 0x3ff80000;
pub const AR_RxFirstAggr: c_uint = 0x20000000;
pub const AR_DecryptBusyErr: c_uint = 0x40000000;
pub const AR_KeyMiss: c_uint = 0x80000000;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath9k_tx_queue {
    ATH9K_TX_QUEUE_INACTIVE = 0,
    ATH9K_TX_QUEUE_DATA,
    ATH9K_TX_QUEUE_BEACON,
    ATH9K_TX_QUEUE_CAB,
    ATH9K_TX_QUEUE_UAPSD,
    ATH9K_TX_QUEUE_PSPOLL
}

pub const ATH9K_NUM_TX_QUEUES: c_int = 10;
// Used as a queue subtype instead of a WMM AC
pub const ATH9K_WME_UPSD: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath9k_tx_queue_flags {
    TXQ_FLAG_TXINT_ENABLE = 0x0001,
    TXQ_FLAG_TXDESCINT_ENABLE = 0x0002,
    TXQ_FLAG_TXEOLINT_ENABLE = 0x0004,
    TXQ_FLAG_TXURNINT_ENABLE = 0x0008,
    TXQ_FLAG_BACKOFF_DISABLE = 0x0010,
    TXQ_FLAG_COMPRESSION_ENABLE = 0x0020,
    TXQ_FLAG_RDYTIME_EXP_POLICY_ENABLE = 0x0040,
    TXQ_FLAG_FRAG_BURST_BACKOFF_ENABLE = 0x0080,
}

pub const ATH9K_TXQ_USE_LOCKOUT_BKOFF_DIS: c_uint = 0x00000001;
pub const ATH9K_DECOMP_MASK_SIZE: c_int = 128;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath9k_pkt_type {
    ATH9K_PKT_TYPE_NORMAL = 0,
    ATH9K_PKT_TYPE_ATIM,
    ATH9K_PKT_TYPE_PSPOLL,
    ATH9K_PKT_TYPE_BEACON,
    ATH9K_PKT_TYPE_PROBE_RESP,
    ATH9K_PKT_TYPE_CHIRP,
    ATH9K_PKT_TYPE_GRP_POLL,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath9k_tx_queue_info {
    pub tqi_ver: u32,
    pub tqi_type: ath9k_tx_queue,
    pub tqi_subtype: c_int,
    pub tqi_qflags: ath9k_tx_queue_flags,
    pub tqi_priority: u32,
    pub tqi_aifs: u32,
    pub tqi_cwmin: u32,
    pub tqi_cwmax: u32,
    pub tqi_shretry: u16,
    pub tqi_lgretry: u16,
    pub tqi_cbrPeriod: u32,
    pub tqi_cbrOverflowLimit: u32,
    pub tqi_burstTime: u32,
    pub tqi_readyTime: u32,
    pub tqi_physCompBuf: u32,
    pub tqi_intFlags: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath9k_rx_filter {
    ATH9K_RX_FILTER_UCAST = 0x00000001,
    ATH9K_RX_FILTER_MCAST = 0x00000002,
    ATH9K_RX_FILTER_BCAST = 0x00000004,
    ATH9K_RX_FILTER_CONTROL = 0x00000008,
    ATH9K_RX_FILTER_BEACON = 0x00000010,
    ATH9K_RX_FILTER_PROM = 0x00000020,
    ATH9K_RX_FILTER_PROBEREQ = 0x00000080,
    ATH9K_RX_FILTER_PHYERR = 0x00000100,
    ATH9K_RX_FILTER_MYBEACON = 0x00000200,
    ATH9K_RX_FILTER_COMP_BAR = 0x00000400,
    ATH9K_RX_FILTER_COMP_BA = 0x00000800,
    ATH9K_RX_FILTER_UNCOMP_BA_BAR = 0x00001000,
    ATH9K_RX_FILTER_PSPOLL = 0x00004000,
    ATH9K_RX_FILTER_PHYRADAR = 0x00002000,
    ATH9K_RX_FILTER_MCAST_BCAST_ALL = 0x00008000,
    ATH9K_RX_FILTER_CONTROL_WRAPPER = 0x00080000,
    ATH9K_RX_FILTER_4ADDRESS = 0x00100000,
}

pub const ATH9K_RATESERIES_RTS_CTS: c_uint = 0x0001;
pub const ATH9K_RATESERIES_2040: c_uint = 0x0002;
pub const ATH9K_RATESERIES_HALFGI: c_uint = 0x0004;
pub const ATH9K_RATESERIES_STBC: c_uint = 0x0008;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath9k_11n_rate_series {
    pub Tries: u32,
    pub Rate: u32,
    pub PktDuration: u32,
    pub ChSel: u32,
    pub RateFlags: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum aggr_type {
    AGGR_BUF_NONE,
    AGGR_BUF_FIRST,
    AGGR_BUF_MIDDLE,
    AGGR_BUF_LAST,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath9k_key_type {
    ATH9K_KEY_TYPE_CLEAR,
    ATH9K_KEY_TYPE_WEP,
    ATH9K_KEY_TYPE_AES,
    ATH9K_KEY_TYPE_TKIP,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath_tx_info {
    pub qcu: u8,
    pub is_first: bool,
    pub is_last: bool,
    pub aggr: aggr_type,
    pub ndelim: u8,
    pub aggr_len: u16,
    pub link: dma_addr_t,
    pub pkt_len: c_int,
    pub flags: u32,
    pub buf_addr: [dma_addr_t; 4],
    pub buf_len: [c_int; 4],
    pub rates: [ath9k_11n_rate_series; 4],
    pub rtscts_rate: u8,
    pub dur_update: bool,
    pub type: ath9k_pkt_type,
    pub keytype: ath9k_key_type,
    pub keyix: u8,
    pub txpower: [u8; 4],
}

extern "C" {
    pub fn ath9k_hw_gettxbuf(ah: *mut ath_hw, q: u32) -> u32;
}
extern "C" {
    pub fn ath9k_hw_puttxbuf(ah: *mut ath_hw, q: u32, txdp: u32);
}
extern "C" {
    pub fn ath9k_hw_txstart(ah: *mut ath_hw, q: u32);
}
extern "C" {
    pub fn ath9k_hw_numtxpending(ah: *mut ath_hw, q: u32) -> u32;
}
extern "C" {
    pub fn ath9k_hw_updatetxtriglevel(ah: *mut ath_hw, bIncTrigLevel: bool) -> bool;
}
extern "C" {
    pub fn ath9k_hw_stop_dma_queue(ah: *mut ath_hw, q: u32) -> bool;
}
extern "C" {
    pub fn ath9k_hw_abort_tx_dma(ah: *mut ath_hw);
}
extern "C" {
    pub fn ath9k_hw_releasetxqueue(ah: *mut ath_hw, q: u32) -> bool;
}
extern "C" {
    pub fn ath9k_hw_resettxqueue(ah: *mut ath_hw, q: u32) -> bool;
}
extern "C" {
    pub fn ath9k_hw_setrxabort(ah: *mut ath_hw, set: bool) -> bool;
}
extern "C" {
    pub fn ath9k_hw_putrxbuf(ah: *mut ath_hw, rxdp: u32);
}
extern "C" {
    pub fn ath9k_hw_startpcureceive(ah: *mut ath_hw, is_scanning: bool);
}
extern "C" {
    pub fn ath9k_hw_abortpcurecv(ah: *mut ath_hw);
}
extern "C" {
    pub fn ath9k_hw_stopdmarecv(ah: *mut ath_hw, reset: *mut bool) -> bool;
}
extern "C" {
    pub fn ath9k_hw_beaconq_setup(ah: *mut ath_hw) -> c_int;
}
extern "C" {
    pub fn ath9k_hw_set_tx_filter(ah: *mut ath_hw, destidx: u8, set: bool);
}
// Interrupt Handling
extern "C" {
    pub fn ath9k_hw_intrpend(ah: *mut ath_hw) -> bool;
}
extern "C" {
    pub fn ath9k_hw_set_interrupts(ah: *mut ath_hw);
}
extern "C" {
    pub fn ath9k_hw_enable_interrupts(ah: *mut ath_hw);
}
extern "C" {
    pub fn ath9k_hw_disable_interrupts(ah: *mut ath_hw);
}
extern "C" {
    pub fn ath9k_hw_kill_interrupts(ah: *mut ath_hw);
}
extern "C" {
    pub fn ath9k_hw_resume_interrupts(ah: *mut ath_hw);
}
extern "C" {
    pub fn ar9002_hw_attach_mac_ops(ah: *mut ath_hw);
}
