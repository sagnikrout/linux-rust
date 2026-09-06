//! Automatically rewritten from C to Rust
//! Source: drivers/input/serio/ps2mult.c
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
// TQC PS/2 Multiplexer driver
//
// Copyright (C) 2010 Dmitry Eremin-Solenikov
//

    MODULE_AUTHOR("Dmitry Eremin-Solenikov <dbaryshkov@gmail.com>");
    MODULE_DESCRIPTION("TQC PS/2 Multiplexer driver");
    MODULE_LICENSE("GPL");
pub const PS2MULT_KB_SELECTOR: c_uint = 0xA0;
pub const PS2MULT_MS_SELECTOR: c_uint = 0xA1;
pub const PS2MULT_ESCAPE: c_uint = 0x7D;
pub const PS2MULT_BSYNC: c_uint = 0x7E;
pub const PS2MULT_SESSION_START: c_uint = 0x55;
pub const PS2MULT_SESSION_END: c_uint = 0x56;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ps2mult_port {
    pub serio: *mut serio,
    pub sel: c_uchar,
    pub registered: bool,
}

pub const PS2MULT_NUM_PORTS: c_int = 2;
pub const PS2MULT_KBD_PORT: c_int = 0;
pub const PS2MULT_MOUSE_PORT: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ps2mult {
    pub mx_serio: *mut serio,
    pub ports: [ps2mult_port; PS2MULT_NUM_PORTS],
    pub lock: spinlock_t,
    pub in_port: *mut ps2mult_port,
    pub out_port: *mut ps2mult_port,
    pub escape: bool,
}

// First MUST come PS2MULT_NUM_PORTS selectors
    static const unsigned char ps2mult_controls[] = {
    PS2MULT_KB_SELECTOR, PS2MULT_MS_SELECTOR,
    PS2MULT_ESCAPE, PS2MULT_BSYNC,
    PS2MULT_SESSION_START, PS2MULT_SESSION_END,
    };
    static const struct serio_device_id ps2mult_serio_ids[] = {
    {
    .type	= SERIO_RS232,
    .proto	= SERIO_PS2MULT,
    .id	= SERIO_ANY,
    .extra	= SERIO_ANY,
    },
    { 0 }
    };
    MODULE_DEVICE_TABLE(serio, ps2mult_serio_ids);
#[no_mangle]
unsafe extern "C" fn ps2mult_select_port(psm: *mut ps2mult, port: *mut ps2mult_port) {
    static void ps2mult_select_port(struct ps2mult *psm, struct ps2mult_port *port)
    {
    struct serio *mx_serio = psm.mx_serio;
    serio_write(mx_serio, port.sel);
    psm.out_port = port;
    dev_dbg(&mx_serio.dev, "switched to sel %02x\n", port.sel);
    }
#[no_mangle]
unsafe extern "C" fn ps2mult_serio_write(serio: *mut serio, data: c_uchar) -> c_int {
    static int ps2mult_serio_write(struct serio *serio, unsigned char data)
    {
    struct serio *mx_port = serio.parent;
    struct ps2mult *psm = serio_get_drvdata(mx_port);
    struct ps2mult_port *port = serio.port_data;
    bool need_escape;
    guard(spinlock_irqsave)(&psm.lock);
    if (psm.out_port != port)
    ps2mult_select_port(psm, port);
    need_escape = memchr(ps2mult_controls, data, sizeof(ps2mult_controls));
    dev_dbg(&serio.dev,
    "write: %s%02x\n", need_escape ? "ESC " : "", data);
    if (need_escape)
    serio_write(mx_port, PS2MULT_ESCAPE);
    serio_write(mx_port, data);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ps2mult_serio_start(serio: *mut serio) -> c_int {
    static int ps2mult_serio_start(struct serio *serio)
    {
    struct ps2mult *psm = serio_get_drvdata(serio.parent);
    struct ps2mult_port *port = serio.port_data;
    guard(spinlock_irqsave)(&psm.lock);
    port.registered = true;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ps2mult_serio_stop(serio: *mut serio) {
    static void ps2mult_serio_stop(struct serio *serio)
    {
    struct ps2mult *psm = serio_get_drvdata(serio.parent);
    struct ps2mult_port *port = serio.port_data;
    guard(spinlock_irqsave)(&psm.lock);
    port.registered = false;
    }
#[no_mangle]
unsafe extern "C" fn ps2mult_create_port(psm: *mut ps2mult, i: c_int) -> c_int {
    static int ps2mult_create_port(struct ps2mult *psm, int i)
    {
    struct serio *mx_serio = psm.mx_serio;
    struct serio *serio;
    serio = kzalloc_obj(*serio);
    if (!serio)
    return -ENOMEM;
    strscpy(serio.name, "TQC PS/2 Multiplexer", sizeof(serio.name));
    snprintf(serio.phys, sizeof(serio.phys),
    "%s/port%d", mx_serio.phys, i);
    serio.id.type = SERIO_8042;
    serio.write = ps2mult_serio_write;
    serio.start = ps2mult_serio_start;
    serio.stop = ps2mult_serio_stop;
    serio.parent = psm.mx_serio;
    serio.port_data = &psm.ports[i];
    psm.ports[i].serio = serio;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ps2mult_reset(psm: *mut ps2mult) {
    static void ps2mult_reset(struct ps2mult *psm)
    {
    guard(spinlock_irqsave)(&psm.lock);
    serio_write(psm.mx_serio, PS2MULT_SESSION_END);
    serio_write(psm.mx_serio, PS2MULT_SESSION_START);
    ps2mult_select_port(psm, &psm.ports[PS2MULT_KBD_PORT]);
    }
#[no_mangle]
unsafe extern "C" fn ps2mult_connect(serio: *mut serio, drv: *mut serio_driver) -> c_int {
    static int ps2mult_connect(struct serio *serio, struct serio_driver *drv)
    {
    struct ps2mult *psm;
    int i;
    int error;
    if (!serio.write)
    return -EINVAL;
    psm = kzalloc_obj(*psm);
    if (!psm)
    return -ENOMEM;
    spin_lock_init(&psm.lock);
    psm.mx_serio = serio;
    for (i = 0; i < PS2MULT_NUM_PORTS; i++) {
    psm.ports[i].sel = ps2mult_controls[i];
    error = ps2mult_create_port(psm, i);
    if (error)
    goto err_out;
    }
    psm.in_port = psm.out_port = &psm.ports[PS2MULT_KBD_PORT];
    serio_set_drvdata(serio, psm);
    error = serio_open(serio, drv);
    if (error)
    goto err_out;
    ps2mult_reset(psm);
    for (i = 0; i <  PS2MULT_NUM_PORTS; i++) {
    struct serio *s = psm.ports[i].serio;
    dev_info(&serio.dev, "%s port at %s\n", s.name, serio.phys);
    serio_register_port(s);
    }
    return 0;
    err_out:
    while (--i >= 0)
    kfree(psm.ports[i].serio);
    kfree(psm);
    return error;
    }
#[no_mangle]
unsafe extern "C" fn ps2mult_disconnect(serio: *mut serio) {
    static void ps2mult_disconnect(struct serio *serio)
    {
    struct ps2mult *psm = serio_get_drvdata(serio);
// Note that serio core already take care of children ports
    serio_write(serio, PS2MULT_SESSION_END);
    serio_close(serio);
    kfree(psm);
    serio_set_drvdata(serio, core::ptr::null_mut());
    }
#[no_mangle]
unsafe extern "C" fn ps2mult_reconnect(serio: *mut serio) -> c_int {
    static int ps2mult_reconnect(struct serio *serio)
    {
    struct ps2mult *psm = serio_get_drvdata(serio);
    ps2mult_reset(psm);
    return 0;
    }
    static irqreturn_t ps2mult_interrupt(struct serio *serio,
    unsigned char data, unsigned int dfl)
    {
    struct ps2mult *psm = serio_get_drvdata(serio);
    struct ps2mult_port *in_port;
    dev_dbg(&serio.dev, "Received %02x flags %02x\n", data, dfl);
    guard(spinlock_irqsave)(&psm.lock);
    if (psm.escape) {
    psm.escape = false;
    in_port = psm.in_port;
    if (in_port.registered)
    serio_interrupt(in_port.serio, data, dfl);
    goto out;
    }
    switch (data) {
    case PS2MULT_ESCAPE:
    dev_dbg(&serio.dev, "ESCAPE\n");
    psm.escape = true;
    break;
    case PS2MULT_BSYNC:
    dev_dbg(&serio.dev, "BSYNC\n");
    psm.in_port = psm.out_port;
    break;
    case PS2MULT_SESSION_START:
    dev_dbg(&serio.dev, "SS\n");
    break;
    case PS2MULT_SESSION_END:
    dev_dbg(&serio.dev, "SE\n");
    break;
    case PS2MULT_KB_SELECTOR:
    dev_dbg(&serio.dev, "KB\n");
    psm.in_port = &psm.ports[PS2MULT_KBD_PORT];
    break;
    case PS2MULT_MS_SELECTOR:
    dev_dbg(&serio.dev, "MS\n");
    psm.in_port = &psm.ports[PS2MULT_MOUSE_PORT];
    break;
    default:
    in_port = psm.in_port;
    if (in_port.registered)
    serio_interrupt(in_port.serio, data, dfl);
    break;
    }
    out:
    return IRQ_HANDLED;
    }
    static struct serio_driver ps2mult_drv = {
    .driver		= {
    .name	= "ps2mult",
    },
    .description	= "TQC PS/2 Multiplexer driver",
    .id_table	= ps2mult_serio_ids,
    .interrupt	= ps2mult_interrupt,
    .connect	= ps2mult_connect,
    .disconnect	= ps2mult_disconnect,
    .reconnect	= ps2mult_reconnect,
    };
    module_serio_driver(ps2mult_drv);
