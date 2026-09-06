//! Automatically rewritten from C to Rust
//! Source: sound/pci/emu10k1/irq.c
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
// Creative Labs, Inc.
// Routines for IRQ control of EMU10K1 chips
//

#[no_mangle]
pub unsafe extern "C" fn snd_emu10k1_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    irqreturn_t snd_emu10k1_interrupt(int irq, void *dev_id)
    {
    struct snd_emu10k1 *emu = dev_id;
    unsigned int status, orig_status;
    let mut handled: c_int = 0;
    let mut timeout: c_int = 0;
    while ((status = inl(emu.port + IPR)) != 0) {
    handled = 1;
    if ((status & 0xffffffff) == 0xffffffff) {
    dev_info(emu.card.dev,
    "Suspected sound card removal\n");
    break;
    }
    if (++timeout == 1000) {
    dev_info(emu.card.dev, "emu10k1 irq routine failure\n");
    break;
    }
    orig_status = status;
    if (status & IPR_PCIERROR) {
    dev_err(emu.card.dev, "interrupt: PCI error\n");
    snd_emu10k1_intr_disable(emu, INTE_PCIERRORENABLE);
    status &= ~IPR_PCIERROR;
    }
    if (status & (IPR_VOLINCR|IPR_VOLDECR|IPR_MUTE)) {
    if (emu.hwvol_interrupt)
    emu.hwvol_interrupt(emu, status);
    else
    snd_emu10k1_intr_disable(emu, INTE_VOLINCRENABLE|INTE_VOLDECRENABLE|INTE_MUTEENABLE);
    status &= ~(IPR_VOLINCR|IPR_VOLDECR|IPR_MUTE);
    }
    if (status & IPR_CHANNELLOOP) {
    struct snd_emu10k1_voice *pvoice;
    int voice;
    let mut voice_max: c_int = status & IPR_CHANNELNUMBERMASK;
    u32 val;
    val = snd_emu10k1_ptr_read(emu, CLIPL, 0);
    pvoice = emu.voices;
    for (voice = 0; voice <= voice_max; voice++) {
    if (voice == 0x20)
    val = snd_emu10k1_ptr_read(emu, CLIPH, 0);
    if (val & 1) {
    if (pvoice.use && pvoice.interrupt != core::ptr::null_mut()) {
    pvoice.interrupt(emu, pvoice);
    snd_emu10k1_voice_intr_ack(emu, voice);
    } else {
    snd_emu10k1_voice_intr_disable(emu, voice);
    }
    }
    val >>= 1;
    pvoice++;
    }
    val = snd_emu10k1_ptr_read(emu, HLIPL, 0);
    pvoice = emu.voices;
    for (voice = 0; voice <= voice_max; voice++) {
    if (voice == 0x20)
    val = snd_emu10k1_ptr_read(emu, HLIPH, 0);
    if (val & 1) {
    if (pvoice.use && pvoice.interrupt != core::ptr::null_mut()) {
    pvoice.interrupt(emu, pvoice);
    snd_emu10k1_voice_half_loop_intr_ack(emu, voice);
    } else {
    snd_emu10k1_voice_half_loop_intr_disable(emu, voice);
    }
    }
    val >>= 1;
    pvoice++;
    }
    status &= ~(IPR_CHANNELLOOP | IPR_CHANNELNUMBERMASK);
    }
    if (status & (IPR_ADCBUFFULL|IPR_ADCBUFHALFFULL)) {
    if (emu.capture_interrupt)
    emu.capture_interrupt(emu, status);
    else
    snd_emu10k1_intr_disable(emu, INTE_ADCBUFENABLE);
    status &= ~(IPR_ADCBUFFULL|IPR_ADCBUFHALFFULL);
    }
    if (status & (IPR_MICBUFFULL|IPR_MICBUFHALFFULL)) {
    if (emu.capture_mic_interrupt)
    emu.capture_mic_interrupt(emu, status);
    else
    snd_emu10k1_intr_disable(emu, INTE_MICBUFENABLE);
    status &= ~(IPR_MICBUFFULL|IPR_MICBUFHALFFULL);
    }
    if (status & (IPR_EFXBUFFULL|IPR_EFXBUFHALFFULL)) {
    if (emu.capture_efx_interrupt)
    emu.capture_efx_interrupt(emu, status);
    else
    snd_emu10k1_intr_disable(emu, INTE_EFXBUFENABLE);
    status &= ~(IPR_EFXBUFFULL|IPR_EFXBUFHALFFULL);
    }
    if (status & (IPR_MIDITRANSBUFEMPTY|IPR_MIDIRECVBUFEMPTY)) {
    if (emu.midi.interrupt)
    emu.midi.interrupt(emu, status);
    else
    snd_emu10k1_intr_disable(emu, INTE_MIDITXENABLE|INTE_MIDIRXENABLE);
    status &= ~(IPR_MIDITRANSBUFEMPTY|IPR_MIDIRECVBUFEMPTY);
    }
    if (status & (IPR_A_MIDITRANSBUFEMPTY2|IPR_A_MIDIRECVBUFEMPTY2)) {
    if (emu.midi2.interrupt)
    emu.midi2.interrupt(emu, status);
    else
    snd_emu10k1_intr_disable(emu, INTE_A_MIDITXENABLE2|INTE_A_MIDIRXENABLE2);
    status &= ~(IPR_A_MIDITRANSBUFEMPTY2|IPR_A_MIDIRECVBUFEMPTY2);
    }
    if (status & IPR_INTERVALTIMER) {
    if (emu.timer)
    snd_timer_interrupt(emu.timer, emu.timer.sticks);
    else
    snd_emu10k1_intr_disable(emu, INTE_INTERVALTIMERENB);
    status &= ~IPR_INTERVALTIMER;
    }
    if (status & (IPR_GPSPDIFSTATUSCHANGE|IPR_CDROMSTATUSCHANGE)) {
    if (emu.spdif_interrupt)
    emu.spdif_interrupt(emu, status);
    else
    snd_emu10k1_intr_disable(emu, INTE_GPSPDIFENABLE|INTE_CDSPDIFENABLE);
    status &= ~(IPR_GPSPDIFSTATUSCHANGE|IPR_CDROMSTATUSCHANGE);
    }
    if (status & IPR_FXDSP) {
    if (emu.dsp_interrupt)
    emu.dsp_interrupt(emu);
    else
    snd_emu10k1_intr_disable(emu, INTE_FXDSPENABLE);
    status &= ~IPR_FXDSP;
    }
    if (status & IPR_P16V) {
    if (emu.p16v_interrupt)
    emu.p16v_interrupt(emu);
    else
    outl(0, emu.port + INTE2);
    status &= ~IPR_P16V;
    }
    if (status & IPR_A_GPIO) {
    if (emu.gpio_interrupt)
    emu.gpio_interrupt(emu);
    else
    snd_emu10k1_intr_disable(emu, INTE_A_GPIOENABLE);
    status &= ~IPR_A_GPIO;
    }
    if (status) {
    dev_err(emu.card.dev,
    "unhandled interrupt: 0x%08x\n", status);
    }
    outl(orig_status, emu.port + IPR); /* ack all */
    }
    return IRQ_RETVAL(handled);
    }
