//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/wil6210/wil6210.h
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


// SPDX-License-Identifier: ISC
//
// Copyright (c) 2012-2017 Qualcomm Atheros, Inc.
// Copyright (c) 2018-2019, The Linux Foundation. All rights reserved.
//

pub const WIL_NUM_LATENCY_BINS: c_int = 200;
// maximum number of virtual interfaces the driver supports
// (including the main interface)
//
pub const WIL_MAX_VIFS: c_int = 4;
//
// extract bits [@b0:@b1] (inclusive) from the value @x
// it should be @b0 <= @b1, or result is incorrect
//

// limit ring size in range [32..32k]

pub const WIL_WMI_CALL_GENERAL_TO_MS: c_int = 100;

// Hardware offload block adds the following:
// 26 bytes - 3-address QoS data header
// 8 bytes - IV + EIV (for GCMP)
// 8 bytes - SNAP
// 16 bytes - MIC (for GCMP)
// 4 bytes - CRC
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wil_suspend_count_stats {
    pub successful_suspends: c_ulong,
    pub successful_resumes: c_ulong,
    pub failed_suspends: c_ulong,
    pub failed_resumes: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wil_suspend_stats {
    pub r_off: wil_suspend_count_stats,
    pub r_on: wil_suspend_count_stats,
    pub /: *mut *mut unsigned long rejected_by_device; / only radio on,
    pub rejected_by_host: c_ulong,
}

// Calculate MAC buffer size for the firmware. It includes all overhead,
// as it will go over the air, and need to be 8 byte aligned
//
extern "C" {
    pub fn ALIGN(WIL_MAX_MPDU_OVERHEAD: mtu +, _arg: 8) -> return;
}
// MTU for Ethernet need to take into account 8-byte SNAP header
// to be added when encapsulating Ethernet frame into 802.11
//

// Max supported by wil6210 value for interrupt threshold is 5sec.

// 802.11REVmc/D5.0, section 9.4.1.8)
//
// Hardware definitions begin
//
// Mapping
// RGF File      | Host addr    |  FW addr
// |              |
// user_rgf      | 0x000000     | 0x880000
// dma_rgf      | 0x001000     | 0x881000
// pcie_rgf      | 0x002000     | 0x882000
// |              |
//
// Where various structures placed in host address space

//
// Interrupt control registers block
//
// each interrupt controlled by the same bit in all registers
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RGF_ICR {
    pub /: *mut *mut u32 ICC; / Cause Control, RW: 0 - W1C, 1 - COR,
    pub /: *mut *mut u32 ICR; / Cause, W1C/COR depending on ICC,
    pub /: *mut *mut u32 ICM; / Cause masked (ICR & ~IMV), W1C/COR depending on ICC,
    pub /: *mut *mut u32 ICS; / Cause Set, WO,
    pub /: *mut *mut u32 IMV; / Mask, RW+S/C,
    pub /: *mut *mut u32 IMS; / Mask Set, write 1 to set,
    pub /: *mut *mut u32 IMC; / Mask Clear, write 1 to clear,
    pub __packed: },
// registers - FW addresses

// b8-15:signature
//

// Legacy interrupt moderation control (before Sparrow v2)

// Offload control (Sparrow B0+)

// New (sparrow v2+) interrupt moderation control

// MAC timer, usec, for packet lifetime

// eDMA

// eDMA status interrupts

// Talyn-MB

// crash codes for FW/Ucode stored here
// ASSERT RGFs

}

// popular locations

// ISR register bits

pub const WIL_DATA_COMPLETION_TO_MS: c_int = 200;
// Hardware definitions end
pub const SPARROW_FW_MAPPING_TABLE_SIZE: c_int = 10;
pub const TALYN_FW_MAPPING_TABLE_SIZE: c_int = 13;
pub const TALYN_MB_FW_MAPPING_TABLE_SIZE: c_int = 19;
pub const MAX_FW_MAPPING_TABLE_SIZE: c_int = 19;
// Common representation of physical address in wil ring
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wil_ring_dma_addr {
    pub addr_low: __le32,
    pub addr_high: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_map {
    pub /: *mut *mut u32 from; / linker address - from, inclusive,
    pub /: *mut *mut u32 to; / linker address - to, exclusive,
    pub /: *mut *mut u32 host; / PCI/Host address - BAR0 + 0x880000,
    pub /: *const *const *const char name; / for debugfs,
    pub /: *mut *mut bool fw; / true if FW mapping, false if UCODE mapping,
    pub /: *mut *mut bool crash_dump; / true if should be dumped during crash dump,
}

// array size should be in sync with actual definition in the wmi.c
//
// mk_cidxtid - construct @cidxtid field
// @cid: CID value
// @tid: TID value
//
// Returns: @cidxtid field encoded as bits 0..3 - CID; 4..7 - TID
//
// parse_cidxtid - parse @cidxtid field
// @cid: store CID value here
// @tid: store TID value here
// @cidxtid: field encoded as bits 0..3 - CID; 4..7 - TID
//
// cid = cidxtid & 0xf;
// tid = (cidxtid >> 4) & 0xf;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wil6210_mbox_ring {
    pub base: u32,
    pub /: *mut *mut u16 entry_size; / max. size of mbox entry, incl. all headers,
    pub size: u16,
    pub tail: u32,
    pub head: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wil6210_mbox_ring_desc {
    pub sync: __le32,
    pub addr: __le32,
    pub __packed: },
// at HOST_OFF_WIL6210_MBOX_CTL
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wil6210_mbox_ctl {
    pub tx: wil6210_mbox_ring,
    pub rx: wil6210_mbox_ring,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wil6210_mbox_hdr {
    pub seq: __le16,
    pub /: *mut *mut __le16 len; / payload, bytes after this header,
    pub type: __le16,
    pub flags: u8,
    pub reserved: u8,
    pub __packed: },

// max. value for wil6210_mbox_hdr.len

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pending_wmi_event {
    pub list: list_head,
    pub hdr: wil6210_mbox_hdr,
    pub wmi: wmi_cmd_hdr,
    pub data: [u8; 0],
    pub event: } __packed,
}

//
// struct wil_ctx - software context for ring descriptor
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wil_ctx {
    pub skb: *mut sk_buff,
    pub nr_frags: u8,
    pub mapped_as: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wil_desc_ring_rx_swtail {
    pub va: *mut u32,
    pub pa: dma_addr_t,
}

//
// A general ring structure, used for RX and TX.
// In legacy DMA it represents the vring,
// In enahnced DMA it represents the descriptor ring (vrings are handled by FW)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wil_ring {
    pub pa: dma_addr_t,
    pub va: *mut volatile union wil_ring_desc,
    pub /: *mut *mut u16 size; / number of wil_ring_desc elements,
    pub swtail: u32,
    pub swhead: u32,
    pub /: *mut *mut u32 hwtail; / write here to inform hw,
    pub /: *mut *mut *mut wil_ctx ctx; / ctx[size] - software context,
    pub edma_rx_swtail: wil_desc_ring_rx_swtail,
    pub is_rx: bool,
}

//
// Additional data for Rx ring.
// Used for enhanced DMA RX chaining.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wil_ring_rx_data {
// the skb being assembled
    pub skb: *mut sk_buff,
// true if we are skipping a bad fragmented packet
    pub skipping: bool,
    pub buff_size: u16,
}

//
// Status ring structure, used for enhanced DMA completions for RX and TX.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wil_status_ring {
    pub pa: dma_addr_t,
    pub /: *mut *mut *mut void va; / pointer to ring_[tr]x_status elements,
    pub /: *mut *mut u16 size; / number of status elements,
    pub /: *mut *mut size_t elem_size; / status element size in bytes,
    pub swhead: u32,
    pub /: *mut *mut u32 hwtail; / write here to inform hw,
    pub is_rx: bool,
    pub /: *mut *mut u8 desc_rdy_pol; / Expected descriptor ready bit polarity,
    pub rx_data: wil_ring_rx_data,
    pub /: *mut *mut u32 invalid_buff_id_cnt; / relevant only for RX,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wil_net_stats {
    pub rx_packets: c_ulong,
    pub tx_packets: c_ulong,
    pub rx_bytes: c_ulong,
    pub tx_bytes: c_ulong,
    pub tx_errors: c_ulong,
    pub tx_latency_min_us: u32,
    pub tx_latency_max_us: u32,
    pub tx_latency_total_us: u64,
    pub rx_dropped: c_ulong,
    pub rx_non_data_frame: c_ulong,
    pub rx_short_frame: c_ulong,
    pub rx_large_frame: c_ulong,
    pub rx_replay: c_ulong,
    pub rx_mic_error: c_ulong,
    pub /: *mut *mut unsigned long rx_key_error; / eDMA specific,
    pub /: *mut *mut unsigned long rx_amsdu_error; / eDMA specific,
    pub rx_csum_err: c_ulong,
    pub last_mcs_rx: u16,
    pub last_cb_mode_rx: u8,
    pub 1]: u64 rx_per_mcs[WIL_MCS_MAX +,
    pub /: *mut *mut u32 ft_roams; / relevant in STA mode,
}

//
// struct wil_txrx_ops - different TX/RX ops for legacy and enhanced
// DMA flow
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wil_txrx_ops {
    pub wil): *mut *mut void (configure_interrupt_moderation)(struct wil6210_priv,
// TX ops
    pub tid): int size, int cid, int,
    pub ring): *mut *mut *mut void (ring_fini_tx)(struct wil6210_priv wil, struct wil_ring,
    pub size): *mut *mut *mut int (ring_init_bcast)(struct wil6210_vif vif, int id, int,
    pub wil): *mut *mut int (tx_init)(struct wil6210_priv,
    pub wil): *mut *mut void (tx_fini)(struct wil6210_priv,
    pub ring_index): u32 len, int,
    pub ctx): *mut wil_ctx,
    pub skb): *mut *mut wil_ring ring, sk_buff,
    pub tid): int cid, int,
    pub cookie): *mut *mut irqreturn_t (irq_tx)(int irq, void,
// RX ops
    pub ring_order): *mut *mut *mut int (rx_init)(struct wil6210_priv wil, uint,
    pub wil): *mut *mut void (rx_fini)(struct wil6210_priv,
    pub timeout): u16 agg_wsize, u16,
    pub retry): *mut *mut *mut *mut int mid, u16 seq, int mcast, int,
    pub security): *mut *mut int cid, int,
    pub skb): *mut *mut *mut int (rx_crypto_check)(struct wil6210_priv wil, struct sk_buff,
    pub stats): *mut wil_net_stats,
    pub wil): *mut *mut bool (is_rx_idle)(struct wil6210_priv,
    pub cookie): *mut *mut irqreturn_t (irq_rx)(int irq, void,
}

//
// Additional data for Tx ring
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wil_ring_tx_data {
    pub dot1x_open: bool,
    pub enabled: c_int,
    pub begin: cycles_t idle, last_idle,,
    pub /: *mut *mut u8 agg_wsize; / agreed aggregation window, 0 - no agg,
    pub agg_timeout: u16,
    pub agg_amsdu: u8,
    pub /: *mut *mut bool addba_in_progress; / if set, agg_xxx is for request in progress,
    pub mid: u8,
    pub lock: spinlock_t,
}

//
// struct wil_tid_ampdu_rx - TID aggregation information (Rx).
//
// @reorder_buf: buffer to reorder incoming aggregated MPDUs
// @last_rx: jiffies of last rx activity
// @head_seq_num: head sequence number in reordering buffer.
// @stored_mpdu_num: number of MPDUs in reordering buffer
// @ssn: Starting Sequence Number expected to be aggregated.
// @buf_size: buffer size for incoming A-MPDUs
// @ssn_last_drop: SSN of the last dropped frame
// @total: total number of processed incoming frames
// @drop_dup: duplicate frames dropped for this reorder buffer
// @drop_old: old frames dropped for this reorder buffer
// @first_time: true when this buffer used 1-st time
// @mcast_last_seq: sequence number (SN) of last received multicast packet
// @drop_dup_mcast: duplicate multicast frames dropped for this reorder buffer
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wil_tid_ampdu_rx {
    pub reorder_buf: *mut sk_buff,
    pub last_rx: c_ulong,
    pub head_seq_num: u16,
    pub stored_mpdu_num: u16,
    pub ssn: u16,
    pub buf_size: u16,
    pub ssn_last_drop: u16,
    pub /: *mut *mut unsigned long long total; / frames processed,
    pub drop_dup: c_ulonglong,
    pub drop_old: c_ulonglong,
    pub /: *mut *mut bool first_time; / is it 1-st time this buffer used?,
    pub /: *mut *mut u16 mcast_last_seq; / multicast dup detection,
    pub drop_dup_mcast: c_ulonglong,
}

//
// struct wil_tid_crypto_rx_single - TID crypto information (Rx).
//
// @pn: GCMP PN for the session
// @key_set: valid key present
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wil_tid_crypto_rx_single {
    pub pn: [u8; IEEE80211_GCMP_PN_LEN],
    pub key_set: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wil_tid_crypto_rx {
    pub key_id: [wil_tid_crypto_rx_single; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wil_p2p_info {
    pub listen_chan: ieee80211_channel,
    pub discovery_started: u8,
    pub cookie: u64,
    pub pending_listen_wdev: *mut wireless_dev,
    pub listen_duration: c_uint,
    pub /: *mut *mut timer_list discovery_timer; / listen/search duration,
    pub /: *mut *mut work_discovery_expired_work; / listen/search expire,
    pub /: *mut *mut work_delayed_listen_work; / listen after scan done,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wil_sta_status {
    wil_sta_unused = 0,
    wil_sta_conn_pending = 1,
    wil_sta_connected = 2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wil_rekey_state {
    WIL_REKEY_IDLE = 0,
    WIL_REKEY_M3_RECEIVED = 1,
    WIL_REKEY_WAIT_M4_SENT = 2,
}

//
// struct wil_sta_info - data for peer
//
// Peer identified by its CID (connection ID)
// NIC performs beam forming for each peer;
// if no beam forming done, frame exchange is not
// possible.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wil_sta_info {
    pub addr: [u8; ETH_ALEN],
    pub mid: u8,
    pub status: wil_sta_status,
    pub stats: wil_net_stats,
//
// 20 latency bins. 1st bin counts packets with latency
// of 0..tx_latency_res, last bin counts packets with latency
// of 19*tx_latency_res and above.
// tx_latency_res is configured from "tx_latency" debug-fs.
//
    pub tx_latency_bins: *mut u64,
    pub fw_stats_basic: wmi_link_stats_basic,
// Rx BACK
    pub tid_rx: [*mut wil_tid_ampdu_rx; WIL_STA_TID_NUM],
    pub /: *mut *mut spinlock_t tid_rx_lock; / guarding tid_rx array,
    pub tid_rx_timer_expired: [c_ulong; BITS_TO_LONGS(WIL_STA_TID_NUM)],
    pub tid_rx_stop_requested: [c_ulong; BITS_TO_LONGS(WIL_STA_TID_NUM)],
    pub tid_crypto_rx: [wil_tid_crypto_rx; WIL_STA_TID_NUM],
    pub group_crypto_rx: wil_tid_crypto_rx,
    pub /: *mut *mut u8 aid; / 1-254; 0 if unknown/not reported,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wil_probe_client_req {
    pub list: list_head,
    pub cookie: u64,
    pub cid: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pmc_ctx {
// alloc, free, and read operations must own the lock
    pub lock: mutex,
    pub pring_va: *mut vring_tx_desc,
    pub pring_pa: dma_addr_t,
    pub descriptors: *mut desc_alloc_info,
    pub last_cmd_status: c_int,
    pub num_descriptors: c_int,
    pub descriptor_size: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wil_halp {
    pub /: *mut *mut mutex lock; / protect halp ref_cnt,
    pub ref_cnt: c_uint,
    pub comp: completion,
    pub handle_icr: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wil_blob_wrapper {
    pub wil: *mut wil6210_priv,
    pub blob: debugfs_blob_wrapper,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct blink_on_off_time {
    pub on_ms: u32,
    pub off_ms: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wil_debugfs_iomem_data {
    pub offset: *mut c_void,
    pub wil: *mut wil6210_priv,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wil_debugfs_data {
    pub data_arr: *mut wil_debugfs_iomem_data,
    pub iomem_data_count: c_int,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wil6210_vif_status {
    wil_vif_fwconnecting,
    wil_vif_fwconnected,
    wil_vif_ft_roam,
    wil_vif_status_last /* keep last */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wil6210_vif {
    pub wdev: wireless_dev,
    pub ndev: *mut net_device,
    pub wil: *mut wil6210_priv,
    pub mid: u8,
    pub wil_vif_status_last): DECLARE_BITMAP(status,,
    pub /: *mut *mut u32 privacy; / secure connection?,
    pub /: *mut *mut u16 channel; / relevant in AP mode,
    pub /: *mut *mut u8 wmi_edmg_channel; / relevant in AP mode,
    pub /: *mut *mut u8 hidden_ssid; / relevant in AP mode,
    pub /: *mut *mut u32 ap_isolate; / no intra-BSS communication,
    pub pbss: bool,
    pub bi: c_int,
    pub assocresp_ies: *mut *mut *mut u8 proberesp, proberesp_ies,,
    pub assocresp_ies_len: size_t proberesp_len, proberesp_ies_len,,
    pub ssid: [u8; IEEE80211_MAX_SSID_LEN],
    pub ssid_len: usize,
    pub gtk_index: u8,
    pub gtk: [u8; WMI_MAX_KEY_LEN],
    pub gtk_len: usize,
    pub bcast_ring: c_int,
    pub /: *mut *mut *mut cfg80211_bss bss; / connected bss, relevant in STA mode,
    pub /: *mut *mut int locally_generated_disc; / relevant in STA mode,
    pub connect_timer: timer_list,
    pub disconnect_worker: work_struct,
// scan
    pub scan_request: *mut cfg80211_scan_request,
    pub /: *mut *mut timer_list scan_timer; / detect scan timeout,
    pub p2p: wil_p2p_info,
// keep alive
    pub probe_client_pending: list_head,
    pub /: *mut *mut mutex probe_client_mutex; / protect @probe_client_pending,
    pub probe_client_worker: work_struct,
    pub /: *mut *mut int net_queue_stopped; / netif_tx_stop_all_queues invoked,
    pub /: *mut *mut bool fw_stats_ready; / per-cid statistics are ready inside sta_info,
    pub /: *mut *mut u64 fw_stats_tsf; / measurement timestamp,
// PTK rekey race prevention, this is relevant to station mode only
    pub ptk_rekey_state: wil_rekey_state,
    pub enable_tx_key_worker: work_struct,
}

//
// RX buffer allocated for enhanced DMA RX descriptors
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wil_rx_buff {
    pub skb: *mut sk_buff,
    pub list: list_head,
    pub id: c_int,
}

//
// During Rx completion processing, the driver extracts a buffer ID which
// is used as an index to the rx_buff_mgmt.buff_arr array and then the SKB
// is given to the network stack and the buffer is moved from the 'active'
// list to the 'free' list.
// During Rx refill, SKBs are attached to free buffers and moved to the
// 'active' list.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wil_rx_buff_mgmt {
    pub buff_arr: *mut wil_rx_buff,
    pub /: *mut *mut size_t size; / number of items in buff_arr,
    pub active: list_head,
    pub free: list_head,
    pub /: *mut *mut unsigned long free_list_empty_cnt; / statistics,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wil_fw_stats_global {
    pub ready: bool,
    pub /: *mut *mut u64 tsf; / measurement timestamp,
    pub stats: wmi_link_stats_global,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wil_brd_info {
    pub file_addr: u32,
    pub file_max_size: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wil6210_priv {
    pub pdev: *mut pci_dev,
    pub bar_size: u32,
    pub wiphy: *mut wiphy,
    pub main_ndev: *mut net_device,
    pub n_msi: c_int,
    pub csr: *mut void __iomem,
    pub wil_status_last): DECLARE_BITMAP(status,,
    pub fw_version: [u8; ETHTOOL_FWVERS_LEN],
    pub hw_version: u32,
    pub chip_revision: u8,
    pub hw_name: *const c_char,
    pub wil_fw_name: *const c_char,
    pub board_file: *mut c_char,
    pub num_of_brd_entries: u32,
    pub brd_info: *mut wil_brd_info,
    pub hw_capa_last): DECLARE_BITMAP(hw_capa,,
    pub WMI_FW_CAPABILITY_MAX): DECLARE_BITMAP(fw_capabilities,,
    pub WIL_PLATFORM_CAPA_MAX): DECLARE_BITMAP(platform_capa,,
    pub /: *mut *mut u32 recovery_count; / num of FW recovery attempts in a short time,
    pub /: *mut *mut u32 recovery_state; / FW recovery state machine,
    pub /: *mut *mut unsigned long last_fw_recovery; / jiffies of last fw recovery,
    pub /: *mut *mut wait_queue_head_t wq; / for all wait_event() use,
    pub /: *mut *mut u8 max_vifs; / maximum number of interfaces, including main,
    pub vifs: [*mut wil6210_vif; WIL_MAX_VIFS],
    pub /: *mut *mut mutex vif_mutex; / protects access to VIF entries,
    pub connected_vifs: core::sync::atomic::AtomicI32,
    pub /: *mut *mut u32 max_assoc_sta; / max sta's supported by the driver and the FW,
// profile
    pub monitor_chandef: cfg80211_chan_def,
    pub monitor_flags: u32,
    pub sinfo_gen: c_int,
// interrupt moderation
    pub tx_max_burst_duration: u32,
    pub tx_interframe_timeout: u32,
    pub rx_max_burst_duration: u32,
    pub rx_interframe_timeout: u32,
// cached ISR registers
    pub isr_misc: u32,
// mailbox related
    pub wmi_mutex: mutex,
    pub mbox_ctl: wil6210_mbox_ctl,
    pub wmi_ready: completion,
    pub wmi_call: completion,
    pub wmi_seq: u16,
    pub /: *mut *mut *mut u16 reply_id; /< wait for this WMI event,
    pub reply_mid: u8,
    pub reply_buf: *mut c_void,
    pub reply_size: u16,
    pub /: *mut *mut *mut workqueue_wmi_wq; / for deferred calls,
    pub wmi_event_worker: work_struct,
    pub wq_service: *mut workqueue_struct,
    pub /: *mut *mut work_fw_error_worker; / for FW error recovery,
    pub pending_wmi_ev: list_head,
//
// protect pending_wmi_ev
// - fill in IRQ from wil6210_irq_misc,
// - consumed in thread by wmi_event_worker
//
    pub wmi_ev_lock: spinlock_t,
    pub /: *mut *mut spinlock_t net_queue_lock; / guarding stop/wake netif queue,
    pub /: *mut *mut spinlock_t eap_lock; / guarding access to eap rekey fields,
    pub napi_rx: napi_struct,
    pub napi_tx: napi_struct,
    pub /: *mut *mut *mut net_device napi_ndev; / dummy net_device serving all VIFs,
// DMA related
    pub ring_rx: wil_ring,
    pub rx_buf_len: c_uint,
    pub ring_tx: [wil_ring; WIL6210_MAX_TX_RINGS],
    pub ring_tx_data: [wil_ring_tx_data; WIL6210_MAX_TX_RINGS],
    pub srings: [wil_status_ring; WIL6210_MAX_STATUS_RINGS],
    pub num_rx_status_rings: u8,
    pub tx_sring_idx: c_int,
    pub /: *mut *mut u8 ring2cid_tid[WIL6210_MAX_TX_RINGS][2]; / [0] - CID, [1] - TID,
    pub sta: [wil_sta_info; WIL6210_MAX_CID],
    pub /: *mut *mut u32 ring_idle_trsh; / HW fetches up to 16 descriptors at once,
    pub /: *mut *mut u32 dma_addr_size; / indicates dma addr size,
    pub rx_buff_mgmt: wil_rx_buff_mgmt,
    pub use_enhanced_dma_hw: bool,
    pub txrx_ops: wil_txrx_ops,
    pub /: *mut *mut mutex mutex; / for wil6210_priv access in wil_{up|down},
// for synchronizing device memory access while reset or suspend
    pub mem_lock: rw_semaphore,
// statistics
    pub isr_count_tx: atomic_t isr_count_rx,,
// debugfs
    pub debug: *mut dentry,
    pub blobs: [wil_blob_wrapper; MAX_FW_MAPPING_TABLE_SIZE],
    pub discovery_mode: u8,
    pub abft_len: u8,
    pub wakeup_trigger: u8,
    pub suspend_stats: wil_suspend_stats,
    pub dbg_data: wil_debugfs_data,
    pub /: *mut *mut bool tx_latency; / collect TX latency measurements,
    pub /: *mut *mut size_t tx_latency_res; / bin resolution in usec,
    pub platform_handle: *mut c_void,
    pub platform_ops: wil_platform_ops,
    pub keep_radio_on_during_sleep: bool,
    pub pmc: pmc_ctx,
    pub p2p_dev_started: u8,
// P2P_DEVICE vif
    pub p2p_wdev: *mut wireless_dev,
    pub radio_wdev: *mut wireless_dev,
// High Access Latency Policy voting
    pub halp: wil_halp,
    pub ps_profile: wmi_ps_profile_type,
    pub fw_calib_result: c_int,
    pub pm_notify: notifier_block,
    pub suspend_resp_rcvd: bool,
    pub suspend_resp_comp: bool,
    pub bus_request_kbps: u32,
    pub bus_request_kbps_pre_suspend: u32,
    pub rgf_fw_assert_code_addr: u32,
    pub rgf_ucode_assert_code_addr: u32,
    pub iccm_base: u32,
// relevant only for eDMA
    pub use_compressed_rx_status: bool,
    pub rx_status_ring_order: u32,
    pub tx_status_ring_order: u32,
    pub rx_buff_id_count: u32,
    pub amsdu_en: bool,
    pub use_rx_hw_reordering: bool,
    pub secured_boot: bool,
    pub boot_config: u8,
    pub fw_stats_global: wil_fw_stats_global,
    pub max_agg_wsize: u32,
    pub max_ampdu_size: u32,
    pub multicast_to_unicast: u8,
    pub cqm_rssi_thold: i32,
}

// main interface is shared with P2P device
extern "C" {
    pub fn ndev_to_vif(_arg: wil->main_ndev) -> return;
}
extern "C" {
    pub fn container_of(_arg: wdev, wil6210_vif: struct, _arg: wdev) -> return;
}
// main interface is shared with P2P device
extern "C" {
    pub fn vif_to_wdev(_arg: vif) -> return;
}
extern "C" {
    pub fn wil_dbg_trace(wil: *mut wil6210_priv, fmt: *const c_char, ...);
}
extern "C" {
    pub fn __wil_err(wil: *mut wil6210_priv, fmt: *const c_char, ...);
}
extern "C" {
    pub fn __wil_err_ratelimited(wil: *mut wil6210_priv, fmt: *const c_char, ...);
}
extern "C" {
    pub fn __wil_info(wil: *mut wil6210_priv, fmt: *const c_char, ...);
}
extern "C" {
    pub fn wil_dbg_ratelimited(wil: *const wil6210_priv, fmt: *const c_char, ...);
}

// target operations
// register read
extern "C" {
    pub fn readl(HOSTADDR(reg): wil->csr +) -> return;
}
// register write. wmb() to make sure it is completed
// register set = read, OR, write
// register clear = read, AND with inverted, write
//
// wil_cid_valid - check cid is valid
//
extern "C" {
    pub fn wil_get_board_file(wil: *mut wil6210_priv, buf: *mut c_char, len: usize);
}

extern "C" {
    pub fn wil_mem_access_lock(wil: *mut wil6210_priv) -> c_int;
}
extern "C" {
    pub fn wil_mem_access_unlock(wil: *mut wil6210_priv);
}
extern "C" {
    pub fn wil_vif_free(vif: *mut wil6210_vif);
}
extern "C" {
    pub fn wil_has_active_ifaces(wil: *mut wil6210_priv, up: bool, ok: bool) -> bool;
}
extern "C" {
    pub fn wil_if_free(wil: *mut wil6210_priv);
}
extern "C" {
    pub fn wil_vif_add(wil: *mut wil6210_priv, vif: *mut wil6210_vif) -> c_int;
}
extern "C" {
    pub fn wil_if_add(wil: *mut wil6210_priv) -> c_int;
}
extern "C" {
    pub fn wil_vif_remove(wil: *mut wil6210_priv, mid: u8);
}
extern "C" {
    pub fn wil_if_remove(wil: *mut wil6210_priv);
}
extern "C" {
    pub fn wil_priv_init(wil: *mut wil6210_priv) -> c_int;
}
extern "C" {
    pub fn wil_priv_deinit(wil: *mut wil6210_priv);
}
extern "C" {
    pub fn wil_reset(wil: *mut wil6210_priv, no_fw: bool) -> c_int;
}
extern "C" {
    pub fn wil_fw_error_recovery(wil: *mut wil6210_priv);
}
extern "C" {
    pub fn wil_set_recovery_state(wil: *mut wil6210_priv, state: c_int);
}
extern "C" {
    pub fn wil_is_recovery_blocked(wil: *mut wil6210_priv) -> bool;
}
extern "C" {
    pub fn wil_up(wil: *mut wil6210_priv) -> c_int;
}
extern "C" {
    pub fn __wil_up(wil: *mut wil6210_priv) -> c_int;
}
extern "C" {
    pub fn wil_down(wil: *mut wil6210_priv) -> c_int;
}
extern "C" {
    pub fn __wil_down(wil: *mut wil6210_priv) -> c_int;
}
extern "C" {
    pub fn wil_refresh_fw_capabilities(wil: *mut wil6210_priv);
}
extern "C" {
    pub fn wil_mbox_ring_le2cpus(r: *mut wil6210_mbox_ring);
}
extern "C" {
    pub fn wil_find_cid(wil: *mut wil6210_priv, mid: u8, mac: *const u8) -> c_int;
}
extern "C" {
    pub fn wil_find_cid_by_idx(wil: *mut wil6210_priv, mid: u8, idx: c_int) -> c_int;
}
extern "C" {
    pub fn wil_set_ethtoolops(ndev: *mut net_device);
}
extern "C" {
    pub fn wmi_send(wil: *mut wil6210_priv, cmdid: u16, mid: u8, buf: *mut c_void, len: u16) -> c_int;
}
extern "C" {
    pub fn wmi_recv_cmd(wil: *mut wil6210_priv);
}
extern "C" {
    pub fn wmi_event_worker(work: *mut work_struct);
}
extern "C" {
    pub fn wmi_event_flush(wil: *mut wil6210_priv);
}
extern "C" {
    pub fn wmi_set_ssid(vif: *mut wil6210_vif, ssid_len: u8, ssid: *const c_void) -> c_int;
}
extern "C" {
    pub fn wmi_get_ssid(vif: *mut wil6210_vif, ssid_len: *mut u8, ssid: *mut c_void) -> c_int;
}
extern "C" {
    pub fn wmi_set_channel(wil: *mut wil6210_priv, channel: c_int) -> c_int;
}
extern "C" {
    pub fn wmi_get_channel(wil: *mut wil6210_priv, channel: *mut c_int) -> c_int;
}
extern "C" {
    pub fn wmi_echo(wil: *mut wil6210_priv) -> c_int;
}
extern "C" {
    pub fn wmi_set_ie(vif: *mut wil6210_vif, type: u8, ie_len: u16, ie: *const c_void) -> c_int;
}
extern "C" {
    pub fn wmi_rx_chain_add(wil: *mut wil6210_priv, vring: *mut wil_ring) -> c_int;
}
extern "C" {
    pub fn wmi_update_ft_ies(vif: *mut wil6210_vif, ie_len: u16, ie: *const c_void) -> c_int;
}
extern "C" {
    pub fn wmi_rxon(wil: *mut wil6210_priv, on: bool) -> c_int;
}
extern "C" {
    pub fn wmi_get_temperature(wil: *mut wil6210_priv, t_m: *mut u32, t_r: *mut u32) -> c_int;
}
// sense_all_evt);
extern "C" {
    pub fn wmi_delba_tx(wil: *mut wil6210_priv, mid: u8, ringid: u8, reason: u16) -> c_int;
}
extern "C" {
    pub fn wmi_delba_rx(wil: *mut wil6210_priv, mid: u8, cid: u8, tid: u8, reason: u16) -> c_int;
}
extern "C" {
    pub fn wmi_set_mgmt_retry(wil: *mut wil6210_priv, retry_short: u8) -> c_int;
}
extern "C" {
    pub fn wmi_get_mgmt_retry(wil: *mut wil6210_priv, retry_short: *mut u8) -> c_int;
}
extern "C" {
    pub fn wmi_new_sta(vif: *mut wil6210_vif, mac: *const u8, aid: u8) -> c_int;
}
extern "C" {
    pub fn wmi_port_delete(wil: *mut wil6210_priv, mid: u8) -> c_int;
}
extern "C" {
    pub fn wmi_link_stats_cfg(vif: *mut wil6210_vif, type: u32, cid: u8, interval: u32) -> c_int;
}
extern "C" {
    pub fn wil_addba_tx_request(wil: *mut wil6210_priv, ringid: u8, wsize: u16) -> c_int;
}
extern "C" {
    pub fn wil6210_clear_irq(wil: *mut wil6210_priv);
}
extern "C" {
    pub fn wil6210_init_irq(wil: *mut wil6210_priv, irq: c_int) -> c_int;
}
extern "C" {
    pub fn wil6210_fini_irq(wil: *mut wil6210_priv, irq: c_int);
}
extern "C" {
    pub fn wil_mask_irq(wil: *mut wil6210_priv);
}
extern "C" {
    pub fn wil_unmask_irq(wil: *mut wil6210_priv);
}
extern "C" {
    pub fn wil_configure_interrupt_moderation(wil: *mut wil6210_priv);
}
extern "C" {
    pub fn wil_disable_irq(wil: *mut wil6210_priv);
}
extern "C" {
    pub fn wil_enable_irq(wil: *mut wil6210_priv);
}
extern "C" {
    pub fn wil6210_mask_halp(wil: *mut wil6210_priv);
}
// P2P
extern "C" {
    pub fn wil_p2p_is_social_scan(request: *mut cfg80211_scan_request) -> bool;
}
extern "C" {
    pub fn wil_p2p_stop_discovery(vif: *mut wil6210_vif) -> u8;
}
extern "C" {
    pub fn wil_p2p_cancel_listen(vif: *mut wil6210_vif, cookie: u64) -> c_int;
}
extern "C" {
    pub fn wil_p2p_listen_expired(work: *mut work_struct);
}
extern "C" {
    pub fn wil_p2p_search_expired(work: *mut work_struct);
}
extern "C" {
    pub fn wil_p2p_stop_radio_operations(wil: *mut wil6210_priv);
}
extern "C" {
    pub fn wil_p2p_delayed_listen_work(work: *mut work_struct);
}
// WMI for P2P
extern "C" {
    pub fn wmi_p2p_cfg(vif: *mut wil6210_vif, channel: c_int, bi: c_int) -> c_int;
}
extern "C" {
    pub fn wmi_start_listen(vif: *mut wil6210_vif) -> c_int;
}
extern "C" {
    pub fn wmi_start_search(vif: *mut wil6210_vif) -> c_int;
}
extern "C" {
    pub fn wmi_stop_discovery(vif: *mut wil6210_vif) -> c_int;
}
extern "C" {
    pub fn wil_cfg80211_ap_recovery(wil: *mut wil6210_priv);
}
extern "C" {
    pub fn wil_vif_prepare_stop(vif: *mut wil6210_vif) -> c_int;
}

extern "C" {
    pub fn wil6210_debugfs_init(wil: *mut wil6210_priv) -> c_int;
}
extern "C" {
    pub fn wil6210_debugfs_remove(wil: *mut wil6210_priv);
}

extern "C" {
    pub fn wil_cfg80211_deinit(wil: *mut wil6210_priv);
}
extern "C" {
    pub fn wil_p2p_wdev_free(wil: *mut wil6210_priv);
}
extern "C" {
    pub fn wmi_set_mac_address(wil: *mut wil6210_priv, addr: *const c_void) -> c_int;
}
extern "C" {
    pub fn wmi_pcp_stop(vif: *mut wil6210_vif) -> c_int;
}
extern "C" {
    pub fn wmi_led_cfg(wil: *mut wil6210_priv, enable: bool) -> c_int;
}
extern "C" {
    pub fn wmi_abort_scan(vif: *mut wil6210_vif) -> c_int;
}
extern "C" {
    pub fn wil_abort_scan(vif: *mut wil6210_vif, sync: bool);
}
extern "C" {
    pub fn wil_abort_scan_all_vifs(wil: *mut wil6210_priv, sync: bool);
}
extern "C" {
    pub fn wil6210_bus_request(wil: *mut wil6210_priv, kbps: u32);
}
extern "C" {
    pub fn wil_probe_client_flush(vif: *mut wil6210_vif);
}
extern "C" {
    pub fn wil_probe_client_worker(work: *mut work_struct);
}
extern "C" {
    pub fn wil_disconnect_worker(work: *mut work_struct);
}
extern "C" {
    pub fn wil_enable_tx_key_worker(work: *mut work_struct);
}
extern "C" {
    pub fn wil_init_txrx_ops(wil: *mut wil6210_priv);
}
// TX API
extern "C" {
    pub fn wil_ring_init_tx(vif: *mut wil6210_vif, cid: c_int) -> c_int;
}
extern "C" {
    pub fn wil_vring_init_bcast(vif: *mut wil6210_vif, id: c_int, size: c_int) -> c_int;
}
extern "C" {
    pub fn wil_bcast_init(vif: *mut wil6210_vif) -> c_int;
}
extern "C" {
    pub fn wil_bcast_fini(vif: *mut wil6210_vif);
}
extern "C" {
    pub fn wil_bcast_fini_all(wil: *mut wil6210_priv);
}
extern "C" {
    pub fn wil_start_xmit(skb: *mut sk_buff, ndev: *mut net_device) -> netdev_tx_t;
}
extern "C" {
    pub fn wil_tx_complete(vif: *mut wil6210_vif, ringid: c_int) -> c_int;
}
extern "C" {
    pub fn wil6210_unmask_irq_tx(wil: *mut wil6210_priv);
}
extern "C" {
    pub fn wil6210_unmask_irq_tx_edma(wil: *mut wil6210_priv);
}
// RX API
extern "C" {
    pub fn wil_rx_handle(wil: *mut wil6210_priv, quota: *mut c_int);
}
extern "C" {
    pub fn wil6210_unmask_irq_rx(wil: *mut wil6210_priv);
}
extern "C" {
    pub fn wil6210_unmask_irq_rx_edma(wil: *mut wil6210_priv);
}
extern "C" {
    pub fn wil_iftype_nl2wmi(type: nl80211_iftype) -> c_int;
}
extern "C" {
    pub fn wil_request_board(wil: *mut wil6210_priv, name: *const c_char) -> c_int;
}
extern "C" {
    pub fn wil_fw_verify_file_exists(wil: *mut wil6210_priv, name: *const c_char) -> bool;
}
extern "C" {
    pub fn wil_pm_runtime_allow(wil: *mut wil6210_priv);
}
extern "C" {
    pub fn wil_pm_runtime_forbid(wil: *mut wil6210_priv);
}
extern "C" {
    pub fn wil_pm_runtime_get(wil: *mut wil6210_priv) -> c_int;
}
extern "C" {
    pub fn wil_pm_runtime_put(wil: *mut wil6210_priv);
}
extern "C" {
    pub fn wil_can_suspend(wil: *mut wil6210_priv, is_runtime: bool) -> c_int;
}
extern "C" {
    pub fn wil_suspend(wil: *mut wil6210_priv, is_runtime: bool, keep_radio_on: bool) -> c_int;
}
extern "C" {
    pub fn wil_resume(wil: *mut wil6210_priv, is_runtime: bool, keep_radio_on: bool) -> c_int;
}
extern "C" {
    pub fn wil_is_wmi_idle(wil: *mut wil6210_priv) -> bool;
}
extern "C" {
    pub fn wmi_resume(wil: *mut wil6210_priv) -> c_int;
}
extern "C" {
    pub fn wmi_suspend(wil: *mut wil6210_priv) -> c_int;
}
extern "C" {
    pub fn wil_is_tx_idle(wil: *mut wil6210_priv) -> bool;
}
extern "C" {
    pub fn wil_fw_copy_crash_dump(wil: *mut wil6210_priv, dest: *mut c_void, size: u32) -> c_int;
}
extern "C" {
    pub fn wil_fw_core_dump(wil: *mut wil6210_priv);
}
extern "C" {
    pub fn wil_halp_vote(wil: *mut wil6210_priv);
}
extern "C" {
    pub fn wil_halp_unvote(wil: *mut wil6210_priv);
}
extern "C" {
    pub fn wil6210_set_halp(wil: *mut wil6210_priv);
}
extern "C" {
    pub fn wil6210_clear_halp(wil: *mut wil6210_priv);
}
extern "C" {
    pub fn wmi_stop_sched_scan(wil: *mut wil6210_priv) -> c_int;
}
extern "C" {
    pub fn wmi_mgmt_tx(vif: *mut wil6210_vif, buf: *const u8, len: usize) -> c_int;
}
extern "C" {
    pub fn wmi_rbufcap_cfg(wil: *mut wil6210_priv, enable: bool, threshold: u16) -> c_int;
}
extern "C" {
    pub fn wil_wmi2spec_ch(wmi_ch: u8, spec_ch: *mut u8) -> c_int;
}
extern "C" {
    pub fn wil_spec2wmi_ch(spec_ch: u8, wmi_ch: *mut u8) -> c_int;
}
extern "C" {
    pub fn wil_update_supported_bands(wil: *mut wil6210_priv);
}
extern "C" {
    pub fn reverse_memcmp(cs: *const c_void, ct: *const c_void, count: usize) -> c_int;
}
// WMI for enhanced DMA
extern "C" {
    pub fn wil_wmi_tx_sring_cfg(wil: *mut wil6210_priv, ring_id: c_int) -> c_int;
}
extern "C" {
    pub fn wil_wmi_rx_sring_add(wil: *mut wil6210_priv, ring_id: u16) -> c_int;
}
extern "C" {
    pub fn wil_wmi_rx_desc_ring_add(wil: *mut wil6210_priv, status_ring_id: c_int) -> c_int;
}
extern "C" {
    pub fn wil_wmi_bcast_desc_ring_add(vif: *mut wil6210_vif, ring_id: c_int) -> c_int;
}
extern "C" {
    pub fn update_supported_bands(wil: *mut wil6210_priv);
}
extern "C" {
    pub fn wil_clear_fw_log_addr(wil: *mut wil6210_priv);
}
