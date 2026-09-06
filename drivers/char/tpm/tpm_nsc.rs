//! Automatically rewritten from C to Rust
//! Source: drivers/char/tpm/tpm_nsc.c
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
// Copyright (C) 2004 IBM Corporation
//
// Authors:
// Leendert van Doorn <leendert@watson.ibm.com>
// Dave Safford <safford@watson.ibm.com>
// Reiner Sailer <sailer@watson.ibm.com>
// Kylene Hall <kjhall@us.ibm.com>
//
// Maintained by: <tpmdd-devel@lists.sourceforge.net>
//
// Device driver for TCG/TCPA TPM (trusted platform module).
// Specifications at www.trustedcomputinggroup.org
//

// National definitions
    enum tpm_nsc_addr{
    TPM_NSC_IRQ = 0x07,
    TPM_NSC_BASE0_HI = 0x60,
    TPM_NSC_BASE0_LO = 0x61,
    TPM_NSC_BASE1_HI = 0x62,
    TPM_NSC_BASE1_LO = 0x63
    };
    enum tpm_nsc_index {
    NSC_LDN_INDEX = 0x07,
    NSC_SID_INDEX = 0x20,
    NSC_LDC_INDEX = 0x30,
    NSC_DIO_INDEX = 0x60,
    NSC_CIO_INDEX = 0x62,
    NSC_IRQ_INDEX = 0x70,
    NSC_ITS_INDEX = 0x71
    };
    enum tpm_nsc_status_loc {
    NSC_STATUS = 0x01,
    NSC_COMMAND = 0x01,
    NSC_DATA = 0x00
    };
// status bits
    enum tpm_nsc_status {
    NSC_STATUS_OBF = 0x01,	/* output buffer full */
    NSC_STATUS_IBF = 0x02,	/* input buffer full */
    NSC_STATUS_F0 = 0x04,	/* F0 */
    NSC_STATUS_A2 = 0x08,	/* A2 */
    NSC_STATUS_RDY = 0x10,	/* ready to receive command */
    NSC_STATUS_IBR = 0x20	/* ready to receive data */
    };
// command bits
    enum tpm_nsc_cmd_mode {
    NSC_COMMAND_NORMAL = 0x01,	/* normal mode */
    NSC_COMMAND_EOC = 0x03,
    NSC_COMMAND_CANCEL = 0x22
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tpm_nsc_priv {
    pub base: c_ulong,
}

//
// Wait for a certain status to appear
//
#[no_mangle]
unsafe extern "C" fn wait_for_stat(chip: *mut tpm_chip, mask: u8, val: u8, data: *mut *mut u8) -> c_int {
    static int wait_for_stat(struct tpm_chip *chip, u8 mask, u8 val, u8 * data)
    {
    struct tpm_nsc_priv *priv = dev_get_drvdata(&chip.dev);
    unsigned long stop;
// status immediately available check
// data = inb(priv->base + NSC_STATUS);
    if ((*data & mask) == val)
    return 0;
// wait for status
    stop = jiffies + 10 * HZ;
    do {
    msleep(TPM_TIMEOUT);
// data = inb(priv->base + 1);
    if ((*data & mask) == val)
    return 0;
    }
    while (time_before(jiffies, stop));
    return -EBUSY;
    }
#[no_mangle]
unsafe extern "C" fn nsc_wait_for_ready(chip: *mut tpm_chip) -> c_int {
    static int nsc_wait_for_ready(struct tpm_chip *chip)
    {
    struct tpm_nsc_priv *priv = dev_get_drvdata(&chip.dev);
    int status;
    unsigned long stop;
// status immediately available check
    status = inb(priv.base + NSC_STATUS);
    if (status & NSC_STATUS_OBF)
    status = inb(priv.base + NSC_DATA);
    if (status & NSC_STATUS_RDY)
    return 0;
// wait for status
    stop = jiffies + 100;
    do {
    msleep(TPM_TIMEOUT);
    status = inb(priv.base + NSC_STATUS);
    if (status & NSC_STATUS_OBF)
    status = inb(priv.base + NSC_DATA);
    if (status & NSC_STATUS_RDY)
    return 0;
    }
    while (time_before(jiffies, stop));
    dev_info(&chip.dev, "wait for ready failed\n");
    return -EBUSY;
    }
#[no_mangle]
unsafe extern "C" fn tpm_nsc_recv(chip: *mut tpm_chip, buf: *mut *mut u8, count: usize) -> c_int {
    static int tpm_nsc_recv(struct tpm_chip *chip, u8 * buf, size_t count)
    {
    struct tpm_nsc_priv *priv = dev_get_drvdata(&chip.dev);
    u8 *buffer = buf;
    u8 data, *p;
    u32 size;
    __be32 *native_size;
    if (count < 6)
    return -EIO;
    if (wait_for_stat(chip, NSC_STATUS_F0, NSC_STATUS_F0, &data) < 0) {
    dev_err(&chip.dev, "F0 timeout\n");
    return -EIO;
    }
    data = inb(priv.base + NSC_DATA);
    if (data != NSC_COMMAND_NORMAL) {
    dev_err(&chip.dev, "not in normal mode (0x%x)\n",
    data);
    return -EIO;
    }
// read the whole packet
    for (p = buffer; p < &buffer[count]; p++) {
    if (wait_for_stat
    (chip, NSC_STATUS_OBF, NSC_STATUS_OBF, &data) < 0) {
    dev_err(&chip.dev,
    "OBF timeout (while reading data)\n");
    return -EIO;
    }
    if (data & NSC_STATUS_F0)
    break;
// p = inb(priv->base + NSC_DATA);
    }
    if ((data & NSC_STATUS_F0) == 0 &&
    (wait_for_stat(chip, NSC_STATUS_F0, NSC_STATUS_F0, &data) < 0)) {
    dev_err(&chip.dev, "F0 not set\n");
    return -EIO;
    }
    data = inb(priv.base + NSC_DATA);
    if (data != NSC_COMMAND_EOC) {
    dev_err(&chip.dev,
    "expected end of command(0x%x)\n", data);
    return -EIO;
    }
    native_size = ( __be32 *) (buf + 2);
    size = be32_to_cpu(*native_size);
    if (count < size)
    return -EIO;
    return size;
    }
    static int tpm_nsc_send(struct tpm_chip *chip, u8 *buf, size_t bufsiz,
    size_t count)
    {
    struct tpm_nsc_priv *priv = dev_get_drvdata(&chip.dev);
    u8 data;
    int i;
//
// If we hit the chip with back to back commands it locks up
// and never set IBF. Hitting it with this "hammer" seems to
// fix it. Not sure why this is needed, we followed the flow
// chart in the manual to the letter.
//
    outb(NSC_COMMAND_CANCEL, priv.base + NSC_COMMAND);
    if (nsc_wait_for_ready(chip) != 0)
    return -EIO;
    if (wait_for_stat(chip, NSC_STATUS_IBF, 0, &data) < 0) {
    dev_err(&chip.dev, "IBF timeout\n");
    return -EIO;
    }
    outb(NSC_COMMAND_NORMAL, priv.base + NSC_COMMAND);
    if (wait_for_stat(chip, NSC_STATUS_IBR, NSC_STATUS_IBR, &data) < 0) {
    dev_err(&chip.dev, "IBR timeout\n");
    return -EIO;
    }
    for (i = 0; i < count; i++) {
    if (wait_for_stat(chip, NSC_STATUS_IBF, 0, &data) < 0) {
    dev_err(&chip.dev,
    "IBF timeout (while writing data)\n");
    return -EIO;
    }
    outb(buf[i], priv.base + NSC_DATA);
    }
    if (wait_for_stat(chip, NSC_STATUS_IBF, 0, &data) < 0) {
    dev_err(&chip.dev, "IBF timeout\n");
    return -EIO;
    }
    outb(NSC_COMMAND_EOC, priv.base + NSC_COMMAND);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tpm_nsc_cancel(chip: *mut tpm_chip) {
    static void tpm_nsc_cancel(struct tpm_chip *chip)
    {
    struct tpm_nsc_priv *priv = dev_get_drvdata(&chip.dev);
    outb(NSC_COMMAND_CANCEL, priv.base + NSC_COMMAND);
    }
#[no_mangle]
unsafe extern "C" fn tpm_nsc_status(chip: *mut tpm_chip) -> u8 {
    static u8 tpm_nsc_status(struct tpm_chip *chip)
    {
    struct tpm_nsc_priv *priv = dev_get_drvdata(&chip.dev);
    return inb(priv.base + NSC_STATUS);
    }
#[no_mangle]
unsafe extern "C" fn tpm_nsc_req_canceled(chip: *mut tpm_chip, status: u8) -> bool {
    static bool tpm_nsc_req_canceled(struct tpm_chip *chip, u8 status)
    {
    return (status == NSC_STATUS_RDY);
    }
    static const struct tpm_class_ops tpm_nsc = {
    .recv = tpm_nsc_recv,
    .send = tpm_nsc_send,
    .cancel = tpm_nsc_cancel,
    .status = tpm_nsc_status,
    .req_complete_mask = NSC_STATUS_OBF,
    .req_complete_val = NSC_STATUS_OBF,
    .req_canceled = tpm_nsc_req_canceled,
    };
    static struct platform_device *pdev = core::ptr::null_mut();
#[no_mangle]
unsafe extern "C" fn tpm_nsc_remove(dev: *mut device) {
    static void tpm_nsc_remove(struct device *dev)
    {
    struct tpm_chip *chip = dev_get_drvdata(dev);
    struct tpm_nsc_priv *priv = dev_get_drvdata(&chip.dev);
    tpm_chip_unregister(chip);
    release_region(priv.base, 2);
    }
    static SIMPLE_DEV_PM_OPS(tpm_nsc_pm, tpm_pm_suspend, tpm_pm_resume);
    static struct platform_driver nsc_drv = {
    .driver          = {
    .name    = "tpm_nsc",
    .pm      = &tpm_nsc_pm,
    },
    };
#[no_mangle]
pub unsafe extern "C" fn tpm_read_index(base: c_int, index: c_int) -> c_int {
    static inline int tpm_read_index(int base, int index)
    {
    outb(index, base);
    return inb(base+1) & 0xFF;
    }
#[no_mangle]
pub unsafe extern "C" fn tpm_write_index(base: c_int, index: c_int, value: c_int) {
    static inline void tpm_write_index(int base, int index, int value)
    {
    outb(index, base);
    outb(value & 0xFF, base+1);
    }
#[no_mangle]
unsafe extern "C" fn init_nsc() -> int __init {
    static int __init init_nsc(void)
    {
    let mut rc: c_int = 0;
    int lo, hi, err;
    let mut nscAddrBase: c_int = TPM_ADDR;
    struct tpm_chip *chip;
    unsigned long base;
    struct tpm_nsc_priv *priv;
// verify that it is a National part (SID)
    if (tpm_read_index(TPM_ADDR, NSC_SID_INDEX) != 0xEF) {
    nscAddrBase = (tpm_read_index(TPM_SUPERIO_ADDR, 0x2C)<<8)|
    (tpm_read_index(TPM_SUPERIO_ADDR, 0x2B)&0xFE);
    if (tpm_read_index(nscAddrBase, NSC_SID_INDEX) != 0xF6)
    return -ENODEV;
    }
    err = platform_driver_register(&nsc_drv);
    if (err)
    return err;
    hi = tpm_read_index(nscAddrBase, TPM_NSC_BASE0_HI);
    lo = tpm_read_index(nscAddrBase, TPM_NSC_BASE0_LO);
    base = (hi<<8) | lo;
// enable the DPM module
    tpm_write_index(nscAddrBase, NSC_LDC_INDEX, 0x01);
    pdev = platform_device_alloc("tpm_nscl0", -1);
    if (!pdev) {
    rc = -ENOMEM;
    goto err_unreg_drv;
    }
    pdev.num_resources = 0;
    pdev.dev.driver = &nsc_drv.driver;
    pdev.dev.release = tpm_nsc_remove;
    if ((rc = platform_device_add(pdev)) < 0)
    goto err_put_dev;
    priv = devm_kzalloc(&pdev.dev, sizeof(*priv), GFP_KERNEL);
    if (!priv) {
    rc = -ENOMEM;
    goto err_del_dev;
    }
    priv.base = base;
    if (request_region(base, 2, "tpm_nsc0") == core::ptr::null_mut() ) {
    rc = -EBUSY;
    goto err_del_dev;
    }
    chip = tpmm_chip_alloc(&pdev.dev, &tpm_nsc);
    if (IS_ERR(chip)) {
    rc = -ENODEV;
    goto err_rel_reg;
    }
    dev_set_drvdata(&chip.dev, priv);
    rc = tpm_chip_register(chip);
    if (rc)
    goto err_rel_reg;
    dev_dbg(&pdev.dev, "NSC TPM detected\n");
    dev_dbg(&pdev.dev,
    "NSC LDN 0x%x, SID 0x%x, SRID 0x%x\n",
    tpm_read_index(nscAddrBase,0x07), tpm_read_index(nscAddrBase,0x20),
    tpm_read_index(nscAddrBase,0x27));
    dev_dbg(&pdev.dev,
    "NSC SIOCF1 0x%x SIOCF5 0x%x SIOCF6 0x%x SIOCF8 0x%x\n",
    tpm_read_index(nscAddrBase,0x21), tpm_read_index(nscAddrBase,0x25),
    tpm_read_index(nscAddrBase,0x26), tpm_read_index(nscAddrBase,0x28));
    dev_dbg(&pdev.dev, "NSC IO Base0 0x%x\n",
    (tpm_read_index(nscAddrBase,0x60) << 8) | tpm_read_index(nscAddrBase,0x61));
    dev_dbg(&pdev.dev, "NSC IO Base1 0x%x\n",
    (tpm_read_index(nscAddrBase,0x62) << 8) | tpm_read_index(nscAddrBase,0x63));
    dev_dbg(&pdev.dev, "NSC Interrupt number and wakeup 0x%x\n",
    tpm_read_index(nscAddrBase,0x70));
    dev_dbg(&pdev.dev, "NSC IRQ type select 0x%x\n",
    tpm_read_index(nscAddrBase,0x71));
    dev_dbg(&pdev.dev,
    "NSC DMA channel select0 0x%x, select1 0x%x\n",
    tpm_read_index(nscAddrBase,0x74), tpm_read_index(nscAddrBase,0x75));
    dev_dbg(&pdev.dev,
    "NSC Config "
    "0x%x 0x%x 0x%x 0x%x 0x%x 0x%x 0x%x 0x%x 0x%x 0x%x\n",
    tpm_read_index(nscAddrBase,0xF0), tpm_read_index(nscAddrBase,0xF1),
    tpm_read_index(nscAddrBase,0xF2), tpm_read_index(nscAddrBase,0xF3),
    tpm_read_index(nscAddrBase,0xF4), tpm_read_index(nscAddrBase,0xF5),
    tpm_read_index(nscAddrBase,0xF6), tpm_read_index(nscAddrBase,0xF7),
    tpm_read_index(nscAddrBase,0xF8), tpm_read_index(nscAddrBase,0xF9));
    dev_info(&pdev.dev,
    "NSC TPM revision %d\n",
    tpm_read_index(nscAddrBase, 0x27) & 0x1F);
    return 0;
    err_rel_reg:
    release_region(base, 2);
    err_del_dev:
    platform_device_del(pdev);
    err_put_dev:
    platform_device_put(pdev);
    err_unreg_drv:
    platform_driver_unregister(&nsc_drv);
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn cleanup_nsc() -> void __exit {
    static void __exit cleanup_nsc(void)
    {
    if (pdev) {
    tpm_nsc_remove(&pdev.dev);
    platform_device_unregister(pdev);
    }
    platform_driver_unregister(&nsc_drv);
    }
    module_init(init_nsc);
    module_exit(cleanup_nsc);
    MODULE_AUTHOR("Leendert van Doorn <leendert@watson.ibm.com>");
    MODULE_DESCRIPTION("TPM Driver");
    MODULE_VERSION("2.0");
    MODULE_LICENSE("GPL");
