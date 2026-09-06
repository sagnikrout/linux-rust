//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/kernel/btext.c
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
// Procedures for drawing on the screen early on in the boot process.
//
// Benjamin Herrenschmidt <benh@kernel.crashing.org>
//

// Macro flag: #define NO_SCROLL

    static void scrollscreen(void);

    static int g_loc_X _data;
    static int g_loc_Y _data;
    static int g_max_loc_X _data;
    static int g_max_loc_Y _data;
    static int dispDeviceRowBytes _data;
    static int dispDeviceDepth  _data;
    static int dispDeviceRect[4] _data;
    static unsigned char *dispDeviceBase _data;
    static unsigned char *logicalDisplayBase _data;
    unsigned long disp_BAT[2] __initdata = {0, 0};
    static int boot_text_mapped _data;
    extern void rmci_on(void);
    extern void rmci_off(void);
#[no_mangle]
pub unsafe extern "C" fn rmci_maybe_on() {
    static inline void rmci_maybe_on(void)
    {

    if (!(mfmsr() & MSR_DR))
    rmci_on();

    }
#[no_mangle]
pub unsafe extern "C" fn rmci_maybe_off() {
    static inline void rmci_maybe_off(void)
    {

    if (!(mfmsr() & MSR_DR))
    rmci_off();

    }

// Calc BAT values for mapping the display and store them
// in disp_BAT.  Those values are then used from head.S to map
// the display during identify_machine() and MMU_Init()
//
// The display is mapped to virtual address 0xD0000000, rather
// than 1:1, because some CHRP machines put the frame buffer
// in the region starting at 0xC0000000 (PAGE_OFFSET).
// This mapping is temporary and will disappear as soon as the
// setup done by MMU_Init() is applied.
//
// For now, we align the BAT and then map 8Mb on 601 and 16Mb
// on other PPCs. This may cause trouble if the framebuffer
// is really badly aligned, but I didn't encounter this case
// yet.
//
#[no_mangle]
pub unsafe extern "C" fn btext_prepare_BAT() -> void __init {
    void __init btext_prepare_BAT(void)
    {
    let mut vaddr: c_ulong = PAGE_OFFSET + 0x10000000;
    unsigned long addr;
    unsigned long lowbits;
    addr = (unsigned long)dispDeviceBase;
    if (!addr) {
    boot_text_mapped = 0;
    return;
    }
    lowbits = addr & ~0xFF000000UL;
    addr &= 0xFF000000UL;
    disp_BAT[0] = vaddr | (BL_16M<<2) | 2;
    disp_BAT[1] = addr | (_PAGE_NO_CACHE | _PAGE_GUARDED | BPP_RW);
    logicalDisplayBase = (void *) (vaddr + lowbits);
    }

// This function can be used to enable the early boot text when doing
// OF booting or within bootx init. It must be followed by a btext_unmap()
// call before the logical address becomes unusable
//
    void __init btext_setup_display(int width, int height, int depth, int pitch,
    unsigned long address)
    {
    g_loc_X = 0;
    g_loc_Y = 0;
    g_max_loc_X = width / 8;
    g_max_loc_Y = height / 16;
    logicalDisplayBase = (unsigned char *)address;
    dispDeviceBase = (unsigned char *)address;
    dispDeviceRowBytes = pitch;
    dispDeviceDepth = depth == 15 ? 16 : depth;
    dispDeviceRect[0] = dispDeviceRect[1] = 0;
    dispDeviceRect[2] = width;
    dispDeviceRect[3] = height;
    boot_text_mapped = 1;
    }
#[no_mangle]
pub unsafe extern "C" fn btext_unmap() -> void __init {
    void __init btext_unmap(void)
    {
    boot_text_mapped = 0;
    }
// Here's a small text engine to use during early boot
// or for debugging purposes
//
// todo:
//
// - build some kind of vgacon with it to enable early printk
// - move to a separate file
// - add a few video driver hooks to keep in sync with display
// changes.
//
#[no_mangle]
pub unsafe extern "C" fn btext_map() {
    void btext_map(void)
    {
    unsigned long base, offset, size;
    unsigned char *vbase;
// By default, we are no longer mapped
    boot_text_mapped = 0;
    if (!dispDeviceBase)
    return;
    base = ((unsigned long) dispDeviceBase) & 0xFFFFF000UL;
    offset = ((unsigned long) dispDeviceBase) - base;
    size = dispDeviceRowBytes * dispDeviceRect[3] + offset
    + dispDeviceRect[0];
    vbase = ioremap_wc(base, size);
    if (!vbase)
    return;
    logicalDisplayBase = vbase + offset;
    boot_text_mapped = 1;
    }
#[no_mangle]
unsafe extern "C" fn btext_initialize(np: *mut device_node) -> int __init {
    static int __init btext_initialize(struct device_node *np)
    {
    unsigned int width, height, depth, pitch;
    let mut address: c_ulong = 0;
    const u32 *prop;
    prop = of_get_property(np, "linux,bootx-width", core::ptr::null_mut());
    if (prop == core::ptr::null_mut())
    prop = of_get_property(np, "width", core::ptr::null_mut());
    if (prop == core::ptr::null_mut())
    return -EINVAL;
    width = *prop;
    prop = of_get_property(np, "linux,bootx-height", core::ptr::null_mut());
    if (prop == core::ptr::null_mut())
    prop = of_get_property(np, "height", core::ptr::null_mut());
    if (prop == core::ptr::null_mut())
    return -EINVAL;
    height = *prop;
    prop = of_get_property(np, "linux,bootx-depth", core::ptr::null_mut());
    if (prop == core::ptr::null_mut())
    prop = of_get_property(np, "depth", core::ptr::null_mut());
    if (prop == core::ptr::null_mut())
    return -EINVAL;
    depth = *prop;
    pitch = width * ((depth + 7) / 8);
    prop = of_get_property(np, "linux,bootx-linebytes", core::ptr::null_mut());
    if (prop == core::ptr::null_mut())
    prop = of_get_property(np, "linebytes", core::ptr::null_mut());
    if (prop && *prop != 0xffffffffu)
    pitch = *prop;
    if (pitch == 1)
    pitch = 0x1000;
    prop = of_get_property(np, "linux,bootx-addr", core::ptr::null_mut());
    if (prop == core::ptr::null_mut())
    prop = of_get_property(np, "address", core::ptr::null_mut());
    if (prop)
    address = *prop;
// FIXME: Add support for PCI reg properties. Right now, only
// reliable on macs
//
    if (address == 0)
    return -EINVAL;
    g_loc_X = 0;
    g_loc_Y = 0;
    g_max_loc_X = width / 8;
    g_max_loc_Y = height / 16;
    dispDeviceBase = (unsigned char *)address;
    dispDeviceRowBytes = pitch;
    dispDeviceDepth = depth == 15 ? 16 : depth;
    dispDeviceRect[0] = dispDeviceRect[1] = 0;
    dispDeviceRect[2] = width;
    dispDeviceRect[3] = height;
    btext_map();
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn btext_find_display(allow_nonstdout: c_int) -> int __init {
    int __init btext_find_display(int allow_nonstdout)
    {
    struct device_node *np = of_stdout;
    let mut rc: c_int = -ENODEV;
    if (!of_node_is_type(np, "display")) {
    printk("boot stdout isn't a display !\n");
    np = core::ptr::null_mut();
    }
    if (np)
    rc = btext_initialize(np);
    if (rc == 0 || !allow_nonstdout)
    return rc;
    for_each_node_by_type(np, "display") {
    if (of_property_read_bool(np, "linux,opened")) {
    printk("trying %pOF ...\n", np);
    rc = btext_initialize(np);
    printk("result: %d\n", rc);
    }
    if (rc == 0) {
    of_node_put(np);
    break;
    }
    }
    return rc;
    }
// Calc the base address of a given point (x,y)
#[no_mangle]
unsafe extern "C" fn calc_base(x: c_int, y: c_int) -> *mut c_uchar {
    static unsigned char * calc_base(int x, int y)
    {
    unsigned char *base;
    base = logicalDisplayBase;
    if (!base)
    base = dispDeviceBase;
    base += (x + dispDeviceRect[0]) * (dispDeviceDepth >> 3);
    base += (y + dispDeviceRect[1]) * dispDeviceRowBytes;
    return base;
    }
// Adjust the display to a new resolution
    void btext_update_display(unsigned long phys, int width, int height,
    int depth, int pitch)
    {
    if (!dispDeviceBase)
    return;
// check it's the same frame buffer (within 256MB)
    if ((phys ^ (unsigned long)dispDeviceBase) & 0xf0000000)
    return;
    dispDeviceBase = (__u8 *) phys;
    dispDeviceRect[0] = 0;
    dispDeviceRect[1] = 0;
    dispDeviceRect[2] = width;
    dispDeviceRect[3] = height;
    dispDeviceDepth = depth;
    dispDeviceRowBytes = pitch;
    if (boot_text_mapped) {
    iounmap(logicalDisplayBase);
    boot_text_mapped = 0;
    }
    btext_map();
    g_loc_X = 0;
    g_loc_Y = 0;
    g_max_loc_X = width / 8;
    g_max_loc_Y = height / 16;
    }
    EXPORT_SYMBOL(btext_update_display);
#[no_mangle]
pub unsafe extern "C" fn btext_clearscreen() -> void __init {
    void __init btext_clearscreen(void)
    {
    unsigned int *base	= (unsigned int *)calc_base(0, 0);
    unsigned long width 	= ((dispDeviceRect[2] - dispDeviceRect[0]) *
    (dispDeviceDepth >> 3)) >> 2;
    int i,j;
    rmci_maybe_on();
    for (i=0; i<(dispDeviceRect[3] - dispDeviceRect[1]); i++)
    {
    unsigned int *ptr = base;
    for(j=width; j; --j)
// (ptr++) = 0;
    base += (dispDeviceRowBytes >> 2);
    }
    rmci_maybe_off();
    }
#[no_mangle]
pub unsafe extern "C" fn btext_flushscreen() -> void __init {
    void __init btext_flushscreen(void)
    {
    unsigned int *base	= (unsigned int *)calc_base(0, 0);
    unsigned long width 	= ((dispDeviceRect[2] - dispDeviceRect[0]) *
    (dispDeviceDepth >> 3)) >> 2;
    int i,j;
    for (i=0; i < (dispDeviceRect[3] - dispDeviceRect[1]); i++)
    {
    unsigned int *ptr = base;
    for(j = width; j > 0; j -= 8) {
    __asm__ __volatile__ ("dcbst 0,%0" :: "r" (ptr));
    ptr += 8;
    }
    base += (dispDeviceRowBytes >> 2);
    }
    __asm__ __volatile__ ("sync" ::: "memory");
    }
#[no_mangle]
pub unsafe extern "C" fn btext_flushline() -> void __init {
    void __init btext_flushline(void)
    {
    unsigned int *base	= (unsigned int *)calc_base(0, g_loc_Y << 4);
    unsigned long width 	= ((dispDeviceRect[2] - dispDeviceRect[0]) *
    (dispDeviceDepth >> 3)) >> 2;
    int i,j;
    for (i=0; i < 16; i++)
    {
    unsigned int *ptr = base;
    for(j = width; j > 0; j -= 8) {
    __asm__ __volatile__ ("dcbst 0,%0" :: "r" (ptr));
    ptr += 8;
    }
    base += (dispDeviceRowBytes >> 2);
    }
    __asm__ __volatile__ ("sync" ::: "memory");
    }

#[no_mangle]
unsafe extern "C" fn scrollscreen() {
    static void scrollscreen(void)
    {
    unsigned int *src     	= (unsigned int *)calc_base(0,16);
    unsigned int *dst     	= (unsigned int *)calc_base(0,0);
    unsigned long width    	= ((dispDeviceRect[2] - dispDeviceRect[0]) *
    (dispDeviceDepth >> 3)) >> 2;
    int i,j;
    rmci_maybe_on();
    for (i=0; i<(dispDeviceRect[3] - dispDeviceRect[1] - 16); i++)
    {
    unsigned int *src_ptr = src;
    unsigned int *dst_ptr = dst;
    for(j=width; j; --j)
// (dst_ptr++) = *(src_ptr++);
    src += (dispDeviceRowBytes >> 2);
    dst += (dispDeviceRowBytes >> 2);
    }
    for (i=0; i<16; i++)
    {
    unsigned int *dst_ptr = dst;
    for(j=width; j; --j)
// (dst_ptr++) = 0;
    dst += (dispDeviceRowBytes >> 2);
    }
    rmci_maybe_off();
    }

    static unsigned int expand_bits_8[16] = {
    0x00000000,
    0x000000ff,
    0x0000ff00,
    0x0000ffff,
    0x00ff0000,
    0x00ff00ff,
    0x00ffff00,
    0x00ffffff,
    0xff000000,
    0xff0000ff,
    0xff00ff00,
    0xff00ffff,
    0xffff0000,
    0xffff00ff,
    0xffffff00,
    0xffffffff
    };
    static unsigned int expand_bits_16[4] = {
    0x00000000,
    0x0000ffff,
    0xffff0000,
    0xffffffff
    };
#[no_mangle]
unsafe extern "C" fn draw_byte_32(font: *const c_uchar, base: *mut c_uint, rb: c_int) {
    static void draw_byte_32(const unsigned char *font, unsigned int *base, int rb)
    {
    int l, bits;
    let mut fg: c_int = 0xFFFFFFFFUL;
    let mut bg: c_int = 0x00000000UL;
    for (l = 0; l < 16; ++l)
    {
    bits = *font++;
    base[0] = (-(bits >> 7) & fg) ^ bg;
    base[1] = (-((bits >> 6) & 1) & fg) ^ bg;
    base[2] = (-((bits >> 5) & 1) & fg) ^ bg;
    base[3] = (-((bits >> 4) & 1) & fg) ^ bg;
    base[4] = (-((bits >> 3) & 1) & fg) ^ bg;
    base[5] = (-((bits >> 2) & 1) & fg) ^ bg;
    base[6] = (-((bits >> 1) & 1) & fg) ^ bg;
    base[7] = (-(bits & 1) & fg) ^ bg;
    base = (unsigned int *) ((char *)base + rb);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn draw_byte_16(font: *const c_uchar, base: *mut c_uint, rb: c_int) {
    static inline void draw_byte_16(const unsigned char *font, unsigned int *base, int rb)
    {
    int l, bits;
    let mut fg: c_int = 0xFFFFFFFFUL;
    let mut bg: c_int = 0x00000000UL;
    unsigned int *eb = (int *)expand_bits_16;
    for (l = 0; l < 16; ++l)
    {
    bits = *font++;
    base[0] = (eb[bits >> 6] & fg) ^ bg;
    base[1] = (eb[(bits >> 4) & 3] & fg) ^ bg;
    base[2] = (eb[(bits >> 2) & 3] & fg) ^ bg;
    base[3] = (eb[bits & 3] & fg) ^ bg;
    base = (unsigned int *) ((char *)base + rb);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn draw_byte_8(font: *const c_uchar, base: *mut c_uint, rb: c_int) {
    static inline void draw_byte_8(const unsigned char *font, unsigned int *base, int rb)
    {
    int l, bits;
    let mut fg: c_int = 0x0F0F0F0FUL;
    let mut bg: c_int = 0x00000000UL;
    unsigned int *eb = (int *)expand_bits_8;
    for (l = 0; l < 16; ++l)
    {
    bits = *font++;
    base[0] = (eb[bits >> 4] & fg) ^ bg;
    base[1] = (eb[bits & 0xf] & fg) ^ bg;
    base = (unsigned int *) ((char *)base + rb);
    }
    }
#[no_mangle]
unsafe extern "C" fn draw_byte(c: c_uchar, locX: c_long, locY: c_long) -> noinline void {
    static noinline void draw_byte(unsigned char c, long locX, long locY)
    {
    unsigned char *base	= calc_base(locX << 3, locY << 4);
    let mut font_index: c_uint = c * 16;
    const unsigned char *font = PTRRELOC(font_sun_8x16.data) + font_index;
    let mut rb: c_int = dispDeviceRowBytes;
    rmci_maybe_on();
    switch(dispDeviceDepth) {
    case 24:
    case 32:
    draw_byte_32(font, (unsigned int *)base, rb);
    break;
    case 15:
    case 16:
    draw_byte_16(font, (unsigned int *)base, rb);
    break;
    case 8:
    draw_byte_8(font, (unsigned int *)base, rb);
    break;
    }
    rmci_maybe_off();
    }
#[no_mangle]
pub unsafe extern "C" fn btext_drawchar(c: c_char) {
    void btext_drawchar(char c)
    {
    let mut cline: c_int = 0;

    int x;

    if (!boot_text_mapped)
    return;
    switch (c) {
    case '\b':
    if (g_loc_X > 0)
    --g_loc_X;
    break;
    case '\t':
    g_loc_X = (g_loc_X & -8) + 8;
    break;
    case '\r':
    g_loc_X = 0;
    break;
    case '\n':
    g_loc_X = 0;
    g_loc_Y++;
    cline = 1;
    break;
    default:
    draw_byte(c, g_loc_X++, g_loc_Y);
    }
    if (g_loc_X >= g_max_loc_X) {
    g_loc_X = 0;
    g_loc_Y++;
    cline = 1;
    }

    while (g_loc_Y >= g_max_loc_Y) {
    scrollscreen();
    g_loc_Y--;
    }

// wrap around from bottom to top of screen so we don't
    waste time scrolling each line.  -- paulus. */
    if (g_loc_Y >= g_max_loc_Y)
    g_loc_Y = 0;
    if (cline) {
    for (x = 0; x < g_max_loc_X; ++x)
    draw_byte(' ', x, g_loc_Y);
    }

    }
#[no_mangle]
pub unsafe extern "C" fn btext_drawstring(c: *const c_char) {
    void btext_drawstring(const char *c)
    {
    if (!boot_text_mapped)
    return;
    while (*c)
    btext_drawchar(*c++);
    }
#[no_mangle]
pub unsafe extern "C" fn btext_drawtext(c: *const c_char, len: c_uint) -> void __init {
    void __init btext_drawtext(const char *c, unsigned int len)
    {
    if (!boot_text_mapped)
    return;
    while (len--)
    btext_drawchar(*c++);
    }
#[no_mangle]
pub unsafe extern "C" fn btext_drawhex(v: c_ulong) -> void __init {
    void __init btext_drawhex(unsigned long v)
    {
    if (!boot_text_mapped)
    return;

    btext_drawchar(hex_asc_hi(v >> 56));
    btext_drawchar(hex_asc_lo(v >> 56));
    btext_drawchar(hex_asc_hi(v >> 48));
    btext_drawchar(hex_asc_lo(v >> 48));
    btext_drawchar(hex_asc_hi(v >> 40));
    btext_drawchar(hex_asc_lo(v >> 40));
    btext_drawchar(hex_asc_hi(v >> 32));
    btext_drawchar(hex_asc_lo(v >> 32));

    btext_drawchar(hex_asc_hi(v >> 24));
    btext_drawchar(hex_asc_lo(v >> 24));
    btext_drawchar(hex_asc_hi(v >> 16));
    btext_drawchar(hex_asc_lo(v >> 16));
    btext_drawchar(hex_asc_hi(v >> 8));
    btext_drawchar(hex_asc_lo(v >> 8));
    btext_drawchar(hex_asc_hi(v));
    btext_drawchar(hex_asc_lo(v));
    btext_drawchar(' ');
    }
#[no_mangle]
pub unsafe extern "C" fn udbg_init_btext() -> void __init {
    void __init udbg_init_btext(void)
    {
// If btext is enabled, we might have a BAT setup for early display,
// thus we do enable some very basic udbg output
//
    udbg_putc = btext_drawchar;
    }
