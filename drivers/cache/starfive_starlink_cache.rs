//! Automatically rewritten from C to Rust
//! Source: drivers/cache/starfive_starlink_cache.c
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
// Cache Management Operations for StarFive's Starlink cache controller
//
// Copyright (C) 2024 Shanghai StarFive Technology Co., Ltd.
//
// Author: Joshua Yeong <joshua.yeong@starfivetech.com>
//

pub const STARLINK_CACHE_FLUSH_START_ADDR: c_uint = 0x0;
pub const STARLINK_CACHE_FLUSH_END_ADDR: c_uint = 0x8;
pub const STARLINK_CACHE_FLUSH_CTL: c_uint = 0x10;
pub const STARLINK_CACHE_ALIGN: c_uint = 0x40;

pub const STARLINK_CACHE_FLUSH_CTL_CLEAN_INVALIDATE: c_int = 0;
pub const STARLINK_CACHE_FLUSH_CTL_MAKE_INVALIDATE: c_int = 1;
pub const STARLINK_CACHE_FLUSH_CTL_CLEAN_SHARED: c_int = 2;
pub const STARLINK_CACHE_FLUSH_POLL_DELAY_US: c_int = 1;
pub const STARLINK_CACHE_FLUSH_TIMEOUT_US: c_int = 5000000;
    static void __iomem *starlink_cache_base;
#[no_mangle]
unsafe extern "C" fn starlink_cache_flush_complete() {
    static void starlink_cache_flush_complete(void)
    {
    volatile void __iomem *ctl = starlink_cache_base + STARLINK_CACHE_FLUSH_CTL;
    u64 v;
    int ret;
    ret = readq_poll_timeout_atomic(ctl, v, !(v & STARLINK_CACHE_FLUSH_CTL_ENABLE_MASK),
    STARLINK_CACHE_FLUSH_POLL_DELAY_US,
    STARLINK_CACHE_FLUSH_TIMEOUT_US);
    if (ret)
    WARN(1, "StarFive Starlink cache flush operation timeout\n");
    }
#[no_mangle]
unsafe extern "C" fn starlink_cache_dma_cache_wback(paddr: phys_addr_t, size: c_ulong) {
    static void starlink_cache_dma_cache_wback(phys_addr_t paddr, unsigned long size)
    {
    writeq(FIELD_PREP(STARLINK_CACHE_ADDRESS_RANGE_MASK, paddr),
    starlink_cache_base + STARLINK_CACHE_FLUSH_START_ADDR);
    writeq(FIELD_PREP(STARLINK_CACHE_ADDRESS_RANGE_MASK, paddr + size),
    starlink_cache_base + STARLINK_CACHE_FLUSH_END_ADDR);
    mb();
    writeq(FIELD_PREP(STARLINK_CACHE_FLUSH_CTL_MODE_MASK,
    STARLINK_CACHE_FLUSH_CTL_CLEAN_SHARED),
    starlink_cache_base + STARLINK_CACHE_FLUSH_CTL);
    starlink_cache_flush_complete();
    }
#[no_mangle]
unsafe extern "C" fn starlink_cache_dma_cache_invalidate(paddr: phys_addr_t, size: c_ulong) {
    static void starlink_cache_dma_cache_invalidate(phys_addr_t paddr, unsigned long size)
    {
    writeq(FIELD_PREP(STARLINK_CACHE_ADDRESS_RANGE_MASK, paddr),
    starlink_cache_base + STARLINK_CACHE_FLUSH_START_ADDR);
    writeq(FIELD_PREP(STARLINK_CACHE_ADDRESS_RANGE_MASK, paddr + size),
    starlink_cache_base + STARLINK_CACHE_FLUSH_END_ADDR);
    mb();
    writeq(FIELD_PREP(STARLINK_CACHE_FLUSH_CTL_MODE_MASK,
    STARLINK_CACHE_FLUSH_CTL_MAKE_INVALIDATE),
    starlink_cache_base + STARLINK_CACHE_FLUSH_CTL);
    starlink_cache_flush_complete();
    }
#[no_mangle]
unsafe extern "C" fn starlink_cache_dma_cache_wback_inv(paddr: phys_addr_t, size: c_ulong) {
    static void starlink_cache_dma_cache_wback_inv(phys_addr_t paddr, unsigned long size)
    {
    writeq(FIELD_PREP(STARLINK_CACHE_ADDRESS_RANGE_MASK, paddr),
    starlink_cache_base + STARLINK_CACHE_FLUSH_START_ADDR);
    writeq(FIELD_PREP(STARLINK_CACHE_ADDRESS_RANGE_MASK, paddr + size),
    starlink_cache_base + STARLINK_CACHE_FLUSH_END_ADDR);
    mb();
    writeq(FIELD_PREP(STARLINK_CACHE_FLUSH_CTL_MODE_MASK,
    STARLINK_CACHE_FLUSH_CTL_CLEAN_INVALIDATE),
    starlink_cache_base + STARLINK_CACHE_FLUSH_CTL);
    starlink_cache_flush_complete();
    }
    static const struct riscv_nonstd_cache_ops starlink_cache_ops = {
    .wback = &starlink_cache_dma_cache_wback,
    .inv = &starlink_cache_dma_cache_invalidate,
    .wback_inv = &starlink_cache_dma_cache_wback_inv,
    };
    static const struct of_device_id starlink_cache_ids[] = {
    { .compatible = "starfive,jh8100-starlink-cache" },
    { /* sentinel */ }
    };
#[no_mangle]
unsafe extern "C" fn starlink_cache_init() -> int __init {
    static int __init starlink_cache_init(void)
    {
    u32 block_size;
    int ret;
    struct device_node *np __free(device_node) =
    of_find_matching_node(core::ptr::null_mut(), starlink_cache_ids);
    if (!of_device_is_available(np))
    return -ENODEV;
    ret = of_property_read_u32(np, "cache-block-size", &block_size);
    if (ret)
    return ret;
    if (block_size % STARLINK_CACHE_ALIGN)
    return -EINVAL;
    starlink_cache_base = of_iomap(np, 0);
    if (!starlink_cache_base)
    return -ENOMEM;
    riscv_cbom_block_size = block_size;
    riscv_noncoherent_supported();
    riscv_noncoherent_register_cache_ops(&starlink_cache_ops);
    return 0;
    }
    arch_initcall(starlink_cache_init);
