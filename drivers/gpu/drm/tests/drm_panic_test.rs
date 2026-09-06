//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/tests/drm_panic_test.c
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
// Copyright (c) 2025 Red Hat.
// Author: Jocelyn Falempe <jfalempe@redhat.com>
//
// KUNIT tests for drm panic
//

// Check the framebuffer color only if the panic colors are the default

    CONFIG_DRM_PANIC_FOREGROUND_COLOR == 0xffffff)
#[no_mangle]
unsafe extern "C" fn drm_panic_check_color_byte(test: *mut kunit, b: u8) {
    static void drm_panic_check_color_byte(struct kunit *test, u8 b)
    {
    KUNIT_EXPECT_TRUE(test, (b == 0 || b == 0xff));
    }

    static void drm_panic_check_color_byte(struct kunit *test, u8 b) {}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_test_mode {
    pub width: c_int,
    pub height: c_int,
    pub format: u32,
    pub sb): *mut *mut void (draw_screen)(struct drm_scanout_buffer,
    pub fname: *const c_char,
}

//
// Run all tests for the 3 panic screens: user, kmsg and qr_code
//

    DRM_PANIC_TEST_MODE(1024, 768, DRM_FORMAT_XRGB8888, func) \
    DRM_PANIC_TEST_MODE(300, 200, DRM_FORMAT_XRGB8888, func) \
    DRM_PANIC_TEST_MODE(1920, 1080, DRM_FORMAT_XRGB8888, func) \
    DRM_PANIC_TEST_MODE(1024, 768, DRM_FORMAT_RGB565, func) \
    DRM_PANIC_TEST_MODE(1024, 768, DRM_FORMAT_RGB888, func) \

    .width = w, \
    .height = h, \
    .format = f, \
    .draw_screen = draw_panic_screen_##name, \
    .fname = #name, \
    }, \
    static const struct drm_test_mode drm_test_modes_cases[] = {
    DRM_TEST_MODE_LIST(user)
    DRM_TEST_MODE_LIST(kmsg)

    DRM_TEST_MODE_LIST(qr_code)

    };

#[no_mangle]
unsafe extern "C" fn drm_test_panic_init(test: *mut kunit) -> c_int {
    static int drm_test_panic_init(struct kunit *test)
    {
    struct drm_scanout_buffer *priv;
    priv = kunit_kzalloc(test, sizeof(*priv), GFP_KERNEL);
    KUNIT_ASSERT_NOT_NULL(test, priv);
    test.priv = priv;
    drm_panic_set_description("Kunit testing");
    return 0;
    }
//
// Test drawing the panic screen, using a memory mapped framebuffer
// Set the whole buffer to 0xa5, and then check that all pixels have been
// written.
//
#[no_mangle]
unsafe extern "C" fn drm_test_panic_screen_user_map(test: *mut kunit) {
    static void drm_test_panic_screen_user_map(struct kunit *test)
    {
    struct drm_scanout_buffer *sb = test.priv;
    const struct drm_test_mode *params = test.param_value;
    char *fb;
    int fb_size;
    int i;
    sb.format = drm_format_info(params.format);
    fb_size = params.width * params.height * sb.format.cpp[0];
    fb = vmalloc(fb_size);
    KUNIT_ASSERT_NOT_NULL(test, fb);
    memset(fb, 0xa5, fb_size);
    iosys_map_set_vaddr(&sb.map[0], fb);
    sb.width = params.width;
    sb.height = params.height;
    sb.pitch[0] = params.width * sb.format.cpp[0];
    params.draw_screen(sb);
    for (i = 0; i < fb_size; i++)
    drm_panic_check_color_byte(test, fb[i]);
    vfree(fb);
    }
//
// Test drawing the panic screen, using a list of pages framebuffer
// Set the whole buffer to 0xa5, and then check that all pixels have been
// written.
//
#[no_mangle]
unsafe extern "C" fn drm_test_panic_screen_user_page(test: *mut kunit) {
    static void drm_test_panic_screen_user_page(struct kunit *test)
    {
    struct drm_scanout_buffer *sb = test.priv;
    const struct drm_test_mode *params = test.param_value;
    int fb_size, p, i, npages;
    struct page **pages;
    u8 *vaddr;
    sb.format = drm_format_info(params.format);
    fb_size = params.width * params.height * sb.format.cpp[0];
    npages = DIV_ROUND_UP(fb_size, PAGE_SIZE);
    pages = kmalloc_objs(struct page *, npages);
    KUNIT_ASSERT_NOT_NULL(test, pages);
    for (p = 0; p < npages; p++) {
    pages[p] = alloc_page(GFP_KERNEL);
    if (!pages[p]) {
    npages = p - 1;
    KUNIT_FAIL(test, "Can't allocate page\n");
    goto free_pages;
    }
    vaddr = kmap_local_page(pages[p]);
    memset(vaddr, 0xa5, PAGE_SIZE);
    kunmap_local(vaddr);
    }
    sb.pages = pages;
    sb.width = params.width;
    sb.height = params.height;
    sb.pitch[0] = params.width * sb.format.cpp[0];
    params.draw_screen(sb);
    for (p = 0; p < npages; p++) {
    let mut bytes_in_page: c_int = (p == npages - 1) ? fb_size - p * PAGE_SIZE : PAGE_SIZE;
    vaddr = kmap_local_page(pages[p]);
    for (i = 0; i < bytes_in_page; i++)
    drm_panic_check_color_byte(test, vaddr[i]);
    kunmap_local(vaddr);
    }
    free_pages:
    for (p = 0; p < npages; p++)
    __free_page(pages[p]);
    kfree(pages);
    }
    static void drm_test_panic_set_pixel(struct drm_scanout_buffer *sb,
    unsigned int x,
    unsigned int y,
    u32 color)
    {
    struct kunit *test = (struct kunit *)sb.private;
    KUNIT_ASSERT_TRUE(test, x < sb.width && y < sb.height);
    }
//
// Test drawing the panic screen, using the set_pixel callback
// Check that all calls to set_pixel() are within the framebuffer
//
#[no_mangle]
unsafe extern "C" fn drm_test_panic_screen_user_set_pixel(test: *mut kunit) {
    static void drm_test_panic_screen_user_set_pixel(struct kunit *test)
    {
    struct drm_scanout_buffer *sb = test.priv;
    const struct drm_test_mode *params = test.param_value;
    sb.format = drm_format_info(params.format);
    sb.set_pixel = drm_test_panic_set_pixel;
    sb.width = params.width;
    sb.height = params.height;
    sb.private = test;
    params.draw_screen(sb);
    }
#[no_mangle]
unsafe extern "C" fn drm_test_panic_desc(t: *const drm_test_mode, desc: *mut c_char) {
    static void drm_test_panic_desc(const struct drm_test_mode *t, char *desc)
    {
    sprintf(desc, "Panic screen %s, mode: %d x %d \t%p4cc",
    t.fname, t.width, t.height, &t.format);
    }
    KUNIT_ARRAY_PARAM(drm_test_panic_screen_user_map, drm_test_modes_cases, drm_test_panic_desc);
    KUNIT_ARRAY_PARAM(drm_test_panic_screen_user_page, drm_test_modes_cases, drm_test_panic_desc);
    KUNIT_ARRAY_PARAM(drm_test_panic_screen_user_set_pixel, drm_test_modes_cases, drm_test_panic_desc);
    static struct kunit_case drm_panic_screen_user_test[] = {
    KUNIT_CASE_PARAM(drm_test_panic_screen_user_map,
    drm_test_panic_screen_user_map_gen_params),
    KUNIT_CASE_PARAM(drm_test_panic_screen_user_page,
    drm_test_panic_screen_user_page_gen_params),
    KUNIT_CASE_PARAM(drm_test_panic_screen_user_set_pixel,
    drm_test_panic_screen_user_set_pixel_gen_params),
    { }
    };
    static struct kunit_suite drm_panic_suite = {
    .name = "drm_panic",
    .init = drm_test_panic_init,
    .test_cases = drm_panic_screen_user_test,
    };
    kunit_test_suite(drm_panic_suite);
