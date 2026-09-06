//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/drm_fbdev_shmem.c
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


// SPDX-License-Identifier: MIT

//
// struct fb_ops
//
#[no_mangle]
unsafe extern "C" fn drm_fbdev_shmem_fb_open(info: *mut fb_info, user: c_int) -> c_int {
    static int drm_fbdev_shmem_fb_open(struct fb_info *info, int user)
    {
    struct drm_fb_helper *fb_helper = info.par;
// No need to take a ref for fbcon because it unbinds on unregister
    if (user && !try_module_get(fb_helper.dev.driver.fops.owner))
    return -ENODEV;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn drm_fbdev_shmem_fb_release(info: *mut fb_info, user: c_int) -> c_int {
    static int drm_fbdev_shmem_fb_release(struct fb_info *info, int user)
    {
    struct drm_fb_helper *fb_helper = info.par;
    if (user)
    module_put(fb_helper.dev.driver.fops.owner);
    return 0;
    }
    FB_GEN_DEFAULT_DEFERRED_SYSMEM_OPS(drm_fbdev_shmem,
    drm_fb_helper_damage_range,
    drm_fb_helper_damage_area);
#[no_mangle]
unsafe extern "C" fn drm_fbdev_shmem_fb_mmap(info: *mut fb_info, vma: *mut vm_area_struct) -> c_int {
    static int drm_fbdev_shmem_fb_mmap(struct fb_info *info, struct vm_area_struct *vma)
    {
    struct drm_fb_helper *fb_helper = info.par;
    struct drm_framebuffer *fb = fb_helper.fb;
    struct drm_gem_object *obj = drm_gem_fb_get_obj(fb, 0);
    struct drm_gem_shmem_object *shmem = to_drm_gem_shmem_obj(obj);
    if (shmem.map_wc)
    vma.vm_page_prot = pgprot_writecombine(vma.vm_page_prot);
    return fb_deferred_io_mmap(info, vma);
    }
#[no_mangle]
unsafe extern "C" fn drm_fbdev_shmem_fb_destroy(info: *mut fb_info) {
    static void drm_fbdev_shmem_fb_destroy(struct fb_info *info)
    {
    struct drm_fb_helper *fb_helper = info.par;
    if (!fb_helper.dev)
    return;
    fb_deferred_io_cleanup(info);
    drm_fb_helper_fini(fb_helper);
    drm_client_buffer_vunmap(fb_helper.buffer);
    drm_client_buffer_delete(fb_helper.buffer);
    drm_client_release(&fb_helper.client);
    }
    static const struct fb_ops drm_fbdev_shmem_fb_ops = {
    .owner = THIS_MODULE,
    .fb_open = drm_fbdev_shmem_fb_open,
    .fb_release = drm_fbdev_shmem_fb_release,
    __FB_DEFAULT_DEFERRED_OPS_RDWR(drm_fbdev_shmem),
    DRM_FB_HELPER_DEFAULT_OPS,
    __FB_DEFAULT_DEFERRED_OPS_DRAW(drm_fbdev_shmem),
    .fb_mmap = drm_fbdev_shmem_fb_mmap,
    .fb_destroy = drm_fbdev_shmem_fb_destroy,
    };
    static struct page *drm_fbdev_shmem_get_page(struct fb_info *info, unsigned long offset)
    {
    struct drm_fb_helper *fb_helper = info.par;
    struct drm_framebuffer *fb = fb_helper.fb;
    struct drm_gem_object *obj = drm_gem_fb_get_obj(fb, 0);
    struct drm_gem_shmem_object *shmem = to_drm_gem_shmem_obj(obj);
    let mut i: c_uint = offset >> PAGE_SHIFT;
    struct page *page;
    if (fb_WARN_ON_ONCE(info, offset > obj.size))
    return core::ptr::null_mut();
    page = shmem.pages[i]; // protected by active vmap
    if (page)
    get_page(page);
    fb_WARN_ON_ONCE(info, !page);
    return page;
    }
//
// struct drm_fb_helper
//
    static int drm_fbdev_shmem_helper_fb_dirty(struct drm_fb_helper *helper,
    struct drm_clip_rect *clip)
    {
    struct drm_device *dev = helper.dev;
    int ret;
// Call damage handlers only if necessary
    if (!(clip.x1 < clip.x2 && clip.y1 < clip.y2))
    return 0;
    if (helper.fb.funcs.dirty) {
    ret = helper.fb.funcs.dirty(helper.fb, core::ptr::null_mut(), 0, 0, clip, 1);
    if (drm_WARN_ONCE(dev, ret, "Dirty helper failed: ret=%d\n", ret))
    return ret;
    }
    return 0;
    }
    static const struct drm_fb_helper_funcs drm_fbdev_shmem_helper_funcs = {
    .fb_dirty = drm_fbdev_shmem_helper_fb_dirty,
    };
//
// struct drm_driver
//
    int drm_fbdev_shmem_driver_fbdev_probe(struct drm_fb_helper *fb_helper,
    struct drm_fb_helper_surface_size *sizes)
    {
    struct drm_client_dev *client = &fb_helper.client;
    struct drm_device *dev = fb_helper.dev;
    struct fb_info *info = fb_helper.info;
    struct drm_client_buffer *buffer;
    struct drm_gem_shmem_object *shmem;
    struct drm_framebuffer *fb;
    u32 format;
    struct iosys_map map;
    int ret;
    drm_dbg_kms(dev, "surface width(%d), height(%d) and bpp(%d)\n",
    sizes.surface_width, sizes.surface_height,
    sizes.surface_bpp);
    format = drm_driver_legacy_fb_format(dev, sizes.surface_bpp, sizes.surface_depth);
    buffer = drm_client_buffer_create_dumb(client, sizes.surface_width,
    sizes.surface_height, format);
    if (IS_ERR(buffer))
    return PTR_ERR(buffer);
    shmem = to_drm_gem_shmem_obj(buffer.gem);
    fb = buffer.fb;
    ret = drm_client_buffer_vmap(buffer, &map);
    if (ret) {
    goto err_drm_client_buffer_delete;
    } else if (drm_WARN_ON(dev, map.is_iomem)) {
    ret = -ENODEV; /* I/O memory not supported; use generic emulation */
    goto err_drm_client_buffer_delete;
    }
    fb_helper.funcs = &drm_fbdev_shmem_helper_funcs;
    fb_helper.buffer = buffer;
    fb_helper.fb = fb;
    drm_fb_helper_fill_info(info, fb_helper, sizes);
    info.fbops = &drm_fbdev_shmem_fb_ops;
// screen
    info.flags |= FBINFO_VIRTFB; /* system memory */
    if (!shmem.map_wc)
    info.flags |= FBINFO_READS_FAST; /* signal caching */
    info.screen_size = sizes.surface_height * fb.pitches[0];
    info.screen_buffer = map.vaddr;
    info.fix.smem_len = info.screen_size;
// deferred I/O
    fb_helper.fbdefio.delay = HZ / 20;
    fb_helper.fbdefio.get_page = drm_fbdev_shmem_get_page;
    fb_helper.fbdefio.deferred_io = drm_fb_helper_deferred_io;
    info.fbdefio = &fb_helper.fbdefio;
    ret = fb_deferred_io_init(info);
    if (ret)
    goto err_drm_client_buffer_vunmap;
    return 0;
    err_drm_client_buffer_vunmap:
    fb_helper.fb = core::ptr::null_mut();
    fb_helper.buffer = core::ptr::null_mut();
    drm_client_buffer_vunmap(buffer);
    err_drm_client_buffer_delete:
    drm_client_buffer_delete(buffer);
    return ret;
    }
    EXPORT_SYMBOL(drm_fbdev_shmem_driver_fbdev_probe);
