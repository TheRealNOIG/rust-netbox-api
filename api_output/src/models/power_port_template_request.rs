/*
 * NetBox REST API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 3.7.1 (3.7)
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PowerPortTemplateRequest : Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PowerPortTemplateRequest {
    #[serde(rename = "device_type", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub device_type: Option<Option<Box<crate::models::NestedDeviceTypeRequest>>>,
    #[serde(rename = "module_type", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub module_type: Option<Option<Box<crate::models::NestedModuleTypeRequest>>>,
    /// {module} is accepted as a substitution for the module bay position when attached to a module type.
    #[serde(rename = "name")]
    pub name: String,
    /// Physical label
    #[serde(rename = "label", skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// * `iec-60320-c6` - C6 * `iec-60320-c8` - C8 * `iec-60320-c14` - C14 * `iec-60320-c16` - C16 * `iec-60320-c20` - C20 * `iec-60320-c22` - C22 * `iec-60309-p-n-e-4h` - P+N+E 4H * `iec-60309-p-n-e-6h` - P+N+E 6H * `iec-60309-p-n-e-9h` - P+N+E 9H * `iec-60309-2p-e-4h` - 2P+E 4H * `iec-60309-2p-e-6h` - 2P+E 6H * `iec-60309-2p-e-9h` - 2P+E 9H * `iec-60309-3p-e-4h` - 3P+E 4H * `iec-60309-3p-e-6h` - 3P+E 6H * `iec-60309-3p-e-9h` - 3P+E 9H * `iec-60309-3p-n-e-4h` - 3P+N+E 4H * `iec-60309-3p-n-e-6h` - 3P+N+E 6H * `iec-60309-3p-n-e-9h` - 3P+N+E 9H * `iec-60906-1` - IEC 60906-1 * `nbr-14136-10a` - 2P+T 10A (NBR 14136) * `nbr-14136-20a` - 2P+T 20A (NBR 14136) * `nema-1-15p` - NEMA 1-15P * `nema-5-15p` - NEMA 5-15P * `nema-5-20p` - NEMA 5-20P * `nema-5-30p` - NEMA 5-30P * `nema-5-50p` - NEMA 5-50P * `nema-6-15p` - NEMA 6-15P * `nema-6-20p` - NEMA 6-20P * `nema-6-30p` - NEMA 6-30P * `nema-6-50p` - NEMA 6-50P * `nema-10-30p` - NEMA 10-30P * `nema-10-50p` - NEMA 10-50P * `nema-14-20p` - NEMA 14-20P * `nema-14-30p` - NEMA 14-30P * `nema-14-50p` - NEMA 14-50P * `nema-14-60p` - NEMA 14-60P * `nema-15-15p` - NEMA 15-15P * `nema-15-20p` - NEMA 15-20P * `nema-15-30p` - NEMA 15-30P * `nema-15-50p` - NEMA 15-50P * `nema-15-60p` - NEMA 15-60P * `nema-l1-15p` - NEMA L1-15P * `nema-l5-15p` - NEMA L5-15P * `nema-l5-20p` - NEMA L5-20P * `nema-l5-30p` - NEMA L5-30P * `nema-l5-50p` - NEMA L5-50P * `nema-l6-15p` - NEMA L6-15P * `nema-l6-20p` - NEMA L6-20P * `nema-l6-30p` - NEMA L6-30P * `nema-l6-50p` - NEMA L6-50P * `nema-l10-30p` - NEMA L10-30P * `nema-l14-20p` - NEMA L14-20P * `nema-l14-30p` - NEMA L14-30P * `nema-l14-50p` - NEMA L14-50P * `nema-l14-60p` - NEMA L14-60P * `nema-l15-20p` - NEMA L15-20P * `nema-l15-30p` - NEMA L15-30P * `nema-l15-50p` - NEMA L15-50P * `nema-l15-60p` - NEMA L15-60P * `nema-l21-20p` - NEMA L21-20P * `nema-l21-30p` - NEMA L21-30P * `nema-l22-30p` - NEMA L22-30P * `cs6361c` - CS6361C * `cs6365c` - CS6365C * `cs8165c` - CS8165C * `cs8265c` - CS8265C * `cs8365c` - CS8365C * `cs8465c` - CS8465C * `ita-c` - ITA Type C (CEE 7/16) * `ita-e` - ITA Type E (CEE 7/6) * `ita-f` - ITA Type F (CEE 7/4) * `ita-ef` - ITA Type E/F (CEE 7/7) * `ita-g` - ITA Type G (BS 1363) * `ita-h` - ITA Type H * `ita-i` - ITA Type I * `ita-j` - ITA Type J * `ita-k` - ITA Type K * `ita-l` - ITA Type L (CEI 23-50) * `ita-m` - ITA Type M (BS 546) * `ita-n` - ITA Type N * `ita-o` - ITA Type O * `usb-a` - USB Type A * `usb-b` - USB Type B * `usb-c` - USB Type C * `usb-mini-a` - USB Mini A * `usb-mini-b` - USB Mini B * `usb-micro-a` - USB Micro A * `usb-micro-b` - USB Micro B * `usb-micro-ab` - USB Micro AB * `usb-3-b` - USB 3.0 Type B * `usb-3-micro-b` - USB 3.0 Micro B * `dc-terminal` - DC Terminal * `saf-d-grid` - Saf-D-Grid * `neutrik-powercon-20` - Neutrik powerCON (20A) * `neutrik-powercon-32` - Neutrik powerCON (32A) * `neutrik-powercon-true1` - Neutrik powerCON TRUE1 * `neutrik-powercon-true1-top` - Neutrik powerCON TRUE1 TOP * `ubiquiti-smartpower` - Ubiquiti SmartPower * `hardwired` - Hardwired * `other` - Other
    #[serde(rename = "type", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Option<Type>>,
    /// Maximum power draw (watts)
    #[serde(rename = "maximum_draw", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub maximum_draw: Option<Option<i32>>,
    /// Allocated power draw (watts)
    #[serde(rename = "allocated_draw", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub allocated_draw: Option<Option<i32>>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl PowerPortTemplateRequest {
    /// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
    pub fn new(name: String) -> PowerPortTemplateRequest {
        PowerPortTemplateRequest {
            device_type: None,
            module_type: None,
            name,
            label: None,
            r#type: None,
            maximum_draw: None,
            allocated_draw: None,
            description: None,
        }
    }
}

/// * `iec-60320-c6` - C6 * `iec-60320-c8` - C8 * `iec-60320-c14` - C14 * `iec-60320-c16` - C16 * `iec-60320-c20` - C20 * `iec-60320-c22` - C22 * `iec-60309-p-n-e-4h` - P+N+E 4H * `iec-60309-p-n-e-6h` - P+N+E 6H * `iec-60309-p-n-e-9h` - P+N+E 9H * `iec-60309-2p-e-4h` - 2P+E 4H * `iec-60309-2p-e-6h` - 2P+E 6H * `iec-60309-2p-e-9h` - 2P+E 9H * `iec-60309-3p-e-4h` - 3P+E 4H * `iec-60309-3p-e-6h` - 3P+E 6H * `iec-60309-3p-e-9h` - 3P+E 9H * `iec-60309-3p-n-e-4h` - 3P+N+E 4H * `iec-60309-3p-n-e-6h` - 3P+N+E 6H * `iec-60309-3p-n-e-9h` - 3P+N+E 9H * `iec-60906-1` - IEC 60906-1 * `nbr-14136-10a` - 2P+T 10A (NBR 14136) * `nbr-14136-20a` - 2P+T 20A (NBR 14136) * `nema-1-15p` - NEMA 1-15P * `nema-5-15p` - NEMA 5-15P * `nema-5-20p` - NEMA 5-20P * `nema-5-30p` - NEMA 5-30P * `nema-5-50p` - NEMA 5-50P * `nema-6-15p` - NEMA 6-15P * `nema-6-20p` - NEMA 6-20P * `nema-6-30p` - NEMA 6-30P * `nema-6-50p` - NEMA 6-50P * `nema-10-30p` - NEMA 10-30P * `nema-10-50p` - NEMA 10-50P * `nema-14-20p` - NEMA 14-20P * `nema-14-30p` - NEMA 14-30P * `nema-14-50p` - NEMA 14-50P * `nema-14-60p` - NEMA 14-60P * `nema-15-15p` - NEMA 15-15P * `nema-15-20p` - NEMA 15-20P * `nema-15-30p` - NEMA 15-30P * `nema-15-50p` - NEMA 15-50P * `nema-15-60p` - NEMA 15-60P * `nema-l1-15p` - NEMA L1-15P * `nema-l5-15p` - NEMA L5-15P * `nema-l5-20p` - NEMA L5-20P * `nema-l5-30p` - NEMA L5-30P * `nema-l5-50p` - NEMA L5-50P * `nema-l6-15p` - NEMA L6-15P * `nema-l6-20p` - NEMA L6-20P * `nema-l6-30p` - NEMA L6-30P * `nema-l6-50p` - NEMA L6-50P * `nema-l10-30p` - NEMA L10-30P * `nema-l14-20p` - NEMA L14-20P * `nema-l14-30p` - NEMA L14-30P * `nema-l14-50p` - NEMA L14-50P * `nema-l14-60p` - NEMA L14-60P * `nema-l15-20p` - NEMA L15-20P * `nema-l15-30p` - NEMA L15-30P * `nema-l15-50p` - NEMA L15-50P * `nema-l15-60p` - NEMA L15-60P * `nema-l21-20p` - NEMA L21-20P * `nema-l21-30p` - NEMA L21-30P * `nema-l22-30p` - NEMA L22-30P * `cs6361c` - CS6361C * `cs6365c` - CS6365C * `cs8165c` - CS8165C * `cs8265c` - CS8265C * `cs8365c` - CS8365C * `cs8465c` - CS8465C * `ita-c` - ITA Type C (CEE 7/16) * `ita-e` - ITA Type E (CEE 7/6) * `ita-f` - ITA Type F (CEE 7/4) * `ita-ef` - ITA Type E/F (CEE 7/7) * `ita-g` - ITA Type G (BS 1363) * `ita-h` - ITA Type H * `ita-i` - ITA Type I * `ita-j` - ITA Type J * `ita-k` - ITA Type K * `ita-l` - ITA Type L (CEI 23-50) * `ita-m` - ITA Type M (BS 546) * `ita-n` - ITA Type N * `ita-o` - ITA Type O * `usb-a` - USB Type A * `usb-b` - USB Type B * `usb-c` - USB Type C * `usb-mini-a` - USB Mini A * `usb-mini-b` - USB Mini B * `usb-micro-a` - USB Micro A * `usb-micro-b` - USB Micro B * `usb-micro-ab` - USB Micro AB * `usb-3-b` - USB 3.0 Type B * `usb-3-micro-b` - USB 3.0 Micro B * `dc-terminal` - DC Terminal * `saf-d-grid` - Saf-D-Grid * `neutrik-powercon-20` - Neutrik powerCON (20A) * `neutrik-powercon-32` - Neutrik powerCON (32A) * `neutrik-powercon-true1` - Neutrik powerCON TRUE1 * `neutrik-powercon-true1-top` - Neutrik powerCON TRUE1 TOP * `ubiquiti-smartpower` - Ubiquiti SmartPower * `hardwired` - Hardwired * `other` - Other
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "iec-60320-c6")]
    Iec60320C6,
    #[serde(rename = "iec-60320-c8")]
    Iec60320C8,
    #[serde(rename = "iec-60320-c14")]
    Iec60320C14,
    #[serde(rename = "iec-60320-c16")]
    Iec60320C16,
    #[serde(rename = "iec-60320-c20")]
    Iec60320C20,
    #[serde(rename = "iec-60320-c22")]
    Iec60320C22,
    #[serde(rename = "iec-60309-p-n-e-4h")]
    Iec60309PNE4h,
    #[serde(rename = "iec-60309-p-n-e-6h")]
    Iec60309PNE6h,
    #[serde(rename = "iec-60309-p-n-e-9h")]
    Iec60309PNE9h,
    #[serde(rename = "iec-60309-2p-e-4h")]
    Iec603092pE4h,
    #[serde(rename = "iec-60309-2p-e-6h")]
    Iec603092pE6h,
    #[serde(rename = "iec-60309-2p-e-9h")]
    Iec603092pE9h,
    #[serde(rename = "iec-60309-3p-e-4h")]
    Iec603093pE4h,
    #[serde(rename = "iec-60309-3p-e-6h")]
    Iec603093pE6h,
    #[serde(rename = "iec-60309-3p-e-9h")]
    Iec603093pE9h,
    #[serde(rename = "iec-60309-3p-n-e-4h")]
    Iec603093pNE4h,
    #[serde(rename = "iec-60309-3p-n-e-6h")]
    Iec603093pNE6h,
    #[serde(rename = "iec-60309-3p-n-e-9h")]
    Iec603093pNE9h,
    #[serde(rename = "iec-60906-1")]
    Iec609061,
    #[serde(rename = "nbr-14136-10a")]
    Nbr1413610a,
    #[serde(rename = "nbr-14136-20a")]
    Nbr1413620a,
    #[serde(rename = "nema-1-15p")]
    Nema115p,
    #[serde(rename = "nema-5-15p")]
    Nema515p,
    #[serde(rename = "nema-5-20p")]
    Nema520p,
    #[serde(rename = "nema-5-30p")]
    Nema530p,
    #[serde(rename = "nema-5-50p")]
    Nema550p,
    #[serde(rename = "nema-6-15p")]
    Nema615p,
    #[serde(rename = "nema-6-20p")]
    Nema620p,
    #[serde(rename = "nema-6-30p")]
    Nema630p,
    #[serde(rename = "nema-6-50p")]
    Nema650p,
    #[serde(rename = "nema-10-30p")]
    Nema1030p,
    #[serde(rename = "nema-10-50p")]
    Nema1050p,
    #[serde(rename = "nema-14-20p")]
    Nema1420p,
    #[serde(rename = "nema-14-30p")]
    Nema1430p,
    #[serde(rename = "nema-14-50p")]
    Nema1450p,
    #[serde(rename = "nema-14-60p")]
    Nema1460p,
    #[serde(rename = "nema-15-15p")]
    Nema1515p,
    #[serde(rename = "nema-15-20p")]
    Nema1520p,
    #[serde(rename = "nema-15-30p")]
    Nema1530p,
    #[serde(rename = "nema-15-50p")]
    Nema1550p,
    #[serde(rename = "nema-15-60p")]
    Nema1560p,
    #[serde(rename = "nema-l1-15p")]
    NemaL115p,
    #[serde(rename = "nema-l5-15p")]
    NemaL515p,
    #[serde(rename = "nema-l5-20p")]
    NemaL520p,
    #[serde(rename = "nema-l5-30p")]
    NemaL530p,
    #[serde(rename = "nema-l5-50p")]
    NemaL550p,
    #[serde(rename = "nema-l6-15p")]
    NemaL615p,
    #[serde(rename = "nema-l6-20p")]
    NemaL620p,
    #[serde(rename = "nema-l6-30p")]
    NemaL630p,
    #[serde(rename = "nema-l6-50p")]
    NemaL650p,
    #[serde(rename = "nema-l10-30p")]
    NemaL1030p,
    #[serde(rename = "nema-l14-20p")]
    NemaL1420p,
    #[serde(rename = "nema-l14-30p")]
    NemaL1430p,
    #[serde(rename = "nema-l14-50p")]
    NemaL1450p,
    #[serde(rename = "nema-l14-60p")]
    NemaL1460p,
    #[serde(rename = "nema-l15-20p")]
    NemaL1520p,
    #[serde(rename = "nema-l15-30p")]
    NemaL1530p,
    #[serde(rename = "nema-l15-50p")]
    NemaL1550p,
    #[serde(rename = "nema-l15-60p")]
    NemaL1560p,
    #[serde(rename = "nema-l21-20p")]
    NemaL2120p,
    #[serde(rename = "nema-l21-30p")]
    NemaL2130p,
    #[serde(rename = "nema-l22-30p")]
    NemaL2230p,
    #[serde(rename = "cs6361c")]
    Cs6361c,
    #[serde(rename = "cs6365c")]
    Cs6365c,
    #[serde(rename = "cs8165c")]
    Cs8165c,
    #[serde(rename = "cs8265c")]
    Cs8265c,
    #[serde(rename = "cs8365c")]
    Cs8365c,
    #[serde(rename = "cs8465c")]
    Cs8465c,
    #[serde(rename = "ita-c")]
    ItaC,
    #[serde(rename = "ita-e")]
    ItaE,
    #[serde(rename = "ita-f")]
    ItaF,
    #[serde(rename = "ita-ef")]
    ItaEf,
    #[serde(rename = "ita-g")]
    ItaG,
    #[serde(rename = "ita-h")]
    ItaH,
    #[serde(rename = "ita-i")]
    ItaI,
    #[serde(rename = "ita-j")]
    ItaJ,
    #[serde(rename = "ita-k")]
    ItaK,
    #[serde(rename = "ita-l")]
    ItaL,
    #[serde(rename = "ita-m")]
    ItaM,
    #[serde(rename = "ita-n")]
    ItaN,
    #[serde(rename = "ita-o")]
    ItaO,
    #[serde(rename = "usb-a")]
    UsbA,
    #[serde(rename = "usb-b")]
    UsbB,
    #[serde(rename = "usb-c")]
    UsbC,
    #[serde(rename = "usb-mini-a")]
    UsbMiniA,
    #[serde(rename = "usb-mini-b")]
    UsbMiniB,
    #[serde(rename = "usb-micro-a")]
    UsbMicroA,
    #[serde(rename = "usb-micro-b")]
    UsbMicroB,
    #[serde(rename = "usb-micro-ab")]
    UsbMicroAb,
    #[serde(rename = "usb-3-b")]
    Usb3B,
    #[serde(rename = "usb-3-micro-b")]
    Usb3MicroB,
    #[serde(rename = "dc-terminal")]
    DcTerminal,
    #[serde(rename = "saf-d-grid")]
    SafDGrid,
    #[serde(rename = "neutrik-powercon-20")]
    NeutrikPowercon20,
    #[serde(rename = "neutrik-powercon-32")]
    NeutrikPowercon32,
    #[serde(rename = "neutrik-powercon-true1")]
    NeutrikPowerconTrue1,
    #[serde(rename = "neutrik-powercon-true1-top")]
    NeutrikPowerconTrue1Top,
    #[serde(rename = "ubiquiti-smartpower")]
    UbiquitiSmartpower,
    #[serde(rename = "hardwired")]
    Hardwired,
    #[serde(rename = "other")]
    Other,
    #[serde(rename = "")]
    Empty,
}

impl Default for Type {
    fn default() -> Type {
        Self::Iec60320C6
    }
}

