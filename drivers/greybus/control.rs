//! Automatically rewritten from C to Rust
//! Source: drivers/greybus/control.c
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
// Greybus CPort control protocol.
//
// Copyright 2015 Google Inc.
// Copyright 2015 Linaro Ltd.
//

// Highest control-protocol version supported
pub const GB_CONTROL_VERSION_MAJOR: c_int = 0;
pub const GB_CONTROL_VERSION_MINOR: c_int = 1;
#[no_mangle]
unsafe extern "C" fn gb_control_get_version(control: *mut gb_control) -> c_int {
    static int gb_control_get_version(struct gb_control *control)
    {
    struct gb_interface *intf = control.connection.intf;
    struct gb_control_version_request request;
    struct gb_control_version_response response;
    int ret;
    request.major = GB_CONTROL_VERSION_MAJOR;
    request.minor = GB_CONTROL_VERSION_MINOR;
    ret = gb_operation_sync(control.connection,
    GB_CONTROL_TYPE_VERSION,
    &request, sizeof(request), &response,
    sizeof(response));
    if (ret) {
    dev_err(&intf.dev,
    "failed to get control-protocol version: %d\n",
    ret);
    return ret;
    }
    if (response.major > request.major) {
    dev_err(&intf.dev,
    "unsupported major control-protocol version (%u > %u)\n",
    response.major, request.major);
    return -ENOTSUPP;
    }
    control.protocol_major = response.major;
    control.protocol_minor = response.minor;
    dev_dbg(&intf.dev, "%s - %u.%u\n", __func__, response.major,
    response.minor);
    return 0;
    }
    static int gb_control_get_bundle_version(struct gb_control *control,
    struct gb_bundle *bundle)
    {
    struct gb_interface *intf = control.connection.intf;
    struct gb_control_bundle_version_request request;
    struct gb_control_bundle_version_response response;
    int ret;
    request.bundle_id = bundle.id;
    ret = gb_operation_sync(control.connection,
    GB_CONTROL_TYPE_BUNDLE_VERSION,
    &request, sizeof(request),
    &response, sizeof(response));
    if (ret) {
    dev_err(&intf.dev,
    "failed to get bundle %u class version: %d\n",
    bundle.id, ret);
    return ret;
    }
    bundle.class_major = response.major;
    bundle.class_minor = response.minor;
    dev_dbg(&intf.dev, "%s - %u: %u.%u\n", __func__, bundle.id,
    response.major, response.minor);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn gb_control_get_bundle_versions(control: *mut gb_control) -> c_int {
    int gb_control_get_bundle_versions(struct gb_control *control)
    {
    struct gb_interface *intf = control.connection.intf;
    struct gb_bundle *bundle;
    int ret;
    if (!control.has_bundle_version)
    return 0;
    list_for_each_entry(bundle, &intf.bundles, links) {
    ret = gb_control_get_bundle_version(control, bundle);
    if (ret)
    return ret;
    }
    return 0;
    }
// Get Manifest's size from the interface
#[no_mangle]
pub unsafe extern "C" fn gb_control_get_manifest_size_operation(intf: *mut gb_interface) -> c_int {
    int gb_control_get_manifest_size_operation(struct gb_interface *intf)
    {
    struct gb_control_get_manifest_size_response response;
    struct gb_connection *connection = intf.control.connection;
    int ret;
    ret = gb_operation_sync(connection, GB_CONTROL_TYPE_GET_MANIFEST_SIZE,
    core::ptr::null_mut(), 0, &response, sizeof(response));
    if (ret) {
    dev_err(&connection.intf.dev,
    "failed to get manifest size: %d\n", ret);
    return ret;
    }
    return le16_to_cpu(response.size);
    }
// Reads Manifest from the interface
    int gb_control_get_manifest_operation(struct gb_interface *intf, void *manifest,
    size_t size)
    {
    struct gb_connection *connection = intf.control.connection;
    return gb_operation_sync(connection, GB_CONTROL_TYPE_GET_MANIFEST,
    core::ptr::null_mut(), 0, manifest, size);
    }
#[no_mangle]
pub unsafe extern "C" fn gb_control_connected_operation(control: *mut gb_control, cport_id: u16) -> c_int {
    int gb_control_connected_operation(struct gb_control *control, u16 cport_id)
    {
    struct gb_control_connected_request request;
    request.cport_id = cpu_to_le16(cport_id);
    return gb_operation_sync(control.connection, GB_CONTROL_TYPE_CONNECTED,
    &request, sizeof(request), core::ptr::null_mut(), 0);
    }
#[no_mangle]
pub unsafe extern "C" fn gb_control_disconnected_operation(control: *mut gb_control, cport_id: u16) -> c_int {
    int gb_control_disconnected_operation(struct gb_control *control, u16 cport_id)
    {
    struct gb_control_disconnected_request request;
    request.cport_id = cpu_to_le16(cport_id);
    return gb_operation_sync(control.connection,
    GB_CONTROL_TYPE_DISCONNECTED, &request,
    sizeof(request), core::ptr::null_mut(), 0);
    }
    int gb_control_disconnecting_operation(struct gb_control *control,
    u16 cport_id)
    {
    struct gb_control_disconnecting_request *request;
    struct gb_operation *operation;
    int ret;
    operation = gb_operation_create_core(control.connection,
    GB_CONTROL_TYPE_DISCONNECTING,
    sizeof(*request), 0, 0,
    GFP_KERNEL);
    if (!operation)
    return -ENOMEM;
    request = operation.request.payload;
    request.cport_id = cpu_to_le16(cport_id);
    ret = gb_operation_request_send_sync(operation);
    if (ret) {
    dev_err(&control.dev, "failed to send disconnecting: %d\n",
    ret);
    }
    gb_operation_put(operation);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn gb_control_mode_switch_operation(control: *mut gb_control) -> c_int {
    int gb_control_mode_switch_operation(struct gb_control *control)
    {
    struct gb_operation *operation;
    int ret;
    operation = gb_operation_create_core(control.connection,
    GB_CONTROL_TYPE_MODE_SWITCH,
    0, 0,
    GB_OPERATION_FLAG_UNIDIRECTIONAL,
    GFP_KERNEL);
    if (!operation)
    return -ENOMEM;
    ret = gb_operation_request_send_sync(operation);
    if (ret)
    dev_err(&control.dev, "failed to send mode switch: %d\n", ret);
    gb_operation_put(operation);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn gb_control_bundle_pm_status_map(status: u8) -> c_int {
    static int gb_control_bundle_pm_status_map(u8 status)
    {
    switch (status) {
    case GB_CONTROL_BUNDLE_PM_INVAL:
    return -EINVAL;
    case GB_CONTROL_BUNDLE_PM_BUSY:
    return -EBUSY;
    case GB_CONTROL_BUNDLE_PM_NA:
    return -ENOMSG;
    case GB_CONTROL_BUNDLE_PM_FAIL:
    default:
    return -EREMOTEIO;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn gb_control_bundle_suspend(control: *mut gb_control, bundle_id: u8) -> c_int {
    int gb_control_bundle_suspend(struct gb_control *control, u8 bundle_id)
    {
    struct gb_control_bundle_pm_request request;
    struct gb_control_bundle_pm_response response;
    int ret;
    request.bundle_id = bundle_id;
    ret = gb_operation_sync(control.connection,
    GB_CONTROL_TYPE_BUNDLE_SUSPEND, &request,
    sizeof(request), &response, sizeof(response));
    if (ret) {
    dev_err(&control.dev, "failed to send bundle %u suspend: %d\n",
    bundle_id, ret);
    return ret;
    }
    if (response.status != GB_CONTROL_BUNDLE_PM_OK) {
    dev_err(&control.dev, "failed to suspend bundle %u: %d\n",
    bundle_id, response.status);
    return gb_control_bundle_pm_status_map(response.status);
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn gb_control_bundle_resume(control: *mut gb_control, bundle_id: u8) -> c_int {
    int gb_control_bundle_resume(struct gb_control *control, u8 bundle_id)
    {
    struct gb_control_bundle_pm_request request;
    struct gb_control_bundle_pm_response response;
    int ret;
    request.bundle_id = bundle_id;
    ret = gb_operation_sync(control.connection,
    GB_CONTROL_TYPE_BUNDLE_RESUME, &request,
    sizeof(request), &response, sizeof(response));
    if (ret) {
    dev_err(&control.dev, "failed to send bundle %u resume: %d\n",
    bundle_id, ret);
    return ret;
    }
    if (response.status != GB_CONTROL_BUNDLE_PM_OK) {
    dev_err(&control.dev, "failed to resume bundle %u: %d\n",
    bundle_id, response.status);
    return gb_control_bundle_pm_status_map(response.status);
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn gb_control_bundle_deactivate(control: *mut gb_control, bundle_id: u8) -> c_int {
    int gb_control_bundle_deactivate(struct gb_control *control, u8 bundle_id)
    {
    struct gb_control_bundle_pm_request request;
    struct gb_control_bundle_pm_response response;
    int ret;
    request.bundle_id = bundle_id;
    ret = gb_operation_sync(control.connection,
    GB_CONTROL_TYPE_BUNDLE_DEACTIVATE, &request,
    sizeof(request), &response, sizeof(response));
    if (ret) {
    dev_err(&control.dev,
    "failed to send bundle %u deactivate: %d\n", bundle_id,
    ret);
    return ret;
    }
    if (response.status != GB_CONTROL_BUNDLE_PM_OK) {
    dev_err(&control.dev, "failed to deactivate bundle %u: %d\n",
    bundle_id, response.status);
    return gb_control_bundle_pm_status_map(response.status);
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn gb_control_bundle_activate(control: *mut gb_control, bundle_id: u8) -> c_int {
    int gb_control_bundle_activate(struct gb_control *control, u8 bundle_id)
    {
    struct gb_control_bundle_pm_request request;
    struct gb_control_bundle_pm_response response;
    int ret;
    if (!control.has_bundle_activate)
    return 0;
    request.bundle_id = bundle_id;
    ret = gb_operation_sync(control.connection,
    GB_CONTROL_TYPE_BUNDLE_ACTIVATE, &request,
    sizeof(request), &response, sizeof(response));
    if (ret) {
    dev_err(&control.dev,
    "failed to send bundle %u activate: %d\n", bundle_id,
    ret);
    return ret;
    }
    if (response.status != GB_CONTROL_BUNDLE_PM_OK) {
    dev_err(&control.dev, "failed to activate bundle %u: %d\n",
    bundle_id, response.status);
    return gb_control_bundle_pm_status_map(response.status);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn gb_control_interface_pm_status_map(status: u8) -> c_int {
    static int gb_control_interface_pm_status_map(u8 status)
    {
    switch (status) {
    case GB_CONTROL_INTF_PM_BUSY:
    return -EBUSY;
    case GB_CONTROL_INTF_PM_NA:
    return -ENOMSG;
    default:
    return -EREMOTEIO;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn gb_control_interface_suspend_prepare(control: *mut gb_control) -> c_int {
    int gb_control_interface_suspend_prepare(struct gb_control *control)
    {
    struct gb_control_intf_pm_response response;
    int ret;
    ret = gb_operation_sync(control.connection,
    GB_CONTROL_TYPE_INTF_SUSPEND_PREPARE, core::ptr::null_mut(), 0,
    &response, sizeof(response));
    if (ret) {
    dev_err(&control.dev,
    "failed to send interface suspend prepare: %d\n", ret);
    return ret;
    }
    if (response.status != GB_CONTROL_INTF_PM_OK) {
    dev_err(&control.dev, "interface error while preparing suspend: %d\n",
    response.status);
    return gb_control_interface_pm_status_map(response.status);
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn gb_control_interface_deactivate_prepare(control: *mut gb_control) -> c_int {
    int gb_control_interface_deactivate_prepare(struct gb_control *control)
    {
    struct gb_control_intf_pm_response response;
    int ret;
    ret = gb_operation_sync(control.connection,
    GB_CONTROL_TYPE_INTF_DEACTIVATE_PREPARE, core::ptr::null_mut(),
    0, &response, sizeof(response));
    if (ret) {
    dev_err(&control.dev, "failed to send interface deactivate prepare: %d\n",
    ret);
    return ret;
    }
    if (response.status != GB_CONTROL_INTF_PM_OK) {
    dev_err(&control.dev, "interface error while preparing deactivate: %d\n",
    response.status);
    return gb_control_interface_pm_status_map(response.status);
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn gb_control_interface_hibernate_abort(control: *mut gb_control) -> c_int {
    int gb_control_interface_hibernate_abort(struct gb_control *control)
    {
    struct gb_control_intf_pm_response response;
    int ret;
    ret = gb_operation_sync(control.connection,
    GB_CONTROL_TYPE_INTF_HIBERNATE_ABORT, core::ptr::null_mut(), 0,
    &response, sizeof(response));
    if (ret) {
    dev_err(&control.dev,
    "failed to send interface aborting hibernate: %d\n",
    ret);
    return ret;
    }
    if (response.status != GB_CONTROL_INTF_PM_OK) {
    dev_err(&control.dev, "interface error while aborting hibernate: %d\n",
    response.status);
    return gb_control_interface_pm_status_map(response.status);
    }
    return 0;
    }
    static ssize_t vendor_string_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct gb_control *control = to_gb_control(dev);
    return scnprintf(buf, PAGE_SIZE, "%s\n", control.vendor_string);
    }
    static DEVICE_ATTR_RO(vendor_string);
    static ssize_t product_string_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct gb_control *control = to_gb_control(dev);
    return scnprintf(buf, PAGE_SIZE, "%s\n", control.product_string);
    }
    static DEVICE_ATTR_RO(product_string);
    static struct attribute *control_attrs[] = {
    &dev_attr_vendor_string.attr,
    &dev_attr_product_string.attr,
    core::ptr::null_mut(),
    };
    ATTRIBUTE_GROUPS(control);
#[no_mangle]
unsafe extern "C" fn gb_control_release(dev: *mut device) {
    static void gb_control_release(struct device *dev)
    {
    struct gb_control *control = to_gb_control(dev);
    gb_connection_destroy(control.connection);
    kfree(control.vendor_string);
    kfree(control.product_string);
    kfree(control);
    }
    const struct device_type greybus_control_type = {
    .name =		"greybus_control",
    .release =	gb_control_release,
    };
    struct gb_control *gb_control_create(struct gb_interface *intf)
    {
    struct gb_connection *connection;
    struct gb_control *control;
    control = kzalloc_obj(*control);
    if (!control)
    return ERR_PTR(-ENOMEM);
    control.intf = intf;
    connection = gb_connection_create_control(intf);
    if (IS_ERR(connection)) {
    dev_err(&intf.dev,
    "failed to create control connection: %ld\n",
    PTR_ERR(connection));
    kfree(control);
    return ERR_CAST(connection);
    }
    control.connection = connection;
    control.dev.parent = &intf.dev;
    control.dev.bus = &greybus_bus_type;
    control.dev.type = &greybus_control_type;
    control.dev.groups = control_groups;
    control.dev.dma_mask = intf.dev.dma_mask;
    device_initialize(&control.dev);
    dev_set_name(&control.dev, "%s.ctrl", dev_name(&intf.dev));
    gb_connection_set_data(control.connection, control);
    return control;
    }
#[no_mangle]
pub unsafe extern "C" fn gb_control_enable(control: *mut gb_control) -> c_int {
    int gb_control_enable(struct gb_control *control)
    {
    int ret;
    dev_dbg(&control.connection.intf.dev, "%s\n", __func__);
    ret = gb_connection_enable_tx(control.connection);
    if (ret) {
    dev_err(&control.connection.intf.dev,
    "failed to enable control connection: %d\n",
    ret);
    return ret;
    }
    ret = gb_control_get_version(control);
    if (ret)
    goto err_disable_connection;
    if (control.protocol_major > 0 || control.protocol_minor > 1)
    control.has_bundle_version = true;
// FIXME: use protocol version instead
    if (!(control.intf.quirks & GB_INTERFACE_QUIRK_NO_BUNDLE_ACTIVATE))
    control.has_bundle_activate = true;
    return 0;
    err_disable_connection:
    gb_connection_disable(control.connection);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn gb_control_disable(control: *mut gb_control) {
    void gb_control_disable(struct gb_control *control)
    {
    dev_dbg(&control.connection.intf.dev, "%s\n", __func__);
    if (control.intf.disconnected)
    gb_connection_disable_forced(control.connection);
    else
    gb_connection_disable(control.connection);
    }
#[no_mangle]
pub unsafe extern "C" fn gb_control_suspend(control: *mut gb_control) -> c_int {
    int gb_control_suspend(struct gb_control *control)
    {
    gb_connection_disable(control.connection);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn gb_control_resume(control: *mut gb_control) -> c_int {
    int gb_control_resume(struct gb_control *control)
    {
    int ret;
    ret = gb_connection_enable_tx(control.connection);
    if (ret) {
    dev_err(&control.connection.intf.dev,
    "failed to enable control connection: %d\n", ret);
    return ret;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn gb_control_add(control: *mut gb_control) -> c_int {
    int gb_control_add(struct gb_control *control)
    {
    int ret;
    ret = device_add(&control.dev);
    if (ret) {
    dev_err(&control.dev,
    "failed to register control device: %d\n",
    ret);
    return ret;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn gb_control_del(control: *mut gb_control) {
    void gb_control_del(struct gb_control *control)
    {
    if (device_is_registered(&control.dev))
    device_del(&control.dev);
    }
    struct gb_control *gb_control_get(struct gb_control *control)
    {
    get_device(&control.dev);
    return control;
    }
#[no_mangle]
pub unsafe extern "C" fn gb_control_put(control: *mut gb_control) {
    void gb_control_put(struct gb_control *control)
    {
    put_device(&control.dev);
    }
#[no_mangle]
pub unsafe extern "C" fn gb_control_mode_switch_prepare(control: *mut gb_control) {
    void gb_control_mode_switch_prepare(struct gb_control *control)
    {
    gb_connection_mode_switch_prepare(control.connection);
    }
#[no_mangle]
pub unsafe extern "C" fn gb_control_mode_switch_complete(control: *mut gb_control) {
    void gb_control_mode_switch_complete(struct gb_control *control)
    {
    gb_connection_mode_switch_complete(control.connection);
    }
