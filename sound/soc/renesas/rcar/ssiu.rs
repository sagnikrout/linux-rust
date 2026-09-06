//! Automatically rewritten from C to Rust
//! Source: sound/soc/renesas/rcar/ssiu.c
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
//
// Renesas R-Car SSIU support
//
// Copyright (c) 2015 Kuninori Morimoto <kuninori.morimoto.gx@renesas.com>

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rsnd_ssiu {
    pub mod: rsnd_mod,
    pub /: *mut *mut u32 busif_status[8]; / for BUSIF0 - BUSIF7,
    pub usrcnt: c_uint,
    pub id: c_int,
    pub id_sub: c_int,
}

// SSI_MODE

    for (i = 0;							\
    (i < rsnd_ssiu_nr(priv)) &&				\
    ((pos) = ((struct rsnd_ssiu *)(priv).ssiu + i));	\
    i++)
//
// SSI	Gen2		Gen3		Gen4		RZ/G3E
// 0	BUSIF0-3	BUSIF0-7	BUSIF0-7	BUSIF0-3
// 1	BUSIF0-3	BUSIF0-7			BUSIF0-3
// 2	BUSIF0-3	BUSIF0-7			BUSIF0-3
// 3	BUSIF0		BUSIF0-7			BUSIF0-3
// 4	BUSIF0		BUSIF0-7			BUSIF0-3
// 5	BUSIF0		BUSIF0				BUSIF0
// 6	BUSIF0		BUSIF0				BUSIF0
// 7	BUSIF0		BUSIF0				BUSIF0
// 8	BUSIF0		BUSIF0				BUSIF0
// 9	BUSIF0-3	BUSIF0-7			BUSIF0-3
// total	22		52		8		28
//
    static const int gen2_id[] = { 0, 4,  8, 12, 13, 14, 15, 16, 17, 18 };
    static const int gen3_id[] = { 0, 8, 16, 24, 32, 40, 41, 42, 43, 44 };
    static const int gen4_id[] = { 0 };
    static const int rzg3e_id[] = { 0, 4, 8, 12, 16, 20, 21, 22, 23, 24 };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rsnd_ssiu_ctrl {
    pub busif_status_count: c_uint,
}

    ((struct rsnd_ssiu_ctrl *)(priv).ssiu_ctrl)
// enable busif buffer over/under run interrupt.

#[no_mangle]
unsafe extern "C" fn rsnd_ssiu_busif_err_irq_ctrl(mod: *mut rsnd_mod, enable: c_int) {
    static void rsnd_ssiu_busif_err_irq_ctrl(struct rsnd_mod *mod, int enable)
    {
    struct rsnd_priv *priv = rsnd_mod_to_priv(mod);
    let mut id: c_int = rsnd_mod_id(mod);
    int shift, offset;
    switch (id) {
    case 0:
    case 1:
    case 2:
    case 3:
    case 4:
    shift  = id;
    offset = 0;
    break;
    case 9:
    shift  = 1;
    offset = 1;
    break;
    default:
    return;
    }
    for (unsigned int i = 0; i < rsnd_priv_to_ssiu_ctrl(priv).busif_status_count; i++) {
    let mut reg: enum rsnd_reg = SSI_SYS_INT_ENABLE((i * 2) + offset);
    let mut val: u32 = 0xf << (shift * 4);
    let mut sys_int_enable: u32 = rsnd_mod_read(mod, reg);
    if (enable)
    sys_int_enable |= val;
    else
    sys_int_enable &= ~val;
    rsnd_mod_write(mod, reg, sys_int_enable);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn rsnd_ssiu_busif_err_status_clear(mod: *mut rsnd_mod) -> bool {
    bool rsnd_ssiu_busif_err_status_clear(struct rsnd_mod *mod)
    {
    struct rsnd_priv *priv = rsnd_mod_to_priv(mod);
    let mut error: bool = false;
    let mut id: c_int = rsnd_mod_id(mod);
    int shift, offset;
    switch (id) {
    case 0:
    case 1:
    case 2:
    case 3:
    case 4:
    shift  = id;
    offset = 0;
    break;
    case 9:
    shift  = 1;
    offset = 1;
    break;
    default:
    goto out;
    }
    for (unsigned int i = 0; i < rsnd_priv_to_ssiu_ctrl(priv).busif_status_count; i++) {
    let mut reg: u32 = SSI_SYS_STATUS(i * 2) + offset;
    let mut status: u32 = rsnd_mod_read(mod, reg);
    let mut val: u32 = 0xf << (shift * 4);
    status &= val;
    if (status) {
    struct device *dev = rsnd_priv_to_dev(priv);
    rsnd_print_irq_status(dev, "%s err status : 0x%08x\n",
    rsnd_mod_name(mod), status);
    error = true;
    }
    rsnd_mod_write(mod, reg, val);
    }
    out:
    return error;
    }
    static u32 *rsnd_ssiu_get_status(struct rsnd_mod *mod,
    struct rsnd_dai_stream *io,
    enum rsnd_mod_type type)
    {
    struct rsnd_ssiu *ssiu = rsnd_mod_to_ssiu(mod);
    let mut busif: c_int = rsnd_mod_id_sub(mod);
    return &ssiu.busif_status[busif];
    }
    static int rsnd_ssiu_init(struct rsnd_mod *mod,
    struct rsnd_dai_stream *io,
    struct rsnd_priv *priv)
    {
    struct rsnd_dai *rdai = rsnd_io_to_rdai(io);
    let mut ssis: u32 = rsnd_ssi_multi_secondaries_runtime(io);
    let mut use_busif: c_int = rsnd_ssi_use_busif(io);
    let mut id: c_int = rsnd_mod_id(mod);
    let mut is_clk_master: c_int = rsnd_rdai_is_clk_master(rdai);
    u32 val1, val2;
// clear status
    rsnd_ssiu_busif_err_status_clear(mod);
// Gen4 doesn't have SSI_MODE
    if (rsnd_is_gen4(priv))
    goto ssi_mode_setting_end;
//
// SSI_MODE0
//
    if (!rsnd_is_rzg3e(priv))
    rsnd_mod_bset(mod, SSI_MODE0, (1 << id), !use_busif << id);
//
// SSI_MODE1 / SSI_MODE2
//
// FIXME
// sharing/multi with SSI0 are mainly supported
//
    val1 = rsnd_mod_read(mod, SSI_MODE1);
    val2 = rsnd_mod_read(mod, SSI_MODE2);
    if (rsnd_ssi_is_pin_sharing(io)) {
    ssis |= (1 << id);
    } else if (ssis) {
//
// Multi SSI
//
// set synchronized bit here
//
// SSI4 is synchronized with SSI3
    if (ssis & (1 << 4))
    val1 |= (1 << 20);
// SSI012 are synchronized
    if (ssis == 0x0006)
    val1 |= (1 << 4);
// SSI0129 are synchronized
    if (ssis == 0x0206)
    val2 |= (1 << 4);
    }
// SSI1 is sharing pin with SSI0
    if (ssis & (1 << 1))
    val1 |= is_clk_master ? 0x2 : 0x1;
// SSI2 is sharing pin with SSI0
    if (ssis & (1 << 2))
    val1 |= is_clk_master ?	0x2 << 2 :
    0x1 << 2;
// SSI4 is sharing pin with SSI3
    if (ssis & (1 << 4))
    val1 |= is_clk_master ? 0x2 << 16 :
    0x1 << 16;
// SSI9 is sharing pin with SSI0
    if (ssis & (1 << 9))
    val2 |= is_clk_master ? 0x2 : 0x1;
    rsnd_mod_bset(mod, SSI_MODE1, 0x0013001f, val1);
    rsnd_mod_bset(mod, SSI_MODE2, 0x00000017, val2);
    ssi_mode_setting_end:
//
// Enable busif buffer over/under run interrupt.
// It will be handled from ssi.c
// see
// __rsnd_ssi_interrupt()
//
    rsnd_ssiu_busif_err_irq_enable(mod);
    return 0;
    }
    static int rsnd_ssiu_quit(struct rsnd_mod *mod,
    struct rsnd_dai_stream *io,
    struct rsnd_priv *priv)
    {
// disable busif buffer over/under run interrupt.
    rsnd_ssiu_busif_err_irq_disable(mod);
    return 0;
    }
    static struct rsnd_mod_ops rsnd_ssiu_ops_gen1 = {
    .name		= SSIU_NAME,
    .init		= rsnd_ssiu_init,
    .quit		= rsnd_ssiu_quit,
    .get_status	= rsnd_ssiu_get_status,
    };
    static int rsnd_ssiu_init_gen2(struct rsnd_mod *mod,
    struct rsnd_dai_stream *io,
    struct rsnd_priv *priv)
    {
    struct rsnd_ssiu *ssiu = rsnd_mod_to_ssiu(mod);
    let mut has_hdmi0: u32 = rsnd_flags_has(io, RSND_STREAM_HDMI0);
    let mut has_hdmi1: u32 = rsnd_flags_has(io, RSND_STREAM_HDMI1);
    int ret;
    let mut mode: u32 = 0;
    ret = rsnd_ssiu_init(mod, io, priv);
    if (ret < 0)
    return ret;
    ssiu.usrcnt++;
//
// TDM Extend/Split Mode
// see
// rsnd_ssi_config_init()
//
    if (rsnd_runtime_is_tdm(io))
    mode = TDM_EXT;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: rsnd_runtime_is_tdm_split(io)) -> else {
    else if (rsnd_runtime_is_tdm_split(io))
    mode = TDM_SPLIT;
    rsnd_mod_write(mod, SSI_MODE, mode);
    if (rsnd_ssi_use_busif(io)) {
    let mut id: c_int = rsnd_mod_id(mod);
    let mut busif: c_int = rsnd_mod_id_sub(mod);
    enum rsnd_reg adinr_reg, mode_reg, dalign_reg;
    if ((id == 9) && (busif >= 4)) {
    adinr_reg = SSI9_BUSIF_ADINR(busif);
    mode_reg = SSI9_BUSIF_MODE(busif);
    dalign_reg = SSI9_BUSIF_DALIGN(busif);
    } else {
    adinr_reg = SSI_BUSIF_ADINR(busif);
    mode_reg = SSI_BUSIF_MODE(busif);
    dalign_reg = SSI_BUSIF_DALIGN(busif);
    }
    rsnd_mod_write(mod, adinr_reg,
    rsnd_get_adinr_bit(mod, io) |
    (rsnd_io_is_play(io) ?
    rsnd_runtime_channel_after_ctu(io) :
    rsnd_runtime_channel_original(io)));
    rsnd_mod_write(mod, mode_reg,
    rsnd_get_busif_shift(io, mod) | 1);
    rsnd_mod_write(mod, dalign_reg,
    rsnd_get_dalign(mod, io));
    }
    if (has_hdmi0 || has_hdmi1) {
    enum rsnd_mod_type rsnd_ssi_array[] = {
    RSND_MOD_SSIM1,
    RSND_MOD_SSIM2,
    RSND_MOD_SSIM3,
    };
    struct rsnd_mod *ssi_mod = rsnd_io_to_mod_ssi(io);
    struct rsnd_mod *pos;
    u32 val;
    int i;
    i = rsnd_mod_id(ssi_mod);
// output all same SSI as default
    val =	i << 16 |
    i << 20 |
    i << 24 |
    i << 28 |
    i;
    for_each_rsnd_mod_array(i, pos, io, rsnd_ssi_array) {
    let mut shift: c_int = (i * 4) + 20;
    val	= (val & ~(0xF << shift)) |
    rsnd_mod_id(pos) << shift;
    }
    if (has_hdmi0)
    rsnd_mod_write(mod, HDMI0_SEL, val);
    if (has_hdmi1)
    rsnd_mod_write(mod, HDMI1_SEL, val);
    }
    return 0;
    }
    static int rsnd_ssiu_start_gen2(struct rsnd_mod *mod,
    struct rsnd_dai_stream *io,
    struct rsnd_priv *priv)
    {
    let mut busif: c_int = rsnd_mod_id_sub(mod);
    if (!rsnd_ssi_use_busif(io))
    return 0;
    rsnd_mod_bset(mod, SSI_CTRL, 1 << (busif * 4), 1 << (busif * 4));
    if (rsnd_ssi_multi_secondaries_runtime(io))
    rsnd_mod_write(mod, SSI_CONTROL, 0x1);
    return 0;
    }
    static int rsnd_ssiu_stop_gen2(struct rsnd_mod *mod,
    struct rsnd_dai_stream *io,
    struct rsnd_priv *priv)
    {
    struct rsnd_ssiu *ssiu = rsnd_mod_to_ssiu(mod);
    let mut busif: c_int = rsnd_mod_id_sub(mod);
    if (!rsnd_ssi_use_busif(io))
    return 0;
    rsnd_mod_bset(mod, SSI_CTRL, 1 << (busif * 4), 0);
    if (--ssiu.usrcnt)
    return 0;
    if (rsnd_ssi_multi_secondaries_runtime(io))
    rsnd_mod_write(mod, SSI_CONTROL, 0);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rsnd_ssiu_id(mod: *mut rsnd_mod) -> c_int {
    static int rsnd_ssiu_id(struct rsnd_mod *mod)
    {
    struct rsnd_ssiu *ssiu = rsnd_mod_to_ssiu(mod);
// see rsnd_ssiu_probe()
    return ssiu.id;
    }
#[no_mangle]
unsafe extern "C" fn rsnd_ssiu_id_sub(mod: *mut rsnd_mod) -> c_int {
    static int rsnd_ssiu_id_sub(struct rsnd_mod *mod)
    {
    struct rsnd_ssiu *ssiu = rsnd_mod_to_ssiu(mod);
// see rsnd_ssiu_probe()
    return ssiu.id_sub;
    }
    static struct dma_chan *rsnd_ssiu_dma_req(struct rsnd_dai_stream *io,
    struct rsnd_mod *mod)
    {
    struct rsnd_priv *priv = rsnd_mod_to_priv(mod);
    let mut is_play: c_int = rsnd_io_is_play(io);
    char *name;
//
// It should use "rcar_sound,ssiu" (R-Car) or "ssiu" (RZ/G3E) on DT.
// We need to keep compatibility for old versions.
//
// If it has "rcar_sound.ssiu" or "ssiu", it will be used.
// If not, "rcar_sound.ssi" or "ssi" will be used.
// see
// rsnd_ssi_dma_req()
// rsnd_dma_of_path()
//
    name = is_play ? "rx" : "tx";
    return rsnd_dma_request_channel(rsnd_ssiu_of_node(priv),
    SSIU_NAME, mod, name);
    }

    static void rsnd_ssiu_debug_info(struct seq_file *m,
    struct rsnd_dai_stream *io,
    struct rsnd_mod *mod)
    {
    rsnd_debugfs_mod_reg_show(m, mod, RSND_BASE_SSIU,
    rsnd_mod_id(mod) * 0x80, 0x80);
    }

// Macro flag: #define DEBUG_INFO

    static struct rsnd_mod_ops rsnd_ssiu_ops_gen2 = {
    .name		= SSIU_NAME,
    .dma_req	= rsnd_ssiu_dma_req,
    .init		= rsnd_ssiu_init_gen2,
    .quit		= rsnd_ssiu_quit,
    .start		= rsnd_ssiu_start_gen2,
    .stop		= rsnd_ssiu_stop_gen2,
    .get_status	= rsnd_ssiu_get_status,
    DEBUG_INFO
    };
    static struct rsnd_mod *rsnd_ssiu_mod_get(struct rsnd_priv *priv, int id)
    {
    if (WARN_ON(id < 0 || id >= rsnd_ssiu_nr(priv)))
    id = 0;
    return rsnd_mod_get((struct rsnd_ssiu *)(priv.ssiu) + id);
    }
    static void rsnd_parse_connect_ssiu_compatible(struct rsnd_priv *priv,
    struct rsnd_dai_stream *io)
    {
    struct rsnd_mod *ssi_mod = rsnd_io_to_mod_ssi(io);
    struct rsnd_ssiu *ssiu;
    int is_dma_mode;
    int i;
    if (!ssi_mod)
    return;
    is_dma_mode = rsnd_ssi_is_dma_mode(ssi_mod);
// select BUSIF0
    for_each_rsnd_ssiu(ssiu, priv, i) {
    struct rsnd_mod *mod = rsnd_mod_get(ssiu);
    if (is_dma_mode &&
    (rsnd_mod_id(ssi_mod) == rsnd_mod_id(mod)) &&
    (rsnd_mod_id_sub(mod) == 0)) {
    rsnd_dai_connect(mod, io, mod.type);
    return;
    }
    }
    }
    void rsnd_parse_connect_ssiu(struct rsnd_dai *rdai,
    struct device_node *playback,
    struct device_node *capture)
    {
    struct rsnd_priv *priv = rsnd_rdai_to_priv(rdai);
    struct device *dev = rsnd_priv_to_dev(priv);
    struct device_node *node = rsnd_ssiu_of_node(priv);
    struct rsnd_dai_stream *io_p = &rdai.playback;
    struct rsnd_dai_stream *io_c = &rdai.capture;
// use rcar_sound,ssiu if exist
    if (node) {
    let mut i: c_int = 0;
    for_each_child_of_node_scoped(node, np) {
    struct rsnd_mod *mod;
    i = rsnd_node_fixed_index(dev, np, SSIU_NAME, i);
    if (i < 0)
    break;
    mod = rsnd_ssiu_mod_get(priv, i);
    if (np == playback)
    rsnd_dai_connect(mod, io_p, mod.type);
    if (np == capture)
    rsnd_dai_connect(mod, io_c, mod.type);
    i++;
    }
    of_node_put(node);
    }
// Keep DT compatibility
    if (!rsnd_io_to_mod_ssiu(io_p))
    rsnd_parse_connect_ssiu_compatible(priv, io_p);
    if (!rsnd_io_to_mod_ssiu(io_c))
    rsnd_parse_connect_ssiu_compatible(priv, io_c);
    }
#[no_mangle]
pub unsafe extern "C" fn rsnd_ssiu_probe(priv: *mut rsnd_priv) -> c_int {
    int rsnd_ssiu_probe(struct rsnd_priv *priv)
    {
    struct device *dev = rsnd_priv_to_dev(priv);
    struct device_node *node __free(device_node) = rsnd_ssiu_of_node(priv);
    struct reset_control *rstc;
    struct rsnd_ssiu_ctrl *ctrl;
    struct rsnd_ssiu *ssiu;
    struct rsnd_mod_ops *ops;
    const int *list = core::ptr::null_mut();
    int i, nr;
//
// Keep DT compatibility.
// if it has "rcar_sound,ssiu", use it.
// if not, use "rcar_sound,ssi"
// see
// rsnd_ssiu_bufsif_to_id()
//
    if (node)
    nr = rsnd_node_count(priv, node, SSIU_NAME);
    else
    nr = priv.ssi_nr;
    if (!nr)
    return -EINVAL;
    ssiu	= devm_kcalloc(dev, nr, sizeof(*ssiu), GFP_KERNEL);
    if (!ssiu)
    return -ENOMEM;
    ctrl = devm_kzalloc(dev, sizeof(*ctrl), GFP_KERNEL);
    if (!ctrl)
    return -ENOMEM;
    ctrl.busif_status_count = rsnd_flags_has(priv, RSND_SSIU_BUSIF_STATUS_COUNT_2) ? 2 : 4;
    priv.ssiu	= ssiu;
    priv.ssiu_nr	= nr;
    priv.ssiu_ctrl	= ctrl;
    if (rsnd_is_gen1(priv))
    ops = &rsnd_ssiu_ops_gen1;
    else
    ops = &rsnd_ssiu_ops_gen2;
// Keep compatibility
    nr = 0;
    if ((node) &&
    (ops == &rsnd_ssiu_ops_gen2)) {
    ops.id		= rsnd_ssiu_id;
    ops.id_sub	= rsnd_ssiu_id_sub;
    if (rsnd_is_gen2(priv)) {
    list	= gen2_id;
    nr	= ARRAY_SIZE(gen2_id);
    } else if (rsnd_is_gen3(priv)) {
    list	= gen3_id;
    nr	= ARRAY_SIZE(gen3_id);
    } else if (rsnd_is_gen4(priv)) {
    list	= gen4_id;
    nr	= ARRAY_SIZE(gen4_id);
    } else if (rsnd_is_rzg3e(priv)) {
    list	= rzg3e_id;
    nr	= ARRAY_SIZE(rzg3e_id);
    } else {
    dev_err(dev, "unknown SSIU\n");
    return -ENODEV;
    }
    }
// Acquire shared reset once for all SSIU modules
    rstc = devm_reset_control_get_optional_shared(dev, "ssi-all");
    if (IS_ERR(rstc))
    return dev_err_probe(dev, PTR_ERR(rstc),
    "failed to get ssi-all reset\n");
    for_each_rsnd_ssiu(ssiu, priv, i) {
    int ret;
    if (node) {
    int j;
//
// see
// rsnd_ssiu_get_id()
// rsnd_ssiu_get_id_sub()
//
    for (j = 0; j < nr; j++) {
    if (list[j] > i)
    break;
    ssiu.id	= j;
    ssiu.id_sub	= i - list[ssiu.id];
    }
    } else {
    ssiu.id = i;
    }
    ret = rsnd_mod_init(priv, rsnd_mod_get(ssiu),
    ops, core::ptr::null_mut(), rstc, RSND_MOD_SSIU, i);
    if (ret)
    return ret;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn rsnd_ssiu_remove(priv: *mut rsnd_priv) {
    void rsnd_ssiu_remove(struct rsnd_priv *priv)
    {
    struct rsnd_ssiu *ssiu;
    int i;
    for_each_rsnd_ssiu(ssiu, priv, i) {
    rsnd_mod_quit(rsnd_mod_get(ssiu));
    }
    }
#[no_mangle]
pub unsafe extern "C" fn rsnd_ssiu_suspend(priv: *mut rsnd_priv) {
    void rsnd_ssiu_suspend(struct rsnd_priv *priv)
    {
    struct rsnd_ssiu *ssiu;
    int i;
    for_each_rsnd_ssiu(ssiu, priv, i)
    rsnd_suspend_clk_reset(rsnd_mod_get(ssiu).clk,
    rsnd_mod_get(ssiu).rstc);
    }
#[no_mangle]
pub unsafe extern "C" fn rsnd_ssiu_resume(priv: *mut rsnd_priv) {
    void rsnd_ssiu_resume(struct rsnd_priv *priv)
    {
    struct rsnd_ssiu *ssiu;
    int i;
    for_each_rsnd_ssiu(ssiu, priv, i)
    rsnd_resume_clk_reset(rsnd_mod_get(ssiu).clk,
    rsnd_mod_get(ssiu).rstc);
    }
