//! Automatically rewritten from C to Rust
//! Source: drivers/input/ff-memless.c
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
// Force feedback support for memoryless devices
//
// Copyright (c) 2006 Anssi Hannula <anssi.hannula@gmail.com>
// Copyright (c) 2006 Dmitry Torokhov <dtor@mail.ru>
//
// #define DEBUG

    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Anssi Hannula <anssi.hannula@gmail.com>");
    MODULE_DESCRIPTION("Force feedback support for memoryless devices");
// Number of effects handled with memoryless devices
pub const FF_MEMLESS_EFFECTS: c_int = 16;
// Envelope update interval in ms
pub const FF_ENVELOPE_INTERVAL: c_int = 50;
pub const FF_EFFECT_STARTED: c_int = 0;
pub const FF_EFFECT_PLAYING: c_int = 1;
pub const FF_EFFECT_ABORTING: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ml_effect_state {
    pub effect: *mut ff_effect,
    pub /: *mut *mut unsigned long flags; / effect state (STARTED, PLAYING, etc),
    pub /: *mut *mut int count; / loop count of the effect,
    pub /: *mut *mut unsigned long play_at; / start time,
    pub /: *mut *mut unsigned long stop_at; / stop time,
    pub /: *mut *mut unsigned long adj_at; / last time the effect was sent,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ml_device {
    pub private: *mut c_void,
    pub states: [ml_effect_state; FF_MEMLESS_EFFECTS],
    pub gain: c_int,
    pub timer: timer_list,
    pub dev: *mut input_dev,
    int (*play_effect)(struct input_dev *dev, void *data,
    pub effect): *mut ff_effect,
}

    static const struct ff_envelope *get_envelope(const struct ff_effect *effect)
    {
    static const struct ff_envelope empty_envelope;
    switch (effect.type) {
    case FF_PERIODIC:
    return &effect.u.periodic.envelope;
    case FF_CONSTANT:
    return &effect.u.constant.envelope;
    default:
    return &empty_envelope;
    }
    }
//
// Check for the next time envelope requires an update on memoryless devices
//
#[no_mangle]
unsafe extern "C" fn calculate_next_time(state: *mut ml_effect_state) -> c_ulong {
    static unsigned long calculate_next_time(struct ml_effect_state *state)
    {
    const struct ff_envelope *envelope = get_envelope(state.effect);
    unsigned long attack_stop, fade_start, next_fade;
    if (envelope.attack_length) {
    attack_stop = state.play_at +
    msecs_to_jiffies(envelope.attack_length);
    if (time_before(state.adj_at, attack_stop))
    return state.adj_at +
    msecs_to_jiffies(FF_ENVELOPE_INTERVAL);
    }
    if (state.effect.replay.length) {
    if (envelope.fade_length) {
// check when fading should start
    fade_start = state.stop_at -
    msecs_to_jiffies(envelope.fade_length);
    if (time_before(state.adj_at, fade_start))
    return fade_start;
// already fading, advance to next checkpoint
    next_fade = state.adj_at +
    msecs_to_jiffies(FF_ENVELOPE_INTERVAL);
    if (time_before(next_fade, state.stop_at))
    return next_fade;
    }
    return state.stop_at;
    }
    return state.play_at;
    }
#[no_mangle]
unsafe extern "C" fn ml_schedule_timer(ml: *mut ml_device) {
    static void ml_schedule_timer(struct ml_device *ml)
    {
    struct ml_effect_state *state;
    let mut now: c_ulong = jiffies;
    let mut earliest: c_ulong = 0;
    unsigned long next_at;
    let mut events: c_int = 0;
    int i;
    pr_debug("calculating next timer\n");
    for (i = 0; i < FF_MEMLESS_EFFECTS; i++) {
    state = &ml.states[i];
    if (!test_bit(FF_EFFECT_STARTED, &state.flags))
    continue;
    if (test_bit(FF_EFFECT_PLAYING, &state.flags))
    next_at = calculate_next_time(state);
    else
    next_at = state.play_at;
    if (time_before_eq(now, next_at) &&
    (++events == 1 || time_before(next_at, earliest)))
    earliest = next_at;
    }
    if (!events) {
    pr_debug("no actions\n");
    timer_delete(&ml.timer);
    } else {
    pr_debug("timer set\n");
    mod_timer(&ml.timer, earliest);
    }
    }
//
// Apply an envelope to a value
//
    static int apply_envelope(struct ml_effect_state *state, int value,
    struct ff_envelope *envelope)
    {
    struct ff_effect *effect = state.effect;
    let mut now: c_ulong = jiffies;
    int time_from_level;
    int time_of_envelope;
    int envelope_level;
    int difference;
    if (envelope.attack_length &&
    time_before(now,
    state.play_at + msecs_to_jiffies(envelope.attack_length))) {
    pr_debug("value = 0x%x, attack_level = 0x%x\n",
    value, envelope.attack_level);
    time_from_level = jiffies_to_msecs(now - state.play_at);
    time_of_envelope = envelope.attack_length;
    envelope_level = min_t(u16, envelope.attack_level, 0x7fff);
    } else if (envelope.fade_length && effect.replay.length &&
    time_after(now,
    state.stop_at - msecs_to_jiffies(envelope.fade_length)) &&
    time_before(now, state.stop_at)) {
    time_from_level = jiffies_to_msecs(state.stop_at - now);
    time_of_envelope = envelope.fade_length;
    envelope_level = min_t(u16, envelope.fade_level, 0x7fff);
    } else
    return value;
    difference = abs(value) - envelope_level;
    pr_debug("difference = %d\n", difference);
    pr_debug("time_from_level = 0x%x\n", time_from_level);
    pr_debug("time_of_envelope = 0x%x\n", time_of_envelope);
    difference = difference * time_from_level / time_of_envelope;
    pr_debug("difference = %d\n", difference);
    return value < 0 ?
    -(difference + envelope_level) : (difference + envelope_level);
    }
//
// Return the type the effect has to be converted into (memless devices)
//
#[no_mangle]
unsafe extern "C" fn get_compatible_type(ff: *mut ff_device, effect_type: c_int) -> c_int {
    static int get_compatible_type(struct ff_device *ff, int effect_type)
    {
    if (test_bit(effect_type, ff.ffbit))
    return effect_type;
    if (effect_type == FF_PERIODIC && test_bit(FF_RUMBLE, ff.ffbit))
    return FF_RUMBLE;
    pr_err("invalid type in get_compatible_type()\n");
    return 0;
    }
//
// Only left/right direction should be used (under/over 0x8000) for
// forward/reverse motor direction (to keep calculation fast & simple).
//
    static u16 ml_calculate_direction(u16 direction, u16 force,
    u16 new_direction, u16 new_force)
    {
    if (!force)
    return new_direction;
    if (!new_force)
    return direction;
    return (((u32)(direction >> 1) * force +
    (new_direction >> 1) * new_force) /
    (force + new_force)) << 1;
    }
pub const FRAC_N: c_int = 8;
#[no_mangle]
pub unsafe extern "C" fn fixp_new16(a: i16) -> i16 {
    static inline s16 fixp_new16(s16 a)
    {
    return ((s32)a) >> (16 - FRAC_N);
    }
#[no_mangle]
pub unsafe extern "C" fn fixp_mult(a: i16, b: i16) -> i16 {
    static inline s16 fixp_mult(s16 a, s16 b)
    {
    a = ((s32)a * 0x100) / 0x7fff;
    return ((s32)(a * b)) >> FRAC_N;
    }
//
// Combine two effects and apply gain.
//
    static void ml_combine_effects(struct ff_effect *effect,
    struct ml_effect_state *state,
    int gain)
    {
    struct ff_effect *new = state.effect;
    unsigned int strong, weak, i;
    int x, y;
    s16 level;
    switch (new.type) {
    case FF_CONSTANT:
    i = new.direction * 360 / 0xffff;
    level = fixp_new16(apply_envelope(state,
    new.u.constant.level,
    &new.u.constant.envelope));
    x = fixp_mult(fixp_sin16(i), level) * gain / 0xffff;
    y = fixp_mult(-fixp_cos16(i), level) * gain / 0xffff;
//
// here we abuse ff_ramp to hold x and y of constant force
// If in future any driver wants something else than x and y
// in s8, this should be changed to something more generic
//
    effect.u.ramp.start_level =
    clamp_val(effect.u.ramp.start_level + x, -0x80, 0x7f);
    effect.u.ramp.end_level =
    clamp_val(effect.u.ramp.end_level + y, -0x80, 0x7f);
    break;
    case FF_RUMBLE:
    strong = (u32)new.u.rumble.strong_magnitude * gain / 0xffff;
    weak = (u32)new.u.rumble.weak_magnitude * gain / 0xffff;
    if (effect.u.rumble.strong_magnitude + strong)
    effect.direction = ml_calculate_direction(
    effect.direction,
    effect.u.rumble.strong_magnitude,
    new.direction, strong);
#[no_mangle]
pub unsafe extern "C" fn if(weak: effect->u.rumble.weak_magnitude +) -> else {
    else if (effect.u.rumble.weak_magnitude + weak)
    effect.direction = ml_calculate_direction(
    effect.direction,
    effect.u.rumble.weak_magnitude,
    new.direction, weak);
    else
    effect.direction = 0;
    effect.u.rumble.strong_magnitude =
    min(strong + effect.u.rumble.strong_magnitude,
    0xffffU);
    effect.u.rumble.weak_magnitude =
    min(weak + effect.u.rumble.weak_magnitude, 0xffffU);
    break;
    case FF_PERIODIC:
    i = apply_envelope(state, abs(new.u.periodic.magnitude),
    &new.u.periodic.envelope);
// here we also scale it 0x7fff => 0xffff
    i = i * gain / 0x7fff;
    if (effect.u.rumble.strong_magnitude + i)
    effect.direction = ml_calculate_direction(
    effect.direction,
    effect.u.rumble.strong_magnitude,
    new.direction, i);
    else
    effect.direction = 0;
    effect.u.rumble.strong_magnitude =
    min(i + effect.u.rumble.strong_magnitude, 0xffffU);
    effect.u.rumble.weak_magnitude =
    min(i + effect.u.rumble.weak_magnitude, 0xffffU);
    break;
    default:
    pr_err("invalid type in ml_combine_effects()\n");
    break;
    }
    }
//
// Because memoryless devices have only one effect per effect type active
// at one time we have to combine multiple effects into one
//
    static int ml_get_combo_effect(struct ml_device *ml,
    unsigned long *effect_handled,
    struct ff_effect *combo_effect)
    {
    struct ff_effect *effect;
    struct ml_effect_state *state;
    int effect_type;
    int i;
    memset(combo_effect, 0, sizeof(struct ff_effect));
    for (i = 0; i < FF_MEMLESS_EFFECTS; i++) {
    if (__test_and_set_bit(i, effect_handled))
    continue;
    state = &ml.states[i];
    effect = state.effect;
    if (!test_bit(FF_EFFECT_STARTED, &state.flags))
    continue;
    if (time_before(jiffies, state.play_at))
    continue;
//
// here we have started effects that are either
// currently playing (and may need be aborted)
// or need to start playing.
//
    effect_type = get_compatible_type(ml.dev.ff, effect.type);
    if (combo_effect.type != effect_type) {
    if (combo_effect.type != 0) {
    __clear_bit(i, effect_handled);
    continue;
    }
    combo_effect.type = effect_type;
    }
    if (__test_and_clear_bit(FF_EFFECT_ABORTING, &state.flags)) {
    __clear_bit(FF_EFFECT_PLAYING, &state.flags);
    __clear_bit(FF_EFFECT_STARTED, &state.flags);
    } else if (effect.replay.length &&
    time_after_eq(jiffies, state.stop_at)) {
    __clear_bit(FF_EFFECT_PLAYING, &state.flags);
    if (--state.count <= 0) {
    __clear_bit(FF_EFFECT_STARTED, &state.flags);
    } else {
    state.play_at = jiffies +
    msecs_to_jiffies(effect.replay.delay);
    state.stop_at = state.play_at +
    msecs_to_jiffies(effect.replay.length);
    }
    } else {
    __set_bit(FF_EFFECT_PLAYING, &state.flags);
    state.adj_at = jiffies;
    ml_combine_effects(combo_effect, state, ml.gain);
    }
    }
    return combo_effect.type != 0;
    }
#[no_mangle]
unsafe extern "C" fn ml_play_effects(ml: *mut ml_device) {
    static void ml_play_effects(struct ml_device *ml)
    {
    struct ff_effect effect;
    DECLARE_BITMAP(handled_bm, FF_MEMLESS_EFFECTS);
    memset(handled_bm, 0, sizeof(handled_bm));
    while (ml_get_combo_effect(ml, handled_bm, &effect))
    ml.play_effect(ml.dev, ml.private, &effect);
    ml_schedule_timer(ml);
    }
#[no_mangle]
unsafe extern "C" fn ml_effect_timer(t: *mut timer_list) {
    static void ml_effect_timer(struct timer_list *t)
    {
    struct ml_device *ml = timer_container_of(ml, t, timer);
    struct input_dev *dev = ml.dev;
    pr_debug("timer: updating effects\n");
    guard(spinlock_irqsave)(&dev.event_lock);
    ml_play_effects(ml);
    }
//
// Sets requested gain for FF effects. Called with dev->event_lock held.
//
#[no_mangle]
unsafe extern "C" fn ml_ff_set_gain(dev: *mut input_dev, gain: u16) {
    static void ml_ff_set_gain(struct input_dev *dev, u16 gain)
    {
    struct ml_device *ml = dev.ff.private;
    int i;
    ml.gain = gain;
    for (i = 0; i < FF_MEMLESS_EFFECTS; i++)
    __clear_bit(FF_EFFECT_PLAYING, &ml.states[i].flags);
    ml_play_effects(ml);
    }
//
// Start/stop specified FF effect. Called with dev->event_lock held.
//
#[no_mangle]
unsafe extern "C" fn ml_ff_playback(dev: *mut input_dev, effect_id: c_int, value: c_int) -> c_int {
    static int ml_ff_playback(struct input_dev *dev, int effect_id, int value)
    {
    struct ml_device *ml = dev.ff.private;
    struct ml_effect_state *state = &ml.states[effect_id];
    if (value > 0) {
    pr_debug("initiated play\n");
    __set_bit(FF_EFFECT_STARTED, &state.flags);
    state.count = value;
    state.play_at = jiffies +
    msecs_to_jiffies(state.effect.replay.delay);
    state.stop_at = state.play_at +
    msecs_to_jiffies(state.effect.replay.length);
    state.adj_at = state.play_at;
    } else {
    pr_debug("initiated stop\n");
    if (test_bit(FF_EFFECT_PLAYING, &state.flags))
    __set_bit(FF_EFFECT_ABORTING, &state.flags);
    else
    __clear_bit(FF_EFFECT_STARTED, &state.flags);
    }
    ml_play_effects(ml);
    return 0;
    }
    static int ml_ff_upload(struct input_dev *dev,
    struct ff_effect *effect, struct ff_effect *old)
    {
    struct ml_device *ml = dev.ff.private;
    struct ml_effect_state *state = &ml.states[effect.id];
    guard(spinlock_irq)(&dev.event_lock);
    if (test_bit(FF_EFFECT_STARTED, &state.flags)) {
    __clear_bit(FF_EFFECT_PLAYING, &state.flags);
    state.play_at = jiffies +
    msecs_to_jiffies(state.effect.replay.delay);
    state.stop_at = state.play_at +
    msecs_to_jiffies(state.effect.replay.length);
    state.adj_at = state.play_at;
    ml_schedule_timer(ml);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ml_ff_destroy(ff: *mut ff_device) {
    static void ml_ff_destroy(struct ff_device *ff)
    {
    struct ml_device *ml = ff.private;
//
// The timer is normally shut down in ml_ff_stop() when the device
// is unregistered. However, we still shut it down here as a safety
// net and for cases where the device was never registered (e.g.
// error paths during probe).
//
    timer_shutdown_sync(&ml.timer);
    kfree(ml.private);
    }
#[no_mangle]
unsafe extern "C" fn ml_ff_stop(ff: *mut ff_device) {
    static void ml_ff_stop(struct ff_device *ff)
    {
    struct ml_device *ml = ff.private;
//
// Even though we stop all playing effects when tearing down an
// input device (by the way of evdev calling input_flush_device()
// that calls into input_ff_flush() that stops and erases all
// effects), we do not actually shutdown the timer, and therefore
// we should do it here to prevent it firing after the input
// device is unregistered and its associated resources are freed.
//
    timer_shutdown_sync(&ml.timer);
    }
//
// input_ff_create_memless() - create memoryless force-feedback device
// @dev: input device supporting force-feedback
// @data: driver-specific data to be passed into @play_effect
// @play_effect: driver-specific method for playing FF effect
//
    int input_ff_create_memless(struct input_dev *dev, void *data,
    int (*play_effect)(struct input_dev *, void *, struct ff_effect *))
    {
    struct ff_device *ff;
    int error;
    int i;
    struct ml_device *ml __free(kfree) = kzalloc_obj(*ml);
    if (!ml)
    return -ENOMEM;
    ml.dev = dev;
    ml.private = data;
    ml.play_effect = play_effect;
    ml.gain = 0xffff;
    timer_setup(&ml.timer, ml_effect_timer, 0);
    set_bit(FF_GAIN, dev.ffbit);
    error = input_ff_create(dev, FF_MEMLESS_EFFECTS);
    if (error)
    return error;
    ff = dev.ff;
    ff.upload = ml_ff_upload;
    ff.playback = ml_ff_playback;
    ff.set_gain = ml_ff_set_gain;
    ff.destroy = ml_ff_destroy;
    ff.stop = ml_ff_stop;
// we can emulate periodic effects with RUMBLE
    if (test_bit(FF_RUMBLE, ff.ffbit)) {
    set_bit(FF_PERIODIC, dev.ffbit);
    set_bit(FF_SINE, dev.ffbit);
    set_bit(FF_TRIANGLE, dev.ffbit);
    set_bit(FF_SQUARE, dev.ffbit);
    }
    for (i = 0; i < FF_MEMLESS_EFFECTS; i++)
    ml.states[i].effect = &ff.effects[i];
    ff.private = no_free_ptr(ml);
    return 0;
    }
    EXPORT_SYMBOL_GPL(input_ff_create_memless);
