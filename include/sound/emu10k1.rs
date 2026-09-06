//! Automatically rewritten from C Header to Rust Module
//! Source: include/sound/emu10k1.h
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
// Copyright (c) by Jaroslav Kysela <perex@perex.cz>,
// Creative Labs, Inc.
// Definitions for EMU10K1 (SB Live!) chips
//

// ------------------- DEFINES --------------------
pub const EMUPAGESIZE: c_int = 4096;

pub const NUM_EFX_PLAYBACK: c_int = 16;
// FIXME? - according to the OSS driver the EMU10K1 needs a 29 bit DMA mask
pub const EMU10K1_DMA_MASK: c_uint = 0x7fffffffUL	/* 31bit */;
pub const AUDIGY_DMA_MASK: c_uint = 0xffffffffUL	/* 32bit mode */;

// This is used to define hardware bit-fields (sub-registers) by combining
// the bit shift and count with the actual register address. The passed
// mask must represent a single run of adjacent bits.
// The non-concatenating (_NC) variant should be used directly only for
// sub-registers that do not follow the <register>_<field> naming pattern.

// Macros for manipulating values of bit-fields declared using the above macros.
// Best used with constant register addresses, as otherwise quite some code is
// generated. The actual register read/write functions handle combined addresses
// automatically, so use of these macros conveys no advantage when accessing a
// single sub-register at a time.

// List terminator for snd_emu10k1_ptr_write_multiple()

// Audigy specify registers are prefixed with 'A_'
//
// PCI function 0 registers, address = <val> + PCIBASE0
//
pub const PTR: c_uint = 0x00		/* Indexed register set pointer register	*/;
// NOTE: The CHANNELNUM and ADDRESS words can
// be modified independently of each other.
pub const PTR_CHANNELNUM_MASK: c_uint = 0x0000003f	/* For each per-channel register, indicates the	*/;
// channel number of the register to be
// accessed.  For non per-channel registers the
// value should be set to zero.
pub const PTR_ADDRESS_MASK: c_uint = 0x07ff0000	/* Register index				*/;
pub const A_PTR_ADDRESS_MASK: c_uint = 0x0fff0000;
pub const DATA: c_uint = 0x04		/* Indexed register set data register		*/;
pub const IPR: c_uint = 0x08		/* Global interrupt pending register		*/;
// Clear pending interrupts by writing a 1 to
// the relevant bits and zero to the other bits
pub const IPR_P16V: c_uint = 0x80000000	/* Bit set when the CA0151 P16V chip wishes;
pub const IPR_WATERMARK_REACHED: c_uint = 0x40000000;
pub const IPR_A_GPIO: c_uint = 0x20000000	/* GPIO input pin change			*/;
// The next two interrupts are for the midi port on the Audigy Drive (A_MPU1)
pub const IPR_A_MIDITRANSBUFEMPTY2: c_uint = 0x10000000	/* MIDI UART transmit buffer empty		*/;
pub const IPR_A_MIDIRECVBUFEMPTY2: c_uint = 0x08000000	/* MIDI UART receive buffer empty		*/;
pub const IPR_SPDIFBUFFULL: c_uint = 0x04000000	/* SPDIF capture related, 10k2 only? (RE)	*/;
pub const IPR_SPDIFBUFHALFFULL: c_uint = 0x02000000	/* SPDIF capture related? (RE)			*/;
pub const IPR_SAMPLERATETRACKER: c_uint = 0x01000000	/* Sample rate tracker lock status change	*/;
pub const IPR_FXDSP: c_uint = 0x00800000	/* Enable FX DSP interrupts			*/;
pub const IPR_FORCEINT: c_uint = 0x00400000	/* Force Sound Blaster interrupt		*/;
pub const IPR_PCIERROR: c_uint = 0x00200000	/* PCI bus error				*/;
pub const IPR_VOLINCR: c_uint = 0x00100000	/* Volume increment button pressed		*/;
pub const IPR_VOLDECR: c_uint = 0x00080000	/* Volume decrement button pressed		*/;
pub const IPR_MUTE: c_uint = 0x00040000	/* Mute button pressed				*/;
pub const IPR_MICBUFFULL: c_uint = 0x00020000	/* Microphone buffer full			*/;
pub const IPR_MICBUFHALFFULL: c_uint = 0x00010000	/* Microphone buffer half full			*/;
pub const IPR_ADCBUFFULL: c_uint = 0x00008000	/* ADC buffer full				*/;
pub const IPR_ADCBUFHALFFULL: c_uint = 0x00004000	/* ADC buffer half full				*/;
pub const IPR_EFXBUFFULL: c_uint = 0x00002000	/* Effects buffer full				*/;
pub const IPR_EFXBUFHALFFULL: c_uint = 0x00001000	/* Effects buffer half full			*/;
pub const IPR_GPSPDIFSTATUSCHANGE: c_uint = 0x00000800	/* GPSPDIF channel status change		*/;
pub const IPR_CDROMSTATUSCHANGE: c_uint = 0x00000400	/* CD-ROM channel status change			*/;
pub const IPR_INTERVALTIMER: c_uint = 0x00000200	/* Interval timer terminal count		*/;
pub const IPR_MIDITRANSBUFEMPTY: c_uint = 0x00000100	/* MIDI UART transmit buffer empty		*/;
pub const IPR_MIDIRECVBUFEMPTY: c_uint = 0x00000080	/* MIDI UART receive buffer empty		*/;
pub const IPR_CHANNELLOOP: c_uint = 0x00000040	/* Channel (half) loop interrupt(s) pending	*/;
// The interrupt is triggered shortly after
// CCR_READADDRESS has crossed the boundary;
// due to the cache, this runs ahead of the
// actual playback position.
pub const IPR_CHANNELNUMBERMASK: c_uint = 0x0000003f	/* When IPR_CHANNELLOOP is set, indicates the	*/;
// highest set channel in CLIPL, CLIPH, HLIPL,
// or HLIPH.  When IPR is written with CL set,
// the bit in H/CLIPL or H/CLIPH corresponding
// to the CN value written will be cleared.
pub const INTE: c_uint = 0x0c		/* Interrupt enable register			*/;
pub const INTE_VIRTUALSB_MASK: c_uint = 0xc0000000	/* Virtual Soundblaster I/O port capture	*/;
pub const INTE_VIRTUALSB_220: c_uint = 0x00000000	/* Capture at I/O base address 0x220-0x22f	*/;
pub const INTE_VIRTUALSB_240: c_uint = 0x40000000	/* Capture at I/O base address 0x240		*/;
pub const INTE_VIRTUALSB_260: c_uint = 0x80000000	/* Capture at I/O base address 0x260		*/;
pub const INTE_VIRTUALSB_280: c_uint = 0xc0000000	/* Capture at I/O base address 0x280		*/;
pub const INTE_VIRTUALMPU_MASK: c_uint = 0x30000000	/* Virtual MPU I/O port capture			*/;
pub const INTE_VIRTUALMPU_300: c_uint = 0x00000000	/* Capture at I/O base address 0x300-0x301	*/;
pub const INTE_VIRTUALMPU_310: c_uint = 0x10000000	/* Capture at I/O base address 0x310		*/;
pub const INTE_VIRTUALMPU_320: c_uint = 0x20000000	/* Capture at I/O base address 0x320		*/;
pub const INTE_VIRTUALMPU_330: c_uint = 0x30000000	/* Capture at I/O base address 0x330		*/;
pub const INTE_MASTERDMAENABLE: c_uint = 0x08000000	/* Master DMA emulation at 0x000-0x00f		*/;
pub const INTE_SLAVEDMAENABLE: c_uint = 0x04000000	/* Slave DMA emulation at 0x0c0-0x0df		*/;
pub const INTE_MASTERPICENABLE: c_uint = 0x02000000	/* Master PIC emulation at 0x020-0x021		*/;
pub const INTE_SLAVEPICENABLE: c_uint = 0x01000000	/* Slave PIC emulation at 0x0a0-0x0a1		*/;
pub const INTE_VSBENABLE: c_uint = 0x00800000	/* Enable virtual Soundblaster			*/;
pub const INTE_ADLIBENABLE: c_uint = 0x00400000	/* Enable AdLib emulation at 0x388-0x38b	*/;
pub const INTE_MPUENABLE: c_uint = 0x00200000	/* Enable virtual MPU				*/;
pub const INTE_FORCEINT: c_uint = 0x00100000	/* Continuously assert INTAN			*/;
pub const INTE_MRHANDENABLE: c_uint = 0x00080000	/* Enable the "Mr. Hand" logic			*/;
// NOTE: There is no reason to use this under
// Linux, and it will cause odd hardware
// behavior and possibly random segfaults and
// lockups if enabled.
pub const INTE_A_GPIOENABLE: c_uint = 0x00040000	/* Enable GPIO input change interrupts		*/;
// The next two interrupts are for the midi port on the Audigy Drive (A_MPU1)
pub const INTE_A_MIDITXENABLE2: c_uint = 0x00020000	/* Enable MIDI transmit-buffer-empty interrupts	*/;
pub const INTE_A_MIDIRXENABLE2: c_uint = 0x00010000	/* Enable MIDI receive-buffer-empty interrupts	*/;
pub const INTE_A_SPDIF_BUFFULL_ENABLE: c_uint = 0x00008000;
pub const INTE_A_SPDIF_HALFBUFFULL_ENABLE: c_uint = 0x00004000;
pub const INTE_SAMPLERATETRACKER: c_uint = 0x00002000	/* Enable sample rate tracker interrupts	*/;
// NOTE: This bit must always be enabled
pub const INTE_FXDSPENABLE: c_uint = 0x00001000	/* Enable FX DSP interrupts			*/;
pub const INTE_PCIERRORENABLE: c_uint = 0x00000800	/* Enable PCI bus error interrupts		*/;
pub const INTE_VOLINCRENABLE: c_uint = 0x00000400	/* Enable volume increment button interrupts	*/;
pub const INTE_VOLDECRENABLE: c_uint = 0x00000200	/* Enable volume decrement button interrupts	*/;
pub const INTE_MUTEENABLE: c_uint = 0x00000100	/* Enable mute button interrupts		*/;
pub const INTE_MICBUFENABLE: c_uint = 0x00000080	/* Enable microphone buffer interrupts		*/;
pub const INTE_ADCBUFENABLE: c_uint = 0x00000040	/* Enable ADC buffer interrupts			*/;
pub const INTE_EFXBUFENABLE: c_uint = 0x00000020	/* Enable Effects buffer interrupts		*/;
pub const INTE_GPSPDIFENABLE: c_uint = 0x00000010	/* Enable GPSPDIF status interrupts		*/;
pub const INTE_CDSPDIFENABLE: c_uint = 0x00000008	/* Enable CDSPDIF status interrupts		*/;
pub const INTE_INTERVALTIMERENB: c_uint = 0x00000004	/* Enable interval timer interrupts		*/;
pub const INTE_MIDITXENABLE: c_uint = 0x00000002	/* Enable MIDI transmit-buffer-empty interrupts	*/;
pub const INTE_MIDIRXENABLE: c_uint = 0x00000001	/* Enable MIDI receive-buffer-empty interrupts	*/;
pub const WC: c_uint = 0x10		/* Wall Clock register				*/;
// NOTE: Each channel takes 1/64th of a sample
// period to be serviced.
pub const HCFG: c_uint = 0x14		/* Hardware config register			*/;
// NOTE: There is no reason to use the legacy
// SoundBlaster emulation stuff described below
// under Linux, and all kinds of weird hardware
// behavior can result if you try.  Don't.
pub const HCFG_LEGACYFUNC_MASK: c_uint = 0xe0000000	/* Legacy function number 			*/;
pub const HCFG_LEGACYFUNC_MPU: c_uint = 0x00000000	/* Legacy MPU	 				*/;
pub const HCFG_LEGACYFUNC_SB: c_uint = 0x40000000	/* Legacy SB					*/;
pub const HCFG_LEGACYFUNC_AD: c_uint = 0x60000000	/* Legacy AD					*/;
pub const HCFG_LEGACYFUNC_MPIC: c_uint = 0x80000000	/* Legacy MPIC					*/;
pub const HCFG_LEGACYFUNC_MDMA: c_uint = 0xa0000000	/* Legacy MDMA					*/;
pub const HCFG_LEGACYFUNC_SPCI: c_uint = 0xc0000000	/* Legacy SPCI					*/;
pub const HCFG_LEGACYFUNC_SDMA: c_uint = 0xe0000000	/* Legacy SDMA					*/;
pub const HCFG_IOCAPTUREADDR: c_uint = 0x1f000000	/* The 4 LSBs of the captured I/O address.	*/;
pub const HCFG_LEGACYWRITE: c_uint = 0x00800000	/* 1 = write, 0 = read 				*/;
pub const HCFG_LEGACYWORD: c_uint = 0x00400000	/* 1 = word, 0 = byte 				*/;
pub const HCFG_LEGACYINT: c_uint = 0x00200000	/* 1 = legacy event captured. Write 1 to clear.	*/;
// NOTE: The rest of the bits in this register
// _are_ relevant under Linux.
pub const HCFG_PUSH_BUTTON_ENABLE: c_uint = 0x00100000	/* Enables Volume Inc/Dec and Mute functions    */;
pub const HCFG_BAUD_RATE: c_uint = 0x00080000	/* 0 = 48kHz, 1 = 44.1kHz			*/;
pub const HCFG_EXPANDED_MEM: c_uint = 0x00040000	/* 1 = any 16M of 4G addr, 0 = 32M of 2G addr	*/;
pub const HCFG_CODECFORMAT_MASK: c_uint = 0x00030000	/* CODEC format					*/;
// Specific to Alice2, CA0102
pub const HCFG_CODECFORMAT_AC97_1: c_uint = 0x00000000	/* AC97 CODEC format -- Ver 1.03		*/;
pub const HCFG_CODECFORMAT_AC97_2: c_uint = 0x00010000	/* AC97 CODEC format -- Ver 2.1			*/;
pub const HCFG_AUTOMUTE_ASYNC: c_uint = 0x00008000	/* When set, the async sample rate convertors	*/;
// will automatically mute their output when
// they are not rate-locked to the external
// async audio source
pub const HCFG_AUTOMUTE_SPDIF: c_uint = 0x00004000	/* When set, the async sample rate convertors	*/;
// will automatically mute their output when
// the SPDIF V-bit indicates invalid audio
pub const HCFG_EMU32_SLAVE: c_uint = 0x00002000	/* 0 = Master, 1 = Slave. Slave for EMU1010	*/;
pub const HCFG_SLOW_RAMP: c_uint = 0x00001000	/* Increases Send Smoothing time constant	*/;
// 0x00000800 not used on Alice2
pub const HCFG_PHASE_TRACK_MASK: c_uint = 0x00000700	/* When set, forces corresponding input to	*/;
// phase track the previous input.
// I2S0 can phase track the last S/PDIF input
pub const HCFG_I2S_ASRC_ENABLE: c_uint = 0x00000070	/* When set, enables asynchronous sample rate   */;
// conversion for the corresponding
// I2S format input
// Rest of HCFG 0x0000000f same as below. LOCKSOUNDCACHE etc.
// Older chips
pub const HCFG_CODECFORMAT_AC97: c_uint = 0x00000000	/* AC97 CODEC format -- Primary Output		*/;
pub const HCFG_CODECFORMAT_I2S: c_uint = 0x00010000	/* I2S CODEC format -- Secondary (Rear) Output	*/;
pub const HCFG_GPINPUT0: c_uint = 0x00004000	/* External pin112				*/;
pub const HCFG_GPINPUT1: c_uint = 0x00002000	/* External pin110				*/;
pub const HCFG_GPOUTPUT_MASK: c_uint = 0x00001c00	/* External pins which may be controlled	*/;
pub const HCFG_GPOUT0: c_uint = 0x00001000	/* External pin? (spdif enable on 5.1)		*/;
pub const HCFG_GPOUT1: c_uint = 0x00000800	/* External pin? (IR)				*/;
pub const HCFG_GPOUT2: c_uint = 0x00000400	/* External pin? (IR)				*/;
pub const HCFG_JOYENABLE: c_uint = 0x00000200	/* Internal joystick enable    			*/;
pub const HCFG_PHASETRACKENABLE: c_uint = 0x00000100	/* Phase tracking enable			*/;
// 1 = Force all 3 async digital inputs to use
// the same async sample rate tracker (ZVIDEO)
pub const HCFG_AC3ENABLE_MASK: c_uint = 0x000000e0	/* AC3 async input control - Not implemented	*/;
pub const HCFG_AC3ENABLE_ZVIDEO: c_uint = 0x00000080	/* Channels 0 and 1 replace ZVIDEO		*/;
pub const HCFG_AC3ENABLE_CDSPDIF: c_uint = 0x00000040	/* Channels 0 and 1 replace CDSPDIF		*/;
pub const HCFG_AC3ENABLE_GPSPDIF: c_uint = 0x00000020      /* Channels 0 and 1 replace GPSPDIF             */;
pub const HCFG_AUTOMUTE: c_uint = 0x00000010	/* When set, the async sample rate convertors	*/;
// will automatically mute their output when
// they are not rate-locked to the external
// async audio source
pub const HCFG_LOCKSOUNDCACHE: c_uint = 0x00000008	/* 1 = Cancel bustmaster accesses to soundcache */;
// NOTE: This should generally never be used.
pub const HCFG_MUTEBUTTONENABLE: c_uint = 0x00000002	/* 1 = Master mute button sets AUDIOENABLE = 0.	*/;
// NOTE: This is a 'cheap' way to implement a
// master mute function on the mute button, and
// in general should not be used unless a more
// sophisticated master mute function has not
// been written.
pub const HCFG_AUDIOENABLE: c_uint = 0x00000001	/* 0 = CODECs transmit zero-valued samples	*/;
// Should be set to 1 when the EMU10K1 is
// completely initialized.
// On Audigy, the MPU port moved to the 0x70-0x74 ptr registers
pub const MUDATA: c_uint = 0x18		/* MPU401 data register (8 bits)       		*/;
pub const MUCMD: c_uint = 0x19		/* MPU401 command register (8 bits)    		*/;
pub const MUCMD_RESET: c_uint = 0xff		/* RESET command				*/;
pub const MUCMD_ENTERUARTMODE: c_uint = 0x3f		/* Enter_UART_mode command			*/;
// NOTE: All other commands are ignored

pub const MUSTAT_IRDYN: c_uint = 0x80		/* 0 = MIDI data or command ACK			*/;
pub const MUSTAT_ORDYN: c_uint = 0x40		/* 0 = MUDATA can accept a command or data	*/;
pub const A_GPIO: c_uint = 0x18		/* GPIO on Audigy card (16bits)			*/;
pub const A_GPINPUT_MASK: c_uint = 0xff00		/* Alice/2 has 8 input pins			*/;
pub const A3_GPINPUT_MASK: c_uint = 0x3f00		/* ... while Tina/2 has only 6			*/;
pub const A_GPOUTPUT_MASK: c_uint = 0x00ff;
// The GPIO port is used for I/O config on Sound Blasters;
// card-specific info can be found in the emu_chip_details table.
// On E-MU cards the port is used as the interface to the FPGA.
// Audigy output/GPIO stuff taken from the kX drivers

pub const A_IOCFG_GPOUT0: c_uint = 0x0044		/* analog/digital				*/;
pub const A_IOCFG_DISABLE_ANALOG: c_uint = 0x0040		/* = 'enable' for Audigy2 (chiprev=4)		*/;
pub const A_IOCFG_ENABLE_DIGITAL: c_uint = 0x0004;
pub const A_IOCFG_ENABLE_DIGITAL_AUDIGY4: c_uint = 0x0080;
pub const A_IOCFG_UNKNOWN_20: c_uint = 0x0020;
pub const A_IOCFG_DISABLE_AC97_FRONT: c_uint = 0x0080  /* turn off ac97 front -> front (10k2.1)	*/;
pub const A_IOCFG_GPOUT1: c_uint = 0x0002		/* IR? drive's internal bypass (?)		*/;
pub const A_IOCFG_GPOUT2: c_uint = 0x0001		/* IR */;
pub const A_IOCFG_MULTIPURPOSE_JACK: c_uint = 0x2000  /* center+lfe+rear_center (a2/a2ex)		*/;
// + digital for generic 10k2
pub const A_IOCFG_DIGITAL_JACK: c_uint = 0x1000          /* digital for a2 platinum			*/;
pub const A_IOCFG_FRONT_JACK: c_uint = 0x4000;
pub const A_IOCFG_REAR_JACK: c_uint = 0x8000;
pub const A_IOCFG_PHONES_JACK: c_uint = 0x0100          /* LiveDrive					*/;
pub const TIMER: c_uint = 0x1a		/* Timer terminal count register		*/;
// NOTE: After the rate is changed, a maximum
// of 1024 sample periods should be allowed
// before the new rate is guaranteed accurate.
pub const TIMER_RATE_MASK: c_uint = 0x03ff		/* Timer interrupt rate in sample periods	*/;
// 0 == 1024 periods, [1..4] are not useful
pub const AC97DATA: c_uint = 0x1c		/* AC97 register set data register (16 bit)	*/;
pub const AC97ADDRESS: c_uint = 0x1e		/* AC97 register set address register (8 bit)	*/;
pub const AC97ADDRESS_READY: c_uint = 0x80		/* Read-only bit, reflects CODEC READY signal	*/;
pub const AC97ADDRESS_ADDRESS: c_uint = 0x7f		/* Address of indexed AC97 register		*/;
// Available on the Audigy 2 and Audigy 4 only. This is the P16V chip.
pub const PTR2: c_uint = 0x20		/* Indexed register set pointer register	*/;
pub const DATA2: c_uint = 0x24		/* Indexed register set data register		*/;
pub const IPR2: c_uint = 0x28		/* P16V interrupt pending register		*/;
pub const IPR2_PLAYBACK_CH_0_LOOP: c_uint = 0x00001000 /* Playback Channel 0 loop                               */;
pub const IPR2_PLAYBACK_CH_0_HALF_LOOP: c_uint = 0x00000100 /* Playback Channel 0 half loop                          */;
pub const IPR2_CAPTURE_CH_0_LOOP: c_uint = 0x00100000 /* Capture Channel 0 loop                               */;
pub const IPR2_CAPTURE_CH_0_HALF_LOOP: c_uint = 0x00010000 /* Capture Channel 0 half loop                          */;
// 0x00000100 Playback. Only in once per period.
// 0x00110000 Capture. Int on half buffer.
//
pub const INTE2: c_uint = 0x2c		/* P16V Interrupt enable register. 	*/;
pub const INTE2_PLAYBACK_CH_0_LOOP: c_uint = 0x00001000 /* Playback Channel 0 loop                               */;
pub const INTE2_PLAYBACK_CH_0_HALF_LOOP: c_uint = 0x00000100 /* Playback Channel 0 half loop                          */;
pub const INTE2_PLAYBACK_CH_1_LOOP: c_uint = 0x00002000 /* Playback Channel 1 loop                               */;
pub const INTE2_PLAYBACK_CH_1_HALF_LOOP: c_uint = 0x00000200 /* Playback Channel 1 half loop                          */;
pub const INTE2_PLAYBACK_CH_2_LOOP: c_uint = 0x00004000 /* Playback Channel 2 loop                               */;
pub const INTE2_PLAYBACK_CH_2_HALF_LOOP: c_uint = 0x00000400 /* Playback Channel 2 half loop                          */;
pub const INTE2_PLAYBACK_CH_3_LOOP: c_uint = 0x00008000 /* Playback Channel 3 loop                               */;
pub const INTE2_PLAYBACK_CH_3_HALF_LOOP: c_uint = 0x00000800 /* Playback Channel 3 half loop                          */;
pub const INTE2_CAPTURE_CH_0_LOOP: c_uint = 0x00100000 /* Capture Channel 0 loop                               */;
pub const INTE2_CAPTURE_CH_0_HALF_LOOP: c_uint = 0x00010000 /* Caputre Channel 0 half loop                          */;
pub const HCFG2: c_uint = 0x34		/* Defaults: 0, win2000 sets it to 00004201 */;
// 0x00000000 2-channel output.
// 0x00000200 8-channel output.
// 0x00000004 pauses stream/irq fail.
// Rest of bits do nothing to sound output
// bit 0: Enable P16V audio.
// bit 1: Lock P16V record memory cache.
// bit 2: Lock P16V playback memory cache.
// bit 3: Dummy record insert zero samples.
// bit 8: Record 8-channel in phase.
// bit 9: Playback 8-channel in phase.
// bit 11-12: Playback mixer attenuation: 0=0dB, 1=-6dB, 2=-12dB, 3=Mute.
// bit 13: Playback mixer enable.
// bit 14: Route SRC48 mixer output to fx engine.
// bit 15: Enable IEEE 1394 chip.
//
pub const IPR3: c_uint = 0x38		/* Cdif interrupt pending register		*/;
pub const INTE3: c_uint = 0x3c		/* Cdif interrupt enable register. 	*/;
//
// PCI function 1 registers, address = <val> + PCIBASE1
//
pub const JOYSTICK1: c_uint = 0x00		/* Analog joystick port register		*/;
pub const JOYSTICK2: c_uint = 0x01		/* Analog joystick port register		*/;
pub const JOYSTICK3: c_uint = 0x02		/* Analog joystick port register		*/;
pub const JOYSTICK4: c_uint = 0x03		/* Analog joystick port register		*/;
pub const JOYSTICK5: c_uint = 0x04		/* Analog joystick port register		*/;
pub const JOYSTICK6: c_uint = 0x05		/* Analog joystick port register		*/;
pub const JOYSTICK7: c_uint = 0x06		/* Analog joystick port register		*/;
pub const JOYSTICK8: c_uint = 0x07		/* Analog joystick port register		*/;
// When writing, any write causes JOYSTICK_COMPARATOR output enable to be pulsed on write.
// When reading, use these bitfields:
pub const JOYSTICK_BUTTONS: c_uint = 0x0f		/* Joystick button data				*/;
pub const JOYSTICK_COMPARATOR: c_uint = 0xf0		/* Joystick comparator data			*/;
//
// Emu10k1 pointer-offset register set, accessed through the PTR and DATA registers
//
// No official documentation was released for EMU10K1, but some info
// about playback can be extrapolated from the EMU8K documents:
// "AWE32/EMU8000 Programmer’s Guide" (emu8kpgm.pdf) - registers
// "AWE32 Developer's Information Pack" (adip301.pdf) - high-level view
// The short version:
// - The engine has 64 playback channels, also called voices. The channels
// operate independently, except when paired for stereo (see below).
// - PCM samples are fetched into the cache; see description of CD0 below.
// - Samples are consumed at the rate CPF_CURRENTPITCH.
// - 8-bit samples are transformed upon use: cooked = (raw ^ 0x80) << 8
// - 8 samples are read at CCR_READADDRESS:CPF_FRACADDRESS and interpolated
// according to CCCA_INTERPROM_*. With CCCA_INTERPROM_0 selected and a zero
// CPF_FRACADDRESS, this results in CCR_READADDRESS[3] being used verbatim.
// - The value is multiplied by CVCF_CURRENTVOL.
// - The value goes through a filter with cutoff CVCF_CURRENTFILTER;
// delay stages Z1 and Z2.
// - The value is added by so-called `sends` to 4 (EMU10K1) / 8 (EMU10K2)
// of the 16 (EMU10K1) / 64 (EMU10K2) FX bus accumulators via FXRT*,
// multiplied by a per-send amount (*_FXSENDAMOUNT_*).
// The scaling of the send amounts is exponential-ish.
// - The DSP has a go at FXBUS* and outputs the values to EXTOUT* or EMU32OUT*.
// - The pitch, volume, and filter cutoff can be modulated by two envelope
// engines and two low frequency oscillators.
// - To avoid abrupt changes to the parameters (which may cause audible
// distortion), the modulation engine sets the target registers, towards
// which the current registers "swerve" gradually.
// For the odd channel in a stereo pair, these registers are meaningless:
// CPF_STEREO, CPF_CURRENTPITCH, PTRX_PITCHTARGET, CCR_CACHEINVALIDSIZE,
// PSST_LOOPSTARTADDR, DSL_LOOPENDADDR, CCCA_CURRADDR
// The somewhat non-obviously still meaningful ones are:
// CPF_STOP, CPF_FRACADDRESS, CCR_READADDRESS (!),
// CCCA_INTERPROM, CCCA_8BITSELECT (!)
// (The envelope engine is ignored here, as stereo matters only for verbatim playback.)
pub const CPF: c_uint = 0x00		/* Current pitch and fraction register			*/;
pub const CPF_STEREO_MASK: c_uint = 0x00008000	/* 1 = Even channel interleave, odd channel locked	*/;
// Can be set only while matching bit in SOLEx is 1
pub const CPF_FRACADDRESS_MASK: c_uint = 0x00003fff	/* Linear fractional address of the current channel	*/;
pub const PTRX: c_uint = 0x01		/* Pitch target and send A/B amounts register		*/;
// Note: the volumes are raw multpliers, so real 100% is impossible.
pub const CVCF: c_uint = 0x02		/* Current volume and filter cutoff register		*/;
pub const VTFT: c_uint = 0x03		/* Volume target and filter cutoff target register	*/;
pub const Z1: c_uint = 0x05		/* Filter delay memory 1 register			*/;
pub const Z2: c_uint = 0x04		/* Filter delay memory 2 register			*/;
pub const PSST: c_uint = 0x06		/* Send C amount and loop start address register	*/;
pub const DSL: c_uint = 0x07		/* Send D amount and loop end address register	*/;
pub const CCCA: c_uint = 0x08		/* Filter Q, interp. ROM, byte size, cur. addr register */;
pub const CCCA_INTERPROM_MASK: c_uint = 0x0e000000	/* Selects passband of interpolation ROM		*/;
// 1 == full band, 7 == lowpass
// ROM 0 is used when pitch shifting downward or less
// then 3 semitones upward.  Increasingly higher ROM
// numbers are used, typically in steps of 3 semitones,
// as upward pitch shifting is performed.
pub const CCCA_INTERPROM_0: c_uint = 0x00000000	/* Select interpolation ROM 0				*/;
pub const CCCA_INTERPROM_1: c_uint = 0x02000000	/* Select interpolation ROM 1				*/;
pub const CCCA_INTERPROM_2: c_uint = 0x04000000	/* Select interpolation ROM 2				*/;
pub const CCCA_INTERPROM_3: c_uint = 0x06000000	/* Select interpolation ROM 3				*/;
pub const CCCA_INTERPROM_4: c_uint = 0x08000000	/* Select interpolation ROM 4				*/;
pub const CCCA_INTERPROM_5: c_uint = 0x0a000000	/* Select interpolation ROM 5				*/;
pub const CCCA_INTERPROM_6: c_uint = 0x0c000000	/* Select interpolation ROM 6				*/;
pub const CCCA_INTERPROM_7: c_uint = 0x0e000000	/* Select interpolation ROM 7				*/;
pub const CCCA_8BITSELECT: c_uint = 0x01000000	/* 1 = Sound memory for this channel uses 8-bit samples	*/;
// 8-bit samples are unsigned, 16-bit ones signed
pub const CCR: c_uint = 0x09		/* Cache control register				*/;
pub const CCR_CACHELOOPFLAG: c_uint = 0x01000000	/* 1 = Cache has a loop service pending			*/;
pub const CCR_INTERLEAVEDSAMPLES: c_uint = 0x00800000	/* 1 = A cache service will fetch interleaved samples	*/;
// Auto-set from CPF_STEREO_MASK
pub const CCR_WORDSIZEDSAMPLES: c_uint = 0x00400000	/* 1 = A cache service will fetch word sized samples	*/;
// Auto-set from CCCA_8BITSELECT
// NOTE: This is valid only if CACHELOOPFLAG is set
pub const CCR_LOOPFLAG: c_uint = 0x00000100	/* Set for a single sample period when a loop occurs	*/;
pub const CLP: c_uint = 0x0a		/* Cache loop register (valid if CCR_CACHELOOPFLAG = 1) */;
// NOTE: This register is normally not used
pub const FXRT: c_uint = 0x0b		/* Effects send routing register			*/;
// NOTE: It is illegal to assign the same routing to
// two effects sends.
pub const FXRT_CHANNELA: c_uint = 0x000f0000	/* Effects send bus number for channel's effects send A	*/;
pub const FXRT_CHANNELB: c_uint = 0x00f00000	/* Effects send bus number for channel's effects send B	*/;
pub const FXRT_CHANNELC: c_uint = 0x0f000000	/* Effects send bus number for channel's effects send C	*/;
pub const FXRT_CHANNELD: c_uint = 0xf0000000	/* Effects send bus number for channel's effects send D	*/;
pub const MAPA: c_uint = 0x0c		/* Cache map A						*/;
pub const MAPB: c_uint = 0x0d		/* Cache map B						*/;
pub const MAP_PTE_MASK0: c_uint = 0xfffff000	/* The 20 MSBs of the PTE indexed by the PTI		*/;
pub const MAP_PTI_MASK0: c_uint = 0x00000fff	/* The 12 bit index to one of the 4096 PTE dwords      	*/;
pub const MAP_PTE_MASK1: c_uint = 0xffffe000	/* The 19 MSBs of the PTE indexed by the PTI		*/;
pub const MAP_PTI_MASK1: c_uint = 0x00001fff	/* The 13 bit index to one of the 8192 PTE dwords      	*/;
// 0x0e, 0x0f: Internal state, at least on Audigy
pub const ENVVOL: c_uint = 0x10		/* Volume envelope register				*/;
pub const ENVVOL_MASK: c_uint = 0x0000ffff	/* Current value of volume envelope state variable	*/;
// 0x8000-n == 666*n usec delay
pub const ATKHLDV: c_uint = 0x11		/* Volume envelope hold and attack register		*/;
pub const ATKHLDV_PHASE0_MASK: c_uint = 0x00008000	/* 0 = Begin attack phase				*/;
pub const ATKHLDV_HOLDTIME_MASK: c_uint = 0x00007f00	/* Envelope hold time (127-n == n*88.2msec)		*/;
pub const ATKHLDV_ATTACKTIME_MASK: c_uint = 0x0000007f	/* Envelope attack time, log encoded			*/;
// 0 = infinite, 1 = 10.9msec, ... 0x7f = 5.5msec
pub const DCYSUSV: c_uint = 0x12		/* Volume envelope sustain and decay register		*/;
pub const DCYSUSV_PHASE1_MASK: c_uint = 0x00008000	/* 0 = Begin decay phase, 1 = begin release phase	*/;
pub const DCYSUSV_SUSTAINLEVEL_MASK: c_uint = 0x00007f00	/* 127 = full, 0 = off, 0.75dB increments		*/;
pub const DCYSUSV_CHANNELENABLE_MASK: c_uint = 0x00000080	/* 0 = Inhibit envelope engine from writing values in	*/;
// this channel and from writing to pitch, filter and
// volume targets.
pub const DCYSUSV_DECAYTIME_MASK: c_uint = 0x0000007f	/* Volume envelope decay time, log encoded     		*/;
// 0 = 43.7msec, 1 = 21.8msec, 0x7f = 22msec
pub const LFOVAL1: c_uint = 0x13		/* Modulation LFO value					*/;
pub const LFOVAL_MASK: c_uint = 0x0000ffff	/* Current value of modulation LFO state variable	*/;
// 0x8000-n == 666*n usec delay
pub const ENVVAL: c_uint = 0x14		/* Modulation envelope register				*/;
pub const ENVVAL_MASK: c_uint = 0x0000ffff	/* Current value of modulation envelope state variable 	*/;
// 0x8000-n == 666*n usec delay
pub const ATKHLDM: c_uint = 0x15		/* Modulation envelope hold and attack register		*/;
pub const ATKHLDM_PHASE0_MASK: c_uint = 0x00008000	/* 0 = Begin attack phase				*/;
pub const ATKHLDM_HOLDTIME: c_uint = 0x00007f00	/* Envelope hold time (127-n == n*42msec)		*/;
pub const ATKHLDM_ATTACKTIME: c_uint = 0x0000007f	/* Envelope attack time, log encoded			*/;
// 0 = infinite, 1 = 11msec, ... 0x7f = 5.5msec
pub const DCYSUSM: c_uint = 0x16		/* Modulation envelope decay and sustain register	*/;
pub const DCYSUSM_PHASE1_MASK: c_uint = 0x00008000	/* 0 = Begin decay phase, 1 = begin release phase	*/;
pub const DCYSUSM_SUSTAINLEVEL_MASK: c_uint = 0x00007f00	/* 127 = full, 0 = off, 0.75dB increments		*/;
pub const DCYSUSM_DECAYTIME_MASK: c_uint = 0x0000007f	/* Envelope decay time, log encoded			*/;
// 0 = 43.7msec, 1 = 21.8msec, 0x7f = 22msec
pub const LFOVAL2: c_uint = 0x17		/* Vibrato LFO register					*/;
pub const LFOVAL2_MASK: c_uint = 0x0000ffff	/* Current value of vibrato LFO state variable 		*/;
// 0x8000-n == 666*n usec delay
pub const IP: c_uint = 0x18		/* Initial pitch register				*/;
pub const IP_MASK: c_uint = 0x0000ffff	/* Exponential initial pitch shift			*/;
// 4 bits of octave, 12 bits of fractional octave
pub const IP_UNITY: c_uint = 0x0000e000	/* Unity pitch shift					*/;
pub const IFATN: c_uint = 0x19		/* Initial filter cutoff and attenuation register	*/;
// 6 most significant bits are semitones
// 2 least significant bits are fractions
pub const PEFE: c_uint = 0x1a		/* Pitch envelope and filter envelope amount register	*/;
// Signed 2's complement, +/- one octave peak extremes
// Signed 2's complement, +/- six octaves peak extremes
pub const FMMOD: c_uint = 0x1b		/* Vibrato/filter modulation from LFO register		*/;
pub const FMMOD_MODVIBRATO: c_uint = 0x0000ff00	/* Vibrato LFO modulation depth				*/;
// Signed 2's complement, +/- one octave extremes
pub const FMMOD_MOFILTER: c_uint = 0x000000ff	/* Filter LFO modulation depth				*/;
// Signed 2's complement, +/- three octave extremes
pub const TREMFRQ: c_uint = 0x1c		/* Tremolo amount and modulation LFO frequency register	*/;
pub const TREMFRQ_DEPTH: c_uint = 0x0000ff00	/* Tremolo depth					*/;
// Signed 2's complement, with +/- 12dB extremes
pub const TREMFRQ_FREQUENCY: c_uint = 0x000000ff	/* Tremolo LFO frequency				*/;
// ??Hz steps, maximum of ?? Hz.
pub const FM2FRQ2: c_uint = 0x1d		/* Vibrato amount and vibrato LFO frequency register	*/;
pub const FM2FRQ2_DEPTH: c_uint = 0x0000ff00	/* Vibrato LFO vibrato depth				*/;
// Signed 2's complement, +/- one octave extremes
pub const FM2FRQ2_FREQUENCY: c_uint = 0x000000ff	/* Vibrato LFO frequency				*/;
// 0.039Hz steps, maximum of 9.85 Hz.
pub const TEMPENV: c_uint = 0x1e		/* Tempory envelope register				*/;
pub const TEMPENV_MASK: c_uint = 0x0000ffff	/* 16-bit value						*/;
// NOTE: All channels contain internal variables; do
// not write to these locations.
// 0x1f: not used
// 32 cache registers (== 128 bytes) per channel follow.
// In stereo mode, the two channels' caches are concatenated into one,
// and hold the interleaved frames.
// The cache holds 64 frames, so the upper half is not used in 8-bit mode.
// All registers mentioned below count in frames. Shortcuts:
// CA = CCCA_CURRADDR, CRA = CCR_READADDRESS,
// CLA = CCR_CACHELOOPADDRHI:CLP_CACHELOOPADDR,
// CIS = CCR_CACHEINVALIDSIZE, LIS = CCR_LOOPINVALSIZE,
// CLF = CCR_CACHELOOPFLAG, LF = CCR_LOOPFLAG
// The cache is a ring buffer; CRA operates modulo 64.
// The cache is filled from (CA - CIS) into (CRA - CIS).
// The engine has a fetch threshold of 32 bytes, so it tries to keep
// CIS below 8 (16-bit stereo), 16 (16-bit mono, 8-bit stereo), or
// 32 (8-bit mono). The actual transfers are pretty unpredictable,
// especially if several voices are running.
// Frames are consumed at CRA, which is incremented afterwards,
// along with CA and CIS. This implies that the actual playback
// position always lags CA by exactly 64 frames.
// When CA reaches DSL_LOOPENDADDR, LF is set for one frame's time.
// LF's rising edge causes the current values of CA and CIS to be
// copied into CLA and LIS, resp., and CLF to be set.
// If CLF is set, the first LIS of the CIS frames are instead
// filled from (CLA - LIS), and CLF is subsequently reset.
pub const CD0: c_uint = 0x20		/* Cache data registers 0 .. 0x1f			*/;
pub const PTB: c_uint = 0x40		/* Page table base register				*/;
pub const PTB_MASK: c_uint = 0xfffff000	/* Physical address of the page table in host memory	*/;
pub const TCB: c_uint = 0x41		/* Tank cache base register    				*/;
pub const TCB_MASK: c_uint = 0xfffff000	/* Physical address of the bottom of host based TRAM	*/;
pub const ADCCR: c_uint = 0x42		/* ADC sample rate/stereo control register		*/;
pub const ADCCR_RCHANENABLE: c_uint = 0x00000010	/* Enables right channel for writing to the host       	*/;
pub const ADCCR_LCHANENABLE: c_uint = 0x00000008	/* Enables left channel for writing to the host		*/;
// NOTE: To guarantee phase coherency, both channels
// must be disabled prior to enabling both channels.
pub const A_ADCCR_RCHANENABLE: c_uint = 0x00000020;
pub const A_ADCCR_LCHANENABLE: c_uint = 0x00000010;
pub const A_ADCCR_SAMPLERATE_MASK: c_uint = 0x0000000F      /* Audigy sample rate convertor output rate		*/;
pub const ADCCR_SAMPLERATE_MASK: c_uint = 0x00000007	/* Sample rate convertor output rate			*/;
pub const ADCCR_SAMPLERATE_48: c_uint = 0x00000000	/* 48kHz sample rate					*/;
pub const ADCCR_SAMPLERATE_44: c_uint = 0x00000001	/* 44.1kHz sample rate					*/;
pub const ADCCR_SAMPLERATE_32: c_uint = 0x00000002	/* 32kHz sample rate					*/;
pub const ADCCR_SAMPLERATE_24: c_uint = 0x00000003	/* 24kHz sample rate					*/;
pub const ADCCR_SAMPLERATE_22: c_uint = 0x00000004	/* 22.05kHz sample rate					*/;
pub const ADCCR_SAMPLERATE_16: c_uint = 0x00000005	/* 16kHz sample rate					*/;
pub const ADCCR_SAMPLERATE_11: c_uint = 0x00000006	/* 11.025kHz sample rate				*/;
pub const ADCCR_SAMPLERATE_8: c_uint = 0x00000007	/* 8kHz sample rate					*/;
pub const A_ADCCR_SAMPLERATE_12: c_uint = 0x00000006	/* 12kHz sample rate					*/;
pub const A_ADCCR_SAMPLERATE_11: c_uint = 0x00000007	/* 11.025kHz sample rate				*/;
pub const A_ADCCR_SAMPLERATE_8: c_uint = 0x00000008	/* 8kHz sample rate					*/;
pub const FXWC: c_uint = 0x43		/* FX output write channels register			*/;
// When set, each bit enables the writing of the
// corresponding FX output channel (internal registers
// 0x20-0x3f) to host memory.  This mode of recording
// is 16bit, 48KHz only. All 32 channels can be enabled
// simultaneously.
pub const A_TBLSZ: c_uint = 0x43	/* Effects Tank Internal Table Size. Only low byte or register used */;
pub const TCBS: c_uint = 0x44		/* Tank cache buffer size register			*/;
pub const TCBS_MASK: c_uint = 0x00000007	/* Tank cache buffer size field				*/;
pub const TCBS_BUFFSIZE_16K: c_uint = 0x00000000;
pub const TCBS_BUFFSIZE_32K: c_uint = 0x00000001;
pub const TCBS_BUFFSIZE_64K: c_uint = 0x00000002;
pub const TCBS_BUFFSIZE_128K: c_uint = 0x00000003;
pub const TCBS_BUFFSIZE_256K: c_uint = 0x00000004;
pub const TCBS_BUFFSIZE_512K: c_uint = 0x00000005;
pub const TCBS_BUFFSIZE_1024K: c_uint = 0x00000006;
pub const TCBS_BUFFSIZE_2048K: c_uint = 0x00000007;
pub const MICBA: c_uint = 0x45		/* AC97 microphone buffer address register		*/;
pub const MICBA_MASK: c_uint = 0xfffff000	/* 20 bit base address					*/;
pub const ADCBA: c_uint = 0x46		/* ADC buffer address register				*/;
pub const ADCBA_MASK: c_uint = 0xfffff000	/* 20 bit base address					*/;
pub const FXBA: c_uint = 0x47		/* FX Buffer Address */;
pub const FXBA_MASK: c_uint = 0xfffff000	/* 20 bit base address					*/;
pub const A_HWM: c_uint = 0x48		/* High PCI Water Mark - word access, defaults to 3f */;
pub const MICBS: c_uint = 0x49		/* Microphone buffer size register			*/;
pub const ADCBS: c_uint = 0x4a		/* ADC buffer size register				*/;
pub const FXBS: c_uint = 0x4b		/* FX buffer size register				*/;
// The following mask values define the size of the ADC, MIC and FX buffers in bytes
pub const ADCBS_BUFSIZE_NONE: c_uint = 0x00000000;
pub const ADCBS_BUFSIZE_384: c_uint = 0x00000001;
pub const ADCBS_BUFSIZE_448: c_uint = 0x00000002;
pub const ADCBS_BUFSIZE_512: c_uint = 0x00000003;
pub const ADCBS_BUFSIZE_640: c_uint = 0x00000004;
pub const ADCBS_BUFSIZE_768: c_uint = 0x00000005;
pub const ADCBS_BUFSIZE_896: c_uint = 0x00000006;
pub const ADCBS_BUFSIZE_1024: c_uint = 0x00000007;
pub const ADCBS_BUFSIZE_1280: c_uint = 0x00000008;
pub const ADCBS_BUFSIZE_1536: c_uint = 0x00000009;
pub const ADCBS_BUFSIZE_1792: c_uint = 0x0000000a;
pub const ADCBS_BUFSIZE_2048: c_uint = 0x0000000b;
pub const ADCBS_BUFSIZE_2560: c_uint = 0x0000000c;
pub const ADCBS_BUFSIZE_3072: c_uint = 0x0000000d;
pub const ADCBS_BUFSIZE_3584: c_uint = 0x0000000e;
pub const ADCBS_BUFSIZE_4096: c_uint = 0x0000000f;
pub const ADCBS_BUFSIZE_5120: c_uint = 0x00000010;
pub const ADCBS_BUFSIZE_6144: c_uint = 0x00000011;
pub const ADCBS_BUFSIZE_7168: c_uint = 0x00000012;
pub const ADCBS_BUFSIZE_8192: c_uint = 0x00000013;
pub const ADCBS_BUFSIZE_10240: c_uint = 0x00000014;
pub const ADCBS_BUFSIZE_12288: c_uint = 0x00000015;
pub const ADCBS_BUFSIZE_14366: c_uint = 0x00000016;
pub const ADCBS_BUFSIZE_16384: c_uint = 0x00000017;
pub const ADCBS_BUFSIZE_20480: c_uint = 0x00000018;
pub const ADCBS_BUFSIZE_24576: c_uint = 0x00000019;
pub const ADCBS_BUFSIZE_28672: c_uint = 0x0000001a;
pub const ADCBS_BUFSIZE_32768: c_uint = 0x0000001b;
pub const ADCBS_BUFSIZE_40960: c_uint = 0x0000001c;
pub const ADCBS_BUFSIZE_49152: c_uint = 0x0000001d;
pub const ADCBS_BUFSIZE_57344: c_uint = 0x0000001e;
pub const ADCBS_BUFSIZE_65536: c_uint = 0x0000001f;
// On Audigy, the FX send amounts are not applied instantly, but determine
// targets towards which the following registers swerve gradually.
pub const A_CSBA: c_uint = 0x4c		/* FX send B & A current amounts			*/;
pub const A_CSDC: c_uint = 0x4d		/* FX send D & C current amounts			*/;
pub const A_CSFE: c_uint = 0x4e		/* FX send F & E current amounts			*/;
pub const A_CSHG: c_uint = 0x4f		/* FX send H & G current amounts			*/;
// NOTE: 0x50,51,52: 64-bit (split over voices 0 & 1)
pub const CDCS: c_uint = 0x50		/* CD-ROM digital channel status register		*/;
pub const GPSCS: c_uint = 0x51		/* General Purpose SPDIF channel status register	*/;
// Corresponding EMU10K1_DBG_* constants are in the public header
pub const DBG: c_uint = 0x52;
pub const A_SPSC: c_uint = 0x52		/* S/PDIF Input C Channel Status			*/;
pub const REG53: c_uint = 0x53		/* DO NOT PROGRAM THIS REGISTER!!! MAY DESTROY CHIP	*/;
// Corresponding A_DBG_* constants are in the public header
pub const A_DBG: c_uint = 0x53;
// NOTE: 0x54,55,56: 64-bit (split over voices 0 & 1)
pub const SPCS0: c_uint = 0x54		/* SPDIF output Channel Status 0 register	*/;
pub const SPCS1: c_uint = 0x55		/* SPDIF output Channel Status 1 register	*/;
pub const SPCS2: c_uint = 0x56		/* SPDIF output Channel Status 2 register	*/;
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
// 0x57: Not used
// The 32-bit CLIx and SOLEx registers all have one bit per channel control/status
pub const CLIEL: c_uint = 0x58		/* Channel loop interrupt enable low register	*/;
pub const CLIEH: c_uint = 0x59		/* Channel loop interrupt enable high register	*/;
pub const CLIPL: c_uint = 0x5a		/* Channel loop interrupt pending low register	*/;
pub const CLIPH: c_uint = 0x5b		/* Channel loop interrupt pending high register	*/;
// These cause CPF_STOP_MASK to be set shortly after CCCA_CURRADDR passes DSL_LOOPENDADDR.
// Subsequent changes to the address registers don't resume; clearing the bit here or in CPF does.
// The registers are NOT synchronized; the next serviced channel picks up immediately.
pub const SOLEL: c_uint = 0x5c		/* Stop on loop enable low register		*/;
pub const SOLEH: c_uint = 0x5d		/* Stop on loop enable high register		*/;
pub const SPBYPASS: c_uint = 0x5e		/* SPDIF BYPASS mode register			*/;
pub const SPBYPASS_SPDIF0_MASK: c_uint = 0x00000003	/* SPDIF 0 bypass mode				*/;
pub const SPBYPASS_SPDIF1_MASK: c_uint = 0x0000000c	/* SPDIF 1 bypass mode				*/;
// bypass mode: 0 - DSP; 1 - SPDIF A, 2 - SPDIF B, 3 - SPDIF C
pub const SPBYPASS_FORMAT: c_uint = 0x00000f00      /* If 1, SPDIF XX uses 24 bit, if 0 - 20 bit	*/;
pub const AC97SLOT: c_uint = 0x5f            /* additional AC97 slots enable bits		*/;
pub const AC97SLOT_REAR_RIGHT: c_uint = 0x01		/* Rear left					*/;
pub const AC97SLOT_REAR_LEFT: c_uint = 0x02		/* Rear right					*/;
pub const AC97SLOT_CNTR: c_uint = 0x10            /* Center enable				*/;
pub const AC97SLOT_LFE: c_uint = 0x20            /* LFE enable					*/;
pub const A_PCB: c_uint = 0x5f		/* PCB Revision					*/;
// NOTE: 0x60,61,62: 64-bit
pub const CDSRCS: c_uint = 0x60		/* CD-ROM Sample Rate Converter status register	*/;
pub const GPSRCS: c_uint = 0x61		/* General Purpose SPDIF sample rate cvt status */;
pub const ZVSRCS: c_uint = 0x62		/* ZVideo sample rate converter status		*/;
// NOTE: This one has no SPDIFLOCKED field
// Assumes sample lock
// These three bitfields apply to CDSRCS, GPSRCS, and (except as noted) ZVSRCS.
pub const SRCS_SPDIFVALID: c_uint = 0x04000000	/* SPDIF stream valid				*/;
pub const SRCS_SPDIFLOCKED: c_uint = 0x02000000	/* SPDIF stream locked				*/;
pub const SRCS_RATELOCKED: c_uint = 0x01000000	/* Sample rate locked				*/;
pub const SRCS_ESTSAMPLERATE: c_uint = 0x0007ffff	/* Do not modify this field.			*/;
// Note that these values can vary +/- by a small amount
pub const SRCS_SPDIFRATE_44: c_uint = 0x0003acd9;
pub const SRCS_SPDIFRATE_48: c_uint = 0x00040000;
pub const SRCS_SPDIFRATE_96: c_uint = 0x00080000;
pub const MICIDX: c_uint = 0x63            /* Microphone recording buffer index register   */;
pub const ADCIDX: c_uint = 0x64		/* ADC recording buffer index register		*/;
pub const A_ADCIDX: c_uint = 0x63;
pub const A_MICIDX: c_uint = 0x64;
pub const FXIDX: c_uint = 0x65		/* FX recording buffer index register		*/;
// The 32-bit HLIEx and HLIPx registers all have one bit per channel control/status
pub const HLIEL: c_uint = 0x66		/* Channel half loop interrupt enable low register	*/;
pub const HLIEH: c_uint = 0x67		/* Channel half loop interrupt enable high register	*/;
pub const HLIPL: c_uint = 0x68		/* Channel half loop interrupt pending low register	*/;
pub const HLIPH: c_uint = 0x69		/* Channel half loop interrupt pending high register	*/;
pub const A_SPRI: c_uint = 0x6a		/* S/PDIF Host Record Index (bypasses SRC)	*/;
pub const A_SPRA: c_uint = 0x6b		/* S/PDIF Host Record Address			*/;
pub const A_SPRC: c_uint = 0x6c		/* S/PDIF Host Record Control			*/;
pub const A_DICE: c_uint = 0x6d		/* Delayed Interrupt Counter & Enable		*/;
pub const A_TTB: c_uint = 0x6e		/* Tank Table Base				*/;
pub const A_TDOF: c_uint = 0x6f		/* Tank Delay Offset				*/;
// This is the MPU port on the card (via the game port)
pub const A_MUDATA1: c_uint = 0x70;
pub const A_MUCMD1: c_uint = 0x71;

// This is the MPU port on the Audigy Drive
pub const A_MUDATA2: c_uint = 0x72;
pub const A_MUCMD2: c_uint = 0x73;

// The next two are the Audigy equivalent of FXWC
// the Audigy can record any output (16bit, 48kHz, up to 64 channels simultaneously)
// Each bit selects a channel for recording
pub const A_FXWC1: c_uint = 0x74            /* Selects 0x7f-0x60 for FX recording           */;
pub const A_FXWC2: c_uint = 0x75		/* Selects 0x9f-0x80 for FX recording           */;
pub const A_EHC: c_uint = 0x76		/* Extended Hardware Control */;

pub const A_SPDIF_RATE_MASK: c_uint = 0x000000e0	/* Any other values for rates, just use 48000	*/;
pub const A_SPDIF_48000: c_uint = 0x00000000	/* kX calls this BYPASS				*/;
pub const A_SPDIF_192000: c_uint = 0x00000020;
pub const A_SPDIF_96000: c_uint = 0x00000040;
pub const A_SPDIF_44100: c_uint = 0x00000080;
pub const A_SPDIF_MUTED: c_uint = 0x000000c0;
// unclear if this sets the ADC rate as well.
pub const A_I2S_CAPTURE_48000: c_uint = 0x0;
pub const A_I2S_CAPTURE_192000: c_uint = 0x1;
pub const A_I2S_CAPTURE_96000: c_uint = 0x2;
pub const A_I2S_CAPTURE_44100: c_uint = 0x4;
pub const A_EHC_SRC48_MASK: c_uint = 0x0000e000	/* This sets the playback PCM rate on the P16V	*/;
pub const A_EHC_SRC48_BYPASS: c_uint = 0x00000000;
pub const A_EHC_SRC48_192: c_uint = 0x00002000;
pub const A_EHC_SRC48_96: c_uint = 0x00004000;
pub const A_EHC_SRC48_44: c_uint = 0x00008000;
pub const A_EHC_SRC48_MUTED: c_uint = 0x0000c000;
pub const A_EHC_P17V_TVM: c_uint = 0x00000001	/* Tank virtual memory mode			*/;
pub const A_EHC_P17V_SEL0_MASK: c_uint = 0x00030000	/* Aka A_EHC_P16V_PB_RATE; 00: 48, 01: 44.1, 10: 96, 11: 192 */;
pub const A_EHC_P17V_SEL1_MASK: c_uint = 0x000c0000;
pub const A_EHC_P17V_SEL2_MASK: c_uint = 0x00300000;
pub const A_EHC_P17V_SEL3_MASK: c_uint = 0x00c00000;
pub const A_EHC_ASYNC_BYPASS: c_uint = 0x80000000;
pub const A_SRT3: c_uint = 0x77		/* I2S0 Sample Rate Tracker Status		*/;
pub const A_SRT4: c_uint = 0x78		/* I2S1 Sample Rate Tracker Status		*/;
pub const A_SRT5: c_uint = 0x79		/* I2S2 Sample Rate Tracker Status		*/;
// - default to 0x01080000 on my audigy 2 ZS --rlrevell
pub const A_SRT_ESTSAMPLERATE: c_uint = 0x001fffff;
pub const A_SRT_RATELOCKED: c_uint = 0x01000000;
pub const A_TTDA: c_uint = 0x7a		/* Tank Table DMA Address			*/;
pub const A_TTDD: c_uint = 0x7b		/* Tank Table DMA Data				*/;
// In A_FXRT1 & A_FXRT2, the 0x80 bit of each byte completely disables the
// filter (CVCF_CURRENTFILTER) for the corresponding channel. There is no
// effect on the volume (CVCF_CURRENTVOLUME) or the interpolator's filter
// (CCCA_INTERPROM_MASK).
pub const A_FXRT2: c_uint = 0x7c;
pub const A_FXRT_CHANNELE: c_uint = 0x0000003f	/* Effects send bus number for channel's effects send E	*/;
pub const A_FXRT_CHANNELF: c_uint = 0x00003f00	/* Effects send bus number for channel's effects send F	*/;
pub const A_FXRT_CHANNELG: c_uint = 0x003f0000	/* Effects send bus number for channel's effects send G	*/;
pub const A_FXRT_CHANNELH: c_uint = 0x3f000000	/* Effects send bus number for channel's effects send H	*/;
pub const A_SENDAMOUNTS: c_uint = 0x7d;
pub const A_FXSENDAMOUNT_E_MASK: c_uint = 0xFF000000;
pub const A_FXSENDAMOUNT_F_MASK: c_uint = 0x00FF0000;
pub const A_FXSENDAMOUNT_G_MASK: c_uint = 0x0000FF00;
pub const A_FXSENDAMOUNT_H_MASK: c_uint = 0x000000FF;
// The send amounts for this one are the same as used with the emu10k1
pub const A_FXRT1: c_uint = 0x7e;
pub const A_FXRT_CHANNELA: c_uint = 0x0000003f;
pub const A_FXRT_CHANNELB: c_uint = 0x00003f00;
pub const A_FXRT_CHANNELC: c_uint = 0x003f0000;
pub const A_FXRT_CHANNELD: c_uint = 0x3f000000;
// 0x7f: Not used
// The public header defines the GPR and TRAM base addresses that
// are valid for _both_ CPU and DSP addressing.
// Each DSP microcode instruction is mapped into 2 doublewords
// NOTE: When writing, always write the LO doubleword first.  Reads can be in either order.
pub const MICROCODEBASE: c_uint = 0x400		/* Microcode data base address			*/;
pub const A_MICROCODEBASE: c_uint = 0x600;
//
// E-MU Digital Audio System overview
//
// - These cards use a regular PCI-attached Audigy chip (Alice2/Tina/Tina2);
// the PCIe variants simply put the Audigy chip behind a PCI bridge.
// - All physical PCM I/O is routed through an additional FPGA; the regular
// EXTIN/EXTOUT ports are unconnected.
// - The FPGA has a signal routing matrix, to connect each destination (output
// socket or capture channel) to a source (input socket or playback channel).
// - The FPGA is controlled via Audigy's GPIO port, while sample data is
// transmitted via proprietary EMU32 serial links. On first-generation
// E-MU 1010 cards, Audigy's I2S inputs are also used for sample data.
// - The Audio/Micro Dock is attached to Hana via EDI, a "network" link.
// - The Audigy chip operates in slave mode; the clock is supplied by the FPGA.
// Gen1 E-MU 1010 cards have two crystals (for 44.1 kHz and 48 kHz multiples),
// while the later cards use a single crystal and a PLL chip.
// - The whole card is switched to 2x/4x mode to achieve 88.2/96/176.4/192 kHz
// sample rates. Alice2/Tina keeps running at 44.1/48 kHz, but multiple channels
// are bundled.
// - The number of available EMU32/EDI channels is hit in 2x/4x mode, so the total
// number of usable inputs/outputs is limited, esp. with ADAT in use.
// - S/PDIF is unavailable in 4x mode (only over TOSLINK on newer 1010 cards) due
// to being unspecified at 176.4/192 kHz. Therefore, the Dock's S/PDIF channels
// can overlap with the Dock's ADC/DAC's high channels.
// - The code names are mentioned below and in the emu_chip_details table.
//
// EMU1010 FPGA registers
//
pub const EMU_HANA_DESTHI: c_uint = 0x00	/* 0000xxx  3 bits Link Destination */;
pub const EMU_HANA_DESTLO: c_uint = 0x01	/* 00xxxxx  5 bits */;
pub const EMU_HANA_SRCHI: c_uint = 0x02	/* 0000xxx  3 bits Link Source */;
pub const EMU_HANA_SRCLO: c_uint = 0x03	/* 00xxxxx  5 bits */;
pub const EMU_HANA_DOCK_PWR: c_uint = 0x04	/* 000000x  1 bits Audio Dock power */;
pub const EMU_HANA_DOCK_PWR_ON: c_uint = 0x01 /* Audio Dock power on */;
pub const EMU_HANA_WCLOCK: c_uint = 0x05	/* 0000xxx  3 bits Word Clock source select  */;
// Must be written after power on to reset DLL
// One is unable to detect the Audio dock without this
pub const EMU_HANA_WCLOCK_SRC_MASK: c_uint = 0x07;
pub const EMU_HANA_WCLOCK_INT_48K: c_uint = 0x00;
pub const EMU_HANA_WCLOCK_INT_44_1K: c_uint = 0x01;
pub const EMU_HANA_WCLOCK_HANA_SPDIF_IN: c_uint = 0x02;
pub const EMU_HANA_WCLOCK_HANA_ADAT_IN: c_uint = 0x03;
pub const EMU_HANA_WCLOCK_SYNC_BNC: c_uint = 0x04;
pub const EMU_HANA_WCLOCK_2ND_HANA: c_uint = 0x05;
pub const EMU_HANA_WCLOCK_SRC_RESERVED: c_uint = 0x06;
pub const EMU_HANA_WCLOCK_OFF: c_uint = 0x07 /* For testing, forces fallback to DEFCLOCK */;
pub const EMU_HANA_WCLOCK_MULT_MASK: c_uint = 0x18;
pub const EMU_HANA_WCLOCK_1X: c_uint = 0x00;
pub const EMU_HANA_WCLOCK_2X: c_uint = 0x08;
pub const EMU_HANA_WCLOCK_4X: c_uint = 0x10;
pub const EMU_HANA_WCLOCK_MULT_RESERVED: c_uint = 0x18;
// If the selected external clock source is/becomes invalid or incompatible
// with the clock multiplier, the clock source is reset to this value, and
// a WCLK_CHANGED interrupt is raised.
pub const EMU_HANA_DEFCLOCK: c_uint = 0x06	/* 000000x  1 bits Default Word Clock  */;
pub const EMU_HANA_DEFCLOCK_48K: c_uint = 0x00;
pub const EMU_HANA_DEFCLOCK_44_1K: c_uint = 0x01;
pub const EMU_HANA_UNMUTE: c_uint = 0x07	/* 000000x  1 bits Mute all audio outputs  */;
pub const EMU_MUTE: c_uint = 0x00;
pub const EMU_UNMUTE: c_uint = 0x01;
pub const EMU_HANA_FPGA_CONFIG: c_uint = 0x08	/* 00000xx  2 bits Config control of FPGAs  */;
pub const EMU_HANA_FPGA_CONFIG_AUDIODOCK: c_uint = 0x01 /* Set in order to program FPGA on Audio Dock */;
pub const EMU_HANA_FPGA_CONFIG_HANA: c_uint = 0x02 /* Set in order to program FPGA on Hana */;
pub const EMU_HANA_IRQ_ENABLE: c_uint = 0x09	/* 000xxxx  4 bits IRQ Enable  */;
pub const EMU_HANA_IRQ_WCLK_CHANGED: c_uint = 0x01;
pub const EMU_HANA_IRQ_ADAT: c_uint = 0x02;
pub const EMU_HANA_IRQ_DOCK: c_uint = 0x04;
pub const EMU_HANA_IRQ_DOCK_LOST: c_uint = 0x08;
pub const EMU_HANA_SPDIF_MODE: c_uint = 0x0a	/* 00xxxxx  5 bits SPDIF MODE  */;
pub const EMU_HANA_SPDIF_MODE_TX_CONSUMER: c_uint = 0x00;
pub const EMU_HANA_SPDIF_MODE_TX_PRO: c_uint = 0x01;
pub const EMU_HANA_SPDIF_MODE_TX_NOCOPY: c_uint = 0x02;
pub const EMU_HANA_SPDIF_MODE_RX_CONSUMER: c_uint = 0x00;
pub const EMU_HANA_SPDIF_MODE_RX_PRO: c_uint = 0x04;
pub const EMU_HANA_SPDIF_MODE_RX_NOCOPY: c_uint = 0x08;
pub const EMU_HANA_SPDIF_MODE_RX_INVALID: c_uint = 0x10;
pub const EMU_HANA_OPTICAL_TYPE: c_uint = 0x0b	/* 00000xx  2 bits ADAT or SPDIF in/out  */;
pub const EMU_HANA_OPTICAL_IN_SPDIF: c_uint = 0x00;
pub const EMU_HANA_OPTICAL_IN_ADAT: c_uint = 0x01;
pub const EMU_HANA_OPTICAL_OUT_SPDIF: c_uint = 0x00;
pub const EMU_HANA_OPTICAL_OUT_ADAT: c_uint = 0x02;
pub const EMU_HANA_MIDI_IN: c_uint = 0x0c	/* 000000x  1 bit  Control MIDI  */;
pub const EMU_HANA_MIDI_INA_FROM_HAMOA: c_uint = 0x01 /* HAMOA MIDI in to Alice 2 MIDI A */;
pub const EMU_HANA_MIDI_INA_FROM_DOCK1: c_uint = 0x02 /* Audio Dock-1 MIDI in to Alice 2 MIDI A */;
pub const EMU_HANA_MIDI_INA_FROM_DOCK2: c_uint = 0x03 /* Audio Dock-2 MIDI in to Alice 2 MIDI A */;
pub const EMU_HANA_MIDI_INB_FROM_HAMOA: c_uint = 0x08 /* HAMOA MIDI in to Alice 2 MIDI B */;
pub const EMU_HANA_MIDI_INB_FROM_DOCK1: c_uint = 0x10 /* Audio Dock-1 MIDI in to Alice 2 MIDI B */;
pub const EMU_HANA_MIDI_INB_FROM_DOCK2: c_uint = 0x18 /* Audio Dock-2 MIDI in to Alice 2 MIDI B */;
pub const EMU_HANA_DOCK_LEDS_1: c_uint = 0x0d	/* 000xxxx  4 bit  Audio Dock LEDs  */;
pub const EMU_HANA_DOCK_LEDS_1_MIDI1: c_uint = 0x01	/* MIDI 1 LED on */;
pub const EMU_HANA_DOCK_LEDS_1_MIDI2: c_uint = 0x02	/* MIDI 2 LED on */;
pub const EMU_HANA_DOCK_LEDS_1_SMPTE_IN: c_uint = 0x04	/* SMPTE IN LED on */;
pub const EMU_HANA_DOCK_LEDS_1_SMPTE_OUT: c_uint = 0x08	/* SMPTE OUT LED on */;
pub const EMU_HANA_DOCK_LEDS_2: c_uint = 0x0e	/* 0xxxxxx  6 bit  Audio Dock LEDs  */;
pub const EMU_HANA_DOCK_LEDS_2_44K: c_uint = 0x01	/* 44.1 kHz LED on */;
pub const EMU_HANA_DOCK_LEDS_2_48K: c_uint = 0x02	/* 48 kHz LED on */;
pub const EMU_HANA_DOCK_LEDS_2_96K: c_uint = 0x04	/* 96 kHz LED on */;
pub const EMU_HANA_DOCK_LEDS_2_192K: c_uint = 0x08	/* 192 kHz LED on */;
pub const EMU_HANA_DOCK_LEDS_2_LOCK: c_uint = 0x10	/* LOCK LED on */;
pub const EMU_HANA_DOCK_LEDS_2_EXT: c_uint = 0x20	/* EXT LED on */;
pub const EMU_HANA_DOCK_LEDS_3: c_uint = 0x0f	/* 0xxxxxx  6 bit  Audio Dock LEDs  */;
pub const EMU_HANA_DOCK_LEDS_3_CLIP_A: c_uint = 0x01	/* Mic A Clip LED on */;
pub const EMU_HANA_DOCK_LEDS_3_CLIP_B: c_uint = 0x02	/* Mic B Clip LED on */;
pub const EMU_HANA_DOCK_LEDS_3_SIGNAL_A: c_uint = 0x04	/* Signal A Clip LED on */;
pub const EMU_HANA_DOCK_LEDS_3_SIGNAL_B: c_uint = 0x08	/* Signal B Clip LED on */;
pub const EMU_HANA_DOCK_LEDS_3_MANUAL_CLIP: c_uint = 0x10	/* Manual Clip detection */;
pub const EMU_HANA_DOCK_LEDS_3_MANUAL_SIGNAL: c_uint = 0x20	/* Manual Signal detection */;
pub const EMU_HANA_ADC_PADS: c_uint = 0x10	/* 0000xxx  3 bit  Audio Dock ADC 14dB pads */;
pub const EMU_HANA_DOCK_ADC_PAD1: c_uint = 0x01	/* 14dB Attenuation on Audio Dock ADC 1 */;
pub const EMU_HANA_DOCK_ADC_PAD2: c_uint = 0x02	/* 14dB Attenuation on Audio Dock ADC 2 */;
pub const EMU_HANA_DOCK_ADC_PAD3: c_uint = 0x04	/* 14dB Attenuation on Audio Dock ADC 3 */;
pub const EMU_HANA_0202_ADC_PAD1: c_uint = 0x08	/* 14dB Attenuation on 0202 ADC 1 */;
pub const EMU_HANA_DOCK_MISC: c_uint = 0x11	/* 0xxxxxx  6 bit  Audio Dock misc bits */;
pub const EMU_HANA_DOCK_DAC1_MUTE: c_uint = 0x01	/* DAC 1 Mute */;
pub const EMU_HANA_DOCK_DAC2_MUTE: c_uint = 0x02	/* DAC 2 Mute */;
pub const EMU_HANA_DOCK_DAC3_MUTE: c_uint = 0x04	/* DAC 3 Mute */;
pub const EMU_HANA_DOCK_DAC4_MUTE: c_uint = 0x08	/* DAC 4 Mute */;
pub const EMU_HANA_DOCK_PHONES_192_DAC1: c_uint = 0x00	/* DAC 1 Headphones source at 192kHz */;
pub const EMU_HANA_DOCK_PHONES_192_DAC2: c_uint = 0x10	/* DAC 2 Headphones source at 192kHz */;
pub const EMU_HANA_DOCK_PHONES_192_DAC3: c_uint = 0x20	/* DAC 3 Headphones source at 192kHz */;
pub const EMU_HANA_DOCK_PHONES_192_DAC4: c_uint = 0x30	/* DAC 4 Headphones source at 192kHz */;
pub const EMU_HANA_MIDI_OUT: c_uint = 0x12	/* 00xxxxx  5 bit  Source for each MIDI out port */;
pub const EMU_HANA_MIDI_OUT_0202: c_uint = 0x01 /* 0202 MIDI from Alice 2. 0 = A, 1 = B */;
pub const EMU_HANA_MIDI_OUT_DOCK1: c_uint = 0x02 /* Audio Dock MIDI1 front, from Alice 2. 0 = A, 1 = B */;
pub const EMU_HANA_MIDI_OUT_DOCK2: c_uint = 0x04 /* Audio Dock MIDI2 rear, from Alice 2. 0 = A, 1 = B */;
pub const EMU_HANA_MIDI_OUT_SYNC2: c_uint = 0x08 /* Sync card. Not the actual MIDI out jack. 0 = A, 1 = B */;
pub const EMU_HANA_MIDI_OUT_LOOP: c_uint = 0x10 /* 0 = bits (3:0) normal. 1 = MIDI loopback enabled. */;
pub const EMU_HANA_DAC_PADS: c_uint = 0x13	/* 00xxxxx  5 bit  DAC 14dB attenuation pads */;
pub const EMU_HANA_DOCK_DAC_PAD1: c_uint = 0x01	/* 14dB Attenuation on AudioDock DAC 1. Left and Right */;
pub const EMU_HANA_DOCK_DAC_PAD2: c_uint = 0x02	/* 14dB Attenuation on AudioDock DAC 2. Left and Right */;
pub const EMU_HANA_DOCK_DAC_PAD3: c_uint = 0x04	/* 14dB Attenuation on AudioDock DAC 3. Left and Right */;
pub const EMU_HANA_DOCK_DAC_PAD4: c_uint = 0x08	/* 14dB Attenuation on AudioDock DAC 4. Left and Right */;
pub const EMU_HANA_0202_DAC_PAD1: c_uint = 0x10	/* 14dB Attenuation on 0202 DAC 1. Left and Right */;
// 0x14 - 0x1f Unused R/W registers
pub const EMU_HANA_IRQ_STATUS: c_uint = 0x20	/* 00xxxxx  5 bits IRQ Status  */;
// Same bits as for EMU_HANA_IRQ_ENABLE
// Reading the register resets it.
pub const EMU_HANA_OPTION_CARDS: c_uint = 0x21	/* 000xxxx  4 bits Presence of option cards */;
pub const EMU_HANA_OPTION_HAMOA: c_uint = 0x01	/* Hamoa (analog I/O) card present */;
pub const EMU_HANA_OPTION_SYNC: c_uint = 0x02	/* Sync card present */;
pub const EMU_HANA_OPTION_DOCK_ONLINE: c_uint = 0x04	/* Audio/Micro dock present and FPGA configured */;
pub const EMU_HANA_OPTION_DOCK_OFFLINE: c_uint = 0x08	/* Audio/Micro dock present and FPGA not configured */;
pub const EMU_HANA_ID: c_uint = 0x22	/* 1010101  7 bits ID byte & 0x7f = 0x55 with Alice2 */;
// 0010101  5 bits ID byte & 0x1f = 0x15 with Tina/2
pub const EMU_HANA_MAJOR_REV: c_uint = 0x23	/* 0000xxx  3 bit  Hana FPGA Major rev */;
pub const EMU_HANA_MINOR_REV: c_uint = 0x24	/* 0000xxx  3 bit  Hana FPGA Minor rev */;
pub const EMU_DOCK_MAJOR_REV: c_uint = 0x25	/* 0000xxx  3 bit  Audio Dock FPGA Major rev */;
pub const EMU_DOCK_MINOR_REV: c_uint = 0x26	/* 0000xxx  3 bit  Audio Dock FPGA Minor rev */;
pub const EMU_DOCK_BOARD_ID: c_uint = 0x27	/* 00000xx  2 bits Audio Dock ID pins */;
pub const EMU_DOCK_BOARD_ID0: c_uint = 0x00	/* ID bit 0 */;
pub const EMU_DOCK_BOARD_ID1: c_uint = 0x03	/* ID bit 1 */;
// The actual code disagrees about the bit width of the registers -
// the formula used is freq = 0x1770000 / (((X_HI << 5) | X_LO) + 1)
pub const EMU_HANA_WC_SPDIF_HI: c_uint = 0x28	/* 0xxxxxx  6 bit  SPDIF IN Word clock, upper 6 bits */;
pub const EMU_HANA_WC_SPDIF_LO: c_uint = 0x29	/* 0xxxxxx  6 bit  SPDIF IN Word clock, lower 6 bits */;
pub const EMU_HANA_WC_ADAT_HI: c_uint = 0x2a	/* 0xxxxxx  6 bit  ADAT IN Word clock, upper 6 bits */;
pub const EMU_HANA_WC_ADAT_LO: c_uint = 0x2b	/* 0xxxxxx  6 bit  ADAT IN Word clock, lower 6 bits */;
pub const EMU_HANA_WC_BNC_LO: c_uint = 0x2c	/* 0xxxxxx  6 bit  BNC IN Word clock, lower 6 bits */;
pub const EMU_HANA_WC_BNC_HI: c_uint = 0x2d	/* 0xxxxxx  6 bit  BNC IN Word clock, upper 6 bits */;
pub const EMU_HANA2_WC_SPDIF_HI: c_uint = 0x2e	/* 0xxxxxx  6 bit  HANA2 SPDIF IN Word clock, upper 6 bits */;
pub const EMU_HANA2_WC_SPDIF_LO: c_uint = 0x2f	/* 0xxxxxx  6 bit  HANA2 SPDIF IN Word clock, lower 6 bits */;
// 0x30 - 0x3f Unused Read only registers
// The meaning of this is not clear; kX-project just calls it "lock" in some info-only code.
pub const EMU_HANA_LOCK_STS_LO: c_uint = 0x38	/* 0xxxxxx  lower 6 bits */;
pub const EMU_HANA_LOCK_STS_HI: c_uint = 0x39	/* 0xxxxxx  upper 6 bits */;
//
// EMU1010 Audio Destinations
//
// Hana, original 1010,1212m,1820[m] using Alice2
// 0x00, 0x00-0x0f: 16 EMU32 channels to Alice2
// 0x01, 0x00-0x1f: 32 EDI channels to Audio Dock
// 0x00: Dock DAC 1 Left
// 0x04: Dock DAC 1 Right
// 0x08: Dock DAC 2 Left
// 0x0c: Dock DAC 2 Right
// 0x10: Dock DAC 3 Left
// 0x12: PHONES Left (n/a in 2x/4x mode; output mirrors DAC4 Left)
// 0x14: Dock DAC 3 Right
// 0x16: PHONES Right (n/a in 2x/4x mode; output mirrors DAC4 Right)
// 0x18: Dock DAC 4 Left
// 0x1a: S/PDIF Left
// 0x1c: Dock DAC 4 Right
// 0x1e: S/PDIF Right
// 0x02, 0x00: Hana S/PDIF Left
// 0x02, 0x01: Hana S/PDIF Right
// 0x03, 0x00: Hamoa DAC Left
// 0x03, 0x01: Hamoa DAC Right
// 0x04, 0x00-0x07: Hana ADAT
// 0x05, 0x00: I2S0 Left to Alice2
// 0x05, 0x01: I2S0 Right to Alice2
// 0x06, 0x00: I2S0 Left to Alice2
// 0x06, 0x01: I2S0 Right to Alice2
// 0x07, 0x00: I2S0 Left to Alice2
// 0x07, 0x01: I2S0 Right to Alice2
//
// Hana2 never released, but used Tina
// Not needed.
//
// Hana3, rev2 1010,1212m,1616[m] using Tina
// 0x00, 0x00-0x0f: 16 EMU32A channels to Tina
// 0x01, 0x00-0x1f: 32 EDI channels to Micro Dock
// 0x00: Dock DAC 1 Left
// 0x04: Dock DAC 1 Right
// 0x08: Dock DAC 2 Left
// 0x0c: Dock DAC 2 Right
// 0x10: Dock DAC 3 Left
// 0x12: Dock S/PDIF Left
// 0x14: Dock DAC 3 Right
// 0x16: Dock S/PDIF Right
// 0x18-0x1f: Dock ADAT 0-7
// 0x02, 0x00: Hana3 S/PDIF Left
// 0x02, 0x01: Hana3 S/PDIF Right
// 0x03, 0x00: Hamoa DAC Left
// 0x03, 0x01: Hamoa DAC Right
// 0x04, 0x00-0x07: Hana3 ADAT 0-7
// 0x05, 0x00-0x0f: 16 EMU32B channels to Tina
// 0x06-0x07: Not used
//
// HanaLite, rev1 0404 using Alice2
// HanaLiteLite, rev2 0404 using Tina
// 0x00, 0x00-0x0f: 16 EMU32 channels to Alice2/Tina
// 0x01: Not used
// 0x02, 0x00: S/PDIF Left
// 0x02, 0x01: S/PDIF Right
// 0x03, 0x00: DAC Left
// 0x03, 0x01: DAC Right
// 0x04-0x07: Not used
//
// Mana, Cardbus 1616 using Tina2
// 0x00, 0x00-0x0f: 16 EMU32A channels to Tina2
// 0x01, 0x00-0x1f: 32 EDI channels to Micro Dock
// (same as rev2 1010)
// 0x02: Not used
// 0x03, 0x00: Mana DAC Left
// 0x03, 0x01: Mana DAC Right
// 0x04, 0x00-0x0f: 16 EMU32B channels to Tina2
// 0x05-0x07: Not used
//
// 32-bit destinations of signal in the Hana FPGA. Destinations are either
// physical outputs of Hana, or outputs going to Alice2/Tina for capture -
// 16 x EMU_DST_ALICE2_EMU32_X (2x on rev2 boards). Which data is fed into
// a channel depends on the mixer control setting for each destination - see
// the register arrays in emumixer.c.
//
pub const EMU_DST_ALICE2_EMU32_0: c_uint = 0x000f	/* 16 EMU32 channels to Alice2 +0 to +0xf */;
// This channel is delayed by one sample.
pub const EMU_DST_ALICE2_EMU32_1: c_uint = 0x0000	/* 16 EMU32 channels to Alice2 +0 to +0xf */;
pub const EMU_DST_ALICE2_EMU32_2: c_uint = 0x0001	/* 16 EMU32 channels to Alice2 +0 to +0xf */;
pub const EMU_DST_ALICE2_EMU32_3: c_uint = 0x0002	/* 16 EMU32 channels to Alice2 +0 to +0xf */;
pub const EMU_DST_ALICE2_EMU32_4: c_uint = 0x0003	/* 16 EMU32 channels to Alice2 +0 to +0xf */;
pub const EMU_DST_ALICE2_EMU32_5: c_uint = 0x0004	/* 16 EMU32 channels to Alice2 +0 to +0xf */;
pub const EMU_DST_ALICE2_EMU32_6: c_uint = 0x0005	/* 16 EMU32 channels to Alice2 +0 to +0xf */;
pub const EMU_DST_ALICE2_EMU32_7: c_uint = 0x0006	/* 16 EMU32 channels to Alice2 +0 to +0xf */;
pub const EMU_DST_ALICE2_EMU32_8: c_uint = 0x0007	/* 16 EMU32 channels to Alice2 +0 to +0xf */;
pub const EMU_DST_ALICE2_EMU32_9: c_uint = 0x0008	/* 16 EMU32 channels to Alice2 +0 to +0xf */;
pub const EMU_DST_ALICE2_EMU32_A: c_uint = 0x0009	/* 16 EMU32 channels to Alice2 +0 to +0xf */;
pub const EMU_DST_ALICE2_EMU32_B: c_uint = 0x000a	/* 16 EMU32 channels to Alice2 +0 to +0xf */;
pub const EMU_DST_ALICE2_EMU32_C: c_uint = 0x000b	/* 16 EMU32 channels to Alice2 +0 to +0xf */;
pub const EMU_DST_ALICE2_EMU32_D: c_uint = 0x000c	/* 16 EMU32 channels to Alice2 +0 to +0xf */;
pub const EMU_DST_ALICE2_EMU32_E: c_uint = 0x000d	/* 16 EMU32 channels to Alice2 +0 to +0xf */;
pub const EMU_DST_ALICE2_EMU32_F: c_uint = 0x000e	/* 16 EMU32 channels to Alice2 +0 to +0xf */;
pub const EMU_DST_DOCK_DAC1_LEFT1: c_uint = 0x0100	/* Audio Dock DAC1 Left, 1st or 48kHz only */;
pub const EMU_DST_DOCK_DAC1_LEFT2: c_uint = 0x0101	/* Audio Dock DAC1 Left, 2nd or 96kHz */;
pub const EMU_DST_DOCK_DAC1_LEFT3: c_uint = 0x0102	/* Audio Dock DAC1 Left, 3rd or 192kHz */;
pub const EMU_DST_DOCK_DAC1_LEFT4: c_uint = 0x0103	/* Audio Dock DAC1 Left, 4th or 192kHz */;
pub const EMU_DST_DOCK_DAC1_RIGHT1: c_uint = 0x0104	/* Audio Dock DAC1 Right, 1st or 48kHz only */;
pub const EMU_DST_DOCK_DAC1_RIGHT2: c_uint = 0x0105	/* Audio Dock DAC1 Right, 2nd or 96kHz */;
pub const EMU_DST_DOCK_DAC1_RIGHT3: c_uint = 0x0106	/* Audio Dock DAC1 Right, 3rd or 192kHz */;
pub const EMU_DST_DOCK_DAC1_RIGHT4: c_uint = 0x0107	/* Audio Dock DAC1 Right, 4th or 192kHz */;
pub const EMU_DST_DOCK_DAC2_LEFT1: c_uint = 0x0108	/* Audio Dock DAC2 Left, 1st or 48kHz only */;
pub const EMU_DST_DOCK_DAC2_LEFT2: c_uint = 0x0109	/* Audio Dock DAC2 Left, 2nd or 96kHz */;
pub const EMU_DST_DOCK_DAC2_LEFT3: c_uint = 0x010a	/* Audio Dock DAC2 Left, 3rd or 192kHz */;
pub const EMU_DST_DOCK_DAC2_LEFT4: c_uint = 0x010b	/* Audio Dock DAC2 Left, 4th or 192kHz */;
pub const EMU_DST_DOCK_DAC2_RIGHT1: c_uint = 0x010c	/* Audio Dock DAC2 Right, 1st or 48kHz only */;
pub const EMU_DST_DOCK_DAC2_RIGHT2: c_uint = 0x010d	/* Audio Dock DAC2 Right, 2nd or 96kHz */;
pub const EMU_DST_DOCK_DAC2_RIGHT3: c_uint = 0x010e	/* Audio Dock DAC2 Right, 3rd or 192kHz */;
pub const EMU_DST_DOCK_DAC2_RIGHT4: c_uint = 0x010f	/* Audio Dock DAC2 Right, 4th or 192kHz */;
pub const EMU_DST_DOCK_DAC3_LEFT1: c_uint = 0x0110	/* Audio Dock DAC1 Left, 1st or 48kHz only */;
pub const EMU_DST_DOCK_DAC3_LEFT2: c_uint = 0x0111	/* Audio Dock DAC1 Left, 2nd or 96kHz */;
pub const EMU_DST_DOCK_DAC3_LEFT3: c_uint = 0x0112	/* Audio Dock DAC1 Left, 3rd or 192kHz */;
pub const EMU_DST_DOCK_DAC3_LEFT4: c_uint = 0x0113	/* Audio Dock DAC1 Left, 4th or 192kHz */;
pub const EMU_DST_DOCK_PHONES_LEFT1: c_uint = 0x0112	/* Audio Dock PHONES Left, 1st or 48kHz only */;
pub const EMU_DST_DOCK_PHONES_LEFT2: c_uint = 0x0113	/* Audio Dock PHONES Left, 2nd or 96kHz */;
pub const EMU_DST_DOCK_DAC3_RIGHT1: c_uint = 0x0114	/* Audio Dock DAC1 Right, 1st or 48kHz only */;
pub const EMU_DST_DOCK_DAC3_RIGHT2: c_uint = 0x0115	/* Audio Dock DAC1 Right, 2nd or 96kHz */;
pub const EMU_DST_DOCK_DAC3_RIGHT3: c_uint = 0x0116	/* Audio Dock DAC1 Right, 3rd or 192kHz */;
pub const EMU_DST_DOCK_DAC3_RIGHT4: c_uint = 0x0117	/* Audio Dock DAC1 Right, 4th or 192kHz */;
pub const EMU_DST_DOCK_PHONES_RIGHT1: c_uint = 0x0116	/* Audio Dock PHONES Right, 1st or 48kHz only */;
pub const EMU_DST_DOCK_PHONES_RIGHT2: c_uint = 0x0117	/* Audio Dock PHONES Right, 2nd or 96kHz */;
pub const EMU_DST_DOCK_DAC4_LEFT1: c_uint = 0x0118	/* Audio Dock DAC2 Left, 1st or 48kHz only */;
pub const EMU_DST_DOCK_DAC4_LEFT2: c_uint = 0x0119	/* Audio Dock DAC2 Left, 2nd or 96kHz */;
pub const EMU_DST_DOCK_DAC4_LEFT3: c_uint = 0x011a	/* Audio Dock DAC2 Left, 3rd or 192kHz */;
pub const EMU_DST_DOCK_DAC4_LEFT4: c_uint = 0x011b	/* Audio Dock DAC2 Left, 4th or 192kHz */;
pub const EMU_DST_DOCK_SPDIF_LEFT1: c_uint = 0x011a	/* Audio Dock SPDIF Left, 1st or 48kHz only */;
pub const EMU_DST_DOCK_SPDIF_LEFT2: c_uint = 0x011b	/* Audio Dock SPDIF Left, 2nd or 96kHz */;
pub const EMU_DST_DOCK_DAC4_RIGHT1: c_uint = 0x011c	/* Audio Dock DAC2 Right, 1st or 48kHz only */;
pub const EMU_DST_DOCK_DAC4_RIGHT2: c_uint = 0x011d	/* Audio Dock DAC2 Right, 2nd or 96kHz */;
pub const EMU_DST_DOCK_DAC4_RIGHT3: c_uint = 0x011e	/* Audio Dock DAC2 Right, 3rd or 192kHz */;
pub const EMU_DST_DOCK_DAC4_RIGHT4: c_uint = 0x011f	/* Audio Dock DAC2 Right, 4th or 192kHz */;
pub const EMU_DST_DOCK_SPDIF_RIGHT1: c_uint = 0x011e	/* Audio Dock SPDIF Right, 1st or 48kHz only */;
pub const EMU_DST_DOCK_SPDIF_RIGHT2: c_uint = 0x011f	/* Audio Dock SPDIF Right, 2nd or 96kHz */;
pub const EMU_DST_HANA_SPDIF_LEFT1: c_uint = 0x0200	/* Hana SPDIF Left, 1st or 48kHz only */;
pub const EMU_DST_HANA_SPDIF_LEFT2: c_uint = 0x0202	/* Hana SPDIF Left, 2nd or 96kHz */;
pub const EMU_DST_HANA_SPDIF_LEFT3: c_uint = 0x0204	/* Hana SPDIF Left, 3rd or 192kHz */;
pub const EMU_DST_HANA_SPDIF_LEFT4: c_uint = 0x0206	/* Hana SPDIF Left, 4th or 192kHz */;
pub const EMU_DST_HANA_SPDIF_RIGHT1: c_uint = 0x0201	/* Hana SPDIF Right, 1st or 48kHz only */;
pub const EMU_DST_HANA_SPDIF_RIGHT2: c_uint = 0x0203	/* Hana SPDIF Right, 2nd or 96kHz */;
pub const EMU_DST_HANA_SPDIF_RIGHT3: c_uint = 0x0205	/* Hana SPDIF Right, 3rd or 192kHz */;
pub const EMU_DST_HANA_SPDIF_RIGHT4: c_uint = 0x0207	/* Hana SPDIF Right, 4th or 192kHz */;
pub const EMU_DST_HAMOA_DAC_LEFT1: c_uint = 0x0300	/* Hamoa DAC Left, 1st or 48kHz only */;
pub const EMU_DST_HAMOA_DAC_LEFT2: c_uint = 0x0302	/* Hamoa DAC Left, 2nd or 96kHz */;
pub const EMU_DST_HAMOA_DAC_LEFT3: c_uint = 0x0304	/* Hamoa DAC Left, 3rd or 192kHz */;
pub const EMU_DST_HAMOA_DAC_LEFT4: c_uint = 0x0306	/* Hamoa DAC Left, 4th or 192kHz */;
pub const EMU_DST_HAMOA_DAC_RIGHT1: c_uint = 0x0301	/* Hamoa DAC Right, 1st or 48kHz only */;
pub const EMU_DST_HAMOA_DAC_RIGHT2: c_uint = 0x0303	/* Hamoa DAC Right, 2nd or 96kHz */;
pub const EMU_DST_HAMOA_DAC_RIGHT3: c_uint = 0x0305	/* Hamoa DAC Right, 3rd or 192kHz */;
pub const EMU_DST_HAMOA_DAC_RIGHT4: c_uint = 0x0307	/* Hamoa DAC Right, 4th or 192kHz */;
// In S/MUX mode, the samples of one channel are adjacent.
pub const EMU_DST_HANA_ADAT: c_uint = 0x0400	/* Hana ADAT 8 channel out +0 to +7 */;
pub const EMU_DST_ALICE_I2S0_LEFT: c_uint = 0x0500	/* Alice2 I2S0 Left */;
pub const EMU_DST_ALICE_I2S0_RIGHT: c_uint = 0x0501	/* Alice2 I2S0 Right */;
pub const EMU_DST_ALICE_I2S1_LEFT: c_uint = 0x0600	/* Alice2 I2S1 Left */;
pub const EMU_DST_ALICE_I2S1_RIGHT: c_uint = 0x0601	/* Alice2 I2S1 Right */;
pub const EMU_DST_ALICE_I2S2_LEFT: c_uint = 0x0700	/* Alice2 I2S2 Left */;
pub const EMU_DST_ALICE_I2S2_RIGHT: c_uint = 0x0701	/* Alice2 I2S2 Right */;
// Additional destinations for 1616(M)/Microdock
pub const EMU_DST_MDOCK_SPDIF_LEFT1: c_uint = 0x0112	/* Microdock S/PDIF OUT Left, 1st or 48kHz only */;
pub const EMU_DST_MDOCK_SPDIF_LEFT2: c_uint = 0x0113	/* Microdock S/PDIF OUT Left, 2nd or 96kHz */;
pub const EMU_DST_MDOCK_SPDIF_RIGHT1: c_uint = 0x0116	/* Microdock S/PDIF OUT Right, 1st or 48kHz only */;
pub const EMU_DST_MDOCK_SPDIF_RIGHT2: c_uint = 0x0117	/* Microdock S/PDIF OUT Right, 2nd or 96kHz  */;
pub const EMU_DST_MDOCK_ADAT: c_uint = 0x0118	/* Microdock S/PDIF ADAT 8 channel out +8 to +f */;
pub const EMU_DST_MANA_DAC_LEFT: c_uint = 0x0300	/* Headphone jack on 1010 cardbus? 44.1/48kHz only? */;
pub const EMU_DST_MANA_DAC_RIGHT: c_uint = 0x0301	/* Headphone jack on 1010 cardbus? 44.1/48kHz only? */;
//
// EMU1010 Audio Sources
//
// Hana, original 1010,1212m,1820[m] using Alice2
// 0x00, 0x00-0x1f: Silence
// 0x01, 0x00-0x1f: 32 EDI channels from Audio Dock
// 0x00: Dock Mic A
// 0x04: Dock Mic B
// 0x08: Dock ADC 1 Left
// 0x0c: Dock ADC 1 Right
// 0x10: Dock ADC 2 Left
// 0x14: Dock ADC 2 Right
// 0x18: Dock ADC 3 Left
// 0x1c: Dock ADC 3 Right
// 0x02, 0x00: Hamoa ADC Left
// 0x02, 0x01: Hamoa ADC Right
// 0x03, 0x00-0x0f: 16 inputs from Alice2 Emu32A output
// 0x03, 0x10-0x1f: 16 inputs from Alice2 Emu32B output
// 0x04, 0x00-0x07: Hana ADAT
// 0x05, 0x00: Hana S/PDIF Left
// 0x05, 0x01: Hana S/PDIF Right
// 0x06-0x07: Not used
//
// Hana2 never released, but used Tina
// Not needed.
//
// Hana3, rev2 1010,1212m,1616[m] using Tina
// 0x00, 0x00-0x1f: Silence
// 0x01, 0x00-0x1f: 32 EDI channels from Micro Dock
// 0x00: Dock Mic A
// 0x04: Dock Mic B
// 0x08: Dock ADC 1 Left
// 0x0c: Dock ADC 1 Right
// 0x10: Dock ADC 2 Left
// 0x12: Dock S/PDIF Left
// 0x14: Dock ADC 2 Right
// 0x16: Dock S/PDIF Right
// 0x18-0x1f: Dock ADAT 0-7
// 0x02, 0x00: Hamoa ADC Left
// 0x02, 0x01: Hamoa ADC Right
// 0x03, 0x00-0x0f: 16 inputs from Tina Emu32A output
// 0x03, 0x10-0x1f: 16 inputs from Tina Emu32B output
// 0x04, 0x00-0x07: Hana3 ADAT
// 0x05, 0x00: Hana3 S/PDIF Left
// 0x05, 0x01: Hana3 S/PDIF Right
// 0x06-0x07: Not used
//
// HanaLite, rev1 0404 using Alice2
// HanaLiteLite, rev2 0404 using Tina
// 0x00, 0x00-0x1f: Silence
// 0x01: Not used
// 0x02, 0x00: ADC Left
// 0x02, 0x01: ADC Right
// 0x03, 0x00-0x0f: 16 inputs from Alice2/Tina Emu32A output
// 0x03, 0x10-0x1f: 16 inputs from Alice2/Tina Emu32B output
// 0x04: Not used
// 0x05, 0x00: S/PDIF Left
// 0x05, 0x01: S/PDIF Right
// 0x06-0x07: Not used
//
// Mana, Cardbus 1616 using Tina2
// 0x00, 0x00-0x1f: Silence
// 0x01, 0x00-0x1f: 32 EDI channels from Micro Dock
// (same as rev2 1010)
// 0x02: Not used
// 0x03, 0x00-0x0f: 16 inputs from Tina2 Emu32A output
// 0x03, 0x10-0x1f: 16 inputs from Tina2 Emu32B output
// 0x04-0x07: Not used
//
// 32-bit sources of signal in the Hana FPGA. The sources are routed to
// destinations using a mixer control for each destination - see emumixer.c.
// Sources are either physical inputs of Hana, or inputs from Alice2/Tina -
// 16 x EMU_SRC_ALICE_EMU32A + 16 x EMU_SRC_ALICE_EMU32B.
//
pub const EMU_SRC_SILENCE: c_uint = 0x0000	/* Silence */;
pub const EMU_SRC_DOCK_MIC_A1: c_uint = 0x0100	/* Audio Dock Mic A, 1st or 48kHz only */;
pub const EMU_SRC_DOCK_MIC_A2: c_uint = 0x0101	/* Audio Dock Mic A, 2nd or 96kHz */;
pub const EMU_SRC_DOCK_MIC_A3: c_uint = 0x0102	/* Audio Dock Mic A, 3rd or 192kHz */;
pub const EMU_SRC_DOCK_MIC_A4: c_uint = 0x0103	/* Audio Dock Mic A, 4th or 192kHz */;
pub const EMU_SRC_DOCK_MIC_B1: c_uint = 0x0104	/* Audio Dock Mic B, 1st or 48kHz only */;
pub const EMU_SRC_DOCK_MIC_B2: c_uint = 0x0105	/* Audio Dock Mic B, 2nd or 96kHz */;
pub const EMU_SRC_DOCK_MIC_B3: c_uint = 0x0106	/* Audio Dock Mic B, 3rd or 192kHz */;
pub const EMU_SRC_DOCK_MIC_B4: c_uint = 0x0107	/* Audio Dock Mic B, 4th or 192kHz */;
pub const EMU_SRC_DOCK_ADC1_LEFT1: c_uint = 0x0108	/* Audio Dock ADC1 Left, 1st or 48kHz only */;
pub const EMU_SRC_DOCK_ADC1_LEFT2: c_uint = 0x0109	/* Audio Dock ADC1 Left, 2nd or 96kHz */;
pub const EMU_SRC_DOCK_ADC1_LEFT3: c_uint = 0x010a	/* Audio Dock ADC1 Left, 3rd or 192kHz */;
pub const EMU_SRC_DOCK_ADC1_LEFT4: c_uint = 0x010b	/* Audio Dock ADC1 Left, 4th or 192kHz */;
pub const EMU_SRC_DOCK_ADC1_RIGHT1: c_uint = 0x010c	/* Audio Dock ADC1 Right, 1st or 48kHz only */;
pub const EMU_SRC_DOCK_ADC1_RIGHT2: c_uint = 0x010d	/* Audio Dock ADC1 Right, 2nd or 96kHz */;
pub const EMU_SRC_DOCK_ADC1_RIGHT3: c_uint = 0x010e	/* Audio Dock ADC1 Right, 3rd or 192kHz */;
pub const EMU_SRC_DOCK_ADC1_RIGHT4: c_uint = 0x010f	/* Audio Dock ADC1 Right, 4th or 192kHz */;
pub const EMU_SRC_DOCK_ADC2_LEFT1: c_uint = 0x0110	/* Audio Dock ADC2 Left, 1st or 48kHz only */;
pub const EMU_SRC_DOCK_ADC2_LEFT2: c_uint = 0x0111	/* Audio Dock ADC2 Left, 2nd or 96kHz */;
pub const EMU_SRC_DOCK_ADC2_LEFT3: c_uint = 0x0112	/* Audio Dock ADC2 Left, 3rd or 192kHz */;
pub const EMU_SRC_DOCK_ADC2_LEFT4: c_uint = 0x0113	/* Audio Dock ADC2 Left, 4th or 192kHz */;
pub const EMU_SRC_DOCK_ADC2_RIGHT1: c_uint = 0x0114	/* Audio Dock ADC2 Right, 1st or 48kHz only */;
pub const EMU_SRC_DOCK_ADC2_RIGHT2: c_uint = 0x0115	/* Audio Dock ADC2 Right, 2nd or 96kHz */;
pub const EMU_SRC_DOCK_ADC2_RIGHT3: c_uint = 0x0116	/* Audio Dock ADC2 Right, 3rd or 192kHz */;
pub const EMU_SRC_DOCK_ADC2_RIGHT4: c_uint = 0x0117	/* Audio Dock ADC2 Right, 4th or 192kHz */;
pub const EMU_SRC_DOCK_ADC3_LEFT1: c_uint = 0x0118	/* Audio Dock ADC3 Left, 1st or 48kHz only */;
pub const EMU_SRC_DOCK_ADC3_LEFT2: c_uint = 0x0119	/* Audio Dock ADC3 Left, 2nd or 96kHz */;
pub const EMU_SRC_DOCK_ADC3_LEFT3: c_uint = 0x011a	/* Audio Dock ADC3 Left, 3rd or 192kHz */;
pub const EMU_SRC_DOCK_ADC3_LEFT4: c_uint = 0x011b	/* Audio Dock ADC3 Left, 4th or 192kHz */;
pub const EMU_SRC_DOCK_ADC3_RIGHT1: c_uint = 0x011c	/* Audio Dock ADC3 Right, 1st or 48kHz only */;
pub const EMU_SRC_DOCK_ADC3_RIGHT2: c_uint = 0x011d	/* Audio Dock ADC3 Right, 2nd or 96kHz */;
pub const EMU_SRC_DOCK_ADC3_RIGHT3: c_uint = 0x011e	/* Audio Dock ADC3 Right, 3rd or 192kHz */;
pub const EMU_SRC_DOCK_ADC3_RIGHT4: c_uint = 0x011f	/* Audio Dock ADC3 Right, 4th or 192kHz */;
pub const EMU_SRC_HAMOA_ADC_LEFT1: c_uint = 0x0200	/* Hamoa ADC Left, 1st or 48kHz only */;
pub const EMU_SRC_HAMOA_ADC_LEFT2: c_uint = 0x0202	/* Hamoa ADC Left, 2nd or 96kHz */;
pub const EMU_SRC_HAMOA_ADC_LEFT3: c_uint = 0x0204	/* Hamoa ADC Left, 3rd or 192kHz */;
pub const EMU_SRC_HAMOA_ADC_LEFT4: c_uint = 0x0206	/* Hamoa ADC Left, 4th or 192kHz */;
pub const EMU_SRC_HAMOA_ADC_RIGHT1: c_uint = 0x0201	/* Hamoa ADC Right, 1st or 48kHz only */;
pub const EMU_SRC_HAMOA_ADC_RIGHT2: c_uint = 0x0203	/* Hamoa ADC Right, 2nd or 96kHz */;
pub const EMU_SRC_HAMOA_ADC_RIGHT3: c_uint = 0x0205	/* Hamoa ADC Right, 3rd or 192kHz */;
pub const EMU_SRC_HAMOA_ADC_RIGHT4: c_uint = 0x0207	/* Hamoa ADC Right, 4th or 192kHz */;
pub const EMU_SRC_ALICE_EMU32A: c_uint = 0x0300	/* Alice2 EMU32a 16 outputs. +0 to +0xf */;
pub const EMU_SRC_ALICE_EMU32B: c_uint = 0x0310	/* Alice2 EMU32b 16 outputs. +0 to +0xf */;
// In S/MUX mode, the samples of one channel are adjacent.
pub const EMU_SRC_HANA_ADAT: c_uint = 0x0400	/* Hana ADAT 8 channel in +0 to +7 */;
pub const EMU_SRC_HANA_SPDIF_LEFT1: c_uint = 0x0500	/* Hana SPDIF Left, 1st or 48kHz only */;
pub const EMU_SRC_HANA_SPDIF_LEFT2: c_uint = 0x0502	/* Hana SPDIF Left, 2nd or 96kHz */;
pub const EMU_SRC_HANA_SPDIF_LEFT3: c_uint = 0x0504	/* Hana SPDIF Left, 3rd or 192kHz */;
pub const EMU_SRC_HANA_SPDIF_LEFT4: c_uint = 0x0506	/* Hana SPDIF Left, 4th or 192kHz */;
pub const EMU_SRC_HANA_SPDIF_RIGHT1: c_uint = 0x0501	/* Hana SPDIF Right, 1st or 48kHz only */;
pub const EMU_SRC_HANA_SPDIF_RIGHT2: c_uint = 0x0503	/* Hana SPDIF Right, 2nd or 96kHz */;
pub const EMU_SRC_HANA_SPDIF_RIGHT3: c_uint = 0x0505	/* Hana SPDIF Right, 3rd or 192kHz */;
pub const EMU_SRC_HANA_SPDIF_RIGHT4: c_uint = 0x0507	/* Hana SPDIF Right, 4th or 192kHz */;
// Additional inputs for 1616(M)/Microdock
pub const EMU_SRC_MDOCK_SPDIF_LEFT1: c_uint = 0x0112	/* Microdock S/PDIF Left, 1st or 48kHz only */;
pub const EMU_SRC_MDOCK_SPDIF_LEFT2: c_uint = 0x0113	/* Microdock S/PDIF Left, 2nd or 96kHz */;
pub const EMU_SRC_MDOCK_SPDIF_RIGHT1: c_uint = 0x0116	/* Microdock S/PDIF Right, 1st or 48kHz only */;
pub const EMU_SRC_MDOCK_SPDIF_RIGHT2: c_uint = 0x0117	/* Microdock S/PDIF Right, 2nd or 96kHz */;
pub const EMU_SRC_MDOCK_ADAT: c_uint = 0x0118	/* Microdock ADAT 8 channel in +8 to +f */;
// 0x600 and 0x700 no used
// ------------------- CONSTANTS --------------------
// ------------------- STRUCTURES --------------------
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_emu10k1_voice {
    pub number: c_uchar,
    pub use: c_uchar,
    pub dirty: c_uchar,
    pub last: c_uchar,
    pub pvoice): *mut *mut *mut void (interrupt)(struct snd_emu10k1 emu, struct snd_emu10k1_voice,
    pub epcm: *mut snd_emu10k1_pcm,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_emu10k1_pcm {
    pub emu: *mut snd_emu10k1,
    pub type: c_int,
    pub substream: *mut snd_pcm_substream,
    pub voices: [*mut snd_emu10k1_voice; NUM_EFX_PLAYBACK],
    pub extra: *mut snd_emu10k1_voice,
    pub running: c_ushort,
    pub first_ptr: c_ushort,
    pub resume_pos: snd_pcm_uframes_t,
    pub memblk: *mut snd_util_memblk,
    pub pitch_target: c_uint,
    pub start_addr: c_uint,
    pub ccca_start_addr: c_uint,
    pub /: *mut *mut unsigned int capture_ipr; / interrupt acknowledge mask,
    pub /: *mut *mut unsigned int capture_inte; / interrupt enable mask,
    pub /: *mut *mut unsigned int capture_ba_reg; / buffer address register,
    pub /: *mut *mut unsigned int capture_bs_reg; / buffer size register,
    pub /: *mut *mut unsigned int capture_idx_reg; / buffer index register,
    pub /: *mut *mut unsigned int capture_cr_val; / control value,
    pub /: *mut *mut unsigned int capture_cr_val2; / control value2 (for audigy),
    pub /: *mut *mut unsigned int capture_bs_val; / buffer size value,
    pub /: *mut *mut unsigned int capture_bufsize; / buffer size in bytes,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_emu10k1_pcm_mixer {
// mono, left, right x 8 sends (4 on emu10k1)
    pub send_routing: [c_uchar; 3][8],
    pub send_volume: [c_uchar; 3][8],
// 0x8000 is neutral. The mixer code rescales it to 0xffff to maintain
// backwards compatibility with user space.
    pub attn: [c_ushort; 3],
    pub epcm: *mut snd_emu10k1_pcm,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_emu10k1_memblk {
    pub mem: snd_util_memblk,
// private part
    pub mapped_page: int first_page, last_page, pages,,
    pub map_locked: c_uint,
    pub mapped_link: list_head,
    pub mapped_order_link: list_head,
}

pub const EMU10K1_MAX_TRAM_BLOCKS_PER_CODE: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_emu10k1_fx8010_ctl {
    pub /: *mut *mut list_head list; / list link container,
    pub vcount: c_uint,
    pub /: *mut *mut unsigned int count; / count of GPR (1..16),
    pub /: *mut *mut unsigned short gpr[32]; / GPR number(s),
    pub value: [c_int; 32],
    pub /: *mut *mut int min; / minimum range,
    pub /: *mut *mut int max; / maximum range,
    pub /: *mut *mut *mut unsigned int translation; / translation type (EMU10K1_GPR_TRANSLATION),
    pub kcontrol: *mut snd_kcontrol,
}

extern "C" {
    pub fn void(emu: *mut snd_fx8010_irq_handler_t)(struct snd_emu10k1, private_data: *mut c_void) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_emu10k1_fx8010_irq {
    pub next: *mut snd_emu10k1_fx8010_irq,
    pub handler: *mut snd_fx8010_irq_handler_t,
    pub gpr_running: c_ushort,
    pub private_data: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_emu10k1_fx8010_pcm {
    pub 1: active:,
    pub /: *mut *mut unsigned int channels; / 16-bit channels count,
    pub /: *mut *mut unsigned int tram_start; / initial ring buffer position in TRAM (in samples),
    pub /: *mut *mut unsigned int buffer_size; / count of buffered samples,
    pub /: *mut *mut unsigned short gpr_size; / GPR containing size of ring buffer in samples (host),
    pub /: *mut *mut unsigned short gpr_ptr; / GPR containing current pointer in the ring buffer (host = reset, FX8010),
    pub /: *mut *mut unsigned short gpr_count; / GPR containing count of samples between two interrupts (host),
    pub /: *mut *mut unsigned short gpr_tmpcount; / GPR containing current count of samples to interrupt (host = set, FX8010),
    pub /: *mut *mut unsigned short gpr_trigger; / GPR containing trigger (activate) information (host),
    pub /: *mut *mut unsigned short gpr_running; / GPR containing info if PCM is running (FX8010),
    pub /: *mut *mut unsigned char etram[32]; / external TRAM address & data,
    pub pcm_rec: snd_pcm_indirect,
    pub tram_pos: c_uint,
    pub tram_shift: c_uint,
    pub irq: snd_emu10k1_fx8010_irq,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_emu10k1_fx8010 {
    pub /: *mut *mut unsigned short extin_mask; / used external inputs (bitmask); not used for Audigy,
    pub /: *mut *mut unsigned short extout_mask; / used external outputs (bitmask); not used for Audigy,
    pub /: *mut *mut unsigned int itram_size; / internal TRAM size in samples,
    pub /: *mut *mut snd_dma_buffer etram_pages; / external TRAM pages and size,
    pub /: *mut *mut unsigned int dbg; / FX debugger register,
    pub name: [c_uchar; 128],
    pub /: *mut *mut int gpr_size; / size of allocated GPR controls,
    pub /: *mut *mut int gpr_count; / count of used kcontrols,
    pub /: *mut *mut list_head gpr_ctl; / GPR controls,
    pub lock: mutex,
    pub pcm: [snd_emu10k1_fx8010_pcm; 8],
    pub irq_lock: spinlock_t,
    pub irq_handlers: *mut snd_emu10k1_fx8010_irq,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_emu10k1_midi {
    pub emu: *mut snd_emu10k1,
    pub rmidi: *mut snd_rawmidi,
    pub substream_input: *mut snd_rawmidi_substream,
    pub substream_output: *mut snd_rawmidi_substream,
    pub midi_mode: c_uint,
    pub input_lock: spinlock_t,
    pub output_lock: spinlock_t,
    pub open_lock: spinlock_t,
    pub rx_enable: int tx_enable,,
    pub port: c_int,
    pub ipr_rx: int ipr_tx,,
    pub status): *mut *mut *mut void (interrupt)(struct snd_emu10k1 emu, unsigned int,
}

// Chip-o-logy:
// - All SB Live! cards use EMU10K1 chips
// - All SB Audigy cards use CA* chips, termed "emu10k2" by the driver
// - Original Audigy uses CA0100 "Alice"
// - Audigy 2 uses CA0102/CA10200 "Alice2"
// - Has an interface for CA0151 (P16V) "Alice3"
// - Audigy 2 Value uses CA0108/CA10300 "Tina"
// - Approximately a CA0102 with an on-chip CA0151 (P17V)
// - Audigy 2 ZS NB uses CA0109 "Tina2"
// - Cardbus version of CA0108
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_emu_chip_details {
    pub vendor: u32,
    pub device: u32,
    pub subsystem: u32,
    pub revision: c_uchar,
    pub /: *mut *mut unsigned char emu_model; / EMU model type,
    pub /: *mut *mut unsigned int emu10k1_chip:1; / Original SB Live. Not SB Live 24bit.,
// Redundant with emu10k2_chip being unset.
    pub /: *mut *mut unsigned int emu10k2_chip:1; / Audigy 1 or Audigy 2.,
    pub /: *mut *mut unsigned int ca0102_chip:1; / Audigy 1 or Audigy 2. Not SB Audigy 2 Value.,
// Redundant with ca0108_chip being unset.
    pub /: *mut *mut unsigned int ca0108_chip:1; / Audigy 2 Value,
    pub /: *mut *mut unsigned int ca_cardbus_chip:1; / Audigy 2 ZS Notebook,
    pub /: *mut *mut unsigned int ca0151_chip:1; / P16V,
    pub /: *mut *mut unsigned int spk20:1; / Stereo only,
    pub /: *mut *mut unsigned int spk71:1; / Has 7.1 speakers,
    pub /: *mut *mut unsigned int no_adat:1; / Has no ADAT, only SPDIF,
    pub /: *mut *mut unsigned int sblive51:1; / SBLive! 5.1 - extout 0x11 -> center, 0x12 -> lfe,
    pub /: *mut *mut unsigned int spdif_bug:1; / Has Spdif phasing bug,
    pub /: *mut *mut unsigned int ac97_chip:2; / Has an AC97 chip: 1 = mandatory, 2 = optional,
    pub /: *mut *mut unsigned int ecard:1; / APS EEPROM,
    pub /: *mut *mut unsigned int spi_dac:1; / SPI interface for DAC; requires ca0108_chip,
    pub /: *mut *mut unsigned int i2c_adc:1; / I2C interface for ADC; requires ca0108_chip,
    pub /: *mut *mut unsigned int adc_1361t:1; / Use Philips 1361T ADC,
    pub /: *mut *mut unsigned int invert_shared_spdif:1; / analog/digital switch inverted,
    pub driver: *const c_char,
    pub name: *const c_char,
    pub /: *const *const *const char id; / for backward compatibility - can be NULL if not needed,
}

pub const NUM_OUTPUT_DESTS: c_int = 28;
pub const NUM_INPUT_DESTS: c_int = 22;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_emu1010 {
    pub output_source: [c_uchar; NUM_OUTPUT_DESTS],
    pub input_source: [c_uchar; NUM_INPUT_DESTS],
    pub /: *mut *mut unsigned int adc_pads; / bit mask,
    pub /: *mut *mut unsigned int dac_pads; / bit mask,
    pub /: *mut *mut unsigned int wclock; / Cached register value,
    pub /: *mut *mut unsigned int word_clock; / Cached effective value,
    pub clock_source: c_uint,
    pub clock_fallback: c_uint,
    pub /: *mut *mut unsigned int optical_in; / 0:SPDIF, 1:ADAT,
    pub /: *mut *mut unsigned int optical_out; / 0:SPDIF, 1:ADAT,
    pub work: work_struct,
    pub lock: mutex,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_emu10k1 {
    pub irq: c_int,
    pub /: *mut *mut unsigned long port; / I/O port number,
    pub 1: enable_ir:,
    pub :1: unsigned int support_tlv,
// Contains profile of card capabilities
    pub card_capabilities: *const snd_emu_chip_details,
    pub /: *mut *mut unsigned int audigy; / is Audigy?,
    pub /: *mut *mut unsigned int revision; / chip revision,
    pub /: *mut *mut unsigned int serial; / serial number,
    pub /: *mut *mut unsigned short model; / subsystem id,
    pub /: *mut *mut unsigned int ecard_ctrl; / ecard control bits,
    pub /: *mut *mut unsigned int address_mode; / address mode,
    pub /: *mut *mut unsigned long dma_mask; / PCI DMA mask,
    pub /: *mut *mut bool iommu_workaround; / IOMMU workaround needed,
    pub /: *mut *mut int max_cache_pages; / max memory size / PAGE_SIZE,
    pub /: *mut *mut snd_dma_buffer silent_page; / silent page,
    pub /: *mut *mut snd_dma_buffer ptb_pages; / page table pages,
    pub p16v_dma_dev: snd_dma_device,
    pub p16v_buffer: *mut snd_dma_buffer,
    pub /: *mut *mut *mut snd_util_memhdr memhdr; / page allocation list,
    pub mapped_link_head: list_head,
    pub mapped_order_link_head: list_head,
    pub page_ptr_table: *mut c_void,
    pub page_addr_table: *mut c_ulong,
    pub memblk_lock: spinlock_t,
    pub /: *mut *mut unsigned int spdif_bits[3]; / s/pdif out setup,
    pub i2c_capture_source: c_uint,
    pub i2c_capture_volume: [u8; 4][2],
    pub /: *mut *mut snd_emu10k1_fx8010 fx8010; / FX8010 info,
    pub gpr_base: c_int,
    pub ac97: *mut snd_ac97,
    pub pci: *mut pci_dev,
    pub card: *mut snd_card,
    pub pcm: *mut snd_pcm,
    pub pcm_mic: *mut snd_pcm,
    pub pcm_efx: *mut snd_pcm,
    pub pcm_multi: *mut snd_pcm,
    pub pcm_p16v: *mut snd_pcm,
    pub synth_lock: spinlock_t,
    pub synth: *mut c_void,
    pub emu): *mut *mut int (get_synth_voice)(struct snd_emu10k1,
    pub lock: spinlock_t reg_lock; // high-level driver,
    pub lock: spinlock_t emu_lock; // low-level i/o,
    pub lock: spinlock_t voice_lock; // voice allocator,
    pub /: *mut *mut spinlock_t spi_lock; / serialises access to spi port,
    pub /: *mut *mut spinlock_t i2c_lock; / serialises access to i2c port,
    pub voices: [snd_emu10k1_voice; NUM_G],
    pub p16v_device_offset: c_int,
    pub p16v_capture_source: u32,
    pub p16v_capture_channel: u32,
    pub emu1010: snd_emu1010,
    pub pcm_mixer: [snd_emu10k1_pcm_mixer; 32],
    pub efx_pcm_mixer: [snd_emu10k1_pcm_mixer; NUM_EFX_PLAYBACK],
    pub ctl_send_routing: *mut snd_kcontrol,
    pub ctl_send_volume: *mut snd_kcontrol,
    pub ctl_attn: *mut snd_kcontrol,
    pub ctl_efx_send_routing: *mut snd_kcontrol,
    pub ctl_efx_send_volume: *mut snd_kcontrol,
    pub ctl_efx_attn: *mut snd_kcontrol,
    pub ctl_clock_source: *mut snd_kcontrol,
    pub status): *mut *mut *mut void (hwvol_interrupt)(struct snd_emu10k1 emu, unsigned int,
    pub status): *mut *mut *mut void (capture_interrupt)(struct snd_emu10k1 emu, unsigned int,
    pub status): *mut *mut *mut void (capture_mic_interrupt)(struct snd_emu10k1 emu, unsigned int,
    pub status): *mut *mut *mut void (capture_efx_interrupt)(struct snd_emu10k1 emu, unsigned int,
    pub status): *mut *mut *mut void (spdif_interrupt)(struct snd_emu10k1 emu, unsigned int,
    pub emu): *mut *mut void (dsp_interrupt)(struct snd_emu10k1,
    pub emu): *mut *mut void (gpio_interrupt)(struct snd_emu10k1,
    pub emu): *mut *mut void (p16v_interrupt)(struct snd_emu10k1,
    pub pcm_capture_substream: *mut snd_pcm_substream,
    pub pcm_capture_mic_substream: *mut snd_pcm_substream,
    pub pcm_capture_efx_substream: *mut snd_pcm_substream,
    pub timer: *mut snd_timer,
    pub midi: snd_emu10k1_midi,
    pub /: *mut *mut snd_emu10k1_midi midi2; / for audigy,
    pub efx_voices_mask: [c_uint; 2],
    pub next_free_voice: c_uint,
    pub firmware: *const firmware,
    pub dock_fw: *const firmware,

    pub saved_ptr: *mut c_uint,
    pub saved_gpr: *mut c_uint,
    pub tram_val_saved: *mut c_uint,
    pub tram_addr_saved: *mut c_uint,
    pub saved_icode: *mut c_uint,
    pub p16v_saved: *mut c_uint,
    pub saved_hcfg: unsigned int saved_a_iocfg,,
    pub suspend: bool,

}

extern "C" {
    pub fn snd_emu10k1_pcm(emu: *mut snd_emu10k1, device: c_int) -> c_int;
}
extern "C" {
    pub fn snd_emu10k1_pcm_mic(emu: *mut snd_emu10k1, device: c_int) -> c_int;
}
extern "C" {
    pub fn snd_emu10k1_pcm_efx(emu: *mut snd_emu10k1, device: c_int) -> c_int;
}
extern "C" {
    pub fn snd_p16v_pcm(emu: *mut snd_emu10k1, device: c_int) -> c_int;
}
extern "C" {
    pub fn snd_p16v_mixer(emu: *mut *mut snd_emu10k1) -> c_int;
}
extern "C" {
    pub fn snd_emu10k1_pcm_multi(emu: *mut snd_emu10k1, device: c_int) -> c_int;
}
extern "C" {
    pub fn snd_emu10k1_fx8010_pcm(emu: *mut snd_emu10k1, device: c_int) -> c_int;
}
extern "C" {
    pub fn snd_emu10k1_mixer(emu: *mut *mut snd_emu10k1, pcm_device: c_int, multi_device: c_int) -> c_int;
}
extern "C" {
    pub fn snd_emu10k1_timer(emu: *mut *mut snd_emu10k1, device: c_int) -> c_int;
}
extern "C" {
    pub fn snd_emu10k1_fx8010_new(emu: *mut snd_emu10k1, device: c_int) -> c_int;
}
extern "C" {
    pub fn snd_emu10k1_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn snd_emu10k1_voice_init(emu: *mut *mut snd_emu10k1, voice: c_int);
}
extern "C" {
    pub fn snd_emu10k1_init_efx(emu: *mut snd_emu10k1) -> c_int;
}
extern "C" {
    pub fn snd_emu10k1_free_efx(emu: *mut snd_emu10k1);
}
extern "C" {
    pub fn snd_emu10k1_fx8010_tram_setup(emu: *mut snd_emu10k1, size: u32) -> c_int;
}
extern "C" {
    pub fn snd_emu10k1_done(emu: *mut *mut snd_emu10k1) -> c_int;
}
// I/O functions
extern "C" {
    pub fn snd_emu10k1_ptr_read(emu: *mut *mut snd_emu10k1, reg: c_uint, chn: c_uint) -> c_uint;
}
extern "C" {
    pub fn snd_emu10k1_ptr_write(emu: *mut snd_emu10k1, reg: c_uint, chn: c_uint, data: c_uint);
}
extern "C" {
    pub fn snd_emu10k1_ptr_write_multiple(emu: *mut snd_emu10k1, chn: c_uint, ...);
}
extern "C" {
    pub fn snd_emu10k1_ptr20_read(emu: *mut *mut snd_emu10k1, reg: c_uint, chn: c_uint) -> c_uint;
}
extern "C" {
    pub fn snd_emu10k1_ptr20_write(emu: *mut snd_emu10k1, reg: c_uint, chn: c_uint, data: c_uint);
}
extern "C" {
    pub fn snd_emu10k1_spi_write(emu: *mut *mut snd_emu10k1, data: c_uint) -> c_int;
}
extern "C" {
    pub fn snd_emu10k1_i2c_write(emu: *mut snd_emu10k1, reg: u32, value: u32) -> c_int;
}
extern "C" {
    pub fn snd_emu1010_fpga_write_lock(emu: *mut snd_emu10k1, reg: u32, value: u32);
}
extern "C" {
    pub fn snd_emu1010_fpga_write(emu: *mut snd_emu10k1, reg: u32, value: u32);
}
extern "C" {
    pub fn snd_emu1010_fpga_read(emu: *mut snd_emu10k1, reg: u32, value: *mut u32);
}
extern "C" {
    pub fn snd_emu1010_fpga_link_dst_src_write(emu: *mut snd_emu10k1, dst: u32, src: u32);
}
extern "C" {
    pub fn snd_emu1010_fpga_link_dst_src_read(emu: *mut snd_emu10k1, dst: u32) -> u32;
}
extern "C" {
    pub fn snd_emu1010_get_raw_rate(emu: *mut snd_emu10k1, src: u8) -> c_int;
}
extern "C" {
    pub fn snd_emu1010_update_clock(emu: *mut snd_emu10k1);
}
extern "C" {
    pub fn snd_emu1010_load_firmware_entry(emu: *mut snd_emu10k1, dock: c_int, fw_entry: *const firmware);
}
extern "C" {
    pub fn snd_emu10k1_efx_read(emu: *mut snd_emu10k1, pc: c_uint) -> c_uint;
}
extern "C" {
    pub fn snd_emu10k1_intr_enable(emu: *mut snd_emu10k1, intrenb: c_uint);
}
extern "C" {
    pub fn snd_emu10k1_intr_disable(emu: *mut snd_emu10k1, intrenb: c_uint);
}
extern "C" {
    pub fn snd_emu10k1_voice_intr_enable(emu: *mut snd_emu10k1, voicenum: c_uint);
}
extern "C" {
    pub fn snd_emu10k1_voice_intr_disable(emu: *mut snd_emu10k1, voicenum: c_uint);
}
extern "C" {
    pub fn snd_emu10k1_voice_intr_ack(emu: *mut snd_emu10k1, voicenum: c_uint);
}
extern "C" {
    pub fn snd_emu10k1_voice_half_loop_intr_enable(emu: *mut snd_emu10k1, voicenum: c_uint);
}
extern "C" {
    pub fn snd_emu10k1_voice_half_loop_intr_disable(emu: *mut snd_emu10k1, voicenum: c_uint);
}
extern "C" {
    pub fn snd_emu10k1_voice_half_loop_intr_ack(emu: *mut snd_emu10k1, voicenum: c_uint);
}

extern "C" {
    pub fn snd_emu10k1_voice_set_loop_stop(emu: *mut snd_emu10k1, voicenum: c_uint);
}
extern "C" {
    pub fn snd_emu10k1_voice_clear_loop_stop(emu: *mut snd_emu10k1, voicenum: c_uint);
}

extern "C" {
    pub fn snd_emu10k1_voice_set_loop_stop_multiple(emu: *mut snd_emu10k1, voices: u64);
}
extern "C" {
    pub fn snd_emu10k1_voice_clear_loop_stop_multiple(emu: *mut snd_emu10k1, voices: u64);
}
extern "C" {
    pub fn snd_emu10k1_voice_clear_loop_stop_multiple_atomic(emu: *mut snd_emu10k1, voices: u64) -> c_int;
}
extern "C" {
    pub fn snd_emu10k1_wait(emu: *mut snd_emu10k1, wait: c_uint);
}
extern "C" {
    pub fn snd_emu10k1_ac97_read(ac97: *mut snd_ac97, reg: c_ushort) -> c_ushort;
}
extern "C" {
    pub fn snd_emu10k1_ac97_write(ac97: *mut snd_ac97, reg: c_ushort, data: c_ushort);
}

extern "C" {
    pub fn snd_emu10k1_suspend_regs(emu: *mut snd_emu10k1);
}
extern "C" {
    pub fn snd_emu10k1_resume_init(emu: *mut snd_emu10k1);
}
extern "C" {
    pub fn snd_emu10k1_resume_regs(emu: *mut snd_emu10k1);
}
extern "C" {
    pub fn snd_emu10k1_efx_alloc_pm_buffer(emu: *mut snd_emu10k1) -> c_int;
}
extern "C" {
    pub fn snd_emu10k1_efx_free_pm_buffer(emu: *mut snd_emu10k1);
}
extern "C" {
    pub fn snd_emu10k1_efx_suspend(emu: *mut snd_emu10k1);
}
extern "C" {
    pub fn snd_emu10k1_efx_resume(emu: *mut snd_emu10k1);
}
extern "C" {
    pub fn snd_p16v_alloc_pm_buffer(emu: *mut snd_emu10k1) -> c_int;
}
extern "C" {
    pub fn snd_p16v_free_pm_buffer(emu: *mut snd_emu10k1);
}
extern "C" {
    pub fn snd_p16v_suspend(emu: *mut snd_emu10k1);
}
extern "C" {
    pub fn snd_p16v_resume(emu: *mut snd_emu10k1);
}

// memory allocation
extern "C" {
    pub fn snd_emu10k1_free_pages(emu: *mut snd_emu10k1, blk: *mut snd_util_memblk) -> c_int;
}
extern "C" {
    pub fn snd_emu10k1_synth_free(emu: *mut snd_emu10k1, blk: *mut snd_util_memblk) -> c_int;
}
extern "C" {
    pub fn snd_emu10k1_synth_memset(emu: *mut snd_emu10k1, blk: *mut snd_util_memblk, offset: c_int, size: c_int, value: u8) -> c_int;
}
extern "C" {
    pub fn snd_emu10k1_synth_copy_from_user(emu: *mut snd_emu10k1, blk: *mut snd_util_memblk, offset: c_int, data: *const char __user, size: c_int, xor: u32) -> c_int;
}
extern "C" {
    pub fn snd_emu10k1_memblk_map(emu: *mut snd_emu10k1, blk: *mut snd_emu10k1_memblk) -> c_int;
}
// voice allocation
extern "C" {
    pub fn snd_emu10k1_voice_free(emu: *mut snd_emu10k1, pvoice: *mut snd_emu10k1_voice) -> c_int;
}
// MIDI uart
extern "C" {
    pub fn snd_emu10k1_midi(emu: *mut *mut snd_emu10k1) -> c_int;
}
extern "C" {
    pub fn snd_emu10k1_audigy_midi(emu: *mut *mut snd_emu10k1) -> c_int;
}
// proc interface
extern "C" {
    pub fn snd_emu10k1_proc_init(emu: *mut *mut snd_emu10k1) -> c_int;
}
// fx8010 irq handler
