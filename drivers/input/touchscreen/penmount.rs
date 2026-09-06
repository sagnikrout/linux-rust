//! Automatically rewritten from C to Rust
//! Source: drivers/input/touchscreen/penmount.c
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
// Penmount serial touchscreen driver
//
// Copyright (c) 2006 Rick Koch <n1gp@hotmail.com>
// Copyright (c) 2011 John Sung <penmount.touch@gmail.com>
//
// Based on ELO driver (drivers/input/touchscreen/elo.c)
// Copyright (c) 2004 Vojtech Pavlik
//

    MODULE_AUTHOR("Rick Koch <n1gp@hotmail.com>");
    MODULE_AUTHOR("John Sung <penmount.touch@gmail.com>");
    MODULE_DESCRIPTION(DRIVER_DESC);
    MODULE_LICENSE("GPL");
//
// Definitions & global arrays.
//
pub const PM_MAX_LENGTH: c_int = 6;
pub const PM_MAX_MTSLOT: c_int = 16;
pub const PM_3000_MTSLOT: c_int = 2;
pub const PM_6250_MTSLOT: c_int = 12;
//
// Multi-touch slot
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt_slot {
    pub y: unsigned short x,,
    pub /: *mut *mut bool active; / is the touch valid?,
}

//
// Per-touchscreen data.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pm {
    pub dev: *mut input_dev,
    pub serio: *mut serio,
    pub idx: c_int,
    pub data: [c_uchar; PM_MAX_LENGTH],
    pub phys: [c_char; 32],
    pub packetsize: c_uchar,
    pub maxcontacts: c_uchar,
    pub slots: [mt_slot; PM_MAX_MTSLOT],
    pub ): *mut *mut void (parse_packet)(struct pm,
}

//
// pm_mtevent() sends mt events and also emulates pointer movement
//
#[no_mangle]
unsafe extern "C" fn pm_mtevent(pm: *mut pm, input: *mut input_dev) {
    static void pm_mtevent(struct pm *pm, struct input_dev *input)
    {
    int i;
    for (i = 0; i < pm.maxcontacts; ++i) {
    input_mt_slot(input, i);
    input_mt_report_slot_state(input, MT_TOOL_FINGER,
    pm.slots[i].active);
    if (pm.slots[i].active) {
    input_event(input, EV_ABS, ABS_MT_POSITION_X, pm.slots[i].x);
    input_event(input, EV_ABS, ABS_MT_POSITION_Y, pm.slots[i].y);
    }
    }
    input_mt_report_pointer_emulation(input, true);
    input_sync(input);
    }
//
// pm_checkpacket() checks if data packet is valid
//
#[no_mangle]
unsafe extern "C" fn pm_checkpacket(packet: *mut c_uchar) -> bool {
    static bool pm_checkpacket(unsigned char *packet)
    {
    let mut total: c_int = 0;
    int i;
    for (i = 0; i < 5; i++)
    total += packet[i];
    return packet[5] == (unsigned char)~(total & 0xff);
    }
#[no_mangle]
unsafe extern "C" fn pm_parse_9000(pm: *mut pm) {
    static void pm_parse_9000(struct pm *pm)
    {
    struct input_dev *dev = pm.dev;
    if ((pm.data[0] & 0x80) && pm.packetsize == ++pm.idx) {
    input_report_abs(dev, ABS_X, pm.data[1] * 128 + pm.data[2]);
    input_report_abs(dev, ABS_Y, pm.data[3] * 128 + pm.data[4]);
    input_report_key(dev, BTN_TOUCH, !!(pm.data[0] & 0x40));
    input_sync(dev);
    pm.idx = 0;
    }
    }
#[no_mangle]
unsafe extern "C" fn pm_parse_6000(pm: *mut pm) {
    static void pm_parse_6000(struct pm *pm)
    {
    struct input_dev *dev = pm.dev;
    if ((pm.data[0] & 0xbf) == 0x30 && pm.packetsize == ++pm.idx) {
    if (pm_checkpacket(pm.data)) {
    input_report_abs(dev, ABS_X,
    pm.data[2] * 256 + pm.data[1]);
    input_report_abs(dev, ABS_Y,
    pm.data[4] * 256 + pm.data[3]);
    input_report_key(dev, BTN_TOUCH, pm.data[0] & 0x40);
    input_sync(dev);
    }
    pm.idx = 0;
    }
    }
#[no_mangle]
unsafe extern "C" fn pm_parse_3000(pm: *mut pm) {
    static void pm_parse_3000(struct pm *pm)
    {
    struct input_dev *dev = pm.dev;
    if ((pm.data[0] & 0xce) == 0x40 && pm.packetsize == ++pm.idx) {
    if (pm_checkpacket(pm.data)) {
    let mut slotnum: c_int = pm.data[0] & 0x0f;
    pm.slots[slotnum].active = pm.data[0] & 0x30;
    pm.slots[slotnum].x = pm.data[2] * 256 + pm.data[1];
    pm.slots[slotnum].y = pm.data[4] * 256 + pm.data[3];
    pm_mtevent(pm, dev);
    }
    pm.idx = 0;
    }
    }
#[no_mangle]
unsafe extern "C" fn pm_parse_6250(pm: *mut pm) {
    static void pm_parse_6250(struct pm *pm)
    {
    struct input_dev *dev = pm.dev;
    if ((pm.data[0] & 0xb0) == 0x30 && pm.packetsize == ++pm.idx) {
    if (pm_checkpacket(pm.data)) {
    let mut slotnum: c_int = pm.data[0] & 0x0f;
    pm.slots[slotnum].active = pm.data[0] & 0x40;
    pm.slots[slotnum].x = pm.data[2] * 256 + pm.data[1];
    pm.slots[slotnum].y = pm.data[4] * 256 + pm.data[3];
    pm_mtevent(pm, dev);
    }
    pm.idx = 0;
    }
    }
    static irqreturn_t pm_interrupt(struct serio *serio,
    unsigned char data, unsigned int flags)
    {
    struct pm *pm = serio_get_drvdata(serio);
    pm.data[pm.idx] = data;
    pm.parse_packet(pm);
    return IRQ_HANDLED;
    }
//
// pm_disconnect() is the opposite of pm_connect()
//
#[no_mangle]
unsafe extern "C" fn pm_disconnect(serio: *mut serio) {
    static void pm_disconnect(struct serio *serio)
    {
    struct pm *pm = serio_get_drvdata(serio);
    serio_close(serio);
    input_unregister_device(pm.dev);
    kfree(pm);
    serio_set_drvdata(serio, core::ptr::null_mut());
    }
//
// pm_connect() is the routine that is called when someone adds a
// new serio device that supports PenMount protocol and registers it as
// an input device.
//
#[no_mangle]
unsafe extern "C" fn pm_connect(serio: *mut serio, drv: *mut serio_driver) -> c_int {
    static int pm_connect(struct serio *serio, struct serio_driver *drv)
    {
    struct pm *pm;
    struct input_dev *input_dev;
    int max_x, max_y;
    int err;
    pm = kzalloc_obj(*pm);
    input_dev = input_allocate_device();
    if (!pm || !input_dev) {
    err = -ENOMEM;
    goto fail1;
    }
    pm.serio = serio;
    pm.dev = input_dev;
    scnprintf(pm.phys, sizeof(pm.phys), "%s/input0", serio.phys);
    pm.maxcontacts = 1;
    input_dev.name = "PenMount Serial TouchScreen";
    input_dev.phys = pm.phys;
    input_dev.id.bustype = BUS_RS232;
    input_dev.id.vendor = SERIO_PENMOUNT;
    input_dev.id.product = 0;
    input_dev.id.version = 0x0100;
    input_dev.dev.parent = &serio.dev;
    input_dev.evbit[0] = BIT_MASK(EV_KEY) | BIT_MASK(EV_ABS);
    input_dev.keybit[BIT_WORD(BTN_TOUCH)] = BIT_MASK(BTN_TOUCH);
    switch (serio.id.id) {
    default:
    case 0:
    pm.packetsize = 5;
    pm.parse_packet = pm_parse_9000;
    input_dev.id.product = 0x9000;
    max_x = max_y = 0x3ff;
    break;
    case 1:
    pm.packetsize = 6;
    pm.parse_packet = pm_parse_6000;
    input_dev.id.product = 0x6000;
    max_x = max_y = 0x3ff;
    break;
    case 2:
    pm.packetsize = 6;
    pm.parse_packet = pm_parse_3000;
    input_dev.id.product = 0x3000;
    max_x = max_y = 0x7ff;
    pm.maxcontacts = PM_3000_MTSLOT;
    break;
    case 3:
    pm.packetsize = 6;
    pm.parse_packet = pm_parse_6250;
    input_dev.id.product = 0x6250;
    max_x = max_y = 0x3ff;
    pm.maxcontacts = PM_6250_MTSLOT;
    break;
    }
    input_set_abs_params(pm.dev, ABS_X, 0, max_x, 0, 0);
    input_set_abs_params(pm.dev, ABS_Y, 0, max_y, 0, 0);
    if (pm.maxcontacts > 1) {
    input_mt_init_slots(pm.dev, pm.maxcontacts, 0);
    input_set_abs_params(pm.dev,
    ABS_MT_POSITION_X, 0, max_x, 0, 0);
    input_set_abs_params(pm.dev,
    ABS_MT_POSITION_Y, 0, max_y, 0, 0);
    }
    serio_set_drvdata(serio, pm);
    err = serio_open(serio, drv);
    if (err)
    goto fail2;
    err = input_register_device(pm.dev);
    if (err)
    goto fail3;
    return 0;
    fail3:	serio_close(serio);
    fail2:	serio_set_drvdata(serio, core::ptr::null_mut());
    fail1:	input_free_device(input_dev);
    kfree(pm);
    return err;
    }
//
// The serio driver structure.
//
    static const struct serio_device_id pm_serio_ids[] = {
    {
    .type	= SERIO_RS232,
    .proto	= SERIO_PENMOUNT,
    .id	= SERIO_ANY,
    .extra	= SERIO_ANY,
    },
    { 0 }
    };
    MODULE_DEVICE_TABLE(serio, pm_serio_ids);
    static struct serio_driver pm_drv = {
    .driver		= {
    .name	= "serio-penmount",
    },
    .description	= DRIVER_DESC,
    .id_table	= pm_serio_ids,
    .interrupt	= pm_interrupt,
    .connect	= pm_connect,
    .disconnect	= pm_disconnect,
    };
    module_serio_driver(pm_drv);
