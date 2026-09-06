//! Automatically rewritten from C to Rust
//! Source: drivers/char/ipmi/kcs_bmc_serio.c
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
// Copyright (c) 2021 IBM Corp.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kcs_bmc_serio {
    pub entry: list_head,
    pub client: kcs_bmc_client,
    pub port: *mut serio,
    pub lock: spinlock_t,
}

    static inline struct kcs_bmc_serio *client_to_kcs_bmc_serio(struct kcs_bmc_client *client)
    {
    return container_of(client, struct kcs_bmc_serio, client);
    }
#[no_mangle]
unsafe extern "C" fn kcs_bmc_serio_event(client: *mut kcs_bmc_client) -> irqreturn_t {
    static irqreturn_t kcs_bmc_serio_event(struct kcs_bmc_client *client)
    {
    struct kcs_bmc_serio *priv;
    let mut handled: u8 = IRQ_NONE;
    u8 status;
    priv = client_to_kcs_bmc_serio(client);
    spin_lock(&priv.lock);
    status = kcs_bmc_read_status(client.dev);
    if (status & KCS_BMC_STR_IBF)
    handled = serio_interrupt(priv.port, kcs_bmc_read_data(client.dev), 0);
    spin_unlock(&priv.lock);
    return handled;
    }
    static const struct kcs_bmc_client_ops kcs_bmc_serio_client_ops = {
    .event = kcs_bmc_serio_event,
    };
#[no_mangle]
unsafe extern "C" fn kcs_bmc_serio_open(port: *mut serio) -> c_int {
    static int kcs_bmc_serio_open(struct serio *port)
    {
    struct kcs_bmc_serio *priv = port.port_data;
    return kcs_bmc_enable_device(priv.client.dev, &priv.client);
    }
#[no_mangle]
unsafe extern "C" fn kcs_bmc_serio_close(port: *mut serio) {
    static void kcs_bmc_serio_close(struct serio *port)
    {
    struct kcs_bmc_serio *priv = port.port_data;
    kcs_bmc_disable_device(priv.client.dev, &priv.client);
    }
    static DEFINE_SPINLOCK(kcs_bmc_serio_instances_lock);
    static LIST_HEAD(kcs_bmc_serio_instances);
#[no_mangle]
unsafe extern "C" fn kcs_bmc_serio_add_device(kcs_bmc: *mut kcs_bmc_device) -> c_int {
    static int kcs_bmc_serio_add_device(struct kcs_bmc_device *kcs_bmc)
    {
    struct kcs_bmc_serio *priv;
    struct serio *port;
    priv = devm_kzalloc(kcs_bmc.dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
// Use kzalloc() as the allocation is cleaned up with kfree() via serio_unregister_port()
    port = kzalloc_obj(*port);
    if (!port)
    return -ENOMEM;
    port.id.type = SERIO_8042;
    port.open = kcs_bmc_serio_open;
    port.close = kcs_bmc_serio_close;
    port.port_data = priv;
    port.dev.parent = kcs_bmc.dev;
    spin_lock_init(&priv.lock);
    priv.port = port;
    priv.client.dev = kcs_bmc;
    priv.client.ops = &kcs_bmc_serio_client_ops;
    spin_lock_irq(&kcs_bmc_serio_instances_lock);
    list_add(&priv.entry, &kcs_bmc_serio_instances);
    spin_unlock_irq(&kcs_bmc_serio_instances_lock);
    serio_register_port(port);
    dev_info(kcs_bmc.dev, "Initialised serio client for channel %d", kcs_bmc.channel);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn kcs_bmc_serio_remove_device(kcs_bmc: *mut kcs_bmc_device) -> c_int {
    static int kcs_bmc_serio_remove_device(struct kcs_bmc_device *kcs_bmc)
    {
    struct kcs_bmc_serio *priv = core::ptr::null_mut(), *pos;
    spin_lock_irq(&kcs_bmc_serio_instances_lock);
    list_for_each_entry(pos, &kcs_bmc_serio_instances, entry) {
    if (pos.client.dev == kcs_bmc) {
    priv = pos;
    list_del(&pos.entry);
    break;
    }
    }
    spin_unlock_irq(&kcs_bmc_serio_instances_lock);
    if (!priv)
    return -ENODEV;
// kfree()s priv->port via put_device()
    serio_unregister_port(priv.port);
// Ensure the IBF IRQ is disabled if we were the active client
    kcs_bmc_disable_device(kcs_bmc, &priv.client);
    devm_kfree(priv.client.dev.dev, priv);
    return 0;
    }
    static const struct kcs_bmc_driver_ops kcs_bmc_serio_driver_ops = {
    .add_device = kcs_bmc_serio_add_device,
    .remove_device = kcs_bmc_serio_remove_device,
    };
    static struct kcs_bmc_driver kcs_bmc_serio_driver = {
    .ops = &kcs_bmc_serio_driver_ops,
    };
#[no_mangle]
unsafe extern "C" fn kcs_bmc_serio_init() -> int __init {
    static int __init kcs_bmc_serio_init(void)
    {
    kcs_bmc_register_driver(&kcs_bmc_serio_driver);
    return 0;
    }
    module_init(kcs_bmc_serio_init);
#[no_mangle]
unsafe extern "C" fn kcs_bmc_serio_exit() -> void __exit {
    static void __exit kcs_bmc_serio_exit(void)
    {
    kcs_bmc_unregister_driver(&kcs_bmc_serio_driver);
    }
    module_exit(kcs_bmc_serio_exit);
    MODULE_LICENSE("GPL v2");
    MODULE_AUTHOR("Andrew Jeffery <andrew@aj.id.au>");
    MODULE_DESCRIPTION("Adapter driver for serio access to BMC KCS devices");
