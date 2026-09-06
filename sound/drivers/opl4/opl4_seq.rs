//! Automatically rewritten from C to Rust
//! Source: sound/drivers/opl4/opl4_seq.c
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
// OPL4 sequencer functions
//
// Copyright (c) 2003 by Clemens Ladisch <clemens@ladisch.de>
// All rights reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met:
// 1. Redistributions of source code must retain the above copyright
// notice, this list of conditions, and the following disclaimer,
// without modification.
// 2. The name of the author may not be used to endorse or promote products
// derived from this software without specific prior written permission.
//
// Alternatively, this software may be distributed and/or modified under the
// terms of the GNU General Public License as published by the Free Software
// Foundation; either version 2 of the License, or (at your option) any later
// version.
//
// THIS SOFTWARE IS PROVIDED BY THE AUTHOR AND CONTRIBUTORS ``AS IS'' AND
// ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
// IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
// ARE DISCLAIMED. IN NO EVENT SHALL THE AUTHOR OR CONTRIBUTORS BE LIABLE FOR
// ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
// DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS
// OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
// HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
// LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY
// OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF
// SUCH DAMAGE.
//

    MODULE_AUTHOR("Clemens Ladisch <clemens@ladisch.de>");
    MODULE_DESCRIPTION("OPL4 wavetable synth driver");
    MODULE_LICENSE("Dual BSD/GPL");
    let mut volume_boost: c_int = 8;
    module_param(volume_boost, int, 0644);
    MODULE_PARM_DESC(volume_boost, "Additional volume for OPL4 wavetable sounds.");
#[no_mangle]
unsafe extern "C" fn snd_opl4_seq_use_inc(opl4: *mut snd_opl4) -> c_int {
    static int snd_opl4_seq_use_inc(struct snd_opl4 *opl4)
    {
    if (!try_module_get(opl4.card.module))
    return -EFAULT;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn snd_opl4_seq_use_dec(opl4: *mut snd_opl4) {
    static void snd_opl4_seq_use_dec(struct snd_opl4 *opl4)
    {
    module_put(opl4.card.module);
    }
#[no_mangle]
unsafe extern "C" fn snd_opl4_seq_use(private_data: *mut c_void, info: *mut snd_seq_port_subscribe) -> c_int {
    static int snd_opl4_seq_use(void *private_data, struct snd_seq_port_subscribe *info)
    {
    struct snd_opl4 *opl4 = private_data;
    int err;
    scoped_guard(mutex, &opl4.access_mutex) {
    if (opl4.used)
    return -EBUSY;
    opl4.used++;
    if (info.sender.client != SNDRV_SEQ_CLIENT_SYSTEM) {
    err = snd_opl4_seq_use_inc(opl4);
    if (err < 0)
    return err;
    }
    }
    snd_opl4_synth_reset(opl4);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn snd_opl4_seq_unuse(private_data: *mut c_void, info: *mut snd_seq_port_subscribe) -> c_int {
    static int snd_opl4_seq_unuse(void *private_data, struct snd_seq_port_subscribe *info)
    {
    struct snd_opl4 *opl4 = private_data;
    snd_opl4_synth_shutdown(opl4);
    scoped_guard(mutex, &opl4.access_mutex) {
    opl4.used--;
    }
    if (info.sender.client != SNDRV_SEQ_CLIENT_SYSTEM)
    snd_opl4_seq_use_dec(opl4);
    return 0;
    }
    static const struct snd_midi_op opl4_ops = {
    .note_on =		snd_opl4_note_on,
    .note_off =		snd_opl4_note_off,
    .note_terminate =	snd_opl4_terminate_note,
    .control =		snd_opl4_control,
    .sysex =		snd_opl4_sysex,
    };
    static int snd_opl4_seq_event_input(struct snd_seq_event *ev, int direct,
    void *private_data, int atomic, int hop)
    {
    struct snd_opl4 *opl4 = private_data;
    snd_midi_process_event(&opl4_ops, ev, opl4.chset);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn snd_opl4_seq_free_port(private_data: *mut c_void) {
    static void snd_opl4_seq_free_port(void *private_data)
    {
    struct snd_opl4 *opl4 = private_data;
    snd_midi_channel_free_set(opl4.chset);
    }
#[no_mangle]
unsafe extern "C" fn snd_opl4_seq_probe(dev: *mut snd_seq_device) -> c_int {
    static int snd_opl4_seq_probe(struct snd_seq_device *dev)
    {
    struct snd_opl4 *opl4;
    int client;
    struct snd_seq_port_callback pcallbacks;
    opl4 = *(struct snd_opl4 **)SNDRV_SEQ_DEVICE_ARGPTR(dev);
    if (!opl4)
    return -EINVAL;
    if (snd_yrw801_detect(opl4) < 0)
    return -ENODEV;
    opl4.chset = snd_midi_channel_alloc_set(16);
    if (!opl4.chset)
    return -ENOMEM;
    opl4.chset.private_data = opl4;
// allocate new client
    client = snd_seq_create_kernel_client(opl4.card, opl4.seq_dev_num,
    "OPL4 Wavetable");
    if (client < 0) {
    snd_midi_channel_free_set(opl4.chset);
    return client;
    }
    opl4.seq_client = client;
    opl4.chset.client = client;
// create new port
    memset(&pcallbacks, 0, sizeof(pcallbacks));
    pcallbacks.owner = THIS_MODULE;
    pcallbacks.use = snd_opl4_seq_use;
    pcallbacks.unuse = snd_opl4_seq_unuse;
    pcallbacks.event_input = snd_opl4_seq_event_input;
    pcallbacks.private_free = snd_opl4_seq_free_port;
    pcallbacks.private_data = opl4;
    opl4.chset.port = snd_seq_event_port_attach(client, &pcallbacks,
    SNDRV_SEQ_PORT_CAP_WRITE |
    SNDRV_SEQ_PORT_CAP_SUBS_WRITE,
    SNDRV_SEQ_PORT_TYPE_MIDI_GENERIC |
    SNDRV_SEQ_PORT_TYPE_MIDI_GM |
    SNDRV_SEQ_PORT_TYPE_HARDWARE |
    SNDRV_SEQ_PORT_TYPE_SYNTHESIZER,
    16, 24,
    "OPL4 Wavetable Port");
    if (opl4.chset.port < 0) {
    let mut err: c_int = opl4.chset.port;
    snd_midi_channel_free_set(opl4.chset);
    snd_seq_delete_kernel_client(client);
    opl4.seq_client = -1;
    return err;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn snd_opl4_seq_remove(dev: *mut snd_seq_device) {
    static void snd_opl4_seq_remove(struct snd_seq_device *dev)
    {
    struct snd_opl4 *opl4;
    opl4 = *(struct snd_opl4 **)SNDRV_SEQ_DEVICE_ARGPTR(dev);
    if (!opl4)
    return;
    if (opl4.seq_client >= 0) {
    snd_seq_delete_kernel_client(opl4.seq_client);
    opl4.seq_client = -1;
    }
    }
    static struct snd_seq_driver opl4_seq_driver = {
    .probe = snd_opl4_seq_probe,
    .remove = snd_opl4_seq_remove,
    .driver = {
    .name = KBUILD_MODNAME,
    },
    .id = SNDRV_SEQ_DEV_ID_OPL4,
    .argsize = sizeof(struct snd_opl4 *),
    };
    module_snd_seq_driver(opl4_seq_driver);
