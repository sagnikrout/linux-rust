//! Automatically rewritten from C to Rust
//! Source: drivers/ssb/bridge_pcmcia_80211.c
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


//
// Broadcom 43xx PCMCIA-SSB bridge module
//
// Copyright (c) 2007 Michael Buesch <m@bues.ch>
//
// Licensed under the GNU/GPL. See COPYING for details.
//

    static const struct pcmcia_device_id ssb_host_pcmcia_tbl[] = {
    PCMCIA_DEVICE_MANF_CARD(0x2D0, 0x448),
    PCMCIA_DEVICE_MANF_CARD(0x2D0, 0x476),
    PCMCIA_DEVICE_NULL,
    };
    MODULE_DEVICE_TABLE(pcmcia, ssb_host_pcmcia_tbl);
#[no_mangle]
unsafe extern "C" fn ssb_host_pcmcia_probe(dev: *mut pcmcia_device) -> c_int {
    static int ssb_host_pcmcia_probe(struct pcmcia_device *dev)
    {
    struct ssb_bus *ssb;
    let mut err: c_int = -ENOMEM;
    let mut res: c_int = 0;
    ssb = kzalloc_obj(*ssb);
    if (!ssb)
    goto out_error;
    err = -ENODEV;
    dev.config_flags |= CONF_ENABLE_IRQ;
    dev.resource[2].flags |=  WIN_ENABLE | WIN_DATA_WIDTH_16 |
    WIN_USE_WAIT;
    dev.resource[2].start = 0;
    dev.resource[2].end = SSB_CORE_SIZE;
    res = pcmcia_request_window(dev, dev.resource[2], 250);
    if (res != 0)
    goto err_kfree_ssb;
    res = pcmcia_map_mem_page(dev, dev.resource[2], 0);
    if (res != 0)
    goto err_disable;
    if (!dev.irq)
    goto err_disable;
    res = pcmcia_enable_device(dev);
    if (res != 0)
    goto err_disable;
    err = ssb_bus_pcmciabus_register(ssb, dev, dev.resource[2].start);
    if (err)
    goto err_disable;
    dev.priv = ssb;
    return 0;
    err_disable:
    pcmcia_disable_device(dev);
    err_kfree_ssb:
    kfree(ssb);
    out_error:
    dev_err(&dev.dev, "Initialization failed (%d, %d)\n", res, err);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn ssb_host_pcmcia_remove(dev: *mut pcmcia_device) {
    static void ssb_host_pcmcia_remove(struct pcmcia_device *dev)
    {
    struct ssb_bus *ssb = dev.priv;
    ssb_bus_unregister(ssb);
    pcmcia_disable_device(dev);
    kfree(ssb);
    dev.priv = core::ptr::null_mut();
    }

#[no_mangle]
unsafe extern "C" fn ssb_host_pcmcia_suspend(dev: *mut pcmcia_device) -> c_int {
    static int ssb_host_pcmcia_suspend(struct pcmcia_device *dev)
    {
    struct ssb_bus *ssb = dev.priv;
    return ssb_bus_suspend(ssb);
    }
#[no_mangle]
unsafe extern "C" fn ssb_host_pcmcia_resume(dev: *mut pcmcia_device) -> c_int {
    static int ssb_host_pcmcia_resume(struct pcmcia_device *dev)
    {
    struct ssb_bus *ssb = dev.priv;
    return ssb_bus_resume(ssb);
    }

    static struct pcmcia_driver ssb_host_pcmcia_driver = {
    .owner		= THIS_MODULE,
    .name		= "ssb-pcmcia",
    .id_table	= ssb_host_pcmcia_tbl,
    .probe		= ssb_host_pcmcia_probe,
    .remove		= ssb_host_pcmcia_remove,
    .suspend	= ssb_host_pcmcia_suspend,
    .resume		= ssb_host_pcmcia_resume,
    };
    static int pcmcia_init_failed;
//
// These are not module init/exit functions!
// The module_pcmcia_driver() helper cannot be used here.
//
#[no_mangle]
pub unsafe extern "C" fn ssb_host_pcmcia_init() -> c_int {
    int ssb_host_pcmcia_init(void)
    {
    pcmcia_init_failed = pcmcia_register_driver(&ssb_host_pcmcia_driver);
    return pcmcia_init_failed;
    }
#[no_mangle]
pub unsafe extern "C" fn ssb_host_pcmcia_exit() {
    void ssb_host_pcmcia_exit(void)
    {
    if (!pcmcia_init_failed)
    pcmcia_unregister_driver(&ssb_host_pcmcia_driver);
    }
