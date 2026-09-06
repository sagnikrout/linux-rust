//! Automatically rewritten from C to Rust
//! Source: drivers/hwmon/via-cputemp.c
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
// via-cputemp.c - Driver for VIA CPU core temperature monitoring
// Copyright (C) 2009 VIA Technologies, Inc.
//
// based on existing coretemp.c, which is
//
// Copyright (C) 2007 Rudolf Marek <r.marek@assembler.cz>
//

    enum { SHOW_TEMP, SHOW_LABEL, SHOW_NAME };
//
// Functions declaration
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct via_cputemp_data {
    pub hwmon_dev: *mut device,
    pub name: *const c_char,
    pub vrm: u8,
    pub id: u32,
    pub msr_temp: u32,
    pub msr_vid: u32,
}

//
// Sysfs stuff
//
    static ssize_t name_show(struct device *dev, struct device_attribute *devattr,
    char *buf)
    {
    int ret;
    struct sensor_device_attribute *attr = to_sensor_dev_attr(devattr);
    struct via_cputemp_data *data = dev_get_drvdata(dev);
    if (attr.index == SHOW_NAME)
    ret = sprintf(buf, "%s\n", data.name);
    else	/* show label */
    ret = sprintf(buf, "Core %d\n", data.id);
    return ret;
    }
    static ssize_t temp_show(struct device *dev, struct device_attribute *devattr,
    char *buf)
    {
    struct via_cputemp_data *data = dev_get_drvdata(dev);
    u64 val;
    int err;
    err = rdmsrq_safe_on_cpu(data.id, data.msr_temp, &val);
    if (err)
    return -EAGAIN;
    return sprintf(buf, "%lu\n", ((unsigned long)val & 0xffffff) * 1000);
    }
    static ssize_t cpu0_vid_show(struct device *dev,
    struct device_attribute *devattr, char *buf)
    {
    struct via_cputemp_data *data = dev_get_drvdata(dev);
    u64 val;
    int err;
    err = rdmsrq_safe_on_cpu(data.id, data.msr_vid, &val);
    if (err)
    return -EAGAIN;
    return sprintf(buf, "%d\n", vid_from_reg(~(val >> 32) & 0x7f, data.vrm));
    }
    static SENSOR_DEVICE_ATTR_RO(temp1_input, temp, SHOW_TEMP);
    static SENSOR_DEVICE_ATTR_RO(temp1_label, name, SHOW_LABEL);
    static SENSOR_DEVICE_ATTR_RO(name, name, SHOW_NAME);
    static struct attribute *via_cputemp_attributes[] = {
    &sensor_dev_attr_name.dev_attr.attr,
    &sensor_dev_attr_temp1_label.dev_attr.attr,
    &sensor_dev_attr_temp1_input.dev_attr.attr,
    core::ptr::null_mut()
    };
    static const struct attribute_group via_cputemp_group = {
    .attrs = via_cputemp_attributes,
    };
// Optional attributes
    static DEVICE_ATTR_RO(cpu0_vid);
#[no_mangle]
unsafe extern "C" fn via_cputemp_probe(pdev: *mut platform_device) -> c_int {
    static int via_cputemp_probe(struct platform_device *pdev)
    {
    struct via_cputemp_data *data;
    struct cpuinfo_x86 *c = &cpu_data(pdev.id);
    int err;
    u64 val;
    data = devm_kzalloc(&pdev.dev, sizeof(struct via_cputemp_data),
    GFP_KERNEL);
    if (!data)
    return -ENOMEM;
    data.id = pdev.id;
    data.name = "via_cputemp";
    if (c.x86 == 7) {
    data.msr_temp = 0x1423;
    } else {
    switch (c.x86_model) {
    case 0xA:
// C7 A
    case 0xD:
// C7 D
    data.msr_temp = 0x1169;
    data.msr_vid = 0x198;
    break;
    case 0xF:
// Nano
    data.msr_temp = 0x1423;
    break;
    default:
    return -ENODEV;
    }
    }
// test if we can access the TEMPERATURE MSR
    err = rdmsrq_safe_on_cpu(data.id, data.msr_temp, &val);
    if (err) {
    dev_err(&pdev.dev,
    "Unable to access TEMPERATURE MSR, giving up\n");
    return err;
    }
    platform_set_drvdata(pdev, data);
    err = sysfs_create_group(&pdev.dev.kobj, &via_cputemp_group);
    if (err)
    return err;
    if (data.msr_vid)
    data.vrm = vid_which_vrm();
    if (data.vrm) {
    err = device_create_file(&pdev.dev, &dev_attr_cpu0_vid);
    if (err)
    goto exit_remove;
    }
    data.hwmon_dev = hwmon_device_register(&pdev.dev);
    if (IS_ERR(data.hwmon_dev)) {
    err = PTR_ERR(data.hwmon_dev);
    dev_err(&pdev.dev, "Class registration failed (%d)\n",
    err);
    goto exit_remove;
    }
    return 0;
    exit_remove:
    if (data.vrm)
    device_remove_file(&pdev.dev, &dev_attr_cpu0_vid);
    sysfs_remove_group(&pdev.dev.kobj, &via_cputemp_group);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn via_cputemp_remove(pdev: *mut platform_device) {
    static void via_cputemp_remove(struct platform_device *pdev)
    {
    struct via_cputemp_data *data = platform_get_drvdata(pdev);
    hwmon_device_unregister(data.hwmon_dev);
    if (data.vrm)
    device_remove_file(&pdev.dev, &dev_attr_cpu0_vid);
    sysfs_remove_group(&pdev.dev.kobj, &via_cputemp_group);
    }
    static struct platform_driver via_cputemp_driver = {
    .driver = {
    .name = DRVNAME,
    },
    .probe = via_cputemp_probe,
    .remove = via_cputemp_remove,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pdev_entry {
    pub list: list_head,
    pub pdev: *mut platform_device,
    pub cpu: c_uint,
}

    static LIST_HEAD(pdev_list);
    static DEFINE_MUTEX(pdev_list_mutex);
#[no_mangle]
unsafe extern "C" fn via_cputemp_online(cpu: c_uint) -> c_int {
    static int via_cputemp_online(unsigned int cpu)
    {
    int err;
    struct platform_device *pdev;
    struct pdev_entry *pdev_entry;
    pdev = platform_device_alloc(DRVNAME, cpu);
    if (!pdev) {
    err = -ENOMEM;
    pr_err("Device allocation failed\n");
    goto exit;
    }
    pdev_entry = kzalloc_obj(struct pdev_entry);
    if (!pdev_entry) {
    err = -ENOMEM;
    goto exit_device_put;
    }
    err = platform_device_add(pdev);
    if (err) {
    pr_err("Device addition failed (%d)\n", err);
    goto exit_device_free;
    }
    pdev_entry.pdev = pdev;
    pdev_entry.cpu = cpu;
    mutex_lock(&pdev_list_mutex);
    list_add_tail(&pdev_entry.list, &pdev_list);
    mutex_unlock(&pdev_list_mutex);
    return 0;
    exit_device_free:
    kfree(pdev_entry);
    exit_device_put:
    platform_device_put(pdev);
    exit:
    return err;
    }
#[no_mangle]
unsafe extern "C" fn via_cputemp_down_prep(cpu: c_uint) -> c_int {
    static int via_cputemp_down_prep(unsigned int cpu)
    {
    struct pdev_entry *p;
    mutex_lock(&pdev_list_mutex);
    list_for_each_entry(p, &pdev_list, list) {
    if (p.cpu == cpu) {
    platform_device_unregister(p.pdev);
    list_del(&p.list);
    mutex_unlock(&pdev_list_mutex);
    kfree(p);
    return 0;
    }
    }
    mutex_unlock(&pdev_list_mutex);
    return 0;
    }
    static const struct x86_cpu_id __initconst cputemp_ids[] = {
    X86_MATCH_VENDOR_FAM_MODEL(CENTAUR, 6, X86_CENTAUR_FAM6_C7_A,	core::ptr::null_mut()),
    X86_MATCH_VENDOR_FAM_MODEL(CENTAUR, 6, X86_CENTAUR_FAM6_C7_D,	core::ptr::null_mut()),
    X86_MATCH_VENDOR_FAM_MODEL(CENTAUR, 6, X86_CENTAUR_FAM6_NANO,	core::ptr::null_mut()),
    X86_MATCH_VENDOR_FAM_MODEL(CENTAUR, 7, X86_MODEL_ANY,		core::ptr::null_mut()),
    {}
    };
    MODULE_DEVICE_TABLE(x86cpu, cputemp_ids);
    static enum cpuhp_state via_temp_online;
#[no_mangle]
unsafe extern "C" fn via_cputemp_init() -> int __init {
    static int __init via_cputemp_init(void)
    {
    int err;
    if (!x86_match_cpu(cputemp_ids))
    return -ENODEV;
    err = platform_driver_register(&via_cputemp_driver);
    if (err)
    goto exit;
    err = cpuhp_setup_state(CPUHP_AP_ONLINE_DYN, "hwmon/via:online",
    via_cputemp_online, via_cputemp_down_prep);
    if (err < 0)
    goto exit_driver_unreg;
    via_temp_online = err;

    if (list_empty(&pdev_list)) {
    err = -ENODEV;
    goto exit_hp_unreg;
    }

    return 0;

    exit_hp_unreg:
    cpuhp_remove_state_nocalls(via_temp_online);

    exit_driver_unreg:
    platform_driver_unregister(&via_cputemp_driver);
    exit:
    return err;
    }
#[no_mangle]
unsafe extern "C" fn via_cputemp_exit() -> void __exit {
    static void __exit via_cputemp_exit(void)
    {
    cpuhp_remove_state(via_temp_online);
    platform_driver_unregister(&via_cputemp_driver);
    }
    MODULE_AUTHOR("Harald Welte <HaraldWelte@viatech.com>");
    MODULE_DESCRIPTION("VIA CPU temperature monitor");
    MODULE_LICENSE("GPL");
    module_init(via_cputemp_init)
    module_exit(via_cputemp_exit)
