//! Automatically rewritten from C to Rust
//! Source: drivers/input/gameport/lightning.c
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
// Copyright (c) 1998-2001 Vojtech Pavlik
//
// PDPI Lightning 4 gamecard driver for Linux.
//

pub const L4_PORT: c_uint = 0x201;
pub const L4_SELECT_ANALOG: c_uint = 0xa4;
pub const L4_SELECT_DIGITAL: c_uint = 0xa5;
pub const L4_SELECT_SECONDARY: c_uint = 0xa6;
pub const L4_CMD_ID: c_uint = 0x80;
pub const L4_CMD_GETCAL: c_uint = 0x92;
pub const L4_CMD_SETCAL: c_uint = 0x93;
pub const L4_ID: c_uint = 0x04;
pub const L4_BUSY: c_uint = 0x01;

    MODULE_AUTHOR("Vojtech Pavlik <vojtech@ucw.cz>");
    MODULE_DESCRIPTION("PDPI Lightning 4 gamecard driver");
    MODULE_LICENSE("GPL");
#[repr(C)]
#[derive(Copy, Clone)]
pub struct l4 {
    pub gameport: *mut gameport,
    pub port: c_uchar,
}

    static struct l4 l4_ports[8];
//
// l4_wait_ready() waits for the L4 to become ready.
//
#[no_mangle]
unsafe extern "C" fn l4_wait_ready() -> c_int {
    static int l4_wait_ready(void)
    {
    let mut t: c_uint = L4_TIMEOUT;
    while ((inb(L4_PORT) & L4_BUSY) && t > 0) t--;
    return -(t <= 0);
    }
//
// l4_cooked_read() reads data from the Lightning 4.
//
#[no_mangle]
unsafe extern "C" fn l4_cooked_read(gameport: *mut gameport, axes: *mut c_int, buttons: *mut c_int) -> c_int {
    static int l4_cooked_read(struct gameport *gameport, int *axes, int *buttons)
    {
    struct l4 *l4 = gameport.port_data;
    unsigned char status;
    int i, result = -1;
    outb(L4_SELECT_ANALOG, L4_PORT);
    outb(L4_SELECT_DIGITAL + (l4.port >> 2), L4_PORT);
    if (inb(L4_PORT) & L4_BUSY) goto fail;
    outb(l4.port & 3, L4_PORT);
    if (l4_wait_ready()) goto fail;
    status = inb(L4_PORT);
    for (i = 0; i < 4; i++)
    if (status & (1 << i)) {
    if (l4_wait_ready()) goto fail;
    axes[i] = inb(L4_PORT);
    if (axes[i] > 252) axes[i] = -1;
    }
    if (status & 0x10) {
    if (l4_wait_ready()) goto fail;
// buttons = inb(L4_PORT) & 0x0f;
    }
    result = 0;
    fail:	outb(L4_SELECT_ANALOG, L4_PORT);
    return result;
    }
#[no_mangle]
unsafe extern "C" fn l4_open(gameport: *mut gameport, mode: c_int) -> c_int {
    static int l4_open(struct gameport *gameport, int mode)
    {
    struct l4 *l4 = gameport.port_data;
    if (l4.port != 0 && mode != GAMEPORT_MODE_COOKED)
    return -1;
    outb(L4_SELECT_ANALOG, L4_PORT);
    return 0;
    }
//
// l4_getcal() reads the L4 with calibration values.
//
#[no_mangle]
unsafe extern "C" fn l4_getcal(port: c_int, cal: *mut c_int) -> c_int {
    static int l4_getcal(int port, int *cal)
    {
    int i, result = -1;
    outb(L4_SELECT_ANALOG, L4_PORT);
    outb(L4_SELECT_DIGITAL + (port >> 2), L4_PORT);
    if (inb(L4_PORT) & L4_BUSY)
    goto out;
    outb(L4_CMD_GETCAL, L4_PORT);
    if (l4_wait_ready())
    goto out;
    if (inb(L4_PORT) != L4_SELECT_DIGITAL + (port >> 2))
    goto out;
    if (l4_wait_ready())
    goto out;
    outb(port & 3, L4_PORT);
    for (i = 0; i < 4; i++) {
    if (l4_wait_ready())
    goto out;
    cal[i] = inb(L4_PORT);
    }
    result = 0;
    out:	outb(L4_SELECT_ANALOG, L4_PORT);
    return result;
    }
//
// l4_setcal() programs the L4 with calibration values.
//
#[no_mangle]
unsafe extern "C" fn l4_setcal(port: c_int, cal: *mut c_int) -> c_int {
    static int l4_setcal(int port, int *cal)
    {
    int i, result = -1;
    outb(L4_SELECT_ANALOG, L4_PORT);
    outb(L4_SELECT_DIGITAL + (port >> 2), L4_PORT);
    if (inb(L4_PORT) & L4_BUSY)
    goto out;
    outb(L4_CMD_SETCAL, L4_PORT);
    if (l4_wait_ready())
    goto out;
    if (inb(L4_PORT) != L4_SELECT_DIGITAL + (port >> 2))
    goto out;
    if (l4_wait_ready())
    goto out;
    outb(port & 3, L4_PORT);
    for (i = 0; i < 4; i++) {
    if (l4_wait_ready())
    goto out;
    outb(cal[i], L4_PORT);
    }
    result = 0;
    out:	outb(L4_SELECT_ANALOG, L4_PORT);
    return result;
    }
//
// l4_calibrate() calibrates the L4 for the attached device, so
// that the device's resistance fits into the L4's 8-bit range.
//
#[no_mangle]
unsafe extern "C" fn l4_calibrate(gameport: *mut gameport, axes: *mut c_int, max: *mut c_int) -> c_int {
    static int l4_calibrate(struct gameport *gameport, int *axes, int *max)
    {
    int i, t;
    int cal[4];
    struct l4 *l4 = gameport.port_data;
    if (l4_getcal(l4.port, cal))
    return -1;
    for (i = 0; i < 4; i++) {
    t = (max[i] * cal[i]) / 200;
    t = (t < 1) ? 1 : ((t > 255) ? 255 : t);
    axes[i] = (axes[i] < 0) ? -1 : (axes[i] * cal[i]) / t;
    axes[i] = (axes[i] > 252) ? 252 : axes[i];
    cal[i] = t;
    }
    if (l4_setcal(l4.port, cal))
    return -1;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn l4_create_ports(card_no: c_int) -> int __init {
    static int __init l4_create_ports(int card_no)
    {
    struct l4 *l4;
    struct gameport *port;
    int i, idx;
    for (i = 0; i < 4; i++) {
    idx = card_no * 4 + i;
    l4 = &l4_ports[idx];
    if (!(l4.gameport = port = gameport_allocate_port())) {
    printk(KERN_ERR "lightning: Memory allocation failed\n");
    while (--i >= 0) {
    gameport_free_port(l4.gameport);
    l4.gameport = core::ptr::null_mut();
    }
    return -ENOMEM;
    }
    l4.port = idx;
    port.port_data = l4;
    port.open = l4_open;
    port.cooked_read = l4_cooked_read;
    port.calibrate = l4_calibrate;
    gameport_set_name(port, "PDPI Lightning 4");
    gameport_set_phys(port, "isa%04x/gameport%d", L4_PORT, idx);
    if (idx == 0)
    port.io = L4_PORT;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn l4_add_card(card_no: c_int) -> int __init {
    static int __init l4_add_card(int card_no)
    {
    int cal[4] = { 255, 255, 255, 255 };
    int i, rev, result;
    struct l4 *l4;
    outb(L4_SELECT_ANALOG, L4_PORT);
    outb(L4_SELECT_DIGITAL + card_no, L4_PORT);
    if (inb(L4_PORT) & L4_BUSY)
    return -1;
    outb(L4_CMD_ID, L4_PORT);
    if (l4_wait_ready())
    return -1;
    if (inb(L4_PORT) != L4_SELECT_DIGITAL + card_no)
    return -1;
    if (l4_wait_ready())
    return -1;
    if (inb(L4_PORT) != L4_ID)
    return -1;
    if (l4_wait_ready())
    return -1;
    rev = inb(L4_PORT);
    if (!rev)
    return -1;
    result = l4_create_ports(card_no);
    if (result)
    return result;
    printk(KERN_INFO "gameport: PDPI Lightning 4 %s card v%d.%d at %#x\n",
    card_no ? "secondary" : "primary", rev >> 4, rev, L4_PORT);
    for (i = 0; i < 4; i++) {
    l4 = &l4_ports[card_no * 4 + i];
    if (rev > 0x28)		/* on 2.9+ the setcal command works correctly */
    l4_setcal(l4.port, cal);
    gameport_register_port(l4.gameport);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn l4_init() -> int __init {
    static int __init l4_init(void)
    {
    int i, cards = 0;
    if (!request_region(L4_PORT, 1, "lightning"))
    return -EBUSY;
    for (i = 0; i < 2; i++)
    if (l4_add_card(i) == 0)
    cards++;
    outb(L4_SELECT_ANALOG, L4_PORT);
    if (!cards) {
    release_region(L4_PORT, 1);
    return -ENODEV;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn l4_exit() -> void __exit {
    static void __exit l4_exit(void)
    {
    int i;
    int cal[4] = { 59, 59, 59, 59 };
    for (i = 0; i < 8; i++)
    if (l4_ports[i].gameport) {
    l4_setcal(l4_ports[i].port, cal);
    gameport_unregister_port(l4_ports[i].gameport);
    }
    outb(L4_SELECT_ANALOG, L4_PORT);
    release_region(L4_PORT, 1);
    }
    module_init(l4_init);
    module_exit(l4_exit);
