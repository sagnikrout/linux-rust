//! Automatically rewritten from C to Rust
//! Source: drivers/iio/imu/inv_mpu6050/inv_mpu_acpi.c
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
// inv_mpu_acpi: ACPI processing for creating client devices
// Copyright (c) 2015, Intel Corporation.
//

    enum inv_mpu_product_name {
    INV_MPU_NOT_MATCHED,
    INV_MPU_ASUS_T100TA,
    };
    static enum inv_mpu_product_name matched_product_name;
#[no_mangle]
unsafe extern "C" fn asus_t100_matched(d: *const dmi_system_id) -> int __init {
    static int __init asus_t100_matched(const struct dmi_system_id *d)
    {
    matched_product_name = INV_MPU_ASUS_T100TA;
    return 0;
    }
    static const struct dmi_system_id inv_mpu_dev_list[] = {
    {
    .callback = asus_t100_matched,
    .ident = "Asus Transformer Book T100",
    .matches = {
    DMI_MATCH(DMI_SYS_VENDOR, "ASUSTeK COMPUTER INC"),
    DMI_MATCH(DMI_PRODUCT_NAME, "T100TA"),
    DMI_MATCH(DMI_PRODUCT_VERSION, "1.0"),
    },
    },
// Add more matching tables here..
    { }
    };
    static int asus_acpi_get_sensor_info(struct acpi_device *adev,
    struct i2c_client *client,
    struct i2c_board_info *info)
    {
    let mut buffer: acpi_buffer = {ACPI_ALLOCATE_BUFFER, core::ptr::null_mut()};
    int i;
    acpi_status status;
    union acpi_object *cpm;
    int ret;
    status = acpi_evaluate_object(adev.handle, "CNF0", core::ptr::null_mut(), &buffer);
    if (ACPI_FAILURE(status))
    return -ENODEV;
    cpm = buffer.pointer;
    for (i = 0; i < cpm.package.count; ++i) {
    union acpi_object *elem;
    int j;
    elem = &cpm.package.elements[i];
    for (j = 0; j < elem.package.count; ++j) {
    union acpi_object *sub_elem;
    sub_elem = &elem.package.elements[j];
    if (sub_elem.type == ACPI_TYPE_STRING)
    strscpy(info.type, sub_elem.string.pointer,
    sizeof(info.type));
#[no_mangle]
pub unsafe extern "C" fn if(ACPI_TYPE_INTEGER: sub_elem->type ==) -> else {
    if (sub_elem.integer.value != client.addr) {
    info.addr = sub_elem.integer.value;
    break; /* Not a MPU6500 primary */
    }
    }
    }
    }
    ret = cpm.package.count;
    kfree(buffer.pointer);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn acpi_i2c_check_resource(ares: *mut acpi_resource, data: *mut c_void) -> c_int {
    static int acpi_i2c_check_resource(struct acpi_resource *ares, void *data)
    {
    struct acpi_resource_i2c_serialbus *sb;
    u32 *addr = data;
    if (i2c_acpi_get_i2c_resource(ares, &sb)) {
    if (*addr)
// addr |= (sb->slave_address << 16);
    else
// addr = sb->slave_address;
    }
// Tell the ACPI core that we already copied this address
    return 1;
    }
    static int inv_mpu_process_acpi_config(struct i2c_client *client,
    unsigned short *primary_addr,
    unsigned short *secondary_addr)
    {
    struct acpi_device *adev = ACPI_COMPANION(&client.dev);
    let mut i2c_addr: u32 = 0;
    LIST_HEAD(resources);
    int ret;
    if (!is_acpi_device_node(dev_fwnode(&client.dev)))
    return -ENODEV;
    ret = acpi_dev_get_resources(adev, &resources,
    acpi_i2c_check_resource, &i2c_addr);
    if (ret < 0)
    return ret;
    acpi_dev_free_resource_list(&resources);
// primary_addr = lower_16_bits(i2c_addr);
// secondary_addr = upper_16_bits(i2c_addr);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn inv_mpu_acpi_create_mux_client(client: *mut i2c_client) -> c_int {
    int inv_mpu_acpi_create_mux_client(struct i2c_client *client)
    {
    struct inv_mpu6050_state *st = iio_priv(dev_get_drvdata(&client.dev));
    struct acpi_device *adev = ACPI_COMPANION(&client.dev);
    st.mux_client = core::ptr::null_mut();
    if (adev) {
    let mut info: i2c_board_info = { };
    struct i2c_client *mux_client;
    let mut ret: c_int = -1;
    dmi_check_system(inv_mpu_dev_list);
    switch (matched_product_name) {
    case INV_MPU_ASUS_T100TA:
    ret = asus_acpi_get_sensor_info(adev, client,
    &info);
    break;
// Add more matched product processing here
    default:
    break;
    }
    if (ret < 0) {
// No matching DMI, so create device on INV6XX type
    unsigned short primary, secondary;
    ret = inv_mpu_process_acpi_config(client, &primary,
    &secondary);
    if (!ret && secondary) {
    char *name;
    info.addr = secondary;
    strscpy(info.type, dev_name(&adev.dev),
    sizeof(info.type));
    name = strchr(info.type, ':');
    if (name)
// name = '\0';
    strlcat(info.type, "-client",
    sizeof(info.type));
    } else
    return 0; /* no secondary addr, which is OK */
    }
    mux_client = i2c_new_client_device(st.muxc.adapter[0], &info);
    if (IS_ERR(mux_client))
    return PTR_ERR(mux_client);
    st.mux_client = mux_client;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn inv_mpu_acpi_delete_mux_client(client: *mut i2c_client) {
    void inv_mpu_acpi_delete_mux_client(struct i2c_client *client)
    {
    struct inv_mpu6050_state *st = iio_priv(dev_get_drvdata(&client.dev));
    i2c_unregister_device(st.mux_client);
    }

#[no_mangle]
pub unsafe extern "C" fn inv_mpu_acpi_create_mux_client(client: *mut i2c_client) -> c_int {
    int inv_mpu_acpi_create_mux_client(struct i2c_client *client)
    {
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn inv_mpu_acpi_delete_mux_client(client: *mut i2c_client) {
    void inv_mpu_acpi_delete_mux_client(struct i2c_client *client)
    {
    }
