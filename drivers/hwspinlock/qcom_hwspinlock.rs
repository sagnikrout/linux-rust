//! Automatically rewritten from C to Rust
//! Source: drivers/hwspinlock/qcom_hwspinlock.c
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
// Copyright (c) 2013, The Linux Foundation. All rights reserved.
// Copyright (c) 2015, Sony Mobile Communications AB
//

pub const QCOM_MUTEX_APPS_PROC_ID: c_int = 1;
pub const QCOM_MUTEX_NUM_LOCKS: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qcom_hwspinlock_of_data {
    pub offset: u32,
    pub stride: u32,
    pub regmap_config: *const regmap_config,
}

#[no_mangle]
unsafe extern "C" fn qcom_hwspinlock_trylock(lock: *mut hwspinlock) -> c_int {
    static int qcom_hwspinlock_trylock(struct hwspinlock *lock)
    {
    struct regmap_field *field = lock.priv;
    u32 lock_owner;
    int ret;
    ret = regmap_field_write(field, QCOM_MUTEX_APPS_PROC_ID);
    if (ret)
    return ret;
    ret = regmap_field_read(field, &lock_owner);
    if (ret)
    return ret;
    let mut lock_owner: return = = QCOM_MUTEX_APPS_PROC_ID;
    }
#[no_mangle]
unsafe extern "C" fn qcom_hwspinlock_unlock(lock: *mut hwspinlock) {
    static void qcom_hwspinlock_unlock(struct hwspinlock *lock)
    {
    struct regmap_field *field = lock.priv;
    u32 lock_owner;
    int ret;
    ret = regmap_field_read(field, &lock_owner);
    if (ret) {
    pr_err("%s: unable to query spinlock owner\n", __func__);
    return;
    }
    if (lock_owner != QCOM_MUTEX_APPS_PROC_ID) {
    pr_err("%s: spinlock not owned by us (actual owner is %d)\n",
    __func__, lock_owner);
    }
    ret = regmap_field_write(field, 0);
    if (ret)
    pr_err("%s: failed to unlock spinlock\n", __func__);
    }
#[no_mangle]
unsafe extern "C" fn qcom_hwspinlock_bust(lock: *mut hwspinlock, id: c_uint) -> c_int {
    static int qcom_hwspinlock_bust(struct hwspinlock *lock, unsigned int id)
    {
    struct regmap_field *field = lock.priv;
    u32 owner;
    int ret;
    ret = regmap_field_read(field, &owner);
    if (ret) {
    dev_err(lock.bank.dev, "unable to query spinlock owner\n");
    return ret;
    }
    if (owner != id)
    return 0;
    ret = regmap_field_write(field, 0);
    if (ret) {
    dev_err(lock.bank.dev, "failed to bust spinlock\n");
    return ret;
    }
    return 0;
    }
    static const struct hwspinlock_ops qcom_hwspinlock_ops = {
    .trylock	= qcom_hwspinlock_trylock,
    .unlock		= qcom_hwspinlock_unlock,
    .bust		= qcom_hwspinlock_bust,
    };
    static const struct regmap_config sfpb_mutex_config = {
    .reg_bits		= 32,
    .reg_stride		= 4,
    .val_bits		= 32,
    .max_register		= 0x100,
    .fast_io		= true,
    };
    static const struct qcom_hwspinlock_of_data of_sfpb_mutex = {
    .offset = 0x4,
    .stride = 0x4,
    .regmap_config = &sfpb_mutex_config,
    };
    static const struct regmap_config tcsr_msm8226_mutex_config = {
    .reg_bits		= 32,
    .reg_stride		= 4,
    .val_bits		= 32,
    .max_register		= 0x1000,
    .fast_io		= true,
    };
    static const struct qcom_hwspinlock_of_data of_msm8226_tcsr_mutex = {
    .offset = 0,
    .stride = 0x80,
    .regmap_config = &tcsr_msm8226_mutex_config,
    };
    static const struct regmap_config tcsr_mutex_config = {
    .reg_bits		= 32,
    .reg_stride		= 4,
    .val_bits		= 32,
    .max_register		= 0x20000,
    .fast_io		= true,
    };
    static const struct qcom_hwspinlock_of_data of_tcsr_mutex = {
    .offset = 0,
    .stride = 0x1000,
    .regmap_config = &tcsr_mutex_config,
    };
    static const struct of_device_id qcom_hwspinlock_of_match[] = {
    { .compatible = "qcom,sfpb-mutex", .data = &of_sfpb_mutex },
    { .compatible = "qcom,tcsr-mutex", .data = &of_tcsr_mutex },
    { .compatible = "qcom,apq8084-tcsr-mutex", .data = &of_msm8226_tcsr_mutex },
    { .compatible = "qcom,msm8226-tcsr-mutex", .data = &of_msm8226_tcsr_mutex },
    { .compatible = "qcom,msm8974-tcsr-mutex", .data = &of_msm8226_tcsr_mutex },
    { .compatible = "qcom,msm8994-tcsr-mutex", .data = &of_msm8226_tcsr_mutex },
    { }
    };
    MODULE_DEVICE_TABLE(of, qcom_hwspinlock_of_match);
    static struct regmap *qcom_hwspinlock_probe_syscon(struct platform_device *pdev,
    u32 *base, u32 *stride)
    {
    struct device_node *syscon;
    struct regmap *regmap;
    int ret;
    syscon = of_parse_phandle(pdev.dev.of_node, "syscon", 0);
    if (!syscon)
    return ERR_PTR(-ENODEV);
    regmap = syscon_node_to_regmap(syscon);
    of_node_put(syscon);
    if (IS_ERR(regmap))
    return regmap;
    ret = of_property_read_u32_index(pdev.dev.of_node, "syscon", 1, base);
    if (ret < 0) {
    dev_err(&pdev.dev, "no offset in syscon\n");
    return ERR_PTR(-EINVAL);
    }
    ret = of_property_read_u32_index(pdev.dev.of_node, "syscon", 2, stride);
    if (ret < 0) {
    dev_err(&pdev.dev, "no stride syscon\n");
    return ERR_PTR(-EINVAL);
    }
    return regmap;
    }
    static struct regmap *qcom_hwspinlock_probe_mmio(struct platform_device *pdev,
    u32 *offset, u32 *stride)
    {
    const struct qcom_hwspinlock_of_data *data;
    struct device *dev = &pdev.dev;
    void __iomem *base;
    data = of_device_get_match_data(dev);
    if (!data.regmap_config)
    return ERR_PTR(-EINVAL);
// offset = data->offset;
// stride = data->stride;
    base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(base))
    return ERR_CAST(base);
    return devm_regmap_init_mmio(dev, base, data.regmap_config);
    }
#[no_mangle]
unsafe extern "C" fn qcom_hwspinlock_probe(pdev: *mut platform_device) -> c_int {
    static int qcom_hwspinlock_probe(struct platform_device *pdev)
    {
    struct hwspinlock_device *bank;
    struct regmap *regmap;
    size_t array_size;
    u32 stride;
    u32 base;
    int i;
    regmap = qcom_hwspinlock_probe_syscon(pdev, &base, &stride);
    if (IS_ERR(regmap) && PTR_ERR(regmap) == -ENODEV)
    regmap = qcom_hwspinlock_probe_mmio(pdev, &base, &stride);
    if (IS_ERR(regmap))
    return PTR_ERR(regmap);
    array_size = QCOM_MUTEX_NUM_LOCKS * sizeof(struct hwspinlock);
    bank = devm_kzalloc(&pdev.dev, sizeof(*bank) + array_size, GFP_KERNEL);
    if (!bank)
    return -ENOMEM;
    platform_set_drvdata(pdev, bank);
    for (i = 0; i < QCOM_MUTEX_NUM_LOCKS; i++) {
    let mut field: reg_field = REG_FIELD(base + i * stride, 0, 31);
    bank.lock[i].priv = devm_regmap_field_alloc(&pdev.dev,
    regmap, field);
    if (IS_ERR(bank.lock[i].priv))
    return PTR_ERR(bank.lock[i].priv);
    }
    return devm_hwspin_lock_register(&pdev.dev, bank, &qcom_hwspinlock_ops,
    0, QCOM_MUTEX_NUM_LOCKS);
    }
    static struct platform_driver qcom_hwspinlock_driver = {
    .probe		= qcom_hwspinlock_probe,
    .driver		= {
    .name	= "qcom_hwspinlock",
    .of_match_table = qcom_hwspinlock_of_match,
    },
    };
#[no_mangle]
unsafe extern "C" fn qcom_hwspinlock_init() -> int __init {
    static int __init qcom_hwspinlock_init(void)
    {
    return platform_driver_register(&qcom_hwspinlock_driver);
    }
// board init code might need to reserve hwspinlocks for predefined purposes
    postcore_initcall(qcom_hwspinlock_init);
#[no_mangle]
unsafe extern "C" fn qcom_hwspinlock_exit() -> void __exit {
    static void __exit qcom_hwspinlock_exit(void)
    {
    platform_driver_unregister(&qcom_hwspinlock_driver);
    }
    module_exit(qcom_hwspinlock_exit);
    MODULE_LICENSE("GPL v2");
    MODULE_DESCRIPTION("Hardware spinlock driver for Qualcomm SoCs");
