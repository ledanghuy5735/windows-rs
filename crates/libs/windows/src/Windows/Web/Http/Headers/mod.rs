#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpCacheDirectiveHeaderValueCollection(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IHttpCacheDirectiveHeaderValueCollection {
    type Vtable = IHttpCacheDirectiveHeaderValueCollection_Vtbl;
}
unsafe impl ::windows::core::Interface for IHttpCacheDirectiveHeaderValueCollection {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9a586b89_d5d0_4fbe_bd9d_b5b3636811b4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpCacheDirectiveHeaderValueCollection_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub MaxAge: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaxAge: usize,
    #[cfg(feature = "Foundation")]
    pub SetMaxAge: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetMaxAge: usize,
    #[cfg(feature = "Foundation")]
    pub MaxStale: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaxStale: usize,
    #[cfg(feature = "Foundation")]
    pub SetMaxStale: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetMaxStale: usize,
    #[cfg(feature = "Foundation")]
    pub MinFresh: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MinFresh: usize,
    #[cfg(feature = "Foundation")]
    pub SetMinFresh: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetMinFresh: usize,
    #[cfg(feature = "Foundation")]
    pub SharedMaxAge: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SharedMaxAge: usize,
    #[cfg(feature = "Foundation")]
    pub SetSharedMaxAge: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetSharedMaxAge: usize,
    pub ParseAdd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub TryParseAdd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpChallengeHeaderValue(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IHttpChallengeHeaderValue {
    type Vtable = IHttpChallengeHeaderValue_Vtbl;
}
unsafe impl ::windows::core::Interface for IHttpChallengeHeaderValue {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x393361af_0f7d_4820_9fdd_a2b956eeaeab);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpChallengeHeaderValue_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Parameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Parameters: usize,
    pub Scheme: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Token: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpChallengeHeaderValueCollection(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IHttpChallengeHeaderValueCollection {
    type Vtable = IHttpChallengeHeaderValueCollection_Vtbl;
}
unsafe impl ::windows::core::Interface for IHttpChallengeHeaderValueCollection {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xca9e5f81_aee0_4353_a10b_e625babd64c2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpChallengeHeaderValueCollection_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ParseAdd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub TryParseAdd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpChallengeHeaderValueFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IHttpChallengeHeaderValueFactory {
    type Vtable = IHttpChallengeHeaderValueFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IHttpChallengeHeaderValueFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc452c451_d99c_40aa_9399_90eeb98fc613);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpChallengeHeaderValueFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateFromScheme: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scheme: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateFromSchemeWithToken: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scheme: *mut ::core::ffi::c_void, token: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpChallengeHeaderValueStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IHttpChallengeHeaderValueStatics {
    type Vtable = IHttpChallengeHeaderValueStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IHttpChallengeHeaderValueStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf3d38a72_fc01_4d01_a008_fcb7c459d635);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpChallengeHeaderValueStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Parse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub TryParse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: *mut ::core::ffi::c_void, challengeheadervalue: *mut *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpConnectionOptionHeaderValue(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IHttpConnectionOptionHeaderValue {
    type Vtable = IHttpConnectionOptionHeaderValue_Vtbl;
}
unsafe impl ::windows::core::Interface for IHttpConnectionOptionHeaderValue {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcb4af27a_4e90_45eb_8dcd_fd1408f4c44f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpConnectionOptionHeaderValue_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Token: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpConnectionOptionHeaderValueCollection(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IHttpConnectionOptionHeaderValueCollection {
    type Vtable = IHttpConnectionOptionHeaderValueCollection_Vtbl;
}
unsafe impl ::windows::core::Interface for IHttpConnectionOptionHeaderValueCollection {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe4f56c1d_5142_4e00_8e0f_019509337629);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpConnectionOptionHeaderValueCollection_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ParseAdd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub TryParseAdd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpConnectionOptionHeaderValueFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IHttpConnectionOptionHeaderValueFactory {
    type Vtable = IHttpConnectionOptionHeaderValueFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IHttpConnectionOptionHeaderValueFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd93ccc1e_0b7d_4c3f_a58d_a2a1bdeabc0a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpConnectionOptionHeaderValueFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpConnectionOptionHeaderValueStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IHttpConnectionOptionHeaderValueStatics {
    type Vtable = IHttpConnectionOptionHeaderValueStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IHttpConnectionOptionHeaderValueStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaaa75d37_a946_4b1f_85af_48b68b3c50bd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpConnectionOptionHeaderValueStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Parse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub TryParse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: *mut ::core::ffi::c_void, connectionoptionheadervalue: *mut *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpContentCodingHeaderValue(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IHttpContentCodingHeaderValue {
    type Vtable = IHttpContentCodingHeaderValue_Vtbl;
}
unsafe impl ::windows::core::Interface for IHttpContentCodingHeaderValue {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbcf7f92a_9376_4d85_bccc_9f4f9acab434);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpContentCodingHeaderValue_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ContentCoding: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpContentCodingHeaderValueCollection(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IHttpContentCodingHeaderValueCollection {
    type Vtable = IHttpContentCodingHeaderValueCollection_Vtbl;
}
unsafe impl ::windows::core::Interface for IHttpContentCodingHeaderValueCollection {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7d221721_a6db_436e_8e83_91596192819c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpContentCodingHeaderValueCollection_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ParseAdd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub TryParseAdd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpContentCodingHeaderValueFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IHttpContentCodingHeaderValueFactory {
    type Vtable = IHttpContentCodingHeaderValueFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IHttpContentCodingHeaderValueFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc53d2bd7_332b_4350_8510_2e67a2289a5a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpContentCodingHeaderValueFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contentcoding: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpContentCodingHeaderValueStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IHttpContentCodingHeaderValueStatics {
    type Vtable = IHttpContentCodingHeaderValueStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IHttpContentCodingHeaderValueStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x94d8602e_f9bf_42f7_aa46_ed272a41e212);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpContentCodingHeaderValueStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Parse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub TryParse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: *mut ::core::ffi::c_void, contentcodingheadervalue: *mut *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpContentCodingWithQualityHeaderValue(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IHttpContentCodingWithQualityHeaderValue {
    type Vtable = IHttpContentCodingWithQualityHeaderValue_Vtbl;
}
unsafe impl ::windows::core::Interface for IHttpContentCodingWithQualityHeaderValue {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x94531cd5_8b13_4d73_8651_f76b38f88495);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpContentCodingWithQualityHeaderValue_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ContentCoding: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Quality: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Quality: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpContentCodingWithQualityHeaderValueCollection(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IHttpContentCodingWithQualityHeaderValueCollection {
    type Vtable = IHttpContentCodingWithQualityHeaderValueCollection_Vtbl;
}
unsafe impl ::windows::core::Interface for IHttpContentCodingWithQualityHeaderValueCollection {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7c0d753e_e899_4378_b5c8_412d820711cc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpContentCodingWithQualityHeaderValueCollection_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ParseAdd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub TryParseAdd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpContentCodingWithQualityHeaderValueFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IHttpContentCodingWithQualityHeaderValueFactory {
    type Vtable = IHttpContentCodingWithQualityHeaderValueFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IHttpContentCodingWithQualityHeaderValueFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc45eee1a_c553_46fc_ade2_d75c1d53df7b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpContentCodingWithQualityHeaderValueFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateFromValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contentcoding: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateFromValueWithQuality: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contentcoding: *mut ::core::ffi::c_void, quality: f64, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpContentCodingWithQualityHeaderValueStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IHttpContentCodingWithQualityHeaderValueStatics {
    type Vtable = IHttpContentCodingWithQualityHeaderValueStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IHttpContentCodingWithQualityHeaderValueStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe8c9357c_8f89_4801_8e75_4c9abfc3de71);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpContentCodingWithQualityHeaderValueStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Parse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub TryParse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: *mut ::core::ffi::c_void, contentcodingwithqualityheadervalue: *mut *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpContentDispositionHeaderValue(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IHttpContentDispositionHeaderValue {
    type Vtable = IHttpContentDispositionHeaderValue_Vtbl;
}
unsafe impl ::windows::core::Interface for IHttpContentDispositionHeaderValue {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf2a2eedc_2629_4b49_9908_96a168e9365e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpContentDispositionHeaderValue_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub DispositionType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetDispositionType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub FileName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetFileName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub FileNameStar: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetFileNameStar: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Parameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Parameters: usize,
    #[cfg(feature = "Foundation")]
    pub Size: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Size: usize,
    #[cfg(feature = "Foundation")]
    pub SetSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetSize: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpContentDispositionHeaderValueFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IHttpContentDispositionHeaderValueFactory {
    type Vtable = IHttpContentDispositionHeaderValueFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IHttpContentDispositionHeaderValueFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9915bbc4_456c_4e81_8295_b2ab3cbcf545);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpContentDispositionHeaderValueFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispositiontype: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpContentDispositionHeaderValueStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IHttpContentDispositionHeaderValueStatics {
    type Vtable = IHttpContentDispositionHeaderValueStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IHttpContentDispositionHeaderValueStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x29c56067_5a37_46e4_b074_c5177d69ca66);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpContentDispositionHeaderValueStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Parse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub TryParse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: *mut ::core::ffi::c_void, contentdispositionheadervalue: *mut *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpContentHeaderCollection(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IHttpContentHeaderCollection {
    type Vtable = IHttpContentHeaderCollection_Vtbl;
}
unsafe impl ::windows::core::Interface for IHttpContentHeaderCollection {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x40612a44_47ae_4b7e_9124_69628b64aa18);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpContentHeaderCollection_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ContentDisposition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetContentDisposition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ContentEncoding: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ContentLanguage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ContentLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ContentLength: usize,
    #[cfg(feature = "Foundation")]
    pub SetContentLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetContentLength: usize,
    #[cfg(feature = "Foundation")]
    pub ContentLocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ContentLocation: usize,
    #[cfg(feature = "Foundation")]
    pub SetContentLocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetContentLocation: usize,
    #[cfg(feature = "Storage_Streams")]
    pub ContentMD5: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    ContentMD5: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetContentMD5: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetContentMD5: usize,
    pub ContentRange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetContentRange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ContentType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetContentType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Expires: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Expires: usize,
    #[cfg(feature = "Foundation")]
    pub SetExpires: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetExpires: usize,
    #[cfg(feature = "Foundation")]
    pub LastModified: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LastModified: usize,
    #[cfg(feature = "Foundation")]
    pub SetLastModified: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetLastModified: usize,
    pub Append: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub TryAppendWithoutValidation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpContentRangeHeaderValue(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IHttpContentRangeHeaderValue {
    type Vtable = IHttpContentRangeHeaderValue_Vtbl;
}
unsafe impl ::windows::core::Interface for IHttpContentRangeHeaderValue {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x04d967d3_a4f6_495c_9530_8579fcba8aa9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpContentRangeHeaderValue_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub FirstBytePosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FirstBytePosition: usize,
    #[cfg(feature = "Foundation")]
    pub LastBytePosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LastBytePosition: usize,
    #[cfg(feature = "Foundation")]
    pub Length: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Length: usize,
    pub Unit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetUnit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpContentRangeHeaderValueFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IHttpContentRangeHeaderValueFactory {
    type Vtable = IHttpContentRangeHeaderValueFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IHttpContentRangeHeaderValueFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3f5bd691_a03c_4456_9a6f_ef27ecd03cae);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpContentRangeHeaderValueFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateFromLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, length: u64, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateFromRange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, from: u64, to: u64, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateFromRangeWithLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, from: u64, to: u64, length: u64, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpContentRangeHeaderValueStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IHttpContentRangeHeaderValueStatics {
    type Vtable = IHttpContentRangeHeaderValueStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IHttpContentRangeHeaderValueStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x80a346ca_174c_4fae_821c_134cd294aa38);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpContentRangeHeaderValueStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Parse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub TryParse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: *mut ::core::ffi::c_void, contentrangeheadervalue: *mut *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpCookiePairHeaderValue(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IHttpCookiePairHeaderValue {
    type Vtable = IHttpCookiePairHeaderValue_Vtbl;
}
unsafe impl ::windows::core::Interface for IHttpCookiePairHeaderValue {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcbd46217_4b29_412b_bd90_b3d814ab8e1b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpCookiePairHeaderValue_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpCookiePairHeaderValueCollection(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IHttpCookiePairHeaderValueCollection {
    type Vtable = IHttpCookiePairHeaderValueCollection_Vtbl;
}
unsafe impl ::windows::core::Interface for IHttpCookiePairHeaderValueCollection {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf3f44350_581e_4ecc_9f59_e507d04f06e6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpCookiePairHeaderValueCollection_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ParseAdd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub TryParseAdd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpCookiePairHeaderValueFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IHttpCookiePairHeaderValueFactory {
    type Vtable = IHttpCookiePairHeaderValueFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IHttpCookiePairHeaderValueFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x635e326f_146f_4f56_aa21_2cb7d6d58b1e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpCookiePairHeaderValueFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateFromName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateFromNameWithValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpCookiePairHeaderValueStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IHttpCookiePairHeaderValueStatics {
    type Vtable = IHttpCookiePairHeaderValueStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IHttpCookiePairHeaderValueStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6e866d48_06af_4462_8158_99388d5dca81);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpCookiePairHeaderValueStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Parse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub TryParse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: *mut ::core::ffi::c_void, cookiepairheadervalue: *mut *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpCredentialsHeaderValue(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IHttpCredentialsHeaderValue {
    type Vtable = IHttpCredentialsHeaderValue_Vtbl;
}
unsafe impl ::windows::core::Interface for IHttpCredentialsHeaderValue {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc34cc3cb_542e_4177_a6c7_b674ce193fbf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpCredentialsHeaderValue_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Parameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Parameters: usize,
    pub Scheme: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Token: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpCredentialsHeaderValueFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IHttpCredentialsHeaderValueFactory {
    type Vtable = IHttpCredentialsHeaderValueFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IHttpCredentialsHeaderValueFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf21d9e91_4d1c_4182_bfd1_34470a62f950);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpCredentialsHeaderValueFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateFromScheme: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scheme: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateFromSchemeWithToken: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scheme: *mut ::core::ffi::c_void, token: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpCredentialsHeaderValueStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IHttpCredentialsHeaderValueStatics {
    type Vtable = IHttpCredentialsHeaderValueStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IHttpCredentialsHeaderValueStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa69b2be6_ce8c_4443_a35a_1b727b131036);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpCredentialsHeaderValueStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Parse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub TryParse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: *mut ::core::ffi::c_void, credentialsheadervalue: *mut *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpDateOrDeltaHeaderValue(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IHttpDateOrDeltaHeaderValue {
    type Vtable = IHttpDateOrDeltaHeaderValue_Vtbl;
}
unsafe impl ::windows::core::Interface for IHttpDateOrDeltaHeaderValue {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeafcaa6a_c4dc_49e2_a27d_043adf5867a3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpDateOrDeltaHeaderValue_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub Date: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Date: usize,
    #[cfg(feature = "Foundation")]
    pub Delta: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Delta: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpDateOrDeltaHeaderValueStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IHttpDateOrDeltaHeaderValueStatics {
    type Vtable = IHttpDateOrDeltaHeaderValueStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IHttpDateOrDeltaHeaderValueStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7c2659a8_6672_4e90_9a9a_f39766f7f576);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpDateOrDeltaHeaderValueStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Parse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub TryParse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: *mut ::core::ffi::c_void, dateordeltaheadervalue: *mut *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpExpectationHeaderValue(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IHttpExpectationHeaderValue {
    type Vtable = IHttpExpectationHeaderValue_Vtbl;
}
unsafe impl ::windows::core::Interface for IHttpExpectationHeaderValue {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4ce585cd_3a99_43af_a2e6_ec232fea9658);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpExpectationHeaderValue_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Parameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Parameters: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpExpectationHeaderValueCollection(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IHttpExpectationHeaderValueCollection {
    type Vtable = IHttpExpectationHeaderValueCollection_Vtbl;
}
unsafe impl ::windows::core::Interface for IHttpExpectationHeaderValueCollection {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe78521b3_a0e2_4ac4_9e66_79706cb9fd58);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpExpectationHeaderValueCollection_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ParseAdd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub TryParseAdd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpExpectationHeaderValueFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IHttpExpectationHeaderValueFactory {
    type Vtable = IHttpExpectationHeaderValueFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IHttpExpectationHeaderValueFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4ea275cb_d53e_4868_8856_1e21a5030dc0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpExpectationHeaderValueFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateFromName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateFromNameWithValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpExpectationHeaderValueStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IHttpExpectationHeaderValueStatics {
    type Vtable = IHttpExpectationHeaderValueStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IHttpExpectationHeaderValueStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3019abe2_cfe5_473b_a57f_fba5b14eb257);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpExpectationHeaderValueStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Parse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub TryParse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: *mut ::core::ffi::c_void, expectationheadervalue: *mut *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpLanguageHeaderValueCollection(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IHttpLanguageHeaderValueCollection {
    type Vtable = IHttpLanguageHeaderValueCollection_Vtbl;
}
unsafe impl ::windows::core::Interface for IHttpLanguageHeaderValueCollection {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9ebd7ca3_8219_44f6_9902_8c56dfd3340c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpLanguageHeaderValueCollection_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ParseAdd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub TryParseAdd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpLanguageRangeWithQualityHeaderValue(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IHttpLanguageRangeWithQualityHeaderValue {
    type Vtable = IHttpLanguageRangeWithQualityHeaderValue_Vtbl;
}
unsafe impl ::windows::core::Interface for IHttpLanguageRangeWithQualityHeaderValue {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7256e102_0080_4db4_a083_7de7b2e5ba4c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpLanguageRangeWithQualityHeaderValue_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub LanguageRange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Quality: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Quality: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpLanguageRangeWithQualityHeaderValueCollection(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IHttpLanguageRangeWithQualityHeaderValueCollection {
    type Vtable = IHttpLanguageRangeWithQualityHeaderValueCollection_Vtbl;
}
unsafe impl ::windows::core::Interface for IHttpLanguageRangeWithQualityHeaderValueCollection {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x885d5abd_4b4f_480a_89ce_8aedcee6e3a0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpLanguageRangeWithQualityHeaderValueCollection_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ParseAdd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub TryParseAdd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpLanguageRangeWithQualityHeaderValueFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IHttpLanguageRangeWithQualityHeaderValueFactory {
    type Vtable = IHttpLanguageRangeWithQualityHeaderValueFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IHttpLanguageRangeWithQualityHeaderValueFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7bb83970_780f_4c83_9fe4_dc3087f6bd55);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpLanguageRangeWithQualityHeaderValueFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateFromLanguageRange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, languagerange: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateFromLanguageRangeWithQuality: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, languagerange: *mut ::core::ffi::c_void, quality: f64, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpLanguageRangeWithQualityHeaderValueStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IHttpLanguageRangeWithQualityHeaderValueStatics {
    type Vtable = IHttpLanguageRangeWithQualityHeaderValueStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IHttpLanguageRangeWithQualityHeaderValueStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2541e146_f308_46f5_b695_42f54024ec68);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpLanguageRangeWithQualityHeaderValueStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Parse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub TryParse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: *mut ::core::ffi::c_void, languagerangewithqualityheadervalue: *mut *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpMediaTypeHeaderValue(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IHttpMediaTypeHeaderValue {
    type Vtable = IHttpMediaTypeHeaderValue_Vtbl;
}
unsafe impl ::windows::core::Interface for IHttpMediaTypeHeaderValue {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x16b28533_e728_4fcb_bdb0_08a431a14844);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpMediaTypeHeaderValue_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CharSet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetCharSet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub MediaType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetMediaType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Parameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Parameters: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpMediaTypeHeaderValueFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IHttpMediaTypeHeaderValueFactory {
    type Vtable = IHttpMediaTypeHeaderValueFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IHttpMediaTypeHeaderValueFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbed747a8_cd17_42dd_9367_ab9c5b56dd7d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpMediaTypeHeaderValueFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mediatype: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpMediaTypeHeaderValueStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IHttpMediaTypeHeaderValueStatics {
    type Vtable = IHttpMediaTypeHeaderValueStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IHttpMediaTypeHeaderValueStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe04d83df_1d41_4d8c_a2de_6fd2ed87399b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpMediaTypeHeaderValueStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Parse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub TryParse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: *mut ::core::ffi::c_void, mediatypeheadervalue: *mut *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpMediaTypeWithQualityHeaderValue(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IHttpMediaTypeWithQualityHeaderValue {
    type Vtable = IHttpMediaTypeWithQualityHeaderValue_Vtbl;
}
unsafe impl ::windows::core::Interface for IHttpMediaTypeWithQualityHeaderValue {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x188d5e32_76be_44a0_b1cd_2074bded2dde);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpMediaTypeWithQualityHeaderValue_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CharSet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetCharSet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub MediaType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetMediaType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Parameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Parameters: usize,
    #[cfg(feature = "Foundation")]
    pub Quality: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Quality: usize,
    #[cfg(feature = "Foundation")]
    pub SetQuality: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetQuality: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpMediaTypeWithQualityHeaderValueCollection(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IHttpMediaTypeWithQualityHeaderValueCollection {
    type Vtable = IHttpMediaTypeWithQualityHeaderValueCollection_Vtbl;
}
unsafe impl ::windows::core::Interface for IHttpMediaTypeWithQualityHeaderValueCollection {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3c0c6b73_1342_4587_a056_18d02ff67165);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpMediaTypeWithQualityHeaderValueCollection_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ParseAdd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub TryParseAdd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpMediaTypeWithQualityHeaderValueFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IHttpMediaTypeWithQualityHeaderValueFactory {
    type Vtable = IHttpMediaTypeWithQualityHeaderValueFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IHttpMediaTypeWithQualityHeaderValueFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4c6d20f4_9457_44e6_a323_d122b958780b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpMediaTypeWithQualityHeaderValueFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateFromMediaType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mediatype: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateFromMediaTypeWithQuality: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mediatype: *mut ::core::ffi::c_void, quality: f64, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpMediaTypeWithQualityHeaderValueStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IHttpMediaTypeWithQualityHeaderValueStatics {
    type Vtable = IHttpMediaTypeWithQualityHeaderValueStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IHttpMediaTypeWithQualityHeaderValueStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5b070cd9_b560_4fc8_9835_7e6c0a657b24);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpMediaTypeWithQualityHeaderValueStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Parse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub TryParse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: *mut ::core::ffi::c_void, mediatypewithqualityheadervalue: *mut *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpMethodHeaderValueCollection(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IHttpMethodHeaderValueCollection {
    type Vtable = IHttpMethodHeaderValueCollection_Vtbl;
}
unsafe impl ::windows::core::Interface for IHttpMethodHeaderValueCollection {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x43bc3ff4_6119_4adf_938c_34bfffcf92ed);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpMethodHeaderValueCollection_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ParseAdd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub TryParseAdd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpNameValueHeaderValue(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IHttpNameValueHeaderValue {
    type Vtable = IHttpNameValueHeaderValue_Vtbl;
}
unsafe impl ::windows::core::Interface for IHttpNameValueHeaderValue {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd8ba7463_5b9a_4d1b_93f9_aa5b44ecfddf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpNameValueHeaderValue_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpNameValueHeaderValueFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IHttpNameValueHeaderValueFactory {
    type Vtable = IHttpNameValueHeaderValueFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IHttpNameValueHeaderValueFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x770e2267_cbf8_4736_a925_93fbe10c7ca8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpNameValueHeaderValueFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateFromName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateFromNameWithValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpNameValueHeaderValueStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IHttpNameValueHeaderValueStatics {
    type Vtable = IHttpNameValueHeaderValueStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IHttpNameValueHeaderValueStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xffd4030f_1130_4152_8659_256909a9d115);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpNameValueHeaderValueStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Parse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub TryParse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: *mut ::core::ffi::c_void, namevalueheadervalue: *mut *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpProductHeaderValue(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IHttpProductHeaderValue {
    type Vtable = IHttpProductHeaderValue_Vtbl;
}
unsafe impl ::windows::core::Interface for IHttpProductHeaderValue {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf4feee03_ebd4_4160_b9ff_807c5183b6e6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpProductHeaderValue_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Version: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpProductHeaderValueFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IHttpProductHeaderValueFactory {
    type Vtable = IHttpProductHeaderValueFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IHttpProductHeaderValueFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x611aa4f5_82bc_42fb_977b_dc00536e5e86);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpProductHeaderValueFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateFromName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productname: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateFromNameWithVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productname: *mut ::core::ffi::c_void, productversion: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpProductHeaderValueStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IHttpProductHeaderValueStatics {
    type Vtable = IHttpProductHeaderValueStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IHttpProductHeaderValueStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x90c33e29_befc_4337_be62_49f097975f53);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpProductHeaderValueStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Parse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub TryParse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: *mut ::core::ffi::c_void, productheadervalue: *mut *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpProductInfoHeaderValue(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IHttpProductInfoHeaderValue {
    type Vtable = IHttpProductInfoHeaderValue_Vtbl;
}
unsafe impl ::windows::core::Interface for IHttpProductInfoHeaderValue {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1b1a8732_4c35_486a_966f_646489198e4d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpProductInfoHeaderValue_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Product: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Comment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpProductInfoHeaderValueCollection(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IHttpProductInfoHeaderValueCollection {
    type Vtable = IHttpProductInfoHeaderValueCollection_Vtbl;
}
unsafe impl ::windows::core::Interface for IHttpProductInfoHeaderValueCollection {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x877df74a_d69b_44f8_ad4f_453af9c42ed0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpProductInfoHeaderValueCollection_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ParseAdd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub TryParseAdd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpProductInfoHeaderValueFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IHttpProductInfoHeaderValueFactory {
    type Vtable = IHttpProductInfoHeaderValueFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IHttpProductInfoHeaderValueFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x24220fbe_eabe_4464_b460_ec010b7c41e2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpProductInfoHeaderValueFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateFromComment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productcomment: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateFromNameWithVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productname: *mut ::core::ffi::c_void, productversion: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpProductInfoHeaderValueStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IHttpProductInfoHeaderValueStatics {
    type Vtable = IHttpProductInfoHeaderValueStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IHttpProductInfoHeaderValueStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdb7fd857_327a_4e73_81e5_7059a302b042);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpProductInfoHeaderValueStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Parse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub TryParse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: *mut ::core::ffi::c_void, productinfoheadervalue: *mut *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpRequestHeaderCollection(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IHttpRequestHeaderCollection {
    type Vtable = IHttpRequestHeaderCollection_Vtbl;
}
unsafe impl ::windows::core::Interface for IHttpRequestHeaderCollection {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaf40329b_b544_469b_86b9_ac3d466fea36);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpRequestHeaderCollection_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Accept: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AcceptEncoding: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AcceptLanguage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Authorization: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetAuthorization: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CacheControl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Connection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Cookie: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Date: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Date: usize,
    #[cfg(feature = "Foundation")]
    pub SetDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetDate: usize,
    pub Expect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub From: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetFrom: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Networking")]
    pub Host: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Networking"))]
    Host: usize,
    #[cfg(feature = "Networking")]
    pub SetHost: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Networking"))]
    SetHost: usize,
    #[cfg(feature = "Foundation")]
    pub IfModifiedSince: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    IfModifiedSince: usize,
    #[cfg(feature = "Foundation")]
    pub SetIfModifiedSince: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetIfModifiedSince: usize,
    #[cfg(feature = "Foundation")]
    pub IfUnmodifiedSince: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    IfUnmodifiedSince: usize,
    #[cfg(feature = "Foundation")]
    pub SetIfUnmodifiedSince: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetIfUnmodifiedSince: usize,
    #[cfg(feature = "Foundation")]
    pub MaxForwards: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaxForwards: usize,
    #[cfg(feature = "Foundation")]
    pub SetMaxForwards: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetMaxForwards: usize,
    pub ProxyAuthorization: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetProxyAuthorization: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Referer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Referer: usize,
    #[cfg(feature = "Foundation")]
    pub SetReferer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetReferer: usize,
    pub TransferEncoding: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub UserAgent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Append: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub TryAppendWithoutValidation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpResponseHeaderCollection(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IHttpResponseHeaderCollection {
    type Vtable = IHttpResponseHeaderCollection_Vtbl;
}
unsafe impl ::windows::core::Interface for IHttpResponseHeaderCollection {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7a990969_fa3f_41ed_aac6_bf957975c16b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpResponseHeaderCollection_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub Age: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Age: usize,
    #[cfg(feature = "Foundation")]
    pub SetAge: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetAge: usize,
    pub Allow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CacheControl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Connection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Date: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Date: usize,
    #[cfg(feature = "Foundation")]
    pub SetDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetDate: usize,
    #[cfg(feature = "Foundation")]
    pub Location: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Location: usize,
    #[cfg(feature = "Foundation")]
    pub SetLocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetLocation: usize,
    pub ProxyAuthenticate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RetryAfter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetRetryAfter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub TransferEncoding: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub WwwAuthenticate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Append: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub TryAppendWithoutValidation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpTransferCodingHeaderValue(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IHttpTransferCodingHeaderValue {
    type Vtable = IHttpTransferCodingHeaderValue_Vtbl;
}
unsafe impl ::windows::core::Interface for IHttpTransferCodingHeaderValue {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x436f32f9_3ded_42bd_b38a_5496a2511ce6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpTransferCodingHeaderValue_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Parameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Parameters: usize,
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpTransferCodingHeaderValueCollection(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IHttpTransferCodingHeaderValueCollection {
    type Vtable = IHttpTransferCodingHeaderValueCollection_Vtbl;
}
unsafe impl ::windows::core::Interface for IHttpTransferCodingHeaderValueCollection {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x202c8c34_2c03_49b8_9665_73e27cb2fc79);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpTransferCodingHeaderValueCollection_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ParseAdd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub TryParseAdd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpTransferCodingHeaderValueFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IHttpTransferCodingHeaderValueFactory {
    type Vtable = IHttpTransferCodingHeaderValueFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IHttpTransferCodingHeaderValueFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbb62dffc_e361_4f08_8e4f_c9e723de703b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpTransferCodingHeaderValueFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHttpTransferCodingHeaderValueStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IHttpTransferCodingHeaderValueStatics {
    type Vtable = IHttpTransferCodingHeaderValueStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IHttpTransferCodingHeaderValueStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6ab8892a_1a98_4d32_a906_7470a9875ce5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpTransferCodingHeaderValueStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Parse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub TryParse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: *mut ::core::ffi::c_void, transfercodingheadervalue: *mut *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Web_Http_Headers\"`*"]
#[repr(transparent)]
pub struct HttpCacheDirectiveHeaderValueCollection(::windows::core::IUnknown);
impl HttpCacheDirectiveHeaderValueCollection {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MaxAge(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MaxAge)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetMaxAge<P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetMaxAge)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MaxStale(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MaxStale)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetMaxStale<P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetMaxStale)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MinFresh(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MinFresh)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetMinFresh<P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetMinFresh)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SharedMaxAge(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SharedMaxAge)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetSharedMaxAge<P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetSharedMaxAge)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    pub fn ParseAdd(&self, input: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).ParseAdd)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(input)).ok() }
    }
    pub fn TryParseAdd(&self, input: &::windows::core::HSTRING) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TryParseAdd)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(input), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IIterator<HttpNameValueHeaderValue>> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IIterable<HttpNameValueHeaderValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).First)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ToString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ToString)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAt(&self, index: u32) -> ::windows::core::Result<HttpNameValueHeaderValue> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpNameValueHeaderValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetAt)(::windows::core::Vtable::as_raw(this), index, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpNameValueHeaderValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Size)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetView(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<HttpNameValueHeaderValue>> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpNameValueHeaderValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetView)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn IndexOf(&self, value: &HttpNameValueHeaderValue, index: &mut u32) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpNameValueHeaderValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IndexOf)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value), index, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetAt(&self, index: u32, value: &HttpNameValueHeaderValue) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpNameValueHeaderValue>>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetAt)(::windows::core::Vtable::as_raw(this), index, ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn InsertAt(&self, index: u32, value: &HttpNameValueHeaderValue) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpNameValueHeaderValue>>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).InsertAt)(::windows::core::Vtable::as_raw(this), index, ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAt(&self, index: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpNameValueHeaderValue>>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveAt)(::windows::core::Vtable::as_raw(this), index).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Append(&self, value: &HttpNameValueHeaderValue) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpNameValueHeaderValue>>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Append)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAtEnd(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpNameValueHeaderValue>>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveAtEnd)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Clear(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpNameValueHeaderValue>>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Clear)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMany(&self, startindex: u32, items: &mut [::core::option::Option<HttpNameValueHeaderValue>]) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpNameValueHeaderValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetMany)(::windows::core::Vtable::as_raw(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ReplaceAll(&self, items: &[::core::option::Option<HttpNameValueHeaderValue>]) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpNameValueHeaderValue>>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).ReplaceAll)(::windows::core::Vtable::as_raw(this), items.len() as u32, ::core::mem::transmute(items.as_ptr())).ok() }
    }
}
impl ::core::clone::Clone for HttpCacheDirectiveHeaderValueCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HttpCacheDirectiveHeaderValueCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpCacheDirectiveHeaderValueCollection {}
impl ::core::fmt::Debug for HttpCacheDirectiveHeaderValueCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HttpCacheDirectiveHeaderValueCollection").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for HttpCacheDirectiveHeaderValueCollection {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Headers.HttpCacheDirectiveHeaderValueCollection;{9a586b89-d5d0-4fbe-bd9d-b5b3636811b4})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for HttpCacheDirectiveHeaderValueCollection {
    type Vtable = IHttpCacheDirectiveHeaderValueCollection_Vtbl;
}
unsafe impl ::windows::core::Interface for HttpCacheDirectiveHeaderValueCollection {
    const IID: ::windows::core::GUID = <IHttpCacheDirectiveHeaderValueCollection as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for HttpCacheDirectiveHeaderValueCollection {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpCacheDirectiveHeaderValueCollection";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for HttpCacheDirectiveHeaderValueCollection {
    type Item = HttpNameValueHeaderValue;
    type IntoIter = super::super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for &HttpCacheDirectiveHeaderValueCollection {
    type Item = HttpNameValueHeaderValue;
    type IntoIter = super::super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        super::super::super::Foundation::Collections::VectorIterator::new(::core::convert::TryInto::try_into(self).ok())
    }
}
::windows::core::interface_hierarchy!(HttpCacheDirectiveHeaderValueCollection, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<HttpCacheDirectiveHeaderValueCollection> for super::super::super::Foundation::Collections::IIterable<HttpNameValueHeaderValue> {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpCacheDirectiveHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&HttpCacheDirectiveHeaderValueCollection> for super::super::super::Foundation::Collections::IIterable<HttpNameValueHeaderValue> {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpCacheDirectiveHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&HttpCacheDirectiveHeaderValueCollection> for ::windows::core::InParam<super::super::super::Foundation::Collections::IIterable<HttpNameValueHeaderValue>> {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpCacheDirectiveHeaderValueCollection) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<HttpCacheDirectiveHeaderValueCollection> for super::super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpCacheDirectiveHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&HttpCacheDirectiveHeaderValueCollection> for super::super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpCacheDirectiveHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&HttpCacheDirectiveHeaderValueCollection> for ::windows::core::InParam<super::super::super::Foundation::IStringable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpCacheDirectiveHeaderValueCollection) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<HttpCacheDirectiveHeaderValueCollection> for super::super::super::Foundation::Collections::IVector<HttpNameValueHeaderValue> {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpCacheDirectiveHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&HttpCacheDirectiveHeaderValueCollection> for super::super::super::Foundation::Collections::IVector<HttpNameValueHeaderValue> {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpCacheDirectiveHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&HttpCacheDirectiveHeaderValueCollection> for ::windows::core::InParam<super::super::super::Foundation::Collections::IVector<HttpNameValueHeaderValue>> {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpCacheDirectiveHeaderValueCollection) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for HttpCacheDirectiveHeaderValueCollection {}
unsafe impl ::core::marker::Sync for HttpCacheDirectiveHeaderValueCollection {}
#[doc = "*Required features: `\"Web_Http_Headers\"`*"]
#[repr(transparent)]
pub struct HttpChallengeHeaderValue(::windows::core::IUnknown);
impl HttpChallengeHeaderValue {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Parameters(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<HttpNameValueHeaderValue>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Parameters)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Scheme(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Scheme)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Token(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Token)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CreateFromScheme(scheme: &::windows::core::HSTRING) -> ::windows::core::Result<HttpChallengeHeaderValue> {
        Self::IHttpChallengeHeaderValueFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateFromScheme)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(scheme), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn CreateFromSchemeWithToken(scheme: &::windows::core::HSTRING, token: &::windows::core::HSTRING) -> ::windows::core::Result<HttpChallengeHeaderValue> {
        Self::IHttpChallengeHeaderValueFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateFromSchemeWithToken)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(scheme), ::core::mem::transmute_copy(token), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn Parse(input: &::windows::core::HSTRING) -> ::windows::core::Result<HttpChallengeHeaderValue> {
        Self::IHttpChallengeHeaderValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Parse)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(input), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn TryParse(input: &::windows::core::HSTRING, challengeheadervalue: &mut ::core::option::Option<HttpChallengeHeaderValue>) -> ::windows::core::Result<bool> {
        Self::IHttpChallengeHeaderValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TryParse)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(input), challengeheadervalue as *mut _ as _, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ToString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ToString)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc(hidden)]
    pub fn IHttpChallengeHeaderValueFactory<R, F: FnOnce(&IHttpChallengeHeaderValueFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<HttpChallengeHeaderValue, IHttpChallengeHeaderValueFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IHttpChallengeHeaderValueStatics<R, F: FnOnce(&IHttpChallengeHeaderValueStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<HttpChallengeHeaderValue, IHttpChallengeHeaderValueStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for HttpChallengeHeaderValue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HttpChallengeHeaderValue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpChallengeHeaderValue {}
impl ::core::fmt::Debug for HttpChallengeHeaderValue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HttpChallengeHeaderValue").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for HttpChallengeHeaderValue {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Headers.HttpChallengeHeaderValue;{393361af-0f7d-4820-9fdd-a2b956eeaeab})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for HttpChallengeHeaderValue {
    type Vtable = IHttpChallengeHeaderValue_Vtbl;
}
unsafe impl ::windows::core::Interface for HttpChallengeHeaderValue {
    const IID: ::windows::core::GUID = <IHttpChallengeHeaderValue as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for HttpChallengeHeaderValue {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpChallengeHeaderValue";
}
::windows::core::interface_hierarchy!(HttpChallengeHeaderValue, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<HttpChallengeHeaderValue> for super::super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpChallengeHeaderValue) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&HttpChallengeHeaderValue> for super::super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpChallengeHeaderValue) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&HttpChallengeHeaderValue> for ::windows::core::InParam<super::super::super::Foundation::IStringable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpChallengeHeaderValue) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for HttpChallengeHeaderValue {}
unsafe impl ::core::marker::Sync for HttpChallengeHeaderValue {}
#[doc = "*Required features: `\"Web_Http_Headers\"`*"]
#[repr(transparent)]
pub struct HttpChallengeHeaderValueCollection(::windows::core::IUnknown);
impl HttpChallengeHeaderValueCollection {
    pub fn ParseAdd(&self, input: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).ParseAdd)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(input)).ok() }
    }
    pub fn TryParseAdd(&self, input: &::windows::core::HSTRING) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TryParseAdd)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(input), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IIterator<HttpChallengeHeaderValue>> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IIterable<HttpChallengeHeaderValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).First)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ToString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ToString)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAt(&self, index: u32) -> ::windows::core::Result<HttpChallengeHeaderValue> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpChallengeHeaderValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetAt)(::windows::core::Vtable::as_raw(this), index, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpChallengeHeaderValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Size)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetView(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<HttpChallengeHeaderValue>> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpChallengeHeaderValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetView)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn IndexOf(&self, value: &HttpChallengeHeaderValue, index: &mut u32) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpChallengeHeaderValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IndexOf)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value), index, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetAt(&self, index: u32, value: &HttpChallengeHeaderValue) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpChallengeHeaderValue>>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetAt)(::windows::core::Vtable::as_raw(this), index, ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn InsertAt(&self, index: u32, value: &HttpChallengeHeaderValue) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpChallengeHeaderValue>>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).InsertAt)(::windows::core::Vtable::as_raw(this), index, ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAt(&self, index: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpChallengeHeaderValue>>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveAt)(::windows::core::Vtable::as_raw(this), index).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Append(&self, value: &HttpChallengeHeaderValue) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpChallengeHeaderValue>>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Append)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAtEnd(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpChallengeHeaderValue>>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveAtEnd)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Clear(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpChallengeHeaderValue>>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Clear)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMany(&self, startindex: u32, items: &mut [::core::option::Option<HttpChallengeHeaderValue>]) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpChallengeHeaderValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetMany)(::windows::core::Vtable::as_raw(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ReplaceAll(&self, items: &[::core::option::Option<HttpChallengeHeaderValue>]) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpChallengeHeaderValue>>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).ReplaceAll)(::windows::core::Vtable::as_raw(this), items.len() as u32, ::core::mem::transmute(items.as_ptr())).ok() }
    }
}
impl ::core::clone::Clone for HttpChallengeHeaderValueCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HttpChallengeHeaderValueCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpChallengeHeaderValueCollection {}
impl ::core::fmt::Debug for HttpChallengeHeaderValueCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HttpChallengeHeaderValueCollection").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for HttpChallengeHeaderValueCollection {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Headers.HttpChallengeHeaderValueCollection;{ca9e5f81-aee0-4353-a10b-e625babd64c2})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for HttpChallengeHeaderValueCollection {
    type Vtable = IHttpChallengeHeaderValueCollection_Vtbl;
}
unsafe impl ::windows::core::Interface for HttpChallengeHeaderValueCollection {
    const IID: ::windows::core::GUID = <IHttpChallengeHeaderValueCollection as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for HttpChallengeHeaderValueCollection {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpChallengeHeaderValueCollection";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for HttpChallengeHeaderValueCollection {
    type Item = HttpChallengeHeaderValue;
    type IntoIter = super::super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for &HttpChallengeHeaderValueCollection {
    type Item = HttpChallengeHeaderValue;
    type IntoIter = super::super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        super::super::super::Foundation::Collections::VectorIterator::new(::core::convert::TryInto::try_into(self).ok())
    }
}
::windows::core::interface_hierarchy!(HttpChallengeHeaderValueCollection, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<HttpChallengeHeaderValueCollection> for super::super::super::Foundation::Collections::IIterable<HttpChallengeHeaderValue> {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpChallengeHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&HttpChallengeHeaderValueCollection> for super::super::super::Foundation::Collections::IIterable<HttpChallengeHeaderValue> {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpChallengeHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&HttpChallengeHeaderValueCollection> for ::windows::core::InParam<super::super::super::Foundation::Collections::IIterable<HttpChallengeHeaderValue>> {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpChallengeHeaderValueCollection) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<HttpChallengeHeaderValueCollection> for super::super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpChallengeHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&HttpChallengeHeaderValueCollection> for super::super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpChallengeHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&HttpChallengeHeaderValueCollection> for ::windows::core::InParam<super::super::super::Foundation::IStringable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpChallengeHeaderValueCollection) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<HttpChallengeHeaderValueCollection> for super::super::super::Foundation::Collections::IVector<HttpChallengeHeaderValue> {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpChallengeHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&HttpChallengeHeaderValueCollection> for super::super::super::Foundation::Collections::IVector<HttpChallengeHeaderValue> {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpChallengeHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&HttpChallengeHeaderValueCollection> for ::windows::core::InParam<super::super::super::Foundation::Collections::IVector<HttpChallengeHeaderValue>> {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpChallengeHeaderValueCollection) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for HttpChallengeHeaderValueCollection {}
unsafe impl ::core::marker::Sync for HttpChallengeHeaderValueCollection {}
#[doc = "*Required features: `\"Web_Http_Headers\"`*"]
#[repr(transparent)]
pub struct HttpConnectionOptionHeaderValue(::windows::core::IUnknown);
impl HttpConnectionOptionHeaderValue {
    pub fn Token(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Token)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Create(token: &::windows::core::HSTRING) -> ::windows::core::Result<HttpConnectionOptionHeaderValue> {
        Self::IHttpConnectionOptionHeaderValueFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Create)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(token), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn Parse(input: &::windows::core::HSTRING) -> ::windows::core::Result<HttpConnectionOptionHeaderValue> {
        Self::IHttpConnectionOptionHeaderValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Parse)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(input), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn TryParse(input: &::windows::core::HSTRING, connectionoptionheadervalue: &mut ::core::option::Option<HttpConnectionOptionHeaderValue>) -> ::windows::core::Result<bool> {
        Self::IHttpConnectionOptionHeaderValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TryParse)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(input), connectionoptionheadervalue as *mut _ as _, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ToString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ToString)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc(hidden)]
    pub fn IHttpConnectionOptionHeaderValueFactory<R, F: FnOnce(&IHttpConnectionOptionHeaderValueFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<HttpConnectionOptionHeaderValue, IHttpConnectionOptionHeaderValueFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IHttpConnectionOptionHeaderValueStatics<R, F: FnOnce(&IHttpConnectionOptionHeaderValueStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<HttpConnectionOptionHeaderValue, IHttpConnectionOptionHeaderValueStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for HttpConnectionOptionHeaderValue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HttpConnectionOptionHeaderValue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpConnectionOptionHeaderValue {}
impl ::core::fmt::Debug for HttpConnectionOptionHeaderValue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HttpConnectionOptionHeaderValue").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for HttpConnectionOptionHeaderValue {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Headers.HttpConnectionOptionHeaderValue;{cb4af27a-4e90-45eb-8dcd-fd1408f4c44f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for HttpConnectionOptionHeaderValue {
    type Vtable = IHttpConnectionOptionHeaderValue_Vtbl;
}
unsafe impl ::windows::core::Interface for HttpConnectionOptionHeaderValue {
    const IID: ::windows::core::GUID = <IHttpConnectionOptionHeaderValue as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for HttpConnectionOptionHeaderValue {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpConnectionOptionHeaderValue";
}
::windows::core::interface_hierarchy!(HttpConnectionOptionHeaderValue, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<HttpConnectionOptionHeaderValue> for super::super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpConnectionOptionHeaderValue) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&HttpConnectionOptionHeaderValue> for super::super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpConnectionOptionHeaderValue) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&HttpConnectionOptionHeaderValue> for ::windows::core::InParam<super::super::super::Foundation::IStringable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpConnectionOptionHeaderValue) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for HttpConnectionOptionHeaderValue {}
unsafe impl ::core::marker::Sync for HttpConnectionOptionHeaderValue {}
#[doc = "*Required features: `\"Web_Http_Headers\"`*"]
#[repr(transparent)]
pub struct HttpConnectionOptionHeaderValueCollection(::windows::core::IUnknown);
impl HttpConnectionOptionHeaderValueCollection {
    pub fn ParseAdd(&self, input: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).ParseAdd)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(input)).ok() }
    }
    pub fn TryParseAdd(&self, input: &::windows::core::HSTRING) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TryParseAdd)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(input), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IIterator<HttpConnectionOptionHeaderValue>> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IIterable<HttpConnectionOptionHeaderValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).First)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ToString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ToString)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAt(&self, index: u32) -> ::windows::core::Result<HttpConnectionOptionHeaderValue> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpConnectionOptionHeaderValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetAt)(::windows::core::Vtable::as_raw(this), index, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpConnectionOptionHeaderValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Size)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetView(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<HttpConnectionOptionHeaderValue>> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpConnectionOptionHeaderValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetView)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn IndexOf(&self, value: &HttpConnectionOptionHeaderValue, index: &mut u32) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpConnectionOptionHeaderValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IndexOf)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value), index, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetAt(&self, index: u32, value: &HttpConnectionOptionHeaderValue) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpConnectionOptionHeaderValue>>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetAt)(::windows::core::Vtable::as_raw(this), index, ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn InsertAt(&self, index: u32, value: &HttpConnectionOptionHeaderValue) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpConnectionOptionHeaderValue>>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).InsertAt)(::windows::core::Vtable::as_raw(this), index, ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAt(&self, index: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpConnectionOptionHeaderValue>>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveAt)(::windows::core::Vtable::as_raw(this), index).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Append(&self, value: &HttpConnectionOptionHeaderValue) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpConnectionOptionHeaderValue>>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Append)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAtEnd(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpConnectionOptionHeaderValue>>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveAtEnd)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Clear(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpConnectionOptionHeaderValue>>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Clear)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMany(&self, startindex: u32, items: &mut [::core::option::Option<HttpConnectionOptionHeaderValue>]) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpConnectionOptionHeaderValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetMany)(::windows::core::Vtable::as_raw(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ReplaceAll(&self, items: &[::core::option::Option<HttpConnectionOptionHeaderValue>]) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpConnectionOptionHeaderValue>>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).ReplaceAll)(::windows::core::Vtable::as_raw(this), items.len() as u32, ::core::mem::transmute(items.as_ptr())).ok() }
    }
}
impl ::core::clone::Clone for HttpConnectionOptionHeaderValueCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HttpConnectionOptionHeaderValueCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpConnectionOptionHeaderValueCollection {}
impl ::core::fmt::Debug for HttpConnectionOptionHeaderValueCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HttpConnectionOptionHeaderValueCollection").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for HttpConnectionOptionHeaderValueCollection {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Headers.HttpConnectionOptionHeaderValueCollection;{e4f56c1d-5142-4e00-8e0f-019509337629})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for HttpConnectionOptionHeaderValueCollection {
    type Vtable = IHttpConnectionOptionHeaderValueCollection_Vtbl;
}
unsafe impl ::windows::core::Interface for HttpConnectionOptionHeaderValueCollection {
    const IID: ::windows::core::GUID = <IHttpConnectionOptionHeaderValueCollection as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for HttpConnectionOptionHeaderValueCollection {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpConnectionOptionHeaderValueCollection";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for HttpConnectionOptionHeaderValueCollection {
    type Item = HttpConnectionOptionHeaderValue;
    type IntoIter = super::super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for &HttpConnectionOptionHeaderValueCollection {
    type Item = HttpConnectionOptionHeaderValue;
    type IntoIter = super::super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        super::super::super::Foundation::Collections::VectorIterator::new(::core::convert::TryInto::try_into(self).ok())
    }
}
::windows::core::interface_hierarchy!(HttpConnectionOptionHeaderValueCollection, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<HttpConnectionOptionHeaderValueCollection> for super::super::super::Foundation::Collections::IIterable<HttpConnectionOptionHeaderValue> {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpConnectionOptionHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&HttpConnectionOptionHeaderValueCollection> for super::super::super::Foundation::Collections::IIterable<HttpConnectionOptionHeaderValue> {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpConnectionOptionHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&HttpConnectionOptionHeaderValueCollection> for ::windows::core::InParam<super::super::super::Foundation::Collections::IIterable<HttpConnectionOptionHeaderValue>> {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpConnectionOptionHeaderValueCollection) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<HttpConnectionOptionHeaderValueCollection> for super::super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpConnectionOptionHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&HttpConnectionOptionHeaderValueCollection> for super::super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpConnectionOptionHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&HttpConnectionOptionHeaderValueCollection> for ::windows::core::InParam<super::super::super::Foundation::IStringable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpConnectionOptionHeaderValueCollection) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<HttpConnectionOptionHeaderValueCollection> for super::super::super::Foundation::Collections::IVector<HttpConnectionOptionHeaderValue> {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpConnectionOptionHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&HttpConnectionOptionHeaderValueCollection> for super::super::super::Foundation::Collections::IVector<HttpConnectionOptionHeaderValue> {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpConnectionOptionHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&HttpConnectionOptionHeaderValueCollection> for ::windows::core::InParam<super::super::super::Foundation::Collections::IVector<HttpConnectionOptionHeaderValue>> {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpConnectionOptionHeaderValueCollection) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for HttpConnectionOptionHeaderValueCollection {}
unsafe impl ::core::marker::Sync for HttpConnectionOptionHeaderValueCollection {}
#[doc = "*Required features: `\"Web_Http_Headers\"`*"]
#[repr(transparent)]
pub struct HttpContentCodingHeaderValue(::windows::core::IUnknown);
impl HttpContentCodingHeaderValue {
    pub fn ContentCoding(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ContentCoding)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Create(contentcoding: &::windows::core::HSTRING) -> ::windows::core::Result<HttpContentCodingHeaderValue> {
        Self::IHttpContentCodingHeaderValueFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Create)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(contentcoding), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn Parse(input: &::windows::core::HSTRING) -> ::windows::core::Result<HttpContentCodingHeaderValue> {
        Self::IHttpContentCodingHeaderValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Parse)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(input), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn TryParse(input: &::windows::core::HSTRING, contentcodingheadervalue: &mut ::core::option::Option<HttpContentCodingHeaderValue>) -> ::windows::core::Result<bool> {
        Self::IHttpContentCodingHeaderValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TryParse)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(input), contentcodingheadervalue as *mut _ as _, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ToString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ToString)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc(hidden)]
    pub fn IHttpContentCodingHeaderValueFactory<R, F: FnOnce(&IHttpContentCodingHeaderValueFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<HttpContentCodingHeaderValue, IHttpContentCodingHeaderValueFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IHttpContentCodingHeaderValueStatics<R, F: FnOnce(&IHttpContentCodingHeaderValueStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<HttpContentCodingHeaderValue, IHttpContentCodingHeaderValueStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for HttpContentCodingHeaderValue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HttpContentCodingHeaderValue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpContentCodingHeaderValue {}
impl ::core::fmt::Debug for HttpContentCodingHeaderValue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HttpContentCodingHeaderValue").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for HttpContentCodingHeaderValue {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Headers.HttpContentCodingHeaderValue;{bcf7f92a-9376-4d85-bccc-9f4f9acab434})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for HttpContentCodingHeaderValue {
    type Vtable = IHttpContentCodingHeaderValue_Vtbl;
}
unsafe impl ::windows::core::Interface for HttpContentCodingHeaderValue {
    const IID: ::windows::core::GUID = <IHttpContentCodingHeaderValue as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for HttpContentCodingHeaderValue {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpContentCodingHeaderValue";
}
::windows::core::interface_hierarchy!(HttpContentCodingHeaderValue, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<HttpContentCodingHeaderValue> for super::super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpContentCodingHeaderValue) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&HttpContentCodingHeaderValue> for super::super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpContentCodingHeaderValue) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&HttpContentCodingHeaderValue> for ::windows::core::InParam<super::super::super::Foundation::IStringable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpContentCodingHeaderValue) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for HttpContentCodingHeaderValue {}
unsafe impl ::core::marker::Sync for HttpContentCodingHeaderValue {}
#[doc = "*Required features: `\"Web_Http_Headers\"`*"]
#[repr(transparent)]
pub struct HttpContentCodingHeaderValueCollection(::windows::core::IUnknown);
impl HttpContentCodingHeaderValueCollection {
    pub fn ParseAdd(&self, input: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).ParseAdd)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(input)).ok() }
    }
    pub fn TryParseAdd(&self, input: &::windows::core::HSTRING) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TryParseAdd)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(input), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IIterator<HttpContentCodingHeaderValue>> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IIterable<HttpContentCodingHeaderValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).First)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ToString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ToString)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAt(&self, index: u32) -> ::windows::core::Result<HttpContentCodingHeaderValue> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpContentCodingHeaderValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetAt)(::windows::core::Vtable::as_raw(this), index, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpContentCodingHeaderValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Size)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetView(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<HttpContentCodingHeaderValue>> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpContentCodingHeaderValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetView)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn IndexOf(&self, value: &HttpContentCodingHeaderValue, index: &mut u32) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpContentCodingHeaderValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IndexOf)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value), index, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetAt(&self, index: u32, value: &HttpContentCodingHeaderValue) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpContentCodingHeaderValue>>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetAt)(::windows::core::Vtable::as_raw(this), index, ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn InsertAt(&self, index: u32, value: &HttpContentCodingHeaderValue) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpContentCodingHeaderValue>>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).InsertAt)(::windows::core::Vtable::as_raw(this), index, ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAt(&self, index: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpContentCodingHeaderValue>>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveAt)(::windows::core::Vtable::as_raw(this), index).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Append(&self, value: &HttpContentCodingHeaderValue) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpContentCodingHeaderValue>>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Append)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAtEnd(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpContentCodingHeaderValue>>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveAtEnd)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Clear(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpContentCodingHeaderValue>>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Clear)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMany(&self, startindex: u32, items: &mut [::core::option::Option<HttpContentCodingHeaderValue>]) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpContentCodingHeaderValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetMany)(::windows::core::Vtable::as_raw(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ReplaceAll(&self, items: &[::core::option::Option<HttpContentCodingHeaderValue>]) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpContentCodingHeaderValue>>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).ReplaceAll)(::windows::core::Vtable::as_raw(this), items.len() as u32, ::core::mem::transmute(items.as_ptr())).ok() }
    }
}
impl ::core::clone::Clone for HttpContentCodingHeaderValueCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HttpContentCodingHeaderValueCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpContentCodingHeaderValueCollection {}
impl ::core::fmt::Debug for HttpContentCodingHeaderValueCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HttpContentCodingHeaderValueCollection").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for HttpContentCodingHeaderValueCollection {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Headers.HttpContentCodingHeaderValueCollection;{7d221721-a6db-436e-8e83-91596192819c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for HttpContentCodingHeaderValueCollection {
    type Vtable = IHttpContentCodingHeaderValueCollection_Vtbl;
}
unsafe impl ::windows::core::Interface for HttpContentCodingHeaderValueCollection {
    const IID: ::windows::core::GUID = <IHttpContentCodingHeaderValueCollection as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for HttpContentCodingHeaderValueCollection {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpContentCodingHeaderValueCollection";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for HttpContentCodingHeaderValueCollection {
    type Item = HttpContentCodingHeaderValue;
    type IntoIter = super::super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for &HttpContentCodingHeaderValueCollection {
    type Item = HttpContentCodingHeaderValue;
    type IntoIter = super::super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        super::super::super::Foundation::Collections::VectorIterator::new(::core::convert::TryInto::try_into(self).ok())
    }
}
::windows::core::interface_hierarchy!(HttpContentCodingHeaderValueCollection, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<HttpContentCodingHeaderValueCollection> for super::super::super::Foundation::Collections::IIterable<HttpContentCodingHeaderValue> {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpContentCodingHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&HttpContentCodingHeaderValueCollection> for super::super::super::Foundation::Collections::IIterable<HttpContentCodingHeaderValue> {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpContentCodingHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&HttpContentCodingHeaderValueCollection> for ::windows::core::InParam<super::super::super::Foundation::Collections::IIterable<HttpContentCodingHeaderValue>> {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpContentCodingHeaderValueCollection) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<HttpContentCodingHeaderValueCollection> for super::super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpContentCodingHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&HttpContentCodingHeaderValueCollection> for super::super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpContentCodingHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&HttpContentCodingHeaderValueCollection> for ::windows::core::InParam<super::super::super::Foundation::IStringable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpContentCodingHeaderValueCollection) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<HttpContentCodingHeaderValueCollection> for super::super::super::Foundation::Collections::IVector<HttpContentCodingHeaderValue> {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpContentCodingHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&HttpContentCodingHeaderValueCollection> for super::super::super::Foundation::Collections::IVector<HttpContentCodingHeaderValue> {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpContentCodingHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&HttpContentCodingHeaderValueCollection> for ::windows::core::InParam<super::super::super::Foundation::Collections::IVector<HttpContentCodingHeaderValue>> {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpContentCodingHeaderValueCollection) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for HttpContentCodingHeaderValueCollection {}
unsafe impl ::core::marker::Sync for HttpContentCodingHeaderValueCollection {}
#[doc = "*Required features: `\"Web_Http_Headers\"`*"]
#[repr(transparent)]
pub struct HttpContentCodingWithQualityHeaderValue(::windows::core::IUnknown);
impl HttpContentCodingWithQualityHeaderValue {
    pub fn ContentCoding(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ContentCoding)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Quality(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Quality)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CreateFromValue(contentcoding: &::windows::core::HSTRING) -> ::windows::core::Result<HttpContentCodingWithQualityHeaderValue> {
        Self::IHttpContentCodingWithQualityHeaderValueFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateFromValue)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(contentcoding), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn CreateFromValueWithQuality(contentcoding: &::windows::core::HSTRING, quality: f64) -> ::windows::core::Result<HttpContentCodingWithQualityHeaderValue> {
        Self::IHttpContentCodingWithQualityHeaderValueFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateFromValueWithQuality)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(contentcoding), quality, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn Parse(input: &::windows::core::HSTRING) -> ::windows::core::Result<HttpContentCodingWithQualityHeaderValue> {
        Self::IHttpContentCodingWithQualityHeaderValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Parse)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(input), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn TryParse(input: &::windows::core::HSTRING, contentcodingwithqualityheadervalue: &mut ::core::option::Option<HttpContentCodingWithQualityHeaderValue>) -> ::windows::core::Result<bool> {
        Self::IHttpContentCodingWithQualityHeaderValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TryParse)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(input), contentcodingwithqualityheadervalue as *mut _ as _, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ToString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ToString)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc(hidden)]
    pub fn IHttpContentCodingWithQualityHeaderValueFactory<R, F: FnOnce(&IHttpContentCodingWithQualityHeaderValueFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<HttpContentCodingWithQualityHeaderValue, IHttpContentCodingWithQualityHeaderValueFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IHttpContentCodingWithQualityHeaderValueStatics<R, F: FnOnce(&IHttpContentCodingWithQualityHeaderValueStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<HttpContentCodingWithQualityHeaderValue, IHttpContentCodingWithQualityHeaderValueStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for HttpContentCodingWithQualityHeaderValue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HttpContentCodingWithQualityHeaderValue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpContentCodingWithQualityHeaderValue {}
impl ::core::fmt::Debug for HttpContentCodingWithQualityHeaderValue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HttpContentCodingWithQualityHeaderValue").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for HttpContentCodingWithQualityHeaderValue {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Headers.HttpContentCodingWithQualityHeaderValue;{94531cd5-8b13-4d73-8651-f76b38f88495})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for HttpContentCodingWithQualityHeaderValue {
    type Vtable = IHttpContentCodingWithQualityHeaderValue_Vtbl;
}
unsafe impl ::windows::core::Interface for HttpContentCodingWithQualityHeaderValue {
    const IID: ::windows::core::GUID = <IHttpContentCodingWithQualityHeaderValue as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for HttpContentCodingWithQualityHeaderValue {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpContentCodingWithQualityHeaderValue";
}
::windows::core::interface_hierarchy!(HttpContentCodingWithQualityHeaderValue, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<HttpContentCodingWithQualityHeaderValue> for super::super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpContentCodingWithQualityHeaderValue) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&HttpContentCodingWithQualityHeaderValue> for super::super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpContentCodingWithQualityHeaderValue) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&HttpContentCodingWithQualityHeaderValue> for ::windows::core::InParam<super::super::super::Foundation::IStringable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpContentCodingWithQualityHeaderValue) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for HttpContentCodingWithQualityHeaderValue {}
unsafe impl ::core::marker::Sync for HttpContentCodingWithQualityHeaderValue {}
#[doc = "*Required features: `\"Web_Http_Headers\"`*"]
#[repr(transparent)]
pub struct HttpContentCodingWithQualityHeaderValueCollection(::windows::core::IUnknown);
impl HttpContentCodingWithQualityHeaderValueCollection {
    pub fn ParseAdd(&self, input: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).ParseAdd)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(input)).ok() }
    }
    pub fn TryParseAdd(&self, input: &::windows::core::HSTRING) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TryParseAdd)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(input), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IIterator<HttpContentCodingWithQualityHeaderValue>> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IIterable<HttpContentCodingWithQualityHeaderValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).First)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ToString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ToString)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAt(&self, index: u32) -> ::windows::core::Result<HttpContentCodingWithQualityHeaderValue> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpContentCodingWithQualityHeaderValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetAt)(::windows::core::Vtable::as_raw(this), index, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpContentCodingWithQualityHeaderValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Size)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetView(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<HttpContentCodingWithQualityHeaderValue>> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpContentCodingWithQualityHeaderValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetView)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn IndexOf(&self, value: &HttpContentCodingWithQualityHeaderValue, index: &mut u32) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpContentCodingWithQualityHeaderValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IndexOf)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value), index, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetAt(&self, index: u32, value: &HttpContentCodingWithQualityHeaderValue) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpContentCodingWithQualityHeaderValue>>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetAt)(::windows::core::Vtable::as_raw(this), index, ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn InsertAt(&self, index: u32, value: &HttpContentCodingWithQualityHeaderValue) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpContentCodingWithQualityHeaderValue>>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).InsertAt)(::windows::core::Vtable::as_raw(this), index, ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAt(&self, index: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpContentCodingWithQualityHeaderValue>>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveAt)(::windows::core::Vtable::as_raw(this), index).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Append(&self, value: &HttpContentCodingWithQualityHeaderValue) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpContentCodingWithQualityHeaderValue>>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Append)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAtEnd(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpContentCodingWithQualityHeaderValue>>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveAtEnd)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Clear(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpContentCodingWithQualityHeaderValue>>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Clear)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMany(&self, startindex: u32, items: &mut [::core::option::Option<HttpContentCodingWithQualityHeaderValue>]) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpContentCodingWithQualityHeaderValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetMany)(::windows::core::Vtable::as_raw(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ReplaceAll(&self, items: &[::core::option::Option<HttpContentCodingWithQualityHeaderValue>]) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpContentCodingWithQualityHeaderValue>>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).ReplaceAll)(::windows::core::Vtable::as_raw(this), items.len() as u32, ::core::mem::transmute(items.as_ptr())).ok() }
    }
}
impl ::core::clone::Clone for HttpContentCodingWithQualityHeaderValueCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HttpContentCodingWithQualityHeaderValueCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpContentCodingWithQualityHeaderValueCollection {}
impl ::core::fmt::Debug for HttpContentCodingWithQualityHeaderValueCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HttpContentCodingWithQualityHeaderValueCollection").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for HttpContentCodingWithQualityHeaderValueCollection {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Headers.HttpContentCodingWithQualityHeaderValueCollection;{7c0d753e-e899-4378-b5c8-412d820711cc})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for HttpContentCodingWithQualityHeaderValueCollection {
    type Vtable = IHttpContentCodingWithQualityHeaderValueCollection_Vtbl;
}
unsafe impl ::windows::core::Interface for HttpContentCodingWithQualityHeaderValueCollection {
    const IID: ::windows::core::GUID = <IHttpContentCodingWithQualityHeaderValueCollection as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for HttpContentCodingWithQualityHeaderValueCollection {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpContentCodingWithQualityHeaderValueCollection";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for HttpContentCodingWithQualityHeaderValueCollection {
    type Item = HttpContentCodingWithQualityHeaderValue;
    type IntoIter = super::super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for &HttpContentCodingWithQualityHeaderValueCollection {
    type Item = HttpContentCodingWithQualityHeaderValue;
    type IntoIter = super::super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        super::super::super::Foundation::Collections::VectorIterator::new(::core::convert::TryInto::try_into(self).ok())
    }
}
::windows::core::interface_hierarchy!(HttpContentCodingWithQualityHeaderValueCollection, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<HttpContentCodingWithQualityHeaderValueCollection> for super::super::super::Foundation::Collections::IIterable<HttpContentCodingWithQualityHeaderValue> {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpContentCodingWithQualityHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&HttpContentCodingWithQualityHeaderValueCollection> for super::super::super::Foundation::Collections::IIterable<HttpContentCodingWithQualityHeaderValue> {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpContentCodingWithQualityHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&HttpContentCodingWithQualityHeaderValueCollection> for ::windows::core::InParam<super::super::super::Foundation::Collections::IIterable<HttpContentCodingWithQualityHeaderValue>> {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpContentCodingWithQualityHeaderValueCollection) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<HttpContentCodingWithQualityHeaderValueCollection> for super::super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpContentCodingWithQualityHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&HttpContentCodingWithQualityHeaderValueCollection> for super::super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpContentCodingWithQualityHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&HttpContentCodingWithQualityHeaderValueCollection> for ::windows::core::InParam<super::super::super::Foundation::IStringable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpContentCodingWithQualityHeaderValueCollection) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<HttpContentCodingWithQualityHeaderValueCollection> for super::super::super::Foundation::Collections::IVector<HttpContentCodingWithQualityHeaderValue> {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpContentCodingWithQualityHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&HttpContentCodingWithQualityHeaderValueCollection> for super::super::super::Foundation::Collections::IVector<HttpContentCodingWithQualityHeaderValue> {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpContentCodingWithQualityHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&HttpContentCodingWithQualityHeaderValueCollection> for ::windows::core::InParam<super::super::super::Foundation::Collections::IVector<HttpContentCodingWithQualityHeaderValue>> {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpContentCodingWithQualityHeaderValueCollection) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for HttpContentCodingWithQualityHeaderValueCollection {}
unsafe impl ::core::marker::Sync for HttpContentCodingWithQualityHeaderValueCollection {}
#[doc = "*Required features: `\"Web_Http_Headers\"`*"]
#[repr(transparent)]
pub struct HttpContentDispositionHeaderValue(::windows::core::IUnknown);
impl HttpContentDispositionHeaderValue {
    pub fn DispositionType(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DispositionType)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetDispositionType(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetDispositionType)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn FileName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FileName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetFileName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetFileName)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn FileNameStar(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FileNameStar)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetFileNameStar(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetFileNameStar)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Name)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetName)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Parameters(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<HttpNameValueHeaderValue>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Parameters)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Size(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<u64>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Size)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetSize<P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::super::Foundation::IReference<u64>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetSize)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    pub fn Create(dispositiontype: &::windows::core::HSTRING) -> ::windows::core::Result<HttpContentDispositionHeaderValue> {
        Self::IHttpContentDispositionHeaderValueFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Create)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(dispositiontype), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn Parse(input: &::windows::core::HSTRING) -> ::windows::core::Result<HttpContentDispositionHeaderValue> {
        Self::IHttpContentDispositionHeaderValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Parse)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(input), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn TryParse(input: &::windows::core::HSTRING, contentdispositionheadervalue: &mut ::core::option::Option<HttpContentDispositionHeaderValue>) -> ::windows::core::Result<bool> {
        Self::IHttpContentDispositionHeaderValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TryParse)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(input), contentdispositionheadervalue as *mut _ as _, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ToString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ToString)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc(hidden)]
    pub fn IHttpContentDispositionHeaderValueFactory<R, F: FnOnce(&IHttpContentDispositionHeaderValueFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<HttpContentDispositionHeaderValue, IHttpContentDispositionHeaderValueFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IHttpContentDispositionHeaderValueStatics<R, F: FnOnce(&IHttpContentDispositionHeaderValueStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<HttpContentDispositionHeaderValue, IHttpContentDispositionHeaderValueStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for HttpContentDispositionHeaderValue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HttpContentDispositionHeaderValue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpContentDispositionHeaderValue {}
impl ::core::fmt::Debug for HttpContentDispositionHeaderValue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HttpContentDispositionHeaderValue").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for HttpContentDispositionHeaderValue {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Headers.HttpContentDispositionHeaderValue;{f2a2eedc-2629-4b49-9908-96a168e9365e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for HttpContentDispositionHeaderValue {
    type Vtable = IHttpContentDispositionHeaderValue_Vtbl;
}
unsafe impl ::windows::core::Interface for HttpContentDispositionHeaderValue {
    const IID: ::windows::core::GUID = <IHttpContentDispositionHeaderValue as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for HttpContentDispositionHeaderValue {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpContentDispositionHeaderValue";
}
::windows::core::interface_hierarchy!(HttpContentDispositionHeaderValue, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<HttpContentDispositionHeaderValue> for super::super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpContentDispositionHeaderValue) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&HttpContentDispositionHeaderValue> for super::super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpContentDispositionHeaderValue) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&HttpContentDispositionHeaderValue> for ::windows::core::InParam<super::super::super::Foundation::IStringable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpContentDispositionHeaderValue) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for HttpContentDispositionHeaderValue {}
unsafe impl ::core::marker::Sync for HttpContentDispositionHeaderValue {}
#[doc = "*Required features: `\"Web_Http_Headers\"`*"]
#[repr(transparent)]
pub struct HttpContentHeaderCollection(::windows::core::IUnknown);
impl HttpContentHeaderCollection {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<HttpContentHeaderCollection, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn ContentDisposition(&self) -> ::windows::core::Result<HttpContentDispositionHeaderValue> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ContentDisposition)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetContentDisposition(&self, value: &HttpContentDispositionHeaderValue) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetContentDisposition)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn ContentEncoding(&self) -> ::windows::core::Result<HttpContentCodingHeaderValueCollection> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ContentEncoding)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ContentLanguage(&self) -> ::windows::core::Result<HttpLanguageHeaderValueCollection> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ContentLanguage)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ContentLength(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<u64>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ContentLength)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetContentLength<P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::super::Foundation::IReference<u64>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetContentLength)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ContentLocation(&self) -> ::windows::core::Result<super::super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ContentLocation)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetContentLocation(&self, value: &super::super::super::Foundation::Uri) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetContentLocation)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn ContentMD5(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ContentMD5)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn SetContentMD5<P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::super::Storage::Streams::IBuffer>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetContentMD5)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    pub fn ContentRange(&self) -> ::windows::core::Result<HttpContentRangeHeaderValue> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ContentRange)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetContentRange(&self, value: &HttpContentRangeHeaderValue) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetContentRange)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn ContentType(&self) -> ::windows::core::Result<HttpMediaTypeHeaderValue> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ContentType)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetContentType(&self, value: &HttpMediaTypeHeaderValue) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetContentType)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Expires(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Expires)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetExpires<P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetExpires)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LastModified(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LastModified)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetLastModified<P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetLastModified)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    pub fn Append(&self, name: &::windows::core::HSTRING, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Append)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(name), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn TryAppendWithoutValidation(&self, name: &::windows::core::HSTRING, value: &::windows::core::HSTRING) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TryAppendWithoutValidation)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(name), ::core::mem::transmute_copy(value), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IIterator<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::HSTRING>>> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::HSTRING>>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).First)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Lookup(&self, key: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Lookup)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(key), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Size)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn HasKey(&self, key: &::windows::core::HSTRING) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).HasKey)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(key), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetView(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::HSTRING>> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetView)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Insert(&self, key: &::windows::core::HSTRING, value: &::windows::core::HSTRING) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Insert)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(key), ::core::mem::transmute_copy(value), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Remove(&self, key: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Remove)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(key)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Clear(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Clear)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ToString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ToString)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for HttpContentHeaderCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HttpContentHeaderCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpContentHeaderCollection {}
impl ::core::fmt::Debug for HttpContentHeaderCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HttpContentHeaderCollection").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for HttpContentHeaderCollection {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Headers.HttpContentHeaderCollection;{40612a44-47ae-4b7e-9124-69628b64aa18})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for HttpContentHeaderCollection {
    type Vtable = IHttpContentHeaderCollection_Vtbl;
}
unsafe impl ::windows::core::Interface for HttpContentHeaderCollection {
    const IID: ::windows::core::GUID = <IHttpContentHeaderCollection as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for HttpContentHeaderCollection {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpContentHeaderCollection";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for HttpContentHeaderCollection {
    type Item = super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::HSTRING>;
    type IntoIter = super::super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for &HttpContentHeaderCollection {
    type Item = super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::HSTRING>;
    type IntoIter = super::super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
::windows::core::interface_hierarchy!(HttpContentHeaderCollection, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<HttpContentHeaderCollection> for super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::HSTRING>> {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpContentHeaderCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&HttpContentHeaderCollection> for super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::HSTRING>> {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpContentHeaderCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&HttpContentHeaderCollection> for ::windows::core::InParam<super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::HSTRING>>> {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpContentHeaderCollection) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<HttpContentHeaderCollection> for super::super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING> {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpContentHeaderCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&HttpContentHeaderCollection> for super::super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING> {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpContentHeaderCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&HttpContentHeaderCollection> for ::windows::core::InParam<super::super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>> {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpContentHeaderCollection) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<HttpContentHeaderCollection> for super::super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpContentHeaderCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&HttpContentHeaderCollection> for super::super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpContentHeaderCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&HttpContentHeaderCollection> for ::windows::core::InParam<super::super::super::Foundation::IStringable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpContentHeaderCollection) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for HttpContentHeaderCollection {}
unsafe impl ::core::marker::Sync for HttpContentHeaderCollection {}
#[doc = "*Required features: `\"Web_Http_Headers\"`*"]
#[repr(transparent)]
pub struct HttpContentRangeHeaderValue(::windows::core::IUnknown);
impl HttpContentRangeHeaderValue {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FirstBytePosition(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<u64>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FirstBytePosition)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LastBytePosition(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<u64>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LastBytePosition)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Length(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<u64>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Length)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Unit(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Unit)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetUnit(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetUnit)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn CreateFromLength(length: u64) -> ::windows::core::Result<HttpContentRangeHeaderValue> {
        Self::IHttpContentRangeHeaderValueFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateFromLength)(::windows::core::Vtable::as_raw(this), length, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn CreateFromRange(from: u64, to: u64) -> ::windows::core::Result<HttpContentRangeHeaderValue> {
        Self::IHttpContentRangeHeaderValueFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateFromRange)(::windows::core::Vtable::as_raw(this), from, to, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn CreateFromRangeWithLength(from: u64, to: u64, length: u64) -> ::windows::core::Result<HttpContentRangeHeaderValue> {
        Self::IHttpContentRangeHeaderValueFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateFromRangeWithLength)(::windows::core::Vtable::as_raw(this), from, to, length, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn Parse(input: &::windows::core::HSTRING) -> ::windows::core::Result<HttpContentRangeHeaderValue> {
        Self::IHttpContentRangeHeaderValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Parse)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(input), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn TryParse(input: &::windows::core::HSTRING, contentrangeheadervalue: &mut ::core::option::Option<HttpContentRangeHeaderValue>) -> ::windows::core::Result<bool> {
        Self::IHttpContentRangeHeaderValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TryParse)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(input), contentrangeheadervalue as *mut _ as _, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ToString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ToString)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc(hidden)]
    pub fn IHttpContentRangeHeaderValueFactory<R, F: FnOnce(&IHttpContentRangeHeaderValueFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<HttpContentRangeHeaderValue, IHttpContentRangeHeaderValueFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IHttpContentRangeHeaderValueStatics<R, F: FnOnce(&IHttpContentRangeHeaderValueStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<HttpContentRangeHeaderValue, IHttpContentRangeHeaderValueStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for HttpContentRangeHeaderValue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HttpContentRangeHeaderValue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpContentRangeHeaderValue {}
impl ::core::fmt::Debug for HttpContentRangeHeaderValue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HttpContentRangeHeaderValue").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for HttpContentRangeHeaderValue {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Headers.HttpContentRangeHeaderValue;{04d967d3-a4f6-495c-9530-8579fcba8aa9})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for HttpContentRangeHeaderValue {
    type Vtable = IHttpContentRangeHeaderValue_Vtbl;
}
unsafe impl ::windows::core::Interface for HttpContentRangeHeaderValue {
    const IID: ::windows::core::GUID = <IHttpContentRangeHeaderValue as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for HttpContentRangeHeaderValue {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpContentRangeHeaderValue";
}
::windows::core::interface_hierarchy!(HttpContentRangeHeaderValue, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<HttpContentRangeHeaderValue> for super::super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpContentRangeHeaderValue) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&HttpContentRangeHeaderValue> for super::super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpContentRangeHeaderValue) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&HttpContentRangeHeaderValue> for ::windows::core::InParam<super::super::super::Foundation::IStringable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpContentRangeHeaderValue) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for HttpContentRangeHeaderValue {}
unsafe impl ::core::marker::Sync for HttpContentRangeHeaderValue {}
#[doc = "*Required features: `\"Web_Http_Headers\"`*"]
#[repr(transparent)]
pub struct HttpCookiePairHeaderValue(::windows::core::IUnknown);
impl HttpCookiePairHeaderValue {
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Name)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Value(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Value)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetValue(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetValue)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn CreateFromName(name: &::windows::core::HSTRING) -> ::windows::core::Result<HttpCookiePairHeaderValue> {
        Self::IHttpCookiePairHeaderValueFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateFromName)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(name), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn CreateFromNameWithValue(name: &::windows::core::HSTRING, value: &::windows::core::HSTRING) -> ::windows::core::Result<HttpCookiePairHeaderValue> {
        Self::IHttpCookiePairHeaderValueFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateFromNameWithValue)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(name), ::core::mem::transmute_copy(value), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn Parse(input: &::windows::core::HSTRING) -> ::windows::core::Result<HttpCookiePairHeaderValue> {
        Self::IHttpCookiePairHeaderValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Parse)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(input), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn TryParse(input: &::windows::core::HSTRING, cookiepairheadervalue: &mut ::core::option::Option<HttpCookiePairHeaderValue>) -> ::windows::core::Result<bool> {
        Self::IHttpCookiePairHeaderValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TryParse)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(input), cookiepairheadervalue as *mut _ as _, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ToString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ToString)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc(hidden)]
    pub fn IHttpCookiePairHeaderValueFactory<R, F: FnOnce(&IHttpCookiePairHeaderValueFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<HttpCookiePairHeaderValue, IHttpCookiePairHeaderValueFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IHttpCookiePairHeaderValueStatics<R, F: FnOnce(&IHttpCookiePairHeaderValueStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<HttpCookiePairHeaderValue, IHttpCookiePairHeaderValueStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for HttpCookiePairHeaderValue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HttpCookiePairHeaderValue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpCookiePairHeaderValue {}
impl ::core::fmt::Debug for HttpCookiePairHeaderValue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HttpCookiePairHeaderValue").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for HttpCookiePairHeaderValue {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Headers.HttpCookiePairHeaderValue;{cbd46217-4b29-412b-bd90-b3d814ab8e1b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for HttpCookiePairHeaderValue {
    type Vtable = IHttpCookiePairHeaderValue_Vtbl;
}
unsafe impl ::windows::core::Interface for HttpCookiePairHeaderValue {
    const IID: ::windows::core::GUID = <IHttpCookiePairHeaderValue as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for HttpCookiePairHeaderValue {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpCookiePairHeaderValue";
}
::windows::core::interface_hierarchy!(HttpCookiePairHeaderValue, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<HttpCookiePairHeaderValue> for super::super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpCookiePairHeaderValue) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&HttpCookiePairHeaderValue> for super::super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpCookiePairHeaderValue) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&HttpCookiePairHeaderValue> for ::windows::core::InParam<super::super::super::Foundation::IStringable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpCookiePairHeaderValue) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for HttpCookiePairHeaderValue {}
unsafe impl ::core::marker::Sync for HttpCookiePairHeaderValue {}
#[doc = "*Required features: `\"Web_Http_Headers\"`*"]
#[repr(transparent)]
pub struct HttpCookiePairHeaderValueCollection(::windows::core::IUnknown);
impl HttpCookiePairHeaderValueCollection {
    pub fn ParseAdd(&self, input: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).ParseAdd)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(input)).ok() }
    }
    pub fn TryParseAdd(&self, input: &::windows::core::HSTRING) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TryParseAdd)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(input), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IIterator<HttpCookiePairHeaderValue>> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IIterable<HttpCookiePairHeaderValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).First)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ToString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ToString)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAt(&self, index: u32) -> ::windows::core::Result<HttpCookiePairHeaderValue> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpCookiePairHeaderValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetAt)(::windows::core::Vtable::as_raw(this), index, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpCookiePairHeaderValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Size)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetView(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<HttpCookiePairHeaderValue>> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpCookiePairHeaderValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetView)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn IndexOf(&self, value: &HttpCookiePairHeaderValue, index: &mut u32) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpCookiePairHeaderValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IndexOf)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value), index, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetAt(&self, index: u32, value: &HttpCookiePairHeaderValue) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpCookiePairHeaderValue>>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetAt)(::windows::core::Vtable::as_raw(this), index, ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn InsertAt(&self, index: u32, value: &HttpCookiePairHeaderValue) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpCookiePairHeaderValue>>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).InsertAt)(::windows::core::Vtable::as_raw(this), index, ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAt(&self, index: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpCookiePairHeaderValue>>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveAt)(::windows::core::Vtable::as_raw(this), index).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Append(&self, value: &HttpCookiePairHeaderValue) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpCookiePairHeaderValue>>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Append)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAtEnd(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpCookiePairHeaderValue>>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveAtEnd)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Clear(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpCookiePairHeaderValue>>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Clear)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMany(&self, startindex: u32, items: &mut [::core::option::Option<HttpCookiePairHeaderValue>]) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpCookiePairHeaderValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetMany)(::windows::core::Vtable::as_raw(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ReplaceAll(&self, items: &[::core::option::Option<HttpCookiePairHeaderValue>]) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpCookiePairHeaderValue>>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).ReplaceAll)(::windows::core::Vtable::as_raw(this), items.len() as u32, ::core::mem::transmute(items.as_ptr())).ok() }
    }
}
impl ::core::clone::Clone for HttpCookiePairHeaderValueCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HttpCookiePairHeaderValueCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpCookiePairHeaderValueCollection {}
impl ::core::fmt::Debug for HttpCookiePairHeaderValueCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HttpCookiePairHeaderValueCollection").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for HttpCookiePairHeaderValueCollection {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Headers.HttpCookiePairHeaderValueCollection;{f3f44350-581e-4ecc-9f59-e507d04f06e6})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for HttpCookiePairHeaderValueCollection {
    type Vtable = IHttpCookiePairHeaderValueCollection_Vtbl;
}
unsafe impl ::windows::core::Interface for HttpCookiePairHeaderValueCollection {
    const IID: ::windows::core::GUID = <IHttpCookiePairHeaderValueCollection as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for HttpCookiePairHeaderValueCollection {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpCookiePairHeaderValueCollection";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for HttpCookiePairHeaderValueCollection {
    type Item = HttpCookiePairHeaderValue;
    type IntoIter = super::super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for &HttpCookiePairHeaderValueCollection {
    type Item = HttpCookiePairHeaderValue;
    type IntoIter = super::super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        super::super::super::Foundation::Collections::VectorIterator::new(::core::convert::TryInto::try_into(self).ok())
    }
}
::windows::core::interface_hierarchy!(HttpCookiePairHeaderValueCollection, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<HttpCookiePairHeaderValueCollection> for super::super::super::Foundation::Collections::IIterable<HttpCookiePairHeaderValue> {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpCookiePairHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&HttpCookiePairHeaderValueCollection> for super::super::super::Foundation::Collections::IIterable<HttpCookiePairHeaderValue> {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpCookiePairHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&HttpCookiePairHeaderValueCollection> for ::windows::core::InParam<super::super::super::Foundation::Collections::IIterable<HttpCookiePairHeaderValue>> {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpCookiePairHeaderValueCollection) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<HttpCookiePairHeaderValueCollection> for super::super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpCookiePairHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&HttpCookiePairHeaderValueCollection> for super::super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpCookiePairHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&HttpCookiePairHeaderValueCollection> for ::windows::core::InParam<super::super::super::Foundation::IStringable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpCookiePairHeaderValueCollection) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<HttpCookiePairHeaderValueCollection> for super::super::super::Foundation::Collections::IVector<HttpCookiePairHeaderValue> {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpCookiePairHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&HttpCookiePairHeaderValueCollection> for super::super::super::Foundation::Collections::IVector<HttpCookiePairHeaderValue> {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpCookiePairHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&HttpCookiePairHeaderValueCollection> for ::windows::core::InParam<super::super::super::Foundation::Collections::IVector<HttpCookiePairHeaderValue>> {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpCookiePairHeaderValueCollection) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for HttpCookiePairHeaderValueCollection {}
unsafe impl ::core::marker::Sync for HttpCookiePairHeaderValueCollection {}
#[doc = "*Required features: `\"Web_Http_Headers\"`*"]
#[repr(transparent)]
pub struct HttpCredentialsHeaderValue(::windows::core::IUnknown);
impl HttpCredentialsHeaderValue {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Parameters(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<HttpNameValueHeaderValue>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Parameters)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Scheme(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Scheme)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Token(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Token)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CreateFromScheme(scheme: &::windows::core::HSTRING) -> ::windows::core::Result<HttpCredentialsHeaderValue> {
        Self::IHttpCredentialsHeaderValueFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateFromScheme)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(scheme), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn CreateFromSchemeWithToken(scheme: &::windows::core::HSTRING, token: &::windows::core::HSTRING) -> ::windows::core::Result<HttpCredentialsHeaderValue> {
        Self::IHttpCredentialsHeaderValueFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateFromSchemeWithToken)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(scheme), ::core::mem::transmute_copy(token), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn Parse(input: &::windows::core::HSTRING) -> ::windows::core::Result<HttpCredentialsHeaderValue> {
        Self::IHttpCredentialsHeaderValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Parse)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(input), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn TryParse(input: &::windows::core::HSTRING, credentialsheadervalue: &mut ::core::option::Option<HttpCredentialsHeaderValue>) -> ::windows::core::Result<bool> {
        Self::IHttpCredentialsHeaderValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TryParse)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(input), credentialsheadervalue as *mut _ as _, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ToString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ToString)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc(hidden)]
    pub fn IHttpCredentialsHeaderValueFactory<R, F: FnOnce(&IHttpCredentialsHeaderValueFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<HttpCredentialsHeaderValue, IHttpCredentialsHeaderValueFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IHttpCredentialsHeaderValueStatics<R, F: FnOnce(&IHttpCredentialsHeaderValueStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<HttpCredentialsHeaderValue, IHttpCredentialsHeaderValueStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for HttpCredentialsHeaderValue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HttpCredentialsHeaderValue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpCredentialsHeaderValue {}
impl ::core::fmt::Debug for HttpCredentialsHeaderValue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HttpCredentialsHeaderValue").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for HttpCredentialsHeaderValue {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Headers.HttpCredentialsHeaderValue;{c34cc3cb-542e-4177-a6c7-b674ce193fbf})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for HttpCredentialsHeaderValue {
    type Vtable = IHttpCredentialsHeaderValue_Vtbl;
}
unsafe impl ::windows::core::Interface for HttpCredentialsHeaderValue {
    const IID: ::windows::core::GUID = <IHttpCredentialsHeaderValue as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for HttpCredentialsHeaderValue {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpCredentialsHeaderValue";
}
::windows::core::interface_hierarchy!(HttpCredentialsHeaderValue, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<HttpCredentialsHeaderValue> for super::super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpCredentialsHeaderValue) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&HttpCredentialsHeaderValue> for super::super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpCredentialsHeaderValue) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&HttpCredentialsHeaderValue> for ::windows::core::InParam<super::super::super::Foundation::IStringable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpCredentialsHeaderValue) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for HttpCredentialsHeaderValue {}
unsafe impl ::core::marker::Sync for HttpCredentialsHeaderValue {}
#[doc = "*Required features: `\"Web_Http_Headers\"`*"]
#[repr(transparent)]
pub struct HttpDateOrDeltaHeaderValue(::windows::core::IUnknown);
impl HttpDateOrDeltaHeaderValue {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Date(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Date)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Delta(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Delta)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Parse(input: &::windows::core::HSTRING) -> ::windows::core::Result<HttpDateOrDeltaHeaderValue> {
        Self::IHttpDateOrDeltaHeaderValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Parse)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(input), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn TryParse(input: &::windows::core::HSTRING, dateordeltaheadervalue: &mut ::core::option::Option<HttpDateOrDeltaHeaderValue>) -> ::windows::core::Result<bool> {
        Self::IHttpDateOrDeltaHeaderValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TryParse)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(input), dateordeltaheadervalue as *mut _ as _, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ToString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ToString)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc(hidden)]
    pub fn IHttpDateOrDeltaHeaderValueStatics<R, F: FnOnce(&IHttpDateOrDeltaHeaderValueStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<HttpDateOrDeltaHeaderValue, IHttpDateOrDeltaHeaderValueStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for HttpDateOrDeltaHeaderValue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HttpDateOrDeltaHeaderValue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpDateOrDeltaHeaderValue {}
impl ::core::fmt::Debug for HttpDateOrDeltaHeaderValue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HttpDateOrDeltaHeaderValue").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for HttpDateOrDeltaHeaderValue {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Headers.HttpDateOrDeltaHeaderValue;{eafcaa6a-c4dc-49e2-a27d-043adf5867a3})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for HttpDateOrDeltaHeaderValue {
    type Vtable = IHttpDateOrDeltaHeaderValue_Vtbl;
}
unsafe impl ::windows::core::Interface for HttpDateOrDeltaHeaderValue {
    const IID: ::windows::core::GUID = <IHttpDateOrDeltaHeaderValue as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for HttpDateOrDeltaHeaderValue {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpDateOrDeltaHeaderValue";
}
::windows::core::interface_hierarchy!(HttpDateOrDeltaHeaderValue, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<HttpDateOrDeltaHeaderValue> for super::super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpDateOrDeltaHeaderValue) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&HttpDateOrDeltaHeaderValue> for super::super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpDateOrDeltaHeaderValue) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&HttpDateOrDeltaHeaderValue> for ::windows::core::InParam<super::super::super::Foundation::IStringable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpDateOrDeltaHeaderValue) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for HttpDateOrDeltaHeaderValue {}
unsafe impl ::core::marker::Sync for HttpDateOrDeltaHeaderValue {}
#[doc = "*Required features: `\"Web_Http_Headers\"`*"]
#[repr(transparent)]
pub struct HttpExpectationHeaderValue(::windows::core::IUnknown);
impl HttpExpectationHeaderValue {
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Name)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Value(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Value)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetValue(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetValue)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Parameters(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<HttpNameValueHeaderValue>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Parameters)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CreateFromName(name: &::windows::core::HSTRING) -> ::windows::core::Result<HttpExpectationHeaderValue> {
        Self::IHttpExpectationHeaderValueFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateFromName)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(name), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn CreateFromNameWithValue(name: &::windows::core::HSTRING, value: &::windows::core::HSTRING) -> ::windows::core::Result<HttpExpectationHeaderValue> {
        Self::IHttpExpectationHeaderValueFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateFromNameWithValue)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(name), ::core::mem::transmute_copy(value), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn Parse(input: &::windows::core::HSTRING) -> ::windows::core::Result<HttpExpectationHeaderValue> {
        Self::IHttpExpectationHeaderValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Parse)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(input), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn TryParse(input: &::windows::core::HSTRING, expectationheadervalue: &mut ::core::option::Option<HttpExpectationHeaderValue>) -> ::windows::core::Result<bool> {
        Self::IHttpExpectationHeaderValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TryParse)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(input), expectationheadervalue as *mut _ as _, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ToString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ToString)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc(hidden)]
    pub fn IHttpExpectationHeaderValueFactory<R, F: FnOnce(&IHttpExpectationHeaderValueFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<HttpExpectationHeaderValue, IHttpExpectationHeaderValueFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IHttpExpectationHeaderValueStatics<R, F: FnOnce(&IHttpExpectationHeaderValueStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<HttpExpectationHeaderValue, IHttpExpectationHeaderValueStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for HttpExpectationHeaderValue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HttpExpectationHeaderValue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpExpectationHeaderValue {}
impl ::core::fmt::Debug for HttpExpectationHeaderValue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HttpExpectationHeaderValue").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for HttpExpectationHeaderValue {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Headers.HttpExpectationHeaderValue;{4ce585cd-3a99-43af-a2e6-ec232fea9658})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for HttpExpectationHeaderValue {
    type Vtable = IHttpExpectationHeaderValue_Vtbl;
}
unsafe impl ::windows::core::Interface for HttpExpectationHeaderValue {
    const IID: ::windows::core::GUID = <IHttpExpectationHeaderValue as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for HttpExpectationHeaderValue {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpExpectationHeaderValue";
}
::windows::core::interface_hierarchy!(HttpExpectationHeaderValue, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<HttpExpectationHeaderValue> for super::super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpExpectationHeaderValue) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&HttpExpectationHeaderValue> for super::super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpExpectationHeaderValue) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&HttpExpectationHeaderValue> for ::windows::core::InParam<super::super::super::Foundation::IStringable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpExpectationHeaderValue) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for HttpExpectationHeaderValue {}
unsafe impl ::core::marker::Sync for HttpExpectationHeaderValue {}
#[doc = "*Required features: `\"Web_Http_Headers\"`*"]
#[repr(transparent)]
pub struct HttpExpectationHeaderValueCollection(::windows::core::IUnknown);
impl HttpExpectationHeaderValueCollection {
    pub fn ParseAdd(&self, input: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).ParseAdd)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(input)).ok() }
    }
    pub fn TryParseAdd(&self, input: &::windows::core::HSTRING) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TryParseAdd)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(input), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IIterator<HttpExpectationHeaderValue>> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IIterable<HttpExpectationHeaderValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).First)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ToString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ToString)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAt(&self, index: u32) -> ::windows::core::Result<HttpExpectationHeaderValue> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpExpectationHeaderValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetAt)(::windows::core::Vtable::as_raw(this), index, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpExpectationHeaderValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Size)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetView(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<HttpExpectationHeaderValue>> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpExpectationHeaderValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetView)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn IndexOf(&self, value: &HttpExpectationHeaderValue, index: &mut u32) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpExpectationHeaderValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IndexOf)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value), index, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetAt(&self, index: u32, value: &HttpExpectationHeaderValue) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpExpectationHeaderValue>>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetAt)(::windows::core::Vtable::as_raw(this), index, ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn InsertAt(&self, index: u32, value: &HttpExpectationHeaderValue) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpExpectationHeaderValue>>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).InsertAt)(::windows::core::Vtable::as_raw(this), index, ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAt(&self, index: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpExpectationHeaderValue>>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveAt)(::windows::core::Vtable::as_raw(this), index).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Append(&self, value: &HttpExpectationHeaderValue) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpExpectationHeaderValue>>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Append)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAtEnd(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpExpectationHeaderValue>>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveAtEnd)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Clear(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpExpectationHeaderValue>>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Clear)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMany(&self, startindex: u32, items: &mut [::core::option::Option<HttpExpectationHeaderValue>]) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpExpectationHeaderValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetMany)(::windows::core::Vtable::as_raw(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ReplaceAll(&self, items: &[::core::option::Option<HttpExpectationHeaderValue>]) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpExpectationHeaderValue>>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).ReplaceAll)(::windows::core::Vtable::as_raw(this), items.len() as u32, ::core::mem::transmute(items.as_ptr())).ok() }
    }
}
impl ::core::clone::Clone for HttpExpectationHeaderValueCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HttpExpectationHeaderValueCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpExpectationHeaderValueCollection {}
impl ::core::fmt::Debug for HttpExpectationHeaderValueCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HttpExpectationHeaderValueCollection").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for HttpExpectationHeaderValueCollection {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Headers.HttpExpectationHeaderValueCollection;{e78521b3-a0e2-4ac4-9e66-79706cb9fd58})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for HttpExpectationHeaderValueCollection {
    type Vtable = IHttpExpectationHeaderValueCollection_Vtbl;
}
unsafe impl ::windows::core::Interface for HttpExpectationHeaderValueCollection {
    const IID: ::windows::core::GUID = <IHttpExpectationHeaderValueCollection as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for HttpExpectationHeaderValueCollection {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpExpectationHeaderValueCollection";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for HttpExpectationHeaderValueCollection {
    type Item = HttpExpectationHeaderValue;
    type IntoIter = super::super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for &HttpExpectationHeaderValueCollection {
    type Item = HttpExpectationHeaderValue;
    type IntoIter = super::super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        super::super::super::Foundation::Collections::VectorIterator::new(::core::convert::TryInto::try_into(self).ok())
    }
}
::windows::core::interface_hierarchy!(HttpExpectationHeaderValueCollection, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<HttpExpectationHeaderValueCollection> for super::super::super::Foundation::Collections::IIterable<HttpExpectationHeaderValue> {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpExpectationHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&HttpExpectationHeaderValueCollection> for super::super::super::Foundation::Collections::IIterable<HttpExpectationHeaderValue> {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpExpectationHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&HttpExpectationHeaderValueCollection> for ::windows::core::InParam<super::super::super::Foundation::Collections::IIterable<HttpExpectationHeaderValue>> {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpExpectationHeaderValueCollection) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<HttpExpectationHeaderValueCollection> for super::super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpExpectationHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&HttpExpectationHeaderValueCollection> for super::super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpExpectationHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&HttpExpectationHeaderValueCollection> for ::windows::core::InParam<super::super::super::Foundation::IStringable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpExpectationHeaderValueCollection) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<HttpExpectationHeaderValueCollection> for super::super::super::Foundation::Collections::IVector<HttpExpectationHeaderValue> {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpExpectationHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&HttpExpectationHeaderValueCollection> for super::super::super::Foundation::Collections::IVector<HttpExpectationHeaderValue> {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpExpectationHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&HttpExpectationHeaderValueCollection> for ::windows::core::InParam<super::super::super::Foundation::Collections::IVector<HttpExpectationHeaderValue>> {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpExpectationHeaderValueCollection) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for HttpExpectationHeaderValueCollection {}
unsafe impl ::core::marker::Sync for HttpExpectationHeaderValueCollection {}
#[doc = "*Required features: `\"Web_Http_Headers\"`*"]
#[repr(transparent)]
pub struct HttpLanguageHeaderValueCollection(::windows::core::IUnknown);
impl HttpLanguageHeaderValueCollection {
    pub fn ParseAdd(&self, input: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).ParseAdd)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(input)).ok() }
    }
    pub fn TryParseAdd(&self, input: &::windows::core::HSTRING) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TryParseAdd)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(input), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Globalization\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Globalization"))]
    pub fn First(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IIterator<super::super::super::Globalization::Language>> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IIterable<super::super::super::Globalization::Language>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).First)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ToString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ToString)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Globalization\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Globalization"))]
    pub fn GetAt(&self, index: u32) -> ::windows::core::Result<super::super::super::Globalization::Language> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<super::super::super::Globalization::Language>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetAt)(::windows::core::Vtable::as_raw(this), index, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Globalization\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Globalization"))]
    pub fn Size(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<super::super::super::Globalization::Language>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Size)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Globalization\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Globalization"))]
    pub fn GetView(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<super::super::super::Globalization::Language>> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<super::super::super::Globalization::Language>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetView)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Globalization\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Globalization"))]
    pub fn IndexOf(&self, value: &super::super::super::Globalization::Language, index: &mut u32) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<super::super::super::Globalization::Language>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IndexOf)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value), index, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Globalization\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Globalization"))]
    pub fn SetAt(&self, index: u32, value: &super::super::super::Globalization::Language) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<super::super::super::Globalization::Language>>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetAt)(::windows::core::Vtable::as_raw(this), index, ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Globalization\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Globalization"))]
    pub fn InsertAt(&self, index: u32, value: &super::super::super::Globalization::Language) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<super::super::super::Globalization::Language>>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).InsertAt)(::windows::core::Vtable::as_raw(this), index, ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Globalization\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Globalization"))]
    pub fn RemoveAt(&self, index: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<super::super::super::Globalization::Language>>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveAt)(::windows::core::Vtable::as_raw(this), index).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Globalization\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Globalization"))]
    pub fn Append(&self, value: &super::super::super::Globalization::Language) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<super::super::super::Globalization::Language>>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Append)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Globalization\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Globalization"))]
    pub fn RemoveAtEnd(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<super::super::super::Globalization::Language>>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveAtEnd)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Globalization\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Globalization"))]
    pub fn Clear(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<super::super::super::Globalization::Language>>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Clear)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Globalization\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Globalization"))]
    pub fn GetMany(&self, startindex: u32, items: &mut [::core::option::Option<super::super::super::Globalization::Language>]) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<super::super::super::Globalization::Language>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetMany)(::windows::core::Vtable::as_raw(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Globalization\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Globalization"))]
    pub fn ReplaceAll(&self, items: &[::core::option::Option<super::super::super::Globalization::Language>]) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<super::super::super::Globalization::Language>>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).ReplaceAll)(::windows::core::Vtable::as_raw(this), items.len() as u32, ::core::mem::transmute(items.as_ptr())).ok() }
    }
}
impl ::core::clone::Clone for HttpLanguageHeaderValueCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HttpLanguageHeaderValueCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpLanguageHeaderValueCollection {}
impl ::core::fmt::Debug for HttpLanguageHeaderValueCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HttpLanguageHeaderValueCollection").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for HttpLanguageHeaderValueCollection {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Headers.HttpLanguageHeaderValueCollection;{9ebd7ca3-8219-44f6-9902-8c56dfd3340c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for HttpLanguageHeaderValueCollection {
    type Vtable = IHttpLanguageHeaderValueCollection_Vtbl;
}
unsafe impl ::windows::core::Interface for HttpLanguageHeaderValueCollection {
    const IID: ::windows::core::GUID = <IHttpLanguageHeaderValueCollection as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for HttpLanguageHeaderValueCollection {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpLanguageHeaderValueCollection";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Globalization"))]
impl ::core::iter::IntoIterator for HttpLanguageHeaderValueCollection {
    type Item = super::super::super::Globalization::Language;
    type IntoIter = super::super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Globalization"))]
impl ::core::iter::IntoIterator for &HttpLanguageHeaderValueCollection {
    type Item = super::super::super::Globalization::Language;
    type IntoIter = super::super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        super::super::super::Foundation::Collections::VectorIterator::new(::core::convert::TryInto::try_into(self).ok())
    }
}
::windows::core::interface_hierarchy!(HttpLanguageHeaderValueCollection, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(all(feature = "Foundation_Collections", feature = "Globalization"))]
impl ::core::convert::TryFrom<HttpLanguageHeaderValueCollection> for super::super::super::Foundation::Collections::IIterable<super::super::super::Globalization::Language> {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpLanguageHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Globalization"))]
impl ::core::convert::TryFrom<&HttpLanguageHeaderValueCollection> for super::super::super::Foundation::Collections::IIterable<super::super::super::Globalization::Language> {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpLanguageHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Globalization"))]
impl ::core::convert::TryFrom<&HttpLanguageHeaderValueCollection> for ::windows::core::InParam<super::super::super::Foundation::Collections::IIterable<super::super::super::Globalization::Language>> {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpLanguageHeaderValueCollection) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<HttpLanguageHeaderValueCollection> for super::super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpLanguageHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&HttpLanguageHeaderValueCollection> for super::super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpLanguageHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&HttpLanguageHeaderValueCollection> for ::windows::core::InParam<super::super::super::Foundation::IStringable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpLanguageHeaderValueCollection) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Globalization"))]
impl ::core::convert::TryFrom<HttpLanguageHeaderValueCollection> for super::super::super::Foundation::Collections::IVector<super::super::super::Globalization::Language> {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpLanguageHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Globalization"))]
impl ::core::convert::TryFrom<&HttpLanguageHeaderValueCollection> for super::super::super::Foundation::Collections::IVector<super::super::super::Globalization::Language> {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpLanguageHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Globalization"))]
impl ::core::convert::TryFrom<&HttpLanguageHeaderValueCollection> for ::windows::core::InParam<super::super::super::Foundation::Collections::IVector<super::super::super::Globalization::Language>> {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpLanguageHeaderValueCollection) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for HttpLanguageHeaderValueCollection {}
unsafe impl ::core::marker::Sync for HttpLanguageHeaderValueCollection {}
#[doc = "*Required features: `\"Web_Http_Headers\"`*"]
#[repr(transparent)]
pub struct HttpLanguageRangeWithQualityHeaderValue(::windows::core::IUnknown);
impl HttpLanguageRangeWithQualityHeaderValue {
    pub fn LanguageRange(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LanguageRange)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Quality(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Quality)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CreateFromLanguageRange(languagerange: &::windows::core::HSTRING) -> ::windows::core::Result<HttpLanguageRangeWithQualityHeaderValue> {
        Self::IHttpLanguageRangeWithQualityHeaderValueFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateFromLanguageRange)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(languagerange), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn CreateFromLanguageRangeWithQuality(languagerange: &::windows::core::HSTRING, quality: f64) -> ::windows::core::Result<HttpLanguageRangeWithQualityHeaderValue> {
        Self::IHttpLanguageRangeWithQualityHeaderValueFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateFromLanguageRangeWithQuality)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(languagerange), quality, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn Parse(input: &::windows::core::HSTRING) -> ::windows::core::Result<HttpLanguageRangeWithQualityHeaderValue> {
        Self::IHttpLanguageRangeWithQualityHeaderValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Parse)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(input), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn TryParse(input: &::windows::core::HSTRING, languagerangewithqualityheadervalue: &mut ::core::option::Option<HttpLanguageRangeWithQualityHeaderValue>) -> ::windows::core::Result<bool> {
        Self::IHttpLanguageRangeWithQualityHeaderValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TryParse)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(input), languagerangewithqualityheadervalue as *mut _ as _, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ToString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ToString)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc(hidden)]
    pub fn IHttpLanguageRangeWithQualityHeaderValueFactory<R, F: FnOnce(&IHttpLanguageRangeWithQualityHeaderValueFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<HttpLanguageRangeWithQualityHeaderValue, IHttpLanguageRangeWithQualityHeaderValueFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IHttpLanguageRangeWithQualityHeaderValueStatics<R, F: FnOnce(&IHttpLanguageRangeWithQualityHeaderValueStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<HttpLanguageRangeWithQualityHeaderValue, IHttpLanguageRangeWithQualityHeaderValueStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for HttpLanguageRangeWithQualityHeaderValue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HttpLanguageRangeWithQualityHeaderValue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpLanguageRangeWithQualityHeaderValue {}
impl ::core::fmt::Debug for HttpLanguageRangeWithQualityHeaderValue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HttpLanguageRangeWithQualityHeaderValue").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for HttpLanguageRangeWithQualityHeaderValue {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Headers.HttpLanguageRangeWithQualityHeaderValue;{7256e102-0080-4db4-a083-7de7b2e5ba4c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for HttpLanguageRangeWithQualityHeaderValue {
    type Vtable = IHttpLanguageRangeWithQualityHeaderValue_Vtbl;
}
unsafe impl ::windows::core::Interface for HttpLanguageRangeWithQualityHeaderValue {
    const IID: ::windows::core::GUID = <IHttpLanguageRangeWithQualityHeaderValue as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for HttpLanguageRangeWithQualityHeaderValue {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpLanguageRangeWithQualityHeaderValue";
}
::windows::core::interface_hierarchy!(HttpLanguageRangeWithQualityHeaderValue, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<HttpLanguageRangeWithQualityHeaderValue> for super::super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpLanguageRangeWithQualityHeaderValue) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&HttpLanguageRangeWithQualityHeaderValue> for super::super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpLanguageRangeWithQualityHeaderValue) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&HttpLanguageRangeWithQualityHeaderValue> for ::windows::core::InParam<super::super::super::Foundation::IStringable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpLanguageRangeWithQualityHeaderValue) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for HttpLanguageRangeWithQualityHeaderValue {}
unsafe impl ::core::marker::Sync for HttpLanguageRangeWithQualityHeaderValue {}
#[doc = "*Required features: `\"Web_Http_Headers\"`*"]
#[repr(transparent)]
pub struct HttpLanguageRangeWithQualityHeaderValueCollection(::windows::core::IUnknown);
impl HttpLanguageRangeWithQualityHeaderValueCollection {
    pub fn ParseAdd(&self, input: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).ParseAdd)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(input)).ok() }
    }
    pub fn TryParseAdd(&self, input: &::windows::core::HSTRING) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TryParseAdd)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(input), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IIterator<HttpLanguageRangeWithQualityHeaderValue>> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IIterable<HttpLanguageRangeWithQualityHeaderValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).First)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ToString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ToString)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAt(&self, index: u32) -> ::windows::core::Result<HttpLanguageRangeWithQualityHeaderValue> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpLanguageRangeWithQualityHeaderValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetAt)(::windows::core::Vtable::as_raw(this), index, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpLanguageRangeWithQualityHeaderValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Size)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetView(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<HttpLanguageRangeWithQualityHeaderValue>> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpLanguageRangeWithQualityHeaderValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetView)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn IndexOf(&self, value: &HttpLanguageRangeWithQualityHeaderValue, index: &mut u32) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpLanguageRangeWithQualityHeaderValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IndexOf)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value), index, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetAt(&self, index: u32, value: &HttpLanguageRangeWithQualityHeaderValue) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpLanguageRangeWithQualityHeaderValue>>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetAt)(::windows::core::Vtable::as_raw(this), index, ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn InsertAt(&self, index: u32, value: &HttpLanguageRangeWithQualityHeaderValue) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpLanguageRangeWithQualityHeaderValue>>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).InsertAt)(::windows::core::Vtable::as_raw(this), index, ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAt(&self, index: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpLanguageRangeWithQualityHeaderValue>>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveAt)(::windows::core::Vtable::as_raw(this), index).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Append(&self, value: &HttpLanguageRangeWithQualityHeaderValue) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpLanguageRangeWithQualityHeaderValue>>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Append)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAtEnd(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpLanguageRangeWithQualityHeaderValue>>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveAtEnd)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Clear(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpLanguageRangeWithQualityHeaderValue>>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Clear)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMany(&self, startindex: u32, items: &mut [::core::option::Option<HttpLanguageRangeWithQualityHeaderValue>]) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpLanguageRangeWithQualityHeaderValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetMany)(::windows::core::Vtable::as_raw(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ReplaceAll(&self, items: &[::core::option::Option<HttpLanguageRangeWithQualityHeaderValue>]) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpLanguageRangeWithQualityHeaderValue>>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).ReplaceAll)(::windows::core::Vtable::as_raw(this), items.len() as u32, ::core::mem::transmute(items.as_ptr())).ok() }
    }
}
impl ::core::clone::Clone for HttpLanguageRangeWithQualityHeaderValueCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HttpLanguageRangeWithQualityHeaderValueCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpLanguageRangeWithQualityHeaderValueCollection {}
impl ::core::fmt::Debug for HttpLanguageRangeWithQualityHeaderValueCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HttpLanguageRangeWithQualityHeaderValueCollection").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for HttpLanguageRangeWithQualityHeaderValueCollection {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Headers.HttpLanguageRangeWithQualityHeaderValueCollection;{885d5abd-4b4f-480a-89ce-8aedcee6e3a0})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for HttpLanguageRangeWithQualityHeaderValueCollection {
    type Vtable = IHttpLanguageRangeWithQualityHeaderValueCollection_Vtbl;
}
unsafe impl ::windows::core::Interface for HttpLanguageRangeWithQualityHeaderValueCollection {
    const IID: ::windows::core::GUID = <IHttpLanguageRangeWithQualityHeaderValueCollection as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for HttpLanguageRangeWithQualityHeaderValueCollection {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpLanguageRangeWithQualityHeaderValueCollection";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for HttpLanguageRangeWithQualityHeaderValueCollection {
    type Item = HttpLanguageRangeWithQualityHeaderValue;
    type IntoIter = super::super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for &HttpLanguageRangeWithQualityHeaderValueCollection {
    type Item = HttpLanguageRangeWithQualityHeaderValue;
    type IntoIter = super::super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        super::super::super::Foundation::Collections::VectorIterator::new(::core::convert::TryInto::try_into(self).ok())
    }
}
::windows::core::interface_hierarchy!(HttpLanguageRangeWithQualityHeaderValueCollection, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<HttpLanguageRangeWithQualityHeaderValueCollection> for super::super::super::Foundation::Collections::IIterable<HttpLanguageRangeWithQualityHeaderValue> {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpLanguageRangeWithQualityHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&HttpLanguageRangeWithQualityHeaderValueCollection> for super::super::super::Foundation::Collections::IIterable<HttpLanguageRangeWithQualityHeaderValue> {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpLanguageRangeWithQualityHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&HttpLanguageRangeWithQualityHeaderValueCollection> for ::windows::core::InParam<super::super::super::Foundation::Collections::IIterable<HttpLanguageRangeWithQualityHeaderValue>> {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpLanguageRangeWithQualityHeaderValueCollection) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<HttpLanguageRangeWithQualityHeaderValueCollection> for super::super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpLanguageRangeWithQualityHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&HttpLanguageRangeWithQualityHeaderValueCollection> for super::super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpLanguageRangeWithQualityHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&HttpLanguageRangeWithQualityHeaderValueCollection> for ::windows::core::InParam<super::super::super::Foundation::IStringable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpLanguageRangeWithQualityHeaderValueCollection) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<HttpLanguageRangeWithQualityHeaderValueCollection> for super::super::super::Foundation::Collections::IVector<HttpLanguageRangeWithQualityHeaderValue> {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpLanguageRangeWithQualityHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&HttpLanguageRangeWithQualityHeaderValueCollection> for super::super::super::Foundation::Collections::IVector<HttpLanguageRangeWithQualityHeaderValue> {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpLanguageRangeWithQualityHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&HttpLanguageRangeWithQualityHeaderValueCollection> for ::windows::core::InParam<super::super::super::Foundation::Collections::IVector<HttpLanguageRangeWithQualityHeaderValue>> {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpLanguageRangeWithQualityHeaderValueCollection) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for HttpLanguageRangeWithQualityHeaderValueCollection {}
unsafe impl ::core::marker::Sync for HttpLanguageRangeWithQualityHeaderValueCollection {}
#[doc = "*Required features: `\"Web_Http_Headers\"`*"]
#[repr(transparent)]
pub struct HttpMediaTypeHeaderValue(::windows::core::IUnknown);
impl HttpMediaTypeHeaderValue {
    pub fn CharSet(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CharSet)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetCharSet(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetCharSet)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn MediaType(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MediaType)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetMediaType(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetMediaType)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Parameters(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<HttpNameValueHeaderValue>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Parameters)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Create(mediatype: &::windows::core::HSTRING) -> ::windows::core::Result<HttpMediaTypeHeaderValue> {
        Self::IHttpMediaTypeHeaderValueFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Create)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(mediatype), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn Parse(input: &::windows::core::HSTRING) -> ::windows::core::Result<HttpMediaTypeHeaderValue> {
        Self::IHttpMediaTypeHeaderValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Parse)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(input), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn TryParse(input: &::windows::core::HSTRING, mediatypeheadervalue: &mut ::core::option::Option<HttpMediaTypeHeaderValue>) -> ::windows::core::Result<bool> {
        Self::IHttpMediaTypeHeaderValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TryParse)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(input), mediatypeheadervalue as *mut _ as _, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ToString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ToString)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc(hidden)]
    pub fn IHttpMediaTypeHeaderValueFactory<R, F: FnOnce(&IHttpMediaTypeHeaderValueFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<HttpMediaTypeHeaderValue, IHttpMediaTypeHeaderValueFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IHttpMediaTypeHeaderValueStatics<R, F: FnOnce(&IHttpMediaTypeHeaderValueStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<HttpMediaTypeHeaderValue, IHttpMediaTypeHeaderValueStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for HttpMediaTypeHeaderValue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HttpMediaTypeHeaderValue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpMediaTypeHeaderValue {}
impl ::core::fmt::Debug for HttpMediaTypeHeaderValue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HttpMediaTypeHeaderValue").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for HttpMediaTypeHeaderValue {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Headers.HttpMediaTypeHeaderValue;{16b28533-e728-4fcb-bdb0-08a431a14844})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for HttpMediaTypeHeaderValue {
    type Vtable = IHttpMediaTypeHeaderValue_Vtbl;
}
unsafe impl ::windows::core::Interface for HttpMediaTypeHeaderValue {
    const IID: ::windows::core::GUID = <IHttpMediaTypeHeaderValue as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for HttpMediaTypeHeaderValue {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpMediaTypeHeaderValue";
}
::windows::core::interface_hierarchy!(HttpMediaTypeHeaderValue, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<HttpMediaTypeHeaderValue> for super::super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpMediaTypeHeaderValue) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&HttpMediaTypeHeaderValue> for super::super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpMediaTypeHeaderValue) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&HttpMediaTypeHeaderValue> for ::windows::core::InParam<super::super::super::Foundation::IStringable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpMediaTypeHeaderValue) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for HttpMediaTypeHeaderValue {}
unsafe impl ::core::marker::Sync for HttpMediaTypeHeaderValue {}
#[doc = "*Required features: `\"Web_Http_Headers\"`*"]
#[repr(transparent)]
pub struct HttpMediaTypeWithQualityHeaderValue(::windows::core::IUnknown);
impl HttpMediaTypeWithQualityHeaderValue {
    pub fn CharSet(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CharSet)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetCharSet(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetCharSet)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn MediaType(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MediaType)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetMediaType(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetMediaType)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Parameters(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<HttpNameValueHeaderValue>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Parameters)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Quality(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Quality)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetQuality<P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::super::Foundation::IReference<f64>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetQuality)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    pub fn CreateFromMediaType(mediatype: &::windows::core::HSTRING) -> ::windows::core::Result<HttpMediaTypeWithQualityHeaderValue> {
        Self::IHttpMediaTypeWithQualityHeaderValueFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateFromMediaType)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(mediatype), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn CreateFromMediaTypeWithQuality(mediatype: &::windows::core::HSTRING, quality: f64) -> ::windows::core::Result<HttpMediaTypeWithQualityHeaderValue> {
        Self::IHttpMediaTypeWithQualityHeaderValueFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateFromMediaTypeWithQuality)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(mediatype), quality, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn Parse(input: &::windows::core::HSTRING) -> ::windows::core::Result<HttpMediaTypeWithQualityHeaderValue> {
        Self::IHttpMediaTypeWithQualityHeaderValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Parse)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(input), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn TryParse(input: &::windows::core::HSTRING, mediatypewithqualityheadervalue: &mut ::core::option::Option<HttpMediaTypeWithQualityHeaderValue>) -> ::windows::core::Result<bool> {
        Self::IHttpMediaTypeWithQualityHeaderValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TryParse)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(input), mediatypewithqualityheadervalue as *mut _ as _, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ToString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ToString)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc(hidden)]
    pub fn IHttpMediaTypeWithQualityHeaderValueFactory<R, F: FnOnce(&IHttpMediaTypeWithQualityHeaderValueFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<HttpMediaTypeWithQualityHeaderValue, IHttpMediaTypeWithQualityHeaderValueFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IHttpMediaTypeWithQualityHeaderValueStatics<R, F: FnOnce(&IHttpMediaTypeWithQualityHeaderValueStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<HttpMediaTypeWithQualityHeaderValue, IHttpMediaTypeWithQualityHeaderValueStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for HttpMediaTypeWithQualityHeaderValue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HttpMediaTypeWithQualityHeaderValue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpMediaTypeWithQualityHeaderValue {}
impl ::core::fmt::Debug for HttpMediaTypeWithQualityHeaderValue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HttpMediaTypeWithQualityHeaderValue").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for HttpMediaTypeWithQualityHeaderValue {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Headers.HttpMediaTypeWithQualityHeaderValue;{188d5e32-76be-44a0-b1cd-2074bded2dde})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for HttpMediaTypeWithQualityHeaderValue {
    type Vtable = IHttpMediaTypeWithQualityHeaderValue_Vtbl;
}
unsafe impl ::windows::core::Interface for HttpMediaTypeWithQualityHeaderValue {
    const IID: ::windows::core::GUID = <IHttpMediaTypeWithQualityHeaderValue as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for HttpMediaTypeWithQualityHeaderValue {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpMediaTypeWithQualityHeaderValue";
}
::windows::core::interface_hierarchy!(HttpMediaTypeWithQualityHeaderValue, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<HttpMediaTypeWithQualityHeaderValue> for super::super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpMediaTypeWithQualityHeaderValue) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&HttpMediaTypeWithQualityHeaderValue> for super::super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpMediaTypeWithQualityHeaderValue) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&HttpMediaTypeWithQualityHeaderValue> for ::windows::core::InParam<super::super::super::Foundation::IStringable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpMediaTypeWithQualityHeaderValue) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for HttpMediaTypeWithQualityHeaderValue {}
unsafe impl ::core::marker::Sync for HttpMediaTypeWithQualityHeaderValue {}
#[doc = "*Required features: `\"Web_Http_Headers\"`*"]
#[repr(transparent)]
pub struct HttpMediaTypeWithQualityHeaderValueCollection(::windows::core::IUnknown);
impl HttpMediaTypeWithQualityHeaderValueCollection {
    pub fn ParseAdd(&self, input: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).ParseAdd)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(input)).ok() }
    }
    pub fn TryParseAdd(&self, input: &::windows::core::HSTRING) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TryParseAdd)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(input), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IIterator<HttpMediaTypeWithQualityHeaderValue>> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IIterable<HttpMediaTypeWithQualityHeaderValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).First)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ToString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ToString)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAt(&self, index: u32) -> ::windows::core::Result<HttpMediaTypeWithQualityHeaderValue> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpMediaTypeWithQualityHeaderValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetAt)(::windows::core::Vtable::as_raw(this), index, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpMediaTypeWithQualityHeaderValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Size)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetView(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<HttpMediaTypeWithQualityHeaderValue>> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpMediaTypeWithQualityHeaderValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetView)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn IndexOf(&self, value: &HttpMediaTypeWithQualityHeaderValue, index: &mut u32) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpMediaTypeWithQualityHeaderValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IndexOf)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value), index, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetAt(&self, index: u32, value: &HttpMediaTypeWithQualityHeaderValue) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpMediaTypeWithQualityHeaderValue>>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetAt)(::windows::core::Vtable::as_raw(this), index, ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn InsertAt(&self, index: u32, value: &HttpMediaTypeWithQualityHeaderValue) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpMediaTypeWithQualityHeaderValue>>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).InsertAt)(::windows::core::Vtable::as_raw(this), index, ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAt(&self, index: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpMediaTypeWithQualityHeaderValue>>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveAt)(::windows::core::Vtable::as_raw(this), index).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Append(&self, value: &HttpMediaTypeWithQualityHeaderValue) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpMediaTypeWithQualityHeaderValue>>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Append)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAtEnd(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpMediaTypeWithQualityHeaderValue>>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveAtEnd)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Clear(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpMediaTypeWithQualityHeaderValue>>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Clear)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMany(&self, startindex: u32, items: &mut [::core::option::Option<HttpMediaTypeWithQualityHeaderValue>]) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpMediaTypeWithQualityHeaderValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetMany)(::windows::core::Vtable::as_raw(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ReplaceAll(&self, items: &[::core::option::Option<HttpMediaTypeWithQualityHeaderValue>]) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpMediaTypeWithQualityHeaderValue>>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).ReplaceAll)(::windows::core::Vtable::as_raw(this), items.len() as u32, ::core::mem::transmute(items.as_ptr())).ok() }
    }
}
impl ::core::clone::Clone for HttpMediaTypeWithQualityHeaderValueCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HttpMediaTypeWithQualityHeaderValueCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpMediaTypeWithQualityHeaderValueCollection {}
impl ::core::fmt::Debug for HttpMediaTypeWithQualityHeaderValueCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HttpMediaTypeWithQualityHeaderValueCollection").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for HttpMediaTypeWithQualityHeaderValueCollection {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Headers.HttpMediaTypeWithQualityHeaderValueCollection;{3c0c6b73-1342-4587-a056-18d02ff67165})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for HttpMediaTypeWithQualityHeaderValueCollection {
    type Vtable = IHttpMediaTypeWithQualityHeaderValueCollection_Vtbl;
}
unsafe impl ::windows::core::Interface for HttpMediaTypeWithQualityHeaderValueCollection {
    const IID: ::windows::core::GUID = <IHttpMediaTypeWithQualityHeaderValueCollection as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for HttpMediaTypeWithQualityHeaderValueCollection {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpMediaTypeWithQualityHeaderValueCollection";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for HttpMediaTypeWithQualityHeaderValueCollection {
    type Item = HttpMediaTypeWithQualityHeaderValue;
    type IntoIter = super::super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for &HttpMediaTypeWithQualityHeaderValueCollection {
    type Item = HttpMediaTypeWithQualityHeaderValue;
    type IntoIter = super::super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        super::super::super::Foundation::Collections::VectorIterator::new(::core::convert::TryInto::try_into(self).ok())
    }
}
::windows::core::interface_hierarchy!(HttpMediaTypeWithQualityHeaderValueCollection, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<HttpMediaTypeWithQualityHeaderValueCollection> for super::super::super::Foundation::Collections::IIterable<HttpMediaTypeWithQualityHeaderValue> {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpMediaTypeWithQualityHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&HttpMediaTypeWithQualityHeaderValueCollection> for super::super::super::Foundation::Collections::IIterable<HttpMediaTypeWithQualityHeaderValue> {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpMediaTypeWithQualityHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&HttpMediaTypeWithQualityHeaderValueCollection> for ::windows::core::InParam<super::super::super::Foundation::Collections::IIterable<HttpMediaTypeWithQualityHeaderValue>> {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpMediaTypeWithQualityHeaderValueCollection) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<HttpMediaTypeWithQualityHeaderValueCollection> for super::super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpMediaTypeWithQualityHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&HttpMediaTypeWithQualityHeaderValueCollection> for super::super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpMediaTypeWithQualityHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&HttpMediaTypeWithQualityHeaderValueCollection> for ::windows::core::InParam<super::super::super::Foundation::IStringable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpMediaTypeWithQualityHeaderValueCollection) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<HttpMediaTypeWithQualityHeaderValueCollection> for super::super::super::Foundation::Collections::IVector<HttpMediaTypeWithQualityHeaderValue> {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpMediaTypeWithQualityHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&HttpMediaTypeWithQualityHeaderValueCollection> for super::super::super::Foundation::Collections::IVector<HttpMediaTypeWithQualityHeaderValue> {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpMediaTypeWithQualityHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&HttpMediaTypeWithQualityHeaderValueCollection> for ::windows::core::InParam<super::super::super::Foundation::Collections::IVector<HttpMediaTypeWithQualityHeaderValue>> {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpMediaTypeWithQualityHeaderValueCollection) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for HttpMediaTypeWithQualityHeaderValueCollection {}
unsafe impl ::core::marker::Sync for HttpMediaTypeWithQualityHeaderValueCollection {}
#[doc = "*Required features: `\"Web_Http_Headers\"`*"]
#[repr(transparent)]
pub struct HttpMethodHeaderValueCollection(::windows::core::IUnknown);
impl HttpMethodHeaderValueCollection {
    pub fn ParseAdd(&self, input: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).ParseAdd)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(input)).ok() }
    }
    pub fn TryParseAdd(&self, input: &::windows::core::HSTRING) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TryParseAdd)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(input), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IIterator<super::HttpMethod>> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IIterable<super::HttpMethod>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).First)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ToString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ToString)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAt(&self, index: u32) -> ::windows::core::Result<super::HttpMethod> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<super::HttpMethod>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetAt)(::windows::core::Vtable::as_raw(this), index, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<super::HttpMethod>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Size)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetView(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<super::HttpMethod>> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<super::HttpMethod>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetView)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn IndexOf(&self, value: &super::HttpMethod, index: &mut u32) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<super::HttpMethod>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IndexOf)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value), index, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetAt(&self, index: u32, value: &super::HttpMethod) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<super::HttpMethod>>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetAt)(::windows::core::Vtable::as_raw(this), index, ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn InsertAt(&self, index: u32, value: &super::HttpMethod) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<super::HttpMethod>>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).InsertAt)(::windows::core::Vtable::as_raw(this), index, ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAt(&self, index: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<super::HttpMethod>>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveAt)(::windows::core::Vtable::as_raw(this), index).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Append(&self, value: &super::HttpMethod) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<super::HttpMethod>>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Append)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAtEnd(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<super::HttpMethod>>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveAtEnd)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Clear(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<super::HttpMethod>>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Clear)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMany(&self, startindex: u32, items: &mut [::core::option::Option<super::HttpMethod>]) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<super::HttpMethod>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetMany)(::windows::core::Vtable::as_raw(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ReplaceAll(&self, items: &[::core::option::Option<super::HttpMethod>]) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<super::HttpMethod>>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).ReplaceAll)(::windows::core::Vtable::as_raw(this), items.len() as u32, ::core::mem::transmute(items.as_ptr())).ok() }
    }
}
impl ::core::clone::Clone for HttpMethodHeaderValueCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HttpMethodHeaderValueCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpMethodHeaderValueCollection {}
impl ::core::fmt::Debug for HttpMethodHeaderValueCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HttpMethodHeaderValueCollection").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for HttpMethodHeaderValueCollection {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Headers.HttpMethodHeaderValueCollection;{43bc3ff4-6119-4adf-938c-34bfffcf92ed})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for HttpMethodHeaderValueCollection {
    type Vtable = IHttpMethodHeaderValueCollection_Vtbl;
}
unsafe impl ::windows::core::Interface for HttpMethodHeaderValueCollection {
    const IID: ::windows::core::GUID = <IHttpMethodHeaderValueCollection as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for HttpMethodHeaderValueCollection {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpMethodHeaderValueCollection";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for HttpMethodHeaderValueCollection {
    type Item = super::HttpMethod;
    type IntoIter = super::super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for &HttpMethodHeaderValueCollection {
    type Item = super::HttpMethod;
    type IntoIter = super::super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        super::super::super::Foundation::Collections::VectorIterator::new(::core::convert::TryInto::try_into(self).ok())
    }
}
::windows::core::interface_hierarchy!(HttpMethodHeaderValueCollection, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<HttpMethodHeaderValueCollection> for super::super::super::Foundation::Collections::IIterable<super::HttpMethod> {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpMethodHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&HttpMethodHeaderValueCollection> for super::super::super::Foundation::Collections::IIterable<super::HttpMethod> {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpMethodHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&HttpMethodHeaderValueCollection> for ::windows::core::InParam<super::super::super::Foundation::Collections::IIterable<super::HttpMethod>> {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpMethodHeaderValueCollection) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<HttpMethodHeaderValueCollection> for super::super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpMethodHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&HttpMethodHeaderValueCollection> for super::super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpMethodHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&HttpMethodHeaderValueCollection> for ::windows::core::InParam<super::super::super::Foundation::IStringable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpMethodHeaderValueCollection) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<HttpMethodHeaderValueCollection> for super::super::super::Foundation::Collections::IVector<super::HttpMethod> {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpMethodHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&HttpMethodHeaderValueCollection> for super::super::super::Foundation::Collections::IVector<super::HttpMethod> {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpMethodHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&HttpMethodHeaderValueCollection> for ::windows::core::InParam<super::super::super::Foundation::Collections::IVector<super::HttpMethod>> {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpMethodHeaderValueCollection) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for HttpMethodHeaderValueCollection {}
unsafe impl ::core::marker::Sync for HttpMethodHeaderValueCollection {}
#[doc = "*Required features: `\"Web_Http_Headers\"`*"]
#[repr(transparent)]
pub struct HttpNameValueHeaderValue(::windows::core::IUnknown);
impl HttpNameValueHeaderValue {
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Name)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Value(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Value)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetValue(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetValue)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn CreateFromName(name: &::windows::core::HSTRING) -> ::windows::core::Result<HttpNameValueHeaderValue> {
        Self::IHttpNameValueHeaderValueFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateFromName)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(name), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn CreateFromNameWithValue(name: &::windows::core::HSTRING, value: &::windows::core::HSTRING) -> ::windows::core::Result<HttpNameValueHeaderValue> {
        Self::IHttpNameValueHeaderValueFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateFromNameWithValue)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(name), ::core::mem::transmute_copy(value), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn Parse(input: &::windows::core::HSTRING) -> ::windows::core::Result<HttpNameValueHeaderValue> {
        Self::IHttpNameValueHeaderValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Parse)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(input), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn TryParse(input: &::windows::core::HSTRING, namevalueheadervalue: &mut ::core::option::Option<HttpNameValueHeaderValue>) -> ::windows::core::Result<bool> {
        Self::IHttpNameValueHeaderValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TryParse)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(input), namevalueheadervalue as *mut _ as _, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ToString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ToString)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc(hidden)]
    pub fn IHttpNameValueHeaderValueFactory<R, F: FnOnce(&IHttpNameValueHeaderValueFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<HttpNameValueHeaderValue, IHttpNameValueHeaderValueFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IHttpNameValueHeaderValueStatics<R, F: FnOnce(&IHttpNameValueHeaderValueStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<HttpNameValueHeaderValue, IHttpNameValueHeaderValueStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for HttpNameValueHeaderValue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HttpNameValueHeaderValue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpNameValueHeaderValue {}
impl ::core::fmt::Debug for HttpNameValueHeaderValue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HttpNameValueHeaderValue").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for HttpNameValueHeaderValue {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Headers.HttpNameValueHeaderValue;{d8ba7463-5b9a-4d1b-93f9-aa5b44ecfddf})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for HttpNameValueHeaderValue {
    type Vtable = IHttpNameValueHeaderValue_Vtbl;
}
unsafe impl ::windows::core::Interface for HttpNameValueHeaderValue {
    const IID: ::windows::core::GUID = <IHttpNameValueHeaderValue as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for HttpNameValueHeaderValue {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpNameValueHeaderValue";
}
::windows::core::interface_hierarchy!(HttpNameValueHeaderValue, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<HttpNameValueHeaderValue> for super::super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpNameValueHeaderValue) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&HttpNameValueHeaderValue> for super::super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpNameValueHeaderValue) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&HttpNameValueHeaderValue> for ::windows::core::InParam<super::super::super::Foundation::IStringable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpNameValueHeaderValue) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for HttpNameValueHeaderValue {}
unsafe impl ::core::marker::Sync for HttpNameValueHeaderValue {}
#[doc = "*Required features: `\"Web_Http_Headers\"`*"]
#[repr(transparent)]
pub struct HttpProductHeaderValue(::windows::core::IUnknown);
impl HttpProductHeaderValue {
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Name)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Version(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Version)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CreateFromName(productname: &::windows::core::HSTRING) -> ::windows::core::Result<HttpProductHeaderValue> {
        Self::IHttpProductHeaderValueFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateFromName)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(productname), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn CreateFromNameWithVersion(productname: &::windows::core::HSTRING, productversion: &::windows::core::HSTRING) -> ::windows::core::Result<HttpProductHeaderValue> {
        Self::IHttpProductHeaderValueFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateFromNameWithVersion)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(productname), ::core::mem::transmute_copy(productversion), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn Parse(input: &::windows::core::HSTRING) -> ::windows::core::Result<HttpProductHeaderValue> {
        Self::IHttpProductHeaderValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Parse)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(input), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn TryParse(input: &::windows::core::HSTRING, productheadervalue: &mut ::core::option::Option<HttpProductHeaderValue>) -> ::windows::core::Result<bool> {
        Self::IHttpProductHeaderValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TryParse)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(input), productheadervalue as *mut _ as _, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ToString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ToString)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc(hidden)]
    pub fn IHttpProductHeaderValueFactory<R, F: FnOnce(&IHttpProductHeaderValueFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<HttpProductHeaderValue, IHttpProductHeaderValueFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IHttpProductHeaderValueStatics<R, F: FnOnce(&IHttpProductHeaderValueStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<HttpProductHeaderValue, IHttpProductHeaderValueStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for HttpProductHeaderValue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HttpProductHeaderValue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpProductHeaderValue {}
impl ::core::fmt::Debug for HttpProductHeaderValue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HttpProductHeaderValue").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for HttpProductHeaderValue {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Headers.HttpProductHeaderValue;{f4feee03-ebd4-4160-b9ff-807c5183b6e6})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for HttpProductHeaderValue {
    type Vtable = IHttpProductHeaderValue_Vtbl;
}
unsafe impl ::windows::core::Interface for HttpProductHeaderValue {
    const IID: ::windows::core::GUID = <IHttpProductHeaderValue as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for HttpProductHeaderValue {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpProductHeaderValue";
}
::windows::core::interface_hierarchy!(HttpProductHeaderValue, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<HttpProductHeaderValue> for super::super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpProductHeaderValue) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&HttpProductHeaderValue> for super::super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpProductHeaderValue) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&HttpProductHeaderValue> for ::windows::core::InParam<super::super::super::Foundation::IStringable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpProductHeaderValue) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for HttpProductHeaderValue {}
unsafe impl ::core::marker::Sync for HttpProductHeaderValue {}
#[doc = "*Required features: `\"Web_Http_Headers\"`*"]
#[repr(transparent)]
pub struct HttpProductInfoHeaderValue(::windows::core::IUnknown);
impl HttpProductInfoHeaderValue {
    pub fn Product(&self) -> ::windows::core::Result<HttpProductHeaderValue> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Product)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Comment(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Comment)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CreateFromComment(productcomment: &::windows::core::HSTRING) -> ::windows::core::Result<HttpProductInfoHeaderValue> {
        Self::IHttpProductInfoHeaderValueFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateFromComment)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(productcomment), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn CreateFromNameWithVersion(productname: &::windows::core::HSTRING, productversion: &::windows::core::HSTRING) -> ::windows::core::Result<HttpProductInfoHeaderValue> {
        Self::IHttpProductInfoHeaderValueFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateFromNameWithVersion)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(productname), ::core::mem::transmute_copy(productversion), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn Parse(input: &::windows::core::HSTRING) -> ::windows::core::Result<HttpProductInfoHeaderValue> {
        Self::IHttpProductInfoHeaderValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Parse)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(input), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn TryParse(input: &::windows::core::HSTRING, productinfoheadervalue: &mut ::core::option::Option<HttpProductInfoHeaderValue>) -> ::windows::core::Result<bool> {
        Self::IHttpProductInfoHeaderValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TryParse)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(input), productinfoheadervalue as *mut _ as _, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ToString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ToString)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc(hidden)]
    pub fn IHttpProductInfoHeaderValueFactory<R, F: FnOnce(&IHttpProductInfoHeaderValueFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<HttpProductInfoHeaderValue, IHttpProductInfoHeaderValueFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IHttpProductInfoHeaderValueStatics<R, F: FnOnce(&IHttpProductInfoHeaderValueStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<HttpProductInfoHeaderValue, IHttpProductInfoHeaderValueStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for HttpProductInfoHeaderValue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HttpProductInfoHeaderValue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpProductInfoHeaderValue {}
impl ::core::fmt::Debug for HttpProductInfoHeaderValue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HttpProductInfoHeaderValue").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for HttpProductInfoHeaderValue {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Headers.HttpProductInfoHeaderValue;{1b1a8732-4c35-486a-966f-646489198e4d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for HttpProductInfoHeaderValue {
    type Vtable = IHttpProductInfoHeaderValue_Vtbl;
}
unsafe impl ::windows::core::Interface for HttpProductInfoHeaderValue {
    const IID: ::windows::core::GUID = <IHttpProductInfoHeaderValue as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for HttpProductInfoHeaderValue {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpProductInfoHeaderValue";
}
::windows::core::interface_hierarchy!(HttpProductInfoHeaderValue, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<HttpProductInfoHeaderValue> for super::super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpProductInfoHeaderValue) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&HttpProductInfoHeaderValue> for super::super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpProductInfoHeaderValue) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&HttpProductInfoHeaderValue> for ::windows::core::InParam<super::super::super::Foundation::IStringable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpProductInfoHeaderValue) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for HttpProductInfoHeaderValue {}
unsafe impl ::core::marker::Sync for HttpProductInfoHeaderValue {}
#[doc = "*Required features: `\"Web_Http_Headers\"`*"]
#[repr(transparent)]
pub struct HttpProductInfoHeaderValueCollection(::windows::core::IUnknown);
impl HttpProductInfoHeaderValueCollection {
    pub fn ParseAdd(&self, input: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).ParseAdd)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(input)).ok() }
    }
    pub fn TryParseAdd(&self, input: &::windows::core::HSTRING) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TryParseAdd)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(input), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IIterator<HttpProductInfoHeaderValue>> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IIterable<HttpProductInfoHeaderValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).First)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ToString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ToString)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAt(&self, index: u32) -> ::windows::core::Result<HttpProductInfoHeaderValue> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpProductInfoHeaderValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetAt)(::windows::core::Vtable::as_raw(this), index, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpProductInfoHeaderValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Size)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetView(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<HttpProductInfoHeaderValue>> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpProductInfoHeaderValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetView)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn IndexOf(&self, value: &HttpProductInfoHeaderValue, index: &mut u32) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpProductInfoHeaderValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IndexOf)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value), index, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetAt(&self, index: u32, value: &HttpProductInfoHeaderValue) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpProductInfoHeaderValue>>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetAt)(::windows::core::Vtable::as_raw(this), index, ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn InsertAt(&self, index: u32, value: &HttpProductInfoHeaderValue) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpProductInfoHeaderValue>>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).InsertAt)(::windows::core::Vtable::as_raw(this), index, ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAt(&self, index: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpProductInfoHeaderValue>>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveAt)(::windows::core::Vtable::as_raw(this), index).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Append(&self, value: &HttpProductInfoHeaderValue) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpProductInfoHeaderValue>>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Append)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAtEnd(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpProductInfoHeaderValue>>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveAtEnd)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Clear(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpProductInfoHeaderValue>>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Clear)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMany(&self, startindex: u32, items: &mut [::core::option::Option<HttpProductInfoHeaderValue>]) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpProductInfoHeaderValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetMany)(::windows::core::Vtable::as_raw(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ReplaceAll(&self, items: &[::core::option::Option<HttpProductInfoHeaderValue>]) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpProductInfoHeaderValue>>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).ReplaceAll)(::windows::core::Vtable::as_raw(this), items.len() as u32, ::core::mem::transmute(items.as_ptr())).ok() }
    }
}
impl ::core::clone::Clone for HttpProductInfoHeaderValueCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HttpProductInfoHeaderValueCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpProductInfoHeaderValueCollection {}
impl ::core::fmt::Debug for HttpProductInfoHeaderValueCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HttpProductInfoHeaderValueCollection").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for HttpProductInfoHeaderValueCollection {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Headers.HttpProductInfoHeaderValueCollection;{877df74a-d69b-44f8-ad4f-453af9c42ed0})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for HttpProductInfoHeaderValueCollection {
    type Vtable = IHttpProductInfoHeaderValueCollection_Vtbl;
}
unsafe impl ::windows::core::Interface for HttpProductInfoHeaderValueCollection {
    const IID: ::windows::core::GUID = <IHttpProductInfoHeaderValueCollection as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for HttpProductInfoHeaderValueCollection {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpProductInfoHeaderValueCollection";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for HttpProductInfoHeaderValueCollection {
    type Item = HttpProductInfoHeaderValue;
    type IntoIter = super::super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for &HttpProductInfoHeaderValueCollection {
    type Item = HttpProductInfoHeaderValue;
    type IntoIter = super::super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        super::super::super::Foundation::Collections::VectorIterator::new(::core::convert::TryInto::try_into(self).ok())
    }
}
::windows::core::interface_hierarchy!(HttpProductInfoHeaderValueCollection, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<HttpProductInfoHeaderValueCollection> for super::super::super::Foundation::Collections::IIterable<HttpProductInfoHeaderValue> {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpProductInfoHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&HttpProductInfoHeaderValueCollection> for super::super::super::Foundation::Collections::IIterable<HttpProductInfoHeaderValue> {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpProductInfoHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&HttpProductInfoHeaderValueCollection> for ::windows::core::InParam<super::super::super::Foundation::Collections::IIterable<HttpProductInfoHeaderValue>> {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpProductInfoHeaderValueCollection) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<HttpProductInfoHeaderValueCollection> for super::super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpProductInfoHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&HttpProductInfoHeaderValueCollection> for super::super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpProductInfoHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&HttpProductInfoHeaderValueCollection> for ::windows::core::InParam<super::super::super::Foundation::IStringable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpProductInfoHeaderValueCollection) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<HttpProductInfoHeaderValueCollection> for super::super::super::Foundation::Collections::IVector<HttpProductInfoHeaderValue> {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpProductInfoHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&HttpProductInfoHeaderValueCollection> for super::super::super::Foundation::Collections::IVector<HttpProductInfoHeaderValue> {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpProductInfoHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&HttpProductInfoHeaderValueCollection> for ::windows::core::InParam<super::super::super::Foundation::Collections::IVector<HttpProductInfoHeaderValue>> {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpProductInfoHeaderValueCollection) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for HttpProductInfoHeaderValueCollection {}
unsafe impl ::core::marker::Sync for HttpProductInfoHeaderValueCollection {}
#[doc = "*Required features: `\"Web_Http_Headers\"`*"]
#[repr(transparent)]
pub struct HttpRequestHeaderCollection(::windows::core::IUnknown);
impl HttpRequestHeaderCollection {
    pub fn Accept(&self) -> ::windows::core::Result<HttpMediaTypeWithQualityHeaderValueCollection> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Accept)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn AcceptEncoding(&self) -> ::windows::core::Result<HttpContentCodingWithQualityHeaderValueCollection> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AcceptEncoding)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn AcceptLanguage(&self) -> ::windows::core::Result<HttpLanguageRangeWithQualityHeaderValueCollection> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AcceptLanguage)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Authorization(&self) -> ::windows::core::Result<HttpCredentialsHeaderValue> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Authorization)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetAuthorization(&self, value: &HttpCredentialsHeaderValue) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetAuthorization)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn CacheControl(&self) -> ::windows::core::Result<HttpCacheDirectiveHeaderValueCollection> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CacheControl)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Connection(&self) -> ::windows::core::Result<HttpConnectionOptionHeaderValueCollection> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Connection)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Cookie(&self) -> ::windows::core::Result<HttpCookiePairHeaderValueCollection> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Cookie)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Date(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Date)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetDate<P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetDate)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    pub fn Expect(&self) -> ::windows::core::Result<HttpExpectationHeaderValueCollection> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Expect)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn From(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).From)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetFrom(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetFrom)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Networking\"`*"]
    #[cfg(feature = "Networking")]
    pub fn Host(&self) -> ::windows::core::Result<super::super::super::Networking::HostName> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Host)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Networking\"`*"]
    #[cfg(feature = "Networking")]
    pub fn SetHost(&self, value: &super::super::super::Networking::HostName) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetHost)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn IfModifiedSince(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IfModifiedSince)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetIfModifiedSince<P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetIfModifiedSince)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn IfUnmodifiedSince(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IfUnmodifiedSince)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetIfUnmodifiedSince<P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetIfUnmodifiedSince)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MaxForwards(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<u32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MaxForwards)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetMaxForwards<P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::super::Foundation::IReference<u32>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetMaxForwards)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    pub fn ProxyAuthorization(&self) -> ::windows::core::Result<HttpCredentialsHeaderValue> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ProxyAuthorization)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetProxyAuthorization(&self, value: &HttpCredentialsHeaderValue) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetProxyAuthorization)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Referer(&self) -> ::windows::core::Result<super::super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Referer)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetReferer(&self, value: &super::super::super::Foundation::Uri) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetReferer)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn TransferEncoding(&self) -> ::windows::core::Result<HttpTransferCodingHeaderValueCollection> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TransferEncoding)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn UserAgent(&self) -> ::windows::core::Result<HttpProductInfoHeaderValueCollection> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).UserAgent)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Append(&self, name: &::windows::core::HSTRING, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Append)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(name), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn TryAppendWithoutValidation(&self, name: &::windows::core::HSTRING, value: &::windows::core::HSTRING) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TryAppendWithoutValidation)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(name), ::core::mem::transmute_copy(value), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IIterator<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::HSTRING>>> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::HSTRING>>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).First)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Lookup(&self, key: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Lookup)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(key), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Size)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn HasKey(&self, key: &::windows::core::HSTRING) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).HasKey)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(key), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetView(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::HSTRING>> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetView)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Insert(&self, key: &::windows::core::HSTRING, value: &::windows::core::HSTRING) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Insert)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(key), ::core::mem::transmute_copy(value), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Remove(&self, key: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Remove)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(key)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Clear(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Clear)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ToString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ToString)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for HttpRequestHeaderCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HttpRequestHeaderCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpRequestHeaderCollection {}
impl ::core::fmt::Debug for HttpRequestHeaderCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HttpRequestHeaderCollection").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for HttpRequestHeaderCollection {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Headers.HttpRequestHeaderCollection;{af40329b-b544-469b-86b9-ac3d466fea36})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for HttpRequestHeaderCollection {
    type Vtable = IHttpRequestHeaderCollection_Vtbl;
}
unsafe impl ::windows::core::Interface for HttpRequestHeaderCollection {
    const IID: ::windows::core::GUID = <IHttpRequestHeaderCollection as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for HttpRequestHeaderCollection {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpRequestHeaderCollection";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for HttpRequestHeaderCollection {
    type Item = super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::HSTRING>;
    type IntoIter = super::super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for &HttpRequestHeaderCollection {
    type Item = super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::HSTRING>;
    type IntoIter = super::super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
::windows::core::interface_hierarchy!(HttpRequestHeaderCollection, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<HttpRequestHeaderCollection> for super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::HSTRING>> {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpRequestHeaderCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&HttpRequestHeaderCollection> for super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::HSTRING>> {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpRequestHeaderCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&HttpRequestHeaderCollection> for ::windows::core::InParam<super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::HSTRING>>> {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpRequestHeaderCollection) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<HttpRequestHeaderCollection> for super::super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING> {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpRequestHeaderCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&HttpRequestHeaderCollection> for super::super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING> {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpRequestHeaderCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&HttpRequestHeaderCollection> for ::windows::core::InParam<super::super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>> {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpRequestHeaderCollection) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<HttpRequestHeaderCollection> for super::super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpRequestHeaderCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&HttpRequestHeaderCollection> for super::super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpRequestHeaderCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&HttpRequestHeaderCollection> for ::windows::core::InParam<super::super::super::Foundation::IStringable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpRequestHeaderCollection) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for HttpRequestHeaderCollection {}
unsafe impl ::core::marker::Sync for HttpRequestHeaderCollection {}
#[doc = "*Required features: `\"Web_Http_Headers\"`*"]
#[repr(transparent)]
pub struct HttpResponseHeaderCollection(::windows::core::IUnknown);
impl HttpResponseHeaderCollection {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Age(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Age)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetAge<P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetAge)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    pub fn Allow(&self) -> ::windows::core::Result<HttpMethodHeaderValueCollection> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Allow)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CacheControl(&self) -> ::windows::core::Result<HttpCacheDirectiveHeaderValueCollection> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CacheControl)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Connection(&self) -> ::windows::core::Result<HttpConnectionOptionHeaderValueCollection> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Connection)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Date(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Date)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetDate<P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetDate)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Location(&self) -> ::windows::core::Result<super::super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Location)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetLocation(&self, value: &super::super::super::Foundation::Uri) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetLocation)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn ProxyAuthenticate(&self) -> ::windows::core::Result<HttpChallengeHeaderValueCollection> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ProxyAuthenticate)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn RetryAfter(&self) -> ::windows::core::Result<HttpDateOrDeltaHeaderValue> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RetryAfter)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetRetryAfter(&self, value: &HttpDateOrDeltaHeaderValue) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetRetryAfter)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn TransferEncoding(&self) -> ::windows::core::Result<HttpTransferCodingHeaderValueCollection> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TransferEncoding)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn WwwAuthenticate(&self) -> ::windows::core::Result<HttpChallengeHeaderValueCollection> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).WwwAuthenticate)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Append(&self, name: &::windows::core::HSTRING, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Append)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(name), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn TryAppendWithoutValidation(&self, name: &::windows::core::HSTRING, value: &::windows::core::HSTRING) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TryAppendWithoutValidation)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(name), ::core::mem::transmute_copy(value), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IIterator<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::HSTRING>>> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::HSTRING>>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).First)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Lookup(&self, key: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Lookup)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(key), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Size)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn HasKey(&self, key: &::windows::core::HSTRING) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).HasKey)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(key), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetView(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::HSTRING>> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetView)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Insert(&self, key: &::windows::core::HSTRING, value: &::windows::core::HSTRING) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Insert)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(key), ::core::mem::transmute_copy(value), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Remove(&self, key: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Remove)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(key)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Clear(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Clear)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ToString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ToString)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for HttpResponseHeaderCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HttpResponseHeaderCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpResponseHeaderCollection {}
impl ::core::fmt::Debug for HttpResponseHeaderCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HttpResponseHeaderCollection").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for HttpResponseHeaderCollection {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Headers.HttpResponseHeaderCollection;{7a990969-fa3f-41ed-aac6-bf957975c16b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for HttpResponseHeaderCollection {
    type Vtable = IHttpResponseHeaderCollection_Vtbl;
}
unsafe impl ::windows::core::Interface for HttpResponseHeaderCollection {
    const IID: ::windows::core::GUID = <IHttpResponseHeaderCollection as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for HttpResponseHeaderCollection {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpResponseHeaderCollection";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for HttpResponseHeaderCollection {
    type Item = super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::HSTRING>;
    type IntoIter = super::super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for &HttpResponseHeaderCollection {
    type Item = super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::HSTRING>;
    type IntoIter = super::super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
::windows::core::interface_hierarchy!(HttpResponseHeaderCollection, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<HttpResponseHeaderCollection> for super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::HSTRING>> {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpResponseHeaderCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&HttpResponseHeaderCollection> for super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::HSTRING>> {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpResponseHeaderCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&HttpResponseHeaderCollection> for ::windows::core::InParam<super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::HSTRING>>> {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpResponseHeaderCollection) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<HttpResponseHeaderCollection> for super::super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING> {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpResponseHeaderCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&HttpResponseHeaderCollection> for super::super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING> {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpResponseHeaderCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&HttpResponseHeaderCollection> for ::windows::core::InParam<super::super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>> {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpResponseHeaderCollection) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<HttpResponseHeaderCollection> for super::super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpResponseHeaderCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&HttpResponseHeaderCollection> for super::super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpResponseHeaderCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&HttpResponseHeaderCollection> for ::windows::core::InParam<super::super::super::Foundation::IStringable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpResponseHeaderCollection) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for HttpResponseHeaderCollection {}
unsafe impl ::core::marker::Sync for HttpResponseHeaderCollection {}
#[doc = "*Required features: `\"Web_Http_Headers\"`*"]
#[repr(transparent)]
pub struct HttpTransferCodingHeaderValue(::windows::core::IUnknown);
impl HttpTransferCodingHeaderValue {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Parameters(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<HttpNameValueHeaderValue>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Parameters)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Value(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Value)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Create(input: &::windows::core::HSTRING) -> ::windows::core::Result<HttpTransferCodingHeaderValue> {
        Self::IHttpTransferCodingHeaderValueFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Create)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(input), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn Parse(input: &::windows::core::HSTRING) -> ::windows::core::Result<HttpTransferCodingHeaderValue> {
        Self::IHttpTransferCodingHeaderValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Parse)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(input), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn TryParse(input: &::windows::core::HSTRING, transfercodingheadervalue: &mut ::core::option::Option<HttpTransferCodingHeaderValue>) -> ::windows::core::Result<bool> {
        Self::IHttpTransferCodingHeaderValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TryParse)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(input), transfercodingheadervalue as *mut _ as _, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ToString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ToString)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc(hidden)]
    pub fn IHttpTransferCodingHeaderValueFactory<R, F: FnOnce(&IHttpTransferCodingHeaderValueFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<HttpTransferCodingHeaderValue, IHttpTransferCodingHeaderValueFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IHttpTransferCodingHeaderValueStatics<R, F: FnOnce(&IHttpTransferCodingHeaderValueStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<HttpTransferCodingHeaderValue, IHttpTransferCodingHeaderValueStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for HttpTransferCodingHeaderValue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HttpTransferCodingHeaderValue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpTransferCodingHeaderValue {}
impl ::core::fmt::Debug for HttpTransferCodingHeaderValue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HttpTransferCodingHeaderValue").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for HttpTransferCodingHeaderValue {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Headers.HttpTransferCodingHeaderValue;{436f32f9-3ded-42bd-b38a-5496a2511ce6})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for HttpTransferCodingHeaderValue {
    type Vtable = IHttpTransferCodingHeaderValue_Vtbl;
}
unsafe impl ::windows::core::Interface for HttpTransferCodingHeaderValue {
    const IID: ::windows::core::GUID = <IHttpTransferCodingHeaderValue as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for HttpTransferCodingHeaderValue {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpTransferCodingHeaderValue";
}
::windows::core::interface_hierarchy!(HttpTransferCodingHeaderValue, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<HttpTransferCodingHeaderValue> for super::super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpTransferCodingHeaderValue) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&HttpTransferCodingHeaderValue> for super::super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpTransferCodingHeaderValue) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&HttpTransferCodingHeaderValue> for ::windows::core::InParam<super::super::super::Foundation::IStringable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpTransferCodingHeaderValue) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for HttpTransferCodingHeaderValue {}
unsafe impl ::core::marker::Sync for HttpTransferCodingHeaderValue {}
#[doc = "*Required features: `\"Web_Http_Headers\"`*"]
#[repr(transparent)]
pub struct HttpTransferCodingHeaderValueCollection(::windows::core::IUnknown);
impl HttpTransferCodingHeaderValueCollection {
    pub fn ParseAdd(&self, input: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).ParseAdd)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(input)).ok() }
    }
    pub fn TryParseAdd(&self, input: &::windows::core::HSTRING) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TryParseAdd)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(input), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IIterator<HttpTransferCodingHeaderValue>> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IIterable<HttpTransferCodingHeaderValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).First)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ToString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ToString)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAt(&self, index: u32) -> ::windows::core::Result<HttpTransferCodingHeaderValue> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpTransferCodingHeaderValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetAt)(::windows::core::Vtable::as_raw(this), index, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpTransferCodingHeaderValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Size)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetView(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<HttpTransferCodingHeaderValue>> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpTransferCodingHeaderValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetView)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn IndexOf(&self, value: &HttpTransferCodingHeaderValue, index: &mut u32) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpTransferCodingHeaderValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IndexOf)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value), index, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetAt(&self, index: u32, value: &HttpTransferCodingHeaderValue) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpTransferCodingHeaderValue>>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetAt)(::windows::core::Vtable::as_raw(this), index, ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn InsertAt(&self, index: u32, value: &HttpTransferCodingHeaderValue) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpTransferCodingHeaderValue>>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).InsertAt)(::windows::core::Vtable::as_raw(this), index, ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAt(&self, index: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpTransferCodingHeaderValue>>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveAt)(::windows::core::Vtable::as_raw(this), index).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Append(&self, value: &HttpTransferCodingHeaderValue) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpTransferCodingHeaderValue>>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Append)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAtEnd(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpTransferCodingHeaderValue>>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveAtEnd)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Clear(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpTransferCodingHeaderValue>>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Clear)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMany(&self, startindex: u32, items: &mut [::core::option::Option<HttpTransferCodingHeaderValue>]) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpTransferCodingHeaderValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetMany)(::windows::core::Vtable::as_raw(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ReplaceAll(&self, items: &[::core::option::Option<HttpTransferCodingHeaderValue>]) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<HttpTransferCodingHeaderValue>>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).ReplaceAll)(::windows::core::Vtable::as_raw(this), items.len() as u32, ::core::mem::transmute(items.as_ptr())).ok() }
    }
}
impl ::core::clone::Clone for HttpTransferCodingHeaderValueCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HttpTransferCodingHeaderValueCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HttpTransferCodingHeaderValueCollection {}
impl ::core::fmt::Debug for HttpTransferCodingHeaderValueCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HttpTransferCodingHeaderValueCollection").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for HttpTransferCodingHeaderValueCollection {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Web.Http.Headers.HttpTransferCodingHeaderValueCollection;{202c8c34-2c03-49b8-9665-73e27cb2fc79})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for HttpTransferCodingHeaderValueCollection {
    type Vtable = IHttpTransferCodingHeaderValueCollection_Vtbl;
}
unsafe impl ::windows::core::Interface for HttpTransferCodingHeaderValueCollection {
    const IID: ::windows::core::GUID = <IHttpTransferCodingHeaderValueCollection as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for HttpTransferCodingHeaderValueCollection {
    const NAME: &'static str = "Windows.Web.Http.Headers.HttpTransferCodingHeaderValueCollection";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for HttpTransferCodingHeaderValueCollection {
    type Item = HttpTransferCodingHeaderValue;
    type IntoIter = super::super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for &HttpTransferCodingHeaderValueCollection {
    type Item = HttpTransferCodingHeaderValue;
    type IntoIter = super::super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        super::super::super::Foundation::Collections::VectorIterator::new(::core::convert::TryInto::try_into(self).ok())
    }
}
::windows::core::interface_hierarchy!(HttpTransferCodingHeaderValueCollection, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<HttpTransferCodingHeaderValueCollection> for super::super::super::Foundation::Collections::IIterable<HttpTransferCodingHeaderValue> {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpTransferCodingHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&HttpTransferCodingHeaderValueCollection> for super::super::super::Foundation::Collections::IIterable<HttpTransferCodingHeaderValue> {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpTransferCodingHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&HttpTransferCodingHeaderValueCollection> for ::windows::core::InParam<super::super::super::Foundation::Collections::IIterable<HttpTransferCodingHeaderValue>> {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpTransferCodingHeaderValueCollection) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<HttpTransferCodingHeaderValueCollection> for super::super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpTransferCodingHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&HttpTransferCodingHeaderValueCollection> for super::super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpTransferCodingHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&HttpTransferCodingHeaderValueCollection> for ::windows::core::InParam<super::super::super::Foundation::IStringable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpTransferCodingHeaderValueCollection) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<HttpTransferCodingHeaderValueCollection> for super::super::super::Foundation::Collections::IVector<HttpTransferCodingHeaderValue> {
    type Error = ::windows::core::Error;
    fn try_from(value: HttpTransferCodingHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&HttpTransferCodingHeaderValueCollection> for super::super::super::Foundation::Collections::IVector<HttpTransferCodingHeaderValue> {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpTransferCodingHeaderValueCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&HttpTransferCodingHeaderValueCollection> for ::windows::core::InParam<super::super::super::Foundation::Collections::IVector<HttpTransferCodingHeaderValue>> {
    type Error = ::windows::core::Error;
    fn try_from(value: &HttpTransferCodingHeaderValueCollection) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for HttpTransferCodingHeaderValueCollection {}
unsafe impl ::core::marker::Sync for HttpTransferCodingHeaderValueCollection {}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
