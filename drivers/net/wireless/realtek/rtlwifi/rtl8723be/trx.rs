//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/realtek/rtlwifi/rtl8723be/trx.h
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
// Copyright(c) 2009-2014  Realtek Corporation.
pub const TX_DESC_SIZE: c_int = 40;
pub const TX_DESC_AGGR_SUBFRAME_SIZE: c_int = 32;
pub const RX_DESC_SIZE: c_int = 32;
pub const RX_DRV_INFO_SIZE_UNIT: c_int = 8;
pub const TX_DESC_NEXT_DESC_OFFSET: c_int = 40;
pub const USB_HWDESC_HEADER_LEN: c_int = 40;
pub const CRCLENGTH: c_int = 4;
extern "C" {
    pub fn le32_get_bits(_arg: *mut __pdesc, _arg: BIT(31)) -> return;
}
// (__pdesc + 10) = cpu_to_le32(__val);
extern "C" {
    pub fn le32_to_cpu(10)): *mut *mut ((__pdesc +) -> return;
}
// (__pdesc + 12) = cpu_to_le32(__val);
extern "C" {
    pub fn le32_get_bits(_arg: *mut __pdesc, _arg: GENMASK(13, _arg: 0)) -> return;
}
extern "C" {
    pub fn le32_get_bits(_arg: *mut __pdesc, _arg: BIT(14)) -> return;
}
extern "C" {
    pub fn le32_get_bits(_arg: *mut __pdesc, _arg: BIT(15)) -> return;
}
extern "C" {
    pub fn le32_get_bits(_arg: *mut __pdesc, _arg: GENMASK(19, _arg: 16)) -> return;
}
extern "C" {
    pub fn le32_get_bits(_arg: *mut __pdesc, _arg: GENMASK(25, _arg: 24)) -> return;
}
extern "C" {
    pub fn le32_get_bits(_arg: *mut __pdesc, _arg: BIT(26)) -> return;
}
extern "C" {
    pub fn le32_get_bits(_arg: *mut __pdesc, _arg: BIT(27)) -> return;
}
extern "C" {
    pub fn le32_get_bits(_arg: *mut __pdesc, _arg: BIT(31)) -> return;
}
extern "C" {
    pub fn le32_get_bits(1): *mut *mut (__pdesc +, _arg: GENMASK(6, _arg: 0)) -> return;
}
extern "C" {
    pub fn le32_get_bits(1): *mut *mut (__pdesc +, _arg: BIT(15)) -> return;
}
extern "C" {
    pub fn le32_get_bits(2): *mut *mut (__pdesc +, _arg: BIT(28)) -> return;
}
extern "C" {
    pub fn le32_get_bits(3): *mut *mut (__pdesc +, _arg: GENMASK(6, _arg: 0)) -> return;
}
extern "C" {
    pub fn le32_get_bits(3): *mut *mut (__pdesc +, _arg: BIT(6)) -> return;
}
extern "C" {
    pub fn le32_get_bits(3): *mut *mut (__pdesc +, _arg: BIT(29)) -> return;
}
extern "C" {
    pub fn le32_get_bits(3): *mut *mut (__pdesc +, _arg: BIT(30)) -> return;
}
extern "C" {
    pub fn le32_get_bits(3): *mut *mut (__pdesc +, _arg: BIT(31)) -> return;
}
extern "C" {
    pub fn le32_get_bits(4): *mut *mut (__pdesc +, _arg: BIT(0)) -> return;
}
extern "C" {
    pub fn le32_get_bits(4): *mut *mut (__pdesc +, _arg: GENMASK(5, _arg: 4)) -> return;
}
extern "C" {
    pub fn le32_to_cpu(5)): *mut *mut ((__pdesc +) -> return;
}
extern "C" {
    pub fn le32_to_cpu(6)): *mut *mut ((__pdesc +) -> return;
}
// (__pdesc + 6) = cpu_to_le32(__val);
// TX report 2 format in Rx desc
extern "C" {
    pub fn le32_to_cpu(4)): *mut *mut ((__rxstatusdesc +) -> return;
}
extern "C" {
    pub fn le32_to_cpu(5)): *mut *mut ((__rxstatusdesc +) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct phy_rx_agc_info_t {

    pub trsw:1: u8 gain:7,,

    pub gain:7: u8 trsw:1,,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct phy_status_rpt {
    pub path_agc: [phy_rx_agc_info_t; 2],
    pub ch_corr: [u8; 2],
    pub cck_sig_qual_ofdm_pwdb_all: u8,
    pub cck_agc_rpt_ofdm_cfosho_a: u8,
    pub cck_rpt_b_ofdm_cfosho_b: u8,
    pub /: *mut *mut u8 rsvd_1;/ ch_corr_msb;,
    pub noise_power_db_msb: u8,
    pub path_cfotail: [i8; 2],
    pub pcts_mask: [u8; 2],
    pub stream_rxevm: [i8; 2],
    pub path_rxsnr: [u8; 2],
    pub noise_power_db_lsb: u8,
    pub rsvd_2: [u8; 3],
    pub stream_csi: [u8; 2],
    pub stream_target_csi: [u8; 2],
    pub sig_evm: u8,
    pub rsvd_3: u8,

    pub /*ex_intf_flg:1;*/: *mut u8 antsel_rx_keep_2:1;,
    pub sgi_en:1: u8,
    pub rxsc:2: u8,
    pub idle_long:1: u8,
    pub r_ant_train_en:1: u8,
    pub ant_sel_b:1: u8,
    pub ant_sel:1: u8,

    pub ant_sel:1: u8,
    pub ant_sel_b:1: u8,
    pub r_ant_train_en:1: u8,
    pub idle_long:1: u8,
    pub rxsc:2: u8,
    pub sgi_en:1: u8,
    pub /*ex_intf_flg:1;*/: *mut u8 antsel_rx_keep_2:1;,

    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rx_fwinfo_8723be {
    pub gain_trsw: [u8; 2],
    pub chl_num:10: u16,
    pub sub_chnl:4: u16,
    pub r_rfmod:2: u16,
    pub pwdb_all: u8,
    pub cfosho: [u8; 4],
    pub cfotail: [u8; 4],
    pub rxevm: [i8; 2],
    pub rxsnr: [i8; 2],
    pub pcts_msk_rpt: [u8; 2],
    pub pdsnr: [u8; 2],
    pub csi_current: [u8; 2],
    pub rx_gain_c: u8,
    pub rx_gain_d: u8,
    pub sigevm: u8,
    pub resvd_0: u8,
    pub antidx_anta:3: u8,
    pub antidx_antb:3: u8,
    pub resvd_1:2: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tx_desc_8723be {
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
    pub macid:6: u32,
    pub rsvd0:2: u32,
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
    pub agg_en:1: u32,
    pub rdg_en:1: u32,
    pub bar_retryht:2: u32,
    pub agg_break:1: u32,
    pub morefrag:1: u32,
    pub raw:1: u32,
    pub ccx:1: u32,
    pub ampdudensity:3: u32,
    pub bt_int:1: u32,
    pub ant_sela:1: u32,
    pub ant_selb:1: u32,
    pub txant_cck:2: u32,
    pub txant_l:2: u32,
    pub txant_ht:2: u32,
    pub nextheadpage:8: u32,
    pub tailpage:8: u32,
    pub seq:12: u32,
    pub cpu_handle:1: u32,
    pub tag1:1: u32,
    pub trigger_int:1: u32,
    pub hwseq_en:1: u32,
    pub rtsrate:5: u32,
    pub apdcfe:1: u32,
    pub qos:1: u32,
    pub hwseq_ssn:1: u32,
    pub userrate:1: u32,
    pub dis_rtsfb:1: u32,
    pub dis_datafb:1: u32,
    pub cts2self:1: u32,
    pub rts_en:1: u32,
    pub hwrts_en:1: u32,
    pub portid:1: u32,
    pub pwr_status:3: u32,
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
    pub sw_offset30:8: u32,
    pub sw_offset31:4: u32,
    pub rsvd1:1: u32,
    pub antsel_c:1: u32,
    pub null_0:1: u32,
    pub null_1:1: u32,
    pub txbuffaddr: u32,
    pub txbufferaddr64: u32,
    pub nextdescaddress: u32,
    pub nextdescaddress64: u32,
    pub reserve_pass_pcie_mm_limit: [u32; 4],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rx_desc_8723be {
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
    pub macid:6: u32,
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
    pub ptcb_desc): *mut u8 hw_queue, struct rtl_tcb_desc,
    pub skb): *mut *mut u8 pdesc, struct sk_buff,
    pub val): *mut bool istx, u8 desc_name, u8,
    pub desc_name): *mut *mut u8 pdesc, bool istx, u8,
    pub index): u8 hw_queue, u16,
    pub hw_queue): *mut *mut void rtl8723be_tx_polling(struct ieee80211_hw hw, u8,
    pub skb): *mut sk_buff,
