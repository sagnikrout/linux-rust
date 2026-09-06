//! Automatically rewritten from C Header to Rust Module
//! Source: sound/pci/au88x0/au88x0.h
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

// Vortex MPU401 defines.
pub const MIDI_CLOCK_DIV: c_uint = 0x61;
// Standart MPU401 defines.
pub const MPU401_RESET: c_uint = 0xff;
pub const MPU401_ENTER_UART: c_uint = 0x3f;
pub const MPU401_ACK: c_uint = 0xfe;
// Get src register value to convert from x to y.

// FIFO software state constants.
pub const FIFO_STOP: c_int = 0;
pub const FIFO_START: c_int = 1;
pub const FIFO_PAUSE: c_int = 2;
// IRQ flags
pub const IRQ_ERR_MASK: c_uint = 0x00ff;
pub const IRQ_FATAL: c_uint = 0x0001;
pub const IRQ_PARITY: c_uint = 0x0002;
pub const IRQ_REG: c_uint = 0x0004;
pub const IRQ_FIFO: c_uint = 0x0008;
pub const IRQ_DMA: c_uint = 0x0010;
pub const IRQ_PCMOUT: c_uint = 0x0020	/* PCM OUT page crossing */;
pub const IRQ_TIMER: c_uint = 0x1000;
pub const IRQ_MIDI: c_uint = 0x2000;
pub const IRQ_MODEM: c_uint = 0x4000;
// ADB Resource
pub const VORTEX_RESOURCE_DMA: c_uint = 0x00000000;
pub const VORTEX_RESOURCE_SRC: c_uint = 0x00000001;
pub const VORTEX_RESOURCE_MIXIN: c_uint = 0x00000002;
pub const VORTEX_RESOURCE_MIXOUT: c_uint = 0x00000003;
pub const VORTEX_RESOURCE_A3D: c_uint = 0x00000004;
pub const VORTEX_RESOURCE_LAST: c_uint = 0x00000005;
// codec io: VORTEX_CODEC_IO bits
pub const VORTEX_CODEC_ID_SHIFT: c_int = 24;
pub const VORTEX_CODEC_WRITE: c_uint = 0x00800000;
pub const VORTEX_CODEC_ADDSHIFT: c_int = 16;
pub const VORTEX_CODEC_ADDMASK: c_uint = 0x7f0000;
pub const VORTEX_CODEC_DATSHIFT: c_int = 0;
pub const VORTEX_CODEC_DATMASK: c_uint = 0xffff;
// Check for SDAC bit in "Extended audio ID" AC97 register
// #define VORTEX_IS_QUAD(x) (((x)->codec == NULL) ?  0 : ((x)->codec->ext_id&0x80))

// Check if chip has bug.

// PCM devices
pub const VORTEX_PCM_ADB: c_int = 0;
pub const VORTEX_PCM_SPDIF: c_int = 1;
pub const VORTEX_PCM_A3D: c_int = 2;
pub const VORTEX_PCM_WT: c_int = 3;
pub const VORTEX_PCM_I2S: c_int = 4;
pub const VORTEX_PCM_LAST: c_int = 5;

pub const NR_WTPB: c_uint = 0x20		/* WT channels per each bank. */;
pub const NR_PCM: c_uint = 0x10;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pcm_vol {
    pub kctl: *mut snd_kcontrol,
    pub active: c_int,
    pub dma: c_int,
    pub mixin: [c_int; 4],
    pub vol: [c_int; 4],
}

// Structs
// int this_08;          /* Still unknown
// Virtual page extender stuff
pub type vortex_t = snd_vortex;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_vortex {
// ALSA structs.
    pub card: *mut snd_card,
    pub pcm: [*mut snd_pcm; VORTEX_PCM_LAST],
    pub /: *mut *mut *mut snd_rawmidi rmidi; / Legacy Midi interface.,
    pub codec: *mut snd_ac97,
// Stream structs.
    pub dma_adb: [stream_t; NR_ADB],
    pub spdif_sr: c_int,
    pub dma_wt: [stream_t; NR_WT],
    pub /: *mut *mut wt_voice_t wt_voice[NR_WT]; / WT register cache.,
    pub /: *mut *mut *mut s8 mixwt[(NR_WT / NR_WTPB)  6]; / WT mixin objects,

// Global resources
    pub mixcapt: [i8; 2],
    pub mixplayb: [i8; 4],    pub mixspdif: [i8; 2],
    pub /: *mut *mut s8 mixa3d[2]; / mixers which collect all a3d streams.,
    pub /: *mut *mut s8 mixxtlk[2]; / crosstalk canceler mixer inputs.,
    pub fixed_res: [u32; 5],
// Hardware equalizer structs
    pub eq: eqlzr_t,
// A3D structs
    pub a3d: [a3dsrc_t; NR_A3D],
// Xtalk canceler
    pub /: *mut *mut int xt_mode; / 1: speakers, 0:headphones.,
    pub pcm_vol: [pcm_vol; NR_PCM],
    pub /: *mut *mut int isquad; / cache of extended ID codec flag.,
// Gameport stuff.
    pub gameport: *mut gameport,
// PCI hardware resources
    pub io: c_ulong,
    pub mmio: *mut void __iomem,
    pub irq: c_uint,
    pub lock: spinlock_t,
// PCI device
    pub pci_dev: *mut pci_dev,
    pub vendor: u16,
    pub device: u16,
    pub rev: u8,
}

// Functions.
// SRC
// DMA Engines.
extern "C" {
    pub fn vortex_adbdma_setstartbuffer(vortex: *mut *mut vortex_t, adbdma: c_int, sb: c_int) -> static void;
}

extern "C" {
    pub fn vortex_wtdma_setstartbuffer(vortex: *mut *mut vortex_t, wtdma: c_int, sb: c_int) -> static void;
}

extern "C" {
    pub fn vortex_adbdma_startfifo(vortex: *mut *mut vortex_t, adbdma: c_int) -> static void;
}
// static void vortex_adbdma_stopfifo(vortex_t *vortex, int adbdma);
extern "C" {
    pub fn vortex_adbdma_pausefifo(vortex: *mut *mut vortex_t, adbdma: c_int) -> static void;
}
extern "C" {
    pub fn vortex_adbdma_resumefifo(vortex: *mut *mut vortex_t, adbdma: c_int) -> static void;
}
extern "C" {
    pub fn vortex_adbdma_getlinearpos(vortex: *mut *mut vortex_t, adbdma: c_int) -> c_int;
}
extern "C" {
    pub fn vortex_adbdma_resetup(vortex: *mut vortex_t, adbdma: c_int) -> static void;
}

extern "C" {
    pub fn vortex_wtdma_startfifo(vortex: *mut *mut vortex_t, wtdma: c_int) -> static void;
}
extern "C" {
    pub fn vortex_wtdma_stopfifo(vortex: *mut *mut vortex_t, wtdma: c_int) -> static void;
}
extern "C" {
    pub fn vortex_wtdma_pausefifo(vortex: *mut *mut vortex_t, wtdma: c_int) -> static void;
}
extern "C" {
    pub fn vortex_wtdma_resumefifo(vortex: *mut *mut vortex_t, wtdma: c_int) -> static void;
}
extern "C" {
    pub fn vortex_wtdma_getlinearpos(vortex: *mut *mut vortex_t, wtdma: c_int) -> c_int;
}

// global stuff.
extern "C" {
    pub fn vortex_codec_init(vortex: *mut *mut vortex_t) -> static void;
}
extern "C" {
    pub fn vortex_codec_read(codec: *mut *mut snd_ac97, addr: c_ushort) -> static unsigned short;
}
extern "C" {
    pub fn vortex_spdif_init(vortex: *mut *mut vortex_t, spdif_sr: c_int, spdif_mode: c_int) -> static void;
}
extern "C" {
    pub fn vortex_core_init(card: *mut *mut vortex_t) -> static int;
}
extern "C" {
    pub fn vortex_core_shutdown(card: *mut *mut vortex_t) -> static int;
}
extern "C" {
    pub fn vortex_enable_int(card: *mut *mut vortex_t) -> static void;
}
extern "C" {
    pub fn vortex_interrupt(irq: c_int, dev_id: *mut c_void) -> static irqreturn_t;
}
extern "C" {
    pub fn vortex_alsafmt_aspfmt(alsafmt: snd_pcm_format_t, v: *mut vortex_t) -> static int;
}
// Connection  stuff.
extern "C" {
    pub fn vortex_connect_default(vortex: *mut *mut vortex_t, en: c_int) -> static void;
}

extern "C" {
    pub fn vortex_wt_allocroute(vortex: *mut *mut vortex_t, dma: c_int, nr_ch: c_int) -> static int;
}
extern "C" {
    pub fn vortex_wt_connect(vortex: *mut *mut vortex_t, en: c_int) -> static void;
}
extern "C" {
    pub fn vortex_wt_init(vortex: *mut *mut vortex_t) -> static void;
}

// A3D functions.

extern "C" {
    pub fn vortex_Vort3D_enable(v: *mut *mut vortex_t) -> static void;
}
extern "C" {
    pub fn vortex_Vort3D_disable(v: *mut *mut vortex_t) -> static void;
}
extern "C" {
    pub fn vortex_Vort3D_connect(vortex: *mut *mut vortex_t, en: c_int) -> static void;
}
extern "C" {
    pub fn vortex_Vort3D_InitializeSource(a: *mut a3dsrc_t, en: c_int, v: *mut vortex_t) -> static void;
}

// Driver stuff.
extern "C" {
    pub fn vortex_gameport_register(card: *mut *mut vortex_t) -> static int;
}
extern "C" {
    pub fn vortex_gameport_unregister(card: *mut *mut vortex_t) -> static void;
}

extern "C" {
    pub fn vortex_eq_init(vortex: *mut *mut vortex_t) -> static int;
}
extern "C" {
    pub fn vortex_eq_free(vortex: *mut *mut vortex_t) -> static int;
}

// ALSA stuff.
extern "C" {
    pub fn snd_vortex_new_pcm(vortex: *mut *mut vortex_t, idx: c_int, nr: c_int) -> static int;
}
extern "C" {
    pub fn snd_vortex_mixer(vortex: *mut *mut vortex_t) -> static int;
}
extern "C" {
    pub fn snd_vortex_midi(vortex: *mut *mut vortex_t) -> static int;
}
