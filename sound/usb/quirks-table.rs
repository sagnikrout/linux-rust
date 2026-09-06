//! Automatically rewritten from C Header to Rust Module
//! Source: sound/usb/quirks-table.h
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
// ALSA USB Audio Driver
//
// Copyright (c) 2002 by Takashi Iwai <tiwai@suse.de>,
// Clemens Ladisch <clemens@ladisch.de>
//
// The contents of this file are part of the driver's id_table.
//
// In a perfect world, this file would be empty.
//
// Use this for devices where other interfaces are standard compliant,
// to prevent the quirk being applied to those interfaces. (To work with
// hotplugging, bDeviceClass must be set to USB_CLASS_PER_INTERFACE.)
//

// A standard entry matching with vid/pid and the audio class/subclass

// Quirk .driver_info, followed by the definition of the quirk entry;
// put like QUIRK_DRIVER_INFO { ... } in each entry of the quirk table
//

//
// Macros for quirk data entries
//
// Quirk data entry for ignoring the interface

// Quirk data entry for a standard audio interface

// Quirk data entry for a standard MIDI interface

// Quirk data entry for a standard mixer interface

// Quirk data entry for Yamaha MIDI

// Quirk data entry for Edirol UAxx

// Quirk data entry for raw bytes interface

// Quirk composite array terminator

// Quirk data entry for composite quirks;
// followed by the quirk array that is terminated with QUIRK_COMPOSITE_END
// e.g. QUIRK_DATA_COMPOSITE { { quirk1 }, { quirk2 },..., QUIRK_COMPOSITE_END }
//

// Quirk data entry for a fixed audio endpoint;
// followed by audioformat definition
// e.g. QUIRK_DATA_AUDIOFORMAT(n) { .formats = xxx, ... }
//

// Quirk data entry for a fixed MIDI endpoint;
// followed by snd_usb_midi_endpoint_info definition
// e.g. QUIRK_DATA_MIDI_FIXED_ENDPOINT(n) { .out_cables = x, .in_cables = y }
//

// Quirk data entry for a MIDIMAN MIDI endpoint

// Quirk data entry for a EMAGIC MIDI endpoint

//
// Here we go... the quirk table definition begins:
//
// FTDI devices
// .vendor_name = "STARR LABS",
// .product_name = "Starr Labs MIDI USB device",
// Creative BT-D1
// E-Mu 0202 USB
// E-Mu 0404 USB
// E-Mu Tracker Pre
// E-Mu 0204 USB
// Ktmicro Usb_audio device
//
// Creative Technology, Ltd Live! Cam Sync HD [VF0770]
// The device advertises 8 formats, but only a rate of 48kHz is honored by the
// hardware and 24 bits give chopped audio, so only report the one working
// combination.
//
// HP Wireless Audio
// When not ignored, causes instability issues for some users, forcing them to
// skip the entire module.
//
// Mixer
// Playback
// Capture
// HID Device, .ifnum = 3
//
// Logitech QuickCam: bDeviceClass is vendor-specific, so generic interface
// class matches do not take effect without an explicit ID match.
//
// Yamaha devices
//

// .vendor_name = "Yamaha",
// .product_name = "MOX6/MOX8",
// .vendor_name = "Yamaha",
// .product_name = "THR10",
// .vendor_name = "Yamaha",
// .product_name = "Steinberg UR22",
// .vendor_name = "Yamaha",
// .product_name = "THR5A",
// .vendor_name = "Yamaha",
// .product_name = "THR10C",
// .vendor_name = "Yamaha",
// .product_name = "CDS3000",
// .vendor_name = "Yamaha",
// .product_name = "P-125",

// this catches most recent vendor-specific Yamaha devices
//
// Roland/RolandED/Edirol/BOSS devices
//
// Has ID 0x0099 when not in "Advanced Driver" mode.
// The UM-2EX has only one input, but we cannot detect this.
// has ID 0x009d when not in "Advanced Driver" mode
// thanks to Emiliano Grilli <emillo@libero.it>
// for helping researching this data
// This quirk is for the "Advanced Driver" mode of the Edirol UA-5.
// If the advanced mode switch at the back of the unit is off, the
// UA-5 has ID 0x0582/0x0011 and is standard compliant (no quirks),
// but offers only 16-bit PCM.
// In advanced mode, the UA-5 will output S24_3LE samples (two
// channels) at the rate indicated on the front switch, including
// the 96kHz sample rate.
//
// has ID 0x0013 when not in "Advanced Driver" mode
// has ID 0x0015 when not in "Advanced Driver" mode
// has ID 0x0017 when not in "Advanced Driver" mode
// has ID 0x001c when not in "Advanced Driver" mode
// has ID 0x001e when not in "Advanced Driver" mode
// has ID 0x0024 when not in "Advanced Driver" mode
//
// This quirk is for the "Advanced Driver" mode. If off, the UA-20
// has ID 0x0026 and is standard compliant, but has only 16-bit PCM
// and no MIDI.
//
// has ID 0x0028 when not in "Advanced Driver" mode
// has ID 0x002a when not in "Advanced Driver" mode
// This quirk is for the "Advanced" modes of the Edirol UA-700.
// If the sample format switch is not in an advanced setting, the
// UA-700 has ID 0x0582/0x002c and is standard compliant (no quirks),
// but offers only 16-bit PCM and no MIDI.
//
// has ID 0x002e when not in "Advanced Driver" mode
// has ID 0x0030 when not in "Advanced Driver" mode
// has ID 0x0034 when not in "Advanced Driver" mode
//
// Has ID 0x0038 when not in "Advanced Driver" mode;
// later revisions use IDs 0x0054 and 0x00a2.
//
// This quirk is for the "Advanced Driver" mode.  If off, the GS-10
// has ID 0x003c and is standard compliant, but has only 16-bit PCM
// and no MIDI.
//
// has ID 0x0041 when not in "Advanced Driver" mode
// has ID 0x0043 when not in "Advanced Driver" mode
// has ID 0x0049 when not in "Advanced Driver" mode
// .vendor_name = "EDIROL",
// .product_name = "UR-80",
// in the 96 kHz modes, only interface 1 is there
// has ID 0x004a when not in "Advanced Driver" mode
// .vendor_name = "EDIROL",
// .product_name = "UR-80",
// has ID 0x004e when not in "Advanced Driver" mode
// has ID 0x004f when not in "Advanced Driver" mode
//
// This quirk is for the "Advanced Driver" mode. If off, the UA-3FX
// is standard compliant, but has only 16-bit PCM.
//
// has ID 0x0066 when not in "Advanced Driver" mode
// .vendor_name = "EDIROL",
// .product_name = "PCR-1",
// has ID 0x0067 when not in "Advanced Driver" mode
// .vendor_name = "EDIROL",
// .product_name = "PCR-1",
// has ID 0x006e when not in "Advanced Driver" mode
// This quirk is for the "Advanced" modes of the Edirol UA-25.
// If the switch is not in an advanced setting, the UA-25 has
// ID 0x0582/0x0073 and is standard compliant (no quirks), but
// offers only 16-bit PCM at 44.1 kHz and no MIDI.
//
// has ID 0x0076 when not in "Advanced Driver" mode
// has ID 0x007b when not in "Advanced Driver" mode
// "RD" or "RD-700SX"?
// has ID 0x0081 when not in "Advanced Driver" mode
// has ID 0x008c when not in "Advanced Driver" mode
//
// This quirk is for the "Advanced Driver" mode. If off, the UA-4FX
// is standard compliant, but has only 16-bit PCM and no MIDI.
//
// Edirol M-16DX
// Advanced modes of the Edirol UA-25EX.
// For the standard mode, UA-25EX has ID 0582:00e7, which
// offers only 16-bit PCM at 44.1 kHz and no MIDI.
//
// Edirol UM-3G
// BOSS ME-25
// only 44.1 kHz works at the moment
// .vendor_name = "Roland",
// .product_name = "OCTO-CAPTURE",
// only 44.1 kHz works at the moment
// .vendor_name = "Roland",
// .product_name = "QUAD-CAPTURE",
// .vendor_name = "Roland",
// .product_name = "UA-22",
// UA101 and co are supported by another driver
// this catches most recent vendor-specific Roland devices
// Guillemot devices
//
// This is for the "Windows Edition" where the external MIDI ports are
// the only MIDI ports; the control data is reported through HID
// interfaces.  The "Macintosh Edition" has ID 0xd002 and uses standard
// compliant USB MIDI ports for external MIDI and controls.
//
// Midiman/M-Audio devices
//
// For hardware revision 1.05; in the later revisions (1.10 and
// 1.21), 0x1031 is the ID for the device without firmware.
// Thanks to Olaf Giesbrecht <Olaf_Giesbrecht@yahoo.de>
//
// Interfaces 0-2 are "Windows-compatible", 16-bit only,
// and share endpoints with the other interfaces.
// Ignore them.  The other interfaces can do 24 bits,
// but captured samples are big-endian (see usbaudio.c).
//
// .vendor_name = "M-Audio",
// .product_name = "Ozone Academic",
// M-Audio Micro
// .vendor_name = "M-Audio",
// .product_name = "Fast Track C400",
// Playback
// Capture
// MIDI: Interface = 4
// .vendor_name = "M-Audio",
// .product_name = "Fast Track C600",
// Playback
// Capture
// MIDI: Interface = 4
// .vendor_name = "M-Audio",
// .product_name = "Fast Track Ultra",
// interface 3 (MIDI) is standard compliant
// .vendor_name = "M-Audio",
// .product_name = "Fast Track Ultra 8R",
// interface 3 (MIDI) is standard compliant
//
// M-Audio Venom
//
// The AudioControl interface times out on every GET_CUR request,
// which adds around 47 seconds to the card registration and
// freezes the device, blocking streaming.
// Using an explicit composite quirk to skip the mixer entirely.
//
// Casio devices
// this ID is used by several devices without a product ID
// Mark of the Unicorn devices
// thanks to Robert A. Lerche <ral 'at' msbit.com>
// Emagic devices
// .product_name = "AMT8",
// .product_name = "MT4",
// KORG devices
// .product_name = "PANDORA PX5D",
// .product_name = "ToneLab ST",
// .product_name = "ToneLab EX",
// AKAI devices
// Akai MPC Element
// Steinberg devices
// Steinberg MI2
// Steinberg MI4
// TerraTec devices
// Stanton ScratchAmp
// Novation EMS devices
// .vendor_name = "Novation",
// .product_name = "Nocturn",
// .vendor_name = "Novation",
// .product_name = "Launchpad",
// .vendor_name = "Novation",
// .product_name = "Mininova",
// Access Music devices
// VirusTI Desktop
// Native Instruments MK2 series
// Komplete Audio 6
// Traktor Audio 6
// Traktor Audio 10
// QinHeng devices
// KeithMcMillen Stringport
// Miditech devices
// Central Music devices
// this ID used by both Miditech MidiStudio-2 and CME UF-x
// Digidesign Mbox
// Thanks to Clemens Ladisch <clemens@ladisch.de>
// DIGIDESIGN MBOX 2
// DIGIDESIGN MBOX 3
// Tascam US122 MKII - playback-only support
// Denon DN-X1600
// Microsoft XboxLive Headset/Xbox Communicator
// playback
// capture
// Reloop Play
//
// ZOOM R16/24 in audio interface mode.
// Playback requires an extra four byte LE length indicator
// at the start of each isochronous packet. This quirk is
// enabled in create_standard_audio_quirk().
//
// Some USB MIDI devices don't have an audio control interface,
// so we have to grab MIDI streaming interfaces here.
//
// Rane SL-1
// disabled due to regression for other devices;
// see https://bugzilla.kernel.org/show_bug.cgi?id=199905
//

//
// Nura's first gen headphones use Cambridge Silicon Radio's vendor
// ID, but it looks like the product ID actually is only for Nura.
// The capture interface does not work at all (even on Windows),
// and only the 48 kHz sample rate works for the playback interface.
//
// Playback

//
// The AudioBox USB advertises S24_3LE as the only supported format
// for both playback and capture. It does not support S16_LE despite
// being a USB full-speed device.
//
// Bower's & Wilkins PX headphones only support the 48 kHz sample rate
// even though it advertises more. The capture interface doesn't work
// even on windows.
//
// Playback
// MOTU Microbook II
//
// PIONEER DJ DDJ-SX3
// PCM is 12 channels out, 10 channels in @ 44.1 fixed
// interface 0, vendor class alt setting 1 for endpoints 5 and 0x86
// The feedback for the output is the input.
//
// Pioneer DJ DJM-250MK2
// PCM is 8 channels out @ 48 fixed (endpoint 0x01)
// and 8 channels in @ 48 fixed (endpoint 0x82).
//
// Both playback and recording is working, even simultaneously.
//
// Playback channels could be mapped to:
// - CH1
// - CH2
// - AUX
//
// Recording channels could be mapped to:
// - Post CH1 Fader
// - Post CH2 Fader
// - Cross Fader A
// - Cross Fader B
// - MIC
// - AUX
// - REC OUT
//
// There is remaining problem with recording directly from PHONO/LINE.
// If we map a channel to:
// - CH1 Control Tone PHONO
// - CH1 Control Tone LINE
// - CH2 Control Tone PHONO
// - CH2 Control Tone LINE
// it is silent.
// There is no signal even on other operating systems with official drivers.
// The signal appears only when a supported application is started.
// This needs to be investigated yet...
// (there is quite a lot communication on the USB in both directions)
//
// In current version this mixer could be used for playback
// and for recording from vinyls (through Post CH* Fader)
// but not for DVS (Digital Vinyl Systems) like in Mixxx.
//
// PIONEER DJ DDJ-RB
// PCM is 4 channels out, 2 dummy channels in @ 44.1 fixed
// The feedback for the output is the dummy input.
//
// PIONEER DJ DDJ-RR
// PCM is 6 channels out & 4 channels in @ 44.1 fixed
//
// PIONEER DJ DDJ-SR2
// PCM is 4 channels out, 6 channels in @ 44.1 fixed
// The Feedback for the output is the input
//
// Pioneer DJ DJM-900NXS2
// 10 channels playback & 12 channels capture @ 44.1/48/96kHz S24LE
//
// PIONEER DJ DDJ-800
// PCM is 6 channels out, 6 channels in @ 44.1 fixed
// The Feedback for the output is the input
//
// Pioneer DJ / AlphaTheta DJM-A9
// 10 channels playback & 12 channels capture @ 44.1/48/96kHz S24LE
//
// Pioneer DJ DJM-V10
//
// Pioneer DJ / AlphaTheta DJM-S11
//
// Audio Control (unknown purpose)
// HID
//
// MacroSilicon MS2100/MS2106 based AV capture cards
//
// These claim 96kHz 1ch in the descriptors, but are actually 48kHz 2ch.
// They also need QUIRK_FLAG_ALIGN_TRANSFER, which makes one wonder if
// they pretend to be 96kHz mono as a workaround for stereo being broken
// by that...
//
// They also have an issue with initial stream alignment that causes the
// channels to be swapped and out of phase, which is dealt with in quirks.c.
//
// MacroSilicon MS2109 based HDMI capture cards
//
// These claim 96kHz 1ch in the descriptors, but are actually 48kHz 2ch.
// They also need QUIRK_FLAG_ALIGN_TRANSFER, which makes one wonder if
// they pretend to be 96kHz mono as a workaround for stereo being broken
// by that...
//
// They also have an issue with initial stream alignment that causes the
// channels to be swapped and out of phase, which is dealt with in quirks.c.
//
// Pioneer DJ DJM-750
// 8 channels playback & 8 channels capture @ 44.1/48/96kHz S24LE
//
// Pioneer DJ DJM-750MK2
// 10 channels playback & 12 channels capture @ 48kHz S24LE
//
// Pioneer DJ DJM-850
// 8 channels playback and 8 channels capture @ 44.1/48/96kHz S24LE
// Playback on EP 0x05
// Capture on EP 0x86
//
// Pioneer DJ DJM-450
// PCM is 8 channels out @ 48 fixed (endpoint 0x01)
// and 8 channels in @ 48 fixed (endpoint 0x82).
//
// Sennheiser GSP670
// Change order of interfaces loaded
//
// Communication
// Recording
// Main
//
// Fiero SC-01 (firmware v1.0.0 @ 48 kHz)
//
// Playback
// Capture
//
// Fiero SC-01 (firmware v1.0.0 @ 96 kHz)
//
// Playback
// Capture
//
// Fiero SC-01 (firmware v1.1.0)
//
// Playback
// Capture
// Advanced modes of the Mythware XA001AU.
// For the standard mode, Mythware XA001AU has ID ffad:a001
//

// Only claim interface 0 */ \
//
// Three modes depending on sample rate band,
// with different channel counts for in/out
// \

// Arturia AudioFuse 16Rig Audio
// AF16Rig MIDI has USB PID 0xaf21 and appears to work OK without quirks

// These are disabled because I don't have the required hardware to test
// them. I suspect that the ADAT clock might not follow 176400 or 192000
// because the AF16Rig won't accept ADAT audio data at those rates.
//

