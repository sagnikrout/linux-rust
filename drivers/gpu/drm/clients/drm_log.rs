//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/clients/drm_log.c
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


// SPDX-License-Identifier: GPL-2.0 or MIT
//
// Copyright (c) 2024 Red Hat.
// Author: Jocelyn Falempe <jfalempe@redhat.com>
//

    MODULE_AUTHOR("Jocelyn Falempe");
    MODULE_DESCRIPTION("DRM boot logger");
    MODULE_LICENSE("GPL");
    let mut scale: static unsigned int = 1;
    module_param(scale, uint, 0444);
    MODULE_PARM_DESC(scale, "Integer scaling factor for drm_log, default is 1");
//
// DOC: overview
//
// This is a simple graphic logger, to print the kernel message on screen, until
// a userspace application is able to take over.
// It is only for debugging purpose.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_log_scanout {
    pub buffer: *mut drm_client_buffer,
    pub font: *const font_desc,
    pub rows: u32,
    pub columns: u32,
    pub scaled_font_h: u32,
    pub scaled_font_w: u32,
    pub line: u32,
    pub format: u32,
    pub px_width: u32,
    pub front_color: u32,
    pub prefix_color: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_log {
    pub lock: mutex,
    pub client: drm_client_dev,
    pub con: console,
    pub probed: bool,
    pub n_scanout: u32,
    pub scanout: *mut drm_log_scanout,
}

    static struct drm_log *client_to_drm_log(struct drm_client_dev *client)
    {
    return container_of(client, struct drm_log, client);
    }
    static struct drm_log *console_to_drm_log(struct console *con)
    {
    return container_of(con, struct drm_log, con);
    }
    static void drm_log_blit(struct iosys_map *dst, unsigned int dst_pitch,
    const u8 *src, unsigned int src_pitch,
    u32 height, u32 width, u32 px_width, u32 color)
    {
    switch (px_width) {
    case 2:
    drm_draw_blit16(dst, dst_pitch, src, src_pitch, height, width, scale, color);
    break;
    case 3:
    drm_draw_blit24(dst, dst_pitch, src, src_pitch, height, width, scale, color);
    break;
    case 4:
    drm_draw_blit32(dst, dst_pitch, src, src_pitch, height, width, scale, color);
    break;
    default:
    WARN_ONCE(1, "Can't blit with pixel width %d\n", px_width);
    }
    }
#[no_mangle]
unsafe extern "C" fn drm_log_clear_line(scanout: *mut drm_log_scanout, line: u32) {
    static void drm_log_clear_line(struct drm_log_scanout *scanout, u32 line)
    {
    struct drm_framebuffer *fb = scanout.buffer.fb;
    let mut height: c_ulong = scanout.scaled_font_h;
    struct iosys_map map;
    let mut r: drm_rect = DRM_RECT_INIT(0, line * height, fb.width, height);
    if (drm_client_buffer_vmap_local(scanout.buffer, &map))
    return;
    iosys_map_memset(&map, r.y1 * fb.pitches[0], 0, height * fb.pitches[0]);
    drm_client_buffer_vunmap_local(scanout.buffer);
    drm_client_buffer_flush(scanout.buffer, &r);
    }
    static void drm_log_draw_line(struct drm_log_scanout *scanout, const char *s,
    unsigned int len, unsigned int prefix_len)
    {
    struct drm_framebuffer *fb = scanout.buffer.fb;
    struct iosys_map map;
    const struct font_desc *font = scanout.font;
    let mut font_pitch: usize = DIV_ROUND_UP(font.width, 8);
    const u8 *src;
    let mut px_width: u32 = fb.format.cpp[0];
    struct drm_rect r = DRM_RECT_INIT(0, scanout.line * scanout.scaled_font_h,
    fb.width, (scanout.line + 1) * scanout.scaled_font_h);
    u32 i;
    if (drm_client_buffer_vmap_local(scanout.buffer, &map))
    return;
    iosys_map_incr(&map, r.y1 * fb.pitches[0]);
    for (i = 0; i < len && i < scanout.columns; i++) {
    let mut color: u32 = (i < prefix_len) ? scanout.prefix_color : scanout.front_color;
    src = font_data_glyph_buf(font.data, font.width, font.height,
    (unsigned char)s[i]);
    if (src)
    drm_log_blit(&map, fb.pitches[0], src, font_pitch,
    scanout.scaled_font_h, scanout.scaled_font_w,
    px_width, color);
    iosys_map_incr(&map, scanout.scaled_font_w * px_width);
    }
    scanout.line++;
    if (scanout.line >= scanout.rows)
    scanout.line = 0;
    drm_client_buffer_vunmap_local(scanout.buffer);
    drm_client_buffer_flush(scanout.buffer, &r);
    }
    static void drm_log_draw_new_line(struct drm_log_scanout *scanout,
    const char *s, unsigned int len, unsigned int prefix_len)
    {
    if (scanout.line == 0) {
    drm_log_clear_line(scanout, 0);
    drm_log_clear_line(scanout, 1);
    drm_log_clear_line(scanout, 2);
    } else if (scanout.line + 2 < scanout.rows)
    drm_log_clear_line(scanout, scanout.line + 2);
    drm_log_draw_line(scanout, s, len, prefix_len);
    }
//
// Depends on print_time() in printk.c
// Timestamp is written with "[%5lu.%06lu]"
//
pub const TS_PREFIX_LEN: c_int = 13;
    static void drm_log_draw_kmsg_record(struct drm_log_scanout *scanout,
    const char *s, unsigned int len)
    {
    let mut prefix_len: u32 = 0;
    if (!len)
    return;
    if (len > TS_PREFIX_LEN && s[0] == '[' && s[6] == '.' && s[TS_PREFIX_LEN] == ']')
    prefix_len = TS_PREFIX_LEN + 1;
// do not print the ending \n character
    if (s[len - 1] == '\n')
    len--;
    while (len > scanout.columns) {
    drm_log_draw_new_line(scanout, s, scanout.columns, prefix_len);
    s += scanout.columns;
    len -= scanout.columns;
    prefix_len = 0;
    }
    if (len)
    drm_log_draw_new_line(scanout, s, len, prefix_len);
    }
#[no_mangle]
unsafe extern "C" fn drm_log_find_usable_format(plane: *mut drm_plane) -> u32 {
    static u32 drm_log_find_usable_format(struct drm_plane *plane)
    {
    int i;
    for (i = 0; i < plane.format_count; i++)
    if (drm_draw_can_convert_from_xrgb8888(plane.format_types[i]))
    return plane.format_types[i];
    return DRM_FORMAT_INVALID;
    }
    static int drm_log_setup_modeset(struct drm_client_dev *client,
    struct drm_mode_set *mode_set,
    struct drm_log_scanout *scanout)
    {
    struct drm_crtc *crtc = mode_set.crtc;
    let mut width: u32 = mode_set.mode.hdisplay;
    let mut height: u32 = mode_set.mode.vdisplay;
    u32 format;
    scanout.font = get_default_font(width, height, core::ptr::null_mut(), core::ptr::null_mut());
    if (!scanout.font)
    return -ENOENT;
    format = drm_log_find_usable_format(crtc.primary);
    if (format == DRM_FORMAT_INVALID)
    return -EINVAL;
    scanout.buffer = drm_client_buffer_create_dumb(client, width, height, format);
    if (IS_ERR(scanout.buffer)) {
    drm_warn(client.dev, "drm_log can't create framebuffer %d %d %p4cc\n",
    width, height, &format);
    return -ENOMEM;
    }
    mode_set.fb = scanout.buffer.fb;
    scanout.scaled_font_h = scanout.font.height * scale;
    scanout.scaled_font_w = scanout.font.width * scale;
    scanout.rows = height / scanout.scaled_font_h;
    scanout.columns = width / scanout.scaled_font_w;
    if (!scanout.rows || !scanout.columns) {
    drm_client_buffer_delete(scanout.buffer);
    scanout.buffer = core::ptr::null_mut();
    mode_set.fb = core::ptr::null_mut();
    return -EINVAL;
    }
    scanout.front_color = drm_draw_color_from_xrgb8888(0xffffff, format);
    scanout.prefix_color = drm_draw_color_from_xrgb8888(0x4e9a06, format);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn drm_log_count_modeset(client: *mut drm_client_dev) -> c_int {
    static int drm_log_count_modeset(struct drm_client_dev *client)
    {
    struct drm_mode_set *mode_set;
    let mut count: c_int = 0;
    mutex_lock(&client.modeset_mutex);
    drm_client_for_each_modeset(mode_set, client)
    count++;
    mutex_unlock(&client.modeset_mutex);
    return count;
    }
#[no_mangle]
unsafe extern "C" fn drm_log_init_client(dlog: *mut drm_log) {
    static void drm_log_init_client(struct drm_log *dlog)
    {
    struct drm_client_dev *client = &dlog.client;
    struct drm_mode_set *mode_set;
    int i, max_modeset;
    let mut n_modeset: c_int = 0;
    dlog.probed = true;
    if (drm_client_modeset_probe(client, 0, 0))
    return;
    max_modeset = drm_log_count_modeset(client);
    if (!max_modeset)
    return;
    dlog.scanout = kzalloc_objs(*dlog.scanout, max_modeset);
    if (!dlog.scanout)
    return;
    mutex_lock(&client.modeset_mutex);
    drm_client_for_each_modeset(mode_set, client) {
    if (!mode_set.mode)
    continue;
    if (drm_log_setup_modeset(client, mode_set, &dlog.scanout[n_modeset]))
    continue;
    n_modeset++;
    }
    mutex_unlock(&client.modeset_mutex);
    if (n_modeset == 0)
    goto err_nomodeset;
    if (drm_client_modeset_commit(client))
    goto err_failed_commit;
    dlog.n_scanout = n_modeset;
    return;
    err_failed_commit:
    for (i = 0; i < n_modeset; i++)
    drm_client_buffer_delete(dlog.scanout[i].buffer);
    err_nomodeset:
    kfree(dlog.scanout);
    dlog.scanout = core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn drm_log_free_scanout(client: *mut drm_client_dev) {
    static void drm_log_free_scanout(struct drm_client_dev *client)
    {
    struct drm_log *dlog = client_to_drm_log(client);
    int i;
    if (dlog.n_scanout) {
    for (i = 0; i < dlog.n_scanout; i++)
    drm_client_buffer_delete(dlog.scanout[i].buffer);
    dlog.n_scanout = 0;
    kfree(dlog.scanout);
    dlog.scanout = core::ptr::null_mut();
    }
    }
#[no_mangle]
unsafe extern "C" fn drm_log_client_free(client: *mut drm_client_dev) {
    static void drm_log_client_free(struct drm_client_dev *client)
    {
    struct drm_log *dlog = client_to_drm_log(client);
    struct drm_device *dev = client.dev;
    kfree(dlog);
    drm_dbg(dev, "Unregistered with drm log\n");
    }
#[no_mangle]
unsafe extern "C" fn drm_log_client_unregister(client: *mut drm_client_dev) {
    static void drm_log_client_unregister(struct drm_client_dev *client)
    {
    struct drm_log *dlog = client_to_drm_log(client);
    unregister_console(&dlog.con);
    mutex_lock(&dlog.lock);
    drm_log_free_scanout(client);
    mutex_unlock(&dlog.lock);
    drm_client_release(client);
    }
#[no_mangle]
unsafe extern "C" fn drm_log_client_restore(client: *mut drm_client_dev, force: bool) -> c_int {
    static int drm_log_client_restore(struct drm_client_dev *client, bool force)
    {
    int ret;
    if (force)
    ret = drm_client_modeset_commit_locked(client);
    else
    ret = drm_client_modeset_commit(client);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn drm_log_client_hotplug(client: *mut drm_client_dev) -> c_int {
    static int drm_log_client_hotplug(struct drm_client_dev *client)
    {
    struct drm_log *dlog = client_to_drm_log(client);
    mutex_lock(&dlog.lock);
    drm_log_free_scanout(client);
    dlog.probed = false;
    mutex_unlock(&dlog.lock);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn drm_log_client_suspend(client: *mut drm_client_dev) -> c_int {
    static int drm_log_client_suspend(struct drm_client_dev *client)
    {
    struct drm_log *dlog = client_to_drm_log(client);
    console_suspend(&dlog.con);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn drm_log_client_resume(client: *mut drm_client_dev) -> c_int {
    static int drm_log_client_resume(struct drm_client_dev *client)
    {
    struct drm_log *dlog = client_to_drm_log(client);
    console_resume(&dlog.con);
    return 0;
    }
    static const struct drm_client_funcs drm_log_client_funcs = {
    .owner		= THIS_MODULE,
    .free		= drm_log_client_free,
    .unregister	= drm_log_client_unregister,
    .restore	= drm_log_client_restore,
    .hotplug	= drm_log_client_hotplug,
    .suspend	= drm_log_client_suspend,
    .resume		= drm_log_client_resume,
    };
#[no_mangle]
unsafe extern "C" fn drm_log_write_thread(con: *mut console, wctxt: *mut nbcon_write_context) {
    static void drm_log_write_thread(struct console *con, struct nbcon_write_context *wctxt)
    {
    struct drm_log *dlog = console_to_drm_log(con);
    int i;
    if (!dlog.probed)
    drm_log_init_client(dlog);
// Check that we are still the master before drawing
    if (drm_master_internal_acquire(dlog.client.dev)) {
    drm_master_internal_release(dlog.client.dev);
    for (i = 0; i < dlog.n_scanout; i++)
    drm_log_draw_kmsg_record(&dlog.scanout[i], wctxt.outbuf, wctxt.len);
    }
    }
#[no_mangle]
unsafe extern "C" fn drm_log_lock(con: *mut console, flags: *mut c_ulong) {
    static void drm_log_lock(struct console *con, unsigned long *flags)
    {
    struct drm_log *dlog = console_to_drm_log(con);
    mutex_lock(&dlog.lock);
    migrate_disable();
    }
#[no_mangle]
unsafe extern "C" fn drm_log_unlock(con: *mut console, flags: c_ulong) {
    static void drm_log_unlock(struct console *con, unsigned long flags)
    {
    struct drm_log *dlog = console_to_drm_log(con);
    migrate_enable();
    mutex_unlock(&dlog.lock);
    }
#[no_mangle]
unsafe extern "C" fn drm_log_register_console(con: *mut console) {
    static void drm_log_register_console(struct console *con)
    {
    strscpy(con.name, "drm_log");
    con.write_thread = drm_log_write_thread;
    con.device_lock = drm_log_lock;
    con.device_unlock = drm_log_unlock;
    con.flags = CON_PRINTBUFFER | CON_NBCON;
    con.index = -1;
    register_console(con);
    }
//
// drm_log_register() - Register a drm device to drm_log
// @dev: the drm device to register.
//
#[no_mangle]
pub unsafe extern "C" fn drm_log_register(dev: *mut drm_device) {
    void drm_log_register(struct drm_device *dev)
    {
    struct drm_log *new;
    if (!scale)
    scale = 1;
    new = kzalloc_obj(*new);
    if (!new)
    goto err_warn;
    mutex_init(&new.lock);
    if (drm_client_init(dev, &new.client, "drm_log", &drm_log_client_funcs))
    goto err_free;
    drm_client_register(&new.client);
    drm_log_register_console(&new.con);
    drm_dbg(dev, "Registered with drm log as %s\n", new.con.name);
    return;
    err_free:
    kfree(new);
    err_warn:
    drm_warn(dev, "Failed to register with drm log\n");
    }
