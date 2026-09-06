//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/i40e/i40e_txrx.h
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
// Copyright(c) 2013 - 2018 Intel Corporation.

// Interrupt Throttling and Rate Limiting Goodies
pub const I40E_DEFAULT_IRQ_WORK: c_int = 256;
// The datasheet for the X710 and XL710 indicate that the maximum value for
// the ITR is 8160usec which is then called out as 0xFF0 with a 2usec
// resolution. 8160 is 0x1FE0 when written out in hex. So instead of storing
// the register value which is divided by 2 lets use the actual values and
// avoid an excessive amount of translation.
//
pub const I40E_ITR_DYNAMIC: c_uint = 0x8000	/* use top bit as a flag */;
pub const I40E_ITR_MASK: c_uint = 0x1FFE	/* mask for ITR register value */;

pub const I40E_ITR_20K: c_int = 50;
pub const I40E_ITR_8K: c_int = 122;

// 0x40 is the enable bit for interrupt rate limiting, and must be set if
// the value of the rate limit is non-zero
//

pub const I40E_MAX_INTRL: c_uint = 0x3B    /* reg uses 4 usec resolution */;

//
// i40e_intrl_usec_to_reg - convert interrupt rate limit to register
// @intrl: interrupt rate limit to convert
//
// This function converts a decimal interrupt rate limit to the appropriate
// register format expected by the firmware when setting interrupt rate limit.
//
pub const I40E_QUEUE_END_OF_LIST: c_uint = 0x7FF;
// this enum matches hardware bits and is meant to be used by DYN_CTLN
// registers and QINT registers or more generally anywhere in the manual
// mentioning ITR_INDX, ITR_NONE cannot be used as an index 'n' into any
// register but instead is a special value meaning "don't update" ITR0/1/2.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum i40e_dyn_idx {
    I40E_IDX_ITR0 = 0,
    I40E_IDX_ITR1 = 1,
    I40E_IDX_ITR2 = 2,
    I40E_ITR_NONE = 3	/* ITR_NONE must not be used as an index */
}

// these are indexes into ITRN registers

// Supported RSS offloads

// Supported Rx Buffer Sizes (a multiple of 128)
pub const I40E_RXBUFFER_256: c_int = 256;

pub const I40E_RXBUFFER_2048: c_int = 2048;

// NOTE: netdev_alloc_skb reserves up to 64 bytes, NET_IP_ALIGN means we
// reserve 2 more, and skb_shared_info adds an additional 384 bytes more,
// this adds up to 512 bytes of extra data meaning the smallest allocation
// we could have is 1K.
// i.e. RXBUFFER_256 --> 960 byte skb (size-1024 slab)
// i.e. RXBUFFER_512 --> 1216 byte skb (size-2048 slab)
//

// Attempt to maximize the headroom available for incoming frames.  We
// use a 2K buffer for receives and need 1536/1534 to store the data for
// the frame.  This leaves us with 512 bytes of room.  From that we need
// to deduct the space needed for the shared info and the padding needed
// to IP align the frame.
//
// Note: For cache line sizes 256 or larger this value is going to end
// up negative.  In these cases we should fall back to the legacy
// receive path.
//

// If a 2K buffer cannot handle a standard Ethernet frame then
// optimize padding for a 3K buffer instead of a 1.5K buffer.
//
// For a 3K buffer we need to add enough padding to allow for
// tailroom due to NET_IP_ALIGN possibly shifting us out of
// cache-line alignment.
//
// if needed make room for NET_IP_ALIGN
extern "C" {
    pub fn i40e_compute_pad(_arg: rx_buf_len) -> return;
}

//
// i40e_test_staterr - tests bits in Rx descriptor status and error fields
// @rx_desc: pointer to receive descriptor (in le64 format)
// @stat_err_bits: value to mask
//
// This function does some fast chicanery in order to return the
// value of the mask which is really only used for boolean tests.
// The status_error_len doesn't need to be shifted because it begins
// at offset zero.
//
// How many Rx Buffers do we bundle into one write to the hardware ?

pub const I40E_MAX_BUFFER_TXD: c_int = 8;
pub const I40E_MIN_TX_LEN: c_int = 17;
// The size limit for a transmit buffer in a descriptor is (16K - 1).
// In order to align with the read requests we will align the value to
// the nearest 4K which represents our maximum read request size.
//
pub const I40E_MAX_READ_REQ_SIZE: c_int = 4096;

//
// i40e_txd_use_count  - estimate the number of descriptors needed for Tx
// @size: transmit request size in bytes
//
// Due to hardware alignment restrictions (4K alignment), we need to
// assume that we can have no more than 12K of data per descriptor, even
// though each descriptor can take up to 16K - 1 bytes of aligned memory.
// Thus, we need to divide by 12K. But division is slow! Instead,
// we decompose the operation into shifts and one relatively cheap
// multiply operation.
//
// To divide by 12K, we first divide by 4K, then divide by 3:
// To divide by 4K, shift right by 12 bits
// To divide by 3, multiply by 85, then divide by 256
// (Divide by 256 is done by shifting right by 8 bits)
// Finally, we add one to round up. Because 256 isn't an exact multiple of
// 3, we'll underestimate near each multiple of 12K. This is actually more
// accurate as we have 4K - 1 of wiggle room that we can fit into the last
// segment.  For our purposes this is accurate out to 1M which is orders of
// magnitude greater than our largest possible GSO size.
//
// This would then be implemented as:
// return (((size >> 12) * 85) >> 8) + 1;
//
// Since multiplication and division are commutative, we can reorder
// operations into:
// return ((size * 85) >> 20) + 1;
//
// Tx Descriptors needed, worst case

pub const I40E_TX_FLAGS_VLAN_MASK: c_uint = 0xffff0000;
pub const I40E_TX_FLAGS_VLAN_PRIO_MASK: c_uint = 0xe0000000;
pub const I40E_TX_FLAGS_VLAN_PRIO_SHIFT: c_int = 29;
pub const I40E_TX_FLAGS_VLAN_SHIFT: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_tx_buffer {
    pub next_to_watch: *mut i40e_tx_desc,
    pub xdpf: *mut xdp_frame,
    pub skb: *mut sk_buff,
    pub raw_buf: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_rx_buffer {
    pub dma: dma_addr_t,
    pub page: *mut page,
    pub page_offset: __u32,
    pub pagecnt_bias: __u16,
    pub page_count: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_queue_stats {
    pub packets: u64,
    pub bytes: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_tx_queue_stats {
    pub restart_queue: u64,
    pub tx_busy: u64,
    pub tx_done_old: u64,
    pub tx_linearize: u64,
    pub tx_force_wb: u64,
    pub tx_stopped: u64,
    pub prev_pkt_ctr: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_rx_queue_stats {
    pub non_eop_descs: u64,
    pub alloc_page_failed: u64,
    pub alloc_buff_failed: u64,
    pub page_reuse_count: u64,
    pub page_alloc_count: u64,
    pub page_waive_count: u64,
    pub page_busy_count: u64,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum i40e_ring_state {
    __I40E_TX_FDIR_INIT_DONE,
    __I40E_TX_XPS_INIT_DONE,
    __I40E_RING_STATE_NBITS /* must be last */
}

// some useful defines for virtchannel interface, which
// is the only remaining user of header split
//
pub const I40E_RX_DTYPE_HEADER_SPLIT: c_int = 1;
pub const I40E_RX_SPLIT_L2: c_uint = 0x1;
pub const I40E_RX_SPLIT_IP: c_uint = 0x2;
pub const I40E_RX_SPLIT_TCP_UDP: c_uint = 0x4;
pub const I40E_RX_SPLIT_SCTP: c_uint = 0x8;
// struct that defines a descriptor ring, associated with a VSI
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_ring {
    pub /: *mut *mut *mut i40e_ring next; / pointer to next ring in q_vector,
    pub /: *mut *mut *mut void desc; / Descriptor ring memory,
    pub /: *mut *mut *mut device dev; / Used for DMA mapping,
    pub /: *mut *mut *mut net_device netdev; / netdev ring maps to,
    pub xdp_prog: *mut bpf_prog,
    pub tx_bi: *mut i40e_tx_buffer,
    pub rx_bi: *mut i40e_rx_buffer,
    pub rx_bi_zc: *mut xdp_buff,
}

// Storing xdp_buff on ring helps in saving the state of partially built
// packet when i40e_clean_rx_ring_irq() must return before it sees EOP
// and to resume packet building for this ring in the next call to
// i40e_clean_rx_ring_irq().
//
// Next descriptor to be processed; next_to_clean is updated only on
// processing EOP descriptor
//
// high bit set means dynamic, use accessor routines to read/write.
// hardware only supports 2us resolution for the ITR registers.
// these values always store the USER setting, and must be converted
// before programming to a register.
//
// used in interrupt processing

// stats structs
pub const I40E_ITR_ADAPTIVE_MIN_INC: c_uint = 0x0002;
pub const I40E_ITR_ADAPTIVE_MIN_USECS: c_uint = 0x0002;
pub const I40E_ITR_ADAPTIVE_MAX_USECS: c_uint = 0x007e;
pub const I40E_ITR_ADAPTIVE_LATENCY: c_uint = 0x8000;
pub const I40E_ITR_ADAPTIVE_BULK: c_uint = 0x0000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_ring_container {
    pub /: *mut *mut *mut i40e_ring ring; / pointer to linked list of ring(s),
    pub /: *mut *mut unsigned long next_update; / jiffies value of next update,
    pub /: *mut *mut unsigned int total_bytes; / total bytes processed this int,
    pub /: *mut *mut unsigned int total_packets; / total packets processed this int,
    pub count: u16,
    pub /: *mut *mut u16 target_itr; / target ITR setting for ring(s),
    pub /: *mut *mut u16 current_itr; / current ITR setting for ring(s),
}

// iterator for handling rings in ring container

extern "C" {
    pub fn i40e_alloc_rx_buffers(rxr: *mut i40e_ring, cleaned_count: u16) -> bool;
}
extern "C" {
    pub fn i40e_lan_xmit_frame(skb: *mut sk_buff, netdev: *mut net_device) -> netdev_tx_t;
}
extern "C" {
    pub fn i40e_clean_tx_ring(tx_ring: *mut i40e_ring);
}
extern "C" {
    pub fn i40e_clean_rx_ring(rx_ring: *mut i40e_ring);
}
extern "C" {
    pub fn i40e_setup_tx_descriptors(tx_ring: *mut i40e_ring) -> c_int;
}
extern "C" {
    pub fn i40e_setup_rx_descriptors(rx_ring: *mut i40e_ring) -> c_int;
}
extern "C" {
    pub fn i40e_free_tx_resources(tx_ring: *mut i40e_ring);
}
extern "C" {
    pub fn i40e_free_rx_resources(rx_ring: *mut i40e_ring);
}
extern "C" {
    pub fn i40e_napi_poll(napi: *mut napi_struct, budget: c_int) -> c_int;
}
extern "C" {
    pub fn i40e_force_wb(vsi: *mut i40e_vsi, q_vector: *mut i40e_q_vector);
}
extern "C" {
    pub fn i40e_get_tx_pending(ring: *mut i40e_ring, in_sw: bool) -> u32;
}
extern "C" {
    pub fn i40e_detect_recover_hung(pf: *mut i40e_pf);
}
extern "C" {
    pub fn __i40e_maybe_stop_tx(tx_ring: *mut i40e_ring, size: c_int) -> c_int;
}
extern "C" {
    pub fn __i40e_chk_linearize(skb: *mut sk_buff) -> bool;
}
//
// i40e_get_head - Retrieve head from head writeback
// @tx_ring:  tx ring to fetch head of
//
// Returns value of Tx ring head based on value stored
// in head write-back location
//
extern "C" {
    pub fn le32_to_cpu()head: *mut *mut (volatile __le32) -> return;
}
//
// i40e_xmit_descriptor_count - calculate number of Tx descriptors needed
// @skb:     send buffer
//
// Returns number of data descriptors needed for this skb. Returns 0 to indicate
// there is not enough descriptors available in this ring since we need at least
// one descriptor.
//
// i40e_maybe_stop_tx - 1st level check for Tx stop conditions
// @tx_ring: the ring to be checked
// @size:    the size buffer we want to assure is available
//
// Returns 0 if stop is not needed
//
extern "C" {
    pub fn __i40e_maybe_stop_tx(_arg: tx_ring, _arg: size) -> return;
}
//
// i40e_chk_linearize - Check if there are more than 8 fragments per packet
// @skb:      send buffer
// @count:    number of buffers used
//
// Note: Our HW can't scatter-gather more than 8 fragments to build
// a packet on the wire and so we need to figure out the cases where we
// need to linearize the skb.
//
// Both TSO and single send will work if count is less than 8
extern "C" {
    pub fn __i40e_chk_linearize(_arg: skb) -> return;
}
// we can support up to 8 data buffers for a single send
//
// txring_txq - Find the netdev Tx ring based on the i40e Tx ring
// @ring: Tx ring to find the netdev equivalent of
//
extern "C" {
    pub fn netdev_get_tx_queue(_arg: ring->netdev, _arg: ring->queue_index) -> return;
}
