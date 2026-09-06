//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/platforms/powernv/opal-elog.c
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
// Error log support on PowerNV.
//
// Copyright 2013,2014 IBM Corp.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct elog_obj {
    pub kobj: kobject,
    pub raw_attr: bin_attribute,
    pub id: u64,
    pub type: u64,
    pub size: usize,
    pub buffer: *mut c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct elog_attribute {
    pub attr: attribute,
    ssize_t (*show)(struct elog_obj *elog, struct elog_attribute *attr,
    pub buf): *mut c_char,
    ssize_t (*store)(struct elog_obj *elog, struct elog_attribute *attr,
    pub count): *const *const char buf, size_t,
}

    static ssize_t elog_id_show(struct elog_obj *elog_obj,
    struct elog_attribute *attr,
    char *buf)
    {
    return sysfs_emit(buf, "0x%llx\n", elog_obj.id);
    }
    static const char *elog_type_to_string(uint64_t type)
    {
    switch (type) {
    case 0: return "PEL";
    default: return "unknown";
    }
    }
    static ssize_t elog_type_show(struct elog_obj *elog_obj,
    struct elog_attribute *attr,
    char *buf)
    {
    return sysfs_emit(buf, "0x%llx %s\n", elog_obj.type,
    elog_type_to_string(elog_obj.type));
    }
    static ssize_t elog_ack_show(struct elog_obj *elog_obj,
    struct elog_attribute *attr,
    char *buf)
    {
    return sysfs_emit(buf, "ack - acknowledge log message\n");
    }
    static ssize_t elog_ack_store(struct elog_obj *elog_obj,
    struct elog_attribute *attr,
    const char *buf,
    size_t count)
    {
//
// Try to self remove this attribute. If we are successful,
// delete the kobject itself.
//
    if (sysfs_remove_file_self(&elog_obj.kobj, &attr.attr)) {
    opal_send_ack_elog(elog_obj.id);
    kobject_put(&elog_obj.kobj);
    }
    return count;
    }
    static struct elog_attribute id_attribute =
    __ATTR(id, 0444, elog_id_show, core::ptr::null_mut());
    static struct elog_attribute type_attribute =
    __ATTR(type, 0444, elog_type_show, core::ptr::null_mut());
    static struct elog_attribute ack_attribute =
    __ATTR(acknowledge, 0660, elog_ack_show, elog_ack_store);
    static struct kset *elog_kset;
    static ssize_t elog_attr_show(struct kobject *kobj,
    struct attribute *attr,
    char *buf)
    {
    struct elog_attribute *attribute;
    struct elog_obj *elog;
    attribute = to_elog_attr(attr);
    elog = to_elog_obj(kobj);
    if (!attribute.show)
    return -EIO;
    return attribute.show(elog, attribute, buf);
    }
    static ssize_t elog_attr_store(struct kobject *kobj,
    struct attribute *attr,
    const char *buf, size_t len)
    {
    struct elog_attribute *attribute;
    struct elog_obj *elog;
    attribute = to_elog_attr(attr);
    elog = to_elog_obj(kobj);
    if (!attribute.store)
    return -EIO;
    return attribute.store(elog, attribute, buf, len);
    }
    static const struct sysfs_ops elog_sysfs_ops = {
    .show = elog_attr_show,
    .store = elog_attr_store,
    };
#[no_mangle]
unsafe extern "C" fn elog_release(kobj: *mut kobject) {
    static void elog_release(struct kobject *kobj)
    {
    struct elog_obj *elog;
    elog = to_elog_obj(kobj);
    kfree(elog.buffer);
    kfree(elog);
    }
    static struct attribute *elog_default_attrs[] = {
    &id_attribute.attr,
    &type_attribute.attr,
    &ack_attribute.attr,
    core::ptr::null_mut(),
    };
    ATTRIBUTE_GROUPS(elog_default);
    static const struct kobj_type elog_ktype = {
    .sysfs_ops = &elog_sysfs_ops,
    .release = &elog_release,
    .default_groups = elog_default_groups,
    };
// Maximum size of a single log on FSP is 16KB
pub const OPAL_MAX_ERRLOG_SIZE: c_int = 16384;
    static ssize_t raw_attr_read(struct file *filep, struct kobject *kobj,
    const struct bin_attribute *bin_attr,
    char *buffer, loff_t pos, size_t count)
    {
    int opal_rc;
    struct elog_obj *elog = to_elog_obj(kobj);
// We may have had an error reading before, so let's retry
    if (!elog.buffer) {
    elog.buffer = kzalloc(elog.size, GFP_KERNEL);
    if (!elog.buffer)
    return -EIO;
    opal_rc = opal_read_elog(__pa(elog.buffer),
    elog.size, elog.id);
    if (opal_rc != OPAL_SUCCESS) {
    pr_err_ratelimited("ELOG: log read failed for log-id=%llx\n",
    elog.id);
    kfree(elog.buffer);
    elog.buffer = core::ptr::null_mut();
    return -EIO;
    }
    }
    memcpy(buffer, elog.buffer + pos, count);
    return count;
    }
#[no_mangle]
unsafe extern "C" fn create_elog_obj(id: u64, size: usize, type: u64) {
    static void create_elog_obj(uint64_t id, size_t size, uint64_t type)
    {
    struct elog_obj *elog;
    int rc;
    elog = kzalloc_obj(*elog);
    if (!elog)
    return;
    elog.kobj.kset = elog_kset;
    kobject_init(&elog.kobj, &elog_ktype);
    sysfs_bin_attr_init(&elog.raw_attr);
    elog.raw_attr.attr.name = "raw";
    elog.raw_attr.attr.mode = 0400;
    elog.raw_attr.size = size;
    elog.raw_attr.read = raw_attr_read;
    elog.id = id;
    elog.size = size;
    elog.type = type;
    elog.buffer = kzalloc(elog.size, GFP_KERNEL);
    if (elog.buffer) {
    rc = opal_read_elog(__pa(elog.buffer),
    elog.size, elog.id);
    if (rc != OPAL_SUCCESS) {
    pr_err("ELOG: log read failed for log-id=%llx\n",
    elog.id);
    kfree(elog.buffer);
    elog.buffer = core::ptr::null_mut();
    }
    }
    rc = kobject_add(&elog.kobj, core::ptr::null_mut(), "0x%llx", id);
    if (rc) {
    kobject_put(&elog.kobj);
    return;
    }
//
// As soon as the sysfs file for this elog is created/activated there is
// a chance the opal_errd daemon (or any userspace) might read and
// acknowledge the elog before kobject_uevent() is called. If that
// happens then there is a potential race between
// elog_ack_store->kobject_put() and kobject_uevent() which leads to a
// use-after-free of a kernfs object resulting in a kernel crash.
//
// To avoid that, we need to take a reference on behalf of the bin file,
// so that our reference remains valid while we call kobject_uevent().
// We then drop our reference before exiting the function, leaving the
// bin file to drop the last reference (if it hasn't already).
//
// Take a reference for the bin file
    kobject_get(&elog.kobj);
    rc = sysfs_create_bin_file(&elog.kobj, &elog.raw_attr);
    if (rc == 0) {
    kobject_uevent(&elog.kobj, KOBJ_ADD);
    } else {
// Drop the reference taken for the bin file
    kobject_put(&elog.kobj);
    }
// Drop our reference
    kobject_put(&elog.kobj);
    return;
    }
#[no_mangle]
unsafe extern "C" fn elog_event(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t elog_event(int irq, void *data)
    {
    __be64 size;
    __be64 id;
    __be64 type;
    uint64_t elog_size;
    uint64_t log_id;
    uint64_t elog_type;
    int rc;
    char name[2+16+1];
    struct kobject *kobj;
    rc = opal_get_elog_size(&id, &size, &type);
    if (rc != OPAL_SUCCESS) {
    pr_err("ELOG: OPAL log info read failed\n");
    return IRQ_HANDLED;
    }
    elog_size = be64_to_cpu(size);
    log_id = be64_to_cpu(id);
    elog_type = be64_to_cpu(type);
    WARN_ON(elog_size > OPAL_MAX_ERRLOG_SIZE);
    if (elog_size >= OPAL_MAX_ERRLOG_SIZE)
    elog_size  =  OPAL_MAX_ERRLOG_SIZE;
    sprintf(name, "0x%llx", log_id);
// we may get notified twice, let's handle
// that gracefully and not create two conflicting
// entries.
//
    kobj = kset_find_obj(elog_kset, name);
    if (kobj) {
// Drop reference added by kset_find_obj()
    kobject_put(kobj);
    return IRQ_HANDLED;
    }
    create_elog_obj(log_id, elog_size, elog_type);
    return IRQ_HANDLED;
    }
#[no_mangle]
pub unsafe extern "C" fn opal_elog_init() -> int __init {
    int __init opal_elog_init(void)
    {
    let mut rc: c_int = 0, irq;
// ELOG not supported by firmware
    if (!opal_check_token(OPAL_ELOG_READ))
    return -1;
    elog_kset = kset_create_and_add("elog", core::ptr::null_mut(), opal_kobj);
    if (!elog_kset) {
    pr_warn("%s: failed to create elog kset\n", __func__);
    return -1;
    }
    irq = opal_event_request(ilog2(OPAL_EVENT_ERROR_LOG_AVAIL));
    if (!irq) {
    pr_err("%s: Can't register OPAL event irq (%d)\n",
    __func__, irq);
    return irq;
    }
    rc = request_threaded_irq(irq, core::ptr::null_mut(), elog_event,
    IRQF_TRIGGER_HIGH | IRQF_ONESHOT, "opal-elog", core::ptr::null_mut());
    if (rc) {
    pr_err("%s: Can't request OPAL event irq (%d)\n",
    __func__, rc);
    return rc;
    }
// We are now ready to pull error logs from opal.
    if (opal_check_token(OPAL_ELOG_RESEND))
    opal_resend_pending_logs();
    return 0;
    }
