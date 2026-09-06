//! Automatically rewritten from C to Rust
//! Source: drivers/hwmon/i5k_amb.c
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
// A hwmon driver for the Intel 5000 series chipset FB-DIMM AMB
// temperature sensors
// Copyright (C) 2007 IBM
//
// Author: Darrick J. Wong <darrick.wong@oracle.com>
//

pub const I5K_REG_AMB_BASE_ADDR: c_uint = 0x48;
pub const I5K_REG_AMB_LEN_ADDR: c_uint = 0x50;
pub const I5K_REG_CHAN0_PRESENCE_ADDR: c_uint = 0x64;
pub const I5K_REG_CHAN1_PRESENCE_ADDR: c_uint = 0x66;
pub const AMB_REG_TEMP_MIN_ADDR: c_uint = 0x80;
pub const AMB_REG_TEMP_MID_ADDR: c_uint = 0x81;
pub const AMB_REG_TEMP_MAX_ADDR: c_uint = 0x82;
pub const AMB_REG_TEMP_STATUS_ADDR: c_uint = 0x84;
pub const AMB_REG_TEMP_ADDR: c_uint = 0x85;
pub const AMB_CONFIG_SIZE: c_int = 2048;
pub const AMB_FUNC_3_OFFSET: c_int = 768;
#[no_mangle]
unsafe extern "C" fn amb_reg_temp_status(amb: c_uint) -> c_ulong {
    static unsigned long amb_reg_temp_status(unsigned int amb)
    {
    return AMB_FUNC_3_OFFSET + AMB_REG_TEMP_STATUS_ADDR +
    AMB_CONFIG_SIZE * amb;
    }
#[no_mangle]
unsafe extern "C" fn amb_reg_temp_min(amb: c_uint) -> c_ulong {
    static unsigned long amb_reg_temp_min(unsigned int amb)
    {
    return AMB_FUNC_3_OFFSET + AMB_REG_TEMP_MIN_ADDR +
    AMB_CONFIG_SIZE * amb;
    }
#[no_mangle]
unsafe extern "C" fn amb_reg_temp_mid(amb: c_uint) -> c_ulong {
    static unsigned long amb_reg_temp_mid(unsigned int amb)
    {
    return AMB_FUNC_3_OFFSET + AMB_REG_TEMP_MID_ADDR +
    AMB_CONFIG_SIZE * amb;
    }
#[no_mangle]
unsafe extern "C" fn amb_reg_temp_max(amb: c_uint) -> c_ulong {
    static unsigned long amb_reg_temp_max(unsigned int amb)
    {
    return AMB_FUNC_3_OFFSET + AMB_REG_TEMP_MAX_ADDR +
    AMB_CONFIG_SIZE * amb;
    }
#[no_mangle]
unsafe extern "C" fn amb_reg_temp(amb: c_uint) -> c_ulong {
    static unsigned long amb_reg_temp(unsigned int amb)
    {
    return AMB_FUNC_3_OFFSET + AMB_REG_TEMP_ADDR +
    AMB_CONFIG_SIZE * amb;
    }
pub const MAX_MEM_CHANNELS: c_int = 4;
pub const MAX_AMBS_PER_CHANNEL: c_int = 16;

    MAX_AMBS_PER_CHANNEL)
pub const CHANNEL_SHIFT: c_int = 4;
pub const DIMM_MASK: c_uint = 0xF;
//
// Ugly hack: For some reason the highest bit is set if there
// are _any_ DIMMs in the channel.  Attempting to read from
// this "high-order" AMB results in a memory bus error, so
// for now we'll just ignore that top bit, even though that
// might prevent us from seeing the 16th DIMM in the channel.
//
pub const REAL_MAX_AMBS_PER_CHANNEL: c_int = 15;
pub const KNOBS_PER_AMB: c_int = 6;
#[no_mangle]
unsafe extern "C" fn amb_num_from_reg(byte_num: c_uint, bit: c_uint) -> c_ulong {
    static unsigned long amb_num_from_reg(unsigned int byte_num, unsigned int bit)
    {
    return byte_num * MAX_AMBS_PER_CHANNEL + bit;
    }
pub const AMB_SYSFS_NAME_LEN: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i5k_device_attribute {
    pub s_attr: sensor_device_attribute,
    pub name: [c_char; AMB_SYSFS_NAME_LEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i5k_amb_data {
    pub hwmon_dev: *mut device,
    pub amb_base: c_ulong,
    pub amb_len: c_ulong,
    pub amb_present: [u16; MAX_MEM_CHANNELS],
    pub amb_mmio: *mut void __iomem,
    pub attrs: *mut i5k_device_attribute,
    pub num_attrs: c_uint,
}

    static DEVICE_STRING_ATTR_RO(name, 0444, DRVNAME);
    static struct platform_device *amb_pdev;
#[no_mangle]
unsafe extern "C" fn amb_read_byte(data: *mut i5k_amb_data, offset: c_ulong) -> u8 {
    static u8 amb_read_byte(struct i5k_amb_data *data, unsigned long offset)
    {
    return ioread8(data.amb_mmio + offset);
    }
    static void amb_write_byte(struct i5k_amb_data *data, unsigned long offset,
    u8 val)
    {
    iowrite8(val, data.amb_mmio + offset);
    }
    static ssize_t show_amb_alarm(struct device *dev,
    struct device_attribute *devattr,
    char *buf)
    {
    struct sensor_device_attribute *attr = to_sensor_dev_attr(devattr);
    struct i5k_amb_data *data = dev_get_drvdata(dev);
    if (!(amb_read_byte(data, amb_reg_temp_status(attr.index)) & 0x20) &&
    (amb_read_byte(data, amb_reg_temp_status(attr.index)) & 0x8))
    return sprintf(buf, "1\n");
    else
    return sprintf(buf, "0\n");
    }
    static ssize_t store_amb_min(struct device *dev,
    struct device_attribute *devattr,
    const char *buf,
    size_t count)
    {
    struct sensor_device_attribute *attr = to_sensor_dev_attr(devattr);
    struct i5k_amb_data *data = dev_get_drvdata(dev);
    unsigned long temp;
    let mut ret: c_int = kstrtoul(buf, 10, &temp);
    if (ret < 0)
    return ret;
    temp = temp / 500;
    if (temp > 255)
    temp = 255;
    amb_write_byte(data, amb_reg_temp_min(attr.index), temp);
    return count;
    }
    static ssize_t store_amb_mid(struct device *dev,
    struct device_attribute *devattr,
    const char *buf,
    size_t count)
    {
    struct sensor_device_attribute *attr = to_sensor_dev_attr(devattr);
    struct i5k_amb_data *data = dev_get_drvdata(dev);
    unsigned long temp;
    let mut ret: c_int = kstrtoul(buf, 10, &temp);
    if (ret < 0)
    return ret;
    temp = temp / 500;
    if (temp > 255)
    temp = 255;
    amb_write_byte(data, amb_reg_temp_mid(attr.index), temp);
    return count;
    }
    static ssize_t store_amb_max(struct device *dev,
    struct device_attribute *devattr,
    const char *buf,
    size_t count)
    {
    struct sensor_device_attribute *attr = to_sensor_dev_attr(devattr);
    struct i5k_amb_data *data = dev_get_drvdata(dev);
    unsigned long temp;
    let mut ret: c_int = kstrtoul(buf, 10, &temp);
    if (ret < 0)
    return ret;
    temp = temp / 500;
    if (temp > 255)
    temp = 255;
    amb_write_byte(data, amb_reg_temp_max(attr.index), temp);
    return count;
    }
    static ssize_t show_amb_min(struct device *dev,
    struct device_attribute *devattr,
    char *buf)
    {
    struct sensor_device_attribute *attr = to_sensor_dev_attr(devattr);
    struct i5k_amb_data *data = dev_get_drvdata(dev);
    return sprintf(buf, "%d\n",
    500 * amb_read_byte(data, amb_reg_temp_min(attr.index)));
    }
    static ssize_t show_amb_mid(struct device *dev,
    struct device_attribute *devattr,
    char *buf)
    {
    struct sensor_device_attribute *attr = to_sensor_dev_attr(devattr);
    struct i5k_amb_data *data = dev_get_drvdata(dev);
    return sprintf(buf, "%d\n",
    500 * amb_read_byte(data, amb_reg_temp_mid(attr.index)));
    }
    static ssize_t show_amb_max(struct device *dev,
    struct device_attribute *devattr,
    char *buf)
    {
    struct sensor_device_attribute *attr = to_sensor_dev_attr(devattr);
    struct i5k_amb_data *data = dev_get_drvdata(dev);
    return sprintf(buf, "%d\n",
    500 * amb_read_byte(data, amb_reg_temp_max(attr.index)));
    }
    static ssize_t show_amb_temp(struct device *dev,
    struct device_attribute *devattr,
    char *buf)
    {
    struct sensor_device_attribute *attr = to_sensor_dev_attr(devattr);
    struct i5k_amb_data *data = dev_get_drvdata(dev);
    return sprintf(buf, "%d\n",
    500 * amb_read_byte(data, amb_reg_temp(attr.index)));
    }
    static ssize_t show_label(struct device *dev,
    struct device_attribute *devattr,
    char *buf)
    {
    struct sensor_device_attribute *attr = to_sensor_dev_attr(devattr);
    return sprintf(buf, "Ch. %d DIMM %d\n", attr.index >> CHANNEL_SHIFT,
    attr.index & DIMM_MASK);
    }
#[no_mangle]
unsafe extern "C" fn i5k_amb_hwmon_init(pdev: *mut platform_device) -> c_int {
    static int i5k_amb_hwmon_init(struct platform_device *pdev)
    {
    int i, j, k, d = 0;
    u16 c;
    let mut res: c_int = 0;
    let mut num_ambs: c_int = 0;
    struct i5k_amb_data *data = platform_get_drvdata(pdev);
// Count the number of AMBs found
// ignore the high-order bit, see "Ugly hack" comment above
    for (i = 0; i < MAX_MEM_CHANNELS; i++)
    num_ambs += hweight16(data.amb_present[i] & 0x7fff);
// Set up sysfs stuff
    data.attrs = kzalloc(array3_size(num_ambs, KNOBS_PER_AMB,
    sizeof(*data.attrs)),
    GFP_KERNEL);
    if (!data.attrs)
    return -ENOMEM;
    data.num_attrs = 0;
    for (i = 0; i < MAX_MEM_CHANNELS; i++) {
    c = data.amb_present[i];
    for (j = 0; j < REAL_MAX_AMBS_PER_CHANNEL; j++, c >>= 1) {
    struct i5k_device_attribute *iattr;
    k = amb_num_from_reg(i, j);
    if (!(c & 0x1))
    continue;
    d++;
// sysfs label
    iattr = data.attrs + data.num_attrs;
    snprintf(iattr.name, AMB_SYSFS_NAME_LEN,
    "temp%d_label", d);
    iattr.s_attr.dev_attr.attr.name = iattr.name;
    iattr.s_attr.dev_attr.attr.mode = 0444;
    iattr.s_attr.dev_attr.show = show_label;
    iattr.s_attr.index = k;
    sysfs_attr_init(&iattr.s_attr.dev_attr.attr);
    res = device_create_file(&pdev.dev,
    &iattr.s_attr.dev_attr);
    if (res)
    goto exit_remove;
    data.num_attrs++;
// Temperature sysfs knob
    iattr = data.attrs + data.num_attrs;
    snprintf(iattr.name, AMB_SYSFS_NAME_LEN,
    "temp%d_input", d);
    iattr.s_attr.dev_attr.attr.name = iattr.name;
    iattr.s_attr.dev_attr.attr.mode = 0444;
    iattr.s_attr.dev_attr.show = show_amb_temp;
    iattr.s_attr.index = k;
    sysfs_attr_init(&iattr.s_attr.dev_attr.attr);
    res = device_create_file(&pdev.dev,
    &iattr.s_attr.dev_attr);
    if (res)
    goto exit_remove;
    data.num_attrs++;
// Temperature min sysfs knob
    iattr = data.attrs + data.num_attrs;
    snprintf(iattr.name, AMB_SYSFS_NAME_LEN,
    "temp%d_min", d);
    iattr.s_attr.dev_attr.attr.name = iattr.name;
    iattr.s_attr.dev_attr.attr.mode = 0644;
    iattr.s_attr.dev_attr.show = show_amb_min;
    iattr.s_attr.dev_attr.store = store_amb_min;
    iattr.s_attr.index = k;
    sysfs_attr_init(&iattr.s_attr.dev_attr.attr);
    res = device_create_file(&pdev.dev,
    &iattr.s_attr.dev_attr);
    if (res)
    goto exit_remove;
    data.num_attrs++;
// Temperature mid sysfs knob
    iattr = data.attrs + data.num_attrs;
    snprintf(iattr.name, AMB_SYSFS_NAME_LEN,
    "temp%d_mid", d);
    iattr.s_attr.dev_attr.attr.name = iattr.name;
    iattr.s_attr.dev_attr.attr.mode = 0644;
    iattr.s_attr.dev_attr.show = show_amb_mid;
    iattr.s_attr.dev_attr.store = store_amb_mid;
    iattr.s_attr.index = k;
    sysfs_attr_init(&iattr.s_attr.dev_attr.attr);
    res = device_create_file(&pdev.dev,
    &iattr.s_attr.dev_attr);
    if (res)
    goto exit_remove;
    data.num_attrs++;
// Temperature max sysfs knob
    iattr = data.attrs + data.num_attrs;
    snprintf(iattr.name, AMB_SYSFS_NAME_LEN,
    "temp%d_max", d);
    iattr.s_attr.dev_attr.attr.name = iattr.name;
    iattr.s_attr.dev_attr.attr.mode = 0644;
    iattr.s_attr.dev_attr.show = show_amb_max;
    iattr.s_attr.dev_attr.store = store_amb_max;
    iattr.s_attr.index = k;
    sysfs_attr_init(&iattr.s_attr.dev_attr.attr);
    res = device_create_file(&pdev.dev,
    &iattr.s_attr.dev_attr);
    if (res)
    goto exit_remove;
    data.num_attrs++;
// Temperature alarm sysfs knob
    iattr = data.attrs + data.num_attrs;
    snprintf(iattr.name, AMB_SYSFS_NAME_LEN,
    "temp%d_alarm", d);
    iattr.s_attr.dev_attr.attr.name = iattr.name;
    iattr.s_attr.dev_attr.attr.mode = 0444;
    iattr.s_attr.dev_attr.show = show_amb_alarm;
    iattr.s_attr.index = k;
    sysfs_attr_init(&iattr.s_attr.dev_attr.attr);
    res = device_create_file(&pdev.dev,
    &iattr.s_attr.dev_attr);
    if (res)
    goto exit_remove;
    data.num_attrs++;
    }
    }
    res = device_create_file(&pdev.dev, &dev_attr_name.attr);
    if (res)
    goto exit_remove;
    data.hwmon_dev = hwmon_device_register(&pdev.dev);
    if (IS_ERR(data.hwmon_dev)) {
    res = PTR_ERR(data.hwmon_dev);
    goto exit_remove;
    }
    return res;
    exit_remove:
    device_remove_file(&pdev.dev, &dev_attr_name.attr);
    for (i = 0; i < data.num_attrs; i++)
    device_remove_file(&pdev.dev, &data.attrs[i].s_attr.dev_attr);
    kfree(data.attrs);
    return res;
    }
#[no_mangle]
unsafe extern "C" fn i5k_amb_add() -> c_int {
    static int i5k_amb_add(void)
    {
    int res;
// only ever going to be one of these
    amb_pdev = platform_device_alloc(DRVNAME, 0);
    if (!amb_pdev)
    return -ENOMEM;
    res = platform_device_add(amb_pdev);
    if (res)
    goto err;
    return 0;
    err:
    platform_device_put(amb_pdev);
    return res;
    }
    static int i5k_find_amb_registers(struct i5k_amb_data *data,
    unsigned long devid)
    {
    struct pci_dev *pcidev;
    u32 val32;
    let mut res: c_int = -ENODEV;
// Find AMB register memory space
    pcidev = pci_get_device(PCI_VENDOR_ID_INTEL,
    devid,
    core::ptr::null_mut());
    if (!pcidev)
    return -ENODEV;
    pci_read_config_dword(pcidev, I5K_REG_AMB_BASE_ADDR, &val32);
    if (val32 == (u32)~0)
    goto out;
    data.amb_base = val32;
    pci_read_config_dword(pcidev, I5K_REG_AMB_LEN_ADDR, &val32);
    if (val32 == (u32)~0)
    goto out;
    data.amb_len = val32;
// Is it big enough?
    if (data.amb_len < AMB_CONFIG_SIZE * MAX_AMBS) {
    dev_err(&pcidev.dev, "AMB region too small!\n");
    goto out;
    }
    res = 0;
    out:
    pci_dev_put(pcidev);
    return res;
    }
#[no_mangle]
unsafe extern "C" fn i5k_channel_probe(amb_present: *mut u16, dev_id: c_ulong) -> c_int {
    static int i5k_channel_probe(u16 *amb_present, unsigned long dev_id)
    {
    struct pci_dev *pcidev;
    u16 val16;
    let mut res: c_int = -ENODEV;
// Copy the DIMM presence map for these two channels
    pcidev = pci_get_device(PCI_VENDOR_ID_INTEL, dev_id, core::ptr::null_mut());
    if (!pcidev)
    return -ENODEV;
    pci_read_config_word(pcidev, I5K_REG_CHAN0_PRESENCE_ADDR, &val16);
    if (val16 == (u16)~0)
    goto out;
    amb_present[0] = val16;
    pci_read_config_word(pcidev, I5K_REG_CHAN1_PRESENCE_ADDR, &val16);
    if (val16 == (u16)~0)
    goto out;
    amb_present[1] = val16;
    res = 0;
    out:
    pci_dev_put(pcidev);
    return res;
    }
    static struct {
    unsigned long err;
    unsigned long fbd0;
    } chipset_ids[]  = {
    { PCI_DEVICE_ID_INTEL_5000_ERR, PCI_DEVICE_ID_INTEL_5000_FBD0 },
    { PCI_DEVICE_ID_INTEL_5400_ERR, PCI_DEVICE_ID_INTEL_5400_FBD0 },
    { 0, 0 }
    };

    static const struct pci_device_id i5k_amb_ids[] = {
    { PCI_DEVICE(PCI_VENDOR_ID_INTEL, PCI_DEVICE_ID_INTEL_5000_ERR) },
    { PCI_DEVICE(PCI_VENDOR_ID_INTEL, PCI_DEVICE_ID_INTEL_5400_ERR) },
    { 0, }
    };
    MODULE_DEVICE_TABLE(pci, i5k_amb_ids);

#[no_mangle]
unsafe extern "C" fn i5k_amb_probe(pdev: *mut platform_device) -> c_int {
    static int i5k_amb_probe(struct platform_device *pdev)
    {
    struct i5k_amb_data *data;
    struct resource *reso;
    int i, res;
    data = kzalloc_obj(*data);
    if (!data)
    return -ENOMEM;
// Figure out where the AMB registers live
    i = 0;
    do {
    res = i5k_find_amb_registers(data, chipset_ids[i].err);
    if (res == 0)
    break;
    i++;
    } while (chipset_ids[i].err);
    if (res)
    goto err;
// Copy the DIMM presence map for the first two channels
    res = i5k_channel_probe(&data.amb_present[0], chipset_ids[i].fbd0);
    if (res)
    goto err;
// Copy the DIMM presence map for the optional second two channels
    i5k_channel_probe(&data.amb_present[2], chipset_ids[i].fbd0 + 1);
// Set up resource regions
    reso = request_mem_region(data.amb_base, data.amb_len, DRVNAME);
    if (!reso) {
    res = -EBUSY;
    goto err;
    }
    data.amb_mmio = ioremap(data.amb_base, data.amb_len);
    if (!data.amb_mmio) {
    res = -EBUSY;
    goto err_map_failed;
    }
    platform_set_drvdata(pdev, data);
    res = i5k_amb_hwmon_init(pdev);
    if (res)
    goto err_init_failed;
    return res;
    err_init_failed:
    iounmap(data.amb_mmio);
    err_map_failed:
    release_mem_region(data.amb_base, data.amb_len);
    err:
    kfree(data);
    return res;
    }
#[no_mangle]
unsafe extern "C" fn i5k_amb_remove(pdev: *mut platform_device) {
    static void i5k_amb_remove(struct platform_device *pdev)
    {
    int i;
    struct i5k_amb_data *data = platform_get_drvdata(pdev);
    hwmon_device_unregister(data.hwmon_dev);
    device_remove_file(&pdev.dev, &dev_attr_name.attr);
    for (i = 0; i < data.num_attrs; i++)
    device_remove_file(&pdev.dev, &data.attrs[i].s_attr.dev_attr);
    kfree(data.attrs);
    iounmap(data.amb_mmio);
    release_mem_region(data.amb_base, data.amb_len);
    kfree(data);
    }
    static struct platform_driver i5k_amb_driver = {
    .driver = {
    .name = DRVNAME,
    },
    .probe = i5k_amb_probe,
    .remove = i5k_amb_remove,
    };
#[no_mangle]
unsafe extern "C" fn i5k_amb_init() -> int __init {
    static int __init i5k_amb_init(void)
    {
    int res;
    res = platform_driver_register(&i5k_amb_driver);
    if (res)
    return res;
    res = i5k_amb_add();
    if (res)
    platform_driver_unregister(&i5k_amb_driver);
    return res;
    }
#[no_mangle]
unsafe extern "C" fn i5k_amb_exit() -> void __exit {
    static void __exit i5k_amb_exit(void)
    {
    platform_device_unregister(amb_pdev);
    platform_driver_unregister(&i5k_amb_driver);
    }
    MODULE_AUTHOR("Darrick J. Wong <darrick.wong@oracle.com>");
    MODULE_DESCRIPTION("Intel 5000 chipset FB-DIMM AMB temperature sensor");
    MODULE_LICENSE("GPL");
    module_init(i5k_amb_init);
    module_exit(i5k_amb_exit);
