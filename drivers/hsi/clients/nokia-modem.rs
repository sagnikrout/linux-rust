//! Automatically rewritten from C to Rust
//! Source: drivers/hsi/clients/nokia-modem.c
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
// nokia-modem.c
//
// HSI client driver for Nokia N900 modem.
//
// Copyright (C) 2014 Sebastian Reichel <sre@kernel.org>
//

    let mut pm: static unsigned int = 1;
    module_param(pm, int, 0400);
    MODULE_PARM_DESC(pm,
    "Enable power management (0=disabled, 1=userland based [default])");
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nokia_modem_gpio {
    pub gpio: *mut gpio_desc,
    pub name: *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nokia_modem_device {
    pub nokia_modem_rst_ind_tasklet: tasklet_struct,
    pub nokia_modem_rst_ind_irq: c_int,
    pub device: *mut device,
    pub gpios: *mut nokia_modem_gpio,
    pub gpio_amount: c_int,
    pub ssi_protocol: *mut hsi_client,
    pub cmt_speech: *mut hsi_client,
}

#[no_mangle]
unsafe extern "C" fn do_nokia_modem_rst_ind_tasklet(data: c_ulong) {
    static void do_nokia_modem_rst_ind_tasklet(unsigned long data)
    {
    struct nokia_modem_device *modem = (struct nokia_modem_device *)data;
    if (!modem)
    return;
    dev_info(modem.device, "CMT rst line change detected\n");
    if (modem.ssi_protocol)
    ssip_reset_event(modem.ssi_protocol);
    }
#[no_mangle]
unsafe extern "C" fn nokia_modem_rst_ind_isr(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t nokia_modem_rst_ind_isr(int irq, void *data)
    {
    struct nokia_modem_device *modem = (struct nokia_modem_device *)data;
    tasklet_schedule(&modem.nokia_modem_rst_ind_tasklet);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn nokia_modem_gpio_unexport(dev: *mut device) {
    static void nokia_modem_gpio_unexport(struct device *dev)
    {
    struct nokia_modem_device *modem = dev_get_drvdata(dev);
    int i;
    for (i = 0; i < modem.gpio_amount; i++) {
    sysfs_remove_link(&dev.kobj, modem.gpios[i].name);
    gpiod_unexport(modem.gpios[i].gpio);
    }
    }
#[no_mangle]
unsafe extern "C" fn nokia_modem_gpio_probe(dev: *mut device) -> c_int {
    static int nokia_modem_gpio_probe(struct device *dev)
    {
    struct device_node *np = dev.of_node;
    struct nokia_modem_device *modem = dev_get_drvdata(dev);
    int gpio_count, gpio_name_count, i, err;
    gpio_count = gpiod_count(dev, core::ptr::null_mut());
    if (gpio_count < 0) {
    dev_err(dev, "missing gpios: %d\n", gpio_count);
    return gpio_count;
    }
    gpio_name_count = of_property_count_strings(np, "gpio-names");
    if (gpio_count != gpio_name_count) {
    dev_err(dev, "number of gpios does not equal number of gpio names\n");
    return -EINVAL;
    }
    modem.gpios = devm_kcalloc(dev, gpio_count, sizeof(*modem.gpios),
    GFP_KERNEL);
    if (!modem.gpios)
    return -ENOMEM;
    modem.gpio_amount = gpio_count;
    for (i = 0; i < gpio_count; i++) {
    modem.gpios[i].gpio = devm_gpiod_get_index(dev, core::ptr::null_mut(), i,
    GPIOD_OUT_LOW);
    if (IS_ERR(modem.gpios[i].gpio)) {
    dev_err(dev, "Could not get gpio %d\n", i);
    return PTR_ERR(modem.gpios[i].gpio);
    }
    err = of_property_read_string_index(np, "gpio-names", i,
    &(modem.gpios[i].name));
    if (err) {
    dev_err(dev, "Could not get gpio name %d\n", i);
    return err;
    }
    err = gpiod_export(modem.gpios[i].gpio, 0);
    if (err)
    return err;
    err = gpiod_export_link(dev, modem.gpios[i].name,
    modem.gpios[i].gpio);
    if (err)
    return err;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn nokia_modem_probe(dev: *mut device) -> c_int {
    static int nokia_modem_probe(struct device *dev)
    {
    struct device_node *np;
    struct nokia_modem_device *modem;
    struct hsi_client *cl = to_hsi_client(dev);
    struct hsi_port *port = hsi_get_port(cl);
    int irq, pflags, err;
    struct hsi_board_info ssip;
    struct hsi_board_info cmtspeech;
    np = dev.of_node;
    if (!np) {
    dev_err(dev, "device tree node not found\n");
    return -ENXIO;
    }
    modem = devm_kzalloc(dev, sizeof(*modem), GFP_KERNEL);
    if (!modem)
    return -ENOMEM;
    dev_set_drvdata(dev, modem);
    modem.device = dev;
    irq = irq_of_parse_and_map(np, 0);
    if (!irq) {
    dev_err(dev, "Invalid rst_ind interrupt (%d)\n", irq);
    return -EINVAL;
    }
    modem.nokia_modem_rst_ind_irq = irq;
    pflags = irq_get_trigger_type(irq);
    tasklet_init(&modem.nokia_modem_rst_ind_tasklet,
    do_nokia_modem_rst_ind_tasklet, (unsigned long)modem);
    err = devm_request_irq(dev, irq, nokia_modem_rst_ind_isr,
    pflags, "modem_rst_ind", modem);
    if (err < 0)
    return err;
    enable_irq_wake(irq);
    if (pm) {
    err = nokia_modem_gpio_probe(dev);
    if (err < 0) {
    dev_err(dev, "Could not probe GPIOs\n");
    goto error1;
    }
    }
    ssip.name = "ssi-protocol";
    ssip.tx_cfg = cl.tx_cfg;
    ssip.rx_cfg = cl.rx_cfg;
    ssip.platform_data = core::ptr::null_mut();
    ssip.archdata = core::ptr::null_mut();
    modem.ssi_protocol = hsi_new_client(port, &ssip);
    if (!modem.ssi_protocol) {
    dev_err(dev, "Could not register ssi-protocol device\n");
    err = -ENOMEM;
    goto error2;
    }
    err = device_attach(&modem.ssi_protocol.device);
    if (err == 0) {
    dev_dbg(dev, "Missing ssi-protocol driver\n");
    err = -EPROBE_DEFER;
    goto error3;
    } else if (err < 0) {
    dev_err(dev, "Could not load ssi-protocol driver (%d)\n", err);
    goto error3;
    }
    cmtspeech.name = "cmt-speech";
    cmtspeech.tx_cfg = cl.tx_cfg;
    cmtspeech.rx_cfg = cl.rx_cfg;
    cmtspeech.platform_data = core::ptr::null_mut();
    cmtspeech.archdata = core::ptr::null_mut();
    modem.cmt_speech = hsi_new_client(port, &cmtspeech);
    if (!modem.cmt_speech) {
    dev_err(dev, "Could not register cmt-speech device\n");
    err = -ENOMEM;
    goto error3;
    }
    err = device_attach(&modem.cmt_speech.device);
    if (err == 0) {
    dev_dbg(dev, "Missing cmt-speech driver\n");
    err = -EPROBE_DEFER;
    goto error4;
    } else if (err < 0) {
    dev_err(dev, "Could not load cmt-speech driver (%d)\n", err);
    goto error4;
    }
    dev_info(dev, "Registered Nokia HSI modem\n");
    return 0;
    error4:
    hsi_remove_client(&modem.cmt_speech.device, core::ptr::null_mut());
    error3:
    hsi_remove_client(&modem.ssi_protocol.device, core::ptr::null_mut());
    error2:
    nokia_modem_gpio_unexport(dev);
    error1:
    disable_irq_wake(modem.nokia_modem_rst_ind_irq);
    tasklet_kill(&modem.nokia_modem_rst_ind_tasklet);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn nokia_modem_remove(dev: *mut device) -> c_int {
    static int nokia_modem_remove(struct device *dev)
    {
    struct nokia_modem_device *modem = dev_get_drvdata(dev);
    if (!modem)
    return 0;
    if (modem.cmt_speech) {
    hsi_remove_client(&modem.cmt_speech.device, core::ptr::null_mut());
    modem.cmt_speech = core::ptr::null_mut();
    }
    if (modem.ssi_protocol) {
    hsi_remove_client(&modem.ssi_protocol.device, core::ptr::null_mut());
    modem.ssi_protocol = core::ptr::null_mut();
    }
    nokia_modem_gpio_unexport(dev);
    dev_set_drvdata(dev, core::ptr::null_mut());
    disable_irq_wake(modem.nokia_modem_rst_ind_irq);
    tasklet_kill(&modem.nokia_modem_rst_ind_tasklet);
    return 0;
    }

    static const struct of_device_id nokia_modem_of_match[] = {
    { .compatible = "nokia,n900-modem", },
    { .compatible = "nokia,n950-modem", },
    { .compatible = "nokia,n9-modem", },
    {},
    };
    MODULE_DEVICE_TABLE(of, nokia_modem_of_match);

    static struct hsi_client_driver nokia_modem_driver = {
    .driver = {
    .name	= "nokia-modem",
    .owner	= THIS_MODULE,
    .probe	= nokia_modem_probe,
    .remove	= nokia_modem_remove,
    .of_match_table = of_match_ptr(nokia_modem_of_match),
    },
    };
#[no_mangle]
unsafe extern "C" fn nokia_modem_init() -> int __init {
    static int __init nokia_modem_init(void)
    {
    return hsi_register_client_driver(&nokia_modem_driver);
    }
    module_init(nokia_modem_init);
#[no_mangle]
unsafe extern "C" fn nokia_modem_exit() -> void __exit {
    static void __exit nokia_modem_exit(void)
    {
    hsi_unregister_client_driver(&nokia_modem_driver);
    }
    module_exit(nokia_modem_exit);
    MODULE_ALIAS("hsi:nokia-modem");
    MODULE_AUTHOR("Sebastian Reichel <sre@kernel.org>");
    MODULE_DESCRIPTION("HSI driver module for Nokia N900 Modem");
    MODULE_LICENSE("GPL");
