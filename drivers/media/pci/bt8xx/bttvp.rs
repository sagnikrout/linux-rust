//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/pci/bt8xx/bttvp.h
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

pub const FORMAT_FLAGS_DITHER: c_uint = 0x01;
pub const FORMAT_FLAGS_PACKED: c_uint = 0x02;
pub const FORMAT_FLAGS_PLANAR: c_uint = 0x04;
pub const FORMAT_FLAGS_RAW: c_uint = 0x08;
pub const FORMAT_FLAGS_CrCb: c_uint = 0x10;
pub const RISC_SLOT_O_VBI: c_int = 4;
pub const RISC_SLOT_O_FIELD: c_int = 6;
pub const RISC_SLOT_E_VBI: c_int = 10;
pub const RISC_SLOT_E_FIELD: c_int = 12;
pub const RISC_SLOT_LOOP: c_int = 14;
pub const RESOURCE_VIDEO_STREAM: c_int = 2;
pub const RESOURCE_VBI: c_int = 4;
pub const RESOURCE_VIDEO_READ: c_int = 8;
pub const RAW_LINES: c_int = 640;
pub const RAW_BPL: c_int = 1024;

// Min. value in VDELAY register.
pub const MIN_VDELAY: c_int = 2;
// Even to get Cb first, odd for Cr.

// Limits scaled width, which must be a multiple of 4.

// ----------------------------------------------------------
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bttv_tvnorm {
    pub v4l2_id: c_int,
    pub name: *mut c_char,
    pub Fsc: u32,
    pub /: *mut *mut u16 swidth, sheight; / scaled standard width, height,
    pub totalwidth: u16,
    pub iform: u8 adelay, bdelay,,
    pub scaledtwidth: u32,
    pub hactivex1: u16 hdelayx1,,
    pub vdelay: u16,
    pub vbipack: u8,
    pub vtotal: u16,
    pub sram: c_int,
// ITU-R frame line number of the first VBI line we can
    pub vbistart: [u16; 2],
// Horizontally this counts fCLKx1 samples following the leading
    pub cropcap: v4l2_cropcap,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bttv_format {
    pub /: *mut *mut int fourcc; / video4linux 2,
    pub /: *mut *mut *mut int btformat; / BT848_COLOR_FMT_,
    pub /: *mut *mut *mut int btswap; / BT848_COLOR_CTL_,
    pub /: *mut *mut int depth; / bit/pixel,
    pub flags: c_int,
    pub /: *mut *mut int hshift,vshift; / for planar modes,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bttv_ir {
    pub dev: *mut rc_dev,
    pub btv: *mut bttv,
    pub timer: timer_list,
    pub name: [c_char; 32],
    pub phys: [c_char; 32],
// Usual gpio signalling
    pub mask_keycode: u32,
    pub mask_keydown: u32,
    pub mask_keyup: u32,
    pub polling: u32,
    pub last_gpio: u32,
    pub shift_by: c_int,
    pub rc5_remote_gap: c_int,
// RC5 gpio
    pub /: *mut *mut bool rc5_gpio; / Is RC5 legacy GPIO enabled?,
    pub /: *mut *mut u32 last_bit; / last raw bit seen,
    pub /: *mut *mut u32 code; / raw code under construction,
    pub /: *mut *mut ktime_t base_time; / time of last seen code,
    pub /: *mut *mut bool active; / building raw code,
}

// ----------------------------------------------------------
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bttv_geometry {
    pub vtc,crop,comb: u8,
    pub width,hscale,hdelay: u16,
    pub sheight,vscale,vdelay,vtotal: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bttv_buffer {
// common v4l buffer stuff -- must be first
    pub vbuf: vb2_v4l2_buffer,
    pub list: list_head,
// bttv specific
    pub btformat: c_int,
    pub btswap: c_int,
    pub geo: bttv_geometry,
    pub top: btcx_riscmem,
    pub bottom: btcx_riscmem,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bttv_buffer_set {
    pub /: *mut *mut *mut bttv_buffer top; / top field buffer,
    pub /: *mut *mut *mut bttv_buffer bottom; / bottom field buffer,
    pub top_irq: c_uint,
    pub frame_irq: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bttv_vbi_fmt {
    pub fmt: v4l2_vbi_format,
// fmt.start[] and count[] refer to this video standard.
    pub tvnorm: *const bttv_tvnorm,
// Earliest possible start of video capturing with this
    pub end: __s32,
}

// bttv-vbi.c
extern "C" {
    pub fn bttv_vbi_fmt_reset(f: *mut bttv_vbi_fmt, norm: c_uint);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bttv_crop {
// A cropping rectangle in struct bttv_tvnorm.cropcap units.
    pub rect: v4l2_rect,
// Scaled image size limits with this crop rect. Divide
    pub min_scaled_width: __s32,
    pub min_scaled_height: __s32,
    pub max_scaled_width: __s32,
    pub max_scaled_height: __s32,
}

// ----------------------------------------------------------
// bttv-risc.c
// risc code generators - capture
// control dma register + risc main loop
extern "C" {
    pub fn bttv_set_dma(btv: *mut bttv, override: c_int);
}
extern "C" {
    pub fn bttv_risc_init_main(btv: *mut bttv) -> c_int;
}
// capture buffer handling
extern "C" {
    pub fn bttv_buffer_risc(btv: *mut bttv, buf: *mut bttv_buffer) -> c_int;
}
extern "C" {
    pub fn bttv_buffer_risc_vbi(btv: *mut bttv, buf: *mut bttv_buffer) -> c_int;
}
// ----------------------------------------------------------
// bttv-vbi.c
//
// 2048 for compatibility with earlier driver versions. The driver really
// stores 1024 + tvnorm->vbipack * 4 samples per line in the buffer. Note
// tvnorm->vbipack is <= 0xFF (limit of VBIPACK_LO + HI is 0x1FF DWORDs) and
// VBI read()s store a frame counter in the last four bytes of the VBI image.
//
pub const VBI_BPL: c_int = 2048;
pub const VBI_DEFLINES: c_int = 16;
extern "C" {
    pub fn bttv_try_fmt_vbi_cap(file: *mut file, fh: *mut c_void, f: *mut v4l2_format) -> c_int;
}
extern "C" {
    pub fn bttv_g_fmt_vbi_cap(file: *mut file, fh: *mut c_void, f: *mut v4l2_format) -> c_int;
}
extern "C" {
    pub fn bttv_s_fmt_vbi_cap(file: *mut file, fh: *mut c_void, f: *mut v4l2_format) -> c_int;
}
// ----------------------------------------------------------
// bttv-gpio.c
extern "C" {
    pub fn bttv_sub_add_device(core: *mut bttv_core, name: *mut c_char) -> c_int;
}
extern "C" {
    pub fn bttv_sub_del_devices(core: *mut bttv_core) -> c_int;
}
// ----------------------------------------------------------
// bttv-input.c
extern "C" {
    pub fn init_bttv_i2c_ir(btv: *mut bttv);
}
// ----------------------------------------------------------
// bttv-i2c.c
extern "C" {
    pub fn init_bttv_i2c(btv: *mut bttv) -> c_int;
}
extern "C" {
    pub fn fini_bttv_i2c(btv: *mut bttv) -> c_int;
}
// ----------------------------------------------------------
// bttv-driver.c
// insmod options
extern "C" {
    pub fn check_alloc_btres_lock(btv: *mut bttv, bit: c_int) -> c_int;
}
extern "C" {
    pub fn free_btres_lock(btv: *mut bttv, bits: c_int);
}
extern "C" {
    pub fn bttv_gpio_tracking(btv: *mut bttv, comment: *mut c_char);
}

pub const BTTV_MAX_FBUF: c_uint = 0x208000;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bttv_pll_info {
    pub /: *mut *mut unsigned int pll_ifreq; / PLL input frequency,
    pub /: *mut *mut unsigned int pll_ofreq; / PLL output frequency,
    pub /: *mut *mut unsigned int pll_crystal; / Crystal used for input,
    pub /: *mut *mut unsigned int pll_current; / Currently programmed ofreq,
}

// for gpio-connected remote control
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bttv_input {
    pub dev: *mut input_dev,
    pub name: [c_char; 32],
    pub phys: [c_char; 32],
    pub mask_keycode: u32,
    pub mask_keydown: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bttv_suspend_state {
    pub gpio_enable: u32,
    pub gpio_data: u32,
    pub disabled: c_int,
    pub loop_irq: c_int,
    pub video: bttv_buffer_set,
    pub vbi: *mut bttv_buffer,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bttv_tea575x_gpio {
    pub most: u8 data, clk, wren,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bttv {
    pub c: bttv_core,
// pci device config
    pub id: c_ushort,
    pub revision: c_uchar,
    pub /: *mut *mut *mut unsigned char __iomem bt848_mmio; / pointer to mmio,
// card configuration info
    pub /: *mut *mut unsigned int cardid; / pci subsystem id (bt878 based ones),
    pub /: *mut *mut unsigned int tuner_type; / tuner chip type,
    pub tda9887_conf: c_uint,
    pub dig: unsigned int svhs,,
    pub has_saa6588:1: c_uint,
    pub pll: bttv_pll_info,
    pub triton1: c_int,
    pub gpioirq: c_int,
    pub use_i2c_hw: c_int,
// old gpio interface
    pub shutdown: c_int,
    pub volume): *mut *mut *mut void (volume_gpio)(struct bttv btv, __u16,
    pub set): *mut *mut *mut *mut void (audio_mode_gpio)(struct bttv btv, struct v4l2_tuner tuner, int,
// new gpio interface
    pub gpio_lock: spinlock_t,
// i2c layer
    pub i2c_algo: i2c_algo_bit_data,
    pub i2c_client: i2c_client,
    pub i2c_rc: int i2c_state,,
    pub i2c_done: c_int,
    pub i2c_queue: wait_queue_head_t,
    pub sd_msp34xx: *mut v4l2_subdev,
    pub sd_tvaudio: *mut v4l2_subdev,
    pub sd_tda7432: *mut v4l2_subdev,
// video4linux (1)
    pub video_dev: video_device,
    pub radio_dev: video_device,
    pub vbi_dev: video_device,
// controls
    pub ctrl_handler: v4l2_ctrl_handler,
    pub radio_ctrl_handler: v4l2_ctrl_handler,
// infrared remote
    pub has_remote: c_int,
    pub remote: *mut bttv_ir,
// I2C remote data
    pub init_data: IR_i2c_init_data,
// locking
    pub s_lock: spinlock_t,
    pub lock: mutex,
    pub resources: c_int,
// video state
    pub input: c_uint,
    pub audio_input: c_uint,
    pub mute: c_uint,
    pub tv_freq: c_ulong,
    pub tvnorm: c_uint,
    pub std: v4l2_std_id,
    pub saturation: int hue, contrast, bright,,
    pub fbuf: v4l2_framebuffer,
    pub field_count: __u32,
// various options
    pub opt_combfilter: c_int,
    pub opt_automute: c_int,
    pub opt_vcr_hack: c_int,
    pub opt_uv_ratio: c_int,
// radio data/state
    pub has_radio: c_int,
    pub has_radio_tuner: c_int,
    pub radio_user: c_int,
    pub radio_uses_msp_demodulator: c_int,
    pub radio_freq: c_ulong,
// miro/pinnacle + Aimslab VHX
    pub has_tea575x: c_int,
    pub tea_gpio: bttv_tea575x_gpio,
    pub tea: snd_tea575x,
// ISA stuff (Terratec Active Radio Upgrade)
    pub mbox_ior: c_int,
    pub mbox_iow: c_int,
    pub mbox_csel: c_int,
// switch status for multi-controller cards
    pub sw_status: [c_char; 4],
// risc memory management data
    pub main: btcx_riscmem,
    pub /: *mut *mut list_head capture; / video capture queue,
    pub /: *mut *mut list_head vcapture; / vbi capture queue,
    pub /: *mut *mut bttv_buffer_set curr; / active buffers,
    pub /: *mut *mut *mut bttv_buffer cvbi; / active vbi buffer,
    pub loop_irq: c_int,
    pub new_input: c_int,
    pub dma_on: c_ulong,
    pub timeout: timer_list,
    pub state: bttv_suspend_state,
// stats
    pub errors: c_uint,
    pub framedrop: c_uint,
    pub irq_total: c_uint,
    pub irq_me: c_uint,
    pub users: c_uint,
    pub fh: v4l2_fh,
    pub type: v4l2_buf_type,
    pub field: v4l2_field,
    pub field_last: c_int,
// video capture
    pub capq: vb2_queue,
    pub fmt: *const bttv_format,
    pub width: c_int,
    pub height: c_int,
// vbi capture
    pub vbiq: vb2_queue,
    pub vbi_fmt: bttv_vbi_fmt,
    pub vbi_count: [c_uint; 2],
// Application called VIDIOC_S_SELECTION.
    pub do_crop: c_int,
// used to make dvb-bt8xx autoloadable
    pub request_module_wk: work_struct,
// Default (0) and current (1) video capturing
    pub crop: [bttv_crop; 2],
// Earliest possible start of video capturing in
    pub vbi_end: __s32,
// Latest possible end of VBI capturing (= crop[x].rect.top when
    pub crop_start: __s32,
}

extern "C" {
    pub fn container_of(_arg: v4l2_dev, bttv: struct, _arg: c.v4l2_dev) -> return;
}
// our devices
pub const BTTV_MAX: c_int = 32;

extern "C" {
    pub fn init_irqreg(btv: *mut bttv);
}

