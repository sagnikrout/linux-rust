//! Automatically rewritten from C to Rust
//! Source: drivers/media/rc/ir-rcmm-decoder.c
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


// SPDX-License-Identifier: GPL-2.0+
// ir-rcmm-decoder.c - A decoder for the RCMM IR protocol
//
// Copyright (C) 2018 by Patrick Lerda <patrick9876@free.fr>

    enum rcmm_state {
    STATE_INACTIVE,
    STATE_LOW,
    STATE_BUMP,
    STATE_VALUE,
    STATE_FINISHED,
    };
#[no_mangle]
unsafe extern "C" fn rcmm_mode(data: *const rcmm_dec) -> bool {
    static bool rcmm_mode(const struct rcmm_dec *data)
    {
    return !((0x000c0000 & data.bits) == 0x000c0000);
    }
#[no_mangle]
unsafe extern "C" fn rcmm_miscmode(dev: *mut rc_dev, data: *mut rcmm_dec) -> c_int {
    static int rcmm_miscmode(struct rc_dev *dev, struct rcmm_dec *data)
    {
    switch (data.count) {
    case 24:
    if (dev.enabled_protocols & RC_PROTO_BIT_RCMM24) {
    rc_keydown(dev, RC_PROTO_RCMM24, data.bits, 0);
    data.state = STATE_INACTIVE;
    return 0;
    }
    return -1;
    case 12:
    if (dev.enabled_protocols & RC_PROTO_BIT_RCMM12) {
    rc_keydown(dev, RC_PROTO_RCMM12, data.bits, 0);
    data.state = STATE_INACTIVE;
    return 0;
    }
    return -1;
    }
    return -1;
    }
//
// ir_rcmm_decode() - Decode one RCMM pulse or space
// @dev:	the struct rc_dev descriptor of the device
// @ev:		the struct ir_raw_event descriptor of the pulse/space
//
// This function returns -EINVAL if the pulse violates the state machine
//
#[no_mangle]
unsafe extern "C" fn ir_rcmm_decode(dev: *mut rc_dev, ev: ir_raw_event) -> c_int {
    static int ir_rcmm_decode(struct rc_dev *dev, struct ir_raw_event ev)
    {
    struct rcmm_dec *data = &dev.raw.rcmm;
    u32 scancode;
    u8 toggle;
    int value;
    if (!(dev.enabled_protocols & (RC_PROTO_BIT_RCMM32 |
    RC_PROTO_BIT_RCMM24 |
    RC_PROTO_BIT_RCMM12)))
    return 0;
    if (!is_timing_event(ev)) {
    if (ev.overflow)
    data.state = STATE_INACTIVE;
    return 0;
    }
    switch (data.state) {
    case STATE_INACTIVE:
    if (!ev.pulse)
    break;
    if (!eq_margin(ev.duration, RCMM_PREFIX_PULSE, RCMM_UNIT))
    break;
    data.state = STATE_LOW;
    data.count = 0;
    data.bits  = 0;
    return 0;
    case STATE_LOW:
    if (ev.pulse)
    break;
    if (!eq_margin(ev.duration, RCMM_PULSE_0, RCMM_UNIT))
    break;
    data.state = STATE_BUMP;
    return 0;
    case STATE_BUMP:
    if (!ev.pulse)
    break;
    if (!eq_margin(ev.duration, RCMM_UNIT, RCMM_UNIT / 2))
    break;
    data.state = STATE_VALUE;
    return 0;
    case STATE_VALUE:
    if (ev.pulse)
    break;
    if (eq_margin(ev.duration, RCMM_PULSE_0, RCMM_UNIT / 2))
    value = 0;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: eq_margin(ev.duration, _arg: RCMM_PULSE_1, 2): RCMM_UNIT /) -> else {
    else if (eq_margin(ev.duration, RCMM_PULSE_1, RCMM_UNIT / 2))
    value = 1;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: eq_margin(ev.duration, _arg: RCMM_PULSE_2, 2): RCMM_UNIT /) -> else {
    else if (eq_margin(ev.duration, RCMM_PULSE_2, RCMM_UNIT / 2))
    value = 2;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: eq_margin(ev.duration, _arg: RCMM_PULSE_3, 2): RCMM_UNIT /) -> else {
    else if (eq_margin(ev.duration, RCMM_PULSE_3, RCMM_UNIT / 2))
    value = 3;
    else
    value = -1;
    if (value == -1) {
    if (!rcmm_miscmode(dev, data))
    return 0;
    break;
    }
    data.bits <<= 2;
    data.bits |= value;
    data.count += 2;
    if (data.count < 32)
    data.state = STATE_BUMP;
    else
    data.state = STATE_FINISHED;
    return 0;
    case STATE_FINISHED:
    if (!ev.pulse)
    break;
    if (!eq_margin(ev.duration, RCMM_UNIT, RCMM_UNIT / 2))
    break;
    if (rcmm_mode(data)) {
    toggle = !!(0x8000 & data.bits);
    scancode = data.bits & ~0x8000;
    } else {
    toggle = 0;
    scancode = data.bits;
    }
    if (dev.enabled_protocols & RC_PROTO_BIT_RCMM32) {
    rc_keydown(dev, RC_PROTO_RCMM32, scancode, toggle);
    data.state = STATE_INACTIVE;
    return 0;
    }
    break;
    }
    dev_dbg(&dev.dev, "RC-MM decode failed at count %d state %d (%uus %s)\n",
    data.count, data.state, ev.duration, TO_STR(ev.pulse));
    data.state = STATE_INACTIVE;
    return -EINVAL;
    }
    static const int rcmmspace[] = {
    RCMM_PULSE_0,
    RCMM_PULSE_1,
    RCMM_PULSE_2,
    RCMM_PULSE_3,
    };
    static int ir_rcmm_rawencoder(struct ir_raw_event **ev, unsigned int max,
    unsigned int n, u32 data)
    {
    int i;
    int ret;
    ret = ir_raw_gen_pulse_space(ev, &max, RCMM_PREFIX_PULSE, RCMM_PULSE_0);
    if (ret)
    return ret;
    for (i = n - 2; i >= 0; i -= 2) {
    let mut space: c_uint = rcmmspace[(data >> i) & 3];
    ret = ir_raw_gen_pulse_space(ev, &max, RCMM_UNIT, space);
    if (ret)
    return ret;
    }
    return ir_raw_gen_pulse_space(ev, &max, RCMM_UNIT, RCMM_PULSE_3 * 2);
    }
    static int ir_rcmm_encode(enum rc_proto protocol, u32 scancode,
    struct ir_raw_event *events, unsigned int max)
    {
    struct ir_raw_event *e = events;
    int ret;
    switch (protocol) {
    case RC_PROTO_RCMM32:
    ret = ir_rcmm_rawencoder(&e, max, 32, scancode);
    break;
    case RC_PROTO_RCMM24:
    ret = ir_rcmm_rawencoder(&e, max, 24, scancode);
    break;
    case RC_PROTO_RCMM12:
    ret = ir_rcmm_rawencoder(&e, max, 12, scancode);
    break;
    default:
    ret = -EINVAL;
    }
    if (ret < 0)
    return ret;
    return e - events;
    }
    static struct ir_raw_handler rcmm_handler = {
    .protocols	= RC_PROTO_BIT_RCMM32 |
    RC_PROTO_BIT_RCMM24 |
    RC_PROTO_BIT_RCMM12,
    .decode		= ir_rcmm_decode,
    .encode         = ir_rcmm_encode,
    .carrier        = 36000,
    .min_timeout	= RCMM_PULSE_3 + RCMM_UNIT,
    };
#[no_mangle]
unsafe extern "C" fn ir_rcmm_decode_init() -> int __init {
    static int __init ir_rcmm_decode_init(void)
    {
    ir_raw_handler_register(&rcmm_handler);
    pr_info("IR RCMM protocol handler initialized\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ir_rcmm_decode_exit() -> void __exit {
    static void __exit ir_rcmm_decode_exit(void)
    {
    ir_raw_handler_unregister(&rcmm_handler);
    }
    module_init(ir_rcmm_decode_init);
    module_exit(ir_rcmm_decode_exit);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Patrick Lerda");
    MODULE_DESCRIPTION("RCMM IR protocol decoder");
