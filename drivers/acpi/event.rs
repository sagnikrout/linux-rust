//! Automatically rewritten from C to Rust
//! Source: drivers/acpi/event.c
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
// event.c - exporting ACPI events via procfs
//
// Copyright (C) 2001, 2002 Andy Grover <andrew.grover@intel.com>
// Copyright (C) 2001, 2002 Paul Diefenbaugh <paul.s.diefenbaugh@intel.com>
//

// ACPI notifier chain
    static BLOCKING_NOTIFIER_HEAD(acpi_chain_head);
    int acpi_notifier_call_chain(const char *device_class,
    const char *bus_id, u32 type, u32 data)
    {
    struct acpi_bus_event event;
    strscpy(event.device_class, device_class);
    strscpy(event.bus_id, bus_id);
    event.type = type;
    event.data = data;
    return (blocking_notifier_call_chain(&acpi_chain_head, 0, (void *)&event)
    == NOTIFY_BAD) ? -EINVAL : 0;
    }
    EXPORT_SYMBOL(acpi_notifier_call_chain);
#[no_mangle]
pub unsafe extern "C" fn register_acpi_notifier(nb: *mut notifier_block) -> c_int {
    int register_acpi_notifier(struct notifier_block *nb)
    {
    return blocking_notifier_chain_register(&acpi_chain_head, nb);
    }
    EXPORT_SYMBOL(register_acpi_notifier);
#[no_mangle]
pub unsafe extern "C" fn unregister_acpi_notifier(nb: *mut notifier_block) -> c_int {
    int unregister_acpi_notifier(struct notifier_block *nb)
    {
    return blocking_notifier_chain_unregister(&acpi_chain_head, nb);
    }
    EXPORT_SYMBOL(unregister_acpi_notifier);

    static unsigned int acpi_event_seqnum;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_genl_event {
    pub device_class: acpi_device_class,
    pub bus_id: [c_char; 15],
    pub type: u32,
    pub data: u32,
}

// attributes of acpi_genl_family
    enum {
    ACPI_GENL_ATTR_UNSPEC,
    ACPI_GENL_ATTR_EVENT,	/* ACPI event info needed by user space */
    __ACPI_GENL_ATTR_MAX,
    };

// commands supported by the acpi_genl_family
    enum {
    ACPI_GENL_CMD_UNSPEC,
    ACPI_GENL_CMD_EVENT,	/* kernel.user notifications for ACPI events */
    __ACPI_GENL_CMD_MAX,
    };

pub const ACPI_GENL_VERSION: c_uint = 0x01;

    static const struct genl_multicast_group acpi_event_mcgrps[] = {
    { .name = ACPI_GENL_MCAST_GROUP_NAME, },
    };
    static struct genl_family acpi_event_genl_family __ro_after_init = {
    .module = THIS_MODULE,
    .name = ACPI_GENL_FAMILY_NAME,
    .version = ACPI_GENL_VERSION,
    .maxattr = ACPI_GENL_ATTR_MAX,
    .mcgrps = acpi_event_mcgrps,
    .n_mcgrps = ARRAY_SIZE(acpi_event_mcgrps),
    };
    int acpi_bus_generate_netlink_event(const char *device_class,
    const char *bus_id,
    u8 type, int data)
    {
    struct sk_buff *skb;
    struct nlattr *attr;
    struct acpi_genl_event *event;
    void *msg_header;
    int size;
// allocate memory
    size = nla_total_size(sizeof(struct acpi_genl_event)) +
    nla_total_size(0);
    skb = genlmsg_new(size, GFP_ATOMIC);
    if (!skb)
    return -ENOMEM;
// add the genetlink message header
    msg_header = genlmsg_put(skb, 0, acpi_event_seqnum++,
    &acpi_event_genl_family, 0,
    ACPI_GENL_CMD_EVENT);
    if (!msg_header) {
    nlmsg_free(skb);
    return -ENOMEM;
    }
// fill the data
    attr =
    nla_reserve(skb, ACPI_GENL_ATTR_EVENT,
    sizeof(struct acpi_genl_event));
    if (!attr) {
    nlmsg_free(skb);
    return -EINVAL;
    }
    event = nla_data(attr);
    memset(event, 0, sizeof(struct acpi_genl_event));
    strscpy(event.device_class, device_class, sizeof(event.device_class));
    strscpy(event.bus_id, bus_id, sizeof(event.bus_id));
    event.type = type;
    event.data = data;
// send multicast genetlink message
    genlmsg_end(skb, msg_header);
    genlmsg_multicast(&acpi_event_genl_family, skb, 0, 0, GFP_ATOMIC);
    return 0;
    }
    EXPORT_SYMBOL(acpi_bus_generate_netlink_event);
#[no_mangle]
unsafe extern "C" fn acpi_event_genetlink_init() -> int __init {
    static int __init acpi_event_genetlink_init(void)
    {
    return genl_register_family(&acpi_event_genl_family);
    }

    int acpi_bus_generate_netlink_event(const char *device_class,
    const char *bus_id,
    u8 type, int data)
    {
    return 0;
    }
    EXPORT_SYMBOL(acpi_bus_generate_netlink_event);
#[no_mangle]
unsafe extern "C" fn acpi_event_genetlink_init() -> c_int {
    static int acpi_event_genetlink_init(void)
    {
    return -ENODEV;
    }

#[no_mangle]
unsafe extern "C" fn acpi_event_init() -> int __init {
    static int __init acpi_event_init(void)
    {
    int error;
    if (acpi_disabled)
    return 0;
// create genetlink for acpi event
    error = acpi_event_genetlink_init();
    if (error)
    pr_warn("Failed to create genetlink family for ACPI event\n");
    return 0;
    }
    fs_initcall(acpi_event_init);
