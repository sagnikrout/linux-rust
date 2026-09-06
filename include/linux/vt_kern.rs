//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/vt_kern.h
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
// this really is an extension of the vc_cons structure in console.c, but
// with information needed by the vt package
//

extern "C" {
    pub fn kd_mksound(hz: c_uint, ticks: c_uint);
}
extern "C" {
    pub fn kbd_rate(rep: *mut kbd_repeat) -> c_int;
}
// console.c
extern "C" {
    pub fn vc_allocate(console: c_uint) -> c_int;
}
extern "C" {
    pub fn vc_cons_allocated(console: c_uint) -> c_int;
}
extern "C" {
    pub fn reset_palette(vc: *mut vc_data);
}
extern "C" {
    pub fn do_blank_screen(entering_gfx: c_int);
}
extern "C" {
    pub fn do_unblank_screen(leaving_gfx: c_int);
}
extern "C" {
    pub fn poke_blanked_console();
}
extern "C" {
    pub fn con_font_op(vc: *mut vc_data, op: *mut console_font_op) -> c_int;
}
extern "C" {
    pub fn con_set_cmap(cmap: *mut unsigned char __user) -> c_int;
}
extern "C" {
    pub fn con_get_cmap(cmap: *mut unsigned char __user) -> c_int;
}
extern "C" {
    pub fn scrollback(vc: *mut vc_data);
}
extern "C" {
    pub fn scrollfront(vc: *mut vc_data, lines: c_int);
}
extern "C" {
    pub fn clear_buffer_attributes(vc: *mut vc_data);
}
extern "C" {
    pub fn update_region(vc: *mut vc_data, start: c_ulong, count: c_int);
}
extern "C" {
    pub fn redraw_screen(vc: *mut vc_data, is_switch: c_int);
}

extern "C" {
    pub fn __vc_resize(_arg: vc, _arg: cols, _arg: lines, _arg: false) -> return;
}
extern "C" {
    pub fn tioclinux(tty: *mut tty_struct, arg: c_ulong) -> c_int;
}

// consolemap.c
extern "C" {
    pub fn con_set_trans_old(table: *mut *mut unsigned char __user) -> c_int;
}
extern "C" {
    pub fn con_get_trans_old(table: *mut *mut unsigned char __user) -> c_int;
}
extern "C" {
    pub fn con_set_trans_new(table: *mut *mut unsigned short __user) -> c_int;
}
extern "C" {
    pub fn con_get_trans_new(table: *mut *mut unsigned short __user) -> c_int;
}
extern "C" {
    pub fn con_clear_unimap(vc: *mut vc_data) -> c_int;
}
extern "C" {
    pub fn con_set_unimap(vc: *mut vc_data, ct: c_ushort, list: *mut unipair __user) -> c_int;
}
extern "C" {
    pub fn con_get_unimap(vc: *mut vc_data, ct: c_ushort, uct: *mut ushort __user, list: *mut unipair __user) -> c_int;
}
extern "C" {
    pub fn con_set_default_unimap(vc: *mut vc_data) -> c_int;
}
extern "C" {
    pub fn con_free_unimap(vc: *mut vc_data);
}
extern "C" {
    pub fn con_copy_unimap(dst_vc: *mut vc_data, src_vc: *mut vc_data) -> c_int;
}

// vt.c
extern "C" {
    pub fn vt_event_post(event: c_uint, old: c_uint, new: c_uint);
}
extern "C" {
    pub fn vt_waitactive(n: c_int) -> c_int;
}
extern "C" {
    pub fn change_console(new_vc: *mut vc_data);
}
extern "C" {
    pub fn reset_vc(vc: *mut vc_data);
}
extern "C" {
    pub fn vty_init(console_fops: *const file_operations) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vt_spawn_console {
    pub lock: spinlock_t,
    pub pid: *mut pid,
    pub sig: c_int,
}

extern "C" {
    pub fn vt_move_to_console(vt: c_uint, alloc: c_int) -> c_int;
}
// Interfaces for VC notification of character events (for accessibility etc)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vt_notifier_param {
    pub /: *mut *mut *mut vc_data vc; / VC on which the update happened,
    pub /: *mut *mut unsigned int c; / Printed char,
}

extern "C" {
    pub fn register_vt_notifier(nb: *mut notifier_block) -> c_int;
}
extern "C" {
    pub fn unregister_vt_notifier(nb: *mut notifier_block) -> c_int;
}
extern "C" {
    pub fn hide_boot_cursor(hide: bool);
}
// keyboard  provided interfaces
extern "C" {
    pub fn vt_do_diacrit(cmd: c_uint, up: *mut void __user, eperm: c_int) -> c_int;
}
extern "C" {
    pub fn vt_do_kdskbmode(console: c_uint, arg: c_uint) -> c_int;
}
extern "C" {
    pub fn vt_do_kdskbmeta(console: c_uint, arg: c_uint) -> c_int;
}
extern "C" {
    pub fn vt_do_kdgkb_ioctl(cmd: c_int, user_kdgkb: *mut kbsentry __user, perm: c_int) -> c_int;
}
extern "C" {
    pub fn vt_do_kdskled(console: c_uint, cmd: c_int, arg: c_ulong, perm: c_int) -> c_int;
}
extern "C" {
    pub fn vt_do_kdgkbmode(console: c_uint) -> c_int;
}
extern "C" {
    pub fn vt_do_kdgkbmeta(console: c_uint) -> c_int;
}
extern "C" {
    pub fn vt_reset_unicode(console: c_uint);
}
extern "C" {
    pub fn vt_get_shift_state() -> c_int;
}
extern "C" {
    pub fn vt_reset_keyboard(console: c_uint);
}
extern "C" {
    pub fn vt_get_leds(console: c_uint, flag: c_int) -> c_int;
}
extern "C" {
    pub fn vt_get_kbd_mode_bit(console: c_uint, bit: c_int) -> c_int;
}
extern "C" {
    pub fn vt_set_kbd_mode_bit(console: c_uint, bit: c_int);
}
extern "C" {
    pub fn vt_clr_kbd_mode_bit(console: c_uint, bit: c_int);
}
extern "C" {
    pub fn vt_set_led_state(console: c_uint, leds: c_int);
}
extern "C" {
    pub fn vt_kbd_con_start(console: c_uint);
}
extern "C" {
    pub fn vt_kbd_con_stop(console: c_uint);
}
