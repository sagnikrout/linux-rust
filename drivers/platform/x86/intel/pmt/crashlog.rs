//! Automatically rewritten from C to Rust
//! Source: drivers/platform/x86/intel/pmt/crashlog.c
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
// Intel Platform Monitoring Technology Crashlog driver
//
// Copyright (c) 2020, Intel Corporation.
// All Rights Reserved.
//
// Author: "Alexander Duyck" <alexander.h.duyck@linux.intel.com>
//

// Crashlog discovery header types
pub const CRASH_TYPE_OOBMSM: c_int = 1;
// Crashlog Discovery Header
pub const CONTROL_OFFSET: c_uint = 0x0;
pub const GUID_OFFSET: c_uint = 0x4;
pub const BASE_OFFSET: c_uint = 0x8;
pub const SIZE_OFFSET: c_uint = 0xC;

// size is in bytes

//
// Type 1 Version 0
// status and control registers are combined.
//
// Bits 29 and 30 control the state of bit 31.
// Bit 29 will clear bit 31, if set, allowing a new crashlog to be captured.
// Bit 30 will immediately trigger a crashlog to be generated, setting bit 31.
// Bit 31 is the read-only status with a 1 indicating log is complete.
//
pub const TYPE1_VER0_STATUS_OFFSET: c_uint = 0x00;
pub const TYPE1_VER0_CONTROL_OFFSET: c_uint = 0x00;

//
// Type 1 Version 2
// status and control are different registers
//
pub const TYPE1_VER2_STATUS_OFFSET: c_uint = 0x00;
pub const TYPE1_VER2_CONTROL_OFFSET: c_uint = 0x14;
// status register

// control register

    (TYPE1_VER2_EXECUTE | TYPE1_VER2_CLEAR | TYPE1_VER2_DISABLE)
// After offset, order alphabetically, not bit ordered
#[repr(C)]
#[derive(Copy, Clone)]
pub struct crashlog_status {
    pub offset: u32,
    pub clear_supported: u32,
    pub cleared: u32,
    pub complete: u32,
    pub consumed: u32,
    pub disabled: u32,
    pub error: u32,
    pub in_progress: u32,
    pub rearmed: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct crashlog_control {
    pub offset: u32,
    pub trigger_mask: u32,
    pub clear: u32,
    pub consume: u32,
    pub disable: u32,
    pub manual: u32,
    pub rearm: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct crashlog_info {
    pub status: crashlog_status,
    pub control: crashlog_control,
    pub attr_grp: *const attribute_group,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct crashlog_entry {
// entry must be first member of struct
    pub entry: intel_pmt_entry,
    pub control_mutex: mutex,
    pub info: *const crashlog_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pmt_crashlog_priv {
    pub num_entries: c_int,
    pub entry: [crashlog_entry; ],
}

//
// I/O
//
// Read, modify, write the control register, setting or clearing @bit based on @set
#[no_mangle]
unsafe extern "C" fn pmt_crashlog_rmw(crashlog: *mut crashlog_entry, bit: u32, set: bool) {
    static void pmt_crashlog_rmw(struct crashlog_entry *crashlog, u32 bit, bool set)
    {
    const struct crashlog_control *control = &crashlog.info.control;
    struct intel_pmt_entry *entry = &crashlog.entry;
    let mut reg: u32 = readl(entry.disc_table + control.offset);
    reg &= ~control.trigger_mask;
    if (set)
    reg |= bit;
    else
    reg &= ~bit;
    writel(reg, entry.disc_table + control.offset);
    }
// Read the status register and see if the specified @bit is set
#[no_mangle]
unsafe extern "C" fn pmt_crashlog_rc(crashlog: *mut crashlog_entry, bit: u32) -> bool {
    static bool pmt_crashlog_rc(struct crashlog_entry *crashlog, u32 bit)
    {
    const struct crashlog_status *status = &crashlog.info.status;
    let mut reg: u32 = readl(crashlog.entry.disc_table + status.offset);
    return !!(reg & bit);
    }
#[no_mangle]
unsafe extern "C" fn pmt_crashlog_complete(crashlog: *mut crashlog_entry) -> bool {
    static bool pmt_crashlog_complete(struct crashlog_entry *crashlog)
    {
// return current value of the crashlog complete flag
    return pmt_crashlog_rc(crashlog, crashlog.info.status.complete);
    }
#[no_mangle]
unsafe extern "C" fn pmt_crashlog_disabled(crashlog: *mut crashlog_entry) -> bool {
    static bool pmt_crashlog_disabled(struct crashlog_entry *crashlog)
    {
// return current value of the crashlog disabled flag
    return pmt_crashlog_rc(crashlog, crashlog.info.status.disabled);
    }
#[no_mangle]
unsafe extern "C" fn pmt_crashlog_supported(entry: *mut intel_pmt_entry, crash_type: *mut u32, version: *mut u32) -> bool {
    static bool pmt_crashlog_supported(struct intel_pmt_entry *entry, u32 *crash_type, u32 *version)
    {
    let mut discovery_header: u32 = readl(entry.disc_table + CONTROL_OFFSET);
// crash_type = GET_TYPE(discovery_header);
// version = GET_VERSION(discovery_header);
//
// Currently we only recognize OOBMSM (type 1) and version 0 or 2
// devices.
//
// Ignore all other crashlog devices in the system.
//
    if (*crash_type == CRASH_TYPE_OOBMSM && (*version == 0 || *version == 2))
    return true;
    return false;
    }
    static void pmt_crashlog_set_disable(struct crashlog_entry *crashlog,
    bool disable)
    {
    pmt_crashlog_rmw(crashlog, crashlog.info.control.disable, disable);
    }
#[no_mangle]
unsafe extern "C" fn pmt_crashlog_set_clear(crashlog: *mut crashlog_entry) {
    static void pmt_crashlog_set_clear(struct crashlog_entry *crashlog)
    {
    pmt_crashlog_rmw(crashlog, crashlog.info.control.clear, true);
    }
#[no_mangle]
unsafe extern "C" fn pmt_crashlog_set_execute(crashlog: *mut crashlog_entry) {
    static void pmt_crashlog_set_execute(struct crashlog_entry *crashlog)
    {
    pmt_crashlog_rmw(crashlog, crashlog.info.control.manual, true);
    }
#[no_mangle]
unsafe extern "C" fn pmt_crashlog_cleared(crashlog: *mut crashlog_entry) -> bool {
    static bool pmt_crashlog_cleared(struct crashlog_entry *crashlog)
    {
    return pmt_crashlog_rc(crashlog, crashlog.info.status.cleared);
    }
#[no_mangle]
unsafe extern "C" fn pmt_crashlog_consumed(crashlog: *mut crashlog_entry) -> bool {
    static bool pmt_crashlog_consumed(struct crashlog_entry *crashlog)
    {
    return pmt_crashlog_rc(crashlog, crashlog.info.status.consumed);
    }
#[no_mangle]
unsafe extern "C" fn pmt_crashlog_set_consumed(crashlog: *mut crashlog_entry) {
    static void pmt_crashlog_set_consumed(struct crashlog_entry *crashlog)
    {
    pmt_crashlog_rmw(crashlog, crashlog.info.control.consume, true);
    }
#[no_mangle]
unsafe extern "C" fn pmt_crashlog_error(crashlog: *mut crashlog_entry) -> bool {
    static bool pmt_crashlog_error(struct crashlog_entry *crashlog)
    {
    return pmt_crashlog_rc(crashlog, crashlog.info.status.error);
    }
#[no_mangle]
unsafe extern "C" fn pmt_crashlog_rearm(crashlog: *mut crashlog_entry) -> bool {
    static bool pmt_crashlog_rearm(struct crashlog_entry *crashlog)
    {
    return pmt_crashlog_rc(crashlog, crashlog.info.status.rearmed);
    }
#[no_mangle]
unsafe extern "C" fn pmt_crashlog_set_rearm(crashlog: *mut crashlog_entry) {
    static void pmt_crashlog_set_rearm(struct crashlog_entry *crashlog)
    {
    pmt_crashlog_rmw(crashlog, crashlog.info.control.rearm, true);
    }
//
// sysfs
//
    static ssize_t
    clear_show(struct device *dev, struct device_attribute *attr, char *buf)
    {
    struct crashlog_entry *crashlog = dev_get_drvdata(dev);
    let mut cleared: bool = pmt_crashlog_cleared(crashlog);
    return sysfs_emit(buf, "%d\n", cleared);
    }
    static ssize_t
    clear_store(struct device *dev, struct device_attribute *attr,
    const char *buf, size_t count)
    {
    struct crashlog_entry *crashlog;
    bool clear;
    int result;
    crashlog = dev_get_drvdata(dev);
    result = kstrtobool(buf, &clear);
    if (result)
    return result;
// set bit only
    if (!clear)
    return -EINVAL;
    guard(mutex)(&crashlog.control_mutex);
    pmt_crashlog_set_clear(crashlog);
    return count;
    }
    static DEVICE_ATTR_RW(clear);
    static ssize_t
    consumed_show(struct device *dev, struct device_attribute *attr, char *buf)
    {
    struct crashlog_entry *crashlog = dev_get_drvdata(dev);
    let mut consumed: bool = pmt_crashlog_consumed(crashlog);
    return sysfs_emit(buf, "%d\n", consumed);
    }
    static ssize_t
    consumed_store(struct device *dev, struct device_attribute *attr, const char *buf,
    size_t count)
    {
    struct crashlog_entry *crashlog;
    bool consumed;
    int result;
    crashlog = dev_get_drvdata(dev);
    result = kstrtobool(buf, &consumed);
    if (result)
    return result;
// set bit only
    if (!consumed)
    return -EINVAL;
    guard(mutex)(&crashlog.control_mutex);
    if (pmt_crashlog_disabled(crashlog))
    return -EBUSY;
    if (!pmt_crashlog_complete(crashlog))
    return -EEXIST;
    pmt_crashlog_set_consumed(crashlog);
    return count;
    }
    static DEVICE_ATTR_RW(consumed);
    static ssize_t
    enable_show(struct device *dev, struct device_attribute *attr, char *buf)
    {
    struct crashlog_entry *crashlog = dev_get_drvdata(dev);
    let mut enabled: bool = !pmt_crashlog_disabled(crashlog);
    return sprintf(buf, "%d\n", enabled);
    }
    static ssize_t
    enable_store(struct device *dev, struct device_attribute *attr,
    const char *buf, size_t count)
    {
    struct crashlog_entry *crashlog;
    bool enabled;
    int result;
    crashlog = dev_get_drvdata(dev);
    result = kstrtobool(buf, &enabled);
    if (result)
    return result;
    guard(mutex)(&crashlog.control_mutex);
    pmt_crashlog_set_disable(crashlog, !enabled);
    return count;
    }
    static DEVICE_ATTR_RW(enable);
    static ssize_t
    error_show(struct device *dev, struct device_attribute *attr, char *buf)
    {
    struct crashlog_entry *crashlog = dev_get_drvdata(dev);
    let mut error: bool = pmt_crashlog_error(crashlog);
    return sysfs_emit(buf, "%d\n", error);
    }
    static DEVICE_ATTR_RO(error);
    static ssize_t
    rearm_show(struct device *dev, struct device_attribute *attr, char *buf)
    {
    struct crashlog_entry *crashlog = dev_get_drvdata(dev);
    let mut rearmed: c_int = pmt_crashlog_rearm(crashlog);
    return sysfs_emit(buf, "%d\n", rearmed);
    }
    static ssize_t
    rearm_store(struct device *dev, struct device_attribute *attr, const char *buf,
    size_t count)
    {
    struct crashlog_entry *crashlog;
    bool rearm;
    int result;
    crashlog = dev_get_drvdata(dev);
    result = kstrtobool(buf, &rearm);
    if (result)
    return result;
// set only
    if (!rearm)
    return -EINVAL;
    guard(mutex)(&crashlog.control_mutex);
    pmt_crashlog_set_rearm(crashlog);
    return count;
    }
    static DEVICE_ATTR_RW(rearm);
    static ssize_t
    trigger_show(struct device *dev, struct device_attribute *attr, char *buf)
    {
    struct crashlog_entry *crashlog;
    bool trigger;
    crashlog = dev_get_drvdata(dev);
    trigger = pmt_crashlog_complete(crashlog);
    return sprintf(buf, "%d\n", trigger);
    }
    static ssize_t
    trigger_store(struct device *dev, struct device_attribute *attr,
    const char *buf, size_t count)
    {
    struct crashlog_entry *crashlog;
    bool trigger;
    int result;
    crashlog = dev_get_drvdata(dev);
    result = kstrtobool(buf, &trigger);
    if (result)
    return result;
    guard(mutex)(&crashlog.control_mutex);
// if device is currently disabled, return busy
    if (pmt_crashlog_disabled(crashlog))
    return -EBUSY;
    if (!trigger) {
    pmt_crashlog_set_clear(crashlog);
    return count;
    }
// we cannot trigger a new crash if one is still pending
    if (pmt_crashlog_complete(crashlog))
    return -EEXIST;
    pmt_crashlog_set_execute(crashlog);
    return count;
    }
    static DEVICE_ATTR_RW(trigger);
    static struct attribute *pmt_crashlog_type1_ver0_attrs[] = {
    &dev_attr_enable.attr,
    &dev_attr_trigger.attr,
    core::ptr::null_mut()
    };
    static struct attribute *pmt_crashlog_type1_ver2_attrs[] = {
    &dev_attr_clear.attr,
    &dev_attr_consumed.attr,
    &dev_attr_enable.attr,
    &dev_attr_error.attr,
    &dev_attr_rearm.attr,
    &dev_attr_trigger.attr,
    core::ptr::null_mut()
    };
    static const struct attribute_group pmt_crashlog_type1_ver0_group = {
    .attrs	= pmt_crashlog_type1_ver0_attrs,
    };
    static const struct attribute_group pmt_crashlog_type1_ver2_group = {
    .attrs = pmt_crashlog_type1_ver2_attrs,
    };
    static const struct crashlog_info crashlog_type1_ver0 = {
    .status.offset = TYPE1_VER0_STATUS_OFFSET,
    .status.cleared = TYPE1_VER0_CLEAR,
    .status.complete = TYPE1_VER0_COMPLETE,
    .status.disabled = TYPE1_VER0_DISABLE,
    .control.offset = TYPE1_VER0_CONTROL_OFFSET,
    .control.trigger_mask = TYPE1_VER0_TRIGGER_MASK,
    .control.clear = TYPE1_VER0_CLEAR,
    .control.disable = TYPE1_VER0_DISABLE,
    .control.manual = TYPE1_VER0_EXECUTE,
    .attr_grp = &pmt_crashlog_type1_ver0_group,
    };
    static const struct crashlog_info crashlog_type1_ver2 = {
    .status.offset = TYPE1_VER2_STATUS_OFFSET,
    .status.clear_supported = TYPE1_VER2_CLEAR_SUPPORT,
    .status.cleared = TYPE1_VER2_CLEARED,
    .status.complete = TYPE1_VER2_COMPLETE,
    .status.consumed = TYPE1_VER2_CONSUMED,
    .status.disabled = TYPE1_VER2_DISABLED,
    .status.error = TYPE1_VER2_ERROR,
    .status.in_progress = TYPE1_VER2_IN_PROGRESS,
    .status.rearmed = TYPE1_VER2_REARMED,
    .control.offset = TYPE1_VER2_CONTROL_OFFSET,
    .control.trigger_mask = TYPE1_VER2_TRIGGER_MASK,
    .control.clear = TYPE1_VER2_CLEAR,
    .control.consume = TYPE1_VER2_CONSUME,
    .control.disable = TYPE1_VER2_DISABLE,
    .control.manual = TYPE1_VER2_EXECUTE,
    .control.rearm = TYPE1_VER2_REARM,
    .attr_grp = &pmt_crashlog_type1_ver2_group,
    };
    static const struct crashlog_info *select_crashlog_info(u32 type, u32 version)
    {
    if (version == 0)
    return &crashlog_type1_ver0;
    return &crashlog_type1_ver2;
    }
    static int pmt_crashlog_pre_decode(struct intel_vsec_device *ivdev,
    struct intel_pmt_entry *entry)
    {
    struct crashlog_entry *crashlog;
    u32 version;
    u32 type;
    if (!pmt_crashlog_supported(entry, &type, &version))
    return 1;
// initialize the crashlog struct
    crashlog = container_of(entry, struct crashlog_entry, entry);
    mutex_init(&crashlog.control_mutex);
    crashlog.info = select_crashlog_info(type, version);
    entry.attr_grp = crashlog.info.attr_grp;
    return 0;
    }
    static int pmt_crashlog_header_decode(struct intel_pmt_entry *entry,
    struct device *dev)
    {
    void __iomem *disc_table = entry.disc_table;
    struct intel_pmt_header *header = &entry.header;
    header.access_type = GET_ACCESS(readl(disc_table));
    header.guid = readl(disc_table + GUID_OFFSET);
    header.base_offset = readl(disc_table + BASE_OFFSET);
// Size is measured in DWORDS, but accessor returns bytes
    header.size = GET_SIZE(readl(disc_table + SIZE_OFFSET));
    return 0;
    }
    static DEFINE_XARRAY_ALLOC(crashlog_array);
    static struct intel_pmt_namespace pmt_crashlog_ns = {
    .name = "crashlog",
    .xa = &crashlog_array,
    .pmt_pre_decode = pmt_crashlog_pre_decode,
    .pmt_header_decode = pmt_crashlog_header_decode,
    };
//
// initialization
//
#[no_mangle]
unsafe extern "C" fn pmt_crashlog_remove(auxdev: *mut auxiliary_device) {
    static void pmt_crashlog_remove(struct auxiliary_device *auxdev)
    {
    struct pmt_crashlog_priv *priv = auxiliary_get_drvdata(auxdev);
    int i;
    for (i = 0; i < priv.num_entries; i++) {
    struct crashlog_entry *crashlog = &priv.entry[i];
    intel_pmt_dev_destroy(&crashlog.entry, &pmt_crashlog_ns);
    mutex_destroy(&crashlog.control_mutex);
    }
    }
    static int pmt_crashlog_probe(struct auxiliary_device *auxdev,
    const struct auxiliary_device_id *id)
    {
    struct intel_vsec_device *intel_vsec_dev = auxdev_to_ivdev(auxdev);
    struct pmt_crashlog_priv *priv;
    size_t size;
    int i, ret;
    size = struct_size(priv, entry, intel_vsec_dev.num_resources);
    priv = devm_kzalloc(&auxdev.dev, size, GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    auxiliary_set_drvdata(auxdev, priv);
    for (i = 0; i < intel_vsec_dev.num_resources; i++) {
    struct intel_pmt_entry *entry = &priv.entry[priv.num_entries].entry;
    ret = intel_pmt_dev_create(entry, &pmt_crashlog_ns, intel_vsec_dev, i);
    if (ret < 0)
    goto abort_probe;
    if (ret)
    continue;
    priv.num_entries++;
    }
    return 0;
    abort_probe:
    pmt_crashlog_remove(auxdev);
    return ret;
    }
    static const struct auxiliary_device_id pmt_crashlog_id_table[] = {
    { .name = "intel_vsec.crashlog" },
    {}
    };
    MODULE_DEVICE_TABLE(auxiliary, pmt_crashlog_id_table);
    static struct auxiliary_driver pmt_crashlog_aux_driver = {
    .id_table	= pmt_crashlog_id_table,
    .remove		= pmt_crashlog_remove,
    .probe		= pmt_crashlog_probe,
    };
#[no_mangle]
unsafe extern "C" fn pmt_crashlog_init() -> int __init {
    static int __init pmt_crashlog_init(void)
    {
    return auxiliary_driver_register(&pmt_crashlog_aux_driver);
    }
#[no_mangle]
unsafe extern "C" fn pmt_crashlog_exit() -> void __exit {
    static void __exit pmt_crashlog_exit(void)
    {
    auxiliary_driver_unregister(&pmt_crashlog_aux_driver);
    xa_destroy(&crashlog_array);
    }
    module_init(pmt_crashlog_init);
    module_exit(pmt_crashlog_exit);
    MODULE_AUTHOR("Alexander Duyck <alexander.h.duyck@linux.intel.com>");
    MODULE_DESCRIPTION("Intel PMT Crashlog driver");
    MODULE_LICENSE("GPL v2");
    MODULE_IMPORT_NS("INTEL_PMT");
