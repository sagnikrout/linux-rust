//! Automatically rewritten from C to Rust
//! Source: net/ipv4/xfrm4_state.c
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
// xfrm4_state.c
//
// Changes:
// YOSHIFUJI Hideaki @USAGI
// Split up af-specific portion
//

    static struct xfrm_state_afinfo xfrm4_state_afinfo = {
    .family			= AF_INET,
    .proto			= IPPROTO_IPIP,
    .output			= xfrm4_output,
    .transport_finish	= xfrm4_transport_finish,
    .local_error		= xfrm4_local_error,
    };
#[no_mangle]
pub unsafe extern "C" fn xfrm4_state_init() -> void __init {
    void __init xfrm4_state_init(void)
    {
    xfrm_state_register_afinfo(&xfrm4_state_afinfo);
    }
