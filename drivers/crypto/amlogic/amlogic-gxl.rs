//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/amlogic/amlogic-gxl.h
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
// amlogic.h - hardware cryptographic offloader for Amlogic SoC
//
// Copyright (C) 2018-2019 Corentin LABBE <clabbe@baylibre.com>
//

pub const MODE_KEY: c_int = 1;
pub const MODE_AES_128: c_uint = 0x8;
pub const MODE_AES_192: c_uint = 0x9;
pub const MODE_AES_256: c_uint = 0xa;
pub const MESON_DECRYPT: c_int = 0;
pub const MESON_ENCRYPT: c_int = 1;
pub const MESON_OPMODE_ECB: c_int = 0;
pub const MESON_OPMODE_CBC: c_int = 1;
pub const MAXFLOW: c_int = 2;
pub const MAXDESC: c_int = 64;

//
// struct meson_desc - Descriptor for DMA operations
// Note that without datasheet, some are unknown
// @t_status:	Descriptor of the cipher operation (see description below)
// @t_src:	Physical address of data to read
// @t_dst:	Physical address of data to write
// t_status is segmented like this:
// @len:	0-16	length of data to operate
// @irq:	17	Ignored by hardware
// @eoc:	18	End means the descriptor is the last
// @loop:	19	Unknown
// @mode:	20-23	Type of algorithm (AES, SHA)
// @begin:	24	Unknown
// @end:	25	Unknown
// @op_mode:	26-27	Blockmode (CBC, ECB)
// @enc:	28	0 means decryption, 1 is for encryption
// @block:	29	Unknown
// @error:	30	Unknown
// @owner:	31	owner of the descriptor, 1 own by HW
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct meson_desc {
    pub t_status: __le32,
    pub t_src: __le32,
    pub t_dst: __le32,
}

//
// struct meson_flow - Information used by each flow
// @engine:	ptr to the crypto_engine for this flow
// @keylen:	keylen for this flow operation
// @complete:	completion for the current task on this flow
// @status:	set to 1 by interrupt if task is done
// @t_phy:	Physical address of task
// @tl:		pointer to the current ce_task for this flow
// @stat_req:	number of request done by this flow
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct meson_flow {
    pub engine: *mut crypto_engine,
    pub complete: completion,
    pub status: c_int,
    pub keylen: c_uint,
    pub t_phy: dma_addr_t,
    pub tl: *mut meson_desc,

    pub stat_req: c_ulong,

}

//
// struct meson_dev - main container for all this driver information
// @base:	base address of amlogic-crypto
// @busclk:	bus clock for amlogic-crypto
// @dev:	the platform device
// @chanlist:	array of all flow
// @flow:	flow to use in next request
// @irqs:	IRQ numbers for amlogic-crypto
// @dbgfs_dir:	Debugfs dentry for statistic directory
// @dbgfs_stats: Debugfs dentry for statistic counters
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct meson_dev {
    pub base: *mut void __iomem,
    pub busclk: *mut clk,
    pub dev: *mut device,
    pub chanlist: *mut meson_flow,
    pub flow: core::sync::atomic::AtomicI32,
    pub irqs: [c_int; MAXFLOW],
    pub dbgfs_dir: *mut dentry,

}

//
// struct meson_cipher_req_ctx - context for a skcipher request
// @op_dir:	direction (encrypt vs decrypt) for this request
// @flow:	the flow to use for this request
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct meson_cipher_req_ctx {
    pub op_dir: u32,
    pub flow: c_int,
    pub end: skcipher_request fallback_req; // keep at the,
}

//
// struct meson_cipher_tfm_ctx - context for a skcipher TFM
// @key:		pointer to key data
// @keylen:		len of the key
// @keymode:		The keymode(type and size of key) associated with this TFM
// @mc:			pointer to the private data of driver handling this TFM
// @fallback_tfm:	pointer to the fallback TFM
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct meson_cipher_tfm_ctx {
    pub key: *mut u32,
    pub keylen: u32,
    pub keymode: u32,
    pub mc: *mut meson_dev,
    pub fallback_tfm: *mut crypto_skcipher,
}

//
// struct meson_alg_template - crypto_alg template
// @type:		the CRYPTO_ALG_TYPE for this template
// @blockmode:		the type of block operation
// @mc:			pointer to the meson_dev structure associated with this template
// @alg:		one of sub struct must be used
// @stat_req:		number of request done on this template
// @stat_fb:		total of all data len done on this template
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct meson_alg_template {
    pub type: u32,
    pub blockmode: u32,
    pub skcipher: skcipher_engine_alg,
    pub alg: },
    pub mc: *mut meson_dev,

    pub stat_req: c_ulong,
    pub stat_fb: c_ulong,

}

extern "C" {
    pub fn meson_cipher_init(tfm: *mut crypto_tfm) -> c_int;
}
extern "C" {
    pub fn meson_cipher_exit(tfm: *mut crypto_tfm);
}
extern "C" {
    pub fn meson_skdecrypt(areq: *mut skcipher_request) -> c_int;
}
extern "C" {
    pub fn meson_skencrypt(areq: *mut skcipher_request) -> c_int;
}
extern "C" {
    pub fn meson_handle_cipher_request(engine: *mut crypto_engine, areq: *mut c_void) -> c_int;
}
