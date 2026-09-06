//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/erspan.h
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


//
// GRE header for ERSPAN type I encapsulation (4 octets [34:37])
// 0                   1                   2                   3
// 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |0|0|0|0|0|00000|000000000|00000|    Protocol Type for ERSPAN   |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//
// The Type I ERSPAN frame format is based on the barebones IP + GRE
// encapsulation (as described above) on top of the raw mirrored frame.
// There is no extra ERSPAN header.
//
// GRE header for ERSPAN type II and II encapsulation (8 octets [34:41])
// 0                   1                   2                   3
// 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |0|0|0|1|0|00000|000000000|00000|    Protocol Type for ERSPAN   |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |      Sequence Number (increments per packet per session)      |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//
// Note that in the above GRE header [RFC1701] out of the C, R, K, S,
// s, Recur, Flags, Version fields only S (bit 03) is set to 1. The
// other fields are set to zero, so only a sequence number follows.
//
// ERSPAN Version 1 (Type II) header (8 octets [42:49])
// 0                   1                   2                   3
// 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |  Ver  |          VLAN         | COS | En|T|    Session ID     |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |      Reserved         |                  Index                |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//
// ERSPAN Version 2 (Type III) header (12 octets [42:49])
// 0                   1                   2                   3
// 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |  Ver  |          VLAN         | COS |BSO|T|     Session ID    |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |                          Timestamp                            |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |             SGT               |P|    FT   |   Hw ID   |D|Gra|O|
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//
// Platform Specific SubHeader (8 octets, optional)
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |  Platf ID |               Platform Specific Info              |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |                  Platform Specific Info                       |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//
// GRE proto ERSPAN type I/II = 0x88BE, type III = 0x22EB
//

pub const ERSPAN_VERSION: c_uint = 0x1	/* ERSPAN type II */;
pub const VER_MASK: c_uint = 0xf000;
pub const VLAN_MASK: c_uint = 0x0fff;
pub const COS_MASK: c_uint = 0xe000;
pub const EN_MASK: c_uint = 0x1800;
pub const T_MASK: c_uint = 0x0400;
pub const ID_MASK: c_uint = 0x03ff;
pub const INDEX_MASK: c_uint = 0xfffff;
pub const ERSPAN_VERSION2: c_uint = 0x2	/* ERSPAN type III*/;

pub const SGT_MASK: c_uint = 0xffff0000;
pub const P_MASK: c_uint = 0x8000;
pub const FT_MASK: c_uint = 0x7c00;
pub const HWID_MASK: c_uint = 0x03f0;
pub const DIR_MASK: c_uint = 0x0008;
pub const GRA_MASK: c_uint = 0x0006;
pub const O_MASK: c_uint = 0x0001;
pub const HWID_OFFSET: c_int = 4;
pub const DIR_OFFSET: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum erspan_encap_type {
    ERSPAN_ENCAP_NOVLAN = 0x0,	/* originally without VLAN tag */
    ERSPAN_ENCAP_ISL = 0x1,		/* originally ISL encapsulated */
    ERSPAN_ENCAP_8021Q = 0x2,	/* originally 802.1Q encapsulated */
    ERSPAN_ENCAP_INFRAME = 0x3,	/* VLAN tag preserved in frame */
}

pub const ERSPAN_V1_MDSIZE: c_int = 4;
pub const ERSPAN_V2_MDSIZE: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct erspan_base_hdr {

    pub vlan:8: __u8,
    pub session_id:8: __u8,

    pub vlan:8: __u8,
    pub session_id:8: __u8,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qtag_prefix {
    pub eth_type: __be16,
    pub tci: __be16,
    pub qp: *mut },
    pub 0: u16 vlan_tci =,
    pub tos: u8,
    pub idx: *mut __be32,
    pub 4): (ipv6_hdr(skb)->flow_lbl[0] >>,
    pub ERSPAN_ENCAP_NOVLAN: enc_type =,
// If mirrored packet has vlan tag, extract tci and
// preserve vlan header in the mirrored frame.
//
    pub ETH_ALEN): *mut *mut *mut qp = (struct qtag_prefix )(skb->data + 2,
    pub ntohs(qp->tci): vlan_tci =,
    pub ERSPAN_ENCAP_INFRAME: enc_type =,
    pub ERSPAN_V1_MDSIZE): *mut *mut skb_push(skb, sizeof(ershdr) +,
    pub )skb->data: *mut ershdr = (struct erspan_base_hdr,
    pub ERSPAN_V1_MDSIZE): *mut *mut memset(ershdr, 0, sizeof(ershdr) +,
// Build base header
    pub ERSPAN_VERSION: ershdr->ver =,
    pub tos_to_cos(tos): ershdr->cos =,
    pub enc_type: ershdr->en =,
    pub truncate: ershdr->t =,
    pub vlan_tci): set_vlan(ershdr,,
    pub id): set_session_id(ershdr,,
// Build metadata
    pub 1): *mut *mut idx = (__be32 )(ershdr +,
// idx = htonl(index & INDEX_MASK);
// ERSPAN GRA: timestamp granularity
// 00b --> granularity = 100 microseconds
// 01b --> granularity = 100 nanoseconds
// 10b --> granularity = IEEE 1588
// Here we only support 100 microseconds.
//
    pub h_usecs: u64,
    pub kt: ktime_t,
    pub ktime_get_real(): kt =,
    pub NSEC_PER_USEC): *mut *mut h_usecs = ktime_divns(kt, 100,
// ERSPAN base header only has 32-bit,
// so it wraps around 4 days.
//
    pub htonl((u32)h_usecs): return,
// ERSPAN BSO (Bad/Short/Oversized), see RFC1757
// 00b --> Good frame with no error, or unknown integrity
// 01b --> Payload is a Short Frame
// 10b --> Payload is an Oversized Frame
// 11b --> Payload is a Bad Frame with CRC or Alignment Error
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum erspan_bso {
    BSO_NOERROR = 0x0,
    BSO_SHORT = 0x1,
    BSO_OVERSIZED = 0x2,
    BSO_BAD = 0x3,
}

// BSO_BAD is not handled because the frame CRC
// or alignment error information is in FCS.
//
    pub BSO_SHORT: return,
    pub BSO_OVERSIZED: return,
    pub BSO_NOERROR: return,
    pub )skb->data: *mut *mut ethhdr eth = (ethhdr,
    pub ershdr: *mut erspan_base_hdr,
    pub md2: *mut erspan_md2,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qtag_prefix {
    pub eth_type: __be16,
    pub tci: __be16,
    pub qp: *mut },
    pub 0: u16 vlan_tci =,
    pub /: *mut *mut u8 gra = 0; / 100 usec,
    pub /: *mut *mut u8 bso = 0; / Bad/Short/Oversized,
    pub 0: u8 sgt =,
    pub tos: u8,
    pub 4): (ipv6_hdr(skb)->flow_lbl[0] >>,
// Unlike v1, v2 does not have En field,
// so only extract vlan tci field.
//
    pub ETH_ALEN): *mut *mut *mut qp = (struct qtag_prefix )(skb->data + 2,
    pub ntohs(qp->tci): vlan_tci =,
    pub erspan_detect_bso(skb): bso =,
    pub ERSPAN_V2_MDSIZE): *mut *mut skb_push(skb, sizeof(ershdr) +,
    pub )skb->data: *mut ershdr = (struct erspan_base_hdr,
    pub ERSPAN_V2_MDSIZE): *mut *mut memset(ershdr, 0, sizeof(ershdr) +,
// Build base header
    pub ERSPAN_VERSION2: ershdr->ver =,
    pub tos_to_cos(tos): ershdr->cos =,
    pub bso: ershdr->en =,
    pub truncate: ershdr->t =,
    pub vlan_tci): set_vlan(ershdr,,
    pub id): set_session_id(ershdr,,
// Build metadata
    pub 1): *mut *mut md2 = (struct erspan_md2 )(ershdr +,
    pub erspan_get_timestamp(): md2->timestamp =,
    pub htons(sgt): md2->sgt =,
    pub 1: md2->p =,
    pub 0: md2->ft =,
    pub direction: md2->dir =,
    pub gra: md2->gra =,
    pub 0: md2->o =,
    pub hwid): set_hwid(md2,,
