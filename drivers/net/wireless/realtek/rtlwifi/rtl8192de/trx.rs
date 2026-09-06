//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/realtek/rtlwifi/rtl8192de/trx.h
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
// Copyright(c) 2009-2012  Realtek Corporation.
pub const TX_DESC_SIZE: c_int = 64;
pub const TX_DESC_AGGR_SUBFRAME_SIZE: c_int = 32;
pub const RX_DESC_SIZE: c_int = 32;
pub const TX_DESC_NEXT_DESC_OFFSET: c_int = 40;
pub const USB_HWDESC_HEADER_LEN: c_int = 32;
pub const CRCLENGTH: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tx_desc_92d {
    pub pktsize:16: u32,
    pub offset:8: u32,
    pub bmc:1: u32,
    pub htc:1: u32,
    pub lastseg:1: u32,
    pub firstseg:1: u32,
    pub linip:1: u32,
    pub noacm:1: u32,
    pub gf:1: u32,
    pub own:1: u32,
    pub macid:5: u32,
    pub agg_en:1: u32,
    pub bk:1: u32,
    pub rdg_en:1: u32,
    pub queuesel:5: u32,
    pub rd_nav_ext:1: u32,
    pub lsig_txop_en:1: u32,
    pub pifs:1: u32,
    pub rateid:4: u32,
    pub nav_usehdr:1: u32,
    pub en_descid:1: u32,
    pub sectype:2: u32,
    pub pktoffset:8: u32,
    pub rts_rc:6: u32,
    pub data_rc:6: u32,
    pub rsvd0:2: u32,
    pub bar_retryht:2: u32,
    pub rsvd1:1: u32,
    pub morefrag:1: u32,
    pub raw:1: u32,
    pub ccx:1: u32,
    pub ampdudensity:3: u32,
    pub rsvd2:1: u32,
    pub ant_sela:1: u32,
    pub ant_selb:1: u32,
    pub txant_cck:2: u32,
    pub txant_l:2: u32,
    pub txant_ht:2: u32,
    pub nextheadpage:8: u32,
    pub tailpage:8: u32,
    pub seq:12: u32,
    pub pktid:4: u32,
    pub rtsrate:5: u32,
    pub apdcfe:1: u32,
    pub qos:1: u32,
    pub hwseq_enable:1: u32,
    pub userrate:1: u32,
    pub dis_rtsfb:1: u32,
    pub dis_datafb:1: u32,
    pub cts2self:1: u32,
    pub rts_en:1: u32,
    pub hwrts_en:1: u32,
    pub portid:1: u32,
    pub rsvd3:3: u32,
    pub waitdcts:1: u32,
    pub cts2ap_en:1: u32,
    pub txsc:2: u32,
    pub stbc:2: u32,
    pub txshort:1: u32,
    pub txbw:1: u32,
    pub rtsshort:1: u32,
    pub rtsbw:1: u32,
    pub rtssc:2: u32,
    pub rtsstbc:2: u32,
    pub txrate:6: u32,
    pub shortgi:1: u32,
    pub ccxt:1: u32,
    pub txrate_fb_lmt:5: u32,
    pub rtsrate_fb_lmt:4: u32,
    pub retrylmt_en:1: u32,
    pub txretrylmt:6: u32,
    pub usb_txaggnum:8: u32,
    pub txagca:5: u32,
    pub txagcb:5: u32,
    pub usemaxlen:1: u32,
    pub maxaggnum:5: u32,
    pub mcsg1maxlen:4: u32,
    pub mcsg2maxlen:4: u32,
    pub mcsg3maxlen:4: u32,
    pub mcs7sgimaxlen:4: u32,
    pub txbuffersize:16: u32,
    pub mcsg4maxlen:4: u32,
    pub mcsg5maxlen:4: u32,
    pub mcsg6maxlen:4: u32,
    pub mcsg15sgimaxlen:4: u32,
    pub txbuffaddr: u32,
    pub txbufferaddr64: u32,
    pub nextdescaddress: u32,
    pub nextdescaddress64: u32,
    pub reserve_pass_pcie_mm_limit: [u32; 4],
    pub __packed: },
    pub ptcb_desc): *mut rtl_tcb_desc,
    pub index): u8 hw_queue, u16,
    pub hw_queue): *mut *mut void rtl92de_tx_polling(struct ieee80211_hw hw, u8,
    pub skb): *mut sk_buff,
