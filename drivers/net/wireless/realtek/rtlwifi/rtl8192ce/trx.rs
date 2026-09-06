//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/realtek/rtlwifi/rtl8192ce/trx.h
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
pub const RX_DRV_INFO_SIZE_UNIT: c_int = 8;
pub const TX_DESC_NEXT_DESC_OFFSET: c_int = 40;
pub const USB_HWDESC_HEADER_LEN: c_int = 32;
pub const CRCLENGTH: c_int = 4;
// macros to read/write various fields in RX or TX descriptors
extern "C" {
    pub fn le32_get_bits(_arg: *mut (__pdesc), _arg: BIT(31)) -> return;
}
// (__pdesc + 8) = cpu_to_le32(__val);
extern "C" {
    pub fn le32_to_cpu(8)): *mut *mut ((__pdesc +) -> return;
}
// (__pdesc + 10) = cpu_to_le32(__val);
extern "C" {
    pub fn le32_get_bits(_arg: *mut (__pdesc), _arg: GENMASK(13, _arg: 0)) -> return;
}
extern "C" {
    pub fn le32_get_bits(_arg: *mut (__pdesc), _arg: BIT(14)) -> return;
}
extern "C" {
    pub fn le32_get_bits(_arg: *mut (__pdesc), _arg: BIT(15)) -> return;
}
extern "C" {
    pub fn le32_get_bits(_arg: *mut (__pdesc), _arg: GENMASK(19, _arg: 16)) -> return;
}
extern "C" {
    pub fn le32_get_bits(_arg: *mut (__pdesc), _arg: GENMASK(25, _arg: 24)) -> return;
}
extern "C" {
    pub fn le32_get_bits(_arg: *mut (__pdesc), _arg: BIT(26)) -> return;
}
extern "C" {
    pub fn le32_get_bits(_arg: *mut (__pdesc), _arg: BIT(27)) -> return;
}
extern "C" {
    pub fn le32_get_bits(_arg: *mut (__pdesc), _arg: BIT(31)) -> return;
}
extern "C" {
    pub fn le32_get_bits(1)): *mut *mut ((__pdesc +, _arg: BIT(14)) -> return;
}
extern "C" {
    pub fn le32_get_bits(1)): *mut *mut ((__pdesc +, _arg: BIT(15)) -> return;
}
extern "C" {
    pub fn le32_get_bits(3)): *mut *mut ((__pdesc +, _arg: GENMASK(5, _arg: 0)) -> return;
}
extern "C" {
    pub fn le32_get_bits(3)): *mut *mut ((__pdesc +, _arg: BIT(6)) -> return;
}
extern "C" {
    pub fn le32_get_bits(3)): *mut *mut ((__pdesc +, _arg: BIT(8)) -> return;
}
extern "C" {
    pub fn le32_get_bits(3)): *mut *mut ((__pdesc +, _arg: BIT(9)) -> return;
}
extern "C" {
    pub fn le32_to_cpu(5)): *mut *mut ((__pdesc +) -> return;
}
extern "C" {
    pub fn le32_to_cpu(6)): *mut *mut ((__pdesc +) -> return;
}
// (__pdesc + 6) = cpu_to_le32(__val);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rx_fwinfo_92c {
    pub gain_trsw: [u8; 4],
    pub pwdb_all: u8,
    pub cfosho: [u8; 4],
    pub cfotail: [u8; 4],
    pub rxevm: [i8; 2],
    pub rxsnr: [i8; 4],
    pub pdsnr: [u8; 2],
    pub csi_current: [u8; 2],
    pub csi_target: [u8; 2],
    pub sigevm: u8,
    pub max_ex_pwr: u8,
    pub ex_intf_flag:1: u8,
    pub sgi_en:1: u8,
    pub rxsc:2: u8,
    pub reserve:4: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tx_desc_92c {
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
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rx_desc_92c {
    pub length:14: u32,
    pub crc32:1: u32,
    pub icverror:1: u32,
    pub drv_infosize:4: u32,
    pub security:3: u32,
    pub qos:1: u32,
    pub shift:2: u32,
    pub phystatus:1: u32,
    pub swdec:1: u32,
    pub lastseg:1: u32,
    pub firstseg:1: u32,
    pub eor:1: u32,
    pub own:1: u32,
    pub macid:5: u32,
    pub tid:4: u32,
    pub hwrsvd:5: u32,
    pub paggr:1: u32,
    pub faggr:1: u32,
    pub a1_fit:4: u32,
    pub a2_fit:4: u32,
    pub pam:1: u32,
    pub pwr:1: u32,
    pub moredata:1: u32,
    pub morefrag:1: u32,
    pub type:2: u32,
    pub mc:1: u32,
    pub bc:1: u32,
    pub seq:12: u32,
    pub frag:4: u32,
    pub nextpktlen:14: u32,
    pub nextind:1: u32,
    pub rsvd:1: u32,
    pub rxmcs:6: u32,
    pub rxht:1: u32,
    pub amsdu:1: u32,
    pub splcp:1: u32,
    pub bandwidth:1: u32,
    pub htc:1: u32,
    pub tcpchk_rpt:1: u32,
    pub ipcchk_rpt:1: u32,
    pub tcpchk_valid:1: u32,
    pub hwpcerr:1: u32,
    pub hwpcind:1: u32,
    pub iv0:16: u32,
    pub iv1: u32,
    pub tsfl: u32,
    pub bufferaddress: u32,
    pub bufferaddress64: u32,
    pub __packed: },
    pub ptcb_desc): *mut rtl_tcb_desc,
    pub skb): *mut *mut u8 pdesc, struct sk_buff,
    pub val): *mut u8 desc_name, u8,
    pub desc_name): bool istx, u8,
    pub index): u8 hw_queue, u16,
    pub hw_queue): *mut *mut void rtl92ce_tx_polling(struct ieee80211_hw hw, u8,
    pub skb): *mut sk_buff,
