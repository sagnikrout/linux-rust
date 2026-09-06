//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/nvme-tcp.h
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
// NVMe over Fabrics TCP protocol header.
// Copyright (c) 2018 Lightbits Labs. All rights reserved.
//

pub const NVME_TCP_DISC_PORT: c_int = 8009;

pub const NVME_TCP_DIGEST_LENGTH: c_int = 4;
pub const NVME_TCP_MIN_MAXH2CDATA: c_int = 4096;
pub const NVME_TCP_MIN_C2HTERM_PLEN: c_int = 24;
pub const NVME_TCP_MAX_C2HTERM_PLEN: c_int = 152;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nvme_tcp_pfv {
    NVME_TCP_PFV_1_0 = 0x0,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nvme_tcp_tls_cipher {
    NVME_TCP_TLS_CIPHER_INVALID     = 0,
    NVME_TCP_TLS_CIPHER_SHA256      = 1,
    NVME_TCP_TLS_CIPHER_SHA384      = 2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nvme_tcp_fatal_error_status {
    NVME_TCP_FES_INVALID_PDU_HDR		= 0x01,
    NVME_TCP_FES_PDU_SEQ_ERR		= 0x02,
    NVME_TCP_FES_HDR_DIGEST_ERR		= 0x03,
    NVME_TCP_FES_DATA_OUT_OF_RANGE		= 0x04,
    NVME_TCP_FES_R2T_LIMIT_EXCEEDED		= 0x05,
    NVME_TCP_FES_DATA_LIMIT_EXCEEDED	= 0x05,
    NVME_TCP_FES_UNSUPPORTED_PARAM		= 0x06,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nvme_tcp_digest_option {
    NVME_TCP_HDR_DIGEST_ENABLE	= (1 << 0),
    NVME_TCP_DATA_DIGEST_ENABLE	= (1 << 1),
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nvme_tcp_pdu_type {
    nvme_tcp_icreq		= 0x0,
    nvme_tcp_icresp		= 0x1,
    nvme_tcp_h2c_term	= 0x2,
    nvme_tcp_c2h_term	= 0x3,
    nvme_tcp_cmd		= 0x4,
    nvme_tcp_rsp		= 0x5,
    nvme_tcp_h2c_data	= 0x6,
    nvme_tcp_c2h_data	= 0x7,
    nvme_tcp_r2t		= 0x9,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nvme_tcp_pdu_flags {
    NVME_TCP_F_HDGST		= (1 << 0),
    NVME_TCP_F_DDGST		= (1 << 1),
    NVME_TCP_F_DATA_LAST		= (1 << 2),
    NVME_TCP_F_DATA_SUCCESS		= (1 << 3),
}

//
// struct nvme_tcp_hdr - nvme tcp pdu common header
//
// @type:          pdu type
// @flags:         pdu specific flags
// @hlen:          pdu header length
// @pdo:           pdu data offset
// @plen:          pdu wire byte length
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvme_tcp_hdr {
    pub type: __u8,
    pub flags: __u8,
    pub hlen: __u8,
    pub pdo: __u8,
    pub plen: __le32,
}

//
// struct nvme_tcp_icreq_pdu - nvme tcp initialize connection request pdu
//
// @hdr:           pdu generic header
// @pfv:           pdu version format
// @hpda:          host pdu data alignment (dwords, 0's based)
// @digest:        digest types enabled
// @maxr2t:        maximum r2ts per request supported
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvme_tcp_icreq_pdu {
    pub hdr: nvme_tcp_hdr,
    pub pfv: __le16,
    pub hpda: __u8,
    pub digest: __u8,
    pub maxr2t: __le32,
    pub rsvd2: [__u8; 112],
}

//
// struct nvme_tcp_icresp_pdu - nvme tcp initialize connection response pdu
//
// @hdr:           pdu common header
// @pfv:           pdu version format
// @cpda:          controller pdu data alignment (dowrds, 0's based)
// @digest:        digest types enabled
// @maxdata:       maximum data capsules per r2t supported
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvme_tcp_icresp_pdu {
    pub hdr: nvme_tcp_hdr,
    pub pfv: __le16,
    pub cpda: __u8,
    pub digest: __u8,
    pub maxdata: __le32,
    pub rsvd: [__u8; 112],
}

//
// struct nvme_tcp_term_pdu - nvme tcp terminate connection pdu
//
// @hdr:           pdu common header
// @fes:           fatal error status
// @feil:          fatal error information (low 16 bits)
// @feih:          fatal error information (high 16 bits)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvme_tcp_term_pdu {
    pub hdr: nvme_tcp_hdr,
    pub fes: __le16,
    pub feil: __le16,
    pub feiu: __le16,
    pub rsvd: [__u8; 10],
}

//
// struct nvme_tcp_cmd_pdu - nvme tcp command capsule pdu
//
// @hdr:           pdu common header
// @cmd:           nvme command
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvme_tcp_cmd_pdu {
    pub hdr: nvme_tcp_hdr,
    pub cmd: nvme_command,
}

//
// struct nvme_tcp_rsp_pdu - nvme tcp response capsule pdu
//
// @hdr:           nvme-tcp generic header
// @cqe:           nvme completion queue entry
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvme_tcp_rsp_pdu {
    pub hdr: nvme_tcp_hdr,
    pub cqe: nvme_completion,
}

//
// struct nvme_tcp_r2t_pdu - nvme tcp ready-to-transfer pdu
//
// @hdr:           pdu common header
// @command_id:    nvme command identifier which this relates to
// @ttag:          transfer tag (controller generated)
// @r2t_offset:    offset from the start of the command data
// @r2t_length:    length the host is allowed to send
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvme_tcp_r2t_pdu {
    pub hdr: nvme_tcp_hdr,
    pub command_id: __u16,
    pub ttag: __u16,
    pub r2t_offset: __le32,
    pub r2t_length: __le32,
    pub rsvd: [__u8; 4],
}

//
// struct nvme_tcp_data_pdu - nvme tcp data pdu
//
// @hdr:           pdu common header
// @command_id:    nvme command identifier which this relates to
// @ttag:          transfer tag (controller generated)
// @data_offset:   offset from the start of the command data
// @data_length:   length of the data stream
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvme_tcp_data_pdu {
    pub hdr: nvme_tcp_hdr,
    pub command_id: __u16,
    pub ttag: __u16,
    pub data_offset: __le32,
    pub data_length: __le32,
    pub rsvd: [__u8; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union nvme_tcp_pdu {
    pub icreq: nvme_tcp_icreq_pdu,
    pub icresp: nvme_tcp_icresp_pdu,
    pub cmd: nvme_tcp_cmd_pdu,
    pub rsp: nvme_tcp_rsp_pdu,
    pub r2t: nvme_tcp_r2t_pdu,
    pub data: nvme_tcp_data_pdu,
}
