//! Automatically rewritten from C to Rust
//! Source: drivers/firmware/efi/libstub/gop.c
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
// -----------------------------------------------------------------------
//
// Copyright 2011 Intel Corporation; author Matt Fleming
//
// -----------------------------------------------------------------------

    enum efi_cmdline_option {
    EFI_CMDLINE_NONE,
    EFI_CMDLINE_MODE_NUM,
    EFI_CMDLINE_RES,
    EFI_CMDLINE_AUTO,
    EFI_CMDLINE_LIST
    };
    static struct {
    enum efi_cmdline_option option;
    union {
    u32 mode;
    struct {
    u32 width, height;
    int format;
    u8 depth;
    } res;
    };
    } cmdline = { .option = EFI_CMDLINE_NONE };
#[no_mangle]
unsafe extern "C" fn parse_modenum(option: *mut c_char, next: *mut c_char) -> bool {
    static bool parse_modenum(char *option, char **next)
    {
    u32 m;
    if (!strstarts(option, "mode="))
    return false;
    option += strlen("mode=");
    m = simple_strtoull(option, &option, 0);
    if (*option && *option++ != ',')
    return false;
    cmdline.option = EFI_CMDLINE_MODE_NUM;
    cmdline.mode   = m;
// next = option;
    return true;
    }
#[no_mangle]
unsafe extern "C" fn parse_res(option: *mut c_char, next: *mut c_char) -> bool {
    static bool parse_res(char *option, char **next)
    {
    u32 w, h, d = 0;
    let mut pf: c_int = -1;
    if (!isdigit(*option))
    return false;
    w = simple_strtoull(option, &option, 10);
    if (*option++ != 'x' || !isdigit(*option))
    return false;
    h = simple_strtoull(option, &option, 10);
    if (*option == '-') {
    option++;
    if (strstarts(option, "rgb")) {
    option += strlen("rgb");
    pf = PIXEL_RGB_RESERVED_8BIT_PER_COLOR;
    } else if (strstarts(option, "bgr")) {
    option += strlen("bgr");
    pf = PIXEL_BGR_RESERVED_8BIT_PER_COLOR;
    } else if (isdigit(*option))
    d = simple_strtoull(option, &option, 10);
    else
    return false;
    }
    if (*option && *option++ != ',')
    return false;
    cmdline.option     = EFI_CMDLINE_RES;
    cmdline.res.width  = w;
    cmdline.res.height = h;
    cmdline.res.format = pf;
    cmdline.res.depth  = d;
// next = option;
    return true;
    }
#[no_mangle]
unsafe extern "C" fn parse_auto(option: *mut c_char, next: *mut c_char) -> bool {
    static bool parse_auto(char *option, char **next)
    {
    if (!strstarts(option, "auto"))
    return false;
    option += strlen("auto");
    if (*option && *option++ != ',')
    return false;
    cmdline.option = EFI_CMDLINE_AUTO;
// next = option;
    return true;
    }
#[no_mangle]
unsafe extern "C" fn parse_list(option: *mut c_char, next: *mut c_char) -> bool {
    static bool parse_list(char *option, char **next)
    {
    if (!strstarts(option, "list"))
    return false;
    option += strlen("list");
    if (*option && *option++ != ',')
    return false;
    cmdline.option = EFI_CMDLINE_LIST;
// next = option;
    return true;
    }
#[no_mangle]
pub unsafe extern "C" fn efi_parse_option_graphics(option: *mut c_char) {
    void efi_parse_option_graphics(char *option)
    {
    while (*option) {
    if (parse_modenum(option, &option))
    continue;
    if (parse_res(option, &option))
    continue;
    if (parse_auto(option, &option))
    continue;
    if (parse_list(option, &option))
    continue;
    while (*option && *option++ != ',')
    ;
    }
    }
#[no_mangle]
unsafe extern "C" fn choose_mode_modenum(gop: *mut efi_graphics_output_protocol_t) -> u32 {
    static u32 choose_mode_modenum(efi_graphics_output_protocol_t *gop)
    {
    efi_graphics_output_mode_info_t *info __free(efi_pool) = core::ptr::null_mut();
    efi_graphics_output_protocol_mode_t *mode;
    unsigned long info_size;
    u32 max_mode, cur_mode;
    efi_status_t status;
    int pf;
    mode = efi_table_attr(gop, mode);
    cur_mode = efi_table_attr(mode, mode);
    if (cmdline.mode == cur_mode)
    return cur_mode;
    max_mode = efi_table_attr(mode, max_mode);
    if (cmdline.mode >= max_mode) {
    efi_err("Requested mode is invalid\n");
    return cur_mode;
    }
    status = efi_call_proto(gop, query_mode, cmdline.mode, &info_size, &info);
    if (status != EFI_SUCCESS) {
    efi_err("Couldn't get mode information\n");
    return cur_mode;
    }
    pf = info.pixel_format;
    if (pf == PIXEL_BLT_ONLY || pf >= PIXEL_FORMAT_MAX) {
    efi_err("Invalid PixelFormat\n");
    return cur_mode;
    }
    return cmdline.mode;
    }
    static u32 choose_mode(efi_graphics_output_protocol_t *gop,
    bool (*match)(const efi_graphics_output_mode_info_t *, u32, void *),
    void *ctx)
    {
    efi_graphics_output_protocol_mode_t *mode = efi_table_attr(gop, mode);
    let mut max_mode: u32 = efi_table_attr(mode, max_mode);
    for (u32 m = 0; m < max_mode; m++) {
    efi_graphics_output_mode_info_t *info __free(efi_pool) = core::ptr::null_mut();
    unsigned long info_size;
    efi_status_t status;
    status = efi_call_proto(gop, query_mode, m, &info_size, &info);
    if (status != EFI_SUCCESS)
    continue;
    if (match(info, m, ctx))
    return m;
    }
    return (unsigned long)ctx;
    }
#[no_mangle]
unsafe extern "C" fn pixel_bpp(pixel_format: c_int, pixel_info: efi_pixel_bitmask_t) -> u8 {
    static u8 pixel_bpp(int pixel_format, efi_pixel_bitmask_t pixel_info)
    {
    if (pixel_format == PIXEL_BIT_MASK) {
    u32 mask = pixel_info.red_mask | pixel_info.green_mask |
    pixel_info.blue_mask | pixel_info.reserved_mask;
    if (!mask)
    return 0;
    return __fls(mask) - __ffs(mask) + 1;
    } else
    return 32;
    }
#[no_mangle]
unsafe extern "C" fn match_res(info: *const efi_graphics_output_mode_info_t, mode: u32, ctx: *mut c_void) -> bool {
    static bool match_res(const efi_graphics_output_mode_info_t *info, u32 mode, void *ctx)
    {
    let mut pi: efi_pixel_bitmask_t = info.pixel_information;
    let mut pf: c_int = info.pixel_format;
    if (pf == PIXEL_BLT_ONLY || pf >= PIXEL_FORMAT_MAX)
    return false;
    return cmdline.res.width == info.horizontal_resolution &&
    cmdline.res.height == info.vertical_resolution &&
    (cmdline.res.format < 0 || cmdline.res.format == pf) &&
    (!cmdline.res.depth || cmdline.res.depth == pixel_bpp(pf, pi));
    }
#[no_mangle]
unsafe extern "C" fn choose_mode_res(gop: *mut efi_graphics_output_protocol_t) -> u32 {
    static u32 choose_mode_res(efi_graphics_output_protocol_t *gop)
    {
    efi_graphics_output_protocol_mode_t *mode = efi_table_attr(gop, mode);
    let mut cur_mode: c_ulong = efi_table_attr(mode, mode);
    if (match_res(efi_table_attr(mode, info), cur_mode, core::ptr::null_mut()))
    return cur_mode;
    return choose_mode(gop, match_res, (void *)cur_mode);
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct match {
    pub mode: u32,
    pub area: u32,
    pub depth: u8,
}

#[no_mangle]
unsafe extern "C" fn match_auto(info: *const efi_graphics_output_mode_info_t, mode: u32, ctx: *mut c_void) -> bool {
    static bool match_auto(const efi_graphics_output_mode_info_t *info, u32 mode, void *ctx)
    {
    let mut area: u32 = info.horizontal_resolution * info.vertical_resolution;
    let mut pi: efi_pixel_bitmask_t = info.pixel_information;
    let mut pf: c_int = info.pixel_format;
    let mut depth: u8 = pixel_bpp(pf, pi);
    struct match *m = ctx;
    if (pf == PIXEL_BLT_ONLY || pf >= PIXEL_FORMAT_MAX)
    return false;
    if (area > m.area || (area == m.area && depth > m.depth))
// m = (struct match){ mode, area, depth };
    return false;
    }
#[no_mangle]
unsafe extern "C" fn choose_mode_auto(gop: *mut efi_graphics_output_protocol_t) -> u32 {
    static u32 choose_mode_auto(efi_graphics_output_protocol_t *gop)
    {
    let mut match: match = {};
    choose_mode(gop, match_auto, &match);
    return match.mode;
    }
#[no_mangle]
unsafe extern "C" fn match_list(info: *const efi_graphics_output_mode_info_t, mode: u32, ctx: *mut c_void) -> bool {
    static bool match_list(const efi_graphics_output_mode_info_t *info, u32 mode, void *ctx)
    {
    let mut pi: efi_pixel_bitmask_t = info.pixel_information;
    let mut cur_mode: u32 = (unsigned long)ctx;
    let mut pf: c_int = info.pixel_format;
    const char *dstr;
    let mut depth: u8 = 0;
    bool valid;
    valid = !(pf == PIXEL_BLT_ONLY || pf >= PIXEL_FORMAT_MAX);
    switch (pf) {
    case PIXEL_RGB_RESERVED_8BIT_PER_COLOR:
    dstr = "rgb";
    break;
    case PIXEL_BGR_RESERVED_8BIT_PER_COLOR:
    dstr = "bgr";
    break;
    case PIXEL_BIT_MASK:
    dstr = "";
    depth = pixel_bpp(pf, pi);
    break;
    case PIXEL_BLT_ONLY:
    dstr = "blt";
    break;
    default:
    dstr = "xxx";
    break;
    }
    efi_printk("Mode %3u %c%c: Resolution %ux%u-%s%.0hhu\n",
    mode,
    (mode == cur_mode) ? '*' : ' ',
    !valid ? '-' : ' ',
    info.horizontal_resolution,
    info.vertical_resolution,
    dstr, depth);
    return false;
    }
#[no_mangle]
unsafe extern "C" fn choose_mode_list(gop: *mut efi_graphics_output_protocol_t) -> u32 {
    static u32 choose_mode_list(efi_graphics_output_protocol_t *gop)
    {
    efi_graphics_output_protocol_mode_t *mode = efi_table_attr(gop, mode);
    let mut cur_mode: c_ulong = efi_table_attr(mode, mode);
    let mut max_mode: u32 = efi_table_attr(mode, max_mode);
    efi_input_key_t key;
    efi_status_t status;
    efi_printk("Available graphics modes are 0-%u\n", max_mode-1);
    efi_puts("  * = current mode\n"
    "  - = unusable mode\n");
    choose_mode(gop, match_list, (void *)cur_mode);
    efi_puts("\nPress any key to continue (or wait 10 seconds)\n");
    status = efi_wait_for_key(10 * EFI_USEC_PER_SEC, &key);
    if (status != EFI_SUCCESS && status != EFI_TIMEOUT) {
    efi_err("Unable to read key, continuing in 10 seconds\n");
    efi_bs_call(stall, 10 * EFI_USEC_PER_SEC);
    }
    return cur_mode;
    }
#[no_mangle]
unsafe extern "C" fn set_mode(gop: *mut efi_graphics_output_protocol_t) {
    static void set_mode(efi_graphics_output_protocol_t *gop)
    {
    efi_graphics_output_protocol_mode_t *mode;
    u32 cur_mode, new_mode;
    switch (cmdline.option) {
    case EFI_CMDLINE_MODE_NUM:
    new_mode = choose_mode_modenum(gop);
    break;
    case EFI_CMDLINE_RES:
    new_mode = choose_mode_res(gop);
    break;
    case EFI_CMDLINE_AUTO:
    new_mode = choose_mode_auto(gop);
    break;
    case EFI_CMDLINE_LIST:
    new_mode = choose_mode_list(gop);
    break;
    default:
    return;
    }
    mode = efi_table_attr(gop, mode);
    cur_mode = efi_table_attr(mode, mode);
    if (new_mode == cur_mode)
    return;
    if (efi_call_proto(gop, set_mode, new_mode) != EFI_SUCCESS)
    efi_err("Failed to set requested mode\n");
    }
#[no_mangle]
unsafe extern "C" fn find_bits(mask: u32, pos: *mut u8, size: *mut u8) {
    static void find_bits(u32 mask, u8 *pos, u8 *size)
    {
    if (!mask) {
// pos = *size = 0;
    return;
    }
// UEFI spec guarantees that the set bits are contiguous
// pos  = __ffs(mask);
// size = __fls(mask) - *pos + 1;
    }
#[no_mangle]
unsafe extern "C" fn setup_screen_info(si: *mut screen_info, gop: *const efi_graphics_output_protocol_t) {
    static void setup_screen_info(struct screen_info *si, const efi_graphics_output_protocol_t *gop)
    {
    const efi_graphics_output_protocol_mode_t *mode = efi_table_attr(gop, mode);
    const efi_graphics_output_mode_info_t *info = efi_table_attr(mode, info);
    si.orig_video_isVGA = VIDEO_TYPE_EFI;
    si.lfb_width  = info.horizontal_resolution;
    si.lfb_height = info.vertical_resolution;
    efi_set_u64_split(efi_table_attr(mode, frame_buffer_base),
    &si.lfb_base, &si.ext_lfb_base);
    if (si.ext_lfb_base)
    si.capabilities |= VIDEO_CAPABILITY_64BIT_BASE;
    si.pages = 1;
    if (info.pixel_format == PIXEL_BIT_MASK) {
    find_bits(info.pixel_information.red_mask, &si.red_pos, &si.red_size);
    find_bits(info.pixel_information.green_mask, &si.green_pos, &si.green_size);
    find_bits(info.pixel_information.blue_mask, &si.blue_pos, &si.blue_size);
    find_bits(info.pixel_information.reserved_mask, &si.rsvd_pos, &si.rsvd_size);
    si.lfb_depth = si.red_size + si.green_size + si.blue_size + si.rsvd_size;
    si.lfb_linelength = (info.pixels_per_scan_line * si.lfb_depth) / 8;
    } else {
    if (info.pixel_format == PIXEL_RGB_RESERVED_8BIT_PER_COLOR) {
    si.red_pos   = 0;
    si.blue_pos  = 16;
    } else /* PIXEL_BGR_RESERVED_8BIT_PER_COLOR */ {
    si.blue_pos  = 0;
    si.red_pos   = 16;
    }
    si.green_pos = 8;
    si.rsvd_pos  = 24;
    si.red_size = 8;
    si.green_size = 8;
    si.blue_size = 8;
    si.rsvd_size = 8;
    si.lfb_depth = 32;
    si.lfb_linelength = info.pixels_per_scan_line * 4;
    }
    si.lfb_size = si.lfb_linelength * si.lfb_height;
    si.capabilities |= VIDEO_CAPABILITY_SKIP_QUIRKS;
    }
#[no_mangle]
unsafe extern "C" fn setup_edid_info(edid: *mut edid_info, gop_size_of_edid: u32, gop_edid: *mut u8) {
    static void setup_edid_info(struct edid_info *edid, u32 gop_size_of_edid, u8 *gop_edid)
    {
    if (!gop_edid || gop_size_of_edid < 128)
    memset(edid.dummy, 0, sizeof(edid.dummy));
    else
    memcpy(edid.dummy, gop_edid, min(gop_size_of_edid, sizeof(edid.dummy)));
    }
    static efi_handle_t find_handle_with_primary_gop(unsigned long num, const efi_handle_t handles[],
    efi_graphics_output_protocol_t **found_gop)
    {
    efi_graphics_output_protocol_t *first_gop;
    efi_handle_t h, first_gop_handle;
    first_gop_handle = core::ptr::null_mut();
    first_gop = core::ptr::null_mut();
    for_each_efi_handle(h, handles, num) {
    efi_status_t status;
    efi_graphics_output_protocol_t *gop;
    efi_graphics_output_protocol_mode_t *mode;
    efi_graphics_output_mode_info_t *info;
    void *dummy = core::ptr::null_mut();
    status = efi_bs_call(handle_protocol, h,
    &EFI_GRAPHICS_OUTPUT_PROTOCOL_GUID,
    (void **)&gop);
    if (status != EFI_SUCCESS)
    continue;
    mode = efi_table_attr(gop, mode);
    info = efi_table_attr(mode, info);
    if (info.pixel_format == PIXEL_BLT_ONLY ||
    info.pixel_format >= PIXEL_FORMAT_MAX)
    continue;
//
// Systems that use the UEFI Console Splitter may
// provide multiple GOP devices, not all of which are
// backed by real hardware. The workaround is to search
// for a GOP implementing the ConOut protocol, and if
// one isn't found, to just fall back to the first GOP.
//
// Once we've found a GOP supporting ConOut,
// don't bother looking any further.
//
    status = efi_bs_call(handle_protocol, h,
    &EFI_CONSOLE_OUT_DEVICE_GUID, &dummy);
    if (status == EFI_SUCCESS) {
    if (found_gop)
// found_gop = gop;
    return h;
    } else if (!first_gop_handle) {
    first_gop_handle = h;
    first_gop = gop;
    }
    }
    if (found_gop)
// found_gop = first_gop;
    return first_gop_handle;
    }
#[no_mangle]
pub unsafe extern "C" fn efi_setup_graphics(si: *mut screen_info, edid: *mut edid_info) -> efi_status_t {
    efi_status_t efi_setup_graphics(struct screen_info *si, struct edid_info *edid)
    {
    efi_handle_t *handles __free(efi_pool) = core::ptr::null_mut();
    efi_handle_t handle;
    efi_graphics_output_protocol_t *gop;
    efi_status_t status;
    unsigned long num;
    status = efi_bs_call(locate_handle_buffer, EFI_LOCATE_BY_PROTOCOL,
    &EFI_GRAPHICS_OUTPUT_PROTOCOL_GUID, core::ptr::null_mut(), &num,
    &handles);
    if (status != EFI_SUCCESS)
    return status;
    handle = find_handle_with_primary_gop(num, handles, &gop);
    if (!handle)
    return EFI_NOT_FOUND;
// Change mode if requested
    set_mode(gop);
// EFI framebuffer
    if (si)
    setup_screen_info(si, gop);
// Display EDID for primary GOP
    if (edid) {
    efi_edid_discovered_protocol_t *discovered_edid;
    efi_edid_active_protocol_t *active_edid;
    let mut gop_size_of_edid: u32 = 0;
    u8 *gop_edid = core::ptr::null_mut();
    status = efi_bs_call(handle_protocol, handle, &EFI_EDID_ACTIVE_PROTOCOL_GUID,
    (void **)&active_edid);
    if (status == EFI_SUCCESS) {
    gop_size_of_edid = efi_table_attr(active_edid, size_of_edid);
    gop_edid = efi_table_attr(active_edid, edid);
    } else {
    status = efi_bs_call(handle_protocol, handle,
    &EFI_EDID_DISCOVERED_PROTOCOL_GUID,
    (void **)&discovered_edid);
    if (status == EFI_SUCCESS) {
    gop_size_of_edid = efi_table_attr(discovered_edid, size_of_edid);
    gop_edid = efi_table_attr(discovered_edid, edid);
    }
    }
    setup_edid_info(edid, gop_size_of_edid, gop_edid);
    }
    return EFI_SUCCESS;
    }
