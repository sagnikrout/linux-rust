//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/rockchip/rk3288_crypto.h
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

// Crypto control registers
pub const RK_CRYPTO_INTSTS: c_uint = 0x0000;

pub const RK_CRYPTO_INTENA: c_uint = 0x0004;

pub const RK_CRYPTO_CTRL: c_uint = 0x0008;

pub const RK_CRYPTO_CONF: c_uint = 0x000c;
// HASH Receive DMA Address Mode:   fix | increment

// Block Transmit DMA Address Mode: fix | increment

// Block Receive DMA Address Mode:  fix | increment

// AES = 0 OR DES = 1

// Block Receiving DMA Start Address Register
pub const RK_CRYPTO_BRDMAS: c_uint = 0x0010;
// Block Transmitting DMA Start Address Register
pub const RK_CRYPTO_BTDMAS: c_uint = 0x0014;
// Block Receiving DMA Length Register
pub const RK_CRYPTO_BRDMAL: c_uint = 0x0018;
// Hash Receiving DMA Start Address Register
pub const RK_CRYPTO_HRDMAS: c_uint = 0x001c;
// Hash Receiving DMA Length Register
pub const RK_CRYPTO_HRDMAL: c_uint = 0x0020;
// AES registers
pub const RK_CRYPTO_AES_CTRL: c_uint = 0x0080;

// Slave = 0 / fifo = 1

// Encryption = 0 , Decryption = 1

pub const RK_CRYPTO_AES_STS: c_uint = 0x0084;

// AES Input Data 0-3 Register
pub const RK_CRYPTO_AES_DIN_0: c_uint = 0x0088;
pub const RK_CRYPTO_AES_DIN_1: c_uint = 0x008c;
pub const RK_CRYPTO_AES_DIN_2: c_uint = 0x0090;
pub const RK_CRYPTO_AES_DIN_3: c_uint = 0x0094;
// AES output Data 0-3 Register
pub const RK_CRYPTO_AES_DOUT_0: c_uint = 0x0098;
pub const RK_CRYPTO_AES_DOUT_1: c_uint = 0x009c;
pub const RK_CRYPTO_AES_DOUT_2: c_uint = 0x00a0;
pub const RK_CRYPTO_AES_DOUT_3: c_uint = 0x00a4;
// AES IV Data 0-3 Register
pub const RK_CRYPTO_AES_IV_0: c_uint = 0x00a8;
pub const RK_CRYPTO_AES_IV_1: c_uint = 0x00ac;
pub const RK_CRYPTO_AES_IV_2: c_uint = 0x00b0;
pub const RK_CRYPTO_AES_IV_3: c_uint = 0x00b4;
// AES Key Data 0-3 Register
pub const RK_CRYPTO_AES_KEY_0: c_uint = 0x00b8;
pub const RK_CRYPTO_AES_KEY_1: c_uint = 0x00bc;
pub const RK_CRYPTO_AES_KEY_2: c_uint = 0x00c0;
pub const RK_CRYPTO_AES_KEY_3: c_uint = 0x00c4;
pub const RK_CRYPTO_AES_KEY_4: c_uint = 0x00c8;
pub const RK_CRYPTO_AES_KEY_5: c_uint = 0x00cc;
pub const RK_CRYPTO_AES_KEY_6: c_uint = 0x00d0;
pub const RK_CRYPTO_AES_KEY_7: c_uint = 0x00d4;
// des/tdes
pub const RK_CRYPTO_TDES_CTRL: c_uint = 0x0100;

// 0: ECB, 1: CBC

// TDES Key Mode, 0 : EDE, 1 : EEE

// 0: DES, 1:TDES

// 0: Slave, 1:Fifo

// Encryption = 0 , Decryption = 1

pub const RK_CRYPTO_TDES_STS: c_uint = 0x0104;

pub const RK_CRYPTO_TDES_DIN_0: c_uint = 0x0108;
pub const RK_CRYPTO_TDES_DIN_1: c_uint = 0x010c;
pub const RK_CRYPTO_TDES_DOUT_0: c_uint = 0x0110;
pub const RK_CRYPTO_TDES_DOUT_1: c_uint = 0x0114;
pub const RK_CRYPTO_TDES_IV_0: c_uint = 0x0118;
pub const RK_CRYPTO_TDES_IV_1: c_uint = 0x011c;
pub const RK_CRYPTO_TDES_KEY1_0: c_uint = 0x0120;
pub const RK_CRYPTO_TDES_KEY1_1: c_uint = 0x0124;
pub const RK_CRYPTO_TDES_KEY2_0: c_uint = 0x0128;
pub const RK_CRYPTO_TDES_KEY2_1: c_uint = 0x012c;
pub const RK_CRYPTO_TDES_KEY3_0: c_uint = 0x0130;
pub const RK_CRYPTO_TDES_KEY3_1: c_uint = 0x0134;
// HASH
pub const RK_CRYPTO_HASH_CTRL: c_uint = 0x0180;

pub const RK_CRYPTO_HASH_STS: c_uint = 0x0184;

pub const RK_CRYPTO_HASH_MSG_LEN: c_uint = 0x0188;
pub const RK_CRYPTO_HASH_DOUT_0: c_uint = 0x018c;
pub const RK_CRYPTO_HASH_DOUT_1: c_uint = 0x0190;
pub const RK_CRYPTO_HASH_DOUT_2: c_uint = 0x0194;
pub const RK_CRYPTO_HASH_DOUT_3: c_uint = 0x0198;
pub const RK_CRYPTO_HASH_DOUT_4: c_uint = 0x019c;
pub const RK_CRYPTO_HASH_DOUT_5: c_uint = 0x01a0;
pub const RK_CRYPTO_HASH_DOUT_6: c_uint = 0x01a4;
pub const RK_CRYPTO_HASH_DOUT_7: c_uint = 0x01a8;

pub const RK_MAX_CLKS: c_int = 4;
//
// struct rockchip_ip - struct for managing a list of RK crypto instance
// @dev_list:		Used for doing a list of rk_crypto_info
// @lock:		Control access to dev_list
// @dbgfs_dir:		Debugfs dentry for statistic directory
// @dbgfs_stats:	Debugfs dentry for statistic counters
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rockchip_ip {
    pub dev_list: list_head,
    pub /: *mut *mut spinlock_t lock; / Control access to dev_list,
    pub dbgfs_dir: *mut dentry,
    pub dbgfs_stats: *mut dentry,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rk_clks {
    pub name: *const c_char,
    pub max: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rk_variant {
    pub num_clks: c_int,
    pub rkclks: [rk_clks; RK_MAX_CLKS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rk_crypto_info {
    pub list: list_head,
    pub dev: *mut device,
    pub clks: *mut clk_bulk_data,
    pub num_clks: c_int,
    pub rst: *mut reset_control,
    pub reg: *mut void __iomem,
    pub irq: c_int,
    pub variant: *const rk_variant,
    pub nreq: c_ulong,
    pub engine: *mut crypto_engine,
    pub complete: completion,
    pub status: c_int,
}

// the private variable of hash
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rk_ahash_ctx {
// for fallback
    pub fallback_tfm: *mut crypto_ahash,
}

// the private variable of hash for fallback
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rk_ahash_rctx {
    pub dev: *mut rk_crypto_info,
    pub fallback_req: ahash_request,
    pub mode: u32,
    pub nrsg: c_int,
}

// the private variable of cipher
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rk_cipher_ctx {
    pub keylen: c_uint,
    pub key: [u8; AES_MAX_KEY_SIZE],
    pub iv: [u8; AES_BLOCK_SIZE],
    pub fallback_tfm: *mut crypto_skcipher,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rk_cipher_rctx {
    pub dev: *mut rk_crypto_info,
    pub backup_iv: [u8; AES_BLOCK_SIZE],
    pub mode: u32,
    pub end: skcipher_request fallback_req; // keep at the,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rk_crypto_tmp {
    pub type: u32,
    pub dev: *mut rk_crypto_info,
    pub skcipher: skcipher_engine_alg,
    pub hash: ahash_engine_alg,
    pub alg: },
    pub stat_req: c_ulong,
    pub stat_fb: c_ulong,
    pub stat_fb_len: c_ulong,
    pub stat_fb_sglen: c_ulong,
    pub stat_fb_align: c_ulong,
    pub stat_fb_sgdiff: c_ulong,
}
