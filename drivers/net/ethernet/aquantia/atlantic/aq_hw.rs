//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/aquantia/atlantic/aq_hw.h
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
// Atlantic Network Driver
//
// Copyright (C) 2014-2019 aQuantia Corporation
// Copyright (C) 2019-2020 Marvell International Ltd.
//
// File aq_hw.h: Declaration of abstract interface for NIC hardware specific
// functions.
//

pub const AQ_HW_TXD_CTL_TS_EN: c_uint = 0x40000000U;
pub const AQ_HW_TXD_CTL_TS_TSG0: c_uint = 0x80000000U;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum aq_tc_mode {
    AQ_TC_MODE_INVALID = -1,
    AQ_TC_MODE_8TCS,
    AQ_TC_MODE_4TCS,
}

pub const AQ_RX_QUEUE_NOT_ASSIGNED: c_uint = 0xFFU;
pub const AQ_FRAC_PER_NS: c_uint = 0x100000000LL;

// Used for rate to Mbps conversion

// NIC H/W capabilities
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aq_hw_caps_s {
    pub hw_features: u64,
    pub link_speed_msk: u64,
    pub hw_priv_flags: c_uint,
    pub media_type: u32,
    pub rxds_max: u32,
    pub txds_max: u32,
    pub rxds_min: u32,
    pub txds_min: u32,
    pub txhwb_alignment: u32,
    pub irq_mask: u32,
    pub vecs: u32,
    pub mtu: u32,
    pub mac_regs_count: u32,
    pub hw_alive_check_addr: u32,
    pub msix_irqs: u8,
    pub tcs_max: u8,
    pub rxd_alignment: u8,
    pub rxd_size: u8,
    pub txd_alignment: u8,
    pub txd_size: u8,
    pub tx_rings: u8,
    pub rx_rings: u8,
    pub flow_control: bool,
    pub is_64_dma: bool,
    pub op64bit: bool,
    pub quirks: u32,
    pub priv_data_len: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aq_hw_link_status_s {
    pub mbps: c_uint,
    pub full_duplex: bool,
    pub lp_link_speed_msk: u32,
    pub lp_flow_control: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aq_stats_s {
    pub brc: u64,
    pub btc: u64,
    pub uprc: u64,
    pub mprc: u64,
    pub bprc: u64,
    pub erpt: u64,
    pub uptc: u64,
    pub mptc: u64,
    pub bptc: u64,
    pub erpr: u64,
    pub mbtc: u64,
    pub bbtc: u64,
    pub mbrc: u64,
    pub bbrc: u64,
    pub ubrc: u64,
    pub ubtc: u64,
    pub dpc: u64,
    pub dma_pkt_rc: u64,
    pub dma_pkt_tc: u64,
    pub dma_oct_rc: u64,
    pub dma_oct_tc: u64,
}

pub const AQ_HW_FLAG_STARTED: c_uint = 0x00000004U;
pub const AQ_HW_FLAG_STOPPING: c_uint = 0x00000008U;
pub const AQ_HW_FLAG_RESETTING: c_uint = 0x00000010U;
pub const AQ_HW_FLAG_CLOSING: c_uint = 0x00000020U;
pub const AQ_HW_PTP_AVAILABLE: c_uint = 0x01000000U;
pub const AQ_HW_LINK_DOWN: c_uint = 0x04000000U;
pub const AQ_HW_FLAG_ERR_UNPLUG: c_uint = 0x40000000U;
pub const AQ_HW_FLAG_ERR_HW: c_uint = 0x80000000U;

pub const AQ_HW_LED_BLINK: c_uint = 0x2U;
pub const AQ_HW_LED_DEFAULT: c_uint = 0x0U;
pub const AQ_HW_MEDIA_DETECT_CNT: c_int = 6000;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum aq_priv_flags {
    AQ_HW_LOOPBACK_DMA_SYS,
    AQ_HW_LOOPBACK_PKT_SYS,
    AQ_HW_LOOPBACK_DMA_NET,
    AQ_HW_LOOPBACK_PHYINT_SYS,
    AQ_HW_LOOPBACK_PHYEXT_SYS,
}

pub const ATL_TSG_CLOCK_SEL_0: c_int = 0;
pub const ATL_TSG_CLOCK_SEL_1: c_int = 1;

pub const ATL_HW_CHIP_MIPS: c_uint = 0x00000001U;
pub const ATL_HW_CHIP_TPO2: c_uint = 0x00000002U;
pub const ATL_HW_CHIP_RPF2: c_uint = 0x00000004U;
pub const ATL_HW_CHIP_MPI_AQ: c_uint = 0x00000010U;
pub const ATL_HW_CHIP_ATLANTIC: c_uint = 0x00800000U;
pub const ATL_HW_CHIP_REVISION_A0: c_uint = 0x01000000U;
pub const ATL_HW_CHIP_REVISION_B0: c_uint = 0x02000000U;
pub const ATL_HW_CHIP_REVISION_B1: c_uint = 0x04000000U;
pub const ATL_HW_CHIP_ANTIGUA: c_uint = 0x08000000U;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aq_hw_s {
    pub flags: core::sync::atomic::AtomicI32,
    pub rbl_enabled:1: u8,
    pub aq_nic_cfg: *mut aq_nic_cfg_s,
    pub aq_fw_ops: *const aq_fw_ops,
    pub mmio: *mut void __iomem,
    pub aq_link_status: aq_hw_link_status_s,
    pub mbox: hw_atl_utils_mbox,
    pub last_stats: hw_atl_stats_s,
    pub curr_stats: aq_stats_s,
    pub speed: u64,
    pub itr_tx: u32,
    pub itr_rx: u32,
    pub chip_features: c_uint,
    pub fw_ver_actual: u32,
    pub dpc: core::sync::atomic::AtomicI32,
    pub mbox_addr: u32,
    pub rpc_addr: u32,
    pub settings_addr: u32,
    pub rpc_tid: u32,
    pub rpc: hw_atl_utils_fw_rpc,
    pub ptp_clk_offset: i64,
    pub clk_select: i8,
    pub phy_id: u16,
    pub priv: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aq_hw_ops {
    pub frags): c_uint,
    pub aq_ring): *mut aq_ring_s,
    pub sw_tail_old): c_uint,
    pub aq_ring): *mut aq_ring_s,
    pub mac_addr): *const *const *const int (hw_set_mac_address)(struct aq_hw_s self, u8,
    pub self): *mut *mut int (hw_soft_reset)(struct aq_hw_s,
    pub fw_ops): *const aq_fw_ops,
    pub self): *mut *mut int (hw_reset)(struct aq_hw_s,
    pub mac_addr): *const *const *const int (hw_init)(struct aq_hw_s self, u8,
    pub self): *mut *mut int (hw_start)(struct aq_hw_s,
    pub self): *mut *mut int (hw_stop)(struct aq_hw_s,
    pub aq_ring_param): *mut aq_ring_param_s,
    pub aq_ring): *mut aq_ring_s,
    pub aq_ring): *mut aq_ring_s,
    pub aq_ring_param): *mut aq_ring_param_s,
    pub aq_ring): *mut aq_ring_s,
    pub aq_ring): *mut aq_ring_s,
    pub mask): *mut *mut *mut int (hw_irq_enable)(struct aq_hw_s self, u64,
    pub mask): *mut *mut *mut int (hw_irq_disable)(struct aq_hw_s self, u64,
    pub mask): *mut *mut *mut int (hw_irq_read)(struct aq_hw_s self, u64,
    pub packet_filter): c_uint,
    pub data): *mut aq_rx_filter_l3l4,
    pub data): *mut aq_rx_filter_l3l4,
    pub data): *mut aq_rx_filter_l2,
    pub data): *mut aq_rx_filter_l2,
    pub aq_vlans): *mut aq_rx_filter_vlan,
    pub enable): *mut *mut *mut int (hw_filter_vlan_ctrl)(struct aq_hw_s self, bool,
    pub count): u32,
    pub self): *mut *mut int (hw_interrupt_moderation_set)(struct aq_hw_s,
    pub rss_params): *mut aq_rss_parameters,
    pub rss_params): *mut aq_rss_parameters,
    pub self): *mut *mut int (hw_tc_rate_limit_set)(struct aq_hw_s,
    pub regs_buff): *mut u32,
    pub self): *mut *mut *mut aq_stats_s (hw_get_hw_stats)(aq_hw_s,
    pub self): *mut *mut u32 (hw_get_fw_version)(struct aq_hw_s,
    pub aq_nic_cfg): *mut aq_nic_cfg_s,
    pub aq_ring): *mut aq_ring_s,
    pub ring): *mut aq_ring_s,
    pub stamp): *mut *mut *mut void (hw_get_ptp_ts)(struct aq_hw_s self, u64,
    pub delta): *mut *mut *mut int (hw_adj_clock_freq)(struct aq_hw_s self, s32,
    pub delta): *mut *mut *mut int (hw_adj_sys_clock)(struct aq_hw_s self, s64,
    pub ts): *mut *mut *mut int (hw_set_sys_clock)(struct aq_hw_s self, u64 time, u64,
    pub time): *mut *mut *mut int (hw_ts_to_sys_clock)(struct aq_hw_s self, u64 ts, u64,
    pub hightime): u32 period, u32,
    pub enable): u32 channel, int,
    pub enable): c_int,
    pub ts): *mut *mut *mut int (hw_get_sync_ts)(struct aq_hw_s self, u64,
    pub timestamp): *mut u64,
    pub timestamp): *mut u64,
    pub ring): *mut *mut u64 (hw_ring_tx_ptp_get_ts)(struct aq_ring_s,
    pub aq_ring): *mut aq_ring_s,
    pub aq_ring): *mut aq_ring_s,
    pub self): *mut *mut u32 (hw_get_clk_sel)(struct aq_hw_s,
    pub tc): *mut *mut *mut int (hw_set_fc)(struct aq_hw_s self, u32 fc, u32,
    pub enable): *mut *mut *mut int (hw_set_loopback)(struct aq_hw_s self, u32 mode, bool,
    pub temp): *mut *mut *mut int (hw_get_mac_temp)(struct aq_hw_s self, u32,
    pub data): *mut u8 reg_start_addr, int len, u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aq_fw_ops {
    pub self): *mut *mut int (init)(struct aq_hw_s,
    pub self): *mut *mut int (deinit)(struct aq_hw_s,
    pub self): *mut *mut int (reset)(struct aq_hw_s,
    pub self): *mut *mut int (renegotiate)(struct aq_hw_s,
    pub mac): *mut *mut *mut int (get_mac_permanent)(struct aq_hw_s self, u8,
    pub speed): *mut *mut *mut int (set_link_speed)(struct aq_hw_s self, u32,
    pub state): hal_atl_utils_fw_state_e,
    pub self): *mut *mut int (update_link_status)(struct aq_hw_s,
    pub self): *mut *mut int (update_stats)(struct aq_hw_s,
    pub temp): *mut *mut *mut int (get_mac_temp)(struct aq_hw_s self, int,
    pub temp): *mut *mut *mut int (get_phy_temp)(struct aq_hw_s self, int,
    pub fcmode): *mut *mut *mut u32 (get_flow_control)(struct aq_hw_s self, u32,
    pub self): *mut *mut int (set_flow_control)(struct aq_hw_s,
    pub mode): *mut *mut *mut int (led_control)(struct aq_hw_s self, u32,
    pub enable): *mut *mut *mut int (set_phyloopback)(struct aq_hw_s self, u32 mode, bool,
    pub mac): *const u8,
    pub size): usize,
    pub enable): *mut *mut *mut void (enable_ptp)(struct aq_hw_s self, int,
    pub adj): *mut *mut *mut void (adjust_ptp)(struct aq_hw_s self, uint64_t,
    pub speed): *mut *mut *mut int (set_eee_rate)(struct aq_hw_s self, u32,
    pub supported_rates): *mut u32,
    pub counter): *mut *mut *mut int (set_downshift)(struct aq_hw_s self, u32,
    pub enable): *mut *mut *mut int (set_media_detect)(struct aq_hw_s self, bool,
    pub self): *mut *mut u32 (get_link_capabilities)(struct aq_hw_s,
    pub resp): *mut macsec_msg_fw_response,
    pub data): *mut u8 reg_start_addr, int len, u8,
}
