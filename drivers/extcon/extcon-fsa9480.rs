//! Automatically rewritten from C to Rust
//! Source: drivers/extcon/extcon-fsa9480.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// extcon-fsa9480.c - Fairchild Semiconductor FSA9480 extcon driver
//
// Copyright (c) 2019 Tomasz Figa <tomasz.figa@gmail.com>
//
// Loosely based on old fsa9480 misc-device driver.
//

// FSA9480 I2C registers
pub const FSA9480_REG_DEVID: c_uint = 0x01;
pub const FSA9480_REG_CTRL: c_uint = 0x02;
pub const FSA9480_REG_INT1: c_uint = 0x03;
pub const FSA9480_REG_INT2: c_uint = 0x04;
pub const FSA9480_REG_INT1_MASK: c_uint = 0x05;
pub const FSA9480_REG_INT2_MASK: c_uint = 0x06;
pub const FSA9480_REG_ADC: c_uint = 0x07;
pub const FSA9480_REG_TIMING1: c_uint = 0x08;
pub const FSA9480_REG_TIMING2: c_uint = 0x09;
pub const FSA9480_REG_DEV_T1: c_uint = 0x0a;
pub const FSA9480_REG_DEV_T2: c_uint = 0x0b;
pub const FSA9480_REG_BTN1: c_uint = 0x0c;
pub const FSA9480_REG_BTN2: c_uint = 0x0d;
pub const FSA9480_REG_CK: c_uint = 0x0e;
pub const FSA9480_REG_CK_INT1: c_uint = 0x0f;
pub const FSA9480_REG_CK_INT2: c_uint = 0x10;
pub const FSA9480_REG_CK_INTMASK1: c_uint = 0x11;
pub const FSA9480_REG_CK_INTMASK2: c_uint = 0x12;
pub const FSA9480_REG_MANSW1: c_uint = 0x13;
pub const FSA9480_REG_MANSW2: c_uint = 0x14;
pub const FSA9480_REG_END: c_uint = 0x15;
// Control

    CON_MANUAL_SW | CON_WAIT)
// Device Type 1
pub const DEV_USB_OTG: c_int = 7;
pub const DEV_DEDICATED_CHG: c_int = 6;
pub const DEV_USB_CHG: c_int = 5;
pub const DEV_CAR_KIT: c_int = 4;
pub const DEV_UART: c_int = 3;
pub const DEV_USB: c_int = 2;
pub const DEV_AUDIO_2: c_int = 1;
pub const DEV_AUDIO_1: c_int = 0;

// Device Type 2
pub const DEV_AV: c_int = 14;
pub const DEV_TTY: c_int = 13;
pub const DEV_PPD: c_int = 12;
pub const DEV_JIG_UART_OFF: c_int = 11;
pub const DEV_JIG_UART_ON: c_int = 10;
pub const DEV_JIG_USB_OFF: c_int = 9;
pub const DEV_JIG_USB_ON: c_int = 8;

    DEV_JIG_UART_OFF | DEV_JIG_UART_ON)
//
// Manual Switch
// D- [7:5] / D+ [4:2]
// 000: Open all / 001: USB / 010: AUDIO / 011: UART / 100: V_AUDIO
//

// Interrupt 1

// Interrupt 2 mask

// Timing Set 1

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsa9480_usbsw {
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub edev: *mut extcon_dev,
    pub cable: u16,
}

    static const unsigned int fsa9480_extcon_cable[] = {
    EXTCON_USB_HOST,
    EXTCON_USB,
    EXTCON_CHG_USB_DCP,
    EXTCON_CHG_USB_SDP,
    EXTCON_CHG_USB_ACA,
    EXTCON_JACK_LINE_OUT,
    EXTCON_JACK_VIDEO_OUT,
    EXTCON_JIG,
    EXTCON_NONE,
    };
    static const u64 cable_types[] = {
    [DEV_USB_OTG] = BIT_ULL(EXTCON_USB_HOST),
    [DEV_DEDICATED_CHG] = BIT_ULL(EXTCON_USB) | BIT_ULL(EXTCON_CHG_USB_DCP),
    [DEV_USB_CHG] = BIT_ULL(EXTCON_USB) | BIT_ULL(EXTCON_CHG_USB_SDP),
    [DEV_CAR_KIT] = BIT_ULL(EXTCON_USB) | BIT_ULL(EXTCON_CHG_USB_SDP)
    | BIT_ULL(EXTCON_JACK_LINE_OUT),
    [DEV_UART] = BIT_ULL(EXTCON_JIG),
    [DEV_USB] = BIT_ULL(EXTCON_USB) | BIT_ULL(EXTCON_CHG_USB_SDP),
    [DEV_AUDIO_2] = BIT_ULL(EXTCON_JACK_LINE_OUT),
    [DEV_AUDIO_1] = BIT_ULL(EXTCON_JACK_LINE_OUT),
    [DEV_AV] = BIT_ULL(EXTCON_JACK_LINE_OUT)
    | BIT_ULL(EXTCON_JACK_VIDEO_OUT),
    [DEV_TTY] = BIT_ULL(EXTCON_JIG),
    [DEV_PPD] = BIT_ULL(EXTCON_JACK_LINE_OUT) | BIT_ULL(EXTCON_CHG_USB_ACA),
    [DEV_JIG_UART_OFF] = BIT_ULL(EXTCON_JIG),
    [DEV_JIG_UART_ON] = BIT_ULL(EXTCON_JIG),
    [DEV_JIG_USB_OFF] = BIT_ULL(EXTCON_USB) | BIT_ULL(EXTCON_JIG),
    [DEV_JIG_USB_ON] = BIT_ULL(EXTCON_USB) | BIT_ULL(EXTCON_JIG),
    };
// Define regmap configuration of FSA9480 for I2C communication
#[no_mangle]
unsafe extern "C" fn fsa9480_volatile_reg(dev: *mut device, reg: c_uint) -> bool {
    static bool fsa9480_volatile_reg(struct device *dev, unsigned int reg)
    {
    switch (reg) {
    case FSA9480_REG_INT1_MASK:
    return true;
    default:
    break;
    }
    return false;
    }
    static const struct regmap_config fsa9480_regmap_config = {
    .reg_bits	= 8,
    .val_bits	= 8,
    .volatile_reg	= fsa9480_volatile_reg,
    .max_register	= FSA9480_REG_END,
    };
#[no_mangle]
unsafe extern "C" fn fsa9480_write_reg(usbsw: *mut fsa9480_usbsw, reg: c_int, value: c_int) -> c_int {
    static int fsa9480_write_reg(struct fsa9480_usbsw *usbsw, int reg, int value)
    {
    int ret;
    ret = regmap_write(usbsw.regmap, reg, value);
    if (ret < 0)
    dev_err(usbsw.dev, "%s: err %d\n", __func__, ret);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn fsa9480_read_reg(usbsw: *mut fsa9480_usbsw, reg: c_int) -> c_int {
    static int fsa9480_read_reg(struct fsa9480_usbsw *usbsw, int reg)
    {
    int ret, val;
    ret = regmap_read(usbsw.regmap, reg, &val);
    if (ret < 0) {
    dev_err(usbsw.dev, "%s: err %d\n", __func__, ret);
    return ret;
    }
    return val;
    }
#[no_mangle]
unsafe extern "C" fn fsa9480_read_irq(usbsw: *mut fsa9480_usbsw, value: *mut c_int) -> c_int {
    static int fsa9480_read_irq(struct fsa9480_usbsw *usbsw, int *value)
    {
    u8 regs[2];
    int ret;
    ret = regmap_bulk_read(usbsw.regmap, FSA9480_REG_INT1, regs, 2);
    if (ret < 0)
    dev_err(usbsw.dev, "%s: err %d\n", __func__, ret);
// value = regs[1] << 8 | regs[0];
    return ret;
    }
    static void fsa9480_handle_change(struct fsa9480_usbsw *usbsw,
    u16 mask, bool attached)
    {
    while (mask) {
    let mut dev: c_int = fls64(mask) - 1;
    let mut cables: u64 = cable_types[dev];
    while (cables) {
    let mut cable: c_int = fls64(cables) - 1;
    extcon_set_state_sync(usbsw.edev, cable, attached);
    cables &= ~BIT_ULL(cable);
    }
    mask &= ~BIT_ULL(dev);
    }
    }
#[no_mangle]
unsafe extern "C" fn fsa9480_detect_dev(usbsw: *mut fsa9480_usbsw) {
    static void fsa9480_detect_dev(struct fsa9480_usbsw *usbsw)
    {
    int val1, val2;
    u16 val;
    val1 = fsa9480_read_reg(usbsw, FSA9480_REG_DEV_T1);
    val2 = fsa9480_read_reg(usbsw, FSA9480_REG_DEV_T2);
    if (val1 < 0 || val2 < 0) {
    dev_err(usbsw.dev, "%s: failed to read registers", __func__);
    return;
    }
    val = val2 << 8 | val1;
    dev_info(usbsw.dev, "dev1: 0x%x, dev2: 0x%x\n", val1, val2);
// handle detached cables first
    fsa9480_handle_change(usbsw, usbsw.cable & ~val, false);
// then handle attached ones
    fsa9480_handle_change(usbsw, val & ~usbsw.cable, true);
    usbsw.cable = val;
    }
#[no_mangle]
unsafe extern "C" fn fsa9480_irq_handler(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t fsa9480_irq_handler(int irq, void *data)
    {
    struct fsa9480_usbsw *usbsw = data;
    let mut intr: c_int = 0;
// clear interrupt
    fsa9480_read_irq(usbsw, &intr);
    if (!intr)
    return IRQ_NONE;
// device detection
    fsa9480_detect_dev(usbsw);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn fsa9480_probe(client: *mut i2c_client) -> c_int {
    static int fsa9480_probe(struct i2c_client *client)
    {
    struct fsa9480_usbsw *info;
    int ret;
    if (!client.irq) {
    dev_err(&client.dev, "no interrupt provided\n");
    return -EINVAL;
    }
    info = devm_kzalloc(&client.dev, sizeof(*info), GFP_KERNEL);
    if (!info)
    return -ENOMEM;
    info.dev = &client.dev;
    i2c_set_clientdata(client, info);
// External connector
    info.edev = devm_extcon_dev_allocate(info.dev,
    fsa9480_extcon_cable);
    if (IS_ERR(info.edev)) {
    dev_err(info.dev, "failed to allocate memory for extcon\n");
    ret = -ENOMEM;
    return ret;
    }
    ret = devm_extcon_dev_register(info.dev, info.edev);
    if (ret) {
    dev_err(info.dev, "failed to register extcon device\n");
    return ret;
    }
    info.regmap = devm_regmap_init_i2c(client, &fsa9480_regmap_config);
    if (IS_ERR(info.regmap)) {
    ret = PTR_ERR(info.regmap);
    dev_err(info.dev, "failed to allocate register map: %d\n",
    ret);
    return ret;
    }
// ADC Detect Time: 500ms
    fsa9480_write_reg(info, FSA9480_REG_TIMING1, TIMING1_ADC_500MS);
// configure automatic switching
    fsa9480_write_reg(info, FSA9480_REG_CTRL, CON_MASK);
// unmask interrupt (attach/detach only)
    fsa9480_write_reg(info, FSA9480_REG_INT1_MASK,
    INT1_MASK & ~(INT_ATTACH | INT_DETACH));
    fsa9480_write_reg(info, FSA9480_REG_INT2_MASK, INT2_MASK);
    ret = devm_request_threaded_irq(info.dev, client.irq, core::ptr::null_mut(),
    fsa9480_irq_handler,
    IRQF_TRIGGER_FALLING | IRQF_ONESHOT,
    "fsa9480", info);
    if (ret) {
    dev_err(info.dev, "failed to request IRQ\n");
    return ret;
    }
    devm_device_init_wakeup(info.dev);
    fsa9480_detect_dev(info);
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn fsa9480_suspend(dev: *mut device) -> c_int {
    static int fsa9480_suspend(struct device *dev)
    {
    struct i2c_client *client = to_i2c_client(dev);
    if (device_may_wakeup(&client.dev) && client.irq)
    enable_irq_wake(client.irq);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn fsa9480_resume(dev: *mut device) -> c_int {
    static int fsa9480_resume(struct device *dev)
    {
    struct i2c_client *client = to_i2c_client(dev);
    if (device_may_wakeup(&client.dev) && client.irq)
    disable_irq_wake(client.irq);
    return 0;
    }

    static const struct dev_pm_ops fsa9480_pm_ops = {
    SET_SYSTEM_SLEEP_PM_OPS(fsa9480_suspend, fsa9480_resume)
    };
    static const struct i2c_device_id fsa9480_id[] = {
    { "fsa9480" },
    {}
    };
    MODULE_DEVICE_TABLE(i2c, fsa9480_id);
    static const struct of_device_id fsa9480_of_match[] = {
    { .compatible = "fcs,fsa9480", },
    { .compatible = "fcs,fsa880", },
    { .compatible = "ti,tsu6111", },
    { },
    };
    MODULE_DEVICE_TABLE(of, fsa9480_of_match);
    static struct i2c_driver fsa9480_i2c_driver = {
    .driver			= {
    .name		= "fsa9480",
    .pm		= &fsa9480_pm_ops,
    .of_match_table = fsa9480_of_match,
    },
    .probe			= fsa9480_probe,
    .id_table		= fsa9480_id,
    };
#[no_mangle]
unsafe extern "C" fn fsa9480_module_init() -> int __init {
    static int __init fsa9480_module_init(void)
    {
    return i2c_add_driver(&fsa9480_i2c_driver);
    }
    subsys_initcall(fsa9480_module_init);
#[no_mangle]
unsafe extern "C" fn fsa9480_module_exit() -> void __exit {
    static void __exit fsa9480_module_exit(void)
    {
    i2c_del_driver(&fsa9480_i2c_driver);
    }
    module_exit(fsa9480_module_exit);
    MODULE_DESCRIPTION("Fairchild Semiconductor FSA9480 extcon driver");
    MODULE_AUTHOR("Tomasz Figa <tomasz.figa@gmail.com>");
    MODULE_LICENSE("GPL");
