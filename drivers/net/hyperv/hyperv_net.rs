//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/hyperv/hyperv_net.h
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
// Copyright (c) 2011, Microsoft Corporation.
//
// Authors:
// Haiyang Zhang <haiyangz@microsoft.com>
// Hank Janssen  <hjanssen@microsoft.com>
// K. Y. Srinivasan <kys@microsoft.com>
//

// RSS related
pub const OID_GEN_RECEIVE_SCALE_CAPABILITIES: c_uint = 0x00010203  /* query only */;
pub const OID_GEN_RECEIVE_SCALE_PARAMETERS: c_uint = 0x00010204  /* query and set */;
pub const NDIS_OBJECT_TYPE_RSS_CAPABILITIES: c_uint = 0x88;
pub const NDIS_OBJECT_TYPE_RSS_PARAMETERS: c_uint = 0x89;
pub const NDIS_OBJECT_TYPE_OFFLOAD: c_uint = 0xa7;
pub const NDIS_RECEIVE_SCALE_CAPABILITIES_REVISION_2: c_int = 2;
pub const NDIS_RECEIVE_SCALE_PARAMETERS_REVISION_2: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ndis_obj_header {
    pub type: u8,
    pub rev: u8,
    pub size: u16,
    pub __packed: },
// ndis_recv_scale_cap/cap_flag
pub const NDIS_RSS_CAPS_MESSAGE_SIGNALED_INTERRUPTS: c_uint = 0x01000000;
pub const NDIS_RSS_CAPS_CLASSIFICATION_AT_ISR: c_uint = 0x02000000;
pub const NDIS_RSS_CAPS_CLASSIFICATION_AT_DPC: c_uint = 0x04000000;
pub const NDIS_RSS_CAPS_USING_MSI_X: c_uint = 0x08000000;
pub const NDIS_RSS_CAPS_RSS_AVAILABLE_ON_PORTS: c_uint = 0x10000000;
pub const NDIS_RSS_CAPS_SUPPORTS_MSI_X: c_uint = 0x20000000;
pub const NDIS_RSS_CAPS_HASH_TYPE_TCP_IPV4: c_uint = 0x00000100;
pub const NDIS_RSS_CAPS_HASH_TYPE_TCP_IPV6: c_uint = 0x00000200;
pub const NDIS_RSS_CAPS_HASH_TYPE_TCP_IPV6_EX: c_uint = 0x00000400;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ndis_recv_scale_cap {
    pub hdr: ndis_obj_header,
    pub cap_flag: u32,
    pub num_int_msg: u32,
    pub num_recv_que: u32,
    pub num_indirect_tabent: u16,
    pub __packed: },
// ndis_recv_scale_param flags
pub const NDIS_RSS_PARAM_FLAG_BASE_CPU_UNCHANGED: c_uint = 0x0001;
pub const NDIS_RSS_PARAM_FLAG_HASH_INFO_UNCHANGED: c_uint = 0x0002;
pub const NDIS_RSS_PARAM_FLAG_ITABLE_UNCHANGED: c_uint = 0x0004;
pub const NDIS_RSS_PARAM_FLAG_HASH_KEY_UNCHANGED: c_uint = 0x0008;
pub const NDIS_RSS_PARAM_FLAG_DISABLE_RSS: c_uint = 0x0010;
// Hash info bits
pub const NDIS_HASH_FUNC_TOEPLITZ: c_uint = 0x00000001;
pub const NDIS_HASH_IPV4: c_uint = 0x00000100;
pub const NDIS_HASH_TCP_IPV4: c_uint = 0x00000200;
pub const NDIS_HASH_IPV6: c_uint = 0x00000400;
pub const NDIS_HASH_IPV6_EX: c_uint = 0x00000800;
pub const NDIS_HASH_TCP_IPV6: c_uint = 0x00001000;
pub const NDIS_HASH_TCP_IPV6_EX: c_uint = 0x00002000;

pub const NDIS_RSS_HASH_SECRET_KEY_MAX_SIZE_REVISION_2: c_int = 40;
pub const ITAB_NUM: c_int = 128;
pub const ITAB_NUM_MAX: c_int = 256;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ndis_recv_scale_param {
    pub hdr: ndis_obj_header,
// Qualifies the rest of the information
    pub flag: u16,
// The base CPU number to do receive processing. not used
    pub base_cpu_number: u16,
// This describes the hash function and type being enabled
    pub hashinfo: u32,
// The size of indirection table array
    pub indirect_tabsize: u16,
// The offset of the indirection table from the beginning of this
// structure
//
    pub indirect_taboffset: u32,
// The size of the hash secret key
    pub hashkey_size: u16,
// The offset of the secret key from the beginning of this structure
    pub hashkey_offset: u32,
    pub processor_masks_offset: u32,
    pub num_processor_masks: u32,
    pub processor_masks_entry_size: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ndis_tcp_ip_checksum_info {
    pub is_ipv4:1: u32,
    pub is_ipv6:1: u32,
    pub tcp_checksum:1: u32,
    pub udp_checksum:1: u32,
    pub ip_header_checksum:1: u32,
    pub reserved:11: u32,
    pub tcp_header_offset:10: u32,
    pub transmit: },
    pub tcp_checksum_failed:1: u32,
    pub udp_checksum_failed:1: u32,
    pub ip_checksum_failed:1: u32,
    pub tcp_checksum_succeeded:1: u32,
    pub udp_checksum_succeeded:1: u32,
    pub ip_checksum_succeeded:1: u32,
    pub loopback:1: u32,
    pub tcp_checksum_value_invalid:1: u32,
    pub ip_checksum_value_invalid:1: u32,
    pub receive: },
    pub value: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ndis_pkt_8021q_info {
    pub /: *mut *mut u32 pri:3; / User Priority,
    pub /: *mut *mut u32 cfi:1; / Canonical Format ID,
    pub /: *mut *mut u32 vlanid:12; / VLAN ID,
    pub reserved:16: u32,
}

//
// Represent netvsc packet which contains 1 RNDIS and 1 ethernet frame
// within the RNDIS
//
// The size of this structure is less than 48 bytes and we can now
// place this structure in the skb->cb field.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_netvsc_packet {
// Bookkeeping stuff
    pub /: *mut *mut u8 cp_partial; / partial copy into send buffer,
    pub /: *mut *mut u8 rmsg_size; / RNDIS header and PPI size,
    pub page_buf_cnt: u8,
    pub q_idx: u16,
    pub total_packets: u16,
    pub total_bytes: u32,
    pub send_buf_index: u32,
    pub total_data_buflen: u32,
    pub dma_range: *mut hv_dma_range,
}

pub const NETVSC_HASH_KEYLEN: c_int = 40;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct netvsc_device_info {
    pub mac_adr: [c_uchar; ETH_ALEN],
    pub num_chn: u32,
    pub send_sections: u32,
    pub recv_sections: u32,
    pub send_section_size: u32,
    pub recv_section_size: u32,
    pub bprog: *mut bpf_prog,
    pub rss_key: [u8; NETVSC_HASH_KEYLEN],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rndis_device_state {
    RNDIS_DEV_UNINITIALIZED = 0,
    RNDIS_DEV_INITIALIZING,
    RNDIS_DEV_INITIALIZED,
    RNDIS_DEV_DATAINITIALIZED,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rndis_device {
    pub ndev: *mut net_device,
    pub state: rndis_device_state,
    pub new_req_id: core::sync::atomic::AtomicI32,
    pub request_lock: spinlock_t,
    pub req_list: list_head,
    pub mcast_work: work_struct,
    pub filter: u32,
    pub /: *mut *mut bool link_state; / 0 - link up, 1 - link down,
    pub hw_mac_adr: [u8; ETH_ALEN],
    pub rss_key: [u8; NETVSC_HASH_KEYLEN],
}

// Interface
extern "C" {
    pub fn netvsc_workqueue_init() -> c_int;
}
extern "C" {
    pub fn netvsc_workqueue_destroy();
}
extern "C" {
    pub fn netvsc_alloc_recv_comp_ring(net_device: *mut netvsc_device, q_idx: u32) -> c_int;
}
extern "C" {
    pub fn netvsc_device_remove(device: *mut hv_device);
}
extern "C" {
    pub fn netvsc_channel_cb(context: *mut c_void);
}
extern "C" {
    pub fn netvsc_poll(napi: *mut napi_struct, budget: c_int) -> c_int;
}
extern "C" {
    pub fn netvsc_xdp_xmit(skb: *mut sk_buff, ndev: *mut net_device);
}
extern "C" {
    pub fn netvsc_xdp_fraglen(len: c_uint) -> c_uint;
}
extern "C" {
    pub fn netvsc_vf_setxdp(vf_netdev: *mut net_device, prog: *mut bpf_prog) -> c_int;
}
extern "C" {
    pub fn netvsc_bpf(dev: *mut net_device, bpf: *mut netdev_bpf) -> c_int;
}
extern "C" {
    pub fn rndis_filter_open(nvdev: *mut netvsc_device) -> c_int;
}
extern "C" {
    pub fn rndis_filter_close(nvdev: *mut netvsc_device) -> c_int;
}
extern "C" {
    pub fn rndis_filter_update(nvdev: *mut netvsc_device);
}
extern "C" {
    pub fn netvsc_switch_datapath(nv_dev: *mut net_device, vf: bool) -> c_int;
}

pub const NVSP_PROTOCOL_VERSION_1: c_int = 2;
pub const NVSP_PROTOCOL_VERSION_2: c_uint = 0x30002;
pub const NVSP_PROTOCOL_VERSION_4: c_uint = 0x40000;
pub const NVSP_PROTOCOL_VERSION_5: c_uint = 0x50000;
pub const NVSP_PROTOCOL_VERSION_6: c_uint = 0x60000;
pub const NVSP_PROTOCOL_VERSION_61: c_uint = 0x60001;
// Init Messages
// Version 1 Messages
// Version 2 messages
// Version 4 messages
// Version 5 messages
// Version 6 messages
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvsp_message_header {
    pub msg_type: u32,
}

// Init Messages
//
// This message is used by the VSC to initialize the channel after the channels
// has been opened. This message should never include anything other then
// versioning (i.e. this message will be the same for ever).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvsp_message_init {
    pub min_protocol_ver: u32,
    pub max_protocol_ver: u32,
    pub __packed: },
//
// This message is used by the VSP to complete the initialization of the
// channel. This message should never include anything other then versioning
// (i.e. this message will be the same for ever).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvsp_message_init_complete {
    pub negotiated_protocol_ver: u32,
    pub max_mdl_chain_len: u32,
    pub status: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub union nvsp_message_init_uber {
    pub init: nvsp_message_init,
    pub init_complete: nvsp_message_init_complete,
    pub __packed: },
// Version 1 Messages
//
// This message is used by the VSC to send the NDIS version to the VSP. The VSP
// can use this information when handling OIDs sent by the VSC.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvsp_1_message_send_ndis_version {
    pub ndis_major_ver: u32,
    pub ndis_minor_ver: u32,
    pub __packed: },
//
// This message is used by the VSC to send a receive buffer to the VSP. The VSP
// can then use the receive buffer to send data to the VSC.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvsp_1_message_send_receive_buffer {
    pub gpadl_handle: u32,
    pub id: u16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvsp_1_receive_buffer_section {
    pub offset: u32,
    pub sub_alloc_size: u32,
    pub num_sub_allocs: u32,
    pub end_offset: u32,
    pub __packed: },
//
// This message is used by the VSP to acknowledge a receive buffer send by the
// VSC. This message must be sent by the VSP before the VSP uses the receive
// buffer.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvsp_1_message_send_receive_buffer_complete {
    pub status: u32,
    pub num_sections: u32,
//
// The receive buffer is split into two parts, a large suballocation
// section and a small suballocation section. These sections are then
// suballocated by a certain size.
//
// For example, the following break up of the receive buffer has 6
// large suballocations and 10 small suballocations.
//
// |            Large Section          |  |   Small Section   |
// ------------------------------------------------------------
// |     |     |     |     |     |     |  | | | | | | | | | | |
// |                                      |
// LargeOffset                            SmallOffset
//
    pub sections: [nvsp_1_receive_buffer_section; ],
    pub __packed: },
//
// This message is sent by the VSC to revoke the receive buffer.  After the VSP
// completes this transaction, the vsp should never use the receive buffer
// again.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvsp_1_message_revoke_receive_buffer {
    pub id: u16,
}

//
// This message is used by the VSC to send a send buffer to the VSP. The VSC
// can then use the send buffer to send data to the VSP.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvsp_1_message_send_send_buffer {
    pub gpadl_handle: u32,
    pub id: u16,
    pub __packed: },
//
// This message is used by the VSP to acknowledge a send buffer sent by the
// VSC. This message must be sent by the VSP before the VSP uses the sent
// buffer.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvsp_1_message_send_send_buffer_complete {
    pub status: u32,
//
// The VSC gets to choose the size of the send buffer and the VSP gets
// to choose the sections size of the buffer.  This was done to enable
// dynamic reconfigurations when the cost of GPA-direct buffers
// decreases.
//
    pub section_size: u32,
    pub __packed: },
//
// This message is sent by the VSC to revoke the send buffer.  After the VSP
// completes this transaction, the vsp should never use the send buffer again.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvsp_1_message_revoke_send_buffer {
    pub id: u16,
}

//
// This message is used by both the VSP and the VSC to send a RNDIS message to
// the opposite channel endpoint.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvsp_1_message_send_rndis_packet {
//
// This field is specified by RNDIS. They assume there's two different
// channels of communication. However, the Network VSP only has one.
// Therefore, the channel travels with the RNDIS packet.
//
    pub channel_type: u32,
//
// This field is used to send part or all of the data through a send
// buffer. This values specifies an index into the send buffer. If the
// index is 0xFFFFFFFF, then the send buffer is not being used and all
// of the data was sent through other VMBus mechanisms.
//
    pub send_buf_section_index: u32,
    pub send_buf_section_size: u32,
    pub __packed: },
//
// This message is used by both the VSP and the VSC to complete a RNDIS message
// to the opposite channel endpoint. At this point, the initiator of this
// message cannot use any resources associated with the original RNDIS packet.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvsp_1_message_send_rndis_packet_complete {
    pub status: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union nvsp_1_message_uber {
    pub send_ndis_ver: nvsp_1_message_send_ndis_version,
    pub send_recv_buf: nvsp_1_message_send_receive_buffer,
    pub revoke_recv_buf: nvsp_1_message_revoke_receive_buffer,
    pub send_send_buf: nvsp_1_message_send_send_buffer,
    pub send_send_buf_complete: nvsp_1_message_send_send_buffer_complete,
    pub revoke_send_buf: nvsp_1_message_revoke_send_buffer,
    pub send_rndis_pkt: nvsp_1_message_send_rndis_packet,
    pub __packed: },
//
// Network VSP protocol version 2 messages:
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvsp_2_vsc_capability {
    pub data: u64,
    pub vmq:1: u64,
    pub chimney:1: u64,
    pub sriov:1: u64,
    pub ieee8021q:1: u64,
    pub correlation_id:1: u64,
    pub teaming:1: u64,
    pub vsubnetid:1: u64,
    pub rsc:1: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvsp_2_send_ndis_config {
    pub mtu: u32,
    pub reserved: u32,
    pub capability: nvsp_2_vsc_capability,
    pub __packed: },
// Allocate receive buffer
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvsp_2_alloc_rxbuf {
// Allocation ID to match the allocation request and response
    pub alloc_id: u32,
// Length of the VM shared memory receive buffer that needs to
// be allocated
//
    pub len: u32,
    pub __packed: },
// Allocate receive buffer complete
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvsp_2_alloc_rxbuf_comp {
// The NDIS_STATUS code for buffer allocation
    pub status: u32,
    pub alloc_id: u32,
// GPADL handle for the allocated receive buffer
    pub gpadl_handle: u32,
// Receive buffer ID
    pub recv_buf_id: u64,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvsp_2_free_rxbuf {
    pub recv_buf_id: u64,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub union nvsp_2_message_uber {
    pub send_ndis_config: nvsp_2_send_ndis_config,
    pub alloc_rxbuf: nvsp_2_alloc_rxbuf,
    pub alloc_rxbuf_comp: nvsp_2_alloc_rxbuf_comp,
    pub free_rxbuf: nvsp_2_free_rxbuf,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvsp_4_send_vf_association {
// 1: allocated, serial number is valid. 0: not allocated
    pub allocated: u32,
// Serial number of the VF to team with
    pub serial: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nvsp_vm_datapath {
    NVSP_DATAPATH_SYNTHETIC = 0,
    NVSP_DATAPATH_VF,
    NVSP_DATAPATH_MAX
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvsp_4_sw_datapath {
    pub /: *mut *mut u32 active_datapath; / active data path in VM,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub union nvsp_4_message_uber {
    pub vf_assoc: nvsp_4_send_vf_association,
    pub active_dp: nvsp_4_sw_datapath,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nvsp_subchannel_operation {
    NVSP_SUBCHANNEL_NONE = 0,
    NVSP_SUBCHANNEL_ALLOCATE,
    NVSP_SUBCHANNEL_MAX
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvsp_5_subchannel_request {
    pub op: u32,
    pub num_subchannels: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvsp_5_subchannel_complete {
    pub status: u32,
    pub /: *mut *mut u32 num_subchannels; / Actual number of subchannels allocated,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvsp_5_send_indirect_table {
// The number of entries in the send indirection table
    pub count: u32,
// The offset of the send indirection table from the beginning of
// struct nvsp_message.
// The send indirection table tells which channel to put the send
// traffic on. Each entry is a channel number.
//
    pub offset: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub union nvsp_5_message_uber {
    pub subchn_req: nvsp_5_subchannel_request,
    pub subchn_comp: nvsp_5_subchannel_complete,
    pub send_table: nvsp_5_send_indirect_table,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nvsp_6_pd_api_op {
    PD_API_OP_CONFIG = 1,
    PD_API_OP_SW_DATAPATH, /* Switch Datapath */
    PD_API_OP_OPEN_PROVIDER,
    PD_API_OP_CLOSE_PROVIDER,
    PD_API_OP_CREATE_QUEUE,
    PD_API_OP_FLUSH_QUEUE,
    PD_API_OP_FREE_QUEUE,
    PD_API_OP_ALLOC_COM_BUF, /* Allocate Common Buffer */
    PD_API_OP_FREE_COM_BUF, /* Free Common Buffer */
    PD_API_OP_MAX
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct grp_affinity {
    pub mask: u64,
    pub grp: u16,
    pub reserved: [u16; 3],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvsp_6_pd_api_req {
    pub op: u32,
// MMIO information is sent from the VM to VSP
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __packed {
    pub /: *mut *mut u64 mmio_pa; / MMIO Physical Address,
    pub mmio_len: u32,
// Number of PD queues a VM can support
    pub num_subchn: u16,
    pub config: },
// Switch Datapath
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __packed {
// Host Datapath Is PacketDirect
    pub host_dpath_is_pd: u8,
// Guest PacketDirect Is Enabled
    pub guest_pd_enabled: u8,
    pub sw_dpath: },
// Open Provider
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __packed {
    pub /: *mut *mut u32 prov_id; / Provider id,
    pub flag: u32,
    pub open_prov: },
// Close Provider
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __packed {
    pub prov_id: u32,
    pub cls_prov: },
// Create Queue
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __packed {
    pub prov_id: u32,
    pub q_id: u16,
    pub q_size: u16,
    pub is_recv_q: u8,
    pub is_rss_q: u8,
    pub recv_data_len: u32,
    pub affy: grp_affinity,
    pub cr_q: },
// Delete Queue
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __packed {
    pub prov_id: u32,
    pub q_id: u16,
    pub del_q: },
// Flush Queue
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __packed {
    pub prov_id: u32,
    pub q_id: u16,
    pub flush_q: },
// Allocate Common Buffer
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __packed {
    pub len: u32,
    pub /: *mut *mut u32 pf_node; / Preferred Node,
    pub region_id: u16,
    pub alloc_com_buf: },
// Free Common Buffer
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __packed {
    pub len: u32,
    pub /: *mut *mut u64 pa; / Physical Address,
    pub /: *mut *mut u32 pf_node; / Preferred Node,
    pub region_id: u16,
    pub cache_type: u8,
    pub free_com_buf: },
    pub __packed: },
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvsp_6_pd_api_comp {
    pub op: u32,
    pub status: u32,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __packed {
// actual number of PD queues allocated to the VM
    pub num_pd_q: u16,
// Num Receive Rss PD Queues
    pub num_rss_q: u8,
    pub /: *mut *mut u8 is_supported; / Is supported by VSP,
    pub /: *mut *mut u8 is_enabled; / Is enabled by VSP,
    pub config: },
// Open Provider
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __packed {
    pub prov_id: u32,
    pub open_prov: },
// Create Queue
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __packed {
    pub prov_id: u32,
    pub q_id: u16,
    pub q_size: u16,
    pub recv_data_len: u32,
    pub affy: grp_affinity,
    pub cr_q: },
// Allocate Common Buffer
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __packed {
    pub /: *mut *mut u64 pa; / Physical Address,
    pub len: u32,
    pub /: *mut *mut u32 pf_node; / Preferred Node,
    pub region_id: u16,
    pub cache_type: u8,
    pub alloc_com_buf: },
    pub __packed: },
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvsp_6_pd_buf {
    pub region_offset: u32,
    pub region_id: u16,
    pub is_partial:1: u16,
    pub reserved:15: u16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvsp_6_pd_batch_msg {
    pub hdr: nvsp_message_header,
    pub count: u16,
    pub guest2host:1: u16,
    pub is_recv:1: u16,
    pub reserved:14: u16,
    pub pd_buf: [nvsp_6_pd_buf; 0],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub union nvsp_6_message_uber {
    pub pd_req: nvsp_6_pd_api_req,
    pub pd_comp: nvsp_6_pd_api_comp,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub union nvsp_all_messages {
    pub init_msg: nvsp_message_init_uber,
    pub v1_msg: nvsp_1_message_uber,
    pub v2_msg: nvsp_2_message_uber,
    pub v4_msg: nvsp_4_message_uber,
    pub v5_msg: nvsp_5_message_uber,
    pub v6_msg: nvsp_6_message_uber,
    pub __packed: },
// ALL Messages
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvsp_message {
    pub hdr: nvsp_message_header,
    pub msg: nvsp_all_messages,
    pub __packed: },
pub const NETVSC_MTU: c_int = 65535;

// Max buffer sizes allowed by a host

pub const NETVSC_SEND_SECTION_SIZE: c_int = 6144;
pub const NETVSC_RECV_SECTION_SIZE: c_int = 1728;
// Default size of TX buf: 1MB, RX buf: 16MB
pub const NETVSC_MIN_TX_SECTIONS: c_int = 10;

pub const NETVSC_MIN_RX_SECTIONS: c_int = 10;

pub const NETVSC_RECEIVE_BUFFER_ID: c_uint = 0xcafe;
pub const NETVSC_SEND_BUFFER_ID: c_int = 0;

pub const VRSS_CHANNEL_MAX: c_int = 64;
pub const VRSS_CHANNEL_DEFAULT: c_int = 16;
pub const RNDIS_MAX_PKT_DEFAULT: c_int = 8;
pub const RNDIS_PKT_ALIGN_DEFAULT: c_int = 8;
pub const NETVSC_XDP_HDRM: c_int = 256;

// Maximum # of contiguous data ranges that can make up a trasmitted packet.
// Typically it's the max SKB fragments plus 2 for the rndis packet and the
// linear portion of the SKB. But if MAX_SKB_FRAGS is large, the value may
// need to be limited to MAX_PAGE_BUFFER_COUNT, which is the max # of entries
// in a GPA direct packet sent to netvsp over VMBus.
//

// Estimated requestor size:
// out_ring_size/min_out_msg_size + in_ring_size/min_in_msg_size
//
    pub NETVSC_MIN_IN_MSG_SIZE: ringbytes /,
// XFER PAGE packets can specify a maximum of 375 ranges for NDIS >= 6.0
// and a maximum of 64 ranges for NDIS < 6.0 with no RSC; with RSC, this
// limit is raised to 562 (= NVSP_RSC_MAX).
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct multi_send_data {
    pub /: *mut *mut *mut sk_buff skb; / skb containing the pkt,
    pub /: *mut *mut *mut hv_netvsc_packet pkt; / netvsc pkt pending,
    pub /: *mut *mut u32 count; / counter of batched packets,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct recv_comp_data {
    pub /: *mut *mut u64 tid; / transaction id,
    pub status: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct multi_recv_comp {
    pub slots: *mut recv_comp_data,
    pub /: *mut *mut u32 first; / first data entry,
    pub /: *mut *mut u32 next; / next entry for writing,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvsc_rsc {
    pub vlan: ndis_pkt_8021q_info,
    pub csum_info: ndis_tcp_ip_checksum_info,
    pub hash_info: u32,
    pub /: *mut *mut u8 ppi_flags; / valid/present bits for the above PPIs,
    pub /: *mut *mut u8 is_last; / last RNDIS msg in a vmtransfer_page,
    pub /: *mut *mut u32 cnt; / #fragments in an RSC packet,
    pub /: *mut *mut u32 pktlen; / Full packet length,
    pub data: [*mut c_void; NVSP_RSC_MAX],
    pub len: [u32; NVSP_RSC_MAX],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct netvsc_stats_tx {
    pub packets: u64,
    pub bytes: u64,
    pub xdp_xmit: u64,
    pub syncp: u64_stats_sync,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct netvsc_stats_rx {
    pub packets: u64,
    pub bytes: u64,
    pub broadcast: u64,
    pub multicast: u64,
    pub xdp_drop: u64,
    pub xdp_redirect: u64,
    pub xdp_tx: u64,
    pub syncp: u64_stats_sync,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct netvsc_ethtool_stats {
    pub tx_scattered: c_ulong,
    pub tx_no_memory: c_ulong,
    pub tx_no_space: c_ulong,
    pub tx_too_big: c_ulong,
    pub tx_busy: c_ulong,
    pub tx_send_full: c_ulong,
    pub rx_comp_busy: c_ulong,
    pub rx_no_memory: c_ulong,
    pub stop_queue: c_ulong,
    pub wake_queue: c_ulong,
    pub vlan_error: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct netvsc_ethtool_pcpu_stats {
    pub rx_packets: u64,
    pub rx_bytes: u64,
    pub tx_packets: u64,
    pub tx_bytes: u64,
    pub vf_rx_packets: u64,
    pub vf_rx_bytes: u64,
    pub vf_tx_packets: u64,
    pub vf_tx_bytes: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct netvsc_vf_pcpu_stats {
    pub rx_packets: u64,
    pub rx_bytes: u64,
    pub tx_packets: u64,
    pub tx_bytes: u64,
    pub syncp: u64_stats_sync,
    pub tx_dropped: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct netvsc_reconfig {
    pub list: list_head,
    pub event: u32,
}

// L4 hash bits for different protocols
pub const HV_TCP4_L4HASH: c_int = 1;
pub const HV_TCP6_L4HASH: c_int = 2;
pub const HV_UDP4_L4HASH: c_int = 4;
pub const HV_UDP6_L4HASH: c_int = 8;

// The context of the netvsc device
#[repr(C)]
#[derive(Copy, Clone)]
pub struct net_device_context {
// point back to our device context
    pub device_ctx: *mut hv_device,
// netvsc_device
    pub nvdev: *mut netvsc_device __rcu,
// list of netvsc net_devices
    pub list: list_head,
// reconfigure work
    pub dwork: delayed_work,
// last reconfig time
    pub last_reconfig: c_ulong,
// reconfig events
    pub reconfig_events: list_head,
// list protection
    pub lock: spinlock_t,
    pub /: *mut *mut u32 msg_enable; / debug level,
    pub tx_checksum_mask: u32,
    pub tx_table: [u32; VRSS_SEND_TAB_SIZE],
    pub rx_table: *mut u16,
    pub rx_table_sz: u32,
// Ethtool settings
    pub duplex: u8,
    pub speed: u32,
    pub /: *mut *mut u32 l4_hash; / L4 hash settings,
    pub eth_stats: netvsc_ethtool_stats,
// State to manage the associated VF interface.
    pub vf_netdev: *mut net_device __rcu,
    pub vf_stats: *mut netvsc_vf_pcpu_stats __percpu,
    pub vf_takeover: delayed_work,
    pub vfns_work: delayed_work,
// 1: allocated, serial number is valid. 0: not allocated
    pub vf_alloc: u32,
// Serial number of the VF to team with
    pub vf_serial: u32,
// completion variable to confirm vf association
    pub vf_add: completion,
// Is the current data path through the VF NIC?
    pub data_path_is_vf: bool,
// Used to temporarily save the config info across hibernation
    pub saved_netvsc_dev_info: *mut netvsc_device_info,
}

extern "C" {
    pub fn netvsc_vfns_work(w: *mut work_struct);
}
// Azure hosts don't support non-TCP port numbers in hashing for fragmented
// packets. We can use ethtool to change UDP hash level when necessary.
//
extern "C" {
    pub fn skb_get_hash(_arg: skb) -> return;
}
// Per channel data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct netvsc_channel {
    pub channel: *mut vmbus_channel,
    pub net_device: *mut netvsc_device,
    pub /: *mut *mut *mut void recv_buf; / buffer to copy packets out from the receive buffer,
    pub desc: *const vmpacket_descriptor,
    pub napi: napi_struct,
    pub msd: multi_send_data,
    pub mrc: multi_recv_comp,
    pub queue_sends: core::sync::atomic::AtomicI32,
    pub rsc: nvsc_rsc,
    pub bpf_prog: *mut bpf_prog __rcu,
    pub xdp_rxq: xdp_rxq_info,
    pub xdp_flush: bool,
    pub tx_stats: netvsc_stats_tx,
    pub rx_stats: netvsc_stats_rx,
}

// Per netvsc device
#[repr(C)]
#[derive(Copy, Clone)]
pub struct netvsc_device {
    pub nvsp_version: u32,
    pub wait_drain: wait_queue_head_t,
    pub destroy: bool,
    pub /: *mut *mut bool tx_disable; / if true, do not wake up queue again,
// Receive buffer allocated by us but manages by NetVSP
    pub recv_buf: *mut c_void,
    pub /: *mut *mut u32 recv_buf_size; / allocated bytes,
    pub recv_buf_chunks: *mut page,
    pub recv_buf_chunk_cnt: u32,
    pub recv_buf_gpadl_handle: vmbus_gpadl,
    pub recv_section_cnt: u32,
    pub recv_section_size: u32,
    pub recv_completion_cnt: u32,
// Send buffer allocated by us
    pub send_buf: *mut c_void,
    pub send_buf_size: u32,
    pub send_buf_chunks: *mut page,
    pub send_buf_chunk_cnt: u32,
    pub send_buf_gpadl_handle: vmbus_gpadl,
    pub send_section_cnt: u32,
    pub send_section_size: u32,
    pub send_section_map: *mut c_ulong,
// Used for NetVSP initialization protocol
    pub channel_init_wait: completion,
    pub channel_init_pkt: nvsp_message,
    pub revoke_packet: nvsp_message,
    pub max_chn: u32,
    pub num_chn: u32,
    pub netvsc_gso_max_size: u32,
    pub open_chn: core::sync::atomic::AtomicI32,
    pub subchan_work: work_struct,
    pub subchan_open: wait_queue_head_t,
    pub extension: *mut rndis_device,
    pub /: *mut *mut u32 max_pkt; / max number of pkt in one send, e.g. 8,
    pub /: *mut *mut u32 pkt_align; / alignment bytes, e.g. 8,
    pub chan_table: [netvsc_channel; VRSS_CHANNEL_MAX],
    pub rwork: rcu_work,
}

// NdisInitialize message
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rndis_initialize_request {
    pub req_id: u32,
    pub major_ver: u32,
    pub minor_ver: u32,
    pub max_xfer_size: u32,
}

// Response to NdisInitialize
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rndis_initialize_complete {
    pub req_id: u32,
    pub status: u32,
    pub major_ver: u32,
    pub minor_ver: u32,
    pub dev_flags: u32,
    pub medium: u32,
    pub max_pkt_per_msg: u32,
    pub max_xfer_size: u32,
    pub pkt_alignment_factor: u32,
    pub af_list_offset: u32,
    pub af_list_size: u32,
}

// Call manager devices only: Information about an address family
// supported by the device is appended to the response to NdisInitialize.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rndis_co_address_family {
    pub address_family: u32,
    pub major_ver: u32,
    pub minor_ver: u32,
}

// NdisHalt message
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rndis_halt_request {
    pub req_id: u32,
}

// NdisQueryRequest message
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rndis_query_request {
    pub req_id: u32,
    pub oid: u32,
    pub info_buflen: u32,
    pub info_buf_offset: u32,
    pub dev_vc_handle: u32,
}

// Response to NdisQueryRequest
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rndis_query_complete {
    pub req_id: u32,
    pub status: u32,
    pub info_buflen: u32,
    pub info_buf_offset: u32,
}

// NdisSetRequest message
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rndis_set_request {
    pub req_id: u32,
    pub oid: u32,
    pub info_buflen: u32,
    pub info_buf_offset: u32,
    pub dev_vc_handle: u32,
    pub info_buf: [u8; ],
}

// Response to NdisSetRequest
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rndis_set_complete {
    pub req_id: u32,
    pub status: u32,
}

// NdisReset message
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rndis_reset_request {
    pub reserved: u32,
}

// Response to NdisReset
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rndis_reset_complete {
    pub status: u32,
    pub addressing_reset: u32,
}

// NdisMIndicateStatus message
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rndis_indicate_status {
    pub status: u32,
    pub status_buflen: u32,
    pub status_buf_offset: u32,
}

// Diagnostic information passed as the status buffer in
// struct rndis_indicate_status messages signifying error conditions.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rndis_diagnostic_info {
    pub diag_status: u32,
    pub error_offset: u32,
}

// NdisKeepAlive message
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rndis_keepalive_request {
    pub req_id: u32,
}

// Response to NdisKeepAlive
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rndis_keepalive_complete {
    pub req_id: u32,
    pub status: u32,
}

//
// Data message. All Offset fields contain byte offsets from the beginning of
// struct rndis_packet. All Length fields are in bytes.  VcHandle is set
// to 0 for connectionless data, otherwise it contains the VC handle.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rndis_packet {
    pub data_offset: u32,
    pub data_len: u32,
    pub oob_data_offset: u32,
    pub oob_data_len: u32,
    pub num_oob_data_elements: u32,
    pub per_pkt_info_offset: u32,
    pub per_pkt_info_len: u32,
    pub vc_handle: u32,
    pub reserved: u32,
}

// Optional Out of Band data associated with a Data message.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rndis_oobd {
    pub size: u32,
    pub type: u32,
    pub class_info_offset: u32,
}

// Packet extension field contents associated with a Data message.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rndis_per_packet_info {
    pub size: u32,
    pub type:31: u32,
    pub internal:1: u32,
    pub ppi_offset: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ndis_per_pkt_info_type {
    TCPIP_CHKSUM_PKTINFO,
    IPSEC_PKTINFO,
    TCP_LARGESEND_PKTINFO,
    CLASSIFICATION_HANDLE_PKTINFO,
    NDIS_RESERVED,
    SG_LIST_PKTINFO,
    IEEE_8021Q_INFO,
    ORIGINAL_PKTINFO,
    PACKET_CANCEL_ID,
    NBL_HASH_VALUE = PACKET_CANCEL_ID,
    ORIGINAL_NET_BUFLIST,
    CACHED_NET_BUFLIST,
    SHORT_PKT_PADINFO,
    MAX_PER_PKT_INFO
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rndis_per_pkt_info_interal_type {
    RNDIS_PKTINFO_ID = 1,
// Add more members here

    RNDIS_PKTINFO_MAX
}

pub const RNDIS_PKTINFO_ID_V1: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rndis_pktinfo_id {
    pub ver: u8,
    pub flag: u8,
    pub pkt_id: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ndis_object_header {
    pub type: u8,
    pub revision: u8,
    pub size: u16,
}

pub const NDIS_OBJECT_TYPE_DEFAULT: c_uint = 0x80;
pub const NDIS_OFFLOAD_PARAMETERS_REVISION_3: c_int = 3;
pub const NDIS_OFFLOAD_PARAMETERS_REVISION_2: c_int = 2;
pub const NDIS_OFFLOAD_PARAMETERS_REVISION_1: c_int = 1;
pub const NDIS_OFFLOAD_PARAMETERS_NO_CHANGE: c_int = 0;
pub const NDIS_OFFLOAD_PARAMETERS_LSOV2_DISABLED: c_int = 1;
pub const NDIS_OFFLOAD_PARAMETERS_LSOV2_ENABLED: c_int = 2;
pub const NDIS_OFFLOAD_PARAMETERS_LSOV1_ENABLED: c_int = 2;
pub const NDIS_OFFLOAD_PARAMETERS_RSC_DISABLED: c_int = 1;
pub const NDIS_OFFLOAD_PARAMETERS_RSC_ENABLED: c_int = 2;
pub const NDIS_OFFLOAD_PARAMETERS_TX_RX_DISABLED: c_int = 1;
pub const NDIS_OFFLOAD_PARAMETERS_TX_ENABLED_RX_DISABLED: c_int = 2;
pub const NDIS_OFFLOAD_PARAMETERS_RX_ENABLED_TX_DISABLED: c_int = 3;
pub const NDIS_OFFLOAD_PARAMETERS_TX_RX_ENABLED: c_int = 4;
pub const NDIS_TCP_LARGE_SEND_OFFLOAD_V2_TYPE: c_int = 1;
pub const NDIS_TCP_LARGE_SEND_OFFLOAD_IPV4: c_int = 0;
pub const NDIS_TCP_LARGE_SEND_OFFLOAD_IPV6: c_int = 1;
pub const VERSION_4_OFFLOAD_SIZE: c_int = 22;
//
// New offload OIDs for NDIS 6
//
pub const OID_TCP_OFFLOAD_CURRENT_CONFIG: c_uint = 0xFC01020B /* query only */;
pub const OID_TCP_OFFLOAD_PARAMETERS: c_uint = 0xFC01020C		/* set only */;
pub const OID_TCP_OFFLOAD_HARDWARE_CAPABILITIES: c_uint = 0xFC01020D/* query only */;
pub const OID_TCP_CONNECTION_OFFLOAD_CURRENT_CONFIG: c_uint = 0xFC01020E /* query only */;
pub const OID_TCP_CONNECTION_OFFLOAD_HARDWARE_CAPABILITIES: c_uint = 0xFC01020F /* query */;
pub const OID_OFFLOAD_ENCAPSULATION: c_uint = 0x0101010A /* set/query */;
//
// OID_TCP_OFFLOAD_HARDWARE_CAPABILITIES
// ndis_type: NDIS_OBJTYPE_OFFLOAD
//
pub const NDIS_OFFLOAD_ENCAP_NONE: c_uint = 0x0000;
pub const NDIS_OFFLOAD_ENCAP_NULL: c_uint = 0x0001;
pub const NDIS_OFFLOAD_ENCAP_8023: c_uint = 0x0002;
pub const NDIS_OFFLOAD_ENCAP_8023PQ: c_uint = 0x0004;
pub const NDIS_OFFLOAD_ENCAP_8023PQ_OOB: c_uint = 0x0008;
pub const NDIS_OFFLOAD_ENCAP_RFC1483: c_uint = 0x0010;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ndis_csum_offload {
    pub ip4_txenc: u32,
    pub ip4_txcsum: u32,
pub const NDIS_TXCSUM_CAP_IP4OPT: c_uint = 0x001;
pub const NDIS_TXCSUM_CAP_TCP4OPT: c_uint = 0x004;
pub const NDIS_TXCSUM_CAP_TCP4: c_uint = 0x010;
pub const NDIS_TXCSUM_CAP_UDP4: c_uint = 0x040;
pub const NDIS_TXCSUM_CAP_IP4: c_uint = 0x100;

    pub ip4_rxenc: u32,
    pub ip4_rxcsum: u32,
pub const NDIS_RXCSUM_CAP_IP4OPT: c_uint = 0x001;
pub const NDIS_RXCSUM_CAP_TCP4OPT: c_uint = 0x004;
pub const NDIS_RXCSUM_CAP_TCP4: c_uint = 0x010;
pub const NDIS_RXCSUM_CAP_UDP4: c_uint = 0x040;
pub const NDIS_RXCSUM_CAP_IP4: c_uint = 0x100;
    pub ip6_txenc: u32,
    pub ip6_txcsum: u32,
pub const NDIS_TXCSUM_CAP_IP6EXT: c_uint = 0x001;
pub const NDIS_TXCSUM_CAP_TCP6OPT: c_uint = 0x004;
pub const NDIS_TXCSUM_CAP_TCP6: c_uint = 0x010;
pub const NDIS_TXCSUM_CAP_UDP6: c_uint = 0x040;
    pub ip6_rxenc: u32,
    pub ip6_rxcsum: u32,
pub const NDIS_RXCSUM_CAP_IP6EXT: c_uint = 0x001;
pub const NDIS_RXCSUM_CAP_TCP6OPT: c_uint = 0x004;
pub const NDIS_RXCSUM_CAP_TCP6: c_uint = 0x010;
pub const NDIS_RXCSUM_CAP_UDP6: c_uint = 0x040;

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ndis_lsov1_offload {
    pub encap: u32,
    pub maxsize: u32,
    pub minsegs: u32,
    pub opts: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ndis_ipsecv1_offload {
    pub encap: u32,
    pub ah_esp: u32,
    pub xport_tun: u32,
    pub ip4_opts: u32,
    pub flags: u32,
    pub ip4_ah: u32,
    pub ip4_esp: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ndis_lsov2_offload {
    pub ip4_encap: u32,
    pub ip4_maxsz: u32,
    pub ip4_minsg: u32,
    pub ip6_encap: u32,
    pub ip6_maxsz: u32,
    pub ip6_minsg: u32,
    pub ip6_opts: u32,
pub const NDIS_LSOV2_CAP_IP6EXT: c_uint = 0x001;
pub const NDIS_LSOV2_CAP_TCP6OPT: c_uint = 0x004;

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ndis_ipsecv2_offload {
    pub encap: u32,
    pub ip6: u8,
    pub ip4opt: u8,
    pub ip6ext: u8,
    pub ah: u8,
    pub esp: u8,
    pub ah_esp: u8,
    pub xport: u8,
    pub tun: u8,
    pub xport_tun: u8,
    pub lso: u8,
    pub extseq: u8,
    pub udp_esp: u32,
    pub auth: u32,
    pub crypto: u32,
    pub sa_caps: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ndis_rsc_offload {
    pub ip4: u8,
    pub ip6: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ndis_encap_offload {
    pub flags: u32,
    pub maxhdr: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ndis_offload {
    pub header: ndis_object_header,
    pub csum: ndis_csum_offload,
    pub lsov1: ndis_lsov1_offload,
    pub ipsecv1: ndis_ipsecv1_offload,
    pub lsov2: ndis_lsov2_offload,
    pub flags: u32,
// NDIS >= 6.1
    pub ipsecv2: ndis_ipsecv2_offload,
// NDIS >= 6.30
    pub rsc: ndis_rsc_offload,
    pub encap_gre: ndis_encap_offload,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ndis_offload_params {
    pub header: ndis_object_header,
    pub ip_v4_csum: u8,
    pub tcp_ip_v4_csum: u8,
    pub udp_ip_v4_csum: u8,
    pub tcp_ip_v6_csum: u8,
    pub udp_ip_v6_csum: u8,
    pub lso_v1: u8,
    pub ip_sec_v1: u8,
    pub lso_v2_ipv4: u8,
    pub lso_v2_ipv6: u8,
    pub tcp_connection_ip_v4: u8,
    pub tcp_connection_ip_v6: u8,
    pub flags: u32,
    pub ip_sec_v2: u8,
    pub ip_sec_v2_ip_v4: u8,
    pub rsc_ip_v4: u8,
    pub rsc_ip_v6: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ndis_tcp_lso_info {
    pub unused:30: u32,
    pub type:1: u32,
    pub reserved2:1: u32,
    pub transmit: },
    pub mss:20: u32,
    pub tcp_header_offset:10: u32,
    pub type:1: u32,
    pub reserved2:1: u32,
    pub lso_v1_transmit: },
    pub tcp_payload:30: u32,
    pub type:1: u32,
    pub reserved2:1: u32,
    pub lso_v1_transmit_complete: },
    pub mss:20: u32,
    pub tcp_header_offset:10: u32,
    pub type:1: u32,
    pub ip_version:1: u32,
    pub lso_v2_transmit: },
    pub reserved:30: u32,
    pub type:1: u32,
    pub reserved2:1: u32,
    pub lso_v2_transmit_complete: },
    pub value: u32,
}

// Total size of all PPI data

// Format of Information buffer passed in a SetRequest for the OID
// OID_GEN_RNDIS_CONFIG_PARAMETER.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rndis_config_parameter_info {
    pub parameter_name_offset: u32,
    pub parameter_name_length: u32,
    pub parameter_type: u32,
    pub parameter_value_offset: u32,
    pub parameter_value_length: u32,
}

// Values for ParameterType in struct rndis_config_parameter_info
pub const RNDIS_CONFIG_PARAM_TYPE_INTEGER: c_int = 0;
pub const RNDIS_CONFIG_PARAM_TYPE_STRING: c_int = 2;
// CONDIS Miniport messages for connection oriented devices
// that do not implement a call manager.
// CoNdisMiniportCreateVc message
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rcondis_mp_create_vc {
    pub req_id: u32,
    pub ndis_vc_handle: u32,
}

// Response to CoNdisMiniportCreateVc
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rcondis_mp_create_vc_complete {
    pub req_id: u32,
    pub dev_vc_handle: u32,
    pub status: u32,
}

// CoNdisMiniportDeleteVc message
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rcondis_mp_delete_vc {
    pub req_id: u32,
    pub dev_vc_handle: u32,
}

// Response to CoNdisMiniportDeleteVc
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rcondis_mp_delete_vc_complete {
    pub req_id: u32,
    pub status: u32,
}

// CoNdisMiniportQueryRequest message
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rcondis_mp_query_request {
    pub req_id: u32,
    pub request_type: u32,
    pub oid: u32,
    pub dev_vc_handle: u32,
    pub info_buflen: u32,
    pub info_buf_offset: u32,
}

// CoNdisMiniportSetRequest message
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rcondis_mp_set_request {
    pub req_id: u32,
    pub request_type: u32,
    pub oid: u32,
    pub dev_vc_handle: u32,
    pub info_buflen: u32,
    pub info_buf_offset: u32,
}

// CoNdisIndicateStatus message
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rcondis_indicate_status {
    pub ndis_vc_handle: u32,
    pub status: u32,
    pub status_buflen: u32,
    pub status_buf_offset: u32,
}

// CONDIS Call/VC parameters
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rcondis_specific_parameters {
    pub parameter_type: u32,
    pub parameter_length: u32,
    pub parameter_lffset: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rcondis_media_parameters {
    pub flags: u32,
    pub reserved1: u32,
    pub reserved2: u32,
    pub media_specific: rcondis_specific_parameters,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rndis_flowspec {
    pub token_rate: u32,
    pub token_bucket_size: u32,
    pub peak_bandwidth: u32,
    pub latency: u32,
    pub delay_variation: u32,
    pub service_type: u32,
    pub max_sdu_size: u32,
    pub minimum_policed_size: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rcondis_call_manager_parameters {
    pub transmit: rndis_flowspec,
    pub receive: rndis_flowspec,
    pub call_mgr_specific: rcondis_specific_parameters,
}

// CoNdisMiniportActivateVc message
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rcondis_mp_activate_vc_request {
    pub req_id: u32,
    pub flags: u32,
    pub dev_vc_handle: u32,
    pub media_params_offset: u32,
    pub media_params_length: u32,
    pub call_mgr_params_offset: u32,
    pub call_mgr_params_length: u32,
}

// Response to CoNdisMiniportActivateVc
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rcondis_mp_activate_vc_complete {
    pub req_id: u32,
    pub status: u32,
}

// CoNdisMiniportDeactivateVc message
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rcondis_mp_deactivate_vc_request {
    pub req_id: u32,
    pub flags: u32,
    pub dev_vc_handle: u32,
}

// Response to CoNdisMiniportDeactivateVc
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rcondis_mp_deactivate_vc_complete {
    pub req_id: u32,
    pub status: u32,
}

// union with all of the RNDIS messages
#[repr(C)]
#[derive(Copy, Clone)]
pub union rndis_message_container {
    pub pkt: rndis_packet,
    pub init_req: rndis_initialize_request,
    pub halt_req: rndis_halt_request,
    pub query_req: rndis_query_request,
    pub set_req: rndis_set_request,
    pub reset_req: rndis_reset_request,
    pub keep_alive_req: rndis_keepalive_request,
    pub indicate_status: rndis_indicate_status,
    pub init_complete: rndis_initialize_complete,
    pub query_complete: rndis_query_complete,
    pub set_complete: rndis_set_complete,
    pub reset_complete: rndis_reset_complete,
    pub keep_alive_complete: rndis_keepalive_complete,
    pub co_miniport_create_vc: rcondis_mp_create_vc,
    pub co_miniport_delete_vc: rcondis_mp_delete_vc,
    pub co_indicate_status: rcondis_indicate_status,
    pub co_miniport_activate_vc: rcondis_mp_activate_vc_request,
    pub co_miniport_deactivate_vc: rcondis_mp_deactivate_vc_request,
    pub co_miniport_create_vc_complete: rcondis_mp_create_vc_complete,
    pub co_miniport_delete_vc_complete: rcondis_mp_delete_vc_complete,
    pub co_miniport_activate_vc_complete: rcondis_mp_activate_vc_complete,
}

// Remote NDIS message format
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rndis_message {
    pub ndis_msg_type: u32,
// Total length of this message, from the beginning
// of the struct rndis_message, in bytes.
    pub msg_len: u32,
// Actual message
    pub msg: rndis_message_container,
}

// Handy macros
// get the size of an RNDIS message. Pass in the message type,
// struct rndis_set_request, struct rndis_packet for example

pub const NDIS_PACKET_TYPE_DIRECTED: c_uint = 0x00000001;
pub const NDIS_PACKET_TYPE_MULTICAST: c_uint = 0x00000002;
pub const NDIS_PACKET_TYPE_ALL_MULTICAST: c_uint = 0x00000004;
pub const NDIS_PACKET_TYPE_BROADCAST: c_uint = 0x00000008;
pub const NDIS_PACKET_TYPE_SOURCE_ROUTING: c_uint = 0x00000010;
pub const NDIS_PACKET_TYPE_PROMISCUOUS: c_uint = 0x00000020;
pub const NDIS_PACKET_TYPE_SMT: c_uint = 0x00000040;
pub const NDIS_PACKET_TYPE_ALL_LOCAL: c_uint = 0x00000080;
pub const NDIS_PACKET_TYPE_GROUP: c_uint = 0x00000100;
pub const NDIS_PACKET_TYPE_ALL_FUNCTIONAL: c_uint = 0x00000200;
pub const NDIS_PACKET_TYPE_FUNCTIONAL: c_uint = 0x00000400;
pub const NDIS_PACKET_TYPE_MAC_FRAME: c_uint = 0x00000800;
pub const TRANSPORT_INFO_NOT_IP: c_int = 0;
pub const TRANSPORT_INFO_IPV4_TCP: c_uint = 0x01;
pub const TRANSPORT_INFO_IPV4_UDP: c_uint = 0x02;
pub const TRANSPORT_INFO_IPV6_TCP: c_uint = 0x10;
pub const TRANSPORT_INFO_IPV6_UDP: c_uint = 0x20;
pub const RETRY_US_LO: c_int = 5000;
pub const RETRY_US_HI: c_int = 10000;

