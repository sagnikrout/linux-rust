//! Automatically rewritten from C to Rust
//! Source: sound/soc/sdca/sdca_functions.c
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


// SPDX-License-Identifier: (GPL-2.0 OR BSD-3-Clause)
// Copyright(c) 2024 Intel Corporation
//
// The MIPI SDCA specification is available for public downloads at
// https://www.mipi.org/mipi-sdca-v1-0-download
//

//
// Should be long enough to encompass all the MIPI DisCo properties.
//
pub const SDCA_PROPERTY_LENGTH: c_int = 64;
#[no_mangle]
unsafe extern "C" fn patch_sdca_function_type(interface_revision: u32, function_type: *mut u32) -> c_int {
    static int patch_sdca_function_type(u32 interface_revision, u32 *function_type)
    {
//
// Unfortunately early SDCA specifications used different indices for Functions,
// for backwards compatibility we have to reorder the values found.
//
    if (interface_revision < 0x0801) {
    switch (*function_type) {
    case 1:
// function_type = SDCA_FUNCTION_TYPE_SMART_AMP;
    break;
    case 2:
// function_type = SDCA_FUNCTION_TYPE_SMART_MIC;
    break;
    case 3:
// function_type = SDCA_FUNCTION_TYPE_SPEAKER_MIC;
    break;
    case 4:
// function_type = SDCA_FUNCTION_TYPE_UAJ;
    break;
    case 5:
// function_type = SDCA_FUNCTION_TYPE_RJ;
    break;
    case 6:
// function_type = SDCA_FUNCTION_TYPE_HID;
    break;
    default:
    return -EINVAL;
    }
    }
    return 0;
    }
    static const char *get_sdca_function_name(u32 function_type)
    {
    switch (function_type) {
    case SDCA_FUNCTION_TYPE_SMART_AMP:
    return SDCA_FUNCTION_TYPE_SMART_AMP_NAME;
    case SDCA_FUNCTION_TYPE_SMART_MIC:
    return SDCA_FUNCTION_TYPE_SMART_MIC_NAME;
    case SDCA_FUNCTION_TYPE_UAJ:
    return SDCA_FUNCTION_TYPE_UAJ_NAME;
    case SDCA_FUNCTION_TYPE_HID:
    return SDCA_FUNCTION_TYPE_HID_NAME;
    case SDCA_FUNCTION_TYPE_SIMPLE_AMP:
    return SDCA_FUNCTION_TYPE_SIMPLE_AMP_NAME;
    case SDCA_FUNCTION_TYPE_SIMPLE_MIC:
    return SDCA_FUNCTION_TYPE_SIMPLE_MIC_NAME;
    case SDCA_FUNCTION_TYPE_SPEAKER_MIC:
    return SDCA_FUNCTION_TYPE_SPEAKER_MIC_NAME;
    case SDCA_FUNCTION_TYPE_RJ:
    return SDCA_FUNCTION_TYPE_RJ_NAME;
    case SDCA_FUNCTION_TYPE_COMPANION_AMP:
    return SDCA_FUNCTION_TYPE_COMPANION_AMP_NAME;
    case SDCA_FUNCTION_TYPE_IMP_DEF:
    return SDCA_FUNCTION_TYPE_IMP_DEF_NAME;
    default:
    return core::ptr::null_mut();
    }
    }
#[no_mangle]
unsafe extern "C" fn find_sdca_function(adev: *mut acpi_device, data: *mut c_void) -> c_int {
    static int find_sdca_function(struct acpi_device *adev, void *data)
    {
    struct fwnode_handle *function_node = acpi_fwnode_handle(adev);
    struct sdca_device_data *sdca_data = data;
    struct sdw_slave *slave = container_of(sdca_data, struct sdw_slave, sdca_data);
    struct device *dev = &adev.dev;
    struct fwnode_handle *control5; /* used to identify function type */
    const char *function_name;
    u32 function_type;
    int function_index;
    u64 addr;
    int i, ret;
    if (sdca_data.num_functions >= SDCA_MAX_FUNCTION_COUNT) {
    dev_err(dev, "maximum number of functions exceeded\n");
    return -EINVAL;
    }
    ret = acpi_get_local_u64_address(adev.handle, &addr);
    if (ret < 0)
    return ret;
    if (!addr || addr > 0x7) {
    dev_err(dev, "invalid addr: 0x%llx\n", addr);
    return -ENODEV;
    }
//
// Extracting the topology type for an SDCA function is a
// convoluted process.
// The Function type is only visible as a result of a read
// from a control. In theory this would mean reading from the hardware,
// but the SDCA/DisCo specs defined the notion of "DC value" - a constant
// represented with a DSD subproperty.
// Drivers have to query the properties for the control
// SDCA_CONTROL_ENTITY_0_FUNCTION_TOPOLOGY (0x05)
//
    control5 = fwnode_get_named_child_node(function_node,
    "mipi-sdca-control-0x5-subproperties");
    if (!control5)
    return -ENODEV;
    ret = fwnode_property_read_u32(control5, "mipi-sdca-control-dc-value",
    &function_type);
    fwnode_handle_put(control5);
    if (ret < 0) {
    dev_err(dev, "function type only supported as DisCo constant\n");
    return ret;
    }
    if (!sdca_device_quirk_match(slave, SDCA_QUIRKS_SKIP_FUNC_TYPE_PATCHING)) {
    ret = patch_sdca_function_type(sdca_data.interface_revision, &function_type);
    if (ret < 0) {
    dev_err(dev, "SDCA version %#x invalid function type %d\n",
    sdca_data.interface_revision, function_type);
    return ret;
    }
    }
    function_name = get_sdca_function_name(function_type);
    if (!function_name) {
    dev_err(dev, "invalid SDCA function type %d\n", function_type);
    return -EINVAL;
    }
    dev_info(dev, "SDCA function %s (type %d) at 0x%llx\n",
    function_name, function_type, addr);
// store results
    function_index = sdca_data.num_functions;
    for (i = 0; i < function_index; i++) {
    if (sdca_data.function[i].type == function_type) {
    sdca_data.function[function_index].duplicate = true;
    break;
    }
    }
    sdca_data.function[function_index].adr = addr;
    sdca_data.function[function_index].type = function_type;
    sdca_data.function[function_index].name = function_name;
    sdca_data.function[function_index].node = function_node;
    sdca_data.num_functions++;
    return 0;
    }
//
// sdca_lookup_functions - Parse sdca_device_desc for each Function
// @slave: SoundWire slave device to be processed.
//
// Iterate through the available SDCA Functions and fill in a short
// descriptor (struct sdca_function_desc) for each function, this
// information is stored along with the SoundWire slave device and
// used for adding drivers and quirks before the devices have fully
// probed.
//
#[no_mangle]
pub unsafe extern "C" fn sdca_lookup_functions(slave: *mut sdw_slave) {
    void sdca_lookup_functions(struct sdw_slave *slave)
    {
    struct device *sdev = &slave.dev;
    struct acpi_device *adev = to_acpi_device_node(sdev.fwnode);
    if (!adev) {
    dev_info(sdev, "no matching ACPI device found, ignoring peripheral\n");
    return;
    }
    acpi_dev_for_each_child(adev, find_sdca_function, &slave.sdca_data);
    }
    EXPORT_SYMBOL_NS(sdca_lookup_functions, "SND_SOC_SDCA");
#[repr(C)]
#[derive(Copy, Clone)]
pub struct raw_init_write {
    pub addr: __le32,
    pub val: u8,
    pub __packed: },
    static int find_sdca_init_table(struct device *dev,
    struct fwnode_handle *function_node,
    struct sdca_function_data *function)
    {
    pub NULL: *mut *mut raw_init_write raw __free(kfree) =,
    pub init_write: *mut sdca_init_write,
    pub num_init_writes: int i,,
    num_init_writes = fwnode_property_count_u8(function_node,
    if (!num_init_writes || num_init_writes == -EINVAL) {
    pub 0: return,
    } else if (num_init_writes < 0) {
    dev_err(dev, "%pfwP: failed to read initialization table: %d\n",
    pub num_init_writes): function_node,,
    pub num_init_writes: return,
    } else if (num_init_writes % sizeof(*raw) != 0) {
    pub function_node): dev_err(dev, "%pfwP: init table size invalid\n",,
    pub -EINVAL: return,
    }
    pub GFP_KERNEL): raw = kzalloc(num_init_writes,,
    if (!raw)
    pub -ENOMEM: return,
    fwnode_property_read_u8_array(function_node,
    "mipi-sdca-function-initialization-table",
    pub num_init_writes): *mut *mut (u8 )raw,,
    pub sizeof(*raw): *mut num_init_writes /=,
    pub GFP_KERNEL): *mut *mut init_write = devm_kcalloc(dev, num_init_writes, sizeof(init_write),,
    if (!init_write)
    pub -ENOMEM: return,
    pub {: for (i = 0; i < num_init_writes; i++),
    pub le32_to_cpu(raw[i].addr): init_write[i].addr =,
    pub raw[i].val: init_write[i].val =,
    }
    pub num_init_writes: function->num_init_table =,
    pub init_write: function->init_table =,
    pub 0: return,
    }
    static const char *find_sdca_control_label(struct device *dev,
    const struct sdca_entity *entity,
    const struct sdca_control *control)
    {
    switch (SDCA_CTL_TYPE(entity.type, control.sel)) {
    case SDCA_CTL_TYPE_S(IT, MIC_BIAS):
    pub SDCA_CTL_MIC_BIAS_NAME: return,
    case SDCA_CTL_TYPE_S(IT, USAGE):
    case SDCA_CTL_TYPE_S(OT, USAGE):
    pub SDCA_CTL_USAGE_NAME: return,
    case SDCA_CTL_TYPE_S(IT, LATENCY):
    case SDCA_CTL_TYPE_S(OT, LATENCY):
    case SDCA_CTL_TYPE_S(MU, LATENCY):
    case SDCA_CTL_TYPE_S(SU, LATENCY):
    case SDCA_CTL_TYPE_S(FU, LATENCY):
    case SDCA_CTL_TYPE_S(XU, LATENCY):
    case SDCA_CTL_TYPE_S(CRU, LATENCY):
    case SDCA_CTL_TYPE_S(UDMPU, LATENCY):
    case SDCA_CTL_TYPE_S(MFPU, LATENCY):
    case SDCA_CTL_TYPE_S(SMPU, LATENCY):
    case SDCA_CTL_TYPE_S(SAPU, LATENCY):
    case SDCA_CTL_TYPE_S(PPU, LATENCY):
    pub SDCA_CTL_LATENCY_NAME: return,
    case SDCA_CTL_TYPE_S(IT, CLUSTERINDEX):
    case SDCA_CTL_TYPE_S(CRU, CLUSTERINDEX):
    case SDCA_CTL_TYPE_S(UDMPU, CLUSTERINDEX):
    case SDCA_CTL_TYPE_S(MFPU, CLUSTERINDEX):
    pub SDCA_CTL_CLUSTERINDEX_NAME: return,
    case SDCA_CTL_TYPE_S(IT, DATAPORT_SELECTOR):
    case SDCA_CTL_TYPE_S(OT, DATAPORT_SELECTOR):
    pub SDCA_CTL_DATAPORT_SELECTOR_NAME: return,
    case SDCA_CTL_TYPE_S(IT, MATCHING_GUID):
    case SDCA_CTL_TYPE_S(OT, MATCHING_GUID):
    case SDCA_CTL_TYPE_S(ENTITY_0, MATCHING_GUID):
    pub SDCA_CTL_MATCHING_GUID_NAME: return,
    case SDCA_CTL_TYPE_S(IT, KEEP_ALIVE):
    case SDCA_CTL_TYPE_S(OT, KEEP_ALIVE):
    pub SDCA_CTL_KEEP_ALIVE_NAME: return,
    case SDCA_CTL_TYPE_S(IT, NDAI_STREAM):
    case SDCA_CTL_TYPE_S(OT, NDAI_STREAM):
    pub SDCA_CTL_NDAI_STREAM_NAME: return,
    case SDCA_CTL_TYPE_S(IT, NDAI_CATEGORY):
    case SDCA_CTL_TYPE_S(OT, NDAI_CATEGORY):
    pub SDCA_CTL_NDAI_CATEGORY_NAME: return,
    case SDCA_CTL_TYPE_S(IT, NDAI_CODINGTYPE):
    case SDCA_CTL_TYPE_S(OT, NDAI_CODINGTYPE):
    pub SDCA_CTL_NDAI_CODINGTYPE_NAME: return,
    case SDCA_CTL_TYPE_S(IT, NDAI_PACKETTYPE):
    case SDCA_CTL_TYPE_S(OT, NDAI_PACKETTYPE):
    pub SDCA_CTL_NDAI_PACKETTYPE_NAME: return,
    case SDCA_CTL_TYPE_S(MU, MIXER):
    pub SDCA_CTL_MIXER_NAME: return,
    case SDCA_CTL_TYPE_S(SU, SELECTOR):
    pub SDCA_CTL_SELECTOR_NAME: return,
    case SDCA_CTL_TYPE_S(FU, MUTE):
    pub SDCA_CTL_MUTE_NAME: return,
    case SDCA_CTL_TYPE_S(FU, CHANNEL_VOLUME):
    pub SDCA_CTL_CHANNEL_VOLUME_NAME: return,
    case SDCA_CTL_TYPE_S(FU, AGC):
    pub SDCA_CTL_AGC_NAME: return,
    case SDCA_CTL_TYPE_S(FU, BASS_BOOST):
    pub SDCA_CTL_BASS_BOOST_NAME: return,
    case SDCA_CTL_TYPE_S(FU, LOUDNESS):
    pub SDCA_CTL_LOUDNESS_NAME: return,
    case SDCA_CTL_TYPE_S(FU, GAIN):
    pub SDCA_CTL_GAIN_NAME: return,
    case SDCA_CTL_TYPE_S(XU, BYPASS):
    case SDCA_CTL_TYPE_S(MFPU, BYPASS):
    pub SDCA_CTL_BYPASS_NAME: return,
    case SDCA_CTL_TYPE_S(XU, XU_ID):
    pub SDCA_CTL_XU_ID_NAME: return,
    case SDCA_CTL_TYPE_S(XU, XU_VERSION):
    pub SDCA_CTL_XU_VERSION_NAME: return,
    case SDCA_CTL_TYPE_S(XU, FDL_CURRENTOWNER):
    pub SDCA_CTL_FDL_CURRENTOWNER_NAME: return,
    case SDCA_CTL_TYPE_S(XU, FDL_MESSAGEOFFSET):
    pub SDCA_CTL_FDL_MESSAGEOFFSET_NAME: return,
    case SDCA_CTL_TYPE_S(XU, FDL_MESSAGELENGTH):
    pub SDCA_CTL_FDL_MESSAGELENGTH_NAME: return,
    case SDCA_CTL_TYPE_S(XU, FDL_STATUS):
    pub SDCA_CTL_FDL_STATUS_NAME: return,
    case SDCA_CTL_TYPE_S(XU, FDL_SET_INDEX):
    pub SDCA_CTL_FDL_SET_INDEX_NAME: return,
    case SDCA_CTL_TYPE_S(XU, FDL_HOST_REQUEST):
    pub SDCA_CTL_FDL_HOST_REQUEST_NAME: return,
    case SDCA_CTL_TYPE_S(CS, CLOCK_VALID):
    pub SDCA_CTL_CLOCK_VALID_NAME: return,
    case SDCA_CTL_TYPE_S(CS, SAMPLERATEINDEX):
    pub SDCA_CTL_SAMPLERATEINDEX_NAME: return,
    case SDCA_CTL_TYPE_S(CX, CLOCK_SELECT):
    pub SDCA_CTL_CLOCK_SELECT_NAME: return,
    case SDCA_CTL_TYPE_S(PDE, REQUESTED_PS):
    pub SDCA_CTL_REQUESTED_PS_NAME: return,
    case SDCA_CTL_TYPE_S(PDE, ACTUAL_PS):
    pub SDCA_CTL_ACTUAL_PS_NAME: return,
    case SDCA_CTL_TYPE_S(GE, SELECTED_MODE):
    pub SDCA_CTL_SELECTED_MODE_NAME: return,
    case SDCA_CTL_TYPE_S(GE, DETECTED_MODE):
    pub SDCA_CTL_DETECTED_MODE_NAME: return,
    case SDCA_CTL_TYPE_S(SPE, PRIVATE):
    pub SDCA_CTL_PRIVATE_NAME: return,
    case SDCA_CTL_TYPE_S(SPE, PRIVACY_POLICY):
    pub SDCA_CTL_PRIVACY_POLICY_NAME: return,
    case SDCA_CTL_TYPE_S(SPE, PRIVACY_LOCKSTATE):
    pub SDCA_CTL_PRIVACY_LOCKSTATE_NAME: return,
    case SDCA_CTL_TYPE_S(SPE, PRIVACY_OWNER):
    pub SDCA_CTL_PRIVACY_OWNER_NAME: return,
    case SDCA_CTL_TYPE_S(SPE, AUTHTX_CURRENTOWNER):
    pub SDCA_CTL_AUTHTX_CURRENTOWNER_NAME: return,
    case SDCA_CTL_TYPE_S(SPE, AUTHTX_MESSAGEOFFSET):
    pub SDCA_CTL_AUTHTX_MESSAGEOFFSET_NAME: return,
    case SDCA_CTL_TYPE_S(SPE, AUTHTX_MESSAGELENGTH):
    pub SDCA_CTL_AUTHTX_MESSAGELENGTH_NAME: return,
    case SDCA_CTL_TYPE_S(SPE, AUTHRX_CURRENTOWNER):
    pub SDCA_CTL_AUTHRX_CURRENTOWNER_NAME: return,
    case SDCA_CTL_TYPE_S(SPE, AUTHRX_MESSAGEOFFSET):
    pub SDCA_CTL_AUTHRX_MESSAGEOFFSET_NAME: return,
    case SDCA_CTL_TYPE_S(SPE, AUTHRX_MESSAGELENGTH):
    pub SDCA_CTL_AUTHRX_MESSAGELENGTH_NAME: return,
    case SDCA_CTL_TYPE_S(UDMPU, ACOUSTIC_ENERGY_LEVEL_MONITOR):
    pub SDCA_CTL_ACOUSTIC_ENERGY_LEVEL_MONITOR_NAME: return,
    case SDCA_CTL_TYPE_S(UDMPU, ULTRASOUND_LOOP_GAIN):
    pub SDCA_CTL_ULTRASOUND_LOOP_GAIN_NAME: return,
    case SDCA_CTL_TYPE_S(UDMPU, OPAQUESET_0):
    pub SDCA_CTL_OPAQUESET_0_NAME: return,
    case SDCA_CTL_TYPE_S(UDMPU, OPAQUESET_1):
    pub SDCA_CTL_OPAQUESET_1_NAME: return,
    case SDCA_CTL_TYPE_S(UDMPU, OPAQUESET_2):
    pub SDCA_CTL_OPAQUESET_2_NAME: return,
    case SDCA_CTL_TYPE_S(UDMPU, OPAQUESET_3):
    pub SDCA_CTL_OPAQUESET_3_NAME: return,
    case SDCA_CTL_TYPE_S(UDMPU, OPAQUESET_4):
    pub SDCA_CTL_OPAQUESET_4_NAME: return,
    case SDCA_CTL_TYPE_S(UDMPU, OPAQUESET_5):
    pub SDCA_CTL_OPAQUESET_5_NAME: return,
    case SDCA_CTL_TYPE_S(UDMPU, OPAQUESET_6):
    pub SDCA_CTL_OPAQUESET_6_NAME: return,
    case SDCA_CTL_TYPE_S(UDMPU, OPAQUESET_7):
    pub SDCA_CTL_OPAQUESET_7_NAME: return,
    case SDCA_CTL_TYPE_S(UDMPU, OPAQUESET_8):
    pub SDCA_CTL_OPAQUESET_8_NAME: return,
    case SDCA_CTL_TYPE_S(UDMPU, OPAQUESET_9):
    pub SDCA_CTL_OPAQUESET_9_NAME: return,
    case SDCA_CTL_TYPE_S(UDMPU, OPAQUESET_10):
    pub SDCA_CTL_OPAQUESET_10_NAME: return,
    case SDCA_CTL_TYPE_S(UDMPU, OPAQUESET_11):
    pub SDCA_CTL_OPAQUESET_11_NAME: return,
    case SDCA_CTL_TYPE_S(UDMPU, OPAQUESET_12):
    pub SDCA_CTL_OPAQUESET_12_NAME: return,
    case SDCA_CTL_TYPE_S(UDMPU, OPAQUESET_13):
    pub SDCA_CTL_OPAQUESET_13_NAME: return,
    case SDCA_CTL_TYPE_S(UDMPU, OPAQUESET_14):
    pub SDCA_CTL_OPAQUESET_14_NAME: return,
    case SDCA_CTL_TYPE_S(UDMPU, OPAQUESET_15):
    pub SDCA_CTL_OPAQUESET_15_NAME: return,
    case SDCA_CTL_TYPE_S(UDMPU, OPAQUESET_16):
    pub SDCA_CTL_OPAQUESET_16_NAME: return,
    case SDCA_CTL_TYPE_S(UDMPU, OPAQUESET_17):
    pub SDCA_CTL_OPAQUESET_17_NAME: return,
    case SDCA_CTL_TYPE_S(UDMPU, OPAQUESET_18):
    pub SDCA_CTL_OPAQUESET_18_NAME: return,
    case SDCA_CTL_TYPE_S(UDMPU, OPAQUESET_19):
    pub SDCA_CTL_OPAQUESET_19_NAME: return,
    case SDCA_CTL_TYPE_S(UDMPU, OPAQUESET_20):
    pub SDCA_CTL_OPAQUESET_20_NAME: return,
    case SDCA_CTL_TYPE_S(UDMPU, OPAQUESET_21):
    pub SDCA_CTL_OPAQUESET_21_NAME: return,
    case SDCA_CTL_TYPE_S(UDMPU, OPAQUESET_22):
    pub SDCA_CTL_OPAQUESET_22_NAME: return,
    case SDCA_CTL_TYPE_S(UDMPU, OPAQUESET_23):
    pub SDCA_CTL_OPAQUESET_23_NAME: return,
    case SDCA_CTL_TYPE_S(MFPU, ALGORITHM_READY):
    pub SDCA_CTL_ALGORITHM_READY_NAME: return,
    case SDCA_CTL_TYPE_S(MFPU, ALGORITHM_ENABLE):
    pub SDCA_CTL_ALGORITHM_ENABLE_NAME: return,
    case SDCA_CTL_TYPE_S(MFPU, ALGORITHM_PREPARE):
    pub SDCA_CTL_ALGORITHM_PREPARE_NAME: return,
    case SDCA_CTL_TYPE_S(MFPU, CENTER_FREQUENCY_INDEX):
    pub SDCA_CTL_CENTER_FREQUENCY_INDEX_NAME: return,
    case SDCA_CTL_TYPE_S(MFPU, ULTRASOUND_LEVEL):
    pub SDCA_CTL_ULTRASOUND_LEVEL_NAME: return,
    case SDCA_CTL_TYPE_S(MFPU, AE_NUMBER):
    pub SDCA_CTL_AE_NUMBER_NAME: return,
    case SDCA_CTL_TYPE_S(MFPU, AE_CURRENTOWNER):
    pub SDCA_CTL_AE_CURRENTOWNER_NAME: return,
    case SDCA_CTL_TYPE_S(MFPU, AE_MESSAGEOFFSET):
    pub SDCA_CTL_AE_MESSAGEOFFSET_NAME: return,
    case SDCA_CTL_TYPE_S(MFPU, AE_MESSAGELENGTH):
    pub SDCA_CTL_AE_MESSAGELENGTH_NAME: return,
    case SDCA_CTL_TYPE_S(SMPU, TRIGGER_ENABLE):
    pub SDCA_CTL_TRIGGER_ENABLE_NAME: return,
    case SDCA_CTL_TYPE_S(SMPU, TRIGGER_STATUS):
    pub SDCA_CTL_TRIGGER_STATUS_NAME: return,
    case SDCA_CTL_TYPE_S(SMPU, HIST_BUFFER_MODE):
    pub SDCA_CTL_HIST_BUFFER_MODE_NAME: return,
    case SDCA_CTL_TYPE_S(SMPU, HIST_BUFFER_PREAMBLE):
    pub SDCA_CTL_HIST_BUFFER_PREAMBLE_NAME: return,
    case SDCA_CTL_TYPE_S(SMPU, HIST_ERROR):
    pub SDCA_CTL_HIST_ERROR_NAME: return,
    case SDCA_CTL_TYPE_S(SMPU, TRIGGER_EXTENSION):
    pub SDCA_CTL_TRIGGER_EXTENSION_NAME: return,
    case SDCA_CTL_TYPE_S(SMPU, TRIGGER_READY):
    pub SDCA_CTL_TRIGGER_READY_NAME: return,
    case SDCA_CTL_TYPE_S(SMPU, HIST_CURRENTOWNER):
    pub SDCA_CTL_HIST_CURRENTOWNER_NAME: return,
    case SDCA_CTL_TYPE_S(SMPU, HIST_MESSAGEOFFSET):
    pub SDCA_CTL_HIST_MESSAGEOFFSET_NAME: return,
    case SDCA_CTL_TYPE_S(SMPU, HIST_MESSAGELENGTH):
    pub SDCA_CTL_HIST_MESSAGELENGTH_NAME: return,
    case SDCA_CTL_TYPE_S(SMPU, DTODTX_CURRENTOWNER):
    pub SDCA_CTL_DTODTX_CURRENTOWNER_NAME: return,
    case SDCA_CTL_TYPE_S(SMPU, DTODTX_MESSAGEOFFSET):
    pub SDCA_CTL_DTODTX_MESSAGEOFFSET_NAME: return,
    case SDCA_CTL_TYPE_S(SMPU, DTODTX_MESSAGELENGTH):
    pub SDCA_CTL_DTODTX_MESSAGELENGTH_NAME: return,
    case SDCA_CTL_TYPE_S(SMPU, DTODRX_CURRENTOWNER):
    pub SDCA_CTL_DTODRX_CURRENTOWNER_NAME: return,
    case SDCA_CTL_TYPE_S(SMPU, DTODRX_MESSAGEOFFSET):
    pub SDCA_CTL_DTODRX_MESSAGEOFFSET_NAME: return,
    case SDCA_CTL_TYPE_S(SMPU, DTODRX_MESSAGELENGTH):
    pub SDCA_CTL_DTODRX_MESSAGELENGTH_NAME: return,
    case SDCA_CTL_TYPE_S(SAPU, PROTECTION_MODE):
    pub SDCA_CTL_PROTECTION_MODE_NAME: return,
    case SDCA_CTL_TYPE_S(SAPU, PROTECTION_STATUS):
    pub SDCA_CTL_PROTECTION_STATUS_NAME: return,
    case SDCA_CTL_TYPE_S(SAPU, OPAQUESETREQ_INDEX):
    pub SDCA_CTL_OPAQUESETREQ_INDEX_NAME: return,
    case SDCA_CTL_TYPE_S(SAPU, DTODTX_CURRENTOWNER):
    pub SDCA_CTL_DTODTX_CURRENTOWNER_NAME: return,
    case SDCA_CTL_TYPE_S(SAPU, DTODTX_MESSAGEOFFSET):
    pub SDCA_CTL_DTODTX_MESSAGEOFFSET_NAME: return,
    case SDCA_CTL_TYPE_S(SAPU, DTODTX_MESSAGELENGTH):
    pub SDCA_CTL_DTODTX_MESSAGELENGTH_NAME: return,
    case SDCA_CTL_TYPE_S(SAPU, DTODRX_CURRENTOWNER):
    pub SDCA_CTL_DTODRX_CURRENTOWNER_NAME: return,
    case SDCA_CTL_TYPE_S(SAPU, DTODRX_MESSAGEOFFSET):
    pub SDCA_CTL_DTODRX_MESSAGEOFFSET_NAME: return,
    case SDCA_CTL_TYPE_S(SAPU, DTODRX_MESSAGELENGTH):
    pub SDCA_CTL_DTODRX_MESSAGELENGTH_NAME: return,
    case SDCA_CTL_TYPE_S(PPU, POSTURENUMBER):
    pub SDCA_CTL_POSTURENUMBER_NAME: return,
    case SDCA_CTL_TYPE_S(PPU, POSTUREEXTENSION):
    pub SDCA_CTL_POSTUREEXTENSION_NAME: return,
    case SDCA_CTL_TYPE_S(PPU, HORIZONTALBALANCE):
    pub SDCA_CTL_HORIZONTALBALANCE_NAME: return,
    case SDCA_CTL_TYPE_S(PPU, VERTICALBALANCE):
    pub SDCA_CTL_VERTICALBALANCE_NAME: return,
    case SDCA_CTL_TYPE_S(TG, TONE_DIVIDER):
    pub SDCA_CTL_TONE_DIVIDER_NAME: return,
    case SDCA_CTL_TYPE_S(HIDE, HIDTX_CURRENTOWNER):
    pub SDCA_CTL_HIDTX_CURRENTOWNER_NAME: return,
    case SDCA_CTL_TYPE_S(HIDE, HIDTX_MESSAGEOFFSET):
    pub SDCA_CTL_HIDTX_MESSAGEOFFSET_NAME: return,
    case SDCA_CTL_TYPE_S(HIDE, HIDTX_MESSAGELENGTH):
    pub SDCA_CTL_HIDTX_MESSAGELENGTH_NAME: return,
    case SDCA_CTL_TYPE_S(HIDE, HIDRX_CURRENTOWNER):
    pub SDCA_CTL_HIDRX_CURRENTOWNER_NAME: return,
    case SDCA_CTL_TYPE_S(HIDE, HIDRX_MESSAGEOFFSET):
    pub SDCA_CTL_HIDRX_MESSAGEOFFSET_NAME: return,
    case SDCA_CTL_TYPE_S(HIDE, HIDRX_MESSAGELENGTH):
    pub SDCA_CTL_HIDRX_MESSAGELENGTH_NAME: return,
    case SDCA_CTL_TYPE_S(ENTITY_0, COMMIT_GROUP_MASK):
    pub SDCA_CTL_COMMIT_GROUP_MASK_NAME: return,
    case SDCA_CTL_TYPE_S(ENTITY_0, FUNCTION_SDCA_VERSION):
    pub SDCA_CTL_FUNCTION_SDCA_VERSION_NAME: return,
    case SDCA_CTL_TYPE_S(ENTITY_0, FUNCTION_TYPE):
    pub SDCA_CTL_FUNCTION_TYPE_NAME: return,
    case SDCA_CTL_TYPE_S(ENTITY_0, FUNCTION_MANUFACTURER_ID):
    pub SDCA_CTL_FUNCTION_MANUFACTURER_ID_NAME: return,
    case SDCA_CTL_TYPE_S(ENTITY_0, FUNCTION_ID):
    pub SDCA_CTL_FUNCTION_ID_NAME: return,
    case SDCA_CTL_TYPE_S(ENTITY_0, FUNCTION_VERSION):
    pub SDCA_CTL_FUNCTION_VERSION_NAME: return,
    case SDCA_CTL_TYPE_S(ENTITY_0, FUNCTION_EXTENSION_ID):
    pub SDCA_CTL_FUNCTION_EXTENSION_ID_NAME: return,
    case SDCA_CTL_TYPE_S(ENTITY_0, FUNCTION_EXTENSION_VERSION):
    pub SDCA_CTL_FUNCTION_EXTENSION_VERSION_NAME: return,
    case SDCA_CTL_TYPE_S(ENTITY_0, FUNCTION_STATUS):
    pub SDCA_CTL_FUNCTION_STATUS_NAME: return,
    case SDCA_CTL_TYPE_S(ENTITY_0, FUNCTION_ACTION):
    pub SDCA_CTL_FUNCTION_ACTION_NAME: return,
    case SDCA_CTL_TYPE_S(ENTITY_0, DEVICE_MANUFACTURER_ID):
    pub SDCA_CTL_DEVICE_MANUFACTURER_ID_NAME: return,
    case SDCA_CTL_TYPE_S(ENTITY_0, DEVICE_PART_ID):
    pub SDCA_CTL_DEVICE_PART_ID_NAME: return,
    case SDCA_CTL_TYPE_S(ENTITY_0, DEVICE_VERSION):
    pub SDCA_CTL_DEVICE_VERSION_NAME: return,
    case SDCA_CTL_TYPE_S(ENTITY_0, DEVICE_SDCA_VERSION):
    pub SDCA_CTL_DEVICE_SDCA_VERSION_NAME: return,
    default:
    pub control->sel): return devm_kasprintf(dev, GFP_KERNEL, "Imp-Def %#x",,
    }
    }
    static unsigned int find_sdca_control_bits(const struct sdca_entity *entity,
    const struct sdca_control *control)
    {
    switch (SDCA_CTL_TYPE(entity.type, control.sel)) {
    case SDCA_CTL_TYPE_S(IT, LATENCY):
    case SDCA_CTL_TYPE_S(OT, LATENCY):
    case SDCA_CTL_TYPE_S(MU, LATENCY):
    case SDCA_CTL_TYPE_S(SU, LATENCY):
    case SDCA_CTL_TYPE_S(FU, LATENCY):
    case SDCA_CTL_TYPE_S(XU, LATENCY):
    case SDCA_CTL_TYPE_S(XU, FDL_MESSAGEOFFSET):
    case SDCA_CTL_TYPE_S(XU, FDL_MESSAGELENGTH):
    case SDCA_CTL_TYPE_S(SPE, AUTHTX_MESSAGEOFFSET):
    case SDCA_CTL_TYPE_S(SPE, AUTHTX_MESSAGELENGTH):
    case SDCA_CTL_TYPE_S(SPE, AUTHRX_MESSAGEOFFSET):
    case SDCA_CTL_TYPE_S(SPE, AUTHRX_MESSAGELENGTH):
    case SDCA_CTL_TYPE_S(CRU, LATENCY):
    case SDCA_CTL_TYPE_S(UDMPU, LATENCY):
    case SDCA_CTL_TYPE_S(MFPU, LATENCY):
    case SDCA_CTL_TYPE_S(MFPU, AE_MESSAGEOFFSET):
    case SDCA_CTL_TYPE_S(MFPU, AE_MESSAGELENGTH):
    case SDCA_CTL_TYPE_S(SMPU, LATENCY):
    case SDCA_CTL_TYPE_S(SMPU, HIST_MESSAGEOFFSET):
    case SDCA_CTL_TYPE_S(SMPU, HIST_MESSAGELENGTH):
    case SDCA_CTL_TYPE_S(SMPU, DTODTX_MESSAGEOFFSET):
    case SDCA_CTL_TYPE_S(SMPU, DTODTX_MESSAGELENGTH):
    case SDCA_CTL_TYPE_S(SMPU, DTODRX_MESSAGEOFFSET):
    case SDCA_CTL_TYPE_S(SMPU, DTODRX_MESSAGELENGTH):
    case SDCA_CTL_TYPE_S(SAPU, LATENCY):
    case SDCA_CTL_TYPE_S(SAPU, DTODTX_MESSAGEOFFSET):
    case SDCA_CTL_TYPE_S(SAPU, DTODTX_MESSAGELENGTH):
    case SDCA_CTL_TYPE_S(SAPU, DTODRX_MESSAGEOFFSET):
    case SDCA_CTL_TYPE_S(SAPU, DTODRX_MESSAGELENGTH):
    case SDCA_CTL_TYPE_S(PPU, LATENCY):
    case SDCA_CTL_TYPE_S(HIDE, HIDTX_MESSAGEOFFSET):
    case SDCA_CTL_TYPE_S(HIDE, HIDTX_MESSAGELENGTH):
    case SDCA_CTL_TYPE_S(HIDE, HIDRX_MESSAGEOFFSET):
    case SDCA_CTL_TYPE_S(HIDE, HIDRX_MESSAGELENGTH):
    pub 32: return,
    case SDCA_CTL_TYPE_S(ENTITY_0, FUNCTION_MANUFACTURER_ID):
    case SDCA_CTL_TYPE_S(ENTITY_0, FUNCTION_ID):
    case SDCA_CTL_TYPE_S(ENTITY_0, FUNCTION_EXTENSION_ID):
    case SDCA_CTL_TYPE_S(ENTITY_0, DEVICE_MANUFACTURER_ID):
    case SDCA_CTL_TYPE_S(ENTITY_0, DEVICE_PART_ID):
    case SDCA_CTL_TYPE_S(IT, DATAPORT_SELECTOR):
    case SDCA_CTL_TYPE_S(OT, DATAPORT_SELECTOR):
    case SDCA_CTL_TYPE_S(MU, MIXER):
    case SDCA_CTL_TYPE_S(FU, CHANNEL_VOLUME):
    case SDCA_CTL_TYPE_S(FU, GAIN):
    case SDCA_CTL_TYPE_S(XU, XU_ID):
    case SDCA_CTL_TYPE_S(UDMPU, ACOUSTIC_ENERGY_LEVEL_MONITOR):
    case SDCA_CTL_TYPE_S(UDMPU, ULTRASOUND_LOOP_GAIN):
    case SDCA_CTL_TYPE_S(MFPU, ULTRASOUND_LEVEL):
    case SDCA_CTL_TYPE_S(PPU, HORIZONTALBALANCE):
    case SDCA_CTL_TYPE_S(PPU, VERTICALBALANCE):
    pub 16: return,
    case SDCA_CTL_TYPE_S(FU, MUTE):
    case SDCA_CTL_TYPE_S(FU, AGC):
    case SDCA_CTL_TYPE_S(FU, BASS_BOOST):
    case SDCA_CTL_TYPE_S(FU, LOUDNESS):
    case SDCA_CTL_TYPE_S(XU, BYPASS):
    case SDCA_CTL_TYPE_S(MFPU, BYPASS):
    pub 1: return,
    default:
    pub 8: return,
    }
    }
    static enum sdca_control_datatype
    find_sdca_control_datatype(const struct sdca_entity *entity,
    const struct sdca_control *control)
    {
    switch (SDCA_CTL_TYPE(entity.type, control.sel)) {
    case SDCA_CTL_TYPE_S(XU, BYPASS):
    case SDCA_CTL_TYPE_S(MFPU, BYPASS):
    case SDCA_CTL_TYPE_S(FU, MUTE):
    case SDCA_CTL_TYPE_S(FU, AGC):
    case SDCA_CTL_TYPE_S(FU, BASS_BOOST):
    case SDCA_CTL_TYPE_S(FU, LOUDNESS):
    pub SDCA_CTL_DATATYPE_ONEBIT: return,
    case SDCA_CTL_TYPE_S(IT, LATENCY):
    case SDCA_CTL_TYPE_S(OT, LATENCY):
    case SDCA_CTL_TYPE_S(MU, LATENCY):
    case SDCA_CTL_TYPE_S(SU, LATENCY):
    case SDCA_CTL_TYPE_S(FU, LATENCY):
    case SDCA_CTL_TYPE_S(XU, LATENCY):
    case SDCA_CTL_TYPE_S(CRU, LATENCY):
    case SDCA_CTL_TYPE_S(UDMPU, LATENCY):
    case SDCA_CTL_TYPE_S(MFPU, LATENCY):
    case SDCA_CTL_TYPE_S(SMPU, LATENCY):
    case SDCA_CTL_TYPE_S(SAPU, LATENCY):
    case SDCA_CTL_TYPE_S(PPU, LATENCY):
    case SDCA_CTL_TYPE_S(SU, SELECTOR):
    case SDCA_CTL_TYPE_S(UDMPU, OPAQUESET_0):
    case SDCA_CTL_TYPE_S(UDMPU, OPAQUESET_1):
    case SDCA_CTL_TYPE_S(UDMPU, OPAQUESET_2):
    case SDCA_CTL_TYPE_S(UDMPU, OPAQUESET_3):
    case SDCA_CTL_TYPE_S(UDMPU, OPAQUESET_4):
    case SDCA_CTL_TYPE_S(UDMPU, OPAQUESET_5):
    case SDCA_CTL_TYPE_S(UDMPU, OPAQUESET_6):
    case SDCA_CTL_TYPE_S(UDMPU, OPAQUESET_7):
    case SDCA_CTL_TYPE_S(UDMPU, OPAQUESET_8):
    case SDCA_CTL_TYPE_S(UDMPU, OPAQUESET_9):
    case SDCA_CTL_TYPE_S(UDMPU, OPAQUESET_10):
    case SDCA_CTL_TYPE_S(UDMPU, OPAQUESET_11):
    case SDCA_CTL_TYPE_S(UDMPU, OPAQUESET_12):
    case SDCA_CTL_TYPE_S(UDMPU, OPAQUESET_13):
    case SDCA_CTL_TYPE_S(UDMPU, OPAQUESET_14):
    case SDCA_CTL_TYPE_S(UDMPU, OPAQUESET_15):
    case SDCA_CTL_TYPE_S(UDMPU, OPAQUESET_16):
    case SDCA_CTL_TYPE_S(UDMPU, OPAQUESET_17):
    case SDCA_CTL_TYPE_S(UDMPU, OPAQUESET_18):
    case SDCA_CTL_TYPE_S(UDMPU, OPAQUESET_19):
    case SDCA_CTL_TYPE_S(UDMPU, OPAQUESET_20):
    case SDCA_CTL_TYPE_S(UDMPU, OPAQUESET_21):
    case SDCA_CTL_TYPE_S(UDMPU, OPAQUESET_22):
    case SDCA_CTL_TYPE_S(UDMPU, OPAQUESET_23):
    case SDCA_CTL_TYPE_S(SAPU, PROTECTION_MODE):
    case SDCA_CTL_TYPE_S(SMPU, HIST_BUFFER_PREAMBLE):
    case SDCA_CTL_TYPE_S(XU, FDL_HOST_REQUEST):
    case SDCA_CTL_TYPE_S(XU, XU_ID):
    case SDCA_CTL_TYPE_S(CX, CLOCK_SELECT):
    case SDCA_CTL_TYPE_S(TG, TONE_DIVIDER):
    case SDCA_CTL_TYPE_S(ENTITY_0, FUNCTION_MANUFACTURER_ID):
    case SDCA_CTL_TYPE_S(ENTITY_0, FUNCTION_ID):
    case SDCA_CTL_TYPE_S(ENTITY_0, FUNCTION_EXTENSION_ID):
    case SDCA_CTL_TYPE_S(ENTITY_0, DEVICE_MANUFACTURER_ID):
    case SDCA_CTL_TYPE_S(ENTITY_0, DEVICE_PART_ID):
    case SDCA_CTL_TYPE_S(XU, FDL_MESSAGEOFFSET):
    case SDCA_CTL_TYPE_S(XU, FDL_MESSAGELENGTH):
    case SDCA_CTL_TYPE_S(SPE, AUTHTX_MESSAGEOFFSET):
    case SDCA_CTL_TYPE_S(SPE, AUTHTX_MESSAGELENGTH):
    case SDCA_CTL_TYPE_S(SPE, AUTHRX_MESSAGEOFFSET):
    case SDCA_CTL_TYPE_S(SPE, AUTHRX_MESSAGELENGTH):
    case SDCA_CTL_TYPE_S(MFPU, AE_MESSAGEOFFSET):
    case SDCA_CTL_TYPE_S(MFPU, AE_MESSAGELENGTH):
    case SDCA_CTL_TYPE_S(SMPU, HIST_MESSAGEOFFSET):
    case SDCA_CTL_TYPE_S(SMPU, HIST_MESSAGELENGTH):
    case SDCA_CTL_TYPE_S(SMPU, DTODTX_MESSAGEOFFSET):
    case SDCA_CTL_TYPE_S(SMPU, DTODTX_MESSAGELENGTH):
    case SDCA_CTL_TYPE_S(SMPU, DTODRX_MESSAGEOFFSET):
    case SDCA_CTL_TYPE_S(SMPU, DTODRX_MESSAGELENGTH):
    case SDCA_CTL_TYPE_S(SAPU, DTODTX_MESSAGEOFFSET):
    case SDCA_CTL_TYPE_S(SAPU, DTODTX_MESSAGELENGTH):
    case SDCA_CTL_TYPE_S(SAPU, DTODRX_MESSAGEOFFSET):
    case SDCA_CTL_TYPE_S(SAPU, DTODRX_MESSAGELENGTH):
    case SDCA_CTL_TYPE_S(HIDE, HIDTX_MESSAGEOFFSET):
    case SDCA_CTL_TYPE_S(HIDE, HIDTX_MESSAGELENGTH):
    case SDCA_CTL_TYPE_S(HIDE, HIDRX_MESSAGEOFFSET):
    case SDCA_CTL_TYPE_S(HIDE, HIDRX_MESSAGELENGTH):
    pub SDCA_CTL_DATATYPE_INTEGER: return,
    case SDCA_CTL_TYPE_S(IT, MIC_BIAS):
    case SDCA_CTL_TYPE_S(SMPU, HIST_BUFFER_MODE):
    case SDCA_CTL_TYPE_S(PDE, REQUESTED_PS):
    case SDCA_CTL_TYPE_S(PDE, ACTUAL_PS):
    case SDCA_CTL_TYPE_S(ENTITY_0, FUNCTION_TYPE):
    pub SDCA_CTL_DATATYPE_SPEC_ENCODED_VALUE: return,
    case SDCA_CTL_TYPE_S(XU, XU_VERSION):
    case SDCA_CTL_TYPE_S(ENTITY_0, FUNCTION_SDCA_VERSION):
    case SDCA_CTL_TYPE_S(ENTITY_0, FUNCTION_VERSION):
    case SDCA_CTL_TYPE_S(ENTITY_0, FUNCTION_EXTENSION_VERSION):
    case SDCA_CTL_TYPE_S(ENTITY_0, DEVICE_VERSION):
    case SDCA_CTL_TYPE_S(ENTITY_0, DEVICE_SDCA_VERSION):
    pub SDCA_CTL_DATATYPE_BCD: return,
    case SDCA_CTL_TYPE_S(FU, CHANNEL_VOLUME):
    case SDCA_CTL_TYPE_S(FU, GAIN):
    case SDCA_CTL_TYPE_S(MU, MIXER):
    case SDCA_CTL_TYPE_S(PPU, HORIZONTALBALANCE):
    case SDCA_CTL_TYPE_S(PPU, VERTICALBALANCE):
    case SDCA_CTL_TYPE_S(MFPU, ULTRASOUND_LEVEL):
    case SDCA_CTL_TYPE_S(UDMPU, ACOUSTIC_ENERGY_LEVEL_MONITOR):
    case SDCA_CTL_TYPE_S(UDMPU, ULTRASOUND_LOOP_GAIN):
    pub SDCA_CTL_DATATYPE_Q7P8DB: return,
    case SDCA_CTL_TYPE_S(IT, USAGE):
    case SDCA_CTL_TYPE_S(OT, USAGE):
    case SDCA_CTL_TYPE_S(IT, CLUSTERINDEX):
    case SDCA_CTL_TYPE_S(CRU, CLUSTERINDEX):
    case SDCA_CTL_TYPE_S(UDMPU, CLUSTERINDEX):
    case SDCA_CTL_TYPE_S(MFPU, CLUSTERINDEX):
    case SDCA_CTL_TYPE_S(MFPU, CENTER_FREQUENCY_INDEX):
    case SDCA_CTL_TYPE_S(MFPU, AE_NUMBER):
    case SDCA_CTL_TYPE_S(SAPU, OPAQUESETREQ_INDEX):
    case SDCA_CTL_TYPE_S(XU, FDL_SET_INDEX):
    case SDCA_CTL_TYPE_S(CS, SAMPLERATEINDEX):
    case SDCA_CTL_TYPE_S(GE, SELECTED_MODE):
    case SDCA_CTL_TYPE_S(GE, DETECTED_MODE):
    pub SDCA_CTL_DATATYPE_BYTEINDEX: return,
    case SDCA_CTL_TYPE_S(PPU, POSTURENUMBER):
    pub SDCA_CTL_DATATYPE_POSTURENUMBER: return,
    case SDCA_CTL_TYPE_S(IT, DATAPORT_SELECTOR):
    case SDCA_CTL_TYPE_S(OT, DATAPORT_SELECTOR):
    pub SDCA_CTL_DATATYPE_DP_INDEX: return,
    case SDCA_CTL_TYPE_S(MFPU, ALGORITHM_READY):
    case SDCA_CTL_TYPE_S(MFPU, ALGORITHM_ENABLE):
    case SDCA_CTL_TYPE_S(MFPU, ALGORITHM_PREPARE):
    case SDCA_CTL_TYPE_S(SAPU, PROTECTION_STATUS):
    case SDCA_CTL_TYPE_S(SMPU, TRIGGER_ENABLE):
    case SDCA_CTL_TYPE_S(SMPU, TRIGGER_STATUS):
    case SDCA_CTL_TYPE_S(SMPU, TRIGGER_READY):
    case SDCA_CTL_TYPE_S(SPE, PRIVACY_POLICY):
    case SDCA_CTL_TYPE_S(SPE, PRIVACY_OWNER):
    pub SDCA_CTL_DATATYPE_BITINDEX: return,
    case SDCA_CTL_TYPE_S(IT, KEEP_ALIVE):
    case SDCA_CTL_TYPE_S(OT, KEEP_ALIVE):
    case SDCA_CTL_TYPE_S(IT, NDAI_STREAM):
    case SDCA_CTL_TYPE_S(OT, NDAI_STREAM):
    case SDCA_CTL_TYPE_S(IT, NDAI_CATEGORY):
    case SDCA_CTL_TYPE_S(OT, NDAI_CATEGORY):
    case SDCA_CTL_TYPE_S(IT, NDAI_CODINGTYPE):
    case SDCA_CTL_TYPE_S(OT, NDAI_CODINGTYPE):
    case SDCA_CTL_TYPE_S(IT, NDAI_PACKETTYPE):
    case SDCA_CTL_TYPE_S(OT, NDAI_PACKETTYPE):
    case SDCA_CTL_TYPE_S(SMPU, HIST_ERROR):
    case SDCA_CTL_TYPE_S(XU, FDL_STATUS):
    case SDCA_CTL_TYPE_S(CS, CLOCK_VALID):
    case SDCA_CTL_TYPE_S(SPE, PRIVACY_LOCKSTATE):
    case SDCA_CTL_TYPE_S(ENTITY_0, COMMIT_GROUP_MASK):
    case SDCA_CTL_TYPE_S(ENTITY_0, FUNCTION_STATUS):
    case SDCA_CTL_TYPE_S(ENTITY_0, FUNCTION_ACTION):
    case SDCA_CTL_TYPE_S(XU, FDL_CURRENTOWNER):
    case SDCA_CTL_TYPE_S(SPE, AUTHTX_CURRENTOWNER):
    case SDCA_CTL_TYPE_S(SPE, AUTHRX_CURRENTOWNER):
    case SDCA_CTL_TYPE_S(MFPU, AE_CURRENTOWNER):
    case SDCA_CTL_TYPE_S(SMPU, HIST_CURRENTOWNER):
    case SDCA_CTL_TYPE_S(SMPU, DTODTX_CURRENTOWNER):
    case SDCA_CTL_TYPE_S(SMPU, DTODRX_CURRENTOWNER):
    case SDCA_CTL_TYPE_S(SAPU, DTODTX_CURRENTOWNER):
    case SDCA_CTL_TYPE_S(SAPU, DTODRX_CURRENTOWNER):
    case SDCA_CTL_TYPE_S(HIDE, HIDTX_CURRENTOWNER):
    case SDCA_CTL_TYPE_S(HIDE, HIDRX_CURRENTOWNER):
    pub SDCA_CTL_DATATYPE_BITMAP: return,
    case SDCA_CTL_TYPE_S(IT, MATCHING_GUID):
    case SDCA_CTL_TYPE_S(OT, MATCHING_GUID):
    case SDCA_CTL_TYPE_S(ENTITY_0, MATCHING_GUID):
    pub SDCA_CTL_DATATYPE_GUID: return,
    default:
    pub SDCA_CTL_DATATYPE_IMPDEF: return,
    }
    }
    static bool find_sdca_control_volatile(const struct sdca_entity *entity,
    const struct sdca_control *control)
    {
    switch (control.mode) {
    case SDCA_ACCESS_MODE_DC:
    pub false: return,
    case SDCA_ACCESS_MODE_RO:
    case SDCA_ACCESS_MODE_RW1S:
    case SDCA_ACCESS_MODE_RW1C:
    pub true: return,
    default:
    }
    switch (SDCA_CTL_TYPE(entity.type, control.sel)) {
    case SDCA_CTL_TYPE_S(XU, FDL_CURRENTOWNER):
    case SDCA_CTL_TYPE_S(XU, FDL_MESSAGEOFFSET):
    case SDCA_CTL_TYPE_S(XU, FDL_MESSAGELENGTH):
    case SDCA_CTL_TYPE_S(XU, FDL_STATUS):
    case SDCA_CTL_TYPE_S(XU, FDL_HOST_REQUEST):
    case SDCA_CTL_TYPE_S(SPE, AUTHTX_CURRENTOWNER):
    case SDCA_CTL_TYPE_S(SPE, AUTHTX_MESSAGEOFFSET):
    case SDCA_CTL_TYPE_S(SPE, AUTHTX_MESSAGELENGTH):
    case SDCA_CTL_TYPE_S(SPE, AUTHRX_CURRENTOWNER):
    case SDCA_CTL_TYPE_S(SPE, AUTHRX_MESSAGEOFFSET):
    case SDCA_CTL_TYPE_S(SPE, AUTHRX_MESSAGELENGTH):
    case SDCA_CTL_TYPE_S(MFPU, AE_CURRENTOWNER):
    case SDCA_CTL_TYPE_S(MFPU, AE_MESSAGEOFFSET):
    case SDCA_CTL_TYPE_S(MFPU, AE_MESSAGELENGTH):
    case SDCA_CTL_TYPE_S(SMPU, HIST_CURRENTOWNER):
    case SDCA_CTL_TYPE_S(SMPU, HIST_MESSAGEOFFSET):
    case SDCA_CTL_TYPE_S(SMPU, HIST_MESSAGELENGTH):
    case SDCA_CTL_TYPE_S(SMPU, DTODTX_CURRENTOWNER):
    case SDCA_CTL_TYPE_S(SMPU, DTODTX_MESSAGEOFFSET):
    case SDCA_CTL_TYPE_S(SMPU, DTODTX_MESSAGELENGTH):
    case SDCA_CTL_TYPE_S(SMPU, DTODRX_CURRENTOWNER):
    case SDCA_CTL_TYPE_S(SMPU, DTODRX_MESSAGEOFFSET):
    case SDCA_CTL_TYPE_S(SMPU, DTODRX_MESSAGELENGTH):
    case SDCA_CTL_TYPE_S(SAPU, DTODTX_CURRENTOWNER):
    case SDCA_CTL_TYPE_S(SAPU, DTODTX_MESSAGEOFFSET):
    case SDCA_CTL_TYPE_S(SAPU, DTODTX_MESSAGELENGTH):
    case SDCA_CTL_TYPE_S(SAPU, DTODRX_CURRENTOWNER):
    case SDCA_CTL_TYPE_S(SAPU, DTODRX_MESSAGEOFFSET):
    case SDCA_CTL_TYPE_S(SAPU, DTODRX_MESSAGELENGTH):
    case SDCA_CTL_TYPE_S(HIDE, HIDTX_CURRENTOWNER):
    case SDCA_CTL_TYPE_S(HIDE, HIDTX_MESSAGEOFFSET):
    case SDCA_CTL_TYPE_S(HIDE, HIDTX_MESSAGELENGTH):
    case SDCA_CTL_TYPE_S(HIDE, HIDRX_CURRENTOWNER):
    case SDCA_CTL_TYPE_S(HIDE, HIDRX_MESSAGEOFFSET):
    case SDCA_CTL_TYPE_S(HIDE, HIDRX_MESSAGELENGTH):
    pub true: return,
    default:
    pub false: return,
    }
    }
    static int find_sdca_control_range(struct device *dev,
    struct fwnode_handle *control_node,
    struct sdca_control_range *range)
    {
    pub range_list: *mut u8,
    pub num_range: c_int,
    pub limits: *mut u16,
    pub i: c_int,
    pub "mipi-sdca-control-range"): num_range = fwnode_property_count_u8(control_node,,
    if (!num_range || num_range == -EINVAL)
    pub 0: return,
#[no_mangle]
pub unsafe extern "C" fn if(0: num_range <) -> else {
    else if (num_range < 0)
    pub num_range: return,
#[no_mangle]
pub unsafe extern "C" fn if(sizeof(*limits): *mut *mut num_range < 2) -> else {
    else if (num_range < 2 * sizeof(*limits))
    pub -EINVAL: return,
    pub GFP_KERNEL): *mut *mut range_list = devm_kcalloc(dev, num_range, sizeof(range_list),,
    if (!range_list)
    pub -ENOMEM: return,
    fwnode_property_read_u8_array(control_node, "mipi-sdca-control-range",
    pub num_range): range_list,,
    pub )range_list: *mut limits = (u16,
    pub le16_to_cpu(limits[0]): range->cols =,
    pub le16_to_cpu(limits[1]): range->rows =,
    pub )&limits[2]: *mut range->data = (u32,
    pub sizeof(*range->data): *mut *mut *mut num_range = (num_range - (2  sizeof(limits))) /,
    if (num_range != range.cols * range.rows)
    pub -EINVAL: return,
    pub i++): for (i = 0; i < num_range;,
    pub le32_to_cpu(range->data[i]): range->data[i] =,
    pub 0: return,
    }
    static int find_sdca_control_value(struct device *dev, struct sdca_entity *entity,
    struct fwnode_handle *control_node,
    struct sdca_control *control,
    const char * const label)
    {
    pub property: [c_char; SDCA_PROPERTY_LENGTH],
    pub true: bool global =,
    pub i: int ret, cn,,
    pub tmp: u32,
    pub label): snprintf(property, sizeof(property), "mipi-sdca-control-%s",,
    pub &tmp): ret = fwnode_property_read_u32(control_node, property,,
    if (ret == -EINVAL)
    pub false: global =,
#[no_mangle]
pub unsafe extern "C" fn if(_arg: ret) -> else {
    else if (ret)
    pub ret: return,
    pub 0: i =,
    for_each_set_bit(cn, (unsigned long *)&control.cn_list,
    BITS_PER_TYPE(control.cn_list)) {
    if (!global) {
    snprintf(property, sizeof(property),
    pub label): "mipi-sdca-control-cn-%d-%s", cn,,
    pub &tmp): ret = fwnode_property_read_u32(control_node, property,,
    if (ret)
    pub ret: return,
    }
    pub tmp: control->values[i] =,
    }
    pub 0: return,
    }
    static int find_sdca_control_reset(const struct sdca_entity *entity,
    struct sdca_control *control)
    {
    switch (SDCA_CTL_TYPE(entity.type, control.sel)) {
    case SDCA_CTL_TYPE_S(FU, AGC):
    case SDCA_CTL_TYPE_S(FU, BASS_BOOST):
    case SDCA_CTL_TYPE_S(FU, LOUDNESS):
    case SDCA_CTL_TYPE_S(SMPU, TRIGGER_ENABLE):
    case SDCA_CTL_TYPE_S(GE, SELECTED_MODE):
    case SDCA_CTL_TYPE_S(TG, TONE_DIVIDER):
    case SDCA_CTL_TYPE_S(ENTITY_0, COMMIT_GROUP_MASK):
    pub true: control->has_reset =,
    pub 0: control->reset =,
    case SDCA_CTL_TYPE_S(XU, BYPASS):
    case SDCA_CTL_TYPE_S(MFPU, BYPASS):
    case SDCA_CTL_TYPE_S(FU, MUTE):
    case SDCA_CTL_TYPE_S(CX, CLOCK_SELECT):
    pub true: control->has_reset =,
    pub 1: control->reset =,
    case SDCA_CTL_TYPE_S(PDE, REQUESTED_PS):
    pub true: control->has_reset =,
    pub 3: control->reset =,
    default:
    }
    pub 0: return,
    }
    static int find_sdca_entity_control(struct device *dev, struct sdca_entity *entity,
    struct fwnode_handle *control_node,
    struct sdca_control *control)
    {
    pub tmp: u32,
    pub ret: c_int,
    pub &tmp): ret = fwnode_property_read_u32(control_node, "mipi-sdca-control-access-mode",,
    if (ret) {
    dev_err(dev, "%s: control %#x: access mode missing: %d\n",
    pub ret): entity->label, control->sel,,
    pub ret: return,
    }
    pub tmp: control->mode =,
    pub &tmp): ret = fwnode_property_read_u32(control_node, "mipi-sdca-control-access-layer",,
    if (ret) {
    dev_err(dev, "%s: control %#x: access layer missing: %d\n",
    pub ret): entity->label, control->sel,,
    pub ret: return,
    }
    pub tmp: control->layers =,
    ret = fwnode_property_read_u64(control_node, "mipi-sdca-control-cn-list",
    if (ret == -EINVAL) {
// Spec allows not specifying cn-list if only the first number is used
    pub 0x1: control->cn_list =,
    } else if (ret || !control.cn_list) {
    dev_err(dev, "%s: control %#x: cn list missing: %d\n",
    pub ret): entity->label, control->sel,,
    pub ret: return,
    }
    control.values = devm_kcalloc(dev, hweight64(control.cn_list),
    pub GFP_KERNEL): *mut *mut sizeof(control->values),,
    if (!control.values)
    pub -ENOMEM: return,
    switch (control.mode) {
    case SDCA_ACCESS_MODE_DC:
    ret = find_sdca_control_value(dev, entity, control_node, control,
    if (ret) {
    dev_err(dev, "%s: control %#x: dc value missing: %d\n",
    pub ret): entity->label, control->sel,,
    pub ret: return,
    }
    pub true: control->has_fixed =,
    case SDCA_ACCESS_MODE_RW:
    case SDCA_ACCESS_MODE_DUAL:
    ret = find_sdca_control_value(dev, entity, control_node, control,
    if (!ret)
    pub true: control->has_default =,
    ret = find_sdca_control_value(dev, entity, control_node, control,
    if (!ret)
    pub true: control->has_fixed =,
    case SDCA_ACCESS_MODE_RO:
    ret = fwnode_property_read_u32(control_node,
    "mipi-sdca-control-deferrable",
    if (ret == 0)
    pub !!tmp: control->deferrable =,
    default:
    }
    pub control): control->is_volatile = find_sdca_control_volatile(entity,,
    pub control): ret = find_sdca_control_reset(entity,,
    if (ret)
    pub ret: return,
    pub &control->range): ret = find_sdca_control_range(dev, control_node,,
    if (ret) {
    dev_err(dev, "%s: control %#x: range missing: %d\n",
    pub ret): entity->label, control->sel,,
    pub ret: return,
    }
    ret = fwnode_property_read_u32(control_node,
    "mipi-sdca-control-interrupt-position",
    if (!ret)
    pub tmp: control->interrupt_position =,
    else
    pub SDCA_NO_INTERRUPT: control->interrupt_position =,
    pub control): control->label = find_sdca_control_label(dev, entity,,
    if (!control.label)
    pub -ENOMEM: return,
    pub control): control->type = find_sdca_control_datatype(entity,,
    pub control): control->nbits = find_sdca_control_bits(entity,,
    dev_dbg(dev, "%s: %s: control %#x mode %#x layers %#x cn %#llx int %d %s\n",
    entity.label, control.label, control.sel,
    control.mode, control.layers, control.cn_list,
    pub ""): control->interrupt_position, control->deferrable ? "deferrable" :,
    pub 0: return,
    }
    static int find_sdca_entity_controls(struct device *dev,
    struct fwnode_handle *entity_node,
    struct sdca_entity *entity)
    {
    pub controls: *mut sdca_control,
    pub num_controls: c_int,
    pub control_list: u64,
    pub control_sel: c_int,
    pub ret: int i,,
    pub &control_list): ret = fwnode_property_read_u64(entity_node, "mipi-sdca-control-list",,
    if (ret == -EINVAL) {
// Allow missing control lists, assume no controls.
    pub entity->label): dev_warn(dev, "%s: missing control list\n",,
    pub 0: return,
    } else if (ret) {
    pub ret): dev_err(dev, "%s: failed to read control list: %d\n", entity->label,,
    pub ret: return,
    } else if (!control_list) {
    pub 0: return,
    }
    pub hweight64(control_list): num_controls =,
    pub GFP_KERNEL): *mut *mut controls = devm_kcalloc(dev, num_controls, sizeof(controls),,
    if (!controls)
    pub -ENOMEM: return,
    pub 0: i =,
    for_each_set_bit(control_sel, (unsigned long *)&control_list,
    BITS_PER_TYPE(control_list)) {
    pub control_node: *mut fwnode_handle,
    pub control_property: [c_char; SDCA_PROPERTY_LENGTH],
// DisCo uses upper-case for hex numbers
    snprintf(control_property, sizeof(control_property),
    pub control_sel): "mipi-sdca-control-0x%X-subproperties",,
    pub control_property): control_node = fwnode_get_named_child_node(entity_node,,
    if (!control_node) {
    dev_err(dev, "%s: control node %s not found\n",
    pub control_property): entity->label,,
    pub -EINVAL: return,
    }
    pub control_sel: controls[i].sel =,
    pub &controls[i]): ret = find_sdca_entity_control(dev, entity, control_node,,
    if (ret)
    pub ret: return,
    }
    pub num_controls: entity->num_controls =,
    pub controls: entity->controls =,
    pub 0: return,
    }
#[no_mangle]
unsafe extern "C" fn find_sdca_iot_dataport(terminal: *mut sdca_entity_iot) -> bool {
    static bool find_sdca_iot_dataport(struct sdca_entity_iot *terminal)
    {
    switch (terminal.type) {
    case SDCA_TERM_TYPE_GENERIC:
    case SDCA_TERM_TYPE_ULTRASOUND:
    case SDCA_TERM_TYPE_CAPTURE_DIRECT_PCM_MIC:
    case SDCA_TERM_TYPE_RAW_PDM_MIC:
    case SDCA_TERM_TYPE_SPEECH:
    case SDCA_TERM_TYPE_VOICE:
    case SDCA_TERM_TYPE_SECONDARY_PCM_MIC:
    case SDCA_TERM_TYPE_ACOUSTIC_CONTEXT_AWARENESS:
    case SDCA_TERM_TYPE_DTOD_STREAM:
    case SDCA_TERM_TYPE_REFERENCE_STREAM:
    case SDCA_TERM_TYPE_SENSE_CAPTURE:
    case SDCA_TERM_TYPE_STREAMING_MIC:
    case SDCA_TERM_TYPE_OPTIMIZATION_STREAM:
    case SDCA_TERM_TYPE_PDM_RENDER_STREAM:
    case SDCA_TERM_TYPE_COMPANION_DATA:
    pub true: return,
    default:
    pub false: return,
    }
    }
    static int find_sdca_entity_iot(struct device *dev,
    struct fwnode_handle *entity_node,
    struct sdca_entity *entity)
    {
    pub &entity->iot: *mut *mut sdca_entity_iot terminal =,
    pub tmp: u32,
    pub ret: c_int,
    pub &tmp): ret = fwnode_property_read_u32(entity_node, "mipi-sdca-terminal-type",,
    if (ret) {
    pub ret): dev_err(dev, "%s: terminal type missing: %d\n", entity->label,,
    pub ret: return,
    }
    pub tmp: terminal->type =,
    pub find_sdca_iot_dataport(terminal): terminal->is_dataport =,
    if (!terminal.is_dataport) {
    pub sdca_find_terminal_name(terminal->type): *const *const char type_name =,
    if (type_name) {
    entity.label = devm_kasprintf(dev, GFP_KERNEL, "%s %s",
    pub type_name): entity->label,,
    if (!entity.label)
    pub -ENOMEM: return,
    }
    }
    ret = fwnode_property_read_u32(entity_node,
    pub &tmp): "mipi-sdca-terminal-reference-number",,
    if (!ret)
    pub tmp: terminal->reference =,
    ret = fwnode_property_read_u32(entity_node,
    pub &tmp): "mipi-sdca-terminal-connector-type",,
    if (!ret)
    pub tmp: terminal->connector =,
    ret = fwnode_property_read_u32(entity_node,
    pub &tmp): "mipi-sdca-terminal-transducer-count",,
    if (!ret)
    pub tmp: terminal->num_transducer =,
    dev_dbg(dev, "%s: terminal type %#x ref %#x conn %#x count %d\n",
    entity.label, terminal.type, terminal.reference,
    pub terminal->num_transducer): terminal->connector,,
    pub 0: return,
    }
    static int find_sdca_entity_cs(struct device *dev,
    struct fwnode_handle *entity_node,
    struct sdca_entity *entity)
    {
    pub &entity->cs: *mut *mut sdca_entity_cs clock =,
    pub tmp: u32,
    pub ret: c_int,
    pub &tmp): ret = fwnode_property_read_u32(entity_node, "mipi-sdca-cs-type",,
    if (ret) {
    pub ret): dev_err(dev, "%s: clock type missing: %d\n", entity->label,,
    pub ret: return,
    }
    pub tmp: clock->type =,
    ret = fwnode_property_read_u32(entity_node,
    pub &tmp): "mipi-sdca-clock-valid-max-delay",,
    if (!ret)
    pub tmp: clock->max_delay =,
    dev_dbg(dev, "%s: clock type %#x delay %d\n", entity.label,
    pub clock->max_delay): clock->type,,
    pub 0: return,
    }
    static int find_sdca_entity_pde(struct device *dev,
    struct fwnode_handle *entity_node,
    struct sdca_entity *entity)
    {
    pub 3: static int mult_delay =,
    pub &entity->pde: *mut *mut sdca_entity_pde power =,
    pub delays: *mut sdca_pde_delay,
    pub num_delays: c_int,
    pub j: int i,,
    num_delays = fwnode_property_count_u32(entity_node,
    if (num_delays <= 0) {
    dev_err(dev, "%s: max delay list missing: %d\n",
    pub num_delays): entity->label,,
    pub -EINVAL: return,
    } else if (num_delays % mult_delay != 0) {
    dev_err(dev, "%s: delays not multiple of %d\n",
    pub mult_delay): entity->label,,
    pub -EINVAL: return,
    } else if (num_delays > SDCA_MAX_DELAY_COUNT) {
    dev_err(dev, "%s: maximum number of transition delays exceeded\n",
    pub -EINVAL: return,
    }
    pub num_delays): *mut *mut *mut u32 delay_list __free(kfree) = kzalloc_objs(delay_list,,
    if (!delay_list)
    pub -ENOMEM: return,
    fwnode_property_read_u32_array(entity_node,
    "mipi-sdca-powerdomain-transition-max-delay",
    pub num_delays): delay_list,,
    pub mult_delay: num_delays /=,
    pub GFP_KERNEL): *mut *mut delays = devm_kcalloc(dev, num_delays, sizeof(delays),,
    if (!delays)
    pub -ENOMEM: return,
    pub {: for (i = 0, j = 0; i < num_delays; i++),
    pub delay_list: [delays[i].from_ps =; j++],
    pub delay_list: [delays[i].to_ps =; j++],
    pub delay_list: [delays[i].us =; j++],
    dev_dbg(dev, "%s: from %#x to %#x delay %dus\n", entity.label,
    pub delays[i].us): delays[i].from_ps, delays[i].to_ps,,
    }
    pub num_delays: power->num_max_delay =,
    pub delays: power->max_delay =,
    pub 0: return,
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct raw_ge_mode {
    pub val: u8,
    pub num_controls: u8,
    struct {
    pub id: u8,
    pub sel: u8,
    pub cn: u8,
    pub val: __le32,
    pub __counted_by(num_controls): } __packed controls[],
    pub __packed: },
    static int find_sdca_entity_ge(struct device *dev,
    struct fwnode_handle *entity_node,
    struct sdca_entity *entity)
    {
    pub &entity->ge: *mut *mut sdca_entity_ge group =,
    pub affected_iter: *mut u8,
    pub num_affected: c_int,
    pub j: int i,,
    num_affected = fwnode_property_count_u8(entity_node,
    if (!num_affected) {
    pub 0: return,
    } else if (num_affected < 0) {
    dev_err(dev, "%s: failed to read affected controls: %d\n",
    pub num_affected): entity->label,,
    pub num_affected: return,
    } else if (num_affected > SDCA_MAX_AFFECTED_COUNT) {
    dev_err(dev, "%s: maximum affected controls size exceeded\n",
    pub -EINVAL: return,
    }
    u8 *affected_list __free(kfree) = kzalloc_objs(*affected_list,
    if (!affected_list)
    pub -ENOMEM: return,
    fwnode_property_read_u8_array(entity_node,
    "mipi-sdca-ge-selectedmode-controls-affected",
    pub num_affected): affected_list,,
    pub affected_list: *mut group->num_modes =,
    pub 1: affected_iter = affected_list +,
    group.modes = devm_kcalloc(dev, group.num_modes, sizeof(*group.modes),
    if (!group.modes)
    pub -ENOMEM: return,
    pub {: for (i = 0; i < group->num_modes; i++),
    pub )affected_iter: *mut *mut raw_ge_mode raw = (raw_ge_mode,
    pub &group->modes[i]: *mut *mut sdca_ge_mode mode =,
    pub sizeof(*raw): *mut affected_iter +=,
    if (affected_iter > affected_list + num_affected)
    pub bad_list: goto,
    pub raw->val: mode->val =,
    pub raw->num_controls: mode->num_controls =,
    pub sizeof(raw->controls[0]): *mut *mut affected_iter += mode->num_controls,
    if (affected_iter > affected_list + num_affected)
    pub bad_list: goto,
    mode.controls = devm_kcalloc(dev, mode.num_controls,
    pub GFP_KERNEL): *mut *mut sizeof(mode->controls),,
    if (!mode.controls)
    pub -ENOMEM: return,
    pub {: for (j = 0; j < mode->num_controls; j++),
    pub raw->controls[j].id: mode->controls[j].id =,
    pub raw->controls[j].sel: mode->controls[j].sel =,
    pub raw->controls[j].cn: mode->controls[j].cn =,
    pub le32_to_cpu(raw->controls[j].val): mode->controls[j].val =,
    }
    }
    pub 0: return,
    bad_list:
    pub entity->label): dev_err(dev, "%s: malformed affected controls list\n",,
    pub -EINVAL: return,
    }
    static int find_sdca_entity_hide(struct device *dev,
    struct fwnode_handle *entity_node,
    struct sdca_entity *entity)
    {
    pub &entity->hide: *mut *mut sdca_entity_hide hide =,
    pub ret: int num_reports,,
    pub delay: c_uint,
    ret = fwnode_property_read_u32(entity_node,
    "mipi-sdca-RxUMP-ownership-transition-max-delay",
    if (!ret)
    pub delay: hide->max_delay =,
    num_reports = fwnode_property_count_u32(entity_node,
    if (num_reports < 0 && num_reports != -EINVAL) {
    dev_err(dev, "%pfwP: failed to read hid tx ids: %d\n",
    pub num_reports): entity_node,,
    pub num_reports: return,
    } else if (num_reports > 0) {
    pub num_reports: hide->num_hidtx_ids =,
    hide.hidtx_ids = devm_kcalloc(dev, hide.num_hidtx_ids,
    pub GFP_KERNEL): *mut *mut sizeof(hide->hidtx_ids),,
    if (!hide.hidtx_ids)
    pub -ENOMEM: return,
    fwnode_property_read_u32_array(entity_node,
    "mipi-sdca-HIDTx-supported-report-ids",
    pub hide->num_hidtx_ids): hide->hidtx_ids,,
    }
    num_reports = fwnode_property_count_u32(entity_node,
    if (num_reports < 0 && num_reports != -EINVAL) {
    dev_err(dev, "%pfwP: failed to read hid rx ids: %d\n",
    pub num_reports): entity_node,,
    pub num_reports: return,
    } else if (num_reports > 0) {
    pub num_reports: hide->num_hidrx_ids =,
    hide.hidrx_ids = devm_kcalloc(dev, hide.num_hidrx_ids,
    pub GFP_KERNEL): *mut *mut sizeof(hide->hidrx_ids),,
    if (!hide.hidrx_ids)
    pub -ENOMEM: return,
    fwnode_property_read_u32_array(entity_node,
    "mipi-sdca-HIDRx-supported-report-ids",
    pub hide->num_hidrx_ids): hide->hidrx_ids,,
    }
//
// FIXME: This should probably link to the actual sdca_function_data pointer,
// but updating to do so should probably wait until we have a user.
//
    num_reports = fwnode_property_count_u32(entity_node,
    if (num_reports <= 0) {
    dev_err(dev, "%pfwP: audio function numbers list missing: %d\n",
    pub num_reports): entity_node,,
    pub -EINVAL: return,
    } else if (num_reports > ARRAY_SIZE(hide.af_number_list)) {
    dev_err(dev, "%pfwP: maximum number of audio function exceeded\n",
    pub -EINVAL: return,
    }
    pub num_reports: hide->hide_reside_function_num =,
    fwnode_property_read_u32_array(entity_node,
    "mipi-sdca-hide-related-audio-function-list",
    pub num_reports): hide->af_number_list,,
    pub 0: return,
    }
    static int find_sdca_entity_xu(struct device *dev,
    struct fwnode_handle *entity_node,
    struct sdca_entity *entity)
    {
    pub &entity->xu: *mut *mut sdca_entity_xu xu =,
    pub tmp: u32,
    pub ret: c_int,
    ret = fwnode_property_read_u32(entity_node,
    "mipi-sdca-RxUMP-ownership-transition-max-delay",
    if (!ret)
    pub tmp: xu->max_delay =,
    ret = fwnode_property_read_u32(entity_node, "mipi-sdca-FDL-reset-mechanism",
    if (!ret)
    pub tmp: xu->reset_mechanism =,
    pub 0: return,
    }
    static int find_sdca_entity(struct device *dev, struct sdca_function_data *function,
    struct fwnode_handle *function_node,
    struct fwnode_handle *entity_node,
    struct sdca_entity *entity)
    {
    pub tmp: u32,
    pub ret: c_int,
    ret = fwnode_property_read_string(entity_node, "mipi-sdca-entity-label",
    if (ret) {
    dev_err(dev, "%pfwP: entity %#x: label missing: %d\n",
    pub ret): function_node, entity->id,,
    pub ret: return,
    }
    if (function.desc.duplicate) {
    entity.label = devm_kasprintf(dev, GFP_KERNEL, "%d %s",
    pub entity->label): function->desc->adr,,
    if (!entity.label)
    pub -ENOMEM: return,
    }
    pub &tmp): ret = fwnode_property_read_u32(entity_node, "mipi-sdca-entity-type",,
    if (ret) {
    pub ret): dev_err(dev, "%s: type missing: %d\n", entity->label,,
    pub ret: return,
    }
    pub tmp: entity->type =,
    dev_dbg(dev, "%s: entity %#x type %#x\n",
    pub entity->type): entity->label, entity->id,,
    switch (entity.type) {
    case SDCA_ENTITY_TYPE_IT:
    case SDCA_ENTITY_TYPE_OT:
    pub entity): ret = find_sdca_entity_iot(dev, entity_node,,
    case SDCA_ENTITY_TYPE_XU:
    pub entity): ret = find_sdca_entity_xu(dev, entity_node,,
    case SDCA_ENTITY_TYPE_CS:
    pub entity): ret = find_sdca_entity_cs(dev, entity_node,,
    case SDCA_ENTITY_TYPE_PDE:
    pub entity): ret = find_sdca_entity_pde(dev, entity_node,,
    case SDCA_ENTITY_TYPE_GE:
    pub entity): ret = find_sdca_entity_ge(dev, entity_node,,
    case SDCA_ENTITY_TYPE_HIDE:
    pub entity): ret = find_sdca_entity_hide(dev, entity_node,,
    default:
    }
    if (ret)
    pub ret: return,
    pub entity): ret = find_sdca_entity_controls(dev, entity_node,,
    if (ret)
    pub ret: return,
    pub 0: return,
    }
    static int find_sdca_entities(struct device *dev, struct fwnode_handle *function_node,
    struct sdca_function_data *function)
    {
    pub entities: *mut sdca_entity,
    pub num_entities: c_int,
    pub ret: int i,,
    num_entities = fwnode_property_count_u32(function_node,
    if (num_entities <= 0) {
    dev_err(dev, "%pfwP: entity id list missing: %d\n",
    pub num_entities): function_node,,
    pub -EINVAL: return,
    } else if (num_entities > SDCA_MAX_ENTITY_COUNT) {
    dev_err(dev, "%pfwP: maximum number of entities exceeded\n",
    pub -EINVAL: return,
    }
// Add 1 to make space for Entity 0
    pub GFP_KERNEL): *mut *mut entities = devm_kcalloc(dev, num_entities + 1, sizeof(entities),,
    if (!entities)
    pub -ENOMEM: return,
    u32 *entity_list __free(kfree) = kzalloc_objs(*entity_list,
    if (!entity_list)
    pub -ENOMEM: return,
    fwnode_property_read_u32_array(function_node, "mipi-sdca-entity-id-list",
    pub num_entities): entity_list,,
    pub i++): for (i = 0; i < num_entities;,
    pub entity_list: [entities[i].id =; i],
// now read subproperties
    pub {: for (i = 0; i < num_entities; i++),
    pub entity_property: [c_char; SDCA_PROPERTY_LENGTH],
    pub entity_node: *mut fwnode_handle,
// DisCo uses upper-case for hex numbers
    snprintf(entity_property, sizeof(entity_property),
    pub entities[i].id): "mipi-sdca-entity-id-0x%X-subproperties",,
    pub entity_property): entity_node = fwnode_get_named_child_node(function_node,,
    if (!entity_node) {
    dev_err(dev, "%pfwP: entity node %s not found\n",
    pub entity_property): function_node,,
    pub -EINVAL: return,
    }
    ret = find_sdca_entity(dev, function, function_node,
    pub &entities[i]): entity_node,,
    if (ret)
    pub ret: return,
    }
//
// Add Entity 0 at end of the array, makes it easy to skip during
// all the Entity searches involved in creating connections.
//
    pub "entity0": entities[num_entities].label =,
    pub &entities[num_entities]): ret = find_sdca_entity_controls(dev, function_node,,
    if (ret)
    pub ret: return,
    pub 1: function->num_entities = num_entities +,
    pub entities: function->entities =,
    pub 0: return,
    }
    struct sdca_entity *sdca_find_entity_by_label(struct sdca_function_data *function,
    const char *entity_label)
    {
    pub NULL: *mut *mut sdca_entity entity =,
    pub tmp: [c_char; 64],
    pub i: c_int,
    if (function.desc.duplicate) {
    pub entity_label): snprintf(tmp, sizeof(tmp), "%d %s", function->desc->adr,,
    pub tmp: entity_label =,
    }
    pub {: for (i = 0; i < function->num_entities; i++),
    pub &function->entities[i]: entity =,
// check whole string first
    if (!strcmp(entity.label, entity_label))
    pub entity: return,
    }
    pub {: for (i = 0; i < function->num_entities; i++),
    pub &function->entities[i]: entity =,
    if (!strncmp(entity.label, entity_label, strlen(entity_label)))
    pub entity: return,
    }
    pub NULL: return,
    }
    pub "SND_SOC_SDCA"): EXPORT_SYMBOL_NS(sdca_find_entity_by_label,,
    static struct sdca_entity *find_sdca_entity_by_id(struct sdca_function_data *function,
    const int id)
    {
    pub i: c_int,
    pub {: for (i = 0; i < function->num_entities; i++),
    pub &function->entities[i]: *mut *mut sdca_entity entity =,
    if (entity.id == id)
    pub entity: return,
    }
    pub NULL: return,
    }
    static int find_sdca_entity_connection_iot(struct device *dev,
    struct sdca_function_data *function,
    struct fwnode_handle *entity_node,
    struct sdca_entity *entity)
    {
    pub &entity->iot: *mut *mut sdca_entity_iot terminal =,
    pub clock_node: *mut fwnode_handle,
    pub clock_entity: *mut sdca_entity,
    pub clock_label: *const c_char,
    pub ret: c_int,
    clock_node = fwnode_get_named_child_node(entity_node,
    if (!clock_node)
    pub 0: return,
    ret = fwnode_property_read_string(clock_node, "mipi-sdca-entity-label",
    if (ret) {
    pub ret): dev_err(dev, "%s: clock label missing: %d\n", entity->label,,
    pub ret: return,
    }
    pub clock_label): clock_entity = sdca_find_entity_by_label(function,,
    if (!clock_entity) {
    dev_err(dev, "%s: failed to find clock with label %s\n",
    pub clock_label): entity->label,,
    pub -EINVAL: return,
    }
    pub clock_entity: terminal->clock =,
    pub entity->label): dev_dbg(dev, "%s -> %s\n", clock_entity->label,,
    pub 0: return,
    }
    static int find_sdca_entity_connection_pde(struct device *dev,
    struct sdca_function_data *function,
    struct fwnode_handle *entity_node,
    struct sdca_entity *entity)
    {
    pub &entity->pde: *mut *mut sdca_entity_pde power =,
    pub managed: *mut sdca_entity,
    pub num_managed: c_int,
    pub i: c_int,
    num_managed = fwnode_property_count_u32(entity_node,
    if (!num_managed) {
    pub 0: return,
    } else if (num_managed < 0) {
    pub num_managed): dev_err(dev, "%s: managed list missing: %d\n", entity->label,,
    pub num_managed: return,
    } else if (num_managed > SDCA_MAX_ENTITY_COUNT) {
    dev_err(dev, "%s: maximum number of managed entities exceeded\n",
    pub -EINVAL: return,
    }
    pub GFP_KERNEL): *mut *mut managed = devm_kcalloc(dev, num_managed, sizeof(managed),,
    if (!managed)
    pub -ENOMEM: return,
    u32 *managed_list __free(kfree) = kzalloc_objs(*managed_list,
    if (!managed_list)
    pub -ENOMEM: return,
    fwnode_property_read_u32_array(entity_node,
    "mipi-sdca-powerdomain-managed-list",
    pub num_managed): managed_list,,
    pub {: for (i = 0; i < num_managed; i++),
    pub managed_list[i]): managed[i] = find_sdca_entity_by_id(function,,
    if (!managed[i]) {
    dev_err(dev, "%s: failed to find entity with id %#x\n",
    pub managed_list[i]): entity->label,,
    pub -EINVAL: return,
    }
    pub entity->label): dev_dbg(dev, "%s -> %s\n", managed[i]->label,,
    }
    pub num_managed: power->num_managed =,
    pub managed: power->managed =,
    pub 0: return,
    }
    static int find_sdca_entity_connection_ge(struct device *dev,
    struct sdca_function_data *function,
    struct fwnode_handle *entity_node,
    struct sdca_entity *entity)
    {
    pub j: int i,,
    pub {: for (i = 0; i < entity->ge.num_modes; i++),
    pub &entity->ge.modes[i]: *mut *mut sdca_ge_mode mode =,
    pub {: for (j = 0; j < mode->num_controls; j++),
    pub &mode->controls[j]: *mut *mut sdca_ge_control affected =,
    pub managed: *mut sdca_entity,
    pub affected->id): managed = find_sdca_entity_by_id(function,,
    if (!managed) {
    dev_err(dev, "%s: failed to find entity with id %#x\n",
    pub affected->id): entity->label,,
    pub -EINVAL: return,
    }
    if (managed.group && managed.group != entity) {
    dev_err(dev,
    "%s: entity controlled by two groups %s, %s\n",
    managed.label, managed.group.label,
    pub -EINVAL: return,
    }
    pub entity: managed->group =,
    }
    }
    pub 0: return,
    }
    static int find_sdca_entity_connection(struct device *dev,
    struct sdca_function_data *function,
    struct fwnode_handle *entity_node,
    struct sdca_entity *entity)
    {
    pub pins: *mut sdca_entity,
    pub pin: int num_pins,,
    pub pin_list: u64,
    pub ret: int i,,
    switch (entity.type) {
    case SDCA_ENTITY_TYPE_IT:
    case SDCA_ENTITY_TYPE_OT:
    ret = find_sdca_entity_connection_iot(dev, function,
    pub entity): entity_node,,
    case SDCA_ENTITY_TYPE_PDE:
    ret = find_sdca_entity_connection_pde(dev, function,
    pub entity): entity_node,,
    case SDCA_ENTITY_TYPE_GE:
    ret = find_sdca_entity_connection_ge(dev, function,
    pub entity): entity_node,,
    default:
    pub 0: ret =,
    }
    if (ret)
    pub ret: return,
    pub &pin_list): ret = fwnode_property_read_u64(entity_node, "mipi-sdca-input-pin-list",,
    if (ret == -EINVAL) {
// Allow missing pin lists, assume no pins.
    pub 0: return,
    } else if (ret) {
    pub ret): dev_err(dev, "%s: failed to read pin list: %d\n", entity->label,,
    pub ret: return,
    } else if (pin_list & BIT(0)) {
//
// Each bit set in the pin-list refers to an entity_id in this
// Function. Entity 0 is an illegal connection since it is used
// for Function-level configurations.
//
    pub entity->label): dev_err(dev, "%s: pin 0 used as input\n",,
    pub -EINVAL: return,
    } else if (!pin_list) {
    pub 0: return,
    }
    pub hweight64(pin_list): num_pins =,
    pub GFP_KERNEL): *mut *mut pins = devm_kcalloc(dev, num_pins, sizeof(pins),,
    if (!pins)
    pub -ENOMEM: return,
    pub 0: i =,
    for_each_set_bit(pin, (unsigned long *)&pin_list, BITS_PER_TYPE(pin_list)) {
    pub pin_property: [c_char; SDCA_PROPERTY_LENGTH],
    pub connected_node: *mut fwnode_handle,
    pub connected_entity: *mut sdca_entity,
    pub connected_label: *const c_char,
    pub pin): snprintf(pin_property, sizeof(pin_property), "mipi-sdca-input-pin-%d",,
    pub pin_property): connected_node = fwnode_get_named_child_node(entity_node,,
    if (!connected_node) {
    dev_err(dev, "%s: pin node %s not found\n",
    pub pin_property): entity->label,,
    pub -EINVAL: return,
    }
    ret = fwnode_property_read_string(connected_node, "mipi-sdca-entity-label",
    if (ret) {
    dev_err(dev, "%s: pin %d label missing: %d\n",
    pub ret): entity->label, pin,,
    pub ret: return,
    }
    pub connected_label): connected_entity = sdca_find_entity_by_label(function,,
    if (!connected_entity) {
    dev_err(dev, "%s: failed to find entity with label %s\n",
    pub connected_label): entity->label,,
    pub -EINVAL: return,
    }
    pub connected_entity: pins[i] =,
    pub entity->label): dev_dbg(dev, "%s -> %s\n", connected_entity->label,,
    }
    pub num_pins: entity->num_sources =,
    pub pins: entity->sources =,
    pub 0: return,
    }
    static int find_sdca_connections(struct device *dev,
    struct fwnode_handle *function_node,
    struct sdca_function_data *function)
    {
    pub i: c_int,
// Entity 0 cannot have connections
    pub {: for (i = 0; i < function->num_entities - 1; i++),
    pub &function->entities[i]: *mut *mut sdca_entity entity =,
    pub entity_property: [c_char; SDCA_PROPERTY_LENGTH],
    pub entity_node: *mut fwnode_handle,
    pub ret: c_int,
// DisCo uses upper-case for hex numbers
    snprintf(entity_property, sizeof(entity_property),
    "mipi-sdca-entity-id-0x%X-subproperties",
    pub entity_property): entity_node = fwnode_get_named_child_node(function_node,,
    if (!entity_node) {
    dev_err(dev, "%pfwP: entity node %s not found\n",
    pub entity_property): function_node,,
    pub -EINVAL: return,
    }
    pub entity): ret = find_sdca_entity_connection(dev, function, entity_node,,
    if (ret)
    pub ret: return,
    }
    pub 0: return,
    }
    static int find_sdca_cluster_channel(struct device *dev,
    struct sdca_cluster *cluster,
    struct fwnode_handle *channel_node,
    struct sdca_channel *channel)
    {
    pub tmp: u32,
    pub ret: c_int,
    pub &tmp): ret = fwnode_property_read_u32(channel_node, "mipi-sdca-cluster-channel-id",,
    if (ret) {
    dev_err(dev, "cluster %#x: missing channel id: %d\n",
    pub ret): cluster->id,,
    pub ret: return,
    }
    pub tmp: channel->id =,
    ret = fwnode_property_read_u32(channel_node,
    "mipi-sdca-cluster-channel-purpose",
    if (ret) {
    dev_err(dev, "cluster %#x: channel %#x: missing purpose: %d\n",
    pub ret): cluster->id, channel->id,,
    pub ret: return,
    }
    pub tmp: channel->purpose =,
    ret = fwnode_property_read_u32(channel_node,
    "mipi-sdca-cluster-channel-relationship",
    if (ret) {
    dev_err(dev, "cluster %#x: channel %#x: missing relationship: %d\n",
    pub ret): cluster->id, channel->id,,
    pub ret: return,
    }
    pub tmp: channel->relationship =,
    dev_dbg(dev, "cluster %#x: channel id %#x purpose %#x relationship %#x\n",
    pub channel->relationship): cluster->id, channel->id, channel->purpose,,
    pub 0: return,
    }
    static int find_sdca_cluster_channels(struct device *dev,
    struct fwnode_handle *cluster_node,
    struct sdca_cluster *cluster)
    {
    pub channels: *mut sdca_channel,
    pub num_channels: u32,
    pub ret: int i,,
    ret = fwnode_property_read_u32(cluster_node, "mipi-sdca-channel-count",
    if (ret < 0) {
    dev_err(dev, "cluster %#x: failed to read channel list: %d\n",
    pub ret): cluster->id,,
    pub ret: return,
    } else if (num_channels > SDCA_MAX_CHANNEL_COUNT) {
    dev_err(dev, "cluster %#x: maximum number of channels exceeded\n",
    pub -EINVAL: return,
    }
    pub GFP_KERNEL): *mut *mut channels = devm_kcalloc(dev, num_channels, sizeof(channels),,
    if (!channels)
    pub -ENOMEM: return,
    pub {: for (i = 0; i < num_channels; i++),
    pub channel_property: [c_char; SDCA_PROPERTY_LENGTH],
    pub channel_node: *mut fwnode_handle,
// DisCo uses upper-case for hex numbers
    snprintf(channel_property, sizeof(channel_property),
    pub 1): "mipi-sdca-channel-%d-subproperties", i +,
    pub channel_property): channel_node = fwnode_get_named_child_node(cluster_node,,
    if (!channel_node) {
    dev_err(dev, "cluster %#x: channel node %s not found\n",
    pub channel_property): cluster->id,,
    pub -EINVAL: return,
    }
    pub &channels[i]): ret = find_sdca_cluster_channel(dev, cluster, channel_node,,
    if (ret)
    pub ret: return,
    }
    pub num_channels: cluster->num_channels =,
    pub channels: cluster->channels =,
    pub 0: return,
    }
    static int find_sdca_clusters(struct device *dev,
    struct fwnode_handle *function_node,
    struct sdca_function_data *function)
    {
    pub clusters: *mut sdca_cluster,
    pub num_clusters: c_int,
    pub ret: int i,,
    pub "mipi-sdca-cluster-id-list"): num_clusters = fwnode_property_count_u32(function_node,,
    if (!num_clusters || num_clusters == -EINVAL) {
    pub 0: return,
    } else if (num_clusters < 0) {
    dev_err(dev, "%pfwP: failed to read cluster id list: %d\n",
    pub num_clusters): function_node,,
    pub num_clusters: return,
    } else if (num_clusters > SDCA_MAX_CLUSTER_COUNT) {
    pub function_node): dev_err(dev, "%pfwP: maximum number of clusters exceeded\n",,
    pub -EINVAL: return,
    }
    pub GFP_KERNEL): *mut *mut clusters = devm_kcalloc(dev, num_clusters, sizeof(clusters),,
    if (!clusters)
    pub -ENOMEM: return,
    u32 *cluster_list __free(kfree) = kzalloc_objs(*cluster_list,
    if (!cluster_list)
    pub -ENOMEM: return,
    fwnode_property_read_u32_array(function_node, "mipi-sdca-cluster-id-list",
    pub num_clusters): cluster_list,,
    pub i++): for (i = 0; i < num_clusters;,
    pub cluster_list: [clusters[i].id =; i],
// now read subproperties
    pub {: for (i = 0; i < num_clusters; i++),
    pub cluster_property: [c_char; SDCA_PROPERTY_LENGTH],
    pub cluster_node: *mut fwnode_handle,
// DisCo uses upper-case for hex numbers
    snprintf(cluster_property, sizeof(cluster_property),
    pub clusters[i].id): "mipi-sdca-cluster-id-0x%X-subproperties",,
    pub cluster_property): cluster_node = fwnode_get_named_child_node(function_node,,
    if (!cluster_node) {
    dev_err(dev, "%pfwP: cluster node %s not found\n",
    pub cluster_property): function_node,,
    pub -EINVAL: return,
    }
    pub &clusters[i]): ret = find_sdca_cluster_channels(dev, cluster_node,,
    if (ret)
    pub ret: return,
    }
    pub num_clusters: function->num_clusters =,
    pub clusters: function->clusters =,
    pub 0: return,
    }
    static int find_sdca_filesets(struct device *dev, struct fwnode_handle *function_node,
    struct sdca_function_data *function)
    {
    pub 3: static int mult_fileset =,
    pub fileset_name: [c_char; SDCA_PROPERTY_LENGTH],
    pub sets: *mut sdca_fdl_set,
    pub num_sets: c_int,
    pub j: int i,,
    num_sets = fwnode_property_count_u32(function_node,
    if (num_sets == 0 || num_sets == -EINVAL) {
    pub function_node): dev_dbg(dev, "%pfwP: file set id list missing\n",,
    pub 0: return,
    } else if (num_sets < 0) {
    dev_err(dev, "%pfwP: failed to read file set list: %d\n",
    pub num_sets): function_node,,
    pub num_sets: return,
    }
    u32 *filesets_list __free(kfree) = kcalloc(num_sets, sizeof(u32),
    if (!filesets_list)
    pub -ENOMEM: return,
    fwnode_property_read_u32_array(function_node, "mipi-sdca-file-set-id-list",
    pub num_sets): filesets_list,,
    pub GFP_KERNEL): *mut *mut sets = devm_kcalloc(dev, num_sets, sizeof(sets),,
    if (!sets)
    pub -ENOMEM: return,
    pub {: for (i = 0; i < num_sets; i++),
    pub &sets[i]: *mut *mut sdca_fdl_set set =,
    pub files: *mut sdca_fdl_file,
    pub num_entries: int num_files,,
    snprintf(fileset_name, sizeof(fileset_name),
    pub filesets_list[i]): "mipi-sdca-file-set-id-0x%X",,
    pub fileset_name): num_entries = fwnode_property_count_u32(function_node,,
    if (num_entries <= 0) {
    dev_err(dev, "%pfwP: file set %d missing entries: %d\n",
    pub num_entries): function_node, filesets_list[i],,
    pub -EINVAL: return,
    } else if (num_entries % mult_fileset != 0) {
    dev_err(dev, "%pfwP: file set %d files not multiple of %d\n",
    pub mult_fileset): function_node, filesets_list[i],,
    pub -EINVAL: return,
    }
    pub filesets_list[i]): dev_dbg(dev, "fileset: %#x\n",,
    files = devm_kcalloc(dev, num_entries / mult_fileset,
    pub GFP_KERNEL): *mut *mut sizeof(files),,
    if (!files)
    pub -ENOMEM: return,
    u32 *fileset_entries __free(kfree) = kcalloc(num_entries, sizeof(u32),
    if (!fileset_entries)
    pub -ENOMEM: return,
    fwnode_property_read_u32_array(function_node, fileset_name,
    pub num_entries): fileset_entries,,
    pub {: for (j = 0, num_files = 0; j < num_entries; num_files++),
    pub &files[num_files]: *mut *mut sdca_fdl_file file =,
    pub fileset_entries: [file->vendor_id =; j++],
    pub fileset_entries: [file->file_id =; j++],
    pub fileset_entries: [file->fdl_offset =; j++],
    dev_dbg(dev, "file: %#x, vendor: %#x, offset: %#x\n",
    pub file->fdl_offset): file->file_id, file->vendor_id,,
    }
    pub filesets_list: [set->id =; i],
    pub num_files: set->num_files =,
    pub files: set->files =,
    }
    pub num_sets: function->fdl_data.num_sets =,
    pub sets: function->fdl_data.sets =,
    pub 0: return,
    }
    static int find_sdca_hid(struct device *dev, struct fwnode_handle *function_node,
    struct sdca_function_data *function)
    {
    pub num_desc: c_int,
    pub "mipi-sdca-hid-descriptor"): num_desc = fwnode_property_count_u8(function_node,,
    if (!num_desc) {
    pub 0: return,
    } else if (num_desc < 0) {
    dev_err(dev, "%pfwP: failed to read hid descriptor: %d\n",
    pub num_desc): function_node,,
    pub num_desc: return,
    } else if (num_desc > sizeof(function.hid.desc)) {
    dev_err(dev, "%pfwP: hid descriptor too large: %d\n",
    pub num_desc): function_node,,
    pub -EINVAL: return,
    }
    fwnode_property_read_u8_array(function_node, "mipi-sdca-hid-descriptor",
    pub num_desc): *mut *mut (u8 )&function->hid.desc,,
    if (!function.hid.desc.bNumDescriptors)
    pub 0: return,
    pub "mipi-sdca-report-descriptor"): num_desc = fwnode_property_count_u8(function_node,,
    if (num_desc <= 0) {
    dev_err(dev, "%pfwP: failed to read report descriptor: %d\n",
    pub num_desc): function_node,,
    if (!num_desc)
    pub -EINVAL: return,
    pub num_desc: return,
    }
    pub GFP_KERNEL): function->hid.report_desc = devm_kzalloc(dev, num_desc,,
    if (!function.hid.report_desc)
    pub -ENOMEM: return,
    fwnode_property_read_u8_array(function_node, "mipi-sdca-report-descriptor",
    pub num_desc): function->hid.report_desc,,
    pub 0: return,
    }
//
// sdca_parse_function - parse ACPI DisCo for a Function
// @dev: Pointer to device against which function data will be allocated.
// @function: Pointer to the Function information, to be populated.
//
// Return: Returns 0 for success.
//
#[no_mangle]
pub unsafe extern "C" fn sdca_parse_function(dev: *mut device, function: *mut sdca_function_data) -> c_int {
    int sdca_parse_function(struct device *dev, struct sdca_function_data *function)
    {
    pub function->desc->node: *mut *mut fwnode_handle node =,
    pub tmp: u32,
    pub ret: c_int,
    pub &tmp): ret = fwnode_property_read_u32(node, "mipi-sdca-function-busy-max-delay",,
    if (!ret)
    pub tmp: function->busy_max_delay =,
    pub &tmp): ret = fwnode_property_read_u32(node, "mipi-sdca-function-reset-max-delay",,
    if (ret || tmp == 0) {
    pub 100mS\n"): dev_dbg(dev, "reset delay missing, defaulting to,
    pub 100000: function->reset_max_delay =,
    } else {
    pub tmp: function->reset_max_delay =,
    }
    dev_dbg(dev, "%pfwP: name %s busy delay %dus reset delay %dus\n",
    node, function.desc.name, function.busy_max_delay,
    pub function): ret = find_sdca_init_table(dev, node,,
    if (ret)
    pub ret: return,
    pub function): ret = find_sdca_entities(dev, node,,
    if (ret)
    pub ret: return,
    pub function): ret = find_sdca_connections(dev, node,,
    if (ret)
    pub ret: return,
    pub function): ret = find_sdca_clusters(dev, node,,
    if (ret < 0)
    pub ret: return,
    pub function): ret = find_sdca_filesets(dev, node,,
    if (ret)
    pub ret: return,
    switch (function.desc.type) {
    case SDCA_FUNCTION_TYPE_HID:
    pub function): ret = find_sdca_hid(dev, node,,
    if (ret)
    pub ret: return,
    default:
    }
    pub 0: return,
    }
    pub "SND_SOC_SDCA"): EXPORT_SYMBOL_NS(sdca_parse_function,,
    const char *sdca_find_terminal_name(enum sdca_terminal_type type)
    {
    switch (type) {
    case SDCA_TERM_TYPE_LINEIN_STEREO:
    pub SDCA_TERM_TYPE_LINEIN_STEREO_NAME: return,
    case SDCA_TERM_TYPE_LINEIN_FRONT_LR:
    pub SDCA_TERM_TYPE_LINEIN_FRONT_LR_NAME: return,
    case SDCA_TERM_TYPE_LINEIN_CENTER_LFE:
    pub SDCA_TERM_TYPE_LINEIN_CENTER_LFE_NAME: return,
    case SDCA_TERM_TYPE_LINEIN_SURROUND_LR:
    pub SDCA_TERM_TYPE_LINEIN_SURROUND_LR_NAME: return,
    case SDCA_TERM_TYPE_LINEIN_REAR_LR:
    pub SDCA_TERM_TYPE_LINEIN_REAR_LR_NAME: return,
    case SDCA_TERM_TYPE_LINEOUT_STEREO:
    pub SDCA_TERM_TYPE_LINEOUT_STEREO_NAME: return,
    case SDCA_TERM_TYPE_LINEOUT_FRONT_LR:
    pub SDCA_TERM_TYPE_LINEOUT_FRONT_LR_NAME: return,
    case SDCA_TERM_TYPE_LINEOUT_CENTER_LFE:
    pub SDCA_TERM_TYPE_LINEOUT_CENTER_LFE_NAME: return,
    case SDCA_TERM_TYPE_LINEOUT_SURROUND_LR:
    pub SDCA_TERM_TYPE_LINEOUT_SURROUND_LR_NAME: return,
    case SDCA_TERM_TYPE_LINEOUT_REAR_LR:
    pub SDCA_TERM_TYPE_LINEOUT_REAR_LR_NAME: return,
    case SDCA_TERM_TYPE_MIC_JACK:
    pub SDCA_TERM_TYPE_MIC_JACK_NAME: return,
    case SDCA_TERM_TYPE_STEREO_JACK:
    pub SDCA_TERM_TYPE_STEREO_JACK_NAME: return,
    case SDCA_TERM_TYPE_FRONT_LR_JACK:
    pub SDCA_TERM_TYPE_FRONT_LR_JACK_NAME: return,
    case SDCA_TERM_TYPE_CENTER_LFE_JACK:
    pub SDCA_TERM_TYPE_CENTER_LFE_JACK_NAME: return,
    case SDCA_TERM_TYPE_SURROUND_LR_JACK:
    pub SDCA_TERM_TYPE_SURROUND_LR_JACK_NAME: return,
    case SDCA_TERM_TYPE_REAR_LR_JACK:
    pub SDCA_TERM_TYPE_REAR_LR_JACK_NAME: return,
    case SDCA_TERM_TYPE_HEADPHONE_JACK:
    pub SDCA_TERM_TYPE_HEADPHONE_JACK_NAME: return,
    case SDCA_TERM_TYPE_HEADSET_JACK:
    pub SDCA_TERM_TYPE_HEADSET_JACK_NAME: return,
    default:
    pub NULL: return,
    }
    }
    pub "SND_SOC_SDCA"): EXPORT_SYMBOL_NS(sdca_find_terminal_name,,
    struct sdca_control *sdca_selector_find_control(struct device *dev,
    struct sdca_entity *entity,
    const int sel)
    {
    pub i: c_int,
    pub {: for (i = 0; i < entity->num_controls; i++),
    pub &entity->controls[i]: *mut *mut sdca_control control =,
    if (control.sel == sel)
    pub control: return,
    }
    pub sel): dev_err(dev, "%s: control %#x: missing\n", entity->label,,
    pub NULL: return,
    }
    pub "SND_SOC_SDCA"): EXPORT_SYMBOL_NS(sdca_selector_find_control,,
    struct sdca_control_range *sdca_control_find_range(struct device *dev,
    struct sdca_entity *entity,
    struct sdca_control *control,
    int cols, int rows)
    {
    pub &control->range: *mut *mut sdca_control_range range =,
    if ((cols && range.cols != cols) || (rows && range.rows != rows) ||
    !range.data) {
    dev_err(dev, "%s: control %#x: ranges invalid (%d,%d)\n",
    pub range->rows): entity->label, control->sel, range->cols,,
    pub NULL: return,
    }
    pub range: return,
    }
    pub "SND_SOC_SDCA"): EXPORT_SYMBOL_NS(sdca_control_find_range,,
    struct sdca_control_range *sdca_selector_find_range(struct device *dev,
    struct sdca_entity *entity,
    int sel, int cols, int rows)
    {
    pub control: *mut sdca_control,
    pub sel): control = sdca_selector_find_control(dev, entity,,
    if (!control)
    pub NULL: return,
    pub rows): return sdca_control_find_range(dev, entity, control, cols,,
    }
    pub "SND_SOC_SDCA"): EXPORT_SYMBOL_NS(sdca_selector_find_range,,
    struct sdca_cluster *sdca_id_find_cluster(struct device *dev,
    struct sdca_function_data *function,
    const int id)
    {
    pub i: c_int,
    pub {: for (i = 0; i < function->num_clusters; i++),
    pub &function->clusters[i]: *mut *mut sdca_cluster cluster =,
    if (cluster.id == id)
    pub cluster: return,
    }
    pub id): dev_err(dev, "%s: cluster %#x: missing\n", function->desc->name,,
    pub NULL: return,
    }
    pub "SND_SOC_SDCA"): EXPORT_SYMBOL_NS(sdca_id_find_cluster,,
    pub BSD/GPL"): MODULE_LICENSE("Dual,
    pub library"): MODULE_DESCRIPTION("SDCA,
