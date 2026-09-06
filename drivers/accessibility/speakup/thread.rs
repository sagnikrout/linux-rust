//! Automatically rewritten from C to Rust
//! Source: drivers/accessibility/speakup/thread.c
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

    DECLARE_WAIT_QUEUE_HEAD(speakup_event);
    EXPORT_SYMBOL_GPL(speakup_event);
#[no_mangle]
pub unsafe extern "C" fn speakup_thread(data: *mut c_void) -> c_int {
    int speakup_thread(void *data)
    {
    unsigned long flags;
    int should_break;
    struct bleep our_sound;
    our_sound.active = 0;
    our_sound.freq = 0;
    our_sound.jiffies = 0;
    mutex_lock(&spk_mutex);
    while (1) {
    DEFINE_WAIT(wait);
    while (1) {
    spin_lock_irqsave(&speakup_info.spinlock, flags);
    our_sound = spk_unprocessed_sound;
    spk_unprocessed_sound.active = 0;
    prepare_to_wait(&speakup_event, &wait,
    TASK_INTERRUPTIBLE);
    should_break = kthread_should_stop() ||
    our_sound.active ||
    (synth && synth.catch_up && synth.alive &&
    (speakup_info.flushing ||
    !synth_buffer_empty()));
    spin_unlock_irqrestore(&speakup_info.spinlock, flags);
    if (should_break)
    break;
    mutex_unlock(&spk_mutex);
    schedule();
    mutex_lock(&spk_mutex);
    }
    finish_wait(&speakup_event, &wait);
    if (kthread_should_stop())
    break;
    if (our_sound.active)
    kd_mksound(our_sound.freq, our_sound.jiffies);
    if (synth && synth.catch_up && synth.alive) {
//
// It is up to the callee to take the lock, so that it
// can sleep whenever it likes
//
    synth.catch_up(synth);
    }
    speakup_start_ttys();
    }
    mutex_unlock(&spk_mutex);
    return 0;
    }
