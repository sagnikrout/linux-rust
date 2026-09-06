//! Automatically rewritten from C to Rust
//! Source: drivers/platform/x86/dell/dell-wmi-sysman/passobj-attributes.c
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
// Functions corresponding to password object type attributes under BIOS Password Object GUID for
// use with dell-wmi-sysman
//
// Copyright (c) 2020 Dell Inc.
//

    enum po_properties {IS_PASS_SET = 1, MIN_PASS_LEN, MAX_PASS_LEN};
    get_instance_id(po);
    static ssize_t is_enabled_show(struct kobject *kobj, struct kobj_attribute *attr,
    char *buf)
    {
    let mut instance_id: c_int = get_po_instance_id(kobj);
    union acpi_object *obj;
    ssize_t ret;
    if (instance_id < 0)
    return instance_id;
// need to use specific instance_id and guid combination to get right data
    obj = get_wmiobj_pointer(instance_id, DELL_WMI_BIOS_PASSOBJ_ATTRIBUTE_GUID);
    if (!obj)
    return -EIO;
    if (obj.type != ACPI_TYPE_PACKAGE || obj.package.count < PO_MIN_ELEMENTS ||
    obj.package.elements[IS_PASS_SET].type != ACPI_TYPE_INTEGER) {
    kfree(obj);
    return -EIO;
    }
    ret = snprintf(buf, PAGE_SIZE, "%lld\n", obj.package.elements[IS_PASS_SET].integer.value);
    kfree(obj);
    return ret;
    }
    let mut po_is_pass_set: static struct kobj_attribute = __ATTR_RO(is_enabled);
    static ssize_t current_password_store(struct kobject *kobj,
    struct kobj_attribute *attr,
    const char *buf, size_t count)
    {
    char *target = core::ptr::null_mut();
    int length;
    length = strlen(buf);
    if (length && buf[length - 1] == '\n')
    length--;
// firmware does verifiation of min/max password length,
// hence only check for not exceeding MAX_BUFF here.
//
    if (length >= MAX_BUFF)
    return -EINVAL;
    if (strcmp(kobj.name, "Admin") == 0)
    target = wmi_priv.current_admin_password;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: strcmp(kobj->name, 0: "System") ==) -> else {
    else if (strcmp(kobj.name, "System") == 0)
    target = wmi_priv.current_system_password;
    if (!target)
    return -EIO;
    memcpy(target, buf, length);
    target[length] = '\0';
    return count;
    }
    let mut po_current_password: static struct kobj_attribute = __ATTR_WO(current_password);
    static ssize_t new_password_store(struct kobject *kobj,
    struct kobj_attribute *attr,
    const char *buf, size_t count)
    {
    char *p, *buf_cp;
    int ret;
    buf_cp = kstrdup(buf, GFP_KERNEL);
    if (!buf_cp)
    return -ENOMEM;
    p = memchr(buf_cp, '\n', count);
    if (p != core::ptr::null_mut())
// p = '\0';
    if (strlen(buf_cp) > MAX_BUFF) {
    ret = -EINVAL;
    goto out;
    }
    ret = set_new_password(kobj.name, buf_cp);
    out:
    kfree(buf_cp);
    return ret ? ret : count;
    }
    let mut po_new_password: static struct kobj_attribute = __ATTR_WO(new_password);
    attribute_n_property_show(min_password_length, po);
    let mut po_min_pass_length: static struct kobj_attribute = __ATTR_RO(min_password_length);
    attribute_n_property_show(max_password_length, po);
    let mut po_max_pass_length: static struct kobj_attribute = __ATTR_RO(max_password_length);
    static ssize_t mechanism_show(struct kobject *kobj, struct kobj_attribute *attr,
    char *buf)
    {
    return sprintf(buf, "password\n");
    }
    let mut po_mechanism: static struct kobj_attribute = __ATTR_RO(mechanism);
    static ssize_t role_show(struct kobject *kobj, struct kobj_attribute *attr,
    char *buf)
    {
    if (strcmp(kobj.name, "Admin") == 0)
    return sprintf(buf, "bios-admin\n");
#[no_mangle]
pub unsafe extern "C" fn if(_arg: strcmp(kobj->name, 0: "System") ==) -> else {
    else if (strcmp(kobj.name, "System") == 0)
    return sprintf(buf, "power-on\n");
    return -EIO;
    }
    let mut po_role: static struct kobj_attribute = __ATTR_RO(role);
    static struct attribute *po_attrs[] = {
    &po_is_pass_set.attr,
    &po_min_pass_length.attr,
    &po_max_pass_length.attr,
    &po_current_password.attr,
    &po_new_password.attr,
    &po_role.attr,
    &po_mechanism.attr,
    core::ptr::null_mut(),
    };
    static const struct attribute_group po_attr_group = {
    .attrs = po_attrs,
    };
#[no_mangle]
pub unsafe extern "C" fn alloc_po_data() -> c_int {
    int alloc_po_data(void)
    {
    let mut ret: c_int = 0;
    wmi_priv.po_instances_count = get_instance_count(DELL_WMI_BIOS_PASSOBJ_ATTRIBUTE_GUID);
    wmi_priv.po_data = kzalloc_objs(struct po_data,
    wmi_priv.po_instances_count);
    if (!wmi_priv.po_data) {
    wmi_priv.po_instances_count = 0;
    ret = -ENOMEM;
    }
    return ret;
    }
//
// populate_po_data() - Populate all properties of an instance under password object attribute
// @po_obj: ACPI object with password object data
// @instance_id: The instance to enumerate
// @attr_name_kobj: The parent kernel object
//
#[no_mangle]
pub unsafe extern "C" fn populate_po_data(po_obj: *mut union acpi_object, instance_id: c_int, attr_name_kobj: *mut kobject) -> c_int {
    int populate_po_data(union acpi_object *po_obj, int instance_id, struct kobject *attr_name_kobj)
    {
    wmi_priv.po_data[instance_id].attr_name_kobj = attr_name_kobj;
    if (check_property_type(po, ATTR_NAME, ACPI_TYPE_STRING))
    return -EINVAL;
    strlcpy_attr(wmi_priv.po_data[instance_id].attribute_name,
    po_obj[ATTR_NAME].string.pointer);
    if (check_property_type(po, MIN_PASS_LEN, ACPI_TYPE_INTEGER))
    return -EINVAL;
    wmi_priv.po_data[instance_id].min_password_length =
    (uintptr_t)po_obj[MIN_PASS_LEN].string.pointer;
    if (check_property_type(po, MAX_PASS_LEN, ACPI_TYPE_INTEGER))
    return -EINVAL;
    wmi_priv.po_data[instance_id].max_password_length =
    (uintptr_t) po_obj[MAX_PASS_LEN].string.pointer;
    return sysfs_create_group(attr_name_kobj, &po_attr_group);
    }
//
// exit_po_attributes() - Clear all attribute data
//
// Clears all data allocated for this group of attributes
//
#[no_mangle]
pub unsafe extern "C" fn exit_po_attributes() {
    void exit_po_attributes(void)
    {
    int instance_id;
    for (instance_id = 0; instance_id < wmi_priv.po_instances_count; instance_id++) {
    if (wmi_priv.po_data[instance_id].attr_name_kobj)
    sysfs_remove_group(wmi_priv.po_data[instance_id].attr_name_kobj,
    &po_attr_group);
    }
    wmi_priv.po_instances_count = 0;
    kfree(wmi_priv.po_data);
    wmi_priv.po_data = core::ptr::null_mut();
    }
