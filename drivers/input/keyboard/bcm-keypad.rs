//! Automatically rewritten from C to Rust
//! Source: drivers/input/keyboard/bcm-keypad.c
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
// Copyright (C) 2014 Broadcom Corporation

pub const DEFAULT_CLK_HZ: c_int = 31250;
pub const MAX_ROWS: c_int = 8;
pub const MAX_COLS: c_int = 8;
// Register/field definitions
pub const KPCR_OFFSET: c_uint = 0x00000080;
pub const KPCR_MODE: c_uint = 0x00000002;
pub const KPCR_MODE_SHIFT: c_int = 1;
pub const KPCR_MODE_MASK: c_int = 1;
pub const KPCR_ENABLE: c_uint = 0x00000001;
pub const KPCR_STATUSFILTERENABLE: c_uint = 0x00008000;
pub const KPCR_STATUSFILTERTYPE_SHIFT: c_int = 12;
pub const KPCR_COLFILTERENABLE: c_uint = 0x00000800;
pub const KPCR_COLFILTERTYPE_SHIFT: c_int = 8;
pub const KPCR_ROWWIDTH_SHIFT: c_int = 20;
pub const KPCR_COLUMNWIDTH_SHIFT: c_int = 16;
pub const KPIOR_OFFSET: c_uint = 0x00000084;
pub const KPIOR_ROWOCONTRL_SHIFT: c_int = 24;
pub const KPIOR_ROWOCONTRL_MASK: c_uint = 0xFF000000;
pub const KPIOR_COLUMNOCONTRL_SHIFT: c_int = 16;
pub const KPIOR_COLUMNOCONTRL_MASK: c_uint = 0x00FF0000;
pub const KPIOR_COLUMN_IO_DATA_SHIFT: c_int = 0;
pub const KPEMR0_OFFSET: c_uint = 0x00000090;
pub const KPEMR1_OFFSET: c_uint = 0x00000094;
pub const KPEMR2_OFFSET: c_uint = 0x00000098;
pub const KPEMR3_OFFSET: c_uint = 0x0000009C;
pub const KPEMR_EDGETYPE_BOTH: c_int = 3;
pub const KPSSR0_OFFSET: c_uint = 0x000000A0;
pub const KPSSR1_OFFSET: c_uint = 0x000000A4;

pub const KPIMR0_OFFSET: c_uint = 0x000000B0;
pub const KPIMR1_OFFSET: c_uint = 0x000000B4;
pub const KPICR0_OFFSET: c_uint = 0x000000B8;
pub const KPICR1_OFFSET: c_uint = 0x000000BC;

pub const KPISR0_OFFSET: c_uint = 0x000000C0;
pub const KPISR1_OFFSET: c_uint = 0x000000C4;
pub const KPCR_STATUSFILTERTYPE_MAX: c_int = 7;
pub const KPCR_COLFILTERTYPE_MAX: c_int = 7;
// Macros to determine the row/column from a bit that is set in SSR0/1.

// Structure representing various run-time entities
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcm_kp {
    pub base: *mut void __iomem,
    pub irq: c_int,
    pub clk: *mut clk,
    pub input_dev: *mut input_dev,
    pub last_state: [c_ulong; 2],
    pub n_rows: c_uint,
    pub n_cols: c_uint,
    pub kpcr: u32,
    pub kpior: u32,
    pub kpemr: u32,
    pub imr0_val: u32,
    pub imr1_val: u32,
}

//
// Returns the keycode from the input device keymap given the row and
// column.
//
#[no_mangle]
unsafe extern "C" fn bcm_kp_get_keycode(kp: *mut bcm_kp, row: c_int, col: c_int) -> c_int {
    static int bcm_kp_get_keycode(struct bcm_kp *kp, int row, int col)
    {
    let mut row_shift: c_uint = get_count_order(kp.n_cols);
    unsigned short *keymap = kp.input_dev.keycode;
    return keymap[MATRIX_SCAN_CODE(row, col, row_shift)];
    }
#[no_mangle]
unsafe extern "C" fn bcm_kp_report_keys(kp: *mut bcm_kp, reg_num: c_int, pull_mode: c_int) {
    static void bcm_kp_report_keys(struct bcm_kp *kp, int reg_num, int pull_mode)
    {
    unsigned long state, change;
    int bit_nr;
    int key_press;
    int row, col;
    unsigned int keycode;
// Clear interrupts
    writel(0xFFFFFFFF, kp.base + KPICRN_OFFSET(reg_num));
    state = readl(kp.base + KPSSRN_OFFSET(reg_num));
    change = kp.last_state[reg_num] ^ state;
    kp.last_state[reg_num] = state;
    for_each_set_bit(bit_nr, &change, BITS_PER_LONG) {
    key_press = state & BIT(bit_nr);
// The meaning of SSR register depends on pull mode.
    key_press = pull_mode ? !key_press : key_press;
    row = BIT_TO_ROW_SSRN(bit_nr, reg_num);
    col = BIT_TO_COL(bit_nr);
    keycode = bcm_kp_get_keycode(kp, row, col);
    input_report_key(kp.input_dev, keycode, key_press);
    }
    }
#[no_mangle]
unsafe extern "C" fn bcm_kp_isr_thread(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t bcm_kp_isr_thread(int irq, void *dev_id)
    {
    struct bcm_kp *kp = dev_id;
    let mut pull_mode: c_int = (kp.kpcr >> KPCR_MODE_SHIFT) & KPCR_MODE_MASK;
    int reg_num;
    for (reg_num = 0; reg_num <= 1; reg_num++)
    bcm_kp_report_keys(kp, reg_num, pull_mode);
    input_sync(kp.input_dev);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn bcm_kp_start(kp: *mut bcm_kp) -> c_int {
    static int bcm_kp_start(struct bcm_kp *kp)
    {
    int error;
    if (kp.clk) {
    error = clk_prepare_enable(kp.clk);
    if (error)
    return error;
    }
    writel(kp.kpior, kp.base + KPIOR_OFFSET);
    writel(kp.imr0_val, kp.base + KPIMR0_OFFSET);
    writel(kp.imr1_val, kp.base + KPIMR1_OFFSET);
    writel(kp.kpemr, kp.base + KPEMR0_OFFSET);
    writel(kp.kpemr, kp.base + KPEMR1_OFFSET);
    writel(kp.kpemr, kp.base + KPEMR2_OFFSET);
    writel(kp.kpemr, kp.base + KPEMR3_OFFSET);
    writel(0xFFFFFFFF, kp.base + KPICR0_OFFSET);
    writel(0xFFFFFFFF, kp.base + KPICR1_OFFSET);
    kp.last_state[0] = readl(kp.base + KPSSR0_OFFSET);
    kp.last_state[0] = readl(kp.base + KPSSR1_OFFSET);
    writel(kp.kpcr | KPCR_ENABLE, kp.base + KPCR_OFFSET);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bcm_kp_stop(kp: *const bcm_kp) {
    static void bcm_kp_stop(const struct bcm_kp *kp)
    {
    u32 val;
    val = readl(kp.base + KPCR_OFFSET);
    val &= ~KPCR_ENABLE;
    writel(0, kp.base + KPCR_OFFSET);
    writel(0, kp.base + KPIMR0_OFFSET);
    writel(0, kp.base + KPIMR1_OFFSET);
    writel(0xFFFFFFFF, kp.base + KPICR0_OFFSET);
    writel(0xFFFFFFFF, kp.base + KPICR1_OFFSET);
    clk_disable_unprepare(kp.clk);
    }
#[no_mangle]
unsafe extern "C" fn bcm_kp_open(dev: *mut input_dev) -> c_int {
    static int bcm_kp_open(struct input_dev *dev)
    {
    struct bcm_kp *kp = input_get_drvdata(dev);
    return bcm_kp_start(kp);
    }
#[no_mangle]
unsafe extern "C" fn bcm_kp_close(dev: *mut input_dev) {
    static void bcm_kp_close(struct input_dev *dev)
    {
    struct bcm_kp *kp = input_get_drvdata(dev);
    bcm_kp_stop(kp);
    }
#[no_mangle]
unsafe extern "C" fn bcm_kp_matrix_key_parse_dt(kp: *mut bcm_kp) -> c_int {
    static int bcm_kp_matrix_key_parse_dt(struct bcm_kp *kp)
    {
    struct device *dev = kp.input_dev.dev.parent;
    struct device_node *np = dev.of_node;
    int error;
    unsigned int dt_val;
    unsigned int i;
    unsigned int num_rows, col_mask, rows_set;
// Initialize the KPCR Keypad Configuration Register
    kp.kpcr = KPCR_STATUSFILTERENABLE | KPCR_COLFILTERENABLE;
    error = matrix_keypad_parse_properties(dev, &kp.n_rows, &kp.n_cols);
    if (error) {
    dev_err(dev, "failed to parse kp params\n");
    return error;
    }
// Set row width for the ASIC block.
    kp.kpcr |= (kp.n_rows - 1) << KPCR_ROWWIDTH_SHIFT;
// Set column width for the ASIC block.
    kp.kpcr |= (kp.n_cols - 1) << KPCR_COLUMNWIDTH_SHIFT;
// Configure the IMR registers
//
// IMR registers contain interrupt enable bits for 8x8 matrix
// IMR0 register format: <row3> <row2> <row1> <row0>
// IMR1 register format: <row7> <row6> <row5> <row4>
//
    col_mask = (1 << (kp.n_cols)) - 1;
    num_rows = kp.n_rows;
// Set column bits in rows 0 to 3 in IMR0
    kp.imr0_val = col_mask;
    rows_set = 1;
    while (--num_rows && rows_set++ < 4)
    kp.imr0_val |= kp.imr0_val << MAX_COLS;
// Set column bits in rows 4 to 7 in IMR1
    kp.imr1_val = 0;
    if (num_rows) {
    kp.imr1_val = col_mask;
    while (--num_rows)
    kp.imr1_val |= kp.imr1_val << MAX_COLS;
    }
// Initialize the KPEMR Keypress Edge Mode Registers
// Trigger on both edges
    kp.kpemr = 0;
    for (i = 0; i <= 30; i += 2)
    kp.kpemr |= (KPEMR_EDGETYPE_BOTH << i);
//
// Obtain the Status filter debounce value and verify against the
// possible values specified in the DT binding.
//
    of_property_read_u32(np, "status-debounce-filter-period", &dt_val);
    if (dt_val > KPCR_STATUSFILTERTYPE_MAX) {
    dev_err(dev, "Invalid Status filter debounce value %d\n",
    dt_val);
    return -EINVAL;
    }
    kp.kpcr |= dt_val << KPCR_STATUSFILTERTYPE_SHIFT;
//
// Obtain the Column filter debounce value and verify against the
// possible values specified in the DT binding.
//
    of_property_read_u32(np, "col-debounce-filter-period", &dt_val);
    if (dt_val > KPCR_COLFILTERTYPE_MAX) {
    dev_err(dev, "Invalid Column filter debounce value %d\n",
    dt_val);
    return -EINVAL;
    }
    kp.kpcr |= dt_val << KPCR_COLFILTERTYPE_SHIFT;
//
// Determine between the row and column,
// which should be configured as output.
//
    if (of_property_read_bool(np, "row-output-enabled")) {
//
// Set RowOContrl or ColumnOContrl in KPIOR
// to the number of pins to drive as outputs
//
    kp.kpior = ((1 << kp.n_rows) - 1) <<
    KPIOR_ROWOCONTRL_SHIFT;
    } else {
    kp.kpior = ((1 << kp.n_cols) - 1) <<
    KPIOR_COLUMNOCONTRL_SHIFT;
    }
//
// Determine if the scan pull up needs to be enabled
//
    if (of_property_read_bool(np, "pull-up-enabled"))
    kp.kpcr |= KPCR_MODE;
    dev_dbg(dev, "n_rows=%d n_col=%d kpcr=%x kpior=%x kpemr=%x\n",
    kp.n_rows, kp.n_cols,
    kp.kpcr, kp.kpior, kp.kpemr);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bcm_kp_probe(pdev: *mut platform_device) -> c_int {
    static int bcm_kp_probe(struct platform_device *pdev)
    {
    struct bcm_kp *kp;
    struct input_dev *input_dev;
    int error;
    kp = devm_kzalloc(&pdev.dev, sizeof(*kp), GFP_KERNEL);
    if (!kp)
    return -ENOMEM;
    input_dev = devm_input_allocate_device(&pdev.dev);
    if (!input_dev) {
    dev_err(&pdev.dev, "failed to allocate the input device\n");
    return -ENOMEM;
    }
    __set_bit(EV_KEY, input_dev.evbit);
// Enable auto repeat feature of Linux input subsystem
    if (of_property_read_bool(pdev.dev.of_node, "autorepeat"))
    __set_bit(EV_REP, input_dev.evbit);
    input_dev.name = pdev.name;
    input_dev.phys = "keypad/input0";
    input_dev.dev.parent = &pdev.dev;
    input_dev.open = bcm_kp_open;
    input_dev.close = bcm_kp_close;
    input_dev.id.bustype = BUS_HOST;
    input_dev.id.vendor = 0x0001;
    input_dev.id.product = 0x0001;
    input_dev.id.version = 0x0100;
    input_set_drvdata(input_dev, kp);
    kp.input_dev = input_dev;
    error = bcm_kp_matrix_key_parse_dt(kp);
    if (error)
    return error;
    error = matrix_keypad_build_keymap(core::ptr::null_mut(), core::ptr::null_mut(),
    kp.n_rows, kp.n_cols,
    core::ptr::null_mut(), input_dev);
    if (error) {
    dev_err(&pdev.dev, "failed to build keymap\n");
    return error;
    }
    kp.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(kp.base))
    return PTR_ERR(kp.base);
// Enable clock
    kp.clk = devm_clk_get_optional(&pdev.dev, "peri_clk");
    if (IS_ERR(kp.clk)) {
    return dev_err_probe(&pdev.dev, PTR_ERR(kp.clk), "Failed to get clock\n");
    } else if (!kp.clk) {
    dev_dbg(&pdev.dev, "No clock specified. Assuming it's enabled\n");
    } else {
    unsigned int desired_rate;
    long actual_rate;
    error = of_property_read_u32(pdev.dev.of_node,
    "clock-frequency", &desired_rate);
    if (error < 0)
    desired_rate = DEFAULT_CLK_HZ;
    actual_rate = clk_round_rate(kp.clk, desired_rate);
    if (actual_rate <= 0)
    return -EINVAL;
    error = clk_set_rate(kp.clk, actual_rate);
    if (error)
    return error;
    error = clk_prepare_enable(kp.clk);
    if (error)
    return error;
    }
// Put the kp into a known sane state
    bcm_kp_stop(kp);
    kp.irq = platform_get_irq(pdev, 0);
    if (kp.irq < 0)
    return -EINVAL;
    error = devm_request_threaded_irq(&pdev.dev, kp.irq,
    core::ptr::null_mut(), bcm_kp_isr_thread,
    IRQF_ONESHOT, pdev.name, kp);
    if (error) {
    dev_err(&pdev.dev, "failed to request IRQ\n");
    return error;
    }
    error = input_register_device(input_dev);
    if (error) {
    dev_err(&pdev.dev, "failed to register input device\n");
    return error;
    }
    return 0;
    }
    static const struct of_device_id bcm_kp_of_match[] = {
    { .compatible = "brcm,bcm-keypad" },
    { },
    };
    MODULE_DEVICE_TABLE(of, bcm_kp_of_match);
    static struct platform_driver bcm_kp_device_driver = {
    .probe		= bcm_kp_probe,
    .driver		= {
    .name	= "bcm-keypad",
    .of_match_table = bcm_kp_of_match,
    }
    };
    module_platform_driver(bcm_kp_device_driver);
    MODULE_AUTHOR("Broadcom Corporation");
    MODULE_DESCRIPTION("BCM Keypad Driver");
    MODULE_LICENSE("GPL v2");
