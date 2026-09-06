//! Automatically rewritten from C to Rust
//! Source: sound/soc/sdca/sdca_jack.c
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
// Copyright (C) 2025 Cirrus Logic, Inc. and
// Cirrus Logic International Semiconductor Ltd.
//
// The MIPI SDCA specification is available for public downloads at
// https://www.mipi.org/mipi-sdca-v1-0-download
//

//
// sdca_jack_process - Process an SDCA jack event
// @interrupt: SDCA interrupt structure
//
// Return: Zero on success or a negative error code.
//
#[no_mangle]
pub unsafe extern "C" fn sdca_jack_process(interrupt: *mut sdca_interrupt) -> c_int {
    int sdca_jack_process(struct sdca_interrupt *interrupt)
    {
    struct device *dev = interrupt.dev;
    struct snd_soc_component *component = interrupt.component;
    struct snd_soc_card *card = component.card;
    struct rw_semaphore *rwsem = &card.snd_card.controls_rwsem;
    struct jack_state *state = interrupt.priv;
    struct snd_kcontrol *kctl = state.kctl;
    struct snd_ctl_elem_value *ucontrol __free(kfree) = core::ptr::null_mut();
    struct soc_enum *soc_enum;
    unsigned int reg, val;
    int ret;
    guard(rwsem_write)(rwsem);
    reg = SDW_SDCA_CTL(interrupt.function.desc.adr, interrupt.entity.id,
    interrupt.control.sel, 0);
    ret = regmap_read(interrupt.function_regmap, reg, &val);
    if (ret < 0) {
    dev_err(dev, "failed to read detected mode: %d\n", ret);
    return ret;
    }
    reg = SDW_SDCA_CTL(interrupt.function.desc.adr, interrupt.entity.id,
    SDCA_CTL_GE_SELECTED_MODE, 0);
    switch (val) {
    case SDCA_DETECTED_MODE_DETECTION_IN_PROGRESS:
    case SDCA_DETECTED_MODE_JACK_UNKNOWN:
//
// Selected mode is not normally marked as volatile register
// (RW), but here force a read from the hardware. If the
// detected mode is unknown we need to see what the device
// selected as a "safe" option.
//
    regcache_drop_region(interrupt.function_regmap, reg, reg);
    ret = regmap_read(interrupt.function_regmap, reg, &val);
    if (ret) {
    dev_err(dev, "failed to re-check selected mode: %d\n", ret);
    return ret;
    }
    break;
    default:
    break;
    }
    dev_dbg(dev, "%s: %#x\n", interrupt.name, val);
    ucontrol = kzalloc_obj(*ucontrol);
    if (!ucontrol)
    return -ENOMEM;
    soc_enum = (struct soc_enum *)kctl.private_value;
    ucontrol.value.enumerated.item[0] = snd_soc_enum_val_to_item(soc_enum, val);
    ret = snd_soc_dapm_put_enum_double(kctl, ucontrol);
    if (ret < 0) {
    dev_err(dev, "failed to update selected mode: %d\n", ret);
    return ret;
    }
    snd_ctl_notify(card.snd_card, SNDRV_CTL_EVENT_MASK_VALUE, &kctl.id);
    return sdca_jack_report(interrupt);
    }
    EXPORT_SYMBOL_NS_GPL(sdca_jack_process, "SND_SOC_SDCA");
//
// sdca_jack_alloc_state - allocate state for a jack interrupt
// @interrupt: SDCA interrupt structure.
//
// Return: Zero on success or a negative error code.
//
#[no_mangle]
pub unsafe extern "C" fn sdca_jack_alloc_state(interrupt: *mut sdca_interrupt) -> c_int {
    int sdca_jack_alloc_state(struct sdca_interrupt *interrupt)
    {
    struct jack_state *jack_state;
    jack_state = kzalloc_obj(*jack_state);
    if (!jack_state)
    return -ENOMEM;
    interrupt.priv = jack_state;
    return 0;
    }
    EXPORT_SYMBOL_NS_GPL(sdca_jack_alloc_state, "SND_SOC_SDCA");
//
// sdca_jack_free_state - free state for a jack interrupt
// @interrupt: SDCA interrupt structure.
//
#[no_mangle]
pub unsafe extern "C" fn sdca_jack_free_state(interrupt: *mut sdca_interrupt) {
    void sdca_jack_free_state(struct sdca_interrupt *interrupt)
    {
    kfree(interrupt.priv);
    }
    EXPORT_SYMBOL_NS_GPL(sdca_jack_free_state, "SND_SOC_SDCA");
//
// sdca_jack_init_state - Initialise transient state for a jack interrupt
// @interrupt: SDCA interrupt structure.
//
// Return: Zero on success or a negative error code.
//
#[no_mangle]
pub unsafe extern "C" fn sdca_jack_init_state(interrupt: *mut sdca_interrupt) -> c_int {
    int sdca_jack_init_state(struct sdca_interrupt *interrupt)
    {
    struct jack_state *jack_state = interrupt.priv;
    const char *name __free(kfree) = kasprintf(GFP_KERNEL, "%s %s",
    interrupt.entity.label,
    SDCA_CTL_SELECTED_MODE_NAME);
    if (!name)
    return -ENOMEM;
    jack_state.kctl = snd_soc_component_get_kcontrol(interrupt.component, name);
    if (!jack_state.kctl) {
    dev_err(interrupt.dev, "control not found: %s\n", name);
    return -ENODEV;
    }
    return 0;
    }
    EXPORT_SYMBOL_NS_GPL(sdca_jack_init_state, "SND_SOC_SDCA");
#[no_mangle]
unsafe extern "C" fn type_get_mask(type: enum sdca_terminal_type) -> c_int {
    static int type_get_mask(enum sdca_terminal_type type)
    {
    switch (type) {
    case SDCA_TERM_TYPE_LINEIN_STEREO:
    case SDCA_TERM_TYPE_LINEIN_FRONT_LR:
    case SDCA_TERM_TYPE_LINEIN_CENTER_LFE:
    case SDCA_TERM_TYPE_LINEIN_SURROUND_LR:
    case SDCA_TERM_TYPE_LINEIN_REAR_LR:
    return SND_JACK_LINEIN;
    case SDCA_TERM_TYPE_LINEOUT_STEREO:
    case SDCA_TERM_TYPE_LINEOUT_FRONT_LR:
    case SDCA_TERM_TYPE_LINEOUT_CENTER_LFE:
    case SDCA_TERM_TYPE_LINEOUT_SURROUND_LR:
    case SDCA_TERM_TYPE_LINEOUT_REAR_LR:
    return SND_JACK_LINEOUT;
    case SDCA_TERM_TYPE_MIC_JACK:
    return SND_JACK_MICROPHONE;
    case SDCA_TERM_TYPE_HEADPHONE_JACK:
    return SND_JACK_HEADPHONE;
    case SDCA_TERM_TYPE_HEADSET_JACK:
    return SND_JACK_HEADSET;
    default:
    return 0;
    }
    }
//
// sdca_jack_set_jack - attach an ASoC jack to SDCA
// @info: SDCA interrupt information.
// @jack: ASoC jack to be attached.
//
// Return: Zero on success or a negative error code.
//
#[no_mangle]
pub unsafe extern "C" fn sdca_jack_set_jack(info: *mut sdca_interrupt_info, jack: *mut snd_soc_jack) -> c_int {
    int sdca_jack_set_jack(struct sdca_interrupt_info *info, struct snd_soc_jack *jack)
    {
    int i, j;
    int ret;
    guard(mutex)(&info.irq_lock);
    for (i = 0; i < SDCA_MAX_INTERRUPTS; i++) {
    struct sdca_interrupt *interrupt = &info.irqs[i];
    struct sdca_control *control = interrupt.control;
    struct sdca_entity *entity = interrupt.entity;
    struct sdca_control_range *range;
    struct jack_state *jack_state;
    if (!interrupt.dev)
    continue;
    switch (SDCA_CTL_TYPE(entity.type, control.sel)) {
    case SDCA_CTL_TYPE_S(GE, DETECTED_MODE):
    range = sdca_selector_find_range(interrupt.dev, entity,
    SDCA_CTL_GE_SELECTED_MODE,
    SDCA_SELECTED_MODE_NCOLS, 0);
    if (!range)
    return -EINVAL;
    jack_state = interrupt.priv;
    for (j = 0; j < range.rows; j++) {
    enum sdca_terminal_type type;
    type = sdca_range(range, SDCA_SELECTED_MODE_TERM_TYPE, j);
    jack_state.mask |= type_get_mask(type);
    }
    jack_state.jack = jack;
// Report initial state in case IRQ was already handled
    ret = sdca_jack_report(interrupt);
    if (ret)
    return ret;
    break;
    default:
    break;
    }
    }
    return 0;
    }
    EXPORT_SYMBOL_NS_GPL(sdca_jack_set_jack, "SND_SOC_SDCA");
#[no_mangle]
pub unsafe extern "C" fn sdca_jack_report(interrupt: *mut sdca_interrupt) -> c_int {
    int sdca_jack_report(struct sdca_interrupt *interrupt)
    {
    struct jack_state *jack_state = interrupt.priv;
    struct sdca_control_range *range;
    enum sdca_terminal_type type;
    unsigned int reg, val;
    int ret;
    reg = SDW_SDCA_CTL(interrupt.function.desc.adr, interrupt.entity.id,
    SDCA_CTL_GE_SELECTED_MODE, 0);
    ret = regmap_read(interrupt.function_regmap, reg, &val);
    if (ret) {
    dev_err(interrupt.dev, "failed to read selected mode: %d\n", ret);
    return ret;
    }
    range = sdca_selector_find_range(interrupt.dev, interrupt.entity,
    SDCA_CTL_GE_SELECTED_MODE,
    SDCA_SELECTED_MODE_NCOLS, 0);
    if (!range)
    return -EINVAL;
    type = sdca_range_search(range, SDCA_SELECTED_MODE_INDEX,
    val, SDCA_SELECTED_MODE_TERM_TYPE);
    snd_soc_jack_report(jack_state.jack, type_get_mask(type), jack_state.mask);
    return 0;
    }
    EXPORT_SYMBOL_NS_GPL(sdca_jack_report, "SND_SOC_SDCA");
