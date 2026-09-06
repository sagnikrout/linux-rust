//! Automatically rewritten from C to Rust
//! Source: drivers/crypto/caam/pkc_desc.c
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
// caam - Freescale FSL CAAM support for Public Key Cryptography descriptors
//
// Copyright 2016 Freescale Semiconductor, Inc.
//
// There is no Shared Descriptor for PKC so that the Job Descriptor must carry
// all the desired key parameters, input and output pointers.
//

// Descriptor for RSA Public operation
#[no_mangle]
pub unsafe extern "C" fn init_rsa_pub_desc(desc: *mut u32, pdb: *mut rsa_pub_pdb) {
    void init_rsa_pub_desc(u32 *desc, struct rsa_pub_pdb *pdb)
    {
    init_job_desc_pdb(desc, 0, SIZEOF_RSA_PUB_PDB);
    append_cmd(desc, pdb.sgf);
    append_ptr(desc, pdb.f_dma);
    append_ptr(desc, pdb.g_dma);
    append_ptr(desc, pdb.n_dma);
    append_ptr(desc, pdb.e_dma);
    append_cmd(desc, pdb.f_len);
    append_operation(desc, OP_TYPE_UNI_PROTOCOL | OP_PCLID_RSAENC_PUBKEY);
    }
// Descriptor for RSA Private operation - Private Key Form #1
#[no_mangle]
pub unsafe extern "C" fn init_rsa_priv_f1_desc(desc: *mut u32, pdb: *mut rsa_priv_f1_pdb) {
    void init_rsa_priv_f1_desc(u32 *desc, struct rsa_priv_f1_pdb *pdb)
    {
    init_job_desc_pdb(desc, 0, SIZEOF_RSA_PRIV_F1_PDB);
    append_cmd(desc, pdb.sgf);
    append_ptr(desc, pdb.g_dma);
    append_ptr(desc, pdb.f_dma);
    append_ptr(desc, pdb.n_dma);
    append_ptr(desc, pdb.d_dma);
    append_operation(desc, OP_TYPE_UNI_PROTOCOL | OP_PCLID_RSADEC_PRVKEY |
    RSA_PRIV_KEY_FRM_1);
    }
// Descriptor for RSA Private operation - Private Key Form #2
#[no_mangle]
pub unsafe extern "C" fn init_rsa_priv_f2_desc(desc: *mut u32, pdb: *mut rsa_priv_f2_pdb) {
    void init_rsa_priv_f2_desc(u32 *desc, struct rsa_priv_f2_pdb *pdb)
    {
    init_job_desc_pdb(desc, 0, SIZEOF_RSA_PRIV_F2_PDB);
    append_cmd(desc, pdb.sgf);
    append_ptr(desc, pdb.g_dma);
    append_ptr(desc, pdb.f_dma);
    append_ptr(desc, pdb.d_dma);
    append_ptr(desc, pdb.p_dma);
    append_ptr(desc, pdb.q_dma);
    append_ptr(desc, pdb.tmp1_dma);
    append_ptr(desc, pdb.tmp2_dma);
    append_cmd(desc, pdb.p_q_len);
    append_operation(desc, OP_TYPE_UNI_PROTOCOL | OP_PCLID_RSADEC_PRVKEY |
    RSA_PRIV_KEY_FRM_2);
    }
// Descriptor for RSA Private operation - Private Key Form #3
#[no_mangle]
pub unsafe extern "C" fn init_rsa_priv_f3_desc(desc: *mut u32, pdb: *mut rsa_priv_f3_pdb) {
    void init_rsa_priv_f3_desc(u32 *desc, struct rsa_priv_f3_pdb *pdb)
    {
    init_job_desc_pdb(desc, 0, SIZEOF_RSA_PRIV_F3_PDB);
    append_cmd(desc, pdb.sgf);
    append_ptr(desc, pdb.g_dma);
    append_ptr(desc, pdb.f_dma);
    append_ptr(desc, pdb.c_dma);
    append_ptr(desc, pdb.p_dma);
    append_ptr(desc, pdb.q_dma);
    append_ptr(desc, pdb.dp_dma);
    append_ptr(desc, pdb.dq_dma);
    append_ptr(desc, pdb.tmp1_dma);
    append_ptr(desc, pdb.tmp2_dma);
    append_cmd(desc, pdb.p_q_len);
    append_operation(desc, OP_TYPE_UNI_PROTOCOL | OP_PCLID_RSADEC_PRVKEY |
    RSA_PRIV_KEY_FRM_3);
    }
