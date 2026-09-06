//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/platforms/powernv/opal-flash.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// PowerNV OPAL Firmware Update Interface
//
// Copyright 2013 IBM Corp.
//
// Macro flag: #define DEBUG

// FLASH status codes

// Validate image status values

// Manage image status values

// Flash image status values

// Manage operation tokens

// Update tokens

// Validate image update result tokens

//
// Current T side will be committed to P side before being replace with new
// image, and the new image is downlevel from current image
//
pub const VALIDATE_TMP_COMMIT_DL: c_int = 4;
//
// Current T side will be committed to P side before being replaced with new
// image
//
pub const VALIDATE_TMP_COMMIT: c_int = 5;
//
// T side will be updated with a downlevel image
//
pub const VALIDATE_TMP_UPDATE_DL: c_int = 6;
//
// The candidate image's release date is later than the system's firmware
// service entitlement date - service warranty period has expired
//
pub const VALIDATE_OUT_OF_WRNTY: c_int = 7;
// Validate buffer size
pub const VALIDATE_BUF_SIZE: c_int = 4096;
// XXX: Assume candidate image size is <= 1GB
pub const MAX_IMAGE_SIZE: c_uint = 0x40000000;
// Image status
    enum {
    IMAGE_INVALID,
    IMAGE_LOADING,
    IMAGE_READY,
    };
// Candidate image data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct image_data_t {
    pub status: c_int,
    pub data: *mut c_void,
    pub size: u32,
}

// Candidate image header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct image_header_t {
    pub magic: u16,
    pub version: u16,
    pub size: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct validate_flash_t {
    pub /: *mut *mut int status; / Return status,
    pub /: *mut *mut *mut void buf; / Candidate image buffer,
    pub /: *mut *mut uint32_t buf_size; / Image size,
    pub /: *mut *mut uint32_t result; / Update results token,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct manage_flash_t {
    pub /: *mut *mut int status; / Return status,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct update_flash_t {
    pub /: *mut *mut int status; / Return status,
}

    static struct image_header_t	image_header;
    static struct image_data_t	image_data;
    static struct validate_flash_t	validate_flash_data;
    static struct manage_flash_t	manage_flash_data;
// Initialize update_flash_data status to No Operation
    static struct update_flash_t	update_flash_data = {
    .status = FLASH_NO_OP,
    };
    static DEFINE_MUTEX(image_data_mutex);
//
// Validate candidate image
//
#[no_mangle]
pub unsafe extern "C" fn opal_flash_validate() {
    static inline void opal_flash_validate(void)
    {
    long ret;
    void *buf = validate_flash_data.buf;
    let mut size: __be32 = cpu_to_be32(validate_flash_data.buf_size);
    __be32 result;
    ret = opal_validate_flash(__pa(buf), &size, &result);
    validate_flash_data.status = ret;
    validate_flash_data.buf_size = be32_to_cpu(size);
    validate_flash_data.result = be32_to_cpu(result);
    }
//
// Validate output format:
// validate result token
// current image version details
// new image version details
//
    static ssize_t validate_show(struct kobject *kobj,
    struct kobj_attribute *attr, char *buf)
    {
    struct validate_flash_t *args_buf = &validate_flash_data;
    int len;
// Candidate image is not validated
    if (args_buf.status < VALIDATE_TMP_UPDATE) {
    len = sprintf(buf, "%d\n", args_buf.status);
    goto out;
    }
// Result token
    len = sprintf(buf, "%d\n", args_buf.result);
// Current and candidate image version details
    if ((args_buf.result != VALIDATE_TMP_UPDATE) &&
    (args_buf.result < VALIDATE_CUR_UNKNOWN))
    goto out;
    if (args_buf.buf_size > (VALIDATE_BUF_SIZE - len)) {
    memcpy(buf + len, args_buf.buf, VALIDATE_BUF_SIZE - len);
    len = VALIDATE_BUF_SIZE;
    } else {
    memcpy(buf + len, args_buf.buf, args_buf.buf_size);
    len += args_buf.buf_size;
    }
    out:
// Set status to default
    args_buf.status = FLASH_NO_OP;
    return len;
    }
//
// Validate candidate firmware image
//
// Note:
// We are only interested in first 4K bytes of the
// candidate image.
//
    static ssize_t validate_store(struct kobject *kobj,
    struct kobj_attribute *attr,
    const char *buf, size_t count)
    {
    struct validate_flash_t *args_buf = &validate_flash_data;
    if (buf[0] != '1')
    return -EINVAL;
    mutex_lock(&image_data_mutex);
    if (image_data.status != IMAGE_READY ||
    image_data.size < VALIDATE_BUF_SIZE) {
    args_buf.result = VALIDATE_INVALID_IMG;
    args_buf.status = VALIDATE_IMG_INCOMPLETE;
    goto out;
    }
// Copy first 4k bytes of candidate image
    memcpy(args_buf.buf, image_data.data, VALIDATE_BUF_SIZE);
    args_buf.status = VALIDATE_IMG_READY;
    args_buf.buf_size = VALIDATE_BUF_SIZE;
// Validate candidate image
    opal_flash_validate();
    out:
    mutex_unlock(&image_data_mutex);
    return count;
    }
//
// Manage flash routine
//
#[no_mangle]
pub unsafe extern "C" fn opal_flash_manage(op: u8) {
    static inline void opal_flash_manage(uint8_t op)
    {
    let mut args_buf: *mut manage_flash_t const = &manage_flash_data;
    args_buf.status = opal_manage_flash(op);
    }
//
// Show manage flash status
//
    static ssize_t manage_show(struct kobject *kobj,
    struct kobj_attribute *attr, char *buf)
    {
    let mut args_buf: *mut manage_flash_t const = &manage_flash_data;
    int rc;
    rc = sysfs_emit(buf, "%d\n", args_buf.status);
// Set status to default
    args_buf.status = FLASH_NO_OP;
    return rc;
    }
//
// Manage operations:
// 0 - Reject
// 1 - Commit
//
    static ssize_t manage_store(struct kobject *kobj,
    struct kobj_attribute *attr,
    const char *buf, size_t count)
    {
    uint8_t op;
    switch (buf[0]) {
    case '0':
    op = FLASH_REJECT_TMP_SIDE;
    break;
    case '1':
    op = FLASH_COMMIT_TMP_SIDE;
    break;
    default:
    return -EINVAL;
    }
// commit/reject temporary image
    opal_flash_manage(op);
    return count;
    }
//
// OPAL update flash
//
#[no_mangle]
unsafe extern "C" fn opal_flash_update(op: c_int) -> c_int {
    static int opal_flash_update(int op)
    {
    struct opal_sg_list *list;
    unsigned long addr;
    let mut rc: i64 = OPAL_PARAMETER;
    if (op == FLASH_UPDATE_CANCEL) {
    pr_alert("FLASH: Image update cancelled\n");
    addr = '\0';
    goto flash;
    }
    list = opal_vmalloc_to_sg_list(image_data.data, image_data.size);
    if (!list)
    goto invalid_img;
// First entry address
    addr = __pa(list);
    flash:
    rc = opal_update_flash(addr);
    invalid_img:
    return rc;
    }
// This gets called just before system reboots
#[no_mangle]
pub unsafe extern "C" fn opal_flash_update_print_message() {
    void opal_flash_update_print_message(void)
    {
    if (update_flash_data.status != FLASH_IMG_READY)
    return;
    pr_alert("FLASH: Flashing new firmware\n");
    pr_alert("FLASH: Image is %u bytes\n", image_data.size);
    pr_alert("FLASH: Performing flash and reboot/shutdown\n");
    pr_alert("FLASH: This will take several minutes. Do not power off!\n");
// Small delay to help getting the above message out
    msleep(500);
    }
//
// Show candidate image status
//
    static ssize_t update_show(struct kobject *kobj,
    struct kobj_attribute *attr, char *buf)
    {
    let mut args_buf: *mut update_flash_t const = &update_flash_data;
    return sysfs_emit(buf, "%d\n", args_buf.status);
    }
//
// Set update image flag
// 1 - Flash new image
// 0 - Cancel flash request
//
    static ssize_t update_store(struct kobject *kobj,
    struct kobj_attribute *attr,
    const char *buf, size_t count)
    {
    let mut args_buf: *mut update_flash_t const = &update_flash_data;
    let mut rc: c_int = count;
    mutex_lock(&image_data_mutex);
    switch (buf[0]) {
    case '0':
    if (args_buf.status == FLASH_IMG_READY)
    opal_flash_update(FLASH_UPDATE_CANCEL);
    args_buf.status = FLASH_NO_OP;
    break;
    case '1':
// Image is loaded?
    if (image_data.status == IMAGE_READY)
    args_buf.status =
    opal_flash_update(FLASH_UPDATE_INIT);
    else
    args_buf.status = FLASH_INVALID_IMG;
    break;
    default:
    rc = -EINVAL;
    }
    mutex_unlock(&image_data_mutex);
    return rc;
    }
//
// Free image buffer
//
#[no_mangle]
unsafe extern "C" fn free_image_buf() {
    static void free_image_buf(void)
    {
    void *addr;
    int size;
    addr = image_data.data;
    size = PAGE_ALIGN(image_data.size);
    while (size > 0) {
    ClearPageReserved(vmalloc_to_page(addr));
    addr += PAGE_SIZE;
    size -= PAGE_SIZE;
    }
    vfree(image_data.data);
    image_data.data = core::ptr::null_mut();
    image_data.status = IMAGE_INVALID;
    }
//
// Allocate image buffer.
//
#[no_mangle]
unsafe extern "C" fn alloc_image_buf(buffer: *mut c_char, count: usize) -> c_int {
    static int alloc_image_buf(char *buffer, size_t count)
    {
    void *addr;
    int size;
    if (count < sizeof(image_header)) {
    pr_warn("FLASH: Invalid candidate image\n");
    return -EINVAL;
    }
    memcpy(&image_header, (void *)buffer, sizeof(image_header));
    image_data.size = be32_to_cpu(image_header.size);
    pr_debug("FLASH: Candidate image size = %u\n", image_data.size);
    if (image_data.size > MAX_IMAGE_SIZE) {
    pr_warn("FLASH: Too large image\n");
    return -EINVAL;
    }
    if (image_data.size < VALIDATE_BUF_SIZE) {
    pr_warn("FLASH: Image is shorter than expected\n");
    return -EINVAL;
    }
    image_data.data = vzalloc(PAGE_ALIGN(image_data.size));
    if (!image_data.data) {
    pr_err("%s : Failed to allocate memory\n", __func__);
    return -ENOMEM;
    }
// Pin memory
    addr = image_data.data;
    size = PAGE_ALIGN(image_data.size);
    while (size > 0) {
    SetPageReserved(vmalloc_to_page(addr));
    addr += PAGE_SIZE;
    size -= PAGE_SIZE;
    }
    image_data.status = IMAGE_LOADING;
    return 0;
    }
//
// Copy candidate image
//
// Parse candidate image header to get total image size
// and pre-allocate required memory.
//
    static ssize_t image_data_write(struct file *filp, struct kobject *kobj,
    const struct bin_attribute *bin_attr,
    char *buffer, loff_t pos, size_t count)
    {
    int rc;
    mutex_lock(&image_data_mutex);
// New image ?
    if (pos == 0) {
// Free memory, if already allocated
    if (image_data.data)
    free_image_buf();
// Cancel outstanding image update request
    if (update_flash_data.status == FLASH_IMG_READY)
    opal_flash_update(FLASH_UPDATE_CANCEL);
// Allocate memory
    rc = alloc_image_buf(buffer, count);
    if (rc)
    goto out;
    }
    if (image_data.status != IMAGE_LOADING) {
    rc = -ENOMEM;
    goto out;
    }
    if ((pos + count) > image_data.size) {
    rc = -EINVAL;
    goto out;
    }
    memcpy(image_data.data + pos, (void *)buffer, count);
    rc = count;
// Set image status
    if ((pos + count) == image_data.size) {
    pr_debug("FLASH: Candidate image loaded....\n");
    image_data.status = IMAGE_READY;
    }
    out:
    mutex_unlock(&image_data_mutex);
    return rc;
    }
//
// sysfs interface :
// OPAL uses below sysfs files for code update.
// We create these files under /sys/firmware/opal.
//
// image		: Interface to load candidate firmware image
// validate_flash	: Validate firmware image
// manage_flash	: Commit/Reject firmware image
// update_flash	: Flash new firmware image
//
    static const struct bin_attribute image_data_attr = {
    .attr = {.name = "image", .mode = 0200},
    .size = MAX_IMAGE_SIZE,	/* Limit image size */
    .write = image_data_write,
    };
    static struct kobj_attribute validate_attribute =
    __ATTR(validate_flash, 0600, validate_show, validate_store);
    static struct kobj_attribute manage_attribute =
    __ATTR(manage_flash, 0600, manage_show, manage_store);
    static struct kobj_attribute update_attribute =
    __ATTR(update_flash, 0600, update_show, update_store);
    static struct attribute *image_op_attrs[] = {
    &validate_attribute.attr,
    &manage_attribute.attr,
    &update_attribute.attr,
    core::ptr::null_mut()	/* need to core::ptr::null_mut() terminate the list of attributes */
    };
    static const struct attribute_group image_op_attr_group = {
    .attrs = image_op_attrs,
    };
#[no_mangle]
pub unsafe extern "C" fn opal_flash_update_init() -> void __init {
    void __init opal_flash_update_init(void)
    {
    int ret;
// Firmware update is not supported by firmware
    if (!opal_check_token(OPAL_FLASH_VALIDATE))
    return;
// Allocate validate image buffer
    validate_flash_data.buf = kzalloc(VALIDATE_BUF_SIZE, GFP_KERNEL);
    if (!validate_flash_data.buf) {
    pr_err("%s : Failed to allocate memory\n", __func__);
    return;
    }
// Make sure /sys/firmware/opal directory is created
    if (!opal_kobj) {
    pr_warn("FLASH: opal kobject is not available\n");
    goto nokobj;
    }
// Create the sysfs files
    ret = sysfs_create_group(opal_kobj, &image_op_attr_group);
    if (ret) {
    pr_warn("FLASH: Failed to create sysfs files\n");
    goto nokobj;
    }
    ret = sysfs_create_bin_file(opal_kobj, &image_data_attr);
    if (ret) {
    pr_warn("FLASH: Failed to create sysfs files\n");
    goto nosysfs_file;
    }
// Set default status
    validate_flash_data.status = FLASH_NO_OP;
    manage_flash_data.status = FLASH_NO_OP;
    update_flash_data.status = FLASH_NO_OP;
    image_data.status = IMAGE_INVALID;
    return;
    nosysfs_file:
    sysfs_remove_group(opal_kobj, &image_op_attr_group);
    nokobj:
    kfree(validate_flash_data.buf);
    return;
    }
