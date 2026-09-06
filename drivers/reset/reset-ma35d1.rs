//! Automatically rewritten from C to Rust
//! Source: drivers/reset/reset-ma35d1.c
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
// Copyright (C) 2023 Nuvoton Technology Corp.
// Author: Chi-Fang Li <cfli0@nuvoton.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ma35d1_reset_data {
    pub rcdev: reset_controller_dev,
    pub base: *mut void __iomem,
// protect registers against concurrent read-modify-write
    pub lock: spinlock_t,
}

    static const struct {
    u32 reg_ofs;
    u32 bit;
    } ma35d1_reset_map[] = {
    [MA35D1_RESET_CHIP] =    {0x20, 0},
    [MA35D1_RESET_CA35CR0] = {0x20, 1},
    [MA35D1_RESET_CA35CR1] = {0x20, 2},
    [MA35D1_RESET_CM4] =     {0x20, 3},
    [MA35D1_RESET_PDMA0] =   {0x20, 4},
    [MA35D1_RESET_PDMA1] =   {0x20, 5},
    [MA35D1_RESET_PDMA2] =   {0x20, 6},
    [MA35D1_RESET_PDMA3] =   {0x20, 7},
    [MA35D1_RESET_DISP] =    {0x20, 9},
    [MA35D1_RESET_VCAP0] =   {0x20, 10},
    [MA35D1_RESET_VCAP1] =   {0x20, 11},
    [MA35D1_RESET_GFX] =     {0x20, 12},
    [MA35D1_RESET_VDEC] =    {0x20, 13},
    [MA35D1_RESET_WHC0] =    {0x20, 14},
    [MA35D1_RESET_WHC1] =    {0x20, 15},
    [MA35D1_RESET_GMAC0] =   {0x20, 16},
    [MA35D1_RESET_GMAC1] =   {0x20, 17},
    [MA35D1_RESET_HWSEM] =   {0x20, 18},
    [MA35D1_RESET_EBI] =     {0x20, 19},
    [MA35D1_RESET_HSUSBH0] = {0x20, 20},
    [MA35D1_RESET_HSUSBH1] = {0x20, 21},
    [MA35D1_RESET_HSUSBD] =  {0x20, 22},
    [MA35D1_RESET_USBHL] =   {0x20, 23},
    [MA35D1_RESET_SDH0] =    {0x20, 24},
    [MA35D1_RESET_SDH1] =    {0x20, 25},
    [MA35D1_RESET_NAND] =    {0x20, 26},
    [MA35D1_RESET_GPIO] =    {0x20, 27},
    [MA35D1_RESET_MCTLP] =   {0x20, 28},
    [MA35D1_RESET_MCTLC] =   {0x20, 29},
    [MA35D1_RESET_DDRPUB] =  {0x20, 30},
    [MA35D1_RESET_TMR0] =    {0x24, 2},
    [MA35D1_RESET_TMR1] =    {0x24, 3},
    [MA35D1_RESET_TMR2] =    {0x24, 4},
    [MA35D1_RESET_TMR3] =    {0x24, 5},
    [MA35D1_RESET_I2C0] =    {0x24, 8},
    [MA35D1_RESET_I2C1] =    {0x24, 9},
    [MA35D1_RESET_I2C2] =    {0x24, 10},
    [MA35D1_RESET_I2C3] =    {0x24, 11},
    [MA35D1_RESET_QSPI0] =   {0x24, 12},
    [MA35D1_RESET_SPI0] =    {0x24, 13},
    [MA35D1_RESET_SPI1] =    {0x24, 14},
    [MA35D1_RESET_SPI2] =    {0x24, 15},
    [MA35D1_RESET_UART0] =   {0x24, 16},
    [MA35D1_RESET_UART1] =   {0x24, 17},
    [MA35D1_RESET_UART2] =   {0x24, 18},
    [MA35D1_RESET_UART3] =   {0x24, 19},
    [MA35D1_RESET_UART4] =   {0x24, 20},
    [MA35D1_RESET_UART5] =   {0x24, 21},
    [MA35D1_RESET_UART6] =   {0x24, 22},
    [MA35D1_RESET_UART7] =   {0x24, 23},
    [MA35D1_RESET_CANFD0] =  {0x24, 24},
    [MA35D1_RESET_CANFD1] =  {0x24, 25},
    [MA35D1_RESET_EADC0] =   {0x24, 28},
    [MA35D1_RESET_I2S0] =    {0x24, 29},
    [MA35D1_RESET_SC0] =     {0x28, 0},
    [MA35D1_RESET_SC1] =     {0x28, 1},
    [MA35D1_RESET_QSPI1] =   {0x28, 4},
    [MA35D1_RESET_SPI3] =    {0x28, 6},
    [MA35D1_RESET_EPWM0] =   {0x28, 16},
    [MA35D1_RESET_EPWM1] =   {0x28, 17},
    [MA35D1_RESET_QEI0] =    {0x28, 22},
    [MA35D1_RESET_QEI1] =    {0x28, 23},
    [MA35D1_RESET_ECAP0] =   {0x28, 26},
    [MA35D1_RESET_ECAP1] =   {0x28, 27},
    [MA35D1_RESET_CANFD2] =  {0x28, 28},
    [MA35D1_RESET_ADC0] =    {0x28, 31},
    [MA35D1_RESET_TMR4] =    {0x2C, 0},
    [MA35D1_RESET_TMR5] =    {0x2C, 1},
    [MA35D1_RESET_TMR6] =    {0x2C, 2},
    [MA35D1_RESET_TMR7] =    {0x2C, 3},
    [MA35D1_RESET_TMR8] =    {0x2C, 4},
    [MA35D1_RESET_TMR9] =    {0x2C, 5},
    [MA35D1_RESET_TMR10] =   {0x2C, 6},
    [MA35D1_RESET_TMR11] =   {0x2C, 7},
    [MA35D1_RESET_UART8] =   {0x2C, 8},
    [MA35D1_RESET_UART9] =   {0x2C, 9},
    [MA35D1_RESET_UART10] =  {0x2C, 10},
    [MA35D1_RESET_UART11] =  {0x2C, 11},
    [MA35D1_RESET_UART12] =  {0x2C, 12},
    [MA35D1_RESET_UART13] =  {0x2C, 13},
    [MA35D1_RESET_UART14] =  {0x2C, 14},
    [MA35D1_RESET_UART15] =  {0x2C, 15},
    [MA35D1_RESET_UART16] =  {0x2C, 16},
    [MA35D1_RESET_I2S1] =    {0x2C, 17},
    [MA35D1_RESET_I2C4] =    {0x2C, 18},
    [MA35D1_RESET_I2C5] =    {0x2C, 19},
    [MA35D1_RESET_EPWM2] =   {0x2C, 20},
    [MA35D1_RESET_ECAP2] =   {0x2C, 21},
    [MA35D1_RESET_QEI2] =    {0x2C, 22},
    [MA35D1_RESET_CANFD3] =  {0x2C, 23},
    [MA35D1_RESET_KPI] =     {0x2C, 24},
    [MA35D1_RESET_GIC] =     {0x2C, 28},
    [MA35D1_RESET_SSMCC] =   {0x2C, 30},
    [MA35D1_RESET_SSPCC] =   {0x2C, 31}
    };
#[no_mangle]
unsafe extern "C" fn ma35d1_restart_handler(sys_off_data: *mut sys_off_data) -> c_int {
    static int ma35d1_restart_handler(struct sys_off_data *sys_off_data)
    {
    struct ma35d1_reset_data *data = sys_off_data.cb_data;
    let mut id: u32 = MA35D1_RESET_CHIP;
    writel_relaxed(BIT(ma35d1_reset_map[id].bit),
    data.base + ma35d1_reset_map[id].reg_ofs);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ma35d1_reset_update(rcdev: *mut reset_controller_dev, id: c_ulong, assert: bool) -> c_int {
    static int ma35d1_reset_update(struct reset_controller_dev *rcdev, unsigned long id, bool assert)
    {
    struct ma35d1_reset_data *data = container_of(rcdev, struct ma35d1_reset_data, rcdev);
    unsigned long flags;
    u32 reg;
    if (WARN_ON_ONCE(id >= ARRAY_SIZE(ma35d1_reset_map)))
    return -EINVAL;
    spin_lock_irqsave(&data.lock, flags);
    reg = readl_relaxed(data.base + ma35d1_reset_map[id].reg_ofs);
    if (assert)
    reg |= BIT(ma35d1_reset_map[id].bit);
    else
    reg &= ~(BIT(ma35d1_reset_map[id].bit));
    writel_relaxed(reg, data.base + ma35d1_reset_map[id].reg_ofs);
    spin_unlock_irqrestore(&data.lock, flags);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ma35d1_reset_assert(rcdev: *mut reset_controller_dev, id: c_ulong) -> c_int {
    static int ma35d1_reset_assert(struct reset_controller_dev *rcdev, unsigned long id)
    {
    return ma35d1_reset_update(rcdev, id, true);
    }
#[no_mangle]
unsafe extern "C" fn ma35d1_reset_deassert(rcdev: *mut reset_controller_dev, id: c_ulong) -> c_int {
    static int ma35d1_reset_deassert(struct reset_controller_dev *rcdev, unsigned long id)
    {
    return ma35d1_reset_update(rcdev, id, false);
    }
#[no_mangle]
unsafe extern "C" fn ma35d1_reset_status(rcdev: *mut reset_controller_dev, id: c_ulong) -> c_int {
    static int ma35d1_reset_status(struct reset_controller_dev *rcdev, unsigned long id)
    {
    struct ma35d1_reset_data *data = container_of(rcdev, struct ma35d1_reset_data, rcdev);
    u32 reg;
    if (WARN_ON_ONCE(id >= ARRAY_SIZE(ma35d1_reset_map)))
    return -EINVAL;
    reg = readl_relaxed(data.base + ma35d1_reset_map[id].reg_ofs);
    return !!(reg & BIT(ma35d1_reset_map[id].bit));
    }
    static const struct reset_control_ops ma35d1_reset_ops = {
    .assert = ma35d1_reset_assert,
    .deassert = ma35d1_reset_deassert,
    .status = ma35d1_reset_status,
    };
    static const struct of_device_id ma35d1_reset_dt_ids[] = {
    { .compatible = "nuvoton,ma35d1-reset" },
    { },
    };
#[no_mangle]
unsafe extern "C" fn ma35d1_reset_probe(pdev: *mut platform_device) -> c_int {
    static int ma35d1_reset_probe(struct platform_device *pdev)
    {
    struct ma35d1_reset_data *reset_data;
    struct device *dev = &pdev.dev;
    int err;
    if (!pdev.dev.of_node) {
    dev_err(&pdev.dev, "Device tree node not found\n");
    return -EINVAL;
    }
    reset_data = devm_kzalloc(dev, sizeof(*reset_data), GFP_KERNEL);
    if (!reset_data)
    return -ENOMEM;
    reset_data.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(reset_data.base))
    return PTR_ERR(reset_data.base);
    reset_data.rcdev.owner = THIS_MODULE;
    reset_data.rcdev.nr_resets = MA35D1_RESET_COUNT;
    reset_data.rcdev.ops = &ma35d1_reset_ops;
    reset_data.rcdev.of_node = dev.of_node;
    spin_lock_init(&reset_data.lock);
    err = devm_register_sys_off_handler(dev, SYS_OFF_MODE_RESTART, 192,
    ma35d1_restart_handler, reset_data);
    if (err)
    dev_warn(&pdev.dev, "failed to register restart handler\n");
    return devm_reset_controller_register(dev, &reset_data.rcdev);
    }
    static struct platform_driver ma35d1_reset_driver = {
    .probe = ma35d1_reset_probe,
    .driver = {
    .name = "ma35d1-reset",
    .of_match_table	= ma35d1_reset_dt_ids,
    },
    };
    builtin_platform_driver(ma35d1_reset_driver);
