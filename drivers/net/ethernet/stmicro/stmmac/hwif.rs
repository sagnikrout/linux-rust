//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/stmicro/stmmac/hwif.h
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


// SPDX-License-Identifier: (GPL-2.0 OR MIT)
// Copyright (c) 2018 Synopsys, Inc. and/or its affiliates.
// stmmac HW Interface Callbacks

// Descriptors helpers
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stmmac_desc_ops {
// DMA RX descriptor ring initialization
    pub bfsize): u8 descriptor_mode, int end, int,
// DMA TX descriptor ring initialization
    pub end): *mut *mut *mut void (init_tx_desc)(struct dma_desc p, u8 descriptor_mode, int,
// Invoked by the xmit function to prepare the tx descriptor
    pub tot_pkt_len): bool ls, unsigned int,
    pub tcppayloadlen): c_uint,
// Set/get the owner of the descriptor
    pub p): *mut *mut void (set_tx_owner)(struct dma_desc,
// Clean the tx descriptor as soon as the tx irq is received
    pub descriptor_mode): *mut *mut *mut void (release_tx_desc)(struct dma_desc p, u8,
// Clear interrupt on tx frame completion. When this bit is
// set an interrupt happens as soon as the frame is transmitted
    pub p): *mut *mut void (set_tx_ic)(struct dma_desc,
// Get the tag of the descriptor
    pub p): *mut *mut u16 (get_rx_vlan_tci)(struct dma_desc,
// Get the valid status of descriptor
    pub p): *mut *mut bool (get_rx_vlan_valid)(struct dma_desc,
// Return the transmit status looking at the TDES1
    pub ioaddr): *mut *mut dma_desc p, void __iomem,
// Handle extra events on specific interrupts hw dependent
    pub disable_rx_ic): *mut *mut *mut void (set_rx_owner)(struct dma_desc p, int,
// Get the receive frame size
    pub rx_coe_type): *mut *mut *mut int (get_rx_frame_len)(struct dma_desc p, int,
// Return the reception status looking at the RDES1
    pub p): *mut dma_desc,
    pub p): *mut dma_extended_desc,
// Set tx timestamp enable bit
    pub p): *mut *mut void (enable_tx_timestamp) (struct dma_desc,
// get tx timestamp status
    pub p): *mut *mut int (get_tx_timestamp_status) (struct dma_desc,
// get timestamp value
    pub ts): *mut *mut *mut void (get_timestamp)(void desc, u32 ats, u64,
// get rx timestamp status
    pub ats): *mut *mut *mut *mut int (get_rx_timestamp_status)(void desc, void next_desc, u32,
// Display ring
    pub desc_size): dma_addr_t dma_rx_phy, unsigned int,
// set MSS via context descriptor
    pub mss): *mut *mut *mut void (set_mss)(struct dma_desc p, unsigned int,
// set descriptor skbuff address
    pub addr): *mut *mut *mut void (set_addr)(struct dma_desc p, dma_addr_t,
// clear descriptor
    pub p): *mut *mut void (clear)(struct dma_desc,
// RSS
    pub type): *mut pkt_hash_types,
    pub len): *mut *mut *mut void (get_rx_header_len)(struct dma_desc p, unsigned int,
    pub buf2_valid): *mut *mut *mut void (set_sec_addr)(struct dma_desc p, dma_addr_t addr, bool,
    pub sarc_type): *mut *mut *mut void (set_sarc)(struct dma_desc p, u32,
    pub inner_type): u32,
    pub type): *mut *mut *mut void (set_vlan)(struct dma_desc p, u32,
    pub nsec): *mut *mut *mut void (set_tbs)(struct dma_edesc p, u32 sec, u32,
}

// Specific DMA helpers
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stmmac_dma_ops {
// DMA core initialization
    pub ioaddr): *mut *mut int (reset)(void __iomem,
    pub dma_cfg): *mut *mut *mut void (init)(void __iomem ioaddr, struct stmmac_dma_cfg,
    pub chan): *mut *mut stmmac_dma_cfg dma_cfg, u32,
    pub chan): u32,
    pub chan): dma_addr_t phy, u32,
    pub chan): dma_addr_t phy, u32,
// Configure the AXI Bus Mode Register
    pub axi): *mut *mut *mut void (axi)(void __iomem ioaddr, struct stmmac_axi,
// Dump DMA registers
    pub reg_space): *mut u32,
    pub qmode): int fifosz, u8,
    pub qmode): int mode, u32 channel, int fifosz, u8,
// To track extra statistic (if supported)
    pub ioaddr): *mut void __iomem,
    pub chan): *mut *mut *mut void (enable_dma_transmission)(void __iomem ioaddr, u32,
    pub chan): *mut *mut *mut void (enable_dma_reception)(void __iomem ioaddr, u32,
    pub tx): u32 chan, bool rx, bool,
    pub tx): u32 chan, bool rx, bool,
    pub chan): u32,
    pub chan): u32,
    pub chan): u32,
    pub chan): u32,
    pub dir): *mut *mut stmmac_extra_stats x, u32 chan, u32,
// If supported then get the optional core features
    pub dma_cap): *mut dma_features,
// Program the HW RX Watchdog
    pub queue): u32 riwt, u32,
    pub chan): u32 len, u32,
    pub chan): u32 len, u32,
    pub chan): u32 tail_ptr, u32,
    pub chan): u32 tail_ptr, u32,
    pub chan): bool en, u32,
    pub qmode): u32 channel, u8,
    pub chan): int bfsize, u32,
    pub chan): bool en, u32,
    pub chan): bool en, u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum stmmac_lpi_mode {
    STMMAC_LPI_DISABLE,
    STMMAC_LPI_FORCED,
    STMMAC_LPI_TIMER,
}

// Helpers to program the MAC core
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stmmac_ops {
// Initialise any PCS instances
    pub priv): *mut *mut int (pcs_init)(struct stmmac_priv,
// MAC core initialization
    pub dev): *mut *mut *mut void (core_init)(struct mac_device_info hw, struct net_device,
// Update MAC capabilities
    pub priv): *mut *mut void (update_caps)(struct stmmac_priv,
// Change the interrupt enable setting. Enable takes precedence.
    pub enable): *mut *mut *mut void (irq_modify)(struct mac_device_info hw, u32 disable, u32,
// Enable the MAC RX/TX
    pub enable): *mut *mut *mut void (set_mac)(void __iomem ioaddr, bool,
// Enable and verify that the IPC module is supported
    pub hw): *mut *mut int (rx_ipc)(struct mac_device_info,
// Enable RX Queues
    pub queue): *mut *mut *mut void (rx_queue_enable)(struct mac_device_info hw, u8 mode, u32,
// RX Queues Priority
    pub queue): *mut *mut *mut void (rx_queue_prio)(struct mac_device_info hw, u32 prio, u32,
// TX Queues Priority
    pub queue): *mut *mut *mut void (tx_queue_prio)(struct mac_device_info hw, u32 prio, u32,
// RX Queues Routing
    pub queue): u32,
// Program RX Algorithms
    pub rx_alg): *mut *mut *mut void (prog_mtl_rx_algorithms)(struct mac_device_info hw, u32,
// Program TX Algorithms
    pub tx_alg): *mut *mut *mut void (prog_mtl_tx_algorithms)(struct mac_device_info hw, u32,
// Set MTL TX queues weight
    pub queue): u32 weight, u32,
// RX MTL queue to RX dma mapping
    pub chan): *mut *mut *mut void (map_mtl_to_dma)(struct mac_device_info hw, u32 queue, u32,
// Configure AV Algorithm
    pub queue): u32 low_credit, u32,
// Dump MAC registers
    pub reg_space): *mut *mut *mut void (dump_regs)(struct mac_device_info hw, u32,
// Handle extra events on specific interrupts hw dependent
    pub x): *mut stmmac_extra_stats,
// Handle MTL interrupts
    pub chan): *mut *mut mac_device_info hw, u32,
// Multicast filter setting
    pub dev): *mut *mut *mut void (set_filter)(struct mac_device_info hw, struct net_device,
// Flow control setting
    pub tx_cnt): unsigned int fc, unsigned int pause_time, u8,
// Set power management mode (e.g. magic frame)
    pub mode): *mut *mut *mut void (pmt)(struct mac_device_info hw, unsigned long,
// Set/Get Unicast MAC addresses
    pub reg_n): c_uint,
    pub reg_n): c_uint,
    pub et): bool en_tx_lpi_clockgating, u32,
    pub tw): *mut *mut *mut void (set_eee_timer)(struct mac_device_info hw, int ls, int,
    pub link): *mut *mut *mut void (set_eee_pls)(struct mac_device_info hw, int,
    pub tx_queues): u32,
// PCS calls
    pub srgmi_ral): bool,
// Safety Features
    pub safety_cfg): *mut stmmac_safety_feature_cfg,
    pub stats): *mut stmmac_safety_stats,
    pub desc): *const *const int index, unsigned long count, char,
// Flexible RX Parser
    pub count): c_uint,
// Flexible PPS
    pub systime_flags): u32 sub_second_inc, u32,
// Loopback for selftests
    pub enable): *mut *mut *mut void (set_mac_loopback)(void __iomem ioaddr, bool,
// RSS
    pub num_rxq): *mut *mut stmmac_rss cfg, u32,
// TX Timestamp
    pub ts): *mut *mut *mut int (get_mac_tx_timestamp)(struct mac_device_info hw, u64,
// Source Address Insertion / Replacement
    pub val): *mut *mut *mut void (sarc_configure)(void __iomem ioaddr, int,
// Filtering
    pub match): u32,
    pub match): u32,
    pub addr): *mut *mut *mut void (set_arp_offload)(struct mac_device_info hw, bool en, u32,
    pub pclass): u32,
}

// PTP and HW Timer helpers
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stmmac_hwtimestamp {
    pub data): *mut *mut *mut void (config_hw_tstamping) (void __iomem ioaddr, u32,
    pub ssinc): *mut int gmac4, u32,
    pub nsec): *mut *mut *mut int (init_systime) (void __iomem ioaddr, u32 sec, u32,
    pub addend): *mut *mut *mut int (config_addend) (void __iomem ioaddr, u32,
    pub gmac4): int add_sub, int,
    pub systime): *mut *mut *mut void (get_systime) (void __iomem ioaddr, u64,
    pub ptp_time): *mut *mut *mut void (get_ptptime)(void __iomem ioaddr, u64,
    pub priv): *mut *mut void (timestamp_interrupt)(struct stmmac_priv,
    pub priv): *mut *mut void (hwtstamp_correct_latency)(struct stmmac_priv,
}

// Helpers to manage the descriptors for chain and ring modes
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stmmac_mode_ops {
    pub extend_desc): c_uint,
    pub enh_desc): *mut *mut bool (is_jumbo_frm)(unsigned int len, bool,
    pub csum): c_int,
    pub mtu): *mut *mut int (set_16kib_bfsize)(int,
    pub p): *mut *mut void (init_desc3)(struct dma_desc,
    pub p): *mut *mut *mut void (refill_desc3)(struct stmmac_rx_queue rx_q, struct dma_desc,
    pub p): *mut *mut *mut void (clean_desc3)(struct stmmac_tx_queue tx_q, struct dma_desc,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stmmac_tc_ops {
    pub priv): *mut *mut int (init)(struct stmmac_priv,
    pub cls): *mut tc_cls_u32_offload,
    pub qopt): *mut tc_cbs_qopt_offload,
    pub cls): *mut flow_cls_offload,
    pub qopt): *mut tc_taprio_qopt_offload,
    pub qopt): *mut tc_etf_qopt_offload,
    pub base): *mut tc_query_caps_base,
    pub qopt): *mut tc_mqprio_qopt_offload,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stmmac_mmc_ops {
    pub mode): *mut *mut *mut void (ctrl)(void __iomem ioaddr, unsigned int,
    pub ioaddr): *mut *mut void (intr_all_mask)(void __iomem,
    pub mmc): *mut *mut *mut void (read)(void __iomem ioaddr, struct stmmac_counters,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stmmac_est_ops {
    pub ptp_rate): c_uint,
    pub txqcnt): *mut *mut stmmac_extra_stats x, u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stmmac_vlan_ops {
// VLAN
    pub is_double): u16 perfect_match, bool,
    pub type): *mut *mut *mut void (enable_vlan)(struct mac_device_info hw, u32,
    pub skb): *mut sk_buff,
    pub hw): *mut *mut void (set_hw_vlan_mode)(struct mac_device_info,
    pub vid): __be16 proto, u16,
    pub vid): __be16 proto, u16,
    pub hw): *mut mac_device_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stmmac_regs_off {
    pub fpe_reg: *const stmmac_fpe_reg,
    pub ptp_off: u32,
    pub mmc_off: u32,
    pub est_off: u32,
}

pub const GMAC_VERSION: c_uint = 0x00000020	/* GMAC CORE Version */;
pub const GMAC4_VERSION: c_uint = 0x00000110	/* GMAC4+ CORE Version */;
extern "C" {
    pub fn stmmac_reset(priv: *mut stmmac_priv) -> c_int;
}
extern "C" {
    pub fn stmmac_hwif_init(priv: *mut stmmac_priv) -> c_int;
}
