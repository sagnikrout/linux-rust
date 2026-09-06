//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/s390/net/qeth_core.h
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
//
// Copyright IBM Corp. 2007
// Author(s): Utz Bacher <utz.bacher@de.ibm.com>,
// Frank Pavlic <fpavlic@de.ibm.com>,
// Thomas Spatzier <tspat@de.ibm.com>,
// Frank Blaschka <frank.blaschka@de.ibm.com>
//

//
// Debug Facility stuff
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qeth_dbf_names {
    QETH_DBF_SETUP,
    QETH_DBF_MSG,
    QETH_DBF_CTRL,
    QETH_DBF_INFOS	/* must be last element */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qeth_dbf_info {
    pub name: [c_char; DEBUG_MAX_NAME_LEN],
    pub pages: c_int,
    pub areas: c_int,
    pub len: c_int,
    pub level: c_int,
    pub view: *mut debug_view,
    pub id: *mut debug_info_t,
}

pub const SENSE_COMMAND_REJECT_BYTE: c_int = 0;
pub const SENSE_COMMAND_REJECT_FLAG: c_uint = 0x80;
pub const SENSE_RESETTING_EVENT_BYTE: c_int = 1;
pub const SENSE_RESETTING_EVENT_FLAG: c_uint = 0x80;
//
// Common IO related definitions
//

// Routing stuff
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qeth_routing_info {
    pub type: qeth_routing_types,
}

// SETBRIDGEPORT stuff
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qeth_sbp_roles {
    QETH_SBP_ROLE_NONE	= 0,
    QETH_SBP_ROLE_PRIMARY	= 1,
    QETH_SBP_ROLE_SECONDARY	= 2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qeth_sbp_states {
    QETH_SBP_STATE_INACTIVE	= 0,
    QETH_SBP_STATE_STANDBY	= 1,
    QETH_SBP_STATE_ACTIVE	= 2,
}

pub const QETH_SBP_HOST_NOTIFICATION: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qeth_sbp_info {
    pub supported_funcs: __u32,
    pub role: qeth_sbp_roles,
    pub hostnotification:1: __u32,
    pub reflect_promisc:1: __u32,
    pub reflect_promisc_primary:1: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qeth_vnicc_info {
// supported/currently configured VNICCs; updated in IPA exchanges
    pub sup_chars: u32,
    pub cur_chars: u32,
// supported commands: bitmasks which VNICCs support respective cmd
    pub set_char_sup: u32,
    pub getset_timeout_sup: u32,
// timeout value for the learning characteristic
    pub learning_timeout: u32,
// characteristics wanted/configured by user
    pub wanted_chars: u32,
// has user explicitly enabled rx_bcast while online?
    pub rx_bcast_enabled: bool,
}

pub const QETH_IDX_FUNC_LEVEL_OSD: c_uint = 0x0101;
pub const QETH_IDX_FUNC_LEVEL_IQD: c_uint = 0x4108;
pub const QETH_BUFSIZE: c_int = 4096;
pub const CCW_CMD_WRITE: c_uint = 0x01;
pub const CCW_CMD_READ: c_uint = 0x02;
//
// some more defs
//

pub const QETH_MAX_PORTNO: c_int = 15;
//
// QDIO queue and buffer handling
//
pub const QETH_MAX_OUT_QUEUES: c_int = 4;

pub const QETH_IQD_MCAST_TXQ: c_int = 0;
pub const QETH_IQD_MIN_UCAST_TXQ: c_int = 1;
pub const QETH_MAX_IN_QUEUES: c_int = 2;

pub const QETH_IN_BUF_SIZE_DEFAULT: c_int = 65536;
pub const QETH_IN_BUF_COUNT_DEFAULT: c_int = 64;
pub const QETH_IN_BUF_COUNT_HSDEFAULT: c_int = 128;

// buffers we have to be behind before we get a PCI

// enqueued free buffers left before we get a PCI
pub const QETH_PCI_THRESHOLD_B(card): c_int = 0;
// not used unless the microcode gets patched
pub const QETH_PCI_TIMER_VALUE(card): c_int = 3;
// priority queing

pub const QETH_DEFAULT_QUEUE: c_int = 2;
pub const QETH_NO_PRIO_QUEUEING: c_int = 0;
pub const QETH_PRIO_Q_ING_PREC: c_int = 1;
pub const QETH_PRIO_Q_ING_TOS: c_int = 2;
pub const QETH_PRIO_Q_ING_SKB: c_int = 3;
pub const QETH_PRIO_Q_ING_VLAN: c_int = 4;
pub const QETH_PRIO_Q_ING_FIXED: c_int = 5;
// Packing
pub const QETH_LOW_WATERMARK_PACK: c_int = 2;
pub const QETH_HIGH_WATERMARK_PACK: c_int = 5;
pub const QETH_WATERMARK_PACK_FUZZ: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qeth_hdr_layer3 {
    pub id: __u8,
    pub flags: __u8,
    pub /: *mut *mut __u16 inbound_checksum; /TSO:__u16 seqno,
    pub /: *mut *mut __u32 token; /TSO: __u32 reserved,
    pub length: __u16,
    pub vlan_prio: __u8,
    pub ext_flags: __u8,
    pub vlan_id: __u16,
    pub frame_offset: __u16,
// TX:
    pub addr: in6_addr,
// RX:
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rx {
    pub res1: [u8; 2],
    pub src_mac: [u8; 6],
    pub res2: [u8; 4],
    pub vlan_id: u16,
    pub res3: [u8; 2],
    pub rx: },
    pub next_hop: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qeth_hdr_layer2 {
    pub id: __u8,
    pub flags: [__u8; 3],
    pub port_no: __u8,
    pub hdr_length: __u8,
    pub pkt_length: __u16,
    pub seq_no: __u16,
    pub vlan_id: __u16,
    pub reserved: __u32,
    pub reserved2: [__u8; 16],
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qeth_hdr {
    pub l2: qeth_hdr_layer2,
    pub l3: qeth_hdr_layer3,
    pub hdr: },
// C attribute field omitted
pub const QETH_QIB_PQUE_ORDER_RR: c_int = 0;
pub const QETH_QIB_PQUE_UNITS_SBAL: c_int = 2;
pub const QETH_QIB_PQUE_PRIO_DEFAULT: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qeth_qib_parms {
    pub pcit_magic: [c_char; 4],
    pub pcit_a: u32,
    pub pcit_b: u32,
    pub pcit_c: u32,
    pub blkt_magic: [c_char; 4],
    pub blkt_total: u32,
    pub blkt_inter_packet: u32,
    pub blkt_inter_packet_jumbo: u32,
    pub pque_magic: [c_char; 4],
    pub pque_order: u8,
    pub pque_units: u8,
    pub reserved: u16,
    pub pque_priority: [u32; 4],
}

// TCP Segmentation Offload header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qeth_hdr_ext_tso {
    pub hdr_tot_len: __u16,
    pub imb_hdr_no: __u8,
    pub reserved: __u8,
    pub hdr_type: __u8,
    pub hdr_version: __u8,
    pub hdr_len: __u16,
    pub payload_len: __u32,
    pub mss: __u16,
    pub dg_hdr_len: __u16,
    pub padding: [__u8; 16],
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qeth_hdr_tso {
    pub /*hdr->hdr.l3.xxx*/: *mut qeth_hdr hdr;,
    pub ext: qeth_hdr_ext_tso,
// C attribute field omitted
// flags for qeth_hdr.flags
pub const QETH_HDR_PASSTHRU: c_uint = 0x10;
pub const QETH_HDR_IPV6: c_uint = 0x80;
pub const QETH_HDR_CAST_MASK: c_uint = 0x07;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qeth_cast_flags {
    QETH_CAST_UNICAST   = 0x06,
    QETH_CAST_MULTICAST = 0x04,
    QETH_CAST_BROADCAST = 0x05,
    QETH_CAST_ANYCAST   = 0x07,
    QETH_CAST_NOCAST    = 0x00,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qeth_layer2_frame_flags {
    QETH_LAYER2_FLAG_MULTICAST = 0x01,
    QETH_LAYER2_FLAG_BROADCAST = 0x02,
    QETH_LAYER2_FLAG_UNICAST   = 0x04,
    QETH_LAYER2_FLAG_VLAN      = 0x10,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qeth_header_ids {
    QETH_HEADER_TYPE_LAYER3 = 0x01,
    QETH_HEADER_TYPE_LAYER2 = 0x02,
    QETH_HEADER_TYPE_L3_TSO	= 0x03,
    QETH_HEADER_TYPE_L2_TSO	= 0x06,
    QETH_HEADER_MASK_INVAL	= 0x80,
}

// flags for qeth_hdr.ext_flags
pub const QETH_HDR_EXT_VLAN_FRAME: c_uint = 0x01;
pub const QETH_HDR_EXT_TOKEN_ID: c_uint = 0x02;
pub const QETH_HDR_EXT_INCLUDE_VLAN_TAG: c_uint = 0x04;
pub const QETH_HDR_EXT_SRC_MAC_ADDR: c_uint = 0x08;
pub const QETH_HDR_EXT_CSUM_HDR_REQ: c_uint = 0x10;
pub const QETH_HDR_EXT_CSUM_TRANSP_REQ: c_uint = 0x20;
pub const QETH_HDR_EXT_UDP: c_uint = 0x40 /*bit off for TCP*/;
    pub h2->vlan_id: h1->vlan_id ==,
    pub h2->vlan_id: h1->vlan_id ==,
    pub &h2->next_hop.addr): ipv6_addr_equal(&h1->next_hop.addr,,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qeth_local_addr {
    pub hnode: hlist_node,
    pub rcu: rcu_head,
    pub addr: in6_addr,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qeth_qdio_info_states {
    QETH_QDIO_UNINITIALIZED,
    QETH_QDIO_ALLOCATED,
    QETH_QDIO_ESTABLISHED,
    QETH_QDIO_CLEANING
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qeth_buffer_pool_entry {
    pub list: list_head,
    pub init_list: list_head,
    pub elements: [*mut page; QDIO_MAX_ELEMENTS_PER_BUFFER],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qeth_qdio_buffer_pool {
    pub entry_list: list_head,
    pub buf_count: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qeth_qdio_buffer {
    pub buffer: *mut qdio_buffer,
// the buffer pool entry currently associated to this buffer
    pub pool_entry: *mut qeth_buffer_pool_entry,
    pub rx_skb: *mut sk_buff,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qeth_qdio_q {
    pub qdio_bufs: [*mut qdio_buffer; QDIO_MAX_BUFFERS_PER_Q],
    pub bufs: [qeth_qdio_buffer; QDIO_MAX_BUFFERS_PER_Q],
    pub next_buf_to_init: c_int,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qeth_qdio_out_buffer_state {
// Owned by driver, in order to be filled.
    QETH_QDIO_BUF_EMPTY,
// Filled by driver; owned by hardware in order to be sent.
    QETH_QDIO_BUF_PRIMED,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qeth_qaob_state {
    QETH_QAOB_ISSUED,
    QETH_QAOB_PENDING,
    QETH_QAOB_DONE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qeth_qaob_priv1 {
    pub state: c_uint,
    pub queue_no: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qeth_qdio_out_buffer {
    pub buffer: *mut qdio_buffer,
    pub state: core::sync::atomic::AtomicI32,
    pub next_element_to_fill: c_int,
    pub frames: c_uint,
    pub bytes: c_uint,
    pub skb_list: sk_buff_head,
    pub QDIO_MAX_ELEMENTS_PER_BUFFER): DECLARE_BITMAP(from_kmem_cache,,
    pub list_entry: list_head,
    pub aob: *mut qaob,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qeth_card_stats {
    pub rx_bufs: u64,
    pub rx_skb_csum: u64,
    pub rx_sg_skbs: u64,
    pub rx_sg_frags: u64,
    pub rx_sg_alloc_page: u64,
    pub rx_dropped_nomem: u64,
    pub rx_dropped_notsupp: u64,
    pub rx_dropped_runt: u64,
// rtnl_link_stats64
    pub rx_packets: u64,
    pub rx_bytes: u64,
    pub rx_multicast: u64,
    pub rx_length_errors: u64,
    pub rx_frame_errors: u64,
    pub rx_fifo_errors: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qeth_out_q_stats {
    pub bufs: u64,
    pub bufs_pack: u64,
    pub buf_elements: u64,
    pub skbs_pack: u64,
    pub skbs_sg: u64,
    pub skbs_csum: u64,
    pub skbs_tso: u64,
    pub skbs_linearized: u64,
    pub skbs_linearized_fail: u64,
    pub tso_bytes: u64,
    pub packing_mode_switch: u64,
    pub stopped: u64,
    pub doorbell: u64,
    pub coal_frames: u64,
    pub completion_irq: u64,
    pub completion_yield: u64,
    pub completion_timer: u64,
// rtnl_link_stats64
    pub tx_packets: u64,
    pub tx_bytes: u64,
    pub tx_errors: u64,
    pub tx_dropped: u64,
}

pub const QETH_TX_MAX_COALESCED_FRAMES: c_int = 1;
pub const QETH_TX_COALESCE_USECS: c_int = 25;
pub const QETH_TX_TIMER_USECS: c_int = 500;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qeth_qdio_out_q {
    pub qdio_bufs: [*mut qdio_buffer; QDIO_MAX_BUFFERS_PER_Q],
    pub bufs: [*mut qeth_qdio_out_buffer; QDIO_MAX_BUFFERS_PER_Q],
    pub pending_bufs: list_head,
    pub stats: qeth_out_q_stats,
    pub lock: spinlock_t,
    pub priority: c_uint,
    pub next_buf_to_fill: u8,
    pub max_elements: u8,
    pub queue_no: u8,
    pub do_pack: u8,
    pub card: *mut qeth_card,
//
// number of buffers that are currently filled (PRIMED)
// -> these buffers are hardware-owned
//
    pub used_buffers: core::sync::atomic::AtomicI32,
// indicates whether PCI flag must be set (or if one is outstanding)
    pub set_pci_flags_count: core::sync::atomic::AtomicI32,
    pub napi: napi_struct,
    pub timer: timer_list,
    pub prev_hdr: *mut qeth_hdr,
    pub coalesced_frames: c_uint,
    pub bulk_start: u8,
    pub bulk_count: u8,
    pub bulk_max: u8,
    pub coalesce_usecs: c_uint,
    pub max_coalesced_frames: c_uint,
    pub rescan_usecs: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qeth_qdio_info {
    pub state: core::sync::atomic::AtomicI32,
// input
    pub in_q: *mut qeth_qdio_q,
    pub c_q: *mut qeth_qdio_q,
    pub in_buf_pool: qeth_qdio_buffer_pool,
    pub init_pool: qeth_qdio_buffer_pool,
    pub in_buf_size: c_int,
// output
    pub no_out_queues: c_uint,
    pub out_qs: [*mut qeth_qdio_out_q; QETH_MAX_OUT_QUEUES],
// priority queueing
    pub do_prio_queueing: c_int,
    pub default_out_queue: c_int,
}

//
// channel state machine
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qeth_channel_states {
    CH_STATE_UP,
    CH_STATE_DOWN,
    CH_STATE_HALTED,
    CH_STATE_STOPPED,
}

//
// card state machine
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qeth_card_states {
    CARD_STATE_DOWN,
    CARD_STATE_SOFTSETUP,
}

//
// Protocol versions
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qeth_prot_versions {
    QETH_PROT_NONE = 0x0000,
    QETH_PROT_IPV4 = 0x0004,
    QETH_PROT_IPV6 = 0x0006,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qeth_cq {
    QETH_CQ_DISABLED = 0,
    QETH_CQ_ENABLED = 1,
    QETH_CQ_NOTAVAILABLE = 2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qeth_ipato {
    pub enabled: bool,
    pub invert4: bool,
    pub invert6: bool,
    pub entries: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qeth_channel {
    pub ccwdev: *mut ccw_device,
    pub active_cmd: *mut qeth_cmd_buffer,
    pub state: qeth_channel_states,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qeth_reply {
    pub data): c_ulong,
    pub param: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qeth_cmd_buffer {
    pub list_entry: list_head,
    pub done: completion,
    pub lock: spinlock_t,
    pub length: c_uint,
    pub ref_count: refcount_t,
    pub channel: *mut qeth_channel,
    pub reply: qeth_reply,
    pub timeout: c_long,
    pub data: *mut c_uchar,
    pub iob): *mut *mut *mut void (finalize)(struct qeth_card card, struct qeth_cmd_buffer,
    pub reply): *mut qeth_cmd_buffer,
    pub data_length): c_uint,
    pub rc: c_int,
}

//
// OSA card related definitions
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qeth_token {
    pub issuer_rm_w: __u32,
    pub issuer_rm_r: __u32,
    pub cm_filter_w: __u32,
    pub cm_filter_r: __u32,
    pub cm_connection_w: __u32,
    pub cm_connection_r: __u32,
    pub ulp_filter_w: __u32,
    pub ulp_filter_r: __u32,
    pub ulp_connection_w: __u32,
    pub ulp_connection_r: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qeth_seqno {
    pub trans_hdr: __u32,
    pub pdu_hdr: __u32,
    pub pdu_hdr_ack: __u32,
    pub ipa: __u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qeth_card_blkt {
    pub time_total: c_int,
    pub inter_packet: c_int,
    pub inter_packet_jumbo: c_int,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qeth_pnso_mode {
    QETH_PNSO_NONE,
    QETH_PNSO_BRIDGEPORT,
    QETH_PNSO_ADDR_INFO,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qeth_link_mode {
    QETH_LINK_MODE_UNKNOWN,
    QETH_LINK_MODE_FIBRE_SHORT,
    QETH_LINK_MODE_FIBRE_LONG,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qeth_link_info {
    pub speed: u32,
    pub duplex: u8,
    pub port: u8,
    pub link_mode: qeth_link_mode,
}

pub const QETH_BROADCAST_WITH_ECHO: c_uint = 0x01;
pub const QETH_BROADCAST_WITHOUT_ECHO: c_uint = 0x02;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qeth_card_info {
    pub unit_addr2: c_ushort,
    pub cula: c_ushort,
    pub func_level: __u16,
    pub 1]: char mcl_level[QETH_MCL_LENGTH +,
// doubleword below corresponds to net_if_token
    pub ddev_devno: u16,
    pub cssid: u8,
    pub iid: u8,
    pub ssid: u8,
    pub chpid: u8,
    pub chid: u16,
    pub /: *mut *mut u8 ids_valid:1; / cssid,iid,chid,
    pub dev_addr_is_registered:1: u8,
    pub promisc_mode:1: u8,
    pub use_v1_blkt:1: u8,
    pub is_vm_nic:1: u8,
// no bitfield, we take a pointer on these two:
    pub has_lp2lp_cso_v6: u8,
    pub has_lp2lp_cso_v4: u8,
    pub pnso_mode: qeth_pnso_mode,
    pub type: qeth_card_types,
    pub link_type: qeth_link_types,
    pub broadcast_capable: c_int,
    pub layer_enforced: bool,
    pub blkt: qeth_card_blkt,
    pub diagass_support: __u32,
    pub hwtrap: __u32,
    pub link_info: qeth_link_info,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qeth_discipline_id {
    QETH_DISCIPLINE_UNDETERMINED = -1,
    QETH_DISCIPLINE_LAYER3 = 0,
    QETH_DISCIPLINE_LAYER2 = 1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qeth_card_options {
    pub ipa4: qeth_ipa_caps,
    pub ipa6: qeth_ipa_caps,
    pub route4: qeth_routing_info,
    pub route6: qeth_routing_info,
    pub /: *mut *mut qeth_ipa_caps adp; / Adapter parameters,
    pub /: *mut *mut qeth_sbp_info sbp; / SETBRIDGEPORT options,
    pub /: *mut *mut qeth_vnicc_info vnicc; / VNICC options,
    pub layer: qeth_discipline_id,
    pub isolation: qeth_ipa_isolation_modes,
    pub sniffer: c_int,
    pub cq: qeth_cq,
    pub hsuid: [c_char; 9],
}

//
// thread bits for qeth_card thread masks
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qeth_threads {
    QETH_RECOVER_THREAD = 1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qeth_discipline {
    pub ): *mut *mut int (setup) (struct ccwgroup_device,
    pub ): *mut *mut void (remove) (struct ccwgroup_device,
    pub carrier_ok): *mut *mut *mut int (set_online)(struct qeth_card card, bool,
    pub card): *mut *mut void (set_offline)(struct qeth_card,
    pub cmd): *mut qeth_ipa_cmd,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qeth_addr_disposition {
    QETH_DISP_ADDR_DELETE = 0,
    QETH_DISP_ADDR_DO_NOTHING = 1,
    QETH_DISP_ADDR_ADD = 2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qeth_rx {
    pub b_count: c_int,
    pub b_index: c_int,
    pub buf_element: u8,
    pub e_offset: c_int,
    pub qdio_err: c_int,
    pub bufs_refill: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qeth_switch_info {
    pub capabilities: __u32,
    pub settings: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qeth_priv {
    pub rx_copybreak: c_uint,
    pub tx_wanted_queues: c_uint,
    pub brport_hw_features: u32,
    pub brport_features: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qeth_card {
    pub state: qeth_card_states,
    pub lock: spinlock_t,
    pub gdev: *mut ccwgroup_device,
    pub read_cmd: *mut qeth_cmd_buffer,
    pub read: qeth_channel,
    pub write: qeth_channel,
    pub data: qeth_channel,
    pub dev: *mut net_device,
    pub debugfs: *mut dentry,
    pub stats: qeth_card_stats,
    pub info: qeth_card_info,
    pub token: qeth_token,
    pub seqno: qeth_seqno,
    pub options: qeth_card_options,
    pub event_wq: *mut workqueue_struct,
    pub cmd_wq: *mut workqueue_struct,
    pub wait_q: wait_queue_head_t,
    pub ip_lock: mutex,
// protected by ip_lock:
    pub 4): DECLARE_HASHTABLE(ip_htable,,
    pub ipato: qeth_ipato,
    pub 4): DECLARE_HASHTABLE(local_addrs4,,
    pub 4): DECLARE_HASHTABLE(local_addrs6,,
    pub local_addrs4_lock: spinlock_t,
    pub local_addrs6_lock: spinlock_t,
    pub 4): DECLARE_HASHTABLE(rx_mode_addrs,,
    pub rx_mode_work: work_struct,
    pub kernel_thread_starter: work_struct,
    pub thread_mask_lock: spinlock_t,
    pub thread_start_mask: c_ulong,
    pub thread_allowed_mask: c_ulong,
    pub thread_running_mask: c_ulong,
    pub cmd_waiter_list: list_head,
// QDIO buffer handling
    pub qdio: qeth_qdio_info,
    pub read_or_write_problem: c_int,
    pub discipline: *const qeth_discipline,
    pub force_alloc_skb: core::sync::atomic::AtomicI32,
    pub qeth_service_level: service_level,
    pub ssqd: qdio_ssqd_desc,
    pub debug: *mut debug_info_t,
    pub sbp_lock: mutex,
    pub conf_mutex: mutex,
    pub discipline_mutex: mutex,
    pub napi: napi_struct,
    pub rx: qeth_rx,
    pub buffer_reclaim_work: delayed_work,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qeth_trap_id {
    pub lparnr: __u16,
    pub vmname: [c_char; 8],
    pub chpid: __u8,
    pub ssid: __u8,
    pub devno: __u16,
    pub __packed: },
    pub QETH_NO_PRIO_QUEUEING: return card->qdio.do_prio_queueing !=,
    pub netdev_priv(card->dev): *mut *mut qeth_priv priv =,
    pub card->qdio.no_out_queues): return min(card->dev->num_tx_queues,,
    pub card->qdio.no_out_queues): return min(priv->tx_wanted_queues,,
    pub 1: return dev->num_tx_queues -,
    pub QETH_IQD_MCAST_TXQ: return,
    pub txq: return,
    pub i: c_uint,
    pub i++): for (i = 0; i < elements;,
    pub qdio_buffer_element)): memset(&buf->element[i], 0, sizeof(struct,
    pub 0: buf->element[14].sflags =,
    pub 0: buf->element[15].sflags =,
//
// qeth_get_elements_for_range() -	find number of SBALEs to cover range.
// @start:				Start of the address range.
// @end:				Address after the end of the range.
//
// Returns the number of pages, and thus QDIO buffer elements, needed to cover
// the specified address range.
//
    pub PFN_DOWN(start): return PFN_UP(end) -,
    pub eth_hdr(skb)->h_dest: *mut *mut u8 addr =,
    pub RTN_UNICAST: return,
    pub skb_dst(skb): *mut *mut dst_entry dst =,
    pub rt: *mut rt6_info,
    pub dst_rt6_info(dst): rt =,
    pub rt6_get_cookie(rt)): dst = dst_check(dst,,
    pub 0): dst = dst_check(dst,,
    pub dst: return,
    pub dst_rt6_info(dst): *mut *mut rt6_info rt =,
    pub &rt->rt6i_gateway: return,
    pub &ipv6_hdr(skb)->daddr: return,
// flags |= QETH_HDR_EXT_CSUM_TRANSP_REQ;
// flags |= QETH_HDR_EXT_UDP;
    pub &card->qdio.in_buf_pool.entry_list): list_add_tail(&entry->list,,
    pub (__u32)cmd: return card->info.diagass_support &,
    pub prot): qeth_prot_versions,
// IPv4 variant
    pub QETH_PROT_IPV4): data,,
    pub QETH_PROT_IPV6): data,,
    pub qeth_l2_discipline: extern struct qeth_discipline,
    pub qeth_l3_discipline: extern struct qeth_discipline,
    pub qeth_ethtool_ops: extern struct ethtool_ops,
    pub qeth_dev_groups: [*const extern struct attribute_group; ],
    pub ): *const *const char qeth_get_cardname_short(struct qeth_card,
    pub count): *mut *mut int qeth_resize_buffer_pool(struct qeth_card card, unsigned int,
    pub disc): *mut *mut int qeth_setup_discipline(struct qeth_card card, enum qeth_discipline_id,
    pub card): *mut void qeth_remove_discipline(struct qeth_card,
// exports for qeth discipline device drivers
    pub qeth_dbf: [extern struct qeth_dbf_info; QETH_DBF_INFOS],
    pub orig): *mut *mut net_device qeth_clone_netdev(net_device,
    pub clear_start_mask): c_int,
    pub long): *mut *mut int qeth_threads_running(struct qeth_card , unsigned,
    pub resetting): bool,
    pub ): *mut c_void,
    pub data_length): c_uint,
    pub prot): qeth_prot_versions,
    pub data_length): c_uint,
    pub card): *mut int qeth_schedule_recovery(struct qeth_card,
    pub budget): *mut *mut int qeth_poll(struct napi_struct napi, int,
    pub enable): *mut *mut void qeth_setadp_promisc_mode(struct qeth_card card, bool,
    pub ): *mut int qeth_setadpparms_change_macaddr(struct qeth_card,
    pub txqueue): *mut *mut void qeth_tx_timeout(struct net_device , unsigned int,
    pub sw_info): *mut qeth_switch_info,
    pub link_info): *mut qeth_link_info,
    pub mode): qeth_ipa_isolation_modes,
    pub cmd): *mut *mut *mut int qeth_do_ioctl(struct net_device dev, struct ifreq rq, int,
    pub cmd): *mut *mut void __user data, int,
    pub ...): *mut *mut *mut void qeth_dbf_longtext(debug_info_t id, int level, char text,,
    pub qeth_cq): *mut *mut int qeth_configure_cq(struct qeth_card , enum,
    pub qeth_diags_trap_action): *mut *mut int qeth_hw_trap(struct qeth_card , enum,
    pub long): *mut *mut *mut int qeth_setassparms_cb(struct qeth_card , struct qeth_reply , unsigned,
    pub netdev_features_t): *mut *mut int qeth_set_features(struct net_device ,,
    pub dev): *mut void qeth_enable_hw_features(struct net_device,
    pub netdev_features_t): *mut *mut netdev_features_t qeth_fix_features(struct net_device ,,
    pub features): netdev_features_t,
    pub stats): *mut *mut void qeth_get_stats64(struct net_device dev, struct rtnl_link_stats64,
    pub count): *mut *mut int qeth_set_real_num_tx_queues(struct qeth_card card, unsigned int,
    pub sb_dev): *mut u8 cast_type, struct net_device,
    pub sb_dev): *mut net_device,
    pub dev): *mut int qeth_open(struct net_device,
    pub dev): *mut int qeth_stop(struct net_device,
    pub card): *mut int qeth_vm_request_mac(struct qeth_card,
    pub data_len)): __be16 proto, unsigned int,
