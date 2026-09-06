//! Automatically rewritten from C to Rust
//! Source: lib/raid/xor/s390/xor.c
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
// Optimized xor_block operation for RAID4/5
//
// Copyright IBM Corp. 2016
// Author(s): Martin Schwidefsky <schwidefsky@de.ibm.com>
//

    static void xor_xc_2(unsigned long bytes, unsigned long * __restrict p1,
    const unsigned long * __restrict p2)
    {
    asm volatile(
    "	aghi	%0,-1\n"
    "	jm	3f\n"
    "	srlg	0,%0,8\n"
    "	ltgr	0,0\n"
    "	jz	1f\n"
    "0:	xc	0(256,%1),0(%2)\n"
    "	la	%1,256(%1)\n"
    "	la	%2,256(%2)\n"
    "	brctg	0,0b\n"
    "1:	exrl	%0,2f\n"
    "	j	3f\n"
    "2:	xc	0(1,%1),0(%2)\n"
    "3:"
    : "+a" (bytes), "+a" (p1), "+a" (p2)
    : : "0", "cc", "memory");
    }
    static void xor_xc_3(unsigned long bytes, unsigned long * __restrict p1,
    const unsigned long * __restrict p2,
    const unsigned long * __restrict p3)
    {
    asm volatile(
    "	aghi	%0,-1\n"
    "	jm	4f\n"
    "	srlg	0,%0,8\n"
    "	ltgr	0,0\n"
    "	jz	1f\n"
    "0:	xc	0(256,%1),0(%2)\n"
    "	xc	0(256,%1),0(%3)\n"
    "	la	%1,256(%1)\n"
    "	la	%2,256(%2)\n"
    "	la	%3,256(%3)\n"
    "	brctg	0,0b\n"
    "1:	exrl	%0,2f\n"
    "	exrl	%0,3f\n"
    "	j	4f\n"
    "2:	xc	0(1,%1),0(%2)\n"
    "3:	xc	0(1,%1),0(%3)\n"
    "4:"
    : "+a" (bytes), "+a" (p1), "+a" (p2), "+a" (p3)
    : : "0", "cc", "memory");
    }
    static void xor_xc_4(unsigned long bytes, unsigned long * __restrict p1,
    const unsigned long * __restrict p2,
    const unsigned long * __restrict p3,
    const unsigned long * __restrict p4)
    {
    asm volatile(
    "	aghi	%0,-1\n"
    "	jm	5f\n"
    "	srlg	0,%0,8\n"
    "	ltgr	0,0\n"
    "	jz	1f\n"
    "0:	xc	0(256,%1),0(%2)\n"
    "	xc	0(256,%1),0(%3)\n"
    "	xc	0(256,%1),0(%4)\n"
    "	la	%1,256(%1)\n"
    "	la	%2,256(%2)\n"
    "	la	%3,256(%3)\n"
    "	la	%4,256(%4)\n"
    "	brctg	0,0b\n"
    "1:	exrl	%0,2f\n"
    "	exrl	%0,3f\n"
    "	exrl	%0,4f\n"
    "	j	5f\n"
    "2:	xc	0(1,%1),0(%2)\n"
    "3:	xc	0(1,%1),0(%3)\n"
    "4:	xc	0(1,%1),0(%4)\n"
    "5:"
    : "+a" (bytes), "+a" (p1), "+a" (p2), "+a" (p3), "+a" (p4)
    : : "0", "cc", "memory");
    }
    static void xor_xc_5(unsigned long bytes, unsigned long * __restrict p1,
    const unsigned long * __restrict p2,
    const unsigned long * __restrict p3,
    const unsigned long * __restrict p4,
    const unsigned long * __restrict p5)
    {
    asm volatile(
    "	aghi	%0,-1\n"
    "	jm	6f\n"
    "	srlg	0,%0,8\n"
    "	ltgr	0,0\n"
    "	jz	1f\n"
    "0:	xc	0(256,%1),0(%2)\n"
    "	xc	0(256,%1),0(%3)\n"
    "	xc	0(256,%1),0(%4)\n"
    "	xc	0(256,%1),0(%5)\n"
    "	la	%1,256(%1)\n"
    "	la	%2,256(%2)\n"
    "	la	%3,256(%3)\n"
    "	la	%4,256(%4)\n"
    "	la	%5,256(%5)\n"
    "	brctg	0,0b\n"
    "1:	exrl	%0,2f\n"
    "	exrl	%0,3f\n"
    "	exrl	%0,4f\n"
    "	exrl	%0,5f\n"
    "	j	6f\n"
    "2:	xc	0(1,%1),0(%2)\n"
    "3:	xc	0(1,%1),0(%3)\n"
    "4:	xc	0(1,%1),0(%4)\n"
    "5:	xc	0(1,%1),0(%5)\n"
    "6:"
    : "+a" (bytes), "+a" (p1), "+a" (p2), "+a" (p3), "+a" (p4),
    "+a" (p5)
    : : "0", "cc", "memory");
    }
    DO_XOR_BLOCKS(xc, xor_xc_2, xor_xc_3, xor_xc_4, xor_xc_5);
    struct xor_block_template xor_block_xc = {
    .name		= "xc",
    .xor_gen	= xor_gen_xc,
    };
