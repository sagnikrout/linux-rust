//! Automatically rewritten from C to Rust
//! Source: sound/core/seq/seq.c
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
// ALSA sequencer main module
// Copyright (c) 1998-1999 by Frank van de Pol <fvdpol@coil.demon.nl>
//

    int seq_client_load[15] = {[0] = SNDRV_SEQ_CLIENT_DUMMY, [1 ... 14] = -1};

    int seq_client_load[15] = {[0 ... 14] = -1};

    let mut seq_default_timer_class: c_int = SNDRV_TIMER_CLASS_GLOBAL;
    let mut seq_default_timer_sclass: c_int = SNDRV_TIMER_SCLASS_NONE;
    let mut seq_default_timer_card: c_int = -1;
    int seq_default_timer_device =

    SNDRV_TIMER_GLOBAL_HRTIMER

    SNDRV_TIMER_GLOBAL_SYSTEM

    ;
    let mut seq_default_timer_subdevice: c_int = 0;
    int seq_default_timer_resolution = 0;	/* Hz */
    MODULE_AUTHOR("Frank van de Pol <fvdpol@coil.demon.nl>, Jaroslav Kysela <perex@perex.cz>");
    MODULE_DESCRIPTION("Advanced Linux Sound Architecture sequencer.");
    MODULE_LICENSE("GPL");
    module_param_array(seq_client_load, int, core::ptr::null_mut(), 0444);
    MODULE_PARM_DESC(seq_client_load, "The numbers of global (system) clients to load through kmod.");
    module_param(seq_default_timer_class, int, 0644);
    MODULE_PARM_DESC(seq_default_timer_class, "The default timer class.");
    module_param(seq_default_timer_sclass, int, 0644);
    MODULE_PARM_DESC(seq_default_timer_sclass, "The default timer slave class.");
    module_param(seq_default_timer_card, int, 0644);
    MODULE_PARM_DESC(seq_default_timer_card, "The default timer card number.");
    module_param(seq_default_timer_device, int, 0644);
    MODULE_PARM_DESC(seq_default_timer_device, "The default timer device number.");
    module_param(seq_default_timer_subdevice, int, 0644);
    MODULE_PARM_DESC(seq_default_timer_subdevice, "The default timer subdevice number.");
    module_param(seq_default_timer_resolution, int, 0644);
    MODULE_PARM_DESC(seq_default_timer_resolution, "The default timer resolution in Hz.");
    MODULE_ALIAS_CHARDEV(CONFIG_SND_MAJOR, SNDRV_MINOR_SEQUENCER);
    MODULE_ALIAS("devname:snd/seq");
//
// INIT PART
//
#[no_mangle]
unsafe extern "C" fn alsa_seq_init() -> int __init {
    static int __init alsa_seq_init(void)
    {
    int err;
    err = client_init_data();
    if (err < 0)
    goto error;
// register sequencer device
    err = snd_sequencer_device_init();
    if (err < 0)
    goto error;
// register proc interface
    err = snd_seq_info_init();
    if (err < 0)
    goto error_device;
// register our internal client
    err = snd_seq_system_client_init();
    if (err < 0)
    goto error_info;
    snd_seq_autoload_init();
    return 0;
    error_info:
    snd_seq_info_done();
    error_device:
    snd_sequencer_device_done();
    error:
    return err;
    }
#[no_mangle]
unsafe extern "C" fn alsa_seq_exit() -> void __exit {
    static void __exit alsa_seq_exit(void)
    {
// unregister our internal client
    snd_seq_system_client_done();
// unregister proc interface
    snd_seq_info_done();
// delete timing queues
    snd_seq_queues_delete();
// unregister sequencer device
    snd_sequencer_device_done();
    snd_seq_autoload_exit();
    }
    module_init(alsa_seq_init)
    module_exit(alsa_seq_exit)
