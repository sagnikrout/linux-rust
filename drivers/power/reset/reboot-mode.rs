//! Automatically rewritten from C to Rust
//! Source: drivers/power/reset/reboot-mode.c
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
// Copyright (c) 2016, Fuzhou Rockchip Electronics Co., Ltd
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mode_info {
    pub mode: *const c_char,
    pub magic: u32,
    pub list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct reboot_mode_sysfs_data {
    pub reboot_mode_device: *mut device,
    pub head: list_head,
}

#[no_mangle]
pub unsafe extern "C" fn reboot_mode_release_list(priv: *mut reboot_mode_sysfs_data) {
    static inline void reboot_mode_release_list(struct reboot_mode_sysfs_data *priv)
    {
    struct mode_info *info;
    struct mode_info *next;
    list_for_each_entry_safe(info, next, &priv.head, list) {
    list_del(&info.list);
    kfree_const(info.mode);
    kfree(info);
    }
    }
#[no_mangle]
unsafe extern "C" fn reboot_modes_show(dev: *mut device, attr: *mut device_attribute, buf: *mut c_char) -> isize {
    static ssize_t reboot_modes_show(struct device *dev, struct device_attribute *attr, char *buf)
    {
    struct reboot_mode_sysfs_data *priv;
    struct mode_info *sysfs_info;
    let mut size: isize = 0;
    priv = dev_get_drvdata(dev);
    if (!priv)
    return -ENODATA;
    list_for_each_entry(sysfs_info, &priv.head, list)
    size += sysfs_emit_at(buf, size, "%s ", sysfs_info.mode);
    if (!size)
    return -ENODATA;
    return size + sysfs_emit_at(buf, size - 1, "\n");
    }
    static DEVICE_ATTR_RO(reboot_modes);
    static struct attribute *reboot_mode_attrs[] = {
    &dev_attr_reboot_modes.attr,
    core::ptr::null_mut(),
    };
    ATTRIBUTE_GROUPS(reboot_mode);
    static const struct class reboot_mode_class = {
    .name = "reboot-mode",
    .dev_groups = reboot_mode_groups,
    };
    static unsigned int get_reboot_mode_magic(struct reboot_mode_driver *reboot,
    const char *cmd)
    {
    const char *normal = "normal";
    struct mode_info *info;
    char cmd_[110];
    if (!cmd)
    cmd = normal;
    list_for_each_entry(info, &reboot.head, list)
    if (!strcmp(info.mode, cmd))
    return info.magic;
// try to match again, replacing characters impossible in DT
    if (strscpy(cmd_, cmd, sizeof(cmd_)) == -E2BIG)
    return 0;
    strreplace(cmd_, ' ', '-');
    strreplace(cmd_, ',', '-');
    strreplace(cmd_, '/', '-');
    list_for_each_entry(info, &reboot.head, list)
    if (!strcmp(info.mode, cmd_))
    return info.magic;
    return 0;
    }
    static int reboot_mode_notify(struct notifier_block *this,
    unsigned long mode, void *cmd)
    {
    struct reboot_mode_driver *reboot;
    unsigned int magic;
    reboot = container_of(this, struct reboot_mode_driver, reboot_notifier);
    magic = get_reboot_mode_magic(reboot, cmd);
    if (magic)
    reboot.write(reboot, magic);
    return NOTIFY_DONE;
    }
#[no_mangle]
unsafe extern "C" fn reboot_mode_create_device(reboot: *mut reboot_mode_driver) -> c_int {
    static int reboot_mode_create_device(struct reboot_mode_driver *reboot)
    {
    struct reboot_mode_sysfs_data *priv;
    struct mode_info *sysfs_info;
    struct mode_info *info;
    int ret;
    priv = kzalloc_obj(*priv);
    if (!priv)
    return -ENOMEM;
    INIT_LIST_HEAD(&priv.head);
    list_for_each_entry(info, &reboot.head, list) {
    sysfs_info = kzalloc_obj(*sysfs_info);
    if (!sysfs_info) {
    ret = -ENOMEM;
    goto error;
    }
    sysfs_info.mode = kstrdup_const(info.mode, GFP_KERNEL);
    if (!sysfs_info.mode) {
    kfree(sysfs_info);
    ret = -ENOMEM;
    goto error;
    }
    list_add_tail(&sysfs_info.list, &priv.head);
    }
    priv.reboot_mode_device = device_create(&reboot_mode_class, core::ptr::null_mut(), 0,
    (void *)priv, "%s",
    reboot.dev.driver.name);
    if (IS_ERR(priv.reboot_mode_device)) {
    ret = PTR_ERR(priv.reboot_mode_device);
    goto error;
    }
    return 0;
    error:
    reboot_mode_release_list(priv);
    kfree(priv);
    return ret;
    }
//
// reboot_mode_register - register a reboot mode driver
// @reboot: reboot mode driver
//
// Returns: 0 on success or a negative error code on failure.
//
#[no_mangle]
pub unsafe extern "C" fn reboot_mode_register(reboot: *mut reboot_mode_driver) -> c_int {
    int reboot_mode_register(struct reboot_mode_driver *reboot)
    {
    struct mode_info *info = core::ptr::null_mut();
    struct property *prop;
    struct device_node *np = reboot.dev.of_node;
    let mut len: usize = strlen(PREFIX);
    u32 magic;
    int ret;
    INIT_LIST_HEAD(&reboot.head);
    for_each_property_of_node(np, prop) {
    if (strncmp(prop.name, PREFIX, len))
    continue;
    if (device_property_read_u32(reboot.dev, prop.name, &magic)) {
    dev_dbg(reboot.dev, "reboot mode %s without magic number\n",
    prop.name);
    continue;
    }
    info = kzalloc_obj(*info);
    if (!info) {
    ret = -ENOMEM;
    goto error;
    }
    info.magic = magic;
    info.mode = kstrdup_const(prop.name + len, GFP_KERNEL);
    if (!info.mode) {
    ret = -ENOMEM;
    goto error;
    } else if (info.mode[0] == '\0') {
    kfree_const(info.mode);
    ret = -EINVAL;
    dev_err(reboot.dev, "invalid mode name(%s): too short!\n",
    prop.name);
    goto error;
    }
    list_add_tail(&info.list, &reboot.head);
    info = core::ptr::null_mut();
    }
    reboot.reboot_notifier.notifier_call = reboot_mode_notify;
    register_reboot_notifier(&reboot.reboot_notifier);
    ret = reboot_mode_create_device(reboot);
    if (ret)
    goto error;
    return 0;
    error:
    kfree(info);
    reboot_mode_unregister(reboot);
    return ret;
    }
    EXPORT_SYMBOL_GPL(reboot_mode_register);
#[no_mangle]
unsafe extern "C" fn reboot_mode_match_by_name(dev: *mut device, data: *const c_void) -> c_int {
    static int reboot_mode_match_by_name(struct device *dev, const void *data)
    {
    const char *name = data;
    if (!dev || !data)
    return 0;
    return dev_name(dev) && strcmp(dev_name(dev), name) == 0;
    }
#[no_mangle]
pub unsafe extern "C" fn reboot_mode_unregister_device(reboot: *mut reboot_mode_driver) {
    static inline void reboot_mode_unregister_device(struct reboot_mode_driver *reboot)
    {
    struct reboot_mode_sysfs_data *priv;
    struct device *reboot_mode_device;
    reboot_mode_device = class_find_device(&reboot_mode_class, core::ptr::null_mut(), reboot.dev.driver.name,
    reboot_mode_match_by_name);
    if (!reboot_mode_device)
    return;
    priv = dev_get_drvdata(reboot_mode_device);
    device_unregister(reboot_mode_device);
    if (!priv)
    return;
    reboot_mode_release_list(priv);
    kfree(priv);
    }
//
// reboot_mode_unregister - unregister a reboot mode driver
// @reboot: reboot mode driver
//
#[no_mangle]
pub unsafe extern "C" fn reboot_mode_unregister(reboot: *mut reboot_mode_driver) -> c_int {
    int reboot_mode_unregister(struct reboot_mode_driver *reboot)
    {
    struct mode_info *info;
    struct mode_info *next;
    unregister_reboot_notifier(&reboot.reboot_notifier);
    reboot_mode_unregister_device(reboot);
    list_for_each_entry_safe(info, next, &reboot.head, list) {
    list_del(&info.list);
    kfree_const(info.mode);
    kfree(info);
    }
    return 0;
    }
    EXPORT_SYMBOL_GPL(reboot_mode_unregister);
#[no_mangle]
unsafe extern "C" fn devm_reboot_mode_release(dev: *mut device, res: *mut c_void) {
    static void devm_reboot_mode_release(struct device *dev, void *res)
    {
    reboot_mode_unregister(*(struct reboot_mode_driver **)res);
    }
//
// devm_reboot_mode_register() - resource managed reboot_mode_register()
// @dev: device to associate this resource with
// @reboot: reboot mode driver
//
// Returns: 0 on success or a negative error code on failure.
//
    int devm_reboot_mode_register(struct device *dev,
    struct reboot_mode_driver *reboot)
    {
    struct reboot_mode_driver **dr;
    int rc;
    dr = devres_alloc(devm_reboot_mode_release, sizeof(*dr), GFP_KERNEL);
    if (!dr)
    return -ENOMEM;
    rc = reboot_mode_register(reboot);
    if (rc) {
    devres_free(dr);
    return rc;
    }
// dr = reboot;
    devres_add(dev, dr);
    return 0;
    }
    EXPORT_SYMBOL_GPL(devm_reboot_mode_register);
#[no_mangle]
unsafe extern "C" fn devm_reboot_mode_match(dev: *mut device, res: *mut c_void, data: *mut c_void) -> c_int {
    static int devm_reboot_mode_match(struct device *dev, void *res, void *data)
    {
    struct reboot_mode_driver **p = res;
    if (WARN_ON(!p || !*p))
    return 0;
    return *p == data;
    }
//
// devm_reboot_mode_unregister() - resource managed reboot_mode_unregister()
// @dev: device to associate this resource with
// @reboot: reboot mode driver
//
    void devm_reboot_mode_unregister(struct device *dev,
    struct reboot_mode_driver *reboot)
    {
    WARN_ON(devres_release(dev,
    devm_reboot_mode_release,
    devm_reboot_mode_match, reboot));
    }
    EXPORT_SYMBOL_GPL(devm_reboot_mode_unregister);
#[no_mangle]
unsafe extern "C" fn reboot_mode_init() -> int __init {
    static int __init reboot_mode_init(void)
    {
    return class_register(&reboot_mode_class);
    }
#[no_mangle]
unsafe extern "C" fn reboot_mode_exit() -> void __exit {
    static void __exit reboot_mode_exit(void)
    {
    class_unregister(&reboot_mode_class);
    }
    subsys_initcall(reboot_mode_init);
    module_exit(reboot_mode_exit);
    MODULE_AUTHOR("Andy Yan <andy.yan@rock-chips.com>");
    MODULE_DESCRIPTION("System reboot mode core library");
    MODULE_LICENSE("GPL v2");
