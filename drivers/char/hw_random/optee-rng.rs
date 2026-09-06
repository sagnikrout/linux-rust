//! Automatically rewritten from C to Rust
//! Source: drivers/char/hw_random/optee-rng.c
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
// Copyright (C) 2018-2019 Linaro Ltd.
//

pub const TEE_ERROR_HEALTH_TEST_FAIL: c_uint = 0x00000001;
//
// TA_CMD_GET_ENTROPY - Get Entropy from RNG
//
// param[0] (inout memref) - Entropy buffer memory reference
// param[1] unused
// param[2] unused
// param[3] unused
//
// Result:
// TEE_SUCCESS - Invoke command success
// TEE_ERROR_BAD_PARAMETERS - Incorrect input param
// TEE_ERROR_NOT_SUPPORTED - Requested entropy size greater than size of pool
// TEE_ERROR_HEALTH_TEST_FAIL - Continuous health testing failed
//
pub const TA_CMD_GET_ENTROPY: c_uint = 0x0;
//
// TA_CMD_GET_RNG_INFO - Get RNG information
//
// param[0] (out value) - value.a: RNG data-rate in bytes per second
// value.b: Quality/Entropy per 1024 bit of data
// param[1] unused
// param[2] unused
// param[3] unused
//
// Result:
// TEE_SUCCESS - Invoke command success
// TEE_ERROR_BAD_PARAMETERS - Incorrect input param
//
pub const TA_CMD_GET_RNG_INFO: c_uint = 0x1;

//
// struct optee_rng_private - OP-TEE Random Number Generator private data
// @dev:		OP-TEE based RNG device.
// @ctx:		OP-TEE context handler.
// @session_id:		RNG TA session identifier.
// @data_rate:		RNG data rate.
// @entropy_shm_pool:	Memory pool shared with RNG device.
// @optee_rng:		OP-TEE RNG driver structure.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct optee_rng_private {
    pub dev: *mut device,
    pub ctx: *mut tee_context,
    pub session_id: u32,
    pub data_rate: u32,
    pub entropy_shm_pool: *mut tee_shm,
    pub optee_rng: hwrng,
}

    container_of(r, struct optee_rng_private, optee_rng)
    static size_t get_optee_rng_data(struct optee_rng_private *pvt_data,
    void *buf, size_t req_size)
    {
    let mut ret: c_int = 0;
    u8 *rng_data = core::ptr::null_mut();
    let mut rng_size: usize = 0;
    struct tee_ioctl_invoke_arg inv_arg;
    struct tee_param param[4];
    memset(&inv_arg, 0, sizeof(inv_arg));
    memset(&param, 0, sizeof(param));
// Invoke TA_CMD_GET_ENTROPY function of Trusted App
    inv_arg.func = TA_CMD_GET_ENTROPY;
    inv_arg.session = pvt_data.session_id;
    inv_arg.num_params = 4;
// Fill invoke cmd params
    param[0].attr = TEE_IOCTL_PARAM_ATTR_TYPE_MEMREF_INOUT;
    param[0].u.memref.shm = pvt_data.entropy_shm_pool;
    param[0].u.memref.size = req_size;
    param[0].u.memref.shm_offs = 0;
    ret = tee_client_invoke_func(pvt_data.ctx, &inv_arg, param);
    if ((ret < 0) || (inv_arg.ret != 0)) {
    dev_err(pvt_data.dev, "TA_CMD_GET_ENTROPY invoke err: %x\n",
    inv_arg.ret);
    return 0;
    }
    rng_data = tee_shm_get_va(pvt_data.entropy_shm_pool, 0);
    if (IS_ERR(rng_data)) {
    dev_err(pvt_data.dev, "tee_shm_get_va failed\n");
    return 0;
    }
    rng_size = param[0].u.memref.size;
    memcpy(buf, rng_data, rng_size);
    return rng_size;
    }
#[no_mangle]
unsafe extern "C" fn optee_rng_read(rng: *mut hwrng, buf: *mut c_void, max: usize, wait: bool) -> c_int {
    static int optee_rng_read(struct hwrng *rng, void *buf, size_t max, bool wait)
    {
    struct optee_rng_private *pvt_data = to_optee_rng_private(rng);
    let mut read: usize = 0, rng_size;
    let mut timeout: c_int = 1;
    u8 *data = buf;
    if (max > MAX_ENTROPY_REQ_SZ)
    max = MAX_ENTROPY_REQ_SZ;
    while (read < max) {
    rng_size = get_optee_rng_data(pvt_data, data, (max - read));
    data += rng_size;
    read += rng_size;
    if (wait && pvt_data.data_rate) {
    if ((timeout-- == 0) || (read == max))
    return read;
    msleep((1000 * (max - read)) / pvt_data.data_rate);
    } else {
    return read;
    }
    }
    return read;
    }
#[no_mangle]
unsafe extern "C" fn optee_rng_init(rng: *mut hwrng) -> c_int {
    static int optee_rng_init(struct hwrng *rng)
    {
    struct optee_rng_private *pvt_data = to_optee_rng_private(rng);
    struct tee_shm *entropy_shm_pool = core::ptr::null_mut();
    entropy_shm_pool = tee_shm_alloc_kernel_buf(pvt_data.ctx,
    MAX_ENTROPY_REQ_SZ);
    if (IS_ERR(entropy_shm_pool)) {
    dev_err(pvt_data.dev, "tee_shm_alloc_kernel_buf failed\n");
    return PTR_ERR(entropy_shm_pool);
    }
    pvt_data.entropy_shm_pool = entropy_shm_pool;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn optee_rng_cleanup(rng: *mut hwrng) {
    static void optee_rng_cleanup(struct hwrng *rng)
    {
    struct optee_rng_private *pvt_data = to_optee_rng_private(rng);
    tee_shm_free(pvt_data.entropy_shm_pool);
    }
    static struct optee_rng_private pvt_data = {
    .optee_rng = {
    .name		= DRIVER_NAME,
    .init		= optee_rng_init,
    .cleanup	= optee_rng_cleanup,
    .read		= optee_rng_read,
    }
    };
#[no_mangle]
unsafe extern "C" fn get_optee_rng_info(dev: *mut device) -> c_int {
    static int get_optee_rng_info(struct device *dev)
    {
    let mut ret: c_int = 0;
    struct tee_ioctl_invoke_arg inv_arg;
    struct tee_param param[4];
    memset(&inv_arg, 0, sizeof(inv_arg));
    memset(&param, 0, sizeof(param));
// Invoke TA_CMD_GET_RNG_INFO function of Trusted App
    inv_arg.func = TA_CMD_GET_RNG_INFO;
    inv_arg.session = pvt_data.session_id;
    inv_arg.num_params = 4;
// Fill invoke cmd params
    param[0].attr = TEE_IOCTL_PARAM_ATTR_TYPE_VALUE_OUTPUT;
    ret = tee_client_invoke_func(pvt_data.ctx, &inv_arg, param);
    if ((ret < 0) || (inv_arg.ret != 0)) {
    dev_err(dev, "TA_CMD_GET_RNG_INFO invoke err: %x\n",
    inv_arg.ret);
    return -EINVAL;
    }
    pvt_data.data_rate = param[0].u.value.a;
    pvt_data.optee_rng.quality = param[0].u.value.b;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn optee_ctx_match(ver: *mut tee_ioctl_version_data, data: *const c_void) -> c_int {
    static int optee_ctx_match(struct tee_ioctl_version_data *ver, const void *data)
    {
    return (ver.impl_id == TEE_IMPL_ID_OPTEE);
    }
#[no_mangle]
unsafe extern "C" fn optee_rng_probe(rng_device: *mut tee_client_device) -> c_int {
    static int optee_rng_probe(struct tee_client_device *rng_device)
    {
    struct device *dev = &rng_device.dev;
    let mut ret: c_int = 0, err = -ENODEV;
    struct tee_ioctl_open_session_arg sess_arg;
    memset(&sess_arg, 0, sizeof(sess_arg));
// Open context with TEE driver
    pvt_data.ctx = tee_client_open_context(core::ptr::null_mut(), optee_ctx_match, core::ptr::null_mut(),
    core::ptr::null_mut());
    if (IS_ERR(pvt_data.ctx))
    return -ENODEV;
// Open session with hwrng Trusted App
    export_uuid(sess_arg.uuid, &rng_device.id.uuid);
    sess_arg.clnt_login = TEE_IOCTL_LOGIN_PUBLIC;
    sess_arg.num_params = 0;
    ret = tee_client_open_session(pvt_data.ctx, &sess_arg, core::ptr::null_mut());
    if ((ret < 0) || (sess_arg.ret != 0)) {
    dev_err(dev, "tee_client_open_session failed, err: %x\n",
    sess_arg.ret);
    err = -EINVAL;
    goto out_ctx;
    }
    pvt_data.session_id = sess_arg.session;
    err = get_optee_rng_info(dev);
    if (err)
    goto out_sess;
    err = devm_hwrng_register(dev, &pvt_data.optee_rng);
    if (err) {
    dev_err(dev, "hwrng registration failed (%d)\n", err);
    goto out_sess;
    }
    pvt_data.dev = dev;
    return 0;
    out_sess:
    tee_client_close_session(pvt_data.ctx, pvt_data.session_id);
    out_ctx:
    tee_client_close_context(pvt_data.ctx);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn optee_rng_remove(tee_dev: *mut tee_client_device) {
    static void optee_rng_remove(struct tee_client_device *tee_dev)
    {
    tee_client_close_session(pvt_data.ctx, pvt_data.session_id);
    tee_client_close_context(pvt_data.ctx);
    }
    static const struct tee_client_device_id optee_rng_id_table[] = {
    {UUID_INIT(0xab7a617c, 0xb8e7, 0x4d8f,
    0x83, 0x01, 0xd0, 0x9b, 0x61, 0x03, 0x6b, 0x64)},
    {}
    };
    MODULE_DEVICE_TABLE(tee, optee_rng_id_table);
    static struct tee_client_driver optee_rng_driver = {
    .probe		= optee_rng_probe,
    .remove		= optee_rng_remove,
    .id_table	= optee_rng_id_table,
    .driver		= {
    .name		= DRIVER_NAME,
    },
    };
    module_tee_client_driver(optee_rng_driver);
    MODULE_LICENSE("GPL v2");
    MODULE_AUTHOR("Sumit Garg <sumit.garg@linaro.org>");
    MODULE_DESCRIPTION("OP-TEE based random number generator driver");
