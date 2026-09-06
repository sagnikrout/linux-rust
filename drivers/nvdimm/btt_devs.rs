//! Automatically rewritten from C to Rust
//! Source: drivers/nvdimm/btt_devs.c
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
// Copyright(c) 2013-2015 Intel Corporation. All rights reserved.
//

#[no_mangle]
unsafe extern "C" fn nd_btt_release(dev: *mut device) {
    static void nd_btt_release(struct device *dev)
    {
    struct nd_region *nd_region = to_nd_region(dev.parent);
    struct nd_btt *nd_btt = to_nd_btt(dev);
    dev_dbg(dev, "trace\n");
    nd_detach_ndns(&nd_btt.dev, &nd_btt.ndns);
    ida_free(&nd_region.btt_ida, nd_btt.id);
    kfree(nd_btt.uuid);
    kfree(nd_btt);
    }
    struct nd_btt *to_nd_btt(struct device *dev)
    {
    struct nd_btt *nd_btt = container_of(dev, struct nd_btt, dev);
    WARN_ON(!is_nd_btt(dev));
    return nd_btt;
    }
    EXPORT_SYMBOL(to_nd_btt);
    static const unsigned long btt_lbasize_supported[] = { 512, 520, 528,
    4096, 4104, 4160, 4224, 0 };
    static ssize_t sector_size_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct nd_btt *nd_btt = to_nd_btt(dev);
    return nd_size_select_show(nd_btt.lbasize, btt_lbasize_supported, buf);
    }
    static ssize_t sector_size_store(struct device *dev,
    struct device_attribute *attr, const char *buf, size_t len)
    {
    struct nd_btt *nd_btt = to_nd_btt(dev);
    ssize_t rc;
    guard(device)(dev);
    guard(nvdimm_bus)(dev);
    rc = nd_size_select_store(dev, buf, &nd_btt.lbasize,
    btt_lbasize_supported);
    dev_dbg(dev, "result: %zd wrote: %s%s", rc, buf,
    buf[len - 1] == '\n' ? "" : "\n");
    return rc ? rc : len;
    }
    static DEVICE_ATTR_RW(sector_size);
    static ssize_t uuid_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct nd_btt *nd_btt = to_nd_btt(dev);
    if (nd_btt.uuid)
    return sprintf(buf, "%pUb\n", nd_btt.uuid);
    return sprintf(buf, "\n");
    }
    static ssize_t uuid_store(struct device *dev,
    struct device_attribute *attr, const char *buf, size_t len)
    {
    struct nd_btt *nd_btt = to_nd_btt(dev);
    ssize_t rc;
    device_lock(dev);
    rc = nd_uuid_store(dev, &nd_btt.uuid, buf, len);
    dev_dbg(dev, "result: %zd wrote: %s%s", rc, buf,
    buf[len - 1] == '\n' ? "" : "\n");
    device_unlock(dev);
    return rc ? rc : len;
    }
    static DEVICE_ATTR_RW(uuid);
    static ssize_t namespace_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct nd_btt *nd_btt = to_nd_btt(dev);
    guard(nvdimm_bus)(dev);
    return sprintf(buf, "%s\n", nd_btt.ndns
    ? dev_name(&nd_btt.ndns.dev) : "");
    }
    static ssize_t namespace_store(struct device *dev,
    struct device_attribute *attr, const char *buf, size_t len)
    {
    struct nd_btt *nd_btt = to_nd_btt(dev);
    ssize_t rc;
    guard(device)(dev);
    guard(nvdimm_bus)(dev);
    rc = nd_namespace_store(dev, &nd_btt.ndns, buf, len);
    dev_dbg(dev, "result: %zd wrote: %s%s", rc, buf,
    buf[len - 1] == '\n' ? "" : "\n");
    return rc;
    }
    static DEVICE_ATTR_RW(namespace);
    static ssize_t size_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct nd_btt *nd_btt = to_nd_btt(dev);
    ssize_t rc;
    device_lock(dev);
    if (dev.driver)
    rc = sprintf(buf, "%llu\n", nd_btt.size);
    else {
// no size to convey if the btt instance is disabled
    rc = -ENXIO;
    }
    device_unlock(dev);
    return rc;
    }
    static DEVICE_ATTR_RO(size);
    static ssize_t log_zero_flags_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    return sprintf(buf, "Y\n");
    }
    static DEVICE_ATTR_RO(log_zero_flags);
    static struct attribute *nd_btt_attributes[] = {
    &dev_attr_sector_size.attr,
    &dev_attr_namespace.attr,
    &dev_attr_uuid.attr,
    &dev_attr_size.attr,
    &dev_attr_log_zero_flags.attr,
    core::ptr::null_mut(),
    };
    static struct attribute_group nd_btt_attribute_group = {
    .attrs = nd_btt_attributes,
    };
    static const struct attribute_group *nd_btt_attribute_groups[] = {
    &nd_btt_attribute_group,
    &nd_device_attribute_group,
    &nd_numa_attribute_group,
    core::ptr::null_mut(),
    };
    static const struct device_type nd_btt_device_type = {
    .name = "nd_btt",
    .release = nd_btt_release,
    .groups = nd_btt_attribute_groups,
    };
#[no_mangle]
pub unsafe extern "C" fn is_nd_btt(dev: *mut device) -> bool {
    bool is_nd_btt(struct device *dev)
    {
    return dev.type == &nd_btt_device_type;
    }
    EXPORT_SYMBOL(is_nd_btt);
    static struct lock_class_key nvdimm_btt_key;
    static struct device *__nd_btt_create(struct nd_region *nd_region,
    unsigned long lbasize, uuid_t *uuid,
    struct nd_namespace_common *ndns)
    {
    struct nd_btt *nd_btt;
    struct device *dev;
    nd_btt = kzalloc_obj(*nd_btt);
    if (!nd_btt)
    return core::ptr::null_mut();
    nd_btt.id = ida_alloc(&nd_region.btt_ida, GFP_KERNEL);
    if (nd_btt.id < 0)
    goto out_nd_btt;
    nd_btt.lbasize = lbasize;
    if (uuid) {
    uuid = kmemdup(uuid, 16, GFP_KERNEL);
    if (!uuid)
    goto out_put_id;
    }
    nd_btt.uuid = uuid;
    dev = &nd_btt.dev;
    dev_set_name(dev, "btt%d.%d", nd_region.id, nd_btt.id);
    dev.parent = &nd_region.dev;
    dev.type = &nd_btt_device_type;
    device_initialize(&nd_btt.dev);
    lockdep_set_class(&nd_btt.dev.mutex, &nvdimm_btt_key);
    if (ndns && !__nd_attach_ndns(&nd_btt.dev, ndns, &nd_btt.ndns)) {
    dev_dbg(&ndns.dev, "failed, already claimed by %s\n",
    dev_name(ndns.claim));
    put_device(dev);
    return core::ptr::null_mut();
    }
    return dev;
    out_put_id:
    ida_free(&nd_region.btt_ida, nd_btt.id);
    out_nd_btt:
    kfree(nd_btt);
    return core::ptr::null_mut();
    }
    struct device *nd_btt_create(struct nd_region *nd_region)
    {
    struct device *dev = __nd_btt_create(nd_region, 0, core::ptr::null_mut(), core::ptr::null_mut());
    nd_device_register(dev);
    return dev;
    }
//
// nd_btt_arena_is_valid - check if the metadata layout is valid
// @nd_btt:	device with BTT geometry and backing device info
// @super:	pointer to the arena's info block being tested
//
// Check consistency of the btt info block with itself by validating
// the checksum, and with the parent namespace by verifying the
// parent_uuid contained in the info block with the one supplied in.
//
// Returns:
// false for an invalid info block, true for a valid one
//
#[no_mangle]
pub unsafe extern "C" fn nd_btt_arena_is_valid(nd_btt: *mut nd_btt, super: *mut btt_sb) -> bool {
    bool nd_btt_arena_is_valid(struct nd_btt *nd_btt, struct btt_sb *super)
    {
    const uuid_t *ns_uuid = nd_dev_to_uuid(&nd_btt.ndns.dev);
    uuid_t parent_uuid;
    u64 checksum;
    if (memcmp(super.signature, BTT_SIG, BTT_SIG_LEN) != 0)
    return false;
    import_uuid(&parent_uuid, super.parent_uuid);
    if (!uuid_is_null(&parent_uuid))
    if (!uuid_equal(&parent_uuid, ns_uuid))
    return false;
    checksum = le64_to_cpu(super.checksum);
    super.checksum = 0;
    if (checksum != nd_sb_checksum((struct nd_gen_sb *) super))
    return false;
    super.checksum = cpu_to_le64(checksum);
// TODO: figure out action for this
    if ((le32_to_cpu(super.flags) & IB_FLAG_ERROR_MASK) != 0)
    dev_info(&nd_btt.dev, "Found arena with an error flag\n");
    return true;
    }
    EXPORT_SYMBOL(nd_btt_arena_is_valid);
    int nd_btt_version(struct nd_btt *nd_btt, struct nd_namespace_common *ndns,
    struct btt_sb *btt_sb)
    {
    if (ndns.claim_class == NVDIMM_CCLASS_BTT2) {
// Probe/setup for BTT v2.0
    nd_btt.initial_offset = 0;
    nd_btt.version_major = 2;
    nd_btt.version_minor = 0;
    if (nvdimm_read_bytes(ndns, 0, btt_sb, sizeof(*btt_sb), 0))
    return -ENXIO;
    if (!nd_btt_arena_is_valid(nd_btt, btt_sb))
    return -ENODEV;
    if ((le16_to_cpu(btt_sb.version_major) != 2) ||
    (le16_to_cpu(btt_sb.version_minor) != 0))
    return -ENODEV;
    } else {
//
// Probe/setup for BTT v1.1 (NVDIMM_CCLASS_NONE or
// NVDIMM_CCLASS_BTT)
//
    nd_btt.initial_offset = SZ_4K;
    nd_btt.version_major = 1;
    nd_btt.version_minor = 1;
    if (nvdimm_read_bytes(ndns, SZ_4K, btt_sb, sizeof(*btt_sb), 0))
    return -ENXIO;
    if (!nd_btt_arena_is_valid(nd_btt, btt_sb))
    return -ENODEV;
    if ((le16_to_cpu(btt_sb.version_major) != 1) ||
    (le16_to_cpu(btt_sb.version_minor) != 1))
    return -ENODEV;
    }
    return 0;
    }
    EXPORT_SYMBOL(nd_btt_version);
    static int __nd_btt_probe(struct nd_btt *nd_btt,
    struct nd_namespace_common *ndns, struct btt_sb *btt_sb)
    {
    int rc;
    if (!btt_sb || !ndns || !nd_btt)
    return -ENODEV;
    if (nvdimm_namespace_capacity(ndns) < SZ_16M)
    return -ENXIO;
    rc = nd_btt_version(nd_btt, ndns, btt_sb);
    if (rc < 0)
    return rc;
    nd_btt.lbasize = le32_to_cpu(btt_sb.external_lbasize);
    nd_btt.uuid = kmemdup(&btt_sb.uuid, sizeof(uuid_t), GFP_KERNEL);
    if (!nd_btt.uuid)
    return -ENOMEM;
    nd_device_register(&nd_btt.dev);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn nd_btt_probe(dev: *mut device, ndns: *mut nd_namespace_common) -> c_int {
    int nd_btt_probe(struct device *dev, struct nd_namespace_common *ndns)
    {
    int rc;
    struct device *btt_dev;
    struct btt_sb *btt_sb;
    struct nd_region *nd_region = to_nd_region(ndns.dev.parent);
    if (ndns.force_raw)
    return -ENODEV;
    switch (ndns.claim_class) {
    case NVDIMM_CCLASS_NONE:
    case NVDIMM_CCLASS_BTT:
    case NVDIMM_CCLASS_BTT2:
    break;
    default:
    return -ENODEV;
    }
    scoped_guard(nvdimm_bus, &ndns.dev)
    btt_dev = __nd_btt_create(nd_region, 0, core::ptr::null_mut(), ndns);
    if (!btt_dev)
    return -ENOMEM;
    btt_sb = devm_kzalloc(dev, sizeof(*btt_sb), GFP_KERNEL);
    rc = __nd_btt_probe(to_nd_btt(btt_dev), ndns, btt_sb);
    dev_dbg(dev, "btt: %s\n", rc == 0 ? dev_name(btt_dev) : "<none>");
    if (rc < 0) {
    struct nd_btt *nd_btt = to_nd_btt(btt_dev);
    nd_detach_ndns(btt_dev, &nd_btt.ndns);
    put_device(btt_dev);
    }
    return rc;
    }
    EXPORT_SYMBOL(nd_btt_probe);
