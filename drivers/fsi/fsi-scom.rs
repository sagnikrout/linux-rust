//! Automatically rewritten from C to Rust
//! Source: drivers/fsi/fsi-scom.c
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
// SCOM FSI Client device driver
//
// Copyright (C) IBM Corporation 2016
//

pub const FSI_ENGID_SCOM: c_uint = 0x5;
// SCOM engine register set
pub const SCOM_DATA0_REG: c_uint = 0x00;
pub const SCOM_DATA1_REG: c_uint = 0x04;
pub const SCOM_CMD_REG: c_uint = 0x08;
pub const SCOM_FSI2PIB_RESET_REG: c_uint = 0x18;
pub const SCOM_STATUS_REG: c_uint = 0x1C /* Read */;
pub const SCOM_PIB_RESET_REG: c_uint = 0x1C /* Write */;
// Command register
pub const SCOM_WRITE_CMD: c_uint = 0x80000000;
pub const SCOM_READ_CMD: c_uint = 0x00000000;
// Status register bits
pub const SCOM_STATUS_ERR_SUMMARY: c_uint = 0x80000000;
pub const SCOM_STATUS_PROTECTION: c_uint = 0x01000000;
pub const SCOM_STATUS_PARITY: c_uint = 0x04000000;
pub const SCOM_STATUS_PIB_ABORT: c_uint = 0x00100000;
pub const SCOM_STATUS_PIB_RESP_MASK: c_uint = 0x00007000;
pub const SCOM_STATUS_PIB_RESP_SHIFT: c_int = 12;

    SCOM_STATUS_PARITY |		\
    SCOM_STATUS_PIB_ABORT)

    SCOM_STATUS_PIB_RESP_MASK)
// SCOM address encodings

// SCOM indirect stuff
pub const XSCOM_ADDR_DIRECT_PART: c_uint = 0x7fffffffull;
pub const XSCOM_ADDR_INDIRECT_PART: c_uint = 0x000fffff00000000ull;

pub const XSCOM_DATA_IND_ERR_MASK: c_uint = 0x70000000ull;
pub const XSCOM_DATA_IND_ERR_SHIFT: c_int = 28;
pub const XSCOM_DATA_IND_DATA: c_uint = 0x0000ffffull;
pub const XSCOM_DATA_IND_FORM1_DATA: c_uint = 0x000fffffffffffffull;
pub const XSCOM_ADDR_FORM1_LOW: c_uint = 0x000ffffffffull;
pub const XSCOM_ADDR_FORM1_HI: c_uint = 0xfff00000000ull;
pub const XSCOM_ADDR_FORM1_HI_SHIFT: c_int = 20;
// Retries

#[repr(C)]
#[derive(Copy, Clone)]
pub struct scom_device {
    pub link: list_head,
    pub fsi_dev: *mut fsi_device,
    pub dev: device,
    pub cdev: cdev,
    pub lock: mutex,
    pub dead: bool,
}

    static int __put_scom(struct scom_device *scom_dev, uint64_t value,
    uint32_t addr, uint32_t *status)
    {
    __be32 data, raw_status;
    int rc;
    data = cpu_to_be32((value >> 32) & 0xffffffff);
    rc = fsi_device_write(scom_dev.fsi_dev, SCOM_DATA0_REG, &data,
    sizeof(uint32_t));
    if (rc)
    return rc;
    data = cpu_to_be32(value & 0xffffffff);
    rc = fsi_device_write(scom_dev.fsi_dev, SCOM_DATA1_REG, &data,
    sizeof(uint32_t));
    if (rc)
    return rc;
    data = cpu_to_be32(SCOM_WRITE_CMD | addr);
    rc = fsi_device_write(scom_dev.fsi_dev, SCOM_CMD_REG, &data,
    sizeof(uint32_t));
    if (rc)
    return rc;
    rc = fsi_device_read(scom_dev.fsi_dev, SCOM_STATUS_REG, &raw_status,
    sizeof(uint32_t));
    if (rc)
    return rc;
// status = be32_to_cpu(raw_status);
    return 0;
    }
    static int __get_scom(struct scom_device *scom_dev, uint64_t *value,
    uint32_t addr, uint32_t *status)
    {
    __be32 data, raw_status;
    int rc;
// value = 0ULL;
    data = cpu_to_be32(SCOM_READ_CMD | addr);
    rc = fsi_device_write(scom_dev.fsi_dev, SCOM_CMD_REG, &data,
    sizeof(uint32_t));
    if (rc)
    return rc;
    rc = fsi_device_read(scom_dev.fsi_dev, SCOM_STATUS_REG, &raw_status,
    sizeof(uint32_t));
    if (rc)
    return rc;
//
// Read the data registers even on error, so we don't have
// to interpret the status register here.
//
    rc = fsi_device_read(scom_dev.fsi_dev, SCOM_DATA0_REG, &data,
    sizeof(uint32_t));
    if (rc)
    return rc;
// value |= (uint64_t)be32_to_cpu(data) << 32;
    rc = fsi_device_read(scom_dev.fsi_dev, SCOM_DATA1_REG, &data,
    sizeof(uint32_t));
    if (rc)
    return rc;
// value |= be32_to_cpu(data);
// status = be32_to_cpu(raw_status);
    return rc;
    }
    static int put_indirect_scom_form0(struct scom_device *scom, uint64_t value,
    uint64_t addr, uint32_t *status)
    {
    uint64_t ind_data, ind_addr;
    int rc, err;
    if (value & ~XSCOM_DATA_IND_DATA)
    return -EINVAL;
    ind_addr = addr & XSCOM_ADDR_DIRECT_PART;
    ind_data = (addr & XSCOM_ADDR_INDIRECT_PART) | value;
    rc = __put_scom(scom, ind_data, ind_addr, status);
    if (rc || (*status & SCOM_STATUS_ANY_ERR))
    return rc;
    rc = __get_scom(scom, &ind_data, addr, status);
    if (rc || (*status & SCOM_STATUS_ANY_ERR))
    return rc;
    err = (ind_data & XSCOM_DATA_IND_ERR_MASK) >> XSCOM_DATA_IND_ERR_SHIFT;
// status = err << SCOM_STATUS_PIB_RESP_SHIFT;
    return 0;
    }
    static int put_indirect_scom_form1(struct scom_device *scom, uint64_t value,
    uint64_t addr, uint32_t *status)
    {
    uint64_t ind_data, ind_addr;
    if (value & ~XSCOM_DATA_IND_FORM1_DATA)
    return -EINVAL;
    ind_addr = addr & XSCOM_ADDR_FORM1_LOW;
    ind_data = value | (addr & XSCOM_ADDR_FORM1_HI) << XSCOM_ADDR_FORM1_HI_SHIFT;
    return __put_scom(scom, ind_data, ind_addr, status);
    }
    static int get_indirect_scom_form0(struct scom_device *scom, uint64_t *value,
    uint64_t addr, uint32_t *status)
    {
    uint64_t ind_data, ind_addr;
    int rc, err;
    ind_addr = addr & XSCOM_ADDR_DIRECT_PART;
    ind_data = (addr & XSCOM_ADDR_INDIRECT_PART) | XSCOM_DATA_IND_READ;
    rc = __put_scom(scom, ind_data, ind_addr, status);
    if (rc || (*status & SCOM_STATUS_ANY_ERR))
    return rc;
    rc = __get_scom(scom, &ind_data, addr, status);
    if (rc || (*status & SCOM_STATUS_ANY_ERR))
    return rc;
    err = (ind_data & XSCOM_DATA_IND_ERR_MASK) >> XSCOM_DATA_IND_ERR_SHIFT;
// status = err << SCOM_STATUS_PIB_RESP_SHIFT;
// value = ind_data & XSCOM_DATA_IND_DATA;
    return 0;
    }
    static int raw_put_scom(struct scom_device *scom, uint64_t value,
    uint64_t addr, uint32_t *status)
    {
    if (addr & XSCOM_ADDR_IND_FLAG) {
    if (addr & XSCOM_ADDR_INF_FORM1)
    return put_indirect_scom_form1(scom, value, addr, status);
    else
    return put_indirect_scom_form0(scom, value, addr, status);
    } else
    return __put_scom(scom, value, addr, status);
    }
    static int raw_get_scom(struct scom_device *scom, uint64_t *value,
    uint64_t addr, uint32_t *status)
    {
    if (addr & XSCOM_ADDR_IND_FLAG) {
    if (addr & XSCOM_ADDR_INF_FORM1)
    return -ENXIO;
    return get_indirect_scom_form0(scom, value, addr, status);
    } else
    return __get_scom(scom, value, addr, status);
    }
#[no_mangle]
unsafe extern "C" fn handle_fsi2pib_status(scom: *mut scom_device, status: u32) -> c_int {
    static int handle_fsi2pib_status(struct scom_device *scom, uint32_t status)
    {
    let mut dummy: u32 = -1;
    if (status & SCOM_STATUS_FSI2PIB_ERROR)
    fsi_device_write(scom.fsi_dev, SCOM_FSI2PIB_RESET_REG, &dummy,
    sizeof(uint32_t));
    if (status & SCOM_STATUS_PROTECTION)
    return -EPERM;
    if (status & SCOM_STATUS_PARITY)
    return -EIO;
    if (status & SCOM_STATUS_PIB_ABORT)
    return -EBUSY;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn handle_pib_status(scom: *mut scom_device, status: u8) -> c_int {
    static int handle_pib_status(struct scom_device *scom, uint8_t status)
    {
    let mut dummy: u32 = -1;
    if (status == SCOM_PIB_SUCCESS)
    return 0;
    if (status == SCOM_PIB_BLOCKED)
    return -EBUSY;
// Reset the bridge
    fsi_device_write(scom.fsi_dev, SCOM_FSI2PIB_RESET_REG, &dummy,
    sizeof(uint32_t));
    switch(status) {
    case SCOM_PIB_OFFLINE:
    return -ENODEV;
    case SCOM_PIB_BAD_ADDR:
    return -ENXIO;
    case SCOM_PIB_TIMEOUT:
    return -ETIMEDOUT;
    case SCOM_PIB_PARTIAL:
    case SCOM_PIB_CLK_ERR:
    case SCOM_PIB_PARITY_ERR:
    default:
    return -EIO;
    }
    }
    static int put_scom(struct scom_device *scom, uint64_t value,
    uint64_t addr)
    {
    uint32_t status;
    int rc;
    rc = raw_put_scom(scom, value, addr, &status);
    if (rc)
    return rc;
    rc = handle_fsi2pib_status(scom, status);
    if (rc)
    return rc;
    return handle_pib_status(scom,
    (status & SCOM_STATUS_PIB_RESP_MASK)
    >> SCOM_STATUS_PIB_RESP_SHIFT);
    }
    static int get_scom(struct scom_device *scom, uint64_t *value,
    uint64_t addr)
    {
    uint32_t status;
    int rc;
    rc = raw_get_scom(scom, value, addr, &status);
    if (rc)
    return rc;
    rc = handle_fsi2pib_status(scom, status);
    if (rc)
    return rc;
    return handle_pib_status(scom,
    (status & SCOM_STATUS_PIB_RESP_MASK)
    >> SCOM_STATUS_PIB_RESP_SHIFT);
    }
    static ssize_t scom_read(struct file *filep, char __user *buf, size_t len,
    loff_t *offset)
    {
    struct scom_device *scom = filep.private_data;
    struct device *dev = &scom.fsi_dev.dev;
    uint64_t val;
    int rc;
    if (len != sizeof(uint64_t))
    return -EINVAL;
    mutex_lock(&scom.lock);
    if (scom.dead)
    rc = -ENODEV;
    else
    rc = get_scom(scom, &val, *offset);
    mutex_unlock(&scom.lock);
    if (rc) {
    dev_dbg(dev, "get_scom fail:%d\n", rc);
    return rc;
    }
    rc = copy_to_user(buf, &val, len);
    if (rc)
    dev_dbg(dev, "copy to user failed:%d\n", rc);
    return rc ? rc : len;
    }
    static ssize_t scom_write(struct file *filep, const char __user *buf,
    size_t len, loff_t *offset)
    {
    int rc;
    struct scom_device *scom = filep.private_data;
    struct device *dev = &scom.fsi_dev.dev;
    uint64_t val;
    if (len != sizeof(uint64_t))
    return -EINVAL;
    rc = copy_from_user(&val, buf, len);
    if (rc) {
    dev_dbg(dev, "copy from user failed:%d\n", rc);
    return -EINVAL;
    }
    mutex_lock(&scom.lock);
    if (scom.dead)
    rc = -ENODEV;
    else
    rc = put_scom(scom, val, *offset);
    mutex_unlock(&scom.lock);
    if (rc) {
    dev_dbg(dev, "put_scom failed with:%d\n", rc);
    return rc;
    }
    return len;
    }
#[no_mangle]
unsafe extern "C" fn scom_llseek(file: *mut file, offset: loff_t, whence: c_int) -> loff_t {
    static loff_t scom_llseek(struct file *file, loff_t offset, int whence)
    {
    switch (whence) {
    case SEEK_CUR:
    break;
    case SEEK_SET:
    file.f_pos = offset;
    break;
    default:
    return -EINVAL;
    }
    return offset;
    }
#[no_mangle]
unsafe extern "C" fn raw_convert_status(acc: *mut scom_access, status: u32) {
    static void raw_convert_status(struct scom_access *acc, uint32_t status)
    {
    acc.pib_status = (status & SCOM_STATUS_PIB_RESP_MASK) >>
    SCOM_STATUS_PIB_RESP_SHIFT;
    acc.intf_errors = 0;
    if (status & SCOM_STATUS_PROTECTION)
    acc.intf_errors |= SCOM_INTF_ERR_PROTECTION;
#[no_mangle]
pub unsafe extern "C" fn if(SCOM_STATUS_PARITY: status &) -> else {
    else if (status & SCOM_STATUS_PARITY)
    acc.intf_errors |= SCOM_INTF_ERR_PARITY;
#[no_mangle]
pub unsafe extern "C" fn if(SCOM_STATUS_PIB_ABORT: status &) -> else {
    else if (status & SCOM_STATUS_PIB_ABORT)
    acc.intf_errors |= SCOM_INTF_ERR_ABORT;
#[no_mangle]
pub unsafe extern "C" fn if(SCOM_STATUS_ERR_SUMMARY: status &) -> else {
    else if (status & SCOM_STATUS_ERR_SUMMARY)
    acc.intf_errors |= SCOM_INTF_ERR_UNKNOWN;
    }
#[no_mangle]
unsafe extern "C" fn scom_raw_read(scom: *mut scom_device, argp: *mut void __user) -> c_int {
    static int scom_raw_read(struct scom_device *scom, void __user *argp)
    {
    struct scom_access acc;
    uint32_t status;
    int rc;
    if (copy_from_user(&acc, argp, sizeof(struct scom_access)))
    return -EFAULT;
    rc = raw_get_scom(scom, &acc.data, acc.addr, &status);
    if (rc)
    return rc;
    raw_convert_status(&acc, status);
    if (copy_to_user(argp, &acc, sizeof(struct scom_access)))
    return -EFAULT;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn scom_raw_write(scom: *mut scom_device, argp: *mut void __user) -> c_int {
    static int scom_raw_write(struct scom_device *scom, void __user *argp)
    {
    u64 prev_data, mask, data;
    struct scom_access acc;
    uint32_t status;
    int rc;
    if (copy_from_user(&acc, argp, sizeof(struct scom_access)))
    return -EFAULT;
    if (acc.mask) {
    rc = raw_get_scom(scom, &prev_data, acc.addr, &status);
    if (rc)
    return rc;
    if (status & SCOM_STATUS_ANY_ERR)
    goto fail;
    mask = acc.mask;
    } else {
    prev_data = mask = -1ull;
    }
    data = (prev_data & ~mask) | (acc.data & mask);
    rc = raw_put_scom(scom, data, acc.addr, &status);
    if (rc)
    return rc;
    fail:
    raw_convert_status(&acc, status);
    if (copy_to_user(argp, &acc, sizeof(struct scom_access)))
    return -EFAULT;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn scom_reset(scom: *mut scom_device, argp: *mut void __user) -> c_int {
    static int scom_reset(struct scom_device *scom, void __user *argp)
    {
    uint32_t flags, dummy = -1;
    let mut rc: c_int = 0;
    if (get_user(flags, (__u32 __user *)argp))
    return -EFAULT;
    if (flags & SCOM_RESET_PIB)
    rc = fsi_device_write(scom.fsi_dev, SCOM_PIB_RESET_REG, &dummy,
    sizeof(uint32_t));
    if (!rc && (flags & (SCOM_RESET_PIB | SCOM_RESET_INTF)))
    rc = fsi_device_write(scom.fsi_dev, SCOM_FSI2PIB_RESET_REG, &dummy,
    sizeof(uint32_t));
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn scom_check(scom: *mut scom_device, argp: *mut void __user) -> c_int {
    static int scom_check(struct scom_device *scom, void __user *argp)
    {
// Still need to find out how to get "protected"
    return put_user(SCOM_CHECK_SUPPORTED, (__u32 __user *)argp);
    }
#[no_mangle]
unsafe extern "C" fn scom_ioctl(file: *mut file, cmd: c_uint, arg: c_ulong) -> c_long {
    static long scom_ioctl(struct file *file, unsigned int cmd, unsigned long arg)
    {
    struct scom_device *scom = file.private_data;
    void __user *argp = (void __user *)arg;
    let mut rc: c_int = -ENOTTY;
    mutex_lock(&scom.lock);
    if (scom.dead) {
    mutex_unlock(&scom.lock);
    return -ENODEV;
    }
    switch(cmd) {
    case FSI_SCOM_CHECK:
    rc = scom_check(scom, argp);
    break;
    case FSI_SCOM_READ:
    rc = scom_raw_read(scom, argp);
    break;
    case FSI_SCOM_WRITE:
    rc = scom_raw_write(scom, argp);
    break;
    case FSI_SCOM_RESET:
    rc = scom_reset(scom, argp);
    break;
    }
    mutex_unlock(&scom.lock);
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn scom_open(inode: *mut inode, file: *mut file) -> c_int {
    static int scom_open(struct inode *inode, struct file *file)
    {
    struct scom_device *scom = container_of(inode.i_cdev, struct scom_device, cdev);
    file.private_data = scom;
    return 0;
    }
    static const struct file_operations scom_fops = {
    .owner		= THIS_MODULE,
    .open		= scom_open,
    .llseek		= scom_llseek,
    .read		= scom_read,
    .write		= scom_write,
    .unlocked_ioctl	= scom_ioctl,
    };
#[no_mangle]
unsafe extern "C" fn scom_free(dev: *mut device) {
    static void scom_free(struct device *dev)
    {
    struct scom_device *scom = container_of(dev, struct scom_device, dev);
    put_device(&scom.fsi_dev.dev);
    kfree(scom);
    }
#[no_mangle]
unsafe extern "C" fn scom_probe(fsi_dev: *mut fsi_device) -> c_int {
    static int scom_probe(struct fsi_device *fsi_dev)
    {
    struct device *dev = &fsi_dev.dev;
    struct scom_device *scom;
    int rc, didx;
    scom = kzalloc_obj(*scom);
    if (!scom)
    return -ENOMEM;
    fsi_set_drvdata(fsi_dev, scom);
    mutex_init(&scom.lock);
// Grab a reference to the device (parent of our cdev), we'll drop it later
    if (!get_device(dev)) {
    kfree(scom);
    return -ENODEV;
    }
    scom.fsi_dev = fsi_dev;
// Create chardev for userspace access
    scom.dev.type = &fsi_cdev_type;
    scom.dev.parent = dev;
    scom.dev.release = scom_free;
    device_initialize(&scom.dev);
// Allocate a minor in the FSI space
    rc = fsi_get_new_minor(fsi_dev, fsi_dev_scom, &scom.dev.devt, &didx);
    if (rc)
    goto err;
    dev_set_name(&scom.dev, "scom%d", didx);
    cdev_init(&scom.cdev, &scom_fops);
    rc = cdev_device_add(&scom.cdev, &scom.dev);
    if (rc) {
    dev_err(dev, "Error %d creating char device %s\n",
    rc, dev_name(&scom.dev));
    goto err_free_minor;
    }
    return 0;
    err_free_minor:
    fsi_free_minor(scom.dev.devt);
    err:
    put_device(&scom.dev);
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn scom_remove(fsi_dev: *mut fsi_device) {
    static void scom_remove(struct fsi_device *fsi_dev)
    {
    struct scom_device *scom = fsi_get_drvdata(fsi_dev);
    mutex_lock(&scom.lock);
    scom.dead = true;
    mutex_unlock(&scom.lock);
    cdev_device_del(&scom.cdev, &scom.dev);
    fsi_free_minor(scom.dev.devt);
    put_device(&scom.dev);
    }
    static const struct of_device_id scom_of_ids[] = {
    { .compatible = "ibm,fsi2pib" },
    { }
    };
    MODULE_DEVICE_TABLE(of, scom_of_ids);
    static const struct fsi_device_id scom_ids[] = {
    {
    .engine_type = FSI_ENGID_SCOM,
    .version = FSI_VERSION_ANY,
    },
    { 0 }
    };
    static struct fsi_driver scom_drv = {
    .id_table = scom_ids,
    .probe = scom_probe,
    .remove = scom_remove,
    .drv = {
    .name = "scom",
    .of_match_table = scom_of_ids,
    }
    };
    module_fsi_driver(scom_drv);
    MODULE_DESCRIPTION("SCOM FSI Client device driver");
    MODULE_LICENSE("GPL");
