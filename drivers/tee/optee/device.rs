//! Automatically rewritten from C to Rust
//! Source: drivers/tee/optee/device.c
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
// Copyright (C) 2019 Linaro Ltd.
//

#[no_mangle]
unsafe extern "C" fn optee_ctx_match(ver: *mut tee_ioctl_version_data, data: *const c_void) -> c_int {
    static int optee_ctx_match(struct tee_ioctl_version_data *ver, const void *data)
    {
    return (ver.impl_id == TEE_IMPL_ID_OPTEE);
    }
    static int get_devices(struct tee_context *ctx, u32 session,
    struct tee_shm *device_shm, u32 *shm_size,
    u32 func)
    {
    let mut ret: c_int = 0;
    struct tee_ioctl_invoke_arg inv_arg;
    struct tee_param param[4];
    memset(&inv_arg, 0, sizeof(inv_arg));
    memset(&param, 0, sizeof(param));
    inv_arg.func = func;
    inv_arg.session = session;
    inv_arg.num_params = 4;
// Fill invoke cmd params
    param[0].attr = TEE_IOCTL_PARAM_ATTR_TYPE_MEMREF_OUTPUT;
    param[0].u.memref.shm = device_shm;
    param[0].u.memref.size = *shm_size;
    param[0].u.memref.shm_offs = 0;
    ret = tee_client_invoke_func(ctx, &inv_arg, param);
    if ((ret < 0) || ((inv_arg.ret != TEEC_SUCCESS) &&
    (inv_arg.ret != TEEC_ERROR_SHORT_BUFFER))) {
//
// TEE_ERROR_STORAGE_NOT_AVAILABLE is returned when getting
// the list of device TAs that depends on RPMB but a usable
// RPMB device isn't found.
//
    if (inv_arg.ret == TEE_ERROR_STORAGE_NOT_AVAILABLE)
    return -ENODEV;
    pr_err("PTA_CMD_GET_DEVICES invoke function err: %x\n",
    inv_arg.ret);
    return -EINVAL;
    }
// shm_size = param[0].u.memref.size;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn optee_release_device(dev: *mut device) {
    static void optee_release_device(struct device *dev)
    {
    struct tee_client_device *optee_device = to_tee_client_device(dev);
    kfree(optee_device);
    }
    static ssize_t need_supplicant_show(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    return 0;
    }
    static DEVICE_ATTR_RO(need_supplicant);
#[no_mangle]
unsafe extern "C" fn optee_register_device(device_uuid: *const uuid_t, func: u32) -> c_int {
    static int optee_register_device(const uuid_t *device_uuid, u32 func)
    {
    struct tee_client_device *optee_device = core::ptr::null_mut();
    int rc;
    optee_device = kzalloc_obj(*optee_device);
    if (!optee_device)
    return -ENOMEM;
    optee_device.dev.bus = &tee_bus_type;
    optee_device.dev.release = optee_release_device;
    if (dev_set_name(&optee_device.dev, "optee-ta-%pUb", device_uuid)) {
    kfree(optee_device);
    return -ENOMEM;
    }
    uuid_copy(&optee_device.id.uuid, device_uuid);
    rc = device_register(&optee_device.dev);
    if (rc) {
    pr_err("device registration failed, err: %d\n", rc);
    put_device(&optee_device.dev);
    return rc;
    }
    if (func == PTA_CMD_GET_DEVICES_SUPP)
    device_create_file(&optee_device.dev,
    &dev_attr_need_supplicant);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn __optee_enumerate_devices(func: u32) -> c_int {
    static int __optee_enumerate_devices(u32 func)
    {
    const uuid_t pta_uuid =
    UUID_INIT(0x7011a688, 0xddde, 0x4053,
    0xa5, 0xa9, 0x7b, 0x3c, 0x4d, 0xdf, 0x13, 0xb8);
    struct tee_ioctl_open_session_arg sess_arg;
    struct tee_shm *device_shm = core::ptr::null_mut();
    const uuid_t *device_uuid = core::ptr::null_mut();
    struct tee_context *ctx = core::ptr::null_mut();
    let mut shm_size: u32 = 0, idx, num_devices = 0;
    int rc;
    memset(&sess_arg, 0, sizeof(sess_arg));
// Open context with OP-TEE driver
    ctx = tee_client_open_context(core::ptr::null_mut(), optee_ctx_match, core::ptr::null_mut(), core::ptr::null_mut());
    if (IS_ERR(ctx))
    return -ENODEV;
// Open session with device enumeration pseudo TA
    export_uuid(sess_arg.uuid, &pta_uuid);
    sess_arg.clnt_login = TEE_IOCTL_LOGIN_PUBLIC;
    sess_arg.num_params = 0;
    rc = tee_client_open_session(ctx, &sess_arg, core::ptr::null_mut());
    if ((rc < 0) || (sess_arg.ret != TEEC_SUCCESS)) {
// Device enumeration pseudo TA not found
    rc = 0;
    goto out_ctx;
    }
    rc = get_devices(ctx, sess_arg.session, core::ptr::null_mut(), &shm_size, func);
    if (rc < 0 || !shm_size)
    goto out_sess;
    device_shm = tee_shm_alloc_kernel_buf(ctx, shm_size);
    if (IS_ERR(device_shm)) {
    pr_err("tee_shm_alloc_kernel_buf failed\n");
    rc = PTR_ERR(device_shm);
    goto out_sess;
    }
    rc = get_devices(ctx, sess_arg.session, device_shm, &shm_size, func);
    if (rc < 0)
    goto out_shm;
    device_uuid = tee_shm_get_va(device_shm, 0);
    if (IS_ERR(device_uuid)) {
    pr_err("tee_shm_get_va failed\n");
    rc = PTR_ERR(device_uuid);
    goto out_shm;
    }
    num_devices = shm_size / sizeof(uuid_t);
    for (idx = 0; idx < num_devices; idx++) {
    rc = optee_register_device(&device_uuid[idx], func);
    if (rc)
    goto out_shm;
    }
    out_shm:
    tee_shm_free(device_shm);
    out_sess:
    tee_client_close_session(ctx, sess_arg.session);
    out_ctx:
    tee_client_close_context(ctx);
    return rc;
    }
#[no_mangle]
pub unsafe extern "C" fn optee_enumerate_devices(func: u32) -> c_int {
    int optee_enumerate_devices(u32 func)
    {
    return  __optee_enumerate_devices(func);
    }
#[no_mangle]
unsafe extern "C" fn __optee_unregister_device(dev: *mut device, data: *mut c_void) -> c_int {
    static int __optee_unregister_device(struct device *dev, void *data)
    {
    if (!strncmp(dev_name(dev), "optee-ta", strlen("optee-ta")))
    device_unregister(dev);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn optee_unregister_devices() {
    void optee_unregister_devices(void)
    {
    bus_for_each_dev(&tee_bus_type, core::ptr::null_mut(), core::ptr::null_mut(),
    __optee_unregister_device);
    }
