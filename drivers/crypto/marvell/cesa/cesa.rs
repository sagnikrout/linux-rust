//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/marvell/cesa/cesa.h
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

pub const CESA_TDMA_BYTE_CNT: c_uint = 0x800;
pub const CESA_TDMA_SRC_ADDR: c_uint = 0x810;
pub const CESA_TDMA_DST_ADDR: c_uint = 0x820;
pub const CESA_TDMA_NEXT_ADDR: c_uint = 0x830;
pub const CESA_TDMA_CONTROL: c_uint = 0x840;

pub const CESA_TDMA_DST_BURST_32B: c_int = 3;
pub const CESA_TDMA_DST_BURST_128B: c_int = 4;

pub const CESA_TDMA_CUR: c_uint = 0x870;
pub const CESA_TDMA_ERROR_CAUSE: c_uint = 0x8c8;
pub const CESA_TDMA_ERROR_MSK: c_uint = 0x8cc;

pub const CESA_SA_CMD: c_uint = 0xde00;

pub const CESA_SA_DESC_P0: c_uint = 0xde04;
pub const CESA_SA_DESC_P1: c_uint = 0xde14;
pub const CESA_SA_CFG: c_uint = 0xde08;

pub const CESA_SA_CFG_DIG_ERR_CONT: c_int = 0;
pub const CESA_SA_CFG_DIG_ERR_SKIP: c_int = 1;
pub const CESA_SA_CFG_DIG_ERR_STOP: c_int = 3;

pub const CESA_SA_ACCEL_STATUS: c_uint = 0xde0c;

//
// CESA_SA_FPGA_INT_STATUS looks like an FPGA leftover and is documented only
// in Errata 4.12. It looks like that it was part of an IRQ-controller in FPGA
// and someone forgot to remove  it while switching to the core and moving to
// CESA_SA_INT_STATUS.
//
pub const CESA_SA_FPGA_INT_STATUS: c_uint = 0xdd68;
pub const CESA_SA_INT_STATUS: c_uint = 0xde20;

pub const CESA_SA_INT_MSK: c_uint = 0xde24;
pub const CESA_SA_DESC_CFG_OP_MAC_ONLY: c_int = 0;
pub const CESA_SA_DESC_CFG_OP_CRYPT_ONLY: c_int = 1;
pub const CESA_SA_DESC_CFG_OP_MAC_CRYPT: c_int = 2;
pub const CESA_SA_DESC_CFG_OP_CRYPT_MAC: c_int = 3;

//
// /-----------\ 0
// | ACCEL CFG |	4 * 8
// |-----------| 0x20
// | CRYPT KEY |	8 * 4
// |-----------| 0x40
// |  IV   IN  |	4 * 4
// |-----------| 0x40 (inplace)
// |  IV BUF   |	4 * 4
// |-----------| 0x80
// |  DATA IN  |	16 * x (max ->max_req_size)
// |-----------| 0x80 (inplace operation)
// |  DATA OUT |	16 * x (max ->max_req_size)
// \-----------/ SRAM size
//
// Hashing memory map:
// /-----------\ 0
// | ACCEL CFG |        4 * 8
// |-----------| 0x20
// | Inner IV  |        8 * 4
// |-----------| 0x40
// | Outer IV  |        8 * 4
// |-----------| 0x60
// | Output BUF|        8 * 4
// |-----------| 0x80
// |  DATA IN  |        64 * x (max ->max_req_size)
// \-----------/ SRAM size
//
pub const CESA_SA_CFG_SRAM_OFFSET: c_uint = 0x00;
pub const CESA_SA_DATA_SRAM_OFFSET: c_uint = 0x80;
pub const CESA_SA_CRYPT_KEY_SRAM_OFFSET: c_uint = 0x20;
pub const CESA_SA_CRYPT_IV_SRAM_OFFSET: c_uint = 0x40;
pub const CESA_SA_MAC_IIV_SRAM_OFFSET: c_uint = 0x20;
pub const CESA_SA_MAC_OIV_SRAM_OFFSET: c_uint = 0x40;
pub const CESA_SA_MAC_DIG_SRAM_OFFSET: c_uint = 0x60;

pub const CESA_SA_DESC_MAC_SRC_TOTAL_LEN_MAX: c_uint = 0xffff;

pub const CESA_SA_SRAM_SIZE: c_int = 2048;

pub const CESA_SA_DEFAULT_SRAM_SIZE: c_int = 2048;
pub const CESA_SA_MIN_SRAM_SIZE: c_int = 1024;

pub const CESA_MAX_HASH_BLOCK_SIZE: c_int = 64;

//
// struct mv_cesa_sec_accel_desc - security accelerator descriptor
// @config:	engine config
// @enc_p:	input and output data pointers for a cipher operation
// @enc_len:	cipher operation length
// @enc_key_p:	cipher key pointer
// @enc_iv:	cipher IV pointers
// @mac_src_p:	input pointer and total hash length
// @mac_digest:	digest pointer and hash operation length
// @mac_iv:	hmac IV pointers
//
// Structure passed to the CESA engine to describe the crypto operation
// to be executed.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mv_cesa_sec_accel_desc {
    pub config: __le32,
    pub enc_p: __le32,
    pub enc_len: __le32,
    pub enc_key_p: __le32,
    pub enc_iv: __le32,
    pub mac_src_p: __le32,
    pub mac_digest: __le32,
    pub mac_iv: __le32,
}

//
// struct mv_cesa_skcipher_op_ctx - cipher operation context
// @key:	cipher key
// @iv:		cipher IV
//
// Context associated to a cipher operation.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mv_cesa_skcipher_op_ctx {
    pub key: [__le32; 8],
    pub iv: [u32; 4],
}

//
// struct mv_cesa_hash_op_ctx - hash or hmac operation context
// @key:	cipher key
// @iv:		cipher IV
//
// Context associated to an hash or hmac operation.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mv_cesa_hash_op_ctx {
    pub iv: [u32; 16],
    pub hash: [__le32; 8],
}

//
// struct mv_cesa_op_ctx - crypto operation context
// @desc:	CESA descriptor
// @ctx:	context associated to the crypto operation
//
// Context associated to a crypto operation.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mv_cesa_op_ctx {
    pub desc: mv_cesa_sec_accel_desc,
    pub skcipher: mv_cesa_skcipher_op_ctx,
    pub hash: mv_cesa_hash_op_ctx,
    pub ctx: },
}

// TDMA descriptor flags

pub const CESA_TDMA_DUMMY: c_int = 0;
pub const CESA_TDMA_DATA: c_int = 1;
pub const CESA_TDMA_OP: c_int = 2;
pub const CESA_TDMA_RESULT: c_int = 3;
//
// struct mv_cesa_tdma_desc - TDMA descriptor
// @byte_cnt:	number of bytes to transfer
// @src:	DMA address of the source
// @dst:	DMA address of the destination
// @next_dma:	DMA address of the next TDMA descriptor
// @cur_dma:	DMA address of this TDMA descriptor
// @next:	pointer to the next TDMA descriptor
// @op:		CESA operation attached to this TDMA descriptor
// @data:	raw data attached to this TDMA descriptor
// @flags:	flags describing the TDMA transfer. See the
// "TDMA descriptor flags" section above
//
// TDMA descriptor used to create a transfer chain describing a crypto
// operation.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mv_cesa_tdma_desc {
    pub byte_cnt: __le32,
    pub src: __le32,
    pub src_dma: u32,
}

// Software state
//
// struct mv_cesa_sg_dma_iter - scatter-gather iterator
// @dir:	transfer direction
// @sg:		scatter list
// @offset:	current position in the scatter list
// @op_offset:	current position in the crypto operation
//
// Iterator used to iterate over a scatterlist while creating a TDMA chain for
// a crypto operation.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mv_cesa_sg_dma_iter {
    pub dir: dma_data_direction,
    pub sg: *mut scatterlist,
    pub offset: c_uint,
    pub op_offset: c_uint,
}

//
// struct mv_cesa_dma_iter - crypto operation iterator
// @len:	the crypto operation length
// @offset:	current position in the crypto operation
// @op_len:	sub-operation length (the crypto engine can only act on 2kb
// chunks)
//
// Iterator used to create a TDMA chain for a given crypto operation.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mv_cesa_dma_iter {
    pub len: c_uint,
    pub offset: c_uint,
    pub op_len: c_uint,
}

//
// struct mv_cesa_tdma_chain - TDMA chain
// @first:	first entry in the TDMA chain
// @last:	last entry in the TDMA chain
//
// Stores a TDMA chain for a specific crypto operation.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mv_cesa_tdma_chain {
    pub first: *mut mv_cesa_tdma_desc,
    pub last: *mut mv_cesa_tdma_desc,
}

//
// struct mv_cesa_caps - CESA device capabilities
// @engines:		number of engines
// @has_tdma:		whether this device has a TDMA block
// @cipher_algs:	supported cipher algorithms
// @ncipher_algs:	number of supported cipher algorithms
// @ahash_algs:		supported hash algorithms
// @nahash_algs:	number of supported hash algorithms
//
// Structure used to describe CESA device capabilities.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mv_cesa_caps {
    pub nengines: c_int,
    pub has_tdma: bool,
    pub cipher_algs: *mut skcipher_alg,
    pub ncipher_algs: c_int,
    pub ahash_algs: *mut ahash_alg,
    pub nahash_algs: c_int,
}

//
// struct mv_cesa_dev_dma - DMA pools
// @tdma_desc_pool:	TDMA desc pool
// @op_pool:		crypto operation pool
// @cache_pool:		data cache pool (used by hash implementation when the
// hash request is smaller than the hash block size)
// @padding_pool:	padding pool (used by hash implementation when hardware
// padding cannot be used)
//
// Structure containing the different DMA pools used by this driver.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mv_cesa_dev_dma {
    pub tdma_desc_pool: *mut dma_pool,
    pub op_pool: *mut dma_pool,
    pub cache_pool: *mut dma_pool,
    pub padding_pool: *mut dma_pool,
}

//
// struct mv_cesa_engine - CESA engine
// @id:			engine id
// @regs:		engine registers
// @sram:		SRAM memory region
// @sram_pool:		SRAM memory region from pool
// @sram_dma:		DMA address of the SRAM memory region
// @lock:		engine lock
// @req:		current crypto request
// @clk:		engine clk
// @zclk:		engine zclk
// @max_req_len:	maximum chunk length (useful to create the TDMA chain)
// @int_mask:		interrupt mask cache
// @cesa:		back-pointer to the parent CESA device
// @pool:		memory pool pointing to the memory region reserved in
// SRAM
// @queue:		fifo of the pending crypto requests
// @load:		engine load counter, useful for load balancing
// @chain_hw:		list of the current tdma descriptors being processed
// by the hardware.
// @chain_sw:		list of the current tdma descriptors that will be
// submitted to the hardware.
// @complete_queue:	fifo of the processed requests by the engine
//
// Structure storing CESA engine information.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mv_cesa_engine {
    pub id: c_int,
    pub regs: *mut void __iomem,
    pub sram: *mut void __iomem,
    pub sram_pool: *mut c_void,
}

//
// struct mv_cesa_dev - CESA device
// @caps:	device capabilities
// @regs:	device registers
// @sram_size:	usable SRAM size
// @lock:	device lock
// @dma:	dma pools
// @engines:	array of engines
//
// Structure storing CESA device information.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mv_cesa_dev {
    pub caps: *const mv_cesa_caps,
    pub regs: *mut void __iomem,
    pub dev: *mut device,
    pub sram_size: c_uint,
    pub lock: spinlock_t,
    pub dma: *mut mv_cesa_dev_dma,
    pub engines: [mv_cesa_engine; ],
}

//
// struct mv_cesa_req_ops - CESA request operations
// @process:	process a request chunk result (should return 0 if the
// operation, -EINPROGRESS if it needs more steps or an error
// code)
// @step:	launch the crypto operation on the next chunk
// @cleanup:	cleanup the crypto request (release associated data)
// @complete:	complete the request, i.e copy result or context from sram when
// needed.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mv_cesa_req_ops {
    pub status): *mut *mut *mut int (process)(struct crypto_async_request req, u32,
    pub req): *mut *mut void (step)(struct crypto_async_request,
    pub req): *mut *mut void (cleanup)(struct crypto_async_request,
    pub req): *mut *mut void (complete)(struct crypto_async_request,
}

//
// struct mv_cesa_ctx - CESA operation context
// @ops:	crypto operations
//
// Base context structure inherited by operation specific ones.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mv_cesa_ctx {
    pub ops: *const mv_cesa_req_ops,
}

//
// struct mv_cesa_hash_ctx - CESA hash operation context
// @base:	base context structure
//
// Hash context structure.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mv_cesa_hash_ctx {
    pub base: mv_cesa_ctx,
}

//
// struct mv_cesa_hash_ctx - CESA hmac operation context
// @base:	base context structure
// @iv:		initialization vectors
//
// HMAC context structure.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mv_cesa_hmac_ctx {
    pub base: mv_cesa_ctx,
    pub iv: [__be32; 16],
}

//
// enum mv_cesa_req_type - request type definitions
// @CESA_STD_REQ:	standard request
// @CESA_DMA_REQ:	DMA request
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mv_cesa_req_type {
    CESA_STD_REQ,
    CESA_DMA_REQ,
}

//
// struct mv_cesa_req - CESA request
// @engine:	engine associated with this request
// @chain:	list of tdma descriptors associated  with this request
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mv_cesa_req {
    pub engine: *mut mv_cesa_engine,
    pub chain: mv_cesa_tdma_chain,
}

//
// struct mv_cesa_sg_std_iter - CESA scatter-gather iterator for standard
// requests
// @iter:	sg mapping iterator
// @offset:	current offset in the SG entry mapped in memory
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mv_cesa_sg_std_iter {
    pub iter: sg_mapping_iter,
    pub offset: c_uint,
}

//
// struct mv_cesa_skcipher_std_req - cipher standard request
// @op:		operation context
// @offset:	current operation offset
// @size:	size of the crypto operation
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mv_cesa_skcipher_std_req {
    pub op: mv_cesa_op_ctx,
    pub offset: c_uint,
    pub size: c_uint,
    pub skip_ctx: bool,
}

//
// struct mv_cesa_skcipher_req - cipher request
// @req:	type specific request information
// @src_nents:	number of entries in the src sg list
// @dst_nents:	number of entries in the dest sg list
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mv_cesa_skcipher_req {
    pub base: mv_cesa_req,
    pub std: mv_cesa_skcipher_std_req,
    pub src_nents: c_int,
    pub dst_nents: c_int,
}

//
// struct mv_cesa_ahash_std_req - standard hash request
// @offset:	current operation offset
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mv_cesa_ahash_std_req {
    pub offset: c_uint,
}

//
// struct mv_cesa_ahash_dma_req - DMA hash request
// @padding:		padding buffer
// @padding_dma:	DMA address of the padding buffer
// @cache_dma:		DMA address of the cache buffer
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mv_cesa_ahash_dma_req {
    pub padding: *mut u8,
    pub padding_dma: dma_addr_t,
    pub cache: *mut u8,
    pub cache_dma: dma_addr_t,
}

//
// struct mv_cesa_ahash_req - hash request
// @req:		type specific request information
// @cache:		cache buffer
// @cache_ptr:		write pointer in the cache buffer
// @len:		hash total length
// @src_nents:		number of entries in the scatterlist
// @last_req:		define whether the current operation is the last one
// or not
// @state:		hash state
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mv_cesa_ahash_req {
    pub base: mv_cesa_req,
    pub dma: mv_cesa_ahash_dma_req,
    pub std: mv_cesa_ahash_std_req,
    pub req: },
    pub op_tmpl: mv_cesa_op_ctx,
    pub cache: [u8; CESA_MAX_HASH_BLOCK_SIZE],
    pub cache_ptr: c_uint,
    pub len: u64,
    pub src_nents: c_int,
    pub last_req: bool,
    pub algo_le: bool,
    pub state: [u32; 8],
}

// CESA functions
extern "C" {
    pub fn le32_to_cpu(_arg: op->desc.config) -> return;
}
//
// Helper function that indicates whether a crypto request needs to be
// cleaned up or not after being enqueued using mv_cesa_queue_req().
//
// The queue still had some space, the request was queued
// normally, so there's no need to clean it up.
//
// The queue had not space left, but since the request is
// flagged with CRYPTO_TFM_REQ_MAY_BACKLOG, it was added to
// the backlog and will be processed later. There's no need to
// clean it up.
//
// Request wasn't queued, we need to clean it up
// TDMA functions
extern "C" {
    pub fn mv_cesa_dma_step(dreq: *mut mv_cesa_req);
}
extern "C" {
    pub fn mv_cesa_dma_cleanup(dreq: *mut mv_cesa_req);
}
extern "C" {
    pub fn mv_cesa_tdma_process(engine: *mut mv_cesa_engine, status: u32) -> c_int;
}
extern "C" {
    pub fn mv_cesa_dma_add_dummy_launch(chain: *mut mv_cesa_tdma_chain, flags: gfp_t) -> c_int;
}
extern "C" {
    pub fn mv_cesa_dma_add_dummy_end(chain: *mut mv_cesa_tdma_chain, flags: gfp_t) -> c_int;
}
// Algorithm definitions
