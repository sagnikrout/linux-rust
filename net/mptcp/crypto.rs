//! Automatically rewritten from C to Rust
//! Source: net/mptcp/crypto.c
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
// Multipath TCP cryptographic functions
// Copyright (c) 2017 - 2019, Intel Corporation.
//
// Note: This code is based on mptcp_ctrl.c, mptcp_ipv4.c, and
// mptcp_ipv6 from multipath-tcp.org, authored by:
//
// Sébastien Barré <sebastien.barre@uclouvain.be>
// Christoph Paasch <christoph.paasch@uclouvain.be>
// Jaakko Korkeaniemi <jaakko.korkeaniemi@aalto.fi>
// Gregory Detal <gregory.detal@uclouvain.be>
// Fabien Duchêne <fabien.duchene@uclouvain.be>
// Andreas Seelinger <Andreas.Seelinger@rwth-aachen.de>
// Lavkesh Lahngir <lavkesh51@gmail.com>
// Andreas Ripke <ripke@neclab.eu>
// Vlad Dogaru <vlad.dogaru@intel.com>
// Octavian Purdila <octavian.purdila@intel.com>
// John Ronan <jronan@tssg.org>
// Catalin Nicutar <catalin.nicutar@gmail.com>
// Brandon Heller <brandonh@stanford.edu>
//

#[no_mangle]
pub unsafe extern "C" fn mptcp_crypto_key_sha(key: u64, token: *mut u32, idsn: *mut u64) {
    void mptcp_crypto_key_sha(u64 key, u32 *token, u64 *idsn)
    {
    __be32 mptcp_hashed_key[SHA256_DIGEST_WORDS];
    let mut input: __be64 = cpu_to_be64(key);
    sha256(( u8 *)&input, sizeof(input), (u8 *)mptcp_hashed_key);
    if (token)
// token = be32_to_cpu(mptcp_hashed_key[0]);
    if (idsn)
// idsn = be64_to_cpu(*((__be64 *)&mptcp_hashed_key[6]));
    }
#[no_mangle]
pub unsafe extern "C" fn mptcp_crypto_hmac_sha(key1: u64, key2: u64, msg: *mut u8, len: c_int, hmac: *mut c_void) {
    void mptcp_crypto_hmac_sha(u64 key1, u64 key2, u8 *msg, int len, void *hmac)
    {
    __be64 key[2] = { cpu_to_be64(key1), cpu_to_be64(key2) };
    hmac_sha256_usingrawkey((const u8 *)key, sizeof(key), msg, len, hmac);
    }

    EXPORT_SYMBOL_GPL(mptcp_crypto_hmac_sha);
