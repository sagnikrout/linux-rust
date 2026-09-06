//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/nsh.h
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


pub const __NET_NSH_H: c_int = 1;

//
// Network Service Header:
// 0                   1                   2                   3
// 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |Ver|O|U|    TTL    |   Length  |U|U|U|U|MD Type| Next Protocol |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |          Service Path Identifier (SPI)        | Service Index |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |                                                               |
// ~               Mandatory/Optional Context Headers              ~
// |                                                               |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//
// Version: The version field is used to ensure backward compatibility
// going forward with future NSH specification updates.  It MUST be set
// to 0x0 by the sender, in this first revision of NSH.  Given the
// widespread implementation of existing hardware that uses the first
// nibble after an MPLS label stack for ECMP decision processing, this
// document reserves version 01b and this value MUST NOT be used in
// future versions of the protocol.  Please see [RFC7325] for further
// discussion of MPLS-related forwarding requirements.
//
// O bit: Setting this bit indicates an Operations, Administration, and
// Maintenance (OAM) packet.  The actual format and processing of SFC
// OAM packets is outside the scope of this specification (see for
// example [I-D.ietf-sfc-oam-framework] for one approach).
//
// The O bit MUST be set for OAM packets and MUST NOT be set for non-OAM
// packets.  The O bit MUST NOT be modified along the SFP.
//
// SF/SFF/SFC Proxy/Classifier implementations that do not support SFC
// OAM procedures SHOULD discard packets with O bit set, but MAY support
// a configurable parameter to enable forwarding received SFC OAM
// packets unmodified to the next element in the chain.  Forwarding OAM
// packets unmodified by SFC elements that do not support SFC OAM
// procedures may be acceptable for a subset of OAM functions, but can
// result in unexpected outcomes for others, thus it is recommended to
// analyze the impact of forwarding an OAM packet for all OAM functions
// prior to enabling this behavior.  The configurable parameter MUST be
// disabled by default.
//
// TTL: Indicates the maximum SFF hops for an SFP.  This field is used
// for service plane loop detection.  The initial TTL value SHOULD be
// configurable via the control plane; the configured initial value can
// be specific to one or more SFPs.  If no initial value is explicitly
// provided, the default initial TTL value of 63 MUST be used.  Each SFF
// involved in forwarding an NSH packet MUST decrement the TTL value by
// 1 prior to NSH forwarding lookup.  Decrementing by 1 from an incoming
// value of 0 shall result in a TTL value of 63.  The packet MUST NOT be
// forwarded if TTL is, after decrement, 0.
//
// All other flag fields, marked U, are unassigned and available for
// future use, see Section 11.2.1.  Unassigned bits MUST be set to zero
// upon origination, and MUST be ignored and preserved unmodified by
// other NSH supporting elements.  Elements which do not understand the
// meaning of any of these bits MUST NOT modify their actions based on
// those unknown bits.
//
// Length: The total length, in 4-byte words, of NSH including the Base
// Header, the Service Path Header, the Fixed Length Context Header or
// Variable Length Context Header(s).  The length MUST be 0x6 for MD
// Type equal to 0x1, and MUST be 0x2 or greater for MD Type equal to
// 0x2.  The length of the NSH header MUST be an integer multiple of 4
// bytes, thus variable length metadata is always padded out to a
// multiple of 4 bytes.
//
// MD Type: Indicates the format of NSH beyond the mandatory Base Header
// and the Service Path Header.  MD Type defines the format of the
// metadata being carried.
//
// 0x0 - This is a reserved value.  Implementations SHOULD silently
// discard packets with MD Type 0x0.
//
// 0x1 - This indicates that the format of the header includes a fixed
// length Context Header (see Figure 4 below).
//
// 0x2 - This does not mandate any headers beyond the Base Header and
// Service Path Header, but may contain optional variable length Context
// Header(s).  The semantics of the variable length Context Header(s)
// are not defined in this document.  The format of the optional
// variable length Context Headers is provided in Section 2.5.1.
//
// 0xF - This value is reserved for experimentation and testing, as per
// [RFC3692].  Implementations not explicitly configured to be part of
// an experiment SHOULD silently discard packets with MD Type 0xF.
//
// Next Protocol: indicates the protocol type of the encapsulated data.
// NSH does not alter the inner payload, and the semantics on the inner
// protocol remain unchanged due to NSH service function chaining.
// Please see the IANA Considerations section below, Section 11.2.5.
//
// This document defines the following Next Protocol values:
//
// 0x1: IPv4
// 0x2: IPv6
// 0x3: Ethernet
// 0x4: NSH
// 0x5: MPLS
// 0xFE: Experiment 1
// 0xFF: Experiment 2
//
// Packets with Next Protocol values not supported SHOULD be silently
// dropped by default, although an implementation MAY provide a
// configuration parameter to forward them.  Additionally, an
// implementation not explicitly configured for a specific experiment
// [RFC3692] SHOULD silently drop packets with Next Protocol values 0xFE
// and 0xFF.
//
// Service Path Identifier (SPI): Identifies a service path.
// Participating nodes MUST use this identifier for Service Function
// Path selection.  The initial classifier MUST set the appropriate SPI
// for a given classification result.
//
// Service Index (SI): Provides location within the SFP.  The initial
// classifier for a given SFP SHOULD set the SI to 255, however the
// control plane MAY configure the initial value of SI as appropriate
// (i.e., taking into account the length of the service function path).
// The Service Index MUST be decremented by a value of 1 by Service
// Functions or by SFC Proxy nodes after performing required services
// and the new decremented SI value MUST be used in the egress packet's
// NSH.  The initial Classifier MUST send the packet to the first SFF in
// the identified SFP for forwarding along an SFP.  If re-classification
// occurs, and that re-classification results in a new SPI, the
// (re)classifier is, in effect, the initial classifier for the
// resultant SPI.
//
// The SI is used in conjunction the with Service Path Identifier for
// Service Function Path Selection and for determining the next SFF/SF
// in the path.  The SI is also valuable when troubleshooting or
// reporting service paths.  Additionally, while the TTL field is the
// main mechanism for service plane loop detection, the SI can also be
// used for detecting service plane loops.
//
// When the Base Header specifies MD Type = 0x1, a Fixed Length Context
// Header (16-bytes) MUST be present immediately following the Service
// Path Header. The value of a Fixed Length Context
// Header that carries no metadata MUST be set to zero.
//
// When the base header specifies MD Type = 0x2, zero or more Variable
// Length Context Headers MAY be added, immediately following the
// Service Path Header (see Figure 5).  Therefore, Length = 0x2,
// indicates that only the Base Header followed by the Service Path
// Header are present.  The optional Variable Length Context Headers
// MUST be of an integer number of 4-bytes.  The base header Length
// field MUST be used to determine the offset to locate the original
// packet or frame for SFC nodes that require access to that
// information.
//
// The format of the optional variable length Context Headers
//
// 0                   1                   2                   3
// 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |          Metadata Class       |      Type     |U|    Length   |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |                      Variable Metadata                        |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//
// Metadata Class (MD Class): Defines the scope of the 'Type' field to
// provide a hierarchical namespace.  The IANA Considerations
// Section 11.2.4 defines how the MD Class values can be allocated to
// standards bodies, vendors, and others.
//
// Type: Indicates the explicit type of metadata being carried.  The
// definition of the Type is the responsibility of the MD Class owner.
//
// Unassigned bit: One unassigned bit is available for future use. This
// bit MUST NOT be set, and MUST be ignored on receipt.
//
// Length: Indicates the length of the variable metadata, in bytes.  In
// case the metadata length is not an integer number of 4-byte words,
// the sender MUST add pad bytes immediately following the last metadata
// byte to extend the metadata to an integer number of 4-byte words.
// The receiver MUST round up the length field to the nearest 4-byte
// word boundary, to locate and process the next field in the packet.
// The receiver MUST access only those bytes in the metadata indicated
// by the length field (i.e., actual number of bytes) and MUST ignore
// the remaining bytes up to the nearest 4-byte word boundary.  The
// Length may be 0 or greater.
//
// A value of 0 denotes a Context Header without a Variable Metadata
// field.
//
// [0] https://datatracker.ietf.org/doc/draft-ietf-sfc-nsh
//
// struct nsh_md1_ctx - Keeps track of NSH context data
// @context: NSH Contexts.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nsh_md1_ctx {
    pub context: [__be32; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nsh_md2_tlv {
    pub md_class: __be16,
    pub type: u8,
    pub length: u8,
    pub md_value: [u8; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nshhdr {
    pub ver_flags_ttl_len: __be16,
    pub mdtype: u8,
    pub np: u8,
    pub path_hdr: __be32,
    pub md1: nsh_md1_ctx,
    pub md2: nsh_md2_tlv,
}

// Masking NSH header fields.
pub const NSH_VER_MASK: c_uint = 0xc000;
pub const NSH_VER_SHIFT: c_int = 14;
pub const NSH_FLAGS_MASK: c_uint = 0x3000;
pub const NSH_FLAGS_SHIFT: c_int = 12;
pub const NSH_TTL_MASK: c_uint = 0x0fc0;
pub const NSH_TTL_SHIFT: c_int = 6;
pub const NSH_LEN_MASK: c_uint = 0x003f;
pub const NSH_LEN_SHIFT: c_int = 0;
pub const NSH_MDTYPE_MASK: c_uint = 0x0f;
pub const NSH_MDTYPE_SHIFT: c_int = 0;
pub const NSH_SPI_MASK: c_uint = 0xffffff00;
pub const NSH_SPI_SHIFT: c_int = 8;
pub const NSH_SI_MASK: c_uint = 0x000000ff;
pub const NSH_SI_SHIFT: c_int = 0;
// MD Type Registry.
pub const NSH_M_TYPE1: c_uint = 0x01;
pub const NSH_M_TYPE2: c_uint = 0x02;
pub const NSH_M_EXP1: c_uint = 0xFE;
pub const NSH_M_EXP2: c_uint = 0xFF;
// NSH Base Header Length
pub const NSH_BASE_HDR_LEN: c_int = 8;
// NSH MD Type 1 header Length.
pub const NSH_M_TYPE1_LEN: c_int = 24;
// NSH header maximum Length.

// NSH context headers maximum Length.

extern "C" {
    pub fn nsh_push(skb: *mut sk_buff, pushed_nh: *const nshhdr) -> c_int;
}
extern "C" {
    pub fn nsh_pop(skb: *mut sk_buff) -> c_int;
}
