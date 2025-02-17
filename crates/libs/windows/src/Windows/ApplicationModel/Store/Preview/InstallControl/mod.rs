#[doc(hidden)]
#[repr(transparent)]
pub struct IAppInstallItem(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IAppInstallItem {
    type Vtable = IAppInstallItem_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppInstallItem {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x49d3dfab_168a_4cbf_a93a_9e448c82737d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInstallItem_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ProductId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub PackageFamilyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub InstallType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppInstallType) -> ::windows::core::HRESULT,
    pub IsUserInitiated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub GetCurrentStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Cancel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Pause: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Restart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Completed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Completed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub StatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StatusChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveStatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStatusChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppInstallItem2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IAppInstallItem2 {
    type Vtable = IAppInstallItem2_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppInstallItem2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd3972af8_40c0_4fd7_aa6c_0aa13ca6188c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInstallItem2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CancelWithTelemetry: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, correlationvector: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub PauseWithTelemetry: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, correlationvector: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RestartWithTelemetry: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, correlationvector: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppInstallItem3(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IAppInstallItem3 {
    type Vtable = IAppInstallItem3_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppInstallItem3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6f3dc998_dd47_433c_9234_560172d67a45);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInstallItem3_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Children: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Children: usize,
    pub ItemOperationsMightAffectOtherItems: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppInstallItem4(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IAppInstallItem4 {
    type Vtable = IAppInstallItem4_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppInstallItem4 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc2d1ce12_71ff_4fc8_b540_453d4b37e1d1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInstallItem4_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub LaunchAfterInstall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetLaunchAfterInstall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppInstallItem5(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IAppInstallItem5 {
    type Vtable = IAppInstallItem5_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppInstallItem5 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5510e7cc_4076_4a0b_9472_c21d9d380e55);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInstallItem5_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub PinToDesktopAfterInstall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetPinToDesktopAfterInstall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub PinToStartAfterInstall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetPinToStartAfterInstall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub PinToTaskbarAfterInstall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetPinToTaskbarAfterInstall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub CompletedInstallToastNotificationMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppInstallationToastNotificationMode) -> ::windows::core::HRESULT,
    pub SetCompletedInstallToastNotificationMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: AppInstallationToastNotificationMode) -> ::windows::core::HRESULT,
    pub InstallInProgressToastNotificationMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppInstallationToastNotificationMode) -> ::windows::core::HRESULT,
    pub SetInstallInProgressToastNotificationMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: AppInstallationToastNotificationMode) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppInstallManager(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IAppInstallManager {
    type Vtable = IAppInstallManager_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppInstallManager {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9353e170_8441_4b45_bd72_7c2fa925beee);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInstallManager_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub AppInstallItems: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AppInstallItems: usize,
    pub Cancel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Pause: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Restart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ItemCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ItemCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveItemCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveItemCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub ItemStatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ItemStatusChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveItemStatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveItemStatusChanged: usize,
    pub AutoUpdateSetting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AutoUpdateSetting) -> ::windows::core::HRESULT,
    pub SetAutoUpdateSetting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: AutoUpdateSetting) -> ::windows::core::HRESULT,
    pub AcquisitionIdentity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetAcquisitionIdentity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetIsApplicableAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: *mut ::core::ffi::c_void, skuid: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetIsApplicableAsync: usize,
    #[cfg(feature = "Foundation")]
    pub StartAppInstallAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: *mut ::core::ffi::c_void, skuid: *mut ::core::ffi::c_void, repair: bool, forceuseofnonremovablestorage: bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartAppInstallAsync: usize,
    #[cfg(feature = "Foundation")]
    pub UpdateAppByPackageFamilyNameAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packagefamilyname: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UpdateAppByPackageFamilyNameAsync: usize,
    #[cfg(feature = "Foundation")]
    pub SearchForUpdatesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: *mut ::core::ffi::c_void, skuid: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SearchForUpdatesAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SearchForAllUpdatesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SearchForAllUpdatesAsync: usize,
    #[cfg(feature = "Foundation")]
    pub IsStoreBlockedByPolicyAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storeclientname: *mut ::core::ffi::c_void, storeclientpublisher: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    IsStoreBlockedByPolicyAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetIsAppAllowedToInstallAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetIsAppAllowedToInstallAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppInstallManager2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IAppInstallManager2 {
    type Vtable = IAppInstallManager2_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppInstallManager2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x16937851_ed37_480d_8314_52e27c03f04a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInstallManager2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub StartAppInstallWithTelemetryAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: *mut ::core::ffi::c_void, skuid: *mut ::core::ffi::c_void, repair: bool, forceuseofnonremovablestorage: bool, catalogid: *mut ::core::ffi::c_void, bundleid: *mut ::core::ffi::c_void, correlationvector: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartAppInstallWithTelemetryAsync: usize,
    #[cfg(feature = "Foundation")]
    pub UpdateAppByPackageFamilyNameWithTelemetryAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packagefamilyname: *mut ::core::ffi::c_void, correlationvector: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UpdateAppByPackageFamilyNameWithTelemetryAsync: usize,
    #[cfg(feature = "Foundation")]
    pub SearchForUpdatesWithTelemetryAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: *mut ::core::ffi::c_void, skuid: *mut ::core::ffi::c_void, catalogid: *mut ::core::ffi::c_void, correlationvector: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SearchForUpdatesWithTelemetryAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SearchForAllUpdatesWithTelemetryAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, correlationvector: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SearchForAllUpdatesWithTelemetryAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetIsAppAllowedToInstallWithTelemetryAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: *mut ::core::ffi::c_void, skuid: *mut ::core::ffi::c_void, catalogid: *mut ::core::ffi::c_void, correlationvector: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetIsAppAllowedToInstallWithTelemetryAsync: usize,
    pub CancelWithTelemetry: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: *mut ::core::ffi::c_void, correlationvector: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub PauseWithTelemetry: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: *mut ::core::ffi::c_void, correlationvector: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RestartWithTelemetry: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: *mut ::core::ffi::c_void, correlationvector: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppInstallManager3(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IAppInstallManager3 {
    type Vtable = IAppInstallManager3_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppInstallManager3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x95b24b17_e96a_4d0e_84e1_c8cb417a0178);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInstallManager3_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "Management_Deployment"))]
    pub StartProductInstallAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: *mut ::core::ffi::c_void, catalogid: *mut ::core::ffi::c_void, flightid: *mut ::core::ffi::c_void, clientid: *mut ::core::ffi::c_void, repair: bool, forceuseofnonremovablestorage: bool, correlationvector: *mut ::core::ffi::c_void, targetvolume: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Management_Deployment")))]
    StartProductInstallAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Management_Deployment", feature = "System"))]
    pub StartProductInstallForUserAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: *mut ::core::ffi::c_void, productid: *mut ::core::ffi::c_void, catalogid: *mut ::core::ffi::c_void, flightid: *mut ::core::ffi::c_void, clientid: *mut ::core::ffi::c_void, repair: bool, forceuseofnonremovablestorage: bool, correlationvector: *mut ::core::ffi::c_void, targetvolume: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Management_Deployment", feature = "System")))]
    StartProductInstallForUserAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub UpdateAppByPackageFamilyNameForUserAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: *mut ::core::ffi::c_void, packagefamilyname: *mut ::core::ffi::c_void, correlationvector: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System")))]
    UpdateAppByPackageFamilyNameForUserAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub SearchForUpdatesForUserAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: *mut ::core::ffi::c_void, productid: *mut ::core::ffi::c_void, skuid: *mut ::core::ffi::c_void, catalogid: *mut ::core::ffi::c_void, correlationvector: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System")))]
    SearchForUpdatesForUserAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "System"))]
    pub SearchForAllUpdatesForUserAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: *mut ::core::ffi::c_void, correlationvector: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "System")))]
    SearchForAllUpdatesForUserAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub GetIsAppAllowedToInstallForUserAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: *mut ::core::ffi::c_void, productid: *mut ::core::ffi::c_void, skuid: *mut ::core::ffi::c_void, catalogid: *mut ::core::ffi::c_void, correlationvector: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System")))]
    GetIsAppAllowedToInstallForUserAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub GetIsApplicableForUserAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: *mut ::core::ffi::c_void, productid: *mut ::core::ffi::c_void, skuid: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System")))]
    GetIsApplicableForUserAsync: usize,
    pub MoveToFrontOfDownloadQueue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: *mut ::core::ffi::c_void, correlationvector: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppInstallManager4(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IAppInstallManager4 {
    type Vtable = IAppInstallManager4_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppInstallManager4 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x260a2a16_5a9e_4ebd_b944_f2ba75c31159);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInstallManager4_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub GetFreeUserEntitlementAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storeid: *mut ::core::ffi::c_void, campaignid: *mut ::core::ffi::c_void, correlationvector: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetFreeUserEntitlementAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub GetFreeUserEntitlementForUserAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: *mut ::core::ffi::c_void, storeid: *mut ::core::ffi::c_void, campaignid: *mut ::core::ffi::c_void, correlationvector: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System")))]
    GetFreeUserEntitlementForUserAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetFreeDeviceEntitlementAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storeid: *mut ::core::ffi::c_void, campaignid: *mut ::core::ffi::c_void, correlationvector: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetFreeDeviceEntitlementAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppInstallManager5(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IAppInstallManager5 {
    type Vtable = IAppInstallManager5_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppInstallManager5 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3cd7be4c_1be9_4f7f_b675_aa1d64a529b2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInstallManager5_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub AppInstallItemsWithGroupSupport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AppInstallItemsWithGroupSupport: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppInstallManager6(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IAppInstallManager6 {
    type Vtable = IAppInstallManager6_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppInstallManager6 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc9e7d408_f27a_4471_b2f4_e76efcbebcca);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInstallManager6_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub SearchForAllUpdatesWithUpdateOptionsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, correlationvector: *mut ::core::ffi::c_void, clientid: *mut ::core::ffi::c_void, updateoptions: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SearchForAllUpdatesWithUpdateOptionsAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "System"))]
    pub SearchForAllUpdatesWithUpdateOptionsForUserAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: *mut ::core::ffi::c_void, correlationvector: *mut ::core::ffi::c_void, clientid: *mut ::core::ffi::c_void, updateoptions: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "System")))]
    SearchForAllUpdatesWithUpdateOptionsForUserAsync: usize,
    #[cfg(feature = "Foundation")]
    pub SearchForUpdatesWithUpdateOptionsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: *mut ::core::ffi::c_void, skuid: *mut ::core::ffi::c_void, correlationvector: *mut ::core::ffi::c_void, clientid: *mut ::core::ffi::c_void, updateoptions: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SearchForUpdatesWithUpdateOptionsAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub SearchForUpdatesWithUpdateOptionsForUserAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: *mut ::core::ffi::c_void, productid: *mut ::core::ffi::c_void, skuid: *mut ::core::ffi::c_void, correlationvector: *mut ::core::ffi::c_void, clientid: *mut ::core::ffi::c_void, updateoptions: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System")))]
    SearchForUpdatesWithUpdateOptionsForUserAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub StartProductInstallWithOptionsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: *mut ::core::ffi::c_void, flightid: *mut ::core::ffi::c_void, clientid: *mut ::core::ffi::c_void, correlationvector: *mut ::core::ffi::c_void, installoptions: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    StartProductInstallWithOptionsAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "System"))]
    pub StartProductInstallWithOptionsForUserAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: *mut ::core::ffi::c_void, productid: *mut ::core::ffi::c_void, flightid: *mut ::core::ffi::c_void, clientid: *mut ::core::ffi::c_void, correlationvector: *mut ::core::ffi::c_void, installoptions: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "System")))]
    StartProductInstallWithOptionsForUserAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetIsPackageIdentityAllowedToInstallAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, correlationvector: *mut ::core::ffi::c_void, packageidentityname: *mut ::core::ffi::c_void, publishercertificatename: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetIsPackageIdentityAllowedToInstallAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub GetIsPackageIdentityAllowedToInstallForUserAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: *mut ::core::ffi::c_void, correlationvector: *mut ::core::ffi::c_void, packageidentityname: *mut ::core::ffi::c_void, publishercertificatename: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System")))]
    GetIsPackageIdentityAllowedToInstallForUserAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppInstallManager7(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IAppInstallManager7 {
    type Vtable = IAppInstallManager7_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppInstallManager7 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa5ee7b30_d5e4_49a3_9853_3db03203321d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInstallManager7_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CanInstallForAllUsers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppInstallManagerItemEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IAppInstallManagerItemEventArgs {
    type Vtable = IAppInstallManagerItemEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppInstallManagerItemEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbc505743_4674_4dd1_957e_c25682086a14);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInstallManagerItemEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppInstallOptions(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IAppInstallOptions {
    type Vtable = IAppInstallOptions_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppInstallOptions {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc9808300_1cb8_4eb6_8c9f_6a30c64a5b51);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInstallOptions_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CatalogId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetCatalogId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ForceUseOfNonRemovableStorage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetForceUseOfNonRemovableStorage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub AllowForcedAppRestart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetAllowForcedAppRestart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub Repair: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetRepair: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Management_Deployment")]
    pub TargetVolume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Management_Deployment"))]
    TargetVolume: usize,
    #[cfg(feature = "Management_Deployment")]
    pub SetTargetVolume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Management_Deployment"))]
    SetTargetVolume: usize,
    pub LaunchAfterInstall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetLaunchAfterInstall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppInstallOptions2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IAppInstallOptions2 {
    type Vtable = IAppInstallOptions2_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppInstallOptions2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8a04c0d7_c94b_425e_95b4_bf27faeaee89);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInstallOptions2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub PinToDesktopAfterInstall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetPinToDesktopAfterInstall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub PinToStartAfterInstall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetPinToStartAfterInstall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub PinToTaskbarAfterInstall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetPinToTaskbarAfterInstall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub CompletedInstallToastNotificationMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppInstallationToastNotificationMode) -> ::windows::core::HRESULT,
    pub SetCompletedInstallToastNotificationMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: AppInstallationToastNotificationMode) -> ::windows::core::HRESULT,
    pub InstallInProgressToastNotificationMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppInstallationToastNotificationMode) -> ::windows::core::HRESULT,
    pub SetInstallInProgressToastNotificationMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: AppInstallationToastNotificationMode) -> ::windows::core::HRESULT,
    pub InstallForAllUsers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetInstallForAllUsers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub StageButDoNotInstall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetStageButDoNotInstall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub CampaignId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetCampaignId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ExtendedCampaignId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetExtendedCampaignId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppInstallStatus(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IAppInstallStatus {
    type Vtable = IAppInstallStatus_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppInstallStatus {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x936dccfa_2450_4126_88b1_6127a644dd5c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInstallStatus_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub InstallState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppInstallState) -> ::windows::core::HRESULT,
    pub DownloadSizeInBytes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT,
    pub BytesDownloaded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT,
    pub PercentComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub ErrorCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppInstallStatus2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IAppInstallStatus2 {
    type Vtable = IAppInstallStatus2_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppInstallStatus2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96e7818a_5e92_4aa9_8edc_58fed4b87e00);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInstallStatus2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "System")]
    pub User: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))]
    User: usize,
    pub ReadyForLaunch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppInstallStatus3(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IAppInstallStatus3 {
    type Vtable = IAppInstallStatus3_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppInstallStatus3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcb880c56_837b_4b4c_9ebb_6d44a0a96307);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInstallStatus3_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub IsStaged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppUpdateOptions(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IAppUpdateOptions {
    type Vtable = IAppUpdateOptions_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppUpdateOptions {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x26f0b02f_c2f3_4aea_af8c_6308dd9db85f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppUpdateOptions_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CatalogId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetCatalogId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AllowForcedAppRestart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetAllowForcedAppRestart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppUpdateOptions2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IAppUpdateOptions2 {
    type Vtable = IAppUpdateOptions2_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppUpdateOptions2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf4646e08_ed26_4bf9_9679_48f628e53df8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppUpdateOptions2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub AutomaticallyDownloadAndInstallUpdateIfFound: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetAutomaticallyDownloadAndInstallUpdateIfFound: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGetEntitlementResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IGetEntitlementResult {
    type Vtable = IGetEntitlementResult_Vtbl;
}
unsafe impl ::windows::core::Interface for IGetEntitlementResult {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x74fc843f_1a9e_4609_8e4d_819086d08a3d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGetEntitlementResult_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut GetEntitlementStatus) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`*"]
#[repr(transparent)]
pub struct AppInstallItem(::windows::core::IUnknown);
impl AppInstallItem {
    pub fn ProductId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ProductId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn PackageFamilyName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PackageFamilyName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn InstallType(&self) -> ::windows::core::Result<AppInstallType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).InstallType)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsUserInitiated(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsUserInitiated)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetCurrentStatus(&self) -> ::windows::core::Result<AppInstallStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetCurrentStatus)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Cancel(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Cancel)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    pub fn Pause(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Pause)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    pub fn Restart(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Restart)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Completed(&self, handler: &super::super::super::super::Foundation::TypedEventHandler<AppInstallItem, ::windows::core::IInspectable>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Completed)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveCompleted(&self, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveCompleted)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StatusChanged(&self, handler: &super::super::super::super::Foundation::TypedEventHandler<AppInstallItem, ::windows::core::IInspectable>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).StatusChanged)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveStatusChanged(&self, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveStatusChanged)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    pub fn CancelWithTelemetry(&self, correlationvector: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppInstallItem2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).CancelWithTelemetry)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(correlationvector)).ok() }
    }
    pub fn PauseWithTelemetry(&self, correlationvector: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppInstallItem2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).PauseWithTelemetry)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(correlationvector)).ok() }
    }
    pub fn RestartWithTelemetry(&self, correlationvector: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppInstallItem2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).RestartWithTelemetry)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(correlationvector)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Children(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<AppInstallItem>> {
        let this = &::windows::core::Interface::cast::<IAppInstallItem3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Children)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ItemOperationsMightAffectOtherItems(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IAppInstallItem3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ItemOperationsMightAffectOtherItems)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn LaunchAfterInstall(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IAppInstallItem4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LaunchAfterInstall)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetLaunchAfterInstall(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppInstallItem4>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetLaunchAfterInstall)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn PinToDesktopAfterInstall(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IAppInstallItem5>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PinToDesktopAfterInstall)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetPinToDesktopAfterInstall(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppInstallItem5>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetPinToDesktopAfterInstall)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn PinToStartAfterInstall(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IAppInstallItem5>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PinToStartAfterInstall)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetPinToStartAfterInstall(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppInstallItem5>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetPinToStartAfterInstall)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn PinToTaskbarAfterInstall(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IAppInstallItem5>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PinToTaskbarAfterInstall)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetPinToTaskbarAfterInstall(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppInstallItem5>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetPinToTaskbarAfterInstall)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn CompletedInstallToastNotificationMode(&self) -> ::windows::core::Result<AppInstallationToastNotificationMode> {
        let this = &::windows::core::Interface::cast::<IAppInstallItem5>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CompletedInstallToastNotificationMode)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetCompletedInstallToastNotificationMode(&self, value: AppInstallationToastNotificationMode) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppInstallItem5>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetCompletedInstallToastNotificationMode)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn InstallInProgressToastNotificationMode(&self) -> ::windows::core::Result<AppInstallationToastNotificationMode> {
        let this = &::windows::core::Interface::cast::<IAppInstallItem5>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).InstallInProgressToastNotificationMode)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetInstallInProgressToastNotificationMode(&self, value: AppInstallationToastNotificationMode) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppInstallItem5>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetInstallInProgressToastNotificationMode)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
}
impl ::core::clone::Clone for AppInstallItem {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppInstallItem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppInstallItem {}
impl ::core::fmt::Debug for AppInstallItem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppInstallItem").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppInstallItem {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallItem;{49d3dfab-168a-4cbf-a93a-9e448c82737d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for AppInstallItem {
    type Vtable = IAppInstallItem_Vtbl;
}
unsafe impl ::windows::core::Interface for AppInstallItem {
    const IID: ::windows::core::GUID = <IAppInstallItem as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppInstallItem {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallItem";
}
::windows::core::interface_hierarchy!(AppInstallItem, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for AppInstallItem {}
unsafe impl ::core::marker::Sync for AppInstallItem {}
#[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`*"]
#[repr(transparent)]
pub struct AppInstallManager(::windows::core::IUnknown);
impl AppInstallManager {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<AppInstallManager, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn AppInstallItems(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<AppInstallItem>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AppInstallItems)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Cancel(&self, productid: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Cancel)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(productid)).ok() }
    }
    pub fn Pause(&self, productid: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Pause)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(productid)).ok() }
    }
    pub fn Restart(&self, productid: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Restart)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(productid)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ItemCompleted(&self, handler: &super::super::super::super::Foundation::TypedEventHandler<AppInstallManager, AppInstallManagerItemEventArgs>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ItemCompleted)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveItemCompleted(&self, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveItemCompleted)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ItemStatusChanged(&self, handler: &super::super::super::super::Foundation::TypedEventHandler<AppInstallManager, AppInstallManagerItemEventArgs>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ItemStatusChanged)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveItemStatusChanged(&self, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveItemStatusChanged)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    pub fn AutoUpdateSetting(&self) -> ::windows::core::Result<AutoUpdateSetting> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AutoUpdateSetting)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetAutoUpdateSetting(&self, value: AutoUpdateSetting) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetAutoUpdateSetting)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn AcquisitionIdentity(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AcquisitionIdentity)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetAcquisitionIdentity(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetAcquisitionIdentity)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetIsApplicableAsync(&self, productid: &::windows::core::HSTRING, skuid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetIsApplicableAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(productid), ::core::mem::transmute_copy(skuid), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StartAppInstallAsync(&self, productid: &::windows::core::HSTRING, skuid: &::windows::core::HSTRING, repair: bool, forceuseofnonremovablestorage: bool) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<AppInstallItem>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).StartAppInstallAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(productid), ::core::mem::transmute_copy(skuid), repair, forceuseofnonremovablestorage, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn UpdateAppByPackageFamilyNameAsync(&self, packagefamilyname: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<AppInstallItem>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).UpdateAppByPackageFamilyNameAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(packagefamilyname), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SearchForUpdatesAsync(&self, productid: &::windows::core::HSTRING, skuid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<AppInstallItem>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SearchForUpdatesAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(productid), ::core::mem::transmute_copy(skuid), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SearchForAllUpdatesAsync(&self) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Foundation::Collections::IVectorView<AppInstallItem>>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SearchForAllUpdatesAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn IsStoreBlockedByPolicyAsync(&self, storeclientname: &::windows::core::HSTRING, storeclientpublisher: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsStoreBlockedByPolicyAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(storeclientname), ::core::mem::transmute_copy(storeclientpublisher), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetIsAppAllowedToInstallAsync(&self, productid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetIsAppAllowedToInstallAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(productid), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StartAppInstallWithTelemetryAsync(&self, productid: &::windows::core::HSTRING, skuid: &::windows::core::HSTRING, repair: bool, forceuseofnonremovablestorage: bool, catalogid: &::windows::core::HSTRING, bundleid: &::windows::core::HSTRING, correlationvector: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<AppInstallItem>> {
        let this = &::windows::core::Interface::cast::<IAppInstallManager2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).StartAppInstallWithTelemetryAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(productid), ::core::mem::transmute_copy(skuid), repair, forceuseofnonremovablestorage, ::core::mem::transmute_copy(catalogid), ::core::mem::transmute_copy(bundleid), ::core::mem::transmute_copy(correlationvector), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn UpdateAppByPackageFamilyNameWithTelemetryAsync(&self, packagefamilyname: &::windows::core::HSTRING, correlationvector: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<AppInstallItem>> {
        let this = &::windows::core::Interface::cast::<IAppInstallManager2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).UpdateAppByPackageFamilyNameWithTelemetryAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(packagefamilyname), ::core::mem::transmute_copy(correlationvector), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SearchForUpdatesWithTelemetryAsync(&self, productid: &::windows::core::HSTRING, skuid: &::windows::core::HSTRING, catalogid: &::windows::core::HSTRING, correlationvector: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<AppInstallItem>> {
        let this = &::windows::core::Interface::cast::<IAppInstallManager2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SearchForUpdatesWithTelemetryAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(productid), ::core::mem::transmute_copy(skuid), ::core::mem::transmute_copy(catalogid), ::core::mem::transmute_copy(correlationvector), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SearchForAllUpdatesWithTelemetryAsync(&self, correlationvector: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Foundation::Collections::IVectorView<AppInstallItem>>> {
        let this = &::windows::core::Interface::cast::<IAppInstallManager2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SearchForAllUpdatesWithTelemetryAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(correlationvector), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetIsAppAllowedToInstallWithTelemetryAsync(&self, productid: &::windows::core::HSTRING, skuid: &::windows::core::HSTRING, catalogid: &::windows::core::HSTRING, correlationvector: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::core::Interface::cast::<IAppInstallManager2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetIsAppAllowedToInstallWithTelemetryAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(productid), ::core::mem::transmute_copy(skuid), ::core::mem::transmute_copy(catalogid), ::core::mem::transmute_copy(correlationvector), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CancelWithTelemetry(&self, productid: &::windows::core::HSTRING, correlationvector: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppInstallManager2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).CancelWithTelemetry)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(productid), ::core::mem::transmute_copy(correlationvector)).ok() }
    }
    pub fn PauseWithTelemetry(&self, productid: &::windows::core::HSTRING, correlationvector: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppInstallManager2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).PauseWithTelemetry)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(productid), ::core::mem::transmute_copy(correlationvector)).ok() }
    }
    pub fn RestartWithTelemetry(&self, productid: &::windows::core::HSTRING, correlationvector: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppInstallManager2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).RestartWithTelemetry)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(productid), ::core::mem::transmute_copy(correlationvector)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Management_Deployment\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Management_Deployment"))]
    pub fn StartProductInstallAsync(&self, productid: &::windows::core::HSTRING, catalogid: &::windows::core::HSTRING, flightid: &::windows::core::HSTRING, clientid: &::windows::core::HSTRING, repair: bool, forceuseofnonremovablestorage: bool, correlationvector: &::windows::core::HSTRING, targetvolume: &super::super::super::super::Management::Deployment::PackageVolume) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Foundation::Collections::IVectorView<AppInstallItem>>> {
        let this = &::windows::core::Interface::cast::<IAppInstallManager3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).StartProductInstallAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(productid), ::core::mem::transmute_copy(catalogid), ::core::mem::transmute_copy(flightid), ::core::mem::transmute_copy(clientid), repair, forceuseofnonremovablestorage, ::core::mem::transmute_copy(correlationvector), ::core::mem::transmute_copy(targetvolume), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Management_Deployment\"`, `\"System\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Management_Deployment", feature = "System"))]
    pub fn StartProductInstallForUserAsync(&self, user: &super::super::super::super::System::User, productid: &::windows::core::HSTRING, catalogid: &::windows::core::HSTRING, flightid: &::windows::core::HSTRING, clientid: &::windows::core::HSTRING, repair: bool, forceuseofnonremovablestorage: bool, correlationvector: &::windows::core::HSTRING, targetvolume: &super::super::super::super::Management::Deployment::PackageVolume) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Foundation::Collections::IVectorView<AppInstallItem>>> {
        let this = &::windows::core::Interface::cast::<IAppInstallManager3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).StartProductInstallForUserAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(user), ::core::mem::transmute_copy(productid), ::core::mem::transmute_copy(catalogid), ::core::mem::transmute_copy(flightid), ::core::mem::transmute_copy(clientid), repair, forceuseofnonremovablestorage, ::core::mem::transmute_copy(correlationvector), ::core::mem::transmute_copy(targetvolume), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"System\"`*"]
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub fn UpdateAppByPackageFamilyNameForUserAsync(&self, user: &super::super::super::super::System::User, packagefamilyname: &::windows::core::HSTRING, correlationvector: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<AppInstallItem>> {
        let this = &::windows::core::Interface::cast::<IAppInstallManager3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).UpdateAppByPackageFamilyNameForUserAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(user), ::core::mem::transmute_copy(packagefamilyname), ::core::mem::transmute_copy(correlationvector), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"System\"`*"]
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub fn SearchForUpdatesForUserAsync(&self, user: &super::super::super::super::System::User, productid: &::windows::core::HSTRING, skuid: &::windows::core::HSTRING, catalogid: &::windows::core::HSTRING, correlationvector: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<AppInstallItem>> {
        let this = &::windows::core::Interface::cast::<IAppInstallManager3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SearchForUpdatesForUserAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(user), ::core::mem::transmute_copy(productid), ::core::mem::transmute_copy(skuid), ::core::mem::transmute_copy(catalogid), ::core::mem::transmute_copy(correlationvector), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"System\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "System"))]
    pub fn SearchForAllUpdatesForUserAsync(&self, user: &super::super::super::super::System::User, correlationvector: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Foundation::Collections::IVectorView<AppInstallItem>>> {
        let this = &::windows::core::Interface::cast::<IAppInstallManager3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SearchForAllUpdatesForUserAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(user), ::core::mem::transmute_copy(correlationvector), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"System\"`*"]
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub fn GetIsAppAllowedToInstallForUserAsync(&self, user: &super::super::super::super::System::User, productid: &::windows::core::HSTRING, skuid: &::windows::core::HSTRING, catalogid: &::windows::core::HSTRING, correlationvector: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::core::Interface::cast::<IAppInstallManager3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetIsAppAllowedToInstallForUserAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(user), ::core::mem::transmute_copy(productid), ::core::mem::transmute_copy(skuid), ::core::mem::transmute_copy(catalogid), ::core::mem::transmute_copy(correlationvector), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"System\"`*"]
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub fn GetIsApplicableForUserAsync(&self, user: &super::super::super::super::System::User, productid: &::windows::core::HSTRING, skuid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::core::Interface::cast::<IAppInstallManager3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetIsApplicableForUserAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(user), ::core::mem::transmute_copy(productid), ::core::mem::transmute_copy(skuid), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn MoveToFrontOfDownloadQueue(&self, productid: &::windows::core::HSTRING, correlationvector: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppInstallManager3>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).MoveToFrontOfDownloadQueue)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(productid), ::core::mem::transmute_copy(correlationvector)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetFreeUserEntitlementAsync(&self, storeid: &::windows::core::HSTRING, campaignid: &::windows::core::HSTRING, correlationvector: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<GetEntitlementResult>> {
        let this = &::windows::core::Interface::cast::<IAppInstallManager4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetFreeUserEntitlementAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(storeid), ::core::mem::transmute_copy(campaignid), ::core::mem::transmute_copy(correlationvector), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"System\"`*"]
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub fn GetFreeUserEntitlementForUserAsync(&self, user: &super::super::super::super::System::User, storeid: &::windows::core::HSTRING, campaignid: &::windows::core::HSTRING, correlationvector: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<GetEntitlementResult>> {
        let this = &::windows::core::Interface::cast::<IAppInstallManager4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetFreeUserEntitlementForUserAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(user), ::core::mem::transmute_copy(storeid), ::core::mem::transmute_copy(campaignid), ::core::mem::transmute_copy(correlationvector), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetFreeDeviceEntitlementAsync(&self, storeid: &::windows::core::HSTRING, campaignid: &::windows::core::HSTRING, correlationvector: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<GetEntitlementResult>> {
        let this = &::windows::core::Interface::cast::<IAppInstallManager4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetFreeDeviceEntitlementAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(storeid), ::core::mem::transmute_copy(campaignid), ::core::mem::transmute_copy(correlationvector), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn AppInstallItemsWithGroupSupport(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<AppInstallItem>> {
        let this = &::windows::core::Interface::cast::<IAppInstallManager5>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AppInstallItemsWithGroupSupport)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SearchForAllUpdatesWithUpdateOptionsAsync(&self, correlationvector: &::windows::core::HSTRING, clientid: &::windows::core::HSTRING, updateoptions: &AppUpdateOptions) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Foundation::Collections::IVectorView<AppInstallItem>>> {
        let this = &::windows::core::Interface::cast::<IAppInstallManager6>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SearchForAllUpdatesWithUpdateOptionsAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(correlationvector), ::core::mem::transmute_copy(clientid), ::core::mem::transmute_copy(updateoptions), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"System\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "System"))]
    pub fn SearchForAllUpdatesWithUpdateOptionsForUserAsync(&self, user: &super::super::super::super::System::User, correlationvector: &::windows::core::HSTRING, clientid: &::windows::core::HSTRING, updateoptions: &AppUpdateOptions) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Foundation::Collections::IVectorView<AppInstallItem>>> {
        let this = &::windows::core::Interface::cast::<IAppInstallManager6>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SearchForAllUpdatesWithUpdateOptionsForUserAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(user), ::core::mem::transmute_copy(correlationvector), ::core::mem::transmute_copy(clientid), ::core::mem::transmute_copy(updateoptions), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SearchForUpdatesWithUpdateOptionsAsync(&self, productid: &::windows::core::HSTRING, skuid: &::windows::core::HSTRING, correlationvector: &::windows::core::HSTRING, clientid: &::windows::core::HSTRING, updateoptions: &AppUpdateOptions) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<AppInstallItem>> {
        let this = &::windows::core::Interface::cast::<IAppInstallManager6>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SearchForUpdatesWithUpdateOptionsAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(productid), ::core::mem::transmute_copy(skuid), ::core::mem::transmute_copy(correlationvector), ::core::mem::transmute_copy(clientid), ::core::mem::transmute_copy(updateoptions), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"System\"`*"]
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub fn SearchForUpdatesWithUpdateOptionsForUserAsync(&self, user: &super::super::super::super::System::User, productid: &::windows::core::HSTRING, skuid: &::windows::core::HSTRING, correlationvector: &::windows::core::HSTRING, clientid: &::windows::core::HSTRING, updateoptions: &AppUpdateOptions) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<AppInstallItem>> {
        let this = &::windows::core::Interface::cast::<IAppInstallManager6>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SearchForUpdatesWithUpdateOptionsForUserAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(user), ::core::mem::transmute_copy(productid), ::core::mem::transmute_copy(skuid), ::core::mem::transmute_copy(correlationvector), ::core::mem::transmute_copy(clientid), ::core::mem::transmute_copy(updateoptions), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn StartProductInstallWithOptionsAsync(&self, productid: &::windows::core::HSTRING, flightid: &::windows::core::HSTRING, clientid: &::windows::core::HSTRING, correlationvector: &::windows::core::HSTRING, installoptions: &AppInstallOptions) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Foundation::Collections::IVectorView<AppInstallItem>>> {
        let this = &::windows::core::Interface::cast::<IAppInstallManager6>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).StartProductInstallWithOptionsAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(productid), ::core::mem::transmute_copy(flightid), ::core::mem::transmute_copy(clientid), ::core::mem::transmute_copy(correlationvector), ::core::mem::transmute_copy(installoptions), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"System\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "System"))]
    pub fn StartProductInstallWithOptionsForUserAsync(&self, user: &super::super::super::super::System::User, productid: &::windows::core::HSTRING, flightid: &::windows::core::HSTRING, clientid: &::windows::core::HSTRING, correlationvector: &::windows::core::HSTRING, installoptions: &AppInstallOptions) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Foundation::Collections::IVectorView<AppInstallItem>>> {
        let this = &::windows::core::Interface::cast::<IAppInstallManager6>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).StartProductInstallWithOptionsForUserAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(user), ::core::mem::transmute_copy(productid), ::core::mem::transmute_copy(flightid), ::core::mem::transmute_copy(clientid), ::core::mem::transmute_copy(correlationvector), ::core::mem::transmute_copy(installoptions), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetIsPackageIdentityAllowedToInstallAsync(&self, correlationvector: &::windows::core::HSTRING, packageidentityname: &::windows::core::HSTRING, publishercertificatename: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::core::Interface::cast::<IAppInstallManager6>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetIsPackageIdentityAllowedToInstallAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(correlationvector), ::core::mem::transmute_copy(packageidentityname), ::core::mem::transmute_copy(publishercertificatename), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"System\"`*"]
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub fn GetIsPackageIdentityAllowedToInstallForUserAsync(&self, user: &super::super::super::super::System::User, correlationvector: &::windows::core::HSTRING, packageidentityname: &::windows::core::HSTRING, publishercertificatename: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::core::Interface::cast::<IAppInstallManager6>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetIsPackageIdentityAllowedToInstallForUserAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(user), ::core::mem::transmute_copy(correlationvector), ::core::mem::transmute_copy(packageidentityname), ::core::mem::transmute_copy(publishercertificatename), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CanInstallForAllUsers(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IAppInstallManager7>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CanInstallForAllUsers)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for AppInstallManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppInstallManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppInstallManager {}
impl ::core::fmt::Debug for AppInstallManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppInstallManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppInstallManager {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallManager;{9353e170-8441-4b45-bd72-7c2fa925beee})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for AppInstallManager {
    type Vtable = IAppInstallManager_Vtbl;
}
unsafe impl ::windows::core::Interface for AppInstallManager {
    const IID: ::windows::core::GUID = <IAppInstallManager as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppInstallManager {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallManager";
}
::windows::core::interface_hierarchy!(AppInstallManager, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for AppInstallManager {}
unsafe impl ::core::marker::Sync for AppInstallManager {}
#[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`*"]
#[repr(transparent)]
pub struct AppInstallManagerItemEventArgs(::windows::core::IUnknown);
impl AppInstallManagerItemEventArgs {
    pub fn Item(&self) -> ::windows::core::Result<AppInstallItem> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Item)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for AppInstallManagerItemEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppInstallManagerItemEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppInstallManagerItemEventArgs {}
impl ::core::fmt::Debug for AppInstallManagerItemEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppInstallManagerItemEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppInstallManagerItemEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallManagerItemEventArgs;{bc505743-4674-4dd1-957e-c25682086a14})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for AppInstallManagerItemEventArgs {
    type Vtable = IAppInstallManagerItemEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for AppInstallManagerItemEventArgs {
    const IID: ::windows::core::GUID = <IAppInstallManagerItemEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppInstallManagerItemEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallManagerItemEventArgs";
}
::windows::core::interface_hierarchy!(AppInstallManagerItemEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for AppInstallManagerItemEventArgs {}
unsafe impl ::core::marker::Sync for AppInstallManagerItemEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`*"]
#[repr(transparent)]
pub struct AppInstallOptions(::windows::core::IUnknown);
impl AppInstallOptions {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<AppInstallOptions, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn CatalogId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CatalogId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetCatalogId(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetCatalogId)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn ForceUseOfNonRemovableStorage(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ForceUseOfNonRemovableStorage)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetForceUseOfNonRemovableStorage(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetForceUseOfNonRemovableStorage)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn AllowForcedAppRestart(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AllowForcedAppRestart)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetAllowForcedAppRestart(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetAllowForcedAppRestart)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn Repair(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Repair)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetRepair(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetRepair)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Management_Deployment\"`*"]
    #[cfg(feature = "Management_Deployment")]
    pub fn TargetVolume(&self) -> ::windows::core::Result<super::super::super::super::Management::Deployment::PackageVolume> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TargetVolume)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Management_Deployment\"`*"]
    #[cfg(feature = "Management_Deployment")]
    pub fn SetTargetVolume(&self, value: &super::super::super::super::Management::Deployment::PackageVolume) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetTargetVolume)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn LaunchAfterInstall(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LaunchAfterInstall)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetLaunchAfterInstall(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetLaunchAfterInstall)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn PinToDesktopAfterInstall(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IAppInstallOptions2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PinToDesktopAfterInstall)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetPinToDesktopAfterInstall(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppInstallOptions2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetPinToDesktopAfterInstall)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn PinToStartAfterInstall(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IAppInstallOptions2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PinToStartAfterInstall)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetPinToStartAfterInstall(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppInstallOptions2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetPinToStartAfterInstall)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn PinToTaskbarAfterInstall(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IAppInstallOptions2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PinToTaskbarAfterInstall)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetPinToTaskbarAfterInstall(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppInstallOptions2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetPinToTaskbarAfterInstall)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn CompletedInstallToastNotificationMode(&self) -> ::windows::core::Result<AppInstallationToastNotificationMode> {
        let this = &::windows::core::Interface::cast::<IAppInstallOptions2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CompletedInstallToastNotificationMode)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetCompletedInstallToastNotificationMode(&self, value: AppInstallationToastNotificationMode) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppInstallOptions2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetCompletedInstallToastNotificationMode)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn InstallInProgressToastNotificationMode(&self) -> ::windows::core::Result<AppInstallationToastNotificationMode> {
        let this = &::windows::core::Interface::cast::<IAppInstallOptions2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).InstallInProgressToastNotificationMode)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetInstallInProgressToastNotificationMode(&self, value: AppInstallationToastNotificationMode) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppInstallOptions2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetInstallInProgressToastNotificationMode)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn InstallForAllUsers(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IAppInstallOptions2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).InstallForAllUsers)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetInstallForAllUsers(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppInstallOptions2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetInstallForAllUsers)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn StageButDoNotInstall(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IAppInstallOptions2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).StageButDoNotInstall)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetStageButDoNotInstall(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppInstallOptions2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetStageButDoNotInstall)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn CampaignId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IAppInstallOptions2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CampaignId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetCampaignId(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppInstallOptions2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetCampaignId)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn ExtendedCampaignId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IAppInstallOptions2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ExtendedCampaignId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetExtendedCampaignId(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppInstallOptions2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetExtendedCampaignId)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
}
impl ::core::clone::Clone for AppInstallOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppInstallOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppInstallOptions {}
impl ::core::fmt::Debug for AppInstallOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppInstallOptions").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppInstallOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallOptions;{c9808300-1cb8-4eb6-8c9f-6a30c64a5b51})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for AppInstallOptions {
    type Vtable = IAppInstallOptions_Vtbl;
}
unsafe impl ::windows::core::Interface for AppInstallOptions {
    const IID: ::windows::core::GUID = <IAppInstallOptions as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppInstallOptions {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallOptions";
}
::windows::core::interface_hierarchy!(AppInstallOptions, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for AppInstallOptions {}
unsafe impl ::core::marker::Sync for AppInstallOptions {}
#[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`*"]
#[repr(transparent)]
pub struct AppInstallStatus(::windows::core::IUnknown);
impl AppInstallStatus {
    pub fn InstallState(&self) -> ::windows::core::Result<AppInstallState> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).InstallState)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn DownloadSizeInBytes(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DownloadSizeInBytes)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn BytesDownloaded(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BytesDownloaded)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn PercentComplete(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PercentComplete)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ErrorCode(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ErrorCode)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::core::Result<super::super::super::super::System::User> {
        let this = &::windows::core::Interface::cast::<IAppInstallStatus2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).User)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ReadyForLaunch(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IAppInstallStatus2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReadyForLaunch)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsStaged(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IAppInstallStatus3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsStaged)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for AppInstallStatus {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppInstallStatus {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppInstallStatus {}
impl ::core::fmt::Debug for AppInstallStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppInstallStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppInstallStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallStatus;{936dccfa-2450-4126-88b1-6127a644dd5c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for AppInstallStatus {
    type Vtable = IAppInstallStatus_Vtbl;
}
unsafe impl ::windows::core::Interface for AppInstallStatus {
    const IID: ::windows::core::GUID = <IAppInstallStatus as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppInstallStatus {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallStatus";
}
::windows::core::interface_hierarchy!(AppInstallStatus, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for AppInstallStatus {}
unsafe impl ::core::marker::Sync for AppInstallStatus {}
#[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`*"]
#[repr(transparent)]
pub struct AppUpdateOptions(::windows::core::IUnknown);
impl AppUpdateOptions {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<AppUpdateOptions, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn CatalogId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CatalogId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetCatalogId(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetCatalogId)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn AllowForcedAppRestart(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AllowForcedAppRestart)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetAllowForcedAppRestart(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetAllowForcedAppRestart)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn AutomaticallyDownloadAndInstallUpdateIfFound(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IAppUpdateOptions2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AutomaticallyDownloadAndInstallUpdateIfFound)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetAutomaticallyDownloadAndInstallUpdateIfFound(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppUpdateOptions2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetAutomaticallyDownloadAndInstallUpdateIfFound)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
}
impl ::core::clone::Clone for AppUpdateOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppUpdateOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppUpdateOptions {}
impl ::core::fmt::Debug for AppUpdateOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppUpdateOptions").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppUpdateOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Store.Preview.InstallControl.AppUpdateOptions;{26f0b02f-c2f3-4aea-af8c-6308dd9db85f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for AppUpdateOptions {
    type Vtable = IAppUpdateOptions_Vtbl;
}
unsafe impl ::windows::core::Interface for AppUpdateOptions {
    const IID: ::windows::core::GUID = <IAppUpdateOptions as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppUpdateOptions {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.InstallControl.AppUpdateOptions";
}
::windows::core::interface_hierarchy!(AppUpdateOptions, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for AppUpdateOptions {}
unsafe impl ::core::marker::Sync for AppUpdateOptions {}
#[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`*"]
#[repr(transparent)]
pub struct GetEntitlementResult(::windows::core::IUnknown);
impl GetEntitlementResult {
    pub fn Status(&self) -> ::windows::core::Result<GetEntitlementStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Status)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for GetEntitlementResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for GetEntitlementResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GetEntitlementResult {}
impl ::core::fmt::Debug for GetEntitlementResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GetEntitlementResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for GetEntitlementResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Store.Preview.InstallControl.GetEntitlementResult;{74fc843f-1a9e-4609-8e4d-819086d08a3d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for GetEntitlementResult {
    type Vtable = IGetEntitlementResult_Vtbl;
}
unsafe impl ::windows::core::Interface for GetEntitlementResult {
    const IID: ::windows::core::GUID = <IGetEntitlementResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for GetEntitlementResult {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.InstallControl.GetEntitlementResult";
}
::windows::core::interface_hierarchy!(GetEntitlementResult, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for GetEntitlementResult {}
unsafe impl ::core::marker::Sync for GetEntitlementResult {}
#[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AppInstallState(pub i32);
impl AppInstallState {
    pub const Pending: Self = Self(0i32);
    pub const Starting: Self = Self(1i32);
    pub const AcquiringLicense: Self = Self(2i32);
    pub const Downloading: Self = Self(3i32);
    pub const RestoringData: Self = Self(4i32);
    pub const Installing: Self = Self(5i32);
    pub const Completed: Self = Self(6i32);
    pub const Canceled: Self = Self(7i32);
    pub const Paused: Self = Self(8i32);
    pub const Error: Self = Self(9i32);
    pub const PausedLowBattery: Self = Self(10i32);
    pub const PausedWiFiRecommended: Self = Self(11i32);
    pub const PausedWiFiRequired: Self = Self(12i32);
    pub const ReadyToDownload: Self = Self(13i32);
}
impl ::core::marker::Copy for AppInstallState {}
impl ::core::clone::Clone for AppInstallState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppInstallState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for AppInstallState {
    type Abi = Self;
}
impl ::core::fmt::Debug for AppInstallState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppInstallState").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppInstallState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AppInstallType(pub i32);
impl AppInstallType {
    pub const Install: Self = Self(0i32);
    pub const Update: Self = Self(1i32);
    pub const Repair: Self = Self(2i32);
}
impl ::core::marker::Copy for AppInstallType {}
impl ::core::clone::Clone for AppInstallType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppInstallType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for AppInstallType {
    type Abi = Self;
}
impl ::core::fmt::Debug for AppInstallType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppInstallType").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppInstallType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AppInstallationToastNotificationMode(pub i32);
impl AppInstallationToastNotificationMode {
    pub const Default: Self = Self(0i32);
    pub const Toast: Self = Self(1i32);
    pub const ToastWithoutPopup: Self = Self(2i32);
    pub const NoToast: Self = Self(3i32);
}
impl ::core::marker::Copy for AppInstallationToastNotificationMode {}
impl ::core::clone::Clone for AppInstallationToastNotificationMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppInstallationToastNotificationMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for AppInstallationToastNotificationMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for AppInstallationToastNotificationMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppInstallationToastNotificationMode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppInstallationToastNotificationMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallationToastNotificationMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AutoUpdateSetting(pub i32);
impl AutoUpdateSetting {
    pub const Disabled: Self = Self(0i32);
    pub const Enabled: Self = Self(1i32);
    pub const DisabledByPolicy: Self = Self(2i32);
    pub const EnabledByPolicy: Self = Self(3i32);
}
impl ::core::marker::Copy for AutoUpdateSetting {}
impl ::core::clone::Clone for AutoUpdateSetting {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AutoUpdateSetting {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for AutoUpdateSetting {
    type Abi = Self;
}
impl ::core::fmt::Debug for AutoUpdateSetting {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AutoUpdateSetting").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AutoUpdateSetting {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Store.Preview.InstallControl.AutoUpdateSetting;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct GetEntitlementStatus(pub i32);
impl GetEntitlementStatus {
    pub const Succeeded: Self = Self(0i32);
    pub const NoStoreAccount: Self = Self(1i32);
    pub const NetworkError: Self = Self(2i32);
    pub const ServerError: Self = Self(3i32);
}
impl ::core::marker::Copy for GetEntitlementStatus {}
impl ::core::clone::Clone for GetEntitlementStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GetEntitlementStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for GetEntitlementStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for GetEntitlementStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GetEntitlementStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for GetEntitlementStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Store.Preview.InstallControl.GetEntitlementStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
