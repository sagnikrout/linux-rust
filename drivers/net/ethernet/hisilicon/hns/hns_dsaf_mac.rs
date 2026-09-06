//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/hisilicon/hns/hns_dsaf_mac.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Copyright (c) 2014-2015 Hisilicon Limited.
//

pub const MAC_MAX_MTU: c_int = 9600;
pub const MAC_MAX_MTU_V2: c_int = 9728;
pub const MAC_MIN_MTU: c_int = 68;

pub const MAC_DEFAULT_PAUSE_TIME: c_uint = 0xffff;
pub const MAC_GMAC_IDX: c_int = 0;
pub const MAC_XGMAC_IDX: c_int = 1;
pub const ETH_STATIC_REG: c_int = 1;
pub const ETH_DUMP_REG: c_int = 5;
// check mac addr broadcast

// check mac addr is 01-00-5e-xx-xx-xx

// check the mac addr is 0 in all bit

// check mac addr multicast

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mac_priv {
    pub mac: *mut c_void,
}

// net speed
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mac_speed {
    MAC_SPEED_10	= 10,	   /**< 10 Mbps */
    MAC_SPEED_100	= 100,	  /**< 100 Mbps */
    MAC_SPEED_1000  = 1000,	 /**< 1000 Mbps = 1 Gbps */
    MAC_SPEED_10000 = 10000	 /**< 10000 Mbps = 10 Gbps */
}

// mac interface keyword
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mac_intf {
    MAC_IF_NONE  = 0x00000000,   /**< interface not invalid */
    MAC_IF_MII   = 0x00010000,   /**< MII interface */
    MAC_IF_RMII  = 0x00020000,   /**< RMII interface */
    MAC_IF_SMII  = 0x00030000,   /**< SMII interface */
    MAC_IF_GMII  = 0x00040000,   /**< GMII interface */
    MAC_IF_RGMII = 0x00050000,   /**< RGMII interface */
    MAC_IF_TBI   = 0x00060000,   /**< TBI interface */
    MAC_IF_RTBI  = 0x00070000,   /**< RTBI interface */
    MAC_IF_SGMII = 0x00080000,   /**< SGMII interface */
    MAC_IF_XGMII = 0x00090000,   /**< XGMII interface */
    MAC_IF_QSGMII = 0x000a0000	/**< QSGMII interface */
}

// mac mode
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mac_mode {
// < Invalid Ethernet mode
    MAC_MODE_INVALID	 = 0,
// <	10 Mbps MII
    MAC_MODE_MII_10	  = (MAC_IF_MII   | MAC_SPEED_10),
// <   100 Mbps MII
    MAC_MODE_MII_100	 = (MAC_IF_MII   | MAC_SPEED_100),
// <	10 Mbps RMII
    MAC_MODE_RMII_10	 = (MAC_IF_RMII  | MAC_SPEED_10),
// <   100 Mbps RMII
    MAC_MODE_RMII_100	= (MAC_IF_RMII  | MAC_SPEED_100),
// <	10 Mbps SMII
    MAC_MODE_SMII_10	 = (MAC_IF_SMII  | MAC_SPEED_10),
// <   100 Mbps SMII
    MAC_MODE_SMII_100	= (MAC_IF_SMII  | MAC_SPEED_100),
// <  1000 Mbps GMII
    MAC_MODE_GMII_1000   = (MAC_IF_GMII  | MAC_SPEED_1000),
// <	10 Mbps RGMII
    MAC_MODE_RGMII_10	= (MAC_IF_RGMII | MAC_SPEED_10),
// <   100 Mbps RGMII
    MAC_MODE_RGMII_100   = (MAC_IF_RGMII | MAC_SPEED_100),
// <  1000 Mbps RGMII
    MAC_MODE_RGMII_1000  = (MAC_IF_RGMII | MAC_SPEED_1000),
// <  1000 Mbps TBI
    MAC_MODE_TBI_1000	= (MAC_IF_TBI   | MAC_SPEED_1000),
// <  1000 Mbps RTBI
    MAC_MODE_RTBI_1000   = (MAC_IF_RTBI  | MAC_SPEED_1000),
// <	10 Mbps SGMII
    MAC_MODE_SGMII_10	= (MAC_IF_SGMII | MAC_SPEED_10),
// <   100 Mbps SGMII
    MAC_MODE_SGMII_100   = (MAC_IF_SGMII | MAC_SPEED_100),
// <  1000 Mbps SGMII
    MAC_MODE_SGMII_1000  = (MAC_IF_SGMII | MAC_SPEED_1000),
// < 10000 Mbps XGMII
    MAC_MODE_XGMII_10000 = (MAC_IF_XGMII | MAC_SPEED_10000),
// <  1000 Mbps QSGMII
    MAC_MODE_QSGMII_1000 = (MAC_IF_QSGMII | MAC_SPEED_1000)
}

// mac communicate mode
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mac_commom_mode {
    MAC_COMM_MODE_NONE	  = 0, /**< No transmit/receive communication */
    MAC_COMM_MODE_RX		= 1, /**< Only receive communication */
    MAC_COMM_MODE_TX		= 2, /**< Only transmit communication */
    MAC_COMM_MODE_RX_AND_TX = 3  /**< Both tx and rx communication */
}

// mac statistics
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mac_statistics {
    pub /: *mut *mut u64 stat_pkts64; / r-10G tr-DT 64 byte frame counter,
    pub /: *mut *mut u64 stat_pkts65to127; / r-10G 65 to 127 byte frame counter,
    pub /: *mut *mut u64 stat_pkts128to255; / r-10G 128 to 255 byte frame counter,
    pub /: *mut *mut u64 stat_pkts256to511; /r-10G 256 to 511 byte frame counter,
    pub /: *mut *mut u64 stat_pkts512to1023;/ r-10G 512 to 1023 byte frame counter,
    pub /: *mut *mut u64 stat_pkts1024to1518; / r-10G 1024 to 1518 byte frame counter,
    pub count*/: *mut *mut u64 stat_pkts1519to1522; / r-10G 1519 to 1522 byte good frame,
// Total number of packets that were less than 64 octets
// long with a wrong CRC.
    pub stat_fragments: u64,
// Total number of packets longer than valid maximum length octets
    pub stat_jabbers: u64,
// number of dropped packets due to internal errors of
// the MAC Client.
    pub stat_drop_events: u64,
// Incremented when frames of correct length but with
// CRC error are received.
    pub stat_crc_align_errors: u64,
// Total number of packets that were less than 64 octets
// long with a good CRC.
    pub stat_undersize_pkts: u64,
    pub T,B.D*/: *mut *mut *mut u64 stat_oversize_pkts; /<,
    pub /: *mut *mut *mut u64 stat_rx_pause; /< Pause MAC Control received,
    pub /: *mut *mut *mut u64 stat_tx_pause; /< Pause MAC Control sent,
    pub /: *mut *mut *mut u64 in_octets; /< Total number of byte received.,
    pub received.*/: *mut *mut u64 in_pkts; / Total number of packets,
    pub /: *mut *mut u64 in_mcast_pkts; / Total number of multicast frame received,
    pub /: *mut *mut u64 in_bcast_pkts; / Total number of broadcast frame received,
// Frames received, but discarded due to
// problems within the MAC RX.
    pub in_discards: u64,
    pub /: *mut *mut u64 in_errors; / Number of frames received with error:,
// - FIFO Overflow Error
// - CRC Error
// - Frame Too Long Error
// - Alignment Error
    pub /: *mut *mut u64 out_octets; /Total number of byte sent.,
    pub .*/: *mut *mut *mut u64 out_pkts; /< Total number of packets sent,
    pub /: *mut *mut u64 out_mcast_pkts; / Total number of multicast frame sent,
    pub /: *mut *mut u64 out_bcast_pkts; / Total number of multicast frame sent,
// Frames received, but discarded due to problems within
// the MAC TX N/A!.
    pub out_discards: u64,
    pub /: *mut *mut u64 out_errors; /Number of frames transmitted with error:,
// - FIFO Overflow Error
// - FIFO Underflow Error
// - Other
}

// mac para struct ,mac get param from nic or dsaf when initialize
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mac_params {
    pub addr: [c_char; ETH_ALEN],
    pub address*/: *mut *mut *mut u8 __iomem vaddr; /virtual,
    pub dev: *mut device,
    pub mac_id: u8,
// < Ethernet operation mode (MAC-PHY interface and speed)
    pub mac_mode: mac_mode,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mac_info {
    pub /: *mut *mut u16 speed;/ The forced speed (lower bits) in,
// *mbps. Please use
// * ethtool_cmd_speed()/_set() to
// * access it
    pub /: *mut *mut u8 duplex; / Duplex, half or full,
    pub /: *mut *mut u8 auto_neg; / Enable or disable autonegotiation,
    pub loop_mode: hnae_loop,
    pub tx_pause_en: u8,
    pub tx_pause_time: u8,
    pub rx_pause_en: u8,
    pub pad_and_crc_en: u8,
    pub promiscuous_en: u8,
    pub enable*/: *mut *mut u8 port_en; /port,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mac_entry_idx {
    pub addr: [u8; ETH_ALEN],
    pub vlan_id:12: u16,
    pub valid:1: u16,
    pub qos:3: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mac_hw_stats {
    pub /: *mut *mut u64 rx_good_pkts; / only for xgmac,
    pub rx_good_bytes: u64,
    pub /: *mut *mut u64 rx_total_pkts; / only for xgmac,
    pub /: *mut *mut u64 rx_total_bytes; / only for xgmac,
    pub /: *mut *mut u64 rx_bad_bytes; / only for gmac,
    pub rx_uc_pkts: u64,
    pub rx_mc_pkts: u64,
    pub rx_bc_pkts: u64,
    pub /: *mut *mut u64 rx_fragment_err; / only for xgmac,
    pub /: *mut *mut u64 rx_undersize; / only for xgmac,
    pub rx_under_min: u64,
    pub /: *mut *mut u64 rx_minto64; / only for gmac,
    pub rx_64bytes: u64,
    pub rx_65to127: u64,
    pub rx_128to255: u64,
    pub rx_256to511: u64,
    pub rx_512to1023: u64,
    pub rx_1024to1518: u64,
    pub rx_1519tomax: u64,
    pub /: *mut *mut u64 rx_1519tomax_good; / only for xgmac,
    pub rx_oversize: u64,
    pub rx_jabber_err: u64,
    pub rx_fcs_err: u64,
    pub /: *mut *mut u64 rx_vlan_pkts; / only for gmac,
    pub /: *mut *mut u64 rx_data_err; / only for gmac,
    pub /: *mut *mut u64 rx_align_err; / only for gmac,
    pub /: *mut *mut u64 rx_long_err; / only for gmac,
    pub rx_pfc_tc0: u64,
    pub /: *mut *mut u64 rx_pfc_tc1; / only for xgmac,
    pub /: *mut *mut u64 rx_pfc_tc2; / only for xgmac,
    pub /: *mut *mut u64 rx_pfc_tc3; / only for xgmac,
    pub /: *mut *mut u64 rx_pfc_tc4; / only for xgmac,
    pub /: *mut *mut u64 rx_pfc_tc5; / only for xgmac,
    pub /: *mut *mut u64 rx_pfc_tc6; / only for xgmac,
    pub /: *mut *mut u64 rx_pfc_tc7; / only for xgmac,
    pub rx_unknown_ctrl: u64,
    pub /: *mut *mut u64 rx_filter_pkts; / only for gmac,
    pub /: *mut *mut u64 rx_filter_bytes; / only for gmac,
    pub /: *mut *mut u64 rx_fifo_overrun_err;/ only for gmac,
    pub /: *mut *mut u64 rx_len_err; / only for gmac,
    pub /: *mut *mut u64 rx_comma_err; / only for gmac,
    pub /: *mut *mut u64 rx_symbol_err; / only for xgmac,
    pub /: *mut *mut u64 tx_good_to_sw; / only for xgmac,
    pub /: *mut *mut u64 tx_bad_to_sw; / only for xgmac,
    pub /: *mut *mut u64 rx_1731_pkts; / only for xgmac,
    pub tx_good_bytes: u64,
    pub /: *mut *mut u64 tx_good_pkts; / only for xgmac,
    pub /: *mut *mut u64 tx_total_bytes; / only for xgmac,
    pub /: *mut *mut u64 tx_total_pkts; / only for xgmac,
    pub /: *mut *mut u64 tx_bad_bytes; / only for gmac,
    pub /: *mut *mut u64 tx_bad_pkts; / only for xgmac,
    pub tx_uc_pkts: u64,
    pub tx_mc_pkts: u64,
    pub tx_bc_pkts: u64,
    pub /: *mut *mut u64 tx_undersize; / only for xgmac,
    pub /: *mut *mut u64 tx_fragment_err; / only for xgmac,
    pub /: *mut *mut u64 tx_under_min_pkts; / only for gmac,
    pub tx_64bytes: u64,
    pub tx_65to127: u64,
    pub tx_128to255: u64,
    pub tx_256to511: u64,
    pub tx_512to1023: u64,
    pub tx_1024to1518: u64,
    pub tx_1519tomax: u64,
    pub /: *mut *mut u64 tx_1519tomax_good; / only for xgmac,
    pub /: *mut *mut u64 tx_oversize; / only for xgmac,
    pub tx_jabber_err: u64,
    pub /: *mut *mut u64 tx_underrun_err; / only for gmac,
    pub /: *mut *mut u64 tx_vlan; / only for gmac,
    pub /: *mut *mut u64 tx_crc_err; / only for gmac,
    pub tx_pfc_tc0: u64,
    pub /: *mut *mut u64 tx_pfc_tc1; / only for xgmac,
    pub /: *mut *mut u64 tx_pfc_tc2; / only for xgmac,
    pub /: *mut *mut u64 tx_pfc_tc3; / only for xgmac,
    pub /: *mut *mut u64 tx_pfc_tc4; / only for xgmac,
    pub /: *mut *mut u64 tx_pfc_tc5; / only for xgmac,
    pub /: *mut *mut u64 tx_pfc_tc6; / only for xgmac,
    pub /: *mut *mut u64 tx_pfc_tc7; / only for xgmac,
    pub /: *mut *mut u64 tx_ctrl; / only for xgmac,
    pub /: *mut *mut u64 tx_1731_pkts; / only for xgmac,
    pub /: *mut *mut u64 tx_1588_pkts; / only for xgmac,
    pub /: *mut *mut u64 rx_good_from_sw; / only for xgmac,
    pub /: *mut *mut u64 rx_bad_from_sw; / only for xgmac,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hns_mac_cb {
    pub dev: *mut device,
    pub dsaf_dev: *mut dsaf_device,
    pub priv: mac_priv,
    pub fw_port: *mut fwnode_handle,
    pub vaddr: *mut u8 __iomem,
    pub sys_ctl_vaddr: *mut u8 __iomem,
    pub serdes_vaddr: *mut u8 __iomem,
    pub serdes_ctrl: *mut regmap,
    pub cpld_ctrl: *mut regmap,
    pub mc_mask: [c_char; ETH_ALEN],
    pub cpld_ctrl_reg: u32,
    pub port_rst_off: u32,
    pub port_mode_off: u32,
    pub addr_entry_idx: [mac_entry_idx; DSAF_MAX_VM_NUM],
    pub sfp_prsnt: u8,
    pub cpld_led_value: u8,
    pub mac_id: u8,
    pub link: u8,
    pub half_duplex: u8,
    pub speed: u16,
    pub max_speed: u16,
    pub max_frm: u16,
    pub tx_pause_frm_time: u16,
    pub if_support: u32,
    pub txpkt_for_led: u64,
    pub rxpkt_for_led: u64,
    pub mac_type: hnae_port_type,
    pub media_type: hnae_media_type,
    pub phy_if: phy_interface_t,
    pub loop_mode: hnae_loop,
    pub phy_dev: *mut phy_device,
    pub hw_stats: mac_hw_stats,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mac_driver {
// init Mac when init nic or dsaf
    pub mac_drv): *mut *mut void (mac_init)(void,
// remove mac when remove nic or dsaf
    pub mac_drv): *mut *mut void (mac_free)(void,
// enable mac when enable nic or dsaf
    pub mode): *mut *mut *mut void (mac_enable)(void mac_drv, enum mac_commom_mode,
// disable mac when disable nic or dsaf
    pub mode): *mut *mut *mut void (mac_disable)(void mac_drv, enum mac_commom_mode,
// config mac address
    pub mac_addr): *const *const *const void (set_mac_addr)(void mac_drv, char,
// adjust mac mode of port,include speed and duplex
    pub full_duplex): u32,
// need adjust link
    pub duplex): c_int,
// config autoegotaite mode of port
    pub enable): *mut *mut *mut void (set_an_mode)(void mac_drv, u8,
// config loopbank mode
    pub enable): u8,
// config mtu
    pub newval): *mut *mut *mut void (config_max_frame_length)(void mac_drv, u16,
// config PAD and CRC enable
    pub newval): *mut *mut *mut void (config_pad_and_crc)(void mac_drv, u8,
// config tx pause time,if pause_time is zero,disable tx pause enable
    pub pause_time): *mut *mut *mut void (set_tx_auto_pause_frames)(void mac_drv, u16,
// config rx mode for promiscuous
    pub enable): *mut *mut *mut void (set_promiscuous)(void mac_drv, u8,
    pub tx_en): *mut *mut *mut void (mac_pausefrm_cfg)(void mac_drv, u32 rx_en, u32,
    pub enable): *mut *mut *mut void (autoneg_stat)(void mac_drv, u32,
    pub tx_en): *mut *mut *mut int (set_pause_enable)(void mac_drv, u32 rx_en, u32,
    pub tx_en): *mut *mut *mut *mut void (get_pause_enable)(void mac_drv, u32 rx_en, u32,
    pub link_stat): *mut *mut *mut void (get_link_status)(void mac_drv, u32,
// get the imporant regs
    pub data): *mut *mut *mut void (get_regs)(void mac_drv, void,
    pub (*get_regs_count)(void): *mut c_int,
// get strings name for ethtool statistic
    pub data): *mut *mut void (get_strings)(u32 stringset, u8,
// get the number of strings
    pub stringset): *mut *mut int (get_sset_count)(int,
// get the statistic by ethtools
    pub data): *mut *mut *mut void (get_ethtool_stats)(void mac_drv, u64,
// get mac information
    pub mac_info): *mut *mut *mut void (get_info)(void mac_drv, struct mac_info,
    pub mac_drv): *mut *mut void (update_stats)(void,
    pub mac_drv): *mut *mut int (wait_fifo_clean)(void,
    pub mac_mode: mac_mode,
    pub mac_id: u8,
    pub mac_cb: *mut hns_mac_cb,
    pub io_base: *mut u8 __iomem,
    pub twice*/: *mut *mut unsigned int mac_en_flg;/you'd better don't enable mac,
    pub virt_dev_num: c_uint,
    pub dev: *mut device,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mac_stats_string {
    pub desc: [c_char; ETH_GSTRING_LEN],
    pub offset: c_ulong,
}

extern "C" {
    pub fn hns_mac_init(dsaf_dev: *mut dsaf_device) -> c_int;
}
extern "C" {
    pub fn hns_mac_need_adjust_link(mac_cb: *mut hns_mac_cb, speed: c_int, duplex: c_int) -> bool;
}
extern "C" {
    pub fn hns_mac_get_link_status(mac_cb: *mut hns_mac_cb, link_status: *mut u32);
}
extern "C" {
    pub fn hns_mac_vm_config_bc_en(mac_cb: *mut hns_mac_cb, vm: u32, enable: bool) -> c_int;
}
extern "C" {
    pub fn hns_mac_start(mac_cb: *mut hns_mac_cb);
}
extern "C" {
    pub fn hns_mac_stop(mac_cb: *mut hns_mac_cb);
}
extern "C" {
    pub fn hns_mac_uninit(dsaf_dev: *mut dsaf_device);
}
extern "C" {
    pub fn hns_mac_adjust_link(mac_cb: *mut hns_mac_cb, speed: c_int, duplex: c_int);
}
extern "C" {
    pub fn hns_mac_reset(mac_cb: *mut hns_mac_cb);
}
extern "C" {
    pub fn hns_mac_get_autoneg(mac_cb: *mut hns_mac_cb, auto_neg: *mut u32);
}
extern "C" {
    pub fn hns_mac_get_pauseparam(mac_cb: *mut hns_mac_cb, rx_en: *mut u32, tx_en: *mut u32);
}
extern "C" {
    pub fn hns_mac_set_autoneg(mac_cb: *mut hns_mac_cb, enable: u8) -> c_int;
}
extern "C" {
    pub fn hns_mac_set_pauseparam(mac_cb: *mut hns_mac_cb, rx_en: u32, tx_en: u32) -> c_int;
}
extern "C" {
    pub fn hns_mac_set_mtu(mac_cb: *mut hns_mac_cb, new_mtu: u32, buf_size: u32) -> c_int;
}
extern "C" {
    pub fn hns_mac_update_stats(mac_cb: *mut hns_mac_cb);
}
extern "C" {
    pub fn hns_mac_get_stats(mac_cb: *mut hns_mac_cb, data: *mut u64);
}
extern "C" {
    pub fn hns_mac_get_strings(mac_cb: *mut hns_mac_cb, stringset: c_int, data: *mut u8);
}
extern "C" {
    pub fn hns_mac_get_sset_count(mac_cb: *mut hns_mac_cb, stringset: c_int) -> c_int;
}
extern "C" {
    pub fn hns_mac_get_regs(mac_cb: *mut hns_mac_cb, data: *mut c_void);
}
extern "C" {
    pub fn hns_mac_get_regs_count(mac_cb: *mut hns_mac_cb) -> c_int;
}
extern "C" {
    pub fn hns_set_led_opt(mac_cb: *mut hns_mac_cb);
}
extern "C" {
    pub fn hns_mac_set_promisc(mac_cb: *mut hns_mac_cb, en: u8);
}
extern "C" {
    pub fn hns_mac_clr_multicast(mac_cb: *mut hns_mac_cb, vfn: c_int) -> c_int;
}
extern "C" {
    pub fn hns_mac_enable(mac_cb: *mut hns_mac_cb, mode: mac_commom_mode);
}
extern "C" {
    pub fn hns_mac_disable(mac_cb: *mut hns_mac_cb, mode: mac_commom_mode);
}
extern "C" {
    pub fn hns_mac_wait_fifo_clean(mac_cb: *mut hns_mac_cb) -> c_int;
}
