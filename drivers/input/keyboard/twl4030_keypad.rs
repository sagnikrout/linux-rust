//! Automatically rewritten from C to Rust
//! Source: drivers/input/keyboard/twl4030_keypad.c
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
// twl4030_keypad.c - driver for 8x8 keypad controller in twl4030 chips
//
// Copyright (C) 2007 Texas Instruments, Inc.
// Copyright (C) 2008 Nokia Corporation
//
// Code re-written for 2430SDP by:
// Syed Mohammed Khasim <x0khasim@ti.com>
//
// Initial Code:
// Manjunatha G K <manjugk@ti.com>
//

//
// The TWL4030 family chips include a keypad controller that supports
// up to an 8x8 switch matrix.  The controller can issue system wakeup
// events, since it uses only the always-on 32KiHz oscillator, and has
// an internal state machine that decodes pressed keys, including
// multi-key combinations.
//
// See the TPS65950 documentation; that's the general availability
// version of the TWL5030 second generation part.
//

pub const TWL4030_MAX_COLS: c_int = 8;
//
// Note that we add space for an extra column so that we can handle
// row lines connected to the gnd (see twl4030_col_xlate()).
//
pub const TWL4030_ROW_SHIFT: c_int = 4;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct twl4030_keypad {
    pub keymap: [c_ushort; TWL4030_KEYMAP_SIZE],
    pub kp_state: [u16; TWL4030_MAX_ROWS],
    pub n_rows: c_uint,
    pub n_cols: c_uint,
    pub irq: c_int,
    pub dbg_dev: *mut device,
    pub input: *mut input_dev,
}

// ----------------------------------------------------------------------
// arbitrary prescaler value 0..7
pub const PTV_PRESCALER: c_int = 4;
// Register Offsets
pub const KEYP_CTRL: c_uint = 0x00;
pub const KEYP_DEB: c_uint = 0x01;
pub const KEYP_LONG_KEY: c_uint = 0x02;
pub const KEYP_LK_PTV: c_uint = 0x03;
pub const KEYP_TIMEOUT_L: c_uint = 0x04;
pub const KEYP_TIMEOUT_H: c_uint = 0x05;
pub const KEYP_KBC: c_uint = 0x06;
pub const KEYP_KBR: c_uint = 0x07;
pub const KEYP_SMS: c_uint = 0x08;
pub const KEYP_FULL_CODE_7_0: c_uint = 0x09	/* row 0 column status */;
pub const KEYP_FULL_CODE_15_8: c_uint = 0x0a	/* ... row 1 ... */;
pub const KEYP_FULL_CODE_23_16: c_uint = 0x0b;
pub const KEYP_FULL_CODE_31_24: c_uint = 0x0c;
pub const KEYP_FULL_CODE_39_32: c_uint = 0x0d;
pub const KEYP_FULL_CODE_47_40: c_uint = 0x0e;
pub const KEYP_FULL_CODE_55_48: c_uint = 0x0f;
pub const KEYP_FULL_CODE_63_56: c_uint = 0x10;
pub const KEYP_ISR1: c_uint = 0x11;
pub const KEYP_IMR1: c_uint = 0x12;
pub const KEYP_ISR2: c_uint = 0x13;
pub const KEYP_IMR2: c_uint = 0x14;
pub const KEYP_SIR: c_uint = 0x15;
pub const KEYP_EDR: c_uint = 0x16	/* edge triggers */;
pub const KEYP_SIH_CTRL: c_uint = 0x17;
// KEYP_CTRL_REG Fields

// KEYP_DEB, KEYP_LONG_KEY, KEYP_TIMEOUT_x

// KEYP_LK_PTV_REG Fields
pub const KEYP_LK_PTV_PTV_SHIFT: c_int = 5;
// KEYP_{IMR,ISR,SIR} Fields

// KEYP_EDR Fields
pub const KEYP_EDR_KP_FALLING: c_uint = 0x01;
pub const KEYP_EDR_KP_RISING: c_uint = 0x02;
pub const KEYP_EDR_KP_BOTH: c_uint = 0x03;
pub const KEYP_EDR_LK_FALLING: c_uint = 0x04;
pub const KEYP_EDR_LK_RISING: c_uint = 0x08;
pub const KEYP_EDR_TO_FALLING: c_uint = 0x10;
pub const KEYP_EDR_TO_RISING: c_uint = 0x20;
pub const KEYP_EDR_MIS_FALLING: c_uint = 0x40;
pub const KEYP_EDR_MIS_RISING: c_uint = 0x80;
// ----------------------------------------------------------------------
    static int twl4030_kpread(struct twl4030_keypad *kp,
    u8 *data, u32 reg, u8 num_bytes)
    {
    let mut ret: c_int = twl_i2c_read(TWL4030_MODULE_KEYPAD, data, reg, num_bytes);
    if (ret < 0)
    dev_warn(kp.dbg_dev,
    "Couldn't read TWL4030: %X - ret %d[%x]\n",
    reg, ret, ret);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn twl4030_kpwrite_u8(kp: *mut twl4030_keypad, data: u8, reg: u32) -> c_int {
    static int twl4030_kpwrite_u8(struct twl4030_keypad *kp, u8 data, u32 reg)
    {
    let mut ret: c_int = twl_i2c_write_u8(TWL4030_MODULE_KEYPAD, data, reg);
    if (ret < 0)
    dev_warn(kp.dbg_dev,
    "Could not write TWL4030: %X - ret %d[%x]\n",
    reg, ret, ret);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn twl4030_col_xlate(kp: *mut twl4030_keypad, col: u8) -> u16 {
    static inline u16 twl4030_col_xlate(struct twl4030_keypad *kp, u8 col)
    {
//
// If all bits in a row are active for all columns then
// we have that row line connected to gnd. Mark this
// key on as if it was on matrix position n_cols (i.e.
// one higher than the size of the matrix).
//
    if (col == 0xFF)
    return 1 << kp.n_cols;
    else
    return col & ((1 << kp.n_cols) - 1);
    }
#[no_mangle]
unsafe extern "C" fn twl4030_read_kp_matrix_state(kp: *mut twl4030_keypad, state: *mut u16) -> c_int {
    static int twl4030_read_kp_matrix_state(struct twl4030_keypad *kp, u16 *state)
    {
    u8 new_state[TWL4030_MAX_ROWS];
    int row;
    int ret = twl4030_kpread(kp, new_state,
    KEYP_FULL_CODE_7_0, kp.n_rows);
    if (ret >= 0)
    for (row = 0; row < kp.n_rows; row++)
    state[row] = twl4030_col_xlate(kp, new_state[row]);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn twl4030_is_in_ghost_state(kp: *mut twl4030_keypad, key_state: *mut u16) -> bool {
    static bool twl4030_is_in_ghost_state(struct twl4030_keypad *kp, u16 *key_state)
    {
    int i;
    let mut check: u16 = 0;
    for (i = 0; i < kp.n_rows; i++) {
    let mut col: u16 = key_state[i];
    if ((col & check) && hweight16(col) > 1)
    return true;
    check |= col;
    }
    return false;
    }
#[no_mangle]
unsafe extern "C" fn twl4030_kp_scan(kp: *mut twl4030_keypad, release_all: bool) {
    static void twl4030_kp_scan(struct twl4030_keypad *kp, bool release_all)
    {
    struct input_dev *input = kp.input;
    u16 new_state[TWL4030_MAX_ROWS];
    int col, row;
    if (release_all) {
    memset(new_state, 0, sizeof(new_state));
    } else {
// check for any changes
    let mut ret: c_int = twl4030_read_kp_matrix_state(kp, new_state);
    if (ret < 0)	/* panic ... */
    return;
    if (twl4030_is_in_ghost_state(kp, new_state))
    return;
    }
// check for changes and print those
    for (row = 0; row < kp.n_rows; row++) {
    let mut changed: c_int = new_state[row] ^ kp.kp_state[row];
    if (!changed)
    continue;
// Extra column handles "all gnd" rows
    for (col = 0; col < kp.n_cols + 1; col++) {
    int code;
    if (!(changed & (1 << col)))
    continue;
    dev_dbg(kp.dbg_dev, "key [%d:%d] %s\n", row, col,
    (new_state[row] & (1 << col)) ?
    "press" : "release");
    code = MATRIX_SCAN_CODE(row, col, TWL4030_ROW_SHIFT);
    input_event(input, EV_MSC, MSC_SCAN, code);
    input_report_key(input, kp.keymap[code],
    new_state[row] & (1 << col));
    }
    kp.kp_state[row] = new_state[row];
    }
    input_sync(input);
    }
//
// Keypad interrupt handler
//
#[no_mangle]
unsafe extern "C" fn do_kp_irq(irq: c_int, _kp: *mut c_void) -> irqreturn_t {
    static irqreturn_t do_kp_irq(int irq, void *_kp)
    {
    struct twl4030_keypad *kp = _kp;
    u8 reg;
    int ret;
// Read & Clear TWL4030 pending interrupt
    ret = twl4030_kpread(kp, &reg, KEYP_ISR1, 1);
//
// Release all keys if I2C has gone bad or
// the KEYP has gone to idle state.
//
    if (ret >= 0 && (reg & KEYP_IMR1_KP))
    twl4030_kp_scan(kp, false);
    else
    twl4030_kp_scan(kp, true);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn twl4030_kp_program(kp: *mut twl4030_keypad) -> c_int {
    static int twl4030_kp_program(struct twl4030_keypad *kp)
    {
    u8 reg;
    int i;
// Enable controller, with hardware decoding but not autorepeat
    reg = KEYP_CTRL_SOFT_NRST | KEYP_CTRL_SOFTMODEN
    | KEYP_CTRL_TOE_EN | KEYP_CTRL_KBD_ON;
    if (twl4030_kpwrite_u8(kp, reg, KEYP_CTRL) < 0)
    return -EIO;
//
// NOTE: we could use sih_setup() here to package keypad
// event sources as four different IRQs ... but we don't.
//
// Enable TO rising and KP rising and falling edge detection
    reg = KEYP_EDR_KP_BOTH | KEYP_EDR_TO_RISING;
    if (twl4030_kpwrite_u8(kp, reg, KEYP_EDR) < 0)
    return -EIO;
// Set PTV prescaler Field
    reg = (PTV_PRESCALER << KEYP_LK_PTV_PTV_SHIFT);
    if (twl4030_kpwrite_u8(kp, reg, KEYP_LK_PTV) < 0)
    return -EIO;
// Set key debounce time to 20 ms
    i = KEYP_PERIOD_US(20000, PTV_PRESCALER);
    if (twl4030_kpwrite_u8(kp, i, KEYP_DEB) < 0)
    return -EIO;
// Set timeout period to 200 ms
    i = KEYP_PERIOD_US(200000, PTV_PRESCALER);
    if (twl4030_kpwrite_u8(kp, (i & 0xFF), KEYP_TIMEOUT_L) < 0)
    return -EIO;
    if (twl4030_kpwrite_u8(kp, (i >> 8), KEYP_TIMEOUT_H) < 0)
    return -EIO;
//
// Enable Clear-on-Read; disable remembering events that fire
// after the IRQ but before our handler acks (reads) them.
//
    reg = TWL4030_SIH_CTRL_COR_MASK | TWL4030_SIH_CTRL_PENDDIS_MASK;
    if (twl4030_kpwrite_u8(kp, reg, KEYP_SIH_CTRL) < 0)
    return -EIO;
// initialize key state; irqs update it from here on
    if (twl4030_read_kp_matrix_state(kp, kp.kp_state) < 0)
    return -EIO;
    return 0;
    }
//
// Registers keypad device with input subsystem
// and configures TWL4030 keypad registers
//
#[no_mangle]
unsafe extern "C" fn twl4030_kp_probe(pdev: *mut platform_device) -> c_int {
    static int twl4030_kp_probe(struct platform_device *pdev)
    {
    struct twl4030_keypad *kp;
    struct input_dev *input;
    u8 reg;
    int error;
    kp = devm_kzalloc(&pdev.dev, sizeof(*kp), GFP_KERNEL);
    if (!kp)
    return -ENOMEM;
    input = devm_input_allocate_device(&pdev.dev);
    if (!input)
    return -ENOMEM;
// get the debug device
    kp.dbg_dev		= &pdev.dev;
    kp.input		= input;
// setup input device
    input.name		= "TWL4030 Keypad";
    input.phys		= "twl4030_keypad/input0";
    input.id.bustype	= BUS_HOST;
    input.id.vendor	= 0x0001;
    input.id.product	= 0x0001;
    input.id.version	= 0x0003;
    error = matrix_keypad_parse_properties(&pdev.dev,
    &kp.n_rows, &kp.n_cols);
    if (error)
    return error;
    if (kp.n_rows > TWL4030_MAX_ROWS || kp.n_cols > TWL4030_MAX_COLS) {
    dev_err(&pdev.dev,
    "Invalid rows/cols amount specified in platform/devicetree data\n");
    return -EINVAL;
    }
    kp.irq = platform_get_irq(pdev, 0);
    if (kp.irq < 0)
    return kp.irq;
    error = matrix_keypad_build_keymap(core::ptr::null_mut(), core::ptr::null_mut(),
    TWL4030_MAX_ROWS,
    1 << TWL4030_ROW_SHIFT,
    kp.keymap, input);
    if (error) {
    dev_err(kp.dbg_dev, "Failed to build keymap\n");
    return error;
    }
    input_set_capability(input, EV_MSC, MSC_SCAN);
    __set_bit(EV_REP, input.evbit);
    error = input_register_device(input);
    if (error) {
    dev_err(kp.dbg_dev,
    "Unable to register twl4030 keypad device\n");
    return error;
    }
    error = twl4030_kp_program(kp);
    if (error)
    return error;
//
// This ISR will always execute in kernel thread context because of
// the need to access the TWL4030 over the I2C bus.
//
// NOTE:  we assume this host is wired to TWL4040 INT1, not INT2 ...
//
    error = devm_request_threaded_irq(&pdev.dev, kp.irq, core::ptr::null_mut(), do_kp_irq,
    0, pdev.name, kp);
    if (error) {
    dev_info(kp.dbg_dev, "request_irq failed for irq no=%d: %d\n",
    kp.irq, error);
    return error;
    }
// Enable KP and TO interrupts now.
    reg = (u8) ~(KEYP_IMR1_KP | KEYP_IMR1_TO);
    if (twl4030_kpwrite_u8(kp, reg, KEYP_IMR1)) {
// mask all events - we don't care about the result
    (void) twl4030_kpwrite_u8(kp, 0xff, KEYP_IMR1);
    return -EIO;
    }
    return 0;
    }

    static const struct of_device_id twl4030_keypad_dt_match_table[] = {
    { .compatible = "ti,twl4030-keypad" },
    {},
    };
    MODULE_DEVICE_TABLE(of, twl4030_keypad_dt_match_table);

//
// NOTE: twl4030 are multi-function devices connected via I2C.
// So this device is a child of an I2C parent, thus it needs to
// support unplug/replug (which most platform devices don't).
//
    static struct platform_driver twl4030_kp_driver = {
    .probe		= twl4030_kp_probe,
    .driver		= {
    .name	= "twl4030_keypad",
    .of_match_table = of_match_ptr(twl4030_keypad_dt_match_table),
    },
    };
    module_platform_driver(twl4030_kp_driver);
    MODULE_AUTHOR("Texas Instruments");
    MODULE_DESCRIPTION("TWL4030 Keypad Driver");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("platform:twl4030_keypad");
