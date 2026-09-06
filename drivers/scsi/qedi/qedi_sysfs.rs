//! Automatically rewritten from C to Rust
//! Source: drivers/scsi/qedi/qedi_sysfs.c
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
// QLogic iSCSI Offload Driver
// Copyright (c) 2016 Cavium Inc.
//

    static inline struct qedi_ctx *qedi_dev_to_hba(struct device *dev)
    {
    struct Scsi_Host *shost = class_to_shost(dev);
    return iscsi_host_priv(shost);
    }
    static ssize_t port_state_show(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    struct qedi_ctx *qedi = qedi_dev_to_hba(dev);
    if (atomic_read(&qedi.link_state) == QEDI_LINK_UP)
    return sprintf(buf, "Online\n");
    else
    return sprintf(buf, "Linkdown\n");
    }
    static ssize_t speed_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct qedi_ctx *qedi = qedi_dev_to_hba(dev);
    struct qed_link_output if_link;
    qedi_ops.common.get_link(qedi.cdev, &if_link);
    return sprintf(buf, "%d Gbit\n", if_link.speed / 1000);
    }
    static DEVICE_ATTR_RO(port_state);
    static DEVICE_ATTR_RO(speed);
    static struct attribute *qedi_shost_attrs[] = {
    &dev_attr_port_state.attr,
    &dev_attr_speed.attr,
    core::ptr::null_mut()
    };
    static const struct attribute_group qedi_shost_attr_group = {
    .attrs = qedi_shost_attrs
    };
    const struct attribute_group *qedi_shost_groups[] = {
    &qedi_shost_attr_group,
    core::ptr::null_mut()
    };
