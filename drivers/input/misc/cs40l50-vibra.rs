//! Automatically rewritten from C to Rust
//! Source: drivers/input/misc/cs40l50-vibra.c
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
// CS40L50 Advanced Haptic Driver with waveform memory,
// integrated DSP, and closed-loop algorithms
//
// Copyright 2024 Cirrus Logic, Inc.
//
// Author: James Ogletree <james.ogletree@cirrus.com>
//

// Wavetables
pub const CS40L50_RAM_INDEX_START: c_uint = 0x1000000;
pub const CS40L50_RAM_INDEX_END: c_uint = 0x100007F;
pub const CS40L50_RTH_INDEX_START: c_uint = 0x1400000;
pub const CS40L50_RTH_INDEX_END: c_uint = 0x1400001;
pub const CS40L50_ROM_INDEX_START: c_uint = 0x1800000;
pub const CS40L50_ROM_INDEX_END: c_uint = 0x180001A;
pub const CS40L50_TYPE_PCM: c_int = 8;
pub const CS40L50_TYPE_PWLE: c_int = 12;
pub const CS40L50_PCM_ID: c_uint = 0x0;
pub const CS40L50_OWT_CUSTOM_DATA_SIZE: c_int = 2;
pub const CS40L50_CUSTOM_DATA_MASK: c_uint = 0xFFFFU;
// DSP
pub const CS40L50_GPIO_BASE: c_uint = 0x2804140;
pub const CS40L50_OWT_BASE: c_uint = 0x2805C34;
pub const CS40L50_OWT_SIZE: c_uint = 0x2805C38;
pub const CS40L50_OWT_NEXT: c_uint = 0x2805C3C;
pub const CS40L50_EFFECTS_MAX: c_int = 1;
// GPIO

pub const CS40L50_GPIO_MAPPING_NONE: c_int = 0;
pub const CS40L50_GPIO_DISABLE: c_uint = 0x1FF;
    enum cs40l50_bank_type {
    CS40L50_WVFRM_BANK_RAM,
    CS40L50_WVFRM_BANK_ROM,
    CS40L50_WVFRM_BANK_OWT,
    CS40L50_WVFRM_BANK_NUM,
    };
// Describes an area in DSP memory populated by effects
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cs40l50_bank {
    pub type: enum cs40l50_bank_type,
    pub base_index: u32,
    pub max_index: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cs40l50_effect {
    pub type: enum cs40l50_bank_type,
    pub list: list_head,
    pub gpio_reg: u32,
    pub index: u32,
    pub id: c_int,
}

// Describes haptic interface of loaded DSP firmware
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cs40l50_vibra_dsp {
    pub banks: *mut cs40l50_bank,
    pub gpio_base_reg: u32,
    pub owt_offset_reg: u32,
    pub owt_size_reg: u32,
    pub owt_base_reg: u32,
    pub push_owt_cmd: u32,
    pub delete_owt_cmd: u32,
    pub stop_cmd: u32,
    pub val): *mut *mut *mut *mut int (write)(struct device dev, struct regmap regmap, u32,
}

// Describes configuration and state of haptic operations
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cs40l50_vibra {
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub input: *mut input_dev,
    pub vib_wq: *mut workqueue_struct,
    pub effect_head: list_head,
    pub dsp: cs40l50_vibra_dsp,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cs40l50_work {
    pub vib: *mut cs40l50_vibra,
    pub effect: *mut ff_effect,
    pub work: work_struct,
    pub custom_data: *mut i16,
    pub custom_len: c_int,
    pub count: c_int,
    pub error: c_int,
}

    static struct cs40l50_bank cs40l50_banks[] = {
    {
    .type =		CS40L50_WVFRM_BANK_RAM,
    .base_index =	CS40L50_RAM_INDEX_START,
    .max_index =	CS40L50_RAM_INDEX_END,
    },
    {
    .type =		CS40L50_WVFRM_BANK_ROM,
    .base_index =	CS40L50_ROM_INDEX_START,
    .max_index =	CS40L50_ROM_INDEX_END,
    },
    {
    .type =		CS40L50_WVFRM_BANK_OWT,
    .base_index =	CS40L50_RTH_INDEX_START,
    .max_index =	CS40L50_RTH_INDEX_END,
    },
    };
    static struct cs40l50_vibra_dsp cs40l50_dsp = {
    .banks =		cs40l50_banks,
    .gpio_base_reg =	CS40L50_GPIO_BASE,
    .owt_base_reg =		CS40L50_OWT_BASE,
    .owt_offset_reg =	CS40L50_OWT_NEXT,
    .owt_size_reg =		CS40L50_OWT_SIZE,
    .push_owt_cmd =		CS40L50_OWT_PUSH,
    .delete_owt_cmd =	CS40L50_OWT_DELETE,
    .stop_cmd =		CS40L50_STOP_PLAYBACK,
    .write =		cs40l50_dsp_write,
    };
    static struct cs40l50_effect *cs40l50_find_effect(int id, struct list_head *effect_head)
    {
    struct cs40l50_effect *effect;
    list_for_each_entry(effect, effect_head, list)
    if (effect.id == id)
    return effect;
    return core::ptr::null_mut();
    }
    static int cs40l50_effect_bank_set(struct cs40l50_work *work_data,
    struct cs40l50_effect *effect)
    {
    let mut bank_type: u32 = work_data.custom_data[0] & CS40L50_CUSTOM_DATA_MASK;
    if (bank_type >= CS40L50_WVFRM_BANK_NUM) {
    dev_err(work_data.vib.dev, "Invalid bank (%u)\n", bank_type);
    return -EINVAL;
    }
    if (work_data.custom_len > CS40L50_OWT_CUSTOM_DATA_SIZE)
    effect.type = CS40L50_WVFRM_BANK_OWT;
    else
    effect.type = bank_type;
    return 0;
    }
    static int cs40l50_effect_index_set(struct cs40l50_work *work_data,
    struct cs40l50_effect *effect)
    {
    struct cs40l50_vibra *vib = work_data.vib;
    struct cs40l50_effect *owt_effect;
    u32 base_index, max_index;
    base_index = vib.dsp.banks[effect.type].base_index;
    max_index = vib.dsp.banks[effect.type].max_index;
    effect.index = base_index;
    switch (effect.type) {
    case CS40L50_WVFRM_BANK_OWT:
    list_for_each_entry(owt_effect, &vib.effect_head, list)
    if (owt_effect.type == CS40L50_WVFRM_BANK_OWT)
    effect.index++;
    break;
    case CS40L50_WVFRM_BANK_ROM:
    case CS40L50_WVFRM_BANK_RAM:
    effect.index += work_data.custom_data[1] & CS40L50_CUSTOM_DATA_MASK;
    break;
    default:
    dev_err(vib.dev, "Bank type %d not supported\n", effect.type);
    return -EINVAL;
    }
    if (effect.index > max_index || effect.index < base_index) {
    dev_err(vib.dev, "Index out of bounds: %u\n", effect.index);
    return -ENOSPC;
    }
    return 0;
    }
    static int cs40l50_effect_gpio_mapping_set(struct cs40l50_work *work_data,
    struct cs40l50_effect *effect)
    {
    u16 gpio_edge, gpio_num, button = work_data.effect.trigger.button;
    struct cs40l50_vibra *vib = work_data.vib;
    if (button) {
    gpio_num = FIELD_GET(CS40L50_GPIO_NUM_MASK, button);
    gpio_edge = FIELD_GET(CS40L50_GPIO_EDGE_MASK, button);
    effect.gpio_reg = vib.dsp.gpio_base_reg + (gpio_num * 8) - gpio_edge;
    return regmap_write(vib.regmap, effect.gpio_reg, button);
    }
    effect.gpio_reg = CS40L50_GPIO_MAPPING_NONE;
    return 0;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cs40l50_owt_header {
    pub type: u32,
    pub data_words: u32,
    pub offset: u32,
    pub __packed: },
#[no_mangle]
unsafe extern "C" fn cs40l50_upload_owt(work_data: *mut cs40l50_work) -> c_int {
    static int cs40l50_upload_owt(struct cs40l50_work *work_data)
    {
    pub NULL: *mut *mut u8 new_owt_effect_data __free(kfree) =,
    pub work_data->vib: *mut *mut cs40l50_vibra vib =,
    pub 2: *mut *mut size_t len = work_data->custom_len,
    pub header: cs40l50_owt_header,
    pub size: u32 offset,,
    pub error: c_int,
    pub &size): error = regmap_read(vib->regmap, vib->dsp.owt_size_reg,,
    if (error)
    pub error: return,
    if ((size * sizeof(u32)) < sizeof(header) + len) {
    pub effect\n"): dev_err(vib->dev, "No space in open wavetable for,
    pub -ENOSPC: return,
    }
    header.type = work_data.custom_data[0] == CS40L50_PCM_ID ? CS40L50_TYPE_PCM :
    pub sizeof(u32): header.offset = sizeof(header) /,
    pub sizeof(u32): header.data_words = len /,
    pub GFP_KERNEL): new_owt_effect_data = kmalloc(sizeof(header) + len,,
    if (!new_owt_effect_data)
    pub -ENOMEM: return,
    pub sizeof(header)): memcpy(new_owt_effect_data, &header,,
    pub len): memcpy(new_owt_effect_data + sizeof(header), work_data->custom_data,,
    pub &offset): error = regmap_read(vib->regmap, vib->dsp.owt_offset_reg,,
    if (error)
    pub error: return,
    error = regmap_bulk_write(vib.regmap, vib.dsp.owt_base_reg +
    (offset * sizeof(u32)), new_owt_effect_data,
    pub len): sizeof(header) +,
    if (error)
    pub error: return,
    pub vib->dsp.push_owt_cmd): error = vib->dsp.write(vib->dev, vib->regmap,,
    if (error)
    pub error: return,
    pub 0: return,
    }
#[no_mangle]
unsafe extern "C" fn cs40l50_add_worker(work: *mut work_struct) {
    static void cs40l50_add_worker(struct work_struct *work)
    {
    pub work): *mut *mut cs40l50_work work_data = container_of(work, cs40l50_work,,
    pub work_data->vib: *mut *mut cs40l50_vibra vib =,
    pub effect: *mut cs40l50_effect,
    pub false: bool is_new =,
    pub error: c_int,
    pub pm_runtime_resume_and_get(vib->dev): error =,
    if (error)
    pub err_exit: goto,
// Update effect if already uploaded, otherwise create new effect
    pub &vib->effect_head): effect = cs40l50_find_effect(work_data->effect->id,,
    if (!effect) {
    pub kzalloc_obj(*effect): *mut effect =,
    if (!effect) {
    pub -ENOMEM: error =,
    pub err_pm: goto,
    }
    pub work_data->effect->id: effect->id =,
    pub true: is_new =,
    }
    pub effect): error = cs40l50_effect_bank_set(work_data,,
    if (error)
    pub err_free: goto,
    pub effect): error = cs40l50_effect_index_set(work_data,,
    if (error)
    pub err_free: goto,
    pub effect): error = cs40l50_effect_gpio_mapping_set(work_data,,
    if (error)
    pub err_free: goto,
    if (effect.type == CS40L50_WVFRM_BANK_OWT)
    pub cs40l50_upload_owt(work_data): error =,
    err_free:
    if (is_new) {
    if (error)
    else
    pub &vib->effect_head): list_add(&effect->list,,
    }
    err_pm:
    err_exit:
    pub error: work_data->error =,
    }
    static int cs40l50_add(struct input_dev *dev, struct ff_effect *effect,
    struct ff_effect *old)
    {
    pub &effect->u.periodic: *mut *mut ff_periodic_effect periodic =,
    pub input_get_drvdata(dev): *mut *mut cs40l50_vibra vib =,
    pub work_data: cs40l50_work,
    if (effect.type != FF_PERIODIC || periodic.waveform != FF_CUSTOM) {
    dev_err(vib.dev, "Type (%#X) or waveform (%#X) unsupported\n",
    pub periodic->waveform): effect->type,,
    pub -EINVAL: return,
    }
    if (periodic.custom_len < CS40L50_OWT_CUSTOM_DATA_SIZE) {
    dev_err(vib.dev, "Invalid custom data length (%u)\n",
    pub -EINVAL: return,
    }
    work_data.custom_data = memdup_array_user(effect.u.periodic.custom_data,
    effect.u.periodic.custom_len,
    if (IS_ERR(work_data.custom_data))
    pub PTR_ERR(work_data.custom_data): return,
    pub effect->u.periodic.custom_len: work_data.custom_len =,
    pub vib: work_data.vib =,
    pub effect: work_data.effect =,
    pub cs40l50_add_worker): INIT_WORK_ONSTACK(&work_data.work,,
// Push to the workqueue to serialize with playbacks
    pub &work_data.work): queue_work(vib->vib_wq,,
    pub work_data.error: return,
    }
#[no_mangle]
unsafe extern "C" fn cs40l50_start_worker(work: *mut work_struct) {
    static void cs40l50_start_worker(struct work_struct *work)
    {
    pub work): *mut *mut cs40l50_work work_data = container_of(work, cs40l50_work,,
    pub work_data->vib: *mut *mut cs40l50_vibra vib =,
    pub start_effect: *mut cs40l50_effect,
    if (pm_runtime_resume_and_get(vib.dev) < 0)
    pub err_free: goto,
    pub &vib->effect_head): start_effect = cs40l50_find_effect(work_data->effect->id,,
    if (start_effect) {
    while (--work_data.count >= 0) {
    pub start_effect->index): vib->dsp.write(vib->dev, vib->regmap,,
    usleep_range(work_data.effect.replay.length,
    pub 100): work_data->effect->replay.length +,
    }
    } else {
    pub found\n"): dev_err(vib->dev, "Effect to play not,
    }
    err_free:
    }
#[no_mangle]
unsafe extern "C" fn cs40l50_stop_worker(work: *mut work_struct) {
    static void cs40l50_stop_worker(struct work_struct *work)
    {
    pub work): *mut *mut cs40l50_work work_data = container_of(work, cs40l50_work,,
    pub work_data->vib: *mut *mut cs40l50_vibra vib =,
    if (pm_runtime_resume_and_get(vib.dev) < 0)
    pub vib->dsp.stop_cmd): vib->dsp.write(vib->dev, vib->regmap,,
    }
#[no_mangle]
unsafe extern "C" fn cs40l50_playback(dev: *mut input_dev, effect_id: c_int, val: c_int) -> c_int {
    static int cs40l50_playback(struct input_dev *dev, int effect_id, int val)
    {
    pub input_get_drvdata(dev): *mut *mut cs40l50_vibra vib =,
    pub work_data: *mut cs40l50_work,
    pub GFP_ATOMIC): *mut *mut work_data = kzalloc_obj(work_data,,
    if (!work_data)
    pub -ENOMEM: return,
    pub vib: work_data->vib =,
    if (val > 0) {
    pub &dev->ff->effects[effect_id]: work_data->effect =,
    pub val: work_data->count =,
    pub cs40l50_start_worker): INIT_WORK(&work_data->work,,
    } else {
// Stop the amplifier as device drives only one effect
    pub cs40l50_stop_worker): INIT_WORK(&work_data->work,,
    }
    pub &work_data->work): queue_work(vib->vib_wq,,
    pub 0: return,
    }
#[no_mangle]
unsafe extern "C" fn cs40l50_erase_worker(work: *mut work_struct) {
    static void cs40l50_erase_worker(struct work_struct *work)
    {
    pub work): *mut *mut cs40l50_work work_data = container_of(work, cs40l50_work,,
    pub owt_effect: *mut *mut cs40l50_effect erase_effect,,
    pub work_data->vib: *mut *mut cs40l50_vibra vib =,
    pub error: c_int,
    pub pm_runtime_resume_and_get(vib->dev): error =,
    if (error)
    pub err_exit: goto,
    pub &vib->effect_head): erase_effect = cs40l50_find_effect(work_data->effect->id,,
    if (!erase_effect) {
    pub found\n"): dev_err(vib->dev, "Effect to erase not,
    pub -EINVAL: error =,
    pub err_pm: goto,
    }
    if (erase_effect.gpio_reg != CS40L50_GPIO_MAPPING_NONE) {
    error = regmap_write(vib.regmap, erase_effect.gpio_reg,
    if (error)
    pub err_pm: goto,
    }
    if (erase_effect.type == CS40L50_WVFRM_BANK_OWT) {
    error = vib.dsp.write(vib.dev, vib.regmap,
    vib.dsp.delete_owt_cmd |
    pub 0xFF)): (erase_effect->index &,
    if (error)
    pub err_pm: goto,
    list_for_each_entry(owt_effect, &vib.effect_head, list)
    if (owt_effect.type == CS40L50_WVFRM_BANK_OWT &&
    owt_effect.index > erase_effect.index)
    }
    err_pm:
    err_exit:
    pub error: work_data->error =,
    }
#[no_mangle]
unsafe extern "C" fn cs40l50_erase(dev: *mut input_dev, effect_id: c_int) -> c_int {
    static int cs40l50_erase(struct input_dev *dev, int effect_id)
    {
    pub input_get_drvdata(dev): *mut *mut cs40l50_vibra vib =,
    pub work_data: cs40l50_work,
    pub vib: work_data.vib =,
    pub &dev->ff->effects[effect_id]: work_data.effect =,
    pub cs40l50_erase_worker): INIT_WORK_ONSTACK(&work_data.work,,
// Push to workqueue to serialize with playbacks
    pub &work_data.work): queue_work(vib->vib_wq,,
    pub work_data.error: return,
    }
#[no_mangle]
unsafe extern "C" fn cs40l50_remove_wq(data: *mut c_void) {
    static void cs40l50_remove_wq(void *data)
    {
    }
#[no_mangle]
unsafe extern "C" fn cs40l50_vibra_probe(pdev: *mut platform_device) -> c_int {
    static int cs40l50_vibra_probe(struct platform_device *pdev)
    {
    pub dev_get_drvdata(pdev->dev.parent): *mut *mut cs40l50 cs40l50 =,
    pub vib: *mut cs40l50_vibra,
    pub error: c_int,
    pub GFP_KERNEL): *mut *mut vib = devm_kzalloc(pdev->dev.parent, sizeof(vib),,
    if (!vib)
    pub -ENOMEM: return,
    pub cs40l50->dev: vib->dev =,
    pub cs40l50->regmap: vib->regmap =,
    pub cs40l50_dsp: vib->dsp =,
    pub devm_input_allocate_device(vib->dev): vib->input =,
    if (!vib.input)
    pub -ENOMEM: return,
    pub cs40l50->devid: vib->input->id.product =,
    pub cs40l50->revid: vib->input->id.version =,
    pub "cs40l50_vibra": vib->input->name =,
    pub vib): input_set_drvdata(vib->input,,
    pub FF_PERIODIC): input_set_capability(vib->input, EV_FF,,
    pub FF_CUSTOM): input_set_capability(vib->input, EV_FF,,
    pub CS40L50_EFFECTS_MAX): error = input_ff_create(vib->input,,
    if (error) {
    pub device\n"): dev_err(vib->dev, "Failed to create input,
    pub error: return,
    }
    pub cs40l50_add: vib->input->ff->upload =,
    pub cs40l50_playback: vib->input->ff->playback =,
    pub cs40l50_erase: vib->input->ff->erase =,
    pub WQ_HIGHPRI): vib->vib_wq = alloc_ordered_workqueue("vib_wq",,
    if (!vib.vib_wq)
    pub -ENOMEM: return,
    pub vib->vib_wq): error = devm_add_action_or_reset(vib->dev, cs40l50_remove_wq,,
    if (error)
    pub error: return,
    pub input_register_device(vib->input): error =,
    if (error)
    pub error: return,
    pub 0: return,
    }
    static const struct platform_device_id cs40l50_vibra_id_match[] = {
    { "cs40l50-vibra", },
    {}
}

    MODULE_DEVICE_TABLE(platform, cs40l50_vibra_id_match);
    static struct platform_driver cs40l50_vibra_driver = {
    .probe		= cs40l50_vibra_probe,
    .id_table	= cs40l50_vibra_id_match,
    .driver		= {
    .name	= "cs40l50-vibra",
    },
    };
    module_platform_driver(cs40l50_vibra_driver);
    MODULE_DESCRIPTION("CS40L50 Advanced Haptic Driver");
    MODULE_AUTHOR("James Ogletree, Cirrus Logic Inc. <james.ogletree@cirrus.com>");
    MODULE_LICENSE("GPL");
