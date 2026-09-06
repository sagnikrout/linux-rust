//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/xdp.h
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
// include/net/xdp.h
//
// Copyright (c) 2017 Jesper Dangaard Brouer, Red Hat Inc.
//

//
// DOC: XDP RX-queue information
//
// The XDP RX-queue info (xdp_rxq_info) is associated with the driver
// level RX-ring queues.  It is information that is specific to how
// the driver has configured a given RX-ring queue.
//
// Each xdp_buff frame received in the driver carries a (pointer)
// reference to this xdp_rxq_info structure.  This provides the XDP
// data-path read-access to RX-info for both kernel and bpf-side
// (limited subset).
//
// For now, direct access is only safe while running in NAPI/softirq
// context.  Contents are read-mostly and must not be updated during
// driver NAPI/softirq poll.
//
// The driver usage API is a register and unregister API.
//
// The struct is not directly tied to the XDP prog.  A new XDP prog
// can be attached as long as it doesn't change the underlying
// RX-ring.  If the RX-ring does change significantly, the NIC driver
// naturally needs to stop the RX-ring before purging and reallocating
// memory.  In that process the driver MUST call unregister (which
// also applies for driver shutdown and unload).  The register API is
// also mandatory during RX-ring setup.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xdp_mem_type {
    MEM_TYPE_PAGE_SHARED = 0, /* Split-page refcnt based model */
    MEM_TYPE_PAGE_ORDER0,     /* Orig XDP full page model */
    MEM_TYPE_PAGE_POOL,
    MEM_TYPE_XSK_BUFF_POOL,
    MEM_TYPE_MAX,
}

// XDP flags for ndo_xdp_xmit

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xdp_mem_info {
    pub /: *mut *mut u32 type; / enum xdp_mem_type, but known size type,
    pub id: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xdp_rxq_info {
    pub dev: *mut net_device,
    pub queue_index: u32,
    pub reg_state: u32,
    pub mem: xdp_mem_info,
    pub frag_size: u32,
    pub /: *mut *mut } ____cacheline_aligned; / perf critical, avoid false-sharing,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xdp_txq_info {
    pub dev: *mut net_device,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xdp_buff_flags {
    XDP_FLAGS_HAS_FRAGS		= BIT(0), /* non-linear xdp buff */
    XDP_FLAGS_FRAGS_PF_MEMALLOC	= BIT(1), /* xdp paged memory is under
// pressure
//
// frags have unreadable mem, this can't be true for real XDP packets,
// but drivers may use XDP helpers to construct Rx pkt state even when
// XDP program is not attached.
//
    XDP_FLAGS_FRAGS_UNREADABLE	= BIT(2),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xdp_buff {
    pub data: *mut c_void,
    pub data_end: *mut c_void,
    pub data_meta: *mut c_void,
    pub data_hard_start: *mut c_void,
    pub rxq: *mut xdp_rxq_info,
    pub txq: *mut xdp_txq_info,
// frame size to deduce data_hard_end/tailroom
    pub frame_sz: u32,
// supported values defined in xdp_buff_flags
    pub flags: u32,
}

// Used to micro-optimize xdp_init_buff(), don't use directly

//
// Force the compilers to initialize ::flags and assign ::frame_sz with
// one write on 64-bit LE architectures as they're often unable to do
// it themselves.
//

// Reserve memory area at end-of data area.
//
// This macro reserves tailroom in the XDP buffer by limiting the
// XDP/BPF data access to data_hard_end.  Notice same area (and size)
// is used for XDP_PASS, when constructing the SKB via build_skb().
//

extern "C" {
    pub fn xdp_return_frag(netmem: netmem_ref, xdp: *const xdp_buff);
}
//
// __xdp_buff_add_frag - attach frag to &xdp_buff
// @xdp: XDP buffer to attach the frag to
// @netmem: network memory containing the frag
// @offset: offset at which the frag starts
// @size: size of the frag
// @truesize: total memory size occupied by the frag
// @try_coalesce: whether to try coalescing the frags (not valid for XSk)
//
// Attach frag to the XDP buffer. If it currently has no frags attached,
// initialize the related fields, otherwise check that the frag number
// didn't reach the limit of ``MAX_SKB_FRAGS``. If possible, try coalescing
// the frag with the previous one.
// The function doesn't check/update the pfmemalloc bit. Please use the
// non-underscored wrapper in drivers.
//
// Return: true on success, false if there's no space for the frag in
// the shared info struct.
//
// Guaranteed to only decrement the refcount
//
// xdp_buff_add_frag - attach frag to &xdp_buff
// @xdp: XDP buffer to attach the frag to
// @netmem: network memory containing the frag
// @offset: offset at which the frag starts
// @size: size of the frag
// @truesize: total memory size occupied by the frag
//
// Version of __xdp_buff_add_frag() which takes care of the pfmemalloc bit.
//
// Return: true on success, false if there's no space for the frag in
// the shared info struct.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xdp_frame {
    pub data: *mut c_void,
    pub len: u32,
    pub headroom: u32,
    pub /: *mut *mut u32 metasize; / uses lower 8-bits,
// Lifetime of xdp_rxq_info is limited to NAPI/enqueue time,
// while mem_type is valid on remote CPU.
//
    pub mem_type:32: xdp_mem_type,
    pub /: *mut *mut *mut net_device dev_rx; / used by cpumap,
    pub frame_sz: u32,
    pub /: *mut *mut u32 flags; / supported values defined in xdp_buff_flags,
}

pub const XDP_BULK_QUEUE_SIZE: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xdp_frame_bulk {
    pub count: c_int,
    pub q: [netmem_ref; XDP_BULK_QUEUE_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xdp_cpumap_stats {
    pub redirect: c_uint,
    pub pass: c_uint,
    pub drop: c_uint,
}

// Clear kernel pointers in xdp_frame
//
// ``destructor_arg`` is unionized with ``xdp_frags_{,true}size``,
// reset it after that these fields aren't used anymore.
//
// Avoids inlining WARN macro in fast-path
extern "C" {
    pub fn xdp_warn(msg: *const c_char, func: *const c_char, line: c_int);
}

// Assure headroom is available for storing info
// Catch if driver didn't reserve tailroom for skb_shared_info
// Convert xdp_buff to xdp_frame
extern "C" {
    pub fn xdp_convert_zc_to_xdp_frame(_arg: xdp) -> return;
}
// Store info in top of packet
// rxq only valid until napi_schedule ends, convert to xdp_mem_type
extern "C" {
    pub fn xdp_return_frame(xdpf: *mut xdp_frame);
}
extern "C" {
    pub fn xdp_return_frame_rx_napi(xdpf: *mut xdp_frame);
}
extern "C" {
    pub fn xdp_return_buff(xdp: *mut xdp_buff);
}
extern "C" {
    pub fn __xdp_rxq_info_reg(_arg: xdp_rxq, _arg: dev, _arg: queue_index, _arg: napi_id, _arg: 0) -> return;
}
extern "C" {
    pub fn xdp_rxq_info_unreg(xdp_rxq: *mut xdp_rxq_info);
}
extern "C" {
    pub fn xdp_rxq_info_unused(xdp_rxq: *mut xdp_rxq_info);
}
extern "C" {
    pub fn xdp_rxq_info_is_reg(xdp_rxq: *mut xdp_rxq_info) -> bool;
}
extern "C" {
    pub fn xdp_rxq_info_unreg_mem_model(xdp_rxq: *mut xdp_rxq_info);
}
extern "C" {
    pub fn xdp_unreg_mem_model(mem: *mut xdp_mem_info);
}
extern "C" {
    pub fn xdp_reg_page_pool(pool: *mut page_pool) -> c_int;
}
extern "C" {
    pub fn xdp_unreg_page_pool(pool: *const page_pool);
}
//
// xdp_rxq_info_attach_mem_model - attach registered mem info to RxQ info
// @xdp_rxq: XDP RxQ info to attach the memory info to
// @mem: already registered memory info
//
// If the driver registers its memory providers manually, it must use this
// function instead of xdp_rxq_info_reg_mem_model().
//
// xdp_rxq_info_detach_mem_model - detach registered mem info from RxQ info
// @xdp_rxq: XDP RxQ info to detach the memory info from
//
// If the driver registers its memory providers manually and then attaches it
// via xdp_rxq_info_attach_mem_model(), it must call this function before
// xdp_rxq_info_unreg().
//
// Drivers not supporting XDP metadata can use this helper, which
// rejects any room expansion for metadata as a result.
//
extern "C" {
    pub fn unlikely(xdp->data: xdp->data_meta >) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xdp_attachment_info {
    pub prog: *mut bpf_prog,
    pub flags: u32,
}

// Define the relationship between xdp-rx-metadata kfunc and
// various other entities:
// - xdp_rx_metadata enum
// - netdev netlink enum (Documentation/netlink/specs/netdev.yaml)
// - kfunc name
// - xdp_metadata_ops field
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xdp_rx_metadata {

    XDP_METADATA_KFUNC_xxx

    MAX_XDP_METADATA_KFUNC,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xdp_rss_hash_type {
// First part: Individual bits for L3/L4 types
    XDP_RSS_L3_IPV4		= BIT(0),
    XDP_RSS_L3_IPV6		= BIT(1),

// The fixed (L3) IPv4 and IPv6 headers can both be followed by
// variable/dynamic headers, IPv4 called Options and IPv6 called
// Extension Headers. HW RSS type can contain this info.
//
    XDP_RSS_L3_DYNHDR	= BIT(2),

// When RSS hash covers L4 then drivers MUST set XDP_RSS_L4 bit in
// addition to the protocol specific bit.  This ease interaction with
// SKBs and avoids reserving a fixed mask for future L4 protocol bits.
//
    XDP_RSS_L4		= BIT(3), /* L4 based hash, proto can be unknown */
    XDP_RSS_L4_TCP		= BIT(4),
    XDP_RSS_L4_UDP		= BIT(5),
    XDP_RSS_L4_SCTP		= BIT(6),
    XDP_RSS_L4_IPSEC	= BIT(7), /* L4 based hash include IPSEC SPI */
    XDP_RSS_L4_ICMP		= BIT(8),

// Second part: RSS hash type combinations used for driver HW mapping
    XDP_RSS_TYPE_NONE            = 0,
    XDP_RSS_TYPE_L2              = XDP_RSS_TYPE_NONE,

    XDP_RSS_TYPE_L3_IPV4         = XDP_RSS_L3_IPV4,
    XDP_RSS_TYPE_L3_IPV6         = XDP_RSS_L3_IPV6,
    XDP_RSS_TYPE_L3_IPV4_OPT     = XDP_RSS_L3_IPV4 | XDP_RSS_L3_DYNHDR,
    XDP_RSS_TYPE_L3_IPV6_EX      = XDP_RSS_L3_IPV6 | XDP_RSS_L3_DYNHDR,

    XDP_RSS_TYPE_L4_ANY          = XDP_RSS_L4,
    XDP_RSS_TYPE_L4_IPV4_TCP     = XDP_RSS_L3_IPV4 | XDP_RSS_L4 | XDP_RSS_L4_TCP,
    XDP_RSS_TYPE_L4_IPV4_UDP     = XDP_RSS_L3_IPV4 | XDP_RSS_L4 | XDP_RSS_L4_UDP,
    XDP_RSS_TYPE_L4_IPV4_SCTP    = XDP_RSS_L3_IPV4 | XDP_RSS_L4 | XDP_RSS_L4_SCTP,
    XDP_RSS_TYPE_L4_IPV4_IPSEC   = XDP_RSS_L3_IPV4 | XDP_RSS_L4 | XDP_RSS_L4_IPSEC,
    XDP_RSS_TYPE_L4_IPV4_ICMP    = XDP_RSS_L3_IPV4 | XDP_RSS_L4 | XDP_RSS_L4_ICMP,

    XDP_RSS_TYPE_L4_IPV6_TCP     = XDP_RSS_L3_IPV6 | XDP_RSS_L4 | XDP_RSS_L4_TCP,
    XDP_RSS_TYPE_L4_IPV6_UDP     = XDP_RSS_L3_IPV6 | XDP_RSS_L4 | XDP_RSS_L4_UDP,
    XDP_RSS_TYPE_L4_IPV6_SCTP    = XDP_RSS_L3_IPV6 | XDP_RSS_L4 | XDP_RSS_L4_SCTP,
    XDP_RSS_TYPE_L4_IPV6_IPSEC   = XDP_RSS_L3_IPV6 | XDP_RSS_L4 | XDP_RSS_L4_IPSEC,
    XDP_RSS_TYPE_L4_IPV6_ICMP    = XDP_RSS_L3_IPV6 | XDP_RSS_L4 | XDP_RSS_L4_ICMP,

    XDP_RSS_TYPE_L4_IPV6_TCP_EX  = XDP_RSS_TYPE_L4_IPV6_TCP  | XDP_RSS_L3_DYNHDR,
    XDP_RSS_TYPE_L4_IPV6_UDP_EX  = XDP_RSS_TYPE_L4_IPV6_UDP  | XDP_RSS_L3_DYNHDR,
    XDP_RSS_TYPE_L4_IPV6_SCTP_EX = XDP_RSS_TYPE_L4_IPV6_SCTP | XDP_RSS_L3_DYNHDR,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xdp_metadata_ops {
    pub timestamp): *const *const *const int (xmo_rx_timestamp)(struct xdp_md ctx, u64,
    pub rss_type): *mut xdp_rss_hash_type,
    pub vlan_tci): *mut u16,
}

extern "C" {
    pub fn bpf_xdp_metadata_kfunc_id(id: c_int) -> u32;
}
extern "C" {
    pub fn bpf_dev_bound_kfunc_id(btf_id: u32) -> bool;
}
extern "C" {
    pub fn xdp_set_features_flag(dev: *mut net_device, val: xdp_features_t);
}
extern "C" {
    pub fn xdp_set_features_flag_locked(dev: *mut net_device, val: xdp_features_t);
}
extern "C" {
    pub fn xdp_features_set_redirect_target(dev: *mut net_device, support_sg: bool);
}
extern "C" {
    pub fn xdp_features_clear_redirect_target(dev: *mut net_device);
}
extern "C" {
    pub fn xdp_features_clear_redirect_target_locked(dev: *mut net_device);
}

// Driver XDP hooks are invoked within a single NAPI poll cycle and thus
// under local_bh_disable(), which provides the needed RCU protection
// for accessing map entries.
//
