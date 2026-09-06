//! Automatically rewritten from C to Rust
//! Source: drivers/mtd/ubi/nvmem.c
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
// Copyright (c) 2023 Daniel Golle <daniel@makrotopia.org>
//
// UBI NVMEM provider

// List of all NVMEM devices
    static LIST_HEAD(nvmem_devices);
    static DEFINE_MUTEX(devices_mutex);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ubi_nvmem {
    pub nvmem: *mut nvmem_device,
    pub ubi_num: c_int,
    pub vol_id: c_int,
    pub usable_leb_size: c_int,
    pub list: list_head,
}

    static int ubi_nvmem_reg_read(void *priv, unsigned int from,
    void *val, size_t bytes)
    {
    size_t to_read, bytes_left = bytes;
    struct ubi_nvmem *unv = priv;
    struct ubi_volume_desc *desc;
    uint32_t offs;
    uint32_t lnum;
    let mut err: c_int = 0;
    desc = ubi_open_volume(unv.ubi_num, unv.vol_id, UBI_READONLY);
    if (IS_ERR(desc))
    return PTR_ERR(desc);
    offs = from % unv.usable_leb_size;
    lnum = from / unv.usable_leb_size;
    while (bytes_left) {
    to_read = unv.usable_leb_size - offs;
    if (to_read > bytes_left)
    to_read = bytes_left;
    err = ubi_read(desc, lnum, val, offs, to_read);
    if (err)
    break;
    lnum += 1;
    offs = 0;
    bytes_left -= to_read;
    val += to_read;
    }
    ubi_close_volume(desc);
    if (err)
    return err;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ubi_nvmem_add(vi: *mut ubi_volume_info) -> c_int {
    static int ubi_nvmem_add(struct ubi_volume_info *vi)
    {
    struct device_node *np = dev_of_node(vi.dev);
    let mut config: nvmem_config = {};
    struct ubi_nvmem *unv;
    int ret;
    if (!np)
    return 0;
    if (!of_get_child_by_name(np, "nvmem-layout"))
    return 0;
    if (WARN_ON_ONCE(vi.usable_leb_size <= 0) ||
    WARN_ON_ONCE(vi.size <= 0))
    return -EINVAL;
    unv = kzalloc_obj(struct ubi_nvmem);
    if (!unv)
    return -ENOMEM;
    config.id = NVMEM_DEVID_NONE;
    config.dev = vi.dev;
    config.name = dev_name(vi.dev);
    config.owner = THIS_MODULE;
    config.priv = unv;
    config.reg_read = ubi_nvmem_reg_read;
    config.size = vi.usable_leb_size * vi.size;
    config.word_size = 1;
    config.stride = 1;
    config.read_only = true;
    config.root_only = true;
    config.ignore_wp = true;
    config.of_node = np;
    unv.ubi_num = vi.ubi_num;
    unv.vol_id = vi.vol_id;
    unv.usable_leb_size = vi.usable_leb_size;
    unv.nvmem = nvmem_register(&config);
    if (IS_ERR(unv.nvmem)) {
    ret = dev_err_probe(vi.dev, PTR_ERR(unv.nvmem),
    "Failed to register NVMEM device\n");
    kfree(unv);
    return ret;
    }
    mutex_lock(&devices_mutex);
    list_add_tail(&unv.list, &nvmem_devices);
    mutex_unlock(&devices_mutex);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ubi_nvmem_remove(vi: *mut ubi_volume_info) {
    static void ubi_nvmem_remove(struct ubi_volume_info *vi)
    {
    struct ubi_nvmem *unv_c, *unv = core::ptr::null_mut();
    mutex_lock(&devices_mutex);
    list_for_each_entry(unv_c, &nvmem_devices, list)
    if (unv_c.ubi_num == vi.ubi_num && unv_c.vol_id == vi.vol_id) {
    unv = unv_c;
    break;
    }
    if (!unv) {
    mutex_unlock(&devices_mutex);
    return;
    }
    list_del(&unv.list);
    mutex_unlock(&devices_mutex);
    nvmem_unregister(unv.nvmem);
    kfree(unv);
    }
//
// nvmem_notify - UBI notification handler.
// @nb: registered notifier block
// @l: notification type
// @ns_ptr: pointer to the &struct ubi_notification object
//
    static int nvmem_notify(struct notifier_block *nb, unsigned long l,
    void *ns_ptr)
    {
    struct ubi_notification *nt = ns_ptr;
    switch (l) {
    case UBI_VOLUME_RESIZED:
    ubi_nvmem_remove(&nt.vi);
    fallthrough;
    case UBI_VOLUME_ADDED:
    ubi_nvmem_add(&nt.vi);
    break;
    case UBI_VOLUME_SHUTDOWN:
    ubi_nvmem_remove(&nt.vi);
    break;
    default:
    break;
    }
    return NOTIFY_OK;
    }
    static struct notifier_block nvmem_notifier = {
    .notifier_call = nvmem_notify,
    };
#[no_mangle]
unsafe extern "C" fn ubi_nvmem_init() -> int __init {
    static int __init ubi_nvmem_init(void)
    {
    return ubi_register_volume_notifier(&nvmem_notifier, 0);
    }
#[no_mangle]
unsafe extern "C" fn ubi_nvmem_exit() -> void __exit {
    static void __exit ubi_nvmem_exit(void)
    {
    struct ubi_nvmem *unv, *tmp;
    mutex_lock(&devices_mutex);
    list_for_each_entry_safe(unv, tmp, &nvmem_devices, list) {
    nvmem_unregister(unv.nvmem);
    list_del(&unv.list);
    kfree(unv);
    }
    mutex_unlock(&devices_mutex);
    ubi_unregister_volume_notifier(&nvmem_notifier);
    }
    module_init(ubi_nvmem_init);
    module_exit(ubi_nvmem_exit);
    MODULE_DESCRIPTION("NVMEM layer over UBI volumes");
    MODULE_AUTHOR("Daniel Golle");
    MODULE_LICENSE("GPL");
