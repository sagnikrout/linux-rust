//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/caam/caampkc.h
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

//
// caam_priv_key_form - CAAM RSA private key representation
// CAAM RSA private key may have either of three forms.
//
// 1. The first representation consists of the pair (n, d), where the
// components have the following meanings:
// n      the RSA modulus
// d      the RSA private exponent
//
// 2. The second representation consists of the triplet (p, q, d), where the
// components have the following meanings:
// p      the first prime factor of the RSA modulus n
// q      the second prime factor of the RSA modulus n
// d      the RSA private exponent
//
// 3. The third representation consists of the quintuple (p, q, dP, dQ, qInv),
// where the components have the following meanings:
// p      the first prime factor of the RSA modulus n
// q      the second prime factor of the RSA modulus n
// dP     the first factors's CRT exponent
// dQ     the second factors's CRT exponent
// qInv   the (first) CRT coefficient
//
// The benefit of using the third or the second key form is lower computational
// cost for the decryption and signature operations.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum caam_priv_key_form {
    FORM1,
    FORM2,
    FORM3
}

//
// caam_rsa_key - CAAM RSA key structure. Keys are allocated in DMA zone.
// @n           : RSA modulus raw byte stream
// @e           : RSA public exponent raw byte stream
// @d           : RSA private exponent raw byte stream
// @p           : RSA prime factor p of RSA modulus n
// @q           : RSA prime factor q of RSA modulus n
// @dp          : RSA CRT exponent of p
// @dp          : RSA CRT exponent of q
// @qinv        : RSA CRT coefficient
// @tmp1        : CAAM uses this temporary buffer as internal state buffer.
// It is assumed to be as long as p.
// @tmp2        : CAAM uses this temporary buffer as internal state buffer.
// It is assumed to be as long as q.
// @n_sz        : length in bytes of RSA modulus n
// @e_sz        : length in bytes of RSA public exponent
// @d_sz        : length in bytes of RSA private exponent
// @p_sz        : length in bytes of RSA prime factor p of RSA modulus n
// @q_sz        : length in bytes of RSA prime factor q of RSA modulus n
// @priv_form   : CAAM RSA private key representation
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct caam_rsa_key {
    pub n: *mut u8,
    pub e: *mut u8,
    pub d: *mut u8,
    pub p: *mut u8,
    pub q: *mut u8,
    pub dp: *mut u8,
    pub dq: *mut u8,
    pub qinv: *mut u8,
    pub tmp1: *mut u8,
    pub tmp2: *mut u8,
    pub n_sz: usize,
    pub e_sz: usize,
    pub d_sz: usize,
    pub p_sz: usize,
    pub q_sz: usize,
    pub priv_form: caam_priv_key_form,
}

//
// caam_rsa_ctx - per session context.
// @key         : RSA key in DMA zone
// @dev         : device structure
// @padding_dma : dma address of padding, for adding it to the input
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct caam_rsa_ctx {
    pub key: caam_rsa_key,
    pub dev: *mut device,
    pub padding_dma: dma_addr_t,
}

//
// caam_rsa_req_ctx - per request context.
// @src           : input scatterlist (stripped of leading zeros)
// @fixup_src     : input scatterlist (that might be stripped of leading zeros)
// @fixup_src_len : length of the fixup_src input scatterlist
// @edesc         : s/w-extended rsa descriptor
// @akcipher_op_done : callback used when operation is done
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct caam_rsa_req_ctx {
    pub src: [scatterlist; 2],
    pub fixup_src: *mut scatterlist,
    pub fixup_src_len: c_uint,
    pub edesc: *mut rsa_edesc,
    pub context): *mut c_void,
}

//
// rsa_edesc - s/w-extended rsa descriptor
// @src_nents     : number of segments in input s/w scatterlist
// @dst_nents     : number of segments in output s/w scatterlist
// @mapped_src_nents: number of segments in input h/w link table
// @mapped_dst_nents: number of segments in output h/w link table
// @sec4_sg_bytes : length of h/w link table
// @bklog         : stored to determine if the request needs backlog
// @sec4_sg_dma   : dma address of h/w link table
// @sec4_sg       : pointer to h/w link table
// @pdb           : specific RSA Protocol Data Block (PDB)
// @hw_desc       : descriptor followed by link tables if any
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rsa_edesc {
    pub src_nents: c_int,
    pub dst_nents: c_int,
    pub mapped_src_nents: c_int,
    pub mapped_dst_nents: c_int,
    pub sec4_sg_bytes: c_int,
    pub bklog: bool,
    pub sec4_sg_dma: dma_addr_t,
    pub sec4_sg: *mut sec4_sg_entry,
    pub pub: rsa_pub_pdb,
    pub priv_f1: rsa_priv_f1_pdb,
    pub priv_f2: rsa_priv_f2_pdb,
    pub priv_f3: rsa_priv_f3_pdb,
    pub pdb: },
    pub hw_desc: [u32; ],
}

// Descriptor construction primitives.
extern "C" {
    pub fn init_rsa_pub_desc(desc: *mut u32, pdb: *mut rsa_pub_pdb);
}
extern "C" {
    pub fn init_rsa_priv_f1_desc(desc: *mut u32, pdb: *mut rsa_priv_f1_pdb);
}
extern "C" {
    pub fn init_rsa_priv_f2_desc(desc: *mut u32, pdb: *mut rsa_priv_f2_pdb);
}
extern "C" {
    pub fn init_rsa_priv_f3_desc(desc: *mut u32, pdb: *mut rsa_priv_f3_pdb);
}
