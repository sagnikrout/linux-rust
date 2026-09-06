//! Automatically rewritten from C to Rust
//! Source: drivers/soundwire/sysfs_slave.c
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
// Copyright(c) 2015-2020 Intel Corporation.

//
// Slave sysfs
//
// The sysfs for Slave reflects the MIPI description as given
// in the MIPI DisCo spec.
// status and device_number come directly from the MIPI SoundWire
// 1.x specification.
//
// Base file is device
// |---- status
// |---- device_number
// |---- modalias
// |---- dev-properties
// |---- mipi_revision
// |---- wake_capable
// |---- test_mode_capable
// |---- clk_stop_mode1
// |---- simple_clk_stop_capable
// |---- clk_stop_timeout
// |---- ch_prep_timeout
// |---- reset_behave
// |---- high_PHY_capable
// |---- paging_support
// |---- bank_delay_support
// |---- p15_behave
// |---- master_count
// |---- source_ports
// |---- sink_ports
// |---- dp0
// |---- max_word
// |---- min_word
// |---- words
// |---- BRA_flow_controlled
// |---- simple_ch_prep_sm
// |---- imp_def_interrupts
// |---- dpN_<sink/src>
// |---- max_word
// |---- min_word
// |---- words
// |---- type
// |---- max_grouping
// |---- simple_ch_prep_sm
// |---- ch_prep_timeout
// |---- imp_def_interrupts
// |---- min_ch
// |---- max_ch
// |---- channels
// |---- ch_combinations
// |---- max_async_buffer
// |---- block_pack_mode
// |---- port_encoding
//

    static ssize_t field##_show(struct device *dev,			\
    struct device_attribute *attr,	\
    char *buf)				\
    {								\
    struct sdw_slave *slave = dev_to_sdw_dev(dev);		\
    return sprintf(buf, format_string, slave.prop.field);	\
    }								\
#[no_mangle]
pub unsafe extern "C" fn DEVICE_ATTR_RO(_arg: field) -> static {
    static DEVICE_ATTR_RO(field)
    sdw_slave_attr(mipi_revision, "0x%x\n");
    sdw_slave_attr(wake_capable, "%d\n");
    sdw_slave_attr(test_mode_capable, "%d\n");
    sdw_slave_attr(clk_stop_mode1, "%d\n");
    sdw_slave_attr(simple_clk_stop_capable, "%d\n");
    sdw_slave_attr(clk_stop_timeout, "%d\n");
    sdw_slave_attr(ch_prep_timeout, "%d\n");
    sdw_slave_attr(reset_behave, "%d\n");
    sdw_slave_attr(high_PHY_capable, "%d\n");
    sdw_slave_attr(paging_support, "%d\n");
    sdw_slave_attr(bank_delay_support, "%d\n");
    sdw_slave_attr(p15_behave, "%d\n");
    sdw_slave_attr(master_count, "%d\n");
    sdw_slave_attr(source_ports, "0x%x\n");
    sdw_slave_attr(sink_ports, "0x%x\n");
    static ssize_t modalias_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct sdw_slave *slave = dev_to_sdw_dev(dev);
    return sdw_slave_modalias(slave, buf, 256);
    }
    static DEVICE_ATTR_RO(modalias);
    static struct attribute *slave_attrs[] = {
    &dev_attr_modalias.attr,
    core::ptr::null_mut(),
    };
    static const struct attribute_group slave_attr_group = {
    .attrs = slave_attrs,
    };
    static struct attribute *slave_dev_attrs[] = {
    &dev_attr_mipi_revision.attr,
    &dev_attr_wake_capable.attr,
    &dev_attr_test_mode_capable.attr,
    &dev_attr_clk_stop_mode1.attr,
    &dev_attr_simple_clk_stop_capable.attr,
    &dev_attr_clk_stop_timeout.attr,
    &dev_attr_ch_prep_timeout.attr,
    &dev_attr_reset_behave.attr,
    &dev_attr_high_PHY_capable.attr,
    &dev_attr_paging_support.attr,
    &dev_attr_bank_delay_support.attr,
    &dev_attr_p15_behave.attr,
    &dev_attr_master_count.attr,
    &dev_attr_source_ports.attr,
    &dev_attr_sink_ports.attr,
    core::ptr::null_mut(),
    };
    static const struct attribute_group sdw_slave_dev_attr_group = {
    .attrs	= slave_dev_attrs,
    .name = "dev-properties",
    };
//
// DP0 sysfs
//

    static ssize_t field##_show(struct device *dev,				\
    struct device_attribute *attr,		\
    char *buf)					\
    {									\
    struct sdw_slave *slave = dev_to_sdw_dev(dev);			\
    return sprintf(buf, format_string, slave.prop.dp0_prop.field);\
    }									\
#[no_mangle]
pub unsafe extern "C" fn DEVICE_ATTR_RO(_arg: field) -> static {
    static DEVICE_ATTR_RO(field)
    sdw_dp0_attr(max_word, "%d\n");
    sdw_dp0_attr(min_word, "%d\n");
    sdw_dp0_attr(BRA_flow_controlled, "%d\n");
    sdw_dp0_attr(simple_ch_prep_sm, "%d\n");
    sdw_dp0_attr(imp_def_interrupts, "0x%x\n");
    static ssize_t words_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct sdw_slave *slave = dev_to_sdw_dev(dev);
    let mut size: isize = 0;
    int i;
    for (i = 0; i < slave.prop.dp0_prop.num_words; i++)
    size += sprintf(buf + size, "%d ",
    slave.prop.dp0_prop.words[i]);
    size += sprintf(buf + size, "\n");
    return size;
    }
    static DEVICE_ATTR_RO(words);
    static struct attribute *dp0_attrs[] = {
    &dev_attr_max_word.attr,
    &dev_attr_min_word.attr,
    &dev_attr_words.attr,
    &dev_attr_BRA_flow_controlled.attr,
    &dev_attr_simple_ch_prep_sm.attr,
    &dev_attr_imp_def_interrupts.attr,
    core::ptr::null_mut(),
    };
    static umode_t dp0_attr_visible(struct kobject *kobj, struct attribute *attr,
    int n)
    {
    struct sdw_slave *slave = dev_to_sdw_dev(kobj_to_dev(kobj));
    if (slave.prop.dp0_prop)
    return attr.mode;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dp0_group_visible(kobj: *mut kobject) -> bool {
    static bool dp0_group_visible(struct kobject *kobj)
    {
    struct sdw_slave *slave = dev_to_sdw_dev(kobj_to_dev(kobj));
    if (slave.prop.dp0_prop)
    return true;
    return false;
    }
    DEFINE_SYSFS_GROUP_VISIBLE(dp0);
    static const struct attribute_group dp0_group = {
    .attrs = dp0_attrs,
    .is_visible = SYSFS_GROUP_VISIBLE(dp0),
    .name = "dp0",
    };
    const struct attribute_group *sdw_attr_groups[] = {
    &slave_attr_group,
    &sdw_slave_dev_attr_group,
    &dp0_group,
    core::ptr::null_mut(),
    };
//
// the status is shown in capital letters for UNATTACHED and RESERVED
// on purpose, to highlight users to the fact that these status values
// are not expected.
//
    static const char *const slave_status[] = {
    [SDW_SLAVE_UNATTACHED] =  "UNATTACHED",
    [SDW_SLAVE_ATTACHED] = "Attached",
    [SDW_SLAVE_ALERT] = "Alert",
    [SDW_SLAVE_RESERVED] = "RESERVED",
    };
    static ssize_t status_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct sdw_slave *slave = dev_to_sdw_dev(dev);
    return sprintf(buf, "%s\n", slave_status[slave.status]);
    }
    static DEVICE_ATTR_RO(status);
    static ssize_t device_number_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct sdw_slave *slave = dev_to_sdw_dev(dev);
    if (slave.status == SDW_SLAVE_UNATTACHED)
    return sprintf(buf, "%s", "N/A");
    else
    return sprintf(buf, "%d", slave.dev_num);
    }
    static DEVICE_ATTR_RO(device_number);
    static struct attribute *slave_status_attrs[] = {
    &dev_attr_status.attr,
    &dev_attr_device_number.attr,
    core::ptr::null_mut(),
    };
//
// we don't use ATTRIBUTES_GROUP here since the group is used in a
// separate file and can't be handled as a static.
//
    static const struct attribute_group sdw_slave_status_attr_group = {
    .attrs	= slave_status_attrs,
    };
    const struct attribute_group *sdw_slave_status_attr_groups[] = {
    &sdw_slave_status_attr_group,
    core::ptr::null_mut()
    };
