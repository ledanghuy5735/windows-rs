#[doc(hidden)]
#[repr(transparent)]
pub struct IAdvertisingManagerForUser(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IAdvertisingManagerForUser {
    type Vtable = IAdvertisingManagerForUser_Vtbl;
}
unsafe impl ::windows::core::Interface for IAdvertisingManagerForUser {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x928bf3d0_cf7c_4ab0_a7dc_6dc5bcd44252);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdvertisingManagerForUser_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub AdvertisingId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub User: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAdvertisingManagerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IAdvertisingManagerStatics {
    type Vtable = IAdvertisingManagerStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IAdvertisingManagerStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xadd3468c_a273_48cb_b346_3544522d5581);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdvertisingManagerStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub AdvertisingId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAdvertisingManagerStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IAdvertisingManagerStatics2 {
    type Vtable = IAdvertisingManagerStatics2_Vtbl;
}
unsafe impl ::windows::core::Interface for IAdvertisingManagerStatics2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdd0947af_1a6d_46b0_95bc_f3f9d6beb9fb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdvertisingManagerStatics2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetForUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAssignedAccessSettings(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IAssignedAccessSettings {
    type Vtable = IAssignedAccessSettings_Vtbl;
}
unsafe impl ::windows::core::Interface for IAssignedAccessSettings {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1bc57f1c_e971_5757_b8e0_512f8b8c46d2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAssignedAccessSettings_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub IsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsSingleAppKioskMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub User: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAssignedAccessSettingsStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IAssignedAccessSettingsStatics {
    type Vtable = IAssignedAccessSettingsStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IAssignedAccessSettingsStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x34a81d0d_8a29_5ef3_a7be_618e6ac3bd01);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAssignedAccessSettingsStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetForUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDiagnosticsSettings(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IDiagnosticsSettings {
    type Vtable = IDiagnosticsSettings_Vtbl;
}
unsafe impl ::windows::core::Interface for IDiagnosticsSettings {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe5e9eccd_2711_44e0_973c_491d78048d24);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDiagnosticsSettings_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CanUseDiagnosticsToTailorExperiences: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub User: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDiagnosticsSettingsStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IDiagnosticsSettingsStatics {
    type Vtable = IDiagnosticsSettingsStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IDiagnosticsSettingsStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x72d2e80f_5390_4793_990b_3ccc7d6ac9c8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDiagnosticsSettingsStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetForUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFirstSignInSettings(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IFirstSignInSettings {
    type Vtable = IFirstSignInSettings_Vtbl;
}
unsafe impl ::windows::core::Interface for IFirstSignInSettings {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3e945153_3a5e_452e_a601_f5baad2a4870);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFirstSignInSettings_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFirstSignInSettingsStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IFirstSignInSettingsStatics {
    type Vtable = IFirstSignInSettingsStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IFirstSignInSettingsStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1ce18f0f_1c41_4ea0_b7a2_6f0c1c7e8438);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFirstSignInSettingsStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGlobalizationPreferencesForUser(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IGlobalizationPreferencesForUser {
    type Vtable = IGlobalizationPreferencesForUser_Vtbl;
}
unsafe impl ::windows::core::Interface for IGlobalizationPreferencesForUser {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x150f0795_4f6e_40ba_a010_e27d81bda7f5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGlobalizationPreferencesForUser_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub User: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Calendars: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Calendars: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Clocks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Clocks: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Currencies: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Currencies: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Languages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Languages: usize,
    pub HomeGeographicRegion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Globalization")]
    pub WeekStartsOn: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Globalization::DayOfWeek) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Globalization"))]
    WeekStartsOn: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGlobalizationPreferencesStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IGlobalizationPreferencesStatics {
    type Vtable = IGlobalizationPreferencesStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IGlobalizationPreferencesStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x01bf4326_ed37_4e96_b0e9_c1340d1ea158);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGlobalizationPreferencesStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Calendars: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Calendars: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Clocks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Clocks: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Currencies: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Currencies: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Languages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Languages: usize,
    pub HomeGeographicRegion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Globalization")]
    pub WeekStartsOn: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Globalization::DayOfWeek) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Globalization"))]
    WeekStartsOn: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGlobalizationPreferencesStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IGlobalizationPreferencesStatics2 {
    type Vtable = IGlobalizationPreferencesStatics2_Vtbl;
}
unsafe impl ::windows::core::Interface for IGlobalizationPreferencesStatics2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfcce85f1_4300_4cd0_9cac_1a8e7b7e18f4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGlobalizationPreferencesStatics2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub TrySetHomeGeographicRegion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, region: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub TrySetLanguages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, languagetags: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    TrySetLanguages: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGlobalizationPreferencesStatics3(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IGlobalizationPreferencesStatics3 {
    type Vtable = IGlobalizationPreferencesStatics3_Vtbl;
}
unsafe impl ::windows::core::Interface for IGlobalizationPreferencesStatics3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1e059733_35f5_40d8_b9e8_aef3ef856fce);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGlobalizationPreferencesStatics3_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetForUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILockScreenImageFeedStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ILockScreenImageFeedStatics {
    type Vtable = ILockScreenImageFeedStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for ILockScreenImageFeedStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2c0d73f6_03a9_41a6_9b01_495251ff51d5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILockScreenImageFeedStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub RequestSetImageFeedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, syndicationfeeduri: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestSetImageFeedAsync: usize,
    pub TryRemoveImageFeed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILockScreenStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ILockScreenStatics {
    type Vtable = ILockScreenStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for ILockScreenStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3ee9d3ad_b607_40ae_b426_7631d9821269);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILockScreenStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub OriginalImageFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    OriginalImageFile: usize,
    #[cfg(feature = "Storage_Streams")]
    pub GetImageStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    GetImageStream: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub SetImageFileAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    SetImageFileAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub SetImageStreamAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    SetImageStreamAsync: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IUserInformationStatics(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Vtable for IUserInformationStatics {
    type Vtable = IUserInformationStatics_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for IUserInformationStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x77f3a910_48fa_489c_934e_2ae85ba8f772);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IUserInformationStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub AccountPictureChangeEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    AccountPictureChangeEnabled: usize,
    #[cfg(feature = "deprecated")]
    pub NameAccessAllowed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    NameAccessAllowed: usize,
    #[cfg(all(feature = "Storage", feature = "deprecated"))]
    pub GetAccountPicture: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, kind: AccountPictureKind, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Storage", feature = "deprecated")))]
    GetAccountPicture: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage", feature = "deprecated"))]
    pub SetAccountPictureAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, image: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage", feature = "deprecated")))]
    SetAccountPictureAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage", feature = "deprecated"))]
    pub SetAccountPicturesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, smallimage: *mut ::core::ffi::c_void, largeimage: *mut ::core::ffi::c_void, video: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage", feature = "deprecated")))]
    SetAccountPicturesAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "deprecated"))]
    pub SetAccountPictureFromStreamAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, image: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams", feature = "deprecated")))]
    SetAccountPictureFromStreamAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "deprecated"))]
    pub SetAccountPicturesFromStreamsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, smallimage: *mut ::core::ffi::c_void, largeimage: *mut ::core::ffi::c_void, video: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams", feature = "deprecated")))]
    SetAccountPicturesFromStreamsAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub AccountPictureChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, changehandler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    AccountPictureChanged: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveAccountPictureChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveAccountPictureChanged: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub GetDisplayNameAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    GetDisplayNameAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub GetFirstNameAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    GetFirstNameAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub GetLastNameAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    GetLastNameAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub GetPrincipalNameAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    GetPrincipalNameAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub GetSessionInitiationProtocolUriAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    GetSessionInitiationProtocolUriAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub GetDomainNameAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    GetDomainNameAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserProfilePersonalizationSettings(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IUserProfilePersonalizationSettings {
    type Vtable = IUserProfilePersonalizationSettings_Vtbl;
}
unsafe impl ::windows::core::Interface for IUserProfilePersonalizationSettings {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8ceddab4_7998_46d5_8dd3_184f1c5f9ab9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserProfilePersonalizationSettings_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub TrySetLockScreenImageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, imagefile: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    TrySetLockScreenImageAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub TrySetWallpaperImageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, imagefile: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    TrySetWallpaperImageAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserProfilePersonalizationSettingsStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IUserProfilePersonalizationSettingsStatics {
    type Vtable = IUserProfilePersonalizationSettingsStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IUserProfilePersonalizationSettingsStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x91acb841_5037_454b_9883_bb772d08dd16);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserProfilePersonalizationSettingsStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Current: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub IsSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"System_UserProfile\"`*"]
pub struct AdvertisingManager;
impl AdvertisingManager {
    pub fn AdvertisingId() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IAdvertisingManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AdvertisingId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn GetForUser(user: &super::User) -> ::windows::core::Result<AdvertisingManagerForUser> {
        Self::IAdvertisingManagerStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetForUser)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(user), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IAdvertisingManagerStatics<R, F: FnOnce(&IAdvertisingManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<AdvertisingManager, IAdvertisingManagerStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IAdvertisingManagerStatics2<R, F: FnOnce(&IAdvertisingManagerStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<AdvertisingManager, IAdvertisingManagerStatics2> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for AdvertisingManager {
    const NAME: &'static str = "Windows.System.UserProfile.AdvertisingManager";
}
#[doc = "*Required features: `\"System_UserProfile\"`*"]
#[repr(transparent)]
pub struct AdvertisingManagerForUser(::windows::core::IUnknown);
impl AdvertisingManagerForUser {
    pub fn AdvertisingId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AdvertisingId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn User(&self) -> ::windows::core::Result<super::User> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).User)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for AdvertisingManagerForUser {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AdvertisingManagerForUser {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AdvertisingManagerForUser {}
impl ::core::fmt::Debug for AdvertisingManagerForUser {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AdvertisingManagerForUser").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AdvertisingManagerForUser {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.UserProfile.AdvertisingManagerForUser;{928bf3d0-cf7c-4ab0-a7dc-6dc5bcd44252})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for AdvertisingManagerForUser {
    type Vtable = IAdvertisingManagerForUser_Vtbl;
}
unsafe impl ::windows::core::Interface for AdvertisingManagerForUser {
    const IID: ::windows::core::GUID = <IAdvertisingManagerForUser as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AdvertisingManagerForUser {
    const NAME: &'static str = "Windows.System.UserProfile.AdvertisingManagerForUser";
}
::windows::core::interface_hierarchy!(AdvertisingManagerForUser, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for AdvertisingManagerForUser {}
unsafe impl ::core::marker::Sync for AdvertisingManagerForUser {}
#[doc = "*Required features: `\"System_UserProfile\"`*"]
#[repr(transparent)]
pub struct AssignedAccessSettings(::windows::core::IUnknown);
impl AssignedAccessSettings {
    pub fn IsEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsEnabled)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsSingleAppKioskMode(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsSingleAppKioskMode)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn User(&self) -> ::windows::core::Result<super::User> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).User)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetDefault() -> ::windows::core::Result<AssignedAccessSettings> {
        Self::IAssignedAccessSettingsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDefault)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn GetForUser(user: &super::User) -> ::windows::core::Result<AssignedAccessSettings> {
        Self::IAssignedAccessSettingsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetForUser)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(user), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IAssignedAccessSettingsStatics<R, F: FnOnce(&IAssignedAccessSettingsStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<AssignedAccessSettings, IAssignedAccessSettingsStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for AssignedAccessSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AssignedAccessSettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AssignedAccessSettings {}
impl ::core::fmt::Debug for AssignedAccessSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AssignedAccessSettings").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AssignedAccessSettings {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.UserProfile.AssignedAccessSettings;{1bc57f1c-e971-5757-b8e0-512f8b8c46d2})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for AssignedAccessSettings {
    type Vtable = IAssignedAccessSettings_Vtbl;
}
unsafe impl ::windows::core::Interface for AssignedAccessSettings {
    const IID: ::windows::core::GUID = <IAssignedAccessSettings as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AssignedAccessSettings {
    const NAME: &'static str = "Windows.System.UserProfile.AssignedAccessSettings";
}
::windows::core::interface_hierarchy!(AssignedAccessSettings, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for AssignedAccessSettings {}
unsafe impl ::core::marker::Sync for AssignedAccessSettings {}
#[doc = "*Required features: `\"System_UserProfile\"`*"]
#[repr(transparent)]
pub struct DiagnosticsSettings(::windows::core::IUnknown);
impl DiagnosticsSettings {
    pub fn CanUseDiagnosticsToTailorExperiences(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CanUseDiagnosticsToTailorExperiences)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn User(&self) -> ::windows::core::Result<super::User> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).User)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetDefault() -> ::windows::core::Result<DiagnosticsSettings> {
        Self::IDiagnosticsSettingsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDefault)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn GetForUser(user: &super::User) -> ::windows::core::Result<DiagnosticsSettings> {
        Self::IDiagnosticsSettingsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetForUser)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(user), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IDiagnosticsSettingsStatics<R, F: FnOnce(&IDiagnosticsSettingsStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<DiagnosticsSettings, IDiagnosticsSettingsStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for DiagnosticsSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DiagnosticsSettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DiagnosticsSettings {}
impl ::core::fmt::Debug for DiagnosticsSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DiagnosticsSettings").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DiagnosticsSettings {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.UserProfile.DiagnosticsSettings;{e5e9eccd-2711-44e0-973c-491d78048d24})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for DiagnosticsSettings {
    type Vtable = IDiagnosticsSettings_Vtbl;
}
unsafe impl ::windows::core::Interface for DiagnosticsSettings {
    const IID: ::windows::core::GUID = <IDiagnosticsSettings as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for DiagnosticsSettings {
    const NAME: &'static str = "Windows.System.UserProfile.DiagnosticsSettings";
}
::windows::core::interface_hierarchy!(DiagnosticsSettings, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for DiagnosticsSettings {}
unsafe impl ::core::marker::Sync for DiagnosticsSettings {}
#[doc = "*Required features: `\"System_UserProfile\"`*"]
#[repr(transparent)]
pub struct FirstSignInSettings(::windows::core::IUnknown);
impl FirstSignInSettings {
    pub fn GetDefault() -> ::windows::core::Result<FirstSignInSettings> {
        Self::IFirstSignInSettingsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDefault)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IIterator<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::IInspectable>>> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::IInspectable>>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).First)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Lookup(&self, key: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Lookup)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(key), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Size)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn HasKey(&self, key: &::windows::core::HSTRING) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).HasKey)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(key), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Split(&self, first: &mut ::core::option::Option<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>, second: &mut ::core::option::Option<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Split)(::windows::core::Vtable::as_raw(this), first as *mut _ as _, second as *mut _ as _).ok() }
    }
    #[doc(hidden)]
    pub fn IFirstSignInSettingsStatics<R, F: FnOnce(&IFirstSignInSettingsStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<FirstSignInSettings, IFirstSignInSettingsStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for FirstSignInSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for FirstSignInSettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FirstSignInSettings {}
impl ::core::fmt::Debug for FirstSignInSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FirstSignInSettings").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for FirstSignInSettings {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.UserProfile.FirstSignInSettings;{3e945153-3a5e-452e-a601-f5baad2a4870})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for FirstSignInSettings {
    type Vtable = IFirstSignInSettings_Vtbl;
}
unsafe impl ::windows::core::Interface for FirstSignInSettings {
    const IID: ::windows::core::GUID = <IFirstSignInSettings as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for FirstSignInSettings {
    const NAME: &'static str = "Windows.System.UserProfile.FirstSignInSettings";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for FirstSignInSettings {
    type Item = super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::IInspectable>;
    type IntoIter = super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for &FirstSignInSettings {
    type Item = super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::IInspectable>;
    type IntoIter = super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
::windows::core::interface_hierarchy!(FirstSignInSettings, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<FirstSignInSettings> for super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::IInspectable>> {
    type Error = ::windows::core::Error;
    fn try_from(value: FirstSignInSettings) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&FirstSignInSettings> for super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::IInspectable>> {
    type Error = ::windows::core::Error;
    fn try_from(value: &FirstSignInSettings) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&FirstSignInSettings> for ::windows::core::InParam<super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::IInspectable>>> {
    type Error = ::windows::core::Error;
    fn try_from(value: &FirstSignInSettings) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<FirstSignInSettings> for super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable> {
    type Error = ::windows::core::Error;
    fn try_from(value: FirstSignInSettings) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&FirstSignInSettings> for super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &FirstSignInSettings) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&FirstSignInSettings> for ::windows::core::InParam<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>> {
    type Error = ::windows::core::Error;
    fn try_from(value: &FirstSignInSettings) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for FirstSignInSettings {}
unsafe impl ::core::marker::Sync for FirstSignInSettings {}
#[doc = "*Required features: `\"System_UserProfile\"`*"]
pub struct GlobalizationPreferences;
impl GlobalizationPreferences {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Calendars() -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>> {
        Self::IGlobalizationPreferencesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Calendars)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Clocks() -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>> {
        Self::IGlobalizationPreferencesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Clocks)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Currencies() -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>> {
        Self::IGlobalizationPreferencesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Currencies)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Languages() -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>> {
        Self::IGlobalizationPreferencesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Languages)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn HomeGeographicRegion() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IGlobalizationPreferencesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).HomeGeographicRegion)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Globalization\"`*"]
    #[cfg(feature = "Globalization")]
    pub fn WeekStartsOn() -> ::windows::core::Result<super::super::Globalization::DayOfWeek> {
        Self::IGlobalizationPreferencesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).WeekStartsOn)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn TrySetHomeGeographicRegion(region: &::windows::core::HSTRING) -> ::windows::core::Result<bool> {
        Self::IGlobalizationPreferencesStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TrySetHomeGeographicRegion)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(region), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn TrySetLanguages<P0, E0>(languagetags: P0) -> ::windows::core::Result<bool>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::IGlobalizationPreferencesStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TrySetLanguages)(::windows::core::Vtable::as_raw(this), languagetags.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn GetForUser(user: &super::User) -> ::windows::core::Result<GlobalizationPreferencesForUser> {
        Self::IGlobalizationPreferencesStatics3(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetForUser)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(user), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IGlobalizationPreferencesStatics<R, F: FnOnce(&IGlobalizationPreferencesStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<GlobalizationPreferences, IGlobalizationPreferencesStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IGlobalizationPreferencesStatics2<R, F: FnOnce(&IGlobalizationPreferencesStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<GlobalizationPreferences, IGlobalizationPreferencesStatics2> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IGlobalizationPreferencesStatics3<R, F: FnOnce(&IGlobalizationPreferencesStatics3) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<GlobalizationPreferences, IGlobalizationPreferencesStatics3> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for GlobalizationPreferences {
    const NAME: &'static str = "Windows.System.UserProfile.GlobalizationPreferences";
}
#[doc = "*Required features: `\"System_UserProfile\"`*"]
#[repr(transparent)]
pub struct GlobalizationPreferencesForUser(::windows::core::IUnknown);
impl GlobalizationPreferencesForUser {
    pub fn User(&self) -> ::windows::core::Result<super::User> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).User)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Calendars(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Calendars)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Clocks(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Clocks)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Currencies(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Currencies)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Languages(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Languages)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn HomeGeographicRegion(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).HomeGeographicRegion)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Globalization\"`*"]
    #[cfg(feature = "Globalization")]
    pub fn WeekStartsOn(&self) -> ::windows::core::Result<super::super::Globalization::DayOfWeek> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).WeekStartsOn)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for GlobalizationPreferencesForUser {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for GlobalizationPreferencesForUser {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GlobalizationPreferencesForUser {}
impl ::core::fmt::Debug for GlobalizationPreferencesForUser {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GlobalizationPreferencesForUser").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for GlobalizationPreferencesForUser {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.UserProfile.GlobalizationPreferencesForUser;{150f0795-4f6e-40ba-a010-e27d81bda7f5})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for GlobalizationPreferencesForUser {
    type Vtable = IGlobalizationPreferencesForUser_Vtbl;
}
unsafe impl ::windows::core::Interface for GlobalizationPreferencesForUser {
    const IID: ::windows::core::GUID = <IGlobalizationPreferencesForUser as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for GlobalizationPreferencesForUser {
    const NAME: &'static str = "Windows.System.UserProfile.GlobalizationPreferencesForUser";
}
::windows::core::interface_hierarchy!(GlobalizationPreferencesForUser, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for GlobalizationPreferencesForUser {}
unsafe impl ::core::marker::Sync for GlobalizationPreferencesForUser {}
#[doc = "*Required features: `\"System_UserProfile\"`*"]
pub struct LockScreen;
impl LockScreen {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestSetImageFeedAsync(syndicationfeeduri: &super::super::Foundation::Uri) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SetImageFeedResult>> {
        Self::ILockScreenImageFeedStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RequestSetImageFeedAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(syndicationfeeduri), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn TryRemoveImageFeed() -> ::windows::core::Result<bool> {
        Self::ILockScreenImageFeedStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TryRemoveImageFeed)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn OriginalImageFile() -> ::windows::core::Result<super::super::Foundation::Uri> {
        Self::ILockScreenStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).OriginalImageFile)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn GetImageStream() -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStream> {
        Self::ILockScreenStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetImageStream)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn SetImageFileAsync<P0, E0>(value: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Storage::IStorageFile>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::ILockScreenStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SetImageFileAsync)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn SetImageStreamAsync<P0, E0>(value: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Storage::Streams::IRandomAccessStream>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::ILockScreenStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SetImageStreamAsync)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ILockScreenImageFeedStatics<R, F: FnOnce(&ILockScreenImageFeedStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<LockScreen, ILockScreenImageFeedStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ILockScreenStatics<R, F: FnOnce(&ILockScreenStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<LockScreen, ILockScreenStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for LockScreen {
    const NAME: &'static str = "Windows.System.UserProfile.LockScreen";
}
#[doc = "*Required features: `\"System_UserProfile\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
pub struct UserInformation;
#[cfg(feature = "deprecated")]
impl UserInformation {
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn AccountPictureChangeEnabled() -> ::windows::core::Result<bool> {
        Self::IUserInformationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AccountPictureChangeEnabled)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn NameAccessAllowed() -> ::windows::core::Result<bool> {
        Self::IUserInformationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NameAccessAllowed)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Storage\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Storage", feature = "deprecated"))]
    pub fn GetAccountPicture(kind: AccountPictureKind) -> ::windows::core::Result<super::super::Storage::IStorageFile> {
        Self::IUserInformationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetAccountPicture)(::windows::core::Vtable::as_raw(this), kind, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage", feature = "deprecated"))]
    pub fn SetAccountPictureAsync<P0, E0>(image: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SetAccountPictureResult>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Storage::IStorageFile>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::IUserInformationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SetAccountPictureAsync)(::windows::core::Vtable::as_raw(this), image.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage", feature = "deprecated"))]
    pub fn SetAccountPicturesAsync<P0, E0, P1, E1, P2, E2>(smallimage: P0, largeimage: P1, video: P2) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SetAccountPictureResult>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Storage::IStorageFile>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
        P1: ::std::convert::TryInto<::windows::core::InParam<super::super::Storage::IStorageFile>, Error = E1>,
        E1: ::std::convert::Into<::windows::core::Error>,
        P2: ::std::convert::TryInto<::windows::core::InParam<super::super::Storage::IStorageFile>, Error = E2>,
        E2: ::std::convert::Into<::windows::core::Error>,
    {
        Self::IUserInformationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SetAccountPicturesAsync)(::windows::core::Vtable::as_raw(this), smallimage.try_into().map_err(|e| e.into())?.abi(), largeimage.try_into().map_err(|e| e.into())?.abi(), video.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "deprecated"))]
    pub fn SetAccountPictureFromStreamAsync<P0, E0>(image: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SetAccountPictureResult>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Storage::Streams::IRandomAccessStream>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::IUserInformationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SetAccountPictureFromStreamAsync)(::windows::core::Vtable::as_raw(this), image.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "deprecated"))]
    pub fn SetAccountPicturesFromStreamsAsync<P0, E0, P1, E1, P2, E2>(smallimage: P0, largeimage: P1, video: P2) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SetAccountPictureResult>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Storage::Streams::IRandomAccessStream>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
        P1: ::std::convert::TryInto<::windows::core::InParam<super::super::Storage::Streams::IRandomAccessStream>, Error = E1>,
        E1: ::std::convert::Into<::windows::core::Error>,
        P2: ::std::convert::TryInto<::windows::core::InParam<super::super::Storage::Streams::IRandomAccessStream>, Error = E2>,
        E2: ::std::convert::Into<::windows::core::Error>,
    {
        Self::IUserInformationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SetAccountPicturesFromStreamsAsync)(::windows::core::Vtable::as_raw(this), smallimage.try_into().map_err(|e| e.into())?.abi(), largeimage.try_into().map_err(|e| e.into())?.abi(), video.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn AccountPictureChanged(changehandler: &super::super::Foundation::EventHandler<::windows::core::IInspectable>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        Self::IUserInformationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AccountPictureChanged)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(changehandler), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RemoveAccountPictureChanged(token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        Self::IUserInformationStatics(|this| unsafe { (::windows::core::Vtable::vtable(this).RemoveAccountPictureChanged)(::windows::core::Vtable::as_raw(this), token).ok() })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn GetDisplayNameAsync() -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>> {
        Self::IUserInformationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDisplayNameAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn GetFirstNameAsync() -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>> {
        Self::IUserInformationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetFirstNameAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn GetLastNameAsync() -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>> {
        Self::IUserInformationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetLastNameAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn GetPrincipalNameAsync() -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>> {
        Self::IUserInformationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetPrincipalNameAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn GetSessionInitiationProtocolUriAsync() -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Uri>> {
        Self::IUserInformationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetSessionInitiationProtocolUriAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn GetDomainNameAsync() -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>> {
        Self::IUserInformationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDomainNameAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    #[cfg(feature = "deprecated")]
    pub fn IUserInformationStatics<R, F: FnOnce(&IUserInformationStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<UserInformation, IUserInformationStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for UserInformation {
    const NAME: &'static str = "Windows.System.UserProfile.UserInformation";
}
#[doc = "*Required features: `\"System_UserProfile\"`*"]
#[repr(transparent)]
pub struct UserProfilePersonalizationSettings(::windows::core::IUnknown);
impl UserProfilePersonalizationSettings {
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn TrySetLockScreenImageAsync(&self, imagefile: &super::super::Storage::StorageFile) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TrySetLockScreenImageAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(imagefile), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn TrySetWallpaperImageAsync(&self, imagefile: &super::super::Storage::StorageFile) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TrySetWallpaperImageAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(imagefile), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Current() -> ::windows::core::Result<UserProfilePersonalizationSettings> {
        Self::IUserProfilePersonalizationSettingsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Current)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn IsSupported() -> ::windows::core::Result<bool> {
        Self::IUserProfilePersonalizationSettingsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IUserProfilePersonalizationSettingsStatics<R, F: FnOnce(&IUserProfilePersonalizationSettingsStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<UserProfilePersonalizationSettings, IUserProfilePersonalizationSettingsStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for UserProfilePersonalizationSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UserProfilePersonalizationSettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UserProfilePersonalizationSettings {}
impl ::core::fmt::Debug for UserProfilePersonalizationSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserProfilePersonalizationSettings").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UserProfilePersonalizationSettings {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.UserProfile.UserProfilePersonalizationSettings;{8ceddab4-7998-46d5-8dd3-184f1c5f9ab9})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for UserProfilePersonalizationSettings {
    type Vtable = IUserProfilePersonalizationSettings_Vtbl;
}
unsafe impl ::windows::core::Interface for UserProfilePersonalizationSettings {
    const IID: ::windows::core::GUID = <IUserProfilePersonalizationSettings as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for UserProfilePersonalizationSettings {
    const NAME: &'static str = "Windows.System.UserProfile.UserProfilePersonalizationSettings";
}
::windows::core::interface_hierarchy!(UserProfilePersonalizationSettings, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for UserProfilePersonalizationSettings {}
unsafe impl ::core::marker::Sync for UserProfilePersonalizationSettings {}
#[doc = "*Required features: `\"System_UserProfile\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AccountPictureKind(pub i32);
#[cfg(feature = "deprecated")]
impl AccountPictureKind {
    pub const SmallImage: Self = Self(0i32);
    pub const LargeImage: Self = Self(1i32);
    pub const Video: Self = Self(2i32);
}
#[cfg(feature = "deprecated")]
impl ::core::marker::Copy for AccountPictureKind {}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for AccountPictureKind {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "deprecated")]
impl ::core::default::Default for AccountPictureKind {
    fn default() -> Self {
        Self(0)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Abi for AccountPictureKind {
    type Abi = Self;
}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for AccountPictureKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AccountPictureKind").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for AccountPictureKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.System.UserProfile.AccountPictureKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"System_UserProfile\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SetAccountPictureResult(pub i32);
#[cfg(feature = "deprecated")]
impl SetAccountPictureResult {
    pub const Success: Self = Self(0i32);
    pub const ChangeDisabled: Self = Self(1i32);
    pub const LargeOrDynamicError: Self = Self(2i32);
    pub const VideoFrameSizeError: Self = Self(3i32);
    pub const FileSizeError: Self = Self(4i32);
    pub const Failure: Self = Self(5i32);
}
#[cfg(feature = "deprecated")]
impl ::core::marker::Copy for SetAccountPictureResult {}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for SetAccountPictureResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "deprecated")]
impl ::core::default::Default for SetAccountPictureResult {
    fn default() -> Self {
        Self(0)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Abi for SetAccountPictureResult {
    type Abi = Self;
}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for SetAccountPictureResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SetAccountPictureResult").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for SetAccountPictureResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.System.UserProfile.SetAccountPictureResult;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"System_UserProfile\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SetImageFeedResult(pub i32);
impl SetImageFeedResult {
    pub const Success: Self = Self(0i32);
    pub const ChangeDisabled: Self = Self(1i32);
    pub const UserCanceled: Self = Self(2i32);
}
impl ::core::marker::Copy for SetImageFeedResult {}
impl ::core::clone::Clone for SetImageFeedResult {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SetImageFeedResult {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SetImageFeedResult {
    type Abi = Self;
}
impl ::core::fmt::Debug for SetImageFeedResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SetImageFeedResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SetImageFeedResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.System.UserProfile.SetImageFeedResult;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
