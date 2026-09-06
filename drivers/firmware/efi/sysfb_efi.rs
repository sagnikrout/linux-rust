//! Automatically rewritten from C to Rust
//! Source: drivers/firmware/efi/sysfb_efi.c
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
// Generic System Framebuffers
// Copyright (c) 2012-2013 David Herrmann <dh.herrmann@gmail.com>
//
// EFI Quirks Copyright (c) 2006 Edgar Hucek <gimli@dark-green.com>
//
// EFI Quirks
// Several EFI systems do not correctly advertise their boot framebuffers.
// Hence, we use this static table of known broken machines and fix up the
// information so framebuffer drivers can load correctly.
//

    enum {
    OVERRIDE_NONE = 0x0,
    OVERRIDE_BASE = 0x1,
    OVERRIDE_STRIDE = 0x2,
    OVERRIDE_HEIGHT = 0x4,
    OVERRIDE_WIDTH = 0x8,
    };
    struct efifb_dmi_info efifb_dmi_list[] = {
    [M_I17] = { "i17", 0x80010000, 1472 * 4, 1440, 900, OVERRIDE_NONE },
    [M_I20] = { "i20", 0x80010000, 1728 * 4, 1680, 1050, OVERRIDE_NONE }, /* guess */
    [M_I20_SR] = { "imac7", 0x40010000, 1728 * 4, 1680, 1050, OVERRIDE_NONE },
    [M_I24] = { "i24", 0x80010000, 2048 * 4, 1920, 1200, OVERRIDE_NONE }, /* guess */
    [M_I24_8_1] = { "imac8", 0xc0060000, 2048 * 4, 1920, 1200, OVERRIDE_NONE },
    [M_I24_10_1] = { "imac10", 0xc0010000, 2048 * 4, 1920, 1080, OVERRIDE_NONE },
    [M_I27_11_1] = { "imac11", 0xc0010000, 2560 * 4, 2560, 1440, OVERRIDE_NONE },
    [M_MINI]= { "mini", 0x80000000, 2048 * 4, 1024, 768, OVERRIDE_NONE },
    [M_MINI_3_1] = { "mini31", 0x40010000, 1024 * 4, 1024, 768, OVERRIDE_NONE },
    [M_MINI_4_1] = { "mini41", 0xc0010000, 2048 * 4, 1920, 1200, OVERRIDE_NONE },
    [M_MB] = { "macbook", 0x80000000, 2048 * 4, 1280, 800, OVERRIDE_NONE },
    [M_MB_5_1] = { "macbook51", 0x80010000, 2048 * 4, 1280, 800, OVERRIDE_NONE },
    [M_MB_6_1] = { "macbook61", 0x80010000, 2048 * 4, 1280, 800, OVERRIDE_NONE },
    [M_MB_7_1] = { "macbook71", 0x80010000, 2048 * 4, 1280, 800, OVERRIDE_NONE },
    [M_MBA] = { "mba", 0x80000000, 2048 * 4, 1280, 800, OVERRIDE_NONE },
// 11" Macbook Air 3,1 passes the wrong stride
    [M_MBA_3] = { "mba3", 0, 2048 * 4, 0, 0, OVERRIDE_STRIDE },
    [M_MBP] = { "mbp", 0x80010000, 1472 * 4, 1440, 900, OVERRIDE_NONE },
    [M_MBP_2] = { "mbp2", 0, 0, 0, 0, OVERRIDE_NONE }, /* placeholder */
    [M_MBP_2_2] = { "mbp22", 0x80010000, 1472 * 4, 1440, 900, OVERRIDE_NONE },
    [M_MBP_SR] = { "mbp3", 0x80030000, 2048 * 4, 1440, 900, OVERRIDE_NONE },
    [M_MBP_4] = { "mbp4", 0xc0060000, 2048 * 4, 1920, 1200, OVERRIDE_NONE },
    [M_MBP_5_1] = { "mbp51", 0xc0010000, 2048 * 4, 1440, 900, OVERRIDE_NONE },
    [M_MBP_5_2] = { "mbp52", 0xc0010000, 2048 * 4, 1920, 1200, OVERRIDE_NONE },
    [M_MBP_5_3] = { "mbp53", 0xd0010000, 2048 * 4, 1440, 900, OVERRIDE_NONE },
    [M_MBP_6_1] = { "mbp61", 0x90030000, 2048 * 4, 1920, 1200, OVERRIDE_NONE },
    [M_MBP_6_2] = { "mbp62", 0x90030000, 2048 * 4, 1680, 1050, OVERRIDE_NONE },
    [M_MBP_7_1] = { "mbp71", 0xc0010000, 2048 * 4, 1280, 800, OVERRIDE_NONE },
    [M_MBP_8_2] = { "mbp82", 0x90010000, 1472 * 4, 1440, 900, OVERRIDE_NONE },
    [M_UNKNOWN] = { core::ptr::null_mut(), 0, 0, 0, 0, OVERRIDE_NONE }
    };
#[no_mangle]
pub unsafe extern "C" fn efifb_setup_from_dmi(si: *mut screen_info, opt: *const c_char) {
    void efifb_setup_from_dmi(struct screen_info *si, const char *opt)
    {
    int i;
    for (i = 0; i < M_UNKNOWN; i++) {
    if (efifb_dmi_list[i].base != 0 &&
    !strcmp(opt, efifb_dmi_list[i].optname)) {
    si.lfb_base = efifb_dmi_list[i].base;
    si.lfb_linelength = efifb_dmi_list[i].stride;
    si.lfb_width = efifb_dmi_list[i].width;
    si.lfb_height = efifb_dmi_list[i].height;
    }
    }
    }

    typeof(fwvalue) _ret_ = fwvalue;		\
    if ((flags) & (field))				\
    _ret_ = dmivalue;			\
    else if ((fwvalue) == 0)			\
    _ret_ = dmivalue;			\
    _ret_;						\
    })

#[no_mangle]
unsafe extern "C" fn efifb_set_system(si: *mut screen_info, id: *const dmi_system_id) -> int __init {
    static int __init efifb_set_system(struct screen_info *si, const struct dmi_system_id *id)
    {
    struct efifb_dmi_info *info = id.driver_data;
    if (info.base == 0 && info.height == 0 && info.width == 0 &&
    info.stride == 0)
    return 0;
// Trust the bootloader over the DMI tables
    if (si.lfb_base == 0) {

    struct pci_dev *dev = core::ptr::null_mut();
    let mut found_bar: c_int = 0;

    if (info.base) {
    si.lfb_base = choose_value(info.base,
    si.lfb_base, OVERRIDE_BASE,
    info.flags);

// make sure that the address in the table is actually
// on a VGA device's PCI BAR
    for_each_pci_dev(dev) {
    int i;
    if ((dev.class >> 8) != PCI_CLASS_DISPLAY_VGA)
    continue;
    for (i = 0; i < DEVICE_COUNT_RESOURCE; i++) {
    resource_size_t start, end;
    unsigned long flags;
    flags = pci_resource_flags(dev, i);
    if (!(flags & IORESOURCE_MEM))
    continue;
    if (flags & IORESOURCE_UNSET)
    continue;
    if (pci_resource_len(dev, i) == 0)
    continue;
    start = pci_resource_start(dev, i);
    end = pci_resource_end(dev, i);
    if (si.lfb_base >= start && si.lfb_base < end) {
    found_bar = 1;
    break;
    }
    }
    }
    if (!found_bar)
    si.lfb_base = 0;

    }
    }
    if (si.lfb_base) {
    si.lfb_linelength = choose_value(info.stride,
    si.lfb_linelength, OVERRIDE_STRIDE,
    info.flags);
    si.lfb_width = choose_value(info.width,
    si.lfb_width, OVERRIDE_WIDTH,
    info.flags);
    si.lfb_height = choose_value(info.height,
    si.lfb_height, OVERRIDE_HEIGHT,
    info.flags);
    if (si.orig_video_isVGA == 0)
    si.orig_video_isVGA = VIDEO_TYPE_EFI;
    } else {
    si.lfb_linelength = 0;
    si.lfb_width = 0;
    si.lfb_height = 0;
    si.orig_video_isVGA = 0;
    return 0;
    }
    printk(KERN_INFO "efifb: dmi detected %s - framebuffer at 0x%08x "
    "(%dx%d, stride %d)\n", id.ident,
    si.lfb_base, si.lfb_width,
    si.lfb_height, si.lfb_linelength);
    return 1;
    }
#[no_mangle]
unsafe extern "C" fn efifb_set_system_callback(id: *const dmi_system_id) -> int __init {
    static int __init efifb_set_system_callback(const struct dmi_system_id *id)
    {
    return efifb_set_system(&sysfb_primary_display.screen, id);
    }

    {							\
    efifb_set_system_callback,			\
    name,						\
    {						\
    DMI_MATCH(DMI_BIOS_VENDOR, vendor),	\
    DMI_MATCH(DMI_PRODUCT_NAME, name)	\
    },						\
    &efifb_dmi_list[enumid]				\
    }
    static const struct dmi_system_id efifb_dmi_system_table[] __initconst = {
    EFIFB_DMI_SYSTEM_ID("Apple Computer, Inc.", "iMac4,1", M_I17),
// At least one of these two will be right; maybe both?
    EFIFB_DMI_SYSTEM_ID("Apple Computer, Inc.", "iMac5,1", M_I20),
    EFIFB_DMI_SYSTEM_ID("Apple Inc.", "iMac5,1", M_I20),
// At least one of these two will be right; maybe both?
    EFIFB_DMI_SYSTEM_ID("Apple Computer, Inc.", "iMac6,1", M_I24),
    EFIFB_DMI_SYSTEM_ID("Apple Inc.", "iMac6,1", M_I24),
    EFIFB_DMI_SYSTEM_ID("Apple Inc.", "iMac7,1", M_I20_SR),
    EFIFB_DMI_SYSTEM_ID("Apple Inc.", "iMac8,1", M_I24_8_1),
    EFIFB_DMI_SYSTEM_ID("Apple Inc.", "iMac10,1", M_I24_10_1),
    EFIFB_DMI_SYSTEM_ID("Apple Inc.", "iMac11,1", M_I27_11_1),
    EFIFB_DMI_SYSTEM_ID("Apple Computer, Inc.", "Macmini1,1", M_MINI),
    EFIFB_DMI_SYSTEM_ID("Apple Inc.", "Macmini3,1", M_MINI_3_1),
    EFIFB_DMI_SYSTEM_ID("Apple Inc.", "Macmini4,1", M_MINI_4_1),
    EFIFB_DMI_SYSTEM_ID("Apple Computer, Inc.", "MacBook1,1", M_MB),
// At least one of these two will be right; maybe both?
    EFIFB_DMI_SYSTEM_ID("Apple Computer, Inc.", "MacBook2,1", M_MB),
    EFIFB_DMI_SYSTEM_ID("Apple Inc.", "MacBook2,1", M_MB),
// At least one of these two will be right; maybe both?
    EFIFB_DMI_SYSTEM_ID("Apple Computer, Inc.", "MacBook3,1", M_MB),
    EFIFB_DMI_SYSTEM_ID("Apple Inc.", "MacBook3,1", M_MB),
    EFIFB_DMI_SYSTEM_ID("Apple Inc.", "MacBook4,1", M_MB),
    EFIFB_DMI_SYSTEM_ID("Apple Inc.", "MacBook5,1", M_MB_5_1),
    EFIFB_DMI_SYSTEM_ID("Apple Inc.", "MacBook6,1", M_MB_6_1),
    EFIFB_DMI_SYSTEM_ID("Apple Inc.", "MacBook7,1", M_MB_7_1),
    EFIFB_DMI_SYSTEM_ID("Apple Inc.", "MacBookAir1,1", M_MBA),
    EFIFB_DMI_SYSTEM_ID("Apple Inc.", "MacBookAir3,1", M_MBA_3),
    EFIFB_DMI_SYSTEM_ID("Apple Computer, Inc.", "MacBookPro1,1", M_MBP),
    EFIFB_DMI_SYSTEM_ID("Apple Computer, Inc.", "MacBookPro2,1", M_MBP_2),
    EFIFB_DMI_SYSTEM_ID("Apple Computer, Inc.", "MacBookPro2,2", M_MBP_2_2),
    EFIFB_DMI_SYSTEM_ID("Apple Inc.", "MacBookPro2,1", M_MBP_2),
    EFIFB_DMI_SYSTEM_ID("Apple Computer, Inc.", "MacBookPro3,1", M_MBP_SR),
    EFIFB_DMI_SYSTEM_ID("Apple Inc.", "MacBookPro3,1", M_MBP_SR),
    EFIFB_DMI_SYSTEM_ID("Apple Inc.", "MacBookPro4,1", M_MBP_4),
    EFIFB_DMI_SYSTEM_ID("Apple Inc.", "MacBookPro5,1", M_MBP_5_1),
    EFIFB_DMI_SYSTEM_ID("Apple Inc.", "MacBookPro5,2", M_MBP_5_2),
    EFIFB_DMI_SYSTEM_ID("Apple Inc.", "MacBookPro5,3", M_MBP_5_3),
    EFIFB_DMI_SYSTEM_ID("Apple Inc.", "MacBookPro6,1", M_MBP_6_1),
    EFIFB_DMI_SYSTEM_ID("Apple Inc.", "MacBookPro6,2", M_MBP_6_2),
    EFIFB_DMI_SYSTEM_ID("Apple Inc.", "MacBookPro7,1", M_MBP_7_1),
    EFIFB_DMI_SYSTEM_ID("Apple Inc.", "MacBookPro8,2", M_MBP_8_2),
    {},
    };
#[no_mangle]
unsafe extern "C" fn efifb_swap_width_height(id: *const dmi_system_id) -> int __init {
    static int __init efifb_swap_width_height(const struct dmi_system_id *id)
    {
    struct screen_info *si = &sysfb_primary_display.screen;
    let mut bpp: u32 = __screen_info_lfb_bits_per_pixel(si);
    swap(si.lfb_width, si.lfb_height);
    si.lfb_linelength = bpp * si.lfb_width / BITS_PER_BYTE;
    return 1;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct efifb_mode_fixup {
    pub width: c_uint,
    pub height: c_uint,
    pub linelength: c_uint,
}

    static int __init
    efifb_check_and_swap_width_height(const struct dmi_system_id *id)
    {
    const struct efifb_mode_fixup *data = id.driver_data;
    struct screen_info *si = &sysfb_primary_display.screen;
    if (data.width == si.lfb_width && data.height == si.lfb_height) {
    swap(si.lfb_width, si.lfb_height);
    si.lfb_linelength = data.linelength;
    si.lfb_size = data.linelength * data.width;
    }
    return 1;
    }
    static const struct efifb_mode_fixup efifb_steamdeck_mode_fixup __initconst = {
    .width = 1280,
    .height = 800,
    .linelength = 3328,
    };
//
// Some devices have a portrait LCD but advertise a landscape resolution (and
// pitch). We simply swap width and height for these devices so that we can
// correctly deal with some of them coming with multiple resolutions.
//
    static const struct dmi_system_id efifb_dmi_swap_width_height[] __initconst = {
    {
//
// Lenovo MIIX310-10ICR, only some batches have the troublesome
// 800x1280 portrait screen. Luckily the portrait version has
// its own BIOS version, so we match on that.
//
    .matches = {
    DMI_EXACT_MATCH(DMI_SYS_VENDOR, "LENOVO"),
    DMI_EXACT_MATCH(DMI_PRODUCT_VERSION, "MIIX 310-10ICR"),
    DMI_EXACT_MATCH(DMI_BIOS_VERSION, "1HCN44WW"),
    },
    .callback = efifb_swap_width_height,
    },
    {
// Lenovo MIIX 320-10ICR with 800x1280 portrait screen
    .matches = {
    DMI_EXACT_MATCH(DMI_SYS_VENDOR, "LENOVO"),
    DMI_EXACT_MATCH(DMI_PRODUCT_VERSION,
    "Lenovo MIIX 320-10ICR"),
    },
    .callback = efifb_swap_width_height,
    },
    {
// Lenovo D330 with 800x1280 or 1200x1920 portrait screen
    .matches = {
    DMI_EXACT_MATCH(DMI_SYS_VENDOR, "LENOVO"),
    DMI_EXACT_MATCH(DMI_PRODUCT_VERSION,
    "Lenovo ideapad D330-10IGM"),
    },
    .callback = efifb_swap_width_height,
    },
    {
//
// Lenovo IdeaPad Duet 3 10IGL5 and 10IGL5-LTE with
// 1200x1920 portrait screen
//
    .matches = {
    DMI_EXACT_MATCH(DMI_SYS_VENDOR, "LENOVO"),
// Non exact match to also match the LTE version
    DMI_MATCH(DMI_PRODUCT_VERSION, "IdeaPad Duet 3 10IGL5"),
    },
    .callback = efifb_swap_width_height,
    },
    {
// Lenovo Yoga Book X91F / X91L
    .matches = {
    DMI_EXACT_MATCH(DMI_SYS_VENDOR, "LENOVO"),
// Non exact match to match F + L versions
    DMI_MATCH(DMI_PRODUCT_NAME, "Lenovo YB1-X91"),
    },
    .callback = efifb_swap_width_height,
    },
    {
// Valve Steam Deck (Jupiter)
    .matches = {
    DMI_EXACT_MATCH(DMI_SYS_VENDOR, "Valve"),
    DMI_EXACT_MATCH(DMI_PRODUCT_NAME, "Jupiter"),
    DMI_EXACT_MATCH(DMI_PRODUCT_VERSION, "1"),
    },
    .callback = efifb_check_and_swap_width_height,
    .driver_data = (void *)&efifb_steamdeck_mode_fixup,
    },
    {
// Valve Steam Deck (Galileo)
    .matches = {
    DMI_EXACT_MATCH(DMI_SYS_VENDOR, "Valve"),
    DMI_EXACT_MATCH(DMI_PRODUCT_NAME, "Galileo"),
    DMI_EXACT_MATCH(DMI_PRODUCT_VERSION, "1"),
    },
    .callback = efifb_check_and_swap_width_height,
    .driver_data = (void *)&efifb_steamdeck_mode_fixup,
    },
    {},
    };
    static bool efifb_overlaps_pci_range(const struct screen_info *si,
    const struct of_pci_range *range)
    {
    let mut fb_base: u64 = si.lfb_base;
    if (si.capabilities & VIDEO_CAPABILITY_64BIT_BASE)
    fb_base |= (u64)(unsigned long)si.ext_lfb_base << 32;
    return fb_base >= range.cpu_addr &&
    fb_base < (range.cpu_addr + range.size);
    }
    static struct device_node *find_pci_overlap_node(void)
    {
    struct device_node *np;
    for_each_node_by_type(np, "pci") {
    struct of_pci_range_parser parser;
    struct of_pci_range range;
    int err;
    err = of_pci_range_parser_init(&parser, np);
    if (err) {
    pr_warn("of_pci_range_parser_init() failed: %d\n", err);
    continue;
    }
    for_each_of_pci_range(&parser, &range)
    if (efifb_overlaps_pci_range(&sysfb_primary_display.screen, &range))
    return np;
    }
    return core::ptr::null_mut();
    }
//
// If the efifb framebuffer is backed by a PCI graphics controller, we have
// to ensure that this relation is expressed using a device link when
// running in DT mode, or the probe order may be reversed, resulting in a
// resource reservation conflict on the memory window that the efifb
// framebuffer steals from the PCIe host bridge.
//
#[no_mangle]
unsafe extern "C" fn efifb_add_links(fwnode: *mut fwnode_handle) -> c_int {
    static int efifb_add_links(struct fwnode_handle *fwnode)
    {
    struct device_node *sup_np;
    sup_np = find_pci_overlap_node();
//
// If there's no PCI graphics controller backing the efifb, we are
// done here.
//
    if (!sup_np)
    return 0;
    fwnode_link_add(fwnode, of_fwnode_handle(sup_np), 0);
    of_node_put(sup_np);
    return 0;
    }
    static const struct fwnode_operations efifb_fwnode_ops = {
    .add_links = efifb_add_links,
    };
    static struct fwnode_handle efifb_fwnode;
#[no_mangle]
pub unsafe extern "C" fn sysfb_apply_efi_quirks(si: *mut screen_info) -> __init void {
    __init void sysfb_apply_efi_quirks(struct screen_info *si)
    {
    if (si.orig_video_isVGA != VIDEO_TYPE_EFI ||
    !(si.capabilities & VIDEO_CAPABILITY_SKIP_QUIRKS))
    dmi_check_system(efifb_dmi_system_table);
    if (si.orig_video_isVGA == VIDEO_TYPE_EFI)
    dmi_check_system(efifb_dmi_swap_width_height);
    }
#[no_mangle]
pub unsafe extern "C" fn sysfb_set_efifb_fwnode(si: *const screen_info, pd: *mut platform_device) -> __init void {
    __init void sysfb_set_efifb_fwnode(const struct screen_info *si, struct platform_device *pd)
    {
    if (si.orig_video_isVGA == VIDEO_TYPE_EFI && IS_ENABLED(CONFIG_PCI)) {
    fwnode_init(&efifb_fwnode, &efifb_fwnode_ops);
    pd.dev.fwnode = &efifb_fwnode;
    }
    }
