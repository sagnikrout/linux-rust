//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/dscp.h
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
// Copyright (c) 2024 Pengutronix, Oleksij Rempel <kernel@pengutronix.de>
//
// DSCP Pools and Codepoint Space Division:
//
// The Differentiated Services (Diffserv) architecture defines a method for
// classifying and managing network traffic using the DS field in IPv4 and IPv6
// packet headers. This field can carry one of 64 distinct DSCP (Differentiated
// Services Code Point) values, which are divided into three pools based on
// their Least Significant Bits (LSB) patterns and intended usage. Each pool has
// a specific registration procedure for assigning DSCP values:
//
// Pool 1 (Standards Action Pool):
// - Codepoint Space: xxxxx0
// This pool includes DSCP values ending in '0' (binary), allocated via
// Standards Action. It is intended for globally recognized traffic classes,
// ensuring interoperability across the internet. This pool encompasses
// well-known DSCP values such as CS0-CS7, AFxx, EF, and VOICE-ADMIT.
//
// Pool 2 (Experimental/Local Use Pool):
// - Codepoint Space: xxxx11
// Reserved for DSCP values ending in '11' (binary), this pool is designated
// for Experimental or Local Use. It allows for private or temporary traffic
// marking schemes not intended for standardized global use, facilitating
// testing and network-specific configurations without impacting
// interoperability.
//
// Pool 3 (Preferential Standardization Pool):
// - Codepoint Space: xxxx01
// Initially reserved for experimental or local use, this pool now serves as
// a secondary standardization resource should Pool 1 become exhausted. DSCP
// values ending in '01' (binary) are assigned via Standards Action, with a
// focus on adopting new, standardized traffic classes as the need arises.
//
// For pool updates see:
// https://www.iana.org/assignments/dscp-registry/dscp-registry.xhtml
//
// Pool 1: Standardized DSCP values as per [RFC8126]

// CS0 is some times called default (DF)

// Pool 3: Standardized assignments, previously available for experimental/local
// use
//

pub const DSCP_MAX: c_int = 64;
