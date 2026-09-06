//! Automatically rewritten from C to Rust
//! Source: drivers/media/pci/cx18/cx18-alsa-main.c
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
// ALSA interface to cx18 PCM capture streams
//
// Copyright (C) 2009  Andy Walls <awalls@md.metrocast.net>
// Copyright (C) 2009  Devin Heitmueller <dheitmueller@kernellabs.com>
//
// Portions of this work were sponsored by ONELAN Limited.
//

    int cx18_alsa_debug;

    do { \
    if (cx18_alsa_debug & 2) \
    printk(KERN_INFO "%s: " fmt, "cx18-alsa", ## arg); \
    } while (0);
    module_param_named(debug, cx18_alsa_debug, int, 0644);
    MODULE_PARM_DESC(debug,
    "Debug level (bitmask). Default: 0\n"
    "\t\t\t  1/0x0001: warning\n"
    "\t\t\t  2/0x0002: info\n");
    MODULE_AUTHOR("Andy Walls");
    MODULE_DESCRIPTION("CX23418 ALSA Interface");
    MODULE_LICENSE("GPL");
    MODULE_VERSION(CX18_VERSION);
    static inline
    struct snd_cx18_card *to_snd_cx18_card(struct v4l2_device *v4l2_dev)
    {
    return to_cx18(v4l2_dev).alsa;
    }
#[no_mangle]
unsafe extern "C" fn snd_cx18_card_free(cxsc: *mut snd_cx18_card) {
    static void snd_cx18_card_free(struct snd_cx18_card *cxsc)
    {
    if (cxsc == core::ptr::null_mut())
    return;
    if (cxsc.v4l2_dev != core::ptr::null_mut())
    to_cx18(cxsc.v4l2_dev).alsa = core::ptr::null_mut();
// FIXME - take any other stopping actions needed
    kfree(cxsc);
    }
#[no_mangle]
unsafe extern "C" fn snd_cx18_card_private_free(sc: *mut snd_card) {
    static void snd_cx18_card_private_free(struct snd_card *sc)
    {
    if (sc == core::ptr::null_mut())
    return;
    snd_cx18_card_free(sc.private_data);
    sc.private_data = core::ptr::null_mut();
    sc.private_free = core::ptr::null_mut();
    }
    static int snd_cx18_card_create(struct v4l2_device *v4l2_dev,
    struct snd_card *sc,
    struct snd_cx18_card **cxsc)
    {
// cxsc = kzalloc_obj(struct snd_cx18_card);
    if (*cxsc == core::ptr::null_mut())
    return -ENOMEM;
    (*cxsc).v4l2_dev = v4l2_dev;
    (*cxsc).sc = sc;
    sc.private_data = *cxsc;
    sc.private_free = snd_cx18_card_private_free;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn snd_cx18_card_set_names(cxsc: *mut snd_cx18_card) -> c_int {
    static int snd_cx18_card_set_names(struct snd_cx18_card *cxsc)
    {
    struct cx18 *cx = to_cx18(cxsc.v4l2_dev);
    struct snd_card *sc = cxsc.sc;
// sc->driver is used by alsa-lib's configurator: simple, unique
    strscpy(sc.driver, "CX23418", sizeof(sc.driver));
// sc->shortname is a symlink in /proc/asound: CX18-M -> cardN
    snprintf(sc.shortname,  sizeof(sc.shortname), "CX18-%d",
    cx.instance);
// sc->longname is read from /proc/asound/cards
    snprintf(sc.longname, sizeof(sc.longname),
    "CX23418 #%d %s TV/FM Radio/Line-In Capture",
    cx.instance, cx.card_name);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn snd_cx18_init(v4l2_dev: *mut v4l2_device) -> c_int {
    static int snd_cx18_init(struct v4l2_device *v4l2_dev)
    {
    struct cx18 *cx = to_cx18(v4l2_dev);
    struct snd_card *sc = core::ptr::null_mut();
    struct snd_cx18_card *cxsc;
    int ret;
// Numbrs steps from "Writing an ALSA Driver" by Takashi Iwai
// (1) Check and increment the device index
// This is a no-op for us.  We'll use the cx->instance
// (2) Create a card instance
    ret = snd_card_new(&cx.pci_dev.dev,
    SNDRV_DEFAULT_IDX1, /* use first available id */
    SNDRV_DEFAULT_STR1, /* xid from end of shortname*/
    THIS_MODULE, 0, &sc);
    if (ret) {
    CX18_ALSA_ERR("%s: snd_card_new() failed with err %d\n",
    __func__, ret);
    goto err_exit;
    }
// (3) Create a main component
    ret = snd_cx18_card_create(v4l2_dev, sc, &cxsc);
    if (ret) {
    CX18_ALSA_ERR("%s: snd_cx18_card_create() failed with err %d\n",
    __func__, ret);
    goto err_exit_free;
    }
// (4) Set the driver ID and name strings
    snd_cx18_card_set_names(cxsc);
    ret = snd_cx18_pcm_create(cxsc);
    if (ret) {
    CX18_ALSA_ERR("%s: snd_cx18_pcm_create() failed with err %d\n",
    __func__, ret);
    goto err_exit_free;
    }
// FIXME - proc files
// (7) Set the driver data and return 0
// We do this out of normal order for PCI drivers to avoid races
    cx.alsa = cxsc;
// (6) Register the card instance
    ret = snd_card_register(sc);
    if (ret) {
    cx.alsa = core::ptr::null_mut();
    CX18_ALSA_ERR("%s: snd_card_register() failed with err %d\n",
    __func__, ret);
    goto err_exit_free;
    }
    return 0;
    err_exit_free:
    if (sc != core::ptr::null_mut())
    snd_card_free(sc);
    kfree(cxsc);
    err_exit:
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn cx18_alsa_load(cx: *mut cx18) -> c_int {
    static int cx18_alsa_load(struct cx18 *cx)
    {
    struct v4l2_device *v4l2_dev = &cx.v4l2_dev;
    struct cx18_stream *s;
    if (v4l2_dev == core::ptr::null_mut()) {
    printk(KERN_ERR "cx18-alsa: %s: struct v4l2_device * is core::ptr::null_mut()\n",
    __func__);
    return 0;
    }
    cx = to_cx18(v4l2_dev);
    if (cx == core::ptr::null_mut()) {
    printk(KERN_ERR "cx18-alsa cx is core::ptr::null_mut()\n");
    return 0;
    }
    s = &cx.streams[CX18_ENC_STREAM_TYPE_PCM];
    if (s.video_dev.v4l2_dev == core::ptr::null_mut()) {
    CX18_DEBUG_ALSA_INFO("%s: PCM stream for card is disabled - skipping\n",
    __func__);
    return 0;
    }
    if (cx.alsa != core::ptr::null_mut()) {
    CX18_ALSA_ERR("%s: struct snd_cx18_card * already exists\n",
    __func__);
    return 0;
    }
    if (snd_cx18_init(v4l2_dev)) {
    CX18_ALSA_ERR("%s: failed to create struct snd_cx18_card\n",
    __func__);
    } else {
    CX18_DEBUG_ALSA_INFO("%s: created cx18 ALSA interface instance\n",
    __func__);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cx18_alsa_init() -> int __init {
    static int __init cx18_alsa_init(void)
    {
    printk(KERN_INFO "cx18-alsa: module loading...\n");
    cx18_ext_init = &cx18_alsa_load;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn snd_cx18_exit(cxsc: *mut snd_cx18_card) -> void __exit {
    static void __exit snd_cx18_exit(struct snd_cx18_card *cxsc)
    {
    struct cx18 *cx = to_cx18(cxsc.v4l2_dev);
// FIXME - pointer checks & shutdown cxsc
    snd_card_free(cxsc.sc);
    cx.alsa = core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn cx18_alsa_exit_callback(dev: *mut device, data: *mut c_void) -> int __exit {
    static int __exit cx18_alsa_exit_callback(struct device *dev, void *data)
    {
    struct v4l2_device *v4l2_dev = dev_get_drvdata(dev);
    struct snd_cx18_card *cxsc;
    if (v4l2_dev == core::ptr::null_mut()) {
    printk(KERN_ERR "cx18-alsa: %s: struct v4l2_device * is core::ptr::null_mut()\n",
    __func__);
    return 0;
    }
    cxsc = to_snd_cx18_card(v4l2_dev);
    if (cxsc == core::ptr::null_mut()) {
    CX18_ALSA_WARN("%s: struct snd_cx18_card * is core::ptr::null_mut()\n",
    __func__);
    return 0;
    }
    snd_cx18_exit(cxsc);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cx18_alsa_exit() -> void __exit {
    static void __exit cx18_alsa_exit(void)
    {
    struct device_driver *drv;
    int ret;
    printk(KERN_INFO "cx18-alsa: module unloading...\n");
    drv = driver_find("cx18", &pci_bus_type);
    ret = driver_for_each_device(drv, core::ptr::null_mut(), core::ptr::null_mut(), cx18_alsa_exit_callback);
    (void)ret;	/* suppress compiler warning */
    cx18_ext_init = core::ptr::null_mut();
    printk(KERN_INFO "cx18-alsa: module unload complete\n");
    }
    module_init(cx18_alsa_init);
    module_exit(cx18_alsa_exit);
