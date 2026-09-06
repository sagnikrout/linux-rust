//! Automatically rewritten from C to Rust
//! Source: drivers/soc/qcom/qcom_pd_mapper.c
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
// Qualcomm Protection Domain mapper
//
// Copyright (c) 2023 Linaro Ltd.
//

pub const SERVREG_QMI_VERSION: c_uint = 0x101;
pub const SERVREG_QMI_INSTANCE: c_int = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qcom_pdm_domain_data {
    pub domain: *const c_char,
    pub instance_id: u32,
// NULL-terminated array
    pub services: [*const *const c_char; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qcom_pdm_domain {
    pub list: list_head,
    pub name: *const c_char,
    pub instance_id: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qcom_pdm_service {
    pub list: list_head,
    pub domains: list_head,
    pub name: *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qcom_pdm_data {
    pub refcnt: refcount_t,
    pub handle: qmi_handle,
    pub services: list_head,
}

    static DEFINE_MUTEX(qcom_pdm_mutex); /* protects __qcom_pdm_data */
    static struct qcom_pdm_data *__qcom_pdm_data;
    static struct qcom_pdm_service *qcom_pdm_find(struct qcom_pdm_data *data,
    const char *name)
    {
    struct qcom_pdm_service *service;
    list_for_each_entry(service, &data.services, list) {
    if (!strcmp(service.name, name))
    return service;
    }
    return core::ptr::null_mut();
    }
    static int qcom_pdm_add_service_domain(struct qcom_pdm_data *data,
    const char *service_name,
    const char *domain_name,
    u32 instance_id)
    {
    struct qcom_pdm_service *service;
    struct qcom_pdm_domain *domain;
    service = qcom_pdm_find(data, service_name);
    if (service) {
    list_for_each_entry(domain, &service.domains, list) {
    if (!strcmp(domain.name, domain_name))
    return -EBUSY;
    }
    } else {
    service = kzalloc_obj(*service);
    if (!service)
    return -ENOMEM;
    INIT_LIST_HEAD(&service.domains);
    service.name = service_name;
    list_add_tail(&service.list, &data.services);
    }
    domain = kzalloc_obj(*domain);
    if (!domain) {
    if (list_empty(&service.domains)) {
    list_del(&service.list);
    kfree(service);
    }
    return -ENOMEM;
    }
    domain.name = domain_name;
    domain.instance_id = instance_id;
    list_add_tail(&domain.list, &service.domains);
    return 0;
    }
    static int qcom_pdm_add_domain(struct qcom_pdm_data *data,
    const struct qcom_pdm_domain_data *domain)
    {
    int ret;
    int i;
    ret = qcom_pdm_add_service_domain(data,
    TMS_SERVREG_SERVICE,
    domain.domain,
    domain.instance_id);
    if (ret)
    return ret;
    for (i = 0; domain.services[i]; i++) {
    ret = qcom_pdm_add_service_domain(data,
    domain.services[i],
    domain.domain,
    domain.instance_id);
    if (ret)
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn qcom_pdm_free_domains(data: *mut qcom_pdm_data) {
    static void qcom_pdm_free_domains(struct qcom_pdm_data *data)
    {
    struct qcom_pdm_service *service, *tservice;
    struct qcom_pdm_domain *domain, *tdomain;
    list_for_each_entry_safe(service, tservice, &data.services, list) {
    list_for_each_entry_safe(domain, tdomain, &service.domains, list) {
    list_del(&domain.list);
    kfree(domain);
    }
    list_del(&service.list);
    kfree(service);
    }
    }
    static void qcom_pdm_get_domain_list(struct qmi_handle *qmi,
    struct sockaddr_qrtr *sq,
    struct qmi_txn *txn,
    const void *decoded)
    {
    struct qcom_pdm_data *data = container_of(qmi, struct qcom_pdm_data, handle);
    const struct servreg_get_domain_list_req *req = decoded;
    struct servreg_get_domain_list_resp *rsp;
    struct qcom_pdm_service *service;
    u32 offset;
    int ret;
    rsp = kzalloc_obj(*rsp);
    if (!rsp)
    return;
    offset = req.domain_offset_valid ? req.domain_offset : 0;
    rsp.resp.result = QMI_RESULT_SUCCESS_V01;
    rsp.resp.error = QMI_ERR_NONE_V01;
    rsp.db_rev_count_valid = true;
    rsp.db_rev_count = 1;
    rsp.total_domains_valid = true;
    rsp.total_domains = 0;
    mutex_lock(&qcom_pdm_mutex);
    service = qcom_pdm_find(data, req.service_name);
    if (service) {
    struct qcom_pdm_domain *domain;
    rsp.domain_list_valid = true;
    rsp.domain_list_len = 0;
    list_for_each_entry(domain, &service.domains, list) {
    let mut i: u32 = rsp.total_domains++;
    if (i >= offset && i < SERVREG_DOMAIN_LIST_LENGTH) {
    let mut j: u32 = rsp.domain_list_len++;
    strscpy(rsp.domain_list[j].name, domain.name,
    sizeof(rsp.domain_list[i].name));
    rsp.domain_list[j].instance = domain.instance_id;
    pr_debug("PDM: found %s / %d\n", domain.name,
    domain.instance_id);
    }
    }
    }
    pr_debug("PDM: service '%s' offset %d returning %d domains (of %d)\n", req.service_name,
    req.domain_offset_valid ? req.domain_offset : -1, rsp.domain_list_len, rsp.total_domains);
    ret = qmi_send_response(qmi, sq, txn, SERVREG_GET_DOMAIN_LIST_REQ,
    SERVREG_GET_DOMAIN_LIST_RESP_MAX_LEN,
    servreg_get_domain_list_resp_ei, rsp);
    if (ret)
    pr_err("Error sending servreg response: %d\n", ret);
    mutex_unlock(&qcom_pdm_mutex);
    kfree(rsp);
    }
    static void qcom_pdm_pfr(struct qmi_handle *qmi,
    struct sockaddr_qrtr *sq,
    struct qmi_txn *txn,
    const void *decoded)
    {
    const struct servreg_loc_pfr_req *req = decoded;
    let mut rsp: servreg_loc_pfr_resp = {};
    int ret;
    pr_warn_ratelimited("PDM: service '%s' crash: '%s'\n", req.service, req.reason);
    rsp.rsp.result = QMI_RESULT_SUCCESS_V01;
    rsp.rsp.error = QMI_ERR_NONE_V01;
    ret = qmi_send_response(qmi, sq, txn, SERVREG_LOC_PFR_REQ,
    SERVREG_LOC_PFR_RESP_MAX_LEN,
    servreg_loc_pfr_resp_ei, &rsp);
    if (ret)
    pr_err("Error sending servreg response: %d\n", ret);
    }
    static const struct qmi_msg_handler qcom_pdm_msg_handlers[] = {
    {
    .type = QMI_REQUEST,
    .msg_id = SERVREG_GET_DOMAIN_LIST_REQ,
    .ei = servreg_get_domain_list_req_ei,
    .decoded_size = sizeof(struct servreg_get_domain_list_req),
    .fn = qcom_pdm_get_domain_list,
    },
    {
    .type = QMI_REQUEST,
    .msg_id = SERVREG_LOC_PFR_REQ,
    .ei = servreg_loc_pfr_req_ei,
    .decoded_size = sizeof(struct servreg_loc_pfr_req),
    .fn = qcom_pdm_pfr,
    },
    { },
    };
    static const struct qcom_pdm_domain_data adsp_audio_pd = {
    .domain = "msm/adsp/audio_pd",
    .instance_id = 74,
    .services = {
    "avs/audio",
    core::ptr::null_mut(),
    },
    };
    static const struct qcom_pdm_domain_data adsp_charger_pd = {
    .domain = "msm/adsp/charger_pd",
    .instance_id = 74,
    .services = { core::ptr::null_mut() },
    };
    static const struct qcom_pdm_domain_data adsp_ois_pd = {
    .domain = "msm/adsp/ois_pd",
    .instance_id = 74,
    .services = { core::ptr::null_mut(), },
    };
    static const struct qcom_pdm_domain_data adsp_root_pd = {
    .domain = "msm/adsp/root_pd",
    .instance_id = 74,
    .services = { core::ptr::null_mut() },
    };
    static const struct qcom_pdm_domain_data adsp_root_pd_pdr = {
    .domain = "msm/adsp/root_pd",
    .instance_id = 74,
    .services = {
    "tms/pdr_enabled",
    core::ptr::null_mut(),
    },
    };
    static const struct qcom_pdm_domain_data adsp_sensor_pd = {
    .domain = "msm/adsp/sensor_pd",
    .instance_id = 74,
    .services = { core::ptr::null_mut() },
    };
    static const struct qcom_pdm_domain_data msm8996_adsp_audio_pd = {
    .domain = "msm/adsp/audio_pd",
    .instance_id = 4,
    .services = { core::ptr::null_mut() },
    };
    static const struct qcom_pdm_domain_data msm8996_adsp_root_pd = {
    .domain = "msm/adsp/root_pd",
    .instance_id = 4,
    .services = { core::ptr::null_mut() },
    };
    static const struct qcom_pdm_domain_data cdsp_root_pd = {
    .domain = "msm/cdsp/root_pd",
    .instance_id = 76,
    .services = { core::ptr::null_mut() },
    };
    static const struct qcom_pdm_domain_data cdsp1_root_pd = {
    .domain = "msm/cdsp1/root_pd",
    .instance_id = 125,
    .services = { core::ptr::null_mut() },
    };
    static const struct qcom_pdm_domain_data gpdsp_root_pd = {
    .domain = "msm/gpdsp/root_pd",
    .instance_id = 192,
    .services = { core::ptr::null_mut() },
    };
    static const struct qcom_pdm_domain_data gpdsp1_root_pd = {
    .domain = "msm/gpdsp1/root_pd",
    .instance_id = 241,
    .services = { core::ptr::null_mut() },
    };
    static const struct qcom_pdm_domain_data slpi_root_pd = {
    .domain = "msm/slpi/root_pd",
    .instance_id = 90,
    .services = { core::ptr::null_mut() },
    };
    static const struct qcom_pdm_domain_data slpi_sensor_pd = {
    .domain = "msm/slpi/sensor_pd",
    .instance_id = 90,
    .services = { core::ptr::null_mut() },
    };
    static const struct qcom_pdm_domain_data mpss_root_pd = {
    .domain = "msm/modem/root_pd",
    .instance_id = 180,
    .services = {
    core::ptr::null_mut(),
    },
    };
    static const struct qcom_pdm_domain_data mpss_root_pd_gps = {
    .domain = "msm/modem/root_pd",
    .instance_id = 180,
    .services = {
    "gps/gps_service",
    core::ptr::null_mut(),
    },
    };
    static const struct qcom_pdm_domain_data mpss_root_pd_gps_pdr = {
    .domain = "msm/modem/root_pd",
    .instance_id = 180,
    .services = {
    "gps/gps_service",
    "tms/pdr_enabled",
    core::ptr::null_mut(),
    },
    };
    static const struct qcom_pdm_domain_data msm8996_mpss_root_pd = {
    .domain = "msm/modem/root_pd",
    .instance_id = 100,
    .services = { core::ptr::null_mut() },
    };
    static const struct qcom_pdm_domain_data mpss_wlan_pd = {
    .domain = "msm/modem/wlan_pd",
    .instance_id = 180,
    .services = {
    "kernel/elf_loader",
    "wlan/fw",
    core::ptr::null_mut(),
    },
    };
    static const struct qcom_pdm_domain_data *glymur_domains[] = {
    &adsp_audio_pd,
    &adsp_root_pd,
    &adsp_sensor_pd,
    &cdsp_root_pd,
    core::ptr::null_mut(),
    };
    static const struct qcom_pdm_domain_data *kaanapali_domains[] = {
    &adsp_audio_pd,
    &adsp_ois_pd,
    &adsp_root_pd,
    &adsp_sensor_pd,
    &cdsp_root_pd,
    &mpss_root_pd_gps,
    core::ptr::null_mut(),
    };
    static const struct qcom_pdm_domain_data *msm8996_domains[] = {
    &msm8996_adsp_audio_pd,
    &msm8996_adsp_root_pd,
    &msm8996_mpss_root_pd,
    core::ptr::null_mut(),
    };
    static const struct qcom_pdm_domain_data *msm8998_domains[] = {
    &mpss_root_pd,
    &mpss_wlan_pd,
    core::ptr::null_mut(),
    };
    static const struct qcom_pdm_domain_data *qcm2290_domains[] = {
    &adsp_audio_pd,
    &adsp_root_pd,
    &adsp_sensor_pd,
    &mpss_root_pd_gps,
    &mpss_wlan_pd,
    core::ptr::null_mut(),
    };
    static const struct qcom_pdm_domain_data *qcs404_domains[] = {
    &adsp_audio_pd,
    &adsp_root_pd,
    &adsp_sensor_pd,
    &cdsp_root_pd,
    &mpss_root_pd,
    &mpss_wlan_pd,
    core::ptr::null_mut(),
    };
    static const struct qcom_pdm_domain_data *qcs615_domains[] = {
    &adsp_audio_pd,
    &adsp_root_pd,
    &adsp_sensor_pd,
    &cdsp_root_pd,
    &mpss_root_pd,
    &mpss_wlan_pd,
    core::ptr::null_mut(),
    };
    static const struct qcom_pdm_domain_data *qcs8300_domains[] = {
    &adsp_audio_pd,
    &adsp_root_pd,
    &cdsp_root_pd,
    &gpdsp_root_pd,
    core::ptr::null_mut(),
    };
    static const struct qcom_pdm_domain_data *sa8775p_domains[] = {
    &adsp_audio_pd,
    &adsp_root_pd,
    &cdsp_root_pd,
    &cdsp1_root_pd,
    &gpdsp_root_pd,
    &gpdsp1_root_pd,
    core::ptr::null_mut(),
    };
    static const struct qcom_pdm_domain_data *sc7180_domains[] = {
    &adsp_audio_pd,
    &adsp_root_pd_pdr,
    &adsp_sensor_pd,
    &mpss_root_pd_gps_pdr,
    &mpss_wlan_pd,
    core::ptr::null_mut(),
    };
    static const struct qcom_pdm_domain_data *sc7280_domains[] = {
    &adsp_audio_pd,
    &adsp_root_pd_pdr,
    &adsp_charger_pd,
    &adsp_sensor_pd,
    &cdsp_root_pd,
    &mpss_root_pd_gps_pdr,
    core::ptr::null_mut(),
    };
    static const struct qcom_pdm_domain_data *sc8180x_domains[] = {
    &adsp_audio_pd,
    &adsp_root_pd,
    &adsp_charger_pd,
    &cdsp_root_pd,
    &mpss_root_pd_gps,
    &mpss_wlan_pd,
    core::ptr::null_mut(),
    };
    static const struct qcom_pdm_domain_data *sc8280xp_domains[] = {
    &adsp_audio_pd,
    &adsp_root_pd_pdr,
    &adsp_charger_pd,
    &cdsp_root_pd,
    core::ptr::null_mut(),
    };
// Unlike SDM660, SDM630/636 lack CDSP
    static const struct qcom_pdm_domain_data *sdm630_domains[] = {
    &adsp_audio_pd,
    &adsp_root_pd,
    &adsp_sensor_pd,
    &mpss_root_pd,
    &mpss_wlan_pd,
    core::ptr::null_mut(),
    };
    static const struct qcom_pdm_domain_data *sdm660_domains[] = {
    &adsp_audio_pd,
    &adsp_root_pd,
    &adsp_sensor_pd,
    &cdsp_root_pd,
    &mpss_root_pd,
    &mpss_wlan_pd,
    core::ptr::null_mut(),
    };
    static const struct qcom_pdm_domain_data *sdm670_domains[] = {
    &adsp_audio_pd,
    &adsp_root_pd,
    &cdsp_root_pd,
    &mpss_root_pd,
    &mpss_wlan_pd,
    core::ptr::null_mut(),
    };
    static const struct qcom_pdm_domain_data *sdm845_domains[] = {
    &adsp_audio_pd,
    &adsp_root_pd,
    &cdsp_root_pd,
    &mpss_root_pd,
    &mpss_wlan_pd,
    &slpi_root_pd,
    &slpi_sensor_pd,
    core::ptr::null_mut(),
    };
    static const struct qcom_pdm_domain_data *sm6115_domains[] = {
    &adsp_audio_pd,
    &adsp_root_pd,
    &adsp_sensor_pd,
    &cdsp_root_pd,
    &mpss_root_pd_gps,
    &mpss_wlan_pd,
    core::ptr::null_mut(),
    };
    static const struct qcom_pdm_domain_data *sm6350_domains[] = {
    &adsp_audio_pd,
    &adsp_root_pd,
    &adsp_sensor_pd,
    &cdsp_root_pd,
    &mpss_wlan_pd,
    core::ptr::null_mut(),
    };
    static const struct qcom_pdm_domain_data *sm7150_domains[] = {
    &adsp_audio_pd,
    &adsp_root_pd,
    &adsp_sensor_pd,
    &cdsp_root_pd,
    &mpss_root_pd_gps,
    &mpss_wlan_pd,
    core::ptr::null_mut(),
    };
    static const struct qcom_pdm_domain_data *sm8150_domains[] = {
    &adsp_audio_pd,
    &adsp_root_pd,
    &cdsp_root_pd,
    &mpss_root_pd_gps,
    &mpss_wlan_pd,
    core::ptr::null_mut(),
    };
    static const struct qcom_pdm_domain_data *sm8250_domains[] = {
    &adsp_audio_pd,
    &adsp_root_pd,
    &cdsp_root_pd,
    &slpi_root_pd,
    &slpi_sensor_pd,
    core::ptr::null_mut(),
    };
    static const struct qcom_pdm_domain_data *sm8350_domains[] = {
    &adsp_audio_pd,
    &adsp_root_pd_pdr,
    &adsp_charger_pd,
    &cdsp_root_pd,
    &mpss_root_pd_gps,
    &slpi_root_pd,
    &slpi_sensor_pd,
    core::ptr::null_mut(),
    };
    static const struct qcom_pdm_domain_data *sm8550_domains[] = {
    &adsp_audio_pd,
    &adsp_root_pd,
    &adsp_charger_pd,
    &adsp_sensor_pd,
    &cdsp_root_pd,
    &mpss_root_pd_gps,
    core::ptr::null_mut(),
    };
    static const struct qcom_pdm_domain_data *x1e80100_domains[] = {
    &adsp_audio_pd,
    &adsp_root_pd,
    &adsp_charger_pd,
    &adsp_sensor_pd,
    &cdsp_root_pd,
    core::ptr::null_mut(),
    };
    static const struct of_device_id qcom_pdm_domains[] __maybe_unused = {
    { .compatible = "qcom,apq8016", .data = core::ptr::null_mut(), },
    { .compatible = "qcom,apq8064", .data = core::ptr::null_mut(), },
    { .compatible = "qcom,apq8074", .data = core::ptr::null_mut(), },
    { .compatible = "qcom,apq8084", .data = core::ptr::null_mut(), },
    { .compatible = "qcom,eliza", .data = sm8550_domains, },
    { .compatible = "qcom,apq8096", .data = msm8996_domains, },
    { .compatible = "qcom,glymur", .data = glymur_domains, },
    { .compatible = "qcom,hawi", .data = kaanapali_domains, },
    { .compatible = "qcom,kaanapali", .data = kaanapali_domains, },
    { .compatible = "qcom,mahua", .data = glymur_domains, },
    { .compatible = "qcom,milos", .data = sm8550_domains, },
    { .compatible = "qcom,msm8226", .data = core::ptr::null_mut(), },
    { .compatible = "qcom,msm8909", .data = core::ptr::null_mut(), },
    { .compatible = "qcom,msm8916", .data = core::ptr::null_mut(), },
    { .compatible = "qcom,msm8939", .data = core::ptr::null_mut(), },
    { .compatible = "qcom,msm8974", .data = core::ptr::null_mut(), },
    { .compatible = "qcom,msm8996", .data = msm8996_domains, },
    { .compatible = "qcom,msm8998", .data = msm8998_domains, },
    { .compatible = "qcom,qcm2290", .data = qcm2290_domains, },
    { .compatible = "qcom,qcm6490", .data = sc7280_domains, },
    { .compatible = "qcom,qcs404", .data = qcs404_domains, },
    { .compatible = "qcom,qcs615", .data = qcs615_domains, },
    { .compatible = "qcom,qcs8300", .data = qcs8300_domains, },
    { .compatible = "qcom,sa8775p", .data = sa8775p_domains, },
    { .compatible = "qcom,sc7180", .data = sc7180_domains, },
    { .compatible = "qcom,sc7280", .data = sc7280_domains, },
    { .compatible = "qcom,sc8180x", .data = sc8180x_domains, },
    { .compatible = "qcom,sc8280xp", .data = sc8280xp_domains, },
    { .compatible = "qcom,sdm630", .data = sdm630_domains, },
    { .compatible = "qcom,sdm636", .data = sdm630_domains, },
    { .compatible = "qcom,sda660", .data = sdm660_domains, },
    { .compatible = "qcom,sdm660", .data = sdm660_domains, },
    { .compatible = "qcom,sdm670", .data = sdm670_domains, },
    { .compatible = "qcom,sdm845", .data = sdm845_domains, },
    { .compatible = "qcom,sm4250", .data = sm6115_domains, },
    { .compatible = "qcom,sm6115", .data = sm6115_domains, },
    { .compatible = "qcom,sm6350", .data = sm6350_domains, },
    { .compatible = "qcom,sm7150", .data = sm7150_domains, },
    { .compatible = "qcom,sm7225", .data = sm6350_domains, },
    { .compatible = "qcom,sm7325", .data = sc7280_domains, },
    { .compatible = "qcom,sm8150", .data = sm8150_domains, },
    { .compatible = "qcom,sm8250", .data = sm8250_domains, },
    { .compatible = "qcom,sm8350", .data = sm8350_domains, },
    { .compatible = "qcom,sm8450", .data = sm8350_domains, },
    { .compatible = "qcom,sm8550", .data = sm8550_domains, },
    { .compatible = "qcom,sm8650", .data = sm8550_domains, },
    { .compatible = "qcom,sm8750", .data = sm8550_domains, },
    { .compatible = "qcom,x1e80100", .data = x1e80100_domains, },
    { .compatible = "qcom,x1p42100", .data = x1e80100_domains, },
    {},
    };
#[no_mangle]
unsafe extern "C" fn qcom_pdm_stop(data: *mut qcom_pdm_data) {
    static void qcom_pdm_stop(struct qcom_pdm_data *data)
    {
    qcom_pdm_free_domains(data);
// The server is removed automatically
    qmi_handle_release(&data.handle);
    kfree(data);
    }
    static struct qcom_pdm_data *qcom_pdm_start(void)
    {
    const struct qcom_pdm_domain_data * const *domains;
    const struct of_device_id *match;
    struct qcom_pdm_data *data;
    int ret, i;
    match = of_machine_get_match(qcom_pdm_domains);
    if (!match) {
    pr_notice("PDM: no support for the platform, userspace daemon might be required.\n");
    return ERR_PTR(-ENODEV);
    }
    domains = match.data;
    if (!domains) {
    pr_debug("PDM: no domains\n");
    return ERR_PTR(-ENODEV);
    }
    data = kzalloc_obj(*data);
    if (!data)
    return ERR_PTR(-ENOMEM);
    INIT_LIST_HEAD(&data.services);
    ret = qmi_handle_init(&data.handle, SERVREG_GET_DOMAIN_LIST_REQ_MAX_LEN,
    core::ptr::null_mut(), qcom_pdm_msg_handlers);
    if (ret) {
    kfree(data);
    return ERR_PTR(ret);
    }
    refcount_set(&data.refcnt, 1);
    for (i = 0; domains[i]; i++) {
    ret = qcom_pdm_add_domain(data, domains[i]);
    if (ret)
    goto err_stop;
    }
    ret = qmi_add_server(&data.handle, QMI_SERVICE_ID_SERVREG_LOC,
    SERVREG_QMI_VERSION, SERVREG_QMI_INSTANCE);
    if (ret) {
    pr_err("PDM: error adding server %d\n", ret);
    goto err_stop;
    }
    return data;
    err_stop:
    qcom_pdm_stop(data);
    return ERR_PTR(ret);
    }
    static int qcom_pdm_probe(struct auxiliary_device *auxdev,
    const struct auxiliary_device_id *id)
    {
    struct qcom_pdm_data *data;
    let mut ret: c_int = 0;
    mutex_lock(&qcom_pdm_mutex);
    if (!__qcom_pdm_data) {
    data = qcom_pdm_start();
    if (IS_ERR(data))
    ret = PTR_ERR(data);
    else
    __qcom_pdm_data = data;
    } else {
    refcount_inc(&__qcom_pdm_data.refcnt);
    }
    auxiliary_set_drvdata(auxdev, __qcom_pdm_data);
    mutex_unlock(&qcom_pdm_mutex);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn qcom_pdm_remove(auxdev: *mut auxiliary_device) {
    static void qcom_pdm_remove(struct auxiliary_device *auxdev)
    {
    struct qcom_pdm_data *data;
    data = auxiliary_get_drvdata(auxdev);
    if (!data)
    return;
    if (refcount_dec_and_mutex_lock(&data.refcnt, &qcom_pdm_mutex)) {
    __qcom_pdm_data = core::ptr::null_mut();
    qcom_pdm_stop(data);
    mutex_unlock(&qcom_pdm_mutex);
    }
    }
    static const struct auxiliary_device_id qcom_pdm_table[] = {
    { .name = "qcom_common.pd-mapper" },
    {},
    };
    MODULE_DEVICE_TABLE(auxiliary, qcom_pdm_table);
    static struct auxiliary_driver qcom_pdm_drv = {
    .name = "qcom-pdm-mapper",
    .id_table = qcom_pdm_table,
    .probe = qcom_pdm_probe,
    .remove = qcom_pdm_remove,
    };
    module_auxiliary_driver(qcom_pdm_drv);
    MODULE_DESCRIPTION("Qualcomm Protection Domain Mapper");
    MODULE_LICENSE("GPL");
