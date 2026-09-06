//! Automatically rewritten from C to Rust
//! Source: drivers/net/dsa/mt7530-mmio.c
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

    static const struct of_device_id mt7988_of_match[] = {
    { .compatible = "airoha,an7583-switch", .data = &mt753x_table[ID_AN7583], },
    { .compatible = "airoha,en7581-switch", .data = &mt753x_table[ID_EN7581], },
    { .compatible = "econet,en7528-switch", .data = &mt753x_table[ID_EN7528], },
    { .compatible = "mediatek,mt7988-switch", .data = &mt753x_table[ID_MT7988], },
    { /* sentinel */ },
    };
    MODULE_DEVICE_TABLE(of, mt7988_of_match);
    static const struct regmap_config sw_regmap_config = {
    .name = "switch",
    .reg_bits = 16,
    .val_bits = 32,
    .reg_stride = 4,
    .max_register = MT7530_CREV,
    };
    static int
    mt7988_probe(struct platform_device *pdev)
    {
    struct mt7530_priv *priv;
    void __iomem *base_addr;
    int ret;
    priv = devm_kzalloc(&pdev.dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    priv.bus = core::ptr::null_mut();
    priv.dev = &pdev.dev;
    ret = mt7530_probe_common(priv);
    if (ret)
    return ret;
    priv.rstc = devm_reset_control_get(&pdev.dev, core::ptr::null_mut());
    if (IS_ERR(priv.rstc)) {
    dev_err(&pdev.dev, "Couldn't get our reset line\n");
    return PTR_ERR(priv.rstc);
    }
    base_addr = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(base_addr)) {
    dev_err(&pdev.dev, "cannot request I/O memory space\n");
    return -ENXIO;
    }
    priv.regmap = devm_regmap_init_mmio(&pdev.dev, base_addr,
    &sw_regmap_config);
    if (IS_ERR(priv.regmap))
    return PTR_ERR(priv.regmap);
    return dsa_register_switch(priv.ds);
    }
#[no_mangle]
unsafe extern "C" fn mt7988_remove(pdev: *mut platform_device) {
    static void mt7988_remove(struct platform_device *pdev)
    {
    struct mt7530_priv *priv = platform_get_drvdata(pdev);
    if (priv)
    mt7530_remove_common(priv);
    }
#[no_mangle]
unsafe extern "C" fn mt7988_shutdown(pdev: *mut platform_device) {
    static void mt7988_shutdown(struct platform_device *pdev)
    {
    struct mt7530_priv *priv = platform_get_drvdata(pdev);
    if (!priv)
    return;
    dsa_switch_shutdown(priv.ds);
    dev_set_drvdata(&pdev.dev, core::ptr::null_mut());
    }
    static struct platform_driver mt7988_platform_driver = {
    .probe  = mt7988_probe,
    .remove = mt7988_remove,
    .shutdown = mt7988_shutdown,
    .driver = {
    .name = "mt7530-mmio",
    .of_match_table = mt7988_of_match,
    },
    };
    module_platform_driver(mt7988_platform_driver);
    MODULE_AUTHOR("Daniel Golle <daniel@makrotopia.org>");
    MODULE_DESCRIPTION("Driver for Mediatek MT7530 Switch (MMIO)");
    MODULE_LICENSE("GPL");
