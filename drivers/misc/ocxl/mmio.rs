//! Automatically rewritten from C to Rust
//! Source: drivers/misc/ocxl/mmio.c
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
// Copyright 2019 IBM Corp.

    int ocxl_global_mmio_read32(struct ocxl_afu *afu, size_t offset,
    enum ocxl_endian endian, u32 *val)
    {
    if (offset > afu.config.global_mmio_size - 4)
    return -EINVAL;

    if (endian == OCXL_HOST_ENDIAN)
    endian = OCXL_BIG_ENDIAN;

    switch (endian) {
    case OCXL_BIG_ENDIAN:
// val = readl_be((char *)afu->global_mmio_ptr + offset);
    break;
    default:
// val = readl((char *)afu->global_mmio_ptr + offset);
    break;
    }
    return 0;
    }
    EXPORT_SYMBOL_GPL(ocxl_global_mmio_read32);
    int ocxl_global_mmio_read64(struct ocxl_afu *afu, size_t offset,
    enum ocxl_endian endian, u64 *val)
    {
    if (offset > afu.config.global_mmio_size - 8)
    return -EINVAL;

    if (endian == OCXL_HOST_ENDIAN)
    endian = OCXL_BIG_ENDIAN;

    switch (endian) {
    case OCXL_BIG_ENDIAN:
// val = readq_be((char *)afu->global_mmio_ptr + offset);
    break;
    default:
// val = readq((char *)afu->global_mmio_ptr + offset);
    break;
    }
    return 0;
    }
    EXPORT_SYMBOL_GPL(ocxl_global_mmio_read64);
    int ocxl_global_mmio_write32(struct ocxl_afu *afu, size_t offset,
    enum ocxl_endian endian, u32 val)
    {
    if (offset > afu.config.global_mmio_size - 4)
    return -EINVAL;

    if (endian == OCXL_HOST_ENDIAN)
    endian = OCXL_BIG_ENDIAN;

    switch (endian) {
    case OCXL_BIG_ENDIAN:
    writel_be(val, (char *)afu.global_mmio_ptr + offset);
    break;
    default:
    writel(val, (char *)afu.global_mmio_ptr + offset);
    break;
    }
    return 0;
    }
    EXPORT_SYMBOL_GPL(ocxl_global_mmio_write32);
    int ocxl_global_mmio_write64(struct ocxl_afu *afu, size_t offset,
    enum ocxl_endian endian, u64 val)
    {
    if (offset > afu.config.global_mmio_size - 8)
    return -EINVAL;

    if (endian == OCXL_HOST_ENDIAN)
    endian = OCXL_BIG_ENDIAN;

    switch (endian) {
    case OCXL_BIG_ENDIAN:
    writeq_be(val, (char *)afu.global_mmio_ptr + offset);
    break;
    default:
    writeq(val, (char *)afu.global_mmio_ptr + offset);
    break;
    }
    return 0;
    }
    EXPORT_SYMBOL_GPL(ocxl_global_mmio_write64);
    int ocxl_global_mmio_set32(struct ocxl_afu *afu, size_t offset,
    enum ocxl_endian endian, u32 mask)
    {
    u32 tmp;
    if (offset > afu.config.global_mmio_size - 4)
    return -EINVAL;

    if (endian == OCXL_HOST_ENDIAN)
    endian = OCXL_BIG_ENDIAN;

    switch (endian) {
    case OCXL_BIG_ENDIAN:
    tmp = readl_be((char *)afu.global_mmio_ptr + offset);
    tmp |= mask;
    writel_be(tmp, (char *)afu.global_mmio_ptr + offset);
    break;
    default:
    tmp = readl((char *)afu.global_mmio_ptr + offset);
    tmp |= mask;
    writel(tmp, (char *)afu.global_mmio_ptr + offset);
    break;
    }
    return 0;
    }
    EXPORT_SYMBOL_GPL(ocxl_global_mmio_set32);
    int ocxl_global_mmio_set64(struct ocxl_afu *afu, size_t offset,
    enum ocxl_endian endian, u64 mask)
    {
    u64 tmp;
    if (offset > afu.config.global_mmio_size - 8)
    return -EINVAL;

    if (endian == OCXL_HOST_ENDIAN)
    endian = OCXL_BIG_ENDIAN;

    switch (endian) {
    case OCXL_BIG_ENDIAN:
    tmp = readq_be((char *)afu.global_mmio_ptr + offset);
    tmp |= mask;
    writeq_be(tmp, (char *)afu.global_mmio_ptr + offset);
    break;
    default:
    tmp = readq((char *)afu.global_mmio_ptr + offset);
    tmp |= mask;
    writeq(tmp, (char *)afu.global_mmio_ptr + offset);
    break;
    }
    return 0;
    }
    EXPORT_SYMBOL_GPL(ocxl_global_mmio_set64);
    int ocxl_global_mmio_clear32(struct ocxl_afu *afu, size_t offset,
    enum ocxl_endian endian, u32 mask)
    {
    u32 tmp;
    if (offset > afu.config.global_mmio_size - 4)
    return -EINVAL;

    if (endian == OCXL_HOST_ENDIAN)
    endian = OCXL_BIG_ENDIAN;

    switch (endian) {
    case OCXL_BIG_ENDIAN:
    tmp = readl_be((char *)afu.global_mmio_ptr + offset);
    tmp &= ~mask;
    writel_be(tmp, (char *)afu.global_mmio_ptr + offset);
    break;
    default:
    tmp = readl((char *)afu.global_mmio_ptr + offset);
    tmp &= ~mask;
    writel(tmp, (char *)afu.global_mmio_ptr + offset);
    break;
    }
    return 0;
    }
    EXPORT_SYMBOL_GPL(ocxl_global_mmio_clear32);
    int ocxl_global_mmio_clear64(struct ocxl_afu *afu, size_t offset,
    enum ocxl_endian endian, u64 mask)
    {
    u64 tmp;
    if (offset > afu.config.global_mmio_size - 8)
    return -EINVAL;

    if (endian == OCXL_HOST_ENDIAN)
    endian = OCXL_BIG_ENDIAN;

    switch (endian) {
    case OCXL_BIG_ENDIAN:
    tmp = readq_be((char *)afu.global_mmio_ptr + offset);
    tmp &= ~mask;
    writeq_be(tmp, (char *)afu.global_mmio_ptr + offset);
    break;
    default:
    tmp = readq((char *)afu.global_mmio_ptr + offset);
    tmp &= ~mask;
    writeq(tmp, (char *)afu.global_mmio_ptr + offset);
    break;
    }
    writeq(tmp, (char *)afu.global_mmio_ptr + offset);
    return 0;
    }
    EXPORT_SYMBOL_GPL(ocxl_global_mmio_clear64);
