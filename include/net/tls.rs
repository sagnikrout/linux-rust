//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/tls.h
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
// Copyright (c) 2016-2017, Mellanox Technologies. All rights reserved.
// Copyright (c) 2016-2017, Dave Watson <davejwatson@fb.com>. All rights reserved.
//
// This software is available to you under a choice of one of two
// licenses.  You may choose to be licensed under the terms of the GNU
// General Public License (GPL) Version 2, available from the file
// COPYING in the main directory of this source tree, or the
// OpenIB.org BSD license below:
//
// Redistribution and use in source and binary forms, with or
// without modification, are permitted provided that the following
// conditions are met:
//
// - Redistributions of source code must retain the above
// copyright notice, this list of conditions and the following
// disclaimer.
//
// - Redistributions in binary form must reproduce the above
// copyright notice, this list of conditions and the following
// disclaimer in the documentation and/or other materials
// provided with the distribution.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS
// BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN
// ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//

// Maximum data size carried in a TLS record

// Minimum record size limit as per RFC8449

pub const TLS_HEADER_SIZE: c_int = 5;

pub const TLS_AAD_SPACE_SIZE: c_int = 13;
pub const TLS_MAX_IV_SIZE: c_int = 16;
pub const TLS_MAX_SALT_SIZE: c_int = 4;
pub const TLS_TAG_SIZE: c_int = 16;
pub const TLS_MAX_REC_SEQ_SIZE: c_int = 8;

// For CCM mode, the full 16-bytes of IV is made of '4' fields of given sizes.
//
// IV[16] = b0[1] || implicit nonce[4] || explicit nonce[8] || length[3]
//
// The field 'length' is encoded in field 'b0' as '(length width - 1)'.
// Hence b0 contains (3 - 1) = 2.
//
pub const TLS_AES_CCM_IV_B0_BYTE: c_int = 2;
pub const TLS_SM4_CCM_IV_B0_BYTE: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tx_work {
    pub work: delayed_work,
    pub sk: *mut sock,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tls_sw_context_tx {
    pub aead_send: *mut crypto_aead,
    pub async_wait: crypto_wait,
    pub tx_work: tx_work,
    pub open_rec: *mut tls_rec,
    pub tx_list: list_head,
    pub encrypt_pending: core::sync::atomic::AtomicI32,
    pub async_capable:1: u8,
pub const BIT_TX_SCHEDULED: c_int = 0;
pub const BIT_TX_CLOSING: c_int = 1;
    pub tx_bitmask: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tls_strparser {
    pub sk: *mut sock,
// Bitfield word and msg_ready are serialized by the lower
// socket lock; BH and worker contexts both acquire it.
//
    pub 8: u32 mark :,
    pub 1: u32 stopped :,
    pub 1: u32 copy_mode :,
    pub 1: u32 mixed_decrypted :,
    pub 1: u32 msg_announced :,
    pub msg_ready: bool,
    pub stm: strp_msg,
    pub anchor: *mut sk_buff,
    pub work: work_struct,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tls_sw_context_rx {
    pub aead_recv: *mut crypto_aead,
    pub async_wait: crypto_wait,
    pub /: *mut *mut sk_buff_head rx_list; / list of decrypted 'data' records,
    pub sk): *mut *mut void (saved_data_ready)(struct sock,
    pub reader_present: u8,
    pub async_capable:1: u8,
    pub zc_capable:1: u8,
    pub reader_contended:1: u8,
    pub key_update_pending: bool,
    pub strp: tls_strparser,
    pub decrypt_pending: core::sync::atomic::AtomicI32,
    pub async_hold: sk_buff_head,
    pub wq: wait_queue_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tls_record_info {
    pub list: list_head,
    pub end_seq: u32,
    pub len: c_int,
    pub num_frags: c_int,
    pub frags: [skb_frag_t; MAX_SKB_FRAGS],
}

pub const TLS_DRIVER_STATE_SIZE_TX: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tls_offload_context_tx {
    pub aead_send: *mut crypto_aead,
    pub /: *mut *mut spinlock_t lock; / protects records list,
    pub records_list: list_head,
    pub open_record: *mut tls_record_info,
    pub retransmit_hint: *mut tls_record_info,
    pub hint_record_sn: u64,
    pub unacked_record_sn: u64,
    pub sg_tx_data: [scatterlist; MAX_SKB_FRAGS],
    pub sk): *mut *mut void (sk_destruct)(struct sock,
    pub destruct_work: work_struct,
    pub ctx: *mut tls_context,
// The TLS layer reserves room for driver specific state
// Currently the belief is that there is not enough
// driver specific state to justify another layer of indirection
//
    pub __aligned(8): u8 driver_state[TLS_DRIVER_STATE_SIZE_TX],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tls_context_flags {
// tls_device_down was called after the netdev went down, device state
// was released, and kTLS works in software, even though rx_conf is
// still TLS_HW (needed for transition).
//
    TLS_RX_DEV_DEGRADED = 0,
// Unlike RX where resync is driven entirely by the core in TX only
// the driver knows when things went out of sync, so we need the flag
// to be atomic.
//
    TLS_TX_SYNC_SCHED = 1,
// tls_dev_del was called for the RX side, device state was released,
// but tls_ctx->netdev might still be kept, because TX-side driver
// resources might not be released yet. Used to prevent the second
// tls_dev_del call in tls_device_down if it happens simultaneously.
//
    TLS_RX_DEV_CLOSED = 2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cipher_context {
    pub TLS_MAX_SALT_SIZE]: char iv[TLS_MAX_IV_SIZE +,
    pub rec_seq: [c_char; TLS_MAX_REC_SEQ_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union tls_crypto_context {
    pub info: tls_crypto_info,
    pub aes_gcm_128: tls12_crypto_info_aes_gcm_128,
    pub aes_gcm_256: tls12_crypto_info_aes_gcm_256,
    pub chacha20_poly1305: tls12_crypto_info_chacha20_poly1305,
    pub sm4_gcm: tls12_crypto_info_sm4_gcm,
    pub sm4_ccm: tls12_crypto_info_sm4_ccm,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tls_prot_info {
    pub version: u16,
    pub cipher_type: u16,
    pub prepend_size: u16,
    pub tag_size: u16,
    pub overhead_size: u16,
    pub iv_size: u16,
    pub salt_size: u16,
    pub rec_seq_size: u16,
    pub aad_size: u16,
    pub tail_size: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tls_context {
// read-only cache line
    pub prot_info: tls_prot_info,
    pub tx_conf:3: u8,
    pub rx_conf:3: u8,
    pub zerocopy_sendfile:1: u8,
    pub rx_no_pad:1: u8,
    pub tx_max_payload_len: u16,
    pub flags): *mut *mut *mut int (push_pending_record)(struct sock sk, int,
    pub sk): *mut *mut void (sk_write_space)(struct sock,
    pub priv_ctx_tx: *mut c_void,
    pub priv_ctx_rx: *mut c_void,
    pub netdev: *mut net_device __rcu,
// rw cache line
    pub tx: cipher_context,
    pub rx: cipher_context,
    pub partially_sent_record: *mut scatterlist,
    pub partially_sent_offset: u16,
    pub splicing_pages: bool,
    pub pending_open_record_frags: bool,
    pub and: *mut *mut *mut mutex tx_lock; / protects partially_sent_ fields,
// per-type TX fields
//
    pub flags: c_ulong,
// cache cold stuff
    pub sk_proto: *mut proto,
    pub sk: *mut sock,
    pub sk): *mut *mut void (sk_destruct)(struct sock,
    pub crypto_send: tls_crypto_context,
    pub crypto_recv: tls_crypto_context,
    pub list: list_head,
    pub refcount: refcount_t,
    pub rcu: rcu_head,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tls_offload_ctx_dir {
    TLS_OFFLOAD_CTX_DIR_RX,
    TLS_OFFLOAD_CTX_DIR_TX,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tlsdev_ops {
    pub start_offload_tcp_sn): u32,
    pub direction): tls_offload_ctx_dir,
    pub direction): tls_offload_ctx_dir,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tls_offload_sync_type {
    TLS_OFFLOAD_SYNC_TYPE_DRIVER_REQ = 0,
    TLS_OFFLOAD_SYNC_TYPE_CORE_NEXT_HINT = 1,
    TLS_OFFLOAD_SYNC_TYPE_DRIVER_REQ_ASYNC = 2,
}

pub const TLS_DEVICE_RESYNC_NH_START_IVAL: c_int = 2;
pub const TLS_DEVICE_RESYNC_NH_MAX_IVAL: c_int = 128;
pub const TLS_DEVICE_RESYNC_ASYNC_LOGMAX: c_int = 13;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tls_offload_resync_async {
    pub req: core::sync::atomic::AtomicI64,
    pub loglen: u16,
    pub rcd_delta: u16,
    pub log: [u32; TLS_DEVICE_RESYNC_ASYNC_LOGMAX],
}

pub const TLS_DRIVER_STATE_SIZE_RX: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tls_offload_context_rx {
// sw must be the first member of tls_offload_context_rx
    pub sw: tls_sw_context_rx,
    pub resync_type: tls_offload_sync_type,
// this member is set regardless of resync_type, to avoid branches
    pub resync_nh_reset:1: u8,
// CORE_NEXT_HINT-only member, but use the hole here
    pub resync_nh_do_now:1: u8,
// TLS_OFFLOAD_SYNC_TYPE_DRIVER_REQ
    pub resync_req: core::sync::atomic::AtomicI64,
}

// TLS_OFFLOAD_SYNC_TYPE_CORE_NEXT_HINT
// TLS_OFFLOAD_SYNC_TYPE_DRIVER_REQ_ASYNC
// The TLS layer reserves room for driver specific state
// Currently the belief is that there is not enough
// driver specific state to justify another layer of indirection
//

// Use RCU on icsk_ulp_data only for sock diag code,
// TLS data path doesn't need rcu_dereference().
//
extern "C" {
    pub fn __tls_driver_ctx(_arg: tls_get_ctx(sk), _arg: direction) -> return;
}

// The TLS context is valid until sk_destruct is called
// Log all TLS record header TCP sequences in [seq, seq+len]
// Driver's seq tracking has to be disabled until resync succeeded

extern "C" {
    pub fn tls_device_sk_destruct(sk: *mut sock);
}
extern "C" {
    pub fn tls_offload_tx_resync_request(sk: *mut sock, got_seq: u32, exp_seq: u32);
}

