//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/hisilicon/hibmcge/hbg_common.h
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


// SPDX-License-Identifier: GPL-2.0+
// Copyright (c) 2024 Hisilicon Limited.

pub const HBG_STATUS_DISABLE: c_uint = 0x0;
pub const HBG_STATUS_ENABLE: c_uint = 0x1;
pub const HBG_RX_SKIP1: c_uint = 0x00;
pub const HBG_RX_SKIP2: c_uint = 0x01;
pub const HBG_VECTOR_NUM: c_int = 4;
pub const HBG_PCU_CACHE_LINE_SIZE: c_int = 32;
pub const HBG_TX_TIMEOUT_BUF_LEN: c_int = 1024;
pub const HBG_RX_DESCR: c_uint = 0x01;
pub const HBG_NO_PHY: c_uint = 0xFF;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hbg_dir {
    HBG_DIR_TX = 1 << 0,
    HBG_DIR_RX = 1 << 1,
    HBG_DIR_TX_RX = HBG_DIR_TX | HBG_DIR_RX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hbg_tx_state {
    HBG_TX_STATE_COMPLETE = 0, /* clear state, must fix to 0 */
    HBG_TX_STATE_START,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hbg_nic_state {
    HBG_NIC_STATE_EVENT_HANDLING = 0,
    HBG_NIC_STATE_RESETTING,
    HBG_NIC_STATE_RESET_FAIL,
    HBG_NIC_STATE_NEED_RESET, /* trigger a reset in scheduled task */
    HBG_NIC_STATE_NP_LINK_FAIL,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hbg_reset_type {
    HBG_RESET_TYPE_NONE = 0,
    HBG_RESET_TYPE_FLR,
    HBG_RESET_TYPE_FUNCTION,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hbg_buffer {
    pub state: u32,
    pub state_dma: dma_addr_t,
    pub skb: *mut sk_buff,
    pub skb_dma: dma_addr_t,
    pub skb_len: u32,
    pub page: *mut page,
    pub page_addr: *mut c_void,
    pub page_dma: dma_addr_t,
    pub page_size: u32,
    pub page_offset: u32,
    pub dir: hbg_dir,
    pub ring: *mut hbg_ring,
    pub priv: *mut hbg_priv,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hbg_ring {
    pub queue: *mut hbg_buffer,
    pub queue_dma: dma_addr_t,
    pub head: u32,
    pub ntc: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hbg_hw_event_type {
    HBG_HW_EVENT_NONE = 0,
    HBG_HW_EVENT_INIT, /* driver is loading */
    HBG_HW_EVENT_RESET,
    HBG_HW_EVENT_CORE_RESET,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hbg_dev_specs {
    pub mac_id: u32,
    pub mac_addr: sockaddr,
    pub phy_addr: u32,
    pub mdio_frequency: u32,
    pub rx_fifo_num: u32,
    pub tx_fifo_num: u32,
    pub vlan_layers: u32,
    pub max_mtu: u32,
    pub min_mtu: u32,
    pub uc_mac_num: u32,
    pub max_frame_len: u32,
    pub rx_buf_size: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hbg_irq_info {
    pub name: *const c_char,
    pub mask: u32,
    pub re_enable: bool,
    pub need_print: bool,
    pub need_reset: bool,
    pub info): *const hbg_irq_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hbg_vector {
    pub name: [c_char; HBG_VECTOR_NUM][32],
    pub stats_array: *mut u64,
    pub info_array: *const hbg_irq_info,
    pub info_array_len: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hbg_mac {
    pub mdio_bus: *mut mii_bus,
    pub phydev: *mut phy_device,
    pub phy_addr: u8,
    pub speed: u32,
    pub duplex: u32,
    pub autoneg: u32,
    pub link_status: u32,
    pub pause_autoneg: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hbg_mac_table_entry {
    pub addr: [u8; ETH_ALEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hbg_mac_filter {
    pub mac_table: *mut hbg_mac_table_entry,
    pub table_max_len: u32,
    pub enabled: bool,
}

// saved for restore after rest
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hbg_user_def {
    pub pause_param: ethtool_pauseparam,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hbg_stats {
    pub rx_desc_drop: u64,
    pub rx_desc_l2_err_cnt: u64,
    pub rx_desc_pkt_len_err_cnt: u64,
    pub rx_desc_l3l4_err_cnt: u64,
    pub rx_desc_l3_wrong_head_cnt: u64,
    pub rx_desc_l3_csum_err_cnt: u64,
    pub rx_desc_l3_len_err_cnt: u64,
    pub rx_desc_l3_zero_ttl_cnt: u64,
    pub rx_desc_l3_other_cnt: u64,
    pub rx_desc_l4_err_cnt: u64,
    pub rx_desc_l4_wrong_head_cnt: u64,
    pub rx_desc_l4_len_err_cnt: u64,
    pub rx_desc_l4_csum_err_cnt: u64,
    pub rx_desc_l4_zero_port_num_cnt: u64,
    pub rx_desc_l4_other_cnt: u64,
    pub rx_desc_frag_cnt: u64,
    pub rx_desc_ip_ver_err_cnt: u64,
    pub rx_desc_ipv4_pkt_cnt: u64,
    pub rx_desc_ipv6_pkt_cnt: u64,
    pub rx_desc_no_ip_pkt_cnt: u64,
    pub rx_desc_ip_pkt_cnt: u64,
    pub rx_desc_tcp_pkt_cnt: u64,
    pub rx_desc_udp_pkt_cnt: u64,
    pub rx_desc_vlan_pkt_cnt: u64,
    pub rx_desc_icmp_pkt_cnt: u64,
    pub rx_desc_arp_pkt_cnt: u64,
    pub rx_desc_rarp_pkt_cnt: u64,
    pub rx_desc_multicast_pkt_cnt: u64,
    pub rx_desc_broadcast_pkt_cnt: u64,
    pub rx_desc_ipsec_pkt_cnt: u64,
    pub rx_desc_ip_opt_pkt_cnt: u64,
    pub rx_desc_key_not_match_cnt: u64,
    pub rx_octets_total_ok_cnt: u64,
    pub rx_uc_pkt_cnt: u64,
    pub rx_mc_pkt_cnt: u64,
    pub rx_bc_pkt_cnt: u64,
    pub rx_vlan_pkt_cnt: u64,
    pub rx_octets_bad_cnt: u64,
    pub rx_octets_total_filt_cnt: u64,
    pub rx_filt_pkt_cnt: u64,
    pub rx_trans_pkt_cnt: u64,
    pub rx_framesize_64: u64,
    pub rx_framesize_65_127: u64,
    pub rx_framesize_128_255: u64,
    pub rx_framesize_256_511: u64,
    pub rx_framesize_512_1023: u64,
    pub rx_framesize_1024_1518: u64,
    pub rx_framesize_bt_1518: u64,
    pub rx_fcs_error_cnt: u64,
    pub rx_data_error_cnt: u64,
    pub rx_align_error_cnt: u64,
    pub rx_pause_macctl_frame_cnt: u64,
    pub rx_unknown_macctl_frame_cnt: u64,
// crc ok, > max_frm_size, < 2max_frm_size
    pub rx_frame_long_err_cnt: u64,
// crc fail, > max_frm_size, < 2max_frm_size
    pub rx_jabber_err_cnt: u64,
// > 2max_frm_size
    pub rx_frame_very_long_err_cnt: u64,
// < 64byte, >= short_runts_thr
    pub rx_frame_runt_err_cnt: u64,
// < short_runts_thr
    pub rx_frame_short_err_cnt: u64,
// PCU: dropped when the RX FIFO is full.
    pub rx_overflow_cnt: u64,
// GMAC: the count of overflows of the RX FIFO
    pub rx_overrun_cnt: u64,
// PCU: the count of buffer alloc errors in RX
    pub rx_bufrq_err_cnt: u64,
// PCU: the count of write descriptor errors in RX
    pub rx_we_err_cnt: u64,
// GMAC: the count of pkts that contain PAD but length is not 64
    pub rx_lengthfield_err_cnt: u64,
    pub rx_fail_comma_cnt: u64,
    pub rx_dma_err_cnt: u64,
    pub rx_fifo_less_empty_thrsld_cnt: u64,
    pub tx_octets_total_ok_cnt: u64,
    pub tx_uc_pkt_cnt: u64,
    pub tx_mc_pkt_cnt: u64,
    pub tx_bc_pkt_cnt: u64,
    pub tx_vlan_pkt_cnt: u64,
    pub tx_octets_bad_cnt: u64,
    pub tx_trans_pkt_cnt: u64,
    pub tx_pause_frame_cnt: u64,
    pub tx_framesize_64: u64,
    pub tx_framesize_65_127: u64,
    pub tx_framesize_128_255: u64,
    pub tx_framesize_256_511: u64,
    pub tx_framesize_512_1023: u64,
    pub tx_framesize_1024_1518: u64,
    pub tx_framesize_bt_1518: u64,
// GMAC: the count of times that frames fail to be transmitted
// due to internal errors.
//
    pub tx_underrun_err_cnt: u64,
    pub tx_add_cs_fail_cnt: u64,
// PCU: the count of buffer free errors in TX
    pub tx_bufrl_err_cnt: u64,
    pub tx_crc_err_cnt: u64,
    pub tx_drop_cnt: u64,
    pub tx_excessive_length_drop_cnt: u64,
    pub tx_timeout_cnt: u64,
    pub tx_dma_err_cnt: u64,
    pub np_link_fail_cnt: u64,
    pub reset_fail_cnt: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hbg_priv {
    pub netdev: *mut net_device,
    pub pdev: *mut pci_dev,
    pub io_base: *mut u8 __iomem,
    pub dev_specs: hbg_dev_specs,
    pub state: c_ulong,
    pub mac: hbg_mac,
    pub vectors: hbg_vector,
    pub tx_ring: hbg_ring,
    pub rx_ring: hbg_ring,
    pub filter: hbg_mac_filter,
    pub reset_type: hbg_reset_type,
    pub user_def: hbg_user_def,
    pub stats: hbg_stats,
    pub last_update_stats_time: c_ulong,
    pub service_task: delayed_work,
}

extern "C" {
    pub fn hbg_err_reset_task_schedule(priv: *mut hbg_priv);
}
extern "C" {
    pub fn hbg_np_link_fail_task_schedule(priv: *mut hbg_priv);
}
