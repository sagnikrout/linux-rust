//! Automatically rewritten from C to Rust
//! Source: tools/arch/x86/intel_sdsi/intel_sdsi.c
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
// sdsi: Intel On Demand (formerly Software Defined Silicon) tool for
// provisioning certificates and activation payloads on supported cpus.
//
// See https://github.com/intel/intel-sdsi/blob/master/os-interface.rst
// for register descriptions.
//
// Copyright (C) 2022 Intel Corporation. All rights reserved.
//

    typeof(x) _min1 = (x);                  \
    typeof(y) _min2 = (y);                  \
    (void) (&_min1 == &_min2);              \
    _min1 < _min2 ? _min1 : _min2; })

pub const GUID_V1: c_uint = 0x6dd191;
pub const REGS_SIZE_GUID_V1: c_int = 72;
pub const GUID_V2: c_uint = 0xF210D9EF;
pub const REGS_SIZE_GUID_V2: c_int = 80;
pub const STATE_CERT_MAX_SIZE: c_int = 4096;
pub const METER_CERT_MAX_SIZE: c_int = 4096;
pub const STATE_MAX_NUM_LICENSES: c_int = 16;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvram_content_auth_err_sts {
    pub reserved:3: u64,
    pub sdsi_content_auth_err:1: u64,
    pub reserved1:1: u64,
    pub sdsi_metering_auth_err:1: u64,
    pub reserved2:58: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct enabled_features {
    pub reserved:3: u64,
    pub sdsi:1: u64,
    pub reserved1:8: u64,
    pub attestation:1: u64,
    pub reserved2:13: u64,
    pub metering:1: u64,
    pub reserved3:37: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct key_provision_status {
    pub reserved:1: u64,
    pub license_key_provisioned:1: u64,
    pub reserved2:62: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct auth_fail_count {
    pub key_failure_count:3: u64,
    pub key_failure_threshold:3: u64,
    pub auth_failure_count:3: u64,
    pub auth_failure_threshold:3: u64,
    pub reserved:52: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct availability {
    pub reserved:48: u64,
    pub available:3: u64,
    pub threshold:3: u64,
    pub reserved2:10: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvram_update_limit {
    pub reserved:12: u64,
    pub sdsi_50_pct:1: u64,
    pub sdsi_75_pct:1: u64,
    pub sdsi_90_pct:1: u64,
    pub reserved2:49: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sdsi_regs {
    pub ppin: u64,
    pub auth_err_sts: nvram_content_auth_err_sts,
    pub en_features: enabled_features,
    pub key_prov_sts: key_provision_status,
    pub auth_fail_count: auth_fail_count,
    pub prov_avail: availability,
    pub limits: nvram_update_limit,
    pub pcu_cr3_capid_cfg: u64,
    union {
    struct {
    pub socket_id: u64,
    pub v1: },
    struct {
    pub reserved: u64,
    pub socket_id: u64,
    pub reserved2: u64,
    pub v2: },
    pub extra: },
}

pub const CONTENT_TYPE_LK_ENC: c_uint = 0xD;
pub const CONTENT_TYPE_LK_BLOB_ENC: c_uint = 0xE;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct state_certificate {
    pub content_type: u32,
    pub region_rev_id: u32,
    pub header_size: u32,
    pub total_size: u32,
    pub key_size: u32,
    pub num_licenses: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct license_key_info {
    pub key_rev_id: u32,
    pub key_image_content: [u64; 6],
    pub __packed: },

// License Group Types
pub const LBT_ONE_TIME_UPGRADE: c_int = 1;
pub const LBT_METERED_UPGRADE: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct license_blob_content {
    pub type: u32,
    pub id: u64,
    pub ppin: u64,
    pub previous_ppin: u64,
    pub rev_id: u32,
    pub num_bundles: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bundle_encoding {
    pub encoding: u32,
    pub encoding_rsvd: [u32; 7],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct meter_certificate {
    pub signature: u32,
    pub version: u32,
    pub ppin: u64,
    pub counter_unit: u32,
    pub bundle_length: u32,
    pub reserved: u64,
    pub mmrc_encoding: u32,
    pub mmrc_counter: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bundle_encoding_counter {
    pub encoding: u32,
    pub counter: u32,
}

    ((METER_CERT_MAX_SIZE - sizeof(struct meter_certificate)) /	\
    sizeof(struct bundle_encoding_counter))
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sdsi_dev {
    pub regs: sdsi_regs,
    pub sc: state_certificate,
    pub dev_name: *mut c_char,
    pub dev_path: *mut c_char,
    pub guid: u32,
}

    enum command {
    CMD_SOCKET_INFO,
    CMD_METER_CERT,
    CMD_METER_CURRENT_CERT,
    CMD_STATE_CERT,
    CMD_PROV_AKC,
    CMD_PROV_CAP,
    };
#[no_mangle]
unsafe extern "C" fn sdsi_list_devices() {
    static void sdsi_list_devices(void)
    {
    struct dirent *entry;
    DIR *aux_dir;
    let mut found: bool = false;
    aux_dir = opendir(AUX_DEV_PATH);
    if (!aux_dir) {
    fprintf(stderr, "Cannot open directory %s\n", AUX_DEV_PATH);
    return;
    }
    while ((entry = readdir(aux_dir))) {
    if (!strncmp(SDSI_DEV, entry.d_name, strlen(SDSI_DEV))) {
    found = true;
    printf("%s\n", entry.d_name);
    }
    }
    if (!found)
    fprintf(stderr, "No On Demand devices found.\n");
    }
#[no_mangle]
unsafe extern "C" fn sdsi_update_registers(s: *mut sdsi_dev) -> c_int {
    static int sdsi_update_registers(struct sdsi_dev *s)
    {
    FILE *regs_ptr;
    int ret;
    memset(&s.regs, 0, sizeof(s.regs));
// Open the registers file
    ret = chdir(s.dev_path);
    if (ret == -1) {
    perror("chdir");
    return ret;
    }
    regs_ptr = fopen("registers", "r");
    if (!regs_ptr) {
    perror("Could not open 'registers' file");
    return -1;
    }
    if (s.guid != GUID_V1 && s.guid != GUID_V2) {
    fprintf(stderr, "Unrecognized guid, 0x%x\n", s.guid);
    fclose(regs_ptr);
    return -1;
    }
// Update register info for this guid
    ret = fread(&s.regs, sizeof(uint8_t), sizeof(s.regs), regs_ptr);
    if ((s.guid == GUID_V1 && ret != REGS_SIZE_GUID_V1) ||
    (s.guid == GUID_V2 && ret != REGS_SIZE_GUID_V2)) {
    fprintf(stderr, "Could not read 'registers' file\n");
    fclose(regs_ptr);
    return -1;
    }
    fclose(regs_ptr);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sdsi_read_reg(s: *mut sdsi_dev) -> c_int {
    static int sdsi_read_reg(struct sdsi_dev *s)
    {
    int ret;
    ret = sdsi_update_registers(s);
    if (ret)
    return ret;
// Print register info for this guid
    printf("\n");
    printf("Socket information for device %s\n", s.dev_name);
    printf("\n");
    printf("PPIN:                           0x%lx\n", s.regs.ppin);
    printf("NVRAM Content Authorization Error Status\n");
    printf("    SDSi Auth Err Sts:          %s\n", !!s.regs.auth_err_sts.sdsi_content_auth_err ? "Error" : "Okay");
    if (!!s.regs.en_features.metering)
    printf("    Metering Auth Err Sts:      %s\n", !!s.regs.auth_err_sts.sdsi_metering_auth_err ? "Error" : "Okay");
    printf("Enabled Features\n");
    printf("    On Demand:                  %s\n", !!s.regs.en_features.sdsi ? "Enabled" : "Disabled");
    printf("    Attestation:                %s\n", !!s.regs.en_features.attestation ? "Enabled" : "Disabled");
    printf("    On Demand:                  %s\n", !!s.regs.en_features.sdsi ? "Enabled" : "Disabled");
    printf("    Metering:                   %s\n", !!s.regs.en_features.metering ? "Enabled" : "Disabled");
    printf("License Key (AKC) Provisioned:  %s\n", !!s.regs.key_prov_sts.license_key_provisioned ? "Yes" : "No");
    printf("Authorization Failure Count\n");
    printf("    AKC Failure Count:          %d\n", s.regs.auth_fail_count.key_failure_count);
    printf("    AKC Failure Threshold:      %d\n", s.regs.auth_fail_count.key_failure_threshold);
    printf("    CAP Failure Count:          %d\n", s.regs.auth_fail_count.auth_failure_count);
    printf("    CAP Failure Threshold:      %d\n", s.regs.auth_fail_count.auth_failure_threshold);
    printf("Provisioning Availability\n");
    printf("    Updates Available:          %d\n", s.regs.prov_avail.available);
    printf("    Updates Threshold:          %d\n", s.regs.prov_avail.threshold);
    printf("NVRAM Udate Limit\n");
    printf("    50%% Limit Reached:          %s\n", !!s.regs.limits.sdsi_50_pct ? "Yes" : "No");
    printf("    75%% Limit Reached:          %s\n", !!s.regs.limits.sdsi_75_pct ? "Yes" : "No");
    printf("    90%% Limit Reached:          %s\n", !!s.regs.limits.sdsi_90_pct ? "Yes" : "No");
    if (s.guid == GUID_V1)
    printf("Socket ID:                      %ld\n", s.regs.extra.v1.socket_id & 0xF);
    else
    printf("Socket ID:                      %ld\n", s.regs.extra.v2.socket_id & 0xF);
    return 0;
    }
    static char *license_blob_type(uint32_t type)
    {
    switch (type) {
    case LBT_ONE_TIME_UPGRADE:
    return "One time upgrade";
    case LBT_METERED_UPGRADE:
    return "Metered upgrade";
    default:
    return "Unknown license blob type";
    }
    }
    static char *content_type(uint32_t type)
    {
    switch (type) {
    case  CONTENT_TYPE_LK_ENC:
    return "Licencse key encoding";
    case CONTENT_TYPE_LK_BLOB_ENC:
    return "License key + Blob encoding";
    default:
    return "Unknown content type";
    }
    }
#[no_mangle]
unsafe extern "C" fn get_feature(encoding: u32, feature[5]: c_char) {
    static void get_feature(uint32_t encoding, char feature[5])
    {
    char *name = (char *)&encoding;
    feature[4] = '\0';
    feature[3] = name[0];
    feature[2] = name[1];
    feature[1] = name[2];
    feature[0] = name[3];
    }
#[no_mangle]
unsafe extern "C" fn sdsi_meter_cert_show(s: *mut sdsi_dev, show_current: bool) -> c_int {
    static int sdsi_meter_cert_show(struct sdsi_dev *s, bool show_current)
    {
    char buf[METER_CERT_MAX_SIZE] = {0};
    struct bundle_encoding_counter *bec;
    struct meter_certificate *mc;
    let mut count: u32 = 0;
    FILE *cert_ptr;
    char *cert_fname;
    int ret, size;
    char name[FEAT_LEN];
    ret = sdsi_update_registers(s);
    if (ret)
    return ret;
    if (!s.regs.en_features.sdsi) {
    fprintf(stderr, "SDSi feature is present but not enabled.\n");
    return -1;
    }
    if (!s.regs.en_features.metering) {
    fprintf(stderr, "Metering not supporting on this socket.\n");
    return -1;
    }
    ret = chdir(s.dev_path);
    if (ret == -1) {
    perror("chdir");
    return ret;
    }
    cert_fname = show_current ? "meter_current" : "meter_certificate";
    cert_ptr = fopen(cert_fname, "r");
    if (!cert_ptr) {
    fprintf(stderr, "Could not open '%s' file: %s", cert_fname, strerror(errno));
    return -1;
    }
    size = fread(buf, 1, sizeof(buf), cert_ptr);
    if (!size) {
    fprintf(stderr, "Could not read '%s' file\n", cert_fname);
    fclose(cert_ptr);
    return -1;
    }
    fclose(cert_ptr);
    mc = (struct meter_certificate *)buf;
    printf("\n");
    printf("Meter certificate for device %s\n", s.dev_name);
    printf("\n");
    get_feature(mc.signature, name);
    printf("Signature:                    %s\n", name);
    printf("Version:                      %d\n", mc.version);
    printf("Count Unit:                   %dms\n", mc.counter_unit);
    printf("PPIN:                         0x%lx\n", mc.ppin);
    printf("Feature Bundle Length:        %d\n", mc.bundle_length);
    get_feature(mc.mmrc_encoding, name);
    printf("MMRC encoding:                %s\n", name);
    printf("MMRC counter:                 %d\n", mc.mmrc_counter);
    if (mc.bundle_length % METER_BUNDLE_SIZE) {
    fprintf(stderr, "Invalid bundle length\n");
    return -1;
    }
    if (mc.bundle_length > METER_MAX_NUM_BUNDLES * METER_BUNDLE_SIZE)  {
    fprintf(stderr, "More than %ld bundles: actual %ld\n",
    METER_MAX_NUM_BUNDLES, BUNDLE_COUNT(mc.bundle_length));
    return -1;
    }
    bec = (struct bundle_encoding_counter *)(mc + 1);
    printf("Number of Feature Counters:   %ld\n", BUNDLE_COUNT(mc.bundle_length));
    while (count < BUNDLE_COUNT(mc.bundle_length)) {
    char feature[FEAT_LEN];
    get_feature(bec[count].encoding, feature);
    printf("    %s:          %d\n", feature, bec[count].counter);
    ++count;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sdsi_state_cert_show(s: *mut sdsi_dev) -> c_int {
    static int sdsi_state_cert_show(struct sdsi_dev *s)
    {
    char buf[STATE_CERT_MAX_SIZE] = {0};
    struct state_certificate *sc;
    struct license_key_info *lki;
    let mut offset: u32 = 0;
    let mut count: u32 = 0;
    FILE *cert_ptr;
    int ret, size;
    ret = sdsi_update_registers(s);
    if (ret)
    return ret;
    if (!s.regs.en_features.sdsi) {
    fprintf(stderr, "On Demand feature is present but not enabled.");
    fprintf(stderr, " Unable to read state certificate");
    return -1;
    }
    ret = chdir(s.dev_path);
    if (ret == -1) {
    perror("chdir");
    return ret;
    }
    cert_ptr = fopen("state_certificate", "r");
    if (!cert_ptr) {
    perror("Could not open 'state_certificate' file");
    return -1;
    }
    size = fread(buf, 1, sizeof(buf), cert_ptr);
    if (!size) {
    fprintf(stderr, "Could not read 'state_certificate' file\n");
    fclose(cert_ptr);
    return -1;
    }
    fclose(cert_ptr);
    sc = (struct state_certificate *)buf;
// Print register info for this guid
    printf("\n");
    printf("State certificate for device %s\n", s.dev_name);
    printf("\n");
    printf("Content Type:          %s\n", content_type(sc.content_type));
    printf("Region Revision ID:    %d\n", sc.region_rev_id);
    printf("Header Size:           %d\n", sc.header_size * 4);
    printf("Total Size:            %d\n", sc.total_size);
    printf("OEM Key Size:          %d\n", sc.key_size * 4);
    printf("Number of Licenses:    %d\n", sc.num_licenses);
// Skip over the license sizes 4 bytes per license) to get the license key info
    lki = (void *)sc + sizeof(*sc) + (4 * sc.num_licenses);
    printf("License blob Info:\n");
    printf("    License Key Revision ID:    0x%x\n", lki.key_rev_id);
    printf("    License Key Image Content:  0x%lx%lx%lx%lx%lx%lx\n",
    lki.key_image_content[5], lki.key_image_content[4],
    lki.key_image_content[3], lki.key_image_content[2],
    lki.key_image_content[1], lki.key_image_content[0]);
    while (count++ < sc.num_licenses) {
    let mut blob_size_field: u32 = *(uint32_t *)(buf + 0x14 + count * 4);
    let mut blob_size: u32 = LICENSE_BLOB_SIZE(blob_size_field);
    let mut license_valid: bool = LICENSE_VALID(blob_size_field);
    struct license_blob_content *lbc =
    (void *)(sc) +			// start of the state certificate
    sizeof(*sc) +			// size of the state certificate
    (4 * sc.num_licenses) +	// total size of the blob size blocks
    sizeof(*lki) +			// size of the license key info
    offset;				// offset to this blob content
    struct bundle_encoding *bundle = (void *)(lbc) + sizeof(*lbc);
    char feature[FEAT_LEN];
    uint32_t i;
    printf("     Blob %d:\n", count - 1);
    printf("        License blob size:          %u\n", blob_size);
    printf("        License is valid:           %s\n", license_valid ? "Yes" : "No");
    printf("        License blob type:          %s\n", license_blob_type(lbc.type));
    printf("        License blob ID:            0x%lx\n", lbc.id);
    printf("        PPIN:                       0x%lx\n", lbc.ppin);
    printf("        Previous PPIN:              0x%lx\n", lbc.previous_ppin);
    printf("        Blob revision ID:           %u\n", lbc.rev_id);
    printf("        Number of Features:         %u\n", lbc.num_bundles);
    for (i = 0; i < min(lbc.num_bundles, STATE_MAX_NUM_IN_BUNDLE); i++) {
    get_feature(bundle[i].encoding, feature);
    printf("                 Feature %d:         %s\n", i, feature);
    }
    if (lbc.num_bundles > STATE_MAX_NUM_IN_BUNDLE)
    fprintf(stderr, "        Warning: %d > %d licenses in bundle reported.\n",
    lbc.num_bundles, STATE_MAX_NUM_IN_BUNDLE);
    offset += blob_size;
    };
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sdsi_provision(s: *mut sdsi_dev, bin_file: *mut c_char, command: enum command) -> c_int {
    static int sdsi_provision(struct sdsi_dev *s, char *bin_file, enum command command)
    {
    int bin_fd, prov_fd, size, ret;
    char buf[STATE_CERT_MAX_SIZE] = { 0 };
    char cap[] = "provision_cap";
    char akc[] = "provision_akc";
    char *prov_file;
    if (!bin_file) {
    fprintf(stderr, "No binary file provided\n");
    return -1;
    }
// Open the binary
    bin_fd = open(bin_file, O_RDONLY);
    if (bin_fd == -1) {
    fprintf(stderr, "Could not open file %s: %s\n", bin_file, strerror(errno));
    return bin_fd;
    }
    prov_file = (command == CMD_PROV_AKC) ? akc : cap;
    ret = chdir(s.dev_path);
    if (ret == -1) {
    perror("chdir");
    close(bin_fd);
    return ret;
    }
// Open the provision file
    prov_fd = open(prov_file, O_WRONLY);
    if (prov_fd == -1) {
    fprintf(stderr, "Could not open file %s: %s\n", prov_file, strerror(errno));
    close(bin_fd);
    return prov_fd;
    }
// Read the binary file into the buffer
    size = read(bin_fd, buf, STATE_CERT_MAX_SIZE);
    if (size == -1) {
    close(bin_fd);
    close(prov_fd);
    return -1;
    }
    ret = write(prov_fd, buf, size);
    if (ret == -1) {
    close(bin_fd);
    close(prov_fd);
    perror("Provisioning failed");
    return ret;
    }
    printf("Provisioned %s file %s successfully\n", prov_file, bin_file);
    close(bin_fd);
    close(prov_fd);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sdsi_provision_akc(s: *mut sdsi_dev, bin_file: *mut c_char) -> c_int {
    static int sdsi_provision_akc(struct sdsi_dev *s, char *bin_file)
    {
    int ret;
    ret = sdsi_update_registers(s);
    if (ret)
    return ret;
    if (!s.regs.en_features.sdsi) {
    fprintf(stderr, "On Demand feature is present but not enabled. Unable to provision");
    return -1;
    }
    if (!s.regs.prov_avail.available) {
    fprintf(stderr, "Maximum number of updates (%d) has been reached.\n",
    s.regs.prov_avail.threshold);
    return -1;
    }
    if (s.regs.auth_fail_count.key_failure_count ==
    s.regs.auth_fail_count.key_failure_threshold) {
    fprintf(stderr, "Maximum number of AKC provision failures (%d) has been reached.\n",
    s.regs.auth_fail_count.key_failure_threshold);
    fprintf(stderr, "Power cycle the system to reset the counter\n");
    return -1;
    }
    return sdsi_provision(s, bin_file, CMD_PROV_AKC);
    }
#[no_mangle]
unsafe extern "C" fn sdsi_provision_cap(s: *mut sdsi_dev, bin_file: *mut c_char) -> c_int {
    static int sdsi_provision_cap(struct sdsi_dev *s, char *bin_file)
    {
    int ret;
    ret = sdsi_update_registers(s);
    if (ret)
    return ret;
    if (!s.regs.en_features.sdsi) {
    fprintf(stderr, "On Demand feature is present but not enabled. Unable to provision");
    return -1;
    }
    if (!s.regs.prov_avail.available) {
    fprintf(stderr, "Maximum number of updates (%d) has been reached.\n",
    s.regs.prov_avail.threshold);
    return -1;
    }
    if (s.regs.auth_fail_count.auth_failure_count ==
    s.regs.auth_fail_count.auth_failure_threshold) {
    fprintf(stderr, "Maximum number of CAP provision failures (%d) has been reached.\n",
    s.regs.auth_fail_count.auth_failure_threshold);
    fprintf(stderr, "Power cycle the system to reset the counter\n");
    return -1;
    }
    return sdsi_provision(s, bin_file, CMD_PROV_CAP);
    }
#[no_mangle]
unsafe extern "C" fn read_sysfs_data(file: *const c_char, value: *mut c_int) -> c_int {
    static int read_sysfs_data(const char *file, int *value)
    {
    char buff[16];
    FILE *fp;
    fp = fopen(file, "r");
    if (!fp) {
    perror(file);
    return -1;
    }
    if (!fgets(buff, 16, fp)) {
    fprintf(stderr, "Failed to read file '%s'", file);
    fclose(fp);
    return -1;
    }
    fclose(fp);
// value = strtol(buff, NULL, 0);
    return 0;
    }
    static struct sdsi_dev *sdsi_create_dev(char *dev_no)
    {
    let mut dev_name_len: c_int = sizeof(SDSI_DEV) + strlen(dev_no) + 1;
    struct sdsi_dev *s;
    int guid;
    DIR *dir;
    s = (struct sdsi_dev *)malloc(sizeof(*s));
    if (!s) {
    perror("malloc");
    return core::ptr::null_mut();
    }
    s.dev_name = (char *)malloc(sizeof(SDSI_DEV) + strlen(dev_no) + 1);
    if (!s.dev_name) {
    perror("malloc");
    free(s);
    return core::ptr::null_mut();
    }
    snprintf(s.dev_name, dev_name_len, "%s.%s", SDSI_DEV, dev_no);
    s.dev_path = (char *)malloc(sizeof(AUX_DEV_PATH) + dev_name_len);
    if (!s.dev_path) {
    perror("malloc");
    free(s.dev_name);
    free(s);
    return core::ptr::null_mut();
    }
    snprintf(s.dev_path, sizeof(AUX_DEV_PATH) + dev_name_len, "%s%s", AUX_DEV_PATH,
    s.dev_name);
    dir = opendir(s.dev_path);
    if (!dir) {
    fprintf(stderr, "Could not open directory '%s': %s\n", s.dev_path,
    strerror(errno));
    free(s.dev_path);
    free(s.dev_name);
    free(s);
    return core::ptr::null_mut();
    }
    if (chdir(s.dev_path) == -1) {
    perror("chdir");
    free(s.dev_path);
    free(s.dev_name);
    free(s);
    return core::ptr::null_mut();
    }
    if (read_sysfs_data("guid", &guid)) {
    free(s.dev_path);
    free(s.dev_name);
    free(s);
    return core::ptr::null_mut();
    }
    s.guid = guid;
    return s;
    }
#[no_mangle]
unsafe extern "C" fn sdsi_free_dev(s: *mut sdsi_dev) {
    static void sdsi_free_dev(struct sdsi_dev *s)
    {
    free(s.dev_path);
    free(s.dev_name);
    free(s);
    }
#[no_mangle]
unsafe extern "C" fn usage(prog: *mut c_char) {
    static void usage(char *prog)
    {
    printf("Usage: %s [-l] [-d DEVNO [-i] [-s] [-m | -C] [-a FILE] [-c FILE]\n", prog);
    }
#[no_mangle]
unsafe extern "C" fn show_help() {
    static void show_help(void)
    {
    printf("Commands:\n");
    printf("  %-18s\t%s\n", "-l, --list",           "list available On Demand devices");
    printf("  %-18s\t%s\n", "-d, --devno DEVNO",    "On Demand device number");
    printf("  %-18s\t%s\n", "-i, --info",           "show socket information");
    printf("  %-18s\t%s\n", "-s, --state",          "show state certificate data");
    printf("  %-18s\t%s\n", "-m, --meter",          "show meter certificate data");
    printf("  %-18s\t%s\n", "-C, --meter_current",  "show live unattested meter data");
    printf("  %-18s\t%s\n", "-a, --akc FILE",       "provision socket with AKC FILE");
    printf("  %-18s\t%s\n", "-c, --cap FILE>",      "provision socket with CAP FILE");
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv[]: *mut c_char) -> c_int {
    int main(int argc, char *argv[])
    {
    char bin_file[PATH_MAX], *dev_no = core::ptr::null_mut();
    let mut device_selected: bool = false;
    char *progname;
    let mut command: enum command = -1;
    struct sdsi_dev *s;
    let mut ret: c_int = 0, opt;
    let mut option_index: c_int = 0;
    static struct option long_options[] = {
    {"akc",			required_argument,	0, 'a'},
    {"cap",			required_argument,	0, 'c'},
    {"devno",		required_argument,	0, 'd'},
    {"help",		no_argument,		0, 'h'},
    {"info",		no_argument,		0, 'i'},
    {"list",		no_argument,		0, 'l'},
    {"meter",		no_argument,		0, 'm'},
    {"meter_current",	no_argument,		0, 'C'},
    {"state",		no_argument,		0, 's'},
    {0,			0,			0, 0 }
    };
    progname = argv[0];
    while ((opt = getopt_long_only(argc, argv, "+a:c:d:hilmCs", long_options,
    &option_index)) != -1) {
    switch (opt) {
    case 'd':
    dev_no = optarg;
    device_selected = true;
    break;
    case 'l':
    sdsi_list_devices();
    return 0;
    case 'i':
    command = CMD_SOCKET_INFO;
    break;
    case 'm':
    command = CMD_METER_CERT;
    break;
    case 'C':
    command = CMD_METER_CURRENT_CERT;
    break;
    case 's':
    command = CMD_STATE_CERT;
    break;
    case 'a':
    case 'c':
    if (!access(optarg, F_OK) == 0) {
    fprintf(stderr, "Could not open file '%s': %s\n", optarg,
    strerror(errno));
    return -1;
    }
    if (!realpath(optarg, bin_file)) {
    perror("realpath");
    return -1;
    }
    command = (opt == 'a') ? CMD_PROV_AKC : CMD_PROV_CAP;
    break;
    case 'h':
    usage(progname);
    show_help();
    return 0;
    default:
    usage(progname);
    return -1;
    }
    }
    if (device_selected) {
    s = sdsi_create_dev(dev_no);
    if (!s)
    return -1;
    switch (command) {
    case CMD_SOCKET_INFO:
    ret = sdsi_read_reg(s);
    break;
    case CMD_METER_CERT:
    ret = sdsi_meter_cert_show(s, false);
    break;
    case CMD_METER_CURRENT_CERT:
    ret = sdsi_meter_cert_show(s, true);
    break;
    case CMD_STATE_CERT:
    ret = sdsi_state_cert_show(s);
    break;
    case CMD_PROV_AKC:
    ret = sdsi_provision_akc(s, bin_file);
    break;
    case CMD_PROV_CAP:
    ret = sdsi_provision_cap(s, bin_file);
    break;
    default:
    fprintf(stderr, "No command specified\n");
    return -1;
    }
    sdsi_free_dev(s);
    } else {
    fprintf(stderr, "No device specified\n");
    return -1;
    }
    return ret;
    }
