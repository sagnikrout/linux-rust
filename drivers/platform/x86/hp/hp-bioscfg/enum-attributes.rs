//! Automatically rewritten from C to Rust
//! Source: drivers/platform/x86/hp/hp-bioscfg/enum-attributes.c
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
// BIOS Enumeration GUID for use with hp-bioscfg driver.
//
// Copyright (c) 2022 HP Development Company, L.P.
//

    GET_INSTANCE_ID(enumeration);
#[no_mangle]
unsafe extern "C" fn current_value_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> isize {
    static ssize_t current_value_show(struct kobject *kobj, struct kobj_attribute *attr, char *buf)
    {
    let mut instance_id: c_int = get_enumeration_instance_id(kobj);
    if (instance_id < 0)
    return -EIO;
    return sysfs_emit(buf, "%s\n",
    bioscfg_drv.enumeration_data[instance_id].current_value);
    }
//
// validate_enumeration_input() -
// Validate input of current_value against possible values
//
// @instance_id: The instance on which input is validated
// @buf: Input value
//
#[no_mangle]
unsafe extern "C" fn validate_enumeration_input(instance_id: c_int, buf: *const c_char) -> c_int {
    static int validate_enumeration_input(int instance_id, const char *buf)
    {
    int i;
    let mut found: c_int = 0;
    struct enumeration_data *enum_data = &bioscfg_drv.enumeration_data[instance_id];
// Is it a read only attribute
    if (enum_data.common.is_readonly)
    return -EIO;
    for (i = 0; i < enum_data.possible_values_size && !found; i++)
    if (!strcmp(enum_data.possible_values[i], buf))
    found = 1;
    if (!found)
    return -EINVAL;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn update_enumeration_value(instance_id: c_int, attr_value: *mut c_char) {
    static void update_enumeration_value(int instance_id, char *attr_value)
    {
    struct enumeration_data *enum_data = &bioscfg_drv.enumeration_data[instance_id];
    strscpy(enum_data.current_value, attr_value);
    }
    ATTRIBUTE_S_COMMON_PROPERTY_SHOW(display_name, enumeration);
    static struct kobj_attribute enumeration_display_name =
    __ATTR_RO(display_name);
    ATTRIBUTE_PROPERTY_STORE(current_value, enumeration);
    static struct kobj_attribute enumeration_current_val =
    __ATTR_RW(current_value);
    ATTRIBUTE_VALUES_PROPERTY_SHOW(possible_values, enumeration, SEMICOLON_SEP);
    static struct kobj_attribute enumeration_poss_val =
    __ATTR_RO(possible_values);
    static ssize_t type_show(struct kobject *kobj, struct kobj_attribute *attr,
    char *buf)
    {
    return sysfs_emit(buf, "enumeration\n");
    }
    static struct kobj_attribute enumeration_type =
    __ATTR_RO(type);
    static struct attribute *enumeration_attrs[] = {
    &common_display_langcode.attr,
    &enumeration_display_name.attr,
    &enumeration_current_val.attr,
    &enumeration_poss_val.attr,
    &enumeration_type.attr,
    core::ptr::null_mut()
    };
    static const struct attribute_group enumeration_attr_group = {
    .attrs = enumeration_attrs,
    };
#[no_mangle]
pub unsafe extern "C" fn hp_alloc_enumeration_data() -> c_int {
    int hp_alloc_enumeration_data(void)
    {
    bioscfg_drv.enumeration_instances_count =
    hp_get_instance_count(HP_WMI_BIOS_ENUMERATION_GUID);
    if (!bioscfg_drv.enumeration_instances_count)
    return -EINVAL;
    bioscfg_drv.enumeration_data = kvzalloc_objs(*bioscfg_drv.enumeration_data,
    bioscfg_drv.enumeration_instances_count);
    if (!bioscfg_drv.enumeration_data) {
    bioscfg_drv.enumeration_instances_count = 0;
    return -ENOMEM;
    }
    return 0;
    }
// Expected Values types associated with each element
    static const acpi_object_type expected_enum_types[] = {
    [NAME] = ACPI_TYPE_STRING,
    [VALUE] = ACPI_TYPE_STRING,
    [PATH] = ACPI_TYPE_STRING,
    [IS_READONLY] = ACPI_TYPE_INTEGER,
    [DISPLAY_IN_UI] = ACPI_TYPE_INTEGER,
    [REQUIRES_PHYSICAL_PRESENCE] = ACPI_TYPE_INTEGER,
    [SEQUENCE] = ACPI_TYPE_INTEGER,
    [PREREQUISITES_SIZE] = ACPI_TYPE_INTEGER,
    [PREREQUISITES] = ACPI_TYPE_STRING,
    [SECURITY_LEVEL] = ACPI_TYPE_INTEGER,
    [ENUM_CURRENT_VALUE] = ACPI_TYPE_STRING,
    [ENUM_SIZE] = ACPI_TYPE_INTEGER,
    [ENUM_POSSIBLE_VALUES] = ACPI_TYPE_STRING,
    };
    static int hp_populate_enumeration_elements_from_package(union acpi_object *enum_obj,
    int enum_obj_count,
    int instance_id)
    {
    char *str_value = core::ptr::null_mut();
    int value_len;
    let mut size: u32 = 0;
    let mut int_value: u32 = 0;
    let mut elem: c_int = 0;
    int reqs;
    int pos_values;
    int ret;
    int eloc;
    struct enumeration_data *enum_data = &bioscfg_drv.enumeration_data[instance_id];
    for (elem = 1, eloc = 1; elem < enum_obj_count; elem++, eloc++) {
// ONLY look at the first ENUM_ELEM_CNT elements
    if (eloc == ENUM_ELEM_CNT)
    goto exit_enumeration_package;
    switch (enum_obj[elem].type) {
    case ACPI_TYPE_STRING:
    if (PREREQUISITES != elem && ENUM_POSSIBLE_VALUES != elem) {
    ret = hp_convert_hexstr_to_str(enum_obj[elem].string.pointer,
    enum_obj[elem].string.length,
    &str_value, &value_len);
    if (ret)
    return -EINVAL;
    }
    break;
    case ACPI_TYPE_INTEGER:
    int_value = (u32)enum_obj[elem].integer.value;
    break;
    default:
    pr_warn("Unsupported object type [%d]\n", enum_obj[elem].type);
    continue;
    }
// Check that both expected and read object type match
    if (expected_enum_types[eloc] != enum_obj[elem].type) {
    pr_warn("Unexpected element type at elem %d: expected %d, got %d, skipping\n",
    elem, expected_enum_types[eloc], enum_obj[elem].type);
    kfree(str_value);
    str_value = core::ptr::null_mut();
    continue;
    }
// Assign appropriate element value to corresponding field
    switch (eloc) {
    case NAME:
    case VALUE:
    break;
    case PATH:
    strscpy(enum_data.common.path, str_value);
    break;
    case IS_READONLY:
    enum_data.common.is_readonly = int_value;
    break;
    case DISPLAY_IN_UI:
    enum_data.common.display_in_ui = int_value;
    break;
    case REQUIRES_PHYSICAL_PRESENCE:
    enum_data.common.requires_physical_presence = int_value;
    break;
    case SEQUENCE:
    enum_data.common.sequence = int_value;
    break;
    case PREREQUISITES_SIZE:
    if (int_value > MAX_PREREQUISITES_SIZE) {
    pr_warn("Prerequisites size value exceeded the maximum number of elements supported or data may be malformed\n");
    int_value = MAX_PREREQUISITES_SIZE;
    }
    enum_data.common.prerequisites_size = int_value;
//
// This step is needed to keep the expected
// element list pointing to the right obj[elem].type
// when the size is zero. PREREQUISITES
// object is omitted by BIOS when the size is
// zero.
//
    if (int_value == 0)
    eloc++;
    break;
    case PREREQUISITES:
    size = min_t(u32, enum_data.common.prerequisites_size, MAX_PREREQUISITES_SIZE);
    for (reqs = 0; reqs < size; reqs++) {
    if (elem + reqs >= enum_obj_count) {
    pr_err("Error enum-objects package is too small\n");
    return -EINVAL;
    }
    ret = hp_convert_hexstr_to_str(enum_obj[elem + reqs].string.pointer,
    enum_obj[elem + reqs].string.length,
    &str_value, &value_len);
    if (ret)
    return -EINVAL;
    strscpy(enum_data.common.prerequisites[reqs], str_value);
    kfree(str_value);
    str_value = core::ptr::null_mut();
    }
    if (size)
    elem += size - 1;
    break;
    case SECURITY_LEVEL:
    enum_data.common.security_level = int_value;
    break;
    case ENUM_CURRENT_VALUE:
    strscpy(enum_data.current_value, str_value);
    break;
    case ENUM_SIZE:
    if (int_value > MAX_VALUES_SIZE) {
    pr_warn("Possible number values size value exceeded the maximum number of elements supported or data may be malformed\n");
    int_value = MAX_VALUES_SIZE;
    }
    enum_data.possible_values_size = int_value;
//
// This step is needed to keep the expected
// element list pointing to the right obj[elem].type
// when the size is zero. POSSIBLE_VALUES
// object is omitted by BIOS when the size is zero.
//
    if (int_value == 0)
    eloc++;
    break;
    case ENUM_POSSIBLE_VALUES:
    size = enum_data.possible_values_size;
    for (pos_values = 0; pos_values < size && pos_values < MAX_VALUES_SIZE;
    pos_values++) {
    if (elem + pos_values >= enum_obj_count) {
    pr_err("Error enum-objects package is too small\n");
    return -EINVAL;
    }
    ret = hp_convert_hexstr_to_str(enum_obj[elem + pos_values].string.pointer,
    enum_obj[elem + pos_values].string.length,
    &str_value, &value_len);
    if (ret)
    return -EINVAL;
//
// ignore strings when possible values size
// is greater than MAX_VALUES_SIZE
//
    if (size < MAX_VALUES_SIZE)
    strscpy(enum_data.possible_values[pos_values], str_value);
    kfree(str_value);
    str_value = core::ptr::null_mut();
    }
    if (size)
    elem += (size < MAX_VALUES_SIZE ? size : MAX_VALUES_SIZE) - 1;
    break;
    default:
    pr_warn("Invalid element: %d found in Enumeration attribute or data may be malformed\n", elem);
    break;
    }
    kfree(str_value);
    str_value = core::ptr::null_mut();
    }
    exit_enumeration_package:
    kfree(str_value);
    return 0;
    }
//
// hp_populate_enumeration_package_data() -
// Populate all properties of an instance under enumeration attribute
//
// @enum_obj: ACPI object with enumeration data
// @enum_obj_count: Number of elements in @enum_obj
// @instance_id: The instance to enumerate
// @attr_name_kobj: The parent kernel object
//
    int hp_populate_enumeration_package_data(union acpi_object *enum_obj,
    int enum_obj_count,
    int instance_id,
    struct kobject *attr_name_kobj)
    {
    struct enumeration_data *enum_data = &bioscfg_drv.enumeration_data[instance_id];
    enum_data.attr_name_kobj = attr_name_kobj;
    hp_populate_enumeration_elements_from_package(enum_obj,
    enum_obj_count,
    instance_id);
    hp_update_attribute_permissions(enum_data.common.is_readonly,
    &enumeration_current_val);
//
// Several attributes have names such "MONDAY". Friendly
// user nane is generated to make the name more descriptive
//
    hp_friendly_user_name_update(enum_data.common.path,
    attr_name_kobj.name,
    enum_data.common.display_name,
    sizeof(enum_data.common.display_name));
    return sysfs_create_group(attr_name_kobj, &enumeration_attr_group);
    }
    static int hp_populate_enumeration_elements_from_buffer(u8 *buffer_ptr, u32 *buffer_size,
    int instance_id)
    {
    int values;
    struct enumeration_data *enum_data = &bioscfg_drv.enumeration_data[instance_id];
    let mut ret: c_int = 0;
//
// Only data relevant to this driver and its functionality is
// read. BIOS defines the order in which each * element is
// read. Element 0 data is not relevant to this
// driver hence it is ignored. For clarity, all element names
// (DISPLAY_IN_UI) which defines the order in which is read
// and the name matches the variable where the data is stored.
//
// In earlier implementation, reported errors were ignored
// causing the data to remain uninitialized. It is not
// possible to determine if data read from BIOS is valid or
// not. It is for this reason functions may return a error
// without validating the data itself.
//
// VALUE:
    ret = hp_get_string_from_buffer(&buffer_ptr, buffer_size, enum_data.current_value,
    sizeof(enum_data.current_value));
    if (ret < 0)
    goto buffer_exit;
// COMMON:
    ret = hp_get_common_data_from_buffer(&buffer_ptr, buffer_size, &enum_data.common);
    if (ret < 0)
    goto buffer_exit;
// ENUM_CURRENT_VALUE:
    ret = hp_get_string_from_buffer(&buffer_ptr, buffer_size,
    enum_data.current_value,
    sizeof(enum_data.current_value));
    if (ret < 0)
    goto buffer_exit;
// ENUM_SIZE:
    ret = hp_get_integer_from_buffer(&buffer_ptr, buffer_size,
    &enum_data.possible_values_size);
    if (enum_data.possible_values_size > MAX_VALUES_SIZE) {
// Report a message and limit possible values size to maximum value
    pr_warn("Enum Possible size value exceeded the maximum number of elements supported or data may be malformed\n");
    enum_data.possible_values_size = MAX_VALUES_SIZE;
    }
// ENUM_POSSIBLE_VALUES:
    for (values = 0; values < enum_data.possible_values_size; values++) {
    ret = hp_get_string_from_buffer(&buffer_ptr, buffer_size,
    enum_data.possible_values[values],
    sizeof(enum_data.possible_values[values]));
    if (ret < 0)
    break;
    }
    buffer_exit:
    return ret;
    }
//
// hp_populate_enumeration_buffer_data() -
// Populate all properties of an instance under enumeration attribute
//
// @buffer_ptr: Buffer pointer
// @buffer_size: Buffer size
// @instance_id: The instance to enumerate
// @attr_name_kobj: The parent kernel object
//
    int hp_populate_enumeration_buffer_data(u8 *buffer_ptr, u32 *buffer_size,
    int instance_id,
    struct kobject *attr_name_kobj)
    {
    struct enumeration_data *enum_data = &bioscfg_drv.enumeration_data[instance_id];
    let mut ret: c_int = 0;
    enum_data.attr_name_kobj = attr_name_kobj;
// Populate enumeration elements
    ret = hp_populate_enumeration_elements_from_buffer(buffer_ptr, buffer_size,
    instance_id);
    if (ret < 0)
    return ret;
    hp_update_attribute_permissions(enum_data.common.is_readonly,
    &enumeration_current_val);
//
// Several attributes have names such "MONDAY". A Friendlier
// user nane is generated to make the name more descriptive
//
    hp_friendly_user_name_update(enum_data.common.path,
    attr_name_kobj.name,
    enum_data.common.display_name,
    sizeof(enum_data.common.display_name));
    return sysfs_create_group(attr_name_kobj, &enumeration_attr_group);
    }
//
// hp_exit_enumeration_attributes() - Clear all attribute data
//
// Clears all data allocated for this group of attributes
//
#[no_mangle]
pub unsafe extern "C" fn hp_exit_enumeration_attributes() {
    void hp_exit_enumeration_attributes(void)
    {
    int instance_id;
    for (instance_id = 0; instance_id < bioscfg_drv.enumeration_instances_count;
    instance_id++) {
    struct enumeration_data *enum_data = &bioscfg_drv.enumeration_data[instance_id];
    struct kobject *attr_name_kobj = enum_data.attr_name_kobj;
    if (attr_name_kobj)
    sysfs_remove_group(attr_name_kobj, &enumeration_attr_group);
    }
    bioscfg_drv.enumeration_instances_count = 0;
    kvfree(bioscfg_drv.enumeration_data);
    bioscfg_drv.enumeration_data = core::ptr::null_mut();
    }
