//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/hisilicon/sec/sec_drv.h
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
// Copyright (c) 2016-2017 HiSilicon Limited.

pub const SEC_MAX_SGE_NUM: c_int = 64;
pub const SEC_HW_RING_NUM: c_int = 3;
pub const SEC_CMD_RING: c_int = 0;
pub const SEC_OUTORDER_RING: c_int = 1;
pub const SEC_DBG_RING: c_int = 2;
// A reasonable length to balance memory use against flexibility
pub const SEC_QUEUE_LEN: c_int = 512;
pub const SEC_MAX_SGE_NUM: c_int = 64;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sec_bd_info {

pub const SEC_BD_W0_T_LEN_S: c_int = 0;

pub const SEC_BD_W0_C_WIDTH_S: c_int = 5;
pub const SEC_C_WIDTH_AES_128BIT: c_int = 0;
pub const SEC_C_WIDTH_AES_8BIT: c_int = 1;
pub const SEC_C_WIDTH_AES_1BIT: c_int = 2;
pub const SEC_C_WIDTH_DES_64BIT: c_int = 0;
pub const SEC_C_WIDTH_DES_8BIT: c_int = 1;
pub const SEC_C_WIDTH_DES_1BIT: c_int = 2;

pub const SEC_BD_W0_C_MODE_S: c_int = 7;
pub const SEC_C_MODE_ECB: c_int = 0;
pub const SEC_C_MODE_CBC: c_int = 1;
pub const SEC_C_MODE_CTR: c_int = 4;
pub const SEC_C_MODE_CCM: c_int = 5;
pub const SEC_C_MODE_GCM: c_int = 6;
pub const SEC_C_MODE_XTS: c_int = 7;

pub const SEC_BD_W0_DAT_SKIP_S: c_int = 12;

pub const SEC_BD_W0_C_GRAN_SIZE_19_16_S: c_int = 14;

pub const SEC_BD_W0_CIPHER_S: c_int = 18;
pub const SEC_CIPHER_NULL: c_int = 0;
pub const SEC_CIPHER_ENCRYPT: c_int = 1;
pub const SEC_CIPHER_DECRYPT: c_int = 2;

pub const SEC_BD_W0_AUTH_S: c_int = 20;
pub const SEC_AUTH_NULL: c_int = 0;
pub const SEC_AUTH_MAC: c_int = 1;
pub const SEC_AUTH_VERIF: c_int = 2;

pub const SEC_BD_W0_HM_S: c_int = 25;

pub const SEC_BD_W0_ICV_OR_SKEY_EN_S: c_int = 27;
// Multi purpose field - gran size bits for send, flag for recv

pub const SEC_BD_W0_FLAG_S: c_int = 29;
pub const SEC_BD_W0_C_GRAN_SIZE_21_20_S: c_int = 29;

    pub w0: u32,

pub const SEC_BD_W1_AUTH_GRAN_SIZE_S: c_int = 0;

pub const SEC_BD_W1_A_ALG_S: c_int = 25;
pub const SEC_A_ALG_SHA1: c_int = 0;
pub const SEC_A_ALG_SHA256: c_int = 1;
pub const SEC_A_ALG_MD5: c_int = 2;
pub const SEC_A_ALG_SHA224: c_int = 3;
pub const SEC_A_ALG_HMAC_SHA1: c_int = 8;
pub const SEC_A_ALG_HMAC_SHA224: c_int = 10;
pub const SEC_A_ALG_HMAC_SHA256: c_int = 11;
pub const SEC_A_ALG_HMAC_MD5: c_int = 12;
pub const SEC_A_ALG_AES_XCBC: c_int = 13;
pub const SEC_A_ALG_AES_CMAC: c_int = 14;

pub const SEC_BD_W1_C_ALG_S: c_int = 29;
pub const SEC_C_ALG_DES: c_int = 0;
pub const SEC_C_ALG_3DES: c_int = 1;
pub const SEC_C_ALG_AES: c_int = 2;
    pub w1: u32,

pub const SEC_BD_W2_C_GRAN_SIZE_15_0_S: c_int = 0;

pub const SEC_BD_W2_GRAN_NUM_S: c_int = 16;
    pub w2: u32,

pub const SEC_BD_W3_AUTH_LEN_OFFSET_S: c_int = 0;

pub const SEC_BD_W3_CIPHER_LEN_OFFSET_S: c_int = 10;

pub const SEC_BD_W3_MAC_LEN_S: c_int = 20;

pub const SEC_BD_W3_A_KEY_LEN_S: c_int = 25;

pub const SEC_BD_W3_C_KEY_LEN_S: c_int = 30;
pub const SEC_KEY_LEN_AES_128: c_int = 0;
pub const SEC_KEY_LEN_AES_192: c_int = 1;
pub const SEC_KEY_LEN_AES_256: c_int = 2;
pub const SEC_KEY_LEN_DES: c_int = 1;
pub const SEC_KEY_LEN_3DES_3_KEY: c_int = 1;
pub const SEC_KEY_LEN_3DES_2_KEY: c_int = 3;
    pub w3: u32,
// W4,5
    pub authkey_addr_lo: u32,
    pub authiv_addr_lo: u32,
}

// W6,7
// W8,9
// W10,11
// W12,13
// W14,15
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sec_mem_region {
    SEC_COMMON = 0,
    SEC_SAA,
    SEC_NUM_ADDR_REGIONS
}

pub const SEC_NAME_SIZE: c_int = 64;
pub const SEC_Q_NUM: c_int = 16;
//
// struct sec_queue_ring_cmd - store information about a SEC HW cmd ring
// @used: Local counter used to cheaply establish if the ring is empty.
// @lock: Protect against simultaneous adjusting of the read and write pointers.
// @vaddr: Virtual address for the ram pages used for the ring.
// @paddr: Physical address of the dma mapped region of ram used for the ring.
// @callback: Callback function called on a ring element completing.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sec_queue_ring_cmd {
    pub used: core::sync::atomic::AtomicI32,
    pub lock: mutex,
    pub vaddr: *mut sec_bd_info,
    pub paddr: dma_addr_t,
    pub ctx): *mut *mut *mut void (callback)(struct sec_bd_info resp, void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sec_queue_ring_db {
    pub vaddr: *mut sec_debug_bd_info,
    pub paddr: dma_addr_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sec_queue_ring_cq {
    pub vaddr: *mut sec_out_bd_info,
    pub paddr: dma_addr_t,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sec_cipher_alg {
    SEC_C_DES_ECB_64,
    SEC_C_DES_CBC_64,

    SEC_C_3DES_ECB_192_3KEY,
    SEC_C_3DES_ECB_192_2KEY,

    SEC_C_3DES_CBC_192_3KEY,
    SEC_C_3DES_CBC_192_2KEY,

    SEC_C_AES_ECB_128,
    SEC_C_AES_ECB_192,
    SEC_C_AES_ECB_256,

    SEC_C_AES_CBC_128,
    SEC_C_AES_CBC_192,
    SEC_C_AES_CBC_256,

    SEC_C_AES_CTR_128,
    SEC_C_AES_CTR_192,
    SEC_C_AES_CTR_256,

    SEC_C_AES_XTS_128,
    SEC_C_AES_XTS_256,

    SEC_C_NULL,
}

//
// struct sec_alg_tfm_ctx - hardware specific tranformation context
// @cipher_alg: Cipher algorithm enabled include encryption mode.
// @key: Key storage if required.
// @pkey: DMA address for the key storage.
// @req_template: Request template to save time on setup.
// @queue: The hardware queue associated with this tfm context.
// @lock: Protect key and pkey to ensure they are consistent
// @auth_buf: Current context buffer for auth operations.
// @backlog: The backlog queue used for cases where our buffers aren't
// large enough.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sec_alg_tfm_ctx {
    pub cipher_alg: sec_cipher_alg,
    pub key: *mut u8,
    pub pkey: dma_addr_t,
    pub req_template: sec_bd_info,
    pub queue: *mut sec_queue,
    pub lock: mutex,
    pub auth_buf: *mut u8,
    pub backlog: list_head,
}

//
// struct sec_request - data associate with a single crypto request
// @elements: List of subparts of this request (hardware size restriction)
// @num_elements: The number of subparts (used as an optimization)
// @lock: Protect elements of this structure against concurrent change.
// @tfm_ctx: hardware specific context.
// @len_in: length of in sgl from upper layers
// @len_out: length of out sgl from upper layers
// @dma_iv: initialization vector - phsyical address
// @err: store used to track errors across subelements of this request.
// @req_base: pointer to base element of associate crypto context.
// This is needed to allow shared handling skcipher, ahash etc.
// @cb: completion callback.
// @backlog_head: list head to allow backlog maintenance.
//
// The hardware is limited in the maximum size of data that it can
// process from a single BD.  Typically this is fairly large (32MB)
// but still requires the complexity of splitting the incoming
// skreq up into a number of elements complete with appropriate
// iv chaining.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sec_request {
    pub elements: list_head,
    pub num_elements: c_int,
    pub lock: mutex,
    pub tfm_ctx: *mut sec_alg_tfm_ctx,
    pub len_in: c_int,
    pub len_out: c_int,
    pub dma_iv: dma_addr_t,
    pub err: c_int,
    pub req_base: *mut crypto_async_request,
    pub req): *mut *mut *mut void (cb)(struct sec_bd_info resp, struct crypto_async_request,
    pub backlog_head: list_head,
}

//
// struct sec_request_el - A subpart of a request.
// @head: allow us to attach this to the list in the sec_request
// @req: hardware block descriptor corresponding to this request subpart
// @in: hardware sgl for input - virtual address
// @dma_in: hardware sgl for input - physical address
// @sgl_in: scatterlist for this request subpart
// @out: hardware sgl for output - virtual address
// @dma_out: hardware sgl for output - physical address
// @sgl_out: scatterlist for this request subpart
// @sec_req: The request which this subpart forms a part of
// @el_length: Number of bytes in this subpart. Needed to locate
// last ivsize chunk for iv chaining.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sec_request_el {
    pub head: list_head,
    pub req: sec_bd_info,
    pub in: *mut sec_hw_sgl,
    pub dma_in: dma_addr_t,
    pub sgl_in: *mut scatterlist,
    pub out: *mut sec_hw_sgl,
    pub dma_out: dma_addr_t,
    pub sgl_out: *mut scatterlist,
    pub sec_req: *mut sec_request,
    pub el_length: usize,
}

//
// struct sec_queue - All the information about a HW queue
// @dev_info: The parent SEC device to which this queue belongs.
// @task_irq: Completion interrupt for the queue.
// @name: Human readable queue description also used as irq name.
// @ring: The several HW rings associated with one queue.
// @regs: The iomapped device registers
// @queue_id: Index of the queue used for naming and resource selection.
// @in_use: Flag to say if the queue is in use.
// @expected: The next expected element to finish assuming we were in order.
// @uprocessed: A bitmap to track which OoO elements are done but not handled.
// @softqueue: A software queue used when chaining requirements prevent direct
// use of the hardware queues.
// @havesoftqueue: A flag to say we have a queues - as we may need one for the
// current mode.
// @queuelock: Protect the soft queue from concurrent changes to avoid some
// potential loss of data races.
// @shadow: Pointers back to the shadow copy of the hardware ring element
// need because we can't store any context reference in the bd element.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sec_queue {
    pub dev_info: *mut sec_dev_info,
    pub task_irq: c_int,
    pub name: [c_char; SEC_NAME_SIZE],
    pub ring_cmd: sec_queue_ring_cmd,
    pub ring_cq: sec_queue_ring_cq,
    pub ring_db: sec_queue_ring_db,
    pub regs: *mut void __iomem,
    pub queue_id: u32,
    pub in_use: bool,
    pub expected: c_int,
    pub SEC_QUEUE_LEN): DECLARE_BITMAP(unprocessed,,
    pub )): *mut DECLARE_KFIFO_PTR(softqueue, typeof(struct sec_request_el,
    pub havesoftqueue: bool,
    pub queuelock: spinlock_t,
    pub shadow: [*mut c_void; SEC_QUEUE_LEN],
}

//
// struct sec_hw_sge: Track each of the 64 element SEC HW SGL entries
// @buf: The IOV dma address for this entry.
// @len: Length of this IOV.
// @pad: Reserved space.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sec_hw_sge {
    pub buf: dma_addr_t,
    pub len: c_uint,
    pub pad: c_uint,
}

//
// struct sec_hw_sgl: One hardware SGL entry.
// @next_sgl: The next entry if we need to chain dma address. Null if last.
// @entry_sum_in_chain: The full count of SGEs - only matters for first SGL.
// @entry_sum_in_sgl: The number of SGEs in this SGL element.
// @flag: Unused in skciphers.
// @serial_num: Unsued in skciphers.
// @cpuid: Currently unused.
// @data_bytes_in_sgl: Count of bytes from all SGEs in this SGL.
// @next: Virtual address used to stash the next sgl - useful in completion.
// @reserved: A reserved field not currently used.
// @sge_entries: The (up to) 64 Scatter Gather Entries, representing IOVs.
// @node: Currently unused.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sec_hw_sgl {
    pub next_sgl: dma_addr_t,
    pub entry_sum_in_chain: u16,
    pub entry_sum_in_sgl: u16,
    pub flag: u32,
    pub serial_num: u64,
    pub cpuid: u32,
    pub data_bytes_in_sgl: u32,
    pub next: *mut sec_hw_sgl,
    pub reserved: u64,
    pub sge_entries: [sec_hw_sge; SEC_MAX_SGE_NUM],
    pub node: [u8; 16],
}

//
// struct sec_dev_info: The full SEC unit comprising queues and processors.
// @sec_id: Index used to track which SEC this is when more than one is present.
// @num_saas: The number of backed processors enabled.
// @regs: iomapped register regions shared by whole SEC unit.
// @dev_lock: Protects concurrent queue allocation / freeing for the SEC.
// @queues: The 16 queues that this SEC instance provides.
// @dev: Device pointer.
// @hw_sgl_pool: DMA pool used to mimise mapping for the scatter gather lists.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sec_dev_info {
    pub sec_id: c_int,
    pub num_saas: c_int,
    pub regs: [*mut void __iomem; SEC_NUM_ADDR_REGIONS],
    pub dev_lock: mutex,
    pub queues_in_use: c_int,
    pub queues: [sec_queue; SEC_Q_NUM],
    pub dev: *mut device,
    pub hw_sgl_pool: *mut dma_pool,
}

extern "C" {
    pub fn sec_queue_send(queue: *mut sec_queue, msg: *mut sec_bd_info, ctx: *mut c_void) -> c_int;
}
extern "C" {
    pub fn sec_queue_can_enqueue(queue: *mut sec_queue, num: c_int) -> bool;
}
extern "C" {
    pub fn sec_queue_stop_release(queue: *mut sec_queue) -> c_int;
}
extern "C" {
    pub fn sec_queue_empty(queue: *mut sec_queue) -> bool;
}
// Algorithm specific elements from sec_algs.c
extern "C" {
    pub fn sec_alg_callback(resp: *mut sec_bd_info, ctx: *mut c_void);
}
extern "C" {
    pub fn sec_algs_register() -> c_int;
}
extern "C" {
    pub fn sec_algs_unregister();
}
