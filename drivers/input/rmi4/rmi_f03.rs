//! Automatically rewritten from C to Rust
//! Source: drivers/input/rmi4/rmi_f03.c
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
// Copyright (C) 2015-2016 Red Hat
// Copyright (C) 2015 Lyude Paul <thatslyude@gmail.com>
//

pub const RMI_F03_RX_DATA_OFB: c_uint = 0x01;
pub const RMI_F03_OB_SIZE: c_int = 2;
pub const RMI_F03_OB_OFFSET: c_int = 2;
pub const RMI_F03_OB_DATA_OFFSET: c_int = 1;

pub const RMI_F03_DEVICE_COUNT: c_uint = 0x07;
pub const RMI_F03_BYTES_PER_DEVICE: c_uint = 0x07;
pub const RMI_F03_BYTES_PER_DEVICE_SHIFT: c_int = 4;
pub const RMI_F03_QUEUE_LENGTH: c_uint = 0x0F;
pub const PSMOUSE_OOB_EXTRA_BTNS: c_uint = 0x01;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct f03_data {
    pub fn: *mut rmi_function,
    pub serio: *mut serio,
    pub serio_registered: bool,
    pub overwrite_buttons: c_uint,
    pub device_count: u8,
    pub rx_queue_length: u8,
}

    int rmi_f03_overwrite_button(struct rmi_function *fn, unsigned int button,
    int value)
    {
    struct f03_data *f03 = dev_get_drvdata(&fn.dev);
    unsigned int bit;
    if (button < BTN_LEFT || button > BTN_MIDDLE)
    return -EINVAL;
    bit = BIT(button - BTN_LEFT);
    if (value)
    f03.overwrite_buttons |= bit;
    else
    f03.overwrite_buttons &= ~bit;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn rmi_f03_commit_buttons(fn: *mut rmi_function) {
    void rmi_f03_commit_buttons(struct rmi_function *fn)
    {
    struct f03_data *f03 = dev_get_drvdata(&fn.dev);
    struct serio *serio = f03.serio;
    guard(serio_pause_rx)(serio);
    if (serio.drv) {
    serio.drv.interrupt(serio, PSMOUSE_OOB_EXTRA_BTNS,
    SERIO_OOB_DATA);
    serio.drv.interrupt(serio, f03.overwrite_buttons,
    SERIO_OOB_DATA);
    }
    }
#[no_mangle]
unsafe extern "C" fn rmi_f03_pt_write(id: *mut serio, val: c_uchar) -> c_int {
    static int rmi_f03_pt_write(struct serio *id, unsigned char val)
    {
    struct f03_data *f03 = id.port_data;
    int error;
    rmi_dbg(RMI_DEBUG_FN, &f03.fn.dev,
    "%s: Wrote %.2hhx to PS/2 passthrough address",
    __func__, val);
    error = rmi_write(f03.fn.rmi_dev, f03.fn.fd.data_base_addr, val);
    if (error) {
    dev_err(&f03.fn.dev,
    "%s: Failed to write to F03 TX register (%d).\n",
    __func__, error);
    return error;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rmi_f03_initialize(f03: *mut f03_data) -> c_int {
    static int rmi_f03_initialize(struct f03_data *f03)
    {
    struct rmi_function *fn = f03.fn;
    struct device *dev = &fn.dev;
    int error;
    u8 bytes_per_device;
    u8 query1;
    u8 query2[RMI_F03_DEVICE_COUNT * RMI_F03_BYTES_PER_DEVICE];
    size_t query2_len;
    error = rmi_read(fn.rmi_dev, fn.fd.query_base_addr, &query1);
    if (error) {
    dev_err(dev, "Failed to read query register (%d).\n", error);
    return error;
    }
    f03.device_count = query1 & RMI_F03_DEVICE_COUNT;
    bytes_per_device = (query1 >> RMI_F03_BYTES_PER_DEVICE_SHIFT) &
    RMI_F03_BYTES_PER_DEVICE;
    query2_len = f03.device_count * bytes_per_device;
//
// The first generation of image sensors don't have a second part to
// their f03 query, as such we have to set some of these values manually
//
    if (query2_len < 1) {
    f03.device_count = 1;
    f03.rx_queue_length = 7;
    } else {
    error = rmi_read_block(fn.rmi_dev, fn.fd.query_base_addr + 1,
    query2, query2_len);
    if (error) {
    dev_err(dev,
    "Failed to read second set of query registers (%d).\n",
    error);
    return error;
    }
    f03.rx_queue_length = query2[0] & RMI_F03_QUEUE_LENGTH;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rmi_f03_pt_open(serio: *mut serio) -> c_int {
    static int rmi_f03_pt_open(struct serio *serio)
    {
    struct f03_data *f03 = serio.port_data;
    struct rmi_function *fn = f03.fn;
    let mut ob_len: u8 = f03.rx_queue_length * RMI_F03_OB_SIZE;
    let mut data_addr: u16 = fn.fd.data_base_addr + RMI_F03_OB_OFFSET;
    u8 obs[RMI_F03_QUEUE_LENGTH * RMI_F03_OB_SIZE];
    int error;
//
// Consume any pending data. Some devices like to spam with
// 0xaa 0x00 announcements which may confuse us as we try to
// probe the device.
//
    error = rmi_read_block(fn.rmi_dev, data_addr, &obs, ob_len);
    if (!error)
    rmi_dbg(RMI_DEBUG_FN, &fn.dev,
    "%s: Consumed %*ph (%d) from PS2 guest\n",
    __func__, ob_len, obs, ob_len);
    return fn.rmi_dev.driver.set_irq_bits(fn.rmi_dev, fn.irq_mask);
    }
#[no_mangle]
unsafe extern "C" fn rmi_f03_pt_close(serio: *mut serio) {
    static void rmi_f03_pt_close(struct serio *serio)
    {
    struct f03_data *f03 = serio.port_data;
    struct rmi_function *fn = f03.fn;
    fn.rmi_dev.driver.clear_irq_bits(fn.rmi_dev, fn.irq_mask);
    }
#[no_mangle]
unsafe extern "C" fn rmi_f03_register_pt(f03: *mut f03_data) -> c_int {
    static int rmi_f03_register_pt(struct f03_data *f03)
    {
    struct serio *serio;
    serio = kzalloc_obj(struct serio);
    if (!serio)
    return -ENOMEM;
    serio.id.type = SERIO_PS_PSTHRU;
    serio.write = rmi_f03_pt_write;
    serio.open = rmi_f03_pt_open;
    serio.close = rmi_f03_pt_close;
    serio.port_data = f03;
    strscpy(serio.name, "RMI4 PS/2 pass-through", sizeof(serio.name));
    snprintf(serio.phys, sizeof(serio.phys), "%s/serio0",
    dev_name(&f03.fn.dev));
    serio.dev.parent = &f03.fn.dev;
    f03.serio = serio;
    printk(KERN_INFO "serio: %s port at %s\n",
    serio.name, dev_name(&f03.fn.dev));
    serio_register_port(serio);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rmi_f03_probe(fn: *mut rmi_function) -> c_int {
    static int rmi_f03_probe(struct rmi_function *fn)
    {
    struct device *dev = &fn.dev;
    struct f03_data *f03;
    int error;
    f03 = devm_kzalloc(dev, sizeof(struct f03_data), GFP_KERNEL);
    if (!f03)
    return -ENOMEM;
    f03.fn = fn;
    error = rmi_f03_initialize(f03);
    if (error < 0)
    return error;
    if (f03.device_count != 1)
    dev_warn(dev, "found %d devices on PS/2 passthrough",
    f03.device_count);
    dev_set_drvdata(dev, f03);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rmi_f03_config(fn: *mut rmi_function) -> c_int {
    static int rmi_f03_config(struct rmi_function *fn)
    {
    struct f03_data *f03 = dev_get_drvdata(&fn.dev);
    int error;
    if (!f03.serio_registered) {
    error = rmi_f03_register_pt(f03);
    if (error)
    return error;
    f03.serio_registered = true;
    } else {
//
// We must be re-configuring the sensor, just enable
// interrupts for this function.
//
    fn.rmi_dev.driver.set_irq_bits(fn.rmi_dev, fn.irq_mask);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rmi_f03_attention(irq: c_int, ctx: *mut c_void) -> irqreturn_t {
    static irqreturn_t rmi_f03_attention(int irq, void *ctx)
    {
    struct rmi_function *fn = ctx;
    struct rmi_device *rmi_dev = fn.rmi_dev;
    struct rmi_driver_data *drvdata = dev_get_drvdata(&rmi_dev.dev);
    struct f03_data *f03 = dev_get_drvdata(&fn.dev);
    let mut data_addr: u16 = fn.fd.data_base_addr + RMI_F03_OB_OFFSET;
    let mut ob_len: u8 = f03.rx_queue_length * RMI_F03_OB_SIZE;
    u8 obs[RMI_F03_QUEUE_LENGTH * RMI_F03_OB_SIZE];
    u8 ob_status;
    u8 ob_data;
    unsigned int serio_flags;
    int i;
    int error;
    if (drvdata.attn_data.data) {
// First grab the data passed by the transport device
    if (drvdata.attn_data.size < ob_len) {
    dev_warn(&fn.dev, "F03 interrupted, but data is missing!\n");
    return IRQ_HANDLED;
    }
    memcpy(obs, drvdata.attn_data.data, ob_len);
    drvdata.attn_data.data += ob_len;
    drvdata.attn_data.size -= ob_len;
    } else {
// Grab all of the data registers, and check them for data
    error = rmi_read_block(fn.rmi_dev, data_addr, &obs, ob_len);
    if (error) {
    dev_err(&fn.dev,
    "%s: Failed to read F03 output buffers: %d\n",
    __func__, error);
    serio_interrupt(f03.serio, 0, SERIO_TIMEOUT);
    return IRQ_RETVAL(error);
    }
    }
    for (i = 0; i < ob_len; i += RMI_F03_OB_SIZE) {
    ob_status = obs[i];
    ob_data = obs[i + RMI_F03_OB_DATA_OFFSET];
    serio_flags = 0;
    if (!(ob_status & RMI_F03_RX_DATA_OFB))
    continue;
    if (ob_status & RMI_F03_OB_FLAG_TIMEOUT)
    serio_flags |= SERIO_TIMEOUT;
    if (ob_status & RMI_F03_OB_FLAG_PARITY)
    serio_flags |= SERIO_PARITY;
    rmi_dbg(RMI_DEBUG_FN, &fn.dev,
    "%s: Received %.2hhx from PS2 guest T: %c P: %c\n",
    __func__, ob_data,
    serio_flags & SERIO_TIMEOUT ?  'Y' : 'N',
    serio_flags & SERIO_PARITY ? 'Y' : 'N');
    serio_interrupt(f03.serio, ob_data, serio_flags);
    }
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn rmi_f03_remove(fn: *mut rmi_function) {
    static void rmi_f03_remove(struct rmi_function *fn)
    {
    struct f03_data *f03 = dev_get_drvdata(&fn.dev);
    if (f03.serio_registered)
    serio_unregister_port(f03.serio);
    }
    struct rmi_function_handler rmi_f03_handler = {
    .driver = {
    .name = "rmi4_f03",
    },
    .func = 0x03,
    .probe = rmi_f03_probe,
    .config = rmi_f03_config,
    .attention = rmi_f03_attention,
    .remove = rmi_f03_remove,
    };
    MODULE_AUTHOR("Lyude Paul <thatslyude@gmail.com>");
    MODULE_DESCRIPTION("RMI F03 module");
    MODULE_LICENSE("GPL");
