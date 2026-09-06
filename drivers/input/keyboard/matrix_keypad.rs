//! Automatically rewritten from C to Rust
//! Source: drivers/input/keyboard/matrix_keypad.c
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
//
// GPIO driven matrix keyboard driver
//
// Copyright (c) 2008 Marek Vasut <marek.vasut@gmail.com>
//
// Based on corgikbd.c
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct matrix_keypad {
    pub input_dev: *mut input_dev,
    pub row_shift: c_uint,
    pub col_scan_delay_us: c_uint,
    pub all_cols_on_delay_us: c_uint,
// key debounce interval in milli-second
    pub debounce_ms: c_uint,
    pub drive_inactive_cols: bool,
    pub row_gpios: [*mut gpio_desc; MATRIX_MAX_ROWS],
    pub num_row_gpios: c_uint,
    pub col_gpios: [*mut gpio_desc; MATRIX_MAX_ROWS],
    pub num_col_gpios: c_uint,
    pub row_irqs: [c_uint; MATRIX_MAX_ROWS],
    pub MATRIX_MAX_ROWS): DECLARE_BITMAP(wakeup_enabled_irqs,,
    pub last_key_state: [u32; MATRIX_MAX_COLS],
    pub work: delayed_work,
    pub lock: spinlock_t,
    pub scan_pending: bool,
    pub stopped: bool,
}

//
// NOTE: If drive_inactive_cols is false, then the GPIO has to be put into
// HiZ when de-activated to cause minmal side effect when scanning other
// columns. In that case it is configured here to be input, otherwise it is
// driven with the inactive value.
//
#[no_mangle]
unsafe extern "C" fn __activate_col(keypad: *mut matrix_keypad, col: c_int, on: bool) {
    static void __activate_col(struct matrix_keypad *keypad, int col, bool on)
    {
    if (on) {
    gpiod_direction_output(keypad.col_gpios[col], 1);
    } else {
    gpiod_set_value_cansleep(keypad.col_gpios[col], 0);
    if (!keypad.drive_inactive_cols)
    gpiod_direction_input(keypad.col_gpios[col]);
    }
    }
#[no_mangle]
unsafe extern "C" fn activate_col(keypad: *mut matrix_keypad, col: c_int, on: bool) {
    static void activate_col(struct matrix_keypad *keypad, int col, bool on)
    {
    __activate_col(keypad, col, on);
    if (on && keypad.col_scan_delay_us)
    fsleep(keypad.col_scan_delay_us);
    }
#[no_mangle]
unsafe extern "C" fn activate_all_cols(keypad: *mut matrix_keypad, on: bool) {
    static void activate_all_cols(struct matrix_keypad *keypad, bool on)
    {
    int col;
    for (col = 0; col < keypad.num_col_gpios; col++)
    __activate_col(keypad, col, on);
    if (on && keypad.all_cols_on_delay_us)
    fsleep(keypad.all_cols_on_delay_us);
    }
#[no_mangle]
unsafe extern "C" fn row_asserted(keypad: *mut matrix_keypad, row: c_int) -> bool {
    static bool row_asserted(struct matrix_keypad *keypad, int row)
    {
    return gpiod_get_value_cansleep(keypad.row_gpios[row]);
    }
#[no_mangle]
unsafe extern "C" fn enable_row_irqs(keypad: *mut matrix_keypad) {
    static void enable_row_irqs(struct matrix_keypad *keypad)
    {
    int i;
    for (i = 0; i < keypad.num_row_gpios; i++)
    enable_irq(keypad.row_irqs[i]);
    }
#[no_mangle]
unsafe extern "C" fn disable_row_irqs(keypad: *mut matrix_keypad) {
    static void disable_row_irqs(struct matrix_keypad *keypad)
    {
    int i;
    for (i = 0; i < keypad.num_row_gpios; i++)
    disable_irq_nosync(keypad.row_irqs[i]);
    }
#[no_mangle]
unsafe extern "C" fn read_row_state(keypad: *mut matrix_keypad) -> u32 {
    static uint32_t read_row_state(struct matrix_keypad *keypad)
    {
    int row;
    let mut row_state: u32 = 0;
    for (row = 0; row < keypad.num_row_gpios; row++)
    row_state |= row_asserted(keypad, row) ? BIT(row) : 0;
    return row_state;
    }
//
// This gets the keys from keyboard and reports it to input subsystem
//
#[no_mangle]
unsafe extern "C" fn matrix_keypad_scan(work: *mut work_struct) {
    static void matrix_keypad_scan(struct work_struct *work)
    {
    struct matrix_keypad *keypad =
    container_of(work, struct matrix_keypad, work.work);
    struct input_dev *input_dev = keypad.input_dev;
    const unsigned short *keycodes = input_dev.keycode;
    uint32_t new_state[MATRIX_MAX_COLS];
    int row, col, code;
    u32 init_row_state, new_row_state;
// read initial row state to detect changes between scan
    init_row_state = read_row_state(keypad);
// de-activate all columns for scanning
    activate_all_cols(keypad, false);
    memset(new_state, 0, sizeof(new_state));
    for (row = 0; row < keypad.num_row_gpios; row++)
    gpiod_direction_input(keypad.row_gpios[row]);
// assert each column and read the row status out
    for (col = 0; col < keypad.num_col_gpios; col++) {
    activate_col(keypad, col, true);
    new_state[col] = read_row_state(keypad);
    activate_col(keypad, col, false);
    }
    for (col = 0; col < keypad.num_col_gpios; col++) {
    uint32_t bits_changed;
    bits_changed = keypad.last_key_state[col] ^ new_state[col];
    if (bits_changed == 0)
    continue;
    for (row = 0; row < keypad.num_row_gpios; row++) {
    if (!(bits_changed & BIT(row)))
    continue;
    code = MATRIX_SCAN_CODE(row, col, keypad.row_shift);
    input_event(input_dev, EV_MSC, MSC_SCAN, code);
    input_report_key(input_dev,
    keycodes[code],
    new_state[col] & (1 << row));
    }
    }
    input_sync(input_dev);
    memcpy(keypad.last_key_state, new_state, sizeof(new_state));
    activate_all_cols(keypad, true);
// Enable IRQs again
    scoped_guard(spinlock_irq, &keypad.lock) {
    keypad.scan_pending = false;
    enable_row_irqs(keypad);
    }
// read new row state and detect if value has changed
    new_row_state = read_row_state(keypad);
    if (init_row_state != new_row_state) {
    guard(spinlock_irq)(&keypad.lock);
    if (unlikely(keypad.scan_pending || keypad.stopped))
    return;
    disable_row_irqs(keypad);
    keypad.scan_pending = true;
    schedule_delayed_work(&keypad.work,
    msecs_to_jiffies(keypad.debounce_ms));
    }
    }
#[no_mangle]
unsafe extern "C" fn matrix_keypad_interrupt(irq: c_int, id: *mut c_void) -> irqreturn_t {
    static irqreturn_t matrix_keypad_interrupt(int irq, void *id)
    {
    struct matrix_keypad *keypad = id;
    guard(spinlock_irqsave)(&keypad.lock);
//
// See if another IRQ beaten us to it and scheduled the
// scan already. In that case we should not try to
// disable IRQs again.
//
    if (unlikely(keypad.scan_pending || keypad.stopped))
    goto out;
    disable_row_irqs(keypad);
    keypad.scan_pending = true;
    schedule_delayed_work(&keypad.work,
    msecs_to_jiffies(keypad.debounce_ms));
    out:
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn matrix_keypad_start(dev: *mut input_dev) -> c_int {
    static int matrix_keypad_start(struct input_dev *dev)
    {
    struct matrix_keypad *keypad = input_get_drvdata(dev);
    keypad.stopped = false;
    mb();
//
// Schedule an immediate key scan to capture current key state;
// columns will be activated and IRQs be enabled after the scan.
//
    schedule_delayed_work(&keypad.work, 0);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn matrix_keypad_stop(dev: *mut input_dev) {
    static void matrix_keypad_stop(struct input_dev *dev)
    {
    struct matrix_keypad *keypad = input_get_drvdata(dev);
    scoped_guard(spinlock_irq, &keypad.lock) {
    keypad.stopped = true;
    }
    flush_delayed_work(&keypad.work);
//
// matrix_keypad_scan() will leave IRQs enabled;
// we should disable them now.
//
    disable_row_irqs(keypad);
    }
#[no_mangle]
unsafe extern "C" fn matrix_keypad_enable_wakeup(keypad: *mut matrix_keypad) {
    static void matrix_keypad_enable_wakeup(struct matrix_keypad *keypad)
    {
    int i;
    for_each_clear_bit(i, keypad.wakeup_enabled_irqs,
    keypad.num_row_gpios)
    if (enable_irq_wake(keypad.row_irqs[i]) == 0)
    __set_bit(i, keypad.wakeup_enabled_irqs);
    }
#[no_mangle]
unsafe extern "C" fn matrix_keypad_disable_wakeup(keypad: *mut matrix_keypad) {
    static void matrix_keypad_disable_wakeup(struct matrix_keypad *keypad)
    {
    int i;
    for_each_set_bit(i, keypad.wakeup_enabled_irqs,
    keypad.num_row_gpios) {
    disable_irq_wake(keypad.row_irqs[i]);
    __clear_bit(i, keypad.wakeup_enabled_irqs);
    }
    }
#[no_mangle]
unsafe extern "C" fn matrix_keypad_suspend(dev: *mut device) -> c_int {
    static int matrix_keypad_suspend(struct device *dev)
    {
    struct platform_device *pdev = to_platform_device(dev);
    struct matrix_keypad *keypad = platform_get_drvdata(pdev);
    matrix_keypad_stop(keypad.input_dev);
    if (device_may_wakeup(&pdev.dev))
    matrix_keypad_enable_wakeup(keypad);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn matrix_keypad_resume(dev: *mut device) -> c_int {
    static int matrix_keypad_resume(struct device *dev)
    {
    struct platform_device *pdev = to_platform_device(dev);
    struct matrix_keypad *keypad = platform_get_drvdata(pdev);
    if (device_may_wakeup(&pdev.dev))
    matrix_keypad_disable_wakeup(keypad);
    matrix_keypad_start(keypad.input_dev);
    return 0;
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(matrix_keypad_pm_ops,
    matrix_keypad_suspend, matrix_keypad_resume);
    static int matrix_keypad_init_gpio(struct platform_device *pdev,
    struct matrix_keypad *keypad)
    {
    bool active_low;
    int nrow, ncol;
    int err;
    int i;
    nrow = gpiod_count(&pdev.dev, "row");
    ncol = gpiod_count(&pdev.dev, "col");
    if (nrow < 0 || ncol < 0) {
    dev_err(&pdev.dev, "missing row or column GPIOs\n");
    return -EINVAL;
    }
    keypad.num_row_gpios = nrow;
    keypad.num_col_gpios = ncol;
    active_low = device_property_read_bool(&pdev.dev, "gpio-activelow");
// initialize strobe lines as outputs, activated
    for (i = 0; i < keypad.num_col_gpios; i++) {
    keypad.col_gpios[i] = devm_gpiod_get_index(&pdev.dev, "col",
    i, GPIOD_ASIS);
    err = PTR_ERR_OR_ZERO(keypad.col_gpios[i]);
    if (err) {
    dev_err(&pdev.dev,
    "failed to request GPIO for COL%d: %d\n",
    i, err);
    return err;
    }
    gpiod_set_consumer_name(keypad.col_gpios[i], "matrix_kbd_col");
    if (active_low ^ gpiod_is_active_low(keypad.col_gpios[i]))
    gpiod_toggle_active_low(keypad.col_gpios[i]);
    gpiod_direction_output(keypad.col_gpios[i], 1);
    }
    for (i = 0; i < keypad.num_row_gpios; i++) {
    keypad.row_gpios[i] = devm_gpiod_get_index(&pdev.dev, "row",
    i, GPIOD_IN);
    err = PTR_ERR_OR_ZERO(keypad.row_gpios[i]);
    if (err) {
    dev_err(&pdev.dev,
    "failed to request GPIO for ROW%d: %d\n",
    i, err);
    return err;
    }
    gpiod_set_consumer_name(keypad.row_gpios[i], "matrix_kbd_row");
    if (active_low ^ gpiod_is_active_low(keypad.row_gpios[i]))
    gpiod_toggle_active_low(keypad.row_gpios[i]);
    }
    return 0;
    }
    static int matrix_keypad_setup_interrupts(struct platform_device *pdev,
    struct matrix_keypad *keypad)
    {
    int err;
    int irq;
    int i;
    for (i = 0; i < keypad.num_row_gpios; i++) {
    irq = gpiod_to_irq(keypad.row_gpios[i]);
    if (irq < 0) {
    err = irq;
    dev_err(&pdev.dev,
    "Unable to convert GPIO line %i to irq: %d\n",
    i, err);
    return err;
    }
    err = devm_request_any_context_irq(&pdev.dev, irq,
    matrix_keypad_interrupt,
    IRQF_TRIGGER_RISING |
    IRQF_TRIGGER_FALLING,
    "matrix-keypad", keypad);
    if (err < 0) {
    dev_err(&pdev.dev,
    "Unable to acquire interrupt for row %i: %d\n",
    i, err);
    return err;
    }
    keypad.row_irqs[i] = irq;
    }
// initialized as disabled - enabled by input->open
    disable_row_irqs(keypad);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn matrix_keypad_probe(pdev: *mut platform_device) -> c_int {
    static int matrix_keypad_probe(struct platform_device *pdev)
    {
    struct matrix_keypad *keypad;
    struct input_dev *input_dev;
    bool wakeup;
    int err;
    keypad = devm_kzalloc(&pdev.dev, sizeof(*keypad), GFP_KERNEL);
    if (!keypad)
    return -ENOMEM;
    input_dev = devm_input_allocate_device(&pdev.dev);
    if (!input_dev)
    return -ENOMEM;
    keypad.input_dev = input_dev;
    keypad.stopped = true;
    INIT_DELAYED_WORK(&keypad.work, matrix_keypad_scan);
    spin_lock_init(&keypad.lock);
    keypad.drive_inactive_cols =
    device_property_read_bool(&pdev.dev, "drive-inactive-cols");
    device_property_read_u32(&pdev.dev, "debounce-delay-ms",
    &keypad.debounce_ms);
    device_property_read_u32(&pdev.dev, "col-scan-delay-us",
    &keypad.col_scan_delay_us);
    device_property_read_u32(&pdev.dev, "all-cols-on-delay-us",
    &keypad.all_cols_on_delay_us);
    err = matrix_keypad_init_gpio(pdev, keypad);
    if (err)
    return err;
    keypad.row_shift = get_count_order(keypad.num_col_gpios);
    err = matrix_keypad_setup_interrupts(pdev, keypad);
    if (err)
    return err;
    input_dev.name		= pdev.name;
    input_dev.id.bustype	= BUS_HOST;
    input_dev.open		= matrix_keypad_start;
    input_dev.close	= matrix_keypad_stop;
    err = matrix_keypad_build_keymap(core::ptr::null_mut(), core::ptr::null_mut(),
    keypad.num_row_gpios,
    keypad.num_col_gpios,
    core::ptr::null_mut(), input_dev);
    if (err) {
    dev_err(&pdev.dev, "failed to build keymap\n");
    return -ENOMEM;
    }
    if (!device_property_read_bool(&pdev.dev, "linux,no-autorepeat"))
    __set_bit(EV_REP, input_dev.evbit);
    input_set_capability(input_dev, EV_MSC, MSC_SCAN);
    input_set_drvdata(input_dev, keypad);
    err = input_register_device(keypad.input_dev);
    if (err)
    return err;
    wakeup = device_property_read_bool(&pdev.dev, "wakeup-source") ||
// legacy
    device_property_read_bool(&pdev.dev, "linux,wakeup");
    device_init_wakeup(&pdev.dev, wakeup);
    platform_set_drvdata(pdev, keypad);
    return 0;
    }

    static const struct of_device_id matrix_keypad_dt_match[] = {
    { .compatible = "gpio-matrix-keypad" },
    { }
    };
    MODULE_DEVICE_TABLE(of, matrix_keypad_dt_match);

    static struct platform_driver matrix_keypad_driver = {
    .probe		= matrix_keypad_probe,
    .driver		= {
    .name	= "matrix-keypad",
    .pm	= pm_sleep_ptr(&matrix_keypad_pm_ops),
    .of_match_table = of_match_ptr(matrix_keypad_dt_match),
    },
    };
    module_platform_driver(matrix_keypad_driver);
    MODULE_AUTHOR("Marek Vasut <marek.vasut@gmail.com>");
    MODULE_DESCRIPTION("GPIO Driven Matrix Keypad Driver");
    MODULE_LICENSE("GPL v2");
    MODULE_ALIAS("platform:matrix-keypad");
