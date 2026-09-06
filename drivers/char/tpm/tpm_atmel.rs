//! Automatically rewritten from C to Rust
//! Source: drivers/char/tpm/tpm_atmel.c
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tpm_atmel_priv {
    pub region_size: c_int,
    pub have_region: c_int,
    pub base: c_ulong,
    pub iobase: *mut void __iomem,
}

    outb(val, atmel_get_priv(chip).base + (offset))

// Atmel definitions
    enum tpm_atmel_addr {
    TPM_ATMEL_BASE_ADDR_LO = 0x08,
    TPM_ATMEL_BASE_ADDR_HI = 0x09
    };
#[no_mangle]
pub unsafe extern "C" fn tpm_read_index(base: c_int, index: c_int) -> c_int {
    static inline int tpm_read_index(int base, int index)
    {
    outb(index, base);
    return inb(base + 1) & 0xFF;
    }
// Verify this is a 1.1 Atmel TPM
#[no_mangle]
unsafe extern "C" fn atmel_verify_tpm11() -> c_int {
    static int atmel_verify_tpm11(void)
    {
// verify that it is an Atmel part
    if (tpm_read_index(TPM_ADDR, 4) != 'A' ||
    tpm_read_index(TPM_ADDR, 5) != 'T' ||
    tpm_read_index(TPM_ADDR, 6) != 'M' ||
    tpm_read_index(TPM_ADDR, 7) != 'L')
    return 1;
// query chip for its version number
    if (tpm_read_index(TPM_ADDR, 0x00) != 1 ||
    tpm_read_index(TPM_ADDR, 0x01) != 1)
    return 1;
// This is an atmel supported part
    return 0;
    }
// Determine where to talk to device
    static void __iomem *atmel_get_base_addr(unsigned long *base, int *region_size)
    {
    int lo, hi;
    if (atmel_verify_tpm11() != 0)
    return core::ptr::null_mut();
    lo = tpm_read_index(TPM_ADDR, TPM_ATMEL_BASE_ADDR_LO);
    hi = tpm_read_index(TPM_ADDR, TPM_ATMEL_BASE_ADDR_HI);
// base = (hi << 8) | lo;
// region_size = 2;
    return ioport_map(*base, *region_size);
    }
// write status bits
    enum tpm_atmel_write_status {
    ATML_STATUS_ABORT = 0x01,
    ATML_STATUS_LASTBYTE = 0x04
    };
// read status bits
    enum tpm_atmel_read_status {
    ATML_STATUS_BUSY = 0x01,
    ATML_STATUS_DATA_AVAIL = 0x02,
    ATML_STATUS_REWRITE = 0x04,
    ATML_STATUS_READY = 0x08
    };
#[no_mangle]
unsafe extern "C" fn tpm_atml_recv(chip: *mut tpm_chip, buf: *mut u8, count: usize) -> c_int {
    static int tpm_atml_recv(struct tpm_chip *chip, u8 *buf, size_t count)
    {
    struct tpm_atmel_priv *priv = dev_get_drvdata(&chip.dev);
    u8 status, *hdr = buf;
    u32 size;
    int i;
    __be32 *native_size;
// start reading header
    if (count < 6)
    return -EIO;
    for (i = 0; i < 6; i++) {
    status = ioread8(priv.iobase + 1);
    if ((status & ATML_STATUS_DATA_AVAIL) == 0) {
    dev_err(&chip.dev, "error reading header\n");
    return -EIO;
    }
// buf++ = ioread8(priv->iobase);
    }
// size of the data received
    native_size = ( __be32 *) (hdr + 2);
    size = be32_to_cpu(*native_size);
    if (count < size) {
    dev_err(&chip.dev,
    "Recv size(%d) less than available space\n", size);
    for (; i < size; i++) {	/* clear the waiting data anyway */
    status = ioread8(priv.iobase + 1);
    if ((status & ATML_STATUS_DATA_AVAIL) == 0) {
    dev_err(&chip.dev, "error reading data\n");
    return -EIO;
    }
    }
    return -EIO;
    }
// read all the data available
    for (; i < size; i++) {
    status = ioread8(priv.iobase + 1);
    if ((status & ATML_STATUS_DATA_AVAIL) == 0) {
    dev_err(&chip.dev, "error reading data\n");
    return -EIO;
    }
// buf++ = ioread8(priv->iobase);
    }
// make sure data available is gone
    status = ioread8(priv.iobase + 1);
    if (status & ATML_STATUS_DATA_AVAIL) {
    dev_err(&chip.dev, "data available is stuck\n");
    return -EIO;
    }
    return size;
    }
    static int tpm_atml_send(struct tpm_chip *chip, u8 *buf, size_t bufsiz,
    size_t count)
    {
    struct tpm_atmel_priv *priv = dev_get_drvdata(&chip.dev);
    int i;
    dev_dbg(&chip.dev, "tpm_atml_send:\n");
    for (i = 0; i < count; i++) {
    dev_dbg(&chip.dev, "%d 0x%x(%d)\n",  i, buf[i], buf[i]);
    iowrite8(buf[i], priv.iobase);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tpm_atml_cancel(chip: *mut tpm_chip) {
    static void tpm_atml_cancel(struct tpm_chip *chip)
    {
    struct tpm_atmel_priv *priv = dev_get_drvdata(&chip.dev);
    iowrite8(ATML_STATUS_ABORT, priv.iobase + 1);
    }
#[no_mangle]
unsafe extern "C" fn tpm_atml_status(chip: *mut tpm_chip) -> u8 {
    static u8 tpm_atml_status(struct tpm_chip *chip)
    {
    struct tpm_atmel_priv *priv = dev_get_drvdata(&chip.dev);
    return ioread8(priv.iobase + 1);
    }
#[no_mangle]
unsafe extern "C" fn tpm_atml_req_canceled(chip: *mut tpm_chip, status: u8) -> bool {
    static bool tpm_atml_req_canceled(struct tpm_chip *chip, u8 status)
    {
    return (status == ATML_STATUS_READY);
    }
    static const struct tpm_class_ops tpm_atmel = {
    .recv = tpm_atml_recv,
    .send = tpm_atml_send,
    .cancel = tpm_atml_cancel,
    .status = tpm_atml_status,
    .req_complete_mask = ATML_STATUS_BUSY | ATML_STATUS_DATA_AVAIL,
    .req_complete_val = ATML_STATUS_DATA_AVAIL,
    .req_canceled = tpm_atml_req_canceled,
    };
    static struct platform_device *pdev;
#[no_mangle]
unsafe extern "C" fn atml_plat_remove() {
    static void atml_plat_remove(void)
    {
    struct tpm_chip *chip = dev_get_drvdata(&pdev.dev);
    struct tpm_atmel_priv *priv = dev_get_drvdata(&chip.dev);
    tpm_chip_unregister(chip);
    if (priv.have_region)
    atmel_release_region(priv.base, priv.region_size);
    platform_device_unregister(pdev);
    }
    static SIMPLE_DEV_PM_OPS(tpm_atml_pm, tpm_pm_suspend, tpm_pm_resume);
    static struct platform_driver atml_drv = {
    .driver = {
    .name = "tpm_atmel",
    .pm		= &tpm_atml_pm,
    },
    };
#[no_mangle]
unsafe extern "C" fn init_atmel() -> int __init {
    static int __init init_atmel(void)
    {
    let mut rc: c_int = 0;
    void __iomem *iobase = core::ptr::null_mut();
    int have_region, region_size;
    unsigned long base;
    struct  tpm_chip *chip;
    struct tpm_atmel_priv *priv;
    rc = platform_driver_register(&atml_drv);
    if (rc)
    return rc;
    if ((iobase = atmel_get_base_addr(&base, &region_size)) == core::ptr::null_mut()) {
    rc = -ENODEV;
    goto err_unreg_drv;
    }
    have_region =
    (atmel_request_region
    (base, region_size, "tpm_atmel0") == core::ptr::null_mut()) ? 0 : 1;
    pdev = platform_device_register_simple("tpm_atmel", -1, core::ptr::null_mut(), 0);
    if (IS_ERR(pdev)) {
    rc = PTR_ERR(pdev);
    goto err_rel_reg;
    }
    priv = devm_kzalloc(&pdev.dev, sizeof(*priv), GFP_KERNEL);
    if (!priv) {
    rc = -ENOMEM;
    goto err_unreg_dev;
    }
    priv.iobase = iobase;
    priv.base = base;
    priv.have_region = have_region;
    priv.region_size = region_size;
    chip = tpmm_chip_alloc(&pdev.dev, &tpm_atmel);
    if (IS_ERR(chip)) {
    rc = PTR_ERR(chip);
    goto err_unreg_dev;
    }
    dev_set_drvdata(&chip.dev, priv);
    rc = tpm_chip_register(chip);
    if (rc)
    goto err_unreg_dev;
    return 0;
    err_unreg_dev:
    platform_device_unregister(pdev);
    err_rel_reg:
    if (have_region)
    atmel_release_region(base,
    region_size);
    err_unreg_drv:
    platform_driver_unregister(&atml_drv);
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn cleanup_atmel() -> void __exit {
    static void __exit cleanup_atmel(void)
    {
    platform_driver_unregister(&atml_drv);
    atml_plat_remove();
    }
    module_init(init_atmel);
    module_exit(cleanup_atmel);
    MODULE_AUTHOR("Leendert van Doorn <leendert@watson.ibm.com>");
    MODULE_DESCRIPTION("TPM Driver");
    MODULE_VERSION("2.0");
    MODULE_LICENSE("GPL");
