//! Automatically rewritten from C to Rust
//! Source: scripts/kconfig/lxdialog/yesno.c
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
// yesno.c -- implements the yes/no box
//
// ORIGINAL AUTHOR: Savio Lam (lam836@cs.cuhk.hk)
// MODIFIED FOR LINUX KERNEL CONFIG BY: William Roadcap (roadcap@cfw.com)
//

//
// Display termination buttons
//
#[no_mangle]
unsafe extern "C" fn print_buttons(dialog: *mut *mut WINDOW, height: c_int, width: c_int, selected: c_int) {
    static void print_buttons(WINDOW * dialog, int height, int width, int selected)
    {
    let mut x: c_int = width / 2 - 10;
    let mut y: c_int = height - 2;
    print_button(dialog, " Yes ", y, x, selected == 0);
    print_button(dialog, "  No  ", y, x + 13, selected == 1);
    wmove(dialog, y, x + 1 + 13 * selected);
    wrefresh(dialog);
    }
//
// Display a dialog box with two buttons - Yes and No
//
#[no_mangle]
pub unsafe extern "C" fn dialog_yesno(title: *const c_char, prompt: *const c_char, height: c_int, width: c_int) -> c_int {
    int dialog_yesno(const char *title, const char *prompt, int height, int width)
    {
    int i, x, y, key = 0, button = 0;
    WINDOW *dialog;
    do_resize:
    if (getmaxy(stdscr) < (height + YESNO_HEIGHT_MIN))
    return -ERRDISPLAYTOOSMALL;
    if (getmaxx(stdscr) < (width + YESNO_WIDTH_MIN))
    return -ERRDISPLAYTOOSMALL;
// center dialog box on screen
    x = (getmaxx(stdscr) - width) / 2;
    y = (getmaxy(stdscr) - height) / 2;
    draw_shadow(stdscr, y, x, height, width);
    dialog = newwin(height, width, y, x);
    keypad(dialog, TRUE);
    draw_box(dialog, 0, 0, height, width,
    dlg.dialog.atr, dlg.border.atr);
    wattrset(dialog, dlg.border.atr);
    mvwaddch(dialog, height - 3, 0, ACS_LTEE);
    for (i = 0; i < width - 2; i++)
    waddch(dialog, ACS_HLINE);
    wattrset(dialog, dlg.dialog.atr);
    waddch(dialog, ACS_RTEE);
    print_title(dialog, title, width);
    wattrset(dialog, dlg.dialog.atr);
    print_autowrap(dialog, prompt, width - 2, 1, 3);
    print_buttons(dialog, height, width, 0);
    while (key != KEY_ESC) {
    key = wgetch(dialog);
    switch (key) {
    case 'Y':
    case 'y':
    delwin(dialog);
    return 0;
    case 'N':
    case 'n':
    delwin(dialog);
    return 1;
    case TAB:
    case KEY_LEFT:
    case KEY_RIGHT:
    button = ((key == KEY_LEFT ? --button : ++button) < 0) ? 1 : (button > 1 ? 0 : button);
    print_buttons(dialog, height, width, button);
    wrefresh(dialog);
    break;
    case ' ':
    case '\n':
    delwin(dialog);
    return button;
    case KEY_ESC:
    key = on_key_esc(dialog);
    break;
    case KEY_RESIZE:
    delwin(dialog);
    on_key_resize();
    goto do_resize;
    }
    }
    delwin(dialog);
    return key;		/* ESC pressed */
    }
