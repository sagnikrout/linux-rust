//! Automatically rewritten from C to Rust
//! Source: drivers/scsi/qedf/qedf_attr.c
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
// QLogic FCoE Offload Driver
// Copyright (c) 2016-2018 Cavium Inc.
//

#[no_mangle]
pub unsafe extern "C" fn qedf_is_vport(qedf: *mut qedf_ctx) -> bool {
    inline bool qedf_is_vport(struct qedf_ctx *qedf)
    {
    return qedf.lport.vport != core::ptr::null_mut();
    }
// Get base qedf for physical port from vport
    static struct qedf_ctx *qedf_get_base_qedf(struct qedf_ctx *qedf)
    {
    struct fc_lport *lport;
    struct fc_lport *base_lport;
    if (!(qedf_is_vport(qedf)))
    return core::ptr::null_mut();
    lport = qedf.lport;
    base_lport = shost_priv(vport_to_shost(lport.vport));
    return lport_priv(base_lport);
    }
    static ssize_t fcoe_mac_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct fc_lport *lport = shost_priv(class_to_shost(dev));
    u32 port_id;
    u8 lport_src_id[3];
    u8 fcoe_mac[6];
    port_id = fc_host_port_id(lport.host);
    lport_src_id[2] = (port_id & 0x000000FF);
    lport_src_id[1] = (port_id & 0x0000FF00) >> 8;
    lport_src_id[0] = (port_id & 0x00FF0000) >> 16;
    fc_fcoe_set_mac(fcoe_mac, lport_src_id);
    return scnprintf(buf, PAGE_SIZE, "%pM\n", fcoe_mac);
    }
    static ssize_t fka_period_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct fc_lport *lport = shost_priv(class_to_shost(dev));
    struct qedf_ctx *qedf = lport_priv(lport);
    let mut fka_period: c_int = -1;
    if (qedf_is_vport(qedf))
    qedf = qedf_get_base_qedf(qedf);
    if (qedf.ctlr.sel_fcf)
    fka_period = qedf.ctlr.sel_fcf.fka_period;
    return scnprintf(buf, PAGE_SIZE, "%d\n", fka_period);
    }
    static DEVICE_ATTR_RO(fcoe_mac);
    static DEVICE_ATTR_RO(fka_period);
    static struct attribute *qedf_host_attrs[] = {
    &dev_attr_fcoe_mac.attr,
    &dev_attr_fka_period.attr,
    core::ptr::null_mut(),
    };
    static const struct attribute_group qedf_host_attr_group = {
    .attrs = qedf_host_attrs
    };
    const struct attribute_group *qedf_host_groups[] = {
    &qedf_host_attr_group,
    core::ptr::null_mut()
    };
    extern const struct qed_fcoe_ops *qed_ops;
#[no_mangle]
pub unsafe extern "C" fn qedf_capture_grc_dump(qedf: *mut qedf_ctx) {
    void qedf_capture_grc_dump(struct qedf_ctx *qedf)
    {
    struct qedf_ctx *base_qedf;
// Make sure we use the base qedf to take the GRC dump
    if (qedf_is_vport(qedf))
    base_qedf = qedf_get_base_qedf(qedf);
    else
    base_qedf = qedf;
    if (test_bit(QEDF_GRCDUMP_CAPTURE, &base_qedf.flags)) {
    QEDF_INFO(&(base_qedf.dbg_ctx), QEDF_LOG_INFO,
    "GRC Dump already captured.\n");
    return;
    }
    qedf_get_grc_dump(base_qedf.cdev, qed_ops.common,
    &base_qedf.grcdump, &base_qedf.grcdump_size);
    QEDF_ERR(&(base_qedf.dbg_ctx), "GRC Dump captured.\n");
    set_bit(QEDF_GRCDUMP_CAPTURE, &base_qedf.flags);
    qedf_uevent_emit(base_qedf.lport.host, QEDF_UEVENT_CODE_GRCDUMP,
    core::ptr::null_mut());
    }
    static ssize_t
    qedf_sysfs_read_grcdump(struct file *filep, struct kobject *kobj,
    const struct bin_attribute *ba, char *buf, loff_t off,
    size_t count)
    {
    let mut ret: isize = 0;
    struct fc_lport *lport = shost_priv(dev_to_shost(container_of(kobj,
    struct device, kobj)));
    struct qedf_ctx *qedf = lport_priv(lport);
    if (test_bit(QEDF_GRCDUMP_CAPTURE, &qedf.flags)) {
    ret = memory_read_from_buffer(buf, count, &off,
    qedf.grcdump, qedf.grcdump_size);
    } else {
    QEDF_ERR(&(qedf.dbg_ctx), "GRC Dump not captured!\n");
    }
    return ret;
    }
    static ssize_t
    qedf_sysfs_write_grcdump(struct file *filep, struct kobject *kobj,
    const struct bin_attribute *ba, char *buf, loff_t off,
    size_t count)
    {
    struct fc_lport *lport = core::ptr::null_mut();
    struct qedf_ctx *qedf = core::ptr::null_mut();
    long reading;
    let mut ret: c_int = 0;
    if (off != 0)
    return ret;
    lport = shost_priv(dev_to_shost(container_of(kobj,
    struct device, kobj)));
    qedf = lport_priv(lport);
    buf[1] = 0;
    ret = kstrtol(buf, 10, &reading);
    if (ret) {
    QEDF_ERR(&(qedf.dbg_ctx), "Invalid input, err(%d)\n", ret);
    return ret;
    }
    switch (reading) {
    case 0:
    memset(qedf.grcdump, 0, qedf.grcdump_size);
    clear_bit(QEDF_GRCDUMP_CAPTURE, &qedf.flags);
    break;
    case 1:
    qedf_capture_grc_dump(qedf);
    break;
    }
    return count;
    }
    static const struct bin_attribute sysfs_grcdump_attr = {
    .attr = {
    .name = "grcdump",
    .mode = S_IRUSR | S_IWUSR,
    },
    .size = 0,
    .read = qedf_sysfs_read_grcdump,
    .write = qedf_sysfs_write_grcdump,
    };
    static struct sysfs_bin_attrs bin_file_entries[] = {
    {"grcdump", &sysfs_grcdump_attr},
    {core::ptr::null_mut()},
    };
#[no_mangle]
pub unsafe extern "C" fn qedf_create_sysfs_ctx_attr(qedf: *mut qedf_ctx) {
    void qedf_create_sysfs_ctx_attr(struct qedf_ctx *qedf)
    {
    qedf_create_sysfs_attr(qedf.lport.host, bin_file_entries);
    }
#[no_mangle]
pub unsafe extern "C" fn qedf_remove_sysfs_ctx_attr(qedf: *mut qedf_ctx) {
    void qedf_remove_sysfs_ctx_attr(struct qedf_ctx *qedf)
    {
    qedf_remove_sysfs_attr(qedf.lport.host, bin_file_entries);
    }
