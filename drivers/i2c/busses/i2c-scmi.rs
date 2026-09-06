//! Automatically rewritten from C to Rust
//! Source: drivers/i2c/busses/i2c-scmi.c
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
// SMBus driver for ACPI SMBus CMI
//
// Copyright (C) 2009 Crane Cai <crane.cai@amd.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smbus_methods_t {
    pub mt_info: *mut c_char,
    pub mt_sbr: *mut c_char,
    pub mt_sbw: *mut c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_smbus_cmi {
    pub handle: acpi_handle,
    pub adapter: i2c_adapter,
    pub cap_info:1: u8,
    pub cap_read:1: u8,
    pub cap_write:1: u8,
    pub methods: *const smbus_methods_t,
}

    static const struct smbus_methods_t smbus_methods = {
    .mt_info = "_SBI",
    .mt_sbr  = "_SBR",
    .mt_sbw  = "_SBW",
    };
// Some IBM BIOSes omit the leading underscore
    static const struct smbus_methods_t ibm_smbus_methods = {
    .mt_info = "SBI_",
    .mt_sbr  = "SBR_",
    .mt_sbw  = "SBW_",
    };
    static const struct acpi_device_id acpi_smbus_cmi_ids[] = {
    {"SMBUS01", (kernel_ulong_t)&smbus_methods},
    {ACPI_SMBUS_IBM_HID, (kernel_ulong_t)&ibm_smbus_methods},
    {ACPI_SMBUS_MS_HID, (kernel_ulong_t)&smbus_methods},
    {"", 0}
    };
    MODULE_DEVICE_TABLE(acpi, acpi_smbus_cmi_ids);
pub const ACPI_SMBUS_STATUS_OK: c_uint = 0x00;
pub const ACPI_SMBUS_STATUS_FAIL: c_uint = 0x07;
pub const ACPI_SMBUS_STATUS_DNAK: c_uint = 0x10;
pub const ACPI_SMBUS_STATUS_DERR: c_uint = 0x11;
pub const ACPI_SMBUS_STATUS_CMD_DENY: c_uint = 0x12;
pub const ACPI_SMBUS_STATUS_UNKNOWN: c_uint = 0x13;
pub const ACPI_SMBUS_STATUS_ACC_DENY: c_uint = 0x17;
pub const ACPI_SMBUS_STATUS_TIMEOUT: c_uint = 0x18;
pub const ACPI_SMBUS_STATUS_NOTSUP: c_uint = 0x19;
pub const ACPI_SMBUS_STATUS_BUSY: c_uint = 0x1a;
pub const ACPI_SMBUS_STATUS_PEC: c_uint = 0x1f;
pub const ACPI_SMBUS_PRTCL_WRITE: c_uint = 0x00;
pub const ACPI_SMBUS_PRTCL_READ: c_uint = 0x01;
pub const ACPI_SMBUS_PRTCL_QUICK: c_uint = 0x02;
pub const ACPI_SMBUS_PRTCL_BYTE: c_uint = 0x04;
pub const ACPI_SMBUS_PRTCL_BYTE_DATA: c_uint = 0x06;
pub const ACPI_SMBUS_PRTCL_WORD_DATA: c_uint = 0x08;
pub const ACPI_SMBUS_PRTCL_BLOCK_DATA: c_uint = 0x0a;
    static int
    acpi_smbus_cmi_access(struct i2c_adapter *adap, u16 addr, unsigned short flags,
    char read_write, u8 command, int size,
    union i2c_smbus_data *data)
    {
    let mut result: c_int = 0;
    struct acpi_smbus_cmi *smbus_cmi = adap.algo_data;
    unsigned char protocol;
    let mut status: acpi_status = 0;
    struct acpi_object_list input;
    union acpi_object mt_params[5];
    let mut buffer: acpi_buffer = { ACPI_ALLOCATE_BUFFER, core::ptr::null_mut() };
    union acpi_object *obj;
    union acpi_object *pkg;
    char *method;
    let mut len: c_int = 0;
    dev_dbg(&adap.dev, "access size: %d %s\n", size,
    (read_write) ? "READ" : "WRITE");
    switch (size) {
    case I2C_SMBUS_QUICK:
    protocol = ACPI_SMBUS_PRTCL_QUICK;
    command = 0;
    if (read_write == I2C_SMBUS_WRITE) {
    mt_params[3].type = ACPI_TYPE_INTEGER;
    mt_params[3].integer.value = 0;
    mt_params[4].type = ACPI_TYPE_INTEGER;
    mt_params[4].integer.value = 0;
    }
    break;
    case I2C_SMBUS_BYTE:
    protocol = ACPI_SMBUS_PRTCL_BYTE;
    if (read_write == I2C_SMBUS_WRITE) {
    mt_params[3].type = ACPI_TYPE_INTEGER;
    mt_params[3].integer.value = 0;
    mt_params[4].type = ACPI_TYPE_INTEGER;
    mt_params[4].integer.value = 0;
    } else {
    command = 0;
    }
    break;
    case I2C_SMBUS_BYTE_DATA:
    protocol = ACPI_SMBUS_PRTCL_BYTE_DATA;
    if (read_write == I2C_SMBUS_WRITE) {
    mt_params[3].type = ACPI_TYPE_INTEGER;
    mt_params[3].integer.value = 1;
    mt_params[4].type = ACPI_TYPE_INTEGER;
    mt_params[4].integer.value = data.byte;
    }
    break;
    case I2C_SMBUS_WORD_DATA:
    protocol = ACPI_SMBUS_PRTCL_WORD_DATA;
    if (read_write == I2C_SMBUS_WRITE) {
    mt_params[3].type = ACPI_TYPE_INTEGER;
    mt_params[3].integer.value = 2;
    mt_params[4].type = ACPI_TYPE_INTEGER;
    mt_params[4].integer.value = data.word;
    }
    break;
    case I2C_SMBUS_BLOCK_DATA:
    protocol = ACPI_SMBUS_PRTCL_BLOCK_DATA;
    if (read_write == I2C_SMBUS_WRITE) {
    len = data.block[0];
    if (len == 0 || len > I2C_SMBUS_BLOCK_MAX)
    return -EINVAL;
    mt_params[3].type = ACPI_TYPE_INTEGER;
    mt_params[3].integer.value = len;
    mt_params[4].type = ACPI_TYPE_BUFFER;
    mt_params[4].buffer.length = len;
    mt_params[4].buffer.pointer = data.block + 1;
    }
    break;
    default:
    dev_warn(&adap.dev, "Unsupported transaction %d\n", size);
    return -EOPNOTSUPP;
    }
    if (read_write == I2C_SMBUS_READ) {
    protocol |= ACPI_SMBUS_PRTCL_READ;
    method = smbus_cmi.methods.mt_sbr;
    input.count = 3;
    } else {
    protocol |= ACPI_SMBUS_PRTCL_WRITE;
    method = smbus_cmi.methods.mt_sbw;
    input.count = 5;
    }
    input.pointer = mt_params;
    mt_params[0].type = ACPI_TYPE_INTEGER;
    mt_params[0].integer.value = protocol;
    mt_params[1].type = ACPI_TYPE_INTEGER;
    mt_params[1].integer.value = addr;
    mt_params[2].type = ACPI_TYPE_INTEGER;
    mt_params[2].integer.value = command;
    status = acpi_evaluate_object(smbus_cmi.handle, method, &input,
    &buffer);
    if (ACPI_FAILURE(status)) {
    acpi_handle_err(smbus_cmi.handle,
    "Failed to evaluate %s: %i\n", method, status);
    return -EIO;
    }
    pkg = buffer.pointer;
    if (pkg && pkg.type == ACPI_TYPE_PACKAGE)
    obj = pkg.package.elements;
    else {
    acpi_handle_err(smbus_cmi.handle, "Invalid argument type\n");
    result = -EIO;
    goto out;
    }
    if (obj == core::ptr::null_mut() || obj.type != ACPI_TYPE_INTEGER) {
    acpi_handle_err(smbus_cmi.handle, "Invalid argument type\n");
    result = -EIO;
    goto out;
    }
    result = obj.integer.value;
    acpi_handle_debug(smbus_cmi.handle,  "%s return status: %i\n", method,
    result);
    switch (result) {
    case ACPI_SMBUS_STATUS_OK:
    result = 0;
    break;
    case ACPI_SMBUS_STATUS_BUSY:
    result = -EBUSY;
    goto out;
    case ACPI_SMBUS_STATUS_TIMEOUT:
    result = -ETIMEDOUT;
    goto out;
    case ACPI_SMBUS_STATUS_DNAK:
    result = -ENXIO;
    goto out;
    default:
    result = -EIO;
    goto out;
    }
    if (read_write == I2C_SMBUS_WRITE || size == I2C_SMBUS_QUICK)
    goto out;
    obj = pkg.package.elements + 1;
    if (obj.type != ACPI_TYPE_INTEGER) {
    acpi_handle_err(smbus_cmi.handle, "Invalid argument type\n");
    result = -EIO;
    goto out;
    }
    len = obj.integer.value;
    obj = pkg.package.elements + 2;
    switch (size) {
    case I2C_SMBUS_BYTE:
    case I2C_SMBUS_BYTE_DATA:
    case I2C_SMBUS_WORD_DATA:
    if (obj.type != ACPI_TYPE_INTEGER) {
    acpi_handle_err(smbus_cmi.handle,
    "Invalid argument type\n");
    result = -EIO;
    goto out;
    }
    if (len == 2)
    data.word = obj.integer.value;
    else
    data.byte = obj.integer.value;
    break;
    case I2C_SMBUS_BLOCK_DATA:
    if (obj.type != ACPI_TYPE_BUFFER) {
    acpi_handle_err(smbus_cmi.handle,
    "Invalid argument type\n");
    result = -EIO;
    goto out;
    }
    if (len == 0 || len > I2C_SMBUS_BLOCK_MAX)
    return -EPROTO;
    data.block[0] = len;
    memcpy(data.block + 1, obj.buffer.pointer, len);
    break;
    }
    out:
    kfree(buffer.pointer);
    dev_dbg(&adap.dev, "Transaction status: %i\n", result);
    return result;
    }
#[no_mangle]
unsafe extern "C" fn acpi_smbus_cmi_func(adapter: *mut i2c_adapter) -> u32 {
    static u32 acpi_smbus_cmi_func(struct i2c_adapter *adapter)
    {
    struct acpi_smbus_cmi *smbus_cmi = adapter.algo_data;
    u32 ret;
    ret = smbus_cmi.cap_read | smbus_cmi.cap_write ?
    I2C_FUNC_SMBUS_QUICK : 0;
    ret |= smbus_cmi.cap_read ?
    (I2C_FUNC_SMBUS_READ_BYTE |
    I2C_FUNC_SMBUS_READ_BYTE_DATA |
    I2C_FUNC_SMBUS_READ_WORD_DATA |
    I2C_FUNC_SMBUS_READ_BLOCK_DATA) : 0;
    ret |= smbus_cmi.cap_write ?
    (I2C_FUNC_SMBUS_WRITE_BYTE |
    I2C_FUNC_SMBUS_WRITE_BYTE_DATA |
    I2C_FUNC_SMBUS_WRITE_WORD_DATA |
    I2C_FUNC_SMBUS_WRITE_BLOCK_DATA) : 0;
    return ret;
    }
    static const struct i2c_algorithm acpi_smbus_cmi_algorithm = {
    .smbus_xfer = acpi_smbus_cmi_access,
    .functionality = acpi_smbus_cmi_func,
    };
    static int acpi_smbus_cmi_add_cap(struct acpi_smbus_cmi *smbus_cmi,
    const char *name)
    {
    let mut buffer: acpi_buffer = { ACPI_ALLOCATE_BUFFER, core::ptr::null_mut() };
    struct acpi_handle *handle = smbus_cmi.handle;
    union acpi_object *obj;
    acpi_status status;
    if (!strcmp(name, smbus_cmi.methods.mt_info)) {
    status = acpi_evaluate_object(smbus_cmi.handle,
    smbus_cmi.methods.mt_info,
    core::ptr::null_mut(), &buffer);
    if (ACPI_FAILURE(status)) {
    acpi_handle_err(handle, "Failed to evaluate %s: %i\n",
    smbus_cmi.methods.mt_info, status);
    return -EIO;
    }
    obj = buffer.pointer;
    if (obj && obj.type == ACPI_TYPE_PACKAGE)
    obj = obj.package.elements;
    else {
    acpi_handle_err(handle, "Invalid argument type\n");
    kfree(buffer.pointer);
    return -EIO;
    }
    if (obj.type != ACPI_TYPE_INTEGER) {
    acpi_handle_err(handle, "Invalid argument type\n");
    kfree(buffer.pointer);
    return -EIO;
    } else
    acpi_handle_debug(handle, "SMBus CMI Version %x\n",
    (int)obj.integer.value);
    kfree(buffer.pointer);
    smbus_cmi.cap_info = 1;
    } else if (!strcmp(name, smbus_cmi.methods.mt_sbr))
    smbus_cmi.cap_read = 1;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strcmp(name, _arg: smbus_cmi->methods->mt_sbw)) -> else {
    else if (!strcmp(name, smbus_cmi.methods.mt_sbw))
    smbus_cmi.cap_write = 1;
    else
    acpi_handle_debug(handle, "Unsupported CMI method: %s\n", name);
    return 0;
    }
    static acpi_status acpi_smbus_cmi_query_methods(acpi_handle handle, u32 level,
    void *context, void **return_value)
    {
    char node_name[5];
    let mut buffer: acpi_buffer = { sizeof(node_name), node_name };
    struct acpi_smbus_cmi *smbus_cmi = context;
    acpi_status status;
    status = acpi_get_name(handle, ACPI_SINGLE_NAME, &buffer);
    if (ACPI_SUCCESS(status))
    acpi_smbus_cmi_add_cap(smbus_cmi, node_name);
    return AE_OK;
    }
#[no_mangle]
unsafe extern "C" fn smbus_cmi_probe(device: *mut platform_device) -> c_int {
    static int smbus_cmi_probe(struct platform_device *device)
    {
    struct device *dev = &device.dev;
    struct acpi_smbus_cmi *smbus_cmi;
    int ret;
    smbus_cmi = kzalloc_obj(struct acpi_smbus_cmi);
    if (!smbus_cmi)
    return -ENOMEM;
    smbus_cmi.handle = ACPI_HANDLE(dev);
    smbus_cmi.methods = device_get_match_data(dev);
    platform_set_drvdata(device, smbus_cmi);
    smbus_cmi.cap_info = 0;
    smbus_cmi.cap_read = 0;
    smbus_cmi.cap_write = 0;
    acpi_walk_namespace(ACPI_TYPE_METHOD, smbus_cmi.handle, 1,
    acpi_smbus_cmi_query_methods, core::ptr::null_mut(), smbus_cmi, core::ptr::null_mut());
    if (smbus_cmi.cap_info == 0) {
    ret = -ENODEV;
    goto err;
    }
    snprintf(smbus_cmi.adapter.name, sizeof(smbus_cmi.adapter.name),
    "SMBus CMI adapter %s", dev_name(dev));
    smbus_cmi.adapter.owner = THIS_MODULE;
    smbus_cmi.adapter.algo = &acpi_smbus_cmi_algorithm;
    smbus_cmi.adapter.algo_data = smbus_cmi;
    smbus_cmi.adapter.class = I2C_CLASS_HWMON;
    smbus_cmi.adapter.dev.parent = &device.dev;
    ret = i2c_add_adapter(&smbus_cmi.adapter);
    if (ret) {
    dev_err(&device.dev, "Couldn't register adapter!\n");
    goto err;
    }
    return 0;
    err:
    kfree(smbus_cmi);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn smbus_cmi_remove(device: *mut platform_device) {
    static void smbus_cmi_remove(struct platform_device *device)
    {
    struct acpi_smbus_cmi *smbus_cmi = platform_get_drvdata(device);
    i2c_del_adapter(&smbus_cmi.adapter);
    kfree(smbus_cmi);
    }
    static struct platform_driver smbus_cmi_driver = {
    .probe = smbus_cmi_probe,
    .remove = smbus_cmi_remove,
    .driver = {
    .name   = "smbus_cmi",
    .acpi_match_table = acpi_smbus_cmi_ids,
    },
    };
    module_platform_driver(smbus_cmi_driver);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Crane Cai <crane.cai@amd.com>");
    MODULE_DESCRIPTION("ACPI SMBus CMI driver");
