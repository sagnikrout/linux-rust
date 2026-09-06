//! Automatically rewritten from C Header to Rust Module
//! Source: sound/pci/trident/trident.h
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
// audio@tridentmicro.com
// Fri Feb 19 15:55:28 MST 1999
// Definitions for Trident 4DWave DX/NX chips
//

pub const SNDRV_TRIDENT_VOICE_TYPE_PCM: c_int = 0;
pub const SNDRV_TRIDENT_VOICE_TYPE_SYNTH: c_int = 1;
pub const SNDRV_TRIDENT_VOICE_TYPE_MIDI: c_int = 2;

// TLB code constants
pub const SNDRV_TRIDENT_PAGE_SIZE: c_int = 4096;
pub const SNDRV_TRIDENT_PAGE_SHIFT: c_int = 12;

pub const SNDRV_TRIDENT_MAX_PAGES: c_int = 4096;
//
// Direct registers
//

pub const ID_4DWAVE_DX: c_uint = 0x2000;
pub const ID_4DWAVE_NX: c_uint = 0x2001;
// Bank definitions
pub const T4D_BANK_A: c_int = 0;
pub const T4D_BANK_B: c_int = 1;
pub const T4D_NUM_BANKS: c_int = 2;
// Register definitions
// Global registers
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum global_control_bits {
    CHANNEL_IDX	= 0x0000003f,
    OVERRUN_IE	= 0x00000400,	/* interrupt enable: capture overrun */
    UNDERRUN_IE	= 0x00000800,	/* interrupt enable: playback underrun */
    ENDLP_IE	= 0x00001000,	/* interrupt enable: end of buffer */
    MIDLP_IE	= 0x00002000,	/* interrupt enable: middle buffer */
    ETOG_IE		= 0x00004000,	/* interrupt enable: envelope toggling */
    EDROP_IE	= 0x00008000,	/* interrupt enable: envelope drop */
    BANK_B_EN	= 0x00010000,	/* SiS: enable bank B (64 channels) */
    PCMIN_B_MIX	= 0x00020000,	/* SiS: PCM IN B mixing enable */
    I2S_OUT_ASSIGN	= 0x00040000,	/* SiS: I2S Out contains surround PCM */
    SPDIF_OUT_ASSIGN= 0x00080000,	/* SiS: 0=S/PDIF L/R | 1=PCM Out FIFO */
    MAIN_OUT_ASSIGN = 0x00100000,	/* SiS: 0=PCM Out FIFO | 1=MMC Out buffer */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum miscint_bits {
    PB_UNDERRUN_IRQ = 0x00000001, REC_OVERRUN_IRQ = 0x00000002,
    SB_IRQ		= 0x00000004, MPU401_IRQ      = 0x00000008,
    OPL3_IRQ        = 0x00000010, ADDRESS_IRQ     = 0x00000020,
    ENVELOPE_IRQ    = 0x00000040, PB_UNDERRUN     = 0x00000100,
    REC_OVERRUN	= 0x00000200, MIXER_UNDERFLOW = 0x00000400,
    MIXER_OVERFLOW  = 0x00000800, NX_SB_IRQ_DISABLE = 0x00001000,
    ST_TARGET_REACHED = 0x00008000,
    PB_24K_MODE     = 0x00010000, ST_IRQ_EN       = 0x00800000,
    ACGPIO_IRQ	= 0x01000000
}

// T2 legacy dma control registers.
pub const LEGACY_DMAR0: c_uint = 0x00  // ADR0;
pub const LEGACY_DMAR4: c_uint = 0x04  // CNT0;
pub const LEGACY_DMAR6: c_uint = 0x06  // CNT0 - High bits;
pub const LEGACY_DMAR11: c_uint = 0x0b  // MOD;
pub const LEGACY_DMAR15: c_uint = 0x0f  // MMR;
pub const T4D_START_A: c_uint = 0x80;
pub const T4D_STOP_A: c_uint = 0x84;
pub const T4D_DLY_A: c_uint = 0x88;
pub const T4D_SIGN_CSO_A: c_uint = 0x8c;
pub const T4D_CSPF_A: c_uint = 0x90;
pub const T4D_CSPF_B: c_uint = 0xbc;
pub const T4D_CEBC_A: c_uint = 0x94;
pub const T4D_AINT_A: c_uint = 0x98;
pub const T4D_AINTEN_A: c_uint = 0x9c;
pub const T4D_LFO_GC_CIR: c_uint = 0xa0;
pub const T4D_MUSICVOL_WAVEVOL: c_uint = 0xa8;
pub const T4D_SBDELTA_DELTA_R: c_uint = 0xac;
pub const T4D_MISCINT: c_uint = 0xb0;
pub const T4D_START_B: c_uint = 0xb4;
pub const T4D_STOP_B: c_uint = 0xb8;
pub const T4D_SBBL_SBCL: c_uint = 0xc0;
pub const T4D_SBCTRL_SBE2R_SBDD: c_uint = 0xc4;
pub const T4D_STIMER: c_uint = 0xc8;
pub const T4D_AINT_B: c_uint = 0xd8;
pub const T4D_AINTEN_B: c_uint = 0xdc;
pub const T4D_RCI: c_uint = 0x70;
// MPU-401 UART
pub const T4D_MPU401_BASE: c_uint = 0x20;
pub const T4D_MPUR0: c_uint = 0x20;
pub const T4D_MPUR1: c_uint = 0x21;
pub const T4D_MPUR2: c_uint = 0x22;
pub const T4D_MPUR3: c_uint = 0x23;
// S/PDIF Registers
pub const NX_SPCTRL_SPCSO: c_uint = 0x24;
pub const NX_SPLBA: c_uint = 0x28;
pub const NX_SPESO: c_uint = 0x2c;
pub const NX_SPCSTATUS: c_uint = 0x64;
// Joystick
pub const GAMEPORT_GCR: c_uint = 0x30;
pub const GAMEPORT_MODE_ADC: c_uint = 0x80;
pub const GAMEPORT_LEGACY: c_uint = 0x31;
pub const GAMEPORT_AXES: c_uint = 0x34;
// NX Specific Registers
pub const NX_TLBC: c_uint = 0x6c;
// Channel Registers
pub const CH_START: c_uint = 0xe0;
pub const CH_DX_CSO_ALPHA_FMS: c_uint = 0xe0;
pub const CH_DX_ESO_DELTA: c_uint = 0xe8;
pub const CH_DX_FMC_RVOL_CVOL: c_uint = 0xec;
pub const CH_NX_DELTA_CSO: c_uint = 0xe0;
pub const CH_NX_DELTA_ESO: c_uint = 0xe8;
pub const CH_NX_ALPHA_FMS_FMC_RVOL_CVOL: c_uint = 0xec;
pub const CH_LBA: c_uint = 0xe4;
pub const CH_GVSEL_PAN_VOL_CTRL_EC: c_uint = 0xf0;
pub const CH_EBUF1: c_uint = 0xf4;
pub const CH_EBUF2: c_uint = 0xf8;
// AC-97 Registers
pub const DX_ACR0_AC97_W: c_uint = 0x40;
pub const DX_ACR1_AC97_R: c_uint = 0x44;
pub const DX_ACR2_AC97_COM_STAT: c_uint = 0x48;
pub const NX_ACR0_AC97_COM_STAT: c_uint = 0x40;
pub const NX_ACR1_AC97_W: c_uint = 0x44;
pub const NX_ACR2_AC97_R_PRIMARY: c_uint = 0x48;
pub const NX_ACR3_AC97_R_SECONDARY: c_uint = 0x4c;
pub const SI_AC97_WRITE: c_uint = 0x40;
pub const SI_AC97_READ: c_uint = 0x44;
pub const SI_SERIAL_INTF_CTRL: c_uint = 0x48;
pub const SI_AC97_GPIO: c_uint = 0x4c;
pub const SI_ASR0: c_uint = 0x50;
pub const SI_SPDIF_CS: c_uint = 0x70;
pub const SI_GPIO: c_uint = 0x7c;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum trident_nx_ac97_bits {
// ACR1-3
    NX_AC97_BUSY_WRITE 	= 0x0800,
    NX_AC97_BUSY_READ	= 0x0800,
    NX_AC97_BUSY_DATA 	= 0x0400,
    NX_AC97_WRITE_SECONDARY = 0x0100,
// ACR0
    NX_AC97_SECONDARY_READY = 0x0040,
    NX_AC97_SECONDARY_RECORD = 0x0020,
    NX_AC97_SURROUND_OUTPUT = 0x0010,
    NX_AC97_PRIMARY_READY	= 0x0008,
    NX_AC97_PRIMARY_RECORD	= 0x0004,
    NX_AC97_PCM_OUTPUT	= 0x0002,
    NX_AC97_WARM_RESET	= 0x0001
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum trident_dx_ac97_bits {
    DX_AC97_BUSY_WRITE	= 0x8000,
    DX_AC97_BUSY_READ	= 0x8000,
    DX_AC97_READY		= 0x0010,
    DX_AC97_RECORD		= 0x0008,
    DX_AC97_PLAYBACK	= 0x0002
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sis7018_ac97_bits {
    SI_AC97_BUSY_WRITE =	0x00008000,
    SI_AC97_AUDIO_BUSY =	0x00004000,
    SI_AC97_MODEM_BUSY =	0x00002000,
    SI_AC97_BUSY_READ =	0x00008000,
    SI_AC97_SECONDARY =	0x00000080,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum serial_intf_ctrl_bits {
    WARM_RESET	= 0x00000001,
    COLD_RESET	= 0x00000002,
    I2S_CLOCK	= 0x00000004,
    PCM_SEC_AC97	= 0x00000008,
    AC97_DBL_RATE	= 0x00000010,
    SPDIF_EN	= 0x00000020,
    I2S_OUTPUT_EN	= 0x00000040,
    I2S_INPUT_EN	= 0x00000080,
    PCMIN		= 0x00000100,
    LINE1IN		= 0x00000200,
    MICIN		= 0x00000400,
    LINE2IN		= 0x00000800,
    HEAD_SET_IN	= 0x00001000,
    GPIOIN		= 0x00002000,
// 7018 spec says id = 01 but the demo board routed to 10
    SECONDARY_ID= 0x00004000, */
    SECONDARY_ID	= 0x00004000,
    PCMOUT		= 0x00010000,
    SURROUT		= 0x00020000,
    CENTEROUT	= 0x00040000,
    LFEOUT		= 0x00080000,
    LINE1OUT	= 0x00100000,
    LINE2OUT	= 0x00200000,
    GPIOOUT		= 0x00400000,
    SI_AC97_PRIMARY_READY = 0x01000000,
    SI_AC97_SECONDARY_READY = 0x02000000,
    SI_AC97_POWERDOWN = 0x04000000,
}

// PCM defaults

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_trident_port {
    pub chset: *mut *mut snd_midi_channel_set,
    pub trident: *mut *mut snd_trident,
    pub /: *mut *mut int mode; / operation mode,
    pub /: *mut *mut int client; / sequencer client number,
    pub /: *mut *mut int port; / sequencer port number,
    pub 1: unsigned int midi_has_voices:,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_trident_memblk_arg {
    pub last_page: short first_page,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_trident_tlb {
    pub /: *mut *mut *mut __le32 entries; / 16k-aligned TLB table,
    pub /: *mut *mut dma_addr_t entries_dmaaddr; / 16k-aligned PCI address to TLB table,
    pub buffer: *mut snd_dma_buffer,
    pub /: *mut *mut *mut snd_util_memhdr  memhdr; / page allocation list,
    pub silent_page: *mut snd_dma_buffer,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_trident_voice {
    pub number: c_uint,
    pub 1: midi:,
    pub flags: c_uint,
    pub client: c_uchar,
    pub port: c_uchar,
    pub index: c_uchar,
    pub sample_ops: *mut snd_trident_sample_ops,
// channel parameters
    pub /: *mut *mut unsigned int CSO; / 24 bits (16 on DX),
    pub /: *mut *mut unsigned int ESO; / 24 bits (16 on DX),
    pub /: *mut *mut unsigned int LBA; / 30 bits,
    pub /: *mut *mut unsigned short EC; / 12 bits,
    pub /: *mut *mut unsigned short Alpha; / 12 bits,
    pub /: *mut *mut unsigned short Delta; / 16 bits,
    pub /: *mut *mut unsigned short Attribute; / 16 bits - SiS 7018,
    pub /: *mut *mut unsigned short Vol; / 12 bits (6.6),
    pub /: *mut *mut unsigned char Pan; / 7 bits (1.4.2),
    pub /: *mut *mut unsigned char GVSel; / 1 bit,
    pub /: *mut *mut unsigned char RVol; / 7 bits (5.2),
    pub /: *mut *mut unsigned char CVol; / 7 bits (5.2),
    pub /: *mut *mut unsigned char FMC; / 2 bits,
    pub /: *mut *mut unsigned char CTRL; / 4 bits,
    pub /: *mut *mut unsigned char FMS; / 4 bits,
    pub /: *mut *mut unsigned char LFO; / 8 bits,
    pub /: *mut *mut unsigned int negCSO; / nonzero - use negative CSO,
    pub /: *mut *mut *mut snd_util_memblk memblk; / memory block if TLB enabled,
// PCM data
    pub trident: *mut snd_trident,
    pub substream: *mut snd_pcm_substream,
    pub /: *mut *mut *mut snd_trident_voice extra; / extra PCM voice (acts as interrupt generator),
    pub 1: isync3:,
    pub /: *mut *mut int foldback_chan; / foldback subdevice number,
    pub /: *mut *mut unsigned int stimer; / global sample timer (to detect spurious interrupts),
    pub /: *mut *mut unsigned int spurious_threshold; / spurious threshold,
    pub isync_mark: c_uint,
    pub isync_max: c_uint,
    pub isync_ESO: c_uint,
// ---
    pub private_data: *mut c_void,
    pub voice): *mut *mut void (private_free)(struct snd_trident_voice,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_4dwave {
    pub seq_client: c_int,
    pub seq_ports: [snd_trident_port; 4],
    pub voices: [snd_trident_voice; 64],
    pub /: *mut *mut int ChanSynthCount; / number of allocated synth channels,
    pub /: *mut *mut int max_size; / maximum synth memory size in bytes,
    pub /: *mut *mut int current_size; / current allocated synth mem in bytes,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_trident_pcm_mixer {
    pub /: *mut *mut *mut snd_trident_voice voice; / active voice,
    pub /: *mut *mut unsigned short vol; / front volume,
    pub /: *mut *mut unsigned char pan; / pan control,
    pub /: *mut *mut unsigned char rvol; / rear volume,
    pub /: *mut *mut unsigned char cvol; / center volume,
    pub pad: c_uchar,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_trident {
    pub irq: c_int,
    pub /: *mut *mut unsigned int device; / device ID,
    pub bDMAStart: c_uchar,
    pub port: c_ulong,
    pub midi_port: c_ulong,
    pub spurious_irq_count: c_uint,
    pub spurious_irq_max_delta: c_uint,
    pub /: *mut *mut snd_trident_tlb tlb; / TLB entries for NX cards,
    pub spdif_ctrl: c_uchar,
    pub spdif_pcm_ctrl: c_uchar,
    pub spdif_bits: c_uint,
    pub spdif_pcm_bits: c_uint,
    pub /: *mut *mut *mut snd_kcontrol spdif_pcm_ctl; / S/PDIF settings,
    pub ac97_ctrl: c_uint,
    pub /: *mut *mut unsigned int ChanMap[2]; / allocation map for hardware channels,
    pub /: *mut *mut int ChanPCM; / max number of PCM channels,
    pub /: *mut *mut int ChanPCMcnt; / actual number of PCM channels,
    pub /: *mut *mut unsigned int ac97_detect: 1; / 1 = AC97 in detection phase,
    pub /: *mut *mut unsigned int in_suspend: 1; / 1 during suspend/resume,
    pub /: *mut *mut snd_4dwave synth; / synth specific variables,
    pub event_lock: spinlock_t,
    pub voice_alloc: spinlock_t,
    pub dma_dev: snd_dma_device,
    pub pci: *mut pci_dev,
    pub card: *mut snd_card,
    pub /: *mut *mut *mut snd_pcm pcm; / ADC/DAC PCM,
    pub /: *mut *mut *mut snd_pcm foldback; / Foldback PCM,
    pub /: *mut *mut *mut snd_pcm spdif; / SPDIF PCM,
    pub rmidi: *mut snd_rawmidi,
    pub ac97_bus: *mut snd_ac97_bus,
    pub ac97: *mut snd_ac97,
    pub ac97_sec: *mut snd_ac97,
    pub musicvol_wavevol: c_uint,
    pub pcm_mixer: [snd_trident_pcm_mixer; 32],
    pub /: *mut *mut *mut snd_kcontrol ctl_vol; / front volume,
    pub /: *mut *mut *mut snd_kcontrol ctl_pan; / pan,
    pub /: *mut *mut *mut snd_kcontrol ctl_rvol; / rear volume,
    pub /: *mut *mut *mut snd_kcontrol ctl_cvol; / center volume,
    pub reg_lock: spinlock_t,
    pub gameport: *mut gameport,
}

extern "C" {
    pub fn snd_trident_create_gameport(trident: *mut snd_trident) -> c_int;
}
extern "C" {
    pub fn snd_trident_pcm(trident: *mut snd_trident, device: c_int) -> c_int;
}
extern "C" {
    pub fn snd_trident_foldback_pcm(trident: *mut snd_trident, device: c_int) -> c_int;
}
extern "C" {
    pub fn snd_trident_spdif_pcm(trident: *mut snd_trident, device: c_int) -> c_int;
}
extern "C" {
    pub fn snd_trident_free_voice(trident: *mut *mut snd_trident, voice: *mut snd_trident_voice);
}
extern "C" {
    pub fn snd_trident_start_voice(trident: *mut *mut snd_trident, voice: c_uint);
}
extern "C" {
    pub fn snd_trident_stop_voice(trident: *mut *mut snd_trident, voice: c_uint);
}
extern "C" {
    pub fn snd_trident_write_voice_regs(trident: *mut *mut snd_trident, voice: *mut snd_trident_voice);
}
// TLB memory allocation
extern "C" {
    pub fn snd_trident_free_pages(trident: *mut snd_trident, blk: *mut snd_util_memblk) -> c_int;
}
