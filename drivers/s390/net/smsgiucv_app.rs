//! Automatically rewritten from C to Rust
//! Source: drivers/s390/net/smsgiucv_app.c
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
// Deliver z/VM CP special messages (SMSG) as uevents.
//
// The driver registers for z/VM CP special messages with the
// "APP" prefix. Incoming messages are delivered to user space
// as uevents.
//
// Copyright IBM Corp. 2010
// Author(s): Hendrik Brueckner <brueckner@linux.vnet.ibm.com>
//

// prefix used for SMSG registration

// SMSG related uevent environment variables

    strlen(SMSG_PREFIX) + 1)

// z/VM user ID which is permitted to send SMSGs
// If the value is undefined or empty (""), special messages are
// accepted from any z/VM user ID.
    static char *sender;
    module_param(sender, charp, 0400);
    MODULE_PARM_DESC(sender, "z/VM user ID from which CP SMSGs are accepted");
// SMSG device representation
    static struct device *smsg_app_dev;
// list element for queuing received messages for delivery
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smsg_app_event {
    pub list: list_head,
    pub buf: *mut c_char,
    pub envp: [*mut c_char; 4],
}

// queue for outgoing uevents
    static LIST_HEAD(smsg_event_queue);
    static DEFINE_SPINLOCK(smsg_event_queue_lock);
#[no_mangle]
unsafe extern "C" fn smsg_app_event_free(ev: *mut smsg_app_event) {
    static void smsg_app_event_free(struct smsg_app_event *ev)
    {
    kfree(ev.buf);
    kfree(ev);
    }
    static struct smsg_app_event *smsg_app_event_alloc(const char *from,
    const char *msg)
    {
    struct smsg_app_event *ev;
    ev = kzalloc_obj(*ev, GFP_ATOMIC);
    if (!ev)
    return core::ptr::null_mut();
    ev.buf = kzalloc(ENV_SENDER_LEN + ENV_PREFIX_LEN +
    ENV_TEXT_LEN(msg), GFP_ATOMIC);
    if (!ev.buf) {
    kfree(ev);
    return core::ptr::null_mut();
    }
// setting up environment pointers into buf
    ev.envp[0] = ev.buf;
    ev.envp[1] = ev.envp[0] + ENV_SENDER_LEN;
    ev.envp[2] = ev.envp[1] + ENV_PREFIX_LEN;
    ev.envp[3] = core::ptr::null_mut();
// setting up environment: sender, prefix name, and message text
    scnprintf(ev.envp[0], ENV_SENDER_LEN, ENV_SENDER_STR "%s", from);
    scnprintf(ev.envp[1], ENV_PREFIX_LEN, ENV_PREFIX_STR "%s",
    SMSG_PREFIX);
    scnprintf(ev.envp[2], ENV_TEXT_LEN(msg), ENV_TEXT_STR "%s", msg);
    return ev;
    }
#[no_mangle]
unsafe extern "C" fn smsg_event_work_fn(work: *mut work_struct) {
    static void smsg_event_work_fn(struct work_struct *work)
    {
    LIST_HEAD(event_queue);
    struct smsg_app_event *p, *n;
    struct device *dev;
    dev = get_device(smsg_app_dev);
    if (!dev)
    return;
    spin_lock_bh(&smsg_event_queue_lock);
    list_splice_init(&smsg_event_queue, &event_queue);
    spin_unlock_bh(&smsg_event_queue_lock);
    list_for_each_entry_safe(p, n, &event_queue, list) {
    list_del(&p.list);
    kobject_uevent_env(&dev.kobj, KOBJ_CHANGE, p.envp);
    smsg_app_event_free(p);
    }
    put_device(dev);
    }
    static DECLARE_WORK(smsg_event_work, smsg_event_work_fn);
#[no_mangle]
unsafe extern "C" fn smsg_app_callback(from: *const c_char, msg: *mut c_char) {
    static void smsg_app_callback(const char *from, char *msg)
    {
    struct smsg_app_event *se;
// check if the originating z/VM user ID matches
// the configured sender.
    if (sender && strlen(sender) > 0 && strcmp(from, sender) != 0)
    return;
// get start of message text (skip prefix and leading blanks)
    msg += strlen(SMSG_PREFIX);
    while (*msg && isspace(*msg))
    msg++;
    if (*msg == '\0')
    return;
// allocate event list element and its environment
    se = smsg_app_event_alloc(from, msg);
    if (!se)
    return;
// queue event and schedule work function
    spin_lock(&smsg_event_queue_lock);
    list_add_tail(&se.list, &smsg_event_queue);
    spin_unlock(&smsg_event_queue_lock);
    schedule_work(&smsg_event_work);
    return;
    }
#[no_mangle]
unsafe extern "C" fn smsgiucv_app_init() -> int __init {
    static int __init smsgiucv_app_init(void)
    {
    struct device_driver *smsgiucv_drv;
    int rc;
    if (!machine_is_vm())
    return -ENODEV;
    smsgiucv_drv = driver_find(SMSGIUCV_DRV_NAME, &iucv_bus);
    if (!smsgiucv_drv)
    return -ENODEV;
    smsg_app_dev = iucv_alloc_device(core::ptr::null_mut(), smsgiucv_drv, core::ptr::null_mut(), "smsgiucv_app");
    if (!smsg_app_dev)
    return -ENOMEM;
    rc = device_register(smsg_app_dev);
    if (rc) {
    put_device(smsg_app_dev);
    goto fail;
    }
// convert sender to uppercase characters
    if (sender) {
    let mut len: c_int = strlen(sender);
    while (len--)
    sender[len] = toupper(sender[len]);
    }
// register with the smsgiucv device driver
    rc = smsg_register_callback(SMSG_PREFIX, smsg_app_callback);
    if (rc) {
    device_unregister(smsg_app_dev);
    goto fail;
    }
    rc = 0;
    fail:
    return rc;
    }
    module_init(smsgiucv_app_init);
#[no_mangle]
unsafe extern "C" fn smsgiucv_app_exit() -> void __exit {
    static void __exit smsgiucv_app_exit(void)
    {
// unregister callback
    smsg_unregister_callback(SMSG_PREFIX, smsg_app_callback);
// cancel pending work and flush any queued event work
    cancel_work_sync(&smsg_event_work);
    smsg_event_work_fn(&smsg_event_work);
    device_unregister(smsg_app_dev);
    }
    module_exit(smsgiucv_app_exit);
    MODULE_LICENSE("GPL v2");
    MODULE_DESCRIPTION("Deliver z/VM CP SMSG as uevents");
    MODULE_AUTHOR("Hendrik Brueckner <brueckner@linux.vnet.ibm.com>");
