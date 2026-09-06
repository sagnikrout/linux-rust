//! Automatically rewritten from C to Rust
//! Source: drivers/firmware/dmi-id.c
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
// Export SMBIOS/DMI info via sysfs to userspace
//
// Copyright 2007, Lennart Poettering
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmi_device_attribute {
    pub dev_attr: device_attribute,
    pub field: c_int,
}

    container_of(_dev_attr, struct dmi_device_attribute, dev_attr)
    static ssize_t sys_dmi_field_show(struct device *dev,
    struct device_attribute *attr,
    char *page)
    {
    let mut field: c_int = to_dmi_dev_attr(attr).field;
    ssize_t len;
    len = scnprintf(page, PAGE_SIZE, "%s\n", dmi_get_system_info(field));
    page[len-1] = '\n';
    return len;
    }

    { .dev_attr = __ATTR(_name, _mode, _show, core::ptr::null_mut()),	\
    .field = _field }

    static struct dmi_device_attribute sys_dmi_##_name##_attr =	\
    DMI_ATTR(_name, _mode, sys_dmi_field_show, _field);
    DEFINE_DMI_ATTR_WITH_SHOW(bios_vendor,		0444, DMI_BIOS_VENDOR);
    DEFINE_DMI_ATTR_WITH_SHOW(bios_version,		0444, DMI_BIOS_VERSION);
    DEFINE_DMI_ATTR_WITH_SHOW(bios_date,		0444, DMI_BIOS_DATE);
    DEFINE_DMI_ATTR_WITH_SHOW(sys_vendor,		0444, DMI_SYS_VENDOR);
    DEFINE_DMI_ATTR_WITH_SHOW(bios_release,		0444, DMI_BIOS_RELEASE);
    DEFINE_DMI_ATTR_WITH_SHOW(ec_firmware_release,	0444, DMI_EC_FIRMWARE_RELEASE);
    DEFINE_DMI_ATTR_WITH_SHOW(product_name,		0444, DMI_PRODUCT_NAME);
    DEFINE_DMI_ATTR_WITH_SHOW(product_version,	0444, DMI_PRODUCT_VERSION);
    DEFINE_DMI_ATTR_WITH_SHOW(product_serial,	0400, DMI_PRODUCT_SERIAL);
    DEFINE_DMI_ATTR_WITH_SHOW(product_uuid,		0400, DMI_PRODUCT_UUID);
    DEFINE_DMI_ATTR_WITH_SHOW(product_sku,		0444, DMI_PRODUCT_SKU);
    DEFINE_DMI_ATTR_WITH_SHOW(product_family,	0444, DMI_PRODUCT_FAMILY);
    DEFINE_DMI_ATTR_WITH_SHOW(board_vendor,		0444, DMI_BOARD_VENDOR);
    DEFINE_DMI_ATTR_WITH_SHOW(board_name,		0444, DMI_BOARD_NAME);
    DEFINE_DMI_ATTR_WITH_SHOW(board_version,	0444, DMI_BOARD_VERSION);
    DEFINE_DMI_ATTR_WITH_SHOW(board_serial,		0400, DMI_BOARD_SERIAL);
    DEFINE_DMI_ATTR_WITH_SHOW(board_asset_tag,	0444, DMI_BOARD_ASSET_TAG);
    DEFINE_DMI_ATTR_WITH_SHOW(chassis_vendor,	0444, DMI_CHASSIS_VENDOR);
    DEFINE_DMI_ATTR_WITH_SHOW(chassis_type,		0444, DMI_CHASSIS_TYPE);
    DEFINE_DMI_ATTR_WITH_SHOW(chassis_version,	0444, DMI_CHASSIS_VERSION);
    DEFINE_DMI_ATTR_WITH_SHOW(chassis_serial,	0400, DMI_CHASSIS_SERIAL);
    DEFINE_DMI_ATTR_WITH_SHOW(chassis_asset_tag,	0444, DMI_CHASSIS_ASSET_TAG);
#[no_mangle]
unsafe extern "C" fn ascii_filter(d: *mut c_char, s: *const c_char) {
    static void ascii_filter(char *d, const char *s)
    {
// Filter out characters we don't want to see in the modalias string
    for (; *s; s++)
    if (*s > ' ' && *s < 127 && *s != ':')
// (d++) = *s;
// d = 0;
    }
#[no_mangle]
unsafe extern "C" fn get_modalias(buffer: *mut c_char, buffer_size: usize) -> isize {
    static ssize_t get_modalias(char *buffer, size_t buffer_size)
    {
//
// Note new fields need to be added at the end to keep compatibility
// with udev's hwdb which does matches on "`cat dmi/id/modalias`*".
//
    static const struct mafield {
    const char *prefix;
    int field;
    } fields[] = {
    { "bvn", DMI_BIOS_VENDOR },
    { "bvr", DMI_BIOS_VERSION },
    { "bd",  DMI_BIOS_DATE },
    { "br",  DMI_BIOS_RELEASE },
    { "efr", DMI_EC_FIRMWARE_RELEASE },
    { "svn", DMI_SYS_VENDOR },
    { "pn",  DMI_PRODUCT_NAME },
    { "pvr", DMI_PRODUCT_VERSION },
    { "rvn", DMI_BOARD_VENDOR },
    { "rn",  DMI_BOARD_NAME },
    { "rvr", DMI_BOARD_VERSION },
    { "cvn", DMI_CHASSIS_VENDOR },
    { "ct",  DMI_CHASSIS_TYPE },
    { "cvr", DMI_CHASSIS_VERSION },
    { "sku", DMI_PRODUCT_SKU },
    { "pfa", DMI_PRODUCT_FAMILY },
    { core::ptr::null_mut(),  DMI_NONE }
    };
    ssize_t l, left;
    char *p;
    const struct mafield *f;
    strcpy(buffer, "dmi");
    p = buffer + 3; left = buffer_size - 4;
    for (f = fields; f.prefix && left > 0; f++) {
    const char *c;
    char *t;
    c = dmi_get_system_info(f.field);
    if (!c)
    continue;
    t = kmalloc(strlen(c) + 1, GFP_KERNEL);
    if (!t)
    break;
    ascii_filter(t, c);
    l = scnprintf(p, left, ":%s%s", f.prefix, t);
    kfree(t);
    p += l;
    left -= l;
    }
    p[0] = ':';
    p[1] = 0;
    return p - buffer + 1;
    }
    static ssize_t sys_dmi_modalias_show(struct device *dev,
    struct device_attribute *attr, char *page)
    {
    ssize_t r;
    r = get_modalias(page, PAGE_SIZE-1);
    page[r] = '\n';
    page[r+1] = 0;
    return r+1;
    }
    static struct device_attribute sys_dmi_modalias_attr =
    __ATTR(modalias, 0444, sys_dmi_modalias_show, core::ptr::null_mut());
    static struct attribute *sys_dmi_attributes[DMI_STRING_MAX+2];
    static struct attribute_group sys_dmi_attribute_group = {
    .attrs = sys_dmi_attributes,
    };
    static const struct attribute_group* sys_dmi_attribute_groups[] = {
    &sys_dmi_attribute_group,
    core::ptr::null_mut()
    };
#[no_mangle]
unsafe extern "C" fn dmi_dev_uevent(dev: *const device, env: *mut kobj_uevent_env) -> c_int {
    static int dmi_dev_uevent(const struct device *dev, struct kobj_uevent_env *env)
    {
    ssize_t len;
    if (add_uevent_var(env, "MODALIAS="))
    return -ENOMEM;
    len = get_modalias(&env.buf[env.buflen - 1],
    sizeof(env.buf) - env.buflen);
    if (len >= (sizeof(env.buf) - env.buflen))
    return -ENOMEM;
    env.buflen += len;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dmi_dev_release(dev: *mut device) {
    static void dmi_dev_release(struct device *dev)
    {
    kfree(dev);
    }
    static struct class dmi_class = {
    .name = "dmi",
    .dev_release = dmi_dev_release,
    .dev_uevent = dmi_dev_uevent,
    };
    static struct device *dmi_dev;
// Initialization

    if (dmi_get_system_info(_field)) \
    sys_dmi_attributes[i++] = &sys_dmi_##_name##_attr.dev_attr.attr;
// In a separate function to keep gcc 3.2 happy - do NOT merge this in
    dmi_id_init! */
#[no_mangle]
unsafe extern "C" fn dmi_id_init_attr_table() -> void __init {
    static void __init dmi_id_init_attr_table(void)
    {
    int i;
// Not necessarily all DMI fields are available on all
// systems, hence let's built an attribute table of just
// what's available
    i = 0;
    ADD_DMI_ATTR(bios_vendor,       DMI_BIOS_VENDOR);
    ADD_DMI_ATTR(bios_version,      DMI_BIOS_VERSION);
    ADD_DMI_ATTR(bios_date,         DMI_BIOS_DATE);
    ADD_DMI_ATTR(bios_release,      DMI_BIOS_RELEASE);
    ADD_DMI_ATTR(ec_firmware_release, DMI_EC_FIRMWARE_RELEASE);
    ADD_DMI_ATTR(sys_vendor,        DMI_SYS_VENDOR);
    ADD_DMI_ATTR(product_name,      DMI_PRODUCT_NAME);
    ADD_DMI_ATTR(product_version,   DMI_PRODUCT_VERSION);
    ADD_DMI_ATTR(product_serial,    DMI_PRODUCT_SERIAL);
    ADD_DMI_ATTR(product_uuid,      DMI_PRODUCT_UUID);
    ADD_DMI_ATTR(product_family,    DMI_PRODUCT_FAMILY);
    ADD_DMI_ATTR(product_sku,       DMI_PRODUCT_SKU);
    ADD_DMI_ATTR(board_vendor,      DMI_BOARD_VENDOR);
    ADD_DMI_ATTR(board_name,        DMI_BOARD_NAME);
    ADD_DMI_ATTR(board_version,     DMI_BOARD_VERSION);
    ADD_DMI_ATTR(board_serial,      DMI_BOARD_SERIAL);
    ADD_DMI_ATTR(board_asset_tag,   DMI_BOARD_ASSET_TAG);
    ADD_DMI_ATTR(chassis_vendor,    DMI_CHASSIS_VENDOR);
    ADD_DMI_ATTR(chassis_type,      DMI_CHASSIS_TYPE);
    ADD_DMI_ATTR(chassis_version,   DMI_CHASSIS_VERSION);
    ADD_DMI_ATTR(chassis_serial,    DMI_CHASSIS_SERIAL);
    ADD_DMI_ATTR(chassis_asset_tag, DMI_CHASSIS_ASSET_TAG);
    sys_dmi_attributes[i++] = &sys_dmi_modalias_attr.attr;
    }
#[no_mangle]
unsafe extern "C" fn dmi_id_init() -> int __init {
    static int __init dmi_id_init(void)
    {
    int ret;
    if (!dmi_available)
    return -ENODEV;
    dmi_id_init_attr_table();
    ret = class_register(&dmi_class);
    if (ret)
    return ret;
    dmi_dev = kzalloc_obj(*dmi_dev);
    if (!dmi_dev) {
    ret = -ENOMEM;
    goto fail_class_unregister;
    }
    dmi_dev.class = &dmi_class;
    dev_set_name(dmi_dev, "id");
    dmi_dev.groups = sys_dmi_attribute_groups;
    ret = device_register(dmi_dev);
    if (ret)
    goto fail_put_dmi_dev;
    return 0;
    fail_put_dmi_dev:
    put_device(dmi_dev);
    fail_class_unregister:
    class_unregister(&dmi_class);
    return ret;
    }
    arch_initcall(dmi_id_init);
