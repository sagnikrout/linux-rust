//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/caam/pdb.h
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
// CAAM Protocol Data Block (PDB) definition header file
//
// Copyright 2008-2016 Freescale Semiconductor, Inc.
//

//
// PDB- IPSec ESP Header Modification Options
//
pub const PDBHMO_ESP_DECAP_SHIFT: c_int = 28;
pub const PDBHMO_ESP_ENCAP_SHIFT: c_int = 28;
//
// Encap and Decap - Decrement TTL (Hop Limit) - Based on the value of the
// Options Byte IP version (IPvsn) field:
// if IPv4, decrement the inner IP header TTL field (byte 8);
// if IPv6 decrement the inner IP header Hop Limit field (byte 7).
//

//
// Decap - DiffServ Copy - Copy the IPv4 TOS or IPv6 Traffic Class byte
// from the outer IP header to the inner IP header.
//

//
// Encap- Copy DF bit -if an IPv4 tunnel mode outer IP header is coming from
// the PDB, copy the DF bit from the inner IP header to the outer IP header.
//

pub const PDBNH_ESP_ENCAP_SHIFT: c_int = 16;

pub const PDBHDRLEN_ESP_DECAP_SHIFT: c_int = 16;

pub const PDB_NH_OFFSET_SHIFT: c_int = 8;

//
// PDB - IPSec ESP Encap/Decap Options
//
pub const PDBOPTS_ESP_ARSNONE: c_uint = 0x00 /* no antireplay window */;
pub const PDBOPTS_ESP_ARS32: c_uint = 0x40 /* 32-entry antireplay window */;
pub const PDBOPTS_ESP_ARS128: c_uint = 0x80 /* 128-entry antireplay window */;
pub const PDBOPTS_ESP_ARS64: c_uint = 0xc0 /* 64-entry antireplay window */;
pub const PDBOPTS_ESP_ARS_MASK: c_uint = 0xc0 /* antireplay window mask */;
pub const PDBOPTS_ESP_IVSRC: c_uint = 0x20 /* IV comes from internal random gen */;
pub const PDBOPTS_ESP_ESN: c_uint = 0x10 /* extended sequence included */;
pub const PDBOPTS_ESP_OUTFMT: c_uint = 0x08 /* output only decapsulation (decap) */;
pub const PDBOPTS_ESP_IPHDRSRC: c_uint = 0x08 /* IP header comes from PDB (encap) */;
pub const PDBOPTS_ESP_INCIPHDR: c_uint = 0x04 /* Prepend IP header to output frame */;
pub const PDBOPTS_ESP_IPVSN: c_uint = 0x02 /* process IPv6 header */;
pub const PDBOPTS_ESP_AOFL: c_uint = 0x04 /* adjust out frame len (decap, SEC>=5.3)*/;
pub const PDBOPTS_ESP_TUNNEL: c_uint = 0x01 /* tunnel mode next-header byte */;
pub const PDBOPTS_ESP_IPV6: c_uint = 0x02 /* ip header version is V6 */;
pub const PDBOPTS_ESP_DIFFSERV: c_uint = 0x40 /* copy TOS/TC from inner iphdr */;
pub const PDBOPTS_ESP_UPDATE_CSUM: c_uint = 0x80 /* encap-update ip header checksum */;
pub const PDBOPTS_ESP_VERIFY_CSUM: c_uint = 0x20 /* decap-validate ip header checksum */;
//
// General IPSec encap/decap PDB definitions
//
// ipsec_encap_cbc - PDB part for IPsec CBC encapsulation
// @iv: 16-byte array initialization vector
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipsec_encap_cbc {
    pub iv: [u8; 16],
}

//
// ipsec_encap_ctr - PDB part for IPsec CTR encapsulation
// @ctr_nonce: 4-byte array nonce
// @ctr_initial: initial count constant
// @iv: initialization vector
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipsec_encap_ctr {
    pub ctr_nonce: [u8; 4],
    pub ctr_initial: u32,
    pub iv: u64,
}

//
// ipsec_encap_ccm - PDB part for IPsec CCM encapsulation
// @salt: 3-byte array salt (lower 24 bits)
// @ccm_opt: CCM algorithm options - MSB-LSB description:
// b0_flags (8b) - CCM B0; use 0x5B for 8-byte ICV, 0x6B for 12-byte ICV,
// 0x7B for 16-byte ICV (cf. RFC4309, RFC3610)
// ctr_flags (8b) - counter flags; constant equal to 0x3
// ctr_initial (16b) - initial count constant
// @iv: initialization vector
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipsec_encap_ccm {
    pub salt: [u8; 4],
    pub ccm_opt: u32,
    pub iv: u64,
}

//
// ipsec_encap_gcm - PDB part for IPsec GCM encapsulation
// @salt: 3-byte array salt (lower 24 bits)
// @rsvd: reserved, do not use
// @iv: initialization vector
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipsec_encap_gcm {
    pub salt: [u8; 4],
    pub rsvd1: u32,
    pub iv: u64,
}

//
// ipsec_encap_pdb - PDB for IPsec encapsulation
// @options: MSB-LSB description
// hmo (header manipulation options) - 4b
// reserved - 4b
// next header - 8b
// next header offset - 8b
// option flags (depend on selected algorithm) - 8b
// @seq_num_ext_hi: (optional) IPsec Extended Sequence Number (ESN)
// @seq_num: IPsec sequence number
// @spi: IPsec SPI (Security Parameters Index)
// @ip_hdr_len: optional IP Header length (in bytes)
// reserved - 16b
// Opt. IP Hdr Len - 16b
// @ip_hdr: optional IP Header content
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipsec_encap_pdb {
    pub options: u32,
    pub seq_num_ext_hi: u32,
    pub seq_num: u32,
    pub cbc: ipsec_encap_cbc,
    pub ctr: ipsec_encap_ctr,
    pub ccm: ipsec_encap_ccm,
    pub gcm: ipsec_encap_gcm,
}

//
// ipsec_decap_cbc - PDB part for IPsec CBC decapsulation
// @rsvd: reserved, do not use
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipsec_decap_cbc {
    pub rsvd: [u32; 2],
}

//
// ipsec_decap_ctr - PDB part for IPsec CTR decapsulation
// @ctr_nonce: 4-byte array nonce
// @ctr_initial: initial count constant
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipsec_decap_ctr {
    pub ctr_nonce: [u8; 4],
    pub ctr_initial: u32,
}

//
// ipsec_decap_ccm - PDB part for IPsec CCM decapsulation
// @salt: 3-byte salt (lower 24 bits)
// @ccm_opt: CCM algorithm options - MSB-LSB description:
// b0_flags (8b) - CCM B0; use 0x5B for 8-byte ICV, 0x6B for 12-byte ICV,
// 0x7B for 16-byte ICV (cf. RFC4309, RFC3610)
// ctr_flags (8b) - counter flags; constant equal to 0x3
// ctr_initial (16b) - initial count constant
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipsec_decap_ccm {
    pub salt: [u8; 4],
    pub ccm_opt: u32,
}

//
// ipsec_decap_gcm - PDB part for IPsec GCN decapsulation
// @salt: 4-byte salt
// @rsvd: reserved, do not use
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipsec_decap_gcm {
    pub salt: [u8; 4],
    pub resvd: u32,
}

//
// ipsec_decap_pdb - PDB for IPsec decapsulation
// @options: MSB-LSB description
// hmo (header manipulation options) - 4b
// IP header length - 12b
// next header offset - 8b
// option flags (depend on selected algorithm) - 8b
// @seq_num_ext_hi: (optional) IPsec Extended Sequence Number (ESN)
// @seq_num: IPsec sequence number
// @anti_replay: Anti-replay window; size depends on ARS (option flags)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipsec_decap_pdb {
    pub options: u32,
    pub cbc: ipsec_decap_cbc,
    pub ctr: ipsec_decap_ctr,
    pub ccm: ipsec_decap_ccm,
    pub gcm: ipsec_decap_gcm,
}

//
// IPSec ESP Datapath Protocol Override Register (DPOVRD)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipsec_deco_dpovrd {
pub const IPSEC_ENCAP_DECO_DPOVRD_USE: c_uint = 0x80;
    pub ovrd_ecn: u8,
    pub ip_hdr_len: u8,
    pub nh_offset: u8,
    pub /: *mut *mut u8 next_header; / reserved if decap,
}

//
// IEEE 802.11i WiFi Protocol Data Block
//
pub const WIFI_PDBOPTS_FCS: c_uint = 0x01;
pub const WIFI_PDBOPTS_AR: c_uint = 0x40;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wifi_encap_pdb {
    pub mac_hdr_len: u16,
    pub rsvd: u8,
    pub options: u8,
    pub iv_flags: u8,
    pub pri: u8,
    pub pn1: u16,
    pub pn2: u32,
    pub frm_ctrl_mask: u16,
    pub seq_ctrl_mask: u16,
    pub rsvd1: [u8; 2],
    pub cnst: u8,
    pub key_id: u8,
    pub ctr_flags: u8,
    pub rsvd2: u8,
    pub ctr_init: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wifi_decap_pdb {
    pub mac_hdr_len: u16,
    pub rsvd: u8,
    pub options: u8,
    pub iv_flags: u8,
    pub pri: u8,
    pub pn1: u16,
    pub pn2: u32,
    pub frm_ctrl_mask: u16,
    pub seq_ctrl_mask: u16,
    pub rsvd1: [u8; 4],
    pub ctr_flags: u8,
    pub rsvd2: u8,
    pub ctr_init: u16,
}

//
// IEEE 802.16 WiMAX Protocol Data Block
//
pub const WIMAX_PDBOPTS_FCS: c_uint = 0x01;
pub const WIMAX_PDBOPTS_AR: c_uint = 0x40 /* decap only */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wimax_encap_pdb {
    pub rsvd: [u8; 3],
    pub options: u8,
    pub nonce: u32,
    pub b0_flags: u8,
    pub ctr_flags: u8,
    pub ctr_init: u16,
// begin DECO writeback region
    pub pn: u32,
// end DECO writeback region
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wimax_decap_pdb {
    pub rsvd: [u8; 3],
    pub options: u8,
    pub nonce: u32,
    pub iv_flags: u8,
    pub ctr_flags: u8,
    pub ctr_init: u16,
// begin DECO writeback region
    pub pn: u32,
    pub rsvd1: [u8; 2],
    pub antireplay_len: u16,
    pub antireplay_scorecard: u64,
// end DECO writeback region
}

//
// IEEE 801.AE MacSEC Protocol Data Block
//
pub const MACSEC_PDBOPTS_FCS: c_uint = 0x01;
pub const MACSEC_PDBOPTS_AR: c_uint = 0x40 /* used in decap only */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct macsec_encap_pdb {
    pub aad_len: u16,
    pub rsvd: u8,
    pub options: u8,
    pub sci: u64,
    pub ethertype: u16,
    pub tci_an: u8,
    pub rsvd1: u8,
// begin DECO writeback region
    pub pn: u32,
// end DECO writeback region
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct macsec_decap_pdb {
    pub aad_len: u16,
    pub rsvd: u8,
    pub options: u8,
    pub sci: u64,
    pub rsvd1: [u8; 3],
// begin DECO writeback region
    pub antireplay_len: u8,
    pub pn: u32,
    pub antireplay_scorecard: u64,
// end DECO writeback region
}

//
// SSL/TLS/DTLS Protocol Data Blocks
//
pub const TLS_PDBOPTS_ARS32: c_uint = 0x40;
pub const TLS_PDBOPTS_ARS64: c_uint = 0xc0;
pub const TLS_PDBOPTS_OUTFMT: c_uint = 0x08;
pub const TLS_PDBOPTS_IV_WRTBK: c_uint = 0x02 /* 1.1/1.2/DTLS only */;
pub const TLS_PDBOPTS_EXP_RND_IV: c_uint = 0x01 /* 1.1/1.2/DTLS only */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tls_block_encap_pdb {
    pub type: u8,
    pub version: [u8; 2],
    pub options: u8,
    pub seq_num: u64,
    pub iv: [u32; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tls_stream_encap_pdb {
    pub type: u8,
    pub version: [u8; 2],
    pub options: u8,
    pub seq_num: u64,
    pub i: u8,
    pub j: u8,
    pub rsvd1: [u8; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dtls_block_encap_pdb {
    pub type: u8,
    pub version: [u8; 2],
    pub options: u8,
    pub epoch: u16,
    pub seq_num: [u16; 3],
    pub iv: [u32; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tls_block_decap_pdb {
    pub rsvd: [u8; 3],
    pub options: u8,
    pub seq_num: u64,
    pub iv: [u32; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tls_stream_decap_pdb {
    pub rsvd: [u8; 3],
    pub options: u8,
    pub seq_num: u64,
    pub i: u8,
    pub j: u8,
    pub rsvd1: [u8; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dtls_block_decap_pdb {
    pub rsvd: [u8; 3],
    pub options: u8,
    pub epoch: u16,
    pub seq_num: [u16; 3],
    pub iv: [u32; 4],
    pub antireplay_scorecard: u64,
}

//
// SRTP Protocol Data Blocks
//
pub const SRTP_PDBOPTS_MKI: c_uint = 0x08;
pub const SRTP_PDBOPTS_AR: c_uint = 0x40;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct srtp_encap_pdb {
    pub x_len: u8,
    pub mki_len: u8,
    pub n_tag: u8,
    pub options: u8,
    pub cnst0: u32,
    pub rsvd: [u8; 2],
    pub cnst1: u16,
    pub salt: [u16; 7],
    pub cnst2: u16,
    pub rsvd1: u32,
    pub roc: u32,
    pub opt_mki: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct srtp_decap_pdb {
    pub x_len: u8,
    pub mki_len: u8,
    pub n_tag: u8,
    pub options: u8,
    pub cnst0: u32,
    pub rsvd: [u8; 2],
    pub cnst1: u16,
    pub salt: [u16; 7],
    pub cnst2: u16,
    pub rsvd1: u16,
    pub seq_num: u16,
    pub roc: u32,
    pub antireplay_scorecard: u64,
}

//
// DSA/ECDSA Protocol Data Blocks
// Two of these exist: DSA-SIGN, and DSA-VERIFY. They are similar
// except for the treatment of "w" for verify, "s" for sign,
// and the placement of "a,b".
//
pub const DSA_PDB_SGF_SHIFT: c_int = 24;

pub const DSA_PDB_L_SHIFT: c_int = 7;

pub const DSA_PDB_N_MASK: c_uint = 0x7f;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dsa_sign_pdb {
    pub /: *mut *mut u32 sgf_ln; / Use DSA_PDB_ definitions per above,
    pub q: *mut u8,
    pub r: *mut u8,
    pub /: *mut *mut *mut u8 g; / or Gx,y,
    pub s: *mut u8,
    pub f: *mut u8,
    pub c: *mut u8,
    pub d: *mut u8,
    pub /: *mut *mut *mut u8 ab; / ECC only,
    pub u: *mut u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dsa_verify_pdb {
    pub sgf_ln: u32,
    pub q: *mut u8,
    pub r: *mut u8,
    pub /: *mut *mut *mut u8 g; / or Gx,y,
    pub /: *mut *mut *mut u8 w; / or Wx,y,
    pub f: *mut u8,
    pub c: *mut u8,
    pub d: *mut u8,
    pub /: *mut *mut *mut u8 tmp; / temporary data block,
    pub /: *mut *mut *mut u8 ab; / only used if ECC processing,
}

// RSA Protocol Data Block
pub const RSA_PDB_SGF_SHIFT: c_int = 28;
pub const RSA_PDB_E_SHIFT: c_int = 12;

pub const RSA_PDB_D_SHIFT: c_int = 12;

pub const RSA_PDB_Q_SHIFT: c_int = 12;

pub const RSA_PRIV_KEY_FRM_1: c_int = 0;
pub const RSA_PRIV_KEY_FRM_2: c_int = 1;
pub const RSA_PRIV_KEY_FRM_3: c_int = 2;
//
// RSA Encrypt Protocol Data Block
// @sgf: scatter-gather field
// @f_dma: dma address of input data
// @g_dma: dma address of encrypted output data
// @n_dma: dma address of RSA modulus
// @e_dma: dma address of RSA public exponent
// @f_len: length in octets of the input data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rsa_pub_pdb {
    pub sgf: u32,
    pub f_dma: dma_addr_t,
    pub g_dma: dma_addr_t,
    pub n_dma: dma_addr_t,
    pub e_dma: dma_addr_t,
    pub f_len: u32,
}

//
// RSA Decrypt PDB - Private Key Form #1
// @sgf: scatter-gather field
// @g_dma: dma address of encrypted input data
// @f_dma: dma address of output data
// @n_dma: dma address of RSA modulus
// @d_dma: dma address of RSA private exponent
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rsa_priv_f1_pdb {
    pub sgf: u32,
    pub g_dma: dma_addr_t,
    pub f_dma: dma_addr_t,
    pub n_dma: dma_addr_t,
    pub d_dma: dma_addr_t,
}

//
// RSA Decrypt PDB - Private Key Form #2
// @sgf     : scatter-gather field
// @g_dma   : dma address of encrypted input data
// @f_dma   : dma address of output data
// @d_dma   : dma address of RSA private exponent
// @p_dma   : dma address of RSA prime factor p of RSA modulus n
// @q_dma   : dma address of RSA prime factor q of RSA modulus n
// @tmp1_dma: dma address of temporary buffer. CAAM uses this temporary buffer
// as internal state buffer. It is assumed to be as long as p.
// @tmp2_dma: dma address of temporary buffer. CAAM uses this temporary buffer
// as internal state buffer. It is assumed to be as long as q.
// @p_q_len : length in bytes of first two prime factors of the RSA modulus n
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rsa_priv_f2_pdb {
    pub sgf: u32,
    pub g_dma: dma_addr_t,
    pub f_dma: dma_addr_t,
    pub d_dma: dma_addr_t,
    pub p_dma: dma_addr_t,
    pub q_dma: dma_addr_t,
    pub tmp1_dma: dma_addr_t,
    pub tmp2_dma: dma_addr_t,
    pub p_q_len: u32,
}

//
// RSA Decrypt PDB - Private Key Form #3
// This is the RSA Chinese Reminder Theorem (CRT) form for two prime factors of
// the RSA modulus.
// @sgf     : scatter-gather field
// @g_dma   : dma address of encrypted input data
// @f_dma   : dma address of output data
// @c_dma   : dma address of RSA CRT coefficient
// @p_dma   : dma address of RSA prime factor p of RSA modulus n
// @q_dma   : dma address of RSA prime factor q of RSA modulus n
// @dp_dma  : dma address of RSA CRT exponent of RSA prime factor p
// @dp_dma  : dma address of RSA CRT exponent of RSA prime factor q
// @tmp1_dma: dma address of temporary buffer. CAAM uses this temporary buffer
// as internal state buffer. It is assumed to be as long as p.
// @tmp2_dma: dma address of temporary buffer. CAAM uses this temporary buffer
// as internal state buffer. It is assumed to be as long as q.
// @p_q_len : length in bytes of first two prime factors of the RSA modulus n
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rsa_priv_f3_pdb {
    pub sgf: u32,
    pub g_dma: dma_addr_t,
    pub f_dma: dma_addr_t,
    pub c_dma: dma_addr_t,
    pub p_dma: dma_addr_t,
    pub q_dma: dma_addr_t,
    pub dp_dma: dma_addr_t,
    pub dq_dma: dma_addr_t,
    pub tmp1_dma: dma_addr_t,
    pub tmp2_dma: dma_addr_t,
    pub p_q_len: u32,
}

