//! Automatically rewritten from C Header to Rust Module
//! Source: tools/testing/selftests/bpf/prog_tests/test_xsk.h
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

pub const SO_PREFER_BUSY_POLL: c_int = 69;

pub const SO_BUSY_POLL_BUDGET: c_int = 70;

pub const TEST_PASS: c_int = 0;

pub const TEST_CONTINUE: c_int = 1;
pub const TEST_SKIP: c_int = 2;

pub const MIN_PKT_SIZE: c_int = 64;
pub const MAX_ETH_PKT_SIZE: c_int = 1518;
pub const MAX_INTERFACE_NAME_CHARS: c_int = 16;
pub const MAX_TEST_NAME_SIZE: c_int = 48;
pub const SOCK_RECONF_CTR: c_int = 10;
pub const USLEEP_MAX: c_int = 10000;

// Simple test
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum test_mode {
    TEST_MODE_SKB,
    TEST_MODE_DRV,
    TEST_MODE_ZC,
    TEST_MODE_ALL
}

extern "C" {
    pub fn int(ifobj: *mut *mut validation_func_t)(struct ifobject) -> typedef;
}
extern "C" {
    pub fn int(test: *mut *mut test_func_t)(struct test_spec) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xsk_socket_info {
    pub rx: xsk_ring_cons,
    pub tx: xsk_ring_prod,
    pub umem_real: *mut xsk_umem_info,
    pub umem: *mut xsk_umem_info,
    pub xsk: *mut xsk_socket,
    pub pkt_stream: *mut pkt_stream,
    pub outstanding_tx: u32,
    pub rxqsize: u32,
    pub batch_size: u32,
    pub dst_mac: [u8; ETH_ALEN],
    pub src_mac: [u8; ETH_ALEN],
    pub check_consumer: bool,
}

extern "C" {
    pub fn kick_rx(xsk: *mut xsk_socket_info) -> c_int;
}
extern "C" {
    pub fn kick_tx(xsk: *mut xsk_socket_info) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xsk_umem_info {
    pub fq: xsk_ring_prod,
    pub cq: xsk_ring_cons,
    pub umem: *mut xsk_umem,
    pub next_buffer: u64,
    pub mmap_size: u64,
    pub num_frames: u32,
    pub frame_headroom: u32,
    pub buffer: *mut c_void,
    pub frame_size: u32,
    pub base_addr: u32,
    pub fill_size: u32,
    pub comp_size: u32,
    pub unaligned_mode: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct set_hw_ring {
    pub default_tx: u32,
    pub default_rx: u32,
}

extern "C" {
    pub fn hw_ring_size_reset(ifobj: *mut ifobject) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ifobject {
    pub ifname: [c_char; MAX_INTERFACE_NAME_CHARS],
    pub xsk: *mut xsk_socket_info,
    pub xsk_arr: *mut xsk_socket_info,
    pub func_ptr: thread_func_t,
    pub validation_func: validation_func_t,
    pub xdp_progs: *mut xsk_xdp_progs,
    pub xskmap: *mut bpf_map,
    pub xdp_prog: *mut bpf_program,
    pub ring: ethtool_ringparam,
    pub set_ring: set_hw_ring,
    pub mode: test_mode,
    pub ifindex: c_int,
    pub mtu: c_int,
    pub bind_flags: u32,
    pub xdp_zc_max_segs: u32,
    pub umem_tailroom: u32,
    pub max_skb_frags: u32,
    pub tx_on: bool,
    pub rx_on: bool,
    pub use_poll: bool,
    pub busy_poll: bool,
    pub use_fill_ring: bool,
    pub release_rx: bool,
    pub shared_umem: bool,
    pub use_metadata: bool,
    pub unaligned_supp: bool,
    pub multi_buff_supp: bool,
    pub multi_buff_zc_supp: bool,
    pub hw_ring_size_supp: bool,
}

extern "C" {
    pub fn ifobject_delete(ifobj: *mut ifobject);
}
extern "C" {
    pub fn init_iface(ifobj: *mut ifobject, func_ptr: thread_func_t) -> c_int;
}
extern "C" {
    pub fn xsk_configure_umem(ifobj: *mut ifobject, umem: *mut xsk_umem_info, buffer: *mut c_void, size: u64) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pkt {
    pub offset: c_int,
    pub len: u32,
    pub pkt_nb: u32,
    pub valid: bool,
    pub options: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pkt_stream {
    pub nb_pkts: u32,
    pub current_pkt_nb: u32,
    pub pkts: *mut pkt,
    pub max_pkt_len: u32,
    pub nb_rx_pkts: u32,
    pub nb_valid_entries: u32,
    pub verbatim: bool,
}

extern "C" {
    pub fn pkt_stream_delete(pkt_stream: *mut pkt_stream);
}
extern "C" {
    pub fn pkt_stream_reset(pkt_stream: *mut pkt_stream);
}
extern "C" {
    pub fn pkt_stream_restore_default(test: *mut test_spec);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct test_spec {
    pub ifobj_tx: *mut ifobject,
    pub ifobj_rx: *mut ifobject,
    pub tx_pkt_stream_default: *mut pkt_stream,
    pub rx_pkt_stream_default: *mut pkt_stream,
    pub xdp_prog_rx: *mut bpf_program,
    pub xdp_prog_tx: *mut bpf_program,
    pub xskmap_rx: *mut bpf_map,
    pub xskmap_tx: *mut bpf_map,
    pub test_func: test_func_t,
    pub mtu: c_int,
    pub total_steps: u16,
    pub current_step: u16,
    pub nb_sockets: u16,
    pub fail: bool,
    pub set_ring: bool,
    pub adjust_tail: bool,
    pub adjust_tail_support: bool,
    pub poll_tmout: bool,
    pub use_barrier: bool,
    pub mode: test_mode,
    pub name: [c_char; MAX_TEST_NAME_SIZE],
}

extern "C" {
    pub fn testapp_adjust_tail_grow(test: *mut test_spec) -> c_int;
}
extern "C" {
    pub fn testapp_adjust_tail_grow_mb(test: *mut test_spec) -> c_int;
}
extern "C" {
    pub fn testapp_adjust_tail_shrink(test: *mut test_spec) -> c_int;
}
extern "C" {
    pub fn testapp_adjust_tail_shrink_mb(test: *mut test_spec) -> c_int;
}
extern "C" {
    pub fn testapp_aligned_inv_desc(test: *mut test_spec) -> c_int;
}
extern "C" {
    pub fn testapp_aligned_inv_desc_2k_frame(test: *mut test_spec) -> c_int;
}
extern "C" {
    pub fn testapp_aligned_inv_desc_mb(test: *mut test_spec) -> c_int;
}
extern "C" {
    pub fn testapp_bidirectional(test: *mut test_spec) -> c_int;
}
extern "C" {
    pub fn testapp_headroom(test: *mut test_spec) -> c_int;
}
extern "C" {
    pub fn testapp_hw_sw_max_ring_size(test: *mut test_spec) -> c_int;
}
extern "C" {
    pub fn testapp_hw_sw_min_ring_size(test: *mut test_spec) -> c_int;
}
extern "C" {
    pub fn testapp_poll_rx(test: *mut test_spec) -> c_int;
}
extern "C" {
    pub fn testapp_poll_rxq_tmout(test: *mut test_spec) -> c_int;
}
extern "C" {
    pub fn testapp_poll_tx(test: *mut test_spec) -> c_int;
}
extern "C" {
    pub fn testapp_poll_txq_tmout(test: *mut test_spec) -> c_int;
}
extern "C" {
    pub fn testapp_send_receive(test: *mut test_spec) -> c_int;
}
extern "C" {
    pub fn testapp_send_receive_2k_frame(test: *mut test_spec) -> c_int;
}
extern "C" {
    pub fn testapp_send_receive_mb(test: *mut test_spec) -> c_int;
}
extern "C" {
    pub fn testapp_send_receive_unaligned(test: *mut test_spec) -> c_int;
}
extern "C" {
    pub fn testapp_send_receive_unaligned_mb(test: *mut test_spec) -> c_int;
}
extern "C" {
    pub fn testapp_single_pkt(test: *mut test_spec) -> c_int;
}
extern "C" {
    pub fn testapp_stats_fill_empty(test: *mut test_spec) -> c_int;
}
extern "C" {
    pub fn testapp_stats_rx_dropped(test: *mut test_spec) -> c_int;
}
extern "C" {
    pub fn testapp_stats_tx_invalid_descs(test: *mut test_spec) -> c_int;
}
extern "C" {
    pub fn testapp_stats_rx_full(test: *mut test_spec) -> c_int;
}
extern "C" {
    pub fn testapp_teardown(test: *mut test_spec) -> c_int;
}
extern "C" {
    pub fn testapp_too_many_frags(test: *mut test_spec) -> c_int;
}
extern "C" {
    pub fn testapp_tx_queue_consumer(test: *mut test_spec) -> c_int;
}
extern "C" {
    pub fn testapp_unaligned_inv_desc(test: *mut test_spec) -> c_int;
}
extern "C" {
    pub fn testapp_unaligned_inv_desc_4001_frame(test: *mut test_spec) -> c_int;
}
extern "C" {
    pub fn testapp_unaligned_inv_desc_mb(test: *mut test_spec) -> c_int;
}
extern "C" {
    pub fn testapp_xdp_drop(test: *mut test_spec) -> c_int;
}
extern "C" {
    pub fn testapp_xdp_metadata(test: *mut test_spec) -> c_int;
}
extern "C" {
    pub fn testapp_xdp_metadata_mb(test: *mut test_spec) -> c_int;
}
extern "C" {
    pub fn testapp_xdp_prog_cleanup(test: *mut test_spec) -> c_int;
}
extern "C" {
    pub fn testapp_xdp_shared_umem(test: *mut test_spec) -> c_int;
}
// Flaky tests
// Tests with huge page dependency
// Test with HW ring size dependency
// Too long test
