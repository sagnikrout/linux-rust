//! Automatically rewritten from C to Rust
//! Source: drivers/platform/x86/intel/sdsi.c
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
// Intel On Demand (Software Defined Silicon) driver
//
// Copyright (c) 2022, Intel Corporation.
// All Rights Reserved.
//
// Author: "David E. Box" <david.e.box@linux.intel.com>
//

pub const ACCESS_TYPE_BARID: c_int = 2;
pub const ACCESS_TYPE_LOCAL: c_int = 3;
pub const SDSI_MIN_SIZE_DWORDS: c_int = 276;
pub const SDSI_SIZE_MAILBOX: c_int = 1024;
pub const SDSI_SIZE_REGS: c_int = 80;

//
// Write messages are currently up to the size of the mailbox
// while read messages are up to 4 times the size of the
// mailbox, sent in packets
//

pub const SDSI_ENABLED_FEATURES_OFFSET: c_int = 16;

pub const SDSI_SOCKET_ID_OFFSET: c_int = 64;

pub const SDSI_MBOX_CMD_SUCCESS: c_uint = 0x40;
pub const SDSI_MBOX_CMD_TIMEOUT: c_uint = 0x80;
pub const MBOX_TIMEOUT_US: c_int = 500000;
pub const MBOX_TIMEOUT_ACQUIRE_US: c_int = 1000;
pub const MBOX_POLLING_PERIOD_US: c_int = 100;
pub const MBOX_ACQUIRE_NUM_RETRIES: c_int = 5;
pub const MBOX_ACQUIRE_RETRY_DELAY_MS: c_int = 500;
pub const MBOX_MAX_PACKETS: c_int = 4;
pub const MBOX_OWNER_NONE: c_uint = 0x00;
pub const MBOX_OWNER_INBAND: c_uint = 0x01;

pub const DISC_TABLE_SIZE: c_int = 12;

pub const SDSI_GUID_V1: c_uint = 0x006DD191;
pub const GUID_V1_CNTRL_SIZE: c_int = 8;
pub const GUID_V1_REGS_SIZE: c_int = 72;
pub const SDSI_GUID_V2: c_uint = 0xF210D9EF;
pub const GUID_V2_CNTRL_SIZE: c_int = 16;
pub const GUID_V2_REGS_SIZE: c_int = 80;
    enum sdsi_command {
    SDSI_CMD_PROVISION_AKC		= 0x0004,
    SDSI_CMD_PROVISION_CAP		= 0x0008,
    SDSI_CMD_READ_STATE		= 0x0010,
    SDSI_CMD_READ_METER		= 0x0014,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sdsi_mbox_info {
    pub payload: *mut u64,
    pub buffer: *mut c_void,
    pub control_flags: u64,
    pub size: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct disc_table {
    pub access_info: u32,
    pub guid: u32,
    pub offset: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sdsi_priv {
    pub /: *mut *mut mutex mb_lock; / Mailbox access lock,
    pub dev: *mut device,
    pub control_addr: *mut void __iomem,
    pub mbox_addr: *mut void __iomem,
    pub regs_addr: *mut void __iomem,
    pub control_size: c_int,
    pub maibox_size: c_int,
    pub registers_size: c_int,
    pub guid: u32,
    pub features: u32,
}

// SDSi mailbox operations must be performed using 64bit mov instructions
    static __always_inline void
    sdsi_memcpy64_toio(u64 __iomem *to, const u64 *from, size_t count_bytes)
    {
    let mut count: usize = count_bytes / sizeof(*to);
    int i;
    for (i = 0; i < count; i++)
    writeq(from[i], &to[i]);
    }
    static __always_inline void
    sdsi_memcpy64_fromio(u64 *to, const u64 __iomem *from, size_t count_bytes)
    {
    let mut count: usize = count_bytes / sizeof(*to);
    int i;
    for (i = 0; i < count; i++)
    to[i] = readq(&from[i]);
    }
#[no_mangle]
pub unsafe extern "C" fn sdsi_complete_transaction(priv: *mut sdsi_priv) {
    static inline void sdsi_complete_transaction(struct sdsi_priv *priv)
    {
    let mut control: u64 = FIELD_PREP(CTRL_COMPLETE, 1);
    lockdep_assert_held(&priv.mb_lock);
    writeq(control, priv.control_addr);
    }
#[no_mangle]
unsafe extern "C" fn sdsi_status_to_errno(status: u32) -> c_int {
    static int sdsi_status_to_errno(u32 status)
    {
    switch (status) {
    case SDSI_MBOX_CMD_SUCCESS:
    return 0;
    case SDSI_MBOX_CMD_TIMEOUT:
    return -ETIMEDOUT;
    default:
    return -EIO;
    }
    }
    static int sdsi_mbox_poll(struct sdsi_priv *priv, struct sdsi_mbox_info *info,
    size_t *data_size)
    {
    struct device *dev = priv.dev;
    u32 total, loop, eom, status, message_size;
    u64 control;
    int ret;
    lockdep_assert_held(&priv.mb_lock);
// For reads, data sizes that are larger than the mailbox size are read in packets.
    total = 0;
    loop = 0;
    do {
    u32 packet_size;
// Poll on ready bit
    ret = readq_poll_timeout(priv.control_addr, control, control & CTRL_READY,
    MBOX_POLLING_PERIOD_US, MBOX_TIMEOUT_US);
    if (ret)
    break;
    eom = FIELD_GET(CTRL_EOM, control);
    status = FIELD_GET(CTRL_STATUS, control);
    packet_size = FIELD_GET(CTRL_PACKET_SIZE, control);
    message_size = FIELD_GET(CTRL_MSG_SIZE, control);
    ret = sdsi_status_to_errno(status);
    if (ret)
    break;
    if (!packet_size) {
    sdsi_complete_transaction(priv);
    break;
    }
// Only the last packet can be less than the mailbox size.
    if (!eom && packet_size != SDSI_SIZE_MAILBOX) {
    dev_err(dev, "Invalid packet size\n");
    ret = -EPROTO;
    break;
    }
    if (packet_size > SDSI_SIZE_MAILBOX) {
    dev_err(dev, "Packet size too large\n");
    ret = -EPROTO;
    break;
    }
    if (info.buffer) {
    void *buf = info.buffer + array_size(SDSI_SIZE_MAILBOX, loop);
    sdsi_memcpy64_fromio(buf, priv.mbox_addr,
    round_up(packet_size, SDSI_SIZE_CMD));
    total += packet_size;
    }
    sdsi_complete_transaction(priv);
    } while (!eom && ++loop < MBOX_MAX_PACKETS);
    if (ret) {
    sdsi_complete_transaction(priv);
    return ret;
    }
    if (!eom) {
    dev_err(dev, "Exceeded read attempts\n");
    return -EPROTO;
    }
// Message size check is only valid for multi-packet transfers
    if (loop && total != message_size)
    dev_warn(dev, "Read count %u differs from expected count %u\n",
    total, message_size);
    if (data_size)
// data_size = total;
    return 0;
    }
    static int sdsi_mbox_cmd_read(struct sdsi_priv *priv, struct sdsi_mbox_info *info,
    size_t *data_size)
    {
    u64 control;
    lockdep_assert_held(&priv.mb_lock);
// Format and send the read command
    control = FIELD_PREP(CTRL_EOM, 1) |
    FIELD_PREP(CTRL_SOM, 1) |
    FIELD_PREP(CTRL_RUN_BUSY, 1) |
    FIELD_PREP(CTRL_PACKET_SIZE, info.size) |
    info.control_flags;
    writeq(control, priv.control_addr);
    return sdsi_mbox_poll(priv, info, data_size);
    }
    static int sdsi_mbox_cmd_write(struct sdsi_priv *priv, struct sdsi_mbox_info *info,
    size_t *data_size)
    {
    u64 control;
    lockdep_assert_held(&priv.mb_lock);
// Write rest of the payload
    sdsi_memcpy64_toio(priv.mbox_addr + SDSI_SIZE_CMD, info.payload + 1,
    info.size - SDSI_SIZE_CMD);
// Format and send the write command
    control = FIELD_PREP(CTRL_EOM, 1) |
    FIELD_PREP(CTRL_SOM, 1) |
    FIELD_PREP(CTRL_RUN_BUSY, 1) |
    FIELD_PREP(CTRL_READ_WRITE, 1) |
    FIELD_PREP(CTRL_MSG_SIZE, info.size) |
    FIELD_PREP(CTRL_PACKET_SIZE, info.size);
    writeq(control, priv.control_addr);
    return sdsi_mbox_poll(priv, info, data_size);
    }
#[no_mangle]
unsafe extern "C" fn sdsi_mbox_acquire(priv: *mut sdsi_priv, info: *mut sdsi_mbox_info) -> c_int {
    static int sdsi_mbox_acquire(struct sdsi_priv *priv, struct sdsi_mbox_info *info)
    {
    u64 control;
    u32 owner;
    int ret, retries = 0;
    lockdep_assert_held(&priv.mb_lock);
// Check mailbox is available
    control = readq(priv.control_addr);
    owner = FIELD_GET(CTRL_OWNER, control);
    if (owner != MBOX_OWNER_NONE)
    return -EBUSY;
//
// If there has been no recent transaction and no one owns the mailbox,
// we should acquire it in under 1ms. However, if we've accessed it
// recently it may take up to 2.1 seconds to acquire it again.
//
    do {
// Write first qword of payload
    writeq(info.payload[0], priv.mbox_addr);
// Check for ownership
    ret = readq_poll_timeout(priv.control_addr, control,
    FIELD_GET(CTRL_OWNER, control) == MBOX_OWNER_INBAND,
    MBOX_POLLING_PERIOD_US, MBOX_TIMEOUT_ACQUIRE_US);
    if (FIELD_GET(CTRL_OWNER, control) == MBOX_OWNER_NONE &&
    retries++ < MBOX_ACQUIRE_NUM_RETRIES) {
    msleep(MBOX_ACQUIRE_RETRY_DELAY_MS);
    continue;
    }
// Either we got it or someone else did.
    break;
    } while (true);
    return ret;
    }
    static int sdsi_mbox_write(struct sdsi_priv *priv, struct sdsi_mbox_info *info,
    size_t *data_size)
    {
    int ret;
    lockdep_assert_held(&priv.mb_lock);
    ret = sdsi_mbox_acquire(priv, info);
    if (ret)
    return ret;
    return sdsi_mbox_cmd_write(priv, info, data_size);
    }
#[no_mangle]
unsafe extern "C" fn sdsi_mbox_read(priv: *mut sdsi_priv, info: *mut sdsi_mbox_info, data_size: *mut usize) -> c_int {
    static int sdsi_mbox_read(struct sdsi_priv *priv, struct sdsi_mbox_info *info, size_t *data_size)
    {
    int ret;
    lockdep_assert_held(&priv.mb_lock);
    ret = sdsi_mbox_acquire(priv, info);
    if (ret)
    return ret;
    return sdsi_mbox_cmd_read(priv, info, data_size);
    }
#[no_mangle]
unsafe extern "C" fn sdsi_ib_locked(priv: *mut sdsi_priv) -> bool {
    static bool sdsi_ib_locked(struct sdsi_priv *priv)
    {
    return !!FIELD_GET(CTRL_INBAND_LOCK, readq(priv.control_addr));
    }
    static ssize_t sdsi_provision(struct sdsi_priv *priv, char *buf, size_t count,
    enum sdsi_command command)
    {
    let mut info: sdsi_mbox_info = {};
    int ret;
    if (count > (SDSI_SIZE_WRITE_MSG - SDSI_SIZE_CMD))
    return -EOVERFLOW;
// Make sure In-band lock is not set
    if (sdsi_ib_locked(priv))
    return -EPERM;
// Qword aligned message + command qword
    info.size = round_up(count, SDSI_SIZE_CMD) + SDSI_SIZE_CMD;
    info.payload = kzalloc(info.size, GFP_KERNEL);
    if (!info.payload)
    return -ENOMEM;
// Copy message to payload buffer
    memcpy(info.payload, buf, count);
// Command is last qword of payload buffer
    info.payload[(info.size - SDSI_SIZE_CMD) / SDSI_SIZE_CMD] = command;
    ret = mutex_lock_interruptible(&priv.mb_lock);
    if (ret)
    goto free_payload;
    ret = sdsi_mbox_write(priv, &info, core::ptr::null_mut());
    mutex_unlock(&priv.mb_lock);
    free_payload:
    kfree(info.payload);
    if (ret)
    return ret;
    return count;
    }
    static ssize_t provision_akc_write(struct file *filp, struct kobject *kobj,
    const struct bin_attribute *attr, char *buf,
    loff_t off, size_t count)
    {
    struct device *dev = kobj_to_dev(kobj);
    struct sdsi_priv *priv = dev_get_drvdata(dev);
    if (off)
    return -ESPIPE;
    return sdsi_provision(priv, buf, count, SDSI_CMD_PROVISION_AKC);
    }
    static const BIN_ATTR_WO(provision_akc, SDSI_SIZE_WRITE_MSG);
    static ssize_t provision_cap_write(struct file *filp, struct kobject *kobj,
    const struct bin_attribute *attr, char *buf,
    loff_t off, size_t count)
    {
    struct device *dev = kobj_to_dev(kobj);
    struct sdsi_priv *priv = dev_get_drvdata(dev);
    if (off)
    return -ESPIPE;
    return sdsi_provision(priv, buf, count, SDSI_CMD_PROVISION_CAP);
    }
    static const BIN_ATTR_WO(provision_cap, SDSI_SIZE_WRITE_MSG);
    static ssize_t
    certificate_read(u64 command, u64 control_flags, struct sdsi_priv *priv,
    char *buf, loff_t off, size_t count)
    {
    let mut info: sdsi_mbox_info = {};
    size_t size;
    int ret;
    if (off)
    return 0;
// Buffer for return data
    info.buffer = kmalloc(SDSI_SIZE_READ_MSG, GFP_KERNEL);
    if (!info.buffer)
    return -ENOMEM;
    info.payload = &command;
    info.size = sizeof(command);
    info.control_flags = control_flags;
    ret = mutex_lock_interruptible(&priv.mb_lock);
    if (ret)
    goto free_buffer;
    ret = sdsi_mbox_read(priv, &info, &size);
    mutex_unlock(&priv.mb_lock);
    if (ret < 0)
    goto free_buffer;
    if (size > count)
    size = count;
    memcpy(buf, info.buffer, size);
    free_buffer:
    kfree(info.buffer);
    if (ret)
    return ret;
    return size;
    }
    static ssize_t
    state_certificate_read(struct file *filp, struct kobject *kobj,
    const struct bin_attribute *attr, char *buf, loff_t off,
    size_t count)
    {
    struct device *dev = kobj_to_dev(kobj);
    struct sdsi_priv *priv = dev_get_drvdata(dev);
    return certificate_read(SDSI_CMD_READ_STATE, 0, priv, buf, off, count);
    }
    static const BIN_ATTR_ADMIN_RO(state_certificate, SDSI_SIZE_READ_MSG);
    static ssize_t
    meter_certificate_read(struct file *filp, struct kobject *kobj,
    const struct bin_attribute *attr, char *buf, loff_t off,
    size_t count)
    {
    struct device *dev = kobj_to_dev(kobj);
    struct sdsi_priv *priv = dev_get_drvdata(dev);
    return certificate_read(SDSI_CMD_READ_METER, 0, priv, buf, off, count);
    }
    static const BIN_ATTR_ADMIN_RO(meter_certificate, SDSI_SIZE_READ_MSG);
    static ssize_t
    meter_current_read(struct file *filp, struct kobject *kobj,
    const struct bin_attribute *attr, char *buf, loff_t off,
    size_t count)
    {
    struct device *dev = kobj_to_dev(kobj);
    struct sdsi_priv *priv = dev_get_drvdata(dev);
    return certificate_read(SDSI_CMD_READ_METER, CTRL_METER_ENABLE_DRAM,
    priv, buf, off, count);
    }
    static const BIN_ATTR_ADMIN_RO(meter_current, SDSI_SIZE_READ_MSG);
    static ssize_t registers_read(struct file *filp, struct kobject *kobj,
    const struct bin_attribute *attr, char *buf,
    loff_t off, size_t count)
    {
    struct device *dev = kobj_to_dev(kobj);
    struct sdsi_priv *priv = dev_get_drvdata(dev);
    void __iomem *addr = priv.regs_addr;
    let mut size: c_int = priv.registers_size;
//
// The check below is performed by the sysfs caller based on the static
// file size. But this may be greater than the actual size which is based
// on the GUID. So check here again based on actual size before reading.
//
    if (off >= size)
    return 0;
    if (off + count > size)
    count = size - off;
    memcpy_fromio(buf, addr + off, count);
    return count;
    }
    static const BIN_ATTR_ADMIN_RO(registers, SDSI_SIZE_REGS);
    static const struct bin_attribute *const sdsi_bin_attrs[] = {
    &bin_attr_registers,
    &bin_attr_state_certificate,
    &bin_attr_meter_certificate,
    &bin_attr_meter_current,
    &bin_attr_provision_akc,
    &bin_attr_provision_cap,
    core::ptr::null_mut()
    };
    static umode_t
    sdsi_battr_is_visible(struct kobject *kobj, const struct bin_attribute *attr, int n)
    {
    struct device *dev = kobj_to_dev(kobj);
    struct sdsi_priv *priv = dev_get_drvdata(dev);
// Registers file is always readable if the device is present
    if (attr == &bin_attr_registers)
    return attr.attr.mode;
// All other attributes not visible if BIOS has not enabled On Demand
    if (!(priv.features & SDSI_FEATURE_SDSI))
    return 0;
    if (attr == &bin_attr_meter_certificate || attr == &bin_attr_meter_current)
    return (priv.features & SDSI_FEATURE_METERING) ?
    attr.attr.mode : 0;
    return attr.attr.mode;
    }
#[no_mangle]
unsafe extern "C" fn guid_show(dev: *mut device, attr: *mut device_attribute, buf: *mut c_char) -> isize {
    static ssize_t guid_show(struct device *dev, struct device_attribute *attr, char *buf)
    {
    struct sdsi_priv *priv = dev_get_drvdata(dev);
    return sysfs_emit(buf, "0x%x\n", priv.guid);
    }
    static DEVICE_ATTR_RO(guid);
    static struct attribute *sdsi_attrs[] = {
    &dev_attr_guid.attr,
    core::ptr::null_mut()
    };
    static const struct attribute_group sdsi_group = {
    .attrs = sdsi_attrs,
    .bin_attrs = sdsi_bin_attrs,
    .is_bin_visible = sdsi_battr_is_visible,
    };
    __ATTRIBUTE_GROUPS(sdsi);
#[no_mangle]
unsafe extern "C" fn sdsi_get_layout(priv: *mut sdsi_priv, table: *mut disc_table) -> c_int {
    static int sdsi_get_layout(struct sdsi_priv *priv, struct disc_table *table)
    {
    switch (table.guid) {
    case SDSI_GUID_V1:
    priv.control_size = GUID_V1_CNTRL_SIZE;
    priv.registers_size = GUID_V1_REGS_SIZE;
    break;
    case SDSI_GUID_V2:
    priv.control_size = GUID_V2_CNTRL_SIZE;
    priv.registers_size = GUID_V2_REGS_SIZE;
    break;
    default:
    dev_err(priv.dev, "Unrecognized GUID 0x%x\n", table.guid);
    return -EINVAL;
    }
    return 0;
    }
    static int sdsi_map_mbox_registers(struct sdsi_priv *priv, struct device *dev,
    struct disc_table *disc_table, struct resource *disc_res)
    {
    let mut access_type: u32 = FIELD_GET(DT_ACCESS_TYPE, disc_table.access_info);
    let mut size: u32 = FIELD_GET(DT_SIZE, disc_table.access_info);
    let mut tbir: u32 = FIELD_GET(DT_TBIR, disc_table.offset);
    let mut offset: u32 = DT_OFFSET(disc_table.offset);
    struct pci_dev *parent = to_pci_dev(dev);
    let mut res: resource = {};
// Starting location of SDSi MMIO region based on access type
    switch (access_type) {
    case ACCESS_TYPE_LOCAL:
    if (tbir) {
    dev_err(priv.dev, "Unsupported BAR index %u for access type %u\n",
    tbir, access_type);
    return -EINVAL;
    }
//
// For access_type LOCAL, the base address is as follows:
// base address = end of discovery region + base offset + 1
//
    res.start = disc_res.end + offset + 1;
    break;
    case ACCESS_TYPE_BARID:
    res.start = pci_resource_start(parent, tbir) + offset;
    break;
    default:
    dev_err(priv.dev, "Unrecognized access_type %u\n", access_type);
    return -EINVAL;
    }
    res.end = res.start + size * sizeof(u32) - 1;
    res.flags = IORESOURCE_MEM;
    priv.control_addr = devm_ioremap_resource(priv.dev, &res);
    if (IS_ERR(priv.control_addr))
    return PTR_ERR(priv.control_addr);
    priv.mbox_addr = priv.control_addr + priv.control_size;
    priv.regs_addr = priv.mbox_addr + SDSI_SIZE_MAILBOX;
    priv.features = readq(priv.regs_addr + SDSI_ENABLED_FEATURES_OFFSET);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sdsi_probe(auxdev: *mut auxiliary_device, id: *const auxiliary_device_id) -> c_int {
    static int sdsi_probe(struct auxiliary_device *auxdev, const struct auxiliary_device_id *id)
    {
    struct intel_vsec_device *intel_cap_dev = auxdev_to_ivdev(auxdev);
    struct disc_table disc_table;
    struct resource *disc_res;
    void __iomem *disc_addr;
    struct sdsi_priv *priv;
    int ret;
    priv = devm_kzalloc(&auxdev.dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    priv.dev = &auxdev.dev;
    mutex_init(&priv.mb_lock);
    auxiliary_set_drvdata(auxdev, priv);
// Get the SDSi discovery table
    disc_res = &intel_cap_dev.resource[0];
    disc_addr = devm_ioremap_resource(&auxdev.dev, disc_res);
    if (IS_ERR(disc_addr))
    return PTR_ERR(disc_addr);
    memcpy_fromio(&disc_table, disc_addr, DISC_TABLE_SIZE);
    priv.guid = disc_table.guid;
// Get guid based layout info
    ret = sdsi_get_layout(priv, &disc_table);
    if (ret)
    return ret;
// Map the SDSi mailbox registers
    ret = sdsi_map_mbox_registers(priv, intel_cap_dev.dev, &disc_table, disc_res);
    if (ret)
    return ret;
    return 0;
    }
    static const struct auxiliary_device_id sdsi_aux_id_table[] = {
    { .name = "intel_vsec.sdsi" },
    {}
    };
    MODULE_DEVICE_TABLE(auxiliary, sdsi_aux_id_table);
    static struct auxiliary_driver sdsi_aux_driver = {
    .driver = {
    .dev_groups = sdsi_groups,
    },
    .id_table	= sdsi_aux_id_table,
    .probe		= sdsi_probe,
// No remove. All resources are handled under devm
    };
    module_auxiliary_driver(sdsi_aux_driver);
    MODULE_AUTHOR("David E. Box <david.e.box@linux.intel.com>");
    MODULE_DESCRIPTION("Intel On Demand (SDSi) driver");
    MODULE_LICENSE("GPL");
