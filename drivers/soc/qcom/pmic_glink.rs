//! Automatically rewritten from C to Rust
//! Source: drivers/soc/qcom/pmic_glink.c
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
// Copyright (c) 2019-2020, The Linux Foundation. All rights reserved.
// Copyright (c) 2022, Linaro Ltd
//

    enum {
    PMIC_GLINK_CLIENT_BATT = 0,
    PMIC_GLINK_CLIENT_ALTMODE,
    PMIC_GLINK_CLIENT_UCSI,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pmic_glink_data {
    pub client_mask: c_ulong,
    pub charger_pdr_service_name: *const c_char,
    pub charger_pdr_service_path: *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pmic_glink {
    pub dev: *mut device,
    pub pdr: *mut pdr_handle,
    pub ept: *mut rpmsg_endpoint,
    pub data: *const pmic_glink_data,
    pub altmode_aux: auxiliary_device,
    pub ps_aux: auxiliary_device,
    pub ucsi_aux: auxiliary_device,
// serializing client_state and pdr_state updates
    pub state_lock: mutex,
    pub client_state: c_uint,
    pub pdr_state: c_uint,
    pub pdr_available: bool,
// serializing clients list updates
    pub client_lock: spinlock_t,
    pub clients: list_head,
}

    static struct pmic_glink *__pmic_glink;
    static DEFINE_MUTEX(__pmic_glink_lock);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pmic_glink_client {
    pub node: list_head,
    pub pg: *mut pmic_glink,
    pub id: c_uint,
    pub priv): *const *const *const void (cb)(void data, size_t len, void,
    pub state): *mut *mut *mut void (pdr_notify)(void priv, int,
    pub priv: *mut c_void,
}

#[no_mangle]
unsafe extern "C" fn _devm_pmic_glink_release_client(dev: *mut device, res: *mut c_void) {
    static void _devm_pmic_glink_release_client(struct device *dev, void *res)
    {
    struct pmic_glink_client *client = (struct pmic_glink_client *)res;
    struct pmic_glink *pg = client.pg;
    unsigned long flags;
    spin_lock_irqsave(&pg.client_lock, flags);
    list_del(&client.node);
    spin_unlock_irqrestore(&pg.client_lock, flags);
    }
    struct pmic_glink_client *devm_pmic_glink_client_alloc(struct device *dev,
    unsigned int id,
    void (*cb)(const void *, size_t, void *),
    void (*pdr)(void *, int),
    void *priv)
    {
    struct pmic_glink_client *client;
    struct pmic_glink *pg = dev_get_drvdata(dev.parent);
    client = devres_alloc(_devm_pmic_glink_release_client, sizeof(*client), GFP_KERNEL);
    if (!client)
    return ERR_PTR(-ENOMEM);
    client.pg = pg;
    client.id = id;
    client.cb = cb;
    client.pdr_notify = pdr;
    client.priv = priv;
    INIT_LIST_HEAD(&client.node);
    devres_add(dev, client);
    return client;
    }
    EXPORT_SYMBOL_GPL(devm_pmic_glink_client_alloc);
#[no_mangle]
pub unsafe extern "C" fn pmic_glink_client_register(client: *mut pmic_glink_client) {
    void pmic_glink_client_register(struct pmic_glink_client *client)
    {
    struct pmic_glink *pg = client.pg;
    unsigned long flags;
    guard(mutex)(&pg.state_lock);
    spin_lock_irqsave(&pg.client_lock, flags);
    list_add(&client.node, &pg.clients);
    client.pdr_notify(client.priv, pg.client_state);
    spin_unlock_irqrestore(&pg.client_lock, flags);
    }
    EXPORT_SYMBOL_GPL(pmic_glink_client_register);
#[no_mangle]
pub unsafe extern "C" fn pmic_glink_send(client: *mut pmic_glink_client, data: *mut c_void, len: usize) -> c_int {
    int pmic_glink_send(struct pmic_glink_client *client, void *data, size_t len)
    {
    struct pmic_glink *pg = client.pg;
    let mut timeout_reached: bool = false;
    unsigned long start;
    int ret;
    guard(mutex)(&pg.state_lock);
    if (!pg.ept) {
    return -ECONNRESET;
    }
    start = jiffies;
    for (;;) {
    ret = rpmsg_send(pg.ept, data, len);
    if (ret != -EAGAIN)
    break;
    if (timeout_reached) {
    ret = -ETIMEDOUT;
    break;
    }
    usleep_range(1000, 5000);
    timeout_reached = time_after(jiffies, start + PMIC_GLINK_SEND_TIMEOUT);
    }
    return ret;
    }
    EXPORT_SYMBOL_GPL(pmic_glink_send);
    static int pmic_glink_rpmsg_callback(struct rpmsg_device *rpdev, void *data,
    int len, void *priv, u32 addr)
    {
    struct pmic_glink_client *client;
    struct pmic_glink_hdr *hdr;
    struct pmic_glink *pg = dev_get_drvdata(&rpdev.dev);
    unsigned long flags;
    if (len < sizeof(*hdr)) {
    dev_warn(pg.dev, "ignoring truncated message\n");
    return 0;
    }
    hdr = data;
    spin_lock_irqsave(&pg.client_lock, flags);
    list_for_each_entry(client, &pg.clients, node) {
    if (client.id == le32_to_cpu(hdr.owner))
    client.cb(data, len, client.priv);
    }
    spin_unlock_irqrestore(&pg.client_lock, flags);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pmic_glink_aux_release(dev: *mut device) {
    static void pmic_glink_aux_release(struct device *dev)
    {
    of_node_put(dev.of_node);
    }
    static int pmic_glink_add_aux_device(struct pmic_glink *pg,
    struct auxiliary_device *aux,
    const char *name)
    {
    struct device *parent = pg.dev;
    int ret;
    aux.name = name;
    aux.dev.parent = parent;
    aux.dev.release = pmic_glink_aux_release;
    device_set_of_node_from_dev(&aux.dev, parent);
    ret = auxiliary_device_init(aux);
    if (ret) {
    of_node_put(aux.dev.of_node);
    return ret;
    }
    ret = auxiliary_device_add(aux);
    if (ret)
    auxiliary_device_uninit(aux);
    return ret;
    }
    static void pmic_glink_del_aux_device(struct pmic_glink *pg,
    struct auxiliary_device *aux)
    {
    auxiliary_device_delete(aux);
    auxiliary_device_uninit(aux);
    }
#[no_mangle]
unsafe extern "C" fn pmic_glink_state_notify_clients(pg: *mut pmic_glink) {
    static void pmic_glink_state_notify_clients(struct pmic_glink *pg)
    {
    struct pmic_glink_client *client;
    let mut new_state: c_uint = pg.client_state;
    unsigned long flags;
    if (pg.client_state != SERVREG_SERVICE_STATE_UP) {
    if (pg.pdr_state == SERVREG_SERVICE_STATE_UP && pg.ept)
    new_state = SERVREG_SERVICE_STATE_UP;
    } else {
    if (pg.pdr_state == SERVREG_SERVICE_STATE_DOWN || !pg.ept)
    new_state = SERVREG_SERVICE_STATE_DOWN;
    }
    if (new_state != pg.client_state) {
    spin_lock_irqsave(&pg.client_lock, flags);
    list_for_each_entry(client, &pg.clients, node)
    client.pdr_notify(client.priv, new_state);
    spin_unlock_irqrestore(&pg.client_lock, flags);
    pg.client_state = new_state;
    }
    }
#[no_mangle]
unsafe extern "C" fn pmic_glink_pdr_callback(state: c_int, svc_path: *mut c_char, priv: *mut c_void) {
    static void pmic_glink_pdr_callback(int state, char *svc_path, void *priv)
    {
    struct pmic_glink *pg = priv;
    guard(mutex)(&pg.state_lock);
    pg.pdr_state = state;
    pmic_glink_state_notify_clients(pg);
    }
#[no_mangle]
unsafe extern "C" fn pmic_glink_rpmsg_probe(rpdev: *mut rpmsg_device) -> c_int {
    static int pmic_glink_rpmsg_probe(struct rpmsg_device *rpdev)
    {
    struct pmic_glink *pg;
    guard(mutex)(&__pmic_glink_lock);
    pg = __pmic_glink;
    if (!pg)
    return dev_err_probe(&rpdev.dev, -ENODEV, "no pmic_glink device to attach to\n");
    dev_set_drvdata(&rpdev.dev, pg);
    pg.pdr_available = rpdev.id.driver_data;
    guard(mutex)(&pg.state_lock);
    pg.ept = rpdev.ept;
    if (!pg.pdr_available)
    pg.pdr_state = SERVREG_SERVICE_STATE_UP;
    pmic_glink_state_notify_clients(pg);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pmic_glink_rpmsg_remove(rpdev: *mut rpmsg_device) {
    static void pmic_glink_rpmsg_remove(struct rpmsg_device *rpdev)
    {
    struct pmic_glink *pg;
    guard(mutex)(&__pmic_glink_lock);
    pg = __pmic_glink;
    if (!pg)
    return;
    guard(mutex)(&pg.state_lock);
    pg.ept = core::ptr::null_mut();
    if (!pg.pdr_available)
    pg.pdr_state = SERVREG_SERVICE_STATE_DOWN;
    pmic_glink_state_notify_clients(pg);
    }
    static const struct rpmsg_device_id pmic_glink_rpmsg_id_match[] = {
    {.name = "PMIC_RTR_ADSP_APPS", .driver_data = true },
    {.name = "PMIC_RTR_SOCCP_APPS", .driver_data = false },
    {}
    };
    static struct rpmsg_driver pmic_glink_rpmsg_driver = {
    .probe = pmic_glink_rpmsg_probe,
    .remove = pmic_glink_rpmsg_remove,
    .callback = pmic_glink_rpmsg_callback,
    .id_table = pmic_glink_rpmsg_id_match,
    .drv  = {
    .name  = "qcom_pmic_glink_rpmsg",
    },
    };
#[no_mangle]
unsafe extern "C" fn pmic_glink_probe(pdev: *mut platform_device) -> c_int {
    static int pmic_glink_probe(struct platform_device *pdev)
    {
    struct pdr_service *service;
    struct pmic_glink *pg;
    int ret;
    pg = devm_kzalloc(&pdev.dev, sizeof(*pg), GFP_KERNEL);
    if (!pg)
    return -ENOMEM;
    dev_set_drvdata(&pdev.dev, pg);
    pg.dev = &pdev.dev;
    INIT_LIST_HEAD(&pg.clients);
    spin_lock_init(&pg.client_lock);
    mutex_init(&pg.state_lock);
    pg.data = of_device_get_match_data(&pdev.dev);
    if (!pg.data)
    return -EINVAL;
    pg.pdr = pdr_handle_alloc(pmic_glink_pdr_callback, pg);
    if (IS_ERR(pg.pdr)) {
    ret = dev_err_probe(&pdev.dev, PTR_ERR(pg.pdr),
    "failed to initialize pdr\n");
    return ret;
    }
    if (pg.data.client_mask & BIT(PMIC_GLINK_CLIENT_UCSI)) {
    ret = pmic_glink_add_aux_device(pg, &pg.ucsi_aux, "ucsi");
    if (ret)
    goto out_release_pdr_handle;
    }
    if (pg.data.client_mask & BIT(PMIC_GLINK_CLIENT_ALTMODE)) {
    ret = pmic_glink_add_aux_device(pg, &pg.altmode_aux, "altmode");
    if (ret)
    goto out_release_ucsi_aux;
    }
    if (pg.data.client_mask & BIT(PMIC_GLINK_CLIENT_BATT)) {
    ret = pmic_glink_add_aux_device(pg, &pg.ps_aux, "power-supply");
    if (ret)
    goto out_release_altmode_aux;
    }
    if (pg.data.charger_pdr_service_name && pg.data.charger_pdr_service_path) {
    service = pdr_add_lookup(pg.pdr, pg.data.charger_pdr_service_name,
    pg.data.charger_pdr_service_path);
    if (IS_ERR(service)) {
    ret = dev_err_probe(&pdev.dev, PTR_ERR(service),
    "failed adding pdr lookup for charger_pd\n");
    goto out_release_aux_devices;
    }
    }
    mutex_lock(&__pmic_glink_lock);
    __pmic_glink = pg;
    mutex_unlock(&__pmic_glink_lock);
    return 0;
    out_release_aux_devices:
    if (pg.data.client_mask & BIT(PMIC_GLINK_CLIENT_BATT))
    pmic_glink_del_aux_device(pg, &pg.ps_aux);
    out_release_altmode_aux:
    if (pg.data.client_mask & BIT(PMIC_GLINK_CLIENT_ALTMODE))
    pmic_glink_del_aux_device(pg, &pg.altmode_aux);
    out_release_ucsi_aux:
    if (pg.data.client_mask & BIT(PMIC_GLINK_CLIENT_UCSI))
    pmic_glink_del_aux_device(pg, &pg.ucsi_aux);
    out_release_pdr_handle:
    pdr_handle_release(pg.pdr);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn pmic_glink_remove(pdev: *mut platform_device) {
    static void pmic_glink_remove(struct platform_device *pdev)
    {
    struct pmic_glink *pg = dev_get_drvdata(&pdev.dev);
    pdr_handle_release(pg.pdr);
    if (pg.data.client_mask & BIT(PMIC_GLINK_CLIENT_BATT))
    pmic_glink_del_aux_device(pg, &pg.ps_aux);
    if (pg.data.client_mask & BIT(PMIC_GLINK_CLIENT_ALTMODE))
    pmic_glink_del_aux_device(pg, &pg.altmode_aux);
    if (pg.data.client_mask & BIT(PMIC_GLINK_CLIENT_UCSI))
    pmic_glink_del_aux_device(pg, &pg.ucsi_aux);
    guard(mutex)(&__pmic_glink_lock);
    __pmic_glink = core::ptr::null_mut();
    }
    static const struct pmic_glink_data pmic_glink_adsp_data = {
    .client_mask = BIT(PMIC_GLINK_CLIENT_BATT) |
    BIT(PMIC_GLINK_CLIENT_ALTMODE) |
    BIT(PMIC_GLINK_CLIENT_UCSI),
    .charger_pdr_service_name = "tms/servreg",
    .charger_pdr_service_path = "msm/adsp/charger_pd",
    };
    static const struct pmic_glink_data pmic_glink_soccp_data = {
    .client_mask = BIT(PMIC_GLINK_CLIENT_BATT) |
    BIT(PMIC_GLINK_CLIENT_ALTMODE) |
    BIT(PMIC_GLINK_CLIENT_UCSI),
    };
    static const struct of_device_id pmic_glink_of_match[] = {
    { .compatible = "qcom,glymur-pmic-glink", .data = &pmic_glink_soccp_data },
    { .compatible = "qcom,kaanapali-pmic-glink", .data = &pmic_glink_soccp_data },
    { .compatible = "qcom,pmic-glink", .data = &pmic_glink_adsp_data },
    {}
    };
    MODULE_DEVICE_TABLE(of, pmic_glink_of_match);
    static struct platform_driver pmic_glink_driver = {
    .probe = pmic_glink_probe,
    .remove = pmic_glink_remove,
    .driver = {
    .name = "qcom_pmic_glink",
    .of_match_table = pmic_glink_of_match,
    },
    };
#[no_mangle]
unsafe extern "C" fn pmic_glink_init() -> c_int {
    static int pmic_glink_init(void)
    {
    int ret;
    ret = platform_driver_register(&pmic_glink_driver);
    if (ret < 0)
    return ret;
    ret = register_rpmsg_driver(&pmic_glink_rpmsg_driver);
    if (ret < 0) {
    platform_driver_unregister(&pmic_glink_driver);
    return ret;
    }
    return 0;
    }
    module_init(pmic_glink_init);
#[no_mangle]
unsafe extern "C" fn pmic_glink_exit() {
    static void pmic_glink_exit(void)
    {
    unregister_rpmsg_driver(&pmic_glink_rpmsg_driver);
    platform_driver_unregister(&pmic_glink_driver);
    }
    module_exit(pmic_glink_exit);
    MODULE_DESCRIPTION("Qualcomm PMIC GLINK driver");
    MODULE_LICENSE("GPL");
