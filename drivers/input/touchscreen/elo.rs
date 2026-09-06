//! Automatically rewritten from C to Rust
//! Source: drivers/input/touchscreen/elo.c
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
// Elo serial touchscreen driver
//
// Copyright (c) 2004 Vojtech Pavlik
//
// This driver can handle serial Elo touchscreens using either the Elo standard
// 'E271-2210' 10-byte protocol, Elo legacy 'E281A-4002' 6-byte protocol, Elo
// legacy 'E271-140' 4-byte protocol and Elo legacy 'E261-280' 3-byte protocol.
//

    MODULE_AUTHOR("Vojtech Pavlik <vojtech@ucw.cz>");
    MODULE_DESCRIPTION(DRIVER_DESC);
    MODULE_LICENSE("GPL");
//
// Definitions & global arrays.
//
pub const ELO_MAX_LENGTH: c_int = 10;
pub const ELO10_PACKET_LEN: c_int = 8;
pub const ELO10_TOUCH: c_uint = 0x03;
pub const ELO10_PRESSURE: c_uint = 0x80;

//
// Per-touchscreen data.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct elo {
    pub dev: *mut input_dev,
    pub serio: *mut serio,
    pub cmd_mutex: mutex,
    pub cmd_done: completion,
    pub id: c_int,
    pub idx: c_int,
    pub expected_packet: c_uchar,
    pub csum: c_uchar,
    pub data: [c_uchar; ELO_MAX_LENGTH],
    pub response: [c_uchar; ELO10_PACKET_LEN],
    pub phys: [c_char; 32],
}

#[no_mangle]
unsafe extern "C" fn elo_process_data_10(elo: *mut elo, data: c_uchar) {
    static void elo_process_data_10(struct elo *elo, unsigned char data)
    {
    struct input_dev *dev = elo.dev;
    elo.data[elo.idx] = data;
    switch (elo.idx++) {
    case 0:
    elo.csum = 0xaa;
    if (data != ELO10_LEAD_BYTE) {
    dev_dbg(&elo.serio.dev,
    "unsynchronized data: 0x%02x\n", data);
    elo.idx = 0;
    }
    break;
    case 9:
    elo.idx = 0;
    if (data != elo.csum) {
    dev_dbg(&elo.serio.dev,
    "bad checksum: 0x%02x, expected 0x%02x\n",
    data, elo.csum);
    break;
    }
    if (elo.data[1] != elo.expected_packet) {
    if (elo.data[1] != ELO10_TOUCH_PACKET)
    dev_dbg(&elo.serio.dev,
    "unexpected packet: 0x%02x\n",
    elo.data[1]);
    break;
    }
    if (likely(elo.data[1] == ELO10_TOUCH_PACKET)) {
    input_report_abs(dev, ABS_X, (elo.data[4] << 8) | elo.data[3]);
    input_report_abs(dev, ABS_Y, (elo.data[6] << 8) | elo.data[5]);
    if (elo.data[2] & ELO10_PRESSURE)
    input_report_abs(dev, ABS_PRESSURE,
    (elo.data[8] << 8) | elo.data[7]);
    input_report_key(dev, BTN_TOUCH, elo.data[2] & ELO10_TOUCH);
    input_sync(dev);
    } else if (elo.data[1] == ELO10_ACK_PACKET) {
    if (elo.data[2] == '0')
    elo.expected_packet = ELO10_TOUCH_PACKET;
    complete(&elo.cmd_done);
    } else {
    memcpy(elo.response, &elo.data[1], ELO10_PACKET_LEN);
    elo.expected_packet = ELO10_ACK_PACKET;
    }
    break;
    }
    elo.csum += data;
    }
#[no_mangle]
unsafe extern "C" fn elo_process_data_6(elo: *mut elo, data: c_uchar) {
    static void elo_process_data_6(struct elo *elo, unsigned char data)
    {
    struct input_dev *dev = elo.dev;
    elo.data[elo.idx] = data;
    switch (elo.idx++) {
    case 0:
    if ((data & 0xc0) != 0xc0)
    elo.idx = 0;
    break;
    case 1:
    if ((data & 0xc0) != 0x80)
    elo.idx = 0;
    break;
    case 2:
    if ((data & 0xc0) != 0x40)
    elo.idx = 0;
    break;
    case 3:
    if (data & 0xc0) {
    elo.idx = 0;
    break;
    }
    input_report_abs(dev, ABS_X, ((elo.data[0] & 0x3f) << 6) | (elo.data[1] & 0x3f));
    input_report_abs(dev, ABS_Y, ((elo.data[2] & 0x3f) << 6) | (elo.data[3] & 0x3f));
    if (elo.id == 2) {
    input_report_key(dev, BTN_TOUCH, 1);
    input_sync(dev);
    elo.idx = 0;
    }
    break;
    case 4:
    if (data) {
    input_sync(dev);
    elo.idx = 0;
    }
    break;
    case 5:
    if ((data & 0xf0) == 0) {
    input_report_abs(dev, ABS_PRESSURE, elo.data[5]);
    input_report_key(dev, BTN_TOUCH, !!elo.data[5]);
    }
    input_sync(dev);
    elo.idx = 0;
    break;
    }
    }
#[no_mangle]
unsafe extern "C" fn elo_process_data_3(elo: *mut elo, data: c_uchar) {
    static void elo_process_data_3(struct elo *elo, unsigned char data)
    {
    struct input_dev *dev = elo.dev;
    elo.data[elo.idx] = data;
    switch (elo.idx++) {
    case 0:
    if ((data & 0x7f) != 0x01)
    elo.idx = 0;
    break;
    case 2:
    input_report_key(dev, BTN_TOUCH, !(elo.data[1] & 0x80));
    input_report_abs(dev, ABS_X, elo.data[1]);
    input_report_abs(dev, ABS_Y, elo.data[2]);
    input_sync(dev);
    elo.idx = 0;
    break;
    }
    }
    static irqreturn_t elo_interrupt(struct serio *serio,
    unsigned char data, unsigned int flags)
    {
    struct elo *elo = serio_get_drvdata(serio);
    switch (elo.id) {
    case 0:
    elo_process_data_10(elo, data);
    break;
    case 1:
    case 2:
    elo_process_data_6(elo, data);
    break;
    case 3:
    elo_process_data_3(elo, data);
    break;
    }
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn elo_command_10(elo: *mut elo, packet: *mut c_uchar) -> c_int {
    static int elo_command_10(struct elo *elo, unsigned char *packet)
    {
    int error;
    int i;
    let mut csum: c_uchar = 0xaa + ELO10_LEAD_BYTE;
    guard(mutex)(&elo.cmd_mutex);
    scoped_guard(serio_pause_rx, elo.serio) {
    elo.expected_packet = toupper(packet[0]);
    init_completion(&elo.cmd_done);
    }
    error = serio_write(elo.serio, ELO10_LEAD_BYTE);
    if (error)
    return error;
    for (i = 0; i < ELO10_PACKET_LEN; i++) {
    csum += packet[i];
    error = serio_write(elo.serio, packet[i]);
    if (error)
    return error;
    }
    error = serio_write(elo.serio, csum);
    if (error)
    return error;
    wait_for_completion_timeout(&elo.cmd_done, HZ);
    if (elo.expected_packet != ELO10_TOUCH_PACKET)
    return -EIO;
// We are back in reporting mode, the command was ACKed
    memcpy(packet, elo.response, ELO10_PACKET_LEN);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn elo_setup_10(elo: *mut elo) -> c_int {
    static int elo_setup_10(struct elo *elo)
    {
    static const char * const elo_types[] = { "Accu", "Dura", "Intelli", "Carroll" };
    struct input_dev *dev = elo.dev;
    unsigned char packet[ELO10_PACKET_LEN] = { ELO10_ID_CMD };
    if (elo_command_10(elo, packet))
    return -1;
    dev.id.version = (packet[5] << 8) | packet[4];
    input_set_abs_params(dev, ABS_X, 96, 4000, 0, 0);
    input_set_abs_params(dev, ABS_Y, 96, 4000, 0, 0);
    if (packet[3] & ELO10_PRESSURE)
    input_set_abs_params(dev, ABS_PRESSURE, 0, 255, 0, 0);
    dev_info(&elo.serio.dev,
    "%sTouch touchscreen, fw: %02x.%02x, features: 0x%02x, controller: 0x%02x\n",
    elo_types[(packet[1] - '0') & 0x03],
    packet[5], packet[4], packet[3], packet[7]);
    return 0;
    }
//
// elo_disconnect() is the opposite of elo_connect()
//
#[no_mangle]
unsafe extern "C" fn elo_disconnect(serio: *mut serio) {
    static void elo_disconnect(struct serio *serio)
    {
    struct elo *elo = serio_get_drvdata(serio);
    input_get_device(elo.dev);
    input_unregister_device(elo.dev);
    serio_close(serio);
    serio_set_drvdata(serio, core::ptr::null_mut());
    input_put_device(elo.dev);
    kfree(elo);
    }
//
// elo_connect() is the routine that is called when someone adds a
// new serio device that supports Gunze protocol and registers it as
// an input device.
//
#[no_mangle]
unsafe extern "C" fn elo_connect(serio: *mut serio, drv: *mut serio_driver) -> c_int {
    static int elo_connect(struct serio *serio, struct serio_driver *drv)
    {
    struct elo *elo;
    struct input_dev *input_dev;
    int err;
    elo = kzalloc_obj(*elo);
    input_dev = input_allocate_device();
    if (!elo || !input_dev) {
    err = -ENOMEM;
    goto fail1;
    }
    elo.serio = serio;
    elo.id = serio.id.id;
    elo.dev = input_dev;
    elo.expected_packet = ELO10_TOUCH_PACKET;
    mutex_init(&elo.cmd_mutex);
    init_completion(&elo.cmd_done);
    scnprintf(elo.phys, sizeof(elo.phys), "%s/input0", serio.phys);
    input_dev.name = "Elo Serial TouchScreen";
    input_dev.phys = elo.phys;
    input_dev.id.bustype = BUS_RS232;
    input_dev.id.vendor = SERIO_ELO;
    input_dev.id.product = elo.id;
    input_dev.id.version = 0x0100;
    input_dev.dev.parent = &serio.dev;
    input_dev.evbit[0] = BIT_MASK(EV_KEY) | BIT_MASK(EV_ABS);
    input_dev.keybit[BIT_WORD(BTN_TOUCH)] = BIT_MASK(BTN_TOUCH);
    serio_set_drvdata(serio, elo);
    err = serio_open(serio, drv);
    if (err)
    goto fail2;
    switch (elo.id) {
    case 0: /* 10-byte protocol */
    if (elo_setup_10(elo)) {
    err = -EIO;
    goto fail3;
    }
    break;
    case 1: /* 6-byte protocol */
    input_set_abs_params(input_dev, ABS_PRESSURE, 0, 15, 0, 0);
    fallthrough;
    case 2: /* 4-byte protocol */
    input_set_abs_params(input_dev, ABS_X, 96, 4000, 0, 0);
    input_set_abs_params(input_dev, ABS_Y, 96, 4000, 0, 0);
    break;
    case 3: /* 3-byte protocol */
    input_set_abs_params(input_dev, ABS_X, 0, 255, 0, 0);
    input_set_abs_params(input_dev, ABS_Y, 0, 255, 0, 0);
    break;
    }
    err = input_register_device(elo.dev);
    if (err)
    goto fail3;
    return 0;
    fail3: serio_close(serio);
    fail2:	serio_set_drvdata(serio, core::ptr::null_mut());
    fail1:	input_free_device(input_dev);
    kfree(elo);
    return err;
    }
//
// The serio driver structure.
//
    static const struct serio_device_id elo_serio_ids[] = {
    {
    .type	= SERIO_RS232,
    .proto	= SERIO_ELO,
    .id	= SERIO_ANY,
    .extra	= SERIO_ANY,
    },
    { 0 }
    };
    MODULE_DEVICE_TABLE(serio, elo_serio_ids);
    static struct serio_driver elo_drv = {
    .driver		= {
    .name	= "elo",
    },
    .description	= DRIVER_DESC,
    .id_table	= elo_serio_ids,
    .interrupt	= elo_interrupt,
    .connect	= elo_connect,
    .disconnect	= elo_disconnect,
    };
    module_serio_driver(elo_drv);
