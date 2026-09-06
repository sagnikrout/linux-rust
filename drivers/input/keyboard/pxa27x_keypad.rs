//! Automatically rewritten from C to Rust
//! Source: drivers/input/keyboard/pxa27x_keypad.c
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
// linux/drivers/input/keyboard/pxa27x_keypad.c
//
// Driver for the pxa27x matrix keyboard controller.
//
// Created:	Feb 22, 2007
// Author:	Rodolfo Giometti <giometti@linux.it>
//
// Based on a previous implementations by Kevin O'Connor
// <kevin_at_koconnor.net> and Alex Osborne <bobofdoom@gmail.com> and
// on some suggestions by Nicolas Pitre <nico@fluxnic.net>.
//

//
// Keypad Controller registers
//
pub const KPC: c_uint = 0x0000 /* Keypad Control register */;
pub const KPDK: c_uint = 0x0008 /* Keypad Direct Key register */;
pub const KPREC: c_uint = 0x0010 /* Keypad Rotary Encoder register */;
pub const KPMK: c_uint = 0x0018 /* Keypad Matrix Key register */;
pub const KPAS: c_uint = 0x0020 /* Keypad Automatic Scan register */;
// Keypad Automatic Scan Multiple Key Presser register 0-3
pub const KPASMKP0: c_uint = 0x0028;
pub const KPASMKP1: c_uint = 0x0030;
pub const KPASMKP2: c_uint = 0x0038;
pub const KPASMKP3: c_uint = 0x0040;
pub const KPKDI: c_uint = 0x0048;
// bit definitions

pub const MAX_MATRIX_KEY_ROWS: c_int = 8;
pub const MAX_MATRIX_KEY_COLS: c_int = 8;
pub const MAX_DIRECT_KEY_NUM: c_int = 8;
pub const MAX_ROTARY_ENCODERS: c_int = 2;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pxa27x_keypad_rotary {
    pub key_codes: *mut c_ushort,
    pub rel_code: c_int,
    pub enabled: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pxa27x_keypad {
    pub clk: *mut clk,
    pub input_dev: *mut input_dev,
    pub mmio_base: *mut void __iomem,
    pub irq: c_int,
    pub matrix_key_rows: c_uint,
    pub matrix_key_cols: c_uint,
    pub row_shift: c_uint,
    pub direct_key_num: c_uint,
    pub direct_key_mask: c_uint,
    pub direct_key_low_active: bool,
// key debounce interval
    pub debounce_interval: c_uint,
    pub keycodes: [c_ushort; MAX_KEYPAD_KEYS],
// state row bits of each column scan
    pub matrix_key_state: [u32; MAX_MATRIX_KEY_COLS],
    pub direct_key_state: u32,
    pub rotary: [pxa27x_keypad_rotary; MAX_ROTARY_ENCODERS],
}

#[no_mangle]
unsafe extern "C" fn pxa27x_keypad_matrix_key_parse(keypad: *mut pxa27x_keypad) -> c_int {
    static int pxa27x_keypad_matrix_key_parse(struct pxa27x_keypad *keypad)
    {
    struct input_dev *input_dev = keypad.input_dev;
    struct device *dev = input_dev.dev.parent;
    int error;
    error = matrix_keypad_parse_properties(dev, &keypad.matrix_key_rows,
    &keypad.matrix_key_cols);
    if (error)
    return error;
    if (keypad.matrix_key_rows > MAX_MATRIX_KEY_ROWS ||
    keypad.matrix_key_cols > MAX_MATRIX_KEY_COLS) {
    dev_err(dev, "rows or cols exceeds maximum value\n");
    return -EINVAL;
    }
    keypad.row_shift = get_count_order(keypad.matrix_key_cols);
    error = matrix_keypad_build_keymap(core::ptr::null_mut(), core::ptr::null_mut(),
    keypad.matrix_key_rows,
    keypad.matrix_key_cols,
    keypad.keycodes, input_dev);
    if (error)
    return error;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pxa27x_keypad_direct_key_parse(keypad: *mut pxa27x_keypad) -> c_int {
    static int pxa27x_keypad_direct_key_parse(struct pxa27x_keypad *keypad)
    {
    struct input_dev *input_dev = keypad.input_dev;
    struct device *dev = input_dev.dev.parent;
    unsigned short code;
    int count;
    int i;
    int error;
    error = device_property_read_u32(dev, "marvell,direct-key-count",
    &keypad.direct_key_num);
    if (error) {
//
// If do not have marvel,direct-key-count defined,
// it means direct key is not supported.
//
    let mut error: return = = -EINVAL ? 0 : error;
    }
    error = device_property_read_u32(dev, "marvell,direct-key-mask",
    &keypad.direct_key_mask);
    if (error) {
    if (error != -EINVAL)
    return error;
//
// If marvell,direct-key-mask is not defined, driver will use
// a default value based on number of direct keys set up.
// The default value is calculated in pxa27x_keypad_config().
//
    keypad.direct_key_mask = 0;
    }
    keypad.direct_key_low_active =
    device_property_read_bool(dev, "marvell,direct-key-low-active");
    count = device_property_count_u16(dev, "marvell,direct-key-map");
    if (count <= 0 || count > MAX_DIRECT_KEY_NUM)
    return -EINVAL;
    error = device_property_read_u16_array(dev, "marvell,direct-key-map",
    &keypad.keycodes[MAX_MATRIX_KEY_NUM],
    count);
    for (i = 0; i < count; i++) {
    code = keypad.keycodes[MAX_MATRIX_KEY_NUM + i];
    __set_bit(code, input_dev.keybit);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pxa27x_keypad_rotary_parse(keypad: *mut pxa27x_keypad) -> c_int {
    static int pxa27x_keypad_rotary_parse(struct pxa27x_keypad *keypad)
    {
    static const char * const rotaryname[] = { "marvell,rotary0", "marvell,rotary1" };
    struct input_dev *input_dev = keypad.input_dev;
    struct device *dev = input_dev.dev.parent;
    struct pxa27x_keypad_rotary *encoder;
    unsigned int code;
    int i;
    int error;
    error = device_property_read_u32(dev, "marvell,rotary-rel-key", &code);
    if (!error) {
    for (i = 0; i < MAX_ROTARY_ENCODERS; i++, code >>= 16) {
    encoder = &keypad.rotary[i];
    encoder.enabled = true;
    encoder.rel_code = code & 0xffff;
    input_set_capability(input_dev, EV_REL, encoder.rel_code);
    }
    return 0;
    }
    for (i = 0; i < MAX_ROTARY_ENCODERS; i++) {
    encoder = &keypad.rotary[i];
//
// If the prop is not set, it means keypad does not need
// initialize the rotaryX.
//
    if (!device_property_present(dev, rotaryname[i]))
    continue;
    error = device_property_read_u32(dev, rotaryname[i], &code);
    if (error)
    return error;
//
// Not all up/down key code are valid.
// Now we depends on direct-rel-code.
//
    if (!(code & 0xffff) || !(code >> 16))
    return -EINVAL;
    encoder.enabled = true;
    encoder.rel_code = -1;
    encoder.key_codes = &keypad.keycodes[MAX_MATRIX_KEY_NUM + i * 2];
    encoder.key_codes[0] = code & 0xffff;
    encoder.key_codes[1] = code >> 16;
    input_set_capability(input_dev, EV_KEY, encoder.key_codes[0]);
    input_set_capability(input_dev, EV_KEY, encoder.key_codes[1]);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pxa27x_keypad_parse_properties(keypad: *mut pxa27x_keypad) -> c_int {
    static int pxa27x_keypad_parse_properties(struct pxa27x_keypad *keypad)
    {
    struct input_dev *input_dev = keypad.input_dev;
    struct device *dev = input_dev.dev.parent;
    int error;
    error = pxa27x_keypad_matrix_key_parse(keypad);
    if (error) {
    dev_err(dev, "failed to parse matrix key\n");
    return error;
    }
    error = pxa27x_keypad_direct_key_parse(keypad);
    if (error) {
    dev_err(dev, "failed to parse direct key\n");
    return error;
    }
    error = pxa27x_keypad_rotary_parse(keypad);
    if (error) {
    dev_err(dev, "failed to parse rotary key\n");
    return error;
    }
    error = device_property_read_u32(dev, "marvell,debounce-interval",
    &keypad.debounce_interval);
    if (error) {
    dev_err(dev, "failed to parse debounce-interval\n");
    return error;
    }
//
// The keycodes may not only includes matrix key but also the direct
// key or rotary key.
//
    input_dev.keycodemax = ARRAY_SIZE(keypad.keycodes);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pxa27x_keypad_scan_matrix(keypad: *mut pxa27x_keypad) {
    static void pxa27x_keypad_scan_matrix(struct pxa27x_keypad *keypad)
    {
    struct input_dev *input_dev = keypad.input_dev;
    int row, col, num_keys_pressed = 0;
    u32 new_state[MAX_MATRIX_KEY_COLS];
    let mut kpas: u32 = keypad_readl(KPAS);
    num_keys_pressed = KPAS_MUKP(kpas);
    memset(new_state, 0, sizeof(new_state));
    if (num_keys_pressed == 0)
    goto scan;
    if (num_keys_pressed == 1) {
    col = KPAS_CP(kpas);
    row = KPAS_RP(kpas);
// if invalid row/col, treat as no key pressed
    if (col >= keypad.matrix_key_cols ||
    row >= keypad.matrix_key_rows)
    goto scan;
    new_state[col] = BIT(row);
    goto scan;
    }
    if (num_keys_pressed > 1) {
    let mut kpasmkp0: u32 = keypad_readl(KPASMKP0);
    let mut kpasmkp1: u32 = keypad_readl(KPASMKP1);
    let mut kpasmkp2: u32 = keypad_readl(KPASMKP2);
    let mut kpasmkp3: u32 = keypad_readl(KPASMKP3);
    new_state[0] = kpasmkp0 & KPASMKP_MKC_MASK;
    new_state[1] = (kpasmkp0 >> 16) & KPASMKP_MKC_MASK;
    new_state[2] = kpasmkp1 & KPASMKP_MKC_MASK;
    new_state[3] = (kpasmkp1 >> 16) & KPASMKP_MKC_MASK;
    new_state[4] = kpasmkp2 & KPASMKP_MKC_MASK;
    new_state[5] = (kpasmkp2 >> 16) & KPASMKP_MKC_MASK;
    new_state[6] = kpasmkp3 & KPASMKP_MKC_MASK;
    new_state[7] = (kpasmkp3 >> 16) & KPASMKP_MKC_MASK;
    }
    scan:
    for (col = 0; col < keypad.matrix_key_cols; col++) {
    u32 bits_changed;
    int code;
    bits_changed = keypad.matrix_key_state[col] ^ new_state[col];
    if (bits_changed == 0)
    continue;
    for (row = 0; row < keypad.matrix_key_rows; row++) {
    if ((bits_changed & BIT(row)) == 0)
    continue;
    code = MATRIX_SCAN_CODE(row, col, keypad.row_shift);
    input_event(input_dev, EV_MSC, MSC_SCAN, code);
    input_report_key(input_dev, keypad.keycodes[code],
    new_state[col] & BIT(row));
    }
    }
    input_sync(input_dev);
    memcpy(keypad.matrix_key_state, new_state, sizeof(new_state));
    }

#[no_mangle]
pub unsafe extern "C" fn rotary_delta(kprec: u32) -> c_int {
    static inline int rotary_delta(u32 kprec)
    {
    if (kprec & KPREC_OF0)
    return (kprec & 0xff) + 0x7f;
#[no_mangle]
pub unsafe extern "C" fn if(KPREC_UF0: kprec &) -> else {
    else if (kprec & KPREC_UF0)
    return (kprec & 0xff) - 0x7f - 0xff;
    else
    return (kprec & 0xff) - 0x7f;
    }
#[no_mangle]
unsafe extern "C" fn report_rotary_event(keypad: *mut pxa27x_keypad, r: c_int, delta: c_int) {
    static void report_rotary_event(struct pxa27x_keypad *keypad, int r, int delta)
    {
    struct pxa27x_keypad_rotary *encoder = &keypad.rotary[r];
    struct input_dev *dev = keypad.input_dev;
    if (!encoder.enabled || delta == 0)
    return;
    if (encoder.rel_code == -1) {
    let mut idx: c_int = delta > 0 ? 0 : 1;
    let mut code: c_int = MAX_MATRIX_KEY_NUM + 2 * r + idx;
    let mut keycode: c_uchar = encoder.key_codes[idx];
// simulate a press-n-release
    input_event(dev, EV_MSC, MSC_SCAN, code);
    input_report_key(dev, keycode, 1);
    input_sync(dev);
    input_event(dev, EV_MSC, MSC_SCAN, code);
    input_report_key(dev, keycode, 0);
    input_sync(dev);
    } else {
    input_report_rel(dev, encoder.rel_code, delta);
    input_sync(dev);
    }
    }
#[no_mangle]
unsafe extern "C" fn pxa27x_keypad_scan_rotary(keypad: *mut pxa27x_keypad) {
    static void pxa27x_keypad_scan_rotary(struct pxa27x_keypad *keypad)
    {
    u32 kprec;
    int i;
// read and reset to default count value
    kprec = keypad_readl(KPREC);
    keypad_writel(KPREC, DEFAULT_KPREC);
    for (i = 0; i < MAX_ROTARY_ENCODERS; i++) {
    report_rotary_event(keypad, 0, rotary_delta(kprec));
    kprec >>= 16;
    }
    }
#[no_mangle]
unsafe extern "C" fn pxa27x_keypad_scan_direct(keypad: *mut pxa27x_keypad) {
    static void pxa27x_keypad_scan_direct(struct pxa27x_keypad *keypad)
    {
    struct input_dev *input_dev = keypad.input_dev;
    unsigned int new_state;
    u32 kpdk, bits_changed;
    int i;
    kpdk = keypad_readl(KPDK);
    if (keypad.rotary[0].enabled || keypad.rotary[1].enabled)
    pxa27x_keypad_scan_rotary(keypad);
//
// The KPDR_DK only output the key pin level, so it relates to board,
// and low level may be active.
//
    if (keypad.direct_key_low_active)
    new_state = ~KPDK_DK(kpdk) & keypad.direct_key_mask;
    else
    new_state = KPDK_DK(kpdk) & keypad.direct_key_mask;
    bits_changed = keypad.direct_key_state ^ new_state;
    if (bits_changed == 0)
    return;
    for (i = 0; i < keypad.direct_key_num; i++) {
    if (bits_changed & BIT(i)) {
    let mut code: c_int = MAX_MATRIX_KEY_NUM + i;
    input_event(input_dev, EV_MSC, MSC_SCAN, code);
    input_report_key(input_dev, keypad.keycodes[code],
    new_state & BIT(i));
    }
    }
    input_sync(input_dev);
    keypad.direct_key_state = new_state;
    }
#[no_mangle]
unsafe extern "C" fn pxa27x_keypad_irq_handler(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t pxa27x_keypad_irq_handler(int irq, void *dev_id)
    {
    struct pxa27x_keypad *keypad = dev_id;
    let mut kpc: c_ulong = keypad_readl(KPC);
    if (kpc & KPC_DI)
    pxa27x_keypad_scan_direct(keypad);
    if (kpc & KPC_MI)
    pxa27x_keypad_scan_matrix(keypad);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn pxa27x_keypad_config(keypad: *mut pxa27x_keypad) {
    static void pxa27x_keypad_config(struct pxa27x_keypad *keypad)
    {
    let mut mask: c_uint = 0, direct_key_num = 0;
    let mut kpc: c_ulong = 0;
// clear pending interrupt bit
    keypad_readl(KPC);
// enable matrix keys with automatic scan
    if (keypad.matrix_key_rows && keypad.matrix_key_cols) {
    kpc |= KPC_ASACT | KPC_MIE | KPC_ME | KPC_MS_ALL;
    kpc |= KPC_MKRN(keypad.matrix_key_rows) |
    KPC_MKCN(keypad.matrix_key_cols);
    }
// enable rotary key, debounce interval same as direct keys
    if (keypad.rotary[0].enabled) {
    mask |= 0x03;
    direct_key_num = 2;
    kpc |= KPC_REE0;
    }
    if (keypad.rotary[1].enabled) {
    mask |= 0x0c;
    direct_key_num = 4;
    kpc |= KPC_REE1;
    }
    if (keypad.direct_key_num > direct_key_num)
    direct_key_num = keypad.direct_key_num;
//
// Direct keys usage may not start from KP_DKIN0, check the platfrom
// mask data to config the specific.
//
    if (!keypad.direct_key_mask)
    keypad.direct_key_mask = GENMASK(direct_key_num - 1, 0) & ~mask;
// enable direct key
    if (direct_key_num)
    kpc |= KPC_DE | KPC_DIE | KPC_DKN(direct_key_num);
    keypad_writel(KPC, kpc | KPC_RE_ZERO_DEB);
    keypad_writel(KPREC, DEFAULT_KPREC);
    keypad_writel(KPKDI, keypad.debounce_interval);
    }
#[no_mangle]
unsafe extern "C" fn pxa27x_keypad_open(dev: *mut input_dev) -> c_int {
    static int pxa27x_keypad_open(struct input_dev *dev)
    {
    struct pxa27x_keypad *keypad = input_get_drvdata(dev);
    int ret;
// Enable unit clock
    ret = clk_prepare_enable(keypad.clk);
    if (ret)
    return ret;
    pxa27x_keypad_config(keypad);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pxa27x_keypad_close(dev: *mut input_dev) {
    static void pxa27x_keypad_close(struct input_dev *dev)
    {
    struct pxa27x_keypad *keypad = input_get_drvdata(dev);
// Disable clock unit
    clk_disable_unprepare(keypad.clk);
    }
#[no_mangle]
unsafe extern "C" fn pxa27x_keypad_suspend(dev: *mut device) -> c_int {
    static int pxa27x_keypad_suspend(struct device *dev)
    {
    struct platform_device *pdev = to_platform_device(dev);
    struct pxa27x_keypad *keypad = platform_get_drvdata(pdev);
//
// If the keypad is used a wake up source, clock can not be disabled.
// Or it can not detect the key pressing.
//
    if (device_may_wakeup(&pdev.dev))
    enable_irq_wake(keypad.irq);
    else
    clk_disable_unprepare(keypad.clk);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pxa27x_keypad_resume(dev: *mut device) -> c_int {
    static int pxa27x_keypad_resume(struct device *dev)
    {
    struct platform_device *pdev = to_platform_device(dev);
    struct pxa27x_keypad *keypad = platform_get_drvdata(pdev);
    struct input_dev *input_dev = keypad.input_dev;
    int error;
//
// If the keypad is used as wake up source, the clock is not turned
// off. So do not need configure it again.
//
    if (device_may_wakeup(&pdev.dev)) {
    disable_irq_wake(keypad.irq);
    } else {
    guard(mutex)(&input_dev.mutex);
    if (input_device_enabled(input_dev)) {
// Enable unit clock
    error = clk_prepare_enable(keypad.clk);
    if (error)
    return error;
    pxa27x_keypad_config(keypad);
    }
    }
    return 0;
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(pxa27x_keypad_pm_ops,
    pxa27x_keypad_suspend, pxa27x_keypad_resume);
#[no_mangle]
unsafe extern "C" fn pxa27x_keypad_probe(pdev: *mut platform_device) -> c_int {
    static int pxa27x_keypad_probe(struct platform_device *pdev)
    {
    struct pxa27x_keypad *keypad;
    struct input_dev *input_dev;
    int irq;
    int error;
    irq = platform_get_irq(pdev, 0);
    if (irq < 0)
    return -ENXIO;
    keypad = devm_kzalloc(&pdev.dev, sizeof(*keypad),
    GFP_KERNEL);
    if (!keypad)
    return -ENOMEM;
    input_dev = devm_input_allocate_device(&pdev.dev);
    if (!input_dev)
    return -ENOMEM;
    keypad.input_dev = input_dev;
    keypad.irq = irq;
    keypad.mmio_base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(keypad.mmio_base))
    return PTR_ERR(keypad.mmio_base);
    keypad.clk = devm_clk_get(&pdev.dev, core::ptr::null_mut());
    if (IS_ERR(keypad.clk)) {
    dev_err(&pdev.dev, "failed to get keypad clock\n");
    return PTR_ERR(keypad.clk);
    }
    input_dev.name = pdev.name;
    input_dev.id.bustype = BUS_HOST;
    input_dev.open = pxa27x_keypad_open;
    input_dev.close = pxa27x_keypad_close;
    input_dev.dev.parent = &pdev.dev;
    input_dev.keycode = keypad.keycodes;
    input_dev.keycodesize = sizeof(keypad.keycodes[0]);
    input_dev.keycodemax = ARRAY_SIZE(keypad.keycodes);
    input_set_drvdata(input_dev, keypad);
    input_dev.evbit[0] = BIT_MASK(EV_KEY) | BIT_MASK(EV_REP);
    input_set_capability(input_dev, EV_MSC, MSC_SCAN);
    error = pxa27x_keypad_parse_properties(keypad);
    if (error) {
    dev_err(&pdev.dev, "failed to parse keypad properties\n");
    return error;
    }
    error = devm_request_irq(&pdev.dev, irq, pxa27x_keypad_irq_handler,
    0, pdev.name, keypad);
    if (error) {
    dev_err(&pdev.dev, "failed to request IRQ\n");
    return error;
    }
// Register the input device
    error = input_register_device(input_dev);
    if (error) {
    dev_err(&pdev.dev, "failed to register input device\n");
    return error;
    }
    platform_set_drvdata(pdev, keypad);
    device_init_wakeup(&pdev.dev, 1);
    return 0;
    }

    static const struct of_device_id pxa27x_keypad_dt_match[] = {
    { .compatible = "marvell,pxa27x-keypad" },
    {},
    };
    MODULE_DEVICE_TABLE(of, pxa27x_keypad_dt_match);

    static struct platform_driver pxa27x_keypad_driver = {
    .probe		= pxa27x_keypad_probe,
    .driver		= {
    .name	= "pxa27x-keypad",
    .of_match_table = of_match_ptr(pxa27x_keypad_dt_match),
    .pm	= pm_sleep_ptr(&pxa27x_keypad_pm_ops),
    },
    };
    module_platform_driver(pxa27x_keypad_driver);
    MODULE_DESCRIPTION("PXA27x Keypad Controller Driver");
    MODULE_LICENSE("GPL");
// work with hotplug and coldplug
    MODULE_ALIAS("platform:pxa27x-keypad");
