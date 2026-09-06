//! Automatically rewritten from C to Rust
//! Source: block/blk-crypto-sysfs.c
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
// Copyright 2021 Google LLC
//
// sysfs support for blk-crypto.  This file contains the code which exports the
// crypto capabilities of devices via /sys/block/$disk/queue/crypto/.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct blk_crypto_kobj {
    pub kobj: kobject,
    pub profile: *mut blk_crypto_profile,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct blk_crypto_attr {
    pub attr: attribute,
    ssize_t (*show)(struct blk_crypto_profile *profile,
    pub page): *const *const blk_crypto_attr attr, char,
}

    static struct blk_crypto_profile *kobj_to_crypto_profile(struct kobject *kobj)
    {
    return container_of(kobj, struct blk_crypto_kobj, kobj).profile;
    }
    static const struct blk_crypto_attr *attr_to_crypto_attr(const struct attribute *attr)
    {
    return container_of_const(attr, struct blk_crypto_attr, attr);
    }
    static ssize_t hw_wrapped_keys_show(struct blk_crypto_profile *profile,
    const struct blk_crypto_attr *attr, char *page)
    {
// Always show supported, since the file doesn't exist otherwise.
    return sysfs_emit(page, "supported\n");
    }
    static ssize_t max_dun_bits_show(struct blk_crypto_profile *profile,
    const struct blk_crypto_attr *attr, char *page)
    {
    return sysfs_emit(page, "%u\n", 8 * profile.max_dun_bytes_supported);
    }
    static ssize_t num_keyslots_show(struct blk_crypto_profile *profile,
    const struct blk_crypto_attr *attr, char *page)
    {
    return sysfs_emit(page, "%u\n", profile.num_slots);
    }
    static ssize_t raw_keys_show(struct blk_crypto_profile *profile,
    const struct blk_crypto_attr *attr, char *page)
    {
// Always show supported, since the file doesn't exist otherwise.
    return sysfs_emit(page, "supported\n");
    }

    static const struct blk_crypto_attr _name##_attr = __ATTR_RO(_name)
    BLK_CRYPTO_RO_ATTR(hw_wrapped_keys);
    BLK_CRYPTO_RO_ATTR(max_dun_bits);
    BLK_CRYPTO_RO_ATTR(num_keyslots);
    BLK_CRYPTO_RO_ATTR(raw_keys);
    static umode_t blk_crypto_is_visible(struct kobject *kobj,
    const struct attribute *attr, int n)
    {
    struct blk_crypto_profile *profile = kobj_to_crypto_profile(kobj);
    const struct blk_crypto_attr *a = attr_to_crypto_attr(attr);
    if (a == &hw_wrapped_keys_attr &&
    !(profile.key_types_supported & BLK_CRYPTO_KEY_TYPE_HW_WRAPPED))
    return 0;
    if (a == &raw_keys_attr &&
    !(profile.key_types_supported & BLK_CRYPTO_KEY_TYPE_RAW))
    return 0;
    return 0444;
    }
    static const struct attribute *const blk_crypto_attrs[] = {
    &hw_wrapped_keys_attr.attr,
    &max_dun_bits_attr.attr,
    &num_keyslots_attr.attr,
    &raw_keys_attr.attr,
    core::ptr::null_mut(),
    };
    static const struct attribute_group blk_crypto_attr_group = {
    .attrs_const = blk_crypto_attrs,
    .is_visible_const = blk_crypto_is_visible,
    };
//
// The encryption mode attributes.  To avoid hard-coding the list of encryption
// modes, these are initialized at boot time by blk_crypto_sysfs_init().
//
    static struct blk_crypto_attr __blk_crypto_mode_attrs[BLK_ENCRYPTION_MODE_MAX];
    static const struct attribute *blk_crypto_mode_attrs[BLK_ENCRYPTION_MODE_MAX + 1];
    static umode_t blk_crypto_mode_is_visible(struct kobject *kobj,
    const struct attribute *attr, int n)
    {
    struct blk_crypto_profile *profile = kobj_to_crypto_profile(kobj);
    const struct blk_crypto_attr *a = attr_to_crypto_attr(attr);
    let mut mode_num: c_int = a - __blk_crypto_mode_attrs;
    if (profile.modes_supported[mode_num])
    return 0444;
    return 0;
    }
    static ssize_t blk_crypto_mode_show(struct blk_crypto_profile *profile,
    const struct blk_crypto_attr *attr, char *page)
    {
    let mut mode_num: c_int = attr - __blk_crypto_mode_attrs;
    return sysfs_emit(page, "0x%x\n", profile.modes_supported[mode_num]);
    }
    static const struct attribute_group blk_crypto_modes_attr_group = {
    .name = "modes",
    .attrs_const = blk_crypto_mode_attrs,
    .is_visible_const = blk_crypto_mode_is_visible,
    };
    static const struct attribute_group *blk_crypto_attr_groups[] = {
    &blk_crypto_attr_group,
    &blk_crypto_modes_attr_group,
    core::ptr::null_mut(),
    };
    static ssize_t blk_crypto_attr_show(struct kobject *kobj,
    struct attribute *attr, char *page)
    {
    struct blk_crypto_profile *profile = kobj_to_crypto_profile(kobj);
    const struct blk_crypto_attr *a = attr_to_crypto_attr(attr);
    return a.show(profile, a, page);
    }
    static const struct sysfs_ops blk_crypto_attr_ops = {
    .show = blk_crypto_attr_show,
    };
#[no_mangle]
unsafe extern "C" fn blk_crypto_release(kobj: *mut kobject) {
    static void blk_crypto_release(struct kobject *kobj)
    {
    kfree(container_of(kobj, struct blk_crypto_kobj, kobj));
    }
    static const struct kobj_type blk_crypto_ktype = {
    .default_groups = blk_crypto_attr_groups,
    .sysfs_ops	= &blk_crypto_attr_ops,
    .release	= blk_crypto_release,
    };
//
// If the request_queue has a blk_crypto_profile, create the "crypto"
// subdirectory in sysfs (/sys/block/$disk/queue/crypto/).
//
#[no_mangle]
pub unsafe extern "C" fn blk_crypto_sysfs_register(disk: *mut gendisk) -> c_int {
    int blk_crypto_sysfs_register(struct gendisk *disk)
    {
    struct request_queue *q = disk.queue;
    struct blk_crypto_kobj *obj;
    int err;
    if (!q.crypto_profile)
    return 0;
    obj = kzalloc_obj(*obj);
    if (!obj)
    return -ENOMEM;
    obj.profile = q.crypto_profile;
    err = kobject_init_and_add(&obj.kobj, &blk_crypto_ktype,
    &disk.queue_kobj, "crypto");
    if (err) {
    kobject_put(&obj.kobj);
    return err;
    }
    q.crypto_kobject = &obj.kobj;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn blk_crypto_sysfs_unregister(disk: *mut gendisk) {
    void blk_crypto_sysfs_unregister(struct gendisk *disk)
    {
    kobject_put(disk.queue.crypto_kobject);
    }
#[no_mangle]
unsafe extern "C" fn blk_crypto_sysfs_init() -> int __init {
    static int __init blk_crypto_sysfs_init(void)
    {
    int i;
    BUILD_BUG_ON(BLK_ENCRYPTION_MODE_INVALID != 0);
    for (i = 1; i < BLK_ENCRYPTION_MODE_MAX; i++) {
    struct blk_crypto_attr *attr = &__blk_crypto_mode_attrs[i];
    attr.attr.name = blk_crypto_modes[i].name;
    attr.attr.mode = 0444;
    attr.show = blk_crypto_mode_show;
    blk_crypto_mode_attrs[i - 1] = &attr.attr;
    }
    return 0;
    }
    subsys_initcall(blk_crypto_sysfs_init);
