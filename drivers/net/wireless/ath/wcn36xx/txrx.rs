//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/wcn36xx/txrx.h
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
// Copyright (c) 2013 Eugene Krasnikov <k.eugene.e@gmail.com>
//
// Permission to use, copy, modify, and/or distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY
// SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN ACTION
// OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF OR IN
// CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
//

// TODO describe all properties
pub const WCN36XX_802_11_HEADER_LEN: c_int = 24;
pub const WCN36XX_BMU_WQ_TX: c_int = 25;
pub const WCN36XX_TID: c_int = 7;
// broadcast wq ID
pub const WCN36XX_TX_B_WQ_ID: c_uint = 0xA;
pub const WCN36XX_TX_U_WQ_ID: c_uint = 0x9;
// bd_rate
pub const WCN36XX_BD_RATE_DATA: c_int = 0;
pub const WCN36XX_BD_RATE_MGMT: c_int = 2;
pub const WCN36XX_BD_RATE_CTRL: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wcn36xx_txbd_ssn_type {
    WCN36XX_TXBD_SSN_FILL_HOST = 0,
    WCN36XX_TXBD_SSN_FILL_DPU_NON_QOS = 1,
    WCN36XX_TXBD_SSN_FILL_DPU_QOS = 2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_pdu {
    pub dpu_fb:8: u32,
    pub adu_fb:8: u32,
    pub pdu_id:16: u32,
// 0x04
    pub tail_pdu_idx:16: u32,
    pub head_pdu_idx:16: u32,
// 0x08
    pub pdu_count:7: u32,
    pub mpdu_data_off:9: u32,
    pub mpdu_header_off:8: u32,
    pub mpdu_header_len:8: u32,
// 0x0c
    pub reserved4:8: u32,
    pub tid:4: u32,
    pub bd_ssn:2: u32,
    pub reserved3:2: u32,
    pub mpdu_len:16: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_rx_bd {
    pub bdt:2: u32,
    pub ft:1: u32,
    pub dpu_ne:1: u32,
    pub rx_key_id:3: u32,
    pub ub:1: u32,
    pub rmf:1: u32,
    pub uma_bypass:1: u32,
    pub csr11:1: u32,
    pub reserved0:1: u32,
    pub scan_learn:1: u32,
    pub rx_ch:4: u32,
    pub rtsf:1: u32,
    pub bsf:1: u32,
    pub a2hf:1: u32,
    pub st_auf:1: u32,
    pub dpu_sign:3: u32,
    pub dpu_rf:8: u32,
    pub pdu: wcn36xx_pdu,
// 0x14
    pub addr3:8: u32,
    pub addr2:8: u32,
    pub addr1:8: u32,
    pub dpu_desc_idx:8: u32,
// 0x18
    pub rxp_flags:23: u32,
    pub rate_id:9: u32,
    pub phy_stat0: u32,
    pub phy_stat1: u32,
// 0x24
    pub rx_times: u32,
    pub pmi_cmd: [u32; 6],
// 0x40
    pub reserved7:4: u32,
    pub reorder_slot_id:6: u32,
    pub reorder_fwd_id:6: u32,
    pub reserved6:12: u32,
    pub reorder_code:4: u32,
// 0x44
    pub exp_seq_num:12: u32,
    pub cur_seq_num:12: u32,
    pub rf_band:2: u32,
    pub fr_type_subtype:6: u32,
// 0x48
    pub msdu_size:16: u32,
    pub sub_fr_id:4: u32,
    pub proc_order:4: u32,
    pub reserved9:4: u32,
    pub aef:1: u32,
    pub lsf:1: u32,
    pub esf:1: u32,
    pub asf:1: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_tx_bd {
    pub bdt:2: u32,
    pub ft:1: u32,
    pub dpu_ne:1: u32,
    pub fw_tx_comp:1: u32,
    pub tx_comp:1: u32,
    pub reserved1:1: u32,
    pub ub:1: u32,
    pub rmf:1: u32,
    pub reserved0:12: u32,
    pub dpu_sign:3: u32,
    pub dpu_rf:8: u32,
    pub pdu: wcn36xx_pdu,
// 0x14
    pub reserved5:7: u32,
    pub queue_id:5: u32,
    pub bd_rate:2: u32,
    pub ack_policy:2: u32,
    pub sta_index:8: u32,
    pub dpu_desc_idx:8: u32,
    pub tx_bd_sign: u32,
    pub reserved6: u32,
    pub dxe_start_time: u32,
    pub dxe_end_time: u32,
// u32	tcp_udp_start_off:10;
    pub header_cks:16: u32,
    pub reserved7:6;*/: *mut u32,
}

extern "C" {
    pub fn wcn36xx_rx_skb(wcn: *mut wcn36xx, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn wcn36xx_process_tx_rate(stats: *mut ani_global_class_a_stats_info, info: *mut rate_info);
}
