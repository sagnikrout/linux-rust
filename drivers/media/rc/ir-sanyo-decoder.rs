//! Automatically rewritten from C to Rust
//! Source: drivers/media/rc/ir-sanyo-decoder.c
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
// ir-sanyo-decoder.c - handle SANYO IR Pulse/Space protocol
//
// Copyright (C) 2011 by Mauro Carvalho Chehab
//
// This protocol uses the NEC protocol timings. However, data is formatted as:
// 13 bits Custom Code
// 13 bits NOT(Custom Code)
// 8 bits Key data
// 8 bits NOT(Key data)
//
// According with LIRC, this protocol is used on Sanyo, Aiwa and Chinon
// Information for this protocol is available at the Sanyo LC7461 datasheet.

    enum sanyo_state {
    STATE_INACTIVE,
    STATE_HEADER_SPACE,
    STATE_BIT_PULSE,
    STATE_BIT_SPACE,
    STATE_TRAILER_PULSE,
    STATE_TRAILER_SPACE,
    };
//
// ir_sanyo_decode() - Decode one SANYO pulse or space
// @dev:	the struct rc_dev descriptor of the device
// @ev:		the struct ir_raw_event descriptor of the pulse/space
//
// This function returns -EINVAL if the pulse violates the state machine
//
#[no_mangle]
unsafe extern "C" fn ir_sanyo_decode(dev: *mut rc_dev, ev: ir_raw_event) -> c_int {
    static int ir_sanyo_decode(struct rc_dev *dev, struct ir_raw_event ev)
    {
    struct sanyo_dec *data = &dev.raw.sanyo;
    u32 scancode;
    u16 address;
    u8 command, not_command;
    if (!is_timing_event(ev)) {
    if (ev.overflow) {
    dev_dbg(&dev.dev, "SANYO event overflow received. reset to state 0\n");
    data.state = STATE_INACTIVE;
    }
    return 0;
    }
    dev_dbg(&dev.dev, "SANYO decode started at state %d (%uus %s)\n",
    data.state, ev.duration, TO_STR(ev.pulse));
    switch (data.state) {
    case STATE_INACTIVE:
    if (!ev.pulse)
    break;
    if (eq_margin(ev.duration, SANYO_HEADER_PULSE, SANYO_UNIT / 2)) {
    data.count = 0;
    data.state = STATE_HEADER_SPACE;
    return 0;
    }
    break;
    case STATE_HEADER_SPACE:
    if (ev.pulse)
    break;
    if (eq_margin(ev.duration, SANYO_HEADER_SPACE, SANYO_UNIT / 2)) {
    data.state = STATE_BIT_PULSE;
    return 0;
    }
    break;
    case STATE_BIT_PULSE:
    if (!ev.pulse)
    break;
    if (!eq_margin(ev.duration, SANYO_BIT_PULSE, SANYO_UNIT / 2))
    break;
    data.state = STATE_BIT_SPACE;
    return 0;
    case STATE_BIT_SPACE:
    if (ev.pulse)
    break;
    if (!data.count && geq_margin(ev.duration, SANYO_REPEAT_SPACE, SANYO_UNIT / 2)) {
    rc_repeat(dev);
    dev_dbg(&dev.dev, "SANYO repeat last key\n");
    data.state = STATE_INACTIVE;
    return 0;
    }
    data.bits <<= 1;
    if (eq_margin(ev.duration, SANYO_BIT_1_SPACE, SANYO_UNIT / 2))
    data.bits |= 1;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !eq_margin(ev.duration, _arg: SANYO_BIT_0_SPACE, 2): SANYO_UNIT /) -> else {
    else if (!eq_margin(ev.duration, SANYO_BIT_0_SPACE, SANYO_UNIT / 2))
    break;
    data.count++;
    if (data.count == SANYO_NBITS)
    data.state = STATE_TRAILER_PULSE;
    else
    data.state = STATE_BIT_PULSE;
    return 0;
    case STATE_TRAILER_PULSE:
    if (!ev.pulse)
    break;
    if (!eq_margin(ev.duration, SANYO_TRAILER_PULSE, SANYO_UNIT / 2))
    break;
    data.state = STATE_TRAILER_SPACE;
    return 0;
    case STATE_TRAILER_SPACE:
    if (ev.pulse)
    break;
    if (!geq_margin(ev.duration, SANYO_TRAILER_SPACE, SANYO_UNIT / 2))
    break;
    address     = bitrev16((data.bits >> 29) & 0x1fff) >> 3;
// not_address = bitrev16((data->bits >> 16) & 0x1fff) >> 3;
    command	    = bitrev8((data.bits >>  8) & 0xff);
    not_command = bitrev8((data.bits >>  0) & 0xff);
    if ((command ^ not_command) != 0xff) {
    dev_dbg(&dev.dev, "SANYO checksum error: received 0x%08llx\n",
    data.bits);
    data.state = STATE_INACTIVE;
    return 0;
    }
    scancode = address << 8 | command;
    dev_dbg(&dev.dev, "SANYO scancode: 0x%06x\n", scancode);
    rc_keydown(dev, RC_PROTO_SANYO, scancode, 0);
    data.state = STATE_INACTIVE;
    return 0;
    }
    dev_dbg(&dev.dev, "SANYO decode failed at count %d state %d (%uus %s)\n",
    data.count, data.state, ev.duration, TO_STR(ev.pulse));
    data.state = STATE_INACTIVE;
    return -EINVAL;
    }
    static const struct ir_raw_timings_pd ir_sanyo_timings = {
    .header_pulse  = SANYO_HEADER_PULSE,
    .header_space  = SANYO_HEADER_SPACE,
    .bit_pulse     = SANYO_BIT_PULSE,
    .bit_space[0]  = SANYO_BIT_0_SPACE,
    .bit_space[1]  = SANYO_BIT_1_SPACE,
    .trailer_pulse = SANYO_TRAILER_PULSE,
    .trailer_space = SANYO_TRAILER_SPACE,
    .msb_first     = 1,
    };
//
// ir_sanyo_encode() - Encode a scancode as a stream of raw events
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
    static int ir_sanyo_encode(enum rc_proto protocol, u32 scancode,
    struct ir_raw_event *events, unsigned int max)
    {
    struct ir_raw_event *e = events;
    int ret;
    u64 raw;
    raw = ((u64)(bitrev16(scancode >> 8) & 0xfff8) << (8 + 8 + 13 - 3)) |
    ((u64)(bitrev16(~scancode >> 8) & 0xfff8) << (8 + 8 +  0 - 3)) |
    ((bitrev8(scancode) & 0xff) << 8) |
    (bitrev8(~scancode) & 0xff);
    ret = ir_raw_gen_pd(&e, max, &ir_sanyo_timings, SANYO_NBITS, raw);
    if (ret < 0)
    return ret;
    return e - events;
    }
    static struct ir_raw_handler sanyo_handler = {
    .protocols	= RC_PROTO_BIT_SANYO,
    .decode		= ir_sanyo_decode,
    .encode		= ir_sanyo_encode,
    .carrier	= 38000,
    .min_timeout	= SANYO_TRAILER_SPACE,
    };
#[no_mangle]
unsafe extern "C" fn ir_sanyo_decode_init() -> int __init {
    static int __init ir_sanyo_decode_init(void)
    {
    ir_raw_handler_register(&sanyo_handler);
    printk(KERN_INFO "IR SANYO protocol handler initialized\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ir_sanyo_decode_exit() -> void __exit {
    static void __exit ir_sanyo_decode_exit(void)
    {
    ir_raw_handler_unregister(&sanyo_handler);
    }
    module_init(ir_sanyo_decode_init);
    module_exit(ir_sanyo_decode_exit);
    MODULE_LICENSE("GPL v2");
    MODULE_AUTHOR("Mauro Carvalho Chehab");
    MODULE_AUTHOR("Red Hat Inc. (http://www.redhat.com)");
    MODULE_DESCRIPTION("SANYO IR protocol decoder");
