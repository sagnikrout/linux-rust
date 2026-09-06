//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/nx/nx.h
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

// a scatterlist in the format PHYP is expecting
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nx_sg {
    pub addr: u64,
    pub rsvd: u32,
    pub len: u32,
    pub __attribute((packed)): },

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nx_status {
    NX_DISABLED,
    NX_WAITING,
    NX_OKAY
}

// msc_triplet and max_sync_cop are used only to assist in parsing the
// openFirmware property
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msc_triplet {
    pub keybitlen: u32,
    pub databytelen: u32,
    pub sglen: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct max_sync_cop {
    pub fc: u32,
    pub mode: u32,
    pub triplets: u32,
    pub trip: [msc_triplet; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct alg_props {
    pub databytelen: u32,
    pub sglen: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nx_of {
    pub flags: u32,
    pub max_sg_len: u32,
    pub status: nx_status,
    pub ap: [alg_props; NX_MAX_FC][NX_MAX_MODE][3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nx_stats {
    pub aes_ops: core::sync::atomic::AtomicI32,
    pub aes_bytes: core::sync::atomic::AtomicI64,
    pub sha256_ops: core::sync::atomic::AtomicI32,
    pub sha256_bytes: core::sync::atomic::AtomicI64,
    pub sha512_ops: core::sync::atomic::AtomicI32,
    pub sha512_bytes: core::sync::atomic::AtomicI64,
    pub sync_ops: core::sync::atomic::AtomicI32,
    pub errors: core::sync::atomic::AtomicI32,
    pub last_error: core::sync::atomic::AtomicI32,
    pub last_error_pid: core::sync::atomic::AtomicI32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nx_crypto_driver {
    pub stats: nx_stats,
    pub of: nx_of,
    pub viodev: *mut vio_dev,
    pub viodriver: vio_driver,
    pub dfs_root: *mut dentry,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nx_gcm_rctx {
    pub iv: [u8; 16],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nx_gcm_priv {
    pub iauth_tag: [u8; 16],
    pub nonce: [u8; NX_GCM4106_NONCE_LEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nx_ccm_rctx {
    pub iv: [u8; 16],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nx_ccm_priv {
    pub b0: [u8; 16],
    pub iauth_tag: [u8; 16],
    pub oauth_tag: [u8; 16],
    pub nonce: [u8; NX_CCM4309_NONCE_LEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nx_xcbc_priv {
    pub key: [u8; 16],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nx_ctr_priv {
    pub nonce: [u8; CTR_RFC3686_NONCE_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nx_crypto_ctx {
    pub /: *mut *mut spinlock_t lock; / synchronize access to the context,
    pub /: *mut *mut *mut void kmem; / unaligned, kmalloc'd buffer,
    pub /: *mut *mut size_t kmem_len; / length of kmem,
    pub /: *mut *mut *mut nx_csbcpb csbcpb; / aligned page given to phyp @ hcall time,
    pub /: *mut *mut vio_pfo_op op; / operation with hcall parameters,
    pub /: *mut *mut *mut nx_csbcpb csbcpb_aead; / secondary csbcpb used by AEAD algs,
    pub /: *mut *mut vio_pfo_op op_aead;/ operation for csbcpb_aead,
    pub /: *mut *mut *mut nx_sg in_sg; / aligned pointer into kmem to an sg list,
    pub /: *mut *mut *mut nx_sg out_sg; / aligned pointer into kmem to an sg list,
    pub /: *mut *mut *mut alg_props ap; / pointer into props based on our key size,
    pub /: *mut *mut alg_props props[3];/ openFirmware properties for requests,
    pub stats: *mut *mut *mut nx_stats stats; / pointer into an nx_crypto_driver for,
    pub gcm: nx_gcm_priv,
    pub ccm: nx_ccm_priv,
    pub xcbc: nx_xcbc_priv,
    pub ctr: nx_ctr_priv,
    pub priv: },
}

// prototypes
extern "C" {
    pub fn nx_crypto_ctx_aes_ccm_init(tfm: *mut crypto_aead) -> c_int;
}
extern "C" {
    pub fn nx_crypto_ctx_aes_gcm_init(tfm: *mut crypto_aead) -> c_int;
}
extern "C" {
    pub fn nx_crypto_ctx_aes_xcbc_init(tfm: *mut crypto_shash) -> c_int;
}
extern "C" {
    pub fn nx_crypto_ctx_aes_ctr_init(tfm: *mut crypto_skcipher) -> c_int;
}
extern "C" {
    pub fn nx_crypto_ctx_aes_cbc_init(tfm: *mut crypto_skcipher) -> c_int;
}
extern "C" {
    pub fn nx_crypto_ctx_aes_ecb_init(tfm: *mut crypto_skcipher) -> c_int;
}
extern "C" {
    pub fn nx_crypto_ctx_sha_init(tfm: *mut crypto_shash) -> c_int;
}
extern "C" {
    pub fn nx_crypto_ctx_exit(nx_ctx: *mut nx_crypto_ctx);
}
extern "C" {
    pub fn nx_crypto_ctx_skcipher_exit(tfm: *mut crypto_skcipher);
}
extern "C" {
    pub fn nx_crypto_ctx_aead_exit(tfm: *mut crypto_aead);
}
extern "C" {
    pub fn nx_crypto_ctx_shash_exit(tfm: *mut crypto_shash);
}
extern "C" {
    pub fn nx_ctx_init(nx_ctx: *mut nx_crypto_ctx, function: c_uint);
}

extern "C" {
    pub fn nx_debugfs_init(: *mut nx_crypto_driver);
}
extern "C" {
    pub fn nx_debugfs_fini(: *mut nx_crypto_driver);
}

