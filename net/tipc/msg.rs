//! Automatically rewritten from C Header to Rust Module
//! Source: net/tipc/msg.h
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


//
// net/tipc/msg.h: Include file for TIPC message header routines
//
// Copyright (c) 2000-2007, 2014-2017 Ericsson AB
// Copyright (c) 2005-2008, 2010-2011, Wind River Systems
// All rights reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are met:
//
// 1. Redistributions of source code must retain the above copyright
// notice, this list of conditions and the following disclaimer.
// 2. Redistributions in binary form must reproduce the above copyright
// notice, this list of conditions and the following disclaimer in the
// documentation and/or other materials provided with the distribution.
// 3. Neither the names of the copyright holders nor the names of its
// contributors may be used to endorse or promote products derived from
// this software without specific prior written permission.
//
// Alternatively, this software may be distributed under the terms of the
// GNU General Public License ("GPL") version 2 as published by the Free
// Software Foundation.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
// AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
// IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
// ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR CONTRIBUTORS BE
// LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
// CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
// SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
// INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
// CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
// ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
// POSSIBILITY OF SUCH DAMAGE.
//

//
// Constants and routines used to read and write TIPC payload message headers
//
// Note: Some items are also used with TIPC internal message headers
//
pub const TIPC_VERSION: c_int = 2;
//
// Payload message users are defined in TIPC's public API:
// - TIPC_LOW_IMPORTANCE
// - TIPC_MEDIUM_IMPORTANCE
// - TIPC_HIGH_IMPORTANCE
// - TIPC_CRITICAL_IMPORTANCE
//
pub const TIPC_SYSTEM_IMPORTANCE: c_int = 4;
//
// Payload message types
//
pub const TIPC_CONN_MSG: c_int = 0;
pub const TIPC_MCAST_MSG: c_int = 1;
pub const TIPC_NAMED_MSG: c_int = 2;
pub const TIPC_DIRECT_MSG: c_int = 3;
pub const TIPC_GRP_MEMBER_EVT: c_int = 4;
pub const TIPC_GRP_BCAST_MSG: c_int = 5;
pub const TIPC_GRP_MCAST_MSG: c_int = 6;
pub const TIPC_GRP_UCAST_MSG: c_int = 7;
//
// Internal message users
//
pub const BCAST_PROTOCOL: c_int = 5;
pub const MSG_BUNDLER: c_int = 6;
pub const LINK_PROTOCOL: c_int = 7;
pub const CONN_MANAGER: c_int = 8;
pub const GROUP_PROTOCOL: c_int = 9;
pub const TUNNEL_PROTOCOL: c_int = 10;
pub const NAME_DISTRIBUTOR: c_int = 11;
pub const MSG_FRAGMENTER: c_int = 12;
pub const LINK_CONFIG: c_int = 13;
pub const MSG_CRYPTO: c_int = 14;

//
// Message header sizes
//

pub const TIPC_MEDIA_INFO_OFFSET: c_int = 5;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tipc_skb_cb {
    pub tail: *mut sk_buff,
    pub nxt_retr: c_ulong,
    pub retr_stamp: c_ulong,
    pub bytes_read: u32,
    pub orig_member: u32,
    pub chain_imp: u16,
    pub ackers: u16,
    pub retr_cnt: u16,
    pub __packed: },

    pub rx: *mut tipc_crypto,
    pub last: *mut tipc_aead,
    pub recurs: u8,
    pub __packed: } tx_clone_ctx,

    pub __packed: },
    pub validated:1: u8,

    pub encrypted:1: u8,
    pub decrypted:1: u8,
pub const SKB_PROBING: c_int = 1;
pub const SKB_GRACING: c_int = 2;
    pub xmit_type:2: u8,
    pub tx_clone_deferred:1: u8,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tipc_msg {
    pub hdr: [__be32; 15],
}

// struct tipc_gap_ack - TIPC Gap ACK block
// @ack: seqno of the last consecutive packet in link deferdq
// @gap: number of gap packets since the last ack
//
// E.g:
// link deferdq: 1 2 3 4      10 11      13 14 15       20
// --> Gap ACK blocks:      <4, 5>,   <11, 1>,      <15, 4>, <20, 0>
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tipc_gap_ack {
    pub ack: __be16,
    pub gap: __be16,
}

// struct tipc_gap_ack_blks
// @len: actual length of the record
// @ugack_cnt: number of Gap ACK blocks for unicast (following the broadcast
// ones)
// @start_index: starting index for "valid" broadcast Gap ACK blocks
// @bgack_cnt: number of Gap ACK blocks for broadcast in the record
// @gacks: array of Gap ACK blocks
//
// 31                       16 15                        0
// +-------------+-------------+-------------+-------------+
// |  bgack_cnt  |  ugack_cnt  |            len            |
// +-------------+-------------+-------------+-------------+  -
// |            gap            |            ack            |   |
// +-------------+-------------+-------------+-------------+    > bc gacks
// :                           :                           :   |
// +-------------+-------------+-------------+-------------+  -
// |            gap            |            ack            |   |
// +-------------+-------------+-------------+-------------+    > uc gacks
// :                           :                           :   |
// +-------------+-------------+-------------+-------------+  -
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tipc_gap_ack_blks {
    pub len: __be16,
    pub ugack_cnt: u8,
    pub start_index: u8,
}

pub const MAX_GAP_ACK_BLKS: c_int = 128;

extern "C" {
    pub fn ntohl(_arg: m->hdr[pos]) -> return;
}
//
// Word 0
//
extern "C" {
    pub fn msg_bits(_arg: m, _arg: 0, _arg: 29, _arg: 7) -> return;
}
extern "C" {
    pub fn msg_bits(_arg: m, _arg: 0, _arg: 25, _arg: 0xf) -> return;
}
extern "C" {
    pub fn msg_bits(_arg: m, _arg: 0, _arg: 0, _arg: 0x1ffff) -> return;
}
extern "C" {
    pub fn msg_size(msg_hdr_sz(m: m) -) -> return;
}
extern "C" {
    pub fn msg_bits(_arg: m, _arg: 0, _arg: 20, _arg: 1) -> return;
}
extern "C" {
    pub fn msg_bits(_arg: m, _arg: 0, _arg: 17, _arg: 1) -> return;
}
extern "C" {
    pub fn msg_bits(_arg: m, _arg: 0, _arg: 19, _arg: 1) -> return;
}
extern "C" {
    pub fn msg_bits(_arg: m, _arg: 0, _arg: 19, _arg: 1) -> return;
}
extern "C" {
    pub fn msg_bits(_arg: m, _arg: 0, _arg: 18, _arg: 1) -> return;
}
extern "C" {
    pub fn msg_bits(_arg: m, _arg: 0, _arg: 18, _arg: 1) -> return;
}
extern "C" {
    pub fn msg_bits(_arg: m, _arg: 0, _arg: 18, _arg: 1) -> return;
}
extern "C" {
    pub fn msg_bits(_arg: m, _arg: 0, _arg: 18, _arg: 0x1) -> return;
}
//
// Word 1
//
extern "C" {
    pub fn msg_bits(_arg: m, _arg: 1, _arg: 29, _arg: 0x7) -> return;
}
extern "C" {
    pub fn msg_bits(_arg: m, _arg: 1, _arg: 25, _arg: 0xf) -> return;
}
extern "C" {
    pub fn msg_bits(_arg: m, _arg: 1, _arg: 28, _arg: 0x1) -> return;
}
extern "C" {
    pub fn msg_bits(_arg: m, _arg: 1, _arg: 27, _arg: 0x1) -> return;
}
extern "C" {
    pub fn msg_bits(_arg: m, _arg: 1, _arg: 21, _arg: 0xf) -> return;
}
extern "C" {
    pub fn msg_bits(_arg: m, _arg: 1, _arg: 19, _arg: 0x3) -> return;
}
extern "C" {
    pub fn msg_bits(_arg: m, _arg: 1, _arg: 0, _arg: 0xffff) -> return;
}
// Note: reusing bits in word 1 for ACTIVATE_MSG only, to re-synch
// link peer session number
//
extern "C" {
    pub fn msg_bits(_arg: m, _arg: 1, _arg: 16, _arg: 0x1) -> return;
}
extern "C" {
    pub fn msg_bits(_arg: m, _arg: 1, _arg: 0, _arg: 0xffff) -> return;
}
//
// Word 2
//
extern "C" {
    pub fn msg_bits(_arg: m, _arg: 2, _arg: 16, _arg: 0xffff) -> return;
}
extern "C" {
    pub fn msg_bits(_arg: m, _arg: 2, _arg: 0, _arg: 0xffff) -> return;
}
//
// Words 3-10
//
extern "C" {
    pub fn msg_bits(_arg: m, _arg: 9, _arg: 0, _arg: 0x7) -> return;
}
extern "C" {
    pub fn msg_word(_arg: m, _arg: 3) -> return;
}
extern "C" {
    pub fn msg_word(_arg: m, _arg: 4) -> return;
}
extern "C" {
    pub fn msg_bits(_arg: m, _arg: 4, _arg: 0, _arg: 0xffff) -> return;
}
extern "C" {
    pub fn msg_word(_arg: m, _arg: 5) -> return;
}
extern "C" {
    pub fn msg_word(_arg: m, _arg: 5) -> return;
}
extern "C" {
    pub fn msg_prevnode(_arg: m) -> return;
}
extern "C" {
    pub fn msg_word(_arg: m, _arg: 6) -> return;
}
extern "C" {
    pub fn msg_word(_arg: m, _arg: 7) -> return;
}
extern "C" {
    pub fn msg_word(_arg: m, _arg: 8) -> return;
}
extern "C" {
    pub fn msg_word(_arg: m, _arg: 9) -> return;
}
extern "C" {
    pub fn msg_nameinst(_arg: m) -> return;
}
extern "C" {
    pub fn msg_word(_arg: m, _arg: 10) -> return;
}
//
// Constants and routines used to read and write TIPC internal message headers
//
// Connection management protocol message types
//
pub const CONN_PROBE: c_int = 0;
pub const CONN_PROBE_REPLY: c_int = 1;
pub const CONN_ACK: c_int = 2;
//
// Name distributor message types
//
pub const PUBLICATION: c_int = 0;
pub const WITHDRAWAL: c_int = 1;
//
// Segmentation message types
//
pub const FIRST_FRAGMENT: c_int = 0;
pub const FRAGMENT: c_int = 1;
pub const LAST_FRAGMENT: c_int = 2;
//
// Link management protocol message types
//
pub const STATE_MSG: c_int = 0;
pub const RESET_MSG: c_int = 1;
pub const ACTIVATE_MSG: c_int = 2;
//
// Changeover tunnel message types
//
pub const SYNCH_MSG: c_int = 0;
pub const FAILOVER_MSG: c_int = 1;
//
// Config protocol message types
//
pub const DSC_REQ_MSG: c_int = 0;
pub const DSC_RESP_MSG: c_int = 1;
pub const DSC_TRIAL_MSG: c_int = 2;
pub const DSC_TRIAL_FAIL_MSG: c_int = 3;
//
// Group protocol message types
//
pub const GRP_JOIN_MSG: c_int = 0;
pub const GRP_LEAVE_MSG: c_int = 1;
pub const GRP_ADV_MSG: c_int = 2;
pub const GRP_ACK_MSG: c_int = 3;
pub const GRP_RECLAIM_MSG: c_int = 4;
pub const GRP_REMIT_MSG: c_int = 5;
// Crypto message types
pub const KEY_DISTR_MSG: c_int = 0;
//
// Word 1
//
extern "C" {
    pub fn msg_bits(_arg: m, _arg: 1, _arg: 16, _arg: 0x1fff) -> return;
}
extern "C" {
    pub fn msg_bits(_arg: m, _arg: 1, _arg: 0, _arg: 0xffff) -> return;
}
extern "C" {
    pub fn msg_bits(_arg: m, _arg: 1, _arg: 15, _arg: 0x1fff) -> return;
}
//
// Word 2
//
extern "C" {
    pub fn msg_word(_arg: m, _arg: 2) -> return;
}
extern "C" {
    pub fn msg_bits(_arg: m, _arg: 2, _arg: 0, _arg: 0xffff) -> return;
}
//
// Word 4
//
extern "C" {
    pub fn msg_bits(_arg: m, _arg: 4, _arg: 16, _arg: 0xffff) -> return;
}
extern "C" {
    pub fn msg_bits(_arg: m, _arg: 4, _arg: 0, _arg: 0xffff) -> return;
}
extern "C" {
    pub fn msg_bits(_arg: m, _arg: 4, _arg: 16, _arg: 0xffff) -> return;
}
extern "C" {
    pub fn msg_bits(_arg: m, _arg: 4, _arg: 0, _arg: 0xffff) -> return;
}
extern "C" {
    pub fn msg_word(_arg: m, _arg: 4) -> return;
}
extern "C" {
    pub fn msg_bits(_arg: m, _arg: 4, _arg: 0, _arg: 1) -> return;
}
//
// Word 5
//
extern "C" {
    pub fn msg_bits(_arg: m, _arg: 5, _arg: 16, _arg: 0xffff) -> return;
}
extern "C" {
    pub fn msg_bits(_arg: m, _arg: 5, _arg: 0, _arg: 1) -> return;
}
extern "C" {
    pub fn msg_bits(_arg: m, _arg: 5, _arg: 4, _arg: 0x1f) -> return;
}
extern "C" {
    pub fn msg_bits(_arg: m, _arg: 5, _arg: 9, _arg: 0x7) -> return;
}
extern "C" {
    pub fn msg_bits(_arg: m, _arg: 5, _arg: 12, _arg: 0x1) -> return;
}
extern "C" {
    pub fn msg_bits(_arg: m, _arg: 5, _arg: 13, _arg: 0x1) -> return;
}
extern "C" {
    pub fn msg_bits(_arg: m, _arg: 5, _arg: 14, _arg: 0x1) -> return;
}
extern "C" {
    pub fn msg_bits(_arg: m, _arg: 8, _arg: 0, _arg: 0x3ff) -> return;
}
//
// Word 9
//
extern "C" {
    pub fn msg_bits(_arg: m, _arg: 9, _arg: 16, _arg: 0xffff) -> return;
}
extern "C" {
    pub fn msg_bits(_arg: m, _arg: 9, _arg: 16, _arg: 0xffff) -> return;
}
extern "C" {
    pub fn msg_bits(_arg: m, _arg: 9, _arg: 16, _arg: 0xffff) -> return;
}
extern "C" {
    pub fn msg_bits(_arg: m, _arg: 9, _arg: 0, _arg: 0xffff) -> return;
}
extern "C" {
    pub fn msg_bits(_arg: m, _arg: 9, _arg: 0, _arg: 0xffff) -> return;
}
extern "C" {
    pub fn msg_bits(_arg: m, _arg: 9, _arg: 16, _arg: 0xffff) -> return;
}
extern "C" {
    pub fn msg_bits(_arg: m, _arg: 9, _arg: 16, _arg: 0xffff) -> return;
}
extern "C" {
    pub fn msg_bits(_arg: m, _arg: 9, _arg: 16, _arg: 0xffff) -> return;
}
// Word 10
//
extern "C" {
    pub fn msg_bits(_arg: m, _arg: 10, _arg: 0, _arg: 0x3) -> return;
}
extern "C" {
    pub fn msg_bits(_arg: m, _arg: 10, _arg: 0, _arg: 0x1) -> return;
}
extern "C" {
    pub fn msg_bits(_arg: m, _arg: 10, _arg: 16, _arg: 0xffff) -> return;
}
extern "C" {
    pub fn msg_redundant_link(_arg: m) -> return;
}
// Word 13
//
extern "C" {
    pub fn msg_word(_arg: m, _arg: 13) -> return;
}
// Word 14
//
extern "C" {
    pub fn msg_word(_arg: m, _arg: 14) -> return;
}
extern "C" {
    pub fn tipc_msg_validate(_skb: *mut sk_buff) -> bool;
}
extern "C" {
    pub fn tipc_msg_reverse(own_addr: u32, skb: *mut sk_buff, err: c_int) -> bool;
}
extern "C" {
    pub fn tipc_buf_append(headbuf: *mut sk_buff, buf: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn tipc_msg_extract(skb: *mut sk_buff, iskb: *mut sk_buff, pos: *mut c_int) -> bool;
}
extern "C" {
    pub fn tipc_msg_lookup_dest(net: *mut net, skb: *mut sk_buff, err: *mut c_int) -> bool;
}
extern "C" {
    pub fn tipc_msg_assemble(list: *mut sk_buff_head) -> bool;
}
extern "C" {
    pub fn tipc_msg_reassemble(list: *mut sk_buff_head, rcvq: *mut sk_buff_head) -> bool;
}
extern "C" {
    pub fn tipc_msg_skb_clone(msg: *mut sk_buff_head, cpy: *mut sk_buff_head) -> bool;
}
extern "C" {
    pub fn msg_seqno(_arg: buf_msg(skb)) -> return;
}
// tipc_skb_peek(): peek and reserve first buffer in list
// @list: list to be peeked in
// Returns pointer to first buffer in list, if any
//
// tipc_skb_peek_port(): find a destination port, ignoring all destinations
// up to and including 'filter'.
// Note: ignoring previously tried destinations minimizes the risk of
// contention on the socket lock
// @list: list to be peeked in
// @filter: last destination to be ignored from search
// Returns a destination port number, of applicable.
//
// tipc_skb_dequeue(): unlink first buffer with dest 'dport' from list
// @list: list to be unlinked from
// @dport: selection criteria for buffer to unlink
//
// tipc_skb_queue_splice_tail - append an skb list to lock protected list
// @list: the new list to append. Not lock protected
// @head: target list. Lock protected.
//
// tipc_skb_queue_splice_tail_init - merge two lock protected skb lists
// @list: the new list to add. Lock protected. Will be reinitialized
// @head: target list. Lock protected.
//
// __tipc_skb_dequeue() - dequeue the head skb according to expected seqno
// @list: list to be dequeued from
// @seqno: seqno of the expected msg
//
// returns skb dequeued from the list if its seqno is less than or equal to
// the expected one, otherwise the skb is still hold
//
// Note: must be used with appropriate locks held only
//
