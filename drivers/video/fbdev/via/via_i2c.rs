//! Automatically rewritten from C to Rust
//! Source: drivers/video/fbdev/via/via_i2c.c
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
// Copyright 1998-2009 VIA Technologies, Inc. All Rights Reserved.
// Copyright 2001-2008 S3 Graphics, Inc. All Rights Reserved.
//

//
// There can only be one set of these, so there's no point in having
// them be dynamically allocated...
//
pub const VIAFB_NUM_I2C: c_int = 5;
    static struct via_i2c_stuff via_i2c_par[VIAFB_NUM_I2C];
    static struct viafb_dev *i2c_vdev;  /* Passed in from core */
#[no_mangle]
unsafe extern "C" fn via_i2c_setscl(data: *mut c_void, state: c_int) {
    static void via_i2c_setscl(void *data, int state)
    {
    u8 val;
    struct via_port_cfg *adap_data = data;
    unsigned long flags;
    spin_lock_irqsave(&i2c_vdev.reg_lock, flags);
    val = via_read_reg(adap_data.io_port, adap_data.ioport_index) & 0xF0;
    if (state)
    val |= 0x20;
    else
    val &= ~0x20;
    switch (adap_data.type) {
    case VIA_PORT_I2C:
    val |= 0x01;
    break;
    case VIA_PORT_GPIO:
    val |= 0x82;
    break;
    default:
    printk(KERN_ERR "viafb_i2c: specify wrong i2c type.\n");
    }
    via_write_reg(adap_data.io_port, adap_data.ioport_index, val);
    spin_unlock_irqrestore(&i2c_vdev.reg_lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn via_i2c_getscl(data: *mut c_void) -> c_int {
    static int via_i2c_getscl(void *data)
    {
    struct via_port_cfg *adap_data = data;
    unsigned long flags;
    let mut ret: c_int = 0;
    spin_lock_irqsave(&i2c_vdev.reg_lock, flags);
    if (adap_data.type == VIA_PORT_GPIO)
    via_write_reg_mask(adap_data.io_port, adap_data.ioport_index,
    0, 0x80);
    if (via_read_reg(adap_data.io_port, adap_data.ioport_index) & 0x08)
    ret = 1;
    spin_unlock_irqrestore(&i2c_vdev.reg_lock, flags);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn via_i2c_getsda(data: *mut c_void) -> c_int {
    static int via_i2c_getsda(void *data)
    {
    struct via_port_cfg *adap_data = data;
    unsigned long flags;
    let mut ret: c_int = 0;
    spin_lock_irqsave(&i2c_vdev.reg_lock, flags);
    if (adap_data.type == VIA_PORT_GPIO)
    via_write_reg_mask(adap_data.io_port, adap_data.ioport_index,
    0, 0x40);
    if (via_read_reg(adap_data.io_port, adap_data.ioport_index) & 0x04)
    ret = 1;
    spin_unlock_irqrestore(&i2c_vdev.reg_lock, flags);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn via_i2c_setsda(data: *mut c_void, state: c_int) {
    static void via_i2c_setsda(void *data, int state)
    {
    u8 val;
    struct via_port_cfg *adap_data = data;
    unsigned long flags;
    spin_lock_irqsave(&i2c_vdev.reg_lock, flags);
    val = via_read_reg(adap_data.io_port, adap_data.ioport_index) & 0xF0;
    if (state)
    val |= 0x10;
    else
    val &= ~0x10;
    switch (adap_data.type) {
    case VIA_PORT_I2C:
    val |= 0x01;
    break;
    case VIA_PORT_GPIO:
    val |= 0x42;
    break;
    default:
    printk(KERN_ERR "viafb_i2c: specify wrong i2c type.\n");
    }
    via_write_reg(adap_data.io_port, adap_data.ioport_index, val);
    spin_unlock_irqrestore(&i2c_vdev.reg_lock, flags);
    }
#[no_mangle]
pub unsafe extern "C" fn viafb_i2c_readbyte(adap: u8, target_addr: u8, index: u8, pdata: *mut u8) -> c_int {
    int viafb_i2c_readbyte(u8 adap, u8 target_addr, u8 index, u8 *pdata)
    {
    int ret;
    u8 mm1[] = {0x00};
    struct i2c_msg msgs[2];
    if (!via_i2c_par[adap].is_active)
    return -ENODEV;
// pdata = 0;
    msgs[0].flags = 0;
    msgs[1].flags = I2C_M_RD;
    msgs[0].addr = msgs[1].addr = target_addr / 2;
    mm1[0] = index;
    msgs[0].len = 1; msgs[1].len = 1;
    msgs[0].buf = mm1; msgs[1].buf = pdata;
    ret = i2c_transfer(&via_i2c_par[adap].adapter, msgs, 2);
    if (ret == 2)
    ret = 0;
#[no_mangle]
pub unsafe extern "C" fn if(0: ret >=) -> else {
    else if (ret >= 0)
    ret = -EIO;
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn viafb_i2c_writebyte(adap: u8, target_addr: u8, index: u8, data: u8) -> c_int {
    int viafb_i2c_writebyte(u8 adap, u8 target_addr, u8 index, u8 data)
    {
    int ret;
    u8 msg[2] = { index, data };
    struct i2c_msg msgs;
    if (!via_i2c_par[adap].is_active)
    return -ENODEV;
    msgs.flags = 0;
    msgs.addr = target_addr / 2;
    msgs.len = 2;
    msgs.buf = msg;
    ret = i2c_transfer(&via_i2c_par[adap].adapter, &msgs, 1);
    if (ret == 1)
    ret = 0;
#[no_mangle]
pub unsafe extern "C" fn if(0: ret >=) -> else {
    else if (ret >= 0)
    ret = -EIO;
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn viafb_i2c_readbytes(adap: u8, target_addr: u8, index: u8, buff: *mut u8, buff_len: c_int) -> c_int {
    int viafb_i2c_readbytes(u8 adap, u8 target_addr, u8 index, u8 *buff, int buff_len)
    {
    int ret;
    u8 mm1[] = {0x00};
    struct i2c_msg msgs[2];
    if (!via_i2c_par[adap].is_active)
    return -ENODEV;
    msgs[0].flags = 0;
    msgs[1].flags = I2C_M_RD;
    msgs[0].addr = msgs[1].addr = target_addr / 2;
    mm1[0] = index;
    msgs[0].len = 1; msgs[1].len = buff_len;
    msgs[0].buf = mm1; msgs[1].buf = buff;
    ret = i2c_transfer(&via_i2c_par[adap].adapter, msgs, 2);
    if (ret == 2)
    ret = 0;
#[no_mangle]
pub unsafe extern "C" fn if(0: ret >=) -> else {
    else if (ret >= 0)
    ret = -EIO;
    return ret;
    }
//
// Allow other viafb subdevices to look up a specific adapter
// by port name.
//
    struct i2c_adapter *viafb_find_i2c_adapter(enum viafb_i2c_adap which)
    {
    struct via_i2c_stuff *stuff = &via_i2c_par[which];
    return &stuff.adapter;
    }
    EXPORT_SYMBOL_GPL(viafb_find_i2c_adapter);
    static int create_i2c_bus(struct i2c_adapter *adapter,
    struct i2c_algo_bit_data *algo,
    struct via_port_cfg *adap_cfg,
    struct pci_dev *pdev)
    {
    algo.setsda = via_i2c_setsda;
    algo.setscl = via_i2c_setscl;
    algo.getsda = via_i2c_getsda;
    algo.getscl = via_i2c_getscl;
    algo.udelay = 10;
    algo.timeout = 2;
    algo.data = adap_cfg;
    sprintf(adapter.name, "viafb i2c io_port idx 0x%02x",
    adap_cfg.ioport_index);
    adapter.owner = THIS_MODULE;
    adapter.algo_data = algo;
    if (pdev)
    adapter.dev.parent = &pdev.dev;
    else
    adapter.dev.parent = core::ptr::null_mut();
// i2c_set_adapdata(adapter, adap_cfg);
// Raise SCL and SDA
    via_i2c_setsda(adap_cfg, 1);
    via_i2c_setscl(adap_cfg, 1);
    udelay(20);
    return i2c_bit_add_bus(adapter);
    }
#[no_mangle]
unsafe extern "C" fn viafb_i2c_probe(platdev: *mut platform_device) -> c_int {
    static int viafb_i2c_probe(struct platform_device *platdev)
    {
    int i, ret;
    struct via_port_cfg *configs;
    i2c_vdev = platdev.dev.platform_data;
    configs = i2c_vdev.port_cfg;
    for (i = 0; i < VIAFB_NUM_PORTS; i++) {
    struct via_port_cfg *adap_cfg = configs++;
    struct via_i2c_stuff *i2c_stuff = &via_i2c_par[i];
    i2c_stuff.is_active = 0;
    if (adap_cfg.type == 0 || adap_cfg.mode != VIA_MODE_I2C)
    continue;
    ret = create_i2c_bus(&i2c_stuff.adapter,
    &i2c_stuff.algo, adap_cfg,
    core::ptr::null_mut()); /* FIXME: PCIDEV */
    if (ret < 0) {
    printk(KERN_ERR "viafb: cannot create i2c bus %u:%d\n",
    i, ret);
    continue;  /* Still try to make the rest */
    }
    i2c_stuff.is_active = 1;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn viafb_i2c_remove(platdev: *mut platform_device) {
    static void viafb_i2c_remove(struct platform_device *platdev)
    {
    int i;
    for (i = 0; i < VIAFB_NUM_PORTS; i++) {
    struct via_i2c_stuff *i2c_stuff = &via_i2c_par[i];
//
// Only remove those entries in the array that we've
// actually used (and thus initialized algo_data)
//
    if (i2c_stuff.is_active)
    i2c_del_adapter(&i2c_stuff.adapter);
    }
    }
    static struct platform_driver via_i2c_driver = {
    .driver = {
    .name = "viafb-i2c",
    },
    .probe = viafb_i2c_probe,
    .remove = viafb_i2c_remove,
    };
#[no_mangle]
pub unsafe extern "C" fn viafb_i2c_init() -> c_int {
    int viafb_i2c_init(void)
    {
    return platform_driver_register(&via_i2c_driver);
    }
#[no_mangle]
pub unsafe extern "C" fn viafb_i2c_exit() {
    void viafb_i2c_exit(void)
    {
    platform_driver_unregister(&via_i2c_driver);
    }
