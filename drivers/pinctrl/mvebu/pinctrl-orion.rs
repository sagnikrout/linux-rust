//! Automatically rewritten from C to Rust
//! Source: drivers/pinctrl/mvebu/pinctrl-orion.c
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
// Marvell Orion pinctrl driver based on mvebu pinctrl core
//
// Author: Thomas Petazzoni <thomas.petazzoni@free-electrons.com>
//
// The first 16 MPP pins on Orion are easy to handle: they are
// configured through 2 consecutive registers, located at the base
// address of the MPP device.
//
// However the last 4 MPP pins are handled by a register at offset
// 0x50 from the base address, so it is not consecutive with the first
// two registers.
//

    static void __iomem *mpp_base;
    static void __iomem *high_mpp_base;
    static int orion_mpp_ctrl_get(struct mvebu_mpp_ctrl_data *data,
    unsigned pid, unsigned long *config)
    {
    let mut shift: unsigned = (pid % MVEBU_MPPS_PER_REG) * MVEBU_MPP_BITS;
    if (pid < 16) {
    let mut off: unsigned = (pid / MVEBU_MPPS_PER_REG) * MVEBU_MPP_BITS;
// config = (readl(mpp_base + off) >> shift) & MVEBU_MPP_MASK;
    }
    else {
// config = (readl(high_mpp_base) >> shift) & MVEBU_MPP_MASK;
    }
    return 0;
    }
    static int orion_mpp_ctrl_set(struct mvebu_mpp_ctrl_data *data,
    unsigned pid, unsigned long config)
    {
    let mut shift: unsigned = (pid % MVEBU_MPPS_PER_REG) * MVEBU_MPP_BITS;
    if (pid < 16) {
    let mut off: unsigned = (pid / MVEBU_MPPS_PER_REG) * MVEBU_MPP_BITS;
    let mut reg: u32 = readl(mpp_base + off) & ~(MVEBU_MPP_MASK << shift);
    writel(reg | (config << shift), mpp_base + off);
    }
    else {
    let mut reg: u32 = readl(high_mpp_base) & ~(MVEBU_MPP_MASK << shift);
    writel(reg | (config << shift), high_mpp_base);
    }
    return 0;
    }

    ((f5181 << 0) | (f5182 << 1) | (f5281 << 2))
    enum orion_variant {
    V_5181  = V(1, 0, 0),
    V_5182  = V(0, 1, 0),
    V_5281  = V(0, 0, 1),
    V_ALL   = V(1, 1, 1),
    };
    static struct mvebu_mpp_mode orion_mpp_modes[] = {
    MPP_MODE(0,
    MPP_VAR_FUNCTION(0x0, "pcie", "rstout",    V_ALL),
    MPP_VAR_FUNCTION(0x2, "pci", "req2",       V_ALL),
    MPP_VAR_FUNCTION(0x3, "gpio", core::ptr::null_mut(),        V_ALL)),
    MPP_MODE(1,
    MPP_VAR_FUNCTION(0x0, "gpio", core::ptr::null_mut(),        V_ALL),
    MPP_VAR_FUNCTION(0x2, "pci", "gnt2",       V_ALL)),
    MPP_MODE(2,
    MPP_VAR_FUNCTION(0x0, "gpio", core::ptr::null_mut(),        V_ALL),
    MPP_VAR_FUNCTION(0x2, "pci", "req3",       V_ALL),
    MPP_VAR_FUNCTION(0x3, "pci-1", "pme",      V_ALL)),
    MPP_MODE(3,
    MPP_VAR_FUNCTION(0x0, "gpio", core::ptr::null_mut(),        V_ALL),
    MPP_VAR_FUNCTION(0x2, "pci", "gnt3",       V_ALL)),
    MPP_MODE(4,
    MPP_VAR_FUNCTION(0x0, "gpio", core::ptr::null_mut(),        V_ALL),
    MPP_VAR_FUNCTION(0x2, "pci", "req4",       V_ALL),
    MPP_VAR_FUNCTION(0x4, "bootnand", "re",    V_5182 | V_5281),
    MPP_VAR_FUNCTION(0x5, "sata0", "prsnt",    V_5182)),
    MPP_MODE(5,
    MPP_VAR_FUNCTION(0x0, "gpio", core::ptr::null_mut(),        V_ALL),
    MPP_VAR_FUNCTION(0x2, "pci", "gnt4",       V_ALL),
    MPP_VAR_FUNCTION(0x4, "bootnand", "we",    V_5182 | V_5281),
    MPP_VAR_FUNCTION(0x5, "sata1", "prsnt",    V_5182)),
    MPP_MODE(6,
    MPP_VAR_FUNCTION(0x0, "gpio", core::ptr::null_mut(),        V_ALL),
    MPP_VAR_FUNCTION(0x2, "pci", "req5",       V_ALL),
    MPP_VAR_FUNCTION(0x4, "nand", "re0",       V_5182 | V_5281),
    MPP_VAR_FUNCTION(0x5, "pci-1", "clk",      V_5181),
    MPP_VAR_FUNCTION(0x5, "sata0", "act",      V_5182)),
    MPP_MODE(7,
    MPP_VAR_FUNCTION(0x0, "gpio", core::ptr::null_mut(),        V_ALL),
    MPP_VAR_FUNCTION(0x2, "pci", "gnt5",       V_ALL),
    MPP_VAR_FUNCTION(0x4, "nand", "we0",       V_5182 | V_5281),
    MPP_VAR_FUNCTION(0x5, "pci-1", "clk",      V_5181),
    MPP_VAR_FUNCTION(0x5, "sata1", "act",      V_5182)),
    MPP_MODE(8,
    MPP_VAR_FUNCTION(0x0, "gpio", core::ptr::null_mut(),        V_ALL),
    MPP_VAR_FUNCTION(0x1, "ge", "col",         V_ALL)),
    MPP_MODE(9,
    MPP_VAR_FUNCTION(0x0, "gpio", core::ptr::null_mut(),        V_ALL),
    MPP_VAR_FUNCTION(0x1, "ge", "rxerr",       V_ALL)),
    MPP_MODE(10,
    MPP_VAR_FUNCTION(0x0, "gpio", core::ptr::null_mut(),        V_ALL),
    MPP_VAR_FUNCTION(0x1, "ge", "crs",         V_ALL)),
    MPP_MODE(11,
    MPP_VAR_FUNCTION(0x0, "gpio", core::ptr::null_mut(),        V_ALL),
    MPP_VAR_FUNCTION(0x1, "ge", "txerr",       V_ALL)),
    MPP_MODE(12,
    MPP_VAR_FUNCTION(0x0, "gpio", core::ptr::null_mut(),        V_ALL),
    MPP_VAR_FUNCTION(0x1, "ge", "txd4",        V_ALL),
    MPP_VAR_FUNCTION(0x4, "nand", "re1",       V_5182 | V_5281),
    MPP_VAR_FUNCTION(0x5, "sata0", "ledprsnt", V_5182)),
    MPP_MODE(13,
    MPP_VAR_FUNCTION(0x0, "gpio", core::ptr::null_mut(),        V_ALL),
    MPP_VAR_FUNCTION(0x1, "ge", "txd5",        V_ALL),
    MPP_VAR_FUNCTION(0x4, "nand", "we1",       V_5182 | V_5281),
    MPP_VAR_FUNCTION(0x5, "sata1", "ledprsnt", V_5182)),
    MPP_MODE(14,
    MPP_VAR_FUNCTION(0x0, "gpio", core::ptr::null_mut(),        V_ALL),
    MPP_VAR_FUNCTION(0x1, "ge", "txd6",        V_ALL),
    MPP_VAR_FUNCTION(0x4, "nand", "re2",       V_5182 | V_5281),
    MPP_VAR_FUNCTION(0x5, "sata0", "ledact",   V_5182)),
    MPP_MODE(15,
    MPP_VAR_FUNCTION(0x0, "gpio", core::ptr::null_mut(),        V_ALL),
    MPP_VAR_FUNCTION(0x1, "ge", "txd7",        V_ALL),
    MPP_VAR_FUNCTION(0x4, "nand", "we2",       V_5182 | V_5281),
    MPP_VAR_FUNCTION(0x5, "sata1", "ledact",   V_5182)),
    MPP_MODE(16,
    MPP_VAR_FUNCTION(0x0, "uart1", "rxd",      V_5182 | V_5281),
    MPP_VAR_FUNCTION(0x1, "ge", "rxd4",        V_ALL),
    MPP_VAR_FUNCTION(0x5, "gpio", core::ptr::null_mut(),        V_5182)),
    MPP_MODE(17,
    MPP_VAR_FUNCTION(0x0, "uart1", "txd",      V_5182 | V_5281),
    MPP_VAR_FUNCTION(0x1, "ge", "rxd5",        V_ALL),
    MPP_VAR_FUNCTION(0x5, "gpio", core::ptr::null_mut(),        V_5182)),
    MPP_MODE(18,
    MPP_VAR_FUNCTION(0x0, "uart1", "cts",      V_5182 | V_5281),
    MPP_VAR_FUNCTION(0x1, "ge", "rxd6",        V_ALL),
    MPP_VAR_FUNCTION(0x5, "gpio", core::ptr::null_mut(),        V_5182)),
    MPP_MODE(19,
    MPP_VAR_FUNCTION(0x0, "uart1", "rts",      V_5182 | V_5281),
    MPP_VAR_FUNCTION(0x1, "ge", "rxd7",        V_ALL),
    MPP_VAR_FUNCTION(0x5, "gpio", core::ptr::null_mut(),        V_5182)),
    };
    static const struct mvebu_mpp_ctrl orion_mpp_controls[] = {
    MPP_FUNC_CTRL(0, 19, core::ptr::null_mut(), orion_mpp_ctrl),
    };
    static struct pinctrl_gpio_range mv88f5181_gpio_ranges[] = {
    MPP_GPIO_RANGE(0, 0, 0, 16),
    };
    static struct pinctrl_gpio_range mv88f5182_gpio_ranges[] = {
    MPP_GPIO_RANGE(0, 0, 0, 19),
    };
    static struct pinctrl_gpio_range mv88f5281_gpio_ranges[] = {
    MPP_GPIO_RANGE(0, 0, 0, 16),
    };
    static struct mvebu_pinctrl_soc_info mv88f5181_info = {
    .variant = V_5181,
    .controls = orion_mpp_controls,
    .ncontrols = ARRAY_SIZE(orion_mpp_controls),
    .modes = orion_mpp_modes,
    .nmodes = ARRAY_SIZE(orion_mpp_modes),
    .gpioranges = mv88f5181_gpio_ranges,
    .ngpioranges = ARRAY_SIZE(mv88f5181_gpio_ranges),
    };
    static struct mvebu_pinctrl_soc_info mv88f5182_info = {
    .variant = V_5182,
    .controls = orion_mpp_controls,
    .ncontrols = ARRAY_SIZE(orion_mpp_controls),
    .modes = orion_mpp_modes,
    .nmodes = ARRAY_SIZE(orion_mpp_modes),
    .gpioranges = mv88f5182_gpio_ranges,
    .ngpioranges = ARRAY_SIZE(mv88f5182_gpio_ranges),
    };
    static struct mvebu_pinctrl_soc_info mv88f5281_info = {
    .variant = V_5281,
    .controls = orion_mpp_controls,
    .ncontrols = ARRAY_SIZE(orion_mpp_controls),
    .modes = orion_mpp_modes,
    .nmodes = ARRAY_SIZE(orion_mpp_modes),
    .gpioranges = mv88f5281_gpio_ranges,
    .ngpioranges = ARRAY_SIZE(mv88f5281_gpio_ranges),
    };
//
// There are multiple variants of the Orion SoCs, but in terms of pin
// muxing, they are identical.
//
    static const struct of_device_id orion_pinctrl_of_match[] = {
    { .compatible = "marvell,88f5181-pinctrl", .data = &mv88f5181_info },
    { .compatible = "marvell,88f5181l-pinctrl", .data = &mv88f5181_info },
    { .compatible = "marvell,88f5182-pinctrl", .data = &mv88f5182_info },
    { .compatible = "marvell,88f5281-pinctrl", .data = &mv88f5281_info },
    { }
    };
#[no_mangle]
unsafe extern "C" fn orion_pinctrl_probe(pdev: *mut platform_device) -> c_int {
    static int orion_pinctrl_probe(struct platform_device *pdev)
    {
    pdev.dev.platform_data = (void*)device_get_match_data(&pdev.dev);
    mpp_base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(mpp_base))
    return PTR_ERR(mpp_base);
    high_mpp_base = devm_platform_ioremap_resource(pdev, 1);
    if (IS_ERR(high_mpp_base))
    return PTR_ERR(high_mpp_base);
    return mvebu_pinctrl_probe(pdev);
    }
    static struct platform_driver orion_pinctrl_driver = {
    .driver = {
    .name = "orion-pinctrl",
    .of_match_table = of_match_ptr(orion_pinctrl_of_match),
    },
    .probe = orion_pinctrl_probe,
    };
    builtin_platform_driver(orion_pinctrl_driver);
