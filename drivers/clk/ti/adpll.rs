//! Automatically rewritten from C to Rust
//! Source: drivers/clk/ti/adpll.c
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

pub const ADPLL_PLLSS_MMR_LOCK_OFFSET: c_uint = 0x00	/* Managed by MPPULL */;
pub const ADPLL_PLLSS_MMR_LOCK_ENABLED: c_uint = 0x1f125B64;
pub const ADPLL_PLLSS_MMR_UNLOCK_MAGIC: c_uint = 0x1eda4c3d;
pub const ADPLL_PWRCTRL_OFFSET: c_uint = 0x00;
pub const ADPLL_PWRCTRL_PONIN: c_int = 5;
pub const ADPLL_PWRCTRL_PGOODIN: c_int = 4;
pub const ADPLL_PWRCTRL_RET: c_int = 3;
pub const ADPLL_PWRCTRL_ISORET: c_int = 2;
pub const ADPLL_PWRCTRL_ISOSCAN: c_int = 1;
pub const ADPLL_PWRCTRL_OFFMODE: c_int = 0;
pub const ADPLL_CLKCTRL_OFFSET: c_uint = 0x04;
pub const ADPLL_CLKCTRL_CLKDCOLDOEN: c_int = 29;
pub const ADPLL_CLKCTRL_IDLE: c_int = 23;
pub const ADPLL_CLKCTRL_CLKOUTEN: c_int = 20;

pub const ADPLL_CLKCTRL_CLKOUTLDOEN_ADPLL_LJ: c_int = 19;
pub const ADPLL_CLKCTRL_ULOWCLKEN: c_int = 18;
pub const ADPLL_CLKCTRL_CLKDCOLDOPWDNZ: c_int = 17;
pub const ADPLL_CLKCTRL_M2PWDNZ: c_int = 16;
pub const ADPLL_CLKCTRL_M3PWDNZ_ADPLL_S: c_int = 15;
pub const ADPLL_CLKCTRL_LOWCURRSTDBY_ADPLL_S: c_int = 13;
pub const ADPLL_CLKCTRL_LPMODE_ADPLL_S: c_int = 12;
pub const ADPLL_CLKCTRL_REGM4XEN_ADPLL_S: c_int = 10;
pub const ADPLL_CLKCTRL_SELFREQDCO_ADPLL_LJ: c_int = 10;
pub const ADPLL_CLKCTRL_TINITZ: c_int = 0;
pub const ADPLL_TENABLE_OFFSET: c_uint = 0x08;
pub const ADPLL_TENABLEDIV_OFFSET: c_uint = 0x8c;
pub const ADPLL_M2NDIV_OFFSET: c_uint = 0x10;
pub const ADPLL_M2NDIV_M2: c_int = 16;
pub const ADPLL_M2NDIV_M2_ADPLL_S_WIDTH: c_int = 5;
pub const ADPLL_M2NDIV_M2_ADPLL_LJ_WIDTH: c_int = 7;
pub const ADPLL_MN2DIV_OFFSET: c_uint = 0x14;
pub const ADPLL_MN2DIV_N2: c_int = 16;
pub const ADPLL_FRACDIV_OFFSET: c_uint = 0x18;
pub const ADPLL_FRACDIV_REGSD: c_int = 24;
pub const ADPLL_FRACDIV_FRACTIONALM: c_int = 0;
pub const ADPLL_FRACDIV_FRACTIONALM_MASK: c_uint = 0x3ffff;
pub const ADPLL_BWCTRL_OFFSET: c_uint = 0x1c;
pub const ADPLL_BWCTRL_BWCONTROL: c_int = 1;
pub const ADPLL_BWCTRL_BW_INCR_DECRZ: c_int = 0;
pub const ADPLL_RESERVED_OFFSET: c_uint = 0x20;
pub const ADPLL_STATUS_OFFSET: c_uint = 0x24;
pub const ADPLL_STATUS_PONOUT: c_int = 31;
pub const ADPLL_STATUS_PGOODOUT: c_int = 30;
pub const ADPLL_STATUS_LDOPWDN: c_int = 29;
pub const ADPLL_STATUS_RECAL_BSTATUS3: c_int = 28;
pub const ADPLL_STATUS_RECAL_OPPIN: c_int = 27;
pub const ADPLL_STATUS_PHASELOCK: c_int = 10;
pub const ADPLL_STATUS_FREQLOCK: c_int = 9;
pub const ADPLL_STATUS_BYPASSACK: c_int = 8;
pub const ADPLL_STATUS_LOSSREF: c_int = 6;
pub const ADPLL_STATUS_CLKOUTENACK: c_int = 5;
pub const ADPLL_STATUS_LOCK2: c_int = 4;
pub const ADPLL_STATUS_M2CHANGEACK: c_int = 3;
pub const ADPLL_STATUS_HIGHJITTER: c_int = 1;
pub const ADPLL_STATUS_BYPASS: c_int = 0;

    BIT(ADPLL_STATUS_FREQLOCK))
pub const ADPLL_M3DIV_OFFSET: c_uint = 0x28	/* Only on MPUPLL */;
pub const ADPLL_M3DIV_M3: c_int = 0;
pub const ADPLL_M3DIV_M3_WIDTH: c_int = 5;
pub const ADPLL_M3DIV_M3_MASK: c_uint = 0x1f;
pub const ADPLL_RAMPCTRL_OFFSET: c_uint = 0x2c	/* Only on MPUPLL */;
pub const ADPLL_RAMPCTRL_CLKRAMPLEVEL: c_int = 19;
pub const ADPLL_RAMPCTRL_CLKRAMPRATE: c_int = 16;
pub const ADPLL_RAMPCTRL_RELOCK_RAMP_EN: c_int = 0;
pub const MAX_ADPLL_INPUTS: c_int = 3;
pub const MAX_ADPLL_OUTPUTS: c_int = 4;
pub const ADPLL_MAX_RETRIES: c_int = 5;

    enum ti_adpll_clocks {
    TI_ADPLL_DCO,
    TI_ADPLL_DCO_GATE,
    TI_ADPLL_N2,
    TI_ADPLL_M2,
    TI_ADPLL_M2_GATE,
    TI_ADPLL_BYPASS,
    TI_ADPLL_HIF,
    TI_ADPLL_DIV2,
    TI_ADPLL_CLKOUT,
    TI_ADPLL_CLKOUT2,
    TI_ADPLL_M3,
    };

    enum ti_adpll_inputs {
    TI_ADPLL_CLKINP,
    TI_ADPLL_CLKINPULOW,
    TI_ADPLL_CLKINPHIF,
    };
    enum ti_adpll_s_outputs {
    TI_ADPLL_S_DCOCLKLDO,
    TI_ADPLL_S_CLKOUT,
    TI_ADPLL_S_CLKOUTX2,
    TI_ADPLL_S_CLKOUTHIF,
    };
    enum ti_adpll_lj_outputs {
    TI_ADPLL_LJ_CLKDCOLDO,
    TI_ADPLL_LJ_CLKOUT,
    TI_ADPLL_LJ_CLKOUTLDO,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ti_adpll_platform_data {
    pub is_type_s: bool,
    pub nr_max_inputs: c_int,
    pub nr_max_outputs: c_int,
    pub output_index: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ti_adpll_clock {
    pub clk: *mut clk,
    pub cl: *mut clk_lookup,
    pub clk): *mut *mut void (unregister)(struct clk,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ti_adpll_dco_data {
    pub hw: clk_hw,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ti_adpll_clkout_data {
    pub adpll: *mut ti_adpll_data,
    pub gate: clk_gate,
    pub hw: clk_hw,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ti_adpll_data {
    pub dev: *mut device,
    pub c: *const ti_adpll_platform_data,
    pub np: *mut device_node,
    pub pa: c_ulong,
    pub iobase: *mut void __iomem,
    pub regs: *mut void __iomem,
    pub /: *mut *mut spinlock_t lock; / For ADPLL shared register access,
    pub parent_names: [*const c_char; MAX_ADPLL_INPUTS],
    pub parent_clocks: [*mut clk; MAX_ADPLL_INPUTS],
    pub clocks: *mut ti_adpll_clock,
    pub outputs: clk_onecell_data,
    pub dco: ti_adpll_dco_data,
}

    static const char *ti_adpll_clk_get_name(struct ti_adpll_data *d,
    int output_index,
    const char *postfix)
    {
    const char *name;
    int err;
    if (output_index >= 0) {
    err = of_property_read_string_index(d.np,
    "clock-output-names",
    output_index,
    &name);
    if (err)
    return core::ptr::null_mut();
    } else {
    name = devm_kasprintf(d.dev, GFP_KERNEL, "%08lx.adpll.%s",
    d.pa, postfix);
    }
    return name;
    }

    static int ti_adpll_setup_clock(struct ti_adpll_data *d, struct clk *clock,
    int index, int output_index, const char *name,
    void (*unregister)(struct clk *clk))
    {
    struct clk_lookup *cl;
    const char *postfix = core::ptr::null_mut();
    char con_id[ADPLL_MAX_CON_ID];
    d.clocks[index].clk = clock;
    d.clocks[index].unregister = unregister;
// Separate con_id in format "pll040dcoclkldo" to fit MAX_CON_ID
    postfix = strrchr(name, '.');
    if (postfix && strlen(postfix) > 1) {
    if (strlen(postfix) > ADPLL_MAX_CON_ID)
    dev_warn(d.dev, "clock %s con_id lookup may fail\n",
    name);
    snprintf(con_id, 16, "pll%03lx%s", d.pa & 0xfff, postfix + 1);
    cl = clkdev_create(clock, con_id, core::ptr::null_mut());
    if (!cl)
    return -ENOMEM;
    d.clocks[index].cl = cl;
    } else {
    dev_warn(d.dev, "no con_id for clock %s\n", name);
    }
    if (output_index < 0)
    return 0;
    d.outputs.clks[output_index] = clock;
    d.outputs.clk_num++;
    return 0;
    }
    static int ti_adpll_init_divider(struct ti_adpll_data *d,
    enum ti_adpll_clocks index,
    int output_index, char *name,
    struct clk *parent_clock,
    void __iomem *reg,
    u8 shift, u8 width,
    u8 clk_divider_flags)
    {
    const char *child_name;
    const char *parent_name;
    struct clk *clock;
    child_name = ti_adpll_clk_get_name(d, output_index, name);
    if (!child_name)
    return -EINVAL;
    parent_name = __clk_get_name(parent_clock);
    clock = clk_register_divider(d.dev, child_name, parent_name, 0,
    reg, shift, width, clk_divider_flags,
    &d.lock);
    if (IS_ERR(clock)) {
    dev_err(d.dev, "failed to register divider %s: %li\n",
    name, PTR_ERR(clock));
    return PTR_ERR(clock);
    }
    return ti_adpll_setup_clock(d, clock, index, output_index, child_name,
    clk_unregister_divider);
    }
    static int ti_adpll_init_mux(struct ti_adpll_data *d,
    enum ti_adpll_clocks index,
    char *name, struct clk *clk0,
    struct clk *clk1,
    void __iomem *reg,
    u8 shift)
    {
    const char *child_name;
    const char *parents[2];
    struct clk *clock;
    child_name = ti_adpll_clk_get_name(d, -ENODEV, name);
    if (!child_name)
    return -ENOMEM;
    parents[0] = __clk_get_name(clk0);
    parents[1] = __clk_get_name(clk1);
    clock = clk_register_mux(d.dev, child_name, parents, 2, 0,
    reg, shift, 1, 0, &d.lock);
    if (IS_ERR(clock)) {
    dev_err(d.dev, "failed to register mux %s: %li\n",
    name, PTR_ERR(clock));
    return PTR_ERR(clock);
    }
    return ti_adpll_setup_clock(d, clock, index, -ENODEV, child_name,
    clk_unregister_mux);
    }
    static int ti_adpll_init_gate(struct ti_adpll_data *d,
    enum ti_adpll_clocks index,
    int output_index, char *name,
    struct clk *parent_clock,
    void __iomem *reg,
    u8 bit_idx,
    u8 clk_gate_flags)
    {
    const char *child_name;
    const char *parent_name;
    struct clk *clock;
    child_name = ti_adpll_clk_get_name(d, output_index, name);
    if (!child_name)
    return -EINVAL;
    parent_name = __clk_get_name(parent_clock);
    clock = clk_register_gate(d.dev, child_name, parent_name, 0,
    reg, bit_idx, clk_gate_flags,
    &d.lock);
    if (IS_ERR(clock)) {
    dev_err(d.dev, "failed to register gate %s: %li\n",
    name, PTR_ERR(clock));
    return PTR_ERR(clock);
    }
    return ti_adpll_setup_clock(d, clock, index, output_index, child_name,
    clk_unregister_gate);
    }
    static int ti_adpll_init_fixed_factor(struct ti_adpll_data *d,
    enum ti_adpll_clocks index,
    char *name,
    struct clk *parent_clock,
    unsigned int mult,
    unsigned int div)
    {
    const char *child_name;
    const char *parent_name;
    struct clk *clock;
    child_name = ti_adpll_clk_get_name(d, -ENODEV, name);
    if (!child_name)
    return -ENOMEM;
    parent_name = __clk_get_name(parent_clock);
    clock = clk_register_fixed_factor(d.dev, child_name, parent_name,
    0, mult, div);
    if (IS_ERR(clock))
    return PTR_ERR(clock);
    return ti_adpll_setup_clock(d, clock, index, -ENODEV, child_name,
    clk_unregister);
    }
#[no_mangle]
unsafe extern "C" fn ti_adpll_set_idle_bypass(d: *mut ti_adpll_data) {
    static void ti_adpll_set_idle_bypass(struct ti_adpll_data *d)
    {
    unsigned long flags;
    u32 v;
    spin_lock_irqsave(&d.lock, flags);
    v = readl_relaxed(d.regs + ADPLL_CLKCTRL_OFFSET);
    v |= BIT(ADPLL_CLKCTRL_IDLE);
    writel_relaxed(v, d.regs + ADPLL_CLKCTRL_OFFSET);
    spin_unlock_irqrestore(&d.lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn ti_adpll_clear_idle_bypass(d: *mut ti_adpll_data) {
    static void ti_adpll_clear_idle_bypass(struct ti_adpll_data *d)
    {
    unsigned long flags;
    u32 v;
    spin_lock_irqsave(&d.lock, flags);
    v = readl_relaxed(d.regs + ADPLL_CLKCTRL_OFFSET);
    v &= ~BIT(ADPLL_CLKCTRL_IDLE);
    writel_relaxed(v, d.regs + ADPLL_CLKCTRL_OFFSET);
    spin_unlock_irqrestore(&d.lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn ti_adpll_clock_is_bypass(d: *mut ti_adpll_data) -> bool {
    static bool ti_adpll_clock_is_bypass(struct ti_adpll_data *d)
    {
    u32 v;
    v = readl_relaxed(d.regs + ADPLL_STATUS_OFFSET);
    return v & BIT(ADPLL_STATUS_BYPASS);
    }
//
// Locked and bypass are not actually mutually exclusive:  if you only care
// about the DCO clock and not CLKOUT you can clear M2PWDNZ before enabling
// the PLL, resulting in status (FREQLOCK | PHASELOCK | BYPASS) after lock.
//
#[no_mangle]
unsafe extern "C" fn ti_adpll_is_locked(d: *mut ti_adpll_data) -> bool {
    static bool ti_adpll_is_locked(struct ti_adpll_data *d)
    {
    let mut v: u32 = readl_relaxed(d.regs + ADPLL_STATUS_OFFSET);
    return (v & ADPLL_STATUS_PREPARED_MASK) == ADPLL_STATUS_PREPARED_MASK;
    }
#[no_mangle]
unsafe extern "C" fn ti_adpll_wait_lock(d: *mut ti_adpll_data) -> c_int {
    static int ti_adpll_wait_lock(struct ti_adpll_data *d)
    {
    let mut retries: c_int = ADPLL_MAX_RETRIES;
    do {
    if (ti_adpll_is_locked(d))
    return 0;
    usleep_range(200, 300);
    } while (retries--);
    dev_err(d.dev, "pll failed to lock\n");
    return -ETIMEDOUT;
    }
#[no_mangle]
unsafe extern "C" fn ti_adpll_prepare(hw: *mut clk_hw) -> c_int {
    static int ti_adpll_prepare(struct clk_hw *hw)
    {
    struct ti_adpll_dco_data *dco = to_dco(hw);
    struct ti_adpll_data *d = to_adpll(dco);
    ti_adpll_clear_idle_bypass(d);
    ti_adpll_wait_lock(d);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ti_adpll_unprepare(hw: *mut clk_hw) {
    static void ti_adpll_unprepare(struct clk_hw *hw)
    {
    struct ti_adpll_dco_data *dco = to_dco(hw);
    struct ti_adpll_data *d = to_adpll(dco);
    ti_adpll_set_idle_bypass(d);
    }
#[no_mangle]
unsafe extern "C" fn ti_adpll_is_prepared(hw: *mut clk_hw) -> c_int {
    static int ti_adpll_is_prepared(struct clk_hw *hw)
    {
    struct ti_adpll_dco_data *dco = to_dco(hw);
    struct ti_adpll_data *d = to_adpll(dco);
    return ti_adpll_is_locked(d);
    }
//
// Note that the DCO clock is never subject to bypass: if the PLL is off,
// dcoclk is low.
//
    static unsigned long ti_adpll_recalc_rate(struct clk_hw *hw,
    unsigned long parent_rate)
    {
    struct ti_adpll_dco_data *dco = to_dco(hw);
    struct ti_adpll_data *d = to_adpll(dco);
    u32 frac_m, divider, v;
    u64 rate;
    unsigned long flags;
    if (ti_adpll_clock_is_bypass(d))
    return 0;
    spin_lock_irqsave(&d.lock, flags);
    frac_m = readl_relaxed(d.regs + ADPLL_FRACDIV_OFFSET);
    frac_m &= ADPLL_FRACDIV_FRACTIONALM_MASK;
    rate = (u64)readw_relaxed(d.regs + ADPLL_MN2DIV_OFFSET) << 18;
    rate += frac_m;
    rate *= parent_rate;
    divider = (readw_relaxed(d.regs + ADPLL_M2NDIV_OFFSET) + 1) << 18;
    spin_unlock_irqrestore(&d.lock, flags);
    do_div(rate, divider);
    if (d.c.is_type_s) {
    v = readl_relaxed(d.regs + ADPLL_CLKCTRL_OFFSET);
    if (v & BIT(ADPLL_CLKCTRL_REGM4XEN_ADPLL_S))
    rate *= 4;
    rate *= 2;
    }
    return rate;
    }
// PLL parent is always clkinp, bypass only affects the children
#[no_mangle]
unsafe extern "C" fn ti_adpll_get_parent(hw: *mut clk_hw) -> u8 {
    static u8 ti_adpll_get_parent(struct clk_hw *hw)
    {
    return 0;
    }
    static const struct clk_ops ti_adpll_ops = {
    .prepare = ti_adpll_prepare,
    .unprepare = ti_adpll_unprepare,
    .is_prepared = ti_adpll_is_prepared,
    .recalc_rate = ti_adpll_recalc_rate,
    .get_parent = ti_adpll_get_parent,
    };
#[no_mangle]
unsafe extern "C" fn ti_adpll_init_dco(d: *mut ti_adpll_data) -> c_int {
    static int ti_adpll_init_dco(struct ti_adpll_data *d)
    {
    let mut init: clk_init_data = {};
    struct clk *clock;
    const char *postfix;
    int width, err;
    d.outputs.clks = devm_kcalloc(d.dev,
    MAX_ADPLL_OUTPUTS,
    sizeof(struct clk *),
    GFP_KERNEL);
    if (!d.outputs.clks)
    return -ENOMEM;
    if (d.c.output_index < 0)
    postfix = "dco";
    else
    postfix = core::ptr::null_mut();
    init.name = ti_adpll_clk_get_name(d, d.c.output_index, postfix);
    if (!init.name)
    return -EINVAL;
    init.parent_names = d.parent_names;
    init.num_parents = d.c.nr_max_inputs;
    init.ops = &ti_adpll_ops;
    init.flags = CLK_GET_RATE_NOCACHE;
    d.dco.hw.init = &init;
    if (d.c.is_type_s)
    width = 5;
    else
    width = 4;
// Internal input clock divider N2
    err = ti_adpll_init_divider(d, TI_ADPLL_N2, -ENODEV, "n2",
    d.parent_clocks[TI_ADPLL_CLKINP],
    d.regs + ADPLL_MN2DIV_OFFSET,
    ADPLL_MN2DIV_N2, width, 0);
    if (err)
    return err;
    clock = devm_clk_register(d.dev, &d.dco.hw);
    if (IS_ERR(clock))
    return PTR_ERR(clock);
    return ti_adpll_setup_clock(d, clock, TI_ADPLL_DCO, d.c.output_index,
    init.name, core::ptr::null_mut());
    }
#[no_mangle]
unsafe extern "C" fn ti_adpll_clkout_enable(hw: *mut clk_hw) -> c_int {
    static int ti_adpll_clkout_enable(struct clk_hw *hw)
    {
    struct ti_adpll_clkout_data *co = to_clkout(hw);
    struct clk_hw *gate_hw = &co.gate.hw;
    __clk_hw_set_clk(gate_hw, hw);
    return clk_gate_ops.enable(gate_hw);
    }
#[no_mangle]
unsafe extern "C" fn ti_adpll_clkout_disable(hw: *mut clk_hw) {
    static void ti_adpll_clkout_disable(struct clk_hw *hw)
    {
    struct ti_adpll_clkout_data *co = to_clkout(hw);
    struct clk_hw *gate_hw = &co.gate.hw;
    __clk_hw_set_clk(gate_hw, hw);
    clk_gate_ops.disable(gate_hw);
    }
#[no_mangle]
unsafe extern "C" fn ti_adpll_clkout_is_enabled(hw: *mut clk_hw) -> c_int {
    static int ti_adpll_clkout_is_enabled(struct clk_hw *hw)
    {
    struct ti_adpll_clkout_data *co = to_clkout(hw);
    struct clk_hw *gate_hw = &co.gate.hw;
    __clk_hw_set_clk(gate_hw, hw);
    return clk_gate_ops.is_enabled(gate_hw);
    }
// Setting PLL bypass puts clkout and clkoutx2 into bypass
#[no_mangle]
unsafe extern "C" fn ti_adpll_clkout_get_parent(hw: *mut clk_hw) -> u8 {
    static u8 ti_adpll_clkout_get_parent(struct clk_hw *hw)
    {
    struct ti_adpll_clkout_data *co = to_clkout(hw);
    struct ti_adpll_data *d = co.adpll;
    return ti_adpll_clock_is_bypass(d);
    }
    static int ti_adpll_init_clkout(struct ti_adpll_data *d,
    enum ti_adpll_clocks index,
    int output_index, int gate_bit,
    char *name, struct clk *clk0,
    struct clk *clk1)
    {
    struct ti_adpll_clkout_data *co;
    let mut init: clk_init_data = {};
    struct clk_ops *ops;
    const char *parent_names[2];
    const char *child_name;
    struct clk *clock;
    int err;
    co = devm_kzalloc(d.dev, sizeof(*co), GFP_KERNEL);
    if (!co)
    return -ENOMEM;
    co.adpll = d;
    err = of_property_read_string_index(d.np,
    "clock-output-names",
    output_index,
    &child_name);
    if (err)
    return err;
    ops = devm_kzalloc(d.dev, sizeof(*ops), GFP_KERNEL);
    if (!ops)
    return -ENOMEM;
    init.name = child_name;
    init.ops = ops;
    init.flags = 0;
    co.hw.init = &init;
    parent_names[0] = __clk_get_name(clk0);
    parent_names[1] = __clk_get_name(clk1);
    init.parent_names = parent_names;
    init.num_parents = 2;
    ops.get_parent = ti_adpll_clkout_get_parent;
    ops.determine_rate = __clk_mux_determine_rate;
    if (gate_bit) {
    co.gate.lock = &d.lock;
    co.gate.reg = d.regs + ADPLL_CLKCTRL_OFFSET;
    co.gate.bit_idx = gate_bit;
    ops.enable = ti_adpll_clkout_enable;
    ops.disable = ti_adpll_clkout_disable;
    ops.is_enabled = ti_adpll_clkout_is_enabled;
    }
    clock = devm_clk_register(d.dev, &co.hw);
    if (IS_ERR(clock)) {
    dev_err(d.dev, "failed to register output %s: %li\n",
    name, PTR_ERR(clock));
    return PTR_ERR(clock);
    }
    return ti_adpll_setup_clock(d, clock, index, output_index, child_name,
    core::ptr::null_mut());
    }
#[no_mangle]
unsafe extern "C" fn ti_adpll_init_children_adpll_s(d: *mut ti_adpll_data) -> c_int {
    static int ti_adpll_init_children_adpll_s(struct ti_adpll_data *d)
    {
    int err;
    if (!d.c.is_type_s)
    return 0;
// Internal mux, sources from divider N2 or clkinpulow
    err = ti_adpll_init_mux(d, TI_ADPLL_BYPASS, "bypass",
    d.clocks[TI_ADPLL_N2].clk,
    d.parent_clocks[TI_ADPLL_CLKINPULOW],
    d.regs + ADPLL_CLKCTRL_OFFSET,
    ADPLL_CLKCTRL_ULOWCLKEN);
    if (err)
    return err;
// Internal divider M2, sources DCO
    err = ti_adpll_init_divider(d, TI_ADPLL_M2, -ENODEV, "m2",
    d.clocks[TI_ADPLL_DCO].clk,
    d.regs + ADPLL_M2NDIV_OFFSET,
    ADPLL_M2NDIV_M2,
    ADPLL_M2NDIV_M2_ADPLL_S_WIDTH,
    CLK_DIVIDER_ONE_BASED);
    if (err)
    return err;
// Internal fixed divider, after M2 before clkout
    err = ti_adpll_init_fixed_factor(d, TI_ADPLL_DIV2, "div2",
    d.clocks[TI_ADPLL_M2].clk,
    1, 2);
    if (err)
    return err;
// Output clkout with a mux and gate, sources from div2 or bypass
    err = ti_adpll_init_clkout(d, TI_ADPLL_CLKOUT, TI_ADPLL_S_CLKOUT,
    ADPLL_CLKCTRL_CLKOUTEN, "clkout",
    d.clocks[TI_ADPLL_DIV2].clk,
    d.clocks[TI_ADPLL_BYPASS].clk);
    if (err)
    return err;
// Output clkoutx2 with a mux and gate, sources from M2 or bypass
    err = ti_adpll_init_clkout(d, TI_ADPLL_CLKOUT2, TI_ADPLL_S_CLKOUTX2, 0,
    "clkout2", d.clocks[TI_ADPLL_M2].clk,
    d.clocks[TI_ADPLL_BYPASS].clk);
    if (err)
    return err;
// Internal mux, sources from DCO and clkinphif
    if (d.parent_clocks[TI_ADPLL_CLKINPHIF]) {
    err = ti_adpll_init_mux(d, TI_ADPLL_HIF, "hif",
    d.clocks[TI_ADPLL_DCO].clk,
    d.parent_clocks[TI_ADPLL_CLKINPHIF],
    d.regs + ADPLL_CLKCTRL_OFFSET,
    ADPLL_CLKINPHIFSEL_ADPLL_S);
    if (err)
    return err;
    }
// Output clkouthif with a divider M3, sources from hif
    err = ti_adpll_init_divider(d, TI_ADPLL_M3, TI_ADPLL_S_CLKOUTHIF, "m3",
    d.clocks[TI_ADPLL_HIF].clk,
    d.regs + ADPLL_M3DIV_OFFSET,
    ADPLL_M3DIV_M3,
    ADPLL_M3DIV_M3_WIDTH,
    CLK_DIVIDER_ONE_BASED);
    if (err)
    return err;
// Output clock dcoclkldo is the DCO
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ti_adpll_init_children_adpll_lj(d: *mut ti_adpll_data) -> c_int {
    static int ti_adpll_init_children_adpll_lj(struct ti_adpll_data *d)
    {
    int err;
    if (d.c.is_type_s)
    return 0;
// Output clkdcoldo, gated output of DCO
    err = ti_adpll_init_gate(d, TI_ADPLL_DCO_GATE, TI_ADPLL_LJ_CLKDCOLDO,
    "clkdcoldo", d.clocks[TI_ADPLL_DCO].clk,
    d.regs + ADPLL_CLKCTRL_OFFSET,
    ADPLL_CLKCTRL_CLKDCOLDOEN, 0);
    if (err)
    return err;
// Internal divider M2, sources from DCO
    err = ti_adpll_init_divider(d, TI_ADPLL_M2, -ENODEV,
    "m2", d.clocks[TI_ADPLL_DCO].clk,
    d.regs + ADPLL_M2NDIV_OFFSET,
    ADPLL_M2NDIV_M2,
    ADPLL_M2NDIV_M2_ADPLL_LJ_WIDTH,
    CLK_DIVIDER_ONE_BASED);
    if (err)
    return err;
// Output clkoutldo, gated output of M2
    err = ti_adpll_init_gate(d, TI_ADPLL_M2_GATE, TI_ADPLL_LJ_CLKOUTLDO,
    "clkoutldo", d.clocks[TI_ADPLL_M2].clk,
    d.regs + ADPLL_CLKCTRL_OFFSET,
    ADPLL_CLKCTRL_CLKOUTLDOEN_ADPLL_LJ,
    0);
    if (err)
    return err;
// Internal mux, sources from divider N2 or clkinpulow
    err = ti_adpll_init_mux(d, TI_ADPLL_BYPASS, "bypass",
    d.clocks[TI_ADPLL_N2].clk,
    d.parent_clocks[TI_ADPLL_CLKINPULOW],
    d.regs + ADPLL_CLKCTRL_OFFSET,
    ADPLL_CLKCTRL_ULOWCLKEN);
    if (err)
    return err;
// Output clkout, sources M2 or bypass
    err = ti_adpll_init_clkout(d, TI_ADPLL_CLKOUT, TI_ADPLL_S_CLKOUT,
    ADPLL_CLKCTRL_CLKOUTEN, "clkout",
    d.clocks[TI_ADPLL_M2].clk,
    d.clocks[TI_ADPLL_BYPASS].clk);
    if (err)
    return err;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ti_adpll_free_resources(d: *mut ti_adpll_data) {
    static void ti_adpll_free_resources(struct ti_adpll_data *d)
    {
    int i;
    for (i = TI_ADPLL_M3; i >= 0; i--) {
    struct ti_adpll_clock *ac = &d.clocks[i];
    if (!ac || IS_ERR_OR_NULL(ac.clk))
    continue;
    if (ac.cl)
    clkdev_drop(ac.cl);
    if (ac.unregister)
    ac.unregister(ac.clk);
    }
    }
// MPU PLL manages the lock register for all PLLs
#[no_mangle]
unsafe extern "C" fn ti_adpll_unlock_all(reg: *mut void __iomem) {
    static void ti_adpll_unlock_all(void __iomem *reg)
    {
    u32 v;
    v = readl_relaxed(reg);
    if (v == ADPLL_PLLSS_MMR_LOCK_ENABLED)
    writel_relaxed(ADPLL_PLLSS_MMR_UNLOCK_MAGIC, reg);
    }
#[no_mangle]
unsafe extern "C" fn ti_adpll_init_registers(d: *mut ti_adpll_data) -> c_int {
    static int ti_adpll_init_registers(struct ti_adpll_data *d)
    {
    let mut register_offset: c_int = 0;
    if (d.c.is_type_s) {
    register_offset = 8;
    ti_adpll_unlock_all(d.iobase + ADPLL_PLLSS_MMR_LOCK_OFFSET);
    }
    d.regs = d.iobase + register_offset + ADPLL_PWRCTRL_OFFSET;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ti_adpll_init_inputs(d: *mut ti_adpll_data) -> c_int {
    static int ti_adpll_init_inputs(struct ti_adpll_data *d)
    {
    static const char error[] = "need at least %i inputs";
    struct clk *clock;
    int nr_inputs;
    nr_inputs = of_clk_get_parent_count(d.np);
    if (nr_inputs < d.c.nr_max_inputs) {
    dev_err(d.dev, error, nr_inputs);
    return -EINVAL;
    }
    of_clk_parent_fill(d.np, d.parent_names, nr_inputs);
    clock = devm_clk_get(d.dev, d.parent_names[0]);
    if (IS_ERR(clock)) {
    dev_err(d.dev, "could not get clkinp\n");
    return PTR_ERR(clock);
    }
    d.parent_clocks[TI_ADPLL_CLKINP] = clock;
    clock = devm_clk_get(d.dev, d.parent_names[1]);
    if (IS_ERR(clock)) {
    dev_err(d.dev, "could not get clkinpulow clock\n");
    return PTR_ERR(clock);
    }
    d.parent_clocks[TI_ADPLL_CLKINPULOW] = clock;
    if (d.c.is_type_s) {
    clock =  devm_clk_get(d.dev, d.parent_names[2]);
    if (IS_ERR(clock)) {
    dev_err(d.dev, "could not get clkinphif clock\n");
    return PTR_ERR(clock);
    }
    d.parent_clocks[TI_ADPLL_CLKINPHIF] = clock;
    }
    return 0;
    }
    static const struct ti_adpll_platform_data ti_adpll_type_s = {
    .is_type_s = true,
    .nr_max_inputs = MAX_ADPLL_INPUTS,
    .nr_max_outputs = MAX_ADPLL_OUTPUTS,
    .output_index = TI_ADPLL_S_DCOCLKLDO,
    };
    static const struct ti_adpll_platform_data ti_adpll_type_lj = {
    .is_type_s = false,
    .nr_max_inputs = MAX_ADPLL_INPUTS - 1,
    .nr_max_outputs = MAX_ADPLL_OUTPUTS - 1,
    .output_index = -EINVAL,
    };
    static const struct of_device_id ti_adpll_match[] = {
    { .compatible = "ti,dm814-adpll-s-clock", &ti_adpll_type_s },
    { .compatible = "ti,dm814-adpll-lj-clock", &ti_adpll_type_lj },
    {},
    };
    MODULE_DEVICE_TABLE(of, ti_adpll_match);
#[no_mangle]
unsafe extern "C" fn ti_adpll_probe(pdev: *mut platform_device) -> c_int {
    static int ti_adpll_probe(struct platform_device *pdev)
    {
    struct device_node *node = pdev.dev.of_node;
    struct device *dev = &pdev.dev;
    struct ti_adpll_data *d;
    struct resource *res;
    int err;
    d = devm_kzalloc(dev, sizeof(*d), GFP_KERNEL);
    if (!d)
    return -ENOMEM;
    d.dev = dev;
    d.np = node;
    d.c = device_get_match_data(dev);
    dev_set_drvdata(d.dev, d);
    spin_lock_init(&d.lock);
    d.iobase = devm_platform_get_and_ioremap_resource(pdev, 0, &res);
    if (IS_ERR(d.iobase))
    return PTR_ERR(d.iobase);
    d.pa = res.start;
    err = ti_adpll_init_registers(d);
    if (err)
    return err;
    err = ti_adpll_init_inputs(d);
    if (err)
    return err;
    d.clocks = devm_kcalloc(d.dev,
    TI_ADPLL_NR_CLOCKS,
    sizeof(struct ti_adpll_clock),
    GFP_KERNEL);
    if (!d.clocks)
    return -ENOMEM;
    err = ti_adpll_init_dco(d);
    if (err) {
    dev_err(dev, "could not register dco: %i\n", err);
    goto free;
    }
    err = ti_adpll_init_children_adpll_s(d);
    if (err)
    goto free;
    err = ti_adpll_init_children_adpll_lj(d);
    if (err)
    goto free;
    err = of_clk_add_provider(d.np, of_clk_src_onecell_get, &d.outputs);
    if (err)
    goto free;
    return 0;
    free:
    WARN_ON(1);
    ti_adpll_free_resources(d);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn ti_adpll_remove(pdev: *mut platform_device) {
    static void ti_adpll_remove(struct platform_device *pdev)
    {
    struct ti_adpll_data *d = dev_get_drvdata(&pdev.dev);
    ti_adpll_free_resources(d);
    }
    static struct platform_driver ti_adpll_driver = {
    .driver = {
    .name = "ti-adpll",
    .of_match_table = ti_adpll_match,
    },
    .probe = ti_adpll_probe,
    .remove = ti_adpll_remove,
    };
#[no_mangle]
unsafe extern "C" fn ti_adpll_init() -> int __init {
    static int __init ti_adpll_init(void)
    {
    return platform_driver_register(&ti_adpll_driver);
    }
    core_initcall(ti_adpll_init);
#[no_mangle]
unsafe extern "C" fn ti_adpll_exit() -> void __exit {
    static void __exit ti_adpll_exit(void)
    {
    platform_driver_unregister(&ti_adpll_driver);
    }
    module_exit(ti_adpll_exit);
    MODULE_DESCRIPTION("Clock driver for dm814x ADPLL");
    MODULE_ALIAS("platform:dm814-adpll-clock");
    MODULE_AUTHOR("Tony LIndgren <tony@atomide.com>");
    MODULE_LICENSE("GPL v2");
