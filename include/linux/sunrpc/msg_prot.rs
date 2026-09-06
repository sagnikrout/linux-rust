//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/sunrpc/msg_prot.h
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
// linux/include/linux/sunrpc/msg_prot.h
//
// Copyright (C) 1996, Olaf Kirch <okir@monad.swb.de>
//
pub const RPC_VERSION: c_int = 2;
// spec defines authentication flavor as an unsigned 32 bit integer
pub type rpc_authflavor_t = u32;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rpc_auth_flavors {
    RPC_AUTH_NULL  = 0,
    RPC_AUTH_UNIX  = 1,
    RPC_AUTH_SHORT = 2,
    RPC_AUTH_DES   = 3,
    RPC_AUTH_KRB   = 4,
    RPC_AUTH_GSS   = 6,
    RPC_AUTH_TLS   = 7,
    RPC_AUTH_MAXFLAVOR = 8,
// pseudoflavors:
    RPC_AUTH_GSS_KRB5  = 390003,
    RPC_AUTH_GSS_KRB5I = 390004,
    RPC_AUTH_GSS_KRB5P = 390005,
    RPC_AUTH_GSS_LKEY  = 390006,
    RPC_AUTH_GSS_LKEYI = 390007,
    RPC_AUTH_GSS_LKEYP = 390008,
    RPC_AUTH_GSS_SPKM  = 390009,
    RPC_AUTH_GSS_SPKMI = 390010,
    RPC_AUTH_GSS_SPKMP = 390011,
}

// Maximum size (in octets) of the machinename in an AUTH_UNIX
// credential (per RFC 5531 Appendix A)
//

// Maximum size (in bytes) of an rpc credential or verifier

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rpc_msg_type {
    RPC_CALL = 0,
    RPC_REPLY = 1
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rpc_reply_stat {
    RPC_MSG_ACCEPTED = 0,
    RPC_MSG_DENIED = 1
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rpc_accept_stat {
    RPC_SUCCESS = 0,
    RPC_PROG_UNAVAIL = 1,
    RPC_PROG_MISMATCH = 2,
    RPC_PROC_UNAVAIL = 3,
    RPC_GARBAGE_ARGS = 4,
    RPC_SYSTEM_ERR = 5,
// internal use only
    RPC_DROP_REPLY = 60000,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rpc_reject_stat {
    RPC_MISMATCH = 0,
    RPC_AUTH_ERROR = 1
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rpc_auth_stat {
    RPC_AUTH_OK = 0,		/* success */
    RPC_AUTH_BADCRED = 1,		/* bad credential (seal broken) */
    RPC_AUTH_REJECTEDCRED = 2,	/* client must begin new session */
    RPC_AUTH_BADVERF = 3,		/* bad verifier (seal broken) */
    RPC_AUTH_REJECTEDVERF = 4,	/* verifier expired or replayed */
    RPC_AUTH_TOOWEAK = 5,		/* rejected for security reasons */
    RPC_AUTH_INVALIDRESP = 6,	/* bogus response verifier */
    RPC_AUTH_FAILED = 7,		/* reason unknown */
// RPCSEC_GSS errors
    RPCSEC_GSS_CREDPROBLEM = 13,	/* no credentials for user */
    RPCSEC_GSS_CTXPROBLEM = 14	/* problem with context */
}

pub const RPC_MAXNETNAMELEN: c_int = 256;
//
// From RFC 1831:
//
// "A record is composed of one or more record fragments.  A record
// fragment is a four-byte header followed by 0 to (2**31) - 1 bytes of
// fragment data.  The bytes encode an unsigned binary number; as with
// XDR integers, the byte order is from highest to lowest.  The number
// encodes two values -- a boolean which indicates whether the fragment
// is the last fragment of the record (bit value 1 implies the fragment
// is the last fragment) and a 31-bit unsigned binary value which is the
// length in bytes of the fragment's data.  The boolean value is the
// highest-order bit of the header; the length is the 31 low-order bits.
// (Note that this record specification is NOT in XDR standard form!)"
//
// The Linux RPC client always sends its requests in a single record
// fragment, limiting the maximum payload size for stream transports to
// 2GB.
//
pub type rpc_fraghdr = __be32;

//
// RPC call and reply header size as number of 32bit words (verifier
// size computed separately, see below)
//

//
// Maximum RPC header size, including authentication,
// as number of 32bit words (see RFCs 1831, 1832).
//
// xid			    1 xdr unit = 4 bytes
// mtype			    1
// rpc_version		    1
// program			    1
// prog_version		    1
// procedure		    1
// cred {
// flavor		    1
// length		    1
// body<RPC_MAX_AUTH_SIZE> 100 xdr units = 400 bytes
// }
// verf {
// flavor		    1
// length		    1
// body<RPC_MAX_AUTH_SIZE> 100 xdr units = 400 bytes
// }
// TOTAL			    210 xdr units = 840 bytes
//

//
// Well-known netids. See:
//
// https://www.iana.org/assignments/rpc-netids/rpc-netids.xhtml
//

//
// Note that RFC 1833 does not put any size restrictions on the
// netid string, but all currently defined netid's fit in 5 bytes.
//

//
// Universal addresses are introduced in RFC 1833 and further spelled
// out in RFC 3530.  RPCBIND_MAXUADDRLEN defines a maximum byte length
// of a universal address for use in allocating buffers and character
// arrays.
//
// Quoting RFC 3530, section 2.2:
//
// For TCP over IPv4 and for UDP over IPv4, the format of r_addr is the
// US-ASCII string:
//
// h1.h2.h3.h4.p1.p2
//
// The prefix, "h1.h2.h3.h4", is the standard textual form for
// representing an IPv4 address, which is always four octets long.
// Assuming big-endian ordering, h1, h2, h3, and h4, are respectively,
// the first through fourth octets each converted to ASCII-decimal.
// Assuming big-endian ordering, p1 and p2 are, respectively, the first
// and second octets each converted to ASCII-decimal.  For example, if a
// host, in big-endian order, has an address of 0x0A010307 and there is
// a service listening on, in big endian order, port 0x020F (decimal
// 527), then the complete universal address is "10.1.3.7.2.15".
//
// ...
//
// For TCP over IPv6 and for UDP over IPv6, the format of r_addr is the
// US-ASCII string:
//
// x1:x2:x3:x4:x5:x6:x7:x8.p1.p2
//
// The suffix "p1.p2" is the service port, and is computed the same way
// as with universal addresses for TCP and UDP over IPv4.  The prefix,
// "x1:x2:x3:x4:x5:x6:x7:x8", is the standard textual form for
// representing an IPv6 address as defined in Section 2.2 of [RFC2373].
// Additionally, the two alternative forms specified in Section 2.2 of
// [RFC2373] are also acceptable.
//

// Maximum size of the port number part of a universal address

// Maximum size of an IPv4 universal address

// Maximum size of an IPv6 universal address

// Assume INET6_ADDRSTRLEN will always be larger than INET_ADDRSTRLEN...

