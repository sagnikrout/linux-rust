//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/cisco/enic/enic.h
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
// Copyright 2008-2010 Cisco Systems, Inc.  All rights reserved.
// Copyright 2007 Nuova Systems, Inc.  All rights reserved.
//

pub const PCI_SUBDEV_ID_CISCO_VIC_1225: c_uint = 0x085;
pub const PCI_SUBDEV_ID_CISCO_VIC_1225T: c_uint = 0x0CE;
pub const PCI_SUBDEV_ID_CISCO_VIC_1227: c_uint = 0x12E;
pub const PCI_SUBDEV_ID_CISCO_VIC_1227T: c_uint = 0x139;
pub const PCI_SUBDEV_ID_CISCO_VIC_1240: c_uint = 0x084;
pub const PCI_SUBDEV_ID_CISCO_VIC_1280: c_uint = 0x04F;
pub const PCI_SUBDEV_ID_CISCO_VIC_1285: c_uint = 0x0CD;
pub const PCI_SUBDEV_ID_CISCO_VIC_1340: c_uint = 0x12C;
pub const PCI_SUBDEV_ID_CISCO_VIC_1380: c_uint = 0x137;
pub const PCI_SUBDEV_ID_CISCO_VIC_1385: c_uint = 0x14D;
pub const PCI_SUBDEV_ID_CISCO_VIC_1387: c_uint = 0x15D;
pub const PCI_SUBDEV_ID_CISCO_VIC_1440: c_uint = 0x0215;
pub const PCI_SUBDEV_ID_CISCO_VIC_1455: c_uint = 0x0217;
pub const PCI_SUBDEV_ID_CISCO_VIC_1457: c_uint = 0x0218;
pub const PCI_SUBDEV_ID_CISCO_VIC_1467: c_uint = 0x02AF;
pub const PCI_SUBDEV_ID_CISCO_VIC_1477: c_uint = 0x2B0;
pub const PCI_SUBDEV_ID_CISCO_VIC_1480: c_uint = 0x0216;
pub const PCI_SUBDEV_ID_CISCO_VIC_1485: c_uint = 0x0219;
pub const PCI_SUBDEV_ID_CISCO_VIC_1487: c_uint = 0x021A;
pub const PCI_SUBDEV_ID_CISCO_VIC_1495: c_uint = 0x024A;
pub const PCI_SUBDEV_ID_CISCO_VIC_1497: c_uint = 0x024B;
pub const PCI_SUBDEV_ID_CISCO_VIC_14425: c_uint = 0x02CF;
pub const PCI_SUBDEV_ID_CISCO_VIC_14825: c_uint = 0x02D0;
pub const PCI_SUBDEV_ID_CISCO_VIC_15230: c_uint = 0x02DF;
pub const PCI_SUBDEV_ID_CISCO_VIC_15231: c_uint = 0x02DB;
pub const PCI_SUBDEV_ID_CISCO_VIC_15235: c_uint = 0x02E4;
pub const PCI_SUBDEV_ID_CISCO_VIC_15237: c_uint = 0x02F3;
pub const PCI_SUBDEV_ID_CISCO_VIC_15238: c_uint = 0x02E8;
pub const PCI_SUBDEV_ID_CISCO_VIC_15411: c_uint = 0x02DC;
pub const PCI_SUBDEV_ID_CISCO_VIC_15412: c_uint = 0x02E2;
pub const PCI_SUBDEV_ID_CISCO_VIC_15420: c_uint = 0x02DE;
pub const PCI_SUBDEV_ID_CISCO_VIC_15422: c_uint = 0x02E1;
pub const PCI_SUBDEV_ID_CISCO_VIC_15425: c_uint = 0x02F2;
pub const PCI_SUBDEV_ID_CISCO_VIC_15427: c_uint = 0x02E0;
pub const PCI_SUBDEV_ID_CISCO_VIC_15428: c_uint = 0x02DD;
pub const ENIC_BARS_MAX: c_int = 6;
pub const ENIC_WQ_MAX: c_int = 256;
pub const ENIC_RQ_MAX: c_int = 256;
pub const ENIC_RQ_MIN_DEFAULT: c_int = 8;
pub const ENIC_WQ_NAPI_BUDGET: c_int = 256;
pub const ENIC_AIC_LARGE_PKT_DIFF: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ext_cq {
    ENIC_RQ_CQ_ENTRY_SIZE_16,
    ENIC_RQ_CQ_ENTRY_SIZE_32,
    ENIC_RQ_CQ_ENTRY_SIZE_64,
    ENIC_RQ_CQ_ENTRY_SIZE_MAX,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct enic_msix_entry {
    pub requested: c_int,
    pub 8]: char devname[IFNAMSIZ +,
    pub ): *mut *mut irqreturn_t (isr)(int, void,
    pub devid: *mut c_void,
    pub affinity_mask: cpumask_var_t,
}

// Store only the lower range.  Higher range is given by fw.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct enic_intr_mod_range {
    pub small_pkt_range_start: u32,
    pub large_pkt_range_start: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct enic_intr_mod_table {
    pub rx_rate: u32,
    pub range_percent: u32,
}

pub const ENIC_MAX_LINK_SPEEDS: c_int = 3;
pub const ENIC_LINK_SPEED_10G: c_int = 10000;
pub const ENIC_LINK_SPEED_4G: c_int = 4000;
pub const ENIC_LINK_40G_INDEX: c_int = 2;
pub const ENIC_LINK_10G_INDEX: c_int = 1;
pub const ENIC_LINK_4G_INDEX: c_int = 0;
pub const ENIC_RX_COALESCE_RANGE_END: c_int = 125;
pub const ENIC_AIC_TS_BREAK: c_int = 100;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct enic_rx_coal {
    pub small_pkt_range_start: u32,
    pub large_pkt_range_start: u32,
    pub range_end: u32,
    pub use_adaptive_rx_coalesce: u32,
}

// priv_flags

// enic port profile set flags

#[repr(C)]
#[derive(Copy, Clone)]
pub struct enic_port_profile {
    pub set: u32,
    pub request: u8,
    pub name: [c_char; PORT_PROFILE_MAX],
    pub instance_uuid: [u8; PORT_UUID_MAX],
    pub host_uuid: [u8; PORT_UUID_MAX],
    pub vf_mac: [u8; ETH_ALEN],
    pub mac_addr: [u8; ETH_ALEN],
}

// enic_rfs_fltr_node - rfs filter node in hash table
// @@keys: IPv4 5 tuple
// @flow_id: flow_id of clsf filter provided by kernel
// @fltr_id: filter id of clsf filter returned by adaptor
// @rq_id: desired rq index
// @node: hlist_node
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct enic_rfs_fltr_node {
    pub keys: flow_keys,
    pub flow_id: u32,
    pub fltr_id: u16,
    pub rq_id: u16,
    pub node: hlist_node,
}

// enic_rfs_flw_tbl - rfs flow table
// @max: Maximum number of filters vNIC supports
// @free: Number of free filters available
// @toclean: hash table index to clean next
// @ht_head: hash table list head
// @lock: spin lock
// @rfs_may_expire: timer function for enic_rps_may_expire_flow
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct enic_rfs_flw_tbl {
    pub max: u16,
    pub free: c_int,

    pub toclean:ENIC_RFS_FLW_BITSHIFT: u16,
    pub ENIC_RFS_FLW_BITSHIFT]: hlist_head ht_head[1 <<,
    pub lock: spinlock_t,
    pub rfs_may_expire: timer_list,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vxlan_offload {
    pub vxlan_udp_port_number: u16,
    pub patch_level: u8,
    pub flags: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct enic_wq_stats {
    pub /: *mut *mut u64 packets; / pkts queued for Tx,
    pub /: *mut *mut u64 stopped; / Tx ring almost full, queue stopped,
    pub up*/: *mut *mut u64 wake; / Tx ring no longer full, queue woken,
    pub /: *mut *mut u64 tso; / non-encap tso pkt,
    pub /: *mut *mut u64 encap_tso; / encap tso pkt,
    pub /: *mut *mut u64 encap_csum; / encap HW csum,
    pub /: *mut *mut u64 csum_partial; / skb->ip_summed = CHECKSUM_PARTIAL,
    pub /: *mut *mut u64 csum_none; / HW csum not required,
    pub /: *mut *mut u64 bytes; / bytes queued for Tx,
    pub /: *mut *mut u64 add_vlan; / HW adds vlan tag,
    pub /: *mut *mut u64 cq_work; / Tx completions processed,
    pub /: *mut *mut u64 cq_bytes; / Tx bytes processed,
    pub /: *mut *mut u64 null_pkt; / skb length <= 0,
    pub /: *mut *mut u64 skb_linear_fail; / linearize failures,
    pub /: *mut *mut u64 desc_full_awake; / TX ring full while queue awake,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct enic_rq_stats {
    pub /: *mut *mut u64 packets; / pkts received,
    pub /: *mut *mut u64 bytes; / bytes received,
    pub /: *mut *mut u64 l4_rss_hash; / hashed on l4,
    pub /: *mut *mut u64 l3_rss_hash; / hashed on l3,
    pub /: *mut *mut u64 csum_unnecessary; / HW verified csum,
    pub /: *mut *mut u64 csum_unnecessary_encap; / HW verified csum on encap packet,
    pub /: *mut *mut u64 vlan_stripped; / HW stripped vlan,
    pub /: *mut *mut u64 napi_complete; / napi complete intr reenabled,
    pub /: *mut *mut u64 napi_repoll; / napi poll again,
    pub /: *mut *mut u64 bad_fcs; / bad pkts,
    pub /: *mut *mut u64 pkt_truncated; / truncated pkts,
    pub /: *mut *mut u64 no_skb; / out of skbs,
    pub /: *mut *mut u64 desc_skip; / Rx pkt went into later buffer,
    pub /: *mut *mut u64 pp_alloc_fail; / page pool alloc failure,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct enic_wq {
    pub /: *mut *mut spinlock_t lock; / spinlock for wq,
    pub vwq: vnic_wq,
    pub stats: enic_wq_stats,
    pub ____cacheline_aligned: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct enic_rq {
    pub vrq: vnic_rq,
    pub stats: enic_rq_stats,
    pub pool: *mut page_pool,
    pub ____cacheline_aligned: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum enic_vf_type {
    ENIC_VF_TYPE_NONE,
    ENIC_VF_TYPE_V1,
    ENIC_VF_TYPE_USNIC,
    ENIC_VF_TYPE_V2,
}

// Per-instance private data structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct enic {
    pub netdev: *mut net_device,
    pub pdev: *mut pci_dev,
    pub config: vnic_enet_config,
    pub bar: [vnic_dev_bar; ENIC_BARS_MAX],
    pub vdev: *mut vnic_dev,
    pub notify_timer: timer_list,
    pub reset: work_struct,
    pub tx_hang_reset: work_struct,
    pub change_mtu_work: work_struct,
    pub msix_entry: *mut msix_entry,
    pub msix: *mut enic_msix_entry,
    pub msg_enable: u32,
    pub devcmd_lock: spinlock_t,
    pub mac_addr: [u8; ETH_ALEN],
    pub flags: c_uint,
    pub priv_flags: c_uint,
    pub mc_count: c_uint,
    pub uc_count: c_uint,
    pub port_mtu: u32,
    pub rx_coalesce_setting: enic_rx_coal,
    pub rx_coalesce_usecs: u32,
    pub tx_coalesce_usecs: u32,
    pub num_vfs: u16,
    pub vf_type: enic_vf_type,
    pub vf_registered: bool,
    pub pf_cap_version: u32,
    pub enable_count: c_uint,
    pub enic_api_lock: spinlock_t,
    pub enic_api_busy: bool,
    pub pp: *mut enic_port_profile,
    pub wq: *mut enic_wq,
    pub wq_avail: c_uint,
    pub wq_count: c_uint,
    pub loop_enable: u16,
    pub loop_tag: u16,
    pub rq: *mut enic_rq,
    pub rq_avail: c_uint,
    pub rq_count: c_uint,
    pub vxlan: vxlan_offload,
    pub napi: *mut napi_struct,
    pub intr: *mut vnic_intr,
    pub intr_avail: c_uint,
    pub intr_count: c_uint,
    pub /: *mut *mut *mut u32 __iomem legacy_pba; / memory-mapped,
    pub cq: *mut vnic_cq,
    pub cq_avail: c_uint,
    pub cq_count: c_uint,
    pub rfs_h: enic_rfs_flw_tbl,
    pub rss_key: [u8; ENIC_RSS_LEN],
    pub gen_stats: vnic_gen_stats,
    pub ext_cq: ext_cq,
// Admin channel resources for SR-IOV MBOX
    pub has_admin_channel: bool,
// true only while the admin WQ/RQ/CQ are allocated and enabled; gates
// enic_admin_channel_close() so it is a no-op after a failed (re)open
// left the resources freed.
//
    pub admin_chan_up: bool,
// set on send timeout; cleared on channel re-open
    pub mbox_send_disabled: bool,
    pub admin_wq: vnic_wq,
    pub admin_rq: vnic_rq,
    pub admin_cq: [vnic_cq; 2],
    pub admin_intr: vnic_intr,
    pub admin_poll_work: delayed_work,
    pub admin_intr_index: c_uint,
    pub link_notify_work: work_struct,
    pub admin_msg_work: work_struct,
    pub /: *mut *mut spinlock_t admin_msg_lock; / protects admin_msg_list,
    pub admin_msg_list: list_head,
    pub /: *mut *mut unsigned int admin_msg_count; / current depth of admin_msg_list,
    pub len): c_uint,
// MBOX protocol state — mbox_lock serializes admin WQ sends
    pub mbox_lock: mutex,
    pub mbox_msg_num: u64,
// MBOX request-reply state.  mbox_expected_reply is written and
// cleared by the process-context request helpers (capability/register
// unregister) and only read by the admin_msg_work receive handlers, so
// it is annotated with READ_ONCE()/WRITE_ONCE() rather than locked:
// only one request is in flight at a time (requesters run under RTNL or
// single-threaded probe/remove), so each request is serialized and its
// reply completes mbox_comp before the next request is issued.
//
    pub mbox_comp: completion,
    pub mbox_expected_reply: u8,
    pub mbox_initialized: bool,
// PF: per-VF MBOX state, allocated when SRIOV V2 is enabled
#[repr(C)]
#[derive(Copy, Clone)]
pub struct enic_vf_state {
    pub registered: bool,
    pub vf_state: *mut },
}

// wrappers function for kernel log
//

// MSIX interrupts are organized as the error interrupt, then the notify
// interrupt followed by all the I/O interrupts.  The error interrupt needs
// to fit in 7 bits due to hardware constraints
//
pub const ENIC_MSIX_RESERVED_INTR: c_int = 2;
pub const ENIC_MSIX_ERR_INTR: c_int = 0;
pub const ENIC_MSIX_NOTIFY_INTR: c_int = 1;

pub const ENIC_LEGACY_IO_INTR: c_int = 0;
pub const ENIC_LEGACY_ERR_INTR: c_int = 1;
pub const ENIC_LEGACY_NOTIFY_INTR: c_int = 2;
extern "C" {
    pub fn enic_reset_addr_lists(enic: *mut enic);
}
extern "C" {
    pub fn enic_sriov_enabled(enic: *mut enic) -> c_int;
}
extern "C" {
    pub fn enic_is_valid_vf(enic: *mut enic, vf: c_int) -> c_int;
}
extern "C" {
    pub fn enic_is_dynamic(enic: *mut enic) -> c_int;
}
extern "C" {
    pub fn enic_is_sriov_vf_v2(enic: *mut enic) -> c_int;
}
extern "C" {
    pub fn enic_set_ethtool_ops(netdev: *mut net_device);
}
extern "C" {
    pub fn __enic_set_rsskey(enic: *mut enic) -> c_int;
}
extern "C" {
    pub fn enic_ext_cq(enic: *mut enic);
}
