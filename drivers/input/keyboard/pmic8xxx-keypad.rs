//! Automatically rewritten from C to Rust
//! Source: drivers/input/keyboard/pmic8xxx-keypad.c
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
// Copyright (c) 2009-2011, Code Aurora Forum. All rights reserved.
//

pub const PM8XXX_MAX_ROWS: c_int = 18;
pub const PM8XXX_MAX_COLS: c_int = 8;
pub const PM8XXX_ROW_SHIFT: c_int = 3;

pub const PM8XXX_MIN_ROWS: c_int = 5;
pub const PM8XXX_MIN_COLS: c_int = 5;
pub const MAX_SCAN_DELAY: c_int = 128;
pub const MIN_SCAN_DELAY: c_int = 1;
// in nanoseconds
pub const MAX_ROW_HOLD_DELAY: c_int = 122000;
pub const MIN_ROW_HOLD_DELAY: c_int = 30500;
pub const MAX_DEBOUNCE_TIME: c_int = 20;
pub const MIN_DEBOUNCE_TIME: c_int = 5;
pub const KEYP_CTRL: c_uint = 0x148;

pub const KEYP_CTRL_EVNTS_MASK: c_uint = 0x3;
pub const KEYP_CTRL_SCAN_COLS_SHIFT: c_int = 5;
pub const KEYP_CTRL_SCAN_COLS_MIN: c_int = 5;
pub const KEYP_CTRL_SCAN_COLS_BITS: c_uint = 0x3;
pub const KEYP_CTRL_SCAN_ROWS_SHIFT: c_int = 2;
pub const KEYP_CTRL_SCAN_ROWS_MIN: c_int = 5;
pub const KEYP_CTRL_SCAN_ROWS_BITS: c_uint = 0x7;

pub const KEYP_SCAN: c_uint = 0x149;

pub const KEYP_SCAN_DBOUNCE_SHIFT: c_int = 1;
pub const KEYP_SCAN_PAUSE_SHIFT: c_int = 3;
pub const KEYP_SCAN_ROW_HOLD_SHIFT: c_int = 6;
pub const KEYP_TEST: c_uint = 0x14A;

pub const KEYP_TEST_DBG_SELECT_SHIFT: c_int = 1;
// bits of these registers represent
// '0' for key press
// '1' for key release
//
pub const KEYP_RECENT_DATA: c_uint = 0x14B;
pub const KEYP_OLD_DATA: c_uint = 0x14C;
pub const KEYP_CLOCK_FREQ: c_int = 32768;
//
// struct pmic8xxx_kp - internal keypad data structure
// @num_cols: number of columns of keypad
// @num_rows: number of row of keypad
// @input: input device pointer for keypad
// @regmap: regmap handle
// @key_sense_irq: key press/release irq number
// @key_stuck_irq: key stuck notification irq number
// @keycodes: array to hold the key codes
// @dev: parent device pointer
// @keystate: present key press/release state
// @stuckstate: present state when key stuck irq
// @ctrl_reg: control register value
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pmic8xxx_kp {
    pub num_rows: c_uint,
    pub num_cols: c_uint,
    pub input: *mut input_dev,
    pub regmap: *mut regmap,
    pub key_sense_irq: c_int,
    pub key_stuck_irq: c_int,
    pub keycodes: [c_ushort; PM8XXX_MATRIX_MAX_SIZE],
    pub dev: *mut device,
    pub keystate: [u16; PM8XXX_MAX_ROWS],
    pub stuckstate: [u16; PM8XXX_MAX_ROWS],
    pub ctrl_reg: u8,
}

#[no_mangle]
unsafe extern "C" fn pmic8xxx_col_state(kp: *mut pmic8xxx_kp, col: u8) -> u8 {
    static u8 pmic8xxx_col_state(struct pmic8xxx_kp *kp, u8 col)
    {
// all keys pressed on that particular row?
    if (col == 0x00)
    return 1 << kp.num_cols;
    else
    return col & ((1 << kp.num_cols) - 1);
    }
//
// Synchronous read protocol for RevB0 onwards:
//
// 1. Write '1' to ReadState bit in KEYP_SCAN register
// 2. Wait 2*32KHz clocks, so that HW can successfully enter read mode
// synchronously
// 3. Read rows in old array first if events are more than one
// 4. Read rows in recent array
// 5. Wait 4*32KHz clocks
// 6. Write '0' to ReadState bit of KEYP_SCAN register so that hw can
// synchronously exit read mode.
//
#[no_mangle]
unsafe extern "C" fn pmic8xxx_chk_sync_read(kp: *mut pmic8xxx_kp) -> c_int {
    static int pmic8xxx_chk_sync_read(struct pmic8xxx_kp *kp)
    {
    int rc;
    unsigned int scan_val;
    rc = regmap_read(kp.regmap, KEYP_SCAN, &scan_val);
    if (rc < 0) {
    dev_err(kp.dev, "Error reading KEYP_SCAN reg, rc=%d\n", rc);
    return rc;
    }
    scan_val |= 0x1;
    rc = regmap_write(kp.regmap, KEYP_SCAN, scan_val);
    if (rc < 0) {
    dev_err(kp.dev, "Error writing KEYP_SCAN reg, rc=%d\n", rc);
    return rc;
    }
// 2 * 32KHz clocks
    udelay((2 * DIV_ROUND_UP(USEC_PER_SEC, KEYP_CLOCK_FREQ)) + 1);
    return rc;
    }
    static int pmic8xxx_kp_read_data(struct pmic8xxx_kp *kp, u16 *state,
    u16 data_reg, int read_rows)
    {
    int rc, row;
    unsigned int val;
    for (row = 0; row < read_rows; row++) {
    rc = regmap_read(kp.regmap, data_reg, &val);
    if (rc)
    return rc;
    dev_dbg(kp.dev, "%d = %d\n", row, val);
    state[row] = pmic8xxx_col_state(kp, val);
    }
    return 0;
    }
    static int pmic8xxx_kp_read_matrix(struct pmic8xxx_kp *kp, u16 *new_state,
    u16 *old_state)
    {
    int rc, read_rows;
    unsigned int scan_val;
    if (kp.num_rows < PM8XXX_MIN_ROWS)
    read_rows = PM8XXX_MIN_ROWS;
    else
    read_rows = kp.num_rows;
    pmic8xxx_chk_sync_read(kp);
    if (old_state) {
    rc = pmic8xxx_kp_read_data(kp, old_state, KEYP_OLD_DATA,
    read_rows);
    if (rc < 0) {
    dev_err(kp.dev,
    "Error reading KEYP_OLD_DATA, rc=%d\n", rc);
    return rc;
    }
    }
    rc = pmic8xxx_kp_read_data(kp, new_state, KEYP_RECENT_DATA,
    read_rows);
    if (rc < 0) {
    dev_err(kp.dev,
    "Error reading KEYP_RECENT_DATA, rc=%d\n", rc);
    return rc;
    }
// 4 * 32KHz clocks
    udelay((4 * DIV_ROUND_UP(USEC_PER_SEC, KEYP_CLOCK_FREQ)) + 1);
    rc = regmap_read(kp.regmap, KEYP_SCAN, &scan_val);
    if (rc < 0) {
    dev_err(kp.dev, "Error reading KEYP_SCAN reg, rc=%d\n", rc);
    return rc;
    }
    scan_val &= 0xFE;
    rc = regmap_write(kp.regmap, KEYP_SCAN, scan_val);
    if (rc < 0)
    dev_err(kp.dev, "Error writing KEYP_SCAN reg, rc=%d\n", rc);
    return rc;
    }
    static void __pmic8xxx_kp_scan_matrix(struct pmic8xxx_kp *kp, u16 *new_state,
    u16 *old_state)
    {
    int row, col, code;
    for (row = 0; row < kp.num_rows; row++) {
    let mut bits_changed: c_int = new_state[row] ^ old_state[row];
    if (!bits_changed)
    continue;
    for (col = 0; col < kp.num_cols; col++) {
    if (!(bits_changed & (1 << col)))
    continue;
    dev_dbg(kp.dev, "key [%d:%d] %s\n", row, col,
    !(new_state[row] & (1 << col)) ?
    "pressed" : "released");
    code = MATRIX_SCAN_CODE(row, col, PM8XXX_ROW_SHIFT);
    input_event(kp.input, EV_MSC, MSC_SCAN, code);
    input_report_key(kp.input,
    kp.keycodes[code],
    !(new_state[row] & (1 << col)));
    input_sync(kp.input);
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn pmic8xxx_detect_ghost_keys(kp: *mut pmic8xxx_kp, new_state: *mut u16) -> bool {
    static bool pmic8xxx_detect_ghost_keys(struct pmic8xxx_kp *kp, u16 *new_state)
    {
    int row, found_first = -1;
    u16 check, row_state;
    check = 0;
    for (row = 0; row < kp.num_rows; row++) {
    row_state = (~new_state[row]) &
    ((1 << kp.num_cols) - 1);
    if (hweight16(row_state) > 1) {
    if (found_first == -1)
    found_first = row;
    if (check & row_state) {
    dev_dbg(kp.dev, "detected ghost key on row[%d]"
    " and row[%d]\n", found_first, row);
    return true;
    }
    }
    check |= row_state;
    }
    return false;
    }
#[no_mangle]
unsafe extern "C" fn pmic8xxx_kp_scan_matrix(kp: *mut pmic8xxx_kp, events: c_uint) -> c_int {
    static int pmic8xxx_kp_scan_matrix(struct pmic8xxx_kp *kp, unsigned int events)
    {
    u16 new_state[PM8XXX_MAX_ROWS];
    u16 old_state[PM8XXX_MAX_ROWS];
    int rc;
    switch (events) {
    case 0x1:
    rc = pmic8xxx_kp_read_matrix(kp, new_state, core::ptr::null_mut());
    if (rc < 0)
    return rc;
// detecting ghost key is not an error
    if (pmic8xxx_detect_ghost_keys(kp, new_state))
    return 0;
    __pmic8xxx_kp_scan_matrix(kp, new_state, kp.keystate);
    memcpy(kp.keystate, new_state, sizeof(new_state));
    break;
    case 0x3: /* two events - eventcounter is gray-coded */
    rc = pmic8xxx_kp_read_matrix(kp, new_state, old_state);
    if (rc < 0)
    return rc;
    __pmic8xxx_kp_scan_matrix(kp, old_state, kp.keystate);
    __pmic8xxx_kp_scan_matrix(kp, new_state, old_state);
    memcpy(kp.keystate, new_state, sizeof(new_state));
    break;
    case 0x2:
    dev_dbg(kp.dev, "Some key events were lost\n");
    rc = pmic8xxx_kp_read_matrix(kp, new_state, old_state);
    if (rc < 0)
    return rc;
    __pmic8xxx_kp_scan_matrix(kp, old_state, kp.keystate);
    __pmic8xxx_kp_scan_matrix(kp, new_state, old_state);
    memcpy(kp.keystate, new_state, sizeof(new_state));
    break;
    default:
    rc = -EINVAL;
    }
    return rc;
    }
//
// NOTE: We are reading recent and old data registers blindly
// whenever key-stuck interrupt happens, because events counter doesn't
// get updated when this interrupt happens due to key stuck doesn't get
// considered as key state change.
//
// We are not using old data register contents after they are being read
// because it might report the key which was pressed before the key being stuck
// as stuck key because it's pressed status is stored in the old data
// register.
//
#[no_mangle]
unsafe extern "C" fn pmic8xxx_kp_stuck_irq(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t pmic8xxx_kp_stuck_irq(int irq, void *data)
    {
    u16 new_state[PM8XXX_MAX_ROWS];
    u16 old_state[PM8XXX_MAX_ROWS];
    int rc;
    struct pmic8xxx_kp *kp = data;
    rc = pmic8xxx_kp_read_matrix(kp, new_state, old_state);
    if (rc < 0) {
    dev_err(kp.dev, "failed to read keypad matrix\n");
    return IRQ_HANDLED;
    }
    __pmic8xxx_kp_scan_matrix(kp, new_state, kp.stuckstate);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn pmic8xxx_kp_irq(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t pmic8xxx_kp_irq(int irq, void *data)
    {
    struct pmic8xxx_kp *kp = data;
    unsigned int ctrl_val, events;
    int rc;
    rc = regmap_read(kp.regmap, KEYP_CTRL, &ctrl_val);
    if (rc < 0) {
    dev_err(kp.dev, "failed to read keyp_ctrl register\n");
    return IRQ_HANDLED;
    }
    events = ctrl_val & KEYP_CTRL_EVNTS_MASK;
    rc = pmic8xxx_kp_scan_matrix(kp, events);
    if (rc < 0)
    dev_err(kp.dev, "failed to scan matrix\n");
    return IRQ_HANDLED;
    }
    static int pmic8xxx_kpd_init(struct pmic8xxx_kp *kp,
    struct platform_device *pdev)
    {
    const struct device_node *of_node = pdev.dev.of_node;
    unsigned int scan_delay_ms;
    unsigned int row_hold_ns;
    unsigned int debounce_ms;
    int bits, rc, cycles;
    let mut scan_val: u8 = 0, ctrl_val = 0;
    static const u8 row_bits[] = {
    0, 1, 2, 3, 4, 4, 5, 5, 6, 6, 6, 7, 7, 7,
    };
// Find column bits
    if (kp.num_cols < KEYP_CTRL_SCAN_COLS_MIN)
    bits = 0;
    else
    bits = kp.num_cols - KEYP_CTRL_SCAN_COLS_MIN;
    ctrl_val = (bits & KEYP_CTRL_SCAN_COLS_BITS) <<
    KEYP_CTRL_SCAN_COLS_SHIFT;
// Find row bits
    if (kp.num_rows < KEYP_CTRL_SCAN_ROWS_MIN)
    bits = 0;
    else
    bits = row_bits[kp.num_rows - KEYP_CTRL_SCAN_ROWS_MIN];
    ctrl_val |= (bits << KEYP_CTRL_SCAN_ROWS_SHIFT);
    rc = regmap_write(kp.regmap, KEYP_CTRL, ctrl_val);
    if (rc < 0) {
    dev_err(kp.dev, "Error writing KEYP_CTRL reg, rc=%d\n", rc);
    return rc;
    }
    if (of_property_read_u32(of_node, "scan-delay", &scan_delay_ms))
    scan_delay_ms = MIN_SCAN_DELAY;
    if (scan_delay_ms > MAX_SCAN_DELAY || scan_delay_ms < MIN_SCAN_DELAY ||
    !is_power_of_2(scan_delay_ms)) {
    dev_err(&pdev.dev, "invalid keypad scan time supplied\n");
    return -EINVAL;
    }
    if (of_property_read_u32(of_node, "row-hold", &row_hold_ns))
    row_hold_ns = MIN_ROW_HOLD_DELAY;
    if (row_hold_ns > MAX_ROW_HOLD_DELAY ||
    row_hold_ns < MIN_ROW_HOLD_DELAY ||
    ((row_hold_ns % MIN_ROW_HOLD_DELAY) != 0)) {
    dev_err(&pdev.dev, "invalid keypad row hold time supplied\n");
    return -EINVAL;
    }
    if (of_property_read_u32(of_node, "debounce", &debounce_ms))
    debounce_ms = MIN_DEBOUNCE_TIME;
    if (((debounce_ms % 5) != 0) ||
    debounce_ms > MAX_DEBOUNCE_TIME ||
    debounce_ms < MIN_DEBOUNCE_TIME) {
    dev_err(&pdev.dev, "invalid debounce time supplied\n");
    return -EINVAL;
    }
    bits = (debounce_ms / 5) - 1;
    scan_val |= (bits << KEYP_SCAN_DBOUNCE_SHIFT);
    bits = fls(scan_delay_ms) - 1;
    scan_val |= (bits << KEYP_SCAN_PAUSE_SHIFT);
// Row hold time is a multiple of 32KHz cycles.
    cycles = (row_hold_ns * KEYP_CLOCK_FREQ) / NSEC_PER_SEC;
    scan_val |= (cycles << KEYP_SCAN_ROW_HOLD_SHIFT);
    rc = regmap_write(kp.regmap, KEYP_SCAN, scan_val);
    if (rc)
    dev_err(kp.dev, "Error writing KEYP_SCAN reg, rc=%d\n", rc);
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn pmic8xxx_kp_enable(kp: *mut pmic8xxx_kp) -> c_int {
    static int pmic8xxx_kp_enable(struct pmic8xxx_kp *kp)
    {
    int rc;
    kp.ctrl_reg |= KEYP_CTRL_KEYP_EN;
    rc = regmap_write(kp.regmap, KEYP_CTRL, kp.ctrl_reg);
    if (rc < 0)
    dev_err(kp.dev, "Error writing KEYP_CTRL reg, rc=%d\n", rc);
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn pmic8xxx_kp_disable(kp: *mut pmic8xxx_kp) -> c_int {
    static int pmic8xxx_kp_disable(struct pmic8xxx_kp *kp)
    {
    kp.ctrl_reg &= ~KEYP_CTRL_KEYP_EN;
    return regmap_write(kp.regmap, KEYP_CTRL, kp.ctrl_reg);
    }
#[no_mangle]
unsafe extern "C" fn pmic8xxx_kp_open(dev: *mut input_dev) -> c_int {
    static int pmic8xxx_kp_open(struct input_dev *dev)
    {
    struct pmic8xxx_kp *kp = input_get_drvdata(dev);
    return pmic8xxx_kp_enable(kp);
    }
#[no_mangle]
unsafe extern "C" fn pmic8xxx_kp_close(dev: *mut input_dev) {
    static void pmic8xxx_kp_close(struct input_dev *dev)
    {
    struct pmic8xxx_kp *kp = input_get_drvdata(dev);
    pmic8xxx_kp_disable(kp);
    }
//
// keypad controller should be initialized in the following sequence
// only, otherwise it might get into FSM stuck state.
//
// - Initialize keypad control parameters, like no. of rows, columns,
// timing values etc.,
// - configure rows and column gpios pull up/down.
// - set irq edge type.
// - enable the keypad controller.
//
#[no_mangle]
unsafe extern "C" fn pmic8xxx_kp_probe(pdev: *mut platform_device) -> c_int {
    static int pmic8xxx_kp_probe(struct platform_device *pdev)
    {
    struct device_node *np = pdev.dev.of_node;
    unsigned int rows, cols;
    bool repeat;
    bool wakeup;
    struct pmic8xxx_kp *kp;
    int rc;
    unsigned int ctrl_val;
    rc = matrix_keypad_parse_properties(&pdev.dev, &rows, &cols);
    if (rc)
    return rc;
    if (cols > PM8XXX_MAX_COLS || rows > PM8XXX_MAX_ROWS ||
    cols < PM8XXX_MIN_COLS) {
    dev_err(&pdev.dev, "invalid platform data\n");
    return -EINVAL;
    }
    repeat = !of_property_read_bool(np, "linux,input-no-autorepeat");
    wakeup = of_property_read_bool(np, "wakeup-source") ||
// legacy name
    of_property_read_bool(np, "linux,keypad-wakeup");
    kp = devm_kzalloc(&pdev.dev, sizeof(*kp), GFP_KERNEL);
    if (!kp)
    return -ENOMEM;
    kp.regmap = dev_get_regmap(pdev.dev.parent, core::ptr::null_mut());
    if (!kp.regmap)
    return -ENODEV;
    platform_set_drvdata(pdev, kp);
    kp.num_rows	= rows;
    kp.num_cols	= cols;
    kp.dev		= &pdev.dev;
    kp.input = devm_input_allocate_device(&pdev.dev);
    if (!kp.input) {
    dev_err(&pdev.dev, "unable to allocate input device\n");
    return -ENOMEM;
    }
    kp.key_sense_irq = platform_get_irq(pdev, 0);
    if (kp.key_sense_irq < 0)
    return kp.key_sense_irq;
    kp.key_stuck_irq = platform_get_irq(pdev, 1);
    if (kp.key_stuck_irq < 0)
    return kp.key_stuck_irq;
    kp.input.name = "PMIC8XXX keypad";
    kp.input.phys = "pmic8xxx_keypad/input0";
    kp.input.id.bustype	= BUS_I2C;
    kp.input.id.version	= 0x0001;
    kp.input.id.product	= 0x0001;
    kp.input.id.vendor	= 0x0001;
    kp.input.open		= pmic8xxx_kp_open;
    kp.input.close	= pmic8xxx_kp_close;
    rc = matrix_keypad_build_keymap(core::ptr::null_mut(), core::ptr::null_mut(),
    PM8XXX_MAX_ROWS, PM8XXX_MAX_COLS,
    kp.keycodes, kp.input);
    if (rc) {
    dev_err(&pdev.dev, "failed to build keymap\n");
    return rc;
    }
    if (repeat)
    __set_bit(EV_REP, kp.input.evbit);
    input_set_capability(kp.input, EV_MSC, MSC_SCAN);
    input_set_drvdata(kp.input, kp);
// initialize keypad state
    memset(kp.keystate, 0xff, sizeof(kp.keystate));
    memset(kp.stuckstate, 0xff, sizeof(kp.stuckstate));
    rc = pmic8xxx_kpd_init(kp, pdev);
    if (rc < 0) {
    dev_err(&pdev.dev, "unable to initialize keypad controller\n");
    return rc;
    }
    rc = devm_request_any_context_irq(&pdev.dev, kp.key_sense_irq,
    pmic8xxx_kp_irq, IRQF_TRIGGER_RISING, "pmic-keypad",
    kp);
    if (rc < 0) {
    dev_err(&pdev.dev, "failed to request keypad sense irq\n");
    return rc;
    }
    rc = devm_request_any_context_irq(&pdev.dev, kp.key_stuck_irq,
    pmic8xxx_kp_stuck_irq, IRQF_TRIGGER_RISING,
    "pmic-keypad-stuck", kp);
    if (rc < 0) {
    dev_err(&pdev.dev, "failed to request keypad stuck irq\n");
    return rc;
    }
    rc = regmap_read(kp.regmap, KEYP_CTRL, &ctrl_val);
    if (rc < 0) {
    dev_err(&pdev.dev, "failed to read KEYP_CTRL register\n");
    return rc;
    }
    kp.ctrl_reg = ctrl_val;
    rc = input_register_device(kp.input);
    if (rc < 0) {
    dev_err(&pdev.dev, "unable to register keypad input device\n");
    return rc;
    }
    device_init_wakeup(&pdev.dev, wakeup);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pmic8xxx_kp_suspend(dev: *mut device) -> c_int {
    static int pmic8xxx_kp_suspend(struct device *dev)
    {
    struct platform_device *pdev = to_platform_device(dev);
    struct pmic8xxx_kp *kp = platform_get_drvdata(pdev);
    struct input_dev *input_dev = kp.input;
    if (device_may_wakeup(dev)) {
    enable_irq_wake(kp.key_sense_irq);
    } else {
    guard(mutex)(&input_dev.mutex);
    if (input_device_enabled(input_dev))
    pmic8xxx_kp_disable(kp);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pmic8xxx_kp_resume(dev: *mut device) -> c_int {
    static int pmic8xxx_kp_resume(struct device *dev)
    {
    struct platform_device *pdev = to_platform_device(dev);
    struct pmic8xxx_kp *kp = platform_get_drvdata(pdev);
    struct input_dev *input_dev = kp.input;
    if (device_may_wakeup(dev)) {
    disable_irq_wake(kp.key_sense_irq);
    } else {
    guard(mutex)(&input_dev.mutex);
    if (input_device_enabled(input_dev))
    pmic8xxx_kp_enable(kp);
    }
    return 0;
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(pm8xxx_kp_pm_ops,
    pmic8xxx_kp_suspend, pmic8xxx_kp_resume);
    static const struct of_device_id pm8xxx_match_table[] = {
    { .compatible = "qcom,pm8058-keypad" },
    { .compatible = "qcom,pm8921-keypad" },
    { }
    };
    MODULE_DEVICE_TABLE(of, pm8xxx_match_table);
    static struct platform_driver pmic8xxx_kp_driver = {
    .probe		= pmic8xxx_kp_probe,
    .driver		= {
    .name = "pm8xxx-keypad",
    .pm = pm_sleep_ptr(&pm8xxx_kp_pm_ops),
    .of_match_table = pm8xxx_match_table,
    },
    };
    module_platform_driver(pmic8xxx_kp_driver);
    MODULE_LICENSE("GPL v2");
    MODULE_DESCRIPTION("PMIC8XXX keypad driver");
    MODULE_ALIAS("platform:pmic8xxx_keypad");
    MODULE_AUTHOR("Trilok Soni <tsoni@codeaurora.org>");
