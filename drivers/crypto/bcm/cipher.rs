//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/bcm/cipher.h
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
//
// Copyright 2016 Broadcom
//

// Driver supports up to MAX_SPUS SPU blocks
pub const MAX_SPUS: c_int = 16;
pub const ARC4_STATE_SIZE: c_int = 4;
pub const CCM_AES_IV_SIZE: c_int = 16;
pub const CCM_ESP_IV_SIZE: c_int = 8;
pub const RFC4543_ICV_SIZE: c_int = 16;

pub const MAX_ASSOC_SIZE: c_int = 512;
// size of salt value for AES-GCM-ESP and AES-CCM-ESP
pub const GCM_ESP_SALT_SIZE: c_int = 4;
pub const CCM_ESP_SALT_SIZE: c_int = 3;

pub const GCM_ESP_SALT_OFFSET: c_int = 0;
pub const CCM_ESP_SALT_OFFSET: c_int = 1;
pub const GCM_ESP_DIGESTSIZE: c_int = 16;

//
// Maximum number of bytes from a non-final hash request that can be deferred
// until more data is available. With new crypto API framework, this
// can be no more than one block of data.
//

// Force at least 4-byte alignment of all SPU message fields
pub const SPU_MSG_ALIGN: c_int = 4;
// Number of times to resend mailbox message if mb queue is full
pub const SPU_MB_RETRY_MAX: c_int = 1000;
// op_counts[] indexes
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum op_type {
    SPU_OP_CIPHER,
    SPU_OP_HASH,
    SPU_OP_HMAC,
    SPU_OP_AEAD,
    SPU_OP_NUM
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum spu_spu_type {
    SPU_TYPE_SPUM,
    SPU_TYPE_SPU2,
}

//
// SPUM_NS2 and SPUM_NSP are the SPU-M block on Northstar 2 and Northstar Plus,
// respectively.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum spu_spu_subtype {
    SPU_SUBTYPE_SPUM_NS2,
    SPU_SUBTYPE_SPUM_NSP,
    SPU_SUBTYPE_SPU2_V1,
    SPU_SUBTYPE_SPU2_V2
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct spu_type_subtype {
    pub type: spu_spu_type,
    pub subtype: spu_spu_subtype,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cipher_op {
    pub alg: spu_cipher_alg,
    pub mode: spu_cipher_mode,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct auth_op {
    pub alg: hash_alg,
    pub mode: hash_mode,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iproc_alg_s {
    pub type: u32,
    pub skcipher: skcipher_alg,
    pub hash: ahash_alg,
    pub aead: aead_alg,
    pub alg: },
    pub cipher_info: cipher_op,
    pub auth_info: auth_op,
    pub auth_first: bool,
    pub registered: bool,
}

//
// Buffers for a SPU request/reply message pair. All part of one structure to
// allow a single alloc per request.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct spu_msg_buf {
// Request message fragments
//
// SPU request message header. For SPU-M, holds MH, EMH, SCTX, BDESC,
// and BD header. For SPU2, holds FMD, OMD.
//
    pub SPU_MSG_ALIGN)]: u8 bcm_spu_req_hdr[ALIGN(SPU2_HEADER_ALLOC_LEN,,
// IV or counter. Size to include salt. Also used for XTS tweek.
    pub SPU_MSG_ALIGN)]: *mut *mut u8 iv_ctr[ALIGN(2  AES_BLOCK_SIZE,,
// Hash digest. request and response.
    pub SPU_MSG_ALIGN)]: u8 digest[ALIGN(MAX_DIGEST_SIZE,,
// SPU request message padding
    pub SPU_MSG_ALIGN)]: u8 spu_req_pad[ALIGN(SPU_PAD_LEN_MAX,,
// SPU-M request message STATUS field
    pub SPU_MSG_ALIGN)]: u8 tx_stat[ALIGN(SPU_TX_STATUS_LEN,,
// Response message fragments
// SPU response message header
    pub SPU_MSG_ALIGN)]: u8 spu_resp_hdr[ALIGN(SPU2_HEADER_ALLOC_LEN,,
// SPU response message STATUS field padding
    pub SPU_MSG_ALIGN)]: u8 rx_stat_pad[ALIGN(SPU_STAT_PAD_MAX,,
// SPU response message STATUS field
    pub SPU_MSG_ALIGN)]: u8 rx_stat[ALIGN(SPU_RX_STATUS_LEN,,
// Buffers only used for skcipher
//
// Field used for either SUPDT when RC4 is used
// -OR- tweak value when XTS/AES is used
//
    pub SPU_MSG_ALIGN)]: u8 supdt_tweak[ALIGN(SPU_SUPDT_LEN,,
    pub c: },
// Buffers only used for aead
// SPU response pad for GCM data
    pub SPU_MSG_ALIGN)]: u8 gcmpad[ALIGN(AES_BLOCK_SIZE,,
// SPU request msg padding for GCM AAD
    pub SPU_MSG_ALIGN)]: u8 req_aad_pad[ALIGN(SPU_PAD_LEN_MAX,,
// SPU response data to be discarded
    pub a: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iproc_ctx_s {
    pub ARC4_STATE_SIZE]: u8 enckey[MAX_KEY_SIZE +,
    pub enckeylen: c_uint,
    pub ARC4_STATE_SIZE]: u8 authkey[MAX_KEY_SIZE +,
    pub authkeylen: c_uint,
    pub salt: [u8; MAX_SALT_SIZE],
    pub salt_len: c_uint,
    pub salt_offset: c_uint,
    pub iv: [u8; MAX_IV_SIZE],
    pub digestsize: c_uint,
    pub alg: *mut iproc_alg_s,
    pub is_esp: bool,
    pub cipher: cipher_op,
    pub cipher_type: spu_cipher_type,
    pub auth: auth_op,
    pub auth_first: bool,
//
// The maximum length in bytes of the payload in a SPU message for this
// context. For SPU-M, the payload is the combination of AAD and data.
// For SPU2, the payload is just data. A value of SPU_MAX_PAYLOAD_INF
// indicates that there is no limit to the length of the SPU message
// payload.
//
    pub max_payload: c_uint,
    pub fallback_cipher: *mut crypto_aead,
// auth_type is determined during processing of request
    pub ipad: [u8; MAX_HASH_BLOCK_SIZE],
    pub opad: [u8; MAX_HASH_BLOCK_SIZE],
//
// Buffer to hold SPU message header template. Template is created at
// setkey time for skcipher requests, since most of the fields in the
// header are known at that time. At request time, just fill in a few
// missing pieces related to length of data in the request and IVs, etc.
//
    pub SPU_MSG_ALIGN)]: u8 bcm_spu_req_hdr[ALIGN(SPU2_HEADER_ALLOC_LEN,,
// Length of SPU request header
    pub spu_req_hdr_len: u16,
// Expected length of SPU response header
    pub spu_resp_hdr_len: u16,
//
// shash descriptor - needed to perform incremental hashing in
// software, when hw doesn't support it.
//
    pub shash: *mut shash_desc,
    pub /: *mut *mut bool is_rfc4543; / RFC 4543 style of GMAC,
}

// state from iproc_reqctx_s necessary for hash state export/import
#[repr(C)]
#[derive(Copy, Clone)]
pub struct spu_hash_export_s {
    pub total_todo: c_uint,
    pub total_sent: c_uint,
    pub hash_carry: [u8; HASH_CARRY_MAX],
    pub hash_carry_len: c_uint,
    pub incr_hash: [u8; MAX_DIGEST_SIZE],
    pub is_sw_hmac: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iproc_reqctx_s {
// general context
    pub parent: *mut crypto_async_request,
// only valid after enqueue()
    pub ctx: *mut iproc_ctx_s,
    pub /: *mut *mut u8 chan_idx; / Mailbox channel to be used to submit this request,
// total todo, rx'd, and sent for this request
    pub total_todo: c_uint,
    pub /: *mut *mut unsigned int total_received; / only valid for skcipher,
    pub total_sent: c_uint,
//
// num bytes sent to hw from the src sg in this request. This can differ
// from total_sent for incremental hashing. total_sent includes previous
// init() and update() data. src_sent does not.
//
    pub src_sent: c_uint,
//
// For AEAD requests, start of associated data. This will typically
// point to the beginning of the src scatterlist from the request,
// since assoc data is at the beginning of the src scatterlist rather
// than in its own sg.
//
    pub assoc: *mut scatterlist,
//
// scatterlist entry and offset to start of data for next chunk. Crypto
// API src scatterlist for AEAD starts with AAD, if present. For first
// chunk, src_sg is sg entry at beginning of input data (after AAD).
// src_skip begins at the offset in that sg entry where data begins.
//
    pub src_sg: *mut scatterlist,
    pub /: *mut *mut int src_nents; / Number of src entries with data,
    pub /: *mut *mut u32 src_skip; / bytes of current sg entry already used,
//
// Same for destination. For AEAD, if there is AAD, output data must
// be written at offset following AAD.
//
    pub dst_sg: *mut scatterlist,
    pub /: *mut *mut int dst_nents; / Number of dst entries with data,
    pub /: *mut *mut u32 dst_skip; / bytes of current sg entry already written,
// Mailbox message used to send this request to PDC driver
    pub mb_mssg: brcm_message,
    pub /: *mut *mut bool bd_suppress; / suppress BD field in SPU response?,
// cipher context
    pub is_encrypt: bool,
//
// CBC mode: IV.  CTR mode: counter.  Else empty. Used as a DMA
// buffer for AEAD requests. So allocate as DMAable memory. If IV
// concatenated with salt, includes the salt.
//
    pub iv_ctr: *mut u8,
// Length of IV or counter, in bytes
    pub iv_ctr_len: c_uint,
//
// Hash requests can be of any size, whether initial, update, or final.
// A non-final request must be submitted to the SPU as an integral
// number of blocks. This may leave data at the end of the request
// that is not a full block. Since the request is non-final, it cannot
// be padded. So, we write the remainder to this hash_carry buffer and
// hold it until the next request arrives. The carry data is then
// submitted at the beginning of the data in the next SPU msg.
// hash_carry_len is the number of bytes currently in hash_carry. These
// fields are only used for ahash requests.
//
    pub hash_carry: [u8; HASH_CARRY_MAX],
    pub hash_carry_len: c_uint,
    pub /: *mut *mut unsigned int is_final; / is this the final for the hash op?,
//
// Digest from incremental hash is saved here to include in next hash
// operation. Cannot be stored in req->result for truncated hashes,
// since result may be sized for final digest. Cannot be saved in
// msg_buf because that gets deleted between incremental hash ops
// and is not saved as part of export().
//
    pub incr_hash: [u8; MAX_DIGEST_SIZE],
// hmac context
    pub is_sw_hmac: bool,
    pub gfp: gfp_t,
// Buffers used to build SPU request and response messages
    pub msg_buf: spu_msg_buf,
    pub req: aead_request,
}

//
// Structure encapsulates a set of function pointers specific to the type of
// SPU hardware running. These functions handling creation and parsing of
// SPU request messages and SPU response messages. Includes hardware-specific
// values read from device tree.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct spu_hw {
    pub buf_len): *mut *mut *mut void (spu_dump_msg_hdr)(u8 buf, unsigned int,
    pub blocksize): c_uint,
    pub spu_hdr): *mut *mut u32 (spu_payload_length)(u8,
    pub is_hash): bool,
    pub hash_block_size): u16,
    pub data_size): c_uint,
    pub is_encrypt): unsigned int iv_len, bool,
    pub iv_len): u16,
    pub src_sent): *mut *mut hash_type (spu_hash_type)(u32,
    pub hash_type): enum,
    pub data_size): c_uint,
    pub cipher_parms): *mut spu_cipher_parms,
    pub data_size): c_uint,
    pub status_padding): unsigned int total_sent, u32,
    pub (*spu_xts_tweak_in_payload)(void): *mut u8,
    pub (*spu_tx_status_len)(void): *mut u8,
    pub (*spu_rx_status_len)(void): *mut u8,
    pub statp): *mut *mut int (spu_status_process)(u8,
    pub is_esp): bool is_encrypt, bool,
    pub data_size): *mut *mut u32 (spu_wordalign_padlen)(u32,
// The base virtual address of the SPU hw registers
    pub reg_vbase: [*mut void __iomem; MAX_SPUS],
// Version of the SPU hardware
    pub spu_type: spu_spu_type,
// Sub-version of the SPU hardware
    pub spu_subtype: spu_spu_subtype,
// The number of SPUs on this platform
    pub num_spu: u32,
// The number of SPU channels on this platform
    pub num_chan: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcm_device_private {
    pub pdev: *mut platform_device,
    pub spu: spu_hw,
    pub /: *mut *mut atomic_t session_count; / number of streams active,
    pub /: *mut *mut atomic_t stream_count; / monotonic counter for streamID's,
// Length of BCM header. Set to 0 when hw does not expect BCM HEADER.
    pub bcm_hdr_len: u8,
// The index of the channel to use for the next crypto request
    pub next_chan: core::sync::atomic::AtomicI32,
    pub debugfs_dir: *mut dentry,
    pub debugfs_stats: *mut dentry,
// Number of request bytes processed and result bytes returned
    pub bytes_in: core::sync::atomic::AtomicI64,
    pub bytes_out: core::sync::atomic::AtomicI64,
// Number of operations of each type
    pub op_counts: [core::sync::atomic::AtomicI32; SPU_OP_NUM],
    pub cipher_cnt: [core::sync::atomic::AtomicI32; CIPHER_ALG_LAST][CIPHER_MODE_LAST],
    pub hash_cnt: [core::sync::atomic::AtomicI32; HASH_ALG_LAST],
    pub hmac_cnt: [core::sync::atomic::AtomicI32; HASH_ALG_LAST],
    pub aead_cnt: [core::sync::atomic::AtomicI32; AEAD_TYPE_LAST],
// Number of calls to setkey() for each operation type
    pub setkey_cnt: [core::sync::atomic::AtomicI32; SPU_OP_NUM],
// Number of times request was resubmitted because mb was full
    pub mb_no_spc: core::sync::atomic::AtomicI32,
// Number of mailbox send failures
    pub mb_send_fail: core::sync::atomic::AtomicI32,
// Number of ICV check failures for AEAD messages
    pub bad_icv: core::sync::atomic::AtomicI32,
    pub mcl: mbox_client,
// Array of mailbox channel pointers, one for each channel
    pub mbox: *mut mbox_chan,
}
