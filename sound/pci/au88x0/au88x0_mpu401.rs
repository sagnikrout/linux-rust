//! Automatically rewritten from C to Rust
//! Source: sound/pci/au88x0/au88x0_mpu401.c
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
// Copyright (c) by Jaroslav Kysela <perex@perex.cz>
// Routines for control of MPU-401 in UART mode
//
// Modified for the Aureal Vortex based Soundcards
// by Manuel Jander (mjande@embedded.cl).
//

// Check for mpu401 mmio support.
// MPU401 legacy support is only provided as a emergency fallback
// for older versions of ALSA. Its usage is strongly discouraged.

// Macro flag: #define VORTEX_MPU401_LEGACY

// Vortex MPU401 defines.
pub const MIDI_CLOCK_DIV: c_uint = 0x61;
// Standart MPU401 defines.
pub const MPU401_RESET: c_uint = 0xff;
pub const MPU401_ENTER_UART: c_uint = 0x3f;
pub const MPU401_ACK: c_uint = 0xfe;
#[no_mangle]
unsafe extern "C" fn snd_vortex_midi(vortex: *mut vortex_t) -> c_int {
    static int snd_vortex_midi(vortex_t *vortex)
    {
    struct snd_rawmidi *rmidi;
    int temp, mode;
    struct snd_mpu401 *mpu;
    unsigned long port;

// EnableHardCodedMPU401Port()
// Enable Legacy MIDI Interface port.
    port = (0x03 << 5);	/* FIXME: static address. 0x330 */
    temp =
    (hwread(vortex.mmio, VORTEX_CTRL) & ~CTRL_MIDI_PORT) |
    CTRL_MIDI_EN | port;
    hwwrite(vortex.mmio, VORTEX_CTRL, temp);

// Disable Legacy MIDI Interface port.
    temp =
    (hwread(vortex.mmio, VORTEX_CTRL) & ~CTRL_MIDI_PORT) &
    ~CTRL_MIDI_EN;
    hwwrite(vortex.mmio, VORTEX_CTRL, temp);

// Mpu401UartInit()
    mode = 1;
    temp = hwread(vortex.mmio, VORTEX_CTRL2) & 0xffff00cf;
    temp |= (MIDI_CLOCK_DIV << 8) | ((mode >> 24) & 0xff) << 4;
    hwwrite(vortex.mmio, VORTEX_CTRL2, temp);
    hwwrite(vortex.mmio, VORTEX_MIDI_CMD, MPU401_RESET);
// Check if anything is OK.
    temp = hwread(vortex.mmio, VORTEX_MIDI_DATA);
    if (temp != MPU401_ACK /*0xfe */ ) {
    dev_err(vortex.card.dev, "midi port doesn't acknowledge!\n");
    return -ENODEV;
    }
// Enable MPU401 interrupts.
    hwwrite(vortex.mmio, VORTEX_IRQ_CTRL,
    hwread(vortex.mmio, VORTEX_IRQ_CTRL) | IRQ_MIDI);
// Create MPU401 instance.

    temp = snd_mpu401_uart_new(vortex.card, 0, MPU401_HW_MPU401, 0x330,
    MPU401_INFO_IRQ_HOOK, -1, &rmidi);
    if (temp) {
    hwwrite(vortex.mmio, VORTEX_CTRL,
    (hwread(vortex.mmio, VORTEX_CTRL) &
    ~CTRL_MIDI_PORT) & ~CTRL_MIDI_EN);
    return temp;
    }

    port = (unsigned long)(vortex.mmio + VORTEX_MIDI_DATA);
    temp = snd_mpu401_uart_new(vortex.card, 0, MPU401_HW_AUREAL, port,
    MPU401_INFO_INTEGRATED | MPU401_INFO_MMIO |
    MPU401_INFO_IRQ_HOOK, -1, &rmidi);
    if (temp) {
    hwwrite(vortex.mmio, VORTEX_CTRL,
    (hwread(vortex.mmio, VORTEX_CTRL) &
    ~CTRL_MIDI_PORT) & ~CTRL_MIDI_EN);
    return temp;
    }
    mpu = rmidi.private_data;
    mpu.cport = (unsigned long)(vortex.mmio + VORTEX_MIDI_CMD);

// Overwrite MIDI name
    snprintf(rmidi.name, sizeof(rmidi.name), "%s MIDI %d", CARD_NAME_SHORT , vortex.card.number);
    vortex.rmidi = rmidi;
    return 0;
    }
