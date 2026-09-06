//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/allwinner/sun4i-ss/sun4i-ss.h
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
// sun4i-ss.h - hardware cryptographic accelerator for Allwinner A20 SoC
//
// Copyright (C) 2013-2015 Corentin LABBE <clabbe.montjoie@gmail.com>
//
// Support AES cipher with 128,192,256 bits keysize.
// Support MD5 and SHA1 hash algorithms.
// Support DES and 3DES
//
// You could find the datasheet in Documentation/arch/arm/sunxi.rst
//

pub const SS_CTL: c_uint = 0x00;
pub const SS_KEY0: c_uint = 0x04;
pub const SS_KEY1: c_uint = 0x08;
pub const SS_KEY2: c_uint = 0x0C;
pub const SS_KEY3: c_uint = 0x10;
pub const SS_KEY4: c_uint = 0x14;
pub const SS_KEY5: c_uint = 0x18;
pub const SS_KEY6: c_uint = 0x1C;
pub const SS_KEY7: c_uint = 0x20;
pub const SS_IV0: c_uint = 0x24;
pub const SS_IV1: c_uint = 0x28;
pub const SS_IV2: c_uint = 0x2C;
pub const SS_IV3: c_uint = 0x30;
pub const SS_FCSR: c_uint = 0x44;
pub const SS_MD0: c_uint = 0x4C;
pub const SS_MD1: c_uint = 0x50;
pub const SS_MD2: c_uint = 0x54;
pub const SS_MD3: c_uint = 0x58;
pub const SS_MD4: c_uint = 0x5C;
pub const SS_RXFIFO: c_uint = 0x200;
pub const SS_TXFIFO: c_uint = 0x204;
// SS_CTL configuration values
// IV mode for hash

// SS operation mode - bits 12-13

// Counter width for CNT mode - bits 10-11

// Key size for AES - bits 8-9

// Operation direction - bit 7

// SS Method - bits 4-6

// Data end bit - bit 2

// SS Enable bit - bit 0

// SS_FCSR configuration values
// RX FIFO status - bit 30

// RX FIFO empty spaces - bits 24-29

// TX FIFO status - bit 22

// TX FIFO available spaces - bits 16-21

pub const SS_RX_MAX: c_int = 32;

pub const SS_TX_MAX: c_int = 33;

//
// struct ss_variant - Describe SS hardware variant
// @sha1_in_be:		The SHA1 digest is given by SS in BE, and so need to be inverted.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ss_variant {
    pub sha1_in_be: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sun4i_ss_ctx {
    pub variant: *const ss_variant,
    pub base: *mut void __iomem,
    pub irq: c_int,
    pub busclk: *mut clk,
    pub ssclk: *mut clk,
    pub reset: *mut reset_control,
    pub dev: *mut device,
    pub res: *mut resource,
    pub /: *mut *mut *mut char buf[4  SS_RX_MAX];/ buffer for linearize SG src,
    pub /: *mut *mut *mut char bufo[4  SS_TX_MAX]; / buffer for linearize SG dst,
    pub /: *mut *mut spinlock_t slock; / control the use of the device,
    pub dbgfs_dir: *mut dentry,
    pub dbgfs_stats: *mut dentry,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sun4i_ss_alg_template {
    pub type: u32,
    pub mode: u32,
    pub crypto: skcipher_alg,
    pub hash: ahash_alg,
    pub alg: },
    pub ss: *mut sun4i_ss_ctx,
    pub stat_req: c_ulong,
    pub stat_fb: c_ulong,
    pub stat_bytes: c_ulong,
    pub stat_opti: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sun4i_tfm_ctx {
    pub /: *mut *mut u32 key[AES_MAX_KEY_SIZE / 4];/ divided by sizeof(u32),
    pub keylen: u32,
    pub keymode: u32,
    pub ss: *mut sun4i_ss_ctx,
    pub fallback_tfm: *mut crypto_skcipher,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sun4i_cipher_req_ctx {
    pub mode: u32,
    pub backup_iv: [u8; AES_BLOCK_SIZE],
    pub end: skcipher_request fallback_req; // keep at the,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sun4i_req_ctx {
    pub mode: u32,
    pub /: *mut *mut u64 byte_count; / number of bytes "uploaded" to the device,
    pub /: *mut *mut u32 hash[5]; / for storing SS_IVx register,
    pub buf: [c_char; 64],
    pub len: c_uint,
    pub flags: c_int,
}

extern "C" {
    pub fn sun4i_hash_crainit(tfm: *mut crypto_tfm) -> c_int;
}
extern "C" {
    pub fn sun4i_hash_craexit(tfm: *mut crypto_tfm);
}
extern "C" {
    pub fn sun4i_hash_init(areq: *mut ahash_request) -> c_int;
}
extern "C" {
    pub fn sun4i_hash_update(areq: *mut ahash_request) -> c_int;
}
extern "C" {
    pub fn sun4i_hash_final(areq: *mut ahash_request) -> c_int;
}
extern "C" {
    pub fn sun4i_hash_finup(areq: *mut ahash_request) -> c_int;
}
extern "C" {
    pub fn sun4i_hash_digest(areq: *mut ahash_request) -> c_int;
}
extern "C" {
    pub fn sun4i_hash_export_md5(areq: *mut ahash_request, out: *mut c_void) -> c_int;
}
extern "C" {
    pub fn sun4i_hash_import_md5(areq: *mut ahash_request, in: *const c_void) -> c_int;
}
extern "C" {
    pub fn sun4i_hash_export_sha1(areq: *mut ahash_request, out: *mut c_void) -> c_int;
}
extern "C" {
    pub fn sun4i_hash_import_sha1(areq: *mut ahash_request, in: *const c_void) -> c_int;
}
extern "C" {
    pub fn sun4i_ss_cbc_aes_encrypt(areq: *mut skcipher_request) -> c_int;
}
extern "C" {
    pub fn sun4i_ss_cbc_aes_decrypt(areq: *mut skcipher_request) -> c_int;
}
extern "C" {
    pub fn sun4i_ss_ecb_aes_encrypt(areq: *mut skcipher_request) -> c_int;
}
extern "C" {
    pub fn sun4i_ss_ecb_aes_decrypt(areq: *mut skcipher_request) -> c_int;
}
extern "C" {
    pub fn sun4i_ss_cbc_des_encrypt(areq: *mut skcipher_request) -> c_int;
}
extern "C" {
    pub fn sun4i_ss_cbc_des_decrypt(areq: *mut skcipher_request) -> c_int;
}
extern "C" {
    pub fn sun4i_ss_ecb_des_encrypt(areq: *mut skcipher_request) -> c_int;
}
extern "C" {
    pub fn sun4i_ss_ecb_des_decrypt(areq: *mut skcipher_request) -> c_int;
}
extern "C" {
    pub fn sun4i_ss_cbc_des3_encrypt(areq: *mut skcipher_request) -> c_int;
}
extern "C" {
    pub fn sun4i_ss_cbc_des3_decrypt(areq: *mut skcipher_request) -> c_int;
}
extern "C" {
    pub fn sun4i_ss_ecb_des3_encrypt(areq: *mut skcipher_request) -> c_int;
}
extern "C" {
    pub fn sun4i_ss_ecb_des3_decrypt(areq: *mut skcipher_request) -> c_int;
}
extern "C" {
    pub fn sun4i_ss_cipher_init(tfm: *mut crypto_tfm) -> c_int;
}
extern "C" {
    pub fn sun4i_ss_cipher_exit(tfm: *mut crypto_tfm);
}
