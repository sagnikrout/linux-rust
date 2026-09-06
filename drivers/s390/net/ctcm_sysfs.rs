//! Automatically rewritten from C to Rust
//! Source: drivers/s390/net/ctcm_sysfs.c
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
// Copyright IBM Corp. 2007, 2007
// Authors:	Peter Tiedemann (ptiedem@de.ibm.com)
//

//
// sysfs attributes
//
    static ssize_t ctcm_buffer_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct ctcm_priv *priv = dev_get_drvdata(dev);
    if (!priv)
    return -ENODEV;
    return sysfs_emit(buf, "%d\n", priv.buffer_size);
    }
    static ssize_t ctcm_buffer_write(struct device *dev,
    struct device_attribute *attr, const char *buf, size_t count)
    {
    struct net_device *ndev;
    unsigned int bs1;
    struct ctcm_priv *priv = dev_get_drvdata(dev);
    int rc;
    if (!(priv && priv.channel[CTCM_READ] &&
    priv.channel[CTCM_READ].netdev)) {
    CTCM_DBF_TEXT(SETUP, CTC_DBF_ERROR, "bfnondev");
    return -ENODEV;
    }
    ndev = priv.channel[CTCM_READ].netdev;
    rc = kstrtouint(buf, 0, &bs1);
    if (rc)
    goto einval;
    if (bs1 > CTCM_BUFSIZE_LIMIT)
    goto einval;
    if (bs1 < (576 + LL_HEADER_LENGTH + 2))
    goto einval;
    priv.buffer_size = bs1;	/* just to overwrite the default */
    if ((ndev.flags & IFF_RUNNING) &&
    (bs1 < (ndev.mtu + LL_HEADER_LENGTH + 2)))
    goto einval;
    priv.channel[CTCM_READ].max_bufsize = bs1;
    priv.channel[CTCM_WRITE].max_bufsize = bs1;
    if (!(ndev.flags & IFF_RUNNING))
    ndev.mtu = bs1 - LL_HEADER_LENGTH - 2;
    priv.channel[CTCM_READ].flags |= CHANNEL_FLAGS_BUFSIZE_CHANGED;
    priv.channel[CTCM_WRITE].flags |= CHANNEL_FLAGS_BUFSIZE_CHANGED;
    CTCM_DBF_DEV(SETUP, ndev, buf);
    return count;
    einval:
    CTCM_DBF_DEV(SETUP, ndev, "buff_err");
    return -EINVAL;
    }
#[no_mangle]
unsafe extern "C" fn ctcm_print_statistics(priv: *mut ctcm_priv) {
    static void ctcm_print_statistics(struct ctcm_priv *priv)
    {
    char *sbuf;
    char *p;
    if (!priv)
    return;
    sbuf = kmalloc(2048, GFP_KERNEL);
    if (sbuf == core::ptr::null_mut())
    return;
    p = sbuf;
    p += scnprintf(p, CTCM_STATSIZE_LIMIT, "  Device FSM state: %s\n",
    fsm_getstate_str(priv.fsm));
    p += scnprintf(p, CTCM_STATSIZE_LIMIT, "  RX channel FSM state: %s\n",
    fsm_getstate_str(priv.channel[CTCM_READ].fsm));
    p += scnprintf(p, CTCM_STATSIZE_LIMIT, "  TX channel FSM state: %s\n",
    fsm_getstate_str(priv.channel[CTCM_WRITE].fsm));
    p += scnprintf(p, CTCM_STATSIZE_LIMIT, "  Max. TX buffer used: %ld\n",
    priv.channel[WRITE].prof.maxmulti);
    p += scnprintf(p, CTCM_STATSIZE_LIMIT, "  Max. chained SKBs: %ld\n",
    priv.channel[WRITE].prof.maxcqueue);
    p += scnprintf(p, CTCM_STATSIZE_LIMIT, "  TX single write ops: %ld\n",
    priv.channel[WRITE].prof.doios_single);
    p += scnprintf(p, CTCM_STATSIZE_LIMIT, "  TX multi write ops: %ld\n",
    priv.channel[WRITE].prof.doios_multi);
    p += scnprintf(p, CTCM_STATSIZE_LIMIT, "  Netto bytes written: %ld\n",
    priv.channel[WRITE].prof.txlen);
    p += scnprintf(p, CTCM_STATSIZE_LIMIT, "  Max. TX IO-time: %u\n",
    jiffies_to_usecs(priv.channel[WRITE].prof.tx_time));
    printk(KERN_INFO "Statistics for %s:\n%s",
    priv.channel[CTCM_WRITE].netdev.name, sbuf);
    kfree(sbuf);
    return;
    }
    static ssize_t stats_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct ccwgroup_device *gdev = to_ccwgroupdev(dev);
    struct ctcm_priv *priv = dev_get_drvdata(dev);
    if (!priv || gdev.state != CCWGROUP_ONLINE)
    return -ENODEV;
    ctcm_print_statistics(priv);
    return sysfs_emit(buf, "0\n");
    }
    static ssize_t stats_write(struct device *dev, struct device_attribute *attr,
    const char *buf, size_t count)
    {
    struct ctcm_priv *priv = dev_get_drvdata(dev);
    if (!priv)
    return -ENODEV;
// Reset statistics
    memset(&priv.channel[WRITE].prof, 0,
    sizeof(priv.channel[CTCM_WRITE].prof));
    return count;
    }
    static ssize_t ctcm_proto_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct ctcm_priv *priv = dev_get_drvdata(dev);
    if (!priv)
    return -ENODEV;
    return sysfs_emit(buf, "%d\n", priv.protocol);
    }
    static ssize_t ctcm_proto_store(struct device *dev,
    struct device_attribute *attr, const char *buf, size_t count)
    {
    int value, rc;
    struct ctcm_priv *priv = dev_get_drvdata(dev);
    if (!priv)
    return -ENODEV;
    rc = kstrtoint(buf, 0, &value);
    if (rc ||
    !((value == CTCM_PROTO_S390)  ||
    (value == CTCM_PROTO_LINUX) ||
    (value == CTCM_PROTO_MPC) ||
    (value == CTCM_PROTO_OS390)))
    return -EINVAL;
    priv.protocol = value;
    CTCM_DBF_DEV(SETUP, dev, buf);
    return count;
    }
    static const char *ctcm_type[] = {
    "not a channel",
    "CTC/A",
    "FICON channel",
    "ESCON channel",
    "unknown channel type",
    "unsupported channel type",
    };
    static ssize_t ctcm_type_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct ccwgroup_device *cgdev;
    cgdev = to_ccwgroupdev(dev);
    if (!cgdev)
    return -ENODEV;
    return sysfs_emit(buf, "%s\n",
    ctcm_type[cgdev.cdev[0].id.driver_info]);
    }
    static DEVICE_ATTR(buffer, 0644, ctcm_buffer_show, ctcm_buffer_write);
    static DEVICE_ATTR(protocol, 0644, ctcm_proto_show, ctcm_proto_store);
    static DEVICE_ATTR(type, 0444, ctcm_type_show, core::ptr::null_mut());
    static DEVICE_ATTR(stats, 0644, stats_show, stats_write);
    static struct attribute *ctcm_attr[] = {
    &dev_attr_protocol.attr,
    &dev_attr_type.attr,
    &dev_attr_buffer.attr,
    &dev_attr_stats.attr,
    core::ptr::null_mut(),
    };
    static struct attribute_group ctcm_attr_group = {
    .attrs = ctcm_attr,
    };
    const struct attribute_group *ctcm_attr_groups[] = {
    &ctcm_attr_group,
    core::ptr::null_mut(),
    };
