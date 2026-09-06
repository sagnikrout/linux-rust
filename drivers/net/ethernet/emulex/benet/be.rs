//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/emulex/benet/be.h
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
//
// Copyright (C) 2005 - 2016 Broadcom
// All rights reserved.
//
// Contact Information:
// linux-drivers@emulex.com
//
// Emulex
// 3333 Susan Street
// Costa Mesa, CA 92626
//

pub const BE_VENDOR_ID: c_uint = 0x19a2;
pub const EMULEX_VENDOR_ID: c_uint = 0x10df;
pub const BE_DEVICE_ID1: c_uint = 0x211;
pub const BE_DEVICE_ID2: c_uint = 0x221;
pub const OC_DEVICE_ID1: c_uint = 0x700	/* Device Id for BE2 cards */;
pub const OC_DEVICE_ID2: c_uint = 0x710	/* Device Id for BE3 cards */;
pub const OC_DEVICE_ID3: c_uint = 0xe220	/* Device id for Lancer cards */;
pub const OC_DEVICE_ID4: c_uint = 0xe228   /* Device id for VF in Lancer */;
pub const OC_DEVICE_ID5: c_uint = 0x720	/* Device Id for Skyhawk cards */;
pub const OC_DEVICE_ID6: c_uint = 0x728   /* Device id for VF in SkyHawk */;
pub const OC_SUBSYS_DEVICE_ID1: c_uint = 0xE602;
pub const OC_SUBSYS_DEVICE_ID2: c_uint = 0xE642;
pub const OC_SUBSYS_DEVICE_ID3: c_uint = 0xE612;
pub const OC_SUBSYS_DEVICE_ID4: c_uint = 0xE652;
// Number of bytes of an RX frame that are copied to skb->data

// allocate extra space to allow tunneling decapsulation without head reallocation
pub const BE_RX_SKB_ALLOC_SIZE: c_int = 256;
pub const BE_MAX_JUMBO_FRAME_SIZE: c_int = 9018;
pub const BE_MIN_MTU: c_int = 256;

// Accommodate for QnQ configurations where VLAN insertion is enabled in HW

pub const BE_NUM_VLANS_SUPPORTED: c_int = 64;

pub const BE_MAX_TX_FRAG_COUNT: c_int = 30;
pub const EVNT_Q_LEN: c_int = 1024;
pub const TX_Q_LEN: c_int = 2048;
pub const TX_CQ_LEN: c_int = 1024;

pub const RX_CQ_LEN: c_int = 1024;

pub const MCC_CQ_LEN: c_int = 256;
pub const BE2_MAX_RSS_QS: c_int = 4;
pub const BE3_MAX_RSS_QS: c_int = 16;
pub const BE3_MAX_TX_QS: c_int = 16;
pub const BE3_MAX_EVT_QS: c_int = 16;
pub const BE3_SRIOV_MAX_EVT_QS: c_int = 8;

// and at least 1 is granted to either
// SURF/DPDK
//
pub const MAX_PORT_RSS_TABLES: c_int = 15;
pub const MAX_NIC_FUNCS: c_int = 16;
pub const MAX_RX_QS: c_int = 32;
pub const MAX_EVT_QS: c_int = 32;
pub const MAX_TX_QS: c_int = 32;
pub const MAX_ROCE_EQS: c_int = 5;
pub const MAX_MSIX_VECTORS: c_int = 32;
pub const MIN_MSIX_VECTORS: c_int = 1;

pub const FW_VER_LEN: c_int = 32;

pub const RSS_INDIR_TABLE_LEN: c_int = 128;
pub const RSS_HASH_KEY_LEN: c_int = 40;
pub const BE_UNKNOWN_PHY_STATE: c_uint = 0xFF;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_dma_mem {
    pub va: *mut c_void,
    pub dma: dma_addr_t,
    pub size: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_queue_info {
    pub len: u32,
    pub /: *mut *mut u32 entry_size; / Size of an element in the queue,
    pub head: u32 tail,,
    pub /: *mut *mut atomic_t used; / Number of valid elements in the queue,
    pub id: u32,
    pub dma_mem: be_dma_mem,
    pub created: bool,
}

// index = MODULO((*index + val), limit);
// index = MODULO((*index + 1), limit);
// index = MODULO((*index - 1), limit);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_eq_obj {
    pub q: be_queue_info,
    pub desc: [c_char; 32],
    pub adapter: *mut be_adapter,
    pub napi: napi_struct,
    pub /: *mut *mut u8 idx; / array index,
    pub msix_idx: u8,
    pub spurious_intr: u16,
    pub affinity_mask: cpumask_var_t,
    pub ____cacheline_aligned_in_smp: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_aic_obj {
    pub /: *mut *mut u32 min_eqd; / in usecs,
    pub /: *mut *mut u32 max_eqd; / in usecs,
    pub /: *mut *mut u32 prev_eqd; / in usecs,
    pub /: *mut *mut u32 et_eqd; / configured val when aic is off,
    pub jiffies: c_ulong,
    pub /: *mut *mut u64 rx_pkts_prev; / Used to calculate RX pps,
    pub /: *mut *mut u64 tx_reqs_prev; / Used to calculate TX pps,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_mcc_obj {
    pub q: be_queue_info,
    pub cq: be_queue_info,
    pub rearm_cq: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_tx_stats {
    pub tx_bytes: u64,
    pub tx_pkts: u64,
    pub tx_vxlan_offload_pkts: u64,
    pub tx_reqs: u64,
    pub tx_compl: u64,
    pub tx_stops: u32,
    pub /: *mut *mut u32 tx_drv_drops; / pkts dropped by driver,
// the error counters are described in be_ethtool.c
    pub tx_hdr_parse_err: u32,
    pub tx_dma_err: u32,
    pub tx_tso_err: u32,
    pub tx_spoof_check_err: u32,
    pub tx_qinq_err: u32,
    pub tx_internal_parity_err: u32,
    pub tx_sge_err: u32,
    pub sync: u64_stats_sync,
    pub sync_compl: u64_stats_sync,
}

// Structure to hold some data of interest obtained from a TX CQE
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_tx_compl_info {
    pub /: *mut *mut u8 status; / Completion status,
    pub /: *mut *mut u16 end_index; / Completed TXQ Index,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_tx_obj {
    pub db_offset: u32,
    pub txcp: be_tx_compl_info,
    pub q: be_queue_info,
    pub cq: be_queue_info,
// Remember the skbs that were transmitted
    pub sent_skb_list: [*mut sk_buff; TX_Q_LEN],
    pub stats: be_tx_stats,
    pub /: *mut *mut u16 pend_wrb_cnt; / Number of WRBs yet to be given to HW,
    pub /: *mut *mut u16 last_req_wrb_cnt; / wrb cnt of the last req in the Q,
    pub /: *mut *mut u16 last_req_hdr; / index of the last req's hdr-wrb,
    pub ____cacheline_aligned_in_smp: },
// Struct to remember the pages posted for rx frags
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_rx_page_info {
    pub page: *mut page,
// set to page-addr for last frag of the page & frag-addr otherwise
    pub page_offset: u16,
    pub /: *mut *mut bool last_frag; / last frag of the page,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_rx_stats {
    pub rx_bytes: u64,
    pub rx_pkts: u64,
    pub rx_vxlan_offload_pkts: u64,
    pub /: *mut *mut u32 rx_drops_no_skbs; / skb allocation errors,
    pub /: *mut *mut u32 rx_drops_no_frags; / HW has no fetched frags,
    pub /: *mut *mut u32 rx_post_fail; / page post alloc failures,
    pub rx_compl: u32,
    pub rx_mcast_pkts: u32,
    pub /: *mut *mut u32 rx_compl_err; / completions with err set,
    pub sync: u64_stats_sync,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_rx_compl_info {
    pub rss_hash: u32,
    pub vlan_tag: u16,
    pub pkt_size: u16,
    pub port: u16,
    pub vlanf: u8,
    pub num_rcvd: u8,
    pub err: u8,
    pub ipf: u8,
    pub tcpf: u8,
    pub udpf: u8,
    pub ip_csum: u8,
    pub l4_csum: u8,
    pub ipv6: u8,
    pub qnq: u8,
    pub pkt_type: u8,
    pub ip_frag: u8,
    pub tunneled: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_rx_obj {
    pub adapter: *mut be_adapter,
    pub q: be_queue_info,
    pub cq: be_queue_info,
    pub rxcp: be_rx_compl_info,
    pub page_info_tbl: [be_rx_page_info; RX_Q_LEN],
    pub stats: be_rx_stats,
    pub rss_id: u8,
    pub /: *mut *mut bool rx_post_starved; / Zero rx frags have been posted to BE,
    pub ____cacheline_aligned_in_smp: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_drv_stats {
    pub eth_red_drops: u32,
    pub dma_map_errors: u32,
    pub rx_drops_no_pbuf: u32,
    pub rx_drops_no_txpb: u32,
    pub rx_drops_no_erx_descr: u32,
    pub rx_drops_no_tpre_descr: u32,
    pub rx_drops_too_many_frags: u32,
    pub forwarded_packets: u32,
    pub rx_drops_mtu: u32,
    pub rx_crc_errors: u32,
    pub rx_alignment_symbol_errors: u32,
    pub rx_pause_frames: u32,
    pub rx_priority_pause_frames: u32,
    pub rx_control_frames: u32,
    pub rx_in_range_errors: u32,
    pub rx_out_range_errors: u32,
    pub rx_frame_too_long: u32,
    pub rx_address_filtered: u32,
    pub rx_dropped_too_small: u32,
    pub rx_dropped_too_short: u32,
    pub rx_dropped_header_too_small: u32,
    pub rx_dropped_tcp_length: u32,
    pub rx_dropped_runt: u32,
    pub rx_ip_checksum_errs: u32,
    pub rx_tcp_checksum_errs: u32,
    pub rx_udp_checksum_errs: u32,
    pub tx_pauseframes: u32,
    pub tx_priority_pauseframes: u32,
    pub tx_controlframes: u32,
    pub rxpp_fifo_overflow_drop: u32,
    pub rx_input_fifo_overflow_drop: u32,
    pub pmem_fifo_overflow_drop: u32,
    pub jabber_events: u32,
    pub rx_roce_bytes_lsd: u32,
    pub rx_roce_bytes_msd: u32,
    pub rx_roce_frames: u32,
    pub roce_drops_payload_len: u32,
    pub roce_drops_crc: u32,
}

// A vlan-id of 0xFFFF must be used to clear transparent vlan-tagging
pub const BE_RESET_VLAN_TAG_ID: c_uint = 0xFFFF;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_vf_cfg {
    pub mac_addr: [c_uchar; ETH_ALEN],
    pub if_handle: c_int,
    pub pmac_id: c_int,
    pub vlan_tag: u16,
    pub tx_rate: u32,
    pub plink_tracking: u32,
    pub privileges: u32,
    pub spoofchk: bool,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vf_state {
    ENABLED = 0,
    ASSIGNED = 1
}

pub const BE_UC_PMAC_COUNT: c_int = 30;
pub const BE_VF_UC_PMAC_COUNT: c_int = 2;
pub const MAX_ERR_RECOVERY_RETRY_COUNT: c_int = 3;
pub const ERR_DETECTION_DELAY: c_int = 1000;
// Ethtool set_dump flags
pub const LANCER_INITIATE_FW_DUMP: c_uint = 0x1;
pub const LANCER_DELETE_FW_DUMP: c_uint = 0x2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct phy_info {
// From SFF-8472 spec
pub const SFP_VENDOR_NAME_LEN: c_int = 17;
    pub transceiver: u8,
    pub autoneg: u8,
    pub fc_autoneg: u8,
    pub port_type: u8,
    pub phy_type: u16,
    pub interface_type: u16,
    pub misc_params: u32,
    pub auto_speeds_supported: u16,
    pub fixed_speeds_supported: u16,
    pub link_speed: c_int,
    pub advertising: u32,
    pub supported: u32,
    pub cable_type: u8,
    pub vendor_name: [u8; SFP_VENDOR_NAME_LEN],
    pub vendor_pn: [u8; SFP_VENDOR_NAME_LEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_resources {
    pub /: *mut *mut u16 max_vfs; / Total VFs "really" supported by FW/HW,
    pub max_mcast_mac: u16,
    pub max_tx_qs: u16,
    pub max_rss_qs: u16,
    pub max_rx_qs: u16,
    pub max_cq_count: u16,
    pub /: *mut *mut u16 max_uc_mac; / Max UC MACs programmable,
    pub /: *mut *mut u16 max_vlans; / Number of vlans supported,
    pub max_iface_count: u16,
    pub max_mcc_count: u16,
    pub max_evt_qs: u16,
    pub /: *mut *mut u16 max_nic_evt_qs; / NIC's share of evt qs,
    pub if_cap_flags: u32,
    pub /: *mut *mut u32 vf_if_cap_flags; / VF if capability flags,
    pub flags: u32,
// Calculated PF Pool's share of RSS Tables. This is not enforced by
// the FW, but is a self-imposed driver limitation.
//
    pub max_rss_tables: u16,
}

// These are port-wide values
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_port_resources {
    pub max_vfs: u16,
    pub nic_pfs: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rss_info {
    pub rsstable: [u8; RSS_INDIR_TABLE_LEN],
    pub rss_queue: [u8; RSS_INDIR_TABLE_LEN],
    pub rss_hkey: [u8; RSS_HASH_KEY_LEN],
    pub rss_flags: u64,
}

pub const BE_INVALID_DIE_TEMP: c_uint = 0xFF;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_hwmon {
    pub hwmon_dev: *mut device,
    pub /: *mut *mut u8 be_on_die_temp; / Unit: millidegree Celsius,
}

// Macros to read/write the 'features' word of be_wrb_params structure.
//

// Feature/offload bits
// The structure below provides a HW-agnostic abstraction of WRB params
// retrieved from a TX skb. This is in turn passed to chip specific routines
// during transmit, to set the corresponding params in the WRB.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_wrb_params {
    pub /: *mut *mut u32 features; / Feature bits,
    pub /: *mut *mut u16 vlan_tag; / VLAN tag,
    pub /: *mut *mut u16 lso_mss; / MSS for LSO,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_eth_addr {
    pub mac: [c_uchar; ETH_ALEN],
}

pub const ERR_RECOVERY_MAX_RETRY_COUNT: c_int = 3;

// UE-detection-duration in BEx/Skyhawk:
// All PFs must wait for this duration after they detect UE before reading
// SLIPORT_SEMAPHORE register. At the end of this duration, the Firmware
// guarantees that the SLIPORT_SEMAPHORE register is updated to indicate
// if the UE is recoverable.
//

// Initial idle time (in msec) to elapse after driver load,
// before UE recovery is allowed.
//
pub const ERR_IDLE_HR: c_int = 24;

// Time interval (in msec) after which UE recovery can be repeated
pub const ERR_INTERVAL_HR: c_int = 72;

// BEx/SH UE recovery state machine
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_error_recovery {
    pub /: *mut *mut u8 recovery_retries; / used for Lancer,
    pub /: *mut *mut u8 recovery_state; / used for BEx and Skyhawk,
}

// BEx/Skyhawk error recovery variables
// the chip - PF0 only
//
// of SLIPORT_SEMAPHORE reg
//
// Common to both Lancer & BEx/SH error recovery
// Ethtool priv_flags
pub const BE_DISABLE_TPE_RECOVERY: c_uint = 0x1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_vxlan_port {
    pub list: list_head,
    pub /: *mut *mut __be16 port; / VxLAN UDP dst port,
    pub /: *mut *mut int port_aliases; / alias count,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_adapter {
    pub pdev: *mut pci_dev,
    pub netdev: *mut net_device,
    pub /: *mut *mut *mut u8 __iomem csr; / CSR BAR used only for BE2/3,
    pub /: *mut *mut *mut u8 __iomem db; / Door Bell,
    pub /: *mut *mut *mut u8 __iomem pcicfg; / On SH,BEx only. Shadow of PCI config space,
    pub /: *mut *mut mutex mbox_lock; / For serializing mbox cmds to BE card,
    pub mbox_mem: be_dma_mem,
// Mbox mem is adjusted to align to 16 bytes. The allocated addr
// is stored for freeing purpose
    pub mbox_mem_alloced: be_dma_mem,
    pub mcc_obj: be_mcc_obj,
    pub /: *mut *mut spinlock_t mcc_lock; / For serializing mcc cmds to BE card,
    pub mcc_cq_lock: spinlock_t,
    pub /: *mut *mut u16 cfg_num_rx_irqs; / configured via set-channels,
    pub /: *mut *mut u16 cfg_num_tx_irqs; / configured via set-channels,
    pub num_evt_qs: u16,
    pub num_msix_vec: u16,
    pub eq_obj: [be_eq_obj; MAX_EVT_QS],
    pub msix_entries: [msix_entry; MAX_MSIX_VECTORS],
    pub isr_registered: bool,
// TX Rings
    pub num_tx_qs: u16,
    pub tx_obj: [be_tx_obj; MAX_TX_QS],
// Rx rings
    pub num_rx_qs: u16,
    pub num_rss_qs: u16,
    pub need_def_rxq: u16,
    pub rx_obj: [be_rx_obj; MAX_RX_QS],
    pub /: *mut *mut u32 big_page_size; / Compounded page size shared by rx wrbs,
    pub drv_stats: be_drv_stats,
    pub aic_obj: [be_aic_obj; MAX_EVT_QS],
    pub aic_enabled: bool,
    pub /: *mut *mut u8 vlan_prio_bmap; / Available Priority BitMap,
    pub /: *mut *mut u16 recommended_prio_bits;/ Recommended Priority bits in vlan tag,
    pub /: *mut *mut be_dma_mem rx_filter; / Cmd DMA mem for rx-filter,
    pub stats_cmd: be_dma_mem,
// Work queue used to perform periodic tasks like getting statistics
    pub work: delayed_work,
    pub work_counter: u16,
    pub recovery_retries: u8,
    pub err_flags: u8,
    pub /: *mut *mut bool pcicfg_mapped; / pcicfg obtained via pci_iomap(),
    pub flags: u32,
    pub cmd_privileges: u32,
// Ethtool knobs and info
    pub fw_ver: [c_char; FW_VER_LEN],
    pub fw_on_flash: [c_char; FW_VER_LEN],
// IFACE filtering fields
    pub /: *mut *mut int if_handle; / Used to configure filtering,
    pub /: *mut *mut u32 if_flags; / Interface filtering flags,
    pub /: *mut *mut *mut u32 pmac_id; / MAC addr handle used by BE card,
    pub /: *mut *mut *mut be_eth_addr uc_list;/ list of uc-addrs programmed (not perm),
    pub /: *mut *mut u32 uc_macs; / Count of secondary UC MAC programmed,
    pub /: *mut *mut *mut be_eth_addr mc_list;/ list of mcast addrs programmed,
    pub mc_count: u32,
    pub vids: [c_ulong; BITS_TO_LONGS(VLAN_N_VID)],
    pub vlans_added: u16,
    pub update_uc_list: bool,
    pub update_mc_list: bool,
    pub /: *mut *mut mutex rx_filter_lock;/ For protecting vids[] & mc/uc_list[],
    pub /: *mut *mut u32 beacon_state; / for set_phys_id,
    pub port_num: u32,
    pub port_name: c_char,
    pub mc_type: u8,
    pub function_mode: u32,
    pub function_caps: u32,
    pub /: *mut *mut u32 rx_fc; / Rx flow control,
    pub /: *mut *mut u32 tx_fc; / Tx flow control,
    pub stats_cmd_sent: bool,
    pub size: u32,
    pub total_size: u32,
    pub io_addr: u64,
    pub roce_db: },
    pub num_msix_roce_vec: u32,
    pub ocrdma_dev: *mut ocrdma_dev,
    pub entry: list_head,
    pub flash_status: u32,
    pub et_cmd_compl: completion,
    pub /: *mut *mut be_resources pool_res; / resources available for the port,
    pub /: *mut *mut be_resources res; / resources available for the func,
    pub /: *mut *mut u16 num_vfs; / Number of VFs provisioned by PF,
    pub /: *mut *mut u8 pf_num; / Numbering used by FW, starts at 0,
    pub /: *mut *mut u8 vf_num; / Numbering used by FW, starts at 1,
    pub virtfn: u8,
    pub vf_cfg: *mut be_vf_cfg,
    pub be3_native: bool,
    pub sli_family: u32,
    pub hba_port_num: u8,
    pub pvid: u16,
    pub /: *mut *mut __be16 vxlan_port; / offloaded vxlan port num,
    pub phy: phy_info,
    pub wol_cap: u8,
    pub wol_en: bool,
    pub asic_rev: u16,
    pub qnq_vid: u16,
    pub msg_enable: u32,
    pub be_get_temp_freq: c_int,
    pub hwmon_info: be_hwmon,
    pub rss_info: rss_info,
// Filters for packets that need to be sent to BMC
    pub bmc_filt_mask: u32,
    pub fat_dump_len: u32,
    pub serial_num: [u16; CNTL_SERIAL_NUM_WORDS],
    pub /: *mut *mut u8 phy_state; / state of sfp optics (functional, faulted, etc.,),
    pub dev_mac: [u8; ETH_ALEN],
    pub /: *mut *mut u32 priv_flags; / ethtool get/set_priv_flags(),
    pub error_recovery: be_error_recovery,
}

// Used for deferred FW config cmds. Add fields to this struct as reqd
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_cmd_work {
    pub work: work_struct,
    pub adapter: *mut be_adapter,
}

pub const ON: c_int = 1;
pub const OFF: c_int = 0;

// Max number of EQs available for the function (NIC + RoCE (if enabled))

// Max number of EQs available only for NIC

// Max irqs available for NIC

// Max irqs *needed* for RX queues
// If no RSS, need at least one irq for def-RXQ
extern "C" {
    pub fn min_t(_arg: u16, _arg: num, _arg: be_max_irqs(adapter)) -> return;
}
// Max irqs *needed* for TX queues
extern "C" {
    pub fn min_t(_arg: u16, _arg: be_max_txqs(adapter), _arg: be_max_irqs(adapter)) -> return;
}
// Max irqs *needed* for combined queues
extern "C" {
    pub fn min(_arg: be_max_tx_irqs(adapter), _arg: be_max_rx_irqs(adapter)) -> return;
}
// Max irqs *needed* for RX and TX queues together
extern "C" {
    pub fn max(_arg: be_max_tx_irqs(adapter), _arg: be_max_rx_irqs(adapter)) -> return;
}
// Is BE in pvid_tagging mode

// Is BE in QNQ multi-channel mode

// The default RXQ is the last RXQ

pub const PAGE_SHIFT_4K: c_int = 12;

// Returns number of pages spanned by the data starting at the given addr

// Returns bit offset within a DWORD of a bitfield

// Returns the bit mask of the field that is NOT shifted into location.
// dw &= ~(mask << offset);
// dw |= (mask & value) << offset;

// dw = cpu_to_le32(*dw);

extern "C" {
    pub fn ipv6_ext_hdr(_arg: ipv6_hdr(skb)->nexthdr) -> return;
}

pub const BE_ERROR_EEH: c_int = 1;

pub const BE_CLEAR_ALL: c_uint = 0xFF;
extern "C" {
    pub fn be_link_status_update(adapter: *mut be_adapter, link_status: u8);
}
extern "C" {
    pub fn be_parse_stats(adapter: *mut be_adapter);
}
extern "C" {
    pub fn be_load_fw(adapter: *mut be_adapter, func: *mut u8) -> c_int;
}
extern "C" {
    pub fn be_pause_supported(adapter: *mut be_adapter) -> bool;
}
extern "C" {
    pub fn be_update_queues(adapter: *mut be_adapter) -> c_int;
}
extern "C" {
    pub fn be_poll(napi: *mut napi_struct, budget: c_int) -> c_int;
}
extern "C" {
    pub fn be_eqd_update(adapter: *mut be_adapter, force_update: bool);
}
//
// internal function to initialize-cleanup roce device.
//
extern "C" {
    pub fn be_roce_dev_add(: *mut be_adapter);
}
extern "C" {
    pub fn be_roce_dev_remove(: *mut be_adapter);
}
//
// internal function to open-close roce device during ifup-ifdown.
//
extern "C" {
    pub fn be_roce_dev_shutdown(: *mut be_adapter);
}
