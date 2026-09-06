//! Automatically rewritten from C to Rust
//! Source: drivers/input/keyboard/tegra-kbc.c
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
// Keyboard class input driver for the NVIDIA Tegra SoC internal matrix
// keyboard controller
//
// Copyright (c) 2009-2011, NVIDIA Corporation.
//

pub const KBC_MAX_KPENT: c_int = 8;
// Maximum row/column supported by Tegra KBC yet  is 16x8
pub const KBC_MAX_GPIO: c_int = 24;
// Maximum keys supported by Tegra KBC yet is 16 x 8

pub const KBC_MAX_DEBOUNCE_CNT: c_uint = 0x3ffu;
// KBC row scan time and delay for beginning the row scan.
pub const KBC_ROW_SCAN_TIME: c_int = 16;
pub const KBC_ROW_SCAN_DLY: c_int = 5;
// KBC uses a 32KHz clock so a cycle = 1/32Khz
pub const KBC_CYCLE_MS: c_int = 32;
// KBC Registers
// KBC Control Register
pub const KBC_CONTROL_0: c_uint = 0x0;

// KBC Interrupt Register
pub const KBC_INT_0: c_uint = 0x4;

pub const KBC_ROW_CFG0_0: c_uint = 0x8;
pub const KBC_COL_CFG0_0: c_uint = 0x18;
pub const KBC_TO_CNT_0: c_uint = 0x24;
pub const KBC_INIT_DLY_0: c_uint = 0x28;
pub const KBC_RPT_DLY_0: c_uint = 0x2c;
pub const KBC_KP_ENT0_0: c_uint = 0x30;
pub const KBC_KP_ENT1_0: c_uint = 0x34;
pub const KBC_ROW0_MASK_0: c_uint = 0x38;
pub const KBC_ROW_SHIFT: c_int = 3;
    enum tegra_pin_type {
    PIN_CFG_IGNORE,
    PIN_CFG_COL,
    PIN_CFG_ROW,
    };
// Tegra KBC hw support
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_kbc_hw_support {
    pub max_rows: c_int,
    pub max_columns: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_kbc_pin_cfg {
    pub type: enum tegra_pin_type,
    pub num: c_uchar,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_kbc {
    pub dev: *mut device,
    pub debounce_cnt: c_uint,
    pub repeat_cnt: c_uint,
    pub pin_cfg: [tegra_kbc_pin_cfg; KBC_MAX_GPIO],
    pub keymap_data: *const matrix_keymap_data,
    pub wakeup: bool,
    pub mmio: *mut void __iomem,
    pub idev: *mut input_dev,
    pub irq: c_int,
    pub lock: spinlock_t,
    pub repoll_dly: c_uint,
    pub cp_dly_jiffies: c_ulong,
    pub cp_to_wkup_dly: c_uint,
    pub use_fn_map: bool,
    pub use_ghost_filter: bool,
    pub keypress_caused_wake: bool,
    pub 2]: *mut *mut unsigned short keycode[KBC_MAX_KEY,
    pub current_keys: [c_ushort; KBC_MAX_KPENT],
    pub num_pressed_keys: c_uint,
    pub wakeup_key: u32,
    pub timer: timer_list,
    pub clk: *mut clk,
    pub rst: *mut reset_control,
    pub hw_support: *const tegra_kbc_hw_support,
    pub max_keys: c_int,
    pub num_rows_and_columns: c_int,
}

    static void tegra_kbc_report_released_keys(struct input_dev *input,
    unsigned short old_keycodes[],
    unsigned int old_num_keys,
    unsigned short new_keycodes[],
    unsigned int new_num_keys)
    {
    unsigned int i, j;
    for (i = 0; i < old_num_keys; i++) {
    for (j = 0; j < new_num_keys; j++)
    if (old_keycodes[i] == new_keycodes[j])
    break;
    if (j == new_num_keys)
    input_report_key(input, old_keycodes[i], 0);
    }
    }
    static void tegra_kbc_report_pressed_keys(struct input_dev *input,
    unsigned char scancodes[],
    unsigned short keycodes[],
    unsigned int num_pressed_keys)
    {
    unsigned int i;
    for (i = 0; i < num_pressed_keys; i++) {
    input_event(input, EV_MSC, MSC_SCAN, scancodes[i]);
    input_report_key(input, keycodes[i], 1);
    }
    }
#[no_mangle]
unsafe extern "C" fn tegra_kbc_report_keys(kbc: *mut tegra_kbc) {
    static void tegra_kbc_report_keys(struct tegra_kbc *kbc)
    {
    unsigned char scancodes[KBC_MAX_KPENT];
    unsigned short keycodes[KBC_MAX_KPENT];
    let mut val: u32 = 0;
    unsigned int i;
    let mut num_down: c_uint = 0;
    let mut fn_keypress: bool = false;
    let mut key_in_same_row: bool = false;
    let mut key_in_same_col: bool = false;
    for (i = 0; i < KBC_MAX_KPENT; i++) {
    if ((i % 4) == 0)
    val = readl(kbc.mmio + KBC_KP_ENT0_0 + i);
    if (val & 0x80) {
    let mut col: c_uint = val & 0x07;
    let mut row: c_uint = (val >> 3) & 0x0f;
    unsigned char scancode =
    MATRIX_SCAN_CODE(row, col, KBC_ROW_SHIFT);
    scancodes[num_down] = scancode;
    keycodes[num_down] = kbc.keycode[scancode];
// If driver uses Fn map, do not report the Fn key.
    if ((keycodes[num_down] == KEY_FN) && kbc.use_fn_map)
    fn_keypress = true;
    else
    num_down++;
    }
    val >>= 8;
    }
//
// Matrix keyboard designs are prone to keyboard ghosting.
// Ghosting occurs if there are 3 keys such that -
// any 2 of the 3 keys share a row, and any 2 of them share a column.
// If so ignore the key presses for this iteration.
//
    if (kbc.use_ghost_filter && num_down >= 3) {
    for (i = 0; i < num_down; i++) {
    unsigned int j;
    let mut curr_col: u8 = scancodes[i] & 0x07;
    let mut curr_row: u8 = scancodes[i] >> KBC_ROW_SHIFT;
//
// Find 2 keys such that one key is in the same row
// and the other is in the same column as the i-th key.
//
    for (j = i + 1; j < num_down; j++) {
    let mut col: u8 = scancodes[j] & 0x07;
    let mut row: u8 = scancodes[j] >> KBC_ROW_SHIFT;
    if (col == curr_col)
    key_in_same_col = true;
    if (row == curr_row)
    key_in_same_row = true;
    }
    }
    }
//
// If the platform uses Fn keymaps, translate keys on a Fn keypress.
// Function keycodes are max_keys apart from the plain keycodes.
//
    if (fn_keypress) {
    for (i = 0; i < num_down; i++) {
    scancodes[i] += kbc.max_keys;
    keycodes[i] = kbc.keycode[scancodes[i]];
    }
    }
// Ignore the key presses for this iteration?
    if (key_in_same_col && key_in_same_row)
    return;
    tegra_kbc_report_released_keys(kbc.idev,
    kbc.current_keys, kbc.num_pressed_keys,
    keycodes, num_down);
    tegra_kbc_report_pressed_keys(kbc.idev, scancodes, keycodes, num_down);
    input_sync(kbc.idev);
    memcpy(kbc.current_keys, keycodes, sizeof(kbc.current_keys));
    kbc.num_pressed_keys = num_down;
    }
#[no_mangle]
unsafe extern "C" fn tegra_kbc_set_fifo_interrupt(kbc: *mut tegra_kbc, enable: bool) {
    static void tegra_kbc_set_fifo_interrupt(struct tegra_kbc *kbc, bool enable)
    {
    u32 val;
    val = readl(kbc.mmio + KBC_CONTROL_0);
    if (enable)
    val |= KBC_CONTROL_FIFO_CNT_INT_EN;
    else
    val &= ~KBC_CONTROL_FIFO_CNT_INT_EN;
    writel(val, kbc.mmio + KBC_CONTROL_0);
    }
#[no_mangle]
unsafe extern "C" fn tegra_kbc_keypress_timer(t: *mut timer_list) {
    static void tegra_kbc_keypress_timer(struct timer_list *t)
    {
    struct tegra_kbc *kbc = timer_container_of(kbc, t, timer);
    u32 val;
    unsigned int i;
    guard(spinlock_irqsave)(&kbc.lock);
    val = (readl(kbc.mmio + KBC_INT_0) >> 4) & 0xf;
    if (val) {
    unsigned long dly;
    tegra_kbc_report_keys(kbc);
//
// If more than one keys are pressed we need not wait
// for the repoll delay.
//
    dly = (val == 1) ? kbc.repoll_dly : 1;
    mod_timer(&kbc.timer, jiffies + msecs_to_jiffies(dly));
    } else {
// Release any pressed keys and exit the polling loop
    for (i = 0; i < kbc.num_pressed_keys; i++)
    input_report_key(kbc.idev, kbc.current_keys[i], 0);
    input_sync(kbc.idev);
    kbc.num_pressed_keys = 0;
// All keys are released so enable the keypress interrupt
    tegra_kbc_set_fifo_interrupt(kbc, true);
    }
    }
#[no_mangle]
unsafe extern "C" fn tegra_kbc_isr(irq: c_int, args: *mut c_void) -> irqreturn_t {
    static irqreturn_t tegra_kbc_isr(int irq, void *args)
    {
    struct tegra_kbc *kbc = args;
    u32 val;
    guard(spinlock_irqsave)(&kbc.lock);
//
// Quickly bail out & reenable interrupts if the fifo threshold
// count interrupt wasn't the interrupt source
//
    val = readl(kbc.mmio + KBC_INT_0);
    writel(val, kbc.mmio + KBC_INT_0);
    if (val & KBC_INT_FIFO_CNT_INT_STATUS) {
//
// Until all keys are released, defer further processing to
// the polling loop in tegra_kbc_keypress_timer.
//
    tegra_kbc_set_fifo_interrupt(kbc, false);
    mod_timer(&kbc.timer, jiffies + kbc.cp_dly_jiffies);
    } else if (val & KBC_INT_KEYPRESS_INT_STATUS) {
// We can be here only through system resume path
    kbc.keypress_caused_wake = true;
    }
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn tegra_kbc_setup_wakekeys(kbc: *mut tegra_kbc, filter: bool) {
    static void tegra_kbc_setup_wakekeys(struct tegra_kbc *kbc, bool filter)
    {
    int i;
    unsigned int rst_val;
// Either mask all keys or none.
    rst_val = (filter && !kbc.wakeup) ? ~0 : 0;
    for (i = 0; i < kbc.hw_support.max_rows; i++)
    writel(rst_val, kbc.mmio + KBC_ROW0_MASK_0 + i * 4);
    }
#[no_mangle]
unsafe extern "C" fn tegra_kbc_config_pins(kbc: *mut tegra_kbc) {
    static void tegra_kbc_config_pins(struct tegra_kbc *kbc)
    {
    int i;
    for (i = 0; i < KBC_MAX_GPIO; i++) {
    let mut r_shft: u32 = 5 * (i % 6);
    let mut c_shft: u32 = 4 * (i % 8);
    let mut r_mask: u32 = 0x1f << r_shft;
    let mut c_mask: u32 = 0x0f << c_shft;
    let mut r_offs: u32 = (i / 6) * 4 + KBC_ROW_CFG0_0;
    let mut c_offs: u32 = (i / 8) * 4 + KBC_COL_CFG0_0;
    let mut row_cfg: u32 = readl(kbc.mmio + r_offs);
    let mut col_cfg: u32 = readl(kbc.mmio + c_offs);
    row_cfg &= ~r_mask;
    col_cfg &= ~c_mask;
    switch (kbc.pin_cfg[i].type) {
    case PIN_CFG_ROW:
    row_cfg |= ((kbc.pin_cfg[i].num << 1) | 1) << r_shft;
    break;
    case PIN_CFG_COL:
    col_cfg |= ((kbc.pin_cfg[i].num << 1) | 1) << c_shft;
    break;
    case PIN_CFG_IGNORE:
    break;
    }
    writel(row_cfg, kbc.mmio + r_offs);
    writel(col_cfg, kbc.mmio + c_offs);
    }
    }
#[no_mangle]
unsafe extern "C" fn tegra_kbc_start(kbc: *mut tegra_kbc) -> c_int {
    static int tegra_kbc_start(struct tegra_kbc *kbc)
    {
    unsigned int debounce_cnt;
    let mut val: u32 = 0;
    int ret;
    ret = clk_prepare_enable(kbc.clk);
    if (ret)
    return ret;
// Reset the KBC controller to clear all previous status.
    reset_control_assert(kbc.rst);
    udelay(100);
    reset_control_deassert(kbc.rst);
    udelay(100);
    tegra_kbc_config_pins(kbc);
    tegra_kbc_setup_wakekeys(kbc, false);
    writel(kbc.repeat_cnt, kbc.mmio + KBC_RPT_DLY_0);
// Keyboard debounce count is maximum of 12 bits.
    debounce_cnt = min(kbc.debounce_cnt, KBC_MAX_DEBOUNCE_CNT);
    val = KBC_DEBOUNCE_CNT_SHIFT(debounce_cnt);
    val |= KBC_FIFO_TH_CNT_SHIFT(1); /* set fifo interrupt threshold to 1 */
    val |= KBC_CONTROL_FIFO_CNT_INT_EN;  /* interrupt on FIFO threshold */
    val |= KBC_CONTROL_KBC_EN;     /* enable */
    writel(val, kbc.mmio + KBC_CONTROL_0);
//
// Compute the delay(ns) from interrupt mode to continuous polling
// mode so the timer routine is scheduled appropriately.
//
    val = readl(kbc.mmio + KBC_INIT_DLY_0);
    kbc.cp_dly_jiffies = usecs_to_jiffies((val & 0xfffff) * 32);
    kbc.num_pressed_keys = 0;
//
// Atomically clear out any remaining entries in the key FIFO
// and enable keyboard interrupts.
//
    while (1) {
    val = readl(kbc.mmio + KBC_INT_0);
    val >>= 4;
    if (!val)
    break;
    val = readl(kbc.mmio + KBC_KP_ENT0_0);
    val = readl(kbc.mmio + KBC_KP_ENT1_0);
    }
    writel(0x7, kbc.mmio + KBC_INT_0);
    enable_irq(kbc.irq);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tegra_kbc_stop(kbc: *mut tegra_kbc) {
    static void tegra_kbc_stop(struct tegra_kbc *kbc)
    {
    u32 val;
    scoped_guard(spinlock_irqsave, &kbc.lock) {
    val = readl(kbc.mmio + KBC_CONTROL_0);
    val &= ~1;
    writel(val, kbc.mmio + KBC_CONTROL_0);
    }
    disable_irq(kbc.irq);
    timer_delete_sync(&kbc.timer);
    clk_disable_unprepare(kbc.clk);
    }
#[no_mangle]
unsafe extern "C" fn tegra_kbc_open(dev: *mut input_dev) -> c_int {
    static int tegra_kbc_open(struct input_dev *dev)
    {
    struct tegra_kbc *kbc = input_get_drvdata(dev);
    return tegra_kbc_start(kbc);
    }
#[no_mangle]
unsafe extern "C" fn tegra_kbc_close(dev: *mut input_dev) {
    static void tegra_kbc_close(struct input_dev *dev)
    {
    struct tegra_kbc *kbc = input_get_drvdata(dev);
    return tegra_kbc_stop(kbc);
    }
    static bool tegra_kbc_check_pin_cfg(const struct tegra_kbc *kbc,
    unsigned int *num_rows)
    {
    int i;
// num_rows = 0;
    for (i = 0; i < KBC_MAX_GPIO; i++) {
    const struct tegra_kbc_pin_cfg *pin_cfg = &kbc.pin_cfg[i];
    switch (pin_cfg.type) {
    case PIN_CFG_ROW:
    if (pin_cfg.num >= kbc.hw_support.max_rows) {
    dev_err(kbc.dev,
    "pin_cfg[%d]: invalid row number %d\n",
    i, pin_cfg.num);
    return false;
    }
    (*num_rows)++;
    break;
    case PIN_CFG_COL:
    if (pin_cfg.num >= kbc.hw_support.max_columns) {
    dev_err(kbc.dev,
    "pin_cfg[%d]: invalid column number %d\n",
    i, pin_cfg.num);
    return false;
    }
    break;
    case PIN_CFG_IGNORE:
    break;
    default:
    dev_err(kbc.dev,
    "pin_cfg[%d]: invalid entry type %d\n",
    pin_cfg.type, pin_cfg.num);
    return false;
    }
    }
    return true;
    }
#[no_mangle]
unsafe extern "C" fn tegra_kbc_parse_dt(kbc: *mut tegra_kbc) -> c_int {
    static int tegra_kbc_parse_dt(struct tegra_kbc *kbc)
    {
    struct device_node *np = kbc.dev.of_node;
    u32 prop;
    int i;
    int num_rows;
    int num_cols;
    u32 cols_cfg[KBC_MAX_GPIO];
    u32 rows_cfg[KBC_MAX_GPIO];
    if (!of_property_read_u32(np, "nvidia,debounce-delay-ms", &prop))
    kbc.debounce_cnt = prop;
    if (!of_property_read_u32(np, "nvidia,repeat-delay-ms", &prop))
    kbc.repeat_cnt = prop;
    kbc.use_ghost_filter = of_property_present(np, "nvidia,needs-ghost-filter");
    if (of_property_read_bool(np, "wakeup-source") ||
    of_property_read_bool(np, "nvidia,wakeup-source")) /* legacy */
    kbc.wakeup = true;
    if (!of_property_present(np, "linux,keymap")) {
    dev_err(kbc.dev, "property linux,keymap not found\n");
    return -ENOENT;
    }
// Set all pins as non-configured
    for (i = 0; i < kbc.num_rows_and_columns; i++)
    kbc.pin_cfg[i].type = PIN_CFG_IGNORE;
    num_rows = of_property_read_variable_u32_array(np, "nvidia,kbc-row-pins",
    rows_cfg, 1, KBC_MAX_GPIO);
    if (num_rows < 0) {
    dev_err(kbc.dev, "Rows configurations are not proper\n");
    return num_rows;
    } else if (num_rows > kbc.hw_support.max_rows) {
    dev_err(kbc.dev,
    "Number of rows is more than supported by hardware\n");
    return -EINVAL;
    }
    for (i = 0; i < num_rows; i++) {
    kbc.pin_cfg[rows_cfg[i]].type = PIN_CFG_ROW;
    kbc.pin_cfg[rows_cfg[i]].num = i;
    }
    num_cols = of_property_read_variable_u32_array(np, "nvidia,kbc-col-pins",
    cols_cfg, 1, KBC_MAX_GPIO);
    if (num_cols < 0) {
    dev_err(kbc.dev, "Cols configurations are not proper\n");
    return num_cols;
    } else if (num_cols > kbc.hw_support.max_columns) {
    dev_err(kbc.dev,
    "Number of cols is more than supported by hardware\n");
    return -EINVAL;
    }
    for (i = 0; i < num_cols; i++) {
    kbc.pin_cfg[cols_cfg[i]].type = PIN_CFG_COL;
    kbc.pin_cfg[cols_cfg[i]].num = i;
    }
    if (!num_rows || !num_cols || ((num_rows + num_cols) > KBC_MAX_GPIO)) {
    dev_err(kbc.dev,
    "keypad rows/columns not properly specified\n");
    return -EINVAL;
    }
    return 0;
    }
    static const struct tegra_kbc_hw_support tegra20_kbc_hw_support = {
    .max_rows	= 16,
    .max_columns	= 8,
    };
    static const struct tegra_kbc_hw_support tegra11_kbc_hw_support = {
    .max_rows	= 11,
    .max_columns	= 8,
    };
    static const struct of_device_id tegra_kbc_of_match[] = {
    { .compatible = "nvidia,tegra114-kbc", .data = &tegra11_kbc_hw_support},
    { .compatible = "nvidia,tegra30-kbc", .data = &tegra20_kbc_hw_support},
    { .compatible = "nvidia,tegra20-kbc", .data = &tegra20_kbc_hw_support},
    { },
    };
    MODULE_DEVICE_TABLE(of, tegra_kbc_of_match);
#[no_mangle]
unsafe extern "C" fn tegra_kbc_probe(pdev: *mut platform_device) -> c_int {
    static int tegra_kbc_probe(struct platform_device *pdev)
    {
    struct tegra_kbc *kbc;
    int err;
    let mut num_rows: c_int = 0;
    unsigned int debounce_cnt;
    unsigned int scan_time_rows;
    unsigned int keymap_rows;
    kbc = devm_kzalloc(&pdev.dev, sizeof(*kbc), GFP_KERNEL);
    if (!kbc) {
    dev_err(&pdev.dev, "failed to alloc memory for kbc\n");
    return -ENOMEM;
    }
    kbc.dev = &pdev.dev;
    kbc.hw_support = device_get_match_data(&pdev.dev);
    kbc.max_keys = kbc.hw_support.max_rows *
    kbc.hw_support.max_columns;
    kbc.num_rows_and_columns = kbc.hw_support.max_rows +
    kbc.hw_support.max_columns;
    keymap_rows = kbc.max_keys;
    spin_lock_init(&kbc.lock);
    err = tegra_kbc_parse_dt(kbc);
    if (err)
    return err;
    if (!tegra_kbc_check_pin_cfg(kbc, &num_rows))
    return -EINVAL;
    kbc.irq = platform_get_irq(pdev, 0);
    if (kbc.irq < 0)
    return -ENXIO;
    kbc.idev = devm_input_allocate_device(&pdev.dev);
    if (!kbc.idev) {
    dev_err(&pdev.dev, "failed to allocate input device\n");
    return -ENOMEM;
    }
    timer_setup(&kbc.timer, tegra_kbc_keypress_timer, 0);
    kbc.mmio = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(kbc.mmio))
    return PTR_ERR(kbc.mmio);
    kbc.clk = devm_clk_get(&pdev.dev, core::ptr::null_mut());
    if (IS_ERR(kbc.clk)) {
    dev_err(&pdev.dev, "failed to get keyboard clock\n");
    return PTR_ERR(kbc.clk);
    }
    kbc.rst = devm_reset_control_get(&pdev.dev, "kbc");
    if (IS_ERR(kbc.rst)) {
    dev_err(&pdev.dev, "failed to get keyboard reset\n");
    return PTR_ERR(kbc.rst);
    }
//
// The time delay between two consecutive reads of the FIFO is
// the sum of the repeat time and the time taken for scanning
// the rows. There is an additional delay before the row scanning
// starts. The repoll delay is computed in milliseconds.
//
    debounce_cnt = min(kbc.debounce_cnt, KBC_MAX_DEBOUNCE_CNT);
    scan_time_rows = (KBC_ROW_SCAN_TIME + debounce_cnt) * num_rows;
    kbc.repoll_dly = KBC_ROW_SCAN_DLY + scan_time_rows + kbc.repeat_cnt;
    kbc.repoll_dly = DIV_ROUND_UP(kbc.repoll_dly, KBC_CYCLE_MS);
    kbc.idev.name = pdev.name;
    kbc.idev.id.bustype = BUS_HOST;
    kbc.idev.dev.parent = &pdev.dev;
    kbc.idev.open = tegra_kbc_open;
    kbc.idev.close = tegra_kbc_close;
    if (kbc.keymap_data && kbc.use_fn_map)
    keymap_rows *= 2;
    err = matrix_keypad_build_keymap(kbc.keymap_data, core::ptr::null_mut(),
    keymap_rows,
    kbc.hw_support.max_columns,
    kbc.keycode, kbc.idev);
    if (err) {
    dev_err(&pdev.dev, "failed to setup keymap\n");
    return err;
    }
    __set_bit(EV_REP, kbc.idev.evbit);
    input_set_capability(kbc.idev, EV_MSC, MSC_SCAN);
    input_set_drvdata(kbc.idev, kbc);
    err = devm_request_irq(&pdev.dev, kbc.irq, tegra_kbc_isr,
    IRQF_TRIGGER_HIGH | IRQF_NO_AUTOEN,
    pdev.name, kbc);
    if (err) {
    dev_err(&pdev.dev, "failed to request keyboard IRQ\n");
    return err;
    }
    err = input_register_device(kbc.idev);
    if (err) {
    dev_err(&pdev.dev, "failed to register input device\n");
    return err;
    }
    platform_set_drvdata(pdev, kbc);
    device_init_wakeup(&pdev.dev, kbc.wakeup);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tegra_kbc_set_keypress_interrupt(kbc: *mut tegra_kbc, enable: bool) {
    static void tegra_kbc_set_keypress_interrupt(struct tegra_kbc *kbc, bool enable)
    {
    u32 val;
    val = readl(kbc.mmio + KBC_CONTROL_0);
    if (enable)
    val |= KBC_CONTROL_KEYPRESS_INT_EN;
    else
    val &= ~KBC_CONTROL_KEYPRESS_INT_EN;
    writel(val, kbc.mmio + KBC_CONTROL_0);
    }
#[no_mangle]
unsafe extern "C" fn tegra_kbc_suspend(dev: *mut device) -> c_int {
    static int tegra_kbc_suspend(struct device *dev)
    {
    struct platform_device *pdev = to_platform_device(dev);
    struct tegra_kbc *kbc = platform_get_drvdata(pdev);
    guard(mutex)(&kbc.idev.mutex);
    if (device_may_wakeup(&pdev.dev)) {
    disable_irq(kbc.irq);
    timer_delete_sync(&kbc.timer);
    tegra_kbc_set_fifo_interrupt(kbc, false);
// Forcefully clear the interrupt status
    writel(0x7, kbc.mmio + KBC_INT_0);
//
// Store the previous resident time of continuous polling mode.
// Force the keyboard into interrupt mode.
//
    kbc.cp_to_wkup_dly = readl(kbc.mmio + KBC_TO_CNT_0);
    writel(0, kbc.mmio + KBC_TO_CNT_0);
    tegra_kbc_setup_wakekeys(kbc, true);
    msleep(30);
    kbc.keypress_caused_wake = false;
// Enable keypress interrupt before going into suspend.
    tegra_kbc_set_keypress_interrupt(kbc, true);
    enable_irq(kbc.irq);
    enable_irq_wake(kbc.irq);
    } else if (input_device_enabled(kbc.idev)) {
    tegra_kbc_stop(kbc);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tegra_kbc_resume(dev: *mut device) -> c_int {
    static int tegra_kbc_resume(struct device *dev)
    {
    struct platform_device *pdev = to_platform_device(dev);
    struct tegra_kbc *kbc = platform_get_drvdata(pdev);
    int err;
    guard(mutex)(&kbc.idev.mutex);
    if (device_may_wakeup(&pdev.dev)) {
    disable_irq_wake(kbc.irq);
    tegra_kbc_setup_wakekeys(kbc, false);
// We will use fifo interrupts for key detection.
    tegra_kbc_set_keypress_interrupt(kbc, false);
// Restore the resident time of continuous polling mode.
    writel(kbc.cp_to_wkup_dly, kbc.mmio + KBC_TO_CNT_0);
    tegra_kbc_set_fifo_interrupt(kbc, true);
    if (kbc.keypress_caused_wake && kbc.wakeup_key) {
//
// We can't report events directly from the ISR
// because timekeeping is stopped when processing
// wakeup request and we get a nasty warning when
// we try to call do_gettimeofday() in evdev
// handler.
//
    input_report_key(kbc.idev, kbc.wakeup_key, 1);
    input_sync(kbc.idev);
    input_report_key(kbc.idev, kbc.wakeup_key, 0);
    input_sync(kbc.idev);
    }
    } else if (input_device_enabled(kbc.idev)) {
    err = tegra_kbc_start(kbc);
    if (err)
    return err;
    }
    return 0;
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(tegra_kbc_pm_ops,
    tegra_kbc_suspend, tegra_kbc_resume);
    static struct platform_driver tegra_kbc_driver = {
    .probe		= tegra_kbc_probe,
    .driver	= {
    .name	= "tegra-kbc",
    .pm	= pm_sleep_ptr(&tegra_kbc_pm_ops),
    .of_match_table = tegra_kbc_of_match,
    },
    };
    module_platform_driver(tegra_kbc_driver);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Rakesh Iyer <riyer@nvidia.com>");
    MODULE_DESCRIPTION("Tegra matrix keyboard controller driver");
    MODULE_ALIAS("platform:tegra-kbc");
