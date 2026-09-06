//! Automatically rewritten from C to Rust
//! Source: arch/x86/kernel/cpu/cpuid_0x2_table.c
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

    [_desc] = {				\
    .c_type = (_type),		\
    .c_size = (_size) / SZ_1K,	\
    }

    [_desc] = {				\
    .t_type = (_type),		\
    .entries = (_entries),		\
    }
    const struct leaf_0x2_table cpuid_0x2_table[256] = {
    CACHE_ENTRY(0x06, CACHE_L1_INST,	SZ_8K	),	/* 4-way set assoc, 32 byte line size */
    CACHE_ENTRY(0x08, CACHE_L1_INST,	SZ_16K	),	/* 4-way set assoc, 32 byte line size */
    CACHE_ENTRY(0x09, CACHE_L1_INST,	SZ_32K	),	/* 4-way set assoc, 64 byte line size */
    CACHE_ENTRY(0x0a, CACHE_L1_DATA,	SZ_8K	),	/* 2 way set assoc, 32 byte line size */
    CACHE_ENTRY(0x0c, CACHE_L1_DATA,	SZ_16K	),	/* 4-way set assoc, 32 byte line size */
    CACHE_ENTRY(0x0d, CACHE_L1_DATA,	SZ_16K	),	/* 4-way set assoc, 64 byte line size */
    CACHE_ENTRY(0x0e, CACHE_L1_DATA,	SZ_24K	),	/* 6-way set assoc, 64 byte line size */
    CACHE_ENTRY(0x21, CACHE_L2,		SZ_256K	),	/* 8-way set assoc, 64 byte line size */
    CACHE_ENTRY(0x22, CACHE_L3,		SZ_512K	),	/* 4-way set assoc, sectored cache, 64 byte line size */
    CACHE_ENTRY(0x23, CACHE_L3,		SZ_1M	),	/* 8-way set assoc, sectored cache, 64 byte line size */
    CACHE_ENTRY(0x25, CACHE_L3,		SZ_2M	),	/* 8-way set assoc, sectored cache, 64 byte line size */
    CACHE_ENTRY(0x29, CACHE_L3,		SZ_4M	),	/* 8-way set assoc, sectored cache, 64 byte line size */
    CACHE_ENTRY(0x2c, CACHE_L1_DATA,	SZ_32K	),	/* 8-way set assoc, 64 byte line size */
    CACHE_ENTRY(0x30, CACHE_L1_INST,	SZ_32K	),	/* 8-way set assoc, 64 byte line size */
    CACHE_ENTRY(0x39, CACHE_L2,		SZ_128K	),	/* 4-way set assoc, sectored cache, 64 byte line size */
    CACHE_ENTRY(0x3a, CACHE_L2,		SZ_192K	),	/* 6-way set assoc, sectored cache, 64 byte line size */
    CACHE_ENTRY(0x3b, CACHE_L2,		SZ_128K	),	/* 2-way set assoc, sectored cache, 64 byte line size */
    CACHE_ENTRY(0x3c, CACHE_L2,		SZ_256K	),	/* 4-way set assoc, sectored cache, 64 byte line size */
    CACHE_ENTRY(0x3d, CACHE_L2,		SZ_384K	),	/* 6-way set assoc, sectored cache, 64 byte line size */
    CACHE_ENTRY(0x3e, CACHE_L2,		SZ_512K	),	/* 4-way set assoc, sectored cache, 64 byte line size */
    CACHE_ENTRY(0x3f, CACHE_L2,		SZ_256K	),	/* 2-way set assoc, 64 byte line size */
    CACHE_ENTRY(0x41, CACHE_L2,		SZ_128K	),	/* 4-way set assoc, 32 byte line size */
    CACHE_ENTRY(0x42, CACHE_L2,		SZ_256K	),	/* 4-way set assoc, 32 byte line size */
    CACHE_ENTRY(0x43, CACHE_L2,		SZ_512K	),	/* 4-way set assoc, 32 byte line size */
    CACHE_ENTRY(0x44, CACHE_L2,		SZ_1M	),	/* 4-way set assoc, 32 byte line size */
    CACHE_ENTRY(0x45, CACHE_L2,		SZ_2M	),	/* 4-way set assoc, 32 byte line size */
    CACHE_ENTRY(0x46, CACHE_L3,		SZ_4M	),	/* 4-way set assoc, 64 byte line size */
    CACHE_ENTRY(0x47, CACHE_L3,		SZ_8M	),	/* 8-way set assoc, 64 byte line size */
    CACHE_ENTRY(0x48, CACHE_L2,		SZ_3M	),	/* 12-way set assoc, 64 byte line size */
    CACHE_ENTRY(0x49, CACHE_L3,		SZ_4M	),	/* 16-way set assoc, 64 byte line size */
    CACHE_ENTRY(0x4a, CACHE_L3,		SZ_6M	),	/* 12-way set assoc, 64 byte line size */
    CACHE_ENTRY(0x4b, CACHE_L3,		SZ_8M	),	/* 16-way set assoc, 64 byte line size */
    CACHE_ENTRY(0x4c, CACHE_L3,		SZ_12M	),	/* 12-way set assoc, 64 byte line size */
    CACHE_ENTRY(0x4d, CACHE_L3,		SZ_16M	),	/* 16-way set assoc, 64 byte line size */
    CACHE_ENTRY(0x4e, CACHE_L2,		SZ_6M	),	/* 24-way set assoc, 64 byte line size */
    CACHE_ENTRY(0x60, CACHE_L1_DATA,	SZ_16K	),	/* 8-way set assoc, sectored cache, 64 byte line size */
    CACHE_ENTRY(0x66, CACHE_L1_DATA,	SZ_8K	),	/* 4-way set assoc, sectored cache, 64 byte line size */
    CACHE_ENTRY(0x67, CACHE_L1_DATA,	SZ_16K	),	/* 4-way set assoc, sectored cache, 64 byte line size */
    CACHE_ENTRY(0x68, CACHE_L1_DATA,	SZ_32K	),	/* 4-way set assoc, sectored cache, 64 byte line size */
    CACHE_ENTRY(0x78, CACHE_L2,		SZ_1M	),	/* 4-way set assoc, 64 byte line size */
    CACHE_ENTRY(0x79, CACHE_L2,		SZ_128K	),	/* 8-way set assoc, sectored cache, 64 byte line size */
    CACHE_ENTRY(0x7a, CACHE_L2,		SZ_256K	),	/* 8-way set assoc, sectored cache, 64 byte line size */
    CACHE_ENTRY(0x7b, CACHE_L2,		SZ_512K	),	/* 8-way set assoc, sectored cache, 64 byte line size */
    CACHE_ENTRY(0x7c, CACHE_L2,		SZ_1M	),	/* 8-way set assoc, sectored cache, 64 byte line size */
    CACHE_ENTRY(0x7d, CACHE_L2,		SZ_2M	),	/* 8-way set assoc, 64 byte line size */
    CACHE_ENTRY(0x7f, CACHE_L2,		SZ_512K	),	/* 2-way set assoc, 64 byte line size */
    CACHE_ENTRY(0x80, CACHE_L2,		SZ_512K	),	/* 8-way set assoc, 64 byte line size */
    CACHE_ENTRY(0x82, CACHE_L2,		SZ_256K	),	/* 8-way set assoc, 32 byte line size */
    CACHE_ENTRY(0x83, CACHE_L2,		SZ_512K	),	/* 8-way set assoc, 32 byte line size */
    CACHE_ENTRY(0x84, CACHE_L2,		SZ_1M	),	/* 8-way set assoc, 32 byte line size */
    CACHE_ENTRY(0x85, CACHE_L2,		SZ_2M	),	/* 8-way set assoc, 32 byte line size */
    CACHE_ENTRY(0x86, CACHE_L2,		SZ_512K	),	/* 4-way set assoc, 64 byte line size */
    CACHE_ENTRY(0x87, CACHE_L2,		SZ_1M	),	/* 8-way set assoc, 64 byte line size */
    CACHE_ENTRY(0xd0, CACHE_L3,		SZ_512K	),	/* 4-way set assoc, 64 byte line size */
    CACHE_ENTRY(0xd1, CACHE_L3,		SZ_1M	),	/* 4-way set assoc, 64 byte line size */
    CACHE_ENTRY(0xd2, CACHE_L3,		SZ_2M	),	/* 4-way set assoc, 64 byte line size */
    CACHE_ENTRY(0xd6, CACHE_L3,		SZ_1M	),	/* 8-way set assoc, 64 byte line size */
    CACHE_ENTRY(0xd7, CACHE_L3,		SZ_2M	),	/* 8-way set assoc, 64 byte line size */
    CACHE_ENTRY(0xd8, CACHE_L3,		SZ_4M	),	/* 12-way set assoc, 64 byte line size */
    CACHE_ENTRY(0xdc, CACHE_L3,		SZ_2M	),	/* 12-way set assoc, 64 byte line size */
    CACHE_ENTRY(0xdd, CACHE_L3,		SZ_4M	),	/* 12-way set assoc, 64 byte line size */
    CACHE_ENTRY(0xde, CACHE_L3,		SZ_8M	),	/* 12-way set assoc, 64 byte line size */
    CACHE_ENTRY(0xe2, CACHE_L3,		SZ_2M	),	/* 16-way set assoc, 64 byte line size */
    CACHE_ENTRY(0xe3, CACHE_L3,		SZ_4M	),	/* 16-way set assoc, 64 byte line size */
    CACHE_ENTRY(0xe4, CACHE_L3,		SZ_8M	),	/* 16-way set assoc, 64 byte line size */
    CACHE_ENTRY(0xea, CACHE_L3,		SZ_12M	),	/* 24-way set assoc, 64 byte line size */
    CACHE_ENTRY(0xeb, CACHE_L3,		SZ_18M	),	/* 24-way set assoc, 64 byte line size */
    CACHE_ENTRY(0xec, CACHE_L3,		SZ_24M	),	/* 24-way set assoc, 64 byte line size */
    TLB_ENTRY(  0x01, TLB_INST_4K,		32	),	/* TLB_INST 4 KByte pages, 4-way set associative */
    TLB_ENTRY(  0x02, TLB_INST_4M,		2	),	/* TLB_INST 4 MByte pages, full associative */
    TLB_ENTRY(  0x03, TLB_DATA_4K,		64	),	/* TLB_DATA 4 KByte pages, 4-way set associative */
    TLB_ENTRY(  0x04, TLB_DATA_4M,		8	),	/* TLB_DATA 4 MByte pages, 4-way set associative */
    TLB_ENTRY(  0x05, TLB_DATA_4M,		32	),	/* TLB_DATA 4 MByte pages, 4-way set associative */
    TLB_ENTRY(  0x0b, TLB_INST_4M,		4	),	/* TLB_INST 4 MByte pages, 4-way set associative */
    TLB_ENTRY(  0x4f, TLB_INST_4K,		32	),	/* TLB_INST 4 KByte pages */
    TLB_ENTRY(  0x50, TLB_INST_ALL,		64	),	/* TLB_INST 4 KByte and 2-MByte or 4-MByte pages */
    TLB_ENTRY(  0x51, TLB_INST_ALL,		128	),	/* TLB_INST 4 KByte and 2-MByte or 4-MByte pages */
    TLB_ENTRY(  0x52, TLB_INST_ALL,		256	),	/* TLB_INST 4 KByte and 2-MByte or 4-MByte pages */
    TLB_ENTRY(  0x55, TLB_INST_2M_4M,	7	),	/* TLB_INST 2-MByte or 4-MByte pages, fully associative */
    TLB_ENTRY(  0x56, TLB_DATA0_4M,		16	),	/* TLB_DATA0 4 MByte pages, 4-way set associative */
    TLB_ENTRY(  0x57, TLB_DATA0_4K,		16	),	/* TLB_DATA0 4 KByte pages, 4-way associative */
    TLB_ENTRY(  0x59, TLB_DATA0_4K,		16	),	/* TLB_DATA0 4 KByte pages, fully associative */
    TLB_ENTRY(  0x5a, TLB_DATA0_2M_4M,	32	),	/* TLB_DATA0 2-MByte or 4 MByte pages, 4-way set associative */
    TLB_ENTRY(  0x5b, TLB_DATA_4K_4M,	64	),	/* TLB_DATA 4 KByte and 4 MByte pages */
    TLB_ENTRY(  0x5c, TLB_DATA_4K_4M,	128	),	/* TLB_DATA 4 KByte and 4 MByte pages */
    TLB_ENTRY(  0x5d, TLB_DATA_4K_4M,	256	),	/* TLB_DATA 4 KByte and 4 MByte pages */
    TLB_ENTRY(  0x61, TLB_INST_4K,		48	),	/* TLB_INST 4 KByte pages, full associative */
    TLB_ENTRY(  0x63, TLB_DATA_1G_2M_4M,	4	),	/* TLB_DATA 1 GByte pages, 4-way set associative
// (plus 32 entries TLB_DATA 2 MByte or 4 MByte pages, not encoded here)
    TLB_ENTRY(  0x6b, TLB_DATA_4K,		256	),	/* TLB_DATA 4 KByte pages, 8-way associative */
    TLB_ENTRY(  0x6c, TLB_DATA_2M_4M,	128	),	/* TLB_DATA 2 MByte or 4 MByte pages, 8-way associative */
    TLB_ENTRY(  0x6d, TLB_DATA_1G,		16	),	/* TLB_DATA 1 GByte pages, fully associative */
    TLB_ENTRY(  0x76, TLB_INST_2M_4M,	8	),	/* TLB_INST 2-MByte or 4-MByte pages, fully associative */
    TLB_ENTRY(  0xb0, TLB_INST_4K,		128	),	/* TLB_INST 4 KByte pages, 4-way set associative */
    TLB_ENTRY(  0xb1, TLB_INST_2M_4M,	4	),	/* TLB_INST 2M pages, 4-way, 8 entries or 4M pages, 4-way entries */
    TLB_ENTRY(  0xb2, TLB_INST_4K,		64	),	/* TLB_INST 4KByte pages, 4-way set associative */
    TLB_ENTRY(  0xb3, TLB_DATA_4K,		128	),	/* TLB_DATA 4 KByte pages, 4-way set associative */
    TLB_ENTRY(  0xb4, TLB_DATA_4K,		256	),	/* TLB_DATA 4 KByte pages, 4-way associative */
    TLB_ENTRY(  0xb5, TLB_INST_4K,		64	),	/* TLB_INST 4 KByte pages, 8-way set associative */
    TLB_ENTRY(  0xb6, TLB_INST_4K,		128	),	/* TLB_INST 4 KByte pages, 8-way set associative */
    TLB_ENTRY(  0xba, TLB_DATA_4K,		64	),	/* TLB_DATA 4 KByte pages, 4-way associative */
    TLB_ENTRY(  0xc0, TLB_DATA_4K_4M,	8	),	/* TLB_DATA 4 KByte and 4 MByte pages, 4-way associative */
    TLB_ENTRY(  0xc1, STLB_4K_2M,		1024	),	/* STLB 4 KByte and 2 MByte pages, 8-way associative */
    TLB_ENTRY(  0xc2, TLB_DATA_2M_4M,	16	),	/* TLB_DATA 2 MByte/4MByte pages, 4-way associative */
    TLB_ENTRY(  0xca, STLB_4K,		512	),	/* STLB 4 KByte pages, 4-way associative */
    };
