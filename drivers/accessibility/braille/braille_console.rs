//! Automatically rewritten from C to Rust
//! Source: drivers/accessibility/braille/braille_console.c
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
// Minimalistic braille device kernel support.
//
// By default, shows console messages on the braille device.
// Pressing Insert switches to VC browsing.
//
// Copyright (C) Samuel Thibault <samuel.thibault@ens-lyon.org>
//

    MODULE_AUTHOR("samuel.thibault@ens-lyon.org");
    MODULE_DESCRIPTION("braille device");
//
// Braille device support part.
//
// Emit various sounds
    static bool sound;
    module_param(sound, bool, 0);
    MODULE_PARM_DESC(sound, "emit sounds");
#[no_mangle]
unsafe extern "C" fn beep(freq: c_uint) {
    static void beep(unsigned int freq)
    {
    if (sound)
    kd_mksound(freq, HZ/10);
    }
// mini console
pub const WIDTH: c_int = 40;

    static u16 console_buf[WIDTH];
    static int console_cursor;
// mini view of VC
    static int vc_x, vc_y, lastvc_x, lastvc_y;
// show console ? (or show VC)
    let mut console_show: static int = 1;
// pending newline ?
    let mut console_newline: static int = 1;
    let mut lastVC: static int = -1;
    static struct console *braille_co;
// Very VisioBraille-specific
#[no_mangle]
unsafe extern "C" fn braille_write(buf: *mut u16) {
    static void braille_write(u16 *buf)
    {
    static u16 lastwrite[WIDTH];
    unsigned char data[1 + 1 + 2*WIDTH + 2 + 1], csum = 0, *c;
    u16 out;
    int i;
    if (!braille_co)
    return;
    if (!memcmp(lastwrite, buf, WIDTH * sizeof(*buf)))
    return;
    memcpy(lastwrite, buf, WIDTH * sizeof(*buf));
pub const SOH: c_int = 1;
pub const STX: c_int = 2;
pub const ETX: c_int = 2;
pub const EOT: c_int = 4;
pub const ENQ: c_int = 5;
    data[0] = STX;
    data[1] = '>';
    csum ^= '>';
    c = &data[2];
    for (i = 0; i < WIDTH; i++) {
    out = buf[i];
    if (out >= 0x100)
    out = '?';
#[no_mangle]
pub unsafe extern "C" fn if(0x00: out ==) -> else {
    else if (out == 0x00)
    out = ' ';
    csum ^= out;
    if (out <= 0x05) {
// c++ = SOH;
    out |= 0x40;
    }
// c++ = out;
    }
    if (csum <= 0x05) {
// c++ = SOH;
    csum |= 0x40;
    }
// c++ = csum;
// c++ = ETX;
    braille_co.write(braille_co, data, c - data);
    }
// Follow the VC cursor
#[no_mangle]
unsafe extern "C" fn vc_follow_cursor(vc: *mut vc_data) {
    static void vc_follow_cursor(struct vc_data *vc)
    {
    vc_x = vc.state.x - (vc.state.x % WIDTH);
    vc_y = vc.state.y;
    lastvc_x = vc.state.x;
    lastvc_y = vc.state.y;
    }
// Maybe the VC cursor moved, if so follow it
#[no_mangle]
unsafe extern "C" fn vc_maybe_cursor_moved(vc: *mut vc_data) {
    static void vc_maybe_cursor_moved(struct vc_data *vc)
    {
    if (vc.state.x != lastvc_x || vc.state.y != lastvc_y)
    vc_follow_cursor(vc);
    }
// Show portion of VC at vc_x, vc_y
#[no_mangle]
unsafe extern "C" fn vc_refresh(vc: *mut vc_data) {
    static void vc_refresh(struct vc_data *vc)
    {
    u16 buf[WIDTH];
    int i;
    for (i = 0; i < WIDTH; i++) {
    u16 glyph = screen_glyph(vc,
    2 * (vc_x + i) + vc_y * vc.vc_size_row);
    buf[i] = inverse_translate(vc, glyph, true);
    }
    braille_write(buf);
    }
//
// Link to keyboard
//
    static int keyboard_notifier_call(struct notifier_block *blk,
    unsigned long code, void *_param)
    {
    struct keyboard_notifier_param *param = _param;
    struct vc_data *vc = param.vc;
    let mut ret: c_int = NOTIFY_OK;
    if (!param.down)
    return ret;
    switch (code) {
    case KBD_KEYCODE:
    if (console_show) {
    if (param.value == BRAILLE_KEY) {
    console_show = 0;
    beep(880);
    vc_maybe_cursor_moved(vc);
    vc_refresh(vc);
    ret = NOTIFY_STOP;
    }
    } else {
    ret = NOTIFY_STOP;
    switch (param.value) {
    case KEY_INSERT:
    beep(440);
    console_show = 1;
    lastVC = -1;
    braille_write(console_buf);
    break;
    case KEY_LEFT:
    if (vc_x > 0) {
    vc_x -= WIDTH;
    if (vc_x < 0)
    vc_x = 0;
    } else if (vc_y >= 1) {
    beep(880);
    vc_y--;
    vc_x = vc.vc_cols-WIDTH;
    } else
    beep(220);
    break;
    case KEY_RIGHT:
    if (vc_x + WIDTH < vc.vc_cols) {
    vc_x += WIDTH;
    } else if (vc_y + 1 < vc.vc_rows) {
    beep(880);
    vc_y++;
    vc_x = 0;
    } else
    beep(220);
    break;
    case KEY_DOWN:
    if (vc_y + 1 < vc.vc_rows)
    vc_y++;
    else
    beep(220);
    break;
    case KEY_UP:
    if (vc_y >= 1)
    vc_y--;
    else
    beep(220);
    break;
    case KEY_HOME:
    vc_follow_cursor(vc);
    break;
    case KEY_PAGEUP:
    vc_x = 0;
    vc_y = 0;
    break;
    case KEY_PAGEDOWN:
    vc_x = 0;
    vc_y = vc.vc_rows-1;
    break;
    default:
    ret = NOTIFY_OK;
    break;
    }
    if (ret == NOTIFY_STOP)
    vc_refresh(vc);
    }
    break;
    case KBD_POST_KEYSYM:
    {
    let mut type: c_uchar = KTYP(param.value) - 0xf0;
    if (type == KT_SPEC) {
    let mut val: c_uchar = KVAL(param.value);
    let mut on_off: c_int = -1;
    switch (val) {
    case KVAL(K_CAPS):
    on_off = vt_get_leds(fg_console, VC_CAPSLOCK);
    break;
    case KVAL(K_NUM):
    on_off = vt_get_leds(fg_console, VC_NUMLOCK);
    break;
    case KVAL(K_HOLD):
    on_off = vt_get_leds(fg_console, VC_SCROLLOCK);
    break;
    }
    if (on_off == 1)
    beep(880);
#[no_mangle]
pub unsafe extern "C" fn if(0: on_off ==) -> else {
    else if (on_off == 0)
    beep(440);
    }
    }
    break;
    case KBD_UNBOUND_KEYCODE:
    case KBD_UNICODE:
    case KBD_KEYSYM:
// Unused
    break;
    }
    return ret;
    }
    static struct notifier_block keyboard_notifier_block = {
    .notifier_call = keyboard_notifier_call,
    };
    static int vt_notifier_call(struct notifier_block *blk,
    unsigned long code, void *_param)
    {
    struct vt_notifier_param *param = _param;
    struct vc_data *vc = param.vc;
    switch (code) {
    case VT_ALLOCATE:
    break;
    case VT_DEALLOCATE:
    break;
    case VT_WRITE:
    {
    let mut c: c_uchar = param.c;
    if (vc.vc_num != fg_console)
    break;
    switch (c) {
    case '\b':
    case 127:
    if (console_cursor > 0) {
    console_cursor--;
    console_buf[console_cursor] = ' ';
    }
    break;
    case '\n':
    case '\v':
    case '\f':
    case '\r':
    console_newline = 1;
    break;
    case '\t':
    c = ' ';
    fallthrough;
    default:
    if (c < 32)
// Ignore other control sequences
    break;
    if (console_newline) {
    memset(console_buf, 0, sizeof(console_buf));
    console_cursor = 0;
    console_newline = 0;
    }
    if (console_cursor == WIDTH)
    memmove(console_buf, &console_buf[1],
    (WIDTH-1) * sizeof(*console_buf));
    else
    console_cursor++;
    console_buf[console_cursor-1] = c;
    break;
    }
    if (console_show)
    braille_write(console_buf);
    else {
    vc_maybe_cursor_moved(vc);
    vc_refresh(vc);
    }
    break;
    }
    case VT_UPDATE:
// Maybe a VT switch, flush
    if (console_show) {
    if (vc.vc_num != lastVC) {
    lastVC = vc.vc_num;
    memset(console_buf, 0, sizeof(console_buf));
    console_cursor = 0;
    braille_write(console_buf);
    }
    } else {
    vc_maybe_cursor_moved(vc);
    vc_refresh(vc);
    }
    break;
    }
    return NOTIFY_OK;
    }
    static struct notifier_block vt_notifier_block = {
    .notifier_call = vt_notifier_call,
    };
//
// Called from printk.c when console=brl is given
//
    int braille_register_console(struct console *console, int index,
    char *console_options, char *braille_options)
    {
    int ret;
    if (!console_options)
// Only support VisioBraille for now
    console_options = "57600o8";
    if (braille_co)
    return -ENODEV;
    if (console.setup) {
    ret = console.setup(console, console_options);
    if (ret != 0)
    return ret;
    }
    console.flags |= CON_ENABLED;
    console.index = index;
    braille_co = console;
    register_keyboard_notifier(&keyboard_notifier_block);
    register_vt_notifier(&vt_notifier_block);
    return 1;
    }
#[no_mangle]
pub unsafe extern "C" fn braille_unregister_console(console: *mut console) -> c_int {
    int braille_unregister_console(struct console *console)
    {
    if (braille_co != console)
    return -EINVAL;
    unregister_keyboard_notifier(&keyboard_notifier_block);
    unregister_vt_notifier(&vt_notifier_block);
    braille_co = core::ptr::null_mut();
    return 1;
    }
