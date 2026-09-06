//! Automatically rewritten from C Header to Rust Module
//! Source: sound/pci/ca0106/ca0106.h
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
// Copyright (c) 2004 James Courtier-Dutton <James@superbug.demon.co.uk>
// Driver CA0106 chips. e.g. Sound Blaster Audigy LS and Live 24bit
// Version: 0.0.22
//
// FEATURES currently supported:
// See ca0106_main.c for features.
//
// Changelog:
// Support interrupts per period.
// Removed noise from Center/LFE channel when in Analog mode.
// Rename and remove mixer controls.
// 0.0.6
// Use separate card based DMA buffer for periods table list.
// 0.0.7
// Change remove and rename ctrls into lists.
// 0.0.8
// Try to fix capture sources.
// 0.0.9
// Fix AC3 output.
// Enable S32_LE format support.
// 0.0.10
// Enable playback 48000 and 96000 rates. (Rates other that these do not work, even with "plug:front".)
// 0.0.11
// Add Model name recognition.
// 0.0.12
// Correct interrupt timing. interrupt at end of period, instead of in the middle of a playback period.
// Remove redundent "voice" handling.
// 0.0.13
// Single trigger call for multi channels.
// 0.0.14
// Set limits based on what the sound card hardware can do.
// playback periods_min=2, periods_max=8
// capture hw constraints require period_size = n * 64 bytes.
// playback hw constraints require period_size = n * 64 bytes.
// 0.0.15
// Separated ca0106.c into separate functional .c files.
// 0.0.16
// Implement 192000 sample rate.
// 0.0.17
// Add support for SB0410 and SB0413.
// 0.0.18
// Modified Copyright message.
// 0.0.19
// Added I2C and SPI registers. Filled in interrupt enable.
// 0.0.20
// Added GPIO info for SB Live 24bit.
// 0.0.21
// Implement support for Line-in capture on SB Live 24bit.
// 0.0.22
// Add support for mute control on SB Live 24bit (cards w/ SPI DAC)
//
// This code was initially based on code from ALSA's emu10k1x.c which is:
// Copyright (c) by Francisco Moraes <fmoraes@nc.rr.com>
//
// PCI function 0 registers, address = <val> + PCIBASE0
//
pub const CA0106_PTR: c_uint = 0x00		/* Indexed register set pointer register	*/;
// NOTE: The CHANNELNUM and ADDRESS words can
// be modified independently of each other.
// CNL[1:0], ADDR[27:16]
pub const CA0106_DATA: c_uint = 0x04		/* Indexed register set data register		*/;
// DATA[31:0]
pub const CA0106_IPR: c_uint = 0x08		/* Global interrupt pending register		*/;
// Clear pending interrupts by writing a 1 to
// the relevant bits and zero to the other bits
pub const IPR_MIDI_RX_B: c_uint = 0x00020000	/* MIDI UART-B Receive buffer non-empty		*/;
pub const IPR_MIDI_TX_B: c_uint = 0x00010000	/* MIDI UART-B Transmit buffer empty		*/;
pub const IPR_SPDIF_IN_USER: c_uint = 0x00004000      /* SPDIF input user data has 16 more bits	*/;
pub const IPR_SPDIF_OUT_USER: c_uint = 0x00002000      /* SPDIF output user data needs 16 more bits	*/;
pub const IPR_SPDIF_OUT_FRAME: c_uint = 0x00001000      /* SPDIF frame about to start			*/;
pub const IPR_SPI: c_uint = 0x00000800      /* SPI transaction completed			*/;
pub const IPR_I2C_EEPROM: c_uint = 0x00000400      /* I2C EEPROM transaction completed		*/;
pub const IPR_I2C_DAC: c_uint = 0x00000200      /* I2C DAC transaction completed		*/;
pub const IPR_AI: c_uint = 0x00000100      /* Audio pending register changed. See PTR reg 0x76	*/;
pub const IPR_GPI: c_uint = 0x00000080      /* General Purpose input changed		*/;
pub const IPR_SRC_LOCKED: c_uint = 0x00000040      /* SRC lock status changed			*/;
pub const IPR_SPDIF_STATUS: c_uint = 0x00000020      /* SPDIF status changed				*/;
pub const IPR_TIMER2: c_uint = 0x00000010      /* 192000Hz Timer				*/;
pub const IPR_TIMER1: c_uint = 0x00000008      /* 44100Hz Timer				*/;
pub const IPR_MIDI_RX_A: c_uint = 0x00000004	/* MIDI UART-A Receive buffer non-empty		*/;
pub const IPR_MIDI_TX_A: c_uint = 0x00000002	/* MIDI UART-A Transmit buffer empty		*/;
pub const IPR_PCI: c_uint = 0x00000001	/* PCI Bus error				*/;
pub const CA0106_INTE: c_uint = 0x0c		/* Interrupt enable register			*/;
pub const INTE_MIDI_RX_B: c_uint = 0x00020000	/* MIDI UART-B Receive buffer non-empty		*/;
pub const INTE_MIDI_TX_B: c_uint = 0x00010000	/* MIDI UART-B Transmit buffer empty		*/;
pub const INTE_SPDIF_IN_USER: c_uint = 0x00004000      /* SPDIF input user data has 16 more bits	*/;
pub const INTE_SPDIF_OUT_USER: c_uint = 0x00002000      /* SPDIF output user data needs 16 more bits	*/;
pub const INTE_SPDIF_OUT_FRAME: c_uint = 0x00001000      /* SPDIF frame about to start			*/;
pub const INTE_SPI: c_uint = 0x00000800      /* SPI transaction completed			*/;
pub const INTE_I2C_EEPROM: c_uint = 0x00000400      /* I2C EEPROM transaction completed		*/;
pub const INTE_I2C_DAC: c_uint = 0x00000200      /* I2C DAC transaction completed		*/;
pub const INTE_AI: c_uint = 0x00000100      /* Audio pending register changed. See PTR reg 0x75 */;
pub const INTE_GPI: c_uint = 0x00000080      /* General Purpose input changed		*/;
pub const INTE_SRC_LOCKED: c_uint = 0x00000040      /* SRC lock status changed			*/;
pub const INTE_SPDIF_STATUS: c_uint = 0x00000020      /* SPDIF status changed				*/;
pub const INTE_TIMER2: c_uint = 0x00000010      /* 192000Hz Timer				*/;
pub const INTE_TIMER1: c_uint = 0x00000008      /* 44100Hz Timer				*/;
pub const INTE_MIDI_RX_A: c_uint = 0x00000004	/* MIDI UART-A Receive buffer non-empty		*/;
pub const INTE_MIDI_TX_A: c_uint = 0x00000002	/* MIDI UART-A Transmit buffer empty		*/;
pub const INTE_PCI: c_uint = 0x00000001	/* PCI Bus error				*/;
pub const CA0106_UNKNOWN10: c_uint = 0x10		/* Unknown ??. Defaults to 0 */;
pub const CA0106_HCFG: c_uint = 0x14		/* Hardware config register			*/;
// 0x1000 causes AC3 to fails. It adds a dither bit.
pub const HCFG_STAC: c_uint = 0x10000000	/* Special mode for STAC9460 Codec. */;
pub const HCFG_CAPTURE_I2S_BYPASS: c_uint = 0x08000000	/* 1 = bypass I2S input async SRC. */;
pub const HCFG_CAPTURE_SPDIF_BYPASS: c_uint = 0x04000000	/* 1 = bypass SPDIF input async SRC. */;
pub const HCFG_PLAYBACK_I2S_BYPASS: c_uint = 0x02000000	/* 0 = I2S IN mixer output, 1 = I2S IN1. */;
pub const HCFG_FORCE_LOCK: c_uint = 0x01000000	/* For test only. Force input SRC tracker to lock. */;
pub const HCFG_PLAYBACK_ATTENUATION: c_uint = 0x00006000	/* Playback attenuation mask. 0 = 0dB, 1 = 6dB, 2 = 12dB, 3 = Mute. */;
pub const HCFG_PLAYBACK_DITHER: c_uint = 0x00001000	/* 1 = Add dither bit to all playback channels. */;
pub const HCFG_PLAYBACK_S32_LE: c_uint = 0x00000800	/* 1 = S32_LE, 0 = S16_LE                       */;
pub const HCFG_CAPTURE_S32_LE: c_uint = 0x00000400	/* 1 = S32_LE, 0 = S16_LE (S32_LE current not working)	*/;
pub const HCFG_8_CHANNEL_PLAY: c_uint = 0x00000200	/* 1 = 8 channels, 0 = 2 channels per substream.*/;
pub const HCFG_8_CHANNEL_CAPTURE: c_uint = 0x00000100	/* 1 = 8 channels, 0 = 2 channels per substream.*/;
pub const HCFG_MONO: c_uint = 0x00000080	/* 1 = I2S Input mono                           */;
pub const HCFG_I2S_OUTPUT: c_uint = 0x00000010	/* 1 = I2S Output disabled                      */;
pub const HCFG_AC97: c_uint = 0x00000008	/* 0 = AC97 1.0, 1 = AC97 2.0                   */;
pub const HCFG_LOCK_PLAYBACK_CACHE: c_uint = 0x00000004	/* 1 = Cancel bustmaster accesses to soundcache */;
// NOTE: This should generally never be used.
pub const HCFG_LOCK_CAPTURE_CACHE: c_uint = 0x00000002	/* 1 = Cancel bustmaster accesses to soundcache */;
// NOTE: This should generally never be used.
pub const HCFG_AUDIOENABLE: c_uint = 0x00000001	/* 0 = CODECs transmit zero-valued samples	*/;
// Should be set to 1 when the EMU10K1 is
// completely initialized.
pub const CA0106_GPIO: c_uint = 0x18		/* Defaults: 005f03a3-Analog, 005f02a2-SPDIF.   */;
// Here pins 0,1,2,3,4,,6 are output. 5,7 are input
// For the Audigy LS, pin 0 (or bit 8) controls the SPDIF/Analog jack.
// SB Live 24bit:
// bit 8 0 = SPDIF in and out / 1 = Analog (Mic or Line)-in.
// bit 9 0 = Mute / 1 = Analog out.
// bit 10 0 = Line-in / 1 = Mic-in.
// bit 11 0 = ? / 1 = ?
// bit 12 0 = 48 Khz / 1 = 96 Khz Analog out on SB Live 24bit.
// bit 13 0 = ? / 1 = ?
// bit 14 0 = Mute / 1 = Analog out
// bit 15 0 = ? / 1 = ?
// Both bit 9 and bit 14 have to be set for analog sound to work on the SB Live 24bit.
//
// 8 general purpose programmable In/Out pins.
// GPI [8:0] Read only. Default 0.
// GPO [15:8] Default 0x9. (Default to SPDIF jack enabled for SPDIF)
// GPO Enable [23:16] Default 0x0f. Setting a bit to 1, causes the pin to be an output pin.
//
pub const CA0106_AC97DATA: c_uint = 0x1c		/* AC97 register set data register (16 bit)	*/;
pub const CA0106_AC97ADDRESS: c_uint = 0x1e		/* AC97 register set address register (8 bit)	*/;
//
// CA0106 pointer-offset register set, accessed through the PTR and DATA registers
//
// Initially all registers from 0x00 to 0x3f have zero contents.
pub const PLAYBACK_LIST_ADDR: c_uint = 0x00		/* Base DMA address of a list of pointers to each period/size */;
// One list entry: 4 bytes for DMA address,
// 4 bytes for period_size << 16.
// One list entry is 8 bytes long.
// One list entry for each period in the buffer.
//
// ADDR[31:0], Default: 0x0
pub const PLAYBACK_LIST_SIZE: c_uint = 0x01		/* Size of list in bytes << 16. E.g. 8 periods -> 0x00380000  */;
// SIZE[21:16], Default: 0x8
pub const PLAYBACK_LIST_PTR: c_uint = 0x02		/* Pointer to the current period being played */;
// PTR[5:0], Default: 0x0
pub const PLAYBACK_UNKNOWN3: c_uint = 0x03		/* Not used ?? */;
pub const PLAYBACK_DMA_ADDR: c_uint = 0x04		/* Playback DMA address */;
// DMA[31:0], Default: 0x0
pub const PLAYBACK_PERIOD_SIZE: c_uint = 0x05		/* Playback period size. win2000 uses 0x04000000 */;
// SIZE[31:16], Default: 0x0
pub const PLAYBACK_POINTER: c_uint = 0x06		/* Playback period pointer. Used with PLAYBACK_LIST_PTR to determine buffer position currently in DAC */;
// POINTER[15:0], Default: 0x0
pub const PLAYBACK_PERIOD_END_ADDR: c_uint = 0x07		/* Playback fifo end address */;
// END_ADDR[15:0], FLAG[16] 0 = don't stop, 1 = stop
pub const PLAYBACK_FIFO_OFFSET_ADDRESS: c_uint = 0x08	/* Current fifo offset address [21:16] */;
// Cache size valid [5:0]
pub const PLAYBACK_UNKNOWN9: c_uint = 0x09		/* 0x9 to 0xf Unused */;
pub const CAPTURE_DMA_ADDR: c_uint = 0x10		/* Capture DMA address */;
// DMA[31:0], Default: 0x0
pub const CAPTURE_BUFFER_SIZE: c_uint = 0x11		/* Capture buffer size */;
// SIZE[31:16], Default: 0x0
pub const CAPTURE_POINTER: c_uint = 0x12		/* Capture buffer pointer. Sample currently in ADC */;
// POINTER[15:0], Default: 0x0
pub const CAPTURE_FIFO_OFFSET_ADDRESS: c_uint = 0x13	/* Current fifo offset address [21:16] */;
// Cache size valid [5:0]
pub const PLAYBACK_LAST_SAMPLE: c_uint = 0x20		/* The sample currently being played */;
// 0x21 - 0x3f unused
pub const BASIC_INTERRUPT: c_uint = 0x40		/* Used by both playback and capture interrupt handler */;
// Playback (0x1<<channel_id)
// Capture  (0x100<<channel_id)
// Playback sample rate 96000 = 0x20000
// Start Playback [3:0] (one bit per channel)
// Start Capture [11:8] (one bit per channel)
// Playback rate [23:16] (2 bits per channel) (0=48kHz, 1=44.1kHz, 2=96kHz, 3=192Khz)
// Playback mixer in enable [27:24] (one bit per channel)
// Playback mixer out enable [31:28] (one bit per channel)
//
// The Digital out jack is shared with the Center/LFE Analogue output.
// The jack has 4 poles. I will call 1 - Tip, 2 - Next to 1, 3 - Next to 2, 4 - Next to 3
// For Analogue: 1 -> Center Speaker, 2 -> Sub Woofer, 3 -> Ground, 4 -> Ground
// For Digital: 1 -> Front SPDIF, 2 -> Rear SPDIF, 3 -> Center/Subwoofer SPDIF, 4 -> Ground.
// Standard 4 pole Video A/V cable with RCA outputs: 1 -> White, 2 -> Yellow, 3 -> Shield on all three, 4 -> Red.
// So, from this you can see that you cannot use a Standard 4 pole Video A/V cable with the SB Audigy LS card.
//
// The Front SPDIF PCM gets mixed with samples from the AC97 codec, so can only work for Stereo PCM and not AC3/DTS
// The Rear SPDIF can be used for Stereo PCM and also AC3/DTS
// The Center/LFE SPDIF cannot be used for AC3/DTS, but can be used for Stereo PCM.
// Summary: For ALSA we use the Rear channel for SPDIF Digital AC3/DTS output
//
// A standard 2 pole mono mini-jack to RCA plug can be used for SPDIF Stereo PCM output from the Front channel.
// A standard 3 pole stereo mini-jack to 2 RCA plugs can be used for SPDIF AC3/DTS and Stereo PCM output utilising the Rear channel and just one of the RCA plugs.
//
pub const SPCS0: c_uint = 0x41		/* SPDIF output Channel Status 0 register. For Rear. default=0x02108004, non-audio=0x02108006	*/;
pub const SPCS1: c_uint = 0x42		/* SPDIF output Channel Status 1 register. For Front */;
pub const SPCS2: c_uint = 0x43		/* SPDIF output Channel Status 2 register. For Center/LFE */;
pub const SPCS3: c_uint = 0x44		/* SPDIF output Channel Status 3 register. Unknown */;
// When Channel set to 0:
pub const SPCS_CLKACCYMASK: c_uint = 0x30000000	/* Clock accuracy				*/;
pub const SPCS_CLKACCY_1000PPM: c_uint = 0x00000000	/* 1000 parts per million			*/;
pub const SPCS_CLKACCY_50PPM: c_uint = 0x10000000	/* 50 parts per million				*/;
pub const SPCS_CLKACCY_VARIABLE: c_uint = 0x20000000	/* Variable accuracy				*/;
pub const SPCS_SAMPLERATEMASK: c_uint = 0x0f000000	/* Sample rate					*/;
pub const SPCS_SAMPLERATE_44: c_uint = 0x00000000	/* 44.1kHz sample rate				*/;
pub const SPCS_SAMPLERATE_48: c_uint = 0x02000000	/* 48kHz sample rate				*/;
pub const SPCS_SAMPLERATE_32: c_uint = 0x03000000	/* 32kHz sample rate				*/;
pub const SPCS_CHANNELNUMMASK: c_uint = 0x00f00000	/* Channel number				*/;
pub const SPCS_CHANNELNUM_UNSPEC: c_uint = 0x00000000	/* Unspecified channel number			*/;
pub const SPCS_CHANNELNUM_LEFT: c_uint = 0x00100000	/* Left channel					*/;
pub const SPCS_CHANNELNUM_RIGHT: c_uint = 0x00200000	/* Right channel				*/;
pub const SPCS_SOURCENUMMASK: c_uint = 0x000f0000	/* Source number				*/;
pub const SPCS_SOURCENUM_UNSPEC: c_uint = 0x00000000	/* Unspecified source number			*/;
pub const SPCS_GENERATIONSTATUS: c_uint = 0x00008000	/* Originality flag (see IEC-958 spec)		*/;
pub const SPCS_CATEGORYCODEMASK: c_uint = 0x00007f00	/* Category code (see IEC-958 spec)		*/;
pub const SPCS_MODEMASK: c_uint = 0x000000c0	/* Mode (see IEC-958 spec)			*/;
pub const SPCS_EMPHASISMASK: c_uint = 0x00000038	/* Emphasis					*/;
pub const SPCS_EMPHASIS_NONE: c_uint = 0x00000000	/* No emphasis					*/;
pub const SPCS_EMPHASIS_50_15: c_uint = 0x00000008	/* 50/15 usec 2 channel				*/;
pub const SPCS_COPYRIGHT: c_uint = 0x00000004	/* Copyright asserted flag -- do not modify	*/;
pub const SPCS_NOTAUDIODATA: c_uint = 0x00000002	/* 0 = Digital audio, 1 = not audio		*/;
pub const SPCS_PROFESSIONAL: c_uint = 0x00000001	/* 0 = Consumer (IEC-958), 1 = pro (AES3-1992)	*/;
// When Channel set to 1:
pub const SPCS_WORD_LENGTH_MASK: c_uint = 0x0000000f	/* Word Length Mask				*/;
pub const SPCS_WORD_LENGTH_16: c_uint = 0x00000008	/* Word Length 16 bit				*/;
pub const SPCS_WORD_LENGTH_17: c_uint = 0x00000006	/* Word Length 17 bit				*/;
pub const SPCS_WORD_LENGTH_18: c_uint = 0x00000004	/* Word Length 18 bit				*/;
pub const SPCS_WORD_LENGTH_19: c_uint = 0x00000002	/* Word Length 19 bit				*/;
pub const SPCS_WORD_LENGTH_20A: c_uint = 0x0000000a	/* Word Length 20 bit				*/;
pub const SPCS_WORD_LENGTH_20: c_uint = 0x00000009	/* Word Length 20 bit (both 0xa and 0x9 are 20 bit) */;
pub const SPCS_WORD_LENGTH_21: c_uint = 0x00000007	/* Word Length 21 bit				*/;
pub const SPCS_WORD_LENGTH_22: c_uint = 0x00000005	/* Word Length 22 bit				*/;
pub const SPCS_WORD_LENGTH_23: c_uint = 0x00000003	/* Word Length 23 bit				*/;
pub const SPCS_WORD_LENGTH_24: c_uint = 0x0000000b	/* Word Length 24 bit				*/;
pub const SPCS_ORIGINAL_SAMPLE_RATE_MASK: c_uint = 0x000000f0 /* Original Sample rate			*/;
pub const SPCS_ORIGINAL_SAMPLE_RATE_NONE: c_uint = 0x00000000 /* Original Sample rate not indicated	*/;
pub const SPCS_ORIGINAL_SAMPLE_RATE_16000: c_uint = 0x00000010 /* Original Sample rate	*/;
pub const SPCS_ORIGINAL_SAMPLE_RATE_RES1: c_uint = 0x00000020 /* Original Sample rate	*/;
pub const SPCS_ORIGINAL_SAMPLE_RATE_32000: c_uint = 0x00000030 /* Original Sample rate	*/;
pub const SPCS_ORIGINAL_SAMPLE_RATE_12000: c_uint = 0x00000040 /* Original Sample rate	*/;
pub const SPCS_ORIGINAL_SAMPLE_RATE_11025: c_uint = 0x00000050 /* Original Sample rate	*/;
pub const SPCS_ORIGINAL_SAMPLE_RATE_8000: c_uint = 0x00000060 /* Original Sample rate	*/;
pub const SPCS_ORIGINAL_SAMPLE_RATE_RES2: c_uint = 0x00000070 /* Original Sample rate	*/;
pub const SPCS_ORIGINAL_SAMPLE_RATE_192000: c_uint = 0x00000080 /* Original Sample rate	*/;
pub const SPCS_ORIGINAL_SAMPLE_RATE_24000: c_uint = 0x00000090 /* Original Sample rate	*/;
pub const SPCS_ORIGINAL_SAMPLE_RATE_96000: c_uint = 0x000000a0 /* Original Sample rate	*/;
pub const SPCS_ORIGINAL_SAMPLE_RATE_48000: c_uint = 0x000000b0 /* Original Sample rate	*/;
pub const SPCS_ORIGINAL_SAMPLE_RATE_176400: c_uint = 0x000000c0 /* Original Sample rate	*/;
pub const SPCS_ORIGINAL_SAMPLE_RATE_22050: c_uint = 0x000000d0 /* Original Sample rate	*/;
pub const SPCS_ORIGINAL_SAMPLE_RATE_88200: c_uint = 0x000000e0 /* Original Sample rate	*/;
pub const SPCS_ORIGINAL_SAMPLE_RATE_44100: c_uint = 0x000000f0 /* Original Sample rate	*/;
pub const SPDIF_SELECT1: c_uint = 0x45		/* Enables SPDIF or Analogue outputs 0-SPDIF, 0xf00-Analogue */;
// 0x100 - Front, 0x800 - Rear, 0x200 - Center/LFE.
// But as the jack is shared, use 0xf00.
// The Windows2000 driver uses 0x0000000f for both digital and analog.
// 0xf00 introduces interesting noises onto the Center/LFE.
// If you turn the volume up, you hear computer noise,
// e.g. mouse moving, changing between app windows etc.
// So, I am going to set this to 0x0000000f all the time now,
// same as the windows driver does.
// Use register SPDIF_SELECT2(0x72) to switch between SPDIF and Analog.
//
// When Channel = 0:
// Wide SPDIF format [3:0] (one bit for each channel) (0=20bit, 1=24bit)
// Tristate SPDIF Output [11:8] (one bit for each channel) (0=Not tristate, 1=Tristate)
// SPDIF Bypass enable [19:16] (one bit for each channel) (0=Not bypass, 1=Bypass)
//
// When Channel = 1:
// SPDIF 0 User data [7:0]
// SPDIF 1 User data [15:8]
// SPDIF 0 User data [23:16]
// SPDIF 0 User data [31:24]
// User data can be sent by using the SPDIF output frame pending and SPDIF output user bit interrupts.
//
pub const WATERMARK: c_uint = 0x46		/* Test bit to indicate cache usage level */;
pub const SPDIF_INPUT_STATUS: c_uint = 0x49		/* SPDIF Input status register. Bits the same as SPCS.;
// When Channel = 0: Bits the same as SPCS channel 0.
// When Channel = 1: Bits the same as SPCS channel 1.
// When Channel = 2:
// SPDIF Input User data [16:0]
// SPDIF Input Frame count [21:16]
//
pub const CAPTURE_CACHE_DATA: c_uint = 0x50		/* 0x50-0x5f Recorded samples. */;
pub const CAPTURE_SOURCE: c_uint = 0x60            /* Capture Source 0 = MIC */;
pub const CAPTURE_SOURCE_CHANNEL0: c_uint = 0xf0000000	/* Mask for selecting the Capture sources */;
pub const CAPTURE_SOURCE_CHANNEL1: c_uint = 0x0f000000	/* 0 - SPDIF mixer output. */;
pub const CAPTURE_SOURCE_CHANNEL2: c_uint = 0x00f00000      /* 1 - What you hear or . 2 - ?? */;
pub const CAPTURE_SOURCE_CHANNEL3: c_uint = 0x000f0000	/* 3 - Mic in, Line in, TAD in, Aux in. */;
pub const CAPTURE_SOURCE_RECORD_MAP: c_uint = 0x0000ffff	/* Default 0x00e4 */;
// Record Map [7:0] (2 bits per channel) 0=mapped to channel 0, 1=mapped to channel 1, 2=mapped to channel2, 3=mapped to channel3
// Record source select for channel 0 [18:16]
// Record source select for channel 1 [22:20]
// Record source select for channel 2 [26:24]
// Record source select for channel 3 [30:28]
// 0 - SPDIF mixer output.
// 1 - i2s mixer output.
// 2 - SPDIF input.
// 3 - i2s input.
// 4 - AC97 capture.
// 5 - SRC output.
//
pub const CAPTURE_VOLUME1: c_uint = 0x61            /* Capture  volume per channel 0-3 */;
pub const CAPTURE_VOLUME2: c_uint = 0x62            /* Capture  volume per channel 4-7 */;
pub const PLAYBACK_ROUTING1: c_uint = 0x63            /* Playback routing of channels 0-7. Effects AC3 output. Default 0x32765410 */;
pub const ROUTING1_REAR: c_uint = 0x77000000      /* Channel_id 0 sends to 10, Channel_id 1 sends to 32 */;
pub const ROUTING1_NULL: c_uint = 0x00770000      /* Channel_id 2 sends to 54, Channel_id 3 sends to 76 */;
pub const ROUTING1_CENTER_LFE: c_uint = 0x00007700      /* 0x32765410 means, send Channel_id 0 to FRONT, Channel_id 1 to REAR */;
pub const ROUTING1_FRONT: c_uint = 0x00000077	/* Channel_id 2 to CENTER_LFE, Channel_id 3 to NULL. */;
// Channel_id's handle stereo channels. Channel X is a single mono channel
// Host is input from the PCI bus.
// Host channel 0 [2:0] -> SPDIF Mixer/Router channel 0-7.
// Host channel 1 [6:4] -> SPDIF Mixer/Router channel 0-7.
// Host channel 2 [10:8] -> SPDIF Mixer/Router channel 0-7.
// Host channel 3 [14:12] -> SPDIF Mixer/Router channel 0-7.
// Host channel 4 [18:16] -> SPDIF Mixer/Router channel 0-7.
// Host channel 5 [22:20] -> SPDIF Mixer/Router channel 0-7.
// Host channel 6 [26:24] -> SPDIF Mixer/Router channel 0-7.
// Host channel 7 [30:28] -> SPDIF Mixer/Router channel 0-7.
//
pub const PLAYBACK_ROUTING2: c_uint = 0x64            /* Playback Routing . Feeding Capture channels back into Playback. Effects AC3 output. Default 0x76767676 */;
// SRC is input from the capture inputs.
// SRC channel 0 [2:0] -> SPDIF Mixer/Router channel 0-7.
// SRC channel 1 [6:4] -> SPDIF Mixer/Router channel 0-7.
// SRC channel 2 [10:8] -> SPDIF Mixer/Router channel 0-7.
// SRC channel 3 [14:12] -> SPDIF Mixer/Router channel 0-7.
// SRC channel 4 [18:16] -> SPDIF Mixer/Router channel 0-7.
// SRC channel 5 [22:20] -> SPDIF Mixer/Router channel 0-7.
// SRC channel 6 [26:24] -> SPDIF Mixer/Router channel 0-7.
// SRC channel 7 [30:28] -> SPDIF Mixer/Router channel 0-7.
//
pub const PLAYBACK_MUTE: c_uint = 0x65            /* Unknown. While playing 0x0, while silent 0x00fc0000 */;
// SPDIF Mixer input control:
// Invert SRC to SPDIF Mixer [7-0] (One bit per channel)
// Invert Host to SPDIF Mixer [15:8] (One bit per channel)
// SRC to SPDIF Mixer disable [23:16] (One bit per channel)
// Host to SPDIF Mixer disable [31:24] (One bit per channel)
//
pub const PLAYBACK_VOLUME1: c_uint = 0x66            /* Playback SPDIF volume per channel. Set to the same PLAYBACK_VOLUME(0x6a) */;
// PLAYBACK_VOLUME1 must be set to 30303030 for SPDIF AC3 Playback
// SPDIF mixer input volume. 0=12dB, 0x30=0dB, 0xFE=-51.5dB, 0xff=Mute
// One register for each of the 4 stereo streams.
// SRC Right volume [7:0]
// SRC Left  volume [15:8]
// Host Right volume [23:16]
// Host Left  volume [31:24]
//
pub const CAPTURE_ROUTING1: c_uint = 0x67            /* Capture Routing. Default 0x32765410 */;
// Similar to register 0x63, except that the destination is the I2S mixer instead of the SPDIF mixer. I.E. Outputs to the Analog outputs instead of SPDIF.
pub const CAPTURE_ROUTING2: c_uint = 0x68            /* Unknown Routing. Default 0x76767676 */;
// Similar to register 0x64, except that the destination is the I2S mixer instead of the SPDIF mixer. I.E. Outputs to the Analog outputs instead of SPDIF.
pub const CAPTURE_MUTE: c_uint = 0x69            /* Unknown. While capturing 0x0, while silent 0x00fc0000 */;
// Similar to register 0x65, except that the destination is the I2S mixer instead of the SPDIF mixer. I.E. Outputs to the Analog outputs instead of SPDIF.
pub const PLAYBACK_VOLUME2: c_uint = 0x6a            /* Playback Analog volume per channel. Does not effect AC3 output */;
// Similar to register 0x66, except that the destination is the I2S mixer instead of the SPDIF mixer. I.E. Outputs to the Analog outputs instead of SPDIF.
pub const UNKNOWN6b: c_uint = 0x6b            /* Unknown. Readonly. Default 00400000 00400000 00400000 00400000 */;
pub const MIDI_UART_A_DATA: c_uint = 0x6c            /* Midi Uart A Data */;
pub const MIDI_UART_A_CMD: c_uint = 0x6d            /* Midi Uart A Command/Status */;
pub const MIDI_UART_B_DATA: c_uint = 0x6e            /* Midi Uart B Data (currently unused) */;
pub const MIDI_UART_B_CMD: c_uint = 0x6f            /* Midi Uart B Command/Status (currently unused) */;
// unique channel identifier for midi->channel
pub const CA0106_MIDI_CHAN_A: c_uint = 0x1;
pub const CA0106_MIDI_CHAN_B: c_uint = 0x2;
// from mpu401
pub const CA0106_MIDI_INPUT_AVAIL: c_uint = 0x80;
pub const CA0106_MIDI_OUTPUT_READY: c_uint = 0x40;
pub const CA0106_MPU401_RESET: c_uint = 0xff;
pub const CA0106_MPU401_ENTER_UART: c_uint = 0x3f;
pub const CA0106_MPU401_ACK: c_uint = 0xfe;
pub const SAMPLE_RATE_TRACKER_STATUS: c_uint = 0x70         /* Readonly. Default 00108000 00108000 00500000 00500000 */;
// Estimated sample rate [19:0] Relative to 48kHz. 0x8000 =  1.0
// Rate Locked [20]
// SPDIF Locked [21] For SPDIF channel only.
// Valid Audio [22] For SPDIF channel only.
//
pub const CAPTURE_CONTROL: c_uint = 0x71            /* Some sort of routing. default = 40c81000 30303030 30300000 00700000 */;
// Channel_id 0: 0x40c81000 must be changed to 0x40c80000 for SPDIF AC3 input or output.
// Channel_id 1: 0xffffffff(mute) 0x30303030(max) controls CAPTURE feedback into PLAYBACK.
// Sample rate output control register Channel=0
// Sample output rate [1:0] (0=48kHz, 1=44.1kHz, 2=96kHz, 3=192Khz)
// Sample input rate [3:2] (0=48kHz, 1=Not available, 2=96kHz, 3=192Khz)
// SRC input source select [4] 0=Audio from digital mixer, 1=Audio from analog source.
// Record rate [9:8] (0=48kHz, 1=Not available, 2=96kHz, 3=192Khz)
// Record mixer output enable [12:10]
// I2S input rate master mode [15:14] (0=48kHz, 1=44.1kHz, 2=96kHz, 3=192Khz)
// I2S output rate [17:16] (0=48kHz, 1=44.1kHz, 2=96kHz, 3=192Khz)
// I2S output source select [18] (0=Audio from host, 1=Audio from SRC)
// Record mixer I2S enable [20:19] (enable/disable i2sin1 and i2sin0)
// I2S output master clock select [21] (0=256*I2S output rate, 1=512*I2S output rate.)
// I2S input master clock select [22] (0=256*I2S input rate, 1=512*I2S input rate.)
// I2S input mode [23] (0=Slave, 1=Master)
// SPDIF output rate [25:24] (0=48kHz, 1=44.1kHz, 2=96kHz, 3=192Khz)
// SPDIF output source select [26] (0=host, 1=SRC)
// Not used [27]
// Record Source 0 input [29:28] (0=SPDIF in, 1=I2S in, 2=AC97 Mic, 3=AC97 PCM)
// Record Source 1 input [31:30] (0=SPDIF in, 1=I2S in, 2=AC97 Mic, 3=AC97 PCM)
//
// Sample rate output control register Channel=1
// I2S Input 0 volume Right [7:0]
// I2S Input 0 volume Left [15:8]
// I2S Input 1 volume Right [23:16]
// I2S Input 1 volume Left [31:24]
//
// Sample rate output control register Channel=2
// SPDIF Input volume Right [23:16]
// SPDIF Input volume Left [31:24]
//
// Sample rate output control register Channel=3
// No used
//
pub const SPDIF_SELECT2: c_uint = 0x72            /* Some sort of routing. Channel_id 0 only. default = 0x0f0f003f. Analog 0x000b0000, Digital 0x0b000000 */;
pub const ROUTING2_FRONT_MASK: c_uint = 0x00010000      /* Enable for Front speakers. */;
pub const ROUTING2_CENTER_LFE_MASK: c_uint = 0x00020000     /* Enable for Center/LFE speakers. */;
pub const ROUTING2_REAR_MASK: c_uint = 0x00080000      /* Enable for Rear speakers. */;
// Audio output control
// AC97 output enable [5:0]
// I2S output enable [19:16]
// SPDIF output enable [27:24]
//
pub const UNKNOWN73: c_uint = 0x73            /* Unknown. Readonly. Default 0x0 */;
pub const CHIP_VERSION: c_uint = 0x74            /* P17 Chip version. Channel_id 0 only. Default 00000071 */;
pub const EXTENDED_INT_MASK: c_uint = 0x75            /* Used by both playback and capture interrupt handler */;
// Sets which Interrupts are enabled.
// 0x00000001 = Half period. Playback.
// 0x00000010 = Full period. Playback.
// 0x00000100 = Half buffer. Playback.
// 0x00001000 = Full buffer. Playback.
// 0x00010000 = Half buffer. Capture.
// 0x00100000 = Full buffer. Capture.
// Capture can only do 2 periods.
// 0x01000000 = End audio. Playback.
// 0x40000000 = Half buffer Playback,Caputre xrun.
// 0x80000000 = Full buffer Playback,Caputre xrun.
//
pub const EXTENDED_INT: c_uint = 0x76            /* Used by both playback and capture interrupt handler */;
// Shows which interrupts are active at the moment.
// Same bit layout as EXTENDED_INT_MASK
pub const COUNTER77: c_uint = 0x77		/* Counter range 0 to 0x3fffff, 192000 counts per second. */;
pub const COUNTER78: c_uint = 0x78		/* Counter range 0 to 0x3fffff, 44100 counts per second. */;
pub const EXTENDED_INT_TIMER: c_uint = 0x79            /* Channel_id 0 only. Used by both playback and capture interrupt handler */;
// Causes interrupts based on timer intervals.
pub const SPI: c_uint = 0x7a		/* SPI: Serial Interface Register */;
pub const I2C_A: c_uint = 0x7b		/* I2C Address. 32 bit */;
pub const I2C_D0: c_uint = 0x7c		/* I2C Data Port 0. 32 bit */;
pub const I2C_D1: c_uint = 0x7d		/* I2C Data Port 1. 32 bit */;
// I2C values
pub const I2C_A_ADC_ADD_MASK: c_uint = 0x000000fe	//The address is a 7 bit address;
pub const I2C_A_ADC_RW_MASK: c_uint = 0x00000001	//bit mask for R/W;
pub const I2C_A_ADC_TRANS_MASK: c_uint = 0x00000010  	//Bit mask for I2c address DAC value;
pub const I2C_A_ADC_ABORT_MASK: c_uint = 0x00000020	//Bit mask for I2C transaction abort flag;
pub const I2C_A_ADC_LAST_MASK: c_uint = 0x00000040	//Bit mask for Last word transaction;
pub const I2C_A_ADC_BYTE_MASK: c_uint = 0x00000080	//Bit mask for Byte Mode;
pub const I2C_A_ADC_ADD: c_uint = 0x00000034	//This is the Device address for ADC;
pub const I2C_A_ADC_READ: c_uint = 0x00000001	//To perform a read operation;
pub const I2C_A_ADC_START: c_uint = 0x00000100	//Start I2C transaction;
pub const I2C_A_ADC_ABORT: c_uint = 0x00000200	//I2C transaction abort;
pub const I2C_A_ADC_LAST: c_uint = 0x00000400	//I2C last transaction;
pub const I2C_A_ADC_BYTE: c_uint = 0x00000800	//I2C one byte mode;
pub const I2C_D_ADC_REG_MASK: c_uint = 0xfe000000  	//ADC address register;
pub const I2C_D_ADC_DAT_MASK: c_uint = 0x01ff0000  	//ADC data register;
pub const ADC_TIMEOUT: c_uint = 0x00000007	//ADC Timeout Clock Disable;
pub const ADC_IFC_CTRL: c_uint = 0x0000000b	//ADC Interface Control;
pub const ADC_MASTER: c_uint = 0x0000000c	//ADC Master Mode Control;
pub const ADC_POWER: c_uint = 0x0000000d	//ADC PowerDown Control;
pub const ADC_ATTEN_ADCL: c_uint = 0x0000000e	//ADC Attenuation ADCL;
pub const ADC_ATTEN_ADCR: c_uint = 0x0000000f	//ADC Attenuation ADCR;
pub const ADC_ALC_CTRL1: c_uint = 0x00000010	//ADC ALC Control 1;
pub const ADC_ALC_CTRL2: c_uint = 0x00000011	//ADC ALC Control 2;
pub const ADC_ALC_CTRL3: c_uint = 0x00000012	//ADC ALC Control 3;
pub const ADC_NOISE_CTRL: c_uint = 0x00000013	//ADC Noise Gate Control;
pub const ADC_LIMIT_CTRL: c_uint = 0x00000014	//ADC Limiter Control;
pub const ADC_MUX: c_uint = 0x00000015  	//ADC Mux offset;

// FIXME: Not tested yet.
pub const ADC_GAIN_MASK: c_uint = 0x000000ff	//Mask for ADC Gain;
pub const ADC_ZERODB: c_uint = 0x000000cf	//Value to set ADC to 0dB;
pub const ADC_MUTE_MASK: c_uint = 0x000000c0	//Mask for ADC mute;
pub const ADC_MUTE: c_uint = 0x000000c0	//Value to mute ADC;
pub const ADC_OSR: c_uint = 0x00000008	//Mask for ADC oversample rate select;
pub const ADC_TIMEOUT_DISABLE: c_uint = 0x00000008	//Value and mask to disable Timeout clock;
pub const ADC_HPF_DISABLE: c_uint = 0x00000100	//Value and mask to disable High pass filter;
pub const ADC_TRANWIN_MASK: c_uint = 0x00000070	//Mask for Length of Transient Window;

pub const ADC_MUX_MASK: c_uint = 0x0000000f	//Mask for ADC Mux;
pub const ADC_MUX_PHONE: c_uint = 0x00000001	//Value to select TAD at ADC Mux (Not used);
pub const ADC_MUX_MIC: c_uint = 0x00000002	//Value to select Mic at ADC Mux;
pub const ADC_MUX_LINEIN: c_uint = 0x00000004	//Value to select LineIn at ADC Mux;
pub const ADC_MUX_AUX: c_uint = 0x00000008	//Value to select Aux at ADC Mux;

pub const PCM_FRONT_CHANNEL: c_int = 0;
pub const PCM_REAR_CHANNEL: c_int = 1;
pub const PCM_CENTER_LFE_CHANNEL: c_int = 2;
pub const PCM_UNKNOWN_CHANNEL: c_int = 3;
pub const CONTROL_FRONT_CHANNEL: c_int = 0;
pub const CONTROL_REAR_CHANNEL: c_int = 3;
pub const CONTROL_CENTER_LFE_CHANNEL: c_int = 1;
pub const CONTROL_UNKNOWN_CHANNEL: c_int = 2;
// Based on WM8768 Datasheet Rev 4.2 page 32
pub const SPI_REG_MASK: c_uint = 0x1ff	/* 16-bit SPI writes have a 7-bit address */;

pub const SPI_RDA1_REG: c_int = 1;
pub const SPI_LDA2_REG: c_int = 4;
pub const SPI_RDA2_REG: c_int = 5;
pub const SPI_LDA3_REG: c_int = 6;
pub const SPI_RDA3_REG: c_int = 7;
pub const SPI_LDA4_REG: c_int = 13;
pub const SPI_RDA4_REG: c_int = 14;
pub const SPI_MASTDA_REG: c_int = 8;

pub const SPI_DA_BIT_0dB: c_uint = 0xff	/* 0 dB */;
pub const SPI_DA_BIT_infdB: c_uint = 0x00	/* inf dB attenuation (mute) */;
pub const SPI_PL_REG: c_int = 2;

pub const SPI_IZD_REG: c_int = 2;

pub const SPI_FMT_REG: c_int = 3;

pub const SPI_LRP_REG: c_int = 3;

pub const SPI_BCP_REG: c_int = 3;

pub const SPI_IWL_REG: c_int = 3;

pub const SPI_MS_REG: c_int = 10;

// They really do label the bit for the 4th channel "4" and not "3"
pub const SPI_DMUTE0_REG: c_int = 9;
pub const SPI_DMUTE1_REG: c_int = 9;
pub const SPI_DMUTE2_REG: c_int = 9;
pub const SPI_DMUTE4_REG: c_int = 15;

pub const SPI_PHASE0_REG: c_int = 3;
pub const SPI_PHASE1_REG: c_int = 3;
pub const SPI_PHASE2_REG: c_int = 3;
pub const SPI_PHASE4_REG: c_int = 15;

pub const SPI_DACD1_REG: c_int = 10;
pub const SPI_DACD2_REG: c_int = 10;
pub const SPI_DACD4_REG: c_int = 15;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ca0106_channel {
    pub emu: *mut snd_ca0106,
    pub number: c_int,
    pub use: c_int,
    pub channel): *mut *mut *mut void (interrupt)(struct snd_ca0106 emu, struct snd_ca0106_channel,
    pub epcm: *mut snd_ca0106_pcm,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ca0106_pcm {
    pub emu: *mut snd_ca0106,
    pub substream: *mut snd_pcm_substream,
    pub channel_id: c_int,
    pub running: c_ushort,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ca0106_details {
    pub serial: u32,
    pub name: *mut *mut c_char,
    pub in.: *mut *mut int ac97; / ac97 = 0 -> Select MIC, Line in, TAD in, AUX,
    pub mic-in/line-in: *mut *mut int gpio_type; / gpio_type = 1 -> shared,
    pub volume: *mut *mut int i2c_adc; / with i2c_adc=1, the driver adds some capture,
    pub DACs: *mut *mut u16 spi_dac; / spi_dac = 0 -> no spi interface for,
}

// definition of the chip-specific record
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ca0106 {
    pub card: *mut snd_card,
    pub details: *const snd_ca0106_details,
    pub pci: *mut pci_dev,
    pub port: c_ulong,
    pub irq: c_int,
    pub /: *mut *mut unsigned int serial; / serial number,
    pub /: *mut *mut unsigned short model; / subsystem id,
    pub emu_lock: spinlock_t,
    pub ac97: *mut snd_ac97,
    pub pcm: [*mut snd_pcm; 4],
    pub playback_channels: [snd_ca0106_channel; 4],
    pub capture_channels: [snd_ca0106_channel; 4],
    pub /: *mut *mut u32 spdif_bits[4]; / s/pdif out default setup,
    pub /: *mut *mut u32 spdif_str_bits[4]; / s/pdif out per-stream setup,
    pub spdif_enable: c_int,
    pub capture_source: c_int,
    pub i2c_capture_source: c_int,
    pub i2c_capture_volume: [u8; 4][2],
    pub capture_mic_line_in: c_int,
    pub buffer: *mut snd_dma_buffer,
    pub midi: snd_ca_midi,
    pub midi2: snd_ca_midi,
    pub spi_dac_reg: [u16; 16],
pub const NUM_SAVED_VOLUMES: c_int = 9;
    pub saved_vol: [c_uint; NUM_SAVED_VOLUMES],
}

extern "C" {
    pub fn snd_ca0106_mixer(emu: *mut snd_ca0106) -> c_int;
}
extern "C" {
    pub fn snd_ca0106_proc_init(emu: *mut *mut snd_ca0106) -> c_int;
}
extern "C" {
    pub fn snd_ca0106_i2c_write(emu: *mut snd_ca0106, reg: u32, value: u32) -> c_int;
}

extern "C" {
    pub fn snd_ca0106_mixer_suspend(chip: *mut snd_ca0106);
}
extern "C" {
    pub fn snd_ca0106_mixer_resume(chip: *mut snd_ca0106);
}

