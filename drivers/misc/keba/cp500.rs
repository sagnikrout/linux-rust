//! Automatically rewritten from C to Rust
//! Source: drivers/misc/keba/cp500.c
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
// Copyright (C) KEBA Industrial Automation Gmbh 2024
//
// Driver for KEBA system FPGA
//
// The KEBA system FPGA implements various devices. This driver registers
// auxiliary devices for every device within the FPGA.
//

pub const PCI_VENDOR_ID_KEBA: c_uint = 0xCEBA;
pub const PCI_DEVICE_ID_KEBA_CP035: c_uint = 0x2706;
pub const PCI_DEVICE_ID_KEBA_CP505: c_uint = 0x2703;
pub const PCI_DEVICE_ID_KEBA_CP520: c_uint = 0x2696;
pub const CP500_SYS_BAR: c_int = 0;
pub const CP500_ECM_BAR: c_int = 1;
// BAR 0 registers
pub const CP500_VERSION_REG: c_uint = 0x00;
pub const CP500_RECONFIG_REG: c_uint = 0x11	/* upper 8-bits of STARTUP register */;
pub const CP500_PRESENT_REG: c_uint = 0x20;
pub const CP500_AXI_REG: c_uint = 0x40;
// Bits in BUILD_REG
pub const CP500_BUILD_TEST: c_uint = 0x8000	/* FPGA test version */;
// Bits in RECONFIG_REG
pub const CP500_RECFG_REQ: c_uint = 0x01	/* reconfigure FPGA on next reset */;
// Bits in PRESENT_REG
pub const CP500_PRESENT_FAN0: c_uint = 0x01;
// MSIX
pub const CP500_AXI_MSIX: c_int = 3;
pub const CP500_RFB_UART_MSIX: c_int = 4;
pub const CP500_DEBUG_UART_MSIX: c_int = 5;
pub const CP500_SI1_UART_MSIX: c_int = 6;
pub const CP500_NUM_MSIX: c_int = 8;
pub const CP500_NUM_MSIX_NO_MMI: c_int = 2;
pub const CP500_NUM_MSIX_NO_AXI: c_int = 3;
// EEPROM
pub const CP500_EEPROM_DA_OFFSET: c_uint = 0x016F;
pub const CP500_EEPROM_DA_ESC_TYPE_MASK: c_uint = 0x01;
pub const CP500_EEPROM_ESC_LAN9252: c_uint = 0x00;
pub const CP500_EEPROM_ESC_ET1100: c_uint = 0x01;

pub const CP500_EEPROM_CPU_OFFSET: c_int = 0;
pub const CP500_EEPROM_CPU_SIZE: c_int = 3072;

pub const CP500_EEPROM_USER_OFFSET: c_int = 3072;
pub const CP500_EEPROM_USER_SIZE: c_int = 1024;
// SPI flash running at full speed

// LAN9252

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cp500_dev_info {
    pub offset: off_t,
    pub size: usize,
    pub msix: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cp500_devs {
    pub startup: cp500_dev_info,
    pub spi: cp500_dev_info,
    pub i2c: cp500_dev_info,
    pub fan: cp500_dev_info,
    pub batt: cp500_dev_info,
    pub uart0_rfb: cp500_dev_info,
    pub uart1_dbg: cp500_dev_info,
    pub uart2_si1: cp500_dev_info,
}

// list of devices within FPGA of CP035 family (CP035, CP056, CP057)
    static struct cp500_devs cp035_devices = {
    .startup   = { 0x0000, SZ_4K },
    .spi       = { 0x1000, SZ_4K },
    .i2c       = { 0x4000, SZ_4K },
    .fan       = { 0x9000, SZ_4K },
    .batt      = { 0xA000, SZ_4K },
    .uart0_rfb = { 0xB000, SZ_4K, CP500_RFB_UART_MSIX },
    .uart2_si1 = { 0xD000, SZ_4K, CP500_SI1_UART_MSIX },
    };
// list of devices within FPGA of CP505 family (CP503, CP505, CP507)
    static struct cp500_devs cp505_devices = {
    .startup   = { 0x0000, SZ_4K },
    .spi       = { 0x4000, SZ_4K },
    .i2c       = { 0x5000, SZ_4K },
    .fan       = { 0x9000, SZ_4K },
    .batt      = { 0xA000, SZ_4K },
    .uart0_rfb = { 0xB000, SZ_4K, CP500_RFB_UART_MSIX },
    .uart2_si1 = { 0xD000, SZ_4K, CP500_SI1_UART_MSIX },
    };
// list of devices within FPGA of CP520 family (CP520, CP530)
    static struct cp500_devs cp520_devices = {
    .startup   = { 0x0000, SZ_4K },
    .spi       = { 0x4000, SZ_4K },
    .i2c       = { 0x5000, SZ_4K },
    .fan       = { 0x8000, SZ_4K },
    .batt      = { 0x9000, SZ_4K },
    .uart0_rfb = { 0xC000, SZ_4K, CP500_RFB_UART_MSIX },
    .uart1_dbg = { 0xD000, SZ_4K, CP500_DEBUG_UART_MSIX },
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cp500_nvmem {
    pub base_nvmem: *mut nvmem_device,
    pub offset: c_uint,
    pub nvmem: *mut nvmem_device,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cp500 {
    pub pci_dev: *mut pci_dev,
    pub devs: *mut cp500_devs,
    pub msix_num: c_int,
    struct {
    pub major: c_int,
    pub minor: c_int,
    pub build: c_int,
    pub version: },
    pub nvmem_notifier: notifier_block,
    pub nvmem_notified: core::sync::atomic::AtomicI32,
// system FPGA BAR
    pub sys_hwbase: resource_size_t,
    pub spi: *mut keba_spi_auxdev,
    pub i2c: *mut keba_i2c_auxdev,
    pub fan: *mut keba_fan_auxdev,
    pub batt: *mut keba_batt_auxdev,
    pub uart0_rfb: *mut keba_uart_auxdev,
    pub uart1_dbg: *mut keba_uart_auxdev,
    pub uart2_si1: *mut keba_uart_auxdev,
// ECM EtherCAT BAR
    pub ecm_hwbase: resource_size_t,
// NVMEM devices
    pub nvmem_cpu: cp500_nvmem,
    pub nvmem_user: cp500_nvmem,
    pub system_startup_addr: *mut void __iomem,
}

// I2C devices
pub const CP500_EEPROM_ADDR: c_uint = 0x50;
    static struct i2c_board_info cp500_i2c_info[] = {
    {	/* temperature sensor */
    I2C_BOARD_INFO("emc1403", 0x4c),
    },
    {	/*
// CPU EEPROM
// CP035 family: CPU board
// CP505 family: bridge board
// CP520 family: carrier board
//
    I2C_BOARD_INFO("24c32", CP500_EEPROM_ADDR),
    },
    {	/* interface board EEPROM */
    I2C_BOARD_INFO("24c32", CP500_EEPROM_ADDR + 1),
    },
    {	/*
// EEPROM (optional)
// CP505 family: CPU board
// CP520 family: MMI board
//
    I2C_BOARD_INFO("24c32", CP500_EEPROM_ADDR + 2),
    },
    {	/* extension module 0 EEPROM (optional) */
    I2C_BOARD_INFO("24c32", CP500_EEPROM_ADDR + 3),
    },
    {	/* extension module 1 EEPROM (optional) */
    I2C_BOARD_INFO("24c32", CP500_EEPROM_ADDR + 4),
    },
    {	/* extension module 2 EEPROM (optional) */
    I2C_BOARD_INFO("24c32", CP500_EEPROM_ADDR + 5),
    },
    {	/* extension module 3 EEPROM (optional) */
    I2C_BOARD_INFO("24c32", CP500_EEPROM_ADDR + 6),
    }
    };
// SPI devices
    static struct mtd_partition cp500_partitions[] = {
    {
    .name       = "system-flash-parts",
    .size       = MTDPART_SIZ_FULL,
    .offset     = 0,
    .mask_flags = 0
    }
    };
    static const struct flash_platform_data cp500_w25q32 = {
    .type     = "w25q32",
    .name     = "system-flash",
    .parts    = cp500_partitions,
    .nr_parts = ARRAY_SIZE(cp500_partitions),
    };
    static const struct flash_platform_data cp500_m25p16 = {
    .type     = "m25p16",
    .name     = "system-flash",
    .parts    = cp500_partitions,
    .nr_parts = ARRAY_SIZE(cp500_partitions),
    };
    static struct spi_board_info cp500_spi_info[] = {
    {       /* system FPGA configuration bitstream flash */
    .modalias      = "m25p80",
    .platform_data = &cp500_m25p16,
    .max_speed_hz  = CP500_FLASH_HZ,
    .chip_select   = 0,
    .mode          = SPI_MODE_3,
    }, {    /* LAN9252 EtherCAT slave controller */
    .modalias      = "lan9252",
    .platform_data = core::ptr::null_mut(),
    .max_speed_hz  = CP500_LAN9252_HZ,
    .chip_select   = 1,
    .mode          = SPI_MODE_3,
    }
    };
    static ssize_t cp500_get_fpga_version(struct cp500 *cp500, char *buf,
    size_t max_len)
    {
    int n;
    if (CP500_IS_CP035(cp500))
    n = scnprintf(buf, max_len, "CP035");
#[no_mangle]
pub unsafe extern "C" fn if(_arg: CP500_IS_CP505(cp500)) -> else {
    else if (CP500_IS_CP505(cp500))
    n = scnprintf(buf, max_len, "CP505");
    else
    n = scnprintf(buf, max_len, "CP500");
    n += scnprintf(buf + n, max_len - n, "_FPGA_%d.%02d",
    cp500.version.major, cp500.version.minor);
// test versions have test bit set
    if (cp500.version.build & CP500_BUILD_TEST)
    n += scnprintf(buf + n, max_len - n, "Test%d",
    cp500.version.build & ~CP500_BUILD_TEST);
    n += scnprintf(buf + n, max_len - n, "\n");
    return n;
    }
    static ssize_t version_show(struct device *dev, struct device_attribute *attr,
    char *buf)
    {
    struct cp500 *cp500 = dev_get_drvdata(dev);
    return cp500_get_fpga_version(cp500, buf, PAGE_SIZE);
    }
    static DEVICE_ATTR_RO(version);
    static ssize_t keep_cfg_show(struct device *dev, struct device_attribute *attr,
    char *buf)
    {
    struct cp500 *cp500 = dev_get_drvdata(dev);
    let mut keep_cfg: c_ulong = 1;
//
// FPGA configuration stream is kept during reset when RECONFIG bit is
// zero
//
    if (ioread8(cp500.system_startup_addr + CP500_RECONFIG_REG) &
    CP500_RECFG_REQ)
    keep_cfg = 0;
    return sysfs_emit(buf, "%lu\n", keep_cfg);
    }
    static ssize_t keep_cfg_store(struct device *dev, struct device_attribute *attr,
    const char *buf, size_t count)
    {
    struct cp500 *cp500 = dev_get_drvdata(dev);
    unsigned long keep_cfg;
    if (kstrtoul(buf, 10, &keep_cfg) < 0)
    return -EINVAL;
//
// In normal operation "keep_cfg" is "1". This means that the FPGA keeps
// its configuration stream during a reset.
// In case of a firmware update of the FPGA, the configuration stream
// needs to be reloaded. This can be done without a powercycle by
// writing a "0" into the "keep_cfg" attribute. After a reset/reboot th
// new configuration stream will be loaded.
//
    if (keep_cfg)
    iowrite8(0, cp500.system_startup_addr + CP500_RECONFIG_REG);
    else
    iowrite8(CP500_RECFG_REQ,
    cp500.system_startup_addr + CP500_RECONFIG_REG);
    return count;
    }
    static DEVICE_ATTR_RW(keep_cfg);
    static struct attribute *cp500_attrs[] = {
    &dev_attr_version.attr,
    &dev_attr_keep_cfg.attr,
    core::ptr::null_mut()
    };
    ATTRIBUTE_GROUPS(cp500);
#[no_mangle]
unsafe extern "C" fn cp500_i2c_release(dev: *mut device) {
    static void cp500_i2c_release(struct device *dev)
    {
    struct keba_i2c_auxdev *i2c =
    container_of(dev, struct keba_i2c_auxdev, auxdev.dev);
    kfree(i2c);
    }
#[no_mangle]
unsafe extern "C" fn cp500_register_i2c(cp500: *mut cp500) -> c_int {
    static int cp500_register_i2c(struct cp500 *cp500)
    {
    int ret;
    cp500.i2c = kzalloc_obj(*cp500.i2c);
    if (!cp500.i2c)
    return -ENOMEM;
    cp500.i2c.auxdev.name = "i2c";
    cp500.i2c.auxdev.id = 0;
    cp500.i2c.auxdev.dev.release = cp500_i2c_release;
    cp500.i2c.auxdev.dev.parent = &cp500.pci_dev.dev;
    cp500.i2c.io = (struct resource) {
// I2C register area
    .start = (resource_size_t) cp500.sys_hwbase +
    cp500.devs.i2c.offset,
    .end   = (resource_size_t) cp500.sys_hwbase +
    cp500.devs.i2c.offset +
    cp500.devs.i2c.size - 1,
    .flags = IORESOURCE_MEM,
    };
    cp500.i2c.info_size = ARRAY_SIZE(cp500_i2c_info);
    cp500.i2c.info = cp500_i2c_info;
    ret = auxiliary_device_init(&cp500.i2c.auxdev);
    if (ret) {
    kfree(cp500.i2c);
    cp500.i2c = core::ptr::null_mut();
    return ret;
    }
    ret = __auxiliary_device_add(&cp500.i2c.auxdev, "keba");
    if (ret) {
    auxiliary_device_uninit(&cp500.i2c.auxdev);
    cp500.i2c = core::ptr::null_mut();
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cp500_spi_release(dev: *mut device) {
    static void cp500_spi_release(struct device *dev)
    {
    struct keba_spi_auxdev *spi =
    container_of(dev, struct keba_spi_auxdev, auxdev.dev);
    kfree(spi);
    }
#[no_mangle]
unsafe extern "C" fn cp500_register_spi(cp500: *mut cp500, esc_type: u8) -> c_int {
    static int cp500_register_spi(struct cp500 *cp500, u8 esc_type)
    {
    int info_size;
    int ret;
    cp500.spi = kzalloc_obj(*cp500.spi);
    if (!cp500.spi)
    return -ENOMEM;
    if (CP500_IS_CP035(cp500))
    cp500_spi_info[0].platform_data = &cp500_w25q32;
    if (esc_type == CP500_EEPROM_ESC_LAN9252)
    info_size = ARRAY_SIZE(cp500_spi_info);
    else
    info_size = ARRAY_SIZE(cp500_spi_info) - 1;
    cp500.spi.auxdev.name = "spi";
    cp500.spi.auxdev.id = 0;
    cp500.spi.auxdev.dev.release = cp500_spi_release;
    cp500.spi.auxdev.dev.parent = &cp500.pci_dev.dev;
    cp500.spi.io = (struct resource) {
// SPI register area
    .start = (resource_size_t) cp500.sys_hwbase +
    cp500.devs.spi.offset,
    .end   = (resource_size_t) cp500.sys_hwbase +
    cp500.devs.spi.offset +
    cp500.devs.spi.size - 1,
    .flags = IORESOURCE_MEM,
    };
    cp500.spi.info_size = info_size;
    cp500.spi.info = cp500_spi_info;
    ret = auxiliary_device_init(&cp500.spi.auxdev);
    if (ret) {
    kfree(cp500.spi);
    cp500.spi = core::ptr::null_mut();
    return ret;
    }
    ret = __auxiliary_device_add(&cp500.spi.auxdev, "keba");
    if (ret) {
    auxiliary_device_uninit(&cp500.spi.auxdev);
    cp500.spi = core::ptr::null_mut();
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cp500_fan_release(dev: *mut device) {
    static void cp500_fan_release(struct device *dev)
    {
    struct keba_fan_auxdev *fan =
    container_of(dev, struct keba_fan_auxdev, auxdev.dev);
    kfree(fan);
    }
#[no_mangle]
unsafe extern "C" fn cp500_register_fan(cp500: *mut cp500) -> c_int {
    static int cp500_register_fan(struct cp500 *cp500)
    {
    int ret;
    cp500.fan = kzalloc_obj(*cp500.fan);
    if (!cp500.fan)
    return -ENOMEM;
    cp500.fan.auxdev.name = "fan";
    cp500.fan.auxdev.id = 0;
    cp500.fan.auxdev.dev.release = cp500_fan_release;
    cp500.fan.auxdev.dev.parent = &cp500.pci_dev.dev;
    cp500.fan.io = (struct resource) {
// fan register area
    .start = (resource_size_t) cp500.sys_hwbase +
    cp500.devs.fan.offset,
    .end   = (resource_size_t) cp500.sys_hwbase +
    cp500.devs.fan.offset +
    cp500.devs.fan.size - 1,
    .flags = IORESOURCE_MEM,
    };
    ret = auxiliary_device_init(&cp500.fan.auxdev);
    if (ret) {
    kfree(cp500.fan);
    cp500.fan = core::ptr::null_mut();
    return ret;
    }
    ret = __auxiliary_device_add(&cp500.fan.auxdev, "keba");
    if (ret) {
    auxiliary_device_uninit(&cp500.fan.auxdev);
    cp500.fan = core::ptr::null_mut();
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cp500_batt_release(dev: *mut device) {
    static void cp500_batt_release(struct device *dev)
    {
    struct keba_batt_auxdev *fan =
    container_of(dev, struct keba_batt_auxdev, auxdev.dev);
    kfree(fan);
    }
#[no_mangle]
unsafe extern "C" fn cp500_register_batt(cp500: *mut cp500) -> c_int {
    static int cp500_register_batt(struct cp500 *cp500)
    {
    int ret;
    cp500.batt = kzalloc_obj(*cp500.batt);
    if (!cp500.batt)
    return -ENOMEM;
    cp500.batt.auxdev.name = "batt";
    cp500.batt.auxdev.id = 0;
    cp500.batt.auxdev.dev.release = cp500_batt_release;
    cp500.batt.auxdev.dev.parent = &cp500.pci_dev.dev;
    cp500.batt.io = (struct resource) {
// battery register area
    .start = (resource_size_t) cp500.sys_hwbase +
    cp500.devs.batt.offset,
    .end   = (resource_size_t) cp500.sys_hwbase +
    cp500.devs.batt.offset +
    cp500.devs.batt.size - 1,
    .flags = IORESOURCE_MEM,
    };
    ret = auxiliary_device_init(&cp500.batt.auxdev);
    if (ret) {
    kfree(cp500.batt);
    cp500.batt = core::ptr::null_mut();
    return ret;
    }
    ret = __auxiliary_device_add(&cp500.batt.auxdev, "keba");
    if (ret) {
    auxiliary_device_uninit(&cp500.batt.auxdev);
    cp500.batt = core::ptr::null_mut();
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cp500_uart_release(dev: *mut device) {
    static void cp500_uart_release(struct device *dev)
    {
    struct keba_uart_auxdev *uart =
    container_of(dev, struct keba_uart_auxdev, auxdev.dev);
    kfree(uart);
    }
    static int cp500_register_uart(struct cp500 *cp500,
    struct keba_uart_auxdev **uart, const char *name,
    struct cp500_dev_info *info, unsigned int irq)
    {
    int ret;
// uart = kzalloc_obj(**uart);
    if (!*uart)
    return -ENOMEM;
    (*uart).auxdev.name = name;
    (*uart).auxdev.id = 0;
    (*uart).auxdev.dev.release = cp500_uart_release;
    (*uart).auxdev.dev.parent = &cp500.pci_dev.dev;
    (*uart).io = (struct resource) {
// UART register area
    .start = (resource_size_t) cp500.sys_hwbase + info.offset,
    .end   = (resource_size_t) cp500.sys_hwbase + info.offset +
    info.size - 1,
    .flags = IORESOURCE_MEM,
    };
    (*uart).irq = irq;
    ret = auxiliary_device_init(&(*uart).auxdev);
    if (ret) {
    kfree(*uart);
// uart = NULL;
    return ret;
    }
    ret = __auxiliary_device_add(&(*uart).auxdev, "keba");
    if (ret) {
    auxiliary_device_uninit(&(*uart).auxdev);
// uart = NULL;
    return ret;
    }
    return 0;
    }
    static int cp500_nvmem_read(void *priv, unsigned int offset, void *val,
    size_t bytes)
    {
    struct cp500_nvmem *nvmem = priv;
    int ret;
    ret = nvmem_device_read(nvmem.base_nvmem, nvmem.offset + offset,
    bytes, val);
    if (ret != bytes)
    return ret;
    return 0;
    }
    static int cp500_nvmem_write(void *priv, unsigned int offset, void *val,
    size_t bytes)
    {
    struct cp500_nvmem *nvmem = priv;
    int ret;
    ret = nvmem_device_write(nvmem.base_nvmem, nvmem.offset + offset,
    bytes, val);
    if (ret != bytes)
    return ret;
    return 0;
    }
    static int cp500_nvmem_register(struct cp500 *cp500,
    struct nvmem_device *base_nvmem)
    {
    struct device *dev = &cp500.pci_dev.dev;
    let mut nvmem_config: nvmem_config = {};
    struct nvmem_device *tmp;
//
// The main EEPROM of CP500 devices is logically split into two EEPROMs.
// The first logical EEPROM with 3 kB contains the type label which is
// programmed during production of the device. The second logical EEPROM
// with 1 kB is not programmed during production and can be used for
// arbitrary user data.
//
    nvmem_config.dev = dev;
    nvmem_config.owner = THIS_MODULE;
    nvmem_config.id = NVMEM_DEVID_NONE;
    nvmem_config.type = NVMEM_TYPE_EEPROM;
    nvmem_config.root_only = true;
    nvmem_config.reg_read = cp500_nvmem_read;
    nvmem_config.reg_write = cp500_nvmem_write;
    cp500.nvmem_cpu.base_nvmem = base_nvmem;
    cp500.nvmem_cpu.offset = CP500_EEPROM_CPU_OFFSET;
    nvmem_config.name = CP500_EEPROM_CPU_NAME;
    nvmem_config.size = CP500_EEPROM_CPU_SIZE;
    nvmem_config.priv = &cp500.nvmem_cpu;
    tmp = nvmem_register(&nvmem_config);
    if (IS_ERR(tmp))
    return PTR_ERR(tmp);
    cp500.nvmem_cpu.nvmem = tmp;
    cp500.nvmem_user.base_nvmem = base_nvmem;
    cp500.nvmem_user.offset = CP500_EEPROM_USER_OFFSET;
    nvmem_config.name = CP500_EEPROM_USER_NAME;
    nvmem_config.size = CP500_EEPROM_USER_SIZE;
    nvmem_config.priv = &cp500.nvmem_user;
    tmp = nvmem_register(&nvmem_config);
    if (IS_ERR(tmp)) {
    nvmem_unregister(cp500.nvmem_cpu.nvmem);
    cp500.nvmem_cpu.nvmem = core::ptr::null_mut();
    return PTR_ERR(tmp);
    }
    cp500.nvmem_user.nvmem = tmp;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cp500_nvmem_unregister(cp500: *mut cp500) {
    static void cp500_nvmem_unregister(struct cp500 *cp500)
    {
    int notified;
    if (cp500.nvmem_user.nvmem) {
    nvmem_unregister(cp500.nvmem_user.nvmem);
    cp500.nvmem_user.nvmem = core::ptr::null_mut();
    }
    if (cp500.nvmem_cpu.nvmem) {
    nvmem_unregister(cp500.nvmem_cpu.nvmem);
    cp500.nvmem_cpu.nvmem = core::ptr::null_mut();
    }
// CPU and user nvmem use the same base_nvmem, put only once
    notified = atomic_read(&cp500.nvmem_notified);
    if (notified)
    nvmem_device_put(cp500.nvmem_cpu.base_nvmem);
    }
#[no_mangle]
unsafe extern "C" fn cp500_nvmem_match(dev: *mut device, data: *const c_void) -> c_int {
    static int cp500_nvmem_match(struct device *dev, const void *data)
    {
    const struct cp500 *cp500 = data;
    struct i2c_client *client;
// match only CPU EEPROM below the cp500 device
    dev = dev.parent;
    client = i2c_verify_client(dev);
    if (!client || client.addr != CP500_EEPROM_ADDR)
    return 0;
    while ((dev = dev.parent))
    if (dev == &cp500.pci_dev.dev)
    return 1;
    return 0;
    }
    static int cp500_nvmem(struct notifier_block *nb, unsigned long action,
    void *data)
    {
    struct nvmem_device *nvmem;
    struct cp500 *cp500;
    struct device *dev;
    int notified;
    u8 esc_type;
    int ret;
    if (action != NVMEM_ADD)
    return NOTIFY_DONE;
    cp500 = container_of(nb, struct cp500, nvmem_notifier);
    dev = &cp500.pci_dev.dev;
// process CPU EEPROM content only once
    notified = atomic_read(&cp500.nvmem_notified);
    if (notified)
    return NOTIFY_DONE;
    nvmem = nvmem_device_find(cp500, cp500_nvmem_match);
    if (IS_ERR_OR_NULL(nvmem))
    return NOTIFY_DONE;
    if (!atomic_try_cmpxchg_relaxed(&cp500.nvmem_notified, &notified, 1)) {
    nvmem_device_put(nvmem);
    return NOTIFY_DONE;
    }
    ret = cp500_nvmem_register(cp500, nvmem);
    if (ret)
    return ret;
    ret = nvmem_device_read(nvmem, CP500_EEPROM_DA_OFFSET, sizeof(esc_type),
    (void *)&esc_type);
    if (ret != sizeof(esc_type)) {
    dev_warn(dev, "Failed to read device assembly!\n");
    return NOTIFY_DONE;
    }
    esc_type &= CP500_EEPROM_DA_ESC_TYPE_MASK;
    if (cp500_register_spi(cp500, esc_type))
    dev_warn(dev, "Failed to register SPI!\n");
    return NOTIFY_OK;
    }
#[no_mangle]
unsafe extern "C" fn cp500_register_auxiliary_devs(cp500: *mut cp500) {
    static void cp500_register_auxiliary_devs(struct cp500 *cp500)
    {
    struct device *dev = &cp500.pci_dev.dev;
    let mut present: u8 = ioread8(cp500.system_startup_addr + CP500_PRESENT_REG);
    if (cp500_register_i2c(cp500))
    dev_warn(dev, "Failed to register I2C!\n");
    if (present & CP500_PRESENT_FAN0)
    if (cp500_register_fan(cp500))
    dev_warn(dev, "Failed to register fan!\n");
    if (cp500_register_batt(cp500))
    dev_warn(dev, "Failed to register battery!\n");
    if (cp500.devs.uart0_rfb.size &&
    cp500.devs.uart0_rfb.msix < cp500.msix_num) {
    int irq = pci_irq_vector(cp500.pci_dev,
    cp500.devs.uart0_rfb.msix);
    if (cp500_register_uart(cp500, &cp500.uart0_rfb, "rs485-uart",
    &cp500.devs.uart0_rfb, irq))
    dev_warn(dev, "Failed to register RFB UART!\n");
    }
    if (cp500.devs.uart1_dbg.size &&
    cp500.devs.uart1_dbg.msix < cp500.msix_num) {
    int irq = pci_irq_vector(cp500.pci_dev,
    cp500.devs.uart1_dbg.msix);
    if (cp500_register_uart(cp500, &cp500.uart1_dbg, "rs232-uart",
    &cp500.devs.uart1_dbg, irq))
    dev_warn(dev, "Failed to register debug UART!\n");
    }
    if (cp500.devs.uart2_si1.size &&
    cp500.devs.uart2_si1.msix < cp500.msix_num) {
    int irq = pci_irq_vector(cp500.pci_dev,
    cp500.devs.uart2_si1.msix);
    if (cp500_register_uart(cp500, &cp500.uart2_si1, "uart",
    &cp500.devs.uart2_si1, irq))
    dev_warn(dev, "Failed to register SI1 UART!\n");
    }
    }
#[no_mangle]
unsafe extern "C" fn cp500_unregister_dev(auxdev: *mut auxiliary_device) {
    static void cp500_unregister_dev(struct auxiliary_device *auxdev)
    {
    auxiliary_device_delete(auxdev);
    auxiliary_device_uninit(auxdev);
    }
#[no_mangle]
unsafe extern "C" fn cp500_unregister_auxiliary_devs(cp500: *mut cp500) {
    static void cp500_unregister_auxiliary_devs(struct cp500 *cp500)
    {
    if (cp500.spi) {
    cp500_unregister_dev(&cp500.spi.auxdev);
    cp500.spi = core::ptr::null_mut();
    }
    if (cp500.i2c) {
    cp500_unregister_dev(&cp500.i2c.auxdev);
    cp500.i2c = core::ptr::null_mut();
    }
    if (cp500.fan) {
    cp500_unregister_dev(&cp500.fan.auxdev);
    cp500.fan = core::ptr::null_mut();
    }
    if (cp500.batt) {
    cp500_unregister_dev(&cp500.batt.auxdev);
    cp500.batt = core::ptr::null_mut();
    }
    if (cp500.uart0_rfb) {
    cp500_unregister_dev(&cp500.uart0_rfb.auxdev);
    cp500.uart0_rfb = core::ptr::null_mut();
    }
    if (cp500.uart1_dbg) {
    cp500_unregister_dev(&cp500.uart1_dbg.auxdev);
    cp500.uart1_dbg = core::ptr::null_mut();
    }
    if (cp500.uart2_si1) {
    cp500_unregister_dev(&cp500.uart2_si1.auxdev);
    cp500.uart2_si1 = core::ptr::null_mut();
    }
    }
#[no_mangle]
unsafe extern "C" fn cp500_axi_handler(irq: c_int, dev: *mut c_void) -> irqreturn_t {
    static irqreturn_t cp500_axi_handler(int irq, void *dev)
    {
    struct cp500 *cp500 = dev;
    let mut axi_address: u32 = ioread32(cp500.system_startup_addr + CP500_AXI_REG);
//
// FPGA signals AXI response error, print AXI address to indicate which
// IP core was affected
//
    dev_err(&cp500.pci_dev.dev, "AXI response error at 0x%08x\n",
    axi_address);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn cp500_enable(cp500: *mut cp500) -> c_int {
    static int cp500_enable(struct cp500 *cp500)
    {
    let mut axi_irq: c_int = -1;
    int ret;
    if (cp500.msix_num > CP500_NUM_MSIX_NO_AXI) {
    axi_irq = pci_irq_vector(cp500.pci_dev, CP500_AXI_MSIX);
    ret = request_irq(axi_irq, cp500_axi_handler, 0,
    CP500, cp500);
    if (ret != 0) {
    dev_err(&cp500.pci_dev.dev,
    "Failed to register AXI response error!\n");
    return ret;
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cp500_disable(cp500: *mut cp500) {
    static void cp500_disable(struct cp500 *cp500)
    {
    int axi_irq;
    if (cp500.msix_num > CP500_NUM_MSIX_NO_AXI) {
    axi_irq = pci_irq_vector(cp500.pci_dev, CP500_AXI_MSIX);
    free_irq(axi_irq, cp500);
    }
    }
#[no_mangle]
unsafe extern "C" fn cp500_probe(pci_dev: *mut pci_dev, id: *const pci_device_id) -> c_int {
    static int cp500_probe(struct pci_dev *pci_dev, const struct pci_device_id *id)
    {
    struct device *dev = &pci_dev.dev;
    struct resource startup;
    struct cp500 *cp500;
    u32 cp500_vers;
    char buf[64];
    int ret;
    cp500 = devm_kzalloc(dev, sizeof(*cp500), GFP_KERNEL);
    if (!cp500)
    return -ENOMEM;
    cp500.pci_dev = pci_dev;
    cp500.sys_hwbase = pci_resource_start(pci_dev, CP500_SYS_BAR);
    cp500.ecm_hwbase = pci_resource_start(pci_dev, CP500_ECM_BAR);
    if (!cp500.sys_hwbase || !cp500.ecm_hwbase)
    return -ENODEV;
    if (CP500_IS_CP035(cp500))
    cp500.devs = &cp035_devices;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: CP500_IS_CP505(cp500)) -> else {
    else if (CP500_IS_CP505(cp500))
    cp500.devs = &cp505_devices;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: CP500_IS_CP520(cp500)) -> else {
    else if (CP500_IS_CP520(cp500))
    cp500.devs = &cp520_devices;
    else
    return -ENODEV;
    ret = pcim_enable_device(pci_dev);
    if (ret)
    return ret;
    pci_set_master(pci_dev);
    startup = *pci_resource_n(pci_dev, CP500_SYS_BAR);
    startup.end = startup.start + cp500.devs.startup.size - 1;
    cp500.system_startup_addr = devm_ioremap_resource(&pci_dev.dev,
    &startup);
    if (IS_ERR(cp500.system_startup_addr))
    return PTR_ERR(cp500.system_startup_addr);
    cp500.msix_num = pci_alloc_irq_vectors(pci_dev, CP500_NUM_MSIX_NO_MMI,
    CP500_NUM_MSIX, PCI_IRQ_MSIX);
    if (cp500.msix_num < CP500_NUM_MSIX_NO_MMI) {
    dev_err(&pci_dev.dev,
    "Hardware does not support enough MSI-X interrupts\n");
    return -ENODEV;
    }
    cp500_vers = ioread32(cp500.system_startup_addr + CP500_VERSION_REG);
    cp500.version.major = (cp500_vers & 0xff);
    cp500.version.minor = (cp500_vers >> 8) & 0xff;
    cp500.version.build = (cp500_vers >> 16) & 0xffff;
    cp500_get_fpga_version(cp500, buf, sizeof(buf));
    dev_info(&pci_dev.dev, "FPGA version %s", buf);
    pci_set_drvdata(pci_dev, cp500);
    cp500.nvmem_notifier.notifier_call = cp500_nvmem;
    ret = nvmem_register_notifier(&cp500.nvmem_notifier);
    if (ret != 0)
    goto out_free_irq;
    ret = cp500_enable(cp500);
    if (ret != 0)
    goto out_unregister_nvmem;
    cp500_register_auxiliary_devs(cp500);
    return 0;
    out_unregister_nvmem:
    nvmem_unregister_notifier(&cp500.nvmem_notifier);
    out_free_irq:
    pci_free_irq_vectors(pci_dev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn cp500_remove(pci_dev: *mut pci_dev) {
    static void cp500_remove(struct pci_dev *pci_dev)
    {
    struct cp500 *cp500 = pci_get_drvdata(pci_dev);
//
// unregister CPU and user nvmem and put base_nvmem before parent
// auxiliary device of base_nvmem is unregistered
//
    nvmem_unregister_notifier(&cp500.nvmem_notifier);
    cp500_nvmem_unregister(cp500);
    cp500_unregister_auxiliary_devs(cp500);
    cp500_disable(cp500);
    pci_set_drvdata(pci_dev, 0);
    pci_free_irq_vectors(pci_dev);
    }
    static struct pci_device_id cp500_ids[] = {
    { PCI_DEVICE(PCI_VENDOR_ID_KEBA, PCI_DEVICE_ID_KEBA_CP035) },
    { PCI_DEVICE(PCI_VENDOR_ID_KEBA, PCI_DEVICE_ID_KEBA_CP505) },
    { PCI_DEVICE(PCI_VENDOR_ID_KEBA, PCI_DEVICE_ID_KEBA_CP520) },
    { }
    };
    MODULE_DEVICE_TABLE(pci, cp500_ids);
    static struct pci_driver cp500_driver = {
    .name = CP500,
    .id_table = cp500_ids,
    .probe = cp500_probe,
    .remove = cp500_remove,
    .dev_groups = cp500_groups,
    };
    module_pci_driver(cp500_driver);
    MODULE_AUTHOR("Gerhard Engleder <eg@keba.com>");
    MODULE_DESCRIPTION("KEBA CP500 system FPGA driver");
    MODULE_LICENSE("GPL");
