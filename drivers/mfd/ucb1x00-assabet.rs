//! Automatically rewritten from C to Rust
//! Source: drivers/mfd/ucb1x00-assabet.c
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
// linux/drivers/mfd/ucb1x00-assabet.c
//
// Copyright (C) 2001-2003 Russell King, All Rights Reserved.
//
// We handle the machine-specific bits of the UCB1x00 driver here.
//

// Macro flag: #define UCB1X00_ATTR(name,input)\
    static ssize_t name##_show(struct device *dev, struct device_attribute *attr, \
    char *buf)	\
    {								\
    struct ucb1x00 *ucb = classdev_to_ucb1x00(dev);		\
    int val;						\
    ucb1x00_adc_enable(ucb);				\
    val = ucb1x00_adc_read(ucb, input, UCB_NOSYNC);		\
    ucb1x00_adc_disable(ucb);				\
    return sprintf(buf, "%d\n", val);			\
    }								\
#[no_mangle]
pub unsafe extern "C" fn DEVICE_ATTR_RO(_arg: name) -> static {
    static DEVICE_ATTR_RO(name)
    UCB1X00_ATTR(vbatt, UCB_ADC_INP_AD1);
    UCB1X00_ATTR(vcharger, UCB_ADC_INP_AD0);
    UCB1X00_ATTR(batt_temp, UCB_ADC_INP_AD2);
    static const struct property_entry ucb1x00_gpio_keys_props[] = {
    PROPERTY_ENTRY_STRING("label", "ucb1x00"),
    PROPERTY_ENTRY_U32("poll-interval", 50),
    { }
    };

    struct property_entry ucb1x00_btn##_idx##_props[] = {			\
    PROPERTY_ENTRY_U32("linux,code", BTN_0 + (_idx)),		\
    PROPERTY_ENTRY_GPIO("gpios", &ucb1x00_gpiochip_node,		\
    _idx, GPIO_ACTIVE_HIGH),			\
    PROPERTY_ENTRY_STRING("label", "btn" #_idx),			\
    PROPERTY_ENTRY_BOOL("linux,can-disable"),			\
    { }								\
    }
    static const UCB1X00_BTN_PROPS(0);
    static const UCB1X00_BTN_PROPS(1);
    static const UCB1X00_BTN_PROPS(2);
    static const UCB1X00_BTN_PROPS(3);
    static const UCB1X00_BTN_PROPS(4);
    static const UCB1X00_BTN_PROPS(5);
    static const struct property_entry * const ucb1x00_btn_props[] = {
    ucb1x00_btn0_props,
    ucb1x00_btn1_props,
    ucb1x00_btn2_props,
    ucb1x00_btn3_props,
    ucb1x00_btn4_props,
    ucb1x00_btn5_props,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ucb1x00_assabet_priv {
    pub pdev: *mut platform_device,
    pub keys_node: *mut fwnode_handle,
    pub button_nodes: [*mut fwnode_handle; ARRAY_SIZE(ucb1x00_btn_props)],
}

#[no_mangle]
unsafe extern "C" fn ucb1x00_assabet_remove_nodes(priv: *mut ucb1x00_assabet_priv, n: c_int) {
    static void ucb1x00_assabet_remove_nodes(struct ucb1x00_assabet_priv *priv, int n)
    {
    while (--n >= 0)
    fwnode_remove_software_node(priv.button_nodes[n]);
    fwnode_remove_software_node(priv.keys_node);
    }
#[no_mangle]
unsafe extern "C" fn ucb1x00_assabet_add(dev: *mut ucb1x00_dev) -> c_int {
    static int ucb1x00_assabet_add(struct ucb1x00_dev *dev)
    {
    struct ucb1x00 *ucb = dev.ucb;
    struct platform_device_info pdevinfo = {
    .name = "gpio-keys",
    .id = PLATFORM_DEVID_NONE,
    .parent = &ucb.dev,
    };
    int ret;
    int i;
    struct ucb1x00_assabet_priv *priv;
    priv = kzalloc_obj(*priv);
    if (!priv)
    return -ENOMEM;
    priv.keys_node = fwnode_create_software_node(ucb1x00_gpio_keys_props, core::ptr::null_mut());
    if (IS_ERR(priv.keys_node)) {
    ret = PTR_ERR(priv.keys_node);
    goto err_free_priv;
    }
    for (i = 0; i < ARRAY_SIZE(ucb1x00_btn_props); i++) {
    priv.button_nodes[i] = fwnode_create_software_node(ucb1x00_btn_props[i],
    priv.keys_node);
    if (IS_ERR(priv.button_nodes[i])) {
    ret = PTR_ERR(priv.button_nodes[i]);
    goto err_free_buttons;
    }
    }
    pdevinfo.fwnode = priv.keys_node;
    priv.pdev = platform_device_register_full(&pdevinfo);
    ret = PTR_ERR_OR_ZERO(priv.pdev);
    if (ret)
    goto err_free_buttons;
    device_create_file(&ucb.dev, &dev_attr_vbatt);
    device_create_file(&ucb.dev, &dev_attr_vcharger);
    device_create_file(&ucb.dev, &dev_attr_batt_temp);
    dev.priv = priv;
    return 0;
    err_free_buttons:
    ucb1x00_assabet_remove_nodes(priv, i);
    err_free_priv:
    kfree(priv);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ucb1x00_assabet_remove(dev: *mut ucb1x00_dev) {
    static void ucb1x00_assabet_remove(struct ucb1x00_dev *dev)
    {
    struct ucb1x00_assabet_priv *priv = dev.priv;
    if (!IS_ERR(priv.pdev))
    platform_device_unregister(priv.pdev);
    ucb1x00_assabet_remove_nodes(priv, ARRAY_SIZE(priv.button_nodes));
    device_remove_file(&dev.ucb.dev, &dev_attr_batt_temp);
    device_remove_file(&dev.ucb.dev, &dev_attr_vcharger);
    device_remove_file(&dev.ucb.dev, &dev_attr_vbatt);
    kfree(priv);
    }
    static struct ucb1x00_driver ucb1x00_assabet_driver = {
    .add	= ucb1x00_assabet_add,
    .remove	= ucb1x00_assabet_remove,
    };
#[no_mangle]
unsafe extern "C" fn ucb1x00_assabet_init() -> int __init {
    static int __init ucb1x00_assabet_init(void)
    {
    return ucb1x00_register_driver(&ucb1x00_assabet_driver);
    }
#[no_mangle]
unsafe extern "C" fn ucb1x00_assabet_exit() -> void __exit {
    static void __exit ucb1x00_assabet_exit(void)
    {
    ucb1x00_unregister_driver(&ucb1x00_assabet_driver);
    }
    module_init(ucb1x00_assabet_init);
    module_exit(ucb1x00_assabet_exit);
    MODULE_AUTHOR("Russell King <rmk@arm.linux.org.uk>");
    MODULE_DESCRIPTION("Assabet noddy testing only example ADC driver");
    MODULE_LICENSE("GPL");
