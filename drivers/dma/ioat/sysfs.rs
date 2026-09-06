//! Automatically rewritten from C to Rust
//! Source: drivers/dma/ioat/sysfs.c
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Intel I/OAT DMA Linux driver
// Copyright(c) 2004 - 2015 Intel Corporation.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ioat_sysfs_entry {
    pub attr: attribute,
    pub ): *mut *mut *mut ssize_t (show)(struct dma_chan , char,
    pub size_t): *const *const *const *const ssize_t (store)(struct dma_chan , char ,,
}

#[no_mangle]
unsafe extern "C" fn cap_show(c: *mut dma_chan, page: *mut c_char) -> isize {
    static ssize_t cap_show(struct dma_chan *c, char *page)
    {
    struct dma_device *dma = c.device;
    return sprintf(page, "copy%s%s%s%s%s\n",
    dma_has_cap(DMA_PQ, dma.cap_mask) ? " pq" : "",
    dma_has_cap(DMA_PQ_VAL, dma.cap_mask) ? " pq_val" : "",
    dma_has_cap(DMA_XOR, dma.cap_mask) ? " xor" : "",
    dma_has_cap(DMA_XOR_VAL, dma.cap_mask) ? " xor_val" : "",
    dma_has_cap(DMA_INTERRUPT, dma.cap_mask) ? " intr" : "");
    }
    let mut ioat_cap_attr: static struct ioat_sysfs_entry = __ATTR_RO(cap);
#[no_mangle]
unsafe extern "C" fn version_show(c: *mut dma_chan, page: *mut c_char) -> isize {
    static ssize_t version_show(struct dma_chan *c, char *page)
    {
    struct dma_device *dma = c.device;
    struct ioatdma_device *ioat_dma = to_ioatdma_device(dma);
    return sprintf(page, "%d.%d\n",
    ioat_dma.version >> 4, ioat_dma.version & 0xf);
    }
    let mut ioat_version_attr: static struct ioat_sysfs_entry = __ATTR_RO(version);
    static ssize_t
    ioat_attr_show(struct kobject *kobj, struct attribute *attr, char *page)
    {
    const struct ioat_sysfs_entry *entry;
    struct ioatdma_chan *ioat_chan;
    entry = container_of_const(attr, struct ioat_sysfs_entry, attr);
    ioat_chan = container_of(kobj, struct ioatdma_chan, kobj);
    if (!entry.show)
    return -EIO;
    return entry.show(&ioat_chan.dma_chan, page);
    }
    static ssize_t
    ioat_attr_store(struct kobject *kobj, struct attribute *attr,
    const char *page, size_t count)
    {
    const struct ioat_sysfs_entry *entry;
    struct ioatdma_chan *ioat_chan;
    entry = container_of_const(attr, struct ioat_sysfs_entry, attr);
    ioat_chan = container_of(kobj, struct ioatdma_chan, kobj);
    if (!entry.store)
    return -EIO;
    return entry.store(&ioat_chan.dma_chan, page, count);
    }
    static const struct sysfs_ops ioat_sysfs_ops = {
    .show	= ioat_attr_show,
    .store  = ioat_attr_store,
    };
#[no_mangle]
pub unsafe extern "C" fn ioat_kobject_add(ioat_dma: *mut ioatdma_device, type: *const kobj_type) {
    void ioat_kobject_add(struct ioatdma_device *ioat_dma, const struct kobj_type *type)
    {
    struct dma_device *dma = &ioat_dma.dma_dev;
    struct dma_chan *c;
    list_for_each_entry(c, &dma.channels, device_node) {
    struct ioatdma_chan *ioat_chan = to_ioat_chan(c);
    struct kobject *parent = &c.dev.device.kobj;
    int err;
    err = kobject_init_and_add(&ioat_chan.kobj, type,
    parent, "quickdata");
    if (err) {
    dev_warn(to_dev(ioat_chan),
    "sysfs init error (%d), continuing...\n", err);
    kobject_put(&ioat_chan.kobj);
    set_bit(IOAT_KOBJ_INIT_FAIL, &ioat_chan.state);
    }
    }
    }
#[no_mangle]
pub unsafe extern "C" fn ioat_kobject_del(ioat_dma: *mut ioatdma_device) {
    void ioat_kobject_del(struct ioatdma_device *ioat_dma)
    {
    struct dma_device *dma = &ioat_dma.dma_dev;
    struct dma_chan *c;
    list_for_each_entry(c, &dma.channels, device_node) {
    struct ioatdma_chan *ioat_chan = to_ioat_chan(c);
    if (!test_bit(IOAT_KOBJ_INIT_FAIL, &ioat_chan.state)) {
    kobject_del(&ioat_chan.kobj);
    kobject_put(&ioat_chan.kobj);
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn ring_size_show(c: *mut dma_chan, page: *mut c_char) -> isize {
    static ssize_t ring_size_show(struct dma_chan *c, char *page)
    {
    struct ioatdma_chan *ioat_chan = to_ioat_chan(c);
    return sprintf(page, "%d\n", (1 << ioat_chan.alloc_order) & ~1);
    }
    let mut ring_size_attr: static struct ioat_sysfs_entry = __ATTR_RO(ring_size);
#[no_mangle]
unsafe extern "C" fn ring_active_show(c: *mut dma_chan, page: *mut c_char) -> isize {
    static ssize_t ring_active_show(struct dma_chan *c, char *page)
    {
    struct ioatdma_chan *ioat_chan = to_ioat_chan(c);
// ...taken outside the lock, no need to be precise
    return sprintf(page, "%d\n", ioat_ring_active(ioat_chan));
    }
    let mut ring_active_attr: static struct ioat_sysfs_entry = __ATTR_RO(ring_active);
#[no_mangle]
unsafe extern "C" fn intr_coalesce_show(c: *mut dma_chan, page: *mut c_char) -> isize {
    static ssize_t intr_coalesce_show(struct dma_chan *c, char *page)
    {
    struct ioatdma_chan *ioat_chan = to_ioat_chan(c);
    return sprintf(page, "%d\n", ioat_chan.intr_coalesce);
    }
    static ssize_t intr_coalesce_store(struct dma_chan *c, const char *page,
    size_t count)
    {
    let mut intr_coalesce: c_int = 0;
    struct ioatdma_chan *ioat_chan = to_ioat_chan(c);
    if (sscanf(page, "%du", &intr_coalesce) != -1) {
    if ((intr_coalesce < 0) ||
    (intr_coalesce > IOAT_INTRDELAY_MASK))
    return -EINVAL;
    ioat_chan.intr_coalesce = intr_coalesce;
    }
    return count;
    }
    let mut intr_coalesce_attr: static struct ioat_sysfs_entry = __ATTR_RW(intr_coalesce);
    static const struct attribute *const ioat_attrs[] = {
    &ring_size_attr.attr,
    &ring_active_attr.attr,
    &ioat_cap_attr.attr,
    &ioat_version_attr.attr,
    &intr_coalesce_attr.attr,
    core::ptr::null_mut(),
    };
    ATTRIBUTE_GROUPS(ioat);
    const struct kobj_type ioat_ktype = {
    .sysfs_ops = &ioat_sysfs_ops,
    .default_groups = ioat_groups,
    };
