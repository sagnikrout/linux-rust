//! Automatically rewritten from C to Rust
//! Source: drivers/md/dm-init.c
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
// Copyright (C) 2017 The Chromium OS Authors <chromium-os-dev@chromium.org>
//
// This file is released under the GPLv2.
//

pub const DM_MAX_DEVICES: c_int = 256;
pub const DM_MAX_TARGETS: c_int = 256;
pub const DM_MAX_STR_SIZE: c_int = 4096;
pub const DM_MAX_WAITFOR: c_int = 256;
    static char *create;
    static char *waitfor[DM_MAX_WAITFOR];
//
// Format: dm-mod.create=<name>,<uuid>,<minor>,<flags>,<table>[,<table>+][;<name>,<uuid>,<minor>,<flags>,<table>[,<table>+]+]
// Table format: <start_sector> <num_sectors> <target_type> <target_args>
// Block devices to wait for to become available before setting up tables:
// dm-mod.waitfor=<device1>[,..,<deviceN>]
//
// See Documentation/admin-guide/device-mapper/dm-init.rst for dm-mod.create="..." format
// details.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dm_device {
    pub dmi: dm_ioctl,
    pub table: [*mut dm_target_spec; DM_MAX_TARGETS],
    pub target_args_array: [*mut c_char; DM_MAX_TARGETS],
    pub list: list_head,
}

    static const char * const dm_allowed_targets[] __initconst = {
    "crypt",
    "delay",
    "linear",
    "snapshot-origin",
    "striped",
    "verity",
    };
#[no_mangle]
unsafe extern "C" fn dm_verify_target_type(target: *const c_char) -> int __init {
    static int __init dm_verify_target_type(const char *target)
    {
    unsigned int i;
    for (i = 0; i < ARRAY_SIZE(dm_allowed_targets); i++) {
    if (!strcmp(dm_allowed_targets[i], target))
    return 0;
    }
    return -EINVAL;
    }
#[no_mangle]
unsafe extern "C" fn dm_setup_cleanup(devices: *mut list_head) -> void __init {
    static void __init dm_setup_cleanup(struct list_head *devices)
    {
    struct dm_device *dev, *tmp;
    unsigned int i;
    list_for_each_entry_safe(dev, tmp, devices, list) {
    list_del(&dev.list);
    for (i = 0; i < dev.dmi.target_count; i++) {
    kfree(dev.table[i]);
    kfree(dev.target_args_array[i]);
    }
    kfree(dev);
    }
    }
//
// str_field_delimit - delimit a string based on a separator char.
// @str: the pointer to the string to delimit.
// @separator: char that delimits the field
//
// Find a @separator and replace it by '\0'.
// Remove leading and trailing spaces.
// Return the remainder string after the @separator.
//
    static char __init *str_field_delimit(char **str, char separator)
    {
    char *s;
// TODO: add support for escaped characters
// str = skip_spaces(*str);
    s = strchr(*str, separator);
// Delimit the field and remove trailing spaces
    if (s)
// s = '\0';
// str = strim(*str);
    return s ? ++s : core::ptr::null_mut();
    }
//
// dm_parse_table_entry - parse a table entry
// @dev: device to store the parsed information.
// @str: the pointer to a string with the format:
// <start_sector> <num_sectors> <target_type> <target_args>[, ...]
//
// Return the remainder string after the table entry, i.e, after the comma which
// delimits the entry or NULL if reached the end of the string.
//
    static char __init *dm_parse_table_entry(struct dm_device *dev, char *str)
    {
    let mut n: c_uint = dev.dmi.target_count - 1;
    struct dm_target_spec *sp;
    unsigned int i;
// fields:
    char *field[4];
    char *next;
    field[0] = str;
// Delimit first 3 fields that are separated by space
    for (i = 0; i < ARRAY_SIZE(field) - 1; i++) {
    field[i + 1] = str_field_delimit(&field[i], ' ');
    if (!field[i + 1])
    return ERR_PTR(-EINVAL);
    }
// Delimit last field that can be terminated by comma
    next = str_field_delimit(&field[i], ',');
    sp = kzalloc_obj(*sp);
    if (!sp)
    return ERR_PTR(-ENOMEM);
    dev.table[n] = sp;
// start_sector
    if (kstrtoull(field[0], 0, &sp.sector_start))
    return ERR_PTR(-EINVAL);
// num_sector
    if (kstrtoull(field[1], 0, &sp.length))
    return ERR_PTR(-EINVAL);
// target_type
    strscpy(sp.target_type, field[2], sizeof(sp.target_type));
    if (dm_verify_target_type(sp.target_type)) {
    DMERR("invalid type \"%s\"", sp.target_type);
    return ERR_PTR(-EINVAL);
    }
// target_args
    dev.target_args_array[n] = kstrndup(field[3], DM_MAX_STR_SIZE,
    GFP_KERNEL);
    if (!dev.target_args_array[n])
    return ERR_PTR(-ENOMEM);
    return next;
    }
//
// dm_parse_table - parse "dm-mod.create=" table field
// @dev: device to store the parsed information.
// @str: the pointer to a string with the format:
// <table>[,<table>+]
//
#[no_mangle]
unsafe extern "C" fn dm_parse_table(dev: *mut dm_device, str: *mut c_char) -> int __init {
    static int __init dm_parse_table(struct dm_device *dev, char *str)
    {
    char *table_entry = str;
    while (table_entry) {
    DMDEBUG("parsing table \"%s\"", str);
    if (++dev.dmi.target_count > DM_MAX_TARGETS) {
    DMERR("too many targets %u > %d",
    dev.dmi.target_count, DM_MAX_TARGETS);
    return -EINVAL;
    }
    table_entry = dm_parse_table_entry(dev, table_entry);
    if (IS_ERR(table_entry)) {
    DMERR("couldn't parse table");
    return PTR_ERR(table_entry);
    }
    }
    return 0;
    }
//
// dm_parse_device_entry - parse a device entry
// @dev: device to store the parsed information.
// @str: the pointer to a string with the format:
// name,uuid,minor,flags,table[; ...]
//
// Return the remainder string after the table entry, i.e, after the semi-colon
// which delimits the entry or NULL if reached the end of the string.
//
    static char __init *dm_parse_device_entry(struct dm_device *dev, char *str)
    {
// There are 5 fields: name,uuid,minor,flags,table;
    char *field[5];
    unsigned int i;
    char *next;
    field[0] = str;
// Delimit first 4 fields that are separated by comma
    for (i = 0; i < ARRAY_SIZE(field) - 1; i++) {
    field[i+1] = str_field_delimit(&field[i], ',');
    if (!field[i+1])
    return ERR_PTR(-EINVAL);
    }
// Delimit last field that can be delimited by semi-colon
    next = str_field_delimit(&field[i], ';');
// name
    strscpy(dev.dmi.name, field[0], sizeof(dev.dmi.name));
// uuid
    strscpy(dev.dmi.uuid, field[1], sizeof(dev.dmi.uuid));
// minor
    if (strlen(field[2])) {
    if (kstrtoull(field[2], 0, &dev.dmi.dev) ||
    dev.dmi.dev >= (1 << MINORBITS))
    return ERR_PTR(-EINVAL);
    dev.dmi.dev = huge_encode_dev((dev_t)dev.dmi.dev);
    dev.dmi.flags |= DM_PERSISTENT_DEV_FLAG;
    }
// flags
    if (!strcmp(field[3], "ro"))
    dev.dmi.flags |= DM_READONLY_FLAG;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: strcmp(field[3], _arg: "rw")) -> else {
    else if (strcmp(field[3], "rw"))
    return ERR_PTR(-EINVAL);
// table
    if (dm_parse_table(dev, field[4]))
    return ERR_PTR(-EINVAL);
    return next;
    }
//
// dm_parse_devices - parse "dm-mod.create=" argument
// @devices: list of struct dm_device to store the parsed information.
// @str: the pointer to a string with the format:
// <device>[;<device>+]
//
#[no_mangle]
unsafe extern "C" fn dm_parse_devices(devices: *mut list_head, str: *mut c_char) -> int __init {
    static int __init dm_parse_devices(struct list_head *devices, char *str)
    {
    let mut ndev: c_ulong = 0;
    struct dm_device *dev;
    char *device = str;
    DMDEBUG("parsing \"%s\"", str);
    while (device) {
    dev = kzalloc_obj(*dev);
    if (!dev)
    return -ENOMEM;
    list_add_tail(&dev.list, devices);
    if (++ndev > DM_MAX_DEVICES) {
    DMERR("too many devices %lu > %d",
    ndev, DM_MAX_DEVICES);
    return -EINVAL;
    }
    device = dm_parse_device_entry(dev, device);
    if (IS_ERR(device)) {
    DMERR("couldn't parse device");
    return PTR_ERR(device);
    }
    }
    return 0;
    }
//
// dm_init_init - parse "dm-mod.create=" argument and configure drivers
//
#[no_mangle]
unsafe extern "C" fn dm_init_init() -> int __init {
    static int __init dm_init_init(void)
    {
    struct dm_device *dev;
    LIST_HEAD(devices);
    char *str;
    int i, r;
    if (!create)
    return 0;
    if (strlen(create) >= DM_MAX_STR_SIZE) {
    DMERR("Argument is too big. Limit is %d", DM_MAX_STR_SIZE);
    return -EINVAL;
    }
    str = kstrndup(create, DM_MAX_STR_SIZE, GFP_KERNEL);
    if (!str)
    return -ENOMEM;
    r = dm_parse_devices(&devices, str);
    if (r)
    goto out;
    DMINFO("waiting for all devices to be available before creating mapped devices");
    wait_for_device_probe();
    for (i = 0; i < ARRAY_SIZE(waitfor); i++) {
    if (waitfor[i]) {
    dev_t dev;
    DMINFO("waiting for device %s ...", waitfor[i]);
    while (early_lookup_bdev(waitfor[i], &dev))
    fsleep(5000);
    }
    }
    if (waitfor[0]) {
    wait_for_device_probe();
    DMINFO("all devices available");
    }
    list_for_each_entry(dev, &devices, list) {
    if (dm_early_create(&dev.dmi, dev.table,
    dev.target_args_array))
    break;
    }
    out:
    kfree(str);
    dm_setup_cleanup(&devices);
    return r;
    }
    late_initcall(dm_init_init);
    module_param(create, charp, 0);
    MODULE_PARM_DESC(create, "Create a mapped device in early boot");
    module_param_array(waitfor, charp, core::ptr::null_mut(), 0);
    MODULE_PARM_DESC(waitfor, "Devices to wait for before setting up tables");
