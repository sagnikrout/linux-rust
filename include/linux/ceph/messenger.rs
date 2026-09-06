//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/ceph/messenger.h
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

//
// Ceph defines these callbacks for handling connection events.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_connection_operations {
    pub ): *mut *mut *mut ceph_connection (get)(ceph_connection,
    pub ): *mut *mut void (put)(struct ceph_connection,
// handle an incoming message.
    pub m): *mut *mut *mut void (dispatch) (struct ceph_connection con, struct ceph_msg,
// authorize an outgoing connection
    pub force_new): *mut *mut int proto, int,
    pub challenge_buf_len): c_int,
    pub con): *mut *mut int (verify_authorizer_reply) (struct ceph_connection,
    pub con): *mut *mut int (invalidate_authorizer)(struct ceph_connection,
// there was some error on the socket (disconnect, whatever)
    pub con): *mut *mut void (fault) (struct ceph_connection,
// a remote host as terminated a message exchange session, and messages
// we sent (or they tried to send us) may be lost.
    pub con): *mut *mut void (peer_reset) (struct ceph_connection,
    pub skip): *mut c_int,
    pub msg): *mut *mut void (reencode_message) (struct ceph_msg,
    pub msg): *mut *mut int (sign_message) (struct ceph_msg,
    pub msg): *mut *mut int (check_message_signature) (struct ceph_msg,
// msgr2 authentication exchange
    pub authorizer_len): *mut *mut *mut void authorizer, int,
    pub authorizer_len): *mut *mut *mut void authorizer, int,
    pub con_secret_len): *mut *mut u8 con_secret, int,
    pub mode_cnt): *const *const int allowed_modes, int,
//
// sparse_read: read sparse data
// @con: connection we're reading from
// @cursor: data cursor for reading extents
// @buf: optional buffer to read into
//
// This should be called more than once, each time setting up to
// receive an extent into the current cursor position, and zeroing
// the holes between them.
//
// Returns amount of data to be read (in bytes), 0 if reading is
// complete, or -errno if there was an error.
//
// If @buf is set on a >0 return, then the data should be read into
// the provided buffer. Otherwise, it should be read into the cursor.
//
// The sparse read operation is expected to initialize the cursor
// with a length covering up to the end of the last extent.
//
    pub buf): *mut c_char,
}

// use format string %s%lld

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_messenger {
    pub /: *mut *mut ceph_entity_inst inst; / my name+address,
    pub my_enc_addr: ceph_entity_addr,
    pub stopping: core::sync::atomic::AtomicI32,
    pub net: possible_net_t,
//
// the global_seq counts connections i (attempt to) initiate
// in order to disambiguate certain connect race conditions.
//
    pub global_seq: u32,
    pub global_seq_lock: spinlock_t,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ceph_msg_data_type {
    CEPH_MSG_DATA_NONE,	/* message contains no data payload */
    CEPH_MSG_DATA_PAGES,	/* data source/destination is a page array */
    CEPH_MSG_DATA_PAGELIST,	/* data source/destination is a pagelist */

    CEPH_MSG_DATA_BIO,	/* data source/destination is a bio list */

    CEPH_MSG_DATA_BVECS,	/* data source/destination is a bio_vec array */
    CEPH_MSG_DATA_ITER,	/* data source/destination is an iov_iter */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_bio_iter {
    pub bio: *mut bio,
    pub iter: bvec_iter,
}

//
// Advance @it by @n bytes.
//

//
// Advance @it by @n bytes, executing BVEC_STEP for each bio_vec.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_bvec_iter {
    pub bvecs: *mut bio_vec,
    pub iter: bvec_iter,
}

//
// Advance @it by @n bytes.
//

//
// Advance @it by @n bytes, executing BVEC_STEP for each bio_vec.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_msg_data {
    pub type: ceph_msg_data_type,

    pub bio_pos: ceph_bio_iter,
    pub bio_length: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_msg_data_cursor {
    pub /: *mut *mut size_t total_resid; / across all data items,
    pub /: *mut *mut *mut ceph_msg_data data; / current data item,
    pub /: *mut *mut size_t resid; / bytes not yet consumed,
    pub /: *mut *mut int sr_resid; / residual sparse_read len,
    pub /: *mut *mut bool need_crc; / crc update needed,

    pub bio_iter: ceph_bio_iter,

    pub bvec_iter: bvec_iter,
    pub /: *mut *mut unsigned int page_offset; / offset in page,
    pub /: *mut *mut unsigned short page_index; / index in array,
    pub /: *mut *mut unsigned short page_count; / pages in array,
}

//
// a single message.  it contains a header (src, dest, message type, etc.),
// footer (crc values, mainly), a "front" message body, and possibly a
// data payload (stored in some number of pages).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_msg {
    pub /: *mut *mut ceph_msg_header hdr; / header,
    pub /: *mut *mut ceph_msg_footer footer; / footer,
    pub /: *mut *mut ceph_msg_footer_old old_footer; / old format footer,
}

//
// connection states
//
pub const CEPH_CON_S_CLOSED: c_int = 1;
pub const CEPH_CON_S_PREOPEN: c_int = 2;
pub const CEPH_CON_S_V1_BANNER: c_int = 3;
pub const CEPH_CON_S_V1_CONNECT_MSG: c_int = 4;
pub const CEPH_CON_S_V2_BANNER_PREFIX: c_int = 5;
pub const CEPH_CON_S_V2_BANNER_PAYLOAD: c_int = 6;
pub const CEPH_CON_S_V2_HELLO: c_int = 7;
pub const CEPH_CON_S_V2_AUTH: c_int = 8;
pub const CEPH_CON_S_V2_AUTH_SIGNATURE: c_int = 9;
pub const CEPH_CON_S_V2_SESSION_CONNECT: c_int = 10;
pub const CEPH_CON_S_V2_SESSION_RECONNECT: c_int = 11;
pub const CEPH_CON_S_OPEN: c_int = 12;
pub const CEPH_CON_S_STANDBY: c_int = 13;
//
// ceph_connection flag bits
//

// ceph connection fault delay defaults, for exponential backoff

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_connection_v1_info {
// out_kvec_cur;
    pub /: *mut *mut int out_kvec_left; / kvec's left in out_kvec,
    pub /: *mut *mut int out_skip; / skip this many bytes,
    pub /: *mut *mut int out_kvec_bytes; / total bytes left,
    pub /: *mut *mut bool out_more; / there is more data after the kvecs,
    pub out_msg_done: bool,
    pub auth: *mut ceph_auth_handshake,
    pub /: *mut *mut int auth_retry; / true if we need a newer authorizer,
// connection negotiation temps
    pub in_banner: [u8; CEPH_BANNER_MAX_LEN],
    pub actual_peer_addr: ceph_entity_addr,
    pub peer_addr_for_me: ceph_entity_addr,
    pub out_connect: ceph_msg_connect,
    pub in_reply: ceph_msg_connect_reply,
    pub /: *mut *mut int in_base_pos; / bytes read,
// sparse reads
    pub /: *mut *mut kvec in_sr_kvec; / current location to receive into,
    pub /: *mut *mut u64 in_sr_len; / amount of data in this extent,
// message in temps
    pub /: *mut *mut u8 in_tag; / protocol control byte,
    pub in_hdr: ceph_msg_header,
    pub /: *mut *mut __le64 in_temp_ack; / for reading an ack,
// message out temps
    pub out_hdr: ceph_msg_header,
    pub /: *mut *mut __le64 out_temp_ack; / for writing an ack,
    pub keepalive2: *mut *mut ceph_timespec out_temp_keepalive2; / for writing,
    pub connection: *mut *mut u32 connect_seq; / identify the most recent,
    pub /: *mut *mut u32 peer_global_seq; / peer's global seq for this connection,
}

pub const CEPH_CRC_LEN: c_int = 4;
pub const CEPH_GCM_KEY_LEN: c_int = 16;

pub const CEPH_GCM_BLOCK_LEN: c_int = 16;
pub const CEPH_GCM_TAG_LEN: c_int = 16;
pub const CEPH_PREAMBLE_LEN: c_int = 32;
pub const CEPH_PREAMBLE_INLINE_LEN: c_int = 48;

pub const CEPH_FRAME_MAX_SEGMENT_COUNT: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_frame_desc {
    pub /: *mut *mut *mut int fd_tag; / FRAME_TAG_,
    pub fd_seg_cnt: c_int,
    pub /: *mut *mut int fd_lens[CEPH_FRAME_MAX_SEGMENT_COUNT]; / logical,
    pub fd_aligns: [c_int; CEPH_FRAME_MAX_SEGMENT_COUNT],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_gcm_nonce {
    pub fixed: __le32,
    pub __packed: __le64 counter,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_connection_v2_info {
    pub in_iter: iov_iter,
    pub /: *mut *mut kvec in_kvecs[5]; / recvmsg,
    pub /: *mut *mut bio_vec in_bvec; / recvmsg (in_cursor),
    pub in_kvec_cnt: c_int,
    pub /: *mut *mut *mut int in_state; / IN_S_,
    pub out_iter: iov_iter,
    pub /: *mut *mut kvec out_kvecs[8]; / sendmsg,
    pub out_zero),: *mut *mut bio_vec out_bvec; / sendpage (out_cursor,,
    pub out_kvec_cnt: c_int,
    pub /: *mut *mut *mut int out_state; / OUT_S_,
    pub /: *mut *mut int out_zero; / # of zero bytes to send,
    pub /: *mut *mut bool out_iter_sendpage; / use sendpage if possible,
    pub in_desc: ceph_frame_desc,
    pub in_cursor: ceph_msg_data_cursor,
    pub out_cursor: ceph_msg_data_cursor,
    pub /: *mut *mut hmac_sha256_key hmac_key; / post-auth signature,
    pub hmac_key_set: bool,
    pub /: *mut *mut *mut crypto_aead gcm_tfm; / on-wire encryption,
    pub gcm_req: *mut aead_request,
    pub gcm_wait: crypto_wait,
    pub in_gcm_nonce: ceph_gcm_nonce,
    pub out_gcm_nonce: ceph_gcm_nonce,
    pub in_enc_pages: *mut page,
    pub in_enc_page_cnt: c_int,
    pub in_enc_resid: c_int,
    pub in_enc_i: c_int,
    pub out_enc_pages: *mut page,
    pub out_enc_page_cnt: c_int,
    pub out_enc_resid: c_int,
    pub out_enc_i: c_int,
    pub /: *mut *mut *mut int con_mode; / CEPH_CON_MODE_,
    pub conn_bufs: [*mut c_void; 16],
    pub conn_buf_cnt: c_int,
    pub data_len_remain: c_int,
    pub in_sign_kvecs: [kvec; 8],
    pub out_sign_kvecs: [kvec; 8],
    pub in_sign_kvec_cnt: c_int,
    pub out_sign_kvec_cnt: c_int,
    pub client_cookie: u64,
    pub server_cookie: u64,
    pub global_seq: u64,
    pub connect_seq: u64,
    pub peer_global_seq: u64,
    pub in_buf: [u8; CEPH_PREAMBLE_SECURE_LEN],
    pub out_buf: [u8; CEPH_PREAMBLE_SECURE_LEN],
    pub /: *mut *mut *mut u8 late_status; / FRAME_LATE_STATUS_,
    pub front_crc: u32,
    pub middle_crc: u32,
    pub data_crc: u32,
    pub __packed: },
    pub 1]: u8 pad[CEPH_GCM_BLOCK_LEN -,
}

//
// A single connection with another host.
//
// We maintain a queue of outgoing messages, and some session state to
// ensure that we can preserve the lossless, ordered delivery of
// messages in the case of a TCP disconnect.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_connection {
    pub private: *mut c_void,
    pub ops: *const ceph_connection_operations,
    pub msgr: *mut ceph_messenger,
    pub /: *mut *mut *mut int state; / CEPH_CON_S_,
    pub sock_state: core::sync::atomic::AtomicI32,
    pub sock: *mut socket,
    pub /: *mut *mut *mut unsigned long flags; / CEPH_CON_F_,
    pub /: *const *const *const char error_msg; / error message, if any,
    pub /: *mut *mut ceph_entity_name peer_name; / peer name,
    pub /: *mut *mut ceph_entity_addr peer_addr; / peer address,
    pub peer_features: u64,
    pub mutex: mutex,
// out queue
    pub out_queue: list_head,
    pub /: *mut *mut list_head out_sent; / sending or sent but unacked,
    pub /: *mut *mut u64 out_seq; / last message queued for send,
    pub /: *mut *mut u64 in_seq, in_seq_acked; / last message received, acked,
    pub in_msg: *mut ceph_msg,
    pub of: *mut *mut *mut ceph_msg out_msg; / sending message (== tail,
    pub bounce_page: *mut page,
    pub /: *mut *mut u32 in_front_crc, in_middle_crc, in_data_crc; / calculated crc,
    pub /: *mut *mut timespec64 last_keepalive_ack; / keepalive2 ack stamp,
    pub /: *mut *mut delayed_work work; / send|recv work,
    pub /: *mut *mut unsigned long delay; / current delay interval,
    pub v1: ceph_connection_v1_info,
    pub v2: ceph_connection_v2_info,
}

extern "C" {
    pub fn ceph_con_flag_clear(con: *mut ceph_connection, con_flag: c_ulong);
}
extern "C" {
    pub fn ceph_con_flag_set(con: *mut ceph_connection, con_flag: c_ulong);
}
extern "C" {
    pub fn ceph_con_flag_test(con: *mut ceph_connection, con_flag: c_ulong) -> bool;
}
extern "C" {
    pub fn ceph_encode_my_addr(msgr: *mut ceph_messenger);
}
extern "C" {
    pub fn ceph_tcp_connect(con: *mut ceph_connection) -> c_int;
}
extern "C" {
    pub fn ceph_con_close_socket(con: *mut ceph_connection) -> c_int;
}
extern "C" {
    pub fn ceph_con_reset_session(con: *mut ceph_connection);
}
extern "C" {
    pub fn ceph_get_global_seq(msgr: *mut ceph_messenger, gt: u32) -> u32;
}
extern "C" {
    pub fn ceph_con_discard_sent(con: *mut ceph_connection, ack_seq: u64);
}
extern "C" {
    pub fn ceph_con_discard_requeued(con: *mut ceph_connection, reconnect_seq: u64);
}
extern "C" {
    pub fn ceph_msg_data_advance(cursor: *mut ceph_msg_data_cursor, bytes: usize);
}
extern "C" {
    pub fn ceph_addr_is_blank(addr: *const ceph_entity_addr) -> bool;
}
extern "C" {
    pub fn ceph_addr_port(addr: *const ceph_entity_addr) -> c_int;
}
extern "C" {
    pub fn ceph_addr_set_port(addr: *mut ceph_entity_addr, p: c_int);
}
extern "C" {
    pub fn ceph_con_process_message(con: *mut ceph_connection);
}
// messenger_v1.c
extern "C" {
    pub fn ceph_con_v1_try_read(con: *mut ceph_connection) -> c_int;
}
extern "C" {
    pub fn ceph_con_v1_try_write(con: *mut ceph_connection) -> c_int;
}
extern "C" {
    pub fn ceph_con_v1_revoke(con: *mut ceph_connection, msg: *mut ceph_msg);
}
extern "C" {
    pub fn ceph_con_v1_revoke_incoming(con: *mut ceph_connection);
}
extern "C" {
    pub fn ceph_con_v1_opened(con: *mut ceph_connection) -> bool;
}
extern "C" {
    pub fn ceph_con_v1_reset_session(con: *mut ceph_connection);
}
extern "C" {
    pub fn ceph_con_v1_reset_protocol(con: *mut ceph_connection);
}
// messenger_v2.c
extern "C" {
    pub fn ceph_con_v2_try_read(con: *mut ceph_connection) -> c_int;
}
extern "C" {
    pub fn ceph_con_v2_try_write(con: *mut ceph_connection) -> c_int;
}
extern "C" {
    pub fn ceph_con_v2_revoke(con: *mut ceph_connection, msg: *mut ceph_msg);
}
extern "C" {
    pub fn ceph_con_v2_revoke_incoming(con: *mut ceph_connection);
}
extern "C" {
    pub fn ceph_con_v2_opened(con: *mut ceph_connection) -> bool;
}
extern "C" {
    pub fn ceph_con_v2_reset_session(con: *mut ceph_connection);
}
extern "C" {
    pub fn ceph_con_v2_reset_protocol(con: *mut ceph_connection);
}
extern "C" {
    pub fn ceph_msgr_init() -> c_int;
}
extern "C" {
    pub fn ceph_msgr_exit();
}
extern "C" {
    pub fn ceph_msgr_flush();
}
extern "C" {
    pub fn ceph_messenger_fini(msgr: *mut ceph_messenger);
}
extern "C" {
    pub fn ceph_messenger_reset_nonce(msgr: *mut ceph_messenger);
}
extern "C" {
    pub fn ceph_con_opened(con: *mut ceph_connection) -> bool;
}
extern "C" {
    pub fn ceph_con_close(con: *mut ceph_connection);
}
extern "C" {
    pub fn ceph_con_send(con: *mut ceph_connection, msg: *mut ceph_msg);
}
extern "C" {
    pub fn ceph_msg_revoke(msg: *mut ceph_msg);
}
extern "C" {
    pub fn ceph_msg_revoke_incoming(msg: *mut ceph_msg);
}
extern "C" {
    pub fn ceph_con_keepalive(con: *mut ceph_connection);
}

extern "C" {
    pub fn ceph_msg_put(msg: *mut ceph_msg);
}
extern "C" {
    pub fn ceph_msg_dump(msg: *mut ceph_msg);
}
