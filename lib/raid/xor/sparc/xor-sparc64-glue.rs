//! Automatically rewritten from C to Rust
//! Source: lib/raid/xor/sparc/xor-sparc64-glue.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// High speed xor_block operation for RAID4/5 utilizing the
// UltraSparc Visual Instruction Set and Niagara block-init
// twin-load instructions.
//
// Copyright (C) 1997, 1999 Jakub Jelinek (jj@ultra.linux.cz)
// Copyright (C) 2006 David S. Miller <davem@davemloft.net>
//

    void xor_vis_2(unsigned long bytes, unsigned long * __restrict p1,
    const unsigned long * __restrict p2);
    void xor_vis_3(unsigned long bytes, unsigned long * __restrict p1,
    const unsigned long * __restrict p2,
    const unsigned long * __restrict p3);
    void xor_vis_4(unsigned long bytes, unsigned long * __restrict p1,
    const unsigned long * __restrict p2,
    const unsigned long * __restrict p3,
    const unsigned long * __restrict p4);
    void xor_vis_5(unsigned long bytes, unsigned long * __restrict p1,
    const unsigned long * __restrict p2,
    const unsigned long * __restrict p3,
    const unsigned long * __restrict p4,
    const unsigned long * __restrict p5);
// XXX Ugh, write cheetah versions... -DaveM
    DO_XOR_BLOCKS(vis, xor_vis_2, xor_vis_3, xor_vis_4, xor_vis_5);
    struct xor_block_template xor_block_VIS = {
    .name		= "VIS",
    .xor_gen	= xor_gen_vis,
    };
    void xor_niagara_2(unsigned long bytes, unsigned long * __restrict p1,
    const unsigned long * __restrict p2);
    void xor_niagara_3(unsigned long bytes, unsigned long * __restrict p1,
    const unsigned long * __restrict p2,
    const unsigned long * __restrict p3);
    void xor_niagara_4(unsigned long bytes, unsigned long * __restrict p1,
    const unsigned long * __restrict p2,
    const unsigned long * __restrict p3,
    const unsigned long * __restrict p4);
    void xor_niagara_5(unsigned long bytes, unsigned long * __restrict p1,
    const unsigned long * __restrict p2,
    const unsigned long * __restrict p3,
    const unsigned long * __restrict p4,
    const unsigned long * __restrict p5);
    DO_XOR_BLOCKS(niagara, xor_niagara_2, xor_niagara_3, xor_niagara_4,
    xor_niagara_5);
    struct xor_block_template xor_block_niagara = {
    .name		= "Niagara",
    .xor_gen	= xor_gen_niagara,
    };
