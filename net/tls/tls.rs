//! Automatically rewritten from C Header to Rust Module
//! Source: net/tls/tls.h
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
// Copyright (c) 2016 Tom Herbert <tom@herbertland.com>
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tls_cipher_desc {
    pub nonce: c_uint,
    pub iv: c_uint,
    pub key: c_uint,
    pub salt: c_uint,
    pub tag: c_uint,
    pub rec_seq: c_uint,
    pub iv_offset: c_uint,
    pub key_offset: c_uint,
    pub salt_offset: c_uint,
    pub rec_seq_offset: c_uint,
    pub cipher_name: *mut c_char,
    pub offloadable: bool,
    pub crypto_info: usize,
}

// TLS records are maintained in 'struct tls_rec'. It stores the memory pages
// allocated or mapped for each TLS record. After encryption, the records are
// stores in a linked list.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tls_rec {
    pub list: list_head,
    pub tx_ready: c_int,
    pub tx_flags: c_int,
    pub msg_plaintext: sk_msg,
    pub msg_encrypted: sk_msg,
// AAD | msg_plaintext.sg.data | sg_tag
    pub sg_aead_in: [scatterlist; 2],
// AAD | msg_encrypted.sg.data (data contains overhead for hdr & iv & tag)
    pub sg_aead_out: [scatterlist; 2],
    pub content_type: c_char,
    pub sg_content_type: scatterlist,
    pub sk: *mut sock,
    pub aad_space: [c_char; TLS_AAD_SPACE_SIZE],
    pub iv_data: [u8; TLS_MAX_IV_SIZE],
// Must be last --ends in a flexible-array member.
    pub aead_req: aead_request,
}

extern "C" {
    pub fn tls_proc_init(net: *mut net) -> int __net_init;
}
extern "C" {
    pub fn tls_proc_fini(net: *mut net) -> void __net_exit;
}
extern "C" {
    pub fn tls_ctx_free(sk: *mut sock, ctx: *mut tls_context);
}
extern "C" {
    pub fn update_sk_prot(sk: *mut sock, ctx: *mut tls_context);
}
extern "C" {
    pub fn wait_on_pending_writer(sk: *mut sock, timeo: *mut c_long) -> c_int;
}
extern "C" {
    pub fn tls_err_abort(sk: *mut sock, err: c_int);
}
extern "C" {
    pub fn tls_strp_abort_strp(strp: *mut tls_strparser, err: c_int);
}
extern "C" {
    pub fn tls_update_rx_zc_capable(tls_ctx: *mut tls_context);
}
extern "C" {
    pub fn tls_sw_strparser_arm(sk: *mut sock, ctx: *mut tls_context);
}
extern "C" {
    pub fn tls_sw_strparser_done(tls_ctx: *mut tls_context);
}
extern "C" {
    pub fn tls_sw_sendmsg(sk: *mut sock, msg: *mut msghdr, size: usize) -> c_int;
}
extern "C" {
    pub fn tls_sw_splice_eof(sock: *mut socket);
}
extern "C" {
    pub fn tls_sw_cancel_work_tx(tls_ctx: *mut tls_context);
}
extern "C" {
    pub fn tls_sw_release_resources_tx(sk: *mut sock);
}
extern "C" {
    pub fn tls_sw_free_ctx_tx(tls_ctx: *mut tls_context);
}
extern "C" {
    pub fn tls_sw_free_resources_rx(sk: *mut sock);
}
extern "C" {
    pub fn tls_sw_release_resources_rx(sk: *mut sock);
}
extern "C" {
    pub fn tls_sw_free_ctx_rx(tls_ctx: *mut tls_context);
}
extern "C" {
    pub fn tls_sw_sock_is_readable(sk: *mut sock) -> bool;
}
extern "C" {
    pub fn tls_device_sendmsg(sk: *mut sock, msg: *mut msghdr, size: usize) -> c_int;
}
extern "C" {
    pub fn tls_device_splice_eof(sock: *mut socket);
}
extern "C" {
    pub fn tls_tx_records(sk: *mut sock, flags: c_int) -> c_int;
}
extern "C" {
    pub fn tls_sw_write_space(sk: *mut sock, ctx: *mut tls_context);
}
extern "C" {
    pub fn tls_device_write_space(sk: *mut sock, ctx: *mut tls_context);
}
extern "C" {
    pub fn decrypt_skb(sk: *mut sock, sgout: *mut scatterlist) -> c_int;
}
extern "C" {
    pub fn tls_strp_dev_init() -> c_int;
}
extern "C" {
    pub fn tls_strp_dev_exit();
}
extern "C" {
    pub fn tls_strp_done(strp: *mut tls_strparser);
}
extern "C" {
    pub fn __tls_strp_done(strp: *mut tls_strparser);
}
extern "C" {
    pub fn tls_strp_stop(strp: *mut tls_strparser);
}
extern "C" {
    pub fn tls_strp_init(strp: *mut tls_strparser, sk: *mut sock) -> c_int;
}
extern "C" {
    pub fn tls_strp_data_ready(strp: *mut tls_strparser);
}
extern "C" {
    pub fn tls_strp_check_rcv(strp: *mut tls_strparser, announce: bool);
}
extern "C" {
    pub fn tls_strp_msg_consume(strp: *mut tls_strparser);
}
extern "C" {
    pub fn tls_rx_msg_size(strp: *mut tls_strparser, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn tls_rx_msg_maybe_announce(strp: *mut tls_strparser);
}
extern "C" {
    pub fn tls_strp_msg_load(strp: *mut tls_strparser, force_refresh: bool) -> bool;
}
extern "C" {
    pub fn tls_strp_msg_cow(ctx: *mut tls_sw_context_rx) -> c_int;
}
extern "C" {
    pub fn tls_strp_msg_hold(strp: *mut tls_strparser, dst: *mut sk_buff_head) -> c_int;
}
extern "C" {
    pub fn READ_ONCE(_arg: ctx->strp.msg_ready) -> return;
}

extern "C" {
    pub fn tls_device_init() -> c_int;
}
extern "C" {
    pub fn tls_device_cleanup();
}
extern "C" {
    pub fn tls_set_device_offload(sk: *mut sock) -> c_int;
}
extern "C" {
    pub fn tls_device_free_resources_tx(sk: *mut sock);
}
extern "C" {
    pub fn tls_set_device_offload_rx(sk: *mut sock, ctx: *mut tls_context) -> c_int;
}
extern "C" {
    pub fn tls_device_offload_cleanup_rx(sk: *mut sock);
}
extern "C" {
    pub fn tls_device_rx_resync_new_rec(sk: *mut sock, rcd_len: u32, seq: u32);
}
extern "C" {
    pub fn tls_device_decrypted(sk: *mut sock, tls_ctx: *mut tls_context) -> c_int;
}

extern "C" {
    pub fn tls_free_partial_record(sk: *mut sock, ctx: *mut tls_context);
}
// p = cpu_to_be64(rcd_sn - n);
// we cover nonce explicit here as well, so buf should be of
// size KTLS_DTLS_HEADER_SIZE + KTLS_DTLS_NONCE_EXPLICIT_SIZE
//
// Note that VERSION must be TLS_1_2 for both TLS1.2 and TLS1.3
// we can use IV for nonce explicit according to spec
