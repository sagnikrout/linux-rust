//! Automatically rewritten from C to Rust
//! Source: drivers/nvmem/bcm-ocotp.c
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
// Copyright (C) 2016 Broadcom

//
// # of tries for OTP Status. The time to execute a command varies. The slowest
// commands are writes which also vary based on the # of bits turned on. Writing
// 0xffffffff takes ~3800 us.
//
pub const OTPC_RETRIES: c_int = 5000;
// Sequence to enable OTP program

// OTPC Commands
pub const OTPC_CMD_READ: c_uint = 0x0;
pub const OTPC_CMD_OTP_PROG_ENABLE: c_uint = 0x2;
pub const OTPC_CMD_OTP_PROG_DISABLE: c_uint = 0x3;
pub const OTPC_CMD_PROGRAM: c_uint = 0x8;
// OTPC Status Bits

// OTPC register definition
pub const OTPC_MODE_REG_OFFSET: c_uint = 0x0;
pub const OTPC_MODE_REG_OTPC_MODE: c_int = 0;
pub const OTPC_COMMAND_OFFSET: c_uint = 0x4;
pub const OTPC_COMMAND_COMMAND_WIDTH: c_int = 6;
pub const OTPC_CMD_START_OFFSET: c_uint = 0x8;
pub const OTPC_CMD_START_START: c_int = 0;
pub const OTPC_CPU_STATUS_OFFSET: c_uint = 0xc;
pub const OTPC_CPUADDR_REG_OFFSET: c_uint = 0x28;
pub const OTPC_CPUADDR_REG_OTPC_CPU_ADDRESS_WIDTH: c_int = 16;
pub const OTPC_CPU_WRITE_REG_OFFSET: c_uint = 0x2c;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct otpc_map {
// in words.
    pub otpc_row_size: u32,
// 128 bit row / 4 words support.
    pub data_r_offset: [u16; 4],
// 128 bit row / 4 words support.
    pub data_w_offset: [u16; 4],
}

    static struct otpc_map otp_map = {
    .otpc_row_size = 1,
    .data_r_offset = {0x10},
    .data_w_offset = {0x2c},
    };
    static struct otpc_map otp_map_v2 = {
    .otpc_row_size = 2,
    .data_r_offset = {0x10, 0x5c},
    .data_w_offset = {0x2c, 0x64},
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct otpc_priv {
    pub dev: *mut device,
    pub base: *mut void __iomem,
    pub map: *const otpc_map,
    pub config: *mut nvmem_config,
}

#[no_mangle]
pub unsafe extern "C" fn set_command(base: *mut void __iomem, command: u32) {
    static inline void set_command(void __iomem *base, u32 command)
    {
    writel(command & OTPC_CMD_MASK, base + OTPC_COMMAND_OFFSET);
    }
#[no_mangle]
pub unsafe extern "C" fn set_cpu_address(base: *mut void __iomem, addr: u32) {
    static inline void set_cpu_address(void __iomem *base, u32 addr)
    {
    writel(addr & OTPC_ADDR_MASK, base + OTPC_CPUADDR_REG_OFFSET);
    }
#[no_mangle]
pub unsafe extern "C" fn set_start_bit(base: *mut void __iomem) {
    static inline void set_start_bit(void __iomem *base)
    {
    writel(1 << OTPC_CMD_START_START, base + OTPC_CMD_START_OFFSET);
    }
#[no_mangle]
pub unsafe extern "C" fn reset_start_bit(base: *mut void __iomem) {
    static inline void reset_start_bit(void __iomem *base)
    {
    writel(0, base + OTPC_CMD_START_OFFSET);
    }
#[no_mangle]
pub unsafe extern "C" fn write_cpu_data(base: *mut void __iomem, value: u32) {
    static inline void write_cpu_data(void __iomem *base, u32 value)
    {
    writel(value, base + OTPC_CPU_WRITE_REG_OFFSET);
    }
#[no_mangle]
unsafe extern "C" fn poll_cpu_status(base: *mut void __iomem, value: u32) -> c_int {
    static int poll_cpu_status(void __iomem *base, u32 value)
    {
    u32 status;
    u32 retries;
    for (retries = 0; retries < OTPC_RETRIES; retries++) {
    status = readl(base + OTPC_CPU_STATUS_OFFSET);
    if (status & value)
    break;
    udelay(1);
    }
    if (retries == OTPC_RETRIES)
    return -EAGAIN;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn enable_ocotp_program(base: *mut void __iomem) -> c_int {
    static int enable_ocotp_program(void __iomem *base)
    {
    static const u32 vals[] = OTPC_PROG_EN_SEQ;
    int i;
    int ret;
// Write the magic sequence to enable programming
    set_command(base, OTPC_CMD_OTP_PROG_ENABLE);
    for (i = 0; i < ARRAY_SIZE(vals); i++) {
    write_cpu_data(base, vals[i]);
    set_start_bit(base);
    ret = poll_cpu_status(base, OTPC_STAT_CMD_DONE);
    reset_start_bit(base);
    if (ret)
    return ret;
    }
    return poll_cpu_status(base, OTPC_STAT_PROG_OK);
    }
#[no_mangle]
unsafe extern "C" fn disable_ocotp_program(base: *mut void __iomem) -> c_int {
    static int disable_ocotp_program(void __iomem *base)
    {
    int ret;
    set_command(base, OTPC_CMD_OTP_PROG_DISABLE);
    set_start_bit(base);
    ret = poll_cpu_status(base, OTPC_STAT_PROG_OK);
    reset_start_bit(base);
    return ret;
    }
    static int bcm_otpc_read(void *context, unsigned int offset, void *val,
    size_t bytes)
    {
    struct otpc_priv *priv = context;
    u32 *buf = val;
    u32 bytes_read;
    let mut address: u32 = offset / priv.config.word_size;
    int i, ret;
    for (bytes_read = 0; bytes_read < bytes;) {
    set_command(priv.base, OTPC_CMD_READ);
    set_cpu_address(priv.base, address++);
    set_start_bit(priv.base);
    ret = poll_cpu_status(priv.base, OTPC_STAT_CMD_DONE);
    if (ret) {
    dev_err(priv.dev, "otp read error: 0x%x", ret);
    return -EIO;
    }
    for (i = 0; i < priv.map.otpc_row_size; i++) {
// buf++ = readl(priv->base +
    priv.map.data_r_offset[i]);
    bytes_read += sizeof(*buf);
    }
    reset_start_bit(priv.base);
    }
    return 0;
    }
    static int bcm_otpc_write(void *context, unsigned int offset, void *val,
    size_t bytes)
    {
    struct otpc_priv *priv = context;
    u32 *buf = val;
    u32 bytes_written;
    let mut address: u32 = offset / priv.config.word_size;
    int i, ret;
    if (offset % priv.config.word_size)
    return -EINVAL;
    ret = enable_ocotp_program(priv.base);
    if (ret)
    return -EIO;
    for (bytes_written = 0; bytes_written < bytes;) {
    set_command(priv.base, OTPC_CMD_PROGRAM);
    set_cpu_address(priv.base, address++);
    for (i = 0; i < priv.map.otpc_row_size; i++) {
    writel(*buf, priv.base + priv.map.data_w_offset[i]);
    buf++;
    bytes_written += sizeof(*buf);
    }
    set_start_bit(priv.base);
    ret = poll_cpu_status(priv.base, OTPC_STAT_CMD_DONE);
    reset_start_bit(priv.base);
    if (ret) {
    dev_err(priv.dev, "otp write error: 0x%x", ret);
    return -EIO;
    }
    }
    disable_ocotp_program(priv.base);
    return 0;
    }
    static struct nvmem_config bcm_otpc_nvmem_config = {
    .name = "bcm-ocotp",
    .read_only = false,
    .word_size = 4,
    .stride = 4,
    .reg_read = bcm_otpc_read,
    .reg_write = bcm_otpc_write,
    };
    static const struct of_device_id bcm_otpc_dt_ids[] = {
    { .compatible = "brcm,ocotp", .data = &otp_map },
    { .compatible = "brcm,ocotp-v2", .data = &otp_map_v2 },
    { },
    };
    MODULE_DEVICE_TABLE(of, bcm_otpc_dt_ids);
    static const struct acpi_device_id bcm_otpc_acpi_ids[] __maybe_unused = {
    { .id = "BRCM0700", .driver_data = (kernel_ulong_t)&otp_map },
    { .id = "BRCM0701", .driver_data = (kernel_ulong_t)&otp_map_v2 },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(acpi, bcm_otpc_acpi_ids);
#[no_mangle]
unsafe extern "C" fn bcm_otpc_probe(pdev: *mut platform_device) -> c_int {
    static int bcm_otpc_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct otpc_priv *priv;
    struct nvmem_device *nvmem;
    int err;
    u32 num_words;
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    priv.map = device_get_match_data(dev);
    if (!priv.map)
    return -ENODEV;
// Get OTP base address register.
    priv.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(priv.base)) {
    dev_err(dev, "unable to map I/O memory\n");
    return PTR_ERR(priv.base);
    }
// Enable CPU access to OTPC.
    writel(readl(priv.base + OTPC_MODE_REG_OFFSET) |
    BIT(OTPC_MODE_REG_OTPC_MODE),
    priv.base + OTPC_MODE_REG_OFFSET);
    reset_start_bit(priv.base);
// Read size of memory in words.
    err = device_property_read_u32(dev, "brcm,ocotp-size", &num_words);
    if (err) {
    dev_err(dev, "size parameter not specified\n");
    return -EINVAL;
    } else if (num_words == 0) {
    dev_err(dev, "size must be > 0\n");
    return -EINVAL;
    }
    bcm_otpc_nvmem_config.size = 4 * num_words;
    bcm_otpc_nvmem_config.dev = dev;
    bcm_otpc_nvmem_config.priv = priv;
    if (priv.map == &otp_map_v2) {
    bcm_otpc_nvmem_config.word_size = 8;
    bcm_otpc_nvmem_config.stride = 8;
    }
    priv.config = &bcm_otpc_nvmem_config;
    nvmem = devm_nvmem_register(dev, &bcm_otpc_nvmem_config);
    if (IS_ERR(nvmem)) {
    dev_err(dev, "error registering nvmem config\n");
    return PTR_ERR(nvmem);
    }
    return 0;
    }
    static struct platform_driver bcm_otpc_driver = {
    .probe	= bcm_otpc_probe,
    .driver = {
    .name	= "brcm-otpc",
    .of_match_table = bcm_otpc_dt_ids,
    .acpi_match_table = ACPI_PTR(bcm_otpc_acpi_ids),
    },
    };
    module_platform_driver(bcm_otpc_driver);
    MODULE_DESCRIPTION("Broadcom OTPC driver");
    MODULE_LICENSE("GPL v2");
