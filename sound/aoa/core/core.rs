//! Automatically rewritten from C to Rust
//! Source: sound/aoa/core/core.c
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
// Apple Onboard Audio driver core
//
// Copyright 2006 Johannes Berg <johannes@sipsolutions.net>
//

    MODULE_DESCRIPTION("Apple Onboard Audio Sound Driver");
    MODULE_AUTHOR("Johannes Berg <johannes@sipsolutions.net>");
    MODULE_LICENSE("GPL");
// We allow only one fabric. This simplifies things,
// and more don't really make that much sense
    static struct aoa_fabric *fabric;
    static LIST_HEAD(codec_list);
#[no_mangle]
unsafe extern "C" fn attach_codec_to_fabric(c: *mut aoa_codec) -> c_int {
    static int attach_codec_to_fabric(struct aoa_codec *c)
    {
    int err;
    if (!try_module_get(c.owner))
    return -EBUSY;
// found_codec has to be assigned
    err = -ENOENT;
    if (fabric.found_codec)
    err = fabric.found_codec(c);
    if (err) {
    module_put(c.owner);
    printk(KERN_ERR "snd-aoa: fabric didn't like codec %s\n",
    c.name);
    return err;
    }
    c.fabric = fabric;
    err = 0;
    if (c.init)
    err = c.init(c);
    if (err) {
    printk(KERN_ERR "snd-aoa: codec %s didn't init\n", c.name);
    c.fabric = core::ptr::null_mut();
    if (fabric.remove_codec)
    fabric.remove_codec(c);
    module_put(c.owner);
    return err;
    }
    if (fabric.attached_codec)
    fabric.attached_codec(c);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn aoa_codec_register(codec: *mut aoa_codec) -> c_int {
    int aoa_codec_register(struct aoa_codec *codec)
    {
    let mut err: c_int = 0;
// if there's a fabric already, we can tell if we
// will want to have this codec, so propagate error
// through. Otherwise, this will happen later...
    if (fabric)
    err = attach_codec_to_fabric(codec);
    if (!err)
    list_add(&codec.list, &codec_list);
    return err;
    }
    EXPORT_SYMBOL_GPL(aoa_codec_register);
#[no_mangle]
pub unsafe extern "C" fn aoa_codec_unregister(codec: *mut aoa_codec) {
    void aoa_codec_unregister(struct aoa_codec *codec)
    {
    list_del(&codec.list);
    if (codec.fabric && codec.exit)
    codec.exit(codec);
    if (fabric && fabric.remove_codec)
    fabric.remove_codec(codec);
    codec.fabric = core::ptr::null_mut();
    module_put(codec.owner);
    }
    EXPORT_SYMBOL_GPL(aoa_codec_unregister);
#[no_mangle]
pub unsafe extern "C" fn aoa_fabric_register(new_fabric: *mut aoa_fabric, dev: *mut device) -> c_int {
    int aoa_fabric_register(struct aoa_fabric *new_fabric, struct device *dev)
    {
    struct aoa_codec *c;
    int err;
// allow querying for presence of fabric
// (i.e. do this test first!)
    if (new_fabric == fabric) {
    err = -EALREADY;
    goto attach;
    }
    if (fabric)
    return -EEXIST;
    if (!new_fabric)
    return -EINVAL;
    err = aoa_alsa_init(new_fabric.name, new_fabric.owner, dev);
    if (err)
    return err;
    fabric = new_fabric;
    attach:
    list_for_each_entry(c, &codec_list, list) {
    if (c.fabric != fabric)
    attach_codec_to_fabric(c);
    }
    return err;
    }
    EXPORT_SYMBOL_GPL(aoa_fabric_register);
#[no_mangle]
pub unsafe extern "C" fn aoa_fabric_unregister(old_fabric: *mut aoa_fabric) {
    void aoa_fabric_unregister(struct aoa_fabric *old_fabric)
    {
    struct aoa_codec *c;
    if (fabric != old_fabric)
    return;
    list_for_each_entry(c, &codec_list, list) {
    if (c.fabric)
    aoa_fabric_unlink_codec(c);
    }
    aoa_alsa_cleanup();
    fabric = core::ptr::null_mut();
    }
    EXPORT_SYMBOL_GPL(aoa_fabric_unregister);
#[no_mangle]
pub unsafe extern "C" fn aoa_fabric_unlink_codec(codec: *mut aoa_codec) {
    void aoa_fabric_unlink_codec(struct aoa_codec *codec)
    {
    if (!codec.fabric) {
    printk(KERN_ERR "snd-aoa: fabric unassigned "
    "in aoa_fabric_unlink_codec\n");
    dump_stack();
    return;
    }
    if (codec.exit)
    codec.exit(codec);
    if (codec.fabric.remove_codec)
    codec.fabric.remove_codec(codec);
    codec.fabric = core::ptr::null_mut();
    module_put(codec.owner);
    }
    EXPORT_SYMBOL_GPL(aoa_fabric_unlink_codec);
#[no_mangle]
unsafe extern "C" fn aoa_init() -> int __init {
    static int __init aoa_init(void)
    {
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn aoa_exit() -> void __exit {
    static void __exit aoa_exit(void)
    {
    aoa_alsa_cleanup();
    }
    module_init(aoa_init);
    module_exit(aoa_exit);
