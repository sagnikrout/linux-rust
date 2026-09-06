//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/freescale/dpaa2/dpni.h
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


// SPDX-License-Identifier: (GPL-2.0+ OR BSD-3-Clause)
// Copyright 2013-2016 Freescale Semiconductor Inc.
// Copyright 2016 NXP
// Copyright 2020 NXP
//

// Data Path Network Interface API
// Contains initialization APIs and runtime control APIs for DPNI
//
// General DPNI macros
//
// DPNI_MAX_TC - Maximum number of traffic classes
//
pub const DPNI_MAX_TC: c_int = 8;
//
// DPNI_MAX_DPBP - Maximum number of buffer pools per DPNI
//
pub const DPNI_MAX_DPBP: c_int = 8;
//
// DPNI_ALL_TCS - All traffic classes considered; see dpni_set_queue()
//

//
// DPNI_ALL_TC_FLOWS - All flows within traffic class considered; see
// dpni_set_queue()
//

//
// DPNI_NEW_FLOW_ID - Generate new flow ID; see dpni_set_queue()
//

//
// DPNI_OPT_TX_FRM_RELEASE - Tx traffic is always released to a buffer pool on
// transmit, there are no resources allocated to have the frames confirmed back
// to the source after transmission.
//
pub const DPNI_OPT_TX_FRM_RELEASE: c_uint = 0x000001;
//
// DPNI_OPT_NO_MAC_FILTER - Disables support for MAC address filtering for
// addresses other than primary MAC address. This affects both unicast and
// multicast. Promiscuous mode can still be enabled/disabled for both unicast
// and multicast. If promiscuous mode is disabled, only traffic matching the
// primary MAC address will be accepted.
//
pub const DPNI_OPT_NO_MAC_FILTER: c_uint = 0x000002;
//
// DPNI_OPT_HAS_POLICING - Allocate policers for this DPNI. They can be used to
// rate-limit traffic per traffic class (TC) basis.
//
pub const DPNI_OPT_HAS_POLICING: c_uint = 0x000004;
//
// DPNI_OPT_SHARED_CONGESTION - Congestion can be managed in several ways,
// allowing the buffer pool to deplete on ingress, taildrop on each queue or
// use congestion groups for sets of queues. If set, it configures a single
// congestion groups across all TCs.  If reset, a congestion group is allocated
// for each TC. Only relevant if the DPNI has multiple traffic classes.
//
pub const DPNI_OPT_SHARED_CONGESTION: c_uint = 0x000008;
//
// DPNI_OPT_HAS_KEY_MASKING - Enables TCAM for Flow Steering and QoS look-ups.
// If not specified, all look-ups are exact match. Note that TCAM is not
// available on LS1088 and its variants. Setting this bit on these SoCs will
// trigger an error.
//
pub const DPNI_OPT_HAS_KEY_MASKING: c_uint = 0x000010;
//
// DPNI_OPT_NO_FS - Disables the flow steering table.
//
pub const DPNI_OPT_NO_FS: c_uint = 0x000020;
//
// DPNI_OPT_SHARED_FS - Flow steering table is shared between all traffic
// classes
//
pub const DPNI_OPT_SHARED_FS: c_uint = 0x001000;
pub const DPNI_POOL_ASSOC_QPRI: c_int = 0;
pub const DPNI_POOL_ASSOC_QDBIN: c_int = 1;
//
// struct dpni_pools_cfg - Structure representing buffer pools configuration
// @num_dpbp: Number of DPBPs
// @pool_options: Buffer assignment options.
// This field is a combination of DPNI_POOL_ASSOC_flags
// @pools: Array of buffer pools parameters; The number of valid entries
// must match 'num_dpbp' value
// @pools.dpbp_id: DPBP object ID
// @pools.priority: Priority mask that indicates TC's used with this buffer.
// If set to 0x00 MC will assume value 0xff.
// @pools.buffer_size: Buffer size
// @pools.backup_pool: Backup pool
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpni_pools_cfg {
    pub num_dpbp: u8,
    pub pool_options: u8,
    pub dpbp_id: c_int,
    pub priority_mask: u8,
    pub buffer_size: u16,
    pub backup_pool: c_int,
    pub pools: [}; DPNI_MAX_DPBP],
}

// DPNI IRQ Index and Events
pub const DPNI_IRQ_INDEX: c_int = 0;
// DPNI_IRQ_EVENT_LINK_CHANGED - indicates a change in link state
pub const DPNI_IRQ_EVENT_LINK_CHANGED: c_uint = 0x00000001;
// DPNI_IRQ_EVENT_ENDPOINT_CHANGED - indicates a change in endpoint
pub const DPNI_IRQ_EVENT_ENDPOINT_CHANGED: c_uint = 0x00000002;
//
// struct dpni_attr - Structure representing DPNI attributes
// @options: Any combination of the following options:
// DPNI_OPT_TX_FRM_RELEASE
// DPNI_OPT_NO_MAC_FILTER
// DPNI_OPT_HAS_POLICING
// DPNI_OPT_SHARED_CONGESTION
// DPNI_OPT_HAS_KEY_MASKING
// DPNI_OPT_NO_FS
// @num_queues: Number of Tx and Rx queues used for traffic distribution.
// @num_tcs: Number of traffic classes (TCs), reserved for the DPNI.
// @mac_filter_entries: Number of entries in the MAC address filtering table.
// @vlan_filter_entries: Number of entries in the VLAN address filtering table.
// @qos_entries: Number of entries in the QoS classification table.
// @fs_entries: Number of entries in the flow steering table.
// @qos_key_size: Size, in bytes, of the QoS look-up key. Defining a key larger
// than this when adding QoS entries will result in an error.
// @fs_key_size: Size, in bytes, of the flow steering look-up key. Defining a
// key larger than this when composing the hash + FS key will
// result in an error.
// @wriop_version: Version of WRIOP HW block. The 3 version values are stored
// on 6, 5, 5 bits respectively.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpni_attr {
    pub options: u32,
    pub num_queues: u8,
    pub num_tcs: u8,
    pub mac_filter_entries: u8,
    pub vlan_filter_entries: u8,
    pub qos_entries: u8,
    pub fs_entries: u16,
    pub qos_key_size: u8,
    pub fs_key_size: u8,
    pub wriop_version: u16,
}

// DPNI errors
//
// DPNI_ERROR_EOFHE - Extract out of frame header error
//
pub const DPNI_ERROR_EOFHE: c_uint = 0x00020000;
//
// DPNI_ERROR_FLE - Frame length error
//
pub const DPNI_ERROR_FLE: c_uint = 0x00002000;
//
// DPNI_ERROR_FPE - Frame physical error
//
pub const DPNI_ERROR_FPE: c_uint = 0x00001000;
//
// DPNI_ERROR_PHE - Parsing header error
//
pub const DPNI_ERROR_PHE: c_uint = 0x00000020;
//
// DPNI_ERROR_L3CE - Parser L3 checksum error
//
pub const DPNI_ERROR_L3CE: c_uint = 0x00000004;
//
// DPNI_ERROR_L4CE - Parser L3 checksum error
//
pub const DPNI_ERROR_L4CE: c_uint = 0x00000001;
//
// enum dpni_error_action - Defines DPNI behavior for errors
// @DPNI_ERROR_ACTION_DISCARD: Discard the frame
// @DPNI_ERROR_ACTION_CONTINUE: Continue with the normal flow
// @DPNI_ERROR_ACTION_SEND_TO_ERROR_QUEUE: Send the frame to the error queue
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dpni_error_action {
    DPNI_ERROR_ACTION_DISCARD = 0,
    DPNI_ERROR_ACTION_CONTINUE = 1,
    DPNI_ERROR_ACTION_SEND_TO_ERROR_QUEUE = 2
}

//
// struct dpni_error_cfg - Structure representing DPNI errors treatment
// @errors: Errors mask; use 'DPNI_ERROR__<X>
// @error_action: The desired action for the errors mask
// @set_frame_annotation: Set to '1' to mark the errors in frame annotation
// status (FAS); relevant only for the non-discard action
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpni_error_cfg {
    pub errors: u32,
    pub error_action: dpni_error_action,
    pub set_frame_annotation: c_int,
}

// DPNI buffer layout modification options
//
// DPNI_BUF_LAYOUT_OPT_TIMESTAMP - Select to modify the time-stamp setting
//
pub const DPNI_BUF_LAYOUT_OPT_TIMESTAMP: c_uint = 0x00000001;
//
// DPNI_BUF_LAYOUT_OPT_PARSER_RESULT - Select to modify the parser-result
// setting; not applicable for Tx
//
pub const DPNI_BUF_LAYOUT_OPT_PARSER_RESULT: c_uint = 0x00000002;
//
// DPNI_BUF_LAYOUT_OPT_FRAME_STATUS - Select to modify the frame-status setting
//
pub const DPNI_BUF_LAYOUT_OPT_FRAME_STATUS: c_uint = 0x00000004;
//
// DPNI_BUF_LAYOUT_OPT_PRIVATE_DATA_SIZE - Select to modify the private-data-size setting
//
pub const DPNI_BUF_LAYOUT_OPT_PRIVATE_DATA_SIZE: c_uint = 0x00000008;
//
// DPNI_BUF_LAYOUT_OPT_DATA_ALIGN - Select to modify the data-alignment setting
//
pub const DPNI_BUF_LAYOUT_OPT_DATA_ALIGN: c_uint = 0x00000010;
//
// DPNI_BUF_LAYOUT_OPT_DATA_HEAD_ROOM - Select to modify the data-head-room setting
//
pub const DPNI_BUF_LAYOUT_OPT_DATA_HEAD_ROOM: c_uint = 0x00000020;
//
// DPNI_BUF_LAYOUT_OPT_DATA_TAIL_ROOM - Select to modify the data-tail-room setting
//
pub const DPNI_BUF_LAYOUT_OPT_DATA_TAIL_ROOM: c_uint = 0x00000040;
//
// struct dpni_buffer_layout - Structure representing DPNI buffer layout
// @options: Flags representing the suggested modifications to the buffer
// layout; Use any combination of 'DPNI_BUF_LAYOUT_OPT_<X>' flags
// @pass_timestamp: Pass timestamp value
// @pass_parser_result: Pass parser results
// @pass_frame_status: Pass frame status
// @private_data_size: Size kept for private data (in bytes)
// @data_align: Data alignment
// @data_head_room: Data head room
// @data_tail_room: Data tail room
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpni_buffer_layout {
    pub options: u32,
    pub pass_timestamp: c_int,
    pub pass_parser_result: c_int,
    pub pass_frame_status: c_int,
    pub private_data_size: u16,
    pub data_align: u16,
    pub data_head_room: u16,
    pub data_tail_room: u16,
}

//
// enum dpni_queue_type - Identifies a type of queue targeted by the command
// @DPNI_QUEUE_RX: Rx queue
// @DPNI_QUEUE_TX: Tx queue
// @DPNI_QUEUE_TX_CONFIRM: Tx confirmation queue
// @DPNI_QUEUE_RX_ERR: Rx error queue
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dpni_queue_type {
    DPNI_QUEUE_RX,
    DPNI_QUEUE_TX,
    DPNI_QUEUE_TX_CONFIRM,
    DPNI_QUEUE_RX_ERR,
}

//
// enum dpni_offload - Identifies a type of offload targeted by the command
// @DPNI_OFF_RX_L3_CSUM: Rx L3 checksum validation
// @DPNI_OFF_RX_L4_CSUM: Rx L4 checksum validation
// @DPNI_OFF_TX_L3_CSUM: Tx L3 checksum generation
// @DPNI_OFF_TX_L4_CSUM: Tx L4 checksum generation
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dpni_offload {
    DPNI_OFF_RX_L3_CSUM,
    DPNI_OFF_RX_L4_CSUM,
    DPNI_OFF_TX_L3_CSUM,
    DPNI_OFF_TX_L4_CSUM,
}

pub const DPNI_STATISTICS_CNT: c_int = 7;
//
// union dpni_statistics - Union describing the DPNI statistics
// @page_0: Page_0 statistics structure
// @page_0.ingress_all_frames: Ingress frame count
// @page_0.ingress_all_bytes: Ingress byte count
// @page_0.ingress_multicast_frames: Ingress multicast frame count
// @page_0.ingress_multicast_bytes: Ingress multicast byte count
// @page_0.ingress_broadcast_frames: Ingress broadcast frame count
// @page_0.ingress_broadcast_bytes: Ingress broadcast byte count
// @page_1: Page_1 statistics structure
// @page_1.egress_all_frames: Egress frame count
// @page_1.egress_all_bytes: Egress byte count
// @page_1.egress_multicast_frames: Egress multicast frame count
// @page_1.egress_multicast_bytes: Egress multicast byte count
// @page_1.egress_broadcast_frames: Egress broadcast frame count
// @page_1.egress_broadcast_bytes: Egress broadcast byte count
// @page_2: Page_2 statistics structure
// @page_2.ingress_filtered_frames: Ingress filtered frame count
// @page_2.ingress_discarded_frames: Ingress discarded frame count
// @page_2.ingress_nobuffer_discards: Ingress discarded frame count due to
// lack of buffers
// @page_2.egress_discarded_frames: Egress discarded frame count
// @page_2.egress_confirmed_frames: Egress confirmed frame count
// @page_3: Page_3 statistics structure
// @page_3.egress_dequeue_bytes: Cumulative count of the number of bytes
// dequeued from egress FQs
// @page_3.egress_dequeue_frames: Cumulative count of the number of frames
// dequeued from egress FQs
// @page_3.egress_reject_bytes: Cumulative count of the number of bytes in
// egress frames whose enqueue was rejected
// @page_3.egress_reject_frames: Cumulative count of the number of egress
// frames whose enqueue was rejected
// @page_4: Page_4 statistics structure: congestion points
// @page_4.cgr_reject_frames: number of rejected frames due to congestion point
// @page_4.cgr_reject_bytes: number of rejected bytes due to congestion point
// @page_5: Page_5 statistics structure: policer
// @page_5.policer_cnt_red: NUmber of red colored frames
// @page_5.policer_cnt_yellow: number of yellow colored frames
// @page_5.policer_cnt_green: number of green colored frames
// @page_5.policer_cnt_re_red: number of recolored red frames
// @page_5.policer_cnt_re_yellow: number of recolored yellow frames
// @page_6: Page_6 statistics structure
// @page_6.tx_pending_frames: total number of frames pending in egress FQs
// @raw: raw statistics structure, used to index counters
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union dpni_statistics {
    pub ingress_all_frames: u64,
    pub ingress_all_bytes: u64,
    pub ingress_multicast_frames: u64,
    pub ingress_multicast_bytes: u64,
    pub ingress_broadcast_frames: u64,
    pub ingress_broadcast_bytes: u64,
    pub page_0: },
    pub egress_all_frames: u64,
    pub egress_all_bytes: u64,
    pub egress_multicast_frames: u64,
    pub egress_multicast_bytes: u64,
    pub egress_broadcast_frames: u64,
    pub egress_broadcast_bytes: u64,
    pub page_1: },
    pub ingress_filtered_frames: u64,
    pub ingress_discarded_frames: u64,
    pub ingress_nobuffer_discards: u64,
    pub egress_discarded_frames: u64,
    pub egress_confirmed_frames: u64,
    pub page_2: },
    pub egress_dequeue_bytes: u64,
    pub egress_dequeue_frames: u64,
    pub egress_reject_bytes: u64,
    pub egress_reject_frames: u64,
    pub page_3: },
    pub cgr_reject_frames: u64,
    pub cgr_reject_bytes: u64,
    pub page_4: },
    pub policer_cnt_red: u64,
    pub policer_cnt_yellow: u64,
    pub policer_cnt_green: u64,
    pub policer_cnt_re_red: u64,
    pub policer_cnt_re_yellow: u64,
    pub page_5: },
    pub tx_pending_frames: u64,
    pub page_6: },
    pub counter: [u64; DPNI_STATISTICS_CNT],
    pub raw: },
}

pub const DPNI_LINK_OPT_AUTONEG: c_uint = 0x0000000000000001ULL;
pub const DPNI_LINK_OPT_HALF_DUPLEX: c_uint = 0x0000000000000002ULL;
pub const DPNI_LINK_OPT_PAUSE: c_uint = 0x0000000000000004ULL;
pub const DPNI_LINK_OPT_ASYM_PAUSE: c_uint = 0x0000000000000008ULL;
pub const DPNI_LINK_OPT_PFC_PAUSE: c_uint = 0x0000000000000010ULL;
//
// struct dpni_link_cfg - Structure representing DPNI link configuration
// @rate: Rate
// @options: Mask of available options; use 'DPNI_LINK_OPT_<X>' values
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpni_link_cfg {
    pub rate: u32,
    pub options: u64,
}

//
// struct dpni_link_state - Structure representing DPNI link state
// @rate: Rate
// @options: Mask of available options; use 'DPNI_LINK_OPT_<X>' values
// @up: Link state; '0' for down, '1' for up
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpni_link_state {
    pub rate: u32,
    pub options: u64,
    pub up: c_int,
}

//
// enum dpni_dist_mode - DPNI distribution mode
// @DPNI_DIST_MODE_NONE: No distribution
// @DPNI_DIST_MODE_HASH: Use hash distribution; only relevant if
// the 'DPNI_OPT_DIST_HASH' option was set at DPNI creation
// @DPNI_DIST_MODE_FS:  Use explicit flow steering; only relevant if
// the 'DPNI_OPT_DIST_FS' option was set at DPNI creation
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dpni_dist_mode {
    DPNI_DIST_MODE_NONE = 0,
    DPNI_DIST_MODE_HASH = 1,
    DPNI_DIST_MODE_FS = 2
}

//
// enum dpni_fs_miss_action -   DPNI Flow Steering miss action
// @DPNI_FS_MISS_DROP: In case of no-match, drop the frame
// @DPNI_FS_MISS_EXPLICIT_FLOWID: In case of no-match, use explicit flow-id
// @DPNI_FS_MISS_HASH: In case of no-match, distribute using hash
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dpni_fs_miss_action {
    DPNI_FS_MISS_DROP = 0,
    DPNI_FS_MISS_EXPLICIT_FLOWID = 1,
    DPNI_FS_MISS_HASH = 2
}

//
// struct dpni_fs_tbl_cfg - Flow Steering table configuration
// @miss_action: Miss action selection
// @default_flow_id: Used when 'miss_action = DPNI_FS_MISS_EXPLICIT_FLOWID'
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpni_fs_tbl_cfg {
    pub miss_action: dpni_fs_miss_action,
    pub default_flow_id: u16,
}

//
// struct dpni_rx_tc_dist_cfg - Rx traffic class distribution configuration
// @dist_size: Set the distribution size;
// supported values: 1,2,3,4,6,7,8,12,14,16,24,28,32,48,56,64,96,
// 112,128,192,224,256,384,448,512,768,896,1024
// @dist_mode: Distribution mode
// @key_cfg_iova: I/O virtual address of 256 bytes DMA-able memory filled with
// the extractions to be used for the distribution key by calling
// dpni_prepare_key_cfg() relevant only when
// 'dist_mode != DPNI_DIST_MODE_NONE', otherwise it can be '0'
// @fs_cfg: Flow Steering table configuration; only relevant if
// 'dist_mode = DPNI_DIST_MODE_FS'
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpni_rx_tc_dist_cfg {
    pub dist_size: u16,
    pub dist_mode: dpni_dist_mode,
    pub key_cfg_iova: u64,
    pub fs_cfg: dpni_fs_tbl_cfg,
}

//
// DPNI_FS_MISS_DROP - When used for fs_miss_flow_id in function
// dpni_set_rx_dist, will signal to dpni to drop all unclassified frames
//

//
// struct dpni_rx_dist_cfg - Rx distribution configuration
// @dist_size:	distribution size
// @key_cfg_iova: I/O virtual address of 256 bytes DMA-able memory filled with
// the extractions to be used for the distribution key by calling
// dpni_prepare_key_cfg(); relevant only when enable!=0 otherwise
// it can be '0'
// @enable: enable/disable the distribution.
// @tc: TC id for which distribution is set
// @fs_miss_flow_id: when packet misses all rules from flow steering table and
// hash is disabled it will be put into this queue id; use
// DPNI_FS_MISS_DROP to drop frames. The value of this field is
// used only when flow steering distribution is enabled and hash
// distribution is disabled
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpni_rx_dist_cfg {
    pub dist_size: u16,
    pub key_cfg_iova: u64,
    pub enable: u8,
    pub tc: u8,
    pub fs_miss_flow_id: u16,
}

//
// struct dpni_qos_tbl_cfg - Structure representing QOS table configuration
// @key_cfg_iova: I/O virtual address of 256 bytes DMA-able memory filled with
// key extractions to be used as the QoS criteria by calling
// dpkg_prepare_key_cfg()
// @discard_on_miss: Set to '1' to discard frames in case of no match (miss);
// '0' to use the 'default_tc' in such cases
// @default_tc: Used in case of no-match and 'discard_on_miss'= 0
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpni_qos_tbl_cfg {
    pub key_cfg_iova: u64,
    pub discard_on_miss: c_int,
    pub default_tc: u8,
}

//
// enum dpni_dest - DPNI destination types
// @DPNI_DEST_NONE: Unassigned destination; The queue is set in parked mode and
// does not generate FQDAN notifications; user is expected to
// dequeue from the queue based on polling or other user-defined
// method
// @DPNI_DEST_DPIO: The queue is set in schedule mode and generates FQDAN
// notifications to the specified DPIO; user is expected to dequeue
// from the queue only after notification is received
// @DPNI_DEST_DPCON: The queue is set in schedule mode and does not generate
// FQDAN notifications, but is connected to the specified DPCON
// object; user is expected to dequeue from the DPCON channel
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dpni_dest {
    DPNI_DEST_NONE = 0,
    DPNI_DEST_DPIO = 1,
    DPNI_DEST_DPCON = 2
}

//
// struct dpni_queue - Queue structure
// @destination: - Destination structure
// @destination.id: ID of the destination, only relevant if DEST_TYPE is > 0.
// Identifies either a DPIO or a DPCON object.
// Not relevant for Tx queues.
// @destination.type:	May be one of the following:
// 0 - No destination, queue can be manually
// queried, but will not push traffic or
// notifications to a DPIO;
// 1 - The destination is a DPIO. When traffic
// becomes available in the queue a FQDAN
// (FQ data available notification) will be
// generated to selected DPIO;
// 2 - The destination is a DPCON. The queue is
// associated with a DPCON object for the
// purpose of scheduling between multiple
// queues. The DPCON may be independently
// configured to generate notifications.
// Not relevant for Tx queues.
// @destination.hold_active: Hold active, maintains a queue scheduled for longer
// in a DPIO during dequeue to reduce spread of traffic.
// Only relevant if queues are
// not affined to a single DPIO.
// @user_context: User data, presented to the user along with any frames
// from this queue. Not relevant for Tx queues.
// @flc: FD FLow Context structure
// @flc.value: Default FLC value for traffic dequeued from
// this queue.  Please check description of FD
// structure for more information.
// Note that FLC values set using dpni_add_fs_entry,
// if any, take precedence over values per queue.
// @flc.stash_control: Boolean, indicates whether the 6 lowest
// - significant bits are used for stash control.
// significant bits are used for stash control.  If set, the 6
// least significant bits in value are interpreted as follows:
// - bits 0-1: indicates the number of 64 byte units of context
// that are stashed.  FLC value is interpreted as a memory address
// in this case, excluding the 6 LS bits.
// - bits 2-3: indicates the number of 64 byte units of frame
// annotation to be stashed.  Annotation is placed at FD[ADDR].
// - bits 4-5: indicates the number of 64 byte units of frame
// data to be stashed.  Frame data is placed at FD[ADDR] +
// FD[OFFSET].
// For more details check the Frame Descriptor section in the
// hardware documentation.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpni_queue {
    pub id: u16,
    pub type: dpni_dest,
    pub hold_active: c_char,
    pub priority: u8,
    pub destination: },
    pub user_context: u64,
    pub value: u64,
    pub stash_control: c_char,
    pub flc: },
}

//
// struct dpni_queue_id - Queue identification, used for enqueue commands
// or queue control
// @fqid: FQID used for enqueueing to and/or configuration of this specific FQ
// @qdbin: Queueing bin, used to enqueue using QDID, DQBIN, QPRI. Only relevant
// for Tx queues.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpni_queue_id {
    pub fqid: u32,
    pub qdbin: u16,
}

// Set User Context
pub const DPNI_QUEUE_OPT_USER_CTX: c_uint = 0x00000001;
pub const DPNI_QUEUE_OPT_DEST: c_uint = 0x00000002;
pub const DPNI_QUEUE_OPT_FLC: c_uint = 0x00000004;
pub const DPNI_QUEUE_OPT_HOLD_ACTIVE: c_uint = 0x00000008;
//
// enum dpni_congestion_unit - DPNI congestion units
// @DPNI_CONGESTION_UNIT_BYTES: bytes units
// @DPNI_CONGESTION_UNIT_FRAMES: frames units
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dpni_congestion_unit {
    DPNI_CONGESTION_UNIT_BYTES = 0,
    DPNI_CONGESTION_UNIT_FRAMES
}

//
// enum dpni_congestion_point - Structure representing congestion point
// @DPNI_CP_QUEUE: Set taildrop per queue, identified by QUEUE_TYPE, TC and
// QUEUE_INDEX
// @DPNI_CP_GROUP: Set taildrop per queue group. Depending on options used to
// define the DPNI this can be either per TC (default) or per
// interface (DPNI_OPT_SHARED_CONGESTION set at DPNI create).
// QUEUE_INDEX is ignored if this type is used.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dpni_congestion_point {
    DPNI_CP_QUEUE,
    DPNI_CP_GROUP,
}

//
// struct dpni_dest_cfg - Structure representing DPNI destination parameters
// @dest_type:	Destination type
// @dest_id:	Either DPIO ID or DPCON ID, depending on the destination type
// @priority:	Priority selection within the DPIO or DPCON channel; valid
// values are 0-1 or 0-7, depending on the number of priorities
// in that channel; not relevant for 'DPNI_DEST_NONE' option
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpni_dest_cfg {
    pub dest_type: dpni_dest,
    pub dest_id: c_int,
    pub priority: u8,
}

// DPNI congestion options
//
// DPNI_CONG_OPT_FLOW_CONTROL - This congestion will trigger flow control or
// priority flow control.  This will have effect only if flow control is
// enabled with dpni_set_link_cfg().
//
pub const DPNI_CONG_OPT_FLOW_CONTROL: c_uint = 0x00000040;
//
// struct dpni_congestion_notification_cfg - congestion notification
// configuration
// @units: Units type
// @threshold_entry: Above this threshold we enter a congestion state.
// set it to '0' to disable it
// @threshold_exit: Below this threshold we exit the congestion state.
// @message_ctx: The context that will be part of the CSCN message
// @message_iova: I/O virtual address (must be in DMA-able memory),
// must be 16B aligned; valid only if 'DPNI_CONG_OPT_WRITE_MEM_<X>'
// is contained in 'options'
// @dest_cfg: CSCN can be send to either DPIO or DPCON WQ channel
// @notification_mode: Mask of available options; use 'DPNI_CONG_OPT_<X>' values
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpni_congestion_notification_cfg {
    pub units: dpni_congestion_unit,
    pub threshold_entry: u32,
    pub threshold_exit: u32,
    pub message_ctx: u64,
    pub message_iova: u64,
    pub dest_cfg: dpni_dest_cfg,
    pub notification_mode: u16,
}

//
// struct dpni_taildrop - Structure representing the taildrop
// @enable:	Indicates whether the taildrop is active or not.
// @units:	Indicates the unit of THRESHOLD. Queue taildrop only supports
// byte units, this field is ignored and assumed = 0 if
// CONGESTION_POINT is 0.
// @threshold:	Threshold value, in units identified by UNITS field. Value 0
// cannot be used as a valid taildrop threshold, THRESHOLD must
// be > 0 if the taildrop is enabled.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpni_taildrop {
    pub enable: c_char,
    pub units: dpni_congestion_unit,
    pub threshold: u32,
}

//
// struct dpni_rule_cfg - Rule configuration for table lookup
// @key_iova: I/O virtual address of the key (must be in DMA-able memory)
// @mask_iova: I/O virtual address of the mask (must be in DMA-able memory)
// @key_size: key and mask size (in bytes)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpni_rule_cfg {
    pub key_iova: u64,
    pub mask_iova: u64,
    pub key_size: u8,
}

//
// DPNI_FS_OPT_DISCARD - Discard matching traffic. If set, this takes
// precedence over any other configuration and matching traffic is always
// discarded.
//
pub const DPNI_FS_OPT_DISCARD: c_uint = 0x1;
//
// DPNI_FS_OPT_SET_FLC - Set FLC value. If set, flc member of struct
// dpni_fs_action_cfg is used to override the FLC value set per queue.
// For more details check the Frame Descriptor section in the hardware
// documentation.
//
pub const DPNI_FS_OPT_SET_FLC: c_uint = 0x2;
//
// DPNI_FS_OPT_SET_STASH_CONTROL - Indicates whether the 6 lowest significant
// bits of FLC are used for stash control. If set, the 6 least significant bits
// in value are interpreted as follows:
// - bits 0-1: indicates the number of 64 byte units of context that are
// stashed. FLC value is interpreted as a memory address in this case,
// excluding the 6 LS bits.
// - bits 2-3: indicates the number of 64 byte units of frame annotation
// to be stashed. Annotation is placed at FD[ADDR].
// - bits 4-5: indicates the number of 64 byte units of frame data to be
// stashed. Frame data is placed at FD[ADDR] + FD[OFFSET].
// This flag is ignored if DPNI_FS_OPT_SET_FLC is not specified.
//
pub const DPNI_FS_OPT_SET_STASH_CONTROL: c_uint = 0x4;
//
// struct dpni_fs_action_cfg - Action configuration for table look-up
// @flc:	FLC value for traffic matching this rule. Please check the
// Frame Descriptor section in the hardware documentation for
// more information.
// @flow_id:	Identifies the Rx queue used for matching traffic. Supported
// values are in range 0 to num_queue-1.
// @options:	Any combination of DPNI_FS_OPT_ values.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpni_fs_action_cfg {
    pub flc: u64,
    pub flow_id: u16,
    pub options: u16,
}

//
// struct dpni_tx_shaping_cfg - Structure representing DPNI tx shaping configuration
// @rate_limit:		Rate in Mbps
// @max_burst_size:	Burst size in bytes (up to 64KB)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpni_tx_shaping_cfg {
    pub rate_limit: u32,
    pub max_burst_size: u16,
}

//
// struct dpni_single_step_cfg - configure single step PTP (IEEE 1588)
// @en:		enable single step PTP. When enabled the PTPv1 functionality
// will not work. If the field is zero, offset and ch_update
// parameters will be ignored
// @offset:	start offset from the beginning of the frame where
// timestamp field is found. The offset must respect all MAC
// headers, VLAN tags and other protocol headers
// @ch_update:	when set UDP checksum will be updated inside packet
// @peer_delay:	For peer-to-peer transparent clocks add this value to the
// correction field in addition to the transient time update.
// The value expresses nanoseconds.
// @ptp_onestep_reg_base: 1588 SINGLE_STEP register base address. This address
// is used to update directly the register contents.
// User has to create an address mapping for it.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpni_single_step_cfg {
    pub en: u8,
    pub ch_update: u8,
    pub offset: u16,
    pub peer_delay: u32,
    pub ptp_onestep_reg_base: u32,
}
