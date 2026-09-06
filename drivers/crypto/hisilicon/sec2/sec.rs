//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/hisilicon/sec2/sec.h
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
// Copyright (c) 2019 HiSilicon Limited.

pub const SEC_PBUF_SZ: c_int = 512;
pub const SEC_MAX_MAC_LEN: c_int = 64;
pub const SEC_IV_SIZE: c_int = 24;
pub const SEC_SGE_NR_NUM: c_int = 4;
pub const SEC_SGL_ALIGN_SIZE: c_int = 64;
// Algorithm resource per hardware SEC queue
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sec_alg_res {
    pub pbuf: *mut u8,
    pub pbuf_dma: dma_addr_t,
    pub c_ivin: *mut u8,
    pub c_ivin_dma: dma_addr_t,
    pub a_ivin: *mut u8,
    pub a_ivin_dma: dma_addr_t,
    pub out_mac: *mut u8,
    pub out_mac_dma: dma_addr_t,
    pub depth: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sec_hw_sge {
    pub buf: dma_addr_t,
    pub page_ctrl: *mut c_void,
    pub len: __le32,
    pub pad: __le32,
    pub pad0: __le32,
    pub pad1: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sec_hw_sgl {
    pub next_dma: dma_addr_t,
    pub entry_sum_in_chain: __le16,
    pub entry_sum_in_sgl: __le16,
    pub entry_length_in_sgl: __le16,
    pub pad0: __le16,
    pub pad1: [__le64; 5],
    pub next: *mut sec_hw_sgl,
    pub sge_entries: [sec_hw_sge; SEC_SGE_NR_NUM],
    pub __aligned(SEC_SGL_ALIGN_SIZE): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sec_src_dst_buf {
    pub in: sec_hw_sgl,
    pub out: sec_hw_sgl,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sec_request_buf {
    pub data_buf: sec_src_dst_buf,
    pub pbuf: [__u8; SEC_PBUF_SZ],
}

// Cipher request of SEC private
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sec_cipher_req {
    pub c_out: *mut hisi_acc_hw_sgl,
    pub c_out_dma: dma_addr_t,
    pub c_ivin: *mut u8,
    pub c_ivin_dma: dma_addr_t,
    pub sk_req: *mut skcipher_request,
    pub c_len: u32,
    pub encrypt: bool,
    pub c_ivin_buf: [__u8; SEC_IV_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sec_aead_req {
    pub out_mac: *mut u8,
    pub out_mac_dma: dma_addr_t,
    pub a_ivin: *mut u8,
    pub a_ivin_dma: dma_addr_t,
    pub aead_req: *mut aead_request,
    pub a_ivin_buf: [__u8; SEC_IV_SIZE],
    pub out_mac_buf: [__u8; SEC_MAX_MAC_LEN],
}

// SEC request of Crypto
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sec_req {
    pub sec_sqe: sec_sqe,
    pub sec_sqe3: sec_sqe3,
}

//
// Common parameter of the SEC request.
//
// struct sec_req_op - Operations for SEC request
// @buf_map: DMA map the SGL buffers of the request
// @buf_unmap: DMA unmap the SGL buffers of the request
// @bd_fill: Fill the SEC queue BD
// @bd_send: Send the SEC BD into the hardware queue
// @callback: Call back for the request
// @process: Main processing logic of Skcipher
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sec_req_op {
    pub req): *mut *mut *mut int (buf_map)(struct sec_ctx ctx, struct sec_req,
    pub req): *mut *mut *mut void (buf_unmap)(struct sec_ctx ctx, struct sec_req,
    pub req): *mut *mut *mut void (do_transfer)(struct sec_ctx ctx, struct sec_req,
    pub req): *mut *mut *mut int (bd_fill)(struct sec_ctx ctx, struct sec_req,
    pub req): *mut *mut *mut int (bd_send)(struct sec_ctx ctx, struct sec_req,
    pub err): *mut *mut *mut *mut void (callback)(struct sec_ctx ctx, struct sec_req req, int,
    pub req): *mut *mut *mut int (process)(struct sec_ctx ctx, struct sec_req,
}

// SEC auth context
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sec_auth_ctx {
    pub a_key_dma: dma_addr_t,
    pub a_key: *mut u8,
    pub a_key_len: u8,
    pub a_alg: u8,
    pub hash_tfm: *mut crypto_shash,
    pub fallback_aead_tfm: *mut crypto_aead,
}

// SEC cipher context which cipher's relatives
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sec_cipher_ctx {
    pub c_key: *mut u8,
    pub c_key_dma: dma_addr_t,
    pub iv_offset: sector_t,
    pub c_gran_size: u32,
    pub ivsize: u32,
    pub c_mode: u8,
    pub c_alg: u8,
    pub c_key_len: u8,
// add software support
    pub fallback: bool,
    pub fbtfm: *mut crypto_sync_skcipher,
}

// SEC queue context which defines queue's relatives
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sec_qp_ctx {
    pub qp: *mut hisi_qp,
    pub req_list: *mut sec_req,
    pub req_idr: idr,
    pub res: *mut sec_alg_res,
    pub ctx: *mut sec_ctx,
    pub req_lock: spinlock_t,
    pub id_lock: spinlock_t,
    pub c_in_pool: *mut hisi_acc_sgl_pool,
    pub c_out_pool: *mut hisi_acc_sgl_pool,
    pub send_head: u16,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sec_alg_type {
    SEC_SKCIPHER,
    SEC_AEAD
}

// SEC Crypto TFM context which defines queue and cipher .etc relatives
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sec_ctx {
    pub qp_ctx: *mut sec_qp_ctx,
    pub sec: *mut sec_dev,
    pub req_op: *const sec_req_op,
    pub qps: *mut hisi_qp,
// Current cyclic index to select a queue for encipher
    pub enc_qcyclic: core::sync::atomic::AtomicI32,
// Current cyclic index to select a queue for decipher
    pub dec_qcyclic: core::sync::atomic::AtomicI32,
    pub alg_type: sec_alg_type,
    pub pbuf_supported: bool,
    pub c_ctx: sec_cipher_ctx,
    pub a_ctx: sec_auth_ctx,
    pub type_supported: u8,
    pub dev: *mut device,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sec_debug_file_index {
    SEC_CLEAR_ENABLE,
    SEC_DEBUG_FILE_NUM,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sec_debug_file {
    pub index: sec_debug_file_index,
    pub lock: spinlock_t,
    pub qm: *mut hisi_qm,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sec_dfx {
    pub send_cnt: core::sync::atomic::AtomicI64,
    pub recv_cnt: core::sync::atomic::AtomicI64,
    pub send_busy_cnt: core::sync::atomic::AtomicI64,
    pub recv_busy_cnt: core::sync::atomic::AtomicI64,
    pub err_bd_cnt: core::sync::atomic::AtomicI64,
    pub invalid_req_cnt: core::sync::atomic::AtomicI64,
    pub done_flag_cnt: core::sync::atomic::AtomicI64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sec_debug {
    pub dfx: sec_dfx,
    pub files: [sec_debug_file; SEC_DEBUG_FILE_NUM],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sec_dev {
    pub qm: hisi_qm,
    pub debug: sec_debug,
    pub ctx_q_num: u32,
    pub iommu_used: bool,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sec_cap_type {
    SEC_QM_NFE_MASK_CAP = 0x0,
    SEC_QM_RESET_MASK_CAP,
    SEC_QM_OOO_SHUTDOWN_MASK_CAP,
    SEC_QM_CE_MASK_CAP,
    SEC_NFE_MASK_CAP,
    SEC_RESET_MASK_CAP,
    SEC_OOO_SHUTDOWN_MASK_CAP,
    SEC_CE_MASK_CAP,
    SEC_CLUSTER_NUM_CAP,
    SEC_CORE_TYPE_NUM_CAP,
    SEC_CORE_NUM_CAP,
    SEC_CORES_PER_CLUSTER_NUM_CAP,
    SEC_CORE_ENABLE_BITMAP,
    SEC_DRV_ALG_BITMAP_LOW,
    SEC_DRV_ALG_BITMAP_HIGH,
    SEC_DEV_ALG_BITMAP_LOW,
    SEC_DEV_ALG_BITMAP_HIGH,
    SEC_CORE1_ALG_BITMAP_LOW,
    SEC_CORE1_ALG_BITMAP_HIGH,
    SEC_CORE2_ALG_BITMAP_LOW,
    SEC_CORE2_ALG_BITMAP_HIGH,
    SEC_CORE3_ALG_BITMAP_LOW,
    SEC_CORE3_ALG_BITMAP_HIGH,
    SEC_CORE4_ALG_BITMAP_LOW,
    SEC_CORE4_ALG_BITMAP_HIGH,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sec_cap_table_type {
    QM_RAS_NFE_TYPE = 0x0,
    QM_RAS_NFE_RESET,
    QM_RAS_CE_TYPE,
    SEC_RAS_NFE_TYPE,
    SEC_RAS_NFE_RESET,
    SEC_RAS_CE_TYPE,
    SEC_CORE_INFO,
    SEC_CORE_EN,
    SEC_DRV_ALG_BITMAP_LOW_TB,
    SEC_DRV_ALG_BITMAP_HIGH_TB,
    SEC_ALG_BITMAP_LOW,
    SEC_ALG_BITMAP_HIGH,
    SEC_CORE1_BITMAP_LOW,
    SEC_CORE1_BITMAP_HIGH,
    SEC_CORE2_BITMAP_LOW,
    SEC_CORE2_BITMAP_HIGH,
    SEC_CORE3_BITMAP_LOW,
    SEC_CORE3_BITMAP_HIGH,
    SEC_CORE4_BITMAP_LOW,
    SEC_CORE4_BITMAP_HIGH,
}

extern "C" {
    pub fn sec_destroy_qps(qps: *mut hisi_qp, qp_num: c_int);
}
extern "C" {
    pub fn sec_get_alg_bitmap(qm: *mut hisi_qm, high: u32, low: u32) -> u64;
}
