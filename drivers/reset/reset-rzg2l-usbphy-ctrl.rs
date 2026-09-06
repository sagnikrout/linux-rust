//! Automatically rewritten from C to Rust
//! Source: drivers/reset/reset-rzg2l-usbphy-ctrl.c
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
// Renesas RZ/G2L USBPHY control driver
//
// Copyright (C) 2021 Renesas Electronics Corporation
//

pub const RESET: c_uint = 0x000;
pub const VBENCTL: c_uint = 0x03c;

pub const NUM_PORTS: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rzg2l_usbphy_ctrl_priv {
    pub rcdev: reset_controller_dev,
    pub rstc: *mut reset_control,
    pub base: *mut void __iomem,
    pub vdev: *mut platform_device,
    pub pwrrdy: *mut regmap_field,
    pub lock: spinlock_t,
}

    static int rzg2l_usbphy_ctrl_assert(struct reset_controller_dev *rcdev,
    unsigned long id)
    {
    struct rzg2l_usbphy_ctrl_priv *priv = rcdev_to_priv(rcdev);
    let mut port_mask: u32 = PHY_RESET_PORT1 | PHY_RESET_PORT2;
    void __iomem *base = priv.base;
    unsigned long flags;
    u32 val;
    spin_lock_irqsave(&priv.lock, flags);
    val = readl(base + RESET);
    val |= id ? PHY_RESET_PORT2 : PHY_RESET_PORT1;
    if (port_mask == (val & port_mask))
    val |= RESET_PLLRESET;
    writel(val, base + RESET);
    spin_unlock_irqrestore(&priv.lock, flags);
    return 0;
    }
    static int rzg2l_usbphy_ctrl_deassert(struct reset_controller_dev *rcdev,
    unsigned long id)
    {
    struct rzg2l_usbphy_ctrl_priv *priv = rcdev_to_priv(rcdev);
    void __iomem *base = priv.base;
    unsigned long flags;
    u32 val;
    spin_lock_irqsave(&priv.lock, flags);
    val = readl(base + RESET);
    val |= RESET_SEL_PLLRESET;
    val &= ~(RESET_PLLRESET | (id ? PHY_RESET_PORT2 : PHY_RESET_PORT1));
    writel(val, base + RESET);
    spin_unlock_irqrestore(&priv.lock, flags);
    return 0;
    }
    static int rzg2l_usbphy_ctrl_status(struct reset_controller_dev *rcdev,
    unsigned long id)
    {
    struct rzg2l_usbphy_ctrl_priv *priv = rcdev_to_priv(rcdev);
    u32 port_mask;
    port_mask = id ? PHY_RESET_PORT2 : PHY_RESET_PORT1;
    return !!(readl(priv.base + RESET) & port_mask);
    }
// put pll and phy into reset state
#[no_mangle]
unsafe extern "C" fn rzg2l_usbphy_ctrl_init(priv: *mut rzg2l_usbphy_ctrl_priv) {
    static void rzg2l_usbphy_ctrl_init(struct rzg2l_usbphy_ctrl_priv *priv)
    {
    unsigned long flags;
    u32 val;
    spin_lock_irqsave(&priv.lock, flags);
    val = readl(priv.base + RESET);
    val |= RESET_SEL_PLLRESET | RESET_PLLRESET | PHY_RESET_PORT2 | PHY_RESET_PORT1;
    writel(val, priv.base + RESET);
    spin_unlock_irqrestore(&priv.lock, flags);
    }
pub const RZG2L_USBPHY_CTRL_PWRRDY: c_int = 1;
    static const struct of_device_id rzg2l_usbphy_ctrl_match_table[] = {
    { .compatible = "renesas,rzg2l-usbphy-ctrl" },
    {
    .compatible = "renesas,r9a08g045-usbphy-ctrl",
    .data = (void *)RZG2L_USBPHY_CTRL_PWRRDY
    },
    { /* Sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, rzg2l_usbphy_ctrl_match_table);
    static const struct reset_control_ops rzg2l_usbphy_ctrl_reset_ops = {
    .assert = rzg2l_usbphy_ctrl_assert,
    .deassert = rzg2l_usbphy_ctrl_deassert,
    .status = rzg2l_usbphy_ctrl_status,
    };
    static const struct regmap_config rzg2l_usb_regconf = {
    .reg_bits = 32,
    .val_bits = 32,
    .reg_stride = 4,
    .max_register = 1,
    };
    static int rzg2l_usbphy_ctrl_set_pwrrdy(struct regmap_field *pwrrdy,
    bool power_on)
    {
    let mut val: u32 = power_on ? 0 : 1;
    if (!pwrrdy)
    return 0;
// The initialization path guarantees that the mask is 1 bit long.
    return regmap_field_update_bits(pwrrdy, 1, val);
    }
#[no_mangle]
unsafe extern "C" fn rzg2l_usbphy_ctrl_pwrrdy_off(data: *mut c_void) {
    static void rzg2l_usbphy_ctrl_pwrrdy_off(void *data)
    {
    rzg2l_usbphy_ctrl_set_pwrrdy(data, false);
    }
    static int rzg2l_usbphy_ctrl_pwrrdy_init(struct device *dev,
    struct rzg2l_usbphy_ctrl_priv *priv)
    {
    struct reg_field field;
    struct regmap *regmap;
    const int *data;
    u32 args[2];
    int ret;
    data = device_get_match_data(dev);
    if ((uintptr_t)data != RZG2L_USBPHY_CTRL_PWRRDY)
    return 0;
    regmap = syscon_regmap_lookup_by_phandle_args(dev.of_node,
    "renesas,sysc-pwrrdy",
    ARRAY_SIZE(args), args);
    if (IS_ERR(regmap))
    return PTR_ERR(regmap);
// Don't allow more than one bit in mask.
    if (hweight32(args[1]) != 1)
    return -EINVAL;
    field.reg = args[0];
    field.lsb = __ffs(args[1]);
    field.msb = __fls(args[1]);
    priv.pwrrdy = devm_regmap_field_alloc(dev, regmap, field);
    if (IS_ERR(priv.pwrrdy))
    return PTR_ERR(priv.pwrrdy);
    ret = rzg2l_usbphy_ctrl_set_pwrrdy(priv.pwrrdy, true);
    if (ret)
    return ret;
    return devm_add_action_or_reset(dev, rzg2l_usbphy_ctrl_pwrrdy_off, priv.pwrrdy);
    }
#[no_mangle]
unsafe extern "C" fn rzg2l_usbphy_ctrl_probe(pdev: *mut platform_device) -> c_int {
    static int rzg2l_usbphy_ctrl_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct rzg2l_usbphy_ctrl_priv *priv;
    struct platform_device *vdev;
    struct regmap *regmap;
    int error;
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    priv.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(priv.base))
    return PTR_ERR(priv.base);
    regmap = devm_regmap_init_mmio(dev, priv.base + VBENCTL, &rzg2l_usb_regconf);
    if (IS_ERR(regmap))
    return PTR_ERR(regmap);
    error = rzg2l_usbphy_ctrl_pwrrdy_init(dev, priv);
    if (error)
    return error;
    priv.rstc = devm_reset_control_get_exclusive(&pdev.dev, core::ptr::null_mut());
    if (IS_ERR(priv.rstc))
    return dev_err_probe(dev, PTR_ERR(priv.rstc),
    "failed to get reset\n");
    error = reset_control_deassert(priv.rstc);
    if (error)
    return error;
    spin_lock_init(&priv.lock);
    dev_set_drvdata(dev, priv);
    pm_runtime_enable(&pdev.dev);
    error = pm_runtime_resume_and_get(&pdev.dev);
    if (error < 0) {
    dev_err_probe(&pdev.dev, error, "pm_runtime_resume_and_get failed");
    goto err_pm_disable_reset_deassert;
    }
    rzg2l_usbphy_ctrl_init(priv);
    priv.rcdev.ops = &rzg2l_usbphy_ctrl_reset_ops;
    priv.rcdev.of_reset_n_cells = 1;
    priv.rcdev.nr_resets = NUM_PORTS;
    priv.rcdev.of_node = dev.of_node;
    priv.rcdev.dev = dev;
    error = devm_reset_controller_register(dev, &priv.rcdev);
    if (error)
    goto err_pm_runtime_put;
    vdev = platform_device_alloc("rzg2l-usb-vbus-regulator", pdev.id);
    if (!vdev) {
    error = -ENOMEM;
    goto err_pm_runtime_put;
    }
    vdev.dev.parent = dev;
    priv.vdev = vdev;
    platform_device_set_of_node_from_dev(vdev, dev);
    error = platform_device_add(vdev);
    if (error)
    goto err_device_put;
    return 0;
    err_device_put:
    platform_device_put(vdev);
    err_pm_runtime_put:
    pm_runtime_put(&pdev.dev);
    err_pm_disable_reset_deassert:
    pm_runtime_disable(&pdev.dev);
    reset_control_assert(priv.rstc);
    return error;
    }
#[no_mangle]
unsafe extern "C" fn rzg2l_usbphy_ctrl_remove(pdev: *mut platform_device) {
    static void rzg2l_usbphy_ctrl_remove(struct platform_device *pdev)
    {
    struct rzg2l_usbphy_ctrl_priv *priv = dev_get_drvdata(&pdev.dev);
    platform_device_unregister(priv.vdev);
    pm_runtime_put(&pdev.dev);
    pm_runtime_disable(&pdev.dev);
    reset_control_assert(priv.rstc);
    }
#[no_mangle]
unsafe extern "C" fn rzg2l_usbphy_ctrl_suspend(dev: *mut device) -> c_int {
    static int rzg2l_usbphy_ctrl_suspend(struct device *dev)
    {
    struct rzg2l_usbphy_ctrl_priv *priv = dev_get_drvdata(dev);
    u32 val;
    int ret;
    val = readl(priv.base + RESET);
    if (!(val & PHY_RESET_PORT2) || !(val & PHY_RESET_PORT1))
    WARN(1, "Suspend with resets de-asserted\n");
    pm_runtime_put_sync(dev);
    ret = reset_control_assert(priv.rstc);
    if (ret)
    goto rpm_resume;
    ret = rzg2l_usbphy_ctrl_set_pwrrdy(priv.pwrrdy, false);
    if (ret)
    goto reset_deassert;
    return 0;
    reset_deassert:
    reset_control_deassert(priv.rstc);
    rpm_resume:
    pm_runtime_resume_and_get(dev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn rzg2l_usbphy_ctrl_resume(dev: *mut device) -> c_int {
    static int rzg2l_usbphy_ctrl_resume(struct device *dev)
    {
    struct rzg2l_usbphy_ctrl_priv *priv = dev_get_drvdata(dev);
    int ret;
    ret = rzg2l_usbphy_ctrl_set_pwrrdy(priv.pwrrdy, true);
    if (ret)
    return ret;
    ret = reset_control_deassert(priv.rstc);
    if (ret)
    goto pwrrdy_off;
    ret = pm_runtime_resume_and_get(dev);
    if (ret)
    goto reset_assert;
    rzg2l_usbphy_ctrl_init(priv);
    return 0;
    reset_assert:
    reset_control_assert(priv.rstc);
    pwrrdy_off:
    rzg2l_usbphy_ctrl_set_pwrrdy(priv.pwrrdy, false);
    return ret;
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(rzg2l_usbphy_ctrl_pm_ops,
    rzg2l_usbphy_ctrl_suspend,
    rzg2l_usbphy_ctrl_resume);
    static struct platform_driver rzg2l_usbphy_ctrl_driver = {
    .driver = {
    .name		= "rzg2l_usbphy_ctrl",
    .of_match_table	= rzg2l_usbphy_ctrl_match_table,
    .pm		= pm_ptr(&rzg2l_usbphy_ctrl_pm_ops),
    },
    .probe	= rzg2l_usbphy_ctrl_probe,
    .remove = rzg2l_usbphy_ctrl_remove,
    };
    module_platform_driver(rzg2l_usbphy_ctrl_driver);
    MODULE_LICENSE("GPL v2");
    MODULE_DESCRIPTION("Renesas RZ/G2L USBPHY Control");
    MODULE_AUTHOR("Biju Das <biju.das.jz@bp.renesas.com>");
