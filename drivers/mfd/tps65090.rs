//! Automatically rewritten from C to Rust
//! Source: drivers/mfd/tps65090.c
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
// Core driver for TI TPS65090 PMIC family
//
// Copyright (c) 2012, NVIDIA CORPORATION.  All rights reserved.
//
// Author: Venu Byravarasu <vbyravarasu@nvidia.com>
//

pub const NUM_INT_REG: c_int = 2;
pub const TPS65090_INT1_MASK_VAC_STATUS_CHANGE: c_int = 1;
pub const TPS65090_INT1_MASK_VSYS_STATUS_CHANGE: c_int = 2;
pub const TPS65090_INT1_MASK_BAT_STATUS_CHANGE: c_int = 3;
pub const TPS65090_INT1_MASK_CHARGING_STATUS_CHANGE: c_int = 4;
pub const TPS65090_INT1_MASK_CHARGING_COMPLETE: c_int = 5;
pub const TPS65090_INT1_MASK_OVERLOAD_DCDC1: c_int = 6;
pub const TPS65090_INT1_MASK_OVERLOAD_DCDC2: c_int = 7;
pub const TPS65090_INT2_MASK_OVERLOAD_DCDC3: c_int = 0;
pub const TPS65090_INT2_MASK_OVERLOAD_FET1: c_int = 1;
pub const TPS65090_INT2_MASK_OVERLOAD_FET2: c_int = 2;
pub const TPS65090_INT2_MASK_OVERLOAD_FET3: c_int = 3;
pub const TPS65090_INT2_MASK_OVERLOAD_FET4: c_int = 4;
pub const TPS65090_INT2_MASK_OVERLOAD_FET5: c_int = 5;
pub const TPS65090_INT2_MASK_OVERLOAD_FET6: c_int = 6;
pub const TPS65090_INT2_MASK_OVERLOAD_FET7: c_int = 7;
    static const struct resource charger_resources[] = {
    {
    .start  = TPS65090_IRQ_VAC_STATUS_CHANGE,
    .end    = TPS65090_IRQ_VAC_STATUS_CHANGE,
    .flags  = IORESOURCE_IRQ,
    }
    };
    enum tps65090_cells {
    PMIC = 0,
    CHARGER = 1,
    };
    static struct mfd_cell tps65090s[] = {
    [PMIC] = {
    .name = "tps65090-pmic",
    },
    [CHARGER] = {
    .name = "tps65090-charger",
    .num_resources = ARRAY_SIZE(charger_resources),
    .resources = &charger_resources[0],
    .of_compatible = "ti,tps65090-charger",
    },
    };
    static const struct regmap_irq tps65090_irqs[] = {
// INT1 IRQs
    [TPS65090_IRQ_VAC_STATUS_CHANGE] = {
    .mask = TPS65090_INT1_MASK_VAC_STATUS_CHANGE,
    },
    [TPS65090_IRQ_VSYS_STATUS_CHANGE] = {
    .mask = TPS65090_INT1_MASK_VSYS_STATUS_CHANGE,
    },
    [TPS65090_IRQ_BAT_STATUS_CHANGE] = {
    .mask = TPS65090_INT1_MASK_BAT_STATUS_CHANGE,
    },
    [TPS65090_IRQ_CHARGING_STATUS_CHANGE] = {
    .mask = TPS65090_INT1_MASK_CHARGING_STATUS_CHANGE,
    },
    [TPS65090_IRQ_CHARGING_COMPLETE] = {
    .mask = TPS65090_INT1_MASK_CHARGING_COMPLETE,
    },
    [TPS65090_IRQ_OVERLOAD_DCDC1] = {
    .mask = TPS65090_INT1_MASK_OVERLOAD_DCDC1,
    },
    [TPS65090_IRQ_OVERLOAD_DCDC2] = {
    .mask = TPS65090_INT1_MASK_OVERLOAD_DCDC2,
    },
// INT2 IRQs
    [TPS65090_IRQ_OVERLOAD_DCDC3] = {
    .reg_offset = 1,
    .mask = TPS65090_INT2_MASK_OVERLOAD_DCDC3,
    },
    [TPS65090_IRQ_OVERLOAD_FET1] = {
    .reg_offset = 1,
    .mask = TPS65090_INT2_MASK_OVERLOAD_FET1,
    },
    [TPS65090_IRQ_OVERLOAD_FET2] = {
    .reg_offset = 1,
    .mask = TPS65090_INT2_MASK_OVERLOAD_FET2,
    },
    [TPS65090_IRQ_OVERLOAD_FET3] = {
    .reg_offset = 1,
    .mask = TPS65090_INT2_MASK_OVERLOAD_FET3,
    },
    [TPS65090_IRQ_OVERLOAD_FET4] = {
    .reg_offset = 1,
    .mask = TPS65090_INT2_MASK_OVERLOAD_FET4,
    },
    [TPS65090_IRQ_OVERLOAD_FET5] = {
    .reg_offset = 1,
    .mask = TPS65090_INT2_MASK_OVERLOAD_FET5,
    },
    [TPS65090_IRQ_OVERLOAD_FET6] = {
    .reg_offset = 1,
    .mask = TPS65090_INT2_MASK_OVERLOAD_FET6,
    },
    [TPS65090_IRQ_OVERLOAD_FET7] = {
    .reg_offset = 1,
    .mask = TPS65090_INT2_MASK_OVERLOAD_FET7,
    },
    };
    static const struct regmap_irq_chip tps65090_irq_chip = {
    .name = "tps65090",
    .irqs = tps65090_irqs,
    .num_irqs = ARRAY_SIZE(tps65090_irqs),
    .num_regs = NUM_INT_REG,
    .status_base = TPS65090_REG_INTR_STS,
    .unmask_base = TPS65090_REG_INTR_MASK,
    };
#[no_mangle]
unsafe extern "C" fn is_volatile_reg(dev: *mut device, reg: c_uint) -> bool {
    static bool is_volatile_reg(struct device *dev, unsigned int reg)
    {
// Nearly all registers have status bits mixed in, except a few
    switch (reg) {
    case TPS65090_REG_INTR_MASK:
    case TPS65090_REG_INTR_MASK2:
    case TPS65090_REG_CG_CTRL0:
    case TPS65090_REG_CG_CTRL1:
    case TPS65090_REG_CG_CTRL2:
    case TPS65090_REG_CG_CTRL3:
    case TPS65090_REG_CG_CTRL4:
    case TPS65090_REG_CG_CTRL5:
    return false;
    }
    return true;
    }
    static const struct regmap_config tps65090_regmap_config = {
    .reg_bits = 8,
    .val_bits = 8,
    .max_register = TPS65090_MAX_REG,
    .num_reg_defaults_raw = TPS65090_NUM_REGS,
    .cache_type = REGCACHE_MAPLE,
    .volatile_reg = is_volatile_reg,
    };

    static const struct of_device_id tps65090_of_match[] = {
    { .compatible = "ti,tps65090",},
    {},
    };

#[no_mangle]
unsafe extern "C" fn tps65090_i2c_probe(client: *mut i2c_client) -> c_int {
    static int tps65090_i2c_probe(struct i2c_client *client)
    {
    struct tps65090_platform_data *pdata = dev_get_platdata(&client.dev);
    let mut irq_base: c_int = 0;
    struct tps65090 *tps65090;
    int ret;
    if (!pdata && !client.dev.of_node) {
    dev_err(&client.dev,
    "tps65090 requires platform data or of_node\n");
    return -EINVAL;
    }
    if (pdata)
    irq_base = pdata.irq_base;
    tps65090 = devm_kzalloc(&client.dev, sizeof(*tps65090), GFP_KERNEL);
    if (!tps65090)
    return -ENOMEM;
    tps65090.dev = &client.dev;
    i2c_set_clientdata(client, tps65090);
    tps65090.rmap = devm_regmap_init_i2c(client, &tps65090_regmap_config);
    if (IS_ERR(tps65090.rmap)) {
    ret = PTR_ERR(tps65090.rmap);
    dev_err(&client.dev, "regmap_init failed with err: %d\n", ret);
    return ret;
    }
    if (client.irq) {
    ret = regmap_add_irq_chip(tps65090.rmap, client.irq,
    IRQF_ONESHOT | IRQF_TRIGGER_LOW, irq_base,
    &tps65090_irq_chip, &tps65090.irq_data);
    if (ret) {
    dev_err(&client.dev,
    "IRQ init failed with err: %d\n", ret);
    return ret;
    }
    } else {
// Don't tell children they have an IRQ that'll never fire
    tps65090s[CHARGER].num_resources = 0;
    }
    ret = mfd_add_devices(tps65090.dev, -1, tps65090s,
    ARRAY_SIZE(tps65090s), core::ptr::null_mut(),
    0, regmap_irq_get_domain(tps65090.irq_data));
    if (ret) {
    dev_err(&client.dev, "add mfd devices failed with err: %d\n",
    ret);
    goto err_irq_exit;
    }
    return 0;
    err_irq_exit:
    if (client.irq)
    regmap_del_irq_chip(client.irq, tps65090.irq_data);
    return ret;
    }
    static const struct i2c_device_id tps65090_id_table[] = {
    { "tps65090" },
    { }
    };
    static struct i2c_driver tps65090_driver = {
    .driver	= {
    .name	= "tps65090",
    .suppress_bind_attrs = true,
    .of_match_table = of_match_ptr(tps65090_of_match),
    },
    .probe		= tps65090_i2c_probe,
    .id_table	= tps65090_id_table,
    };
#[no_mangle]
unsafe extern "C" fn tps65090_init() -> int __init {
    static int __init tps65090_init(void)
    {
    return i2c_add_driver(&tps65090_driver);
    }
    subsys_initcall(tps65090_init);
