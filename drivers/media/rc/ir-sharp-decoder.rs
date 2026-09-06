//! Automatically rewritten from C to Rust
//! Source: drivers/media/rc/ir-sharp-decoder.c
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


// SPDX-License-Identifier: GPL-2.0-only
// ir-sharp-decoder.c - handle Sharp IR Pulse/Space protocol
//
// Copyright (C) 2013-2014 Imagination Technologies Ltd.
//
// Based on NEC decoder:
// Copyright (C) 2010 by Mauro Carvalho Chehab
//

pub const SHARP_NBITS: c_int = 15;

    enum sharp_state {
    STATE_INACTIVE,
    STATE_BIT_PULSE,
    STATE_BIT_SPACE,
    STATE_TRAILER_PULSE,
    STATE_ECHO_SPACE,
    STATE_TRAILER_SPACE,
    };
//
// ir_sharp_decode() - Decode one Sharp pulse or space
// @dev:	the struct rc_dev descriptor of the device
// @ev:		the struct ir_raw_event descriptor of the pulse/space
//
// This function returns -EINVAL if the pulse violates the state machine
//
#[no_mangle]
unsafe extern "C" fn ir_sharp_decode(dev: *mut rc_dev, ev: ir_raw_event) -> c_int {
    static int ir_sharp_decode(struct rc_dev *dev, struct ir_raw_event ev)
    {
    struct sharp_dec *data = &dev.raw.sharp;
    u32 msg, echo, address, command, scancode;
    if (!is_timing_event(ev)) {
    if (ev.overflow)
    data.state = STATE_INACTIVE;
    return 0;
    }
    dev_dbg(&dev.dev, "Sharp decode started at state %d (%uus %s)\n",
    data.state, ev.duration, TO_STR(ev.pulse));
    switch (data.state) {
    case STATE_INACTIVE:
    if (!ev.pulse)
    break;
    if (!eq_margin(ev.duration, SHARP_BIT_PULSE,
    SHARP_BIT_PULSE / 2))
    break;
    data.count = 0;
    data.pulse_len = ev.duration;
    data.state = STATE_BIT_SPACE;
    return 0;
    case STATE_BIT_PULSE:
    if (!ev.pulse)
    break;
    if (!eq_margin(ev.duration, SHARP_BIT_PULSE,
    SHARP_BIT_PULSE / 2))
    break;
    data.pulse_len = ev.duration;
    data.state = STATE_BIT_SPACE;
    return 0;
    case STATE_BIT_SPACE:
    if (ev.pulse)
    break;
    data.bits <<= 1;
    if (eq_margin(data.pulse_len + ev.duration, SHARP_BIT_1_PERIOD,
    SHARP_BIT_PULSE * 2))
    data.bits |= 1;
    else if (!eq_margin(data.pulse_len + ev.duration,
    SHARP_BIT_0_PERIOD, SHARP_BIT_PULSE * 2))
    break;
    data.count++;
    if (data.count == SHARP_NBITS ||
    data.count == SHARP_NBITS * 2)
    data.state = STATE_TRAILER_PULSE;
    else
    data.state = STATE_BIT_PULSE;
    return 0;
    case STATE_TRAILER_PULSE:
    if (!ev.pulse)
    break;
    if (!eq_margin(ev.duration, SHARP_BIT_PULSE,
    SHARP_BIT_PULSE / 2))
    break;
    if (data.count == SHARP_NBITS) {
// exp,chk bits should be 1,0
    if ((data.bits & 0x3) != 0x2 &&
// DENON variant, both chk bits 0
    (data.bits & 0x3) != 0x0)
    break;
    data.state = STATE_ECHO_SPACE;
    } else {
    data.state = STATE_TRAILER_SPACE;
    }
    return 0;
    case STATE_ECHO_SPACE:
    if (ev.pulse)
    break;
    if (!eq_margin(ev.duration, SHARP_ECHO_SPACE,
    SHARP_ECHO_SPACE / 4))
    break;
    data.state = STATE_BIT_PULSE;
    return 0;
    case STATE_TRAILER_SPACE:
    if (ev.pulse)
    break;
    if (!geq_margin(ev.duration, SHARP_TRAILER_SPACE,
    SHARP_BIT_PULSE / 2))
    break;
// Validate - command, ext, chk should be inverted in 2nd
    msg = (data.bits >> 15) & 0x7fff;
    echo = data.bits & 0x7fff;
    if ((msg ^ echo) != 0x3ff) {
    dev_dbg(&dev.dev,
    "Sharp checksum error: received 0x%04x, 0x%04x\n",
    msg, echo);
    break;
    }
    address = bitrev8((msg >> 7) & 0xf8);
    command = bitrev8((msg >> 2) & 0xff);
    scancode = address << 8 | command;
    dev_dbg(&dev.dev, "Sharp scancode 0x%04x\n", scancode);
    rc_keydown(dev, RC_PROTO_SHARP, scancode, 0);
    data.state = STATE_INACTIVE;
    return 0;
    }
    dev_dbg(&dev.dev, "Sharp decode failed at count %d state %d (%uus %s)\n",
    data.count, data.state, ev.duration, TO_STR(ev.pulse));
    data.state = STATE_INACTIVE;
    return -EINVAL;
    }
    static const struct ir_raw_timings_pd ir_sharp_timings = {
    .header_pulse  = 0,
    .header_space  = 0,
    .bit_pulse     = SHARP_BIT_PULSE,
    .bit_space[0]  = SHARP_BIT_0_SPACE,
    .bit_space[1]  = SHARP_BIT_1_SPACE,
    .trailer_pulse = SHARP_BIT_PULSE,
    .trailer_space = SHARP_ECHO_SPACE,
    .msb_first     = 1,
    };
//
// ir_sharp_encode() - Encode a scancode as a stream of raw events
//
// @protocol:	protocol to encode
// @scancode:	scancode to encode
// @events:	array of raw ir events to write into
// @max:	maximum size of @events
//
// Returns:	The number of events written.
// -ENOBUFS if there isn't enough space in the array to fit the
// encoding. In this case all @max events will have been written.
//
    static int ir_sharp_encode(enum rc_proto protocol, u32 scancode,
    struct ir_raw_event *events, unsigned int max)
    {
    struct ir_raw_event *e = events;
    int ret;
    u32 raw;
    raw = (((bitrev8(scancode >> 8) >> 3) << 8) & 0x1f00) |
    bitrev8(scancode);
    ret = ir_raw_gen_pd(&e, max, &ir_sharp_timings, SHARP_NBITS,
    (raw << 2) | 2);
    if (ret < 0)
    return ret;
    max -= ret;
    raw = (((bitrev8(scancode >> 8) >> 3) << 8) & 0x1f00) |
    bitrev8(~scancode);
    ret = ir_raw_gen_pd(&e, max, &ir_sharp_timings, SHARP_NBITS,
    (raw << 2) | 1);
    if (ret < 0)
    return ret;
    return e - events;
    }
    static struct ir_raw_handler sharp_handler = {
    .protocols	= RC_PROTO_BIT_SHARP,
    .decode		= ir_sharp_decode,
    .encode		= ir_sharp_encode,
    .carrier	= 38000,
    .min_timeout	= SHARP_ECHO_SPACE + SHARP_ECHO_SPACE / 4,
    };
#[no_mangle]
unsafe extern "C" fn ir_sharp_decode_init() -> int __init {
    static int __init ir_sharp_decode_init(void)
    {
    ir_raw_handler_register(&sharp_handler);
    pr_info("IR Sharp protocol handler initialized\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ir_sharp_decode_exit() -> void __exit {
    static void __exit ir_sharp_decode_exit(void)
    {
    ir_raw_handler_unregister(&sharp_handler);
    }
    module_init(ir_sharp_decode_init);
    module_exit(ir_sharp_decode_exit);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("James Hogan <jhogan@kernel.org>");
    MODULE_DESCRIPTION("Sharp IR protocol decoder");
