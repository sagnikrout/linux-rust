//! Automatically rewritten from C to Rust
//! Source: drivers/misc/tps6594-pfsm.c
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
// PFSM (Pre-configurable Finite State Machine) driver for the following
// PMICs:
// - LP8764
// - TPS65224
// - TPS652G1
// - TPS6594
// - TPS6593
//
// Copyright (C) 2023 BayLibre Incorporated - https://www.baylibre.com
//

pub const TPS6594_STARTUP_DEST_MCU_ONLY_VAL: c_int = 2;
pub const TPS6594_STARTUP_DEST_ACTIVE_VAL: c_int = 3;
pub const TPS6594_STARTUP_DEST_SHIFT: c_int = 5;

    << TPS6594_STARTUP_DEST_SHIFT)

    << TPS6594_STARTUP_DEST_SHIFT)
//
// To update the PMIC firmware, the user must be able to access
// page 0 (user registers) and page 1 (NVM control and configuration).
//
pub const TPS6594_PMIC_MAX_POS: c_uint = 0x200;

//
// struct tps6594_pfsm - device private data structure
//
// @miscdev: misc device infos
// @regmap:  regmap for accessing the device registers
// @chip_id: chip identifier of the device
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tps6594_pfsm {
    pub miscdev: miscdevice,
    pub regmap: *mut regmap,
    pub chip_id: c_ulong,
}

    static ssize_t tps6594_pfsm_read(struct file *f, char __user *buf,
    size_t count, loff_t *ppos)
    {
    struct tps6594_pfsm *pfsm = TPS6594_FILE_TO_PFSM(f);
    let mut pos: loff_t = *ppos;
    unsigned int val;
    int ret;
    int i;
    if (pos < 0)
    return -EINVAL;
    if (pos >= TPS6594_PMIC_MAX_POS)
    return 0;
    if (count > TPS6594_PMIC_MAX_POS - pos)
    count = TPS6594_PMIC_MAX_POS - pos;
    for (i = 0 ; i < count ; i++) {
    ret = regmap_read(pfsm.regmap, pos + i, &val);
    if (ret)
    return ret;
    if (put_user(val, buf + i))
    return -EFAULT;
    }
// ppos = pos + count;
    return count;
    }
    static ssize_t tps6594_pfsm_write(struct file *f, const char __user *buf,
    size_t count, loff_t *ppos)
    {
    struct tps6594_pfsm *pfsm = TPS6594_FILE_TO_PFSM(f);
    let mut pos: loff_t = *ppos;
    char val;
    int ret;
    int i;
    if (pos < 0)
    return -EINVAL;
    if (pos >= TPS6594_PMIC_MAX_POS || !count)
    return 0;
    if (count > TPS6594_PMIC_MAX_POS - pos)
    count = TPS6594_PMIC_MAX_POS - pos;
    for (i = 0 ; i < count ; i++) {
    if (get_user(val, buf + i))
    return -EFAULT;
    ret = regmap_write(pfsm.regmap, pos + i, val);
    if (ret)
    return ret;
    }
// ppos = pos + count;
    return count;
    }
#[no_mangle]
unsafe extern "C" fn tps6594_pfsm_configure_ret_trig(regmap: *mut regmap, gpio_ret: u8, ddr_ret: u8) -> c_int {
    static int tps6594_pfsm_configure_ret_trig(struct regmap *regmap, u8 gpio_ret, u8 ddr_ret)
    {
    int ret;
    if (gpio_ret)
    ret = regmap_set_bits(regmap, TPS6594_REG_FSM_I2C_TRIGGERS,
    TPS6594_BIT_TRIGGER_I2C(5) | TPS6594_BIT_TRIGGER_I2C(6));
    else
    ret = regmap_clear_bits(regmap, TPS6594_REG_FSM_I2C_TRIGGERS,
    TPS6594_BIT_TRIGGER_I2C(5) | TPS6594_BIT_TRIGGER_I2C(6));
    if (ret)
    return ret;
    if (ddr_ret)
    ret = regmap_set_bits(regmap, TPS6594_REG_FSM_I2C_TRIGGERS,
    TPS6594_BIT_TRIGGER_I2C(7));
    else
    ret = regmap_clear_bits(regmap, TPS6594_REG_FSM_I2C_TRIGGERS,
    TPS6594_BIT_TRIGGER_I2C(7));
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn tps6594_pfsm_ioctl(f: *mut file, cmd: c_uint, arg: c_ulong) -> c_long {
    static long tps6594_pfsm_ioctl(struct file *f, unsigned int cmd, unsigned long arg)
    {
    struct tps6594_pfsm *pfsm = TPS6594_FILE_TO_PFSM(f);
    struct pmic_state_opt state_opt;
    void __user *argp = (void __user *)arg;
    unsigned int regmap_reg, mask;
    let mut ret: c_int = -ENOIOCTLCMD;
    switch (cmd) {
    case PMIC_GOTO_STANDBY:
// Disable LP mode on TPS6594 Family PMIC
    if (pfsm.chip_id != TPS65224 && pfsm.chip_id != TPS652G1) {
    ret = regmap_clear_bits(pfsm.regmap, TPS6594_REG_RTC_CTRL_2,
    TPS6594_BIT_LP_STANDBY_SEL);
    if (ret)
    return ret;
    }
// Force trigger
    ret = regmap_write_bits(pfsm.regmap, TPS6594_REG_FSM_I2C_TRIGGERS,
    TPS6594_BIT_TRIGGER_I2C(0), TPS6594_BIT_TRIGGER_I2C(0));
    break;
    case PMIC_GOTO_LP_STANDBY:
// TPS65224/TPS652G1 does not support LP STANDBY
    if (pfsm.chip_id == TPS65224 || pfsm.chip_id == TPS652G1)
    return ret;
// Enable LP mode
    ret = regmap_set_bits(pfsm.regmap, TPS6594_REG_RTC_CTRL_2,
    TPS6594_BIT_LP_STANDBY_SEL);
    if (ret)
    return ret;
// Force trigger
    ret = regmap_write_bits(pfsm.regmap, TPS6594_REG_FSM_I2C_TRIGGERS,
    TPS6594_BIT_TRIGGER_I2C(0), TPS6594_BIT_TRIGGER_I2C(0));
    break;
    case PMIC_UPDATE_PGM:
// Force trigger
    ret = regmap_write_bits(pfsm.regmap, TPS6594_REG_FSM_I2C_TRIGGERS,
    TPS6594_BIT_TRIGGER_I2C(3), TPS6594_BIT_TRIGGER_I2C(3));
    break;
    case PMIC_SET_ACTIVE_STATE:
// Modify NSLEEP1-2 bits
    ret = regmap_set_bits(pfsm.regmap, TPS6594_REG_FSM_NSLEEP_TRIGGERS,
    TPS6594_BIT_NSLEEP1B | TPS6594_BIT_NSLEEP2B);
    break;
    case PMIC_SET_MCU_ONLY_STATE:
// TPS65224/TPS652G1 does not support MCU_ONLY_STATE
    if (pfsm.chip_id == TPS65224 || pfsm.chip_id == TPS652G1)
    return ret;
    if (copy_from_user(&state_opt, argp, sizeof(state_opt)))
    return -EFAULT;
// Configure retention triggers
    ret = tps6594_pfsm_configure_ret_trig(pfsm.regmap, state_opt.gpio_retention,
    state_opt.ddr_retention);
    if (ret)
    return ret;
// Modify NSLEEP1-2 bits
    ret = regmap_clear_bits(pfsm.regmap, TPS6594_REG_FSM_NSLEEP_TRIGGERS,
    TPS6594_BIT_NSLEEP1B);
    if (ret)
    return ret;
    ret = regmap_set_bits(pfsm.regmap, TPS6594_REG_FSM_NSLEEP_TRIGGERS,
    TPS6594_BIT_NSLEEP2B);
    break;
    case PMIC_SET_RETENTION_STATE:
    if (copy_from_user(&state_opt, argp, sizeof(state_opt)))
    return -EFAULT;
// Configure wake-up destination
    if (pfsm.chip_id == TPS65224 || pfsm.chip_id == TPS652G1) {
    regmap_reg = TPS65224_REG_STARTUP_CTRL;
    mask = TPS65224_MASK_STARTUP_DEST;
    } else {
    regmap_reg = TPS6594_REG_RTC_CTRL_2;
    mask = TPS6594_MASK_STARTUP_DEST;
    }
    if (state_opt.mcu_only_startup_dest)
    ret = regmap_write_bits(pfsm.regmap, regmap_reg,
    mask, TPS6594_STARTUP_DEST_MCU_ONLY);
    else
    ret = regmap_write_bits(pfsm.regmap, regmap_reg,
    mask, TPS6594_STARTUP_DEST_ACTIVE);
    if (ret)
    return ret;
// Configure retention triggers
    ret = tps6594_pfsm_configure_ret_trig(pfsm.regmap, state_opt.gpio_retention,
    state_opt.ddr_retention);
    if (ret)
    return ret;
// Modify NSLEEP1-2 bits
    if (pfsm.chip_id == TPS65224 || pfsm.chip_id == TPS652G1)
    ret = regmap_clear_bits(pfsm.regmap,
    TPS6594_REG_FSM_NSLEEP_TRIGGERS,
    TPS6594_BIT_NSLEEP1B);
    else
    ret = regmap_clear_bits(pfsm.regmap,
    TPS6594_REG_FSM_NSLEEP_TRIGGERS,
    TPS6594_BIT_NSLEEP2B);
    break;
    }
    return ret;
    }
    static const struct file_operations tps6594_pfsm_fops = {
    .owner		= THIS_MODULE,
    .llseek		= generic_file_llseek,
    .read		= tps6594_pfsm_read,
    .write		= tps6594_pfsm_write,
    .unlocked_ioctl	= tps6594_pfsm_ioctl,
    .compat_ioctl   = compat_ptr_ioctl,
    };
#[no_mangle]
unsafe extern "C" fn tps6594_pfsm_isr(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t tps6594_pfsm_isr(int irq, void *dev_id)
    {
    struct platform_device *pdev = dev_id;
    int i;
    for (i = 0 ; i < pdev.num_resources ; i++) {
    if (irq == platform_get_irq_byname(pdev, pdev.resource[i].name)) {
    dev_err(pdev.dev.parent, "%s event detected\n", pdev.resource[i].name);
    return IRQ_HANDLED;
    }
    }
    return IRQ_NONE;
    }
#[no_mangle]
unsafe extern "C" fn tps6594_pfsm_probe(pdev: *mut platform_device) -> c_int {
    static int tps6594_pfsm_probe(struct platform_device *pdev)
    {
    struct tps6594_pfsm *pfsm;
    struct tps6594 *tps = dev_get_drvdata(pdev.dev.parent);
    struct device *dev = &pdev.dev;
    int irq;
    int ret;
    int i;
    pfsm = devm_kzalloc(dev, sizeof(struct tps6594_pfsm), GFP_KERNEL);
    if (!pfsm)
    return -ENOMEM;
    pfsm.regmap = tps.regmap;
    pfsm.miscdev.minor = MISC_DYNAMIC_MINOR;
    pfsm.miscdev.name = devm_kasprintf(dev, GFP_KERNEL, "pfsm-%ld-0x%02x",
    tps.chip_id, tps.reg);
    if (!pfsm.miscdev.name)
    return -ENOMEM;
    pfsm.miscdev.fops = &tps6594_pfsm_fops;
    pfsm.miscdev.parent = dev.parent;
    pfsm.chip_id = tps.chip_id;
    for (i = 0 ; i < pdev.num_resources ; i++) {
    irq = platform_get_irq_byname(pdev, pdev.resource[i].name);
    if (irq < 0)
    return irq;
    ret = devm_request_threaded_irq(dev, irq, core::ptr::null_mut(),
    tps6594_pfsm_isr, IRQF_ONESHOT,
    pdev.resource[i].name, pdev);
    if (ret)
    return ret;
    }
    platform_set_drvdata(pdev, pfsm);
    return misc_register(&pfsm.miscdev);
    }
#[no_mangle]
unsafe extern "C" fn tps6594_pfsm_remove(pdev: *mut platform_device) {
    static void tps6594_pfsm_remove(struct platform_device *pdev)
    {
    struct tps6594_pfsm *pfsm = platform_get_drvdata(pdev);
    misc_deregister(&pfsm.miscdev);
    }
    static struct platform_driver tps6594_pfsm_driver = {
    .driver	= {
    .name = "tps6594-pfsm",
    },
    .probe = tps6594_pfsm_probe,
    .remove = tps6594_pfsm_remove,
    };
    module_platform_driver(tps6594_pfsm_driver);
    MODULE_ALIAS("platform:tps6594-pfsm");
    MODULE_AUTHOR("Julien Panis <jpanis@baylibre.com>");
    MODULE_DESCRIPTION("TPS6594 Pre-configurable Finite State Machine Driver");
    MODULE_LICENSE("GPL");
