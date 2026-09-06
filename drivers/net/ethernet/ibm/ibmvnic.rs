//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/ibm/ibmvnic.h
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
// IBM System i and System p Virtual NIC Device Driver
// Copyright (C) 2014 IBM Corp.
// Santiago Leon (santi_leon@yahoo.com)
// Thomas Falcon (tlfalcon@linux.vnet.ibm.com)
// John Allen (jallen@linux.vnet.ibm.com)
//
// This module contains the implementation of a virtual ethernet device
// for use with IBM i/pSeries LPAR Linux.  It utilizes the logical LAN
// option of the RS/6000 Platform Architecture to interface with virtual
// ethernet NICs that are presented to the partition by the hypervisor.
//

pub const IBMVNIC_OPEN_FAILED: c_int = 3;
// basic structures plus 100 2k buffers
pub const IBMVNIC_IO_ENTITLEMENT_DEFAULT: c_int = 610305;
// Initial module_parameters
pub const IBMVNIC_RX_WEIGHT: c_int = 16;
// when changing this, update IBMVNIC_IO_ENTITLEMENT_DEFAULT
pub const IBMVNIC_BUFFS_PER_POOL: c_int = 100;
pub const IBMVNIC_MAX_QUEUES: c_int = 16;
pub const IBMVNIC_MAX_QUEUE_SZ: c_int = 4096;
pub const IBMVNIC_MAX_IND_DESCS: c_int = 128;
pub const IBMVNIC_SAFE_IND_DESC: c_int = 16;

pub const IBMVNIC_TSO_BUF_SZ: c_int = 65536;
pub const IBMVNIC_TSO_BUFS: c_int = 64;
pub const IBMVNIC_TSO_POOL_MASK: c_uint = 0x80000000;
// A VNIC adapter has set of Rx and Tx pools (aka queues). Each Rx/Tx pool
// has a set of buffers. The size of each buffer is determined by the MTU.
//
// Each Rx/Tx pool is also associated with a DMA region that is shared
// with the "hardware" (VIOS) and used to send/receive packets. The DMA
// region is also referred to as a Long Term Buffer or LTB.
//
// The size of the DMA region required for an Rx/Tx pool depends on the
// number and size (MTU) of the buffers in the pool. At the max levels
// of 4096 jumbo frames (MTU=9000) we will need about 9K*4K = 36MB plus
// some padding.
//
// But the size of a single DMA region is limited by MAX_PAGE_ORDER in the
// kernel (about 16MB currently).  To support say 4K Jumbo frames, we
// use a set of LTBs (struct ltb_set) per pool.
//
// IBMVNIC_ONE_LTB_MAX  - max size of each LTB supported by kernel
// IBMVNIC_ONE_LTB_SIZE - current max size of each LTB in an ltb_set
// (must be <= IBMVNIC_ONE_LTB_MAX)
// IBMVNIC_LTB_SET_SIZE - current size of all LTBs in an ltb_set
//
// Each VNIC can have upto 16 Rx, 16 Tx and 16 TSO pools. The TSO pools
// are of fixed length (IBMVNIC_TSO_BUF_SZ * IBMVNIC_TSO_BUFS) of 4MB.
//
// The Rx and Tx pools can have upto 4096 buffers. The max size of these
// buffers is about 9588 (for jumbo frames, including IBMVNIC_BUFFER_HLEN).
// So, setting the IBMVNIC_LTB_SET_SIZE for a pool to 4096 * 9588 ~= 38MB.
//
// There is a trade-off in setting IBMVNIC_ONE_LTB_SIZE. If it is large,
// the allocation of the LTB can fail when system is low in memory. If
// its too small, we would need several mappings for each of the Rx
// Tx/TSO pools but there is a limit of 255 mappings per vnic in the
// VNIC protocol.
//
// So setting IBMVNIC_ONE_LTB_SIZE to 8MB. With IBMVNIC_LTB_SET_SIZE set
// to 38MB, we will need 5 LTBs per Rx and Tx pool and 1 LTB per TSO
// pool for the 4MB. Thus the 16 Rx and Tx queues require 32 * 5 = 160
// plus 16 for the TSO pools for a total of 176 LTB mappings per VNIC.
//

pub const IBMVNIC_BUFFER_HLEN: c_int = 500;
pub const IBMVNIC_RESET_DELAY: c_int = 100;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ibmvnic_login_buffer {
    pub len: __be32,
    pub version: __be32,
pub const INITIAL_VERSION_LB: c_int = 1;
    pub num_txcomp_subcrqs: __be32,
    pub off_txcomp_subcrqs: __be32,
    pub num_rxcomp_subcrqs: __be32,
    pub off_rxcomp_subcrqs: __be32,
    pub login_rsp_ioba: __be32,
    pub login_rsp_len: __be32,
    pub client_data_offset: __be32,
    pub client_data_len: __be32,
    pub __aligned(8): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ibmvnic_login_rsp_buffer {
    pub len: __be32,
    pub version: __be32,
pub const INITIAL_VERSION_LRB: c_int = 1;
    pub num_txsubm_subcrqs: __be32,
    pub off_txsubm_subcrqs: __be32,
    pub num_rxadd_subcrqs: __be32,
    pub off_rxadd_subcrqs: __be32,
    pub off_rxadd_buff_size: __be32,
    pub num_supp_tx_desc: __be32,
    pub off_supp_tx_desc: __be32,
    pub __aligned(8): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ibmvnic_query_ip_offload_buffer {
    pub len: __be32,
    pub version: __be32,
pub const INITIAL_VERSION_IOB: c_int = 1;
    pub ipv4_chksum: u8,
    pub ipv6_chksum: u8,
    pub tcp_ipv4_chksum: u8,
    pub tcp_ipv6_chksum: u8,
    pub udp_ipv4_chksum: u8,
    pub udp_ipv6_chksum: u8,
    pub large_tx_ipv4: u8,
    pub large_tx_ipv6: u8,
    pub large_rx_ipv4: u8,
    pub large_rx_ipv6: u8,
    pub reserved1: [u8; 14],
    pub max_ipv4_header_size: __be16,
    pub max_ipv6_header_size: __be16,
    pub max_tcp_header_size: __be16,
    pub max_udp_header_size: __be16,
    pub max_large_tx_size: __be32,
    pub max_large_rx_size: __be32,
    pub reserved2: [u8; 16],
    pub ipv6_extension_header: u8,
pub const IPV6_EH_NOT_SUPPORTED: c_uint = 0x00;
pub const IPV6_EH_SUPPORTED_LIM: c_uint = 0x01;
pub const IPV6_EH_SUPPORTED: c_uint = 0xFF;
    pub tcp_pseudosum_req: u8,
pub const TCP_PS_NOT_REQUIRED: c_uint = 0x00;
pub const TCP_PS_REQUIRED: c_uint = 0x01;
    pub reserved3: [u8; 30],
    pub num_ipv6_ext_headers: __be16,
    pub off_ipv6_ext_headers: __be32,
    pub reserved4: [u8; 154],
    pub __aligned(8): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ibmvnic_control_ip_offload_buffer {
    pub len: __be32,
    pub version: __be32,
pub const INITIAL_VERSION_IOB: c_int = 1;
    pub ipv4_chksum: u8,
    pub ipv6_chksum: u8,
    pub tcp_ipv4_chksum: u8,
    pub tcp_ipv6_chksum: u8,
    pub udp_ipv4_chksum: u8,
    pub udp_ipv6_chksum: u8,
    pub large_tx_ipv4: u8,
    pub large_tx_ipv6: u8,
    pub bad_packet_rx: u8,
    pub large_rx_ipv4: u8,
    pub large_rx_ipv6: u8,
    pub reserved4: [u8; 111],
    pub __aligned(8): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ibmvnic_fw_component {
    pub name: [u8; 48],
    pub trace_buff_size: __be32,
    pub correlator: u8,
    pub trace_level: u8,
    pub parent_correlator: u8,
    pub error_check_level: u8,
    pub trace_on: u8,
    pub reserved: [u8; 7],
    pub description: [u8; 192],
    pub __aligned(8): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ibmvnic_fw_trace_entry {
    pub trace_id: __be32,
    pub num_valid_data: u8,
    pub reserved: [u8; 3],
    pub pmc_registers: __be64,
    pub timebase: __be64,
    pub trace_data: [__be64; 5],
    pub __aligned(8): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ibmvnic_statistics {
    pub version: __be32,
    pub promiscuous: __be32,
    pub rx_packets: __be64,
    pub rx_bytes: __be64,
    pub tx_packets: __be64,
    pub tx_bytes: __be64,
    pub ucast_tx_packets: __be64,
    pub ucast_rx_packets: __be64,
    pub mcast_tx_packets: __be64,
    pub mcast_rx_packets: __be64,
    pub bcast_tx_packets: __be64,
    pub bcast_rx_packets: __be64,
    pub align_errors: __be64,
    pub fcs_errors: __be64,
    pub single_collision_frames: __be64,
    pub multi_collision_frames: __be64,
    pub sqe_test_errors: __be64,
    pub deferred_tx: __be64,
    pub late_collisions: __be64,
    pub excess_collisions: __be64,
    pub internal_mac_tx_errors: __be64,
    pub carrier_sense: __be64,
    pub too_long_frames: __be64,
    pub internal_mac_rx_errors: __be64,
    pub reserved: [u8; 72],
    pub __aligned(8): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ibmvnic_tx_queue_stats {
    pub batched_packets: u64,
    pub direct_packets: u64,
    pub bytes: u64,
    pub dropped_packets: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ibmvnic_rx_queue_stats {
    pub packets: u64,
    pub bytes: u64,
    pub interrupts: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ibmvnic_acl_buffer {
    pub len: __be32,
    pub version: __be32,
pub const INITIAL_VERSION_IOB: c_int = 1;
    pub mac_acls_restrict: u8,
    pub vlan_acls_restrict: u8,
    pub reserved1: [u8; 22],
    pub num_mac_addrs: __be32,
    pub offset_mac_addrs: __be32,
    pub num_vlan_ids: __be32,
    pub offset_vlan_ids: __be32,
    pub reserved2: [u8; 80],
    pub __aligned(8): } __packed,
// descriptors have been changed, how should this be defined?  1? 4?
pub const IBMVNIC_TX_DESC_VERSIONS: c_int = 3;
// is this still needed?
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ibmvnic_tx_comp_desc {
    pub first: u8,
    pub num_comps: u8,
    pub rcs: [__be16; 5],
    pub correlators: [__be32; 5],
    pub __aligned(8): } __packed,
// some flags that included in v0 descriptor, which is gone
// only used for IBMVNIC_TCP_CHKSUM and IBMVNIC_UDP_CHKSUM
// and only in some offload_flags variable that doesn't seem
// to be used anywhere, can probably be removed?
//
pub const IBMVNIC_TCP_CHKSUM: c_uint = 0x20;
pub const IBMVNIC_UDP_CHKSUM: c_uint = 0x08;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ibmvnic_tx_desc {
    pub first: u8,
    pub type: u8,
pub const IBMVNIC_TX_DESC: c_uint = 0x10;
    pub n_crq_elem: u8,
    pub n_sge: u8,
    pub flags1: u8,
pub const IBMVNIC_TX_COMP_NEEDED: c_uint = 0x80;
pub const IBMVNIC_TX_CHKSUM_OFFLOAD: c_uint = 0x40;
pub const IBMVNIC_TX_LSO: c_uint = 0x20;
pub const IBMVNIC_TX_PROT_TCP: c_uint = 0x10;
pub const IBMVNIC_TX_PROT_UDP: c_uint = 0x08;
pub const IBMVNIC_TX_PROT_IPV4: c_uint = 0x04;
pub const IBMVNIC_TX_PROT_IPV6: c_uint = 0x02;
pub const IBMVNIC_TX_VLAN_PRESENT: c_uint = 0x01;
    pub flags2: u8,
pub const IBMVNIC_TX_VLAN_INSERT: c_uint = 0x80;
    pub mss: __be16,
    pub reserved: [u8; 4],
    pub correlator: __be32,
    pub vlan_id: __be16,
    pub dma_reg: __be16,
    pub sge_len: __be32,
    pub ioba: __be64,
    pub __aligned(8): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ibmvnic_hdr_desc {
    pub first: u8,
    pub type: u8,
pub const IBMVNIC_HDR_DESC: c_uint = 0x11;
    pub len: u8,
    pub l2_len: u8,
    pub l3_len: __be16,
    pub l4_len: u8,
    pub flag: u8,
    pub data: [u8; 24],
    pub __aligned(8): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ibmvnic_hdr_ext_desc {
    pub first: u8,
    pub type: u8,
pub const IBMVNIC_HDR_EXT_DESC: c_uint = 0x12;
    pub len: u8,
    pub data: [u8; 29],
    pub __aligned(8): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ibmvnic_sge_desc {
    pub first: u8,
    pub type: u8,
pub const IBMVNIC_SGE_DESC: c_uint = 0x30;
    pub sge1_dma_reg: __be16,
    pub sge1_len: __be32,
    pub sge1_ioba: __be64,
    pub reserved: __be16,
    pub sge2_dma_reg: __be16,
    pub sge2_len: __be32,
    pub sge2_ioba: __be64,
    pub __aligned(8): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ibmvnic_rx_comp_desc {
    pub first: u8,
    pub flags: u8,
pub const IBMVNIC_IP_CHKSUM_GOOD: c_uint = 0x80;
pub const IBMVNIC_TCP_UDP_CHKSUM_GOOD: c_uint = 0x40;
pub const IBMVNIC_END_FRAME: c_uint = 0x20;
pub const IBMVNIC_EXACT_MC: c_uint = 0x10;
pub const IBMVNIC_VLAN_STRIPPED: c_uint = 0x08;
    pub off_frame_data: __be16,
    pub len: __be32,
    pub correlator: __be64,
    pub vlan_tci: __be16,
    pub rc: __be16,
    pub reserved: [u8; 12],
    pub __aligned(8): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ibmvnic_generic_scrq {
    pub first: u8,
    pub reserved: [u8; 31],
    pub __aligned(8): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ibmvnic_rx_buff_add_desc {
    pub first: u8,
    pub reserved: [u8; 7],
    pub correlator: __be64,
    pub ioba: __be32,
    pub map_id: u8,
    pub len:24: __be32,
    pub reserved2: [u8; 8],
    pub __aligned(8): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ibmvnic_rc {
    pub /: *mut *mut u8 code; / one of enum ibmvnic_rc_codes,
    pub detailed_data: [u8; 3],
    pub __aligned(4): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ibmvnic_generic_crq {
    pub first: u8,
    pub cmd: u8,
    pub params: [u8; 10],
    pub rc: ibmvnic_rc,
    pub __aligned(8): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ibmvnic_version_exchange {
    pub first: u8,
    pub cmd: u8,
    pub version: __be16,
pub const IBMVNIC_INITIAL_VERSION: c_int = 1;
    pub reserved: [u8; 8],
    pub rc: ibmvnic_rc,
    pub __aligned(8): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ibmvnic_capability {
    pub first: u8,
    pub cmd: u8,
    pub /: *mut *mut __be16 capability; / one of ibmvnic_capabilities,
    pub number: __be64,
    pub rc: ibmvnic_rc,
    pub __aligned(8): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ibmvnic_login {
    pub first: u8,
    pub cmd: u8,
    pub reserved: [u8; 6],
    pub ioba: __be32,
    pub len: __be32,
    pub __aligned(8): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ibmvnic_phys_parms {
    pub first: u8,
    pub cmd: u8,
    pub flags1: u8,
pub const IBMVNIC_EXTERNAL_LOOPBACK: c_uint = 0x80;
pub const IBMVNIC_INTERNAL_LOOPBACK: c_uint = 0x40;
pub const IBMVNIC_PROMISC: c_uint = 0x20;
pub const IBMVNIC_PHYS_LINK_ACTIVE: c_uint = 0x10;
pub const IBMVNIC_AUTONEG_DUPLEX: c_uint = 0x08;
pub const IBMVNIC_FULL_DUPLEX: c_uint = 0x04;
pub const IBMVNIC_HALF_DUPLEX: c_uint = 0x02;
pub const IBMVNIC_CAN_CHG_PHYS_PARMS: c_uint = 0x01;
    pub flags2: u8,
pub const IBMVNIC_LOGICAL_LNK_ACTIVE: c_uint = 0x80;
    pub speed: __be32,
pub const IBMVNIC_AUTONEG: c_uint = 0x80000000;
pub const IBMVNIC_10MBPS: c_uint = 0x40000000;
pub const IBMVNIC_100MBPS: c_uint = 0x20000000;
pub const IBMVNIC_1GBPS: c_uint = 0x10000000;
pub const IBMVNIC_10GBPS: c_uint = 0x08000000;
pub const IBMVNIC_40GBPS: c_uint = 0x04000000;
pub const IBMVNIC_100GBPS: c_uint = 0x02000000;
pub const IBMVNIC_25GBPS: c_uint = 0x01000000;
pub const IBMVNIC_50GBPS: c_uint = 0x00800000;
pub const IBMVNIC_200GBPS: c_uint = 0x00400000;
    pub mtu: __be32,
    pub rc: ibmvnic_rc,
    pub __aligned(8): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ibmvnic_logical_link_state {
    pub first: u8,
    pub cmd: u8,
    pub link_state: u8,
pub const IBMVNIC_LOGICAL_LNK_DN: c_uint = 0x00;
pub const IBMVNIC_LOGICAL_LNK_UP: c_uint = 0x01;
pub const IBMVNIC_LOGICAL_LNK_QUERY: c_uint = 0xff;
    pub reserved: [u8; 9],
    pub rc: ibmvnic_rc,
    pub __aligned(8): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ibmvnic_query_ip_offload {
    pub first: u8,
    pub cmd: u8,
    pub reserved: [u8; 2],
    pub len: __be32,
    pub ioba: __be32,
    pub rc: ibmvnic_rc,
    pub __aligned(8): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ibmvnic_control_ip_offload {
    pub first: u8,
    pub cmd: u8,
    pub reserved: [u8; 2],
    pub ioba: __be32,
    pub len: __be32,
    pub rc: ibmvnic_rc,
    pub __aligned(8): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ibmvnic_request_statistics {
    pub first: u8,
    pub cmd: u8,
    pub flags: u8,
pub const IBMVNIC_PHYSICAL_PORT: c_uint = 0x80;
    pub reserved1: u8,
    pub ioba: __be32,
    pub len: __be32,
    pub reserved: [u8; 4],
    pub __aligned(8): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ibmvnic_error_indication {
    pub first: u8,
    pub cmd: u8,
    pub flags: u8,
pub const IBMVNIC_FATAL_ERROR: c_uint = 0x80;
    pub reserved1: u8,
    pub error_id: __be32,
    pub detail_error_sz: __be32,
    pub error_cause: __be16,
    pub reserved2: [u8; 2],
    pub __aligned(8): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ibmvnic_link_state_indication {
    pub first: u8,
    pub cmd: u8,
    pub reserved1: [u8; 2],
    pub phys_link_state: u8,
    pub logical_link_state: u8,
    pub reserved2: [u8; 10],
    pub __aligned(8): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ibmvnic_change_mac_addr {
    pub first: u8,
    pub cmd: u8,
    pub mac_addr: [u8; 6],
    pub reserved: [u8; 4],
    pub rc: ibmvnic_rc,
    pub __aligned(8): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ibmvnic_multicast_ctrl {
    pub first: u8,
    pub cmd: u8,
    pub mac_addr: [u8; 6],
    pub flags: u8,
pub const IBMVNIC_ENABLE_MC: c_uint = 0x80;
pub const IBMVNIC_DISABLE_MC: c_uint = 0x40;
pub const IBMVNIC_ENABLE_ALL: c_uint = 0x20;
pub const IBMVNIC_DISABLE_ALL: c_uint = 0x10;
    pub reserved1: u8,
    pub /: *mut *mut __be16 reserved2; / was num_enabled_mc_addr;,
    pub rc: ibmvnic_rc,
    pub __aligned(8): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ibmvnic_get_vpd_size {
    pub first: u8,
    pub cmd: u8,
    pub reserved: [u8; 14],
    pub __aligned(8): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ibmvnic_get_vpd_size_rsp {
    pub first: u8,
    pub cmd: u8,
    pub reserved: [u8; 2],
    pub len: __be64,
    pub rc: ibmvnic_rc,
    pub __aligned(8): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ibmvnic_get_vpd {
    pub first: u8,
    pub cmd: u8,
    pub reserved1: [u8; 2],
    pub ioba: __be32,
    pub len: __be32,
    pub reserved: [u8; 4],
    pub __aligned(8): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ibmvnic_get_vpd_rsp {
    pub first: u8,
    pub cmd: u8,
    pub reserved: [u8; 10],
    pub rc: ibmvnic_rc,
    pub __aligned(8): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ibmvnic_acl_change_indication {
    pub first: u8,
    pub cmd: u8,
    pub change_type: __be16,
pub const IBMVNIC_MAC_ACL: c_int = 0;
pub const IBMVNIC_VLAN_ACL: c_int = 1;
    pub reserved: [u8; 12],
    pub __aligned(8): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ibmvnic_acl_query {
    pub first: u8,
    pub cmd: u8,
    pub reserved1: [u8; 2],
    pub ioba: __be32,
    pub len: __be32,
    pub reserved2: [u8; 4],
    pub __aligned(8): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ibmvnic_tune {
    pub first: u8,
    pub cmd: u8,
    pub reserved1: [u8; 2],
    pub ioba: __be32,
    pub len: __be32,
    pub reserved2: [u8; 4],
    pub __aligned(8): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ibmvnic_request_map {
    pub first: u8,
    pub cmd: u8,
    pub reserved1: u8,
    pub map_id: u8,
    pub ioba: __be32,
    pub len: __be32,
    pub reserved2: [u8; 4],
    pub __aligned(8): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ibmvnic_request_map_rsp {
    pub first: u8,
    pub cmd: u8,
    pub reserved1: u8,
    pub map_id: u8,
    pub reserved2: [u8; 8],
    pub rc: ibmvnic_rc,
    pub __aligned(8): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ibmvnic_request_unmap {
    pub first: u8,
    pub cmd: u8,
    pub reserved1: u8,
    pub map_id: u8,
    pub reserved2: [u8; 12],
    pub __aligned(8): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ibmvnic_request_unmap_rsp {
    pub first: u8,
    pub cmd: u8,
    pub reserved1: u8,
    pub map_id: u8,
    pub reserved2: [u8; 8],
    pub rc: ibmvnic_rc,
    pub __aligned(8): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ibmvnic_query_map {
    pub first: u8,
    pub cmd: u8,
    pub reserved: [u8; 14],
    pub __aligned(8): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ibmvnic_query_map_rsp {
    pub first: u8,
    pub cmd: u8,
    pub reserved: u8,
    pub page_size: u8,
    pub tot_pages: __be32,
    pub free_pages: __be32,
    pub rc: ibmvnic_rc,
    pub __aligned(8): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub union ibmvnic_crq {
    pub generic: ibmvnic_generic_crq,
    pub version_exchange: ibmvnic_version_exchange,
    pub version_exchange_rsp: ibmvnic_version_exchange,
    pub query_capability: ibmvnic_capability,
    pub query_capability_rsp: ibmvnic_capability,
    pub request_capability: ibmvnic_capability,
    pub request_capability_rsp: ibmvnic_capability,
    pub login: ibmvnic_login,
    pub login_rsp: ibmvnic_generic_crq,
    pub query_phys_parms: ibmvnic_phys_parms,
    pub query_phys_parms_rsp: ibmvnic_phys_parms,
    pub query_phys_capabilities: ibmvnic_phys_parms,
    pub query_phys_capabilities_rsp: ibmvnic_phys_parms,
    pub set_phys_parms: ibmvnic_phys_parms,
    pub set_phys_parms_rsp: ibmvnic_phys_parms,
    pub logical_link_state: ibmvnic_logical_link_state,
    pub logical_link_state_rsp: ibmvnic_logical_link_state,
    pub query_ip_offload: ibmvnic_query_ip_offload,
    pub query_ip_offload_rsp: ibmvnic_query_ip_offload,
    pub control_ip_offload: ibmvnic_control_ip_offload,
    pub control_ip_offload_rsp: ibmvnic_control_ip_offload,
    pub request_statistics: ibmvnic_request_statistics,
    pub request_statistics_rsp: ibmvnic_generic_crq,
    pub error_indication: ibmvnic_error_indication,
    pub link_state_indication: ibmvnic_link_state_indication,
    pub change_mac_addr: ibmvnic_change_mac_addr,
    pub change_mac_addr_rsp: ibmvnic_change_mac_addr,
    pub multicast_ctrl: ibmvnic_multicast_ctrl,
    pub multicast_ctrl_rsp: ibmvnic_multicast_ctrl,
    pub get_vpd_size: ibmvnic_get_vpd_size,
    pub get_vpd_size_rsp: ibmvnic_get_vpd_size_rsp,
    pub get_vpd: ibmvnic_get_vpd,
    pub get_vpd_rsp: ibmvnic_get_vpd_rsp,
    pub acl_change_indication: ibmvnic_acl_change_indication,
    pub acl_query: ibmvnic_acl_query,
    pub acl_query_rsp: ibmvnic_generic_crq,
    pub tune: ibmvnic_tune,
    pub tune_rsp: ibmvnic_generic_crq,
    pub request_map: ibmvnic_request_map,
    pub request_map_rsp: ibmvnic_request_map_rsp,
    pub request_unmap: ibmvnic_request_unmap,
    pub request_unmap_rsp: ibmvnic_request_unmap_rsp,
    pub query_map: ibmvnic_query_map,
    pub query_map_rsp: ibmvnic_query_map_rsp,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ibmvnic_rc_codes {
    SUCCESS = 0,
    PARTIALSUCCESS = 1,
    PERMISSION = 2,
    NOMEMORY = 3,
    PARAMETER = 4,
    UNKNOWNCOMMAND = 5,
    ABORTED = 6,
    INVALIDSTATE = 7,
    INVALIDIOBA = 8,
    INVALIDLENGTH = 9,
    UNSUPPORTEDOPTION = 10,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ibmvnic_capabilities {
    MIN_TX_QUEUES = 1,
    MIN_RX_QUEUES = 2,
    MIN_RX_ADD_QUEUES = 3,
    MAX_TX_QUEUES = 4,
    MAX_RX_QUEUES = 5,
    MAX_RX_ADD_QUEUES = 6,
    REQ_TX_QUEUES = 7,
    REQ_RX_QUEUES = 8,
    REQ_RX_ADD_QUEUES = 9,
    MIN_TX_ENTRIES_PER_SUBCRQ = 10,
    MIN_RX_ADD_ENTRIES_PER_SUBCRQ = 11,
    MAX_TX_ENTRIES_PER_SUBCRQ = 12,
    MAX_RX_ADD_ENTRIES_PER_SUBCRQ = 13,
    REQ_TX_ENTRIES_PER_SUBCRQ = 14,
    REQ_RX_ADD_ENTRIES_PER_SUBCRQ = 15,
    TCP_IP_OFFLOAD = 16,
    PROMISC_REQUESTED = 17,
    PROMISC_SUPPORTED = 18,
    MIN_MTU = 19,
    MAX_MTU = 20,
    REQ_MTU = 21,
    MAX_MULTICAST_FILTERS = 22,
    VLAN_HEADER_INSERTION = 23,
    RX_VLAN_HEADER_INSERTION = 24,
    MAX_TX_SG_ENTRIES = 25,
    RX_SG_SUPPORTED = 26,
    RX_SG_REQUESTED = 27,
    OPT_TX_COMP_SUB_QUEUES = 28,
    OPT_RX_COMP_QUEUES = 29,
    OPT_RX_BUFADD_Q_PER_RX_COMP_Q = 30,
    OPT_TX_ENTRIES_PER_SUBCRQ = 31,
    OPT_RXBA_ENTRIES_PER_SUBCRQ = 32,
    TX_RX_DESC_REQ = 33,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ibmvnic_error_cause {
    ADAPTER_PROBLEM = 0,
    BUS_PROBLEM = 1,
    FW_PROBLEM = 2,
    DD_PROBLEM = 3,
    EEH_RECOVERY = 4,
    FW_UPDATED = 5,
    LOW_MEMORY = 6,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ibmvnic_commands {
    VERSION_EXCHANGE = 0x01,
    VERSION_EXCHANGE_RSP = 0x81,
    QUERY_CAPABILITY = 0x02,
    QUERY_CAPABILITY_RSP = 0x82,
    REQUEST_CAPABILITY = 0x03,
    REQUEST_CAPABILITY_RSP = 0x83,
    LOGIN = 0x04,
    LOGIN_RSP = 0x84,
    QUERY_PHYS_PARMS = 0x05,
    QUERY_PHYS_PARMS_RSP = 0x85,
    QUERY_PHYS_CAPABILITIES = 0x06,
    QUERY_PHYS_CAPABILITIES_RSP = 0x86,
    SET_PHYS_PARMS = 0x07,
    SET_PHYS_PARMS_RSP = 0x87,
    ERROR_INDICATION = 0x08,
    LOGICAL_LINK_STATE = 0x0C,
    LOGICAL_LINK_STATE_RSP = 0x8C,
    REQUEST_STATISTICS = 0x0D,
    REQUEST_STATISTICS_RSP = 0x8D,
    COLLECT_FW_TRACE = 0x11,
    COLLECT_FW_TRACE_RSP = 0x91,
    LINK_STATE_INDICATION = 0x12,
    CHANGE_MAC_ADDR = 0x13,
    CHANGE_MAC_ADDR_RSP = 0x93,
    MULTICAST_CTRL = 0x14,
    MULTICAST_CTRL_RSP = 0x94,
    GET_VPD_SIZE = 0x15,
    GET_VPD_SIZE_RSP = 0x95,
    GET_VPD = 0x16,
    GET_VPD_RSP = 0x96,
    TUNE = 0x17,
    TUNE_RSP = 0x97,
    QUERY_IP_OFFLOAD = 0x18,
    QUERY_IP_OFFLOAD_RSP = 0x98,
    CONTROL_IP_OFFLOAD = 0x19,
    CONTROL_IP_OFFLOAD_RSP = 0x99,
    ACL_CHANGE_INDICATION = 0x1A,
    ACL_QUERY = 0x1B,
    ACL_QUERY_RSP = 0x9B,
    QUERY_MAP = 0x1D,
    QUERY_MAP_RSP = 0x9D,
    REQUEST_MAP = 0x1E,
    REQUEST_MAP_RSP = 0x9E,
    REQUEST_UNMAP = 0x1F,
    REQUEST_UNMAP_RSP = 0x9F,
    VLAN_CTRL = 0x20,
    VLAN_CTRL_RSP = 0xA0,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ibmvnic_crq_type {
    IBMVNIC_CRQ_CMD			= 0x80,
    IBMVNIC_CRQ_CMD_RSP		= 0x80,
    IBMVNIC_CRQ_INIT_CMD		= 0xC0,
    IBMVNIC_CRQ_INIT_RSP		= 0xC0,
    IBMVNIC_CRQ_XPORT_EVENT		= 0xFF,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ibmvfc_crq_format {
    IBMVNIC_CRQ_INIT                 = 0x01,
    IBMVNIC_CRQ_INIT_COMPLETE        = 0x02,
    IBMVNIC_PARTITION_MIGRATED       = 0x06,
    IBMVNIC_DEVICE_FAILOVER          = 0x08,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ibmvnic_crq_queue {
    pub msgs: *mut ibmvnic_crq,
    pub cur: int size,,
    pub msg_token: dma_addr_t,
// Used for serialization of msgs, cur
    pub lock: spinlock_t,
    pub active: bool,
    pub name: [c_char; 32],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union sub_crq {
    pub generic: ibmvnic_generic_scrq,
    pub tx_comp: ibmvnic_tx_comp_desc,
    pub v1: ibmvnic_tx_desc,
    pub hdr: ibmvnic_hdr_desc,
    pub hdr_ext: ibmvnic_hdr_ext_desc,
    pub sge: ibmvnic_sge_desc,
    pub rx_comp: ibmvnic_rx_comp_desc,
    pub rx_add: ibmvnic_rx_buff_add_desc,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ibmvnic_ind_xmit_queue {
    pub indir_arr: *mut sub_crq,
    pub indir_dma: dma_addr_t,
    pub index: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ibmvnic_sub_crq_queue {
    pub msgs: *mut sub_crq,
    pub cur: int size,,
    pub msg_token: dma_addr_t,
    pub crq_num: c_ulong,
    pub hw_irq: c_ulong,
    pub irq: c_uint,
    pub pool_index: c_uint,
    pub scrq_num: c_int,
// Used for serialization of msgs, cur
    pub lock: spinlock_t,
    pub rx_skb_top: *mut sk_buff,
    pub adapter: *mut ibmvnic_adapter,
    pub ind_buf: ibmvnic_ind_xmit_queue,
    pub used: core::sync::atomic::AtomicI32,
    pub name: [c_char; 32],
    pub handle: u64,
    pub affinity_mask: cpumask_var_t,
    pub ____cacheline_aligned: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ibmvnic_long_term_buff {
    pub buff: *mut c_uchar,
    pub addr: dma_addr_t,
    pub size: u64,
    pub map_id: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ibmvnic_ltb_set {
    pub num_ltbs: c_int,
    pub ltbs: *mut ibmvnic_long_term_buff,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ibmvnic_tx_buff {
    pub skb: *mut sk_buff,
    pub index: c_int,
    pub pool_index: c_int,
    pub num_entries: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ibmvnic_tx_pool {
    pub tx_buff: *mut ibmvnic_tx_buff,
    pub free_map: *mut c_int,
    pub consumer_index: c_int,
    pub producer_index: c_int,
    pub ltb_set: ibmvnic_ltb_set,
    pub num_buffers: c_int,
    pub buf_size: c_int,
    pub ____cacheline_aligned: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ibmvnic_rx_buff {
    pub skb: *mut sk_buff,
    pub dma: dma_addr_t,
    pub data: *mut c_uchar,
    pub size: c_int,
    pub pool_index: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ibmvnic_rx_pool {
    pub rx_buff: *mut ibmvnic_rx_buff,
    pub /: *mut *mut int size; / # of buffers in the pool,
    pub index: c_int,
    pub buff_size: c_int,
    pub available: core::sync::atomic::AtomicI32,
    pub free_map: *mut c_int,
    pub next_free: c_int,
    pub next_alloc: c_int,
    pub active: c_int,
    pub ltb_set: ibmvnic_ltb_set,
    pub ____cacheline_aligned: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ibmvnic_vpd {
    pub buff: *mut c_uchar,
    pub dma_addr: dma_addr_t,
    pub len: u64,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vnic_state {
    VNIC_PROBED,
    VNIC_OPENING,
    VNIC_OPEN,
    VNIC_CLOSING,
    VNIC_CLOSED,
    VNIC_REMOVING,
    VNIC_REMOVED,
    VNIC_DOWN};

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ibmvnic_reset_reason {
    VNIC_RESET_MOBILITY,
    VNIC_RESET_FATAL,
    VNIC_RESET_NON_FATAL,
    VNIC_RESET_TIMEOUT,
    VNIC_RESET_CHANGE_PARAM,
    VNIC_RESET_PASSIVE_INIT};

    struct ibmvnic_rwi {
    enum ibmvnic_reset_reason reset_reason;
    struct list_head list;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ibmvnic_tunables {
    pub rx_queues: u64,
    pub tx_queues: u64,
    pub rx_entries: u64,
    pub tx_entries: u64,
    pub mtu: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ibmvnic_adapter {
    pub vdev: *mut vio_dev,
    pub netdev: *mut net_device,
    pub crq: ibmvnic_crq_queue,
    pub mac_addr: [u8; ETH_ALEN],
    pub ip_offload_buf: ibmvnic_query_ip_offload_buffer,
    pub ip_offload_tok: dma_addr_t,
    pub ip_offload_ctrl: ibmvnic_control_ip_offload_buffer,
    pub ip_offload_ctrl_tok: dma_addr_t,
    pub msg_enable: u32,
    pub cur_max_ind_descs: u32,
// Vital Product Data (VPD)
    pub vpd: *mut ibmvnic_vpd,
    pub fw_version: [c_char; 32],
// Statistics
    pub stats: ibmvnic_statistics,
    pub stats_token: dma_addr_t,
    pub stats_done: completion,
    pub replenish_no_mem: c_int,
    pub replenish_add_buff_success: c_int,
    pub replenish_add_buff_failure: c_int,
    pub replenish_task_cycles: c_int,
    pub tx_send_failed: c_int,
    pub tx_map_failed: c_int,
    pub tx_stats_buffers: *mut ibmvnic_tx_queue_stats,
    pub rx_stats_buffers: *mut ibmvnic_rx_queue_stats,
    pub phys_link_state: c_int,
    pub logical_link_state: c_int,
    pub speed: u32,
    pub duplex: u8,
// login data
    pub login_buf: *mut ibmvnic_login_buffer,
    pub login_buf_token: dma_addr_t,
    pub login_buf_sz: c_int,
    pub login_rsp_buf: *mut ibmvnic_login_rsp_buffer,
    pub login_rsp_buf_token: dma_addr_t,
    pub login_rsp_buf_sz: c_int,
    pub running_cap_crqs: core::sync::atomic::AtomicI32,
    pub ____cacheline_aligned: *mut *mut *mut ibmvnic_sub_crq_queue tx_scrq,
    pub ____cacheline_aligned: *mut *mut *mut ibmvnic_sub_crq_queue rx_scrq,
// rx structs
    pub napi: *mut napi_struct,
    pub rx_pool: *mut ibmvnic_rx_pool,
    pub promisc: u64,
    pub tx_pool: *mut ibmvnic_tx_pool,
    pub tso_pool: *mut ibmvnic_tx_pool,
    pub probe_done: completion,
    pub init_done: completion,
    pub init_done_rc: c_int,
    pub fw_done: completion,
// Used for serialization of device commands
    pub fw_lock: mutex,
    pub fw_done_rc: c_int,
    pub reset_done: completion,
    pub reset_done_rc: c_int,
    pub wait_for_reset: bool,
// CPU hotplug instances for online & dead
    pub node: hlist_node,
    pub node_dead: hlist_node,
// partner capabilities
    pub min_tx_queues: u64,
    pub min_rx_queues: u64,
    pub min_rx_add_queues: u64,
    pub max_tx_queues: u64,
    pub max_rx_queues: u64,
    pub max_rx_add_queues: u64,
    pub req_tx_queues: u64,
    pub req_rx_queues: u64,
    pub req_rx_add_queues: u64,
    pub min_tx_entries_per_subcrq: u64,
    pub min_rx_add_entries_per_subcrq: u64,
    pub max_tx_entries_per_subcrq: u64,
    pub max_rx_add_entries_per_subcrq: u64,
    pub req_tx_entries_per_subcrq: u64,
    pub req_rx_add_entries_per_subcrq: u64,
    pub tcp_ip_offload: u64,
    pub promisc_requested: u64,
    pub promisc_supported: u64,
    pub min_mtu: u64,
    pub max_mtu: u64,
    pub req_mtu: u64,
    pub prev_mtu: u64,
    pub max_multicast_filters: u64,
    pub vlan_header_insertion: u64,
    pub rx_vlan_header_insertion: u64,
    pub max_tx_sg_entries: u64,
    pub rx_sg_supported: u64,
    pub rx_sg_requested: u64,
    pub opt_tx_comp_sub_queues: u64,
    pub opt_rx_comp_queues: u64,
    pub opt_rx_bufadd_q_per_rx_comp_q: u64,
    pub opt_tx_entries_per_subcrq: u64,
    pub opt_rxba_entries_per_subcrq: u64,
    pub tx_rx_desc_req: __be64,
pub const MAX_MAP_ID: c_int = 255;
    pub MAX_MAP_ID): DECLARE_BITMAP(map_ids,,
    pub num_active_rx_scrqs: u32,
    pub num_active_rx_pools: u32,
    pub num_active_rx_napi: u32,
    pub num_active_tx_scrqs: u32,
    pub num_active_tx_pools: u32,
    pub prev_rx_pool_size: u32,
    pub prev_tx_pool_size: u32,
    pub cur_rx_buf_sz: u32,
    pub prev_rx_buf_sz: u32,
    pub tasklet: tasklet_struct,
    pub state: vnic_state,
// Used for serialization of state field. When taking both state
// and rwi locks, take state lock first.
//
    pub state_lock: spinlock_t,
    pub reset_reason: ibmvnic_reset_reason,
    pub rwi_list: list_head,
// Used for serialization of rwi_list. When taking both state
// and rwi locks, take state lock first
//
    pub rwi_lock: spinlock_t,
    pub ibmvnic_reset: work_struct,
    pub ibmvnic_delayed_reset: delayed_work,
    pub resetting: c_ulong,
// last device reset time
    pub last_reset_time: c_ulong,
    pub napi_enabled: bool,
    pub from_passive_init: bool,
    pub login_pending: bool,
// protected by rcu
    pub tx_queues_active: bool,
    pub failover_pending: bool,
    pub force_reset_recovery: bool,
    pub desired: ibmvnic_tunables,
    pub fallback: ibmvnic_tunables,
}
