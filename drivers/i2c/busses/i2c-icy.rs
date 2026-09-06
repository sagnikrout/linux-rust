//! Automatically rewritten from C to Rust
//! Source: drivers/i2c/busses/i2c-icy.c
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
// I2C driver for stand-alone PCF8584 style adapters on Zorro cards
//
// Original ICY documentation can be found on Aminet:
// https://aminet.net/package/docs/hard/icy
//
// There has been a modern community re-print of this design in 2019:
// https://www.a1k.org/forum/index.php?threads/70106
//
// The card is basically a Philips PCF8584 connected straight to the
// beginning of the AutoConfig'd address space (register S1 on base+2),
// with /INT on /INT2 on the Zorro bus.
//
// Copyright (c) 2019 Max Staudt <max@enpas.org>
//
// This started as a fork of i2c-elektor.c and has evolved since.
// Thanks go to its authors for providing a base to grow on.
//
// IRQ support is currently not implemented.
//
// As it turns out, i2c-algo-pcf is really written with i2c-elektor's
// edge-triggered ISA interrupts in mind, while the Amiga's Zorro bus has
// level-triggered interrupts. This means that once an interrupt occurs, we
// have to tell the PCF8584 to shut up immediately, or it will keep the
// interrupt line busy and cause an IRQ storm.
// However, because of the PCF8584's host-side protocol, there is no good
// way to just quieten it without side effects. Rather, we have to perform
// the next read/write operation straight away, which will reset the /INT
// pin. This entails re-designing the core of i2c-algo-pcf in the future.
// For now, we never request an IRQ from the PCF8584, and poll it instead.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icy_i2c {
    pub adapter: i2c_adapter,
    pub reg_s0: *mut void __iomem,
    pub reg_s1: *mut void __iomem,
    pub ltc2990_client: *mut i2c_client,
}

//
// Functions called by i2c-algo-pcf
//
#[no_mangle]
unsafe extern "C" fn icy_pcf_setpcf(data: *mut c_void, ctl: c_int, val: c_int) {
    static void icy_pcf_setpcf(void *data, int ctl, int val)
    {
    struct icy_i2c *i2c = (struct icy_i2c *)data;
    u8 __iomem *address = ctl ? i2c.reg_s1 : i2c.reg_s0;
    z_writeb(val, address);
    }
#[no_mangle]
unsafe extern "C" fn icy_pcf_getpcf(data: *mut c_void, ctl: c_int) -> c_int {
    static int icy_pcf_getpcf(void *data, int ctl)
    {
    struct icy_i2c *i2c = (struct icy_i2c *)data;
    u8 __iomem *address = ctl ? i2c.reg_s1 : i2c.reg_s0;
    return z_readb(address);
    }
#[no_mangle]
unsafe extern "C" fn icy_pcf_getown(data: *mut c_void) -> c_int {
    static int icy_pcf_getown(void *data)
    {
    return 0x55;
    }
#[no_mangle]
unsafe extern "C" fn icy_pcf_getclock(data: *mut c_void) -> c_int {
    static int icy_pcf_getclock(void *data)
    {
    return 0x1c;
    }
#[no_mangle]
unsafe extern "C" fn icy_pcf_waitforpin(data: *mut c_void) {
    static void icy_pcf_waitforpin(void *data)
    {
    usleep_range(50, 150);
    }
//
// Main i2c-icy part
//
    static unsigned short const icy_ltc2990_addresses[] = {
    0x4c, 0x4d, 0x4e, 0x4f, I2C_CLIENT_END
    };
//
// Additional sensors exposed once this property is applied:
//
// in1 will be the voltage of the 5V rail, divided by 2.
// in2 will be the voltage of the 12V rail, divided by 4.
// temp3 will be measured using a PCB loop next the chip.
//
    static const u32 icy_ltc2990_meas_mode[] = {0, 3};
    static const struct property_entry icy_ltc2990_props[] = {
    PROPERTY_ENTRY_U32_ARRAY("lltc,meas-mode", icy_ltc2990_meas_mode),
    { }
    };
    static const struct software_node icy_ltc2990_node = {
    .properties = icy_ltc2990_props,
    };
    static int icy_probe(struct zorro_dev *z,
    const struct zorro_device_id *ent)
    {
    struct icy_i2c *i2c;
    struct i2c_algo_pcf_data *algo_data;
    struct i2c_board_info ltc2990_info = {
    .type		= "ltc2990",
    .swnode		= &icy_ltc2990_node,
    };
    i2c = devm_kzalloc(&z.dev, sizeof(*i2c), GFP_KERNEL);
    if (!i2c)
    return -ENOMEM;
    algo_data = devm_kzalloc(&z.dev, sizeof(*algo_data), GFP_KERNEL);
    if (!algo_data)
    return -ENOMEM;
    dev_set_drvdata(&z.dev, i2c);
    i2c.adapter.dev.parent = &z.dev;
    i2c.adapter.owner = THIS_MODULE;
// i2c->adapter.algo assigned by i2c_pcf_add_bus()
    i2c.adapter.algo_data = algo_data;
    strscpy(i2c.adapter.name, "ICY I2C Zorro adapter",
    sizeof(i2c.adapter.name));
    if (!devm_request_mem_region(&z.dev,
    z.resource.start,
    4, i2c.adapter.name))
    return -ENXIO;
// Driver private data
    i2c.reg_s0 = ZTWO_VADDR(z.resource.start);
    i2c.reg_s1 = ZTWO_VADDR(z.resource.start + 2);
    algo_data.data = i2c;
    algo_data.setpcf     = icy_pcf_setpcf;
    algo_data.getpcf     = icy_pcf_getpcf;
    algo_data.getown     = icy_pcf_getown;
    algo_data.getclock   = icy_pcf_getclock;
    algo_data.waitforpin = icy_pcf_waitforpin;
    if (i2c_pcf_add_bus(&i2c.adapter)) {
    dev_err(&z.dev, "i2c_pcf_add_bus() failed\n");
    return -ENXIO;
    }
    dev_info(&z.dev, "ICY I2C controller at %pa, IRQ not implemented\n",
    &z.resource.start);
//
// The 2019 a1k.org PCBs have an LTC2990 at 0x4c, so start
// it automatically once ltc2990 is modprobed.
//
// in0 is the voltage of the internal 5V power supply.
// temp1 is the temperature inside the chip.
//
// See property_entry above for in1, in2, temp3.
//
    i2c.ltc2990_client = i2c_new_scanned_device(&i2c.adapter,
    &ltc2990_info,
    icy_ltc2990_addresses,
    core::ptr::null_mut());
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn icy_remove(z: *mut zorro_dev) {
    static void icy_remove(struct zorro_dev *z)
    {
    struct icy_i2c *i2c = dev_get_drvdata(&z.dev);
    i2c_unregister_device(i2c.ltc2990_client);
    i2c_del_adapter(&i2c.adapter);
    }
    static const struct zorro_device_id icy_zorro_tbl[] = {
    { .id = ZORRO_ID(VMC, 15, 0) },
    { }
    };
    MODULE_DEVICE_TABLE(zorro, icy_zorro_tbl);
    static struct zorro_driver icy_driver = {
    .name           = "i2c-icy",
    .id_table       = icy_zorro_tbl,
    .probe          = icy_probe,
    .remove         = icy_remove,
    };
    module_driver(icy_driver,
    zorro_register_driver,
    zorro_unregister_driver);
    MODULE_AUTHOR("Max Staudt <max@enpas.org>");
    MODULE_DESCRIPTION("I2C bus via PCF8584 on ICY Zorro card");
    MODULE_LICENSE("GPL v2");
