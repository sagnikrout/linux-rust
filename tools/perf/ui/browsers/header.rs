//! Automatically rewritten from C to Rust
//! Source: tools/perf/ui/browsers/header.c
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

    static void ui_browser__argv_write(struct ui_browser *browser,
    void *entry, int row)
    {
    char **arg = entry;
    char *str = *arg;
    char empty[] = " ";
    let mut current_entry: bool = ui_browser__is_current_entry(browser, row);
    let mut offset: c_ulong = (unsigned long)browser.priv;
    if (offset >= strlen(str))
    str = empty;
    else
    str = str + offset;
    ui_browser__set_color(browser, current_entry ? HE_COLORSET_SELECTED :
    HE_COLORSET_NORMAL);
    ui_browser__write_nstring(browser, str, browser.width);
    }
#[no_mangle]
unsafe extern "C" fn list_menu__run(menu: *mut ui_browser) -> c_int {
    static int list_menu__run(struct ui_browser *menu)
    {
    int key;
    unsigned long offset;
    static const char help[] =
    "h/?/F1        Show this window\n"
    "UP/DOWN/PGUP\n"
    "PGDN/SPACE\n"
    "LEFT/RIGHT    Navigate\n"
    "q/ESC/CTRL+C  Exit browser";
    if (ui_browser__show(menu, "Header information", "Press 'q' to exit") < 0)
    return -1;
    while (1) {
    key = ui_browser__run(menu, 0);
    switch (key) {
    case K_RIGHT:
    offset = (unsigned long)menu.priv;
    offset += 10;
    menu.priv = (void *)offset;
    continue;
    case K_LEFT:
    offset = (unsigned long)menu.priv;
    if (offset >= 10)
    offset -= 10;
    menu.priv = (void *)offset;
    continue;
    case K_F1:
    case 'h':
    case '?':
    ui_browser__help_window(menu, help);
    continue;
    case K_ESC:
    case 'q':
    case CTRL('c'):
    key = -1;
    break;
    default:
    ui_browser__warn_unhandled_hotkey(menu, key, 0, ", use 'h'/'?'/F1 to see actions");
    continue;
    }
    break;
    }
    ui_browser__hide(menu);
    return key;
    }
#[no_mangle]
unsafe extern "C" fn ui__list_menu(argc: c_int, argv[]: *const *const c_char) -> c_int {
    static int ui__list_menu(int argc, char * const argv[])
    {
    struct ui_browser menu = {
    .entries    = (void *)argv,
    .refresh    = ui_browser__argv_refresh,
    .seek	    = ui_browser__argv_seek,
    .write	    = ui_browser__argv_write,
    .nr_entries = argc,
    };
    return list_menu__run(&menu);
    }
#[no_mangle]
pub unsafe extern "C" fn tui__header_window(session: *mut perf_session) -> c_int {
    int tui__header_window(struct perf_session *session)
    {
    int i, argc = 0;
    char **argv;
    char *ptr, *pos;
    size_t size;
    FILE *fp = open_memstream(&ptr, &size);
    perf_header__fprintf_info(session, fp, true);
    fclose(fp);
    for (pos = ptr, argc = 0; (pos = strchr(pos, '\n')) != core::ptr::null_mut(); pos++)
    argc++;
    argv = calloc(argc + 1, sizeof(*argv));
    if (argv == core::ptr::null_mut())
    goto out;
    argv[0] = pos = ptr;
    for (i = 1; (pos = strchr(pos, '\n')) != core::ptr::null_mut(); i++) {
// pos++ = '\0';
    argv[i] = pos;
    }
    BUG_ON(i != argc + 1);
    ui__list_menu(argc, argv);
    out:
    free(argv);
    free(ptr);
    return 0;
    }
