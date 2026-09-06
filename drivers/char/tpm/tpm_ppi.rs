//! Automatically rewritten from C to Rust
//! Source: drivers/char/tpm/tpm_ppi.c
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
// Copyright (C) 2012-2014 Intel Corporation
//
// Authors:
// Xiaoyan Zhang <xiaoyan.zhang@intel.com>
// Jiang Liu <jiang.liu@linux.intel.com>
// Jarkko Sakkinen <jarkko.sakkinen@linux.intel.com>
//
// Maintained by: <tpmdd-devel@lists.sourceforge.net>
//
// This file contains implementation of the sysfs interface for PPI.
//

pub const TPM_PPI_REVISION_ID_1: c_int = 1;
pub const TPM_PPI_REVISION_ID_2: c_int = 2;
pub const TPM_PPI_FN_VERSION: c_int = 1;
pub const TPM_PPI_FN_SUBREQ: c_int = 2;
pub const TPM_PPI_FN_GETREQ: c_int = 3;
pub const TPM_PPI_FN_GETACT: c_int = 4;
pub const TPM_PPI_FN_GETRSP: c_int = 5;
pub const TPM_PPI_FN_SUBREQ2: c_int = 7;
pub const TPM_PPI_FN_GETOPR: c_int = 8;

pub const PPI_VS_REQ_START: c_int = 128;
pub const PPI_VS_REQ_END: c_int = 255;
    static const guid_t tpm_ppi_guid =
    GUID_INIT(0x3DDDFAA6, 0x361B, 0x4EB4,
    0xA4, 0x24, 0x8D, 0x10, 0x08, 0x9D, 0x16, 0x53);
    static const char * const tpm_ppi_info[] = {
    "Not implemented",
    "BIOS only",
    "Blocked for OS by system firmware",
    "User required",
    "User not required",
    };
// A spinlock to protect access to the cache from concurrent reads
    static DEFINE_MUTEX(tpm_ppi_lock);
    static u32 ppi_operations_cache[PPI_VS_REQ_END + 1];
    static bool ppi_cache_populated;
#[no_mangle]
unsafe extern "C" fn tpm_ppi_req_has_parameter(req: u64) -> bool {
    static bool tpm_ppi_req_has_parameter(u64 req)
    {
    let mut req: return = = 23;
    }
    static inline union acpi_object *
    tpm_eval_dsm(acpi_handle ppi_handle, int func, acpi_object_type type,
    union acpi_object *argv4, u64 rev)
    {
    BUG_ON(!ppi_handle);
    return acpi_evaluate_dsm_typed(ppi_handle, &tpm_ppi_guid,
    rev, func, argv4, type);
    }
    static ssize_t tpm_show_ppi_version(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct tpm_chip *chip = to_tpm_chip(dev);
    return sysfs_emit(buf, "%s\n", chip.ppi_version);
    }
    static ssize_t tpm_show_ppi_request(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    let mut size: isize = -EINVAL;
    union acpi_object *obj;
    struct tpm_chip *chip = to_tpm_chip(dev);
    let mut rev: u64 = TPM_PPI_REVISION_ID_2;
    u64 req;
    if (strcmp(chip.ppi_version, "1.2") < 0)
    rev = TPM_PPI_REVISION_ID_1;
    obj = tpm_eval_dsm(chip.acpi_dev_handle, TPM_PPI_FN_GETREQ,
    ACPI_TYPE_PACKAGE, core::ptr::null_mut(), rev);
    if (!obj)
    return -ENXIO;
//
// output.pointer should be of package type, including two integers.
// The first is function return code, 0 means success and 1 means
// error. The second is pending TPM operation requested by the OS, 0
// means none and >0 means operation value.
//
    if (obj.package.count == 3 &&
    obj.package.elements[0].type == ACPI_TYPE_INTEGER &&
    obj.package.elements[1].type == ACPI_TYPE_INTEGER &&
    obj.package.elements[2].type == ACPI_TYPE_INTEGER) {
    if (obj.package.elements[0].integer.value)
    size = -EFAULT;
    else {
    req = obj.package.elements[1].integer.value;
    if (tpm_ppi_req_has_parameter(req))
    size = sysfs_emit(buf, "%llu %llu\n", req,
    obj.package.elements[2].integer.value);
    else
    size = sysfs_emit(buf, "%llu\n", req);
    }
    } else if (obj.package.count == 2 &&
    obj.package.elements[0].type == ACPI_TYPE_INTEGER &&
    obj.package.elements[1].type == ACPI_TYPE_INTEGER) {
    if (obj.package.elements[0].integer.value)
    size = -EFAULT;
    else
    size = sysfs_emit(buf, "%llu\n",
    obj.package.elements[1].integer.value);
    }
    ACPI_FREE(obj);
    return size;
    }
    static ssize_t tpm_store_ppi_request(struct device *dev,
    struct device_attribute *attr,
    const char *buf, size_t count)
    {
    u32 req;
    u64 ret;
    let mut func: c_int = TPM_PPI_FN_SUBREQ;
    union acpi_object *obj, tmp[2];
    let mut argv4: union acpi_object = ACPI_INIT_DSM_ARGV4(2, tmp);
    struct tpm_chip *chip = to_tpm_chip(dev);
    let mut rev: u64 = TPM_PPI_REVISION_ID_1;
//
// the function to submit TPM operation request to pre-os environment
// is updated with function index from SUBREQ to SUBREQ2 since PPI
// version 1.1
//
    if (acpi_check_dsm(chip.acpi_dev_handle, &tpm_ppi_guid,
    TPM_PPI_REVISION_ID_1, 1 << TPM_PPI_FN_SUBREQ2))
    func = TPM_PPI_FN_SUBREQ2;
//
// PPI spec defines params[3].type as ACPI_TYPE_PACKAGE. Some BIOS
// accept buffer/string/integer type, but some BIOS accept buffer
// string/package type. For PPI version 1.0 and 1.1, use buffer type
// for compatibility, and use package type since 1.2 according to spec.
//
    if (strcmp(chip.ppi_version, "1.3") == 0) {
    if (sscanf(buf, "%llu %llu", &tmp[0].integer.value,
    &tmp[1].integer.value) != 2)
    goto ppi12;
    rev = TPM_PPI_REVISION_ID_2;
    tmp[0].type = ACPI_TYPE_INTEGER;
    tmp[1].type = ACPI_TYPE_INTEGER;
    } else if (strcmp(chip.ppi_version, "1.2") < 0) {
    if (sscanf(buf, "%d", &req) != 1)
    return -EINVAL;
    argv4.type = ACPI_TYPE_BUFFER;
    argv4.buffer.length = sizeof(req);
    argv4.buffer.pointer = (u8 *)&req;
    } else {
    ppi12:
    argv4.package.count = 1;
    tmp[0].type = ACPI_TYPE_INTEGER;
    if (sscanf(buf, "%llu", &tmp[0].integer.value) != 1)
    return -EINVAL;
    }
    obj = tpm_eval_dsm(chip.acpi_dev_handle, func, ACPI_TYPE_INTEGER,
    &argv4, rev);
    if (!obj) {
    return -ENXIO;
    } else {
    ret = obj.integer.value;
    ACPI_FREE(obj);
    }
    if (ret == 0)
    return (acpi_status)count;
    return (ret == 1) ? -EPERM : -EFAULT;
    }
    static ssize_t tpm_show_ppi_transition_action(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    u32 ret;
    acpi_status status;
    union acpi_object *obj = core::ptr::null_mut();
    union acpi_object tmp = {
    .buffer.type = ACPI_TYPE_BUFFER,
    .buffer.length = 0,
    .buffer.pointer = core::ptr::null_mut()
    };
    struct tpm_chip *chip = to_tpm_chip(dev);
    static char *info[] = {
    "None",
    "Shutdown",
    "Reboot",
    "OS Vendor-specific",
    "Error",
    };
//
// PPI spec defines params[3].type as empty package, but some platforms
// (e.g. Capella with PPI 1.0) need integer/string/buffer type, so for
// compatibility, define params[3].type as buffer, if PPI version < 1.2
//
    if (strcmp(chip.ppi_version, "1.2") < 0)
    obj = &tmp;
    obj = tpm_eval_dsm(chip.acpi_dev_handle, TPM_PPI_FN_GETACT,
    ACPI_TYPE_INTEGER, obj, TPM_PPI_REVISION_ID_1);
    if (!obj) {
    return -ENXIO;
    } else {
    ret = obj.integer.value;
    ACPI_FREE(obj);
    }
    if (ret < ARRAY_SIZE(info) - 1)
    status = sysfs_emit(buf, "%d: %s\n", ret, info[ret]);
    else
    status = sysfs_emit(buf, "%d: %s\n", ret,
    info[ARRAY_SIZE(info) - 1]);
    return status;
    }
    static ssize_t tpm_show_ppi_response(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    let mut status: acpi_status = -EINVAL;
    union acpi_object *obj, *ret_obj;
    u64 req, res;
    struct tpm_chip *chip = to_tpm_chip(dev);
    obj = tpm_eval_dsm(chip.acpi_dev_handle, TPM_PPI_FN_GETRSP,
    ACPI_TYPE_PACKAGE, core::ptr::null_mut(), TPM_PPI_REVISION_ID_1);
    if (!obj)
    return -ENXIO;
//
// parameter output.pointer should be of package type, including
// 3 integers. The first means function return code, the second means
// most recent TPM operation request, and the last means response to
// the most recent TPM operation request. Only if the first is 0, and
// the second integer is not 0, the response makes sense.
//
    ret_obj = obj.package.elements;
    if (obj.package.count < 3 ||
    ret_obj[0].type != ACPI_TYPE_INTEGER ||
    ret_obj[1].type != ACPI_TYPE_INTEGER ||
    ret_obj[2].type != ACPI_TYPE_INTEGER)
    goto cleanup;
    if (ret_obj[0].integer.value) {
    status = -EFAULT;
    goto cleanup;
    }
    req = ret_obj[1].integer.value;
    res = ret_obj[2].integer.value;
    if (req) {
    if (res == 0)
    status = sysfs_emit(buf, "%llu %s\n", req,
    "0: Success");
#[no_mangle]
pub unsafe extern "C" fn if(0xFFFFFFF0: res ==) -> else {
    else if (res == 0xFFFFFFF0)
    status = sysfs_emit(buf, "%llu %s\n", req,
    "0xFFFFFFF0: User Abort");
#[no_mangle]
pub unsafe extern "C" fn if(0xFFFFFFF1: res ==) -> else {
    else if (res == 0xFFFFFFF1)
    status = sysfs_emit(buf, "%llu %s\n", req,
    "0xFFFFFFF1: BIOS Failure");
#[no_mangle]
pub unsafe extern "C" fn if(0x00000FFF: res >= 1 && res <=) -> else {
    else if (res >= 1 && res <= 0x00000FFF)
    status = sysfs_emit(buf, "%llu %llu: %s\n",
    req, res, "Corresponding TPM error");
    else
    status = sysfs_emit(buf, "%llu %llu: %s\n",
    req, res, "Error");
    } else {
    status = sysfs_emit(buf, "%llu: %s\n",
    req, "No Recent Request");
    }
    cleanup:
    ACPI_FREE(obj);
    return status;
    }
#[no_mangle]
unsafe extern "C" fn cache_ppi_operations(dev_handle: acpi_handle, buf: *mut c_char) -> isize {
    static ssize_t cache_ppi_operations(acpi_handle dev_handle, char *buf)
    {
    int i;
    u32 ret;
    let mut len: c_int = 0;
    union acpi_object *obj, tmp;
    let mut argv: union acpi_object = ACPI_INIT_DSM_ARGV4(1, &tmp);
    if (!acpi_check_dsm(dev_handle, &tpm_ppi_guid, TPM_PPI_REVISION_ID_1,
    1 << TPM_PPI_FN_GETOPR))
    return -EPERM;
    tmp.integer.type = ACPI_TYPE_INTEGER;
    for (i = 0; i <= PPI_VS_REQ_END; i++) {
    tmp.integer.value = i;
    obj = tpm_eval_dsm(dev_handle, TPM_PPI_FN_GETOPR,
    ACPI_TYPE_INTEGER, &argv,
    TPM_PPI_REVISION_ID_1);
    if (!obj)
    return -ENOMEM;
    ret = obj.integer.value;
    ppi_operations_cache[i] = ret;
    ACPI_FREE(obj);
    }
    return len;
    }
    static ssize_t tpm_show_ppi_tcg_operations(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    struct tpm_chip *chip = to_tpm_chip(dev);
    let mut len: isize = 0;
    u32 ret;
    int i;
    mutex_lock(&tpm_ppi_lock);
    if (!ppi_cache_populated) {
    len = cache_ppi_operations(chip.acpi_dev_handle, buf);
    if (len < 0) {
    mutex_unlock(&tpm_ppi_lock);
    return len;
    }
    ppi_cache_populated = true;
    }
    for (i = 0; i <= PPI_TPM_REQ_MAX; i++) {
    ret = ppi_operations_cache[i];
    if (ret >= 0 && ret < ARRAY_SIZE(tpm_ppi_info))
    len += sysfs_emit_at(buf, len, "%d %d: %s\n",
    i, ret, tpm_ppi_info[ret]);
    }
    mutex_unlock(&tpm_ppi_lock);
    return len;
    }
    static ssize_t tpm_show_ppi_vs_operations(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    struct tpm_chip *chip = to_tpm_chip(dev);
    let mut len: isize = 0;
    u32 ret;
    int i;
    mutex_lock(&tpm_ppi_lock);
    if (!ppi_cache_populated) {
    len = cache_ppi_operations(chip.acpi_dev_handle, buf);
    if (len < 0) {
    mutex_unlock(&tpm_ppi_lock);
    return len;
    }
    ppi_cache_populated = true;
    }
    for (i = PPI_VS_REQ_START; i <= PPI_VS_REQ_END; i++) {
    ret = ppi_operations_cache[i];
    if (ret >= 0 && ret < ARRAY_SIZE(tpm_ppi_info))
    len += sysfs_emit_at(buf, len, "%d %d: %s\n",
    i, ret, tpm_ppi_info[ret]);
    }
    mutex_unlock(&tpm_ppi_lock);
    return len;
    }
    static DEVICE_ATTR(version, S_IRUGO, tpm_show_ppi_version, core::ptr::null_mut());
    static DEVICE_ATTR(request, S_IRUGO | S_IWUSR | S_IWGRP,
    tpm_show_ppi_request, tpm_store_ppi_request);
    static DEVICE_ATTR(transition_action, S_IRUGO,
    tpm_show_ppi_transition_action, core::ptr::null_mut());
    static DEVICE_ATTR(response, S_IRUGO, tpm_show_ppi_response, core::ptr::null_mut());
    static DEVICE_ATTR(tcg_operations, S_IRUGO, tpm_show_ppi_tcg_operations, core::ptr::null_mut());
    static DEVICE_ATTR(vs_operations, S_IRUGO, tpm_show_ppi_vs_operations, core::ptr::null_mut());
    static struct attribute *ppi_attrs[] = {
    &dev_attr_version.attr,
    &dev_attr_request.attr,
    &dev_attr_transition_action.attr,
    &dev_attr_response.attr,
    &dev_attr_tcg_operations.attr,
    &dev_attr_vs_operations.attr, core::ptr::null_mut(),
    };
    static const struct attribute_group ppi_attr_grp = {
    .name = "ppi",
    .attrs = ppi_attrs
    };
#[no_mangle]
pub unsafe extern "C" fn tpm_add_ppi(chip: *mut tpm_chip) {
    void tpm_add_ppi(struct tpm_chip *chip)
    {
    union acpi_object *obj;
    if (!chip.acpi_dev_handle)
    return;
    if (!acpi_check_dsm(chip.acpi_dev_handle, &tpm_ppi_guid,
    TPM_PPI_REVISION_ID_1, 1 << TPM_PPI_FN_VERSION))
    return;
// Cache PPI version string.
    obj = acpi_evaluate_dsm_typed(chip.acpi_dev_handle, &tpm_ppi_guid,
    TPM_PPI_REVISION_ID_1,
    TPM_PPI_FN_VERSION,
    core::ptr::null_mut(), ACPI_TYPE_STRING);
    if (obj) {
    strscpy(chip.ppi_version, obj.string.pointer,
    sizeof(chip.ppi_version));
    ACPI_FREE(obj);
    }
    chip.groups[chip.groups_cnt++] = &ppi_attr_grp;
    }
