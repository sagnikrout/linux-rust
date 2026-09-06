//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/tiny/bochs.c
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

// ----------------------------------------------------------------------
pub const VBE_DISPI_IOPORT_INDEX: c_uint = 0x01CE;
pub const VBE_DISPI_IOPORT_DATA: c_uint = 0x01CF;
pub const VBE_DISPI_INDEX_ID: c_uint = 0x0;
pub const VBE_DISPI_INDEX_XRES: c_uint = 0x1;
pub const VBE_DISPI_INDEX_YRES: c_uint = 0x2;
pub const VBE_DISPI_INDEX_BPP: c_uint = 0x3;
pub const VBE_DISPI_INDEX_ENABLE: c_uint = 0x4;
pub const VBE_DISPI_INDEX_BANK: c_uint = 0x5;
pub const VBE_DISPI_INDEX_VIRT_WIDTH: c_uint = 0x6;
pub const VBE_DISPI_INDEX_VIRT_HEIGHT: c_uint = 0x7;
pub const VBE_DISPI_INDEX_X_OFFSET: c_uint = 0x8;
pub const VBE_DISPI_INDEX_Y_OFFSET: c_uint = 0x9;
pub const VBE_DISPI_INDEX_VIDEO_MEMORY_64K: c_uint = 0xa;
pub const VBE_DISPI_ID0: c_uint = 0xB0C0;
pub const VBE_DISPI_ID1: c_uint = 0xB0C1;
pub const VBE_DISPI_ID2: c_uint = 0xB0C2;
pub const VBE_DISPI_ID3: c_uint = 0xB0C3;
pub const VBE_DISPI_ID4: c_uint = 0xB0C4;
pub const VBE_DISPI_ID5: c_uint = 0xB0C5;
pub const VBE_DISPI_DISABLED: c_uint = 0x00;
pub const VBE_DISPI_ENABLED: c_uint = 0x01;
pub const VBE_DISPI_GETCAPS: c_uint = 0x02;
pub const VBE_DISPI_8BIT_DAC: c_uint = 0x20;
pub const VBE_DISPI_LFB_ENABLED: c_uint = 0x40;
pub const VBE_DISPI_NOCLEARMEM: c_uint = 0x80;
    let mut bochs_modeset: static int = -1;
    let mut defx: static int = 1024;
    let mut defy: static int = 768;
    module_param_named(modeset, bochs_modeset, int, 0444);
    MODULE_PARM_DESC(modeset, "enable/disable kernel modesetting");
    module_param(defx, int, 0444);
    module_param(defy, int, 0444);
    MODULE_PARM_DESC(defx, "default x resolution");
    MODULE_PARM_DESC(defy, "default y resolution");
// ----------------------------------------------------------------------
    enum bochs_types {
    BOCHS_QEMU_STDVGA,
    BOCHS_SIMICS,
    BOCHS_UNKNOWN,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bochs_device {
    pub dev: drm_device,
// hw
    pub mmio: *mut void __iomem,
    pub ioports: c_int,
    pub fb_map: *mut void __iomem,
    pub fb_base: c_ulong,
    pub fb_size: c_ulong,
    pub qext_size: c_ulong,
// mode
    pub xres: u16,
    pub yres: u16,
    pub yres_virtual: u16,
    pub stride: u32,
    pub bpp: u32,
// drm
    pub primary_plane: drm_plane,
    pub crtc: drm_crtc,
    pub encoder: drm_encoder,
    pub connector: drm_connector,
}

    static struct bochs_device *to_bochs_device(const struct drm_device *dev)
    {
    return container_of(dev, struct bochs_device, dev);
    }
// ----------------------------------------------------------------------
#[no_mangle]
unsafe extern "C" fn bochs_uses_mmio(bochs: *mut bochs_device) -> __always_inline bool {
    static __always_inline bool bochs_uses_mmio(struct bochs_device *bochs)
    {
    return !IS_ENABLED(CONFIG_HAS_IOPORT) || bochs.mmio;
    }
#[no_mangle]
unsafe extern "C" fn bochs_vga_writeb(bochs: *mut bochs_device, ioport: u16, val: u8) {
    static void bochs_vga_writeb(struct bochs_device *bochs, u16 ioport, u8 val)
    {
    if (WARN_ON(ioport < 0x3c0 || ioport > 0x3df))
    return;
    if (bochs_uses_mmio(bochs)) {
    let mut offset: c_int = ioport - 0x3c0 + 0x400;
    writeb(val, bochs.mmio + offset);
    } else {
    outb(val, ioport);
    }
    }
#[no_mangle]
unsafe extern "C" fn bochs_vga_readb(bochs: *mut bochs_device, ioport: u16) -> u8 {
    static u8 bochs_vga_readb(struct bochs_device *bochs, u16 ioport)
    {
    if (WARN_ON(ioport < 0x3c0 || ioport > 0x3df))
    return 0xff;
    if (bochs_uses_mmio(bochs)) {
    let mut offset: c_int = ioport - 0x3c0 + 0x400;
    return readb(bochs.mmio + offset);
    } else {
    return inb(ioport);
    }
    }
#[no_mangle]
unsafe extern "C" fn bochs_dispi_read(bochs: *mut bochs_device, reg: u16) -> u16 {
    static u16 bochs_dispi_read(struct bochs_device *bochs, u16 reg)
    {
    let mut ret: u16 = 0;
    if (bochs_uses_mmio(bochs)) {
    let mut offset: c_int = 0x500 + (reg << 1);
    ret = readw(bochs.mmio + offset);
    } else {
    outw(reg, VBE_DISPI_IOPORT_INDEX);
    ret = inw(VBE_DISPI_IOPORT_DATA);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn bochs_dispi_write(bochs: *mut bochs_device, reg: u16, val: u16) {
    static void bochs_dispi_write(struct bochs_device *bochs, u16 reg, u16 val)
    {
    if (bochs_uses_mmio(bochs)) {
    let mut offset: c_int = 0x500 + (reg << 1);
    writew(val, bochs.mmio + offset);
    } else {
    outw(reg, VBE_DISPI_IOPORT_INDEX);
    outw(val, VBE_DISPI_IOPORT_DATA);
    }
    }
#[no_mangle]
unsafe extern "C" fn bochs_hw_set_big_endian(bochs: *mut bochs_device) {
    static void bochs_hw_set_big_endian(struct bochs_device *bochs)
    {
    if (bochs.qext_size < 8)
    return;
    writel(0xbebebebe, bochs.mmio + 0x604);
    }
#[no_mangle]
unsafe extern "C" fn bochs_hw_set_little_endian(bochs: *mut bochs_device) {
    static void bochs_hw_set_little_endian(struct bochs_device *bochs)
    {
    if (bochs.qext_size < 8)
    return;
    writel(0x1e1e1e1e, bochs.mmio + 0x604);
    }

#[no_mangle]
unsafe extern "C" fn bochs_get_edid_block(data: *mut c_void, buf: *mut u8, block: c_uint, len: usize) -> c_int {
    static int bochs_get_edid_block(void *data, u8 *buf, unsigned int block, size_t len)
    {
    struct bochs_device *bochs = data;
    size_t i, start = block * EDID_LENGTH;
    if (!bochs.mmio)
    return -1;
    if (start + len > 0x400 /* vga register offset */)
    return -1;
    for (i = 0; i < len; i++)
    buf[i] = readb(bochs.mmio + start + i);
    return 0;
    }
    static const struct drm_edid *bochs_hw_read_edid(struct drm_connector *connector)
    {
    struct drm_device *dev = connector.dev;
    struct bochs_device *bochs = to_bochs_device(dev);
    u8 header[8];
// check header to detect whenever edid support is enabled in qemu
    bochs_get_edid_block(bochs, header, 0, ARRAY_SIZE(header));
    if (drm_edid_header_is_valid(header) != 8)
    return core::ptr::null_mut();
    drm_dbg(dev, "Found EDID data blob.\n");
    return drm_edid_read_custom(connector, bochs_get_edid_block, bochs);
    }
#[no_mangle]
unsafe extern "C" fn bochs_hw_init(bochs: *mut bochs_device) -> c_int {
    static int bochs_hw_init(struct bochs_device *bochs)
    {
    struct drm_device *dev = &bochs.dev;
    struct pci_dev *pdev = to_pci_dev(dev.dev);
    unsigned long addr, size, mem, ioaddr, iosize;
    u16 id;
    if (pdev.resource[2].flags & IORESOURCE_MEM) {
    ioaddr = pci_resource_start(pdev, 2);
    iosize = pci_resource_len(pdev, 2);
// mmio bar with vga and bochs registers present
    if (!devm_request_mem_region(&pdev.dev, ioaddr, iosize, "bochs-drm")) {
    DRM_ERROR("Cannot request mmio region\n");
    return -EBUSY;
    }
    bochs.mmio = devm_ioremap(&pdev.dev, ioaddr, iosize);
    if (bochs.mmio == core::ptr::null_mut()) {
    DRM_ERROR("Cannot map mmio region\n");
    return -ENOMEM;
    }
    } else if (IS_ENABLED(CONFIG_HAS_IOPORT)) {
    ioaddr = VBE_DISPI_IOPORT_INDEX;
    iosize = 2;
    if (!devm_request_region(&pdev.dev, ioaddr, iosize, "bochs-drm")) {
    DRM_ERROR("Cannot request ioports\n");
    return -EBUSY;
    }
    bochs.ioports = 1;
    } else {
    drm_err(dev, "I/O ports are not supported\n");
    return -EIO;
    }
    id = bochs_dispi_read(bochs, VBE_DISPI_INDEX_ID);
    mem = bochs_dispi_read(bochs, VBE_DISPI_INDEX_VIDEO_MEMORY_64K)
// 64 * 1024;
    if ((id & 0xfff0) != VBE_DISPI_ID0) {
    DRM_ERROR("ID mismatch\n");
    return -ENODEV;
    }
    if ((pdev.resource[0].flags & IORESOURCE_MEM) == 0)
    return -ENODEV;
    addr = pci_resource_start(pdev, 0);
    size = pci_resource_len(pdev, 0);
    if (addr == 0)
    return -ENODEV;
    if (size != mem) {
    DRM_ERROR("Size mismatch: pci=%ld, bochs=%ld\n",
    size, mem);
    size = min(size, mem);
    }
    if (!devm_request_mem_region(&pdev.dev, addr, size, "bochs-drm"))
    DRM_WARN("Cannot request framebuffer, boot fb still active?\n");
    bochs.fb_map = devm_ioremap_wc(&pdev.dev, addr, size);
    if (bochs.fb_map == core::ptr::null_mut()) {
    DRM_ERROR("Cannot map framebuffer\n");
    return -ENOMEM;
    }
    bochs.fb_base = addr;
    bochs.fb_size = size;
    DRM_INFO("Found bochs VGA, ID 0x%x.\n", id);
    DRM_INFO("Framebuffer size %ld kB @ 0x%lx, %s @ 0x%lx.\n",
    size / 1024, addr,
    bochs.ioports ? "ioports" : "mmio",
    ioaddr);
    if (bochs.mmio && pdev.revision >= 2) {
    bochs.qext_size = readl(bochs.mmio + 0x600);
    if (bochs.qext_size < 4 || bochs.qext_size > iosize) {
    bochs.qext_size = 0;
    goto noext;
    }
    DRM_DEBUG("Found qemu ext regs, size %ld\n",
    bochs.qext_size);
    bochs_hw_set_native_endian(bochs);
    }
    noext:
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bochs_hw_blank(bochs: *mut bochs_device, blank: bool) {
    static void bochs_hw_blank(struct bochs_device *bochs, bool blank)
    {
    DRM_DEBUG_DRIVER("hw_blank %d\n", blank);
// enable color bit (so VGA_IS1_RC access works)
    bochs_vga_writeb(bochs, VGA_MIS_W, VGA_MIS_COLOR);
// discard ar_flip_flop
    (void)bochs_vga_readb(bochs, VGA_IS1_RC);
// blank or unblank; we need only update index and set 0x20
    bochs_vga_writeb(bochs, VGA_ATT_W, blank ? 0 : 0x20);
    }
#[no_mangle]
unsafe extern "C" fn bochs_hw_setmode(bochs: *mut bochs_device, mode: *mut drm_display_mode) {
    static void bochs_hw_setmode(struct bochs_device *bochs, struct drm_display_mode *mode)
    {
    int idx;
    if (!drm_dev_enter(&bochs.dev, &idx))
    return;
    bochs.xres = mode.hdisplay;
    bochs.yres = mode.vdisplay;
    bochs.bpp = 32;
    bochs.stride = mode.hdisplay * (bochs.bpp / 8);
    bochs.yres_virtual = bochs.fb_size / bochs.stride;
    DRM_DEBUG_DRIVER("%dx%d @ %d bpp, vy %d\n",
    bochs.xres, bochs.yres, bochs.bpp,
    bochs.yres_virtual);
    bochs_dispi_write(bochs, VBE_DISPI_INDEX_ENABLE,      0);
    bochs_dispi_write(bochs, VBE_DISPI_INDEX_BPP,         bochs.bpp);
    bochs_dispi_write(bochs, VBE_DISPI_INDEX_XRES,        bochs.xres);
    bochs_dispi_write(bochs, VBE_DISPI_INDEX_YRES,        bochs.yres);
    bochs_dispi_write(bochs, VBE_DISPI_INDEX_BANK,        0);
    bochs_dispi_write(bochs, VBE_DISPI_INDEX_VIRT_WIDTH,  bochs.xres);
    bochs_dispi_write(bochs, VBE_DISPI_INDEX_VIRT_HEIGHT,
    bochs.yres_virtual);
    bochs_dispi_write(bochs, VBE_DISPI_INDEX_X_OFFSET,    0);
    bochs_dispi_write(bochs, VBE_DISPI_INDEX_Y_OFFSET,    0);
    bochs_dispi_write(bochs, VBE_DISPI_INDEX_ENABLE,
    VBE_DISPI_ENABLED | VBE_DISPI_LFB_ENABLED);
    drm_dev_exit(idx);
    }
#[no_mangle]
unsafe extern "C" fn bochs_hw_setformat(bochs: *mut bochs_device, format: *const drm_format_info) {
    static void bochs_hw_setformat(struct bochs_device *bochs, const struct drm_format_info *format)
    {
    int idx;
    if (!drm_dev_enter(&bochs.dev, &idx))
    return;
    DRM_DEBUG_DRIVER("format %c%c%c%c\n",
    (format.format >>  0) & 0xff,
    (format.format >>  8) & 0xff,
    (format.format >> 16) & 0xff,
    (format.format >> 24) & 0xff);
    switch (format.format) {
    case DRM_FORMAT_XRGB8888:
    bochs_hw_set_little_endian(bochs);
    break;
    case DRM_FORMAT_BGRX8888:
    bochs_hw_set_big_endian(bochs);
    break;
    default:
// should not happen
    DRM_ERROR("%s: Huh? Got framebuffer format 0x%x",
    __func__, format.format);
    break;
    }
    drm_dev_exit(idx);
    }
#[no_mangle]
unsafe extern "C" fn bochs_hw_setbase(bochs: *mut bochs_device, x: c_int, y: c_int, stride: c_int, addr: u64) {
    static void bochs_hw_setbase(struct bochs_device *bochs, int x, int y, int stride, u64 addr)
    {
    unsigned long offset;
    unsigned int vx, vy, vwidth, idx;
    if (!drm_dev_enter(&bochs.dev, &idx))
    return;
    bochs.stride = stride;
    offset = (unsigned long)addr +
    y * bochs.stride +
    x * (bochs.bpp / 8);
    vy = offset / bochs.stride;
    vx = (offset % bochs.stride) * 8 / bochs.bpp;
    vwidth = stride * 8 / bochs.bpp;
    DRM_DEBUG_DRIVER("x %d, y %d, addr %llx . offset %lx, vx %d, vy %d\n",
    x, y, addr, offset, vx, vy);
    bochs_dispi_write(bochs, VBE_DISPI_INDEX_VIRT_WIDTH, vwidth);
    bochs_dispi_write(bochs, VBE_DISPI_INDEX_X_OFFSET, vx);
    bochs_dispi_write(bochs, VBE_DISPI_INDEX_Y_OFFSET, vy);
    drm_dev_exit(idx);
    }
// ----------------------------------------------------------------------
    static const uint32_t bochs_primary_plane_formats[] = {
    DRM_FORMAT_XRGB8888,
    DRM_FORMAT_BGRX8888,
    };
    static int bochs_primary_plane_helper_atomic_check(struct drm_plane *plane,
    struct drm_atomic_commit *state)
    {
    struct drm_plane_state *new_plane_state = drm_atomic_get_new_plane_state(state, plane);
    struct drm_crtc *new_crtc = new_plane_state.crtc;
    struct drm_crtc_state *new_crtc_state = core::ptr::null_mut();
    int ret;
    if (new_crtc)
    new_crtc_state = drm_atomic_get_new_crtc_state(state, new_crtc);
    ret = drm_atomic_helper_check_plane_state(new_plane_state, new_crtc_state,
    DRM_PLANE_NO_SCALING,
    DRM_PLANE_NO_SCALING,
    false, false);
    if (ret)
    return ret;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !new_plane_state->visible) -> else {
    else if (!new_plane_state.visible)
    return 0;
    return 0;
    }
    static void bochs_primary_plane_helper_atomic_update(struct drm_plane *plane,
    struct drm_atomic_commit *state)
    {
    struct drm_device *dev = plane.dev;
    struct bochs_device *bochs = to_bochs_device(dev);
    struct drm_plane_state *plane_state = plane.state;
    struct drm_plane_state *old_plane_state = drm_atomic_get_old_plane_state(state, plane);
    struct drm_shadow_plane_state *shadow_plane_state = to_drm_shadow_plane_state(plane_state);
    struct drm_framebuffer *fb = plane_state.fb;
    struct drm_atomic_helper_damage_iter iter;
    struct drm_rect damage;
    if (!fb || !bochs.stride)
    return;
    drm_atomic_helper_damage_iter_init(&iter, old_plane_state, plane_state);
    drm_atomic_for_each_plane_damage(&iter, &damage) {
    let mut dst: iosys_map = IOSYS_MAP_INIT_VADDR_IOMEM(bochs.fb_map);
    iosys_map_incr(&dst, drm_fb_clip_offset(fb.pitches[0], fb.format, &damage));
    drm_fb_memcpy(&dst, fb.pitches, shadow_plane_state.data, fb, &damage);
    }
// Always scanout image at VRAM offset 0
    bochs_hw_setbase(bochs,
    plane_state.crtc_x,
    plane_state.crtc_y,
    fb.pitches[0],
    0);
    bochs_hw_setformat(bochs, fb.format);
    }
    static int bochs_primary_plane_helper_get_scanout_buffer(struct drm_plane *plane,
    struct drm_scanout_buffer *sb)
    {
    struct bochs_device *bochs = to_bochs_device(plane.dev);
    let mut map: iosys_map = IOSYS_MAP_INIT_VADDR_IOMEM(bochs.fb_map);
    if (plane.state && plane.state.fb) {
    sb.format = plane.state.fb.format;
    sb.width = plane.state.fb.width;
    sb.height = plane.state.fb.height;
    sb.pitch[0] = plane.state.fb.pitches[0];
    sb.map[0] = map;
    return 0;
    }
    return -ENODEV;
    }
    static const struct drm_plane_helper_funcs bochs_primary_plane_helper_funcs = {
    DRM_GEM_SHADOW_PLANE_HELPER_FUNCS,
    .atomic_check = bochs_primary_plane_helper_atomic_check,
    .atomic_update = bochs_primary_plane_helper_atomic_update,
    .get_scanout_buffer = bochs_primary_plane_helper_get_scanout_buffer,
    };
    static const struct drm_plane_funcs bochs_primary_plane_funcs = {
    .update_plane = drm_atomic_helper_update_plane,
    .disable_plane = drm_atomic_helper_disable_plane,
    .destroy = drm_plane_cleanup,
    DRM_GEM_SHADOW_PLANE_FUNCS
    };
#[no_mangle]
unsafe extern "C" fn bochs_crtc_helper_mode_set_nofb(crtc: *mut drm_crtc) {
    static void bochs_crtc_helper_mode_set_nofb(struct drm_crtc *crtc)
    {
    struct bochs_device *bochs = to_bochs_device(crtc.dev);
    struct drm_crtc_state *crtc_state = crtc.state;
    bochs_hw_setmode(bochs, &crtc_state.mode);
    }
    static int bochs_crtc_helper_atomic_check(struct drm_crtc *crtc,
    struct drm_atomic_commit *state)
    {
    struct drm_crtc_state *crtc_state = drm_atomic_get_new_crtc_state(state, crtc);
    if (!crtc_state.enable)
    return 0;
    return drm_atomic_helper_check_crtc_primary_plane(crtc_state);
    }
    static void bochs_crtc_helper_atomic_enable(struct drm_crtc *crtc,
    struct drm_atomic_commit *state)
    {
    struct bochs_device *bochs = to_bochs_device(crtc.dev);
    bochs_hw_blank(bochs, false);
    drm_crtc_vblank_on(crtc);
    }
    static void bochs_crtc_helper_atomic_disable(struct drm_crtc *crtc,
    struct drm_atomic_commit *crtc_state)
    {
    struct bochs_device *bochs = to_bochs_device(crtc.dev);
    drm_crtc_vblank_off(crtc);
    bochs_hw_blank(bochs, true);
    }
    static const struct drm_crtc_helper_funcs bochs_crtc_helper_funcs = {
    .mode_set_nofb = bochs_crtc_helper_mode_set_nofb,
    .atomic_check = bochs_crtc_helper_atomic_check,
    .atomic_flush = drm_crtc_vblank_atomic_flush,
    .atomic_enable = bochs_crtc_helper_atomic_enable,
    .atomic_disable = bochs_crtc_helper_atomic_disable,
    };
    static const struct drm_crtc_funcs bochs_crtc_funcs = {
    .reset = drm_atomic_helper_crtc_reset,
    .destroy = drm_crtc_cleanup,
    .set_config = drm_atomic_helper_set_config,
    .page_flip = drm_atomic_helper_page_flip,
    .atomic_duplicate_state = drm_atomic_helper_crtc_duplicate_state,
    .atomic_destroy_state = drm_atomic_helper_crtc_destroy_state,
    DRM_CRTC_VBLANK_TIMER_FUNCS,
    };
    static const struct drm_encoder_funcs bochs_encoder_funcs = {
    .destroy = drm_encoder_cleanup,
    };
#[no_mangle]
unsafe extern "C" fn bochs_connector_helper_get_modes(connector: *mut drm_connector) -> c_int {
    static int bochs_connector_helper_get_modes(struct drm_connector *connector)
    {
    const struct drm_edid *edid;
    int count;
    edid = bochs_hw_read_edid(connector);
    if (edid) {
    drm_edid_connector_update(connector, edid);
    count = drm_edid_connector_add_modes(connector);
    drm_edid_free(edid);
    } else {
    drm_edid_connector_update(connector, core::ptr::null_mut());
    count = drm_add_modes_noedid(connector, 8192, 8192);
    drm_set_preferred_mode(connector, defx, defy);
    }
    return count;
    }
    static const struct drm_connector_helper_funcs bochs_connector_helper_funcs = {
    .get_modes = bochs_connector_helper_get_modes,
    };
    static const struct drm_connector_funcs bochs_connector_funcs = {
    .fill_modes = drm_helper_probe_single_connector_modes,
    .destroy = drm_connector_cleanup,
    .reset = drm_atomic_helper_connector_reset,
    .atomic_duplicate_state = drm_atomic_helper_connector_duplicate_state,
    .atomic_destroy_state = drm_atomic_helper_connector_destroy_state,
    };
    static enum drm_mode_status bochs_mode_config_mode_valid(struct drm_device *dev,
    const struct drm_display_mode *mode)
    {
    struct bochs_device *bochs = to_bochs_device(dev);
    const struct drm_format_info *format = drm_format_info(DRM_FORMAT_XRGB8888);
    u64 pitch;
    if (drm_WARN_ON(dev, !format))
    return MODE_ERROR;
    pitch = drm_format_info_min_pitch(format, 0, mode.hdisplay);
    if (!pitch)
    return MODE_BAD_WIDTH;
    if (mode.vdisplay > DIV_ROUND_DOWN_ULL(bochs.fb_size, pitch))
    return MODE_MEM;
    return MODE_OK;
    }
    static const struct drm_mode_config_funcs bochs_mode_config_funcs = {
    .fb_create = drm_gem_fb_create_with_dirty,
    .mode_valid = bochs_mode_config_mode_valid,
    .atomic_check = drm_atomic_helper_check,
    .atomic_commit = drm_atomic_helper_commit,
    };
#[no_mangle]
unsafe extern "C" fn bochs_kms_init(bochs: *mut bochs_device) -> c_int {
    static int bochs_kms_init(struct bochs_device *bochs)
    {
    struct drm_device *dev = &bochs.dev;
    struct drm_plane *primary_plane;
    struct drm_crtc *crtc;
    struct drm_connector *connector;
    struct drm_encoder *encoder;
    int ret;
    ret = drmm_mode_config_init(dev);
    if (ret)
    return ret;
    dev.mode_config.max_width = 8192;
    dev.mode_config.max_height = 8192;
    dev.mode_config.preferred_depth = 24;
    dev.mode_config.quirk_addfb_prefer_host_byte_order = true;
    dev.mode_config.funcs = &bochs_mode_config_funcs;
    primary_plane = &bochs.primary_plane;
    ret = drm_universal_plane_init(dev, primary_plane, 0,
    &bochs_primary_plane_funcs,
    bochs_primary_plane_formats,
    ARRAY_SIZE(bochs_primary_plane_formats),
    core::ptr::null_mut(),
    DRM_PLANE_TYPE_PRIMARY, core::ptr::null_mut());
    if (ret)
    return ret;
    drm_plane_helper_add(primary_plane, &bochs_primary_plane_helper_funcs);
    drm_plane_enable_fb_damage_clips(primary_plane);
    crtc = &bochs.crtc;
    ret = drm_crtc_init_with_planes(dev, crtc, primary_plane, core::ptr::null_mut(),
    &bochs_crtc_funcs, core::ptr::null_mut());
    if (ret)
    return ret;
    drm_crtc_helper_add(crtc, &bochs_crtc_helper_funcs);
    encoder = &bochs.encoder;
    ret = drm_encoder_init(dev, encoder, &bochs_encoder_funcs,
    DRM_MODE_ENCODER_VIRTUAL, core::ptr::null_mut());
    if (ret)
    return ret;
    encoder.possible_crtcs = drm_crtc_mask(crtc);
    connector = &bochs.connector;
    ret = drm_connector_init(dev, connector, &bochs_connector_funcs,
    DRM_MODE_CONNECTOR_VIRTUAL);
    if (ret)
    return ret;
    drm_connector_helper_add(connector, &bochs_connector_helper_funcs);
    drm_connector_attach_edid_property(connector);
    drm_connector_attach_encoder(connector, encoder);
    ret = drm_vblank_init(dev, 1);
    if (ret)
    return ret;
    drm_mode_config_reset(dev);
    return 0;
    }
// ----------------------------------------------------------------------
// drm interface
#[no_mangle]
unsafe extern "C" fn bochs_load(bochs: *mut bochs_device) -> c_int {
    static int bochs_load(struct bochs_device *bochs)
    {
    int ret;
    ret = bochs_hw_init(bochs);
    if (ret)
    return ret;
    ret = bochs_kms_init(bochs);
    if (ret)
    return ret;
    return 0;
    }
    DEFINE_DRM_GEM_FOPS(bochs_fops);
    static const struct drm_driver bochs_driver = {
    .driver_features	= DRIVER_GEM | DRIVER_MODESET | DRIVER_ATOMIC,
    .fops			= &bochs_fops,
    .name			= "bochs-drm",
    .desc			= "bochs dispi vga interface (qemu stdvga)",
    .major			= 1,
    .minor			= 0,
    DRM_GEM_SHMEM_DRIVER_OPS,
    DRM_FBDEV_SHMEM_DRIVER_OPS,
    };
// ----------------------------------------------------------------------
// pm interface

#[no_mangle]
unsafe extern "C" fn bochs_pm_suspend(dev: *mut device) -> c_int {
    static int bochs_pm_suspend(struct device *dev)
    {
    struct drm_device *drm_dev = dev_get_drvdata(dev);
    return drm_mode_config_helper_suspend(drm_dev);
    }
#[no_mangle]
unsafe extern "C" fn bochs_pm_resume(dev: *mut device) -> c_int {
    static int bochs_pm_resume(struct device *dev)
    {
    struct drm_device *drm_dev = dev_get_drvdata(dev);
    return drm_mode_config_helper_resume(drm_dev);
    }

    static const struct dev_pm_ops bochs_pm_ops = {
    SET_SYSTEM_SLEEP_PM_OPS(bochs_pm_suspend,
    bochs_pm_resume)
    };
// ----------------------------------------------------------------------
// pci interface
#[no_mangle]
unsafe extern "C" fn bochs_pci_probe(pdev: *mut pci_dev, ent: *const pci_device_id) -> c_int {
    static int bochs_pci_probe(struct pci_dev *pdev, const struct pci_device_id *ent)
    {
    struct bochs_device *bochs;
    struct drm_device *dev;
    int ret;
    ret = aperture_remove_conflicting_pci_devices(pdev, bochs_driver.name);
    if (ret)
    return ret;
    bochs = devm_drm_dev_alloc(&pdev.dev, &bochs_driver, struct bochs_device, dev);
    if (IS_ERR(bochs))
    return PTR_ERR(bochs);
    dev = &bochs.dev;
    ret = pcim_enable_device(pdev);
    if (ret)
    return ret;
    pci_set_drvdata(pdev, dev);
    ret = bochs_load(bochs);
    if (ret)
    return ret;
    ret = drm_dev_register(dev, 0);
    if (ret)
    return ret;
    drm_client_setup(dev, core::ptr::null_mut());
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn bochs_pci_remove(pdev: *mut pci_dev) {
    static void bochs_pci_remove(struct pci_dev *pdev)
    {
    struct drm_device *dev = pci_get_drvdata(pdev);
    drm_dev_unplug(dev);
    drm_atomic_helper_shutdown(dev);
    }
#[no_mangle]
unsafe extern "C" fn bochs_pci_shutdown(pdev: *mut pci_dev) {
    static void bochs_pci_shutdown(struct pci_dev *pdev)
    {
    drm_atomic_helper_shutdown(pci_get_drvdata(pdev));
    }
    static const struct pci_device_id bochs_pci_tbl[] = {
    {
    .vendor      = 0x1234,
    .device      = 0x1111,
    .subvendor   = PCI_SUBVENDOR_ID_REDHAT_QUMRANET,
    .subdevice   = PCI_SUBDEVICE_ID_QEMU,
    .driver_data = BOCHS_QEMU_STDVGA,
    },
    {
    .vendor      = 0x1234,
    .device      = 0x1111,
    .subvendor   = PCI_ANY_ID,
    .subdevice   = PCI_ANY_ID,
    .driver_data = BOCHS_UNKNOWN,
    },
    {
    .vendor      = 0x4321,
    .device      = 0x1111,
    .subvendor   = PCI_ANY_ID,
    .subdevice   = PCI_ANY_ID,
    .driver_data = BOCHS_SIMICS,
    },
    { /* end of list */ }
    };
    static struct pci_driver bochs_pci_driver = {
    .name =		"bochs-drm",
    .id_table =	bochs_pci_tbl,
    .probe =	bochs_pci_probe,
    .remove =	bochs_pci_remove,
    .shutdown =	bochs_pci_shutdown,
    .driver.pm =    &bochs_pm_ops,
    };
// ----------------------------------------------------------------------
// module init/exit
    drm_module_pci_driver_if_modeset(bochs_pci_driver, bochs_modeset);
    MODULE_DEVICE_TABLE(pci, bochs_pci_tbl);
    MODULE_AUTHOR("Gerd Hoffmann <kraxel@redhat.com>");
    MODULE_DESCRIPTION("DRM Support for bochs dispi vga interface (qemu stdvga)");
    MODULE_LICENSE("GPL");
