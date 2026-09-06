//! Automatically rewritten from C Header to Rust Module
//! Source: net/packet/internal.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct packet_mclist {
    pub next: *mut packet_mclist,
    pub ifindex: c_int,
    pub count: c_int,
    pub type: c_ushort,
    pub alen: c_ushort,
    pub addr: [c_uchar; MAX_ADDR_LEN],
    pub remove_list: list_head,
}

// kbdq - kernel block descriptor queue
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tpacket_kbdq_core {
    pub pkbdq: *mut pgv,
    pub feature_req_word: c_uint,
    pub hdrlen: c_uint,
    pub reset_pending_on_curr_blk: c_uchar,
    pub kactive_blk_num: c_ushort,
    pub blk_sizeof_priv: c_ushort,
    pub version: c_ushort,
    pub pkblk_start: *mut c_char,
    pub pkblk_end: *mut c_char,
    pub kblk_size: c_int,
    pub max_frame_len: c_uint,
    pub knum_blocks: c_uint,
    pub knxt_seq_num: u64,
    pub prev: *mut c_char,
    pub nxt_offset: *mut c_char,
    pub skb: *mut sk_buff,
    pub blk_fill_in_prog_lock: rwlock_t,
// Default is set to 8ms

    pub interval_ktime: ktime_t,
// timer to retire an outstanding block
    pub retire_blk_timer: hrtimer,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pgv {
    pub buffer: *mut c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct packet_ring_buffer {
    pub pg_vec: *mut pgv,
    pub head: c_uint,
    pub frames_per_block: c_uint,
    pub frame_size: c_uint,
    pub frame_max: c_uint,
    pub pg_vec_order: c_uint,
    pub pg_vec_pages: c_uint,
    pub pg_vec_len: c_uint,
    pub pending_refcnt: *mut unsigned int __percpu,
    pub rx_owner_map: *mut c_ulong,
    pub prb_bdqc: tpacket_kbdq_core,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct packet_fanout {
    pub net: possible_net_t,
    pub num_members: c_uint,
    pub max_num_members: u32,
    pub id: u16,
    pub type: u8,
    pub flags: u8,
    pub rr_cur: core::sync::atomic::AtomicI32,
    pub bpf_prog: *mut bpf_prog __rcu,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct packet_rollover {
    pub sock: c_int,
    pub num: atomic_long_t,
    pub num_huge: atomic_long_t,
    pub num_failed: atomic_long_t,

    pub ____cacheline_aligned: u32 history[ROLLOVER_HLEN],
    pub ____cacheline_aligned_in_smp: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct packet_sock {
// struct sock has to be the first member of packet_sock
    pub sk: sock,
    pub fanout: *mut packet_fanout,
    pub stats: tpacket_stats_u,
    pub rx_ring: packet_ring_buffer,
    pub tx_ring: packet_ring_buffer,
    pub copy_thresh: c_int,
    pub bind_lock: spinlock_t,
    pub pg_vec_lock: mutex,
    pub flags: c_ulong,
    pub /: *mut *mut int ifindex; / bound device,
    pub vnet_hdr_sz: u8,
    pub num: __be16,
    pub rollover: *mut packet_rollover,
    pub mclist: *mut packet_mclist,
    pub mapped: atomic_long_t,
    pub tp_version: tpacket_versions,
    pub tp_hdrlen: c_uint,
    pub tp_reserve: c_uint,
    pub tp_tstamp: c_uint,
    pub skb_completion: completion,
    pub cached_dev: *mut net_device __rcu,
    pub ____cacheline_aligned_in_smp: packet_type prot_hook,
    pub ____cacheline_aligned_in_smp: atomic_t tp_drops,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum packet_sock_flags {
    PACKET_SOCK_ORIGDEV,
    PACKET_SOCK_AUXDATA,
    PACKET_SOCK_TX_HAS_OFF,
    PACKET_SOCK_TP_LOSS,
    PACKET_SOCK_RUNNING,
    PACKET_SOCK_PRESSURE,
    PACKET_SOCK_QDISC_BYPASS,
}

extern "C" {
    pub fn test_bit(_arg: flag, _arg: &po->flags) -> return;
}
