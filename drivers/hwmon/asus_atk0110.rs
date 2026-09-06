//! Automatically rewritten from C to Rust
//! Source: drivers/hwmon/asus_atk0110.c
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
// Copyright (C) 2007-2009 Luca Tettamanti <kronos.it@gmail.com>
//
// See COPYING in the top level directory of the kernel tree.
//

    static bool new_if;
    module_param(new_if, bool, 0);
    MODULE_PARM_DESC(new_if, "Override detection heuristic and force the use of the new ATK0110 interface");
    static const struct dmi_system_id __initconst atk_force_new_if[] = {
    {
// Old interface has broken MCH temp monitoring
    .ident = "Asus Sabertooth X58",
    .matches = {
    DMI_MATCH(DMI_BOARD_NAME, "SABERTOOTH X58")
    }
    }, {
// Old interface reads the same sensor for fan0 and fan1
    .ident = "Asus M5A78L",
    .matches = {
    DMI_MATCH(DMI_BOARD_NAME, "M5A78L")
    }
    },
    { }
    };
//
// Minimum time between readings, enforced in order to avoid
// hogging the CPU.
//

pub const ATK_MUX_HWMON: c_uint = 0x00000006ULL;
pub const ATK_MUX_MGMT: c_uint = 0x00000011ULL;
pub const ATK_CLASS_MASK: c_uint = 0xff000000ULL;
pub const ATK_CLASS_FREQ_CTL: c_uint = 0x03000000ULL;
pub const ATK_CLASS_FAN_CTL: c_uint = 0x04000000ULL;
pub const ATK_CLASS_HWMON: c_uint = 0x06000000ULL;
pub const ATK_CLASS_MGMT: c_uint = 0x11000000ULL;
pub const ATK_TYPE_MASK: c_uint = 0x00ff0000ULL;
pub const HWMON_TYPE_VOLT: c_uint = 0x00020000ULL;
pub const HWMON_TYPE_TEMP: c_uint = 0x00030000ULL;
pub const HWMON_TYPE_FAN: c_uint = 0x00040000ULL;
pub const ATK_ELEMENT_ID_MASK: c_uint = 0x0000ffffULL;
pub const ATK_EC_ID: c_uint = 0x11060004ULL;
    enum atk_pack_member {
    HWMON_PACK_FLAGS,
    HWMON_PACK_NAME,
    HWMON_PACK_LIMIT1,
    HWMON_PACK_LIMIT2,
    HWMON_PACK_ENABLE
    };
// New package format
pub const _HWMON_NEW_PACK_SIZE: c_int = 7;
pub const _HWMON_NEW_PACK_FLAGS: c_int = 0;
pub const _HWMON_NEW_PACK_NAME: c_int = 1;
pub const _HWMON_NEW_PACK_UNK1: c_int = 2;
pub const _HWMON_NEW_PACK_UNK2: c_int = 3;
pub const _HWMON_NEW_PACK_LIMIT1: c_int = 4;
pub const _HWMON_NEW_PACK_LIMIT2: c_int = 5;
pub const _HWMON_NEW_PACK_ENABLE: c_int = 6;
// Old package format
pub const _HWMON_OLD_PACK_SIZE: c_int = 5;
pub const _HWMON_OLD_PACK_FLAGS: c_int = 0;
pub const _HWMON_OLD_PACK_NAME: c_int = 1;
pub const _HWMON_OLD_PACK_LIMIT1: c_int = 2;
pub const _HWMON_OLD_PACK_LIMIT2: c_int = 3;
pub const _HWMON_OLD_PACK_ENABLE: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct atk_data {
    pub hwmon_dev: *mut device,
    pub atk_handle: acpi_handle,
    pub dev: *mut device,
    pub old_interface: bool,
// old interface
    pub rtmp_handle: acpi_handle,
    pub rvlt_handle: acpi_handle,
    pub rfan_handle: acpi_handle,
// new interface
    pub enumerate_handle: acpi_handle,
    pub read_handle: acpi_handle,
    pub write_handle: acpi_handle,
    pub disable_ec: bool,
    pub voltage_count: c_int,
    pub temperature_count: c_int,
    pub fan_count: c_int,
    pub sensor_list: list_head,
    pub attr_group: attribute_group,
    pub attr_groups: [*const attribute_group; 2],
    struct {
    pub root: *mut dentry,
    pub id: u32,
    pub debugfs: },
}

    typedef ssize_t (*sysfs_show_func)(struct device *dev,
    struct device_attribute *attr, char *buf);
    static const struct acpi_device_id atk_ids[] = {
    {ATK_HID, 0},
    {"", 0},
    };
    MODULE_DEVICE_TABLE(acpi, atk_ids);

#[repr(C)]
#[derive(Copy, Clone)]
pub struct atk_sensor_data {
    pub list: list_head,
    pub data: *mut atk_data,
    pub label_attr: device_attribute,
    pub input_attr: device_attribute,
    pub limit1_attr: device_attribute,
    pub limit2_attr: device_attribute,
    pub label_attr_name: [c_char; ATTR_NAME_SIZE],
    pub input_attr_name: [c_char; ATTR_NAME_SIZE],
    pub limit1_attr_name: [c_char; ATTR_NAME_SIZE],
    pub limit2_attr_name: [c_char; ATTR_NAME_SIZE],
    pub id: u64,
    pub type: u64,
    pub limit1: u64,
    pub limit2: u64,
    pub cached_value: u64,
    pub /: *mut *mut unsigned long last_updated; / in jiffies,
    pub is_valid: bool,
    pub acpi_name: *const c_char,
}

//
// Return buffer format:
// [0-3] "value" is valid flag
// [4-7] value
// [8- ] unknown stuff on newer mobos
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct atk_acpi_ret_buffer {
    pub flags: u32,
    pub value: u32,
    pub data: [u8; ],
}

// Input buffer used for GITM and SITM methods
#[repr(C)]
#[derive(Copy, Clone)]
pub struct atk_acpi_input_buf {
    pub id: u32,
    pub param1: u32,
    pub param2: u32,
}

    static int atk_probe(struct platform_device *pdev);
    static void atk_remove(struct platform_device *pdev);
    static void atk_print_sensor(struct atk_data *data, union acpi_object *obj);
    static int atk_read_value(struct atk_sensor_data *sensor, u64 *value);
    static struct platform_driver atk_driver = {
    .probe = atk_probe,
    .remove = atk_remove,
    .driver = {
    .name = ATK_HID,
    .acpi_match_table = atk_ids,
    },
    };

    container_of(attr, struct atk_sensor_data, input_attr)

    container_of(attr, struct atk_sensor_data, label_attr)

    container_of(attr, struct atk_sensor_data, limit1_attr)

    container_of(attr, struct atk_sensor_data, limit2_attr)
    static ssize_t atk_input_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct atk_sensor_data *s = input_to_atk_sensor(attr);
    u64 value;
    int err;
    err = atk_read_value(s, &value);
    if (err)
    return err;
    if (s.type == HWMON_TYPE_TEMP)
// ACPI returns decidegree
    value *= 100;
    return sprintf(buf, "%llu\n", value);
    }
    static ssize_t atk_label_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct atk_sensor_data *s = label_to_atk_sensor(attr);
    return sprintf(buf, "%s\n", s.acpi_name);
    }
    static ssize_t atk_limit1_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct atk_sensor_data *s = limit1_to_atk_sensor(attr);
    let mut value: u64 = s.limit1;
    if (s.type == HWMON_TYPE_TEMP)
    value *= 100;
    return sprintf(buf, "%lld\n", value);
    }
    static ssize_t atk_limit2_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct atk_sensor_data *s = limit2_to_atk_sensor(attr);
    let mut value: u64 = s.limit2;
    if (s.type == HWMON_TYPE_TEMP)
    value *= 100;
    return sprintf(buf, "%lld\n", value);
    }
    static void atk_init_attribute(struct device_attribute *attr, char *name,
    sysfs_show_func show)
    {
    sysfs_attr_init(&attr.attr);
    attr.attr.name = name;
    attr.attr.mode = 0444;
    attr.show = show;
    attr.store = core::ptr::null_mut();
    }
    static union acpi_object *atk_get_pack_member(struct atk_data *data,
    union acpi_object *pack,
    enum atk_pack_member m)
    {
    let mut old_if: bool = data.old_interface;
    int offset;
    switch (m) {
    case HWMON_PACK_FLAGS:
    offset = old_if ? _HWMON_OLD_PACK_FLAGS : _HWMON_NEW_PACK_FLAGS;
    break;
    case HWMON_PACK_NAME:
    offset = old_if ? _HWMON_OLD_PACK_NAME : _HWMON_NEW_PACK_NAME;
    break;
    case HWMON_PACK_LIMIT1:
    offset = old_if ? _HWMON_OLD_PACK_LIMIT1 :
    _HWMON_NEW_PACK_LIMIT1;
    break;
    case HWMON_PACK_LIMIT2:
    offset = old_if ? _HWMON_OLD_PACK_LIMIT2 :
    _HWMON_NEW_PACK_LIMIT2;
    break;
    case HWMON_PACK_ENABLE:
    offset = old_if ? _HWMON_OLD_PACK_ENABLE :
    _HWMON_NEW_PACK_ENABLE;
    break;
    default:
    return core::ptr::null_mut();
    }
    return &pack.package.elements[offset];
    }
//
// New package format is:
// - flag (int)
// class - used for de-muxing the request to the correct GITn
// type (volt, temp, fan)
// sensor id |
// sensor id - used for de-muxing the request _inside_ the GITn
// - name (str)
// - unknown (int)
// - limit1 (int)
// - limit2 (int)
// - enable (int)
//
// The old package has the same format but it's missing the two unknown fields.
//
#[no_mangle]
unsafe extern "C" fn validate_hwmon_pack(data: *mut atk_data, obj: *mut union acpi_object) -> c_int {
    static int validate_hwmon_pack(struct atk_data *data, union acpi_object *obj)
    {
    struct device *dev = data.dev;
    union acpi_object *tmp;
    let mut old_if: bool = data.old_interface;
    int const expected_size = old_if ? _HWMON_OLD_PACK_SIZE :
    _HWMON_NEW_PACK_SIZE;
    if (obj.type != ACPI_TYPE_PACKAGE) {
    dev_warn(dev, "Invalid type: %d\n", obj.type);
    return -EINVAL;
    }
    if (obj.package.count != expected_size) {
    dev_warn(dev, "Invalid package size: %d, expected: %d\n",
    obj.package.count, expected_size);
    return -EINVAL;
    }
    tmp = atk_get_pack_member(data, obj, HWMON_PACK_FLAGS);
    if (tmp.type != ACPI_TYPE_INTEGER) {
    dev_warn(dev, "Invalid type (flag): %d\n", tmp.type);
    return -EINVAL;
    }
    tmp = atk_get_pack_member(data, obj, HWMON_PACK_NAME);
    if (tmp.type != ACPI_TYPE_STRING) {
    dev_warn(dev, "Invalid type (name): %d\n", tmp.type);
    return -EINVAL;
    }
// Don't check... we don't know what they're useful for anyway

    tmp = &obj.package.elements[HWMON_PACK_UNK1];
    if (tmp.type != ACPI_TYPE_INTEGER) {
    dev_warn(dev, "Invalid type (unk1): %d\n", tmp.type);
    return -EINVAL;
    }
    tmp = &obj.package.elements[HWMON_PACK_UNK2];
    if (tmp.type != ACPI_TYPE_INTEGER) {
    dev_warn(dev, "Invalid type (unk2): %d\n", tmp.type);
    return -EINVAL;
    }

    tmp = atk_get_pack_member(data, obj, HWMON_PACK_LIMIT1);
    if (tmp.type != ACPI_TYPE_INTEGER) {
    dev_warn(dev, "Invalid type (limit1): %d\n", tmp.type);
    return -EINVAL;
    }
    tmp = atk_get_pack_member(data, obj, HWMON_PACK_LIMIT2);
    if (tmp.type != ACPI_TYPE_INTEGER) {
    dev_warn(dev, "Invalid type (limit2): %d\n", tmp.type);
    return -EINVAL;
    }
    tmp = atk_get_pack_member(data, obj, HWMON_PACK_ENABLE);
    if (tmp.type != ACPI_TYPE_INTEGER) {
    dev_warn(dev, "Invalid type (enable): %d\n", tmp.type);
    return -EINVAL;
    }
    atk_print_sensor(data, obj);
    return 0;
    }

    static char const *atk_sensor_type(union acpi_object *flags)
    {
    let mut type: u64 = flags.integer.value & ATK_TYPE_MASK;
    char const *what;
    switch (type) {
    case HWMON_TYPE_VOLT:
    what = "voltage";
    break;
    case HWMON_TYPE_TEMP:
    what = "temperature";
    break;
    case HWMON_TYPE_FAN:
    what = "fan";
    break;
    default:
    what = "unknown";
    break;
    }
    return what;
    }

#[no_mangle]
unsafe extern "C" fn atk_print_sensor(data: *mut atk_data, obj: *mut union acpi_object) {
    static void atk_print_sensor(struct atk_data *data, union acpi_object *obj)
    {

    struct device *dev = data.dev;
    union acpi_object *flags;
    union acpi_object *name;
    union acpi_object *limit1;
    union acpi_object *limit2;
    union acpi_object *enable;
    char const *what;
    flags = atk_get_pack_member(data, obj, HWMON_PACK_FLAGS);
    name = atk_get_pack_member(data, obj, HWMON_PACK_NAME);
    limit1 = atk_get_pack_member(data, obj, HWMON_PACK_LIMIT1);
    limit2 = atk_get_pack_member(data, obj, HWMON_PACK_LIMIT2);
    enable = atk_get_pack_member(data, obj, HWMON_PACK_ENABLE);
    what = atk_sensor_type(flags);
    dev_dbg(dev, "%s: %#llx %s [%llu-%llu] %s\n", what,
    flags.integer.value,
    name.string.pointer,
    limit1.integer.value, limit2.integer.value,
    str_enabled_disabled(enable.integer.value));

    }
#[no_mangle]
unsafe extern "C" fn atk_read_value_old(sensor: *mut atk_sensor_data, value: *mut u64) -> c_int {
    static int atk_read_value_old(struct atk_sensor_data *sensor, u64 *value)
    {
    struct atk_data *data = sensor.data;
    struct device *dev = data.dev;
    struct acpi_object_list params;
    union acpi_object id;
    acpi_status status;
    acpi_handle method;
    switch (sensor.type) {
    case HWMON_TYPE_VOLT:
    method = data.rvlt_handle;
    break;
    case HWMON_TYPE_TEMP:
    method = data.rtmp_handle;
    break;
    case HWMON_TYPE_FAN:
    method = data.rfan_handle;
    break;
    default:
    return -EINVAL;
    }
    id.type = ACPI_TYPE_INTEGER;
    id.integer.value = sensor.id;
    params.count = 1;
    params.pointer = &id;
    status = acpi_evaluate_integer(method, core::ptr::null_mut(), &params, value);
    if (status != AE_OK) {
    dev_warn(dev, "%s: ACPI exception: %s\n", __func__,
    acpi_format_exception(status));
    return -EIO;
    }
    return 0;
    }
    static union acpi_object *atk_ggrp(struct atk_data *data, u16 mux)
    {
    struct device *dev = data.dev;
    struct acpi_buffer buf;
    acpi_status ret;
    struct acpi_object_list params;
    union acpi_object id;
    union acpi_object *pack;
    id.type = ACPI_TYPE_INTEGER;
    id.integer.value = mux;
    params.count = 1;
    params.pointer = &id;
    buf.length = ACPI_ALLOCATE_BUFFER;
    ret = acpi_evaluate_object(data.enumerate_handle, core::ptr::null_mut(), &params, &buf);
    if (ret != AE_OK) {
    dev_err(dev, "GGRP[%#x] ACPI exception: %s\n", mux,
    acpi_format_exception(ret));
    return ERR_PTR(-EIO);
    }
    pack = buf.pointer;
    if (pack.type != ACPI_TYPE_PACKAGE) {
// Execution was successful, but the id was not found
    ACPI_FREE(pack);
    return ERR_PTR(-ENOENT);
    }
    if (pack.package.count < 1) {
    dev_err(dev, "GGRP[%#x] package is too small\n", mux);
    ACPI_FREE(pack);
    return ERR_PTR(-EIO);
    }
    return pack;
    }
    static union acpi_object *atk_gitm(struct atk_data *data, u64 id)
    {
    struct device *dev = data.dev;
    struct atk_acpi_input_buf buf;
    union acpi_object tmp;
    struct acpi_object_list params;
    struct acpi_buffer ret;
    union acpi_object *obj;
    acpi_status status;
    buf.id = id;
    buf.param1 = 0;
    buf.param2 = 0;
    tmp.type = ACPI_TYPE_BUFFER;
    tmp.buffer.pointer = (u8 *)&buf;
    tmp.buffer.length = sizeof(buf);
    params.count = 1;
    params.pointer = (void *)&tmp;
    ret.length = ACPI_ALLOCATE_BUFFER;
    status = acpi_evaluate_object_typed(data.read_handle, core::ptr::null_mut(), &params,
    &ret, ACPI_TYPE_BUFFER);
    if (status != AE_OK) {
    dev_warn(dev, "GITM[%#llx] ACPI exception: %s\n", id,
    acpi_format_exception(status));
    return ERR_PTR(-EIO);
    }
    obj = ret.pointer;
// Sanity check
    if (obj.buffer.length < 8) {
    dev_warn(dev, "Unexpected ASBF length: %u\n",
    obj.buffer.length);
    ACPI_FREE(obj);
    return ERR_PTR(-EIO);
    }
    return obj;
    }
    static union acpi_object *atk_sitm(struct atk_data *data,
    struct atk_acpi_input_buf *buf)
    {
    struct device *dev = data.dev;
    struct acpi_object_list params;
    union acpi_object tmp;
    struct acpi_buffer ret;
    union acpi_object *obj;
    acpi_status status;
    tmp.type = ACPI_TYPE_BUFFER;
    tmp.buffer.pointer = (u8 *)buf;
    tmp.buffer.length = sizeof(*buf);
    params.count = 1;
    params.pointer = &tmp;
    ret.length = ACPI_ALLOCATE_BUFFER;
    status = acpi_evaluate_object_typed(data.write_handle, core::ptr::null_mut(), &params,
    &ret, ACPI_TYPE_BUFFER);
    if (status != AE_OK) {
    dev_warn(dev, "SITM[%#x] ACPI exception: %s\n", buf.id,
    acpi_format_exception(status));
    return ERR_PTR(-EIO);
    }
    obj = ret.pointer;
// Sanity check
    if (obj.buffer.length < 8) {
    dev_warn(dev, "Unexpected ASBF length: %u\n",
    obj.buffer.length);
    ACPI_FREE(obj);
    return ERR_PTR(-EIO);
    }
    return obj;
    }
#[no_mangle]
unsafe extern "C" fn atk_read_value_new(sensor: *mut atk_sensor_data, value: *mut u64) -> c_int {
    static int atk_read_value_new(struct atk_sensor_data *sensor, u64 *value)
    {
    struct atk_data *data = sensor.data;
    struct device *dev = data.dev;
    union acpi_object *obj;
    struct atk_acpi_ret_buffer *buf;
    let mut err: c_int = 0;
    obj = atk_gitm(data, sensor.id);
    if (IS_ERR(obj))
    return PTR_ERR(obj);
    buf = (struct atk_acpi_ret_buffer *)obj.buffer.pointer;
    if (buf.flags == 0) {
//
// The reading is not valid, possible causes:
// - sensor failure
// - enumeration was FUBAR (and we didn't notice)
//
    dev_warn(dev, "Read failed, sensor = %#llx\n", sensor.id);
    err = -EIO;
    goto out;
    }
// value = buf->value;
    out:
    ACPI_FREE(obj);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn atk_read_value(sensor: *mut atk_sensor_data, value: *mut u64) -> c_int {
    static int atk_read_value(struct atk_sensor_data *sensor, u64 *value)
    {
    int err;
    if (!sensor.is_valid ||
    time_after(jiffies, sensor.last_updated + CACHE_TIME)) {
    if (sensor.data.old_interface)
    err = atk_read_value_old(sensor, value);
    else
    err = atk_read_value_new(sensor, value);
    if (err)
    return err;
    sensor.is_valid = true;
    sensor.last_updated = jiffies;
    sensor.cached_value = *value;
    } else {
// value = sensor->cached_value;
    err = 0;
    }
    return err;
    }

#[no_mangle]
unsafe extern "C" fn atk_debugfs_gitm_get(p: *mut c_void, val: *mut u64) -> c_int {
    static int atk_debugfs_gitm_get(void *p, u64 *val)
    {
    struct atk_data *data = p;
    union acpi_object *ret;
    struct atk_acpi_ret_buffer *buf;
    let mut err: c_int = 0;
    if (!data.read_handle)
    return -ENODEV;
    if (!data.debugfs.id)
    return -EINVAL;
    ret = atk_gitm(data, data.debugfs.id);
    if (IS_ERR(ret))
    return PTR_ERR(ret);
    buf = (struct atk_acpi_ret_buffer *)ret.buffer.pointer;
    if (buf.flags)
// val = buf->value;
    else
    err = -EIO;
    ACPI_FREE(ret);
    return err;
    }
    DEFINE_DEBUGFS_ATTRIBUTE(atk_debugfs_gitm, atk_debugfs_gitm_get, core::ptr::null_mut(),
    "0x%08llx\n");
#[no_mangle]
unsafe extern "C" fn atk_acpi_print(buf: *mut c_char, sz: usize, obj: *mut union acpi_object) -> c_int {
    static int atk_acpi_print(char *buf, size_t sz, union acpi_object *obj)
    {
    let mut ret: c_int = 0;
    switch (obj.type) {
    case ACPI_TYPE_INTEGER:
    ret = snprintf(buf, sz, "0x%08llx\n", obj.integer.value);
    break;
    case ACPI_TYPE_STRING:
    ret = snprintf(buf, sz, "%s\n", obj.string.pointer);
    break;
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn atk_pack_print(buf: *mut c_char, sz: usize, pack: *mut union acpi_object) {
    static void atk_pack_print(char *buf, size_t sz, union acpi_object *pack)
    {
    int ret;
    int i;
    for (i = 0; i < pack.package.count; i++) {
    union acpi_object *obj = &pack.package.elements[i];
    ret = atk_acpi_print(buf, sz, obj);
    if (ret >= sz)
    break;
    buf += ret;
    sz -= ret;
    }
    }
#[no_mangle]
unsafe extern "C" fn atk_debugfs_ggrp_open(inode: *mut inode, file: *mut file) -> c_int {
    static int atk_debugfs_ggrp_open(struct inode *inode, struct file *file)
    {
    struct atk_data *data = inode.i_private;
    char *buf = core::ptr::null_mut();
    union acpi_object *ret;
    u8 cls;
    int i;
    if (!data.enumerate_handle)
    return -ENODEV;
    if (!data.debugfs.id)
    return -EINVAL;
    cls = (data.debugfs.id & 0xff000000) >> 24;
    ret = atk_ggrp(data, cls);
    if (IS_ERR(ret))
    return PTR_ERR(ret);
    for (i = 0; i < ret.package.count; i++) {
    union acpi_object *pack = &ret.package.elements[i];
    union acpi_object *id;
    if (pack.type != ACPI_TYPE_PACKAGE)
    continue;
    if (!pack.package.count)
    continue;
    id = &pack.package.elements[0];
    if (id.integer.value == data.debugfs.id) {
// Print the package
    buf = kzalloc(512, GFP_KERNEL);
    if (!buf) {
    ACPI_FREE(ret);
    return -ENOMEM;
    }
    atk_pack_print(buf, 512, pack);
    break;
    }
    }
    ACPI_FREE(ret);
    if (!buf)
    return -EINVAL;
    file.private_data = buf;
    return nonseekable_open(inode, file);
    }
    static ssize_t atk_debugfs_ggrp_read(struct file *file, char __user *buf,
    size_t count, loff_t *pos)
    {
    char *str = file.private_data;
    let mut len: usize = strlen(str);
    return simple_read_from_buffer(buf, count, pos, str, len);
    }
#[no_mangle]
unsafe extern "C" fn atk_debugfs_ggrp_release(inode: *mut inode, file: *mut file) -> c_int {
    static int atk_debugfs_ggrp_release(struct inode *inode, struct file *file)
    {
    kfree(file.private_data);
    return 0;
    }
    static const struct file_operations atk_debugfs_ggrp_fops = {
    .read		= atk_debugfs_ggrp_read,
    .open		= atk_debugfs_ggrp_open,
    .release	= atk_debugfs_ggrp_release,
    };
#[no_mangle]
unsafe extern "C" fn atk_debugfs_init(data: *mut atk_data) {
    static void atk_debugfs_init(struct atk_data *data)
    {
    struct dentry *d;
    data.debugfs.id = 0;
    d = debugfs_create_dir("asus_atk0110", core::ptr::null_mut());
    debugfs_create_x32("id", 0600, d, &data.debugfs.id);
    debugfs_create_file_unsafe("gitm", 0400, d, data, &atk_debugfs_gitm);
    debugfs_create_file("ggrp", 0400, d, data, &atk_debugfs_ggrp_fops);
    data.debugfs.root = d;
    }
#[no_mangle]
unsafe extern "C" fn atk_debugfs_cleanup(data: *mut atk_data) {
    static void atk_debugfs_cleanup(struct atk_data *data)
    {
    debugfs_remove_recursive(data.debugfs.root);
    }

#[no_mangle]
unsafe extern "C" fn atk_debugfs_init(data: *mut atk_data) {
    static void atk_debugfs_init(struct atk_data *data)
    {
    }
#[no_mangle]
unsafe extern "C" fn atk_debugfs_cleanup(data: *mut atk_data) {
    static void atk_debugfs_cleanup(struct atk_data *data)
    {
    }

#[no_mangle]
unsafe extern "C" fn atk_add_sensor(data: *mut atk_data, obj: *mut union acpi_object) -> c_int {
    static int atk_add_sensor(struct atk_data *data, union acpi_object *obj)
    {
    struct device *dev = data.dev;
    union acpi_object *flags;
    union acpi_object *name;
    union acpi_object *limit1;
    union acpi_object *limit2;
    union acpi_object *enable;
    struct atk_sensor_data *sensor;
    char const *base_name;
    char const *limit1_name;
    char const *limit2_name;
    u64 type;
    int err;
    int *num;
    int start;
    if (obj.type != ACPI_TYPE_PACKAGE) {
// wft is this?
    dev_warn(dev, "Unknown type for ACPI object: (%d)\n",
    obj.type);
    return -EINVAL;
    }
    err = validate_hwmon_pack(data, obj);
    if (err)
    return err;
// Ok, we have a valid hwmon package
    type = atk_get_pack_member(data, obj, HWMON_PACK_FLAGS).integer.value
    & ATK_TYPE_MASK;
    switch (type) {
    case HWMON_TYPE_VOLT:
    base_name = "in";
    limit1_name = "min";
    limit2_name = "max";
    num = &data.voltage_count;
    start = 0;
    break;
    case HWMON_TYPE_TEMP:
    base_name = "temp";
    limit1_name = "max";
    limit2_name = "crit";
    num = &data.temperature_count;
    start = 1;
    break;
    case HWMON_TYPE_FAN:
    base_name = "fan";
    limit1_name = "min";
    limit2_name = "max";
    num = &data.fan_count;
    start = 1;
    break;
    default:
    dev_warn(dev, "Unknown sensor type: %#llx\n", type);
    return -EINVAL;
    }
    enable = atk_get_pack_member(data, obj, HWMON_PACK_ENABLE);
    if (!enable.integer.value)
// sensor is disabled
    return 0;
    flags = atk_get_pack_member(data, obj, HWMON_PACK_FLAGS);
    name = atk_get_pack_member(data, obj, HWMON_PACK_NAME);
    limit1 = atk_get_pack_member(data, obj, HWMON_PACK_LIMIT1);
    limit2 = atk_get_pack_member(data, obj, HWMON_PACK_LIMIT2);
    sensor = devm_kzalloc(dev, sizeof(*sensor), GFP_KERNEL);
    if (!sensor)
    return -ENOMEM;
    sensor.acpi_name = devm_kstrdup(dev, name.string.pointer, GFP_KERNEL);
    if (!sensor.acpi_name)
    return -ENOMEM;
    INIT_LIST_HEAD(&sensor.list);
    sensor.type = type;
    sensor.data = data;
    sensor.id = flags.integer.value;
    sensor.limit1 = limit1.integer.value;
    if (data.old_interface)
    sensor.limit2 = limit2.integer.value;
    else
// The upper limit is expressed as delta from lower limit
    sensor.limit2 = sensor.limit1 + limit2.integer.value;
    snprintf(sensor.input_attr_name, ATTR_NAME_SIZE,
    "%s%d_input", base_name, start + *num);
    atk_init_attribute(&sensor.input_attr,
    sensor.input_attr_name,
    atk_input_show);
    snprintf(sensor.label_attr_name, ATTR_NAME_SIZE,
    "%s%d_label", base_name, start + *num);
    atk_init_attribute(&sensor.label_attr,
    sensor.label_attr_name,
    atk_label_show);
    snprintf(sensor.limit1_attr_name, ATTR_NAME_SIZE,
    "%s%d_%s", base_name, start + *num, limit1_name);
    atk_init_attribute(&sensor.limit1_attr,
    sensor.limit1_attr_name,
    atk_limit1_show);
    snprintf(sensor.limit2_attr_name, ATTR_NAME_SIZE,
    "%s%d_%s", base_name, start + *num, limit2_name);
    atk_init_attribute(&sensor.limit2_attr,
    sensor.limit2_attr_name,
    atk_limit2_show);
    list_add(&sensor.list, &data.sensor_list);
    (*num)++;
    return 1;
    }
#[no_mangle]
unsafe extern "C" fn atk_enumerate_old_hwmon(data: *mut atk_data) -> c_int {
    static int atk_enumerate_old_hwmon(struct atk_data *data)
    {
    struct device *dev = data.dev;
    struct acpi_buffer buf;
    union acpi_object *pack;
    acpi_status status;
    int i, ret;
    let mut count: c_int = 0;
// Voltages
    buf.length = ACPI_ALLOCATE_BUFFER;
    status = acpi_evaluate_object_typed(data.atk_handle,
    METHOD_OLD_ENUM_VLT, core::ptr::null_mut(), &buf, ACPI_TYPE_PACKAGE);
    if (status != AE_OK) {
    dev_warn(dev, METHOD_OLD_ENUM_VLT ": ACPI exception: %s\n",
    acpi_format_exception(status));
    return -ENODEV;
    }
    pack = buf.pointer;
    for (i = 1; i < pack.package.count; i++) {
    union acpi_object *obj = &pack.package.elements[i];
    ret = atk_add_sensor(data, obj);
    if (ret > 0)
    count++;
    }
    ACPI_FREE(buf.pointer);
// Temperatures
    buf.length = ACPI_ALLOCATE_BUFFER;
    status = acpi_evaluate_object_typed(data.atk_handle,
    METHOD_OLD_ENUM_TMP, core::ptr::null_mut(), &buf, ACPI_TYPE_PACKAGE);
    if (status != AE_OK) {
    dev_warn(dev, METHOD_OLD_ENUM_TMP ": ACPI exception: %s\n",
    acpi_format_exception(status));
    return -ENODEV;
    }
    pack = buf.pointer;
    for (i = 1; i < pack.package.count; i++) {
    union acpi_object *obj = &pack.package.elements[i];
    ret = atk_add_sensor(data, obj);
    if (ret > 0)
    count++;
    }
    ACPI_FREE(buf.pointer);
// Fans
    buf.length = ACPI_ALLOCATE_BUFFER;
    status = acpi_evaluate_object_typed(data.atk_handle,
    METHOD_OLD_ENUM_FAN, core::ptr::null_mut(), &buf, ACPI_TYPE_PACKAGE);
    if (status != AE_OK) {
    dev_warn(dev, METHOD_OLD_ENUM_FAN ": ACPI exception: %s\n",
    acpi_format_exception(status));
    return -ENODEV;
    }
    pack = buf.pointer;
    for (i = 1; i < pack.package.count; i++) {
    union acpi_object *obj = &pack.package.elements[i];
    ret = atk_add_sensor(data, obj);
    if (ret > 0)
    count++;
    }
    ACPI_FREE(buf.pointer);
    return count;
    }
#[no_mangle]
unsafe extern "C" fn atk_ec_present(data: *mut atk_data) -> c_int {
    static int atk_ec_present(struct atk_data *data)
    {
    struct device *dev = data.dev;
    union acpi_object *pack;
    union acpi_object *ec;
    int ret;
    int i;
    pack = atk_ggrp(data, ATK_MUX_MGMT);
    if (IS_ERR(pack)) {
    if (PTR_ERR(pack) == -ENOENT) {
// The MGMT class does not exists - that's ok
    dev_dbg(dev, "Class %#llx not found\n", ATK_MUX_MGMT);
    return 0;
    }
    return PTR_ERR(pack);
    }
// Search the EC
    ec = core::ptr::null_mut();
    for (i = 0; i < pack.package.count; i++) {
    union acpi_object *obj = &pack.package.elements[i];
    union acpi_object *id;
    if (obj.type != ACPI_TYPE_PACKAGE)
    continue;
    if (!obj.package.count)
    continue;
    id = &obj.package.elements[0];
    if (id.type != ACPI_TYPE_INTEGER)
    continue;
    if (id.integer.value == ATK_EC_ID) {
    ec = obj;
    break;
    }
    }
    ret = (ec != core::ptr::null_mut());
    if (!ret)
// The system has no EC
    dev_dbg(dev, "EC not found\n");
    ACPI_FREE(pack);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn atk_ec_enabled(data: *mut atk_data) -> c_int {
    static int atk_ec_enabled(struct atk_data *data)
    {
    struct device *dev = data.dev;
    union acpi_object *obj;
    struct atk_acpi_ret_buffer *buf;
    int err;
    obj = atk_gitm(data, ATK_EC_ID);
    if (IS_ERR(obj)) {
    dev_err(dev, "Unable to query EC status\n");
    return PTR_ERR(obj);
    }
    buf = (struct atk_acpi_ret_buffer *)obj.buffer.pointer;
    if (buf.flags == 0) {
    dev_err(dev, "Unable to query EC status\n");
    err = -EIO;
    } else {
    err = (buf.value != 0);
    dev_dbg(dev, "EC is %s\n", str_enabled_disabled(err));
    }
    ACPI_FREE(obj);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn atk_ec_ctl(data: *mut atk_data, enable: c_int) -> c_int {
    static int atk_ec_ctl(struct atk_data *data, int enable)
    {
    struct device *dev = data.dev;
    union acpi_object *obj;
    struct atk_acpi_input_buf sitm;
    struct atk_acpi_ret_buffer *ec_ret;
    let mut err: c_int = 0;
    sitm.id = ATK_EC_ID;
    sitm.param1 = enable;
    sitm.param2 = 0;
    obj = atk_sitm(data, &sitm);
    if (IS_ERR(obj)) {
    dev_err(dev, "Failed to %s the EC\n", str_enable_disable(enable));
    return PTR_ERR(obj);
    }
    ec_ret = (struct atk_acpi_ret_buffer *)obj.buffer.pointer;
    if (ec_ret.flags == 0) {
    dev_err(dev, "Failed to %s the EC\n", str_enable_disable(enable));
    err = -EIO;
    } else {
    dev_info(dev, "EC %s\n", str_enabled_disabled(enable));
    }
    ACPI_FREE(obj);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn atk_enumerate_new_hwmon(data: *mut atk_data) -> c_int {
    static int atk_enumerate_new_hwmon(struct atk_data *data)
    {
    struct device *dev = data.dev;
    union acpi_object *pack;
    int err;
    int i;
    err = atk_ec_present(data);
    if (err < 0)
    return err;
    if (err) {
    err = atk_ec_enabled(data);
    if (err < 0)
    return err;
// If the EC was disabled we will disable it again on unload
    data.disable_ec = err;
    err = atk_ec_ctl(data, 1);
    if (err) {
    data.disable_ec = false;
    return err;
    }
    }
    dev_dbg(dev, "Enumerating hwmon sensors\n");
    pack = atk_ggrp(data, ATK_MUX_HWMON);
    if (IS_ERR(pack))
    return PTR_ERR(pack);
    for (i = 0; i < pack.package.count; i++) {
    union acpi_object *obj = &pack.package.elements[i];
    atk_add_sensor(data, obj);
    }
    err = data.voltage_count + data.temperature_count + data.fan_count;
    ACPI_FREE(pack);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn atk_init_attribute_groups(data: *mut atk_data) -> c_int {
    static int atk_init_attribute_groups(struct atk_data *data)
    {
    struct device *dev = data.dev;
    struct atk_sensor_data *s;
    struct attribute **attrs;
    let mut i: c_int = 0;
    int len = (data.voltage_count + data.temperature_count
    + data.fan_count) * 4 + 1;
    attrs = devm_kcalloc(dev, len, sizeof(struct attribute *), GFP_KERNEL);
    if (!attrs)
    return -ENOMEM;
    list_for_each_entry(s, &data.sensor_list, list) {
    attrs[i++] = &s.input_attr.attr;
    attrs[i++] = &s.label_attr.attr;
    attrs[i++] = &s.limit1_attr.attr;
    attrs[i++] = &s.limit2_attr.attr;
    }
    data.attr_group.attrs = attrs;
    data.attr_groups[0] = &data.attr_group;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn atk_register_hwmon(data: *mut atk_data) -> c_int {
    static int atk_register_hwmon(struct atk_data *data)
    {
    struct device *dev = data.dev;
    dev_dbg(dev, "registering hwmon device\n");
    data.hwmon_dev = hwmon_device_register_with_groups(dev, "atk0110",
    data,
    data.attr_groups);
    return PTR_ERR_OR_ZERO(data.hwmon_dev);
    }
#[no_mangle]
unsafe extern "C" fn atk_probe_if(data: *mut atk_data) -> c_int {
    static int atk_probe_if(struct atk_data *data)
    {
    struct device *dev = data.dev;
    acpi_handle ret;
    acpi_status status;
    let mut err: c_int = 0;
// RTMP: read temperature
    status = acpi_get_handle(data.atk_handle, METHOD_OLD_READ_TMP, &ret);
    if (ACPI_SUCCESS(status))
    data.rtmp_handle = ret;
    else
    dev_dbg(dev, "method " METHOD_OLD_READ_TMP " not found: %s\n",
    acpi_format_exception(status));
// RVLT: read voltage
    status = acpi_get_handle(data.atk_handle, METHOD_OLD_READ_VLT, &ret);
    if (ACPI_SUCCESS(status))
    data.rvlt_handle = ret;
    else
    dev_dbg(dev, "method " METHOD_OLD_READ_VLT " not found: %s\n",
    acpi_format_exception(status));
// RFAN: read fan status
    status = acpi_get_handle(data.atk_handle, METHOD_OLD_READ_FAN, &ret);
    if (ACPI_SUCCESS(status))
    data.rfan_handle = ret;
    else
    dev_dbg(dev, "method " METHOD_OLD_READ_FAN " not found: %s\n",
    acpi_format_exception(status));
// Enumeration
    status = acpi_get_handle(data.atk_handle, METHOD_ENUMERATE, &ret);
    if (ACPI_SUCCESS(status))
    data.enumerate_handle = ret;
    else
    dev_dbg(dev, "method " METHOD_ENUMERATE " not found: %s\n",
    acpi_format_exception(status));
// De-multiplexer (read)
    status = acpi_get_handle(data.atk_handle, METHOD_READ, &ret);
    if (ACPI_SUCCESS(status))
    data.read_handle = ret;
    else
    dev_dbg(dev, "method " METHOD_READ " not found: %s\n",
    acpi_format_exception(status));
// De-multiplexer (write)
    status = acpi_get_handle(data.atk_handle, METHOD_WRITE, &ret);
    if (ACPI_SUCCESS(status))
    data.write_handle = ret;
    else
    dev_dbg(dev, "method " METHOD_WRITE " not found: %s\n",
    acpi_format_exception(status));
//
// Check for hwmon methods: first check "old" style methods; note that
// both may be present: in this case we stick to the old interface;
// analysis of multiple DSDTs indicates that when both interfaces
// are present the new one (GGRP/GITM) is not functional.
//
    if (new_if)
    dev_info(dev, "Overriding interface detection\n");
    if (data.rtmp_handle &&
    data.rvlt_handle && data.rfan_handle && !new_if)
    data.old_interface = true;
    else if (data.enumerate_handle && data.read_handle &&
    data.write_handle)
    data.old_interface = false;
    else
    err = -ENODEV;
    return err;
    }
#[no_mangle]
unsafe extern "C" fn atk_probe(pdev: *mut platform_device) -> c_int {
    static int atk_probe(struct platform_device *pdev)
    {
    acpi_status ret;
    int err;
    struct acpi_buffer buf;
    union acpi_object *obj;
    struct atk_data *data;
    acpi_handle handle;
    dev_dbg(&pdev.dev, "adding...\n");
    handle = ACPI_HANDLE(&pdev.dev);
    if (!handle)
    return -ENODEV;
    data = devm_kzalloc(&pdev.dev, sizeof(*data), GFP_KERNEL);
    if (!data)
    return -ENOMEM;
    data.dev = &pdev.dev;
    data.atk_handle = handle;
    INIT_LIST_HEAD(&data.sensor_list);
    data.disable_ec = false;
    buf.length = ACPI_ALLOCATE_BUFFER;
    ret = acpi_evaluate_object_typed(data.atk_handle, BOARD_ID, core::ptr::null_mut(),
    &buf, ACPI_TYPE_PACKAGE);
    if (ret != AE_OK) {
    dev_dbg(&pdev.dev, "atk: method MBIF not found\n");
    } else {
    obj = buf.pointer;
    if (obj.package.count >= 2) {
    union acpi_object *id = &obj.package.elements[1];
    if (id.type == ACPI_TYPE_STRING)
    dev_dbg(&pdev.dev, "board ID = %s\n",
    id.string.pointer);
    }
    ACPI_FREE(buf.pointer);
    }
    err = atk_probe_if(data);
    if (err) {
    dev_err(&pdev.dev, "No usable hwmon interface detected\n");
    goto out;
    }
    if (data.old_interface) {
    dev_dbg(&pdev.dev, "Using old hwmon interface\n");
    err = atk_enumerate_old_hwmon(data);
    } else {
    dev_dbg(&pdev.dev, "Using new hwmon interface\n");
    err = atk_enumerate_new_hwmon(data);
    }
    if (err < 0)
    goto out;
    if (err == 0) {
    dev_info(&pdev.dev,
    "No usable sensor detected, bailing out\n");
    err = -ENODEV;
    goto out;
    }
    err = atk_init_attribute_groups(data);
    if (err)
    goto out;
    err = atk_register_hwmon(data);
    if (err)
    goto out;
    atk_debugfs_init(data);
    platform_set_drvdata(pdev, data);
    return 0;
    out:
    if (data.disable_ec)
    atk_ec_ctl(data, 0);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn atk_remove(pdev: *mut platform_device) {
    static void atk_remove(struct platform_device *pdev)
    {
    struct atk_data *data = platform_get_drvdata(pdev);
    dev_dbg(&pdev.dev, "removing...\n");
    atk_debugfs_cleanup(data);
    hwmon_device_unregister(data.hwmon_dev);
    if (data.disable_ec) {
    if (atk_ec_ctl(data, 0))
    dev_err(&pdev.dev, "Failed to disable EC\n");
    }
    }
#[no_mangle]
unsafe extern "C" fn atk0110_init() -> int __init {
    static int __init atk0110_init(void)
    {
    int ret;
// Make sure it's safe to access the device through ACPI
    if (!acpi_resources_are_enforced()) {
    pr_err("Resources not safely usable due to acpi_enforce_resources kernel parameter\n");
    return -EBUSY;
    }
    if (dmi_check_system(atk_force_new_if))
    new_if = true;
    ret = platform_driver_register(&atk_driver);
    if (ret)
    pr_info("platform_driver_register failed: %d\n", ret);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn atk0110_exit() -> void __exit {
    static void __exit atk0110_exit(void)
    {
    platform_driver_unregister(&atk_driver);
    }
    module_init(atk0110_init);
    module_exit(atk0110_exit);
    MODULE_DESCRIPTION("ASUS ATK0110 driver");
    MODULE_LICENSE("GPL");
