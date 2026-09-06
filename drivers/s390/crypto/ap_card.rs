//! Automatically rewritten from C to Rust
//! Source: drivers/s390/crypto/ap_card.c
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
// Copyright IBM Corp. 2016
// Author(s): Martin Schwidefsky <schwidefsky@de.ibm.com>
//
// Adjunct processor bus, card related code.
//

//
// AP card related attributes.
//
    static ssize_t hwtype_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct ap_card *ac = to_ap_card(dev);
    return sysfs_emit(buf, "%d\n", ac.ap_dev.device_type);
    }
    static DEVICE_ATTR_RO(hwtype);
    static ssize_t raw_hwtype_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct ap_card *ac = to_ap_card(dev);
    return sysfs_emit(buf, "%d\n", ac.hwinfo.at);
    }
    static DEVICE_ATTR_RO(raw_hwtype);
    static ssize_t depth_show(struct device *dev, struct device_attribute *attr,
    char *buf)
    {
    struct ap_card *ac = to_ap_card(dev);
    return sysfs_emit(buf, "%d\n", ac.hwinfo.qd + 1);
    }
    static DEVICE_ATTR_RO(depth);
    static ssize_t ap_functions_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct ap_card *ac = to_ap_card(dev);
    return sysfs_emit(buf, "0x%08X\n", ac.hwinfo.fac);
    }
    static DEVICE_ATTR_RO(ap_functions);
    static ssize_t request_count_show(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    struct ap_card *ac = to_ap_card(dev);
    u64 req_cnt;
    req_cnt = 0;
    spin_lock_bh(&ap_queues_lock);
    req_cnt = atomic64_read(&ac.total_request_count);
    spin_unlock_bh(&ap_queues_lock);
    return sysfs_emit(buf, "%llu\n", req_cnt);
    }
    static ssize_t request_count_store(struct device *dev,
    struct device_attribute *attr,
    const char *buf, size_t count)
    {
    int bkt;
    struct ap_queue *aq;
    struct ap_card *ac = to_ap_card(dev);
    spin_lock_bh(&ap_queues_lock);
    hash_for_each(ap_queues, bkt, aq, hnode)
    if (ac == aq.card)
    aq.total_request_count = 0;
    spin_unlock_bh(&ap_queues_lock);
    atomic64_set(&ac.total_request_count, 0);
    return count;
    }
    static DEVICE_ATTR_RW(request_count);
    static ssize_t requestq_count_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    int bkt;
    struct ap_queue *aq;
    unsigned int reqq_cnt;
    struct ap_card *ac = to_ap_card(dev);
    reqq_cnt = 0;
    spin_lock_bh(&ap_queues_lock);
    hash_for_each(ap_queues, bkt, aq, hnode)
    if (ac == aq.card)
    reqq_cnt += aq.requestq_count;
    spin_unlock_bh(&ap_queues_lock);
    return sysfs_emit(buf, "%d\n", reqq_cnt);
    }
    static DEVICE_ATTR_RO(requestq_count);
    static ssize_t pendingq_count_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    int bkt;
    struct ap_queue *aq;
    unsigned int penq_cnt;
    struct ap_card *ac = to_ap_card(dev);
    penq_cnt = 0;
    spin_lock_bh(&ap_queues_lock);
    hash_for_each(ap_queues, bkt, aq, hnode)
    if (ac == aq.card)
    penq_cnt += aq.pendingq_count;
    spin_unlock_bh(&ap_queues_lock);
    return sysfs_emit(buf, "%d\n", penq_cnt);
    }
    static DEVICE_ATTR_RO(pendingq_count);
    static ssize_t modalias_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    return sysfs_emit(buf, "ap:t%02X\n", to_ap_dev(dev).device_type);
    }
    static DEVICE_ATTR_RO(modalias);
    static ssize_t config_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct ap_card *ac = to_ap_card(dev);
    return sysfs_emit(buf, "%d\n", ac.config ? 1 : 0);
    }
    static ssize_t config_store(struct device *dev,
    struct device_attribute *attr,
    const char *buf, size_t count)
    {
    let mut rc: c_int = 0, cfg;
    struct ap_card *ac = to_ap_card(dev);
    if (sscanf(buf, "%d\n", &cfg) != 1 || cfg < 0 || cfg > 1)
    return -EINVAL;
    if (cfg && !ac.config)
    rc = sclp_ap_configure(ac.id);
#[no_mangle]
pub unsafe extern "C" fn if(ac->config: !cfg &&) -> else {
    else if (!cfg && ac.config)
    rc = sclp_ap_deconfigure(ac.id);
    if (rc)
    return rc;
    ac.config = cfg ? true : false;
    ap_send_config_uevent(&ac.ap_dev, ac.config);
    return count;
    }
    static DEVICE_ATTR_RW(config);
    static ssize_t chkstop_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct ap_card *ac = to_ap_card(dev);
    return sysfs_emit(buf, "%d\n", ac.chkstop ? 1 : 0);
    }
    static DEVICE_ATTR_RO(chkstop);
    static ssize_t max_msg_size_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct ap_card *ac = to_ap_card(dev);
    return sysfs_emit(buf, "%u\n", ac.maxmsgsize);
    }
    static DEVICE_ATTR_RO(max_msg_size);
    static struct attribute *ap_card_dev_attrs[] = {
    &dev_attr_hwtype.attr,
    &dev_attr_raw_hwtype.attr,
    &dev_attr_depth.attr,
    &dev_attr_ap_functions.attr,
    &dev_attr_request_count.attr,
    &dev_attr_requestq_count.attr,
    &dev_attr_pendingq_count.attr,
    &dev_attr_modalias.attr,
    &dev_attr_config.attr,
    &dev_attr_chkstop.attr,
    &dev_attr_max_msg_size.attr,
    core::ptr::null_mut()
    };
    static struct attribute_group ap_card_dev_attr_group = {
    .attrs = ap_card_dev_attrs
    };
    static const struct attribute_group *ap_card_dev_attr_groups[] = {
    &ap_card_dev_attr_group,
    core::ptr::null_mut()
    };
    static struct device_type ap_card_type = {
    .name = "ap_card",
    .groups = ap_card_dev_attr_groups,
    };
#[no_mangle]
unsafe extern "C" fn ap_card_device_release(dev: *mut device) {
    static void ap_card_device_release(struct device *dev)
    {
    struct ap_card *ac = to_ap_card(dev);
    kfree(ac);
    }
    struct ap_card *ap_card_create(int id, struct ap_tapq_hwinfo hwinfo,
    int comp_type)
    {
    struct ap_card *ac;
    ac = kzalloc_obj(*ac);
    if (!ac)
    return core::ptr::null_mut();
    ac.ap_dev.device.release = ap_card_device_release;
    ac.ap_dev.device.type = &ap_card_type;
    ac.ap_dev.device_type = comp_type;
    ac.hwinfo = hwinfo;
    ac.id = id;
    ac.maxmsgsize = hwinfo.ml > 3 ?
    hwinfo.ml * AP_TAPQ_ML_FIELD_CHUNK_SIZE : AP_DEFAULT_MAX_MSG_SIZE;
    return ac;
    }
