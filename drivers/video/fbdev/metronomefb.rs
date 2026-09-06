//! Automatically rewritten from C to Rust
//! Source: drivers/video/fbdev/metronomefb.c
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


//
// linux/drivers/video/metronomefb.c -- FB driver for Metronome controller
//
// Copyright (C) 2008, Jaya Kumar
//
// This file is subject to the terms and conditions of the GNU General Public
// License. See the file COPYING in the main directory of this archive for
// more details.
//
// Layout is based on skeletonfb.c by James Simmons and Geert Uytterhoeven.
//
// This work was made possible by help and equipment support from E-Ink
// Corporation. https://www.eink.com
//
// This driver is written to be used with the Metronome display controller.
// It is intended to be architecture independent. A board specific driver
// must be used to perform all the physical IO interactions. An example
// is provided as am200epd.c
//

// Display specific information
pub const DPY_W: c_int = 832;
pub const DPY_H: c_int = 622;
    static int user_wfm_size;
// frame differs from image. frame includes non-visible pixels
#[repr(C)]
#[derive(Copy, Clone)]
pub struct epd_frame {
    pub /: *mut *mut int fw; / frame width,
    pub /: *mut *mut int fh; / frame height,
    pub config: [u16; 4],
    pub wfm_size: c_int,
}

    static struct epd_frame epd_frame_table[] = {
    {
    .fw = 832,
    .fh = 622,
    .config = {
    15 /* sdlew */
    | 2 << 8 /* sdosz */
    | 0 << 11 /* sdor */
    | 0 << 12 /* sdces */
    | 0 << 15, /* sdcer */
    42 /* gdspl */
    | 1 << 8 /* gdr1 */
    | 1 << 9 /* sdshr */
    | 0 << 15, /* gdspp */
    18 /* gdspw */
    | 0 << 15, /* dispc */
    599 /* vdlc */
    | 0 << 11 /* dsi */
    | 0 << 12, /* dsic */
    },
    .wfm_size = 47001,
    },
    {
    .fw = 1088,
    .fh = 791,
    .config = {
    0x0104,
    0x031f,
    0x0088,
    0x02ff,
    },
    .wfm_size = 46770,
    },
    {
    .fw = 1200,
    .fh = 842,
    .config = {
    0x0101,
    0x030e,
    0x0012,
    0x0280,
    },
    .wfm_size = 46770,
    },
    };
    static struct fb_fix_screeninfo metronomefb_fix = {
    .id =		"metronomefb",
    .type =		FB_TYPE_PACKED_PIXELS,
    .visual =	FB_VISUAL_STATIC_PSEUDOCOLOR,
    .xpanstep =	0,
    .ypanstep =	0,
    .ywrapstep =	0,
    .line_length =	DPY_W,
    .accel =	FB_ACCEL_NONE,
    };
    static struct fb_var_screeninfo metronomefb_var = {
    .xres		= DPY_W,
    .yres		= DPY_H,
    .xres_virtual	= DPY_W,
    .yres_virtual	= DPY_H,
    .bits_per_pixel	= 8,
    .grayscale	= 1,
    .nonstd		= 1,
    .red =		{ 4, 3, 0 },
    .green =	{ 0, 0, 0 },
    .blue =		{ 0, 0, 0 },
    .transp =	{ 0, 0, 0 },
    };
// the waveform structure that is coming from userspace firmware
#[repr(C)]
#[derive(Copy, Clone)]
pub struct waveform_hdr {
    pub stuff: [u8; 32],
    pub wmta: [u8; 3],
    pub fvsn: u8,
    pub luts: u8,
    pub mc: u8,
    pub trc: u8,
    pub stuff3: u8,
    pub endb: u8,
    pub swtb: u8,
    pub stuff2a: [u8; 2],
    pub stuff2b: [u8; 3],
    pub wfm_cs: u8,
// C attribute field omitted
// main metronomefb functions
#[no_mangle]
unsafe extern "C" fn calc_cksum(start: c_int, end: c_int, mem: *mut u8) -> u8 {
    static u8 calc_cksum(int start, int end, u8 *mem)
    {
    pub 0: u8 tmp =,
    pub i: c_int,
    pub i++): for (i = start; i < end;,
    pub mem: [tmp +=; i],
    pub tmp: return,
    }
#[no_mangle]
unsafe extern "C" fn calc_img_cksum(start: *mut u16, length: c_int) -> u16 {
    static u16 calc_img_cksum(u16 *start, int length)
    {
    pub 0: u16 tmp =,
    while (length--)
    pub start++: *mut tmp +=,
    pub tmp: return,
    }
// here we decode the incoming waveform file and populate metromem
    static int load_waveform(u8 *mem, size_t size, int m, int t,
    struct metronomefb_par *par)
    {
    pub tta: c_int,
    pub wmta: c_int,
    pub 0: int trn =,
    pub i: c_int,
    pub v: c_uchar,
    pub cksum: u8,
    pub cksum_idx: c_int,
    pub owfm_idx: int wfm_idx,,
    pub 0: int mem_idx =,
    pub wfm_hdr: *mut waveform_hdr,
    pub par->metromem_wfm: *mut *mut u8 metromem =,
    pub par->info->device: *mut *mut device dev =,
    if (user_wfm_size)
    pub user_wfm_size: epd_frame_table[par->dt].wfm_size =,
    if (size != epd_frame_table[par.dt].wfm_size) {
    dev_err(dev, "Error: unexpected size %zd != %d\n", size,
    pub -EINVAL: return,
    }
    pub mem: *mut *mut wfm_hdr = (struct waveform_hdr ),
    if (wfm_hdr.fvsn != 1) {
    pub wfm_hdr->fvsn): dev_err(dev, "Error: bad fvsn %x\n",,
    pub -EINVAL: return,
    }
    if (wfm_hdr.luts != 0) {
    pub wfm_hdr->luts): dev_err(dev, "Error: bad luts %x\n",,
    pub -EINVAL: return,
    }
    pub mem): cksum = calc_cksum(32, 47,,
    if (cksum != wfm_hdr.wfm_cs) {
    dev_err(dev, "Error: bad cksum %x != %x\n", cksum,
    pub -EINVAL: return,
    }
    pub 1: wfm_hdr->mc +=,
    pub 1: wfm_hdr->trc +=,
    pub {: for (i = 0; i < 5; i++),
    if (*(wfm_hdr.stuff2a + i) != 0) {
    pub padding\n"): dev_err(dev, "Error: unexpected value in,
    pub -EINVAL: return,
    }
    }
// calculating trn. trn is something used to index into
    the waveform. presumably selecting the right one for the
    desired temperature. it works out the offset of the first
    v that exceeds the specified temperature */
    if ((sizeof(*wfm_hdr) + wfm_hdr.trc) > size)
    pub -EINVAL: return,
    pub {: *mut *mut *mut for (i = sizeof(wfm_hdr); i <= sizeof(wfm_hdr) + wfm_hdr->trc; i++),
    if (mem[i] > t) {
    pub 1: *mut *mut trn = i - sizeof(wfm_hdr) -,
    }
    }
// check temperature range table checksum
    pub 1: *mut *mut cksum_idx = sizeof(wfm_hdr) + wfm_hdr->trc +,
    if (cksum_idx >= size)
    pub -EINVAL: return,
    pub mem): *mut *mut cksum = calc_cksum(sizeof(wfm_hdr), cksum_idx,,
    if (cksum != mem[cksum_idx]) {
    dev_err(dev, "Error: bad temperature range table cksum"
    pub mem[cksum_idx]): " %x != %x\n", cksum,,
    pub -EINVAL: return,
    }
// check waveform mode table address checksum
    pub 0x00FFFFFF: wmta = get_unaligned_le32(wfm_hdr->wmta) &,
    pub 3: *mut *mut cksum_idx = wmta + m4 +,
    if (cksum_idx >= size)
    pub -EINVAL: return,
    pub mem): cksum = calc_cksum(cksum_idx - 3, cksum_idx,,
    if (cksum != mem[cksum_idx]) {
    dev_err(dev, "Error: bad mode table address cksum"
    pub mem[cksum_idx]): " %x != %x\n", cksum,,
    pub -EINVAL: return,
    }
// check waveform temperature table address checksum
    pub 0x00FFFFFF: *mut *mut tta = get_unaligned_le32(mem + wmta + m  4) &,
    pub 3: *mut *mut cksum_idx = tta + trn4 +,
    if (cksum_idx >= size)
    pub -EINVAL: return,
    pub mem): cksum = calc_cksum(cksum_idx - 3, cksum_idx,,
    if (cksum != mem[cksum_idx]) {
    dev_err(dev, "Error: bad temperature table address cksum"
    pub mem[cksum_idx]): " %x != %x\n", cksum,,
    pub -EINVAL: return,
    }
// here we do the real work of putting the waveform into the
    metromem buffer. this does runlength decoding of the waveform */
    pub 0x00FFFFFF: *mut *mut wfm_idx = get_unaligned_le32(mem + tta + trn  4) &,
    pub wfm_idx: owfm_idx =,
    if (wfm_idx >= size)
    pub -EINVAL: return,
    while (wfm_idx < size) {
    pub rl: c_uchar,
    pub mem: [v =; wfm_idx++],
    if (v == wfm_hdr.swtb) {
    while (((v = mem[wfm_idx++]) != wfm_hdr.swtb) &&
    wfm_idx < size)
    pub v: metromem[mem_idx++] =,
    }
    if (v == wfm_hdr.endb)
    pub mem: [rl =; wfm_idx++],
    pub i++): for (i = 0; i <= rl;,
    pub v: metromem[mem_idx++] =,
    }
    pub wfm_idx: cksum_idx =,
    if (cksum_idx >= size)
    pub -EINVAL: return,
    pub mem): cksum = calc_cksum(owfm_idx, cksum_idx,,
    if (cksum != mem[cksum_idx]) {
    dev_err(dev, "Error: bad waveform data cksum"
    pub mem[cksum_idx]): " %x != %x\n", cksum,,
    pub -EINVAL: return,
    }
    pub (mem_idx/64): par->frame_count =,
    pub 0: return,
    }
#[no_mangle]
unsafe extern "C" fn metronome_display_cmd(par: *mut metronomefb_par) -> c_int {
    static int metronome_display_cmd(struct metronomefb_par *par)
    {
    pub i: c_int,
    pub cs: u16,
    pub opcode: u16,
    pub borderval: static u8,
// setup display command
    we can't immediately set the opcode since the controller
    will try parse the command before we've set it all up
    so we just set cs here and set the opcode at the end */
    if (par.metromem_cmd.opcode == 0xCC40)
    pub 0xCC41: opcode = cs =,
    else
    pub 0xCC40: opcode = cs =,
// set the args ( 2 bytes ) for display
    pub 0: i =,
    par.metromem_cmd.args[i] = 	1 << 3 /* border update */
    | ((borderval++ % 4) & 0x0F) << 4
    pub 8: | (par->frame_count - 1) <<,
    pub par->metromem_cmd->args[i++]: cs +=,
// the rest are 0
    pub (32-i)*2): *mut *mut memset((u8 ) (par->metromem_cmd->args + i), 0,,
    pub cs: par->metromem_cmd->csum =,
    pub /: *mut *mut par->metromem_cmd->opcode = opcode; / display cmd,
    pub par->board->met_wait_event_intr(par): return,
    }
#[no_mangle]
unsafe extern "C" fn metronome_powerup_cmd(par: *mut metronomefb_par) -> c_int {
    static int metronome_powerup_cmd(struct metronomefb_par *par)
    {
    pub i: c_int,
    pub cs: u16,
// setup power up command
    pub /: *mut *mut par->metromem_cmd->opcode = 0x1234; / pwr up pseudo cmd,
    pub par->metromem_cmd->opcode: cs =,
// set pwr1,2,3 to 1024
    pub {: for (i = 0; i < 3; i++),
    pub 1024: par->metromem_cmd->args[i] =,
    pub par->metromem_cmd->args[i]: cs +=,
    }
// the rest are 0
    memset(&par.metromem_cmd.args[i], 0,
    pub 2): *mut *mut (ARRAY_SIZE(par->metromem_cmd->args) - i),
    pub cs: par->metromem_cmd->csum =,
    pub 1): par->board->set_rst(par,,
    pub 1): par->board->set_stdby(par,,
    pub par->board->met_wait_event(par): return,
    }
#[no_mangle]
unsafe extern "C" fn metronome_config_cmd(par: *mut metronomefb_par) -> c_int {
    static int metronome_config_cmd(struct metronomefb_par *par)
    {
// setup config command
    we can't immediately set the opcode since the controller
    will try parse the command before we've set it all up */
    memcpy(par.metromem_cmd.args, epd_frame_table[par.dt].config,
// the rest are 0
    memset(&par.metromem_cmd.args[4], 0,
    pub 2): *mut *mut (ARRAY_SIZE(par->metromem_cmd->args) - 4),
    pub 0xCC10: par->metromem_cmd->csum =,
    pub 4): par->metromem_cmd->csum += calc_img_cksum(par->metromem_cmd->args,,
    pub /: *mut *mut par->metromem_cmd->opcode = 0xCC10; / config cmd,
    pub par->board->met_wait_event(par): return,
    }
#[no_mangle]
unsafe extern "C" fn metronome_init_cmd(par: *mut metronomefb_par) -> c_int {
    static int metronome_init_cmd(struct metronomefb_par *par)
    {
    pub i: c_int,
    pub cs: u16,
// setup init command
    we can't immediately set the opcode since the controller
    will try parse the command before we've set it all up
    so we just set cs here and set the opcode at the end */
    pub 0xCC20: cs =,
// set the args ( 2 bytes ) for init
    pub 0: i =,
    pub 0: par->metromem_cmd->args[i] =,
    pub par->metromem_cmd->args[i++]: cs +=,
// the rest are 0
    pub (32-i)*2): *mut *mut memset((u8 ) (par->metromem_cmd->args + i), 0,,
    pub cs: par->metromem_cmd->csum =,
    pub /: *mut *mut par->metromem_cmd->opcode = 0xCC20; / init cmd,
    pub par->board->met_wait_event(par): return,
    }
#[no_mangle]
unsafe extern "C" fn metronome_init_regs(par: *mut metronomefb_par) -> c_int {
    static int metronome_init_regs(struct metronomefb_par *par)
    {
    pub res: c_int,
    pub par->board->setup_io(par): res =,
    if (res)
    pub res: return,
    pub metronome_powerup_cmd(par): res =,
    if (res)
    pub res: return,
    pub metronome_config_cmd(par): res =,
    if (res)
    pub res: return,
    pub metronome_init_cmd(par): res =,
    pub res: return,
    }
#[no_mangle]
unsafe extern "C" fn metronomefb_dpy_update(par: *mut metronomefb_par) {
    static void metronomefb_dpy_update(struct metronomefb_par *par)
    {
    pub fbsize: c_int,
    pub cksum: u16,
    pub par->info->screen_buffer: *mut *mut unsigned char buf =,
    pub par->info->fix.smem_len: fbsize =,
// copy from vm to metromem
    pub fbsize): memcpy(par->metromem_img, buf,,
    pub fbsize/2): *mut *mut cksum = calc_img_cksum((u16 ) par->metromem_img,,
// ((u16 *)(par->metromem_img) + fbsize/2) = cksum;
    }
#[no_mangle]
unsafe extern "C" fn metronomefb_dpy_update_page(par: *mut metronomefb_par, index: c_int) -> u16 {
    static u16 metronomefb_dpy_update_page(struct metronomefb_par *par, int index)
    {
    pub i: c_int,
    pub 0: u16 csum =,
    pub index): *mut *mut *mut u16 buf = (u16 )(par->info->screen_buffer +,
    pub index): *mut *mut *mut u16 img = (u16 )(par->metromem_img +,
// swizzle from vm to metromem and recalc cksum at the same time
    pub {: for (i = 0; i < PAGE_SIZE/2; i++),
// (img + i) = (buf[i] << 5) & 0xE0E0;
    pub i): *mut *mut csum += (img +,
    }
    pub csum: return,
    }
// this is called back from the deferred io workqueue
#[no_mangle]
unsafe extern "C" fn metronomefb_dpy_deferred_io(info: *mut fb_info, pagereflist: *mut list_head) {
    static void metronomefb_dpy_deferred_io(struct fb_info *info, struct list_head *pagereflist)
    {
    pub cksum: u16,
    pub pageref: *mut fb_deferred_io_pageref,
    pub info->par: *mut *mut metronomefb_par par =,
// walk the written page list and swizzle the data
    list_for_each_entry(pageref, pagereflist, list) {
    pub PAGE_SHIFT: unsigned long pgoffset = pageref->offset >>,
    pub pageref->offset): cksum = metronomefb_dpy_update_page(par,,
    pub par->csum_table[pgoffset]: par->metromem_img_csum -=,
    pub cksum: par->csum_table[pgoffset] =,
    pub cksum: par->metromem_img_csum +=,
    }
    }
#[no_mangle]
unsafe extern "C" fn metronomefb_defio_damage_range(info: *mut fb_info, off: off_t, len: usize) {
    static void metronomefb_defio_damage_range(struct fb_info *info, off_t off, size_t len)
    {
    pub info->par: *mut *mut metronomefb_par par =,
    }
    static void metronomefb_defio_damage_area(struct fb_info *info, u32 x, u32 y,
    u32 width, u32 height)
    {
    pub info->par: *mut *mut metronomefb_par par =,
    }
    FB_GEN_DEFAULT_DEFERRED_SYSMEM_OPS(metronomefb,
    metronomefb_defio_damage_range,
    metronomefb_defio_damage_area)
    static const struct fb_ops metronomefb_ops = {
    .owner	= THIS_MODULE,
    FB_DEFAULT_DEFERRED_OPS(metronomefb),
}

    static struct fb_deferred_io metronomefb_defio = {
    .delay			= HZ,
    .sort_pagereflist	= true,
    .deferred_io		= metronomefb_dpy_deferred_io,
    };
#[no_mangle]
unsafe extern "C" fn metronomefb_probe(dev: *mut platform_device) -> c_int {
    static int metronomefb_probe(struct platform_device *dev)
    {
    struct fb_info *info;
    struct metronome_board *board;
    let mut retval: c_int = -ENOMEM;
    int videomemorysize;
    unsigned char *videomemory;
    struct metronomefb_par *par;
    const struct firmware *fw_entry;
    int i;
    int panel_type;
    int fw, fh;
    int epd_dt_index;
// pick up board specific routines
    board = dev.dev.platform_data;
    if (!board)
    return -EINVAL;
// try to count device specific driver, if can't, platform recalls
    if (!try_module_get(board.owner))
    return -ENODEV;
    info = framebuffer_alloc(sizeof(struct metronomefb_par), &dev.dev);
    if (!info)
    goto err;
// we have two blocks of memory.
    info.screen_buffer which is vm, and is the fb used by apps.
    par.metromem which is physically contiguous memory and
    contains the display controller commands, waveform,
    processed image data and padding. this is the data pulled
    by the device's LCD controller and pushed to Metronome.
    the metromem memory is allocated by the board driver and
    is provided to us */
    panel_type = board.get_panel_type();
    switch (panel_type) {
    case 6:
    epd_dt_index = 0;
    break;
    case 8:
    epd_dt_index = 1;
    break;
    case 97:
    epd_dt_index = 2;
    break;
    default:
    dev_err(&dev.dev, "Unexpected panel type. Defaulting to 6\n");
    epd_dt_index = 0;
    break;
    }
    fw = epd_frame_table[epd_dt_index].fw;
    fh = epd_frame_table[epd_dt_index].fh;
// we need to add a spare page because our csum caching scheme walks
// to the end of the page
    videomemorysize = PAGE_SIZE + (fw * fh);
    videomemory = vzalloc(videomemorysize);
    if (!videomemory)
    goto err_fb_rel;
    info.screen_buffer = videomemory;
    info.fbops = &metronomefb_ops;
    metronomefb_fix.line_length = fw;
    metronomefb_var.xres = fw;
    metronomefb_var.yres = fh;
    metronomefb_var.xres_virtual = fw;
    metronomefb_var.yres_virtual = fh;
    info.var = metronomefb_var;
    info.fix = metronomefb_fix;
    info.fix.smem_len = videomemorysize;
    par = info.par;
    par.info = info;
    par.board = board;
    par.dt = epd_dt_index;
    init_waitqueue_head(&par.waitq);
// this table caches per page csum values.
    par.csum_table = vmalloc(videomemorysize/PAGE_SIZE);
    if (!par.csum_table)
    goto err_vfree;
// the physical framebuffer that we use is setup by
// the platform device driver. It will provide us
// with cmd, wfm and image memory in a contiguous area.
    retval = board.setup_fb(par);
    if (retval) {
    dev_err(&dev.dev, "Failed to setup fb\n");
    goto err_csum_table;
    }
// after this point we should have a framebuffer
    if ((!par.metromem_wfm) ||  (!par.metromem_img) ||
    (!par.metromem_dma)) {
    dev_err(&dev.dev, "fb access failure\n");
    retval = -EINVAL;
    goto err_csum_table;
    }
    info.fix.smem_start = par.metromem_dma;
// load the waveform in. assume mode 3, temp 31 for now
    a) request the waveform file from userspace
    b) process waveform and decode into metromem */
    retval = request_firmware(&fw_entry, "metronome.wbf", &dev.dev);
    if (retval < 0) {
    dev_err(&dev.dev, "Failed to get waveform\n");
    goto err_csum_table;
    }
    retval = load_waveform((u8 *) fw_entry.data, fw_entry.size, 3, 31,
    par);
    release_firmware(fw_entry);
    if (retval < 0) {
    dev_err(&dev.dev, "Failed processing waveform\n");
    goto err_csum_table;
    }
    retval = board.setup_irq(info);
    if (retval)
    goto err_csum_table;
    retval = metronome_init_regs(par);
    if (retval < 0)
    goto err_free_irq;
    info.flags = FBINFO_VIRTFB;
    info.fbdefio = &metronomefb_defio;
    retval = fb_deferred_io_init(info);
    if (retval)
    goto err_free_irq;
    retval = fb_alloc_cmap(&info.cmap, 8, 0);
    if (retval < 0) {
    dev_err(&dev.dev, "Failed to allocate colormap\n");
    goto err_fbdefio;
    }
// set cmap
    for (i = 0; i < 8; i++)
    info.cmap.red[i] = (((2*i)+1)*(0xFFFF))/16;
    memcpy(info.cmap.green, info.cmap.red, sizeof(u16)*8);
    memcpy(info.cmap.blue, info.cmap.red, sizeof(u16)*8);
    retval = register_framebuffer(info);
    if (retval < 0)
    goto err_cmap;
    platform_set_drvdata(dev, info);
    dev_dbg(&dev.dev,
    "fb%d: Metronome frame buffer device, using %dK of video"
    " memory\n", info.node, videomemorysize >> 10);
    return 0;
    err_cmap:
    fb_dealloc_cmap(&info.cmap);
    err_fbdefio:
    fb_deferred_io_cleanup(info);
    err_free_irq:
    board.cleanup(par);
    err_csum_table:
    vfree(par.csum_table);
    err_vfree:
    vfree(videomemory);
    err_fb_rel:
    framebuffer_release(info);
    err:
    module_put(board.owner);
    return retval;
    }
#[no_mangle]
unsafe extern "C" fn metronomefb_remove(dev: *mut platform_device) {
    static void metronomefb_remove(struct platform_device *dev)
    {
    struct fb_info *info = platform_get_drvdata(dev);
    if (info) {
    struct metronomefb_par *par = info.par;
    unregister_framebuffer(info);
    fb_deferred_io_cleanup(info);
    fb_dealloc_cmap(&info.cmap);
    par.board.cleanup(par);
    vfree(par.csum_table);
    vfree(info.screen_buffer);
    module_put(par.board.owner);
    dev_dbg(&dev.dev, "calling release\n");
    framebuffer_release(info);
    }
    }
    static struct platform_driver metronomefb_driver = {
    .probe	= metronomefb_probe,
    .remove	= metronomefb_remove,
    .driver	= {
    .name	= "metronomefb",
    },
    };
    module_platform_driver(metronomefb_driver);
    module_param(user_wfm_size, uint, 0);
    MODULE_PARM_DESC(user_wfm_size, "Set custom waveform size");
    MODULE_DESCRIPTION("fbdev driver for Metronome controller");
    MODULE_AUTHOR("Jaya Kumar");
    MODULE_LICENSE("GPL");
    MODULE_FIRMWARE("metronome.wbf");
