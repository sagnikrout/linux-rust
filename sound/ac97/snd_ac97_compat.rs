//! Automatically rewritten from C to Rust
//! Source: sound/ac97/snd_ac97_compat.c
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
// Copyright (C) 2016 Robert Jarzmik <robert.jarzmik@free.fr>
//

#[no_mangle]
unsafe extern "C" fn compat_ac97_release(dev: *mut device) {
    static void compat_ac97_release(struct device *dev)
    {
    kfree(to_ac97_t(dev));
    }
#[no_mangle]
unsafe extern "C" fn compat_ac97_reset(ac97: *mut snd_ac97) {
    static void compat_ac97_reset(struct snd_ac97 *ac97)
    {
    struct ac97_codec_device *adev = to_ac97_device(ac97.private_data);
    struct ac97_controller *actrl = adev.ac97_ctrl;
    if (actrl.ops.reset)
    actrl.ops.reset(actrl);
    }
#[no_mangle]
unsafe extern "C" fn compat_ac97_warm_reset(ac97: *mut snd_ac97) {
    static void compat_ac97_warm_reset(struct snd_ac97 *ac97)
    {
    struct ac97_codec_device *adev = to_ac97_device(ac97.private_data);
    struct ac97_controller *actrl = adev.ac97_ctrl;
    if (actrl.ops.warm_reset)
    actrl.ops.warm_reset(actrl);
    }
    static void compat_ac97_write(struct snd_ac97 *ac97, unsigned short reg,
    unsigned short val)
    {
    struct ac97_codec_device *adev = to_ac97_device(ac97.private_data);
    struct ac97_controller *actrl = adev.ac97_ctrl;
    actrl.ops.write(actrl, ac97.num, reg, val);
    }
    static unsigned short compat_ac97_read(struct snd_ac97 *ac97,
    unsigned short reg)
    {
    struct ac97_codec_device *adev = to_ac97_device(ac97.private_data);
    struct ac97_controller *actrl = adev.ac97_ctrl;
    return actrl.ops.read(actrl, ac97.num, reg);
    }
    static const struct snd_ac97_bus_ops compat_snd_ac97_bus_ops = {
    .reset = compat_ac97_reset,
    .warm_reset = compat_ac97_warm_reset,
    .write = compat_ac97_write,
    .read = compat_ac97_read,
    };
    static struct snd_ac97_bus compat_soc_ac97_bus = {
    .ops = &compat_snd_ac97_bus_ops,
    };
    struct snd_ac97 *snd_ac97_compat_alloc(struct ac97_codec_device *adev)
    {
    struct snd_ac97 *ac97;
    int ret;
    ac97 = kzalloc_obj(struct snd_ac97);
    if (ac97 == core::ptr::null_mut())
    return ERR_PTR(-ENOMEM);
    ac97.private_data = adev;
    ac97.bus = &compat_soc_ac97_bus;
    ac97.dev.parent = &adev.dev;
    ac97.dev.release = compat_ac97_release;
    dev_set_name(&ac97.dev, "%s-compat", dev_name(&adev.dev));
    ret = device_register(&ac97.dev);
    if (ret) {
    put_device(&ac97.dev);
    return ERR_PTR(ret);
    }
    return ac97;
    }
    EXPORT_SYMBOL_GPL(snd_ac97_compat_alloc);
#[no_mangle]
pub unsafe extern "C" fn snd_ac97_compat_release(ac97: *mut snd_ac97) {
    void snd_ac97_compat_release(struct snd_ac97 *ac97)
    {
    device_unregister(&ac97.dev);
    }
    EXPORT_SYMBOL_GPL(snd_ac97_compat_release);
    int snd_ac97_reset(struct snd_ac97 *ac97, bool try_warm, unsigned int id,
    unsigned int id_mask)
    {
    struct ac97_codec_device *adev = to_ac97_device(ac97.private_data);
    struct ac97_controller *actrl = adev.ac97_ctrl;
    unsigned int scanned;
    if (try_warm) {
    compat_ac97_warm_reset(ac97);
    scanned = snd_ac97_bus_scan_one(actrl, adev.num);
    if (ac97_ids_match(scanned, adev.vendor_id, id_mask))
    return 1;
    }
    compat_ac97_reset(ac97);
    compat_ac97_warm_reset(ac97);
    scanned = snd_ac97_bus_scan_one(actrl, adev.num);
    if (ac97_ids_match(scanned, adev.vendor_id, id_mask))
    return 0;
    return -ENODEV;
    }
    EXPORT_SYMBOL_GPL(snd_ac97_reset);
