//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/block/drbd/drbd_protocol.h
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
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drbd_packet {
// receiver (data socket)
    P_DATA		      = 0x00,
    P_DATA_REPLY	      = 0x01, /* Response to P_DATA_REQUEST */
    P_RS_DATA_REPLY	      = 0x02, /* Response to P_RS_DATA_REQUEST */
    P_BARRIER	      = 0x03,
    P_BITMAP	      = 0x04,
    P_BECOME_SYNC_TARGET  = 0x05,
    P_BECOME_SYNC_SOURCE  = 0x06,
    P_UNPLUG_REMOTE	      = 0x07, /* Used at various times to hint the peer */
    P_DATA_REQUEST	      = 0x08, /* Used to ask for a data block */
    P_RS_DATA_REQUEST     = 0x09, /* Used to ask for a data block for resync */
    P_SYNC_PARAM	      = 0x0a,
    P_PROTOCOL	      = 0x0b,
    P_UUIDS		      = 0x0c,
    P_SIZES		      = 0x0d,
    P_STATE		      = 0x0e,
    P_SYNC_UUID	      = 0x0f,
    P_AUTH_CHALLENGE      = 0x10,
    P_AUTH_RESPONSE	      = 0x11,
    P_STATE_CHG_REQ	      = 0x12,

// (meta socket)
    P_PING		      = 0x13,
    P_PING_ACK	      = 0x14,
    P_RECV_ACK	      = 0x15, /* Used in protocol B */
    P_WRITE_ACK	      = 0x16, /* Used in protocol C */
    P_RS_WRITE_ACK	      = 0x17, /* Is a P_WRITE_ACK, additionally call set_in_sync(). */
    P_SUPERSEDED	      = 0x18, /* Used in proto C, two-primaries conflict detection */
    P_NEG_ACK	      = 0x19, /* Sent if local disk is unusable */
    P_NEG_DREPLY	      = 0x1a, /* Local disk is broken... */
    P_NEG_RS_DREPLY	      = 0x1b, /* Local disk is broken... */
    P_BARRIER_ACK	      = 0x1c,
    P_STATE_CHG_REPLY     = 0x1d,

// "new" commands, no longer fitting into the ordering scheme above

    P_OV_REQUEST	      = 0x1e, /* data socket */
    P_OV_REPLY	      = 0x1f,
    P_OV_RESULT	      = 0x20, /* meta socket */
    P_CSUM_RS_REQUEST     = 0x21, /* data socket */
    P_RS_IS_IN_SYNC	      = 0x22, /* meta socket */
    P_SYNC_PARAM89	      = 0x23, /* data socket, protocol version 89 replacement for P_SYNC_PARAM */
    P_COMPRESSED_BITMAP   = 0x24, /* compressed or otherwise encoded bitmap transfer */
// P_CKPT_FENCE_REQ      = 0x25, * currently reserved for protocol D
// P_CKPT_DISABLE_REQ    = 0x26, * currently reserved for protocol D
    P_DELAY_PROBE         = 0x27, /* is used on BOTH sockets */
    P_OUT_OF_SYNC         = 0x28, /* Mark as out of sync (Outrunning), data socket */
    P_RS_CANCEL           = 0x29, /* meta: Used to cancel RS_DATA_REQUEST packet by SyncSource */
    P_CONN_ST_CHG_REQ     = 0x2a, /* data sock: Connection wide state request */
    P_CONN_ST_CHG_REPLY   = 0x2b, /* meta sock: Connection side state req reply */
    P_RETRY_WRITE	      = 0x2c, /* Protocol C: retry conflicting write request */
    P_PROTOCOL_UPDATE     = 0x2d, /* data sock: is used in established connections */
// 0x2e to 0x30 reserved, used in drbd 9

// REQ_OP_DISCARD. We used "discard" in different contexts before,
// which is why I chose TRIM here, to disambiguate.
    P_TRIM                = 0x31,

// Only use these two if both support FF_THIN_RESYNC
    P_RS_THIN_REQ         = 0x32, /* Request a block for resync or reply P_RS_DEALLOCATED */
    P_RS_DEALLOCATED      = 0x33, /* Contains only zeros on sync source node */

// REQ_WRITE_SAME.
// On a receiving side without REQ_WRITE_SAME,
// we may fall back to an opencoded loop instead.
    P_WSAME               = 0x34,

// 0x35 already claimed in DRBD 9
    P_ZEROES              = 0x36, /* data sock: zero-out, WRITE_ZEROES */

// 0x40 .. 0x48 already claimed in DRBD 9

    P_MAY_IGNORE	      = 0x100, /* Flag to test if (cmd > P_MAY_IGNORE) ... */
    P_MAX_OPT_CMD	      = 0x101,

// special command ids for handshake

    P_INITIAL_META	      = 0xfff1, /* First Packet on the MetaSock */
    P_INITIAL_DATA	      = 0xfff2, /* First Packet on the Socket */

    P_CONNECTION_FEATURES = 0xfffe	/* FIXED for the next century! */
}

// This is the layout for a packet on the wire.
// The byteorder is the network byte order.
// (except block_id and barrier fields.
// these are pointers to local structs
// and have no relevance for the partner,
// which just echoes them as received.)
//
// NOTE that the payload starts at a long aligned offset,
// regardless of 32 or 64 bit arch!
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct p_header80 {
    pub magic: u32,
    pub command: u16,
    pub /: *mut *mut u16 length; / bytes of data after this header,
    pub __packed: },
// Header for big packets, Used for data packets exceeding 64kB
#[repr(C)]
#[derive(Copy, Clone)]
pub struct p_header95 {
    pub /: *mut *mut u16 magic; / use DRBD_MAGIC_BIG here,
    pub command: u16,
    pub length: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct p_header100 {
    pub magic: u32,
    pub volume: u16,
    pub command: u16,
    pub length: u32,
    pub pad: u32,
    pub __packed: },
// These defines must not be changed without changing the protocol version.
// New defines may only be introduced together with protocol version bump or
// new protocol feature flags.
//

pub const DP_MAY_SET_IN_SYNC: c_int = 4;

// possible combinations:
// REQ_OP_WRITE_ZEROES:  DP_DISCARD | DP_ZEROES
// REQ_OP_WRITE_ZEROES + REQ_NOUNMAP: DP_ZEROES
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct p_data {
    pub /: *mut *mut u64 sector; / 64 bits sector number,
    pub /: *mut *mut u64 block_id; / to identify the request in protocol B&C,
    pub seq_num: u32,
    pub dp_flags: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct p_trim {
    pub p_data: p_data,
    pub /: *mut *mut u32 size; / == bio->bi_size,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct p_wsame {
    pub p_data: p_data,
    pub /: *mut *mut u32 size; / == bio->bi_size,
    pub __packed: },
//
// commands which share a struct:
// p_block_ack:
// P_RECV_ACK (proto B), P_WRITE_ACK (proto C),
// P_SUPERSEDED (proto C, two-primaries conflict detection)
// p_block_req:
// P_DATA_REQUEST, P_RS_DATA_REQUEST
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct p_block_ack {
    pub sector: u64,
    pub block_id: u64,
    pub blksize: u32,
    pub seq_num: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct p_block_req {
    pub sector: u64,
    pub block_id: u64,
    pub blksize: u32,
    pub /: *mut *mut u32 pad; / to multiple of 8 Byte,
    pub __packed: },
//
// commands with their own struct for additional fields:
// P_CONNECTION_FEATURES
// P_BARRIER
// P_BARRIER_ACK
// P_SYNC_PARAM
// ReportParams
//
// supports TRIM/DISCARD on the "wire" protocol
pub const DRBD_FF_TRIM: c_int = 1;
// Detect all-zeros during resync, and rather TRIM/UNMAP/DISCARD those blocks
// instead of fully allocate a supposedly thin volume on initial resync
pub const DRBD_FF_THIN_RESYNC: c_int = 2;
// supports REQ_WRITE_SAME on the "wire" protocol.
// Note: this flag is overloaded,
// its presence also
// - indicates support for 128 MiB "batch bios",
// max discard size of 128 MiB
// instead of 4M before that.
// - indicates that we exchange additional settings in p_sizes
// drbd_send_sizes()/receive_sizes()
//
pub const DRBD_FF_WSAME: c_int = 4;
// supports REQ_OP_WRITE_ZEROES on the "wire" protocol.
//
// We used to map that to "discard" on the sending side, and if we cannot
// guarantee that discard zeroes data, the receiving side would map discard
// back to zero-out.
//
// With the introduction of REQ_OP_WRITE_ZEROES,
// we started to use that for both WRITE_ZEROES and DISCARDS,
// hoping that WRITE_ZEROES would "do what we want",
// UNMAP if possible, zero-out the rest.
//
// The example scenario is some LVM "thin" backend.
//
// While an un-allocated block on dm-thin reads as zeroes, on a dm-thin
// with "skip_block_zeroing=true", after a partial block write allocated
// that block, that same block may well map "undefined old garbage" from
// the backends on LBAs that have not yet been written to.
//
// If we cannot distinguish between zero-out and discard on the receiving
// side, to avoid "undefined old garbage" to pop up randomly at later times
// on supposedly zero-initialized blocks, we'd need to map all discards to
// zero-out on the receiving side.  But that would potentially do a full
// alloc on thinly provisioned backends, even when the expectation was to
// unmap/trim/discard/de-allocate.
//
// We need to distinguish on the protocol level, whether we need to guarantee
// zeroes (and thus use zero-out, potentially doing the mentioned full-alloc),
// or if we want to put the emphasis on discard, and only do a "best effort
// zeroing" (by "discarding" blocks aligned to discard-granularity, and zeroing
// only potential unaligned head and tail clippings), to at least *try* to
// avoid "false positives" in an online-verify later, hoping that someone
// set skip_block_zeroing=false.
//
pub const DRBD_FF_WZEROES: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct p_connection_features {
    pub protocol_min: u32,
    pub feature_flags: u32,
    pub protocol_max: u32,
// should be more than enough for future enhancements
// for now, feature_flags and the reserved array shall be zero.
//
    pub _pad: u32,
    pub reserved: [u64; 7],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct p_barrier {
    pub /: *mut *mut u32 barrier; / barrier number _handle_ only,
    pub /: *mut *mut u32 pad; / to multiple of 8 Byte,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct p_barrier_ack {
    pub barrier: u32,
    pub set_size: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct p_rs_param {
    pub resync_rate: u32,
// Since protocol version 88 and higher.
    pub verify_alg: [c_char; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct p_rs_param_89 {
    pub resync_rate: u32,
// protocol version 89:
    pub verify_alg: [c_char; SHARED_SECRET_MAX],
    pub csums_alg: [c_char; SHARED_SECRET_MAX],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct p_rs_param_95 {
    pub resync_rate: u32,
    pub verify_alg: [c_char; SHARED_SECRET_MAX],
    pub csums_alg: [c_char; SHARED_SECRET_MAX],
    pub c_plan_ahead: u32,
    pub c_delay_target: u32,
    pub c_fill_target: u32,
    pub c_max_rate: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drbd_conn_flags {
    CF_DISCARD_MY_DATA = 1,
    CF_DRY_RUN = 2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct p_protocol {
    pub protocol: u32,
    pub after_sb_0p: u32,
    pub after_sb_1p: u32,
    pub after_sb_2p: u32,
    pub conn_flags: u32,
    pub two_primaries: u32,
// Since protocol version 87 and higher.
    pub integrity_alg: [c_char; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct p_uuids {
    pub uuid: [u64; UI_EXTENDED_SIZE],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct p_rs_uuid {
    pub uuid: u64,
    pub __packed: },
// optional queue_limits if (agreed_features & DRBD_FF_WSAME)
// see also struct queue_limits, as of late 2015
#[repr(C)]
#[derive(Copy, Clone)]
pub struct o_qlim {
// we don't need it yet, but we may as well communicate it now
    pub physical_block_size: u32,
// so the original in struct queue_limits is unsigned short,
// but I'd have to put in padding anyways.
    pub logical_block_size: u32,
// One incoming bio becomes one DRBD request,
// which may be translated to several bio on the receiving side.
// We don't need to communicate chunk/boundary/segment ... limits.
//
// various IO hints may be useful with "diskless client" setups
    pub alignment_offset: u32,
    pub io_min: u32,
    pub io_opt: u32,
// We may need to communicate integrity stuff at some point,
// but let's not get ahead of ourselves.
// Backend discard capabilities.
// Receiving side uses "blkdev_issue_discard()", no need to communicate
// more specifics.  If the backend cannot do discards, the DRBD peer
// may fall back to blkdev_issue_zeroout().
//
    pub discard_enabled: u8,
    pub discard_zeroes_data: u8,
    pub write_same_capable: u8,
    pub _pad: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct p_sizes {
    pub /: *mut *mut u64 d_size; / size of disk,
    pub /: *mut *mut u64 u_size; / user requested size,
    pub /: *mut *mut u64 c_size; / current exported size,
    pub /: *mut *mut u32 max_bio_size; / Maximal size of a BIO,
    pub DRBD*/: *mut *mut u16 queue_order_type; / not yet implemented in,
    pub /: *mut *mut u16 dds_flags; / use enum dds_flags here.,
// optional queue_limits if (agreed_features & DRBD_FF_WSAME)
    pub qlim: [o_qlim; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct p_state {
    pub state: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct p_req_state {
    pub mask: u32,
    pub val: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct p_req_state_reply {
    pub retcode: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct p_drbd06_param {
    pub size: u64,
    pub state: u32,
    pub blksize: u32,
    pub protocol: u32,
    pub version: u32,
    pub gen_cnt: [u32; 5],
    pub bit_map_gen: [u32; 5],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct p_block_desc {
    pub sector: u64,
    pub blksize: u32,
    pub /: *mut *mut u32 pad; / to multiple of 8 Byte,
    pub __packed: },
// Valid values for the encoding field.
// Bump proto version when changing this.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drbd_bitmap_code {
// RLE_VLI_Bytes = 0,
// and other bit variants had been defined during
// algorithm evaluation.
    RLE_VLI_Bits = 2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct p_compressed_bm {
// (encoding & 0x0f): actual encoding, see enum drbd_bitmap_code
// (encoding & 0x80): polarity (set/unset) of first runlength
// ((encoding >> 4) & 0x07): pad_bits, number of trailing zero bits
// used to pad up to head.length bytes
//
    pub encoding: u8,
    pub code: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct p_delay_probe93 {
    pub /: *mut *mut u32 seq_num; / sequence number to match the two probe packets,
    pub /: *mut *mut u32 offset; / usecs the probe got sent after the reference time point,
    pub __packed: },
//
// Bitmap packets need to fit within a single page on the sender and receiver,
// so we are limited to 4 KiB (and not to PAGE_SIZE, which can be bigger).
//
pub const DRBD_SOCKET_BUFFER_SIZE: c_int = 4096;
