//! Automatically rewritten from C Header to Rust Module
//! Source: crypto/ecrdsa_defs.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// Definitions of EC-RDSA Curve Parameters
//
// Copyright (c) 2019 Vitaly Chikunov <vt@altlinux.org>
//
// This program is free software; you can redistribute it and/or modify it
// under the terms of the GNU General Public License as published by the Free
// Software Foundation; either version 2 of the License, or (at your option)
// any later version.
//

//
// EC-RDSA uses its own set of curves.
//
// cp256{a,b,c} curves first defined for GOST R 34.10-2001 in RFC 4357 (as
// 256-bit {A,B,C}-ParamSet), but inherited for GOST R 34.10-2012 and
// proposed for use in R 50.1.114-2016 and RFC 7836 as the 256-bit curves.
//
// OID_gostCPSignA 1.2.643.2.2.35.1
// OID_gostCPSignB 1.2.643.2.2.35.2
// OID_gostCPSignC 1.2.643.2.2.35.3
// pre-computed value for Barrett's reduction
// tc512{a,b} curves first recommended in 2013 and then standardized in
// R 50.1.114-2016 and RFC 7836 for use with GOST R 34.10-2012 (as TC26
// 512-bit ParamSet{A,B}).
//
// OID_gostTC26Sign512A 1.2.643.7.1.2.1.2.1
// OID_gostTC26Sign512B 1.2.643.7.1.2.1.2.2
