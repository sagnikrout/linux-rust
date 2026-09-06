//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/libc-compat.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note
//
// Compatibility interface for userspace libc header coordination:
//
// Define compatibility macros that are used to control the inclusion or
// exclusion of UAPI structures and definitions in coordination with another
// userspace C library.
//
// This header is intended to solve the problem of UAPI definitions that
// conflict with userspace definitions. If a UAPI header has such conflicting
// definitions then the solution is as follows:
//
// * Synchronize the UAPI header and the libc headers so either one can be
// used and such that the ABI is preserved. If this is not possible then
// no simple compatibility interface exists (you need to write translating
// wrappers and rename things) and you can't use this interface.
//
// Then follow this process:
//
// (a) Include libc-compat.h in the UAPI header.
// e.g. #include <linux/libc-compat.h>
// This include must be as early as possible.
//
// (b) In libc-compat.h add enough code to detect that the comflicting
// userspace libc header has been included first.
//
// (c) If the userspace libc header has been included first define a set of
// guard macros of the form __UAPI_DEF_FOO and set their values to 1, else
// set their values to 0.
//
// (d) Back in the UAPI header with the conflicting definitions, guard the
// definitions with:
// #if __UAPI_DEF_FOO
// ...
// #endif
//
// This fixes the situation where the linux headers are included *after* the
// libc headers. To fix the problem with the inclusion in the other order the
// userspace libc headers must be fixed like this:
//
// * For all definitions that conflict with kernel definitions wrap those
// defines in the following:
// #if !__UAPI_DEF_FOO
// ...
// #endif
//
// This prevents the redefinition of a construct already defined by the kernel.
//
// We have included glibc headers...

// Coordinate with glibc net/if.h header.

// GLIBC headers included first so don't define anything
// that would already be defined.
pub const __UAPI_DEF_IF_IFCONF: c_int = 0;
pub const __UAPI_DEF_IF_IFMAP: c_int = 0;
pub const __UAPI_DEF_IF_IFNAMSIZ: c_int = 0;
pub const __UAPI_DEF_IF_IFREQ: c_int = 0;
// Everything up to IFF_DYNAMIC, matches net/if.h until glibc 2.23
pub const __UAPI_DEF_IF_NET_DEVICE_FLAGS: c_int = 0;
// For the future if glibc adds IFF_LOWER_UP, IFF_DORMANT and IFF_ECHO
pub const __UAPI_DEF_IF_NET_DEVICE_FLAGS_LOWER_UP_DORMANT_ECHO: c_int = 1;

// Linux headers included first, and we must define everything
// we need. The expectation is that glibc will check the
// __UAPI_DEF_* defines and adjust appropriately.
pub const __UAPI_DEF_IF_IFCONF: c_int = 1;
pub const __UAPI_DEF_IF_IFMAP: c_int = 1;
pub const __UAPI_DEF_IF_IFNAMSIZ: c_int = 1;
pub const __UAPI_DEF_IF_IFREQ: c_int = 1;
// Everything up to IFF_DYNAMIC, matches net/if.h until glibc 2.23
pub const __UAPI_DEF_IF_NET_DEVICE_FLAGS: c_int = 1;
// For the future if glibc adds IFF_LOWER_UP, IFF_DORMANT and IFF_ECHO
pub const __UAPI_DEF_IF_NET_DEVICE_FLAGS_LOWER_UP_DORMANT_ECHO: c_int = 1;

// Coordinate with glibc netinet/in.h header.

// GLIBC headers included first so don't define anything
// that would already be defined.
pub const __UAPI_DEF_IN_ADDR: c_int = 0;
pub const __UAPI_DEF_IN_IPPROTO: c_int = 0;
pub const __UAPI_DEF_IN_PKTINFO: c_int = 0;
pub const __UAPI_DEF_IP_MREQ: c_int = 0;
pub const __UAPI_DEF_SOCKADDR_IN: c_int = 0;
pub const __UAPI_DEF_IN_CLASS: c_int = 0;
pub const __UAPI_DEF_IN6_ADDR: c_int = 0;
// The exception is the in6_addr macros which must be defined
// if the glibc code didn't define them. This guard matches
// the guard in glibc/inet/netinet/in.h which defines the
// additional in6_addr macros e.g. s6_addr16, and s6_addr32.

pub const __UAPI_DEF_IN6_ADDR_ALT: c_int = 0;

pub const __UAPI_DEF_IN6_ADDR_ALT: c_int = 1;

pub const __UAPI_DEF_SOCKADDR_IN6: c_int = 0;
pub const __UAPI_DEF_IPV6_MREQ: c_int = 0;
pub const __UAPI_DEF_IPPROTO_V6: c_int = 0;
pub const __UAPI_DEF_IPV6_OPTIONS: c_int = 0;
pub const __UAPI_DEF_IN6_PKTINFO: c_int = 0;
pub const __UAPI_DEF_IP6_MTUINFO: c_int = 0;

// Linux headers included first, and we must define everything
// we need. The expectation is that glibc will check the
// __UAPI_DEF_* defines and adjust appropriately.
pub const __UAPI_DEF_IN_ADDR: c_int = 1;
pub const __UAPI_DEF_IN_IPPROTO: c_int = 1;
pub const __UAPI_DEF_IN_PKTINFO: c_int = 1;
pub const __UAPI_DEF_IP_MREQ: c_int = 1;
pub const __UAPI_DEF_SOCKADDR_IN: c_int = 1;
pub const __UAPI_DEF_IN_CLASS: c_int = 1;
pub const __UAPI_DEF_IN6_ADDR: c_int = 1;
// We unconditionally define the in6_addr macros and glibc must
// coordinate.
pub const __UAPI_DEF_IN6_ADDR_ALT: c_int = 1;
pub const __UAPI_DEF_SOCKADDR_IN6: c_int = 1;
pub const __UAPI_DEF_IPV6_MREQ: c_int = 1;
pub const __UAPI_DEF_IPPROTO_V6: c_int = 1;
pub const __UAPI_DEF_IPV6_OPTIONS: c_int = 1;
pub const __UAPI_DEF_IN6_PKTINFO: c_int = 1;
pub const __UAPI_DEF_IP6_MTUINFO: c_int = 1;

// Definitions for xattr.h

pub const __UAPI_DEF_XATTR: c_int = 0;

pub const __UAPI_DEF_XATTR: c_int = 1;

// If we did not see any headers from any supported C libraries,
// or we are being included in the kernel, then define everything
// that we need. Check for previous __UAPI_* definitions to give
// unsupported C libraries a way to opt out of any kernel definition.

// Definitions for if.h
pub const __UAPI_DEF_IF_IFCONF: c_int = 1;

pub const __UAPI_DEF_IF_IFMAP: c_int = 1;

pub const __UAPI_DEF_IF_IFNAMSIZ: c_int = 1;

pub const __UAPI_DEF_IF_IFREQ: c_int = 1;

// Everything up to IFF_DYNAMIC, matches net/if.h until glibc 2.23
pub const __UAPI_DEF_IF_NET_DEVICE_FLAGS: c_int = 1;

// For the future if glibc adds IFF_LOWER_UP, IFF_DORMANT and IFF_ECHO
pub const __UAPI_DEF_IF_NET_DEVICE_FLAGS_LOWER_UP_DORMANT_ECHO: c_int = 1;

// Definitions for in.h
pub const __UAPI_DEF_IN_ADDR: c_int = 1;

pub const __UAPI_DEF_IN_IPPROTO: c_int = 1;

pub const __UAPI_DEF_IN_PKTINFO: c_int = 1;

pub const __UAPI_DEF_IP_MREQ: c_int = 1;

pub const __UAPI_DEF_SOCKADDR_IN: c_int = 1;

pub const __UAPI_DEF_IN_CLASS: c_int = 1;

// Definitions for in6.h
pub const __UAPI_DEF_IN6_ADDR: c_int = 1;

pub const __UAPI_DEF_IN6_ADDR_ALT: c_int = 1;

pub const __UAPI_DEF_SOCKADDR_IN6: c_int = 1;

pub const __UAPI_DEF_IPV6_MREQ: c_int = 1;

pub const __UAPI_DEF_IPPROTO_V6: c_int = 1;

pub const __UAPI_DEF_IPV6_OPTIONS: c_int = 1;

pub const __UAPI_DEF_IN6_PKTINFO: c_int = 1;

pub const __UAPI_DEF_IP6_MTUINFO: c_int = 1;

// Definitions for xattr.h
pub const __UAPI_DEF_XATTR: c_int = 1;

