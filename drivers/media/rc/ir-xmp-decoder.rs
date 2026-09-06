//! Automatically rewritten from C to Rust
//! Source: drivers/media/rc/ir-xmp-decoder.c
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
// ir-xmp-decoder.c - handle XMP IR Pulse/Space protocol
//
// Copyright (C) 2014 by Marcel Mol
//
// - Based on info from http://www.hifi-remote.com
// - Ignore Toggle=9 frames
// - Ignore XMP-1 XMP-2 difference, always store 16 bit OBC
//

// should be 80ms but not all duration supliers can go that high
pub const XMP_TRAILER_SPACE: c_int = 20000;
    enum xmp_state {
    STATE_INACTIVE,
    STATE_LEADER_PULSE,
    STATE_NIBBLE_SPACE,
    };
//
// ir_xmp_decode() - Decode one XMP pulse or space
// @dev:	the struct rc_dev descriptor of the device
// @ev:		the struct ir_raw_event descriptor of the pulse/space
//
// This function returns -EINVAL if the pulse violates the state machine
//
#[no_mangle]
unsafe extern "C" fn ir_xmp_decode(dev: *mut rc_dev, ev: ir_raw_event) -> c_int {
    static int ir_xmp_decode(struct rc_dev *dev, struct ir_raw_event ev)
    {
    struct xmp_dec *data = &dev.raw.xmp;
    if (!is_timing_event(ev)) {
    if (ev.overflow)
    data.state = STATE_INACTIVE;
    return 0;
    }
    dev_dbg(&dev.dev, "XMP decode started at state %d %d (%uus %s)\n",
    data.state, data.count, ev.duration, TO_STR(ev.pulse));
    switch (data.state) {
    case STATE_INACTIVE:
    if (!ev.pulse)
    break;
    if (eq_margin(ev.duration, XMP_LEADER, XMP_UNIT / 2)) {
    data.count = 0;
    data.state = STATE_NIBBLE_SPACE;
    }
    return 0;
    case STATE_LEADER_PULSE:
    if (!ev.pulse)
    break;
    if (eq_margin(ev.duration, XMP_LEADER, XMP_UNIT / 2))
    data.state = STATE_NIBBLE_SPACE;
    return 0;
    case STATE_NIBBLE_SPACE:
    if (ev.pulse)
    break;
    if (geq_margin(ev.duration, XMP_TRAILER_SPACE, XMP_NIBBLE_PREFIX)) {
    int divider, i;
    u8 addr, subaddr, subaddr2, toggle, oem, obc1, obc2, sum1, sum2;
    u32 *n;
    u32 scancode;
    if (data.count != 16) {
    dev_dbg(&dev.dev, "received TRAILER period at index %d: %u\n",
    data.count, ev.duration);
    data.state = STATE_INACTIVE;
    return -EINVAL;
    }
    n = data.durations;
//
// the 4th nibble should be 15 so base the divider on this
// to transform durations into nibbles. Subtract 2000 from
// the divider to compensate for fluctuations in the signal
//
    divider = (n[3] - XMP_NIBBLE_PREFIX) / 15 - 2000;
    if (divider < 50) {
    dev_dbg(&dev.dev, "divider to small %d.\n",
    divider);
    data.state = STATE_INACTIVE;
    return -EINVAL;
    }
// convert to nibbles and do some sanity checks
    for (i = 0; i < 16; i++)
    n[i] = (n[i] - XMP_NIBBLE_PREFIX) / divider;
    sum1 = (15 + n[0] + n[1] + n[2] + n[3] +
    n[4] + n[5] + n[6] + n[7]) % 16;
    sum2 = (15 + n[8] + n[9] + n[10] + n[11] +
    n[12] + n[13] + n[14] + n[15]) % 16;
    if (sum1 != 15 || sum2 != 15) {
    dev_dbg(&dev.dev, "checksum errors sum1=0x%X sum2=0x%X\n",
    sum1, sum2);
    data.state = STATE_INACTIVE;
    return -EINVAL;
    }
    subaddr  = n[0] << 4 | n[2];
    subaddr2 = n[8] << 4 | n[11];
    oem      = n[4] << 4 | n[5];
    addr     = n[6] << 4 | n[7];
    toggle   = n[10];
    obc1 = n[12] << 4 | n[13];
    obc2 = n[14] << 4 | n[15];
    if (subaddr != subaddr2) {
    dev_dbg(&dev.dev, "subaddress nibbles mismatch 0x%02X != 0x%02X\n",
    subaddr, subaddr2);
    data.state = STATE_INACTIVE;
    return -EINVAL;
    }
    if (oem != 0x44)
    dev_dbg(&dev.dev, "Warning: OEM nibbles 0x%02X. Expected 0x44\n",
    oem);
    scancode = addr << 24 | subaddr << 16 |
    obc1 << 8 | obc2;
    dev_dbg(&dev.dev, "XMP scancode 0x%06x\n", scancode);
    if (toggle == 0) {
    rc_keydown(dev, RC_PROTO_XMP, scancode, 0);
    } else {
    rc_repeat(dev);
    dev_dbg(&dev.dev, "Repeat last key\n");
    }
    data.state = STATE_INACTIVE;
    return 0;
    } else if (geq_margin(ev.duration, XMP_HALFFRAME_SPACE, XMP_NIBBLE_PREFIX)) {
// Expect 8 or 16 nibble pulses. 16 in case of 'final' frame
    if (data.count == 16) {
    dev_dbg(&dev.dev, "received half frame pulse at index %d. Probably a final frame key-up event: %u\n",
    data.count, ev.duration);
//
// TODO: for now go back to half frame position
// so trailer can be found and key press
// can be handled.
//
    data.count = 8;
    }
#[no_mangle]
pub unsafe extern "C" fn if(8: data->count !=) -> else {
    else if (data.count != 8)
    dev_dbg(&dev.dev, "received half frame pulse at index %d: %u\n",
    data.count, ev.duration);
    data.state = STATE_LEADER_PULSE;
    return 0;
    } else if (geq_margin(ev.duration, XMP_NIBBLE_PREFIX, XMP_UNIT)) {
// store nibble raw data, decode after trailer
    if (data.count == 16) {
    dev_dbg(&dev.dev, "too many pulses (%d) ignoring: %u\n",
    data.count, ev.duration);
    data.state = STATE_INACTIVE;
    return -EINVAL;
    }
    data.durations[data.count] = ev.duration;
    data.count++;
    data.state = STATE_LEADER_PULSE;
    return 0;
    }
    break;
    }
    dev_dbg(&dev.dev, "XMP decode failed at count %d state %d (%uus %s)\n",
    data.count, data.state, ev.duration, TO_STR(ev.pulse));
    data.state = STATE_INACTIVE;
    return -EINVAL;
    }
    static struct ir_raw_handler xmp_handler = {
    .protocols	= RC_PROTO_BIT_XMP,
    .decode		= ir_xmp_decode,
    .min_timeout	= XMP_TRAILER_SPACE,
    };
#[no_mangle]
unsafe extern "C" fn ir_xmp_decode_init() -> int __init {
    static int __init ir_xmp_decode_init(void)
    {
    ir_raw_handler_register(&xmp_handler);
    printk(KERN_INFO "IR XMP protocol handler initialized\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ir_xmp_decode_exit() -> void __exit {
    static void __exit ir_xmp_decode_exit(void)
    {
    ir_raw_handler_unregister(&xmp_handler);
    }
    module_init(ir_xmp_decode_init);
    module_exit(ir_xmp_decode_exit);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Marcel Mol <marcel@mesa.nl>");
    MODULE_AUTHOR("MESA Consulting (http://www.mesa.nl)");
    MODULE_DESCRIPTION("XMP IR protocol decoder");
