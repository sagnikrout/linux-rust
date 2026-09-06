//! Automatically rewritten from C to Rust
//! Source: drivers/platform/x86/dell/dell-wmi-sysman/enum-attributes.c
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
// Functions corresponding to enumeration type attributes under
// BIOS Enumeration GUID for use with dell-wmi-sysman
//
// Copyright (c) 2020 Dell Inc.
//

    get_instance_id(enumeration);
#[no_mangle]
unsafe extern "C" fn append_enum_string(dest: *mut c_char, src: *const c_char) -> c_int {
    static int append_enum_string(char *dest, const char *src)
    {
    let mut dest_len: usize = strlen(dest);
    ssize_t copied;
    if (WARN_ON_ONCE(dest_len >= MAX_BUFF))
    return -EINVAL;
    copied = strscpy(dest + dest_len, src, MAX_BUFF - dest_len);
    if (copied < 0)
    return -EINVAL;
    dest_len += copied;
    copied = strscpy(dest + dest_len, ";", MAX_BUFF - dest_len);
    if (copied < 0)
    return -EINVAL;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn current_value_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> isize {
    static ssize_t current_value_show(struct kobject *kobj, struct kobj_attribute *attr, char *buf)
    {
    let mut instance_id: c_int = get_enumeration_instance_id(kobj);
    union acpi_object *obj;
    ssize_t ret;
    if (instance_id < 0)
    return instance_id;
// need to use specific instance_id and guid combination to get right data
    obj = get_wmiobj_pointer(instance_id, DELL_WMI_BIOS_ENUMERATION_ATTRIBUTE_GUID);
    if (!obj)
    return -EIO;
    if (obj.type != ACPI_TYPE_PACKAGE || obj.package.count < ENUM_MIN_ELEMENTS ||
    obj.package.elements[CURRENT_VAL].type != ACPI_TYPE_STRING) {
    kfree(obj);
    return -EIO;
    }
    ret = snprintf(buf, PAGE_SIZE, "%s\n", obj.package.elements[CURRENT_VAL].string.pointer);
    kfree(obj);
    return ret;
    }
//
// validate_enumeration_input() - Validate input of current_value against possible values
// @instance_id: The instance on which input is validated
// @buf: Input value
//
#[no_mangle]
unsafe extern "C" fn validate_enumeration_input(instance_id: c_int, buf: *const c_char) -> c_int {
    static int validate_enumeration_input(int instance_id, const char *buf)
    {
    char *options, *tmp, *p;
    let mut ret: c_int = -EINVAL;
    options = tmp = kstrdup(wmi_priv.enumeration_data[instance_id].possible_values,
    GFP_KERNEL);
    if (!options)
    return -ENOMEM;
    while ((p = strsep(&options, ";")) != core::ptr::null_mut()) {
    if (!*p)
    continue;
    if (!strcasecmp(p, buf)) {
    ret = 0;
    break;
    }
    }
    kfree(tmp);
    return ret;
    }
    attribute_s_property_show(display_name_language_code, enumeration);
    static struct kobj_attribute displ_langcode =
    __ATTR_RO(display_name_language_code);
    attribute_s_property_show(display_name, enumeration);
    static struct kobj_attribute displ_name =
    __ATTR_RO(display_name);
    attribute_s_property_show(default_value, enumeration);
    static struct kobj_attribute default_val =
    __ATTR_RO(default_value);
    attribute_property_store(current_value, enumeration);
    static struct kobj_attribute current_val =
    __ATTR_RW_MODE(current_value, 0600);
    attribute_s_property_show(dell_modifier, enumeration);
    static struct kobj_attribute modifier =
    __ATTR_RO(dell_modifier);
    attribute_s_property_show(dell_value_modifier, enumeration);
    static struct kobj_attribute value_modfr =
    __ATTR_RO(dell_value_modifier);
    attribute_s_property_show(possible_values, enumeration);
    static struct kobj_attribute poss_val =
    __ATTR_RO(possible_values);
    static ssize_t type_show(struct kobject *kobj, struct kobj_attribute *attr,
    char *buf)
    {
    return sprintf(buf, "enumeration\n");
    }
    static struct kobj_attribute type =
    __ATTR_RO(type);
    static struct attribute *enumeration_attrs[] = {
    &displ_langcode.attr,
    &displ_name.attr,
    &default_val.attr,
    &current_val.attr,
    &modifier.attr,
    &value_modfr.attr,
    &poss_val.attr,
    &type.attr,
    core::ptr::null_mut(),
    };
    static const struct attribute_group enumeration_attr_group = {
    .attrs = enumeration_attrs,
    };
#[no_mangle]
pub unsafe extern "C" fn alloc_enum_data() -> c_int {
    int alloc_enum_data(void)
    {
    let mut ret: c_int = 0;
    wmi_priv.enumeration_instances_count =
    get_instance_count(DELL_WMI_BIOS_ENUMERATION_ATTRIBUTE_GUID);
    wmi_priv.enumeration_data = kzalloc_objs(struct enumeration_data,
    wmi_priv.enumeration_instances_count);
    if (!wmi_priv.enumeration_data) {
    wmi_priv.enumeration_instances_count = 0;
    ret = -ENOMEM;
    }
    return ret;
    }
//
// populate_enum_data() - Populate all properties of an instance under enumeration attribute
// @enumeration_obj: ACPI object with enumeration data
// @instance_id: The instance to enumerate
// @attr_name_kobj: The parent kernel object
// @enum_property_count: Total properties count under enumeration type
//
    int populate_enum_data(union acpi_object *enumeration_obj, int instance_id,
    struct kobject *attr_name_kobj, u32 enum_property_count)
    {
    int i, next_obj, value_modifier_count, possible_values_count;
    wmi_priv.enumeration_data[instance_id].attr_name_kobj = attr_name_kobj;
    if (check_property_type(enumeration, ATTR_NAME, ACPI_TYPE_STRING))
    return -EINVAL;
    strlcpy_attr(wmi_priv.enumeration_data[instance_id].attribute_name,
    enumeration_obj[ATTR_NAME].string.pointer);
    if (check_property_type(enumeration, DISPL_NAME_LANG_CODE, ACPI_TYPE_STRING))
    return -EINVAL;
    strlcpy_attr(wmi_priv.enumeration_data[instance_id].display_name_language_code,
    enumeration_obj[DISPL_NAME_LANG_CODE].string.pointer);
    if (check_property_type(enumeration, DISPLAY_NAME, ACPI_TYPE_STRING))
    return -EINVAL;
    strlcpy_attr(wmi_priv.enumeration_data[instance_id].display_name,
    enumeration_obj[DISPLAY_NAME].string.pointer);
    if (check_property_type(enumeration, DEFAULT_VAL, ACPI_TYPE_STRING))
    return -EINVAL;
    strlcpy_attr(wmi_priv.enumeration_data[instance_id].default_value,
    enumeration_obj[DEFAULT_VAL].string.pointer);
    if (check_property_type(enumeration, MODIFIER, ACPI_TYPE_STRING))
    return -EINVAL;
    strlcpy_attr(wmi_priv.enumeration_data[instance_id].dell_modifier,
    enumeration_obj[MODIFIER].string.pointer);
    next_obj = MODIFIER + 1;
    if (next_obj >= enum_property_count)
    return -EINVAL;
    if (check_property_type(enumeration, next_obj, ACPI_TYPE_INTEGER))
    return -EINVAL;
    value_modifier_count = (uintptr_t)enumeration_obj[next_obj++].string.pointer;
    for (i = 0; i < value_modifier_count; i++) {
    if (next_obj >= enum_property_count)
    return -EINVAL;
    if (check_property_type(enumeration, next_obj, ACPI_TYPE_STRING))
    return -EINVAL;
    if (append_enum_string(wmi_priv.enumeration_data[instance_id].dell_value_modifier,
    enumeration_obj[next_obj++].string.pointer))
    return -EINVAL;
    }
    if (next_obj >= enum_property_count)
    return -EINVAL;
    if (check_property_type(enumeration, next_obj, ACPI_TYPE_INTEGER))
    return -EINVAL;
    possible_values_count = (uintptr_t) enumeration_obj[next_obj++].string.pointer;
    for (i = 0; i < possible_values_count; i++) {
    if (next_obj >= enum_property_count)
    return -EINVAL;
    if (check_property_type(enumeration, next_obj, ACPI_TYPE_STRING))
    return -EINVAL;
    if (append_enum_string(wmi_priv.enumeration_data[instance_id].possible_values,
    enumeration_obj[next_obj++].string.pointer))
    return -EINVAL;
    }
    return sysfs_create_group(attr_name_kobj, &enumeration_attr_group);
    }
//
// exit_enum_attributes() - Clear all attribute data
//
// Clears all data allocated for this group of attributes
//
#[no_mangle]
pub unsafe extern "C" fn exit_enum_attributes() {
    void exit_enum_attributes(void)
    {
    int instance_id;
    for (instance_id = 0; instance_id < wmi_priv.enumeration_instances_count; instance_id++) {
    if (wmi_priv.enumeration_data[instance_id].attr_name_kobj)
    sysfs_remove_group(wmi_priv.enumeration_data[instance_id].attr_name_kobj,
    &enumeration_attr_group);
    }
    wmi_priv.enumeration_instances_count = 0;
    kfree(wmi_priv.enumeration_data);
    wmi_priv.enumeration_data = core::ptr::null_mut();
    }
