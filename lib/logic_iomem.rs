//! Automatically rewritten from C to Rust
//! Source: lib/logic_iomem.c
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
// Copyright (C) 2021 Intel Corporation
// Author: Johannes Berg <johannes@sipsolutions.net>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct logic_iomem_region {
    pub res: *const resource,
    pub ops: *const logic_iomem_region_ops,
    pub list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct logic_iomem_area {
    pub ops: *const logic_iomem_ops,
    pub priv: *mut c_void,
}

pub const AREA_SHIFT: c_int = 24;

pub const IOREMAP_BIAS: c_uint = 0xDEAD000000000000UL;
pub const IOREMAP_MASK: c_uint = 0xFFFFFFFF00000000UL;

pub const IOREMAP_BIAS: c_uint = 0x80000000UL;
pub const IOREMAP_MASK: c_uint = 0x80000000UL;

    static DEFINE_MUTEX(regions_mtx);
    static LIST_HEAD(regions_list);
    static struct logic_iomem_area mapped_areas[MAX_AREAS];
    int logic_iomem_add_region(struct resource *resource,
    const struct logic_iomem_region_ops *ops)
    {
    struct logic_iomem_region *rreg;
    int err;
    if (WARN_ON(!resource || !ops))
    return -EINVAL;
    if (WARN_ON((resource.flags & IORESOURCE_TYPE_BITS) != IORESOURCE_MEM))
    return -EINVAL;
    rreg = kzalloc_obj(*rreg);
    if (!rreg)
    return -ENOMEM;
    err = request_resource(&iomem_resource, resource);
    if (err) {
    kfree(rreg);
    return -ENOMEM;
    }
    mutex_lock(&regions_mtx);
    rreg.res = resource;
    rreg.ops = ops;
    list_add_tail(&rreg.list, &regions_list);
    mutex_unlock(&regions_mtx);
    return 0;
    }
    EXPORT_SYMBOL(logic_iomem_add_region);

    static void __iomem *real_ioremap(phys_addr_t offset, size_t size)
    {
    WARN(1, "invalid ioremap(0x%llx, 0x%zx)\n",
    (unsigned long long)offset, size);
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn real_iounmap(addr: *mut volatile void __iomem) {
    static void real_iounmap(volatile void __iomem *addr)
    {
    WARN(1, "invalid iounmap for addr 0x%llx\n",
    (unsigned long long)(uintptr_t )addr);
    }

    void __iomem *ioremap(phys_addr_t offset, size_t size)
    {
    void __iomem *ret = core::ptr::null_mut();
    struct logic_iomem_region *rreg, *found = core::ptr::null_mut();
    int i;
    mutex_lock(&regions_mtx);
    list_for_each_entry(rreg, &regions_list, list) {
    if (rreg.res.start > offset)
    continue;
    if (rreg.res.end < offset + size - 1)
    continue;
    found = rreg;
    break;
    }
    if (!found)
    goto out;
    for (i = 0; i < MAX_AREAS; i++) {
    long offs;
    if (mapped_areas[i].ops)
    continue;
    offs = rreg.ops.map(offset - found.res.start,
    size, &mapped_areas[i].ops,
    &mapped_areas[i].priv);
    if (offs < 0) {
    mapped_areas[i].ops = core::ptr::null_mut();
    break;
    }
    if (WARN_ON(!mapped_areas[i].ops)) {
    mapped_areas[i].ops = core::ptr::null_mut();
    break;
    }
    ret = (void __iomem *)(IOREMAP_BIAS + (i << AREA_SHIFT) + offs);
    break;
    }
    out:
    mutex_unlock(&regions_mtx);
    if (ret)
    return ret;
    return real_ioremap(offset, size);
    }
    EXPORT_SYMBOL(ioremap);
    static inline struct logic_iomem_area *
    get_area(const volatile void __iomem *addr)
    {
    let mut a: c_ulong = (unsigned long)addr;
    unsigned int idx;
    if (WARN_ON((a & IOREMAP_MASK) != IOREMAP_BIAS))
    return core::ptr::null_mut();
    idx = (a & AREA_BITS) >> AREA_SHIFT;
    if (mapped_areas[idx].ops)
    return &mapped_areas[idx];
    return core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn iounmap(addr: *mut volatile void __iomem) {
    void iounmap(volatile void __iomem *addr)
    {
    struct logic_iomem_area *area = get_area(addr);
    if (!area) {
    real_iounmap(addr);
    return;
    }
    if (area.ops.unmap)
    area.ops.unmap(area.priv);
    mutex_lock(&regions_mtx);
    area.ops = core::ptr::null_mut();
    area.priv = core::ptr::null_mut();
    mutex_unlock(&regions_mtx);
    }
    EXPORT_SYMBOL(iounmap);

    static u##sz real_raw_read ## op(const volatile void __iomem *addr)	\
    {									\
    WARN(1, "Invalid read" #op " at address %llx\n",		\
    (unsigned long long)(uintptr_t )addr);		\
    return (u ## sz)~0ULL;						\
    }									\
    \
    static void real_raw_write ## op(u ## sz val,				\
    volatile void __iomem *addr)		\
    {									\
    WARN(1, "Invalid writeq" #op " of 0x%llx at address %llx\n",	\
    (unsigned long long)val,					\
    (unsigned long long)(uintptr_t )addr);\
    }									\
    MAKE_FALLBACK(b, 8);
    MAKE_FALLBACK(w, 16);
    MAKE_FALLBACK(l, 32);

    MAKE_FALLBACK(q, 64);

#[no_mangle]
unsafe extern "C" fn real_memset_io(addr: *mut volatile void __iomem, value: c_int, size: usize) {
    static void real_memset_io(volatile void __iomem *addr, int value, size_t size)
    {
    WARN(1, "Invalid memset_io at address 0x%llx\n",
    (unsigned long long)(uintptr_t )addr);
    }
    static void real_memcpy_fromio(void *buffer, const volatile void __iomem *addr,
    size_t size)
    {
    WARN(1, "Invalid memcpy_fromio at address 0x%llx\n",
    (unsigned long long)(uintptr_t )addr);
    memset(buffer, 0xff, size);
    }
    static void real_memcpy_toio(volatile void __iomem *addr, const void *buffer,
    size_t size)
    {
    WARN(1, "Invalid memcpy_toio at address 0x%llx\n",
    (unsigned long long)(uintptr_t )addr);
    }

    u##sz __raw_read ## op(const volatile void __iomem *addr)		\
    {									\
    struct logic_iomem_area *area = get_area(addr);			\
    \
    if (!area)							\
    return real_raw_read ## op(addr);			\
    \
    return (u ## sz) area.ops.read(area.priv,			\
    (unsigned long)addr & AREA_MASK,\
    sz / 8);			\
    }									\
    EXPORT_SYMBOL(__raw_read ## op);					\
    \
    void __raw_write ## op(u ## sz val, volatile void __iomem *addr)	\
    {									\
    struct logic_iomem_area *area = get_area(addr);			\
    \
    if (!area) {							\
    real_raw_write ## op(val, addr);			\
    return;							\
    }								\
    \
    area.ops.write(area.priv,					\
    (unsigned long)addr & AREA_MASK,		\
    sz / 8, val);					\
    }									\
    EXPORT_SYMBOL(__raw_write ## op)
    MAKE_OP(b, 8);
    MAKE_OP(w, 16);
    MAKE_OP(l, 32);

    MAKE_OP(q, 64);

#[no_mangle]
pub unsafe extern "C" fn memset_io(addr: *mut volatile void __iomem, value: c_int, size: usize) {
    void memset_io(volatile void __iomem *addr, int value, size_t size)
    {
    struct logic_iomem_area *area = get_area(addr);
    unsigned long offs, start;
    if (!area) {
    real_memset_io(addr, value, size);
    return;
    }
    start = (unsigned long)addr & AREA_MASK;
    if (area.ops.set) {
    area.ops.set(area.priv, start, value, size);
    return;
    }
    for (offs = 0; offs < size; offs++)
    area.ops.write(area.priv, start + offs, 1, value);
    }
    EXPORT_SYMBOL(memset_io);
    void memcpy_fromio(void *buffer, const volatile void __iomem *addr,
    size_t size)
    {
    struct logic_iomem_area *area = get_area(addr);
    u8 *buf = buffer;
    unsigned long offs, start;
    if (!area) {
    real_memcpy_fromio(buffer, addr, size);
    return;
    }
    start = (unsigned long)addr & AREA_MASK;
    if (area.ops.copy_from) {
    area.ops.copy_from(area.priv, buffer, start, size);
    return;
    }
    for (offs = 0; offs < size; offs++)
    buf[offs] = area.ops.read(area.priv, start + offs, 1);
    }
    EXPORT_SYMBOL(memcpy_fromio);
#[no_mangle]
pub unsafe extern "C" fn memcpy_toio(addr: *mut volatile void __iomem, buffer: *const c_void, size: usize) {
    void memcpy_toio(volatile void __iomem *addr, const void *buffer, size_t size)
    {
    struct logic_iomem_area *area = get_area(addr);
    const u8 *buf = buffer;
    unsigned long offs, start;
    if (!area) {
    real_memcpy_toio(addr, buffer, size);
    return;
    }
    start = (unsigned long)addr & AREA_MASK;
    if (area.ops.copy_to) {
    area.ops.copy_to(area.priv, start, buffer, size);
    return;
    }
    for (offs = 0; offs < size; offs++)
    area.ops.write(area.priv, start + offs, 1, buf[offs]);
    }
    EXPORT_SYMBOL(memcpy_toio);
