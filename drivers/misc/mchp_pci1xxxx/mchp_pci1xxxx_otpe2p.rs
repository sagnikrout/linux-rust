//! Automatically rewritten from C to Rust
//! Source: drivers/misc/mchp_pci1xxxx/mchp_pci1xxxx_otpe2p.c
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
// Copyright (C) 2022-2023 Microchip Technology Inc.
// PCI1xxxx OTP/EEPROM driver

pub const PERI_PF3_SYSTEM_REG_ADDR_BASE: c_uint = 0x2000;
pub const PERI_PF3_SYSTEM_REG_LENGTH: c_uint = 0x4000;
pub const EEPROM_SIZE_BYTES: c_int = 8192;
pub const OTP_SIZE_BYTES: c_int = 8192;
pub const CONFIG_REG_ADDR_BASE: c_int = 0;
pub const EEPROM_REG_ADDR_BASE: c_uint = 0x0E00;
pub const OTP_REG_ADDR_BASE: c_uint = 0x1000;

pub const EEPROM_CMD_REG: c_uint = 0x00;
pub const EEPROM_DATA_REG: c_uint = 0x04;

pub const STATUS_READ_DELAY_US: c_int = 1;
pub const STATUS_READ_TIMEOUT_US: c_int = 20000;
pub const OTP_ADDR_HIGH_OFFSET: c_uint = 0x04;
pub const OTP_ADDR_LOW_OFFSET: c_uint = 0x08;
pub const OTP_PRGM_DATA_OFFSET: c_uint = 0x10;
pub const OTP_PRGM_MODE_OFFSET: c_uint = 0x14;
pub const OTP_RD_DATA_OFFSET: c_uint = 0x18;
pub const OTP_FUNC_CMD_OFFSET: c_uint = 0x20;
pub const OTP_CMD_GO_OFFSET: c_uint = 0x28;
pub const OTP_PASS_FAIL_OFFSET: c_uint = 0x2C;
pub const OTP_STATUS_OFFSET: c_uint = 0x30;

pub const OTP_PWR_DN_OFFSET: c_uint = 0x00;
pub const CFG_SYS_LOCK_OFFSET: c_uint = 0xA0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pci1xxxx_otp_eeprom_device {
    pub pdev: *mut auxiliary_device,
    pub reg_base: *mut void __iomem,
    pub nvmem_config_eeprom: nvmem_config,
    pub nvmem_eeprom: *mut nvmem_device,
    pub nvmem_config_otp: nvmem_config,
    pub nvmem_otp: *mut nvmem_device,
}

#[no_mangle]
unsafe extern "C" fn set_sys_lock(priv: *mut pci1xxxx_otp_eeprom_device) -> c_int {
    static int set_sys_lock(struct pci1xxxx_otp_eeprom_device *priv)
    {
    void __iomem *sys_lock = priv.reg_base +
    MMAP_CFG_OFFSET(CFG_SYS_LOCK_OFFSET);
    u8 data;
    writel(CFG_SYS_LOCK_PF3, sys_lock);
    data = readl(sys_lock);
    if (data != CFG_SYS_LOCK_PF3)
    return -EPERM;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn release_sys_lock(priv: *mut pci1xxxx_otp_eeprom_device) {
    static void release_sys_lock(struct pci1xxxx_otp_eeprom_device *priv)
    {
    void __iomem *sys_lock = priv.reg_base +
    MMAP_CFG_OFFSET(CFG_SYS_LOCK_OFFSET);
    writel(0, sys_lock);
    }
#[no_mangle]
unsafe extern "C" fn is_eeprom_responsive(priv: *mut pci1xxxx_otp_eeprom_device) -> bool {
    static bool is_eeprom_responsive(struct pci1xxxx_otp_eeprom_device *priv)
    {
    void __iomem *rb = priv.reg_base;
    u32 regval;
    int ret;
    writel(EEPROM_CMD_EPC_TIMEOUT_BIT,
    rb + MMAP_EEPROM_OFFSET(EEPROM_CMD_REG));
    writel(EEPROM_CMD_EPC_BUSY_BIT,
    rb + MMAP_EEPROM_OFFSET(EEPROM_CMD_REG));
// Wait for the EPC_BUSY bit to get cleared or timeout bit to get set
    ret = read_poll_timeout(readl, regval, !(regval & EEPROM_CMD_EPC_BUSY_BIT),
    STATUS_READ_DELAY_US, STATUS_READ_TIMEOUT_US,
    true, rb + MMAP_EEPROM_OFFSET(EEPROM_CMD_REG));
// Return failure if either of software or hardware timeouts happen
    if (ret < 0 || (!ret && (regval & EEPROM_CMD_EPC_TIMEOUT_BIT)))
    return false;
    return true;
    }
    static int pci1xxxx_eeprom_read(void *priv_t, unsigned int off,
    void *buf_t, size_t count)
    {
    struct pci1xxxx_otp_eeprom_device *priv = priv_t;
    void __iomem *rb = priv.reg_base;
    char *buf = buf_t;
    u32 regval;
    u32 byte;
    int ret;
    if (off >= priv.nvmem_config_eeprom.size)
    return -EFAULT;
    if ((off + count) > priv.nvmem_config_eeprom.size)
    count = priv.nvmem_config_eeprom.size - off;
    ret = set_sys_lock(priv);
    if (ret)
    return ret;
    for (byte = 0; byte < count; byte++) {
    writel(EEPROM_CMD_EPC_BUSY_BIT | (off + byte), rb +
    MMAP_EEPROM_OFFSET(EEPROM_CMD_REG));
    ret = read_poll_timeout(readl, regval,
    !(regval & EEPROM_CMD_EPC_BUSY_BIT),
    STATUS_READ_DELAY_US,
    STATUS_READ_TIMEOUT_US, true,
    rb + MMAP_EEPROM_OFFSET(EEPROM_CMD_REG));
    if (ret < 0 || (!ret && (regval & EEPROM_CMD_EPC_TIMEOUT_BIT))) {
    ret = -EIO;
    goto error;
    }
    buf[byte] = readl(rb + MMAP_EEPROM_OFFSET(EEPROM_DATA_REG));
    }
    error:
    release_sys_lock(priv);
    return ret;
    }
    static int pci1xxxx_eeprom_write(void *priv_t, unsigned int off,
    void *value_t, size_t count)
    {
    struct pci1xxxx_otp_eeprom_device *priv = priv_t;
    void __iomem *rb = priv.reg_base;
    char *value = value_t;
    u32 regval;
    u32 byte;
    int ret;
    if (off >= priv.nvmem_config_eeprom.size)
    return -EFAULT;
    if ((off + count) > priv.nvmem_config_eeprom.size)
    count = priv.nvmem_config_eeprom.size - off;
    ret = set_sys_lock(priv);
    if (ret)
    return ret;
    for (byte = 0; byte < count; byte++) {
    writel(*(value + byte), rb + MMAP_EEPROM_OFFSET(EEPROM_DATA_REG));
    regval = EEPROM_CMD_EPC_TIMEOUT_BIT | EEPROM_CMD_EPC_WRITE |
    (off + byte);
    writel(regval, rb + MMAP_EEPROM_OFFSET(EEPROM_CMD_REG));
    writel(EEPROM_CMD_EPC_BUSY_BIT | regval,
    rb + MMAP_EEPROM_OFFSET(EEPROM_CMD_REG));
    ret = read_poll_timeout(readl, regval,
    !(regval & EEPROM_CMD_EPC_BUSY_BIT),
    STATUS_READ_DELAY_US,
    STATUS_READ_TIMEOUT_US, true,
    rb + MMAP_EEPROM_OFFSET(EEPROM_CMD_REG));
    if (ret < 0 || (!ret && (regval & EEPROM_CMD_EPC_TIMEOUT_BIT))) {
    ret = -EIO;
    goto error;
    }
    }
    error:
    release_sys_lock(priv);
    return ret;
    }
    static void otp_device_set_address(struct pci1xxxx_otp_eeprom_device *priv,
    u16 address)
    {
    u16 lo, hi;
    lo = address & BYTE_LOW;
    hi = (address & BYTE_HIGH) >> 8;
    writew(lo, priv.reg_base + MMAP_OTP_OFFSET(OTP_ADDR_LOW_OFFSET));
    writew(hi, priv.reg_base + MMAP_OTP_OFFSET(OTP_ADDR_HIGH_OFFSET));
    }
    static int pci1xxxx_otp_read(void *priv_t, unsigned int off,
    void *buf_t, size_t count)
    {
    struct pci1xxxx_otp_eeprom_device *priv = priv_t;
    void __iomem *rb = priv.reg_base;
    char *buf = buf_t;
    u32 regval;
    u32 byte;
    int ret;
    u8 data;
    if (off >= priv.nvmem_config_otp.size)
    return -EFAULT;
    if ((off + count) > priv.nvmem_config_otp.size)
    count = priv.nvmem_config_otp.size - off;
    ret = set_sys_lock(priv);
    if (ret)
    return ret;
    for (byte = 0; byte < count; byte++) {
    otp_device_set_address(priv, (u16)(off + byte));
    data = readl(rb + MMAP_OTP_OFFSET(OTP_FUNC_CMD_OFFSET));
    writel(data | OTP_FUNC_RD_BIT,
    rb + MMAP_OTP_OFFSET(OTP_FUNC_CMD_OFFSET));
    data = readl(rb + MMAP_OTP_OFFSET(OTP_CMD_GO_OFFSET));
    writel(data | OTP_CMD_GO_BIT,
    rb + MMAP_OTP_OFFSET(OTP_CMD_GO_OFFSET));
    ret = read_poll_timeout(readl, regval,
    !(regval & OTP_STATUS_BUSY_BIT),
    STATUS_READ_DELAY_US,
    STATUS_READ_TIMEOUT_US, true,
    rb + MMAP_OTP_OFFSET(OTP_STATUS_OFFSET));
    data = readl(rb + MMAP_OTP_OFFSET(OTP_PASS_FAIL_OFFSET));
    if (ret < 0 || data & OTP_FAIL_BIT) {
    ret = -EIO;
    goto error;
    }
    buf[byte] = readl(rb + MMAP_OTP_OFFSET(OTP_RD_DATA_OFFSET));
    }
    error:
    release_sys_lock(priv);
    return ret;
    }
    static int pci1xxxx_otp_write(void *priv_t, unsigned int off,
    void *value_t, size_t count)
    {
    struct pci1xxxx_otp_eeprom_device *priv = priv_t;
    void __iomem *rb = priv.reg_base;
    char *value = value_t;
    u32 regval;
    u32 byte;
    int ret;
    u8 data;
    if (off >= priv.nvmem_config_otp.size)
    return -EFAULT;
    if ((off + count) > priv.nvmem_config_otp.size)
    count = priv.nvmem_config_otp.size - off;
    ret = set_sys_lock(priv);
    if (ret)
    return ret;
    for (byte = 0; byte < count; byte++) {
    otp_device_set_address(priv, (u16)(off + byte));
//
// Set OTP_PGM_MODE_BYTE command bit in OTP_PRGM_MODE register
// to enable Byte programming
//
    data = readl(rb + MMAP_OTP_OFFSET(OTP_PRGM_MODE_OFFSET));
    writel(data | OTP_PGM_MODE_BYTE_BIT,
    rb + MMAP_OTP_OFFSET(OTP_PRGM_MODE_OFFSET));
    writel(*(value + byte), rb + MMAP_OTP_OFFSET(OTP_PRGM_DATA_OFFSET));
    data = readl(rb + MMAP_OTP_OFFSET(OTP_FUNC_CMD_OFFSET));
    writel(data | OTP_FUNC_PGM_BIT,
    rb + MMAP_OTP_OFFSET(OTP_FUNC_CMD_OFFSET));
    data = readl(rb + MMAP_OTP_OFFSET(OTP_CMD_GO_OFFSET));
    writel(data | OTP_CMD_GO_BIT,
    rb + MMAP_OTP_OFFSET(OTP_CMD_GO_OFFSET));
    ret = read_poll_timeout(readl, regval,
    !(regval & OTP_STATUS_BUSY_BIT),
    STATUS_READ_DELAY_US,
    STATUS_READ_TIMEOUT_US, true,
    rb + MMAP_OTP_OFFSET(OTP_STATUS_OFFSET));
    data = readl(rb + MMAP_OTP_OFFSET(OTP_PASS_FAIL_OFFSET));
    if (ret < 0 || data & OTP_FAIL_BIT) {
    ret = -EIO;
    goto error;
    }
    }
    error:
    release_sys_lock(priv);
    return ret;
    }
    static int pci1xxxx_otp_eeprom_probe(struct auxiliary_device *aux_dev,
    const struct auxiliary_device_id *id)
    {
    struct auxiliary_device_wrapper *aux_dev_wrapper;
    struct pci1xxxx_otp_eeprom_device *priv;
    struct gp_aux_data_type *pdata;
    int ret;
    u8 data;
    aux_dev_wrapper = container_of(aux_dev, struct auxiliary_device_wrapper,
    aux_dev);
    pdata = &aux_dev_wrapper.gp_aux_data;
    if (!pdata)
    return -EINVAL;
    priv = devm_kzalloc(&aux_dev.dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    priv.pdev = aux_dev;
    if (!devm_request_mem_region(&aux_dev.dev, pdata.region_start +
    PERI_PF3_SYSTEM_REG_ADDR_BASE,
    PERI_PF3_SYSTEM_REG_LENGTH,
    aux_dev.name))
    return -ENOMEM;
    priv.reg_base = devm_ioremap(&aux_dev.dev, pdata.region_start +
    PERI_PF3_SYSTEM_REG_ADDR_BASE,
    PERI_PF3_SYSTEM_REG_LENGTH);
    if (!priv.reg_base)
    return -ENOMEM;
    ret = set_sys_lock(priv);
    if (ret)
    return ret;
// Set OTP_PWR_DN to 0 to make OTP Operational
    data = readl(priv.reg_base + MMAP_OTP_OFFSET(OTP_PWR_DN_OFFSET));
    writel(data & ~OTP_PWR_DN_BIT,
    priv.reg_base + MMAP_OTP_OFFSET(OTP_PWR_DN_OFFSET));
    dev_set_drvdata(&aux_dev.dev, priv);
    if (is_eeprom_responsive(priv)) {
    priv.nvmem_config_eeprom.type = NVMEM_TYPE_EEPROM;
    priv.nvmem_config_eeprom.name = EEPROM_NAME;
    priv.nvmem_config_eeprom.id = NVMEM_DEVID_AUTO;
    priv.nvmem_config_eeprom.dev = &aux_dev.dev;
    priv.nvmem_config_eeprom.owner = THIS_MODULE;
    priv.nvmem_config_eeprom.reg_read = pci1xxxx_eeprom_read;
    priv.nvmem_config_eeprom.reg_write = pci1xxxx_eeprom_write;
    priv.nvmem_config_eeprom.priv = priv;
    priv.nvmem_config_eeprom.stride = 1;
    priv.nvmem_config_eeprom.word_size = 1;
    priv.nvmem_config_eeprom.size = EEPROM_SIZE_BYTES;
    priv.nvmem_eeprom = devm_nvmem_register(&aux_dev.dev,
    &priv.nvmem_config_eeprom);
    if (IS_ERR(priv.nvmem_eeprom))
    return PTR_ERR(priv.nvmem_eeprom);
    }
    release_sys_lock(priv);
    priv.nvmem_config_otp.type = NVMEM_TYPE_OTP;
    priv.nvmem_config_otp.name = OTP_NAME;
    priv.nvmem_config_otp.id = NVMEM_DEVID_AUTO;
    priv.nvmem_config_otp.dev = &aux_dev.dev;
    priv.nvmem_config_otp.owner = THIS_MODULE;
    priv.nvmem_config_otp.reg_read = pci1xxxx_otp_read;
    priv.nvmem_config_otp.reg_write = pci1xxxx_otp_write;
    priv.nvmem_config_otp.priv = priv;
    priv.nvmem_config_otp.stride = 1;
    priv.nvmem_config_otp.word_size = 1;
    priv.nvmem_config_otp.size = OTP_SIZE_BYTES;
    priv.nvmem_otp = devm_nvmem_register(&aux_dev.dev,
    &priv.nvmem_config_otp);
    if (IS_ERR(priv.nvmem_otp))
    return PTR_ERR(priv.nvmem_otp);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn pci1xxxx_otp_eeprom_remove(aux_dev: *mut auxiliary_device) {
    static void pci1xxxx_otp_eeprom_remove(struct auxiliary_device *aux_dev)
    {
    struct pci1xxxx_otp_eeprom_device *priv;
    void __iomem *sys_lock;
    priv = dev_get_drvdata(&aux_dev.dev);
    sys_lock = priv.reg_base + MMAP_CFG_OFFSET(CFG_SYS_LOCK_OFFSET);
    writel(CFG_SYS_LOCK_PF3, sys_lock);
// Shut down OTP
    writel(OTP_PWR_DN_BIT,
    priv.reg_base + MMAP_OTP_OFFSET(OTP_PWR_DN_OFFSET));
    writel(0, sys_lock);
    }
    static const struct auxiliary_device_id pci1xxxx_otp_eeprom_auxiliary_id_table[] = {
    { .name = "mchp_pci1xxxx_gp.gp_otp_e2p" },
    { }
    };
    MODULE_DEVICE_TABLE(auxiliary, pci1xxxx_otp_eeprom_auxiliary_id_table);
    static struct auxiliary_driver pci1xxxx_otp_eeprom_driver = {
    .driver = {
    .name = AUX_DRIVER_NAME,
    },
    .probe = pci1xxxx_otp_eeprom_probe,
    .remove = pci1xxxx_otp_eeprom_remove,
    .id_table = pci1xxxx_otp_eeprom_auxiliary_id_table
    };
    module_auxiliary_driver(pci1xxxx_otp_eeprom_driver);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Kumaravel Thiagarajan <kumaravel.thiagarajan@microchip.com>");
    MODULE_AUTHOR("Tharun Kumar P <tharunkumar.pasumarthi@microchip.com>");
    MODULE_AUTHOR("Vaibhaav Ram T.L <vaibhaavram.tl@microchip.com>");
    MODULE_DESCRIPTION("Microchip Technology Inc. PCI1xxxx OTP EEPROM Programmer");
