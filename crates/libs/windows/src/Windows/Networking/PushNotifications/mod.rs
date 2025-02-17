#[doc(hidden)]
#[repr(transparent)]
pub struct IPushNotificationChannel(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPushNotificationChannel {
    type Vtable = IPushNotificationChannel_Vtbl;
}
unsafe impl ::windows::core::Interface for IPushNotificationChannel {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2b28102e_ef0b_4f39_9b8a_a3c194de7081);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPushNotificationChannel_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Uri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ExpirationTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ExpirationTime: usize,
    pub Close: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub PushNotificationReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PushNotificationReceived: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePushNotificationReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePushNotificationReceived: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPushNotificationChannelManagerForUser(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPushNotificationChannelManagerForUser {
    type Vtable = IPushNotificationChannelManagerForUser_Vtbl;
}
unsafe impl ::windows::core::Interface for IPushNotificationChannelManagerForUser {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa4c45704_1182_42c7_8890_f563c4890dc4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPushNotificationChannelManagerForUser_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub CreatePushNotificationChannelForApplicationAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreatePushNotificationChannelForApplicationAsync: usize,
    #[cfg(feature = "Foundation")]
    pub CreatePushNotificationChannelForApplicationAsyncWithId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, applicationid: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreatePushNotificationChannelForApplicationAsyncWithId: usize,
    #[cfg(feature = "Foundation")]
    pub CreatePushNotificationChannelForSecondaryTileAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tileid: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreatePushNotificationChannelForSecondaryTileAsync: usize,
    #[cfg(feature = "System")]
    pub User: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))]
    User: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPushNotificationChannelManagerForUser2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPushNotificationChannelManagerForUser2 {
    type Vtable = IPushNotificationChannelManagerForUser2_Vtbl;
}
unsafe impl ::windows::core::Interface for IPushNotificationChannelManagerForUser2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc38b066a_7cc1_4dac_87fd_be6e920414a4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPushNotificationChannelManagerForUser2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub CreateRawPushNotificationChannelWithAlternateKeyForApplicationAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appserverkey: *mut ::core::ffi::c_void, channelid: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    CreateRawPushNotificationChannelWithAlternateKeyForApplicationAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub CreateRawPushNotificationChannelWithAlternateKeyForApplicationAsyncWithId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appserverkey: *mut ::core::ffi::c_void, channelid: *mut ::core::ffi::c_void, appid: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    CreateRawPushNotificationChannelWithAlternateKeyForApplicationAsyncWithId: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPushNotificationChannelManagerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPushNotificationChannelManagerStatics {
    type Vtable = IPushNotificationChannelManagerStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IPushNotificationChannelManagerStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8baf9b65_77a1_4588_bd19_861529a9dcf0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPushNotificationChannelManagerStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub CreatePushNotificationChannelForApplicationAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreatePushNotificationChannelForApplicationAsync: usize,
    #[cfg(feature = "Foundation")]
    pub CreatePushNotificationChannelForApplicationAsyncWithId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, applicationid: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreatePushNotificationChannelForApplicationAsyncWithId: usize,
    #[cfg(feature = "Foundation")]
    pub CreatePushNotificationChannelForSecondaryTileAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tileid: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreatePushNotificationChannelForSecondaryTileAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPushNotificationChannelManagerStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPushNotificationChannelManagerStatics2 {
    type Vtable = IPushNotificationChannelManagerStatics2_Vtbl;
}
unsafe impl ::windows::core::Interface for IPushNotificationChannelManagerStatics2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb444a65d_a7e9_4b28_950e_f375a907f9df);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPushNotificationChannelManagerStatics2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "System")]
    pub GetForUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))]
    GetForUser: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPushNotificationChannelManagerStatics3(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPushNotificationChannelManagerStatics3 {
    type Vtable = IPushNotificationChannelManagerStatics3_Vtbl;
}
unsafe impl ::windows::core::Interface for IPushNotificationChannelManagerStatics3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4701fefe_0ede_4a3f_ae78_bfa471496925);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPushNotificationChannelManagerStatics3_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPushNotificationChannelManagerStatics4(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPushNotificationChannelManagerStatics4 {
    type Vtable = IPushNotificationChannelManagerStatics4_Vtbl;
}
unsafe impl ::windows::core::Interface for IPushNotificationChannelManagerStatics4 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbc540efb_7820_5a5b_9c01_b4757f774025);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPushNotificationChannelManagerStatics4_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub ChannelsRevoked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ChannelsRevoked: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveChannelsRevoked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveChannelsRevoked: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPushNotificationChannelsRevokedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPushNotificationChannelsRevokedEventArgs {
    type Vtable = IPushNotificationChannelsRevokedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IPushNotificationChannelsRevokedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x20e1a24c_1a34_5beb_aae2_40c232c8c140);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPushNotificationChannelsRevokedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPushNotificationReceivedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPushNotificationReceivedEventArgs {
    type Vtable = IPushNotificationReceivedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IPushNotificationReceivedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd1065e0c_36cd_484c_b935_0a99b753cf00);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPushNotificationReceivedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub SetCancel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub Cancel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub NotificationType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PushNotificationType) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Notifications")]
    pub ToastNotification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Notifications"))]
    ToastNotification: usize,
    #[cfg(feature = "UI_Notifications")]
    pub TileNotification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Notifications"))]
    TileNotification: usize,
    #[cfg(feature = "UI_Notifications")]
    pub BadgeNotification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Notifications"))]
    BadgeNotification: usize,
    pub RawNotification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRawNotification(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IRawNotification {
    type Vtable = IRawNotification_Vtbl;
}
unsafe impl ::windows::core::Interface for IRawNotification {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1a227281_3b79_42ac_9963_22ab00d4f0b7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRawNotification_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Content: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRawNotification2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IRawNotification2 {
    type Vtable = IRawNotification2_Vtbl;
}
unsafe impl ::windows::core::Interface for IRawNotification2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe6d0cf19_0c6f_4cdd_9424_eec5be014d26);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRawNotification2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Headers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Headers: usize,
    pub ChannelId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRawNotification3(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IRawNotification3 {
    type Vtable = IRawNotification3_Vtbl;
}
unsafe impl ::windows::core::Interface for IRawNotification3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x62737dde_8a73_424c_ab44_5635f40a96e5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRawNotification3_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub ContentBytes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    ContentBytes: usize,
}
#[doc = "*Required features: `\"Networking_PushNotifications\"`*"]
#[repr(transparent)]
pub struct PushNotificationChannel(::windows::core::IUnknown);
impl PushNotificationChannel {
    pub fn Uri(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Uri)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ExpirationTime(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ExpirationTime)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Close)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PushNotificationReceived(&self, handler: &super::super::Foundation::TypedEventHandler<PushNotificationChannel, PushNotificationReceivedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PushNotificationReceived)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePushNotificationReceived(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemovePushNotificationReceived)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
}
impl ::core::clone::Clone for PushNotificationChannel {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PushNotificationChannel {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PushNotificationChannel {}
impl ::core::fmt::Debug for PushNotificationChannel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PushNotificationChannel").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PushNotificationChannel {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.PushNotifications.PushNotificationChannel;{2b28102e-ef0b-4f39-9b8a-a3c194de7081})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for PushNotificationChannel {
    type Vtable = IPushNotificationChannel_Vtbl;
}
unsafe impl ::windows::core::Interface for PushNotificationChannel {
    const IID: ::windows::core::GUID = <IPushNotificationChannel as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PushNotificationChannel {
    const NAME: &'static str = "Windows.Networking.PushNotifications.PushNotificationChannel";
}
::windows::core::interface_hierarchy!(PushNotificationChannel, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for PushNotificationChannel {}
unsafe impl ::core::marker::Sync for PushNotificationChannel {}
#[doc = "*Required features: `\"Networking_PushNotifications\"`*"]
pub struct PushNotificationChannelManager;
impl PushNotificationChannelManager {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreatePushNotificationChannelForApplicationAsync() -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PushNotificationChannel>> {
        Self::IPushNotificationChannelManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreatePushNotificationChannelForApplicationAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreatePushNotificationChannelForApplicationAsyncWithId(applicationid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PushNotificationChannel>> {
        Self::IPushNotificationChannelManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreatePushNotificationChannelForApplicationAsyncWithId)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(applicationid), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreatePushNotificationChannelForSecondaryTileAsync(tileid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PushNotificationChannel>> {
        Self::IPushNotificationChannelManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreatePushNotificationChannelForSecondaryTileAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(tileid), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn GetForUser(user: &super::super::System::User) -> ::windows::core::Result<PushNotificationChannelManagerForUser> {
        Self::IPushNotificationChannelManagerStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetForUser)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(user), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn GetDefault() -> ::windows::core::Result<PushNotificationChannelManagerForUser> {
        Self::IPushNotificationChannelManagerStatics3(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDefault)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ChannelsRevoked(handler: &super::super::Foundation::EventHandler<PushNotificationChannelsRevokedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        Self::IPushNotificationChannelManagerStatics4(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ChannelsRevoked)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveChannelsRevoked(token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        Self::IPushNotificationChannelManagerStatics4(|this| unsafe { (::windows::core::Vtable::vtable(this).RemoveChannelsRevoked)(::windows::core::Vtable::as_raw(this), token).ok() })
    }
    #[doc(hidden)]
    pub fn IPushNotificationChannelManagerStatics<R, F: FnOnce(&IPushNotificationChannelManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<PushNotificationChannelManager, IPushNotificationChannelManagerStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IPushNotificationChannelManagerStatics2<R, F: FnOnce(&IPushNotificationChannelManagerStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<PushNotificationChannelManager, IPushNotificationChannelManagerStatics2> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IPushNotificationChannelManagerStatics3<R, F: FnOnce(&IPushNotificationChannelManagerStatics3) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<PushNotificationChannelManager, IPushNotificationChannelManagerStatics3> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IPushNotificationChannelManagerStatics4<R, F: FnOnce(&IPushNotificationChannelManagerStatics4) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<PushNotificationChannelManager, IPushNotificationChannelManagerStatics4> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for PushNotificationChannelManager {
    const NAME: &'static str = "Windows.Networking.PushNotifications.PushNotificationChannelManager";
}
#[doc = "*Required features: `\"Networking_PushNotifications\"`*"]
#[repr(transparent)]
pub struct PushNotificationChannelManagerForUser(::windows::core::IUnknown);
impl PushNotificationChannelManagerForUser {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreatePushNotificationChannelForApplicationAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PushNotificationChannel>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreatePushNotificationChannelForApplicationAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreatePushNotificationChannelForApplicationAsyncWithId(&self, applicationid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PushNotificationChannel>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreatePushNotificationChannelForApplicationAsyncWithId)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(applicationid), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreatePushNotificationChannelForSecondaryTileAsync(&self, tileid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PushNotificationChannel>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreatePushNotificationChannelForSecondaryTileAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(tileid), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).User)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn CreateRawPushNotificationChannelWithAlternateKeyForApplicationAsync<P0, E0>(&self, appserverkey: P0, channelid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PushNotificationChannel>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Storage::Streams::IBuffer>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IPushNotificationChannelManagerForUser2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateRawPushNotificationChannelWithAlternateKeyForApplicationAsync)(::windows::core::Vtable::as_raw(this), appserverkey.try_into().map_err(|e| e.into())?.abi(), ::core::mem::transmute_copy(channelid), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn CreateRawPushNotificationChannelWithAlternateKeyForApplicationAsyncWithId<P0, E0>(&self, appserverkey: P0, channelid: &::windows::core::HSTRING, appid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PushNotificationChannel>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Storage::Streams::IBuffer>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IPushNotificationChannelManagerForUser2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateRawPushNotificationChannelWithAlternateKeyForApplicationAsyncWithId)(::windows::core::Vtable::as_raw(this), appserverkey.try_into().map_err(|e| e.into())?.abi(), ::core::mem::transmute_copy(channelid), ::core::mem::transmute_copy(appid), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for PushNotificationChannelManagerForUser {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PushNotificationChannelManagerForUser {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PushNotificationChannelManagerForUser {}
impl ::core::fmt::Debug for PushNotificationChannelManagerForUser {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PushNotificationChannelManagerForUser").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PushNotificationChannelManagerForUser {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.PushNotifications.PushNotificationChannelManagerForUser;{a4c45704-1182-42c7-8890-f563c4890dc4})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for PushNotificationChannelManagerForUser {
    type Vtable = IPushNotificationChannelManagerForUser_Vtbl;
}
unsafe impl ::windows::core::Interface for PushNotificationChannelManagerForUser {
    const IID: ::windows::core::GUID = <IPushNotificationChannelManagerForUser as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PushNotificationChannelManagerForUser {
    const NAME: &'static str = "Windows.Networking.PushNotifications.PushNotificationChannelManagerForUser";
}
::windows::core::interface_hierarchy!(PushNotificationChannelManagerForUser, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for PushNotificationChannelManagerForUser {}
unsafe impl ::core::marker::Sync for PushNotificationChannelManagerForUser {}
#[doc = "*Required features: `\"Networking_PushNotifications\"`*"]
#[repr(transparent)]
pub struct PushNotificationChannelsRevokedEventArgs(::windows::core::IUnknown);
impl PushNotificationChannelsRevokedEventArgs {}
impl ::core::clone::Clone for PushNotificationChannelsRevokedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PushNotificationChannelsRevokedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PushNotificationChannelsRevokedEventArgs {}
impl ::core::fmt::Debug for PushNotificationChannelsRevokedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PushNotificationChannelsRevokedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PushNotificationChannelsRevokedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.PushNotifications.PushNotificationChannelsRevokedEventArgs;{20e1a24c-1a34-5beb-aae2-40c232c8c140})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for PushNotificationChannelsRevokedEventArgs {
    type Vtable = IPushNotificationChannelsRevokedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for PushNotificationChannelsRevokedEventArgs {
    const IID: ::windows::core::GUID = <IPushNotificationChannelsRevokedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PushNotificationChannelsRevokedEventArgs {
    const NAME: &'static str = "Windows.Networking.PushNotifications.PushNotificationChannelsRevokedEventArgs";
}
::windows::core::interface_hierarchy!(PushNotificationChannelsRevokedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for PushNotificationChannelsRevokedEventArgs {}
unsafe impl ::core::marker::Sync for PushNotificationChannelsRevokedEventArgs {}
#[doc = "*Required features: `\"Networking_PushNotifications\"`*"]
#[repr(transparent)]
pub struct PushNotificationReceivedEventArgs(::windows::core::IUnknown);
impl PushNotificationReceivedEventArgs {
    pub fn SetCancel(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetCancel)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn Cancel(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Cancel)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn NotificationType(&self) -> ::windows::core::Result<PushNotificationType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NotificationType)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Notifications\"`*"]
    #[cfg(feature = "UI_Notifications")]
    pub fn ToastNotification(&self) -> ::windows::core::Result<super::super::UI::Notifications::ToastNotification> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ToastNotification)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Notifications\"`*"]
    #[cfg(feature = "UI_Notifications")]
    pub fn TileNotification(&self) -> ::windows::core::Result<super::super::UI::Notifications::TileNotification> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TileNotification)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Notifications\"`*"]
    #[cfg(feature = "UI_Notifications")]
    pub fn BadgeNotification(&self) -> ::windows::core::Result<super::super::UI::Notifications::BadgeNotification> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BadgeNotification)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn RawNotification(&self) -> ::windows::core::Result<RawNotification> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RawNotification)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for PushNotificationReceivedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PushNotificationReceivedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PushNotificationReceivedEventArgs {}
impl ::core::fmt::Debug for PushNotificationReceivedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PushNotificationReceivedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PushNotificationReceivedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.PushNotifications.PushNotificationReceivedEventArgs;{d1065e0c-36cd-484c-b935-0a99b753cf00})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for PushNotificationReceivedEventArgs {
    type Vtable = IPushNotificationReceivedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for PushNotificationReceivedEventArgs {
    const IID: ::windows::core::GUID = <IPushNotificationReceivedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PushNotificationReceivedEventArgs {
    const NAME: &'static str = "Windows.Networking.PushNotifications.PushNotificationReceivedEventArgs";
}
::windows::core::interface_hierarchy!(PushNotificationReceivedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for PushNotificationReceivedEventArgs {}
unsafe impl ::core::marker::Sync for PushNotificationReceivedEventArgs {}
#[doc = "*Required features: `\"Networking_PushNotifications\"`*"]
#[repr(transparent)]
pub struct RawNotification(::windows::core::IUnknown);
impl RawNotification {
    pub fn Content(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Content)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Headers(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::HSTRING>> {
        let this = &::windows::core::Interface::cast::<IRawNotification2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Headers)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ChannelId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IRawNotification2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ChannelId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn ContentBytes(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer> {
        let this = &::windows::core::Interface::cast::<IRawNotification3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ContentBytes)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for RawNotification {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RawNotification {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RawNotification {}
impl ::core::fmt::Debug for RawNotification {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RawNotification").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for RawNotification {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.PushNotifications.RawNotification;{1a227281-3b79-42ac-9963-22ab00d4f0b7})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for RawNotification {
    type Vtable = IRawNotification_Vtbl;
}
unsafe impl ::windows::core::Interface for RawNotification {
    const IID: ::windows::core::GUID = <IRawNotification as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for RawNotification {
    const NAME: &'static str = "Windows.Networking.PushNotifications.RawNotification";
}
::windows::core::interface_hierarchy!(RawNotification, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for RawNotification {}
unsafe impl ::core::marker::Sync for RawNotification {}
#[doc = "*Required features: `\"Networking_PushNotifications\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PushNotificationType(pub i32);
impl PushNotificationType {
    pub const Toast: Self = Self(0i32);
    pub const Tile: Self = Self(1i32);
    pub const Badge: Self = Self(2i32);
    pub const Raw: Self = Self(3i32);
    pub const TileFlyout: Self = Self(4i32);
}
impl ::core::marker::Copy for PushNotificationType {}
impl ::core::clone::Clone for PushNotificationType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PushNotificationType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PushNotificationType {
    type Abi = Self;
}
impl ::core::fmt::Debug for PushNotificationType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PushNotificationType").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PushNotificationType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Networking.PushNotifications.PushNotificationType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
