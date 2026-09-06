//! Automatically rewritten from C Header to Rust Module
//! Source: sound/pci/azt3328.h
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
// "PU" == "power-up value", as tested on PCI168 PCI rev. 10
// "WRITE_ONLY"  == register does not indicate actual bit values
// main I/O area port indices
// (only 0x70 of 0x80 bytes saved/restored by Windows driver)
pub const AZF_IO_SIZE_CTRL: c_uint = 0x80;
pub const AZF_IO_SIZE_CTRL_PM: c_uint = 0x70;
// the driver initialisation suggests a layout of 4 areas
// within the main card control I/O:
// from 0x00 (playback codec), from 0x20 (recording codec)
// and from 0x40 (most certainly I2S out codec).
// And another area from 0x60 to 0x6f (DirectX timer, IRQ management,
// power management etc.???).
pub const AZF_IO_OFFS_CODEC_PLAYBACK: c_uint = 0x00;
pub const AZF_IO_OFFS_CODEC_CAPTURE: c_uint = 0x20;
pub const AZF_IO_OFFS_CODEC_I2S_OUT: c_uint = 0x40;
pub const IDX_IO_CODEC_DMA_FLAGS: c_uint = 0x00 /* PU:0x0000 */;
// able to reactivate output after output muting due to 8/16bit
// output change, just like 0x0002.
// 0x0001 is the only bit that's able to start the DMA counter
pub const DMA_RESUME: c_uint = 0x0001 /* paused if cleared? */;
// 0x0002 *temporarily* set during DMA stopping. hmm
// both 0x0002 and 0x0004 set in playback setup.
// able to reactivate output after output muting due to 8/16bit
// output change, just like 0x0001.
pub const DMA_RUN_SOMETHING1: c_uint = 0x0002 /* \ alternated (toggled) */;
// 0x0004: NOT able to reactivate output
pub const DMA_RUN_SOMETHING2: c_uint = 0x0004 /* / bits */;
pub const SOMETHING_ALMOST_ALWAYS_SET: c_uint = 0x0008 /* ???; can be modified */;
pub const DMA_EPILOGUE_SOMETHING: c_uint = 0x0010;
pub const DMA_SOMETHING_ELSE: c_uint = 0x0020 /* ??? */;
pub const SOMETHING_UNMODIFIABLE: c_uint = 0xffc0 /* unused? not modifiable */;
pub const IDX_IO_CODEC_IRQTYPE: c_uint = 0x02 /* PU:0x0001 */;
// write back to flags in case flags are set, in order to ACK IRQ in handler
// (bit 1 of port 0x64 indicates interrupt for one of these three types)
// sometimes in this case it just writes 0xffff to globally ACK all IRQs
// settings written are not reflected when reading back, though.
// seems to be IRQ, too (frequently used: port |= 0x07 !), but who knows?
pub const IRQ_SOMETHING: c_uint = 0x0001 /* something & ACK */;
pub const IRQ_FINISHED_DMABUF_1: c_uint = 0x0002 /* 1st dmabuf finished & ACK */;
pub const IRQ_FINISHED_DMABUF_2: c_uint = 0x0004 /* 2nd dmabuf finished & ACK */;
pub const IRQMASK_SOME_STATUS_1: c_uint = 0x0008 /* \ related bits */;
pub const IRQMASK_SOME_STATUS_2: c_uint = 0x0010 /* / (checked together in loop) */;
pub const IRQMASK_UNMODIFIABLE: c_uint = 0xffe0 /* unused? not modifiable */;
// start address of 1st DMA transfer area, PU:0x00000000
pub const IDX_IO_CODEC_DMA_START_1: c_uint = 0x04;
// start address of 2nd DMA transfer area, PU:0x00000000
pub const IDX_IO_CODEC_DMA_START_2: c_uint = 0x08;
// both lengths of DMA transfer areas, PU:0x00000000
pub const IDX_IO_CODEC_DMA_LENGTHS: c_uint = 0x0c;
pub const IDX_IO_CODEC_DMA_CURRPOS: c_uint = 0x10 /* current DMA position, PU:0x00000000 */;
// offset within current DMA transfer area, PU:0x0000
pub const IDX_IO_CODEC_DMA_CURROFS: c_uint = 0x14;
pub const IDX_IO_CODEC_SOUNDFORMAT: c_uint = 0x16 /* PU:0x0010 */;
// all unspecified bits can't be modified
pub const SOUNDFORMAT_FREQUENCY_MASK: c_uint = 0x000f;
pub const SOUNDFORMAT_XTAL1: c_uint = 0x00;
pub const SOUNDFORMAT_XTAL2: c_uint = 0x01;
// all _SUSPECTED_ values are not used by Windows drivers, so we don't
// have any hard facts, only rough measurements.
// All we know is that the crystal used on the board has 24.576MHz,
// like many soundcards (which results in the frequencies below when
// using certain divider values selected by the values below)
pub const SOUNDFORMAT_FREQ_SUSPECTED_4000: c_uint = 0x0c | SOUNDFORMAT_XTAL1;
pub const SOUNDFORMAT_FREQ_SUSPECTED_4800: c_uint = 0x0a | SOUNDFORMAT_XTAL1;
pub const SOUNDFORMAT_FREQ_5510: c_uint = 0x0c | SOUNDFORMAT_XTAL2;
pub const SOUNDFORMAT_FREQ_6620: c_uint = 0x0a | SOUNDFORMAT_XTAL2;
pub const SOUNDFORMAT_FREQ_8000: c_uint = 0x00 | SOUNDFORMAT_XTAL1 /* also 0x0e | SOUNDFORMAT_XTAL1? */;
pub const SOUNDFORMAT_FREQ_9600: c_uint = 0x08 | SOUNDFORMAT_XTAL1;
pub const SOUNDFORMAT_FREQ_11025: c_uint = 0x00 | SOUNDFORMAT_XTAL2 /* also 0x0e | SOUNDFORMAT_XTAL2? */;
pub const SOUNDFORMAT_FREQ_SUSPECTED_13240: c_uint = 0x08 | SOUNDFORMAT_XTAL2 /* seems to be 6620 *2 */;
pub const SOUNDFORMAT_FREQ_16000: c_uint = 0x02 | SOUNDFORMAT_XTAL1;
pub const SOUNDFORMAT_FREQ_22050: c_uint = 0x02 | SOUNDFORMAT_XTAL2;
pub const SOUNDFORMAT_FREQ_32000: c_uint = 0x04 | SOUNDFORMAT_XTAL1;
pub const SOUNDFORMAT_FREQ_44100: c_uint = 0x04 | SOUNDFORMAT_XTAL2;
pub const SOUNDFORMAT_FREQ_48000: c_uint = 0x06 | SOUNDFORMAT_XTAL1;
pub const SOUNDFORMAT_FREQ_SUSPECTED_66200: c_uint = 0x06 | SOUNDFORMAT_XTAL2 /* 66200 (13240 * 5); 64000 may have been nicer :-\ */;
pub const SOUNDFORMAT_FLAG_16BIT: c_uint = 0x0010;
pub const SOUNDFORMAT_FLAG_2CHANNELS: c_uint = 0x0020;
// define frequency helpers, for maximum value safety
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum azf_freq_t {

    AZF_FREQ(4000),
    AZF_FREQ(4800),
    AZF_FREQ(5512),
    AZF_FREQ(6620),
    AZF_FREQ(8000),
    AZF_FREQ(9600),
    AZF_FREQ(11025),
    AZF_FREQ(13240),
    AZF_FREQ(16000),
    AZF_FREQ(22050),
    AZF_FREQ(32000),
    AZF_FREQ(44100),
    AZF_FREQ(48000),
    AZF_FREQ(66200),

}

// DirectX timer, main interrupt area (FIXME: and something else?)
pub const IDX_IO_TIMER_VALUE: c_uint = 0x60 /* found this timer area by pure luck :-) */;
// timer countdown value; triggers IRQ when timer is finished
pub const TIMER_VALUE_MASK: c_uint = 0x000fffffUL;
// activate timer countdown
pub const TIMER_COUNTDOWN_ENABLE: c_uint = 0x01000000UL;
// trigger timer IRQ on zero transition
pub const TIMER_IRQ_ENABLE: c_uint = 0x02000000UL;
// being set in IRQ handler in case port 0x00 (hmm, not port 0x64!?!?)
// had 0x0020 set upon IRQ handler
pub const TIMER_IRQ_ACK: c_uint = 0x04000000UL;
pub const IDX_IO_IRQSTATUS: c_uint = 0x64;
// some IRQ bit in here might also be used to signal a power-management timer
// timeout, to request shutdown of the chip (e.g. AD1815JS has such a thing).
// OPL3 hardware contains several timers which confusingly in most cases
// are NOT routed to an IRQ, but some designs (e.g. LM4560) DO support that,
// so I wouldn't be surprised at all to discover that AZF3328
// supports that thing as well...
pub const IRQ_PLAYBACK: c_uint = 0x0001;
pub const IRQ_RECORDING: c_uint = 0x0002;
pub const IRQ_I2S_OUT: c_uint = 0x0004 /* this IS I2S, right!? (untested) */;
pub const IRQ_GAMEPORT: c_uint = 0x0008 /* Interrupt of Digital(ly) Enhanced Game Port */;
pub const IRQ_MPU401: c_uint = 0x0010;
pub const IRQ_TIMER: c_uint = 0x0020 /* DirectX timer */;
pub const IRQ_UNKNOWN2: c_uint = 0x0040 /* probably unused, or possibly OPL3 timer? */;
pub const IRQ_UNKNOWN3: c_uint = 0x0080 /* probably unused, or possibly OPL3 timer? */;
pub const IDX_IO_66H: c_uint = 0x66    /* writing 0xffff returns 0x0000 */;
// this is set to e.g. 0x3ff or 0x300, and writable;
// maybe some buffer limit, but I couldn't find out more, PU:0x00ff:
pub const IDX_IO_SOME_VALUE: c_uint = 0x68;
pub const IO_68_RANDOM_TOGGLE1: c_uint = 0x0100	/* toggles randomly */;
pub const IO_68_RANDOM_TOGGLE2: c_uint = 0x0200	/* toggles randomly */;
// umm, nope, behaviour of these bits changes depending on what we wrote
// to 0x6b!!
// And they change upon playback/stop, too:
// Writing a value to 0x68 will display this exact value during playback,
// too but when stopped it can fall back to a rather different
// seemingly random value). Hmm, possibly this is a register which
// has a remote shadow which needs proper device supply which only exists
// in case playback is active? Or is this driver-induced?
//
// this WORD can be set to have bits 0x0028 activated (FIXME: correct??);
// actually inhibits PCM playback!!! maybe power management??:
pub const IDX_IO_6AH: c_uint = 0x6A /* WRITE_ONLY! */;
// bit 5: enabling this will activate permanent counting of bytes 2/3
// at gameport I/O (0xb402/3) (equal values each) and cause
// gameport legacy I/O at 0x0200 to be _DISABLED_!
// Is this Digital Enhanced Game Port Enable??? Or maybe it's Testmode
// for Enhanced Digital Gameport (see 4D Wave DX card):
pub const IO_6A_SOMETHING1_GAMEPORT: c_uint = 0x0020;
// bit 8; sure, this _pauses_ playback (later resumes at same spot!),
// but what the heck is this really about??:
pub const IO_6A_PAUSE_PLAYBACK_BIT8: c_uint = 0x0100;
// bit 9; sure, this _pauses_ playback (later resumes at same spot!),
// but what the heck is this really about??:
pub const IO_6A_PAUSE_PLAYBACK_BIT9: c_uint = 0x0200;
// BIT8 and BIT9 are _NOT_ able to affect OPL3 MIDI playback,
// thus it suggests influence on PCM only!!
// However OTOH there seems to be no bit anywhere around here
// which is able to disable OPL3...
// bit 10: enabling this actually changes values at legacy gameport
// I/O address (0x200); is this enabling of the Digital Enhanced Game Port???
// Or maybe this simply switches off the NE558 circuit, since enabling this
// still lets us evaluate button states, but not axis states
pub const IO_6A_SOMETHING2_GAMEPORT: c_uint = 0x0400;
// writing 0x0300: causes quite some crackling during
// PC activity such as switching windows (PCI traffic??
// --> FIFO/timing settings???)
// writing 0x0100 plus/or 0x0200 inhibits playback
// since the Windows .INF file has Flag_Enable_JoyStick and
// Flag_Enable_SB_DOS_Emulation directly together, it stands to reason
// that some other bit in this same register might be responsible
// for SB DOS Emulation activation (note that the file did NOT define
// a switch for OPL3!)
pub const IDX_IO_6CH: c_uint = 0x6C	/* unknown; fully read-writable */;
pub const IDX_IO_6EH: c_uint = 0x6E;
// writing 0xffff returns 0x83fe (or 0x03fe only).
// writing 0x83 (and only 0x83!!) to 0x6f will cause 0x6c to switch
// from 0000 to ffff.
// further I/O indices not saved/restored and not readable after writing,
// so probably not used
// Gameport area port indices
// (only 0x06 of 0x08 bytes saved/restored by Windows driver)
pub const AZF_IO_SIZE_GAME: c_uint = 0x08;
pub const AZF_IO_SIZE_GAME_PM: c_uint = 0x06;
pub const IDX_GAME_LEGACY_COMPATIBLE: c_uint = 0x00;
// in some operation mode, writing anything to this port
// triggers an interrupt:
// yup, that's in case IDX_GAME_01H has one of the
// axis measurement bits enabled
// (and of course one needs to have GAME_HWCFG_IRQ_ENABLE, too)
pub const IDX_GAME_AXES_CONFIG: c_uint = 0x01;
// NOTE: layout of this register awfully similar (read: "identical??")
// to AD1815JS.pdf (p.29)
// enables axis 1 (X axis) measurement:
pub const GAME_AXES_ENABLE_1: c_uint = 0x01;
// enables axis 2 (Y axis) measurement:
pub const GAME_AXES_ENABLE_2: c_uint = 0x02;
// enables axis 3 (X axis) measurement:
pub const GAME_AXES_ENABLE_3: c_uint = 0x04;
// enables axis 4 (Y axis) measurement:
pub const GAME_AXES_ENABLE_4: c_uint = 0x08;
// selects the current axis to read the measured value of
// (at IDX_GAME_AXIS_VALUE):
// 00 = axis 1, 01 = axis 2, 10 = axis 3, 11 = axis 4:
pub const GAME_AXES_READ_MASK: c_uint = 0x30;
// enable to have the latch continuously accept ADC values
// (and continuously cause interrupts in case interrupts are enabled);
// AD1815JS.pdf says it's ~16ms interval there:
pub const GAME_AXES_LATCH_ENABLE: c_uint = 0x40;
// joystick data (measured axes) ready for reading:
pub const GAME_AXES_SAMPLING_READY: c_uint = 0x80;
// NOTE: other card specs (SiS960 and others!) state that the
// game position latches should be frozen when reading and be freed
// (== reset?) after reading!!!
// Freezing most likely means disabling 0x40 (GAME_AXES_LATCH_ENABLE),
// but how to free the value?
// An internet search for "gameport latch ADC" should provide some insight
// into how to program such a gameport system.
// writing 0xf0 to 01H once reset both counters to 0, in some special mode!?
// yup, in case 6AH 0x20 is not enabled
// (and 0x40 is sufficient, 0xf0 is not needed)
pub const IDX_GAME_AXIS_VALUE: c_uint = 0x02;
// R: value of currently configured axis (word value!);
// W: trigger axis measurement
pub const IDX_GAME_HWCONFIG: c_uint = 0x04;
// note: bits 4 to 7 are never set (== 0) when reading!
// --> reserved bits?
// enables IRQ notification upon axes measurement ready:
pub const GAME_HWCFG_IRQ_ENABLE: c_uint = 0x01;
// these bits choose a different frequency for the
// internal ADC counter increment.
// hmm, seems to be a combo of bits:
// 00 --> standard frequency
// 10 --> 1/2
// 01 --> 1/20
// 11 --> 1/200:
pub const GAME_HWCFG_ADC_COUNTER_FREQ_MASK: c_uint = 0x06;
// FIXME: these values might be reversed...
pub const GAME_HWCFG_ADC_COUNTER_FREQ_STD: c_int = 0;
pub const GAME_HWCFG_ADC_COUNTER_FREQ_1_2: c_int = 1;
pub const GAME_HWCFG_ADC_COUNTER_FREQ_1_20: c_int = 2;
pub const GAME_HWCFG_ADC_COUNTER_FREQ_1_200: c_int = 3;
// enable gameport legacy I/O address (0x200)
// I was unable to locate any configurability for a different address:
pub const GAME_HWCFG_LEGACY_ADDRESS_ENABLE: c_uint = 0x08;
// MPU401
pub const AZF_IO_SIZE_MPU: c_uint = 0x04;
pub const AZF_IO_SIZE_MPU_PM: c_uint = 0x04;
// OPL3 synth
// (only 0x06 of 0x08 bytes saved/restored by Windows driver)
pub const AZF_IO_SIZE_OPL3: c_uint = 0x08;
pub const AZF_IO_SIZE_OPL3_PM: c_uint = 0x06;
// hmm, given that a standard OPL3 has 4 registers only,
// there might be some enhanced functionality lurking at the end
// (especially since register 0x04 has a "non-empty" value 0xfe)
// mixer I/O area port indices
// (only 0x22 of 0x40 bytes saved/restored by Windows driver)
// UNFORTUNATELY azf3328 is NOT truly AC97 compliant: see main file intro
pub const AZF_IO_SIZE_MIXER: c_uint = 0x40;
pub const AZF_IO_SIZE_MIXER_PM: c_uint = 0x22;
pub const MIXER_VOLUME_RIGHT_MASK: c_uint = 0x001f;
pub const MIXER_VOLUME_LEFT_MASK: c_uint = 0x1f00;
pub const MIXER_MUTE_MASK: c_uint = 0x8000;
pub const IDX_MIXER_RESET: c_uint = 0x00 /* does NOT seem to have AC97 ID bits */;
pub const IDX_MIXER_PLAY_MASTER: c_uint = 0x02;
pub const IDX_MIXER_MODEMOUT: c_uint = 0x04;
pub const IDX_MIXER_BASSTREBLE: c_uint = 0x06;
pub const MIXER_BASSTREBLE_TREBLE_VOLUME_MASK: c_uint = 0x000e;
pub const MIXER_BASSTREBLE_BASS_VOLUME_MASK: c_uint = 0x0e00;
pub const IDX_MIXER_PCBEEP: c_uint = 0x08;
pub const IDX_MIXER_MODEMIN: c_uint = 0x0a;
pub const IDX_MIXER_MIC: c_uint = 0x0c;
pub const MIXER_MIC_MICGAIN_20DB_ENHANCEMENT_MASK: c_uint = 0x0040;
pub const IDX_MIXER_LINEIN: c_uint = 0x0e;
pub const IDX_MIXER_CDAUDIO: c_uint = 0x10;
pub const IDX_MIXER_VIDEO: c_uint = 0x12;
pub const IDX_MIXER_AUX: c_uint = 0x14;
pub const IDX_MIXER_WAVEOUT: c_uint = 0x16;
pub const IDX_MIXER_FMSYNTH: c_uint = 0x18;
pub const IDX_MIXER_REC_SELECT: c_uint = 0x1a;
pub const MIXER_REC_SELECT_MIC: c_uint = 0x00;
pub const MIXER_REC_SELECT_CD: c_uint = 0x01;
pub const MIXER_REC_SELECT_VIDEO: c_uint = 0x02;
pub const MIXER_REC_SELECT_AUX: c_uint = 0x03;
pub const MIXER_REC_SELECT_LINEIN: c_uint = 0x04;
pub const MIXER_REC_SELECT_MIXSTEREO: c_uint = 0x05;
pub const MIXER_REC_SELECT_MIXMONO: c_uint = 0x06;
pub const MIXER_REC_SELECT_MONOIN: c_uint = 0x07;
pub const IDX_MIXER_REC_VOLUME: c_uint = 0x1c;
pub const IDX_MIXER_ADVCTL1: c_uint = 0x1e;
// unlisted bits are unmodifiable
pub const MIXER_ADVCTL1_3DWIDTH_MASK: c_uint = 0x000e;
pub const MIXER_ADVCTL1_HIFI3D_MASK: c_uint = 0x0300 /* yup, this is missing the high bit that official AC97 contains, plus it doesn't have linear bit value range behaviour but instead acts weirdly (possibly we're dealing with two *different* 3D settings here??) */;
pub const IDX_MIXER_ADVCTL2: c_uint = 0x20 /* subset of AC97_GENERAL_PURPOSE reg! */;
// unlisted bits are unmodifiable
pub const MIXER_ADVCTL2_LPBK: c_uint = 0x0080 /* Loopback mode -- Win driver: "WaveOut3DBypass"? mutes WaveOut at LineOut */;
pub const MIXER_ADVCTL2_MS: c_uint = 0x0100 /* Mic Select 0=Mic1, 1=Mic2 -- Win driver: "ModemOutSelect"?? */;
pub const MIXER_ADVCTL2_MIX: c_uint = 0x0200 /* Mono output select 0=Mix, 1=Mic; Win driver: "MonoSelectSource"?? */;
pub const MIXER_ADVCTL2_3D: c_uint = 0x2000 /* 3D Enhancement 1=on */;
pub const MIXER_ADVCTL2_POP: c_uint = 0x8000 /* Pcm Out Path, 0=pre 3D, 1=post 3D */;
pub const IDX_MIXER_SOMETHING30H: c_uint = 0x30 /* used, but unknown??? */;
// driver internal flags
pub const SET_CHAN_LEFT: c_int = 1;
pub const SET_CHAN_RIGHT: c_int = 2;
// helper macro to align I/O port ranges to 32bit I/O width

