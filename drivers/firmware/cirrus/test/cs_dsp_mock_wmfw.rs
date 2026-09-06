//! Automatically rewritten from C to Rust
//! Source: drivers/firmware/cirrus/test/cs_dsp_mock_wmfw.c
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
// wmfw file builder for cs_dsp KUnit tests.
//
// Copyright (C) 2024 Cirrus Logic, Inc. and
// Cirrus Logic International Semiconductor Ltd.

// Buffer large enough for bin file content
pub const CS_DSP_MOCK_WMFW_BUF_SIZE: c_int = 131072;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cs_dsp_mock_wmfw_builder {
    pub test_priv: *mut cs_dsp_test,
    pub format_version: c_int,
    pub buf: *mut c_void,
    pub buf_size_bytes: usize,
    pub write_p: *mut c_void,
    pub bytes_used: usize,
    pub alg_data_header: *mut c_void,
    pub num_coeffs: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmfw_adsp2_halo_header {
    pub header: wmfw_header,
    pub sizes: wmfw_adsp2_sizes,
    pub footer: wmfw_footer,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmfw_long_string {
    pub len: __le16,
    pub __counted_by(len): u8 data[] __nonstring,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmfw_short_string {
    pub len: u8,
    pub __counted_by(len): u8 data[] __nonstring,
    pub __packed: },
    KUNIT_DEFINE_ACTION_WRAPPER(vfree_action_wrapper, vfree, void *)
//
// cs_dsp_mock_wmfw_format_version() - Return format version.
//
// @builder:	Pointer to struct cs_dsp_mock_wmfw_builder.
//
// Return: Format version.
//
#[no_mangle]
pub unsafe extern "C" fn cs_dsp_mock_wmfw_format_version(builder: *mut cs_dsp_mock_wmfw_builder) -> c_int {
    int cs_dsp_mock_wmfw_format_version(struct cs_dsp_mock_wmfw_builder *builder)
    {
    pub builder->format_version: return,
    }
    pub "FW_CS_DSP_KUNIT_TEST_UTILS"): EXPORT_SYMBOL_NS_GPL(cs_dsp_mock_wmfw_format_version,,
//
// cs_dsp_mock_wmfw_get_firmware() - Get struct firmware wrapper for data.
//
// @builder:	Pointer to struct cs_dsp_mock_wmfw_builder.
//
// Return: Pointer to a struct firmware wrapper for the data.
//
    struct firmware *cs_dsp_mock_wmfw_get_firmware(struct cs_dsp_mock_wmfw_builder *builder)
    {
    pub fw: *mut firmware,
    if (!builder)
    pub NULL: return,
    pub GFP_KERNEL): *mut *mut fw = kunit_kzalloc(builder->test_priv->test, sizeof(fw),,
    pub fw): KUNIT_ASSERT_NOT_ERR_OR_NULL(builder->test_priv->test,,
    pub builder->buf: fw->data =,
    pub builder->bytes_used: fw->size =,
    pub fw: return,
    }
    pub "FW_CS_DSP_KUNIT_TEST_UTILS"): EXPORT_SYMBOL_NS_GPL(cs_dsp_mock_wmfw_get_firmware,,
//
// cs_dsp_mock_wmfw_add_raw_block() - Add a block to the wmfw file.
//
// @builder:		Pointer to struct cs_dsp_mock_bin_builder.
// @block_type:		Block type.
// @offset:		Offset.
// @payload_data:	Pointer to buffer containing the payload data,
// or NULL if no data.
// @payload_len_bytes:	Length of payload data in bytes, or zero.
//
    void cs_dsp_mock_wmfw_add_raw_block(struct cs_dsp_mock_wmfw_builder *builder,
    int block_type, unsigned int offset,
    const void *payload_data, size_t payload_len_bytes)
    {
    pub builder->write_p: *mut *mut wmfw_region header =,
    pub payload_len_bytes): unsigned int bytes_needed = struct_size_t(struct wmfw_region, data,,
    KUNIT_ASSERT_TRUE(builder.test_priv.test,
    (builder.write_p + bytes_needed) <
    pub CS_DSP_MOCK_WMFW_BUF_SIZE)): (builder->buf +,
    pub 24)): header->offset = cpu_to_le32(offset | (block_type <<,
    pub cpu_to_le32(payload_len_bytes): header->len =,
    if (payload_len_bytes > 0)
    pub payload_len_bytes): memcpy(header->data, payload_data,,
    pub bytes_needed: builder->write_p +=,
    pub bytes_needed: builder->bytes_used +=,
    }
    pub "FW_CS_DSP_KUNIT_TEST_UTILS"): EXPORT_SYMBOL_NS_GPL(cs_dsp_mock_wmfw_add_raw_block,,
//
// cs_dsp_mock_wmfw_add_info() - Add an info block to the wmfw file.
//
// @builder:	Pointer to struct cs_dsp_mock_bin_builder.
// @info:	Pointer to info string to be copied into the file.
//
// The string will be padded to a length that is a multiple of 4 bytes.
//
    void cs_dsp_mock_wmfw_add_info(struct cs_dsp_mock_wmfw_builder *builder,
    const char *info)
    {
    pub strlen(info): size_t info_len =,
    pub NULL: *mut *mut char tmp =,
    if (info_len % 4) {
// Create a padded string with length a multiple of 4
    pub info_len: size_t copy_len =,
    pub 4): info_len = round_up(info_len,,
    pub GFP_KERNEL): tmp = kunit_kzalloc(builder->test_priv->test, info_len,,
    pub tmp): KUNIT_ASSERT_NOT_ERR_OR_NULL(builder->test_priv->test,,
    pub copy_len): memcpy(tmp, info,,
    pub tmp: info =,
    }
    pub info_len): cs_dsp_mock_wmfw_add_raw_block(builder, WMFW_INFO_TEXT, 0, info,,
    pub tmp): kunit_kfree(builder->test_priv->test,,
    }
    pub "FW_CS_DSP_KUNIT_TEST_UTILS"): EXPORT_SYMBOL_NS_GPL(cs_dsp_mock_wmfw_add_info,,
//
// cs_dsp_mock_wmfw_add_data_block() - Add a data block to the wmfw file.
//
// @builder:		  Pointer to struct cs_dsp_mock_bin_builder.
// @mem_region:		  Memory region for the block.
// @mem_offset_dsp_words: Offset to start of destination in DSP words.
// @payload_data:	  Pointer to buffer containing the payload data.
// @payload_len_bytes:	  Length of payload data in bytes.
//
    void cs_dsp_mock_wmfw_add_data_block(struct cs_dsp_mock_wmfw_builder *builder,
    int mem_region, unsigned int mem_offset_dsp_words,
    const void *payload_data, size_t payload_len_bytes)
    {
// Blob payload length must be a multiple of 4
    pub 0): KUNIT_ASSERT_EQ(builder->test_priv->test, payload_len_bytes % 4,,
    cs_dsp_mock_wmfw_add_raw_block(builder, mem_region, mem_offset_dsp_words,
    pub payload_len_bytes): payload_data,,
    }
    pub "FW_CS_DSP_KUNIT_TEST_UTILS"): EXPORT_SYMBOL_NS_GPL(cs_dsp_mock_wmfw_add_data_block,,
    void cs_dsp_mock_wmfw_start_alg_info_block(struct cs_dsp_mock_wmfw_builder *builder,
    unsigned int alg_id,
    const char *name,
    const char *description)
    {
    pub builder->write_p: *mut *mut wmfw_region rgn =,
    pub v1: *mut wmfw_adsp_alg_data,
    pub shortstring: *mut wmfw_short_string,
    pub longstring: *mut wmfw_long_string,
    pub description_len: size_t bytes_needed, name_len,,
    pub offset: c_int,
    pub 0xffffff): KUNIT_ASSERT_LE(builder->test_priv->test, alg_id,,
// Bytes needed for region header
    pub data): bytes_needed = offsetof(struct wmfw_region,,
    pub builder->write_p: builder->alg_data_header =,
    pub 0: builder->num_coeffs =,
    switch (builder.format_version) {
    case 0:
    pub blocks\n"): KUNIT_FAIL(builder->test_priv->test, "wmfwV0 does not have alg,
    case 1:
    pub data): bytes_needed += offsetof(struct wmfw_adsp_alg_data,,
    KUNIT_ASSERT_TRUE(builder.test_priv.test,
    (builder.write_p + bytes_needed) <
    pub CS_DSP_MOCK_WMFW_BUF_SIZE)): (builder->buf +,
    pub bytes_needed): memset(builder->write_p, 0,,
// Create region header
    pub 24): rgn->offset = cpu_to_le32(WMFW_ALGORITHM_DATA <<,
// Create algorithm entry
    pub )&rgn->data[0]: *mut v1 = (struct wmfw_adsp_alg_data,
    pub cpu_to_le32(alg_id): v1->id =,
    if (name)
    pub sizeof(v1->name)): strscpy(v1->name, name,,
    if (description)
    pub sizeof(v1->descr)): strscpy(v1->descr, description,,
    default:
    pub 0: name_len =,
    pub 0: description_len =,
    if (name)
    pub strlen(name): name_len =,
    if (description)
    pub strlen(description): description_len =,
    pub /: *mut *mut bytes_needed += sizeof(__le32); / alg id,
    pub sizeof(__le32)): bytes_needed += round_up(name_len + sizeof(u8),,
    pub sizeof(__le32)): bytes_needed += round_up(description_len + sizeof(__le16),,
    pub /: *mut *mut bytes_needed += sizeof(__le32); / coeff count,
    KUNIT_ASSERT_TRUE(builder.test_priv.test,
    (builder.write_p + bytes_needed) <
    pub CS_DSP_MOCK_WMFW_BUF_SIZE)): (builder->buf +,
    pub bytes_needed): memset(builder->write_p, 0,,
// Create region header
    pub 24): rgn->offset = cpu_to_le32(WMFW_ALGORITHM_DATA <<,
// Create algorithm entry
// ( __le32 *)&rgn->data[0] = cpu_to_le32(alg_id);
    pub )&rgn->data[4]: *mut shortstring = (struct wmfw_short_string,
    pub name_len: shortstring->len =,
    if (name_len)
    pub name_len): memcpy(shortstring->data, name,,
// Round up to next __le32
    offset = round_up(4 + struct_size_t(struct wmfw_short_string, data, name_len),
    pub )&rgn->data[offset]: *mut longstring = (struct wmfw_long_string,
    pub cpu_to_le16(description_len): longstring->len =,
    if (description_len)
    pub description_len): memcpy(longstring->data, description,,
    }
    pub bytes_needed: builder->write_p +=,
    pub bytes_needed: builder->bytes_used +=,
    }
    pub "FW_CS_DSP_KUNIT_TEST_UTILS"): EXPORT_SYMBOL_NS_GPL(cs_dsp_mock_wmfw_start_alg_info_block,,
    void cs_dsp_mock_wmfw_add_coeff_desc(struct cs_dsp_mock_wmfw_builder *builder,
    const struct cs_dsp_mock_coeff_def *def)
    {
    pub v1: *mut wmfw_adsp_coeff_data,
    pub shortstring: *mut wmfw_short_string,
    pub longstring: *mut wmfw_long_string,
    pub description_len: size_t bytes_needed, shortname_len, fullname_len,,
    pub ple32: *mut __le32,
    pub builder->alg_data_header): KUNIT_ASSERT_NOT_NULL(builder->test_priv->test,,
    switch (builder.format_version) {
    case 0:
    case 1:
    pub data): bytes_needed = offsetof(struct wmfw_adsp_coeff_data,,
    KUNIT_ASSERT_TRUE(builder.test_priv.test,
    (builder.write_p + bytes_needed) <
    pub CS_DSP_MOCK_WMFW_BUF_SIZE)): (builder->buf +,
    pub )builder->write_p: *mut v1 = (struct wmfw_adsp_coeff_data,
    pub sizeof(*v1)): *mut memset(v1, 0,,
    pub cpu_to_le16(def->offset_dsp_words): v1->hdr.offset =,
    pub cpu_to_le16(def->mem_type): v1->hdr.type =,
    pub sizeof(v1->hdr)): v1->hdr.size = cpu_to_le32(bytes_needed -,
    pub cpu_to_le16(def->type): v1->ctl_type =,
    pub cpu_to_le16(def->flags): v1->flags =,
    pub cpu_to_le32(def->length_bytes): v1->len =,
    if (def.fullname)
    pub sizeof(v1->name)): strscpy(v1->name, def->fullname,,
    if (def.description)
    pub sizeof(v1->descr)): strscpy(v1->descr, def->description,,
    default:
    pub 0: fullname_len =,
    pub 0: description_len =,
    pub strlen(def->shortname): shortname_len =,
    if (def.fullname)
    pub strlen(def->fullname): fullname_len =,
    if (def.description)
    pub strlen(def->description): description_len =,
    pub /: *mut *mut *mut bytes_needed = sizeof(__le32)  2; / type, offset and size,
    pub sizeof(__le32)): bytes_needed += round_up(shortname_len + sizeof(u8),,
    pub sizeof(__le32)): bytes_needed += round_up(fullname_len + sizeof(u8),,
    pub sizeof(__le32)): bytes_needed += round_up(description_len + sizeof(__le16),,
    pub /: *mut *mut *mut bytes_needed += sizeof(__le32)  2; / flags, type and length,
    KUNIT_ASSERT_TRUE(builder.test_priv.test,
    (builder.write_p + bytes_needed) <
    pub CS_DSP_MOCK_WMFW_BUF_SIZE)): (builder->buf +,
    pub )builder->write_p: *mut ple32 = ( __le32,
// ple32++ = cpu_to_le32(def->offset_dsp_words | (def->mem_type << 16));
// ple32++ = cpu_to_le32(bytes_needed - sizeof(__le32) - sizeof(__le32));
    pub )ple32: *mut shortstring = ( struct wmfw_short_string,
    pub shortname_len: shortstring->len =,
    pub shortname_len): memcpy(shortstring->data, def->shortname,,
// Round up to next __le32 multiple
    ple32 += round_up(struct_size_t(struct wmfw_short_string, data, shortname_len),
    pub sizeof(*ple32): *mut *mut sizeof(ple32)) /,
    pub )ple32: *mut shortstring = ( struct wmfw_short_string,
    pub fullname_len: shortstring->len =,
    pub fullname_len): memcpy(shortstring->data, def->fullname,,
// Round up to next __le32 multiple
    ple32 += round_up(struct_size_t(struct wmfw_short_string, data, fullname_len),
    pub sizeof(*ple32): *mut *mut sizeof(ple32)) /,
    pub )ple32: *mut longstring = ( struct wmfw_long_string,
    pub cpu_to_le16(description_len): longstring->len =,
    pub description_len): memcpy(longstring->data, def->description,,
// Round up to next __le32 multiple
    ple32 += round_up(struct_size_t(struct wmfw_long_string, data, description_len),
    pub sizeof(*ple32): *mut *mut sizeof(ple32)) /,
// ple32++ = cpu_to_le32(def->type | (def->flags << 16));
// ple32 = cpu_to_le32(def->length_bytes);
    }
    pub bytes_needed: builder->write_p +=,
    pub bytes_needed: builder->bytes_used +=,
    }
    pub "FW_CS_DSP_KUNIT_TEST_UTILS"): EXPORT_SYMBOL_NS_GPL(cs_dsp_mock_wmfw_add_coeff_desc,,
#[no_mangle]
pub unsafe extern "C" fn cs_dsp_mock_wmfw_end_alg_info_block(builder: *mut cs_dsp_mock_wmfw_builder) {
    void cs_dsp_mock_wmfw_end_alg_info_block(struct cs_dsp_mock_wmfw_builder *builder)
    {
    pub builder->alg_data_header: *mut *mut wmfw_region rgn =,
    pub v1: *mut wmfw_adsp_alg_data,
    pub shortstring: *const wmfw_short_string,
    pub longstring: *const wmfw_long_string,
    pub offset: usize,
    pub rgn): KUNIT_ASSERT_NOT_NULL(builder->test_priv->test,,
// Fill in data size
    pub )rgn->data): *mut *mut rgn->len = cpu_to_le32((u8 )builder->write_p - (u8,
// Fill in coefficient count
    switch (builder.format_version) {
    case 0:
    case 1:
    pub )&rgn->data[0]: *mut v1 = (struct wmfw_adsp_alg_data,
    pub cpu_to_le32(builder->num_coeffs): v1->ncoeff =,
    default:
    pub /: *mut *mut offset = 4; / skip alg id,
// Get name length and round up to __le32 multiple
    pub )&rgn->data[offset]: *const shortstring = (struct wmfw_short_string,
    offset += round_up(struct_size_t(struct wmfw_short_string, data, shortstring.len),
// Get description length and round up to __le32 multiple
    pub )&rgn->data[offset]: *const longstring = (struct wmfw_long_string,
    offset += round_up(struct_size_t(struct wmfw_long_string, data,
    le16_to_cpu(longstring.len)),
// ( __le32 *)&rgn->data[offset] = cpu_to_le32(builder->num_coeffs);
    }
    pub NULL: builder->alg_data_header =,
    }
    pub "FW_CS_DSP_KUNIT_TEST_UTILS"): EXPORT_SYMBOL_NS_GPL(cs_dsp_mock_wmfw_end_alg_info_block,,
#[no_mangle]
unsafe extern "C" fn cs_dsp_init_adsp2_halo_wmfw(builder: *mut cs_dsp_mock_wmfw_builder) {
    static void cs_dsp_init_adsp2_halo_wmfw(struct cs_dsp_mock_wmfw_builder *builder)
    {
    pub builder->buf: *mut *mut wmfw_adsp2_halo_header hdr =,
    pub builder->test_priv->dsp: *const *const cs_dsp dsp =,
    pub sizeof(hdr->header.magic)): memcpy(hdr->header.magic, "WMFW",,
    pub cpu_to_le32(sizeof(*hdr)): *mut hdr->header.len =,
    pub builder->format_version: hdr->header.ver =,
    pub dsp->type: hdr->header.core =,
    pub cpu_to_le16(dsp->rev): hdr->header.rev =,
    pub WMFW_ADSP2_PM)): hdr->sizes.pm = cpu_to_le32(cs_dsp_mock_size_of_region(dsp,,
    pub WMFW_ADSP2_XM)): hdr->sizes.xm = cpu_to_le32(cs_dsp_mock_size_of_region(dsp,,
    pub WMFW_ADSP2_YM)): hdr->sizes.ym = cpu_to_le32(cs_dsp_mock_size_of_region(dsp,,
    switch (dsp.type) {
    case WMFW_ADSP2:
    pub WMFW_ADSP2_ZM)): hdr->sizes.zm = cpu_to_le32(cs_dsp_mock_size_of_region(dsp,,
    default:
    }
    pub &hdr[1]: builder->write_p =,
    pub sizeof(*hdr): *mut builder->bytes_used +=,
    }
//
// cs_dsp_mock_wmfw_init() - Initialize a struct cs_dsp_mock_wmfw_builder.
//
// @priv:		Pointer to struct cs_dsp_test.
// @format_version:	Required wmfw format version.
//
// Return: Pointer to created struct cs_dsp_mock_wmfw_builder.
//
    struct cs_dsp_mock_wmfw_builder *cs_dsp_mock_wmfw_init(struct cs_dsp_test *priv,
    int format_version)
    {
    pub builder: *mut cs_dsp_mock_wmfw_builder,
    pub 0xff): KUNIT_ASSERT_LE(priv->test, format_version,,
// If format version isn't given use the default for the target core
    if (format_version < 0) {
    switch (priv.dsp.type) {
    case WMFW_ADSP2:
    pub 2: format_version =,
    default:
    pub 3: format_version =,
    }
    }
    pub GFP_KERNEL): *mut *mut builder = kunit_kzalloc(priv->test, sizeof(builder),,
    pub builder): KUNIT_ASSERT_NOT_ERR_OR_NULL(priv->test,,
    pub priv: builder->test_priv =,
    pub format_version: builder->format_version =,
    pub vmalloc(CS_DSP_MOCK_WMFW_BUF_SIZE): builder->buf =,
    pub builder->buf): KUNIT_ASSERT_NOT_NULL(priv->test,,
    pub builder->buf): kunit_add_action_or_reset(priv->test, vfree_action_wrapper,,
    pub CS_DSP_MOCK_WMFW_BUF_SIZE: builder->buf_size_bytes =,
    switch (priv.dsp.type) {
    case WMFW_ADSP2:
    case WMFW_HALO:
    default:
    }
    pub builder: return,
    }
    pub "FW_CS_DSP_KUNIT_TEST_UTILS"): EXPORT_SYMBOL_NS_GPL(cs_dsp_mock_wmfw_init,,
