#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct INDClient(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Vtable for INDClient {
    type Vtable = INDClient_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for INDClient {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3bd6781b_61b8_46e2_99a5_8abcb6b9f7d6);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct INDClient_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RegistrationCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RegistrationCompleted: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveRegistrationCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveRegistrationCompleted: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub ProximityDetectionCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    ProximityDetectionCompleted: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveProximityDetectionCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveProximityDetectionCompleted: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub LicenseFetchCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    LicenseFetchCompleted: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveLicenseFetchCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveLicenseFetchCompleted: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub ReRegistrationNeeded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    ReRegistrationNeeded: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveReRegistrationNeeded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveReRegistrationNeeded: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub ClosedCaptionDataReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    ClosedCaptionDataReceived: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveClosedCaptionDataReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveClosedCaptionDataReceived: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub StartAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contenturl: *mut ::core::ffi::c_void, startasyncoptions: u32, registrationcustomdata: *mut ::core::ffi::c_void, licensefetchdescriptor: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    StartAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub LicenseFetchAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, licensefetchdescriptor: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    LicenseFetchAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub ReRegistrationAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, registrationcustomdata: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    ReRegistrationAsync: usize,
    #[cfg(feature = "deprecated")]
    pub Close: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Close: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct INDClientFactory(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Vtable for INDClientFactory {
    type Vtable = INDClientFactory_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for INDClientFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3e53dd62_fee8_451f_b0d4_f706cca3e037);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct INDClientFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, downloadengine: *mut ::core::ffi::c_void, streamparser: *mut ::core::ffi::c_void, pmessenger: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    CreateInstance: usize,
}
#[doc = "*Required features: `\"Media_Protection_PlayReady\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct INDClosedCaptionDataReceivedEventArgs(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl INDClosedCaptionDataReceivedEventArgs {
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn ClosedCaptionDataFormat(&self) -> ::windows::core::Result<NDClosedCaptionFormat> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ClosedCaptionDataFormat)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn PresentationTimestamp(&self) -> ::windows::core::Result<i64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PresentationTimestamp)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn ClosedCaptionData(&self) -> ::windows::core::Result<::windows::core::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ClosedCaptionData)(::windows::core::Vtable::as_raw(this), ::windows::core::Array::<u8>::set_abi_len(result__.assume_init_mut()), result__.as_mut_ptr() as *mut _ as _).and_then(|| result__.assume_init())
        }
    }
}
#[cfg(feature = "deprecated")]
::windows::core::interface_hierarchy!(INDClosedCaptionDataReceivedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for INDClosedCaptionDataReceivedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for INDClosedCaptionDataReceivedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for INDClosedCaptionDataReceivedEventArgs {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for INDClosedCaptionDataReceivedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INDClosedCaptionDataReceivedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for INDClosedCaptionDataReceivedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{4738d29f-c345-4649-8468-b8c5fc357190}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Vtable for INDClosedCaptionDataReceivedEventArgs {
    type Vtable = INDClosedCaptionDataReceivedEventArgs_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for INDClosedCaptionDataReceivedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4738d29f_c345_4649_8468_b8c5fc357190);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct INDClosedCaptionDataReceivedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub ClosedCaptionDataFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut NDClosedCaptionFormat) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ClosedCaptionDataFormat: usize,
    #[cfg(feature = "deprecated")]
    pub PresentationTimestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i64) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    PresentationTimestamp: usize,
    #[cfg(feature = "deprecated")]
    pub ClosedCaptionData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ClosedCaptionData: usize,
}
#[doc = "*Required features: `\"Media_Protection_PlayReady\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct INDCustomData(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl INDCustomData {
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn CustomDataTypeID(&self) -> ::windows::core::Result<::windows::core::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CustomDataTypeID)(::windows::core::Vtable::as_raw(this), ::windows::core::Array::<u8>::set_abi_len(result__.assume_init_mut()), result__.as_mut_ptr() as *mut _ as _).and_then(|| result__.assume_init())
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn CustomData(&self) -> ::windows::core::Result<::windows::core::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CustomData)(::windows::core::Vtable::as_raw(this), ::windows::core::Array::<u8>::set_abi_len(result__.assume_init_mut()), result__.as_mut_ptr() as *mut _ as _).and_then(|| result__.assume_init())
        }
    }
}
#[cfg(feature = "deprecated")]
::windows::core::interface_hierarchy!(INDCustomData, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for INDCustomData {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for INDCustomData {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for INDCustomData {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for INDCustomData {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INDCustomData").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for INDCustomData {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{f5cb0fdc-2d09-4f19-b5e1-76a0b3ee9267}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Vtable for INDCustomData {
    type Vtable = INDCustomData_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for INDCustomData {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf5cb0fdc_2d09_4f19_b5e1_76a0b3ee9267);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct INDCustomData_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub CustomDataTypeID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    CustomDataTypeID: usize,
    #[cfg(feature = "deprecated")]
    pub CustomData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    CustomData: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct INDCustomDataFactory(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Vtable for INDCustomDataFactory {
    type Vtable = INDCustomDataFactory_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for INDCustomDataFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd65405ab_3424_4833_8c9a_af5fdeb22872);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct INDCustomDataFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, customDataTypeIDBytes_array_size: u32, customdatatypeidbytes: *const u8, customDataBytes_array_size: u32, customdatabytes: *const u8, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    CreateInstance: usize,
}
#[doc = "*Required features: `\"Media_Protection_PlayReady\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct INDDownloadEngine(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl INDDownloadEngine {
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn Open(&self, uri: &super::super::super::Foundation::Uri, sessionidbytes: &[u8]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Open)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(uri), sessionidbytes.len() as u32, sessionidbytes.as_ptr()).ok() }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn Pause(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Pause)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn Resume(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Resume)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Close)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn Seek(&self, startposition: super::super::super::Foundation::TimeSpan) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Seek)(::windows::core::Vtable::as_raw(this), startposition).ok() }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn CanSeek(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CanSeek)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn BufferFullMinThresholdInSamples(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BufferFullMinThresholdInSamples)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn BufferFullMaxThresholdInSamples(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BufferFullMaxThresholdInSamples)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn Notifier(&self) -> ::windows::core::Result<NDDownloadEngineNotifier> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Notifier)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
#[cfg(feature = "deprecated")]
::windows::core::interface_hierarchy!(INDDownloadEngine, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for INDDownloadEngine {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for INDDownloadEngine {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for INDDownloadEngine {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for INDDownloadEngine {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INDDownloadEngine").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for INDDownloadEngine {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{2d223d65-c4b6-4438-8d46-b96e6d0fb21f}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Vtable for INDDownloadEngine {
    type Vtable = INDDownloadEngine_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for INDDownloadEngine {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2d223d65_c4b6_4438_8d46_b96e6d0fb21f);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct INDDownloadEngine_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub Open: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: *mut ::core::ffi::c_void, sessionIDBytes_array_size: u32, sessionidbytes: *const u8) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    Open: usize,
    #[cfg(feature = "deprecated")]
    pub Pause: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Pause: usize,
    #[cfg(feature = "deprecated")]
    pub Resume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Resume: usize,
    #[cfg(feature = "deprecated")]
    pub Close: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Close: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub Seek: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startposition: super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    Seek: usize,
    #[cfg(feature = "deprecated")]
    pub CanSeek: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    CanSeek: usize,
    #[cfg(feature = "deprecated")]
    pub BufferFullMinThresholdInSamples: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    BufferFullMinThresholdInSamples: usize,
    #[cfg(feature = "deprecated")]
    pub BufferFullMaxThresholdInSamples: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    BufferFullMaxThresholdInSamples: usize,
    #[cfg(feature = "deprecated")]
    pub Notifier: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Notifier: usize,
}
#[doc = "*Required features: `\"Media_Protection_PlayReady\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct INDDownloadEngineNotifier(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl INDDownloadEngineNotifier {
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn OnStreamOpened(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).OnStreamOpened)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn OnPlayReadyObjectReceived(&self, databytes: &[u8]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).OnPlayReadyObjectReceived)(::windows::core::Vtable::as_raw(this), databytes.len() as u32, databytes.as_ptr()).ok() }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn OnContentIDReceived<P0, E0>(&self, licensefetchdescriptor: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<INDLicenseFetchDescriptor>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).OnContentIDReceived)(::windows::core::Vtable::as_raw(this), licensefetchdescriptor.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn OnDataReceived(&self, databytes: &[u8], bytesreceived: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).OnDataReceived)(::windows::core::Vtable::as_raw(this), databytes.len() as u32, databytes.as_ptr(), bytesreceived).ok() }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn OnEndOfStream(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).OnEndOfStream)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn OnNetworkError(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).OnNetworkError)(::windows::core::Vtable::as_raw(this)).ok() }
    }
}
#[cfg(feature = "deprecated")]
::windows::core::interface_hierarchy!(INDDownloadEngineNotifier, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for INDDownloadEngineNotifier {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for INDDownloadEngineNotifier {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for INDDownloadEngineNotifier {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for INDDownloadEngineNotifier {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INDDownloadEngineNotifier").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for INDDownloadEngineNotifier {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{d720b4d4-f4b8-4530-a809-9193a571e7fc}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Vtable for INDDownloadEngineNotifier {
    type Vtable = INDDownloadEngineNotifier_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for INDDownloadEngineNotifier {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd720b4d4_f4b8_4530_a809_9193a571e7fc);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct INDDownloadEngineNotifier_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub OnStreamOpened: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    OnStreamOpened: usize,
    #[cfg(feature = "deprecated")]
    pub OnPlayReadyObjectReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dataBytes_array_size: u32, databytes: *const u8) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    OnPlayReadyObjectReceived: usize,
    #[cfg(feature = "deprecated")]
    pub OnContentIDReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, licensefetchdescriptor: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    OnContentIDReceived: usize,
    #[cfg(feature = "deprecated")]
    pub OnDataReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dataBytes_array_size: u32, databytes: *const u8, bytesreceived: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    OnDataReceived: usize,
    #[cfg(feature = "deprecated")]
    pub OnEndOfStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    OnEndOfStream: usize,
    #[cfg(feature = "deprecated")]
    pub OnNetworkError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    OnNetworkError: usize,
}
#[doc = "*Required features: `\"Media_Protection_PlayReady\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct INDLicenseFetchCompletedEventArgs(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl INDLicenseFetchCompletedEventArgs {
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn ResponseCustomData(&self) -> ::windows::core::Result<INDCustomData> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ResponseCustomData)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
#[cfg(feature = "deprecated")]
::windows::core::interface_hierarchy!(INDLicenseFetchCompletedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for INDLicenseFetchCompletedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for INDLicenseFetchCompletedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for INDLicenseFetchCompletedEventArgs {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for INDLicenseFetchCompletedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INDLicenseFetchCompletedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for INDLicenseFetchCompletedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{1ee30a1a-11b2-4558-8865-e3a516922517}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Vtable for INDLicenseFetchCompletedEventArgs {
    type Vtable = INDLicenseFetchCompletedEventArgs_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for INDLicenseFetchCompletedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1ee30a1a_11b2_4558_8865_e3a516922517);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct INDLicenseFetchCompletedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub ResponseCustomData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ResponseCustomData: usize,
}
#[doc = "*Required features: `\"Media_Protection_PlayReady\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct INDLicenseFetchDescriptor(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl INDLicenseFetchDescriptor {
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn ContentIDType(&self) -> ::windows::core::Result<NDContentIDType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ContentIDType)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn ContentID(&self) -> ::windows::core::Result<::windows::core::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ContentID)(::windows::core::Vtable::as_raw(this), ::windows::core::Array::<u8>::set_abi_len(result__.assume_init_mut()), result__.as_mut_ptr() as *mut _ as _).and_then(|| result__.assume_init())
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn LicenseFetchChallengeCustomData(&self) -> ::windows::core::Result<INDCustomData> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LicenseFetchChallengeCustomData)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn SetLicenseFetchChallengeCustomData<P0, E0>(&self, licensefetchchallengecustomdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<INDCustomData>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetLicenseFetchChallengeCustomData)(::windows::core::Vtable::as_raw(this), licensefetchchallengecustomdata.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
}
#[cfg(feature = "deprecated")]
::windows::core::interface_hierarchy!(INDLicenseFetchDescriptor, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for INDLicenseFetchDescriptor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for INDLicenseFetchDescriptor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for INDLicenseFetchDescriptor {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for INDLicenseFetchDescriptor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INDLicenseFetchDescriptor").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for INDLicenseFetchDescriptor {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{5498d33a-e686-4935-a567-7ca77ad20fa4}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Vtable for INDLicenseFetchDescriptor {
    type Vtable = INDLicenseFetchDescriptor_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for INDLicenseFetchDescriptor {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5498d33a_e686_4935_a567_7ca77ad20fa4);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct INDLicenseFetchDescriptor_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub ContentIDType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut NDContentIDType) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ContentIDType: usize,
    #[cfg(feature = "deprecated")]
    pub ContentID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ContentID: usize,
    #[cfg(feature = "deprecated")]
    pub LicenseFetchChallengeCustomData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    LicenseFetchChallengeCustomData: usize,
    #[cfg(feature = "deprecated")]
    pub SetLicenseFetchChallengeCustomData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, licensefetchchallengecustomdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetLicenseFetchChallengeCustomData: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct INDLicenseFetchDescriptorFactory(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Vtable for INDLicenseFetchDescriptorFactory {
    type Vtable = INDLicenseFetchDescriptorFactory_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for INDLicenseFetchDescriptorFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd0031202_cfac_4f00_ae6a_97af80b848f2);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct INDLicenseFetchDescriptorFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contentidtype: NDContentIDType, contentIDBytes_array_size: u32, contentidbytes: *const u8, licensefetchchallengecustomdata: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    CreateInstance: usize,
}
#[doc = "*Required features: `\"Media_Protection_PlayReady\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct INDLicenseFetchResult(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl INDLicenseFetchResult {
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn ResponseCustomData(&self) -> ::windows::core::Result<INDCustomData> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ResponseCustomData)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
#[cfg(feature = "deprecated")]
::windows::core::interface_hierarchy!(INDLicenseFetchResult, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for INDLicenseFetchResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for INDLicenseFetchResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for INDLicenseFetchResult {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for INDLicenseFetchResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INDLicenseFetchResult").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for INDLicenseFetchResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{21d39698-aa62-45ff-a5ff-8037e5433825}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Vtable for INDLicenseFetchResult {
    type Vtable = INDLicenseFetchResult_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for INDLicenseFetchResult {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x21d39698_aa62_45ff_a5ff_8037e5433825);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct INDLicenseFetchResult_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub ResponseCustomData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ResponseCustomData: usize,
}
#[doc = "*Required features: `\"Media_Protection_PlayReady\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct INDMessenger(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl INDMessenger {
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn SendRegistrationRequestAsync(&self, sessionidbytes: &[u8], challengedatabytes: &[u8]) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<INDSendResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SendRegistrationRequestAsync)(::windows::core::Vtable::as_raw(this), sessionidbytes.len() as u32, sessionidbytes.as_ptr(), challengedatabytes.len() as u32, challengedatabytes.as_ptr(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn SendProximityDetectionStartAsync(&self, pdtype: NDProximityDetectionType, transmitterchannelbytes: &[u8], sessionidbytes: &[u8], challengedatabytes: &[u8]) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<INDSendResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SendProximityDetectionStartAsync)(::windows::core::Vtable::as_raw(this), pdtype, transmitterchannelbytes.len() as u32, transmitterchannelbytes.as_ptr(), sessionidbytes.len() as u32, sessionidbytes.as_ptr(), challengedatabytes.len() as u32, challengedatabytes.as_ptr(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn SendProximityDetectionResponseAsync(&self, pdtype: NDProximityDetectionType, transmitterchannelbytes: &[u8], sessionidbytes: &[u8], responsedatabytes: &[u8]) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<INDSendResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SendProximityDetectionResponseAsync)(::windows::core::Vtable::as_raw(this), pdtype, transmitterchannelbytes.len() as u32, transmitterchannelbytes.as_ptr(), sessionidbytes.len() as u32, sessionidbytes.as_ptr(), responsedatabytes.len() as u32, responsedatabytes.as_ptr(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn SendLicenseFetchRequestAsync(&self, sessionidbytes: &[u8], challengedatabytes: &[u8]) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<INDSendResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SendLicenseFetchRequestAsync)(::windows::core::Vtable::as_raw(this), sessionidbytes.len() as u32, sessionidbytes.as_ptr(), challengedatabytes.len() as u32, challengedatabytes.as_ptr(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
#[cfg(feature = "deprecated")]
::windows::core::interface_hierarchy!(INDMessenger, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for INDMessenger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for INDMessenger {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for INDMessenger {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for INDMessenger {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INDMessenger").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for INDMessenger {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{d42df95d-a75b-47bf-8249-bc83820da38a}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Vtable for INDMessenger {
    type Vtable = INDMessenger_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for INDMessenger {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd42df95d_a75b_47bf_8249_bc83820da38a);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct INDMessenger_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub SendRegistrationRequestAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sessionIDBytes_array_size: u32, sessionidbytes: *const u8, challengeDataBytes_array_size: u32, challengedatabytes: *const u8, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    SendRegistrationRequestAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub SendProximityDetectionStartAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdtype: NDProximityDetectionType, transmitterChannelBytes_array_size: u32, transmitterchannelbytes: *const u8, sessionIDBytes_array_size: u32, sessionidbytes: *const u8, challengeDataBytes_array_size: u32, challengedatabytes: *const u8, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    SendProximityDetectionStartAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub SendProximityDetectionResponseAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdtype: NDProximityDetectionType, transmitterChannelBytes_array_size: u32, transmitterchannelbytes: *const u8, sessionIDBytes_array_size: u32, sessionidbytes: *const u8, responseDataBytes_array_size: u32, responsedatabytes: *const u8, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    SendProximityDetectionResponseAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub SendLicenseFetchRequestAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sessionIDBytes_array_size: u32, sessionidbytes: *const u8, challengeDataBytes_array_size: u32, challengedatabytes: *const u8, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    SendLicenseFetchRequestAsync: usize,
}
#[doc = "*Required features: `\"Media_Protection_PlayReady\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct INDProximityDetectionCompletedEventArgs(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl INDProximityDetectionCompletedEventArgs {
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn ProximityDetectionRetryCount(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ProximityDetectionRetryCount)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
#[cfg(feature = "deprecated")]
::windows::core::interface_hierarchy!(INDProximityDetectionCompletedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for INDProximityDetectionCompletedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for INDProximityDetectionCompletedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for INDProximityDetectionCompletedEventArgs {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for INDProximityDetectionCompletedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INDProximityDetectionCompletedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for INDProximityDetectionCompletedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{2a706328-da25-4f8c-9eb7-5d0fc3658bca}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Vtable for INDProximityDetectionCompletedEventArgs {
    type Vtable = INDProximityDetectionCompletedEventArgs_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for INDProximityDetectionCompletedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2a706328_da25_4f8c_9eb7_5d0fc3658bca);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct INDProximityDetectionCompletedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub ProximityDetectionRetryCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ProximityDetectionRetryCount: usize,
}
#[doc = "*Required features: `\"Media_Protection_PlayReady\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct INDRegistrationCompletedEventArgs(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl INDRegistrationCompletedEventArgs {
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn ResponseCustomData(&self) -> ::windows::core::Result<INDCustomData> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ResponseCustomData)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn TransmitterProperties(&self) -> ::windows::core::Result<INDTransmitterProperties> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TransmitterProperties)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn TransmitterCertificateAccepted(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TransmitterCertificateAccepted)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn SetTransmitterCertificateAccepted(&self, accept: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetTransmitterCertificateAccepted)(::windows::core::Vtable::as_raw(this), accept).ok() }
    }
}
#[cfg(feature = "deprecated")]
::windows::core::interface_hierarchy!(INDRegistrationCompletedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for INDRegistrationCompletedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for INDRegistrationCompletedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for INDRegistrationCompletedEventArgs {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for INDRegistrationCompletedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INDRegistrationCompletedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for INDRegistrationCompletedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{9e39b64d-ab5b-4905-acdc-787a77c6374d}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Vtable for INDRegistrationCompletedEventArgs {
    type Vtable = INDRegistrationCompletedEventArgs_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for INDRegistrationCompletedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9e39b64d_ab5b_4905_acdc_787a77c6374d);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct INDRegistrationCompletedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub ResponseCustomData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ResponseCustomData: usize,
    #[cfg(feature = "deprecated")]
    pub TransmitterProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    TransmitterProperties: usize,
    #[cfg(feature = "deprecated")]
    pub TransmitterCertificateAccepted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    TransmitterCertificateAccepted: usize,
    #[cfg(feature = "deprecated")]
    pub SetTransmitterCertificateAccepted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, accept: bool) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetTransmitterCertificateAccepted: usize,
}
#[doc = "*Required features: `\"Media_Protection_PlayReady\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct INDSendResult(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl INDSendResult {
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn Response(&self) -> ::windows::core::Result<::windows::core::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Response)(::windows::core::Vtable::as_raw(this), ::windows::core::Array::<u8>::set_abi_len(result__.assume_init_mut()), result__.as_mut_ptr() as *mut _ as _).and_then(|| result__.assume_init())
        }
    }
}
#[cfg(feature = "deprecated")]
::windows::core::interface_hierarchy!(INDSendResult, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for INDSendResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for INDSendResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for INDSendResult {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for INDSendResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INDSendResult").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for INDSendResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{e3685517-a584-479d-90b7-d689c7bf7c80}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Vtable for INDSendResult {
    type Vtable = INDSendResult_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for INDSendResult {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe3685517_a584_479d_90b7_d689c7bf7c80);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct INDSendResult_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub Response: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Response: usize,
}
#[doc = "*Required features: `\"Media_Protection_PlayReady\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct INDStartResult(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl INDStartResult {
    #[doc = "*Required features: `\"Media_Core\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Media_Core", feature = "deprecated"))]
    pub fn MediaStreamSource(&self) -> ::windows::core::Result<super::super::Core::MediaStreamSource> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MediaStreamSource)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
#[cfg(feature = "deprecated")]
::windows::core::interface_hierarchy!(INDStartResult, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for INDStartResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for INDStartResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for INDStartResult {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for INDStartResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INDStartResult").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for INDStartResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{79f6e96e-f50f-4015-8ba4-c2bc344ebd4e}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Vtable for INDStartResult {
    type Vtable = INDStartResult_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for INDStartResult {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x79f6e96e_f50f_4015_8ba4_c2bc344ebd4e);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct INDStartResult_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(all(feature = "Media_Core", feature = "deprecated"))]
    pub MediaStreamSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Media_Core", feature = "deprecated")))]
    MediaStreamSource: usize,
}
#[doc = "*Required features: `\"Media_Protection_PlayReady\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct INDStorageFileHelper(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl INDStorageFileHelper {
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Storage\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage", feature = "deprecated"))]
    pub fn GetFileURLs<P0, E0>(&self, file: P0) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::super::Storage::IStorageFile>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetFileURLs)(::windows::core::Vtable::as_raw(this), file.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
#[cfg(feature = "deprecated")]
::windows::core::interface_hierarchy!(INDStorageFileHelper, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for INDStorageFileHelper {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for INDStorageFileHelper {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for INDStorageFileHelper {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for INDStorageFileHelper {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INDStorageFileHelper").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for INDStorageFileHelper {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{d8f0bef8-91d2-4d47-a3f9-eaff4edb729f}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Vtable for INDStorageFileHelper {
    type Vtable = INDStorageFileHelper_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for INDStorageFileHelper {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd8f0bef8_91d2_4d47_a3f9_eaff4edb729f);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct INDStorageFileHelper_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage", feature = "deprecated"))]
    pub GetFileURLs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, file: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage", feature = "deprecated")))]
    GetFileURLs: usize,
}
#[doc = "*Required features: `\"Media_Protection_PlayReady\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct INDStreamParser(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl INDStreamParser {
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn ParseData(&self, databytes: &[u8]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).ParseData)(::windows::core::Vtable::as_raw(this), databytes.len() as u32, databytes.as_ptr()).ok() }
    }
    #[doc = "*Required features: `\"Media_Core\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Media_Core", feature = "deprecated"))]
    pub fn GetStreamInformation<P0, E0>(&self, descriptor: P0, streamtype: &mut NDMediaStreamType) -> ::windows::core::Result<u32>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Core::IMediaStreamDescriptor>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetStreamInformation)(::windows::core::Vtable::as_raw(this), descriptor.try_into().map_err(|e| e.into())?.abi(), streamtype, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn BeginOfStream(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).BeginOfStream)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn EndOfStream(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).EndOfStream)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn Notifier(&self) -> ::windows::core::Result<NDStreamParserNotifier> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Notifier)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
#[cfg(feature = "deprecated")]
::windows::core::interface_hierarchy!(INDStreamParser, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for INDStreamParser {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for INDStreamParser {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for INDStreamParser {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for INDStreamParser {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INDStreamParser").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for INDStreamParser {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{e0baa198-9796-41c9-8695-59437e67e66a}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Vtable for INDStreamParser {
    type Vtable = INDStreamParser_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for INDStreamParser {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe0baa198_9796_41c9_8695_59437e67e66a);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct INDStreamParser_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub ParseData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dataBytes_array_size: u32, databytes: *const u8) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ParseData: usize,
    #[cfg(all(feature = "Media_Core", feature = "deprecated"))]
    pub GetStreamInformation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, descriptor: *mut ::core::ffi::c_void, streamtype: *mut NDMediaStreamType, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Media_Core", feature = "deprecated")))]
    GetStreamInformation: usize,
    #[cfg(feature = "deprecated")]
    pub BeginOfStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    BeginOfStream: usize,
    #[cfg(feature = "deprecated")]
    pub EndOfStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    EndOfStream: usize,
    #[cfg(feature = "deprecated")]
    pub Notifier: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Notifier: usize,
}
#[doc = "*Required features: `\"Media_Protection_PlayReady\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct INDStreamParserNotifier(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl INDStreamParserNotifier {
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn OnContentIDReceived<P0, E0>(&self, licensefetchdescriptor: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<INDLicenseFetchDescriptor>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).OnContentIDReceived)(::windows::core::Vtable::as_raw(this), licensefetchdescriptor.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Media_Core\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Core", feature = "deprecated"))]
    pub fn OnMediaStreamDescriptorCreated<P0, E0, P1, E1>(&self, audiostreamdescriptors: P0, videostreamdescriptors: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::super::Foundation::Collections::IVector<super::super::Core::AudioStreamDescriptor>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
        P1: ::std::convert::TryInto<::windows::core::InParam<super::super::super::Foundation::Collections::IVector<super::super::Core::VideoStreamDescriptor>>, Error = E1>,
        E1: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).OnMediaStreamDescriptorCreated)(::windows::core::Vtable::as_raw(this), audiostreamdescriptors.try_into().map_err(|e| e.into())?.abi(), videostreamdescriptors.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Core\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Media_Core", feature = "deprecated"))]
    pub fn OnSampleParsed(&self, streamid: u32, streamtype: NDMediaStreamType, streamsample: &super::super::Core::MediaStreamSample, pts: i64, ccformat: NDClosedCaptionFormat, ccdatabytes: &[u8]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).OnSampleParsed)(::windows::core::Vtable::as_raw(this), streamid, streamtype, ::core::mem::transmute_copy(streamsample), pts, ccformat, ccdatabytes.len() as u32, ccdatabytes.as_ptr()).ok() }
    }
    #[doc = "*Required features: `\"Media_Core\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Media_Core", feature = "deprecated"))]
    pub fn OnBeginSetupDecryptor<P0, E0>(&self, descriptor: P0, keyid: ::windows::core::GUID, probytes: &[u8]) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Core::IMediaStreamDescriptor>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).OnBeginSetupDecryptor)(::windows::core::Vtable::as_raw(this), descriptor.try_into().map_err(|e| e.into())?.abi(), keyid, probytes.len() as u32, probytes.as_ptr()).ok() }
    }
}
#[cfg(feature = "deprecated")]
::windows::core::interface_hierarchy!(INDStreamParserNotifier, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for INDStreamParserNotifier {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for INDStreamParserNotifier {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for INDStreamParserNotifier {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for INDStreamParserNotifier {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INDStreamParserNotifier").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for INDStreamParserNotifier {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{c167acd0-2ce6-426c-ace5-5e9275fea715}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Vtable for INDStreamParserNotifier {
    type Vtable = INDStreamParserNotifier_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for INDStreamParserNotifier {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc167acd0_2ce6_426c_ace5_5e9275fea715);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct INDStreamParserNotifier_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub OnContentIDReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, licensefetchdescriptor: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    OnContentIDReceived: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Core", feature = "deprecated"))]
    pub OnMediaStreamDescriptorCreated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, audiostreamdescriptors: *mut ::core::ffi::c_void, videostreamdescriptors: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Media_Core", feature = "deprecated")))]
    OnMediaStreamDescriptorCreated: usize,
    #[cfg(all(feature = "Media_Core", feature = "deprecated"))]
    pub OnSampleParsed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, streamid: u32, streamtype: NDMediaStreamType, streamsample: *mut ::core::ffi::c_void, pts: i64, ccformat: NDClosedCaptionFormat, ccDataBytes_array_size: u32, ccdatabytes: *const u8) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Media_Core", feature = "deprecated")))]
    OnSampleParsed: usize,
    #[cfg(all(feature = "Media_Core", feature = "deprecated"))]
    pub OnBeginSetupDecryptor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, descriptor: *mut ::core::ffi::c_void, keyid: ::windows::core::GUID, proBytes_array_size: u32, probytes: *const u8) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Media_Core", feature = "deprecated")))]
    OnBeginSetupDecryptor: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct INDTCPMessengerFactory(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Vtable for INDTCPMessengerFactory {
    type Vtable = INDTCPMessengerFactory_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for INDTCPMessengerFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7dd85cfe_1b99_4f68_8f82_8177f7cedf2b);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct INDTCPMessengerFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, remotehostname: *mut ::core::ffi::c_void, remotehostport: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    CreateInstance: usize,
}
#[doc = "*Required features: `\"Media_Protection_PlayReady\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct INDTransmitterProperties(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl INDTransmitterProperties {
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn CertificateType(&self) -> ::windows::core::Result<NDCertificateType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CertificateType)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn PlatformIdentifier(&self) -> ::windows::core::Result<NDCertificatePlatformID> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PlatformIdentifier)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn SupportedFeatures(&self) -> ::windows::core::Result<::windows::core::Array<NDCertificateFeature>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SupportedFeatures)(::windows::core::Vtable::as_raw(this), ::windows::core::Array::<NDCertificateFeature>::set_abi_len(result__.assume_init_mut()), result__.as_mut_ptr() as *mut _ as _).and_then(|| result__.assume_init())
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn SecurityLevel(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SecurityLevel)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn SecurityVersion(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SecurityVersion)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn ExpirationDate(&self) -> ::windows::core::Result<super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ExpirationDate)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn ClientID(&self) -> ::windows::core::Result<::windows::core::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ClientID)(::windows::core::Vtable::as_raw(this), ::windows::core::Array::<u8>::set_abi_len(result__.assume_init_mut()), result__.as_mut_ptr() as *mut _ as _).and_then(|| result__.assume_init())
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn ModelDigest(&self) -> ::windows::core::Result<::windows::core::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ModelDigest)(::windows::core::Vtable::as_raw(this), ::windows::core::Array::<u8>::set_abi_len(result__.assume_init_mut()), result__.as_mut_ptr() as *mut _ as _).and_then(|| result__.assume_init())
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn ModelManufacturerName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ModelManufacturerName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn ModelName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ModelName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn ModelNumber(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ModelNumber)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
#[cfg(feature = "deprecated")]
::windows::core::interface_hierarchy!(INDTransmitterProperties, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for INDTransmitterProperties {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for INDTransmitterProperties {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for INDTransmitterProperties {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for INDTransmitterProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INDTransmitterProperties").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for INDTransmitterProperties {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{e536af23-ac4f-4adc-8c66-4ff7c2702dd6}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Vtable for INDTransmitterProperties {
    type Vtable = INDTransmitterProperties_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for INDTransmitterProperties {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe536af23_ac4f_4adc_8c66_4ff7c2702dd6);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct INDTransmitterProperties_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub CertificateType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut NDCertificateType) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    CertificateType: usize,
    #[cfg(feature = "deprecated")]
    pub PlatformIdentifier: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut NDCertificatePlatformID) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    PlatformIdentifier: usize,
    #[cfg(feature = "deprecated")]
    pub SupportedFeatures: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut NDCertificateFeature) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SupportedFeatures: usize,
    #[cfg(feature = "deprecated")]
    pub SecurityLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SecurityLevel: usize,
    #[cfg(feature = "deprecated")]
    pub SecurityVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SecurityVersion: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub ExpirationDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    ExpirationDate: usize,
    #[cfg(feature = "deprecated")]
    pub ClientID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ClientID: usize,
    #[cfg(feature = "deprecated")]
    pub ModelDigest: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ModelDigest: usize,
    #[cfg(feature = "deprecated")]
    pub ModelManufacturerName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ModelManufacturerName: usize,
    #[cfg(feature = "deprecated")]
    pub ModelName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ModelName: usize,
    #[cfg(feature = "deprecated")]
    pub ModelNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ModelNumber: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPlayReadyContentHeader(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPlayReadyContentHeader {
    type Vtable = IPlayReadyContentHeader_Vtbl;
}
unsafe impl ::windows::core::Interface for IPlayReadyContentHeader {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9a438a6a_7f4c_452e_88bd_0148c6387a2c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyContentHeader_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub KeyId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub KeyIdString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub LicenseAcquisitionUrl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LicenseAcquisitionUrl: usize,
    #[cfg(feature = "Foundation")]
    pub LicenseAcquisitionUserInterfaceUrl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LicenseAcquisitionUserInterfaceUrl: usize,
    pub DomainServiceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub EncryptionType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PlayReadyEncryptionAlgorithm) -> ::windows::core::HRESULT,
    pub CustomAttributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DecryptorSetup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PlayReadyDecryptorSetup) -> ::windows::core::HRESULT,
    pub GetSerializedHeader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows::core::HRESULT,
    pub HeaderWithEmbeddedUpdates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPlayReadyContentHeader2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPlayReadyContentHeader2 {
    type Vtable = IPlayReadyContentHeader2_Vtbl;
}
unsafe impl ::windows::core::Interface for IPlayReadyContentHeader2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x359c79f4_2180_498c_965b_e754d875eab2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyContentHeader2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub KeyIds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub KeyIdStrings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPlayReadyContentHeaderFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPlayReadyContentHeaderFactory {
    type Vtable = IPlayReadyContentHeaderFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IPlayReadyContentHeaderFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcb97c8ff_b758_4776_bf01_217a8b510b2c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyContentHeaderFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub CreateInstanceFromWindowsMediaDrmHeader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, headerBytes_array_size: u32, headerbytes: *const u8, licenseacquisitionurl: *mut ::core::ffi::c_void, licenseacquisitionuserinterfaceurl: *mut ::core::ffi::c_void, customattributes: *mut ::core::ffi::c_void, domainserviceid: ::windows::core::GUID, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateInstanceFromWindowsMediaDrmHeader: usize,
    #[cfg(feature = "Foundation")]
    pub CreateInstanceFromComponents: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contentkeyid: ::windows::core::GUID, contentkeyidstring: *mut ::core::ffi::c_void, contentencryptionalgorithm: PlayReadyEncryptionAlgorithm, licenseacquisitionurl: *mut ::core::ffi::c_void, licenseacquisitionuserinterfaceurl: *mut ::core::ffi::c_void, customattributes: *mut ::core::ffi::c_void, domainserviceid: ::windows::core::GUID, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateInstanceFromComponents: usize,
    pub CreateInstanceFromPlayReadyHeader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, headerBytes_array_size: u32, headerbytes: *const u8, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPlayReadyContentHeaderFactory2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPlayReadyContentHeaderFactory2 {
    type Vtable = IPlayReadyContentHeaderFactory2_Vtbl;
}
unsafe impl ::windows::core::Interface for IPlayReadyContentHeaderFactory2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd1239cf5_ae6d_4778_97fd_6e3a2eeadbeb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyContentHeaderFactory2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub CreateInstanceFromComponents2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32, contentKeyIds_array_size: u32, contentkeyids: *const ::windows::core::GUID, contentKeyIdStrings_array_size: u32, contentkeyidstrings: *const *mut ::core::ffi::c_void, contentencryptionalgorithm: PlayReadyEncryptionAlgorithm, licenseacquisitionurl: *mut ::core::ffi::c_void, licenseacquisitionuserinterfaceurl: *mut ::core::ffi::c_void, customattributes: *mut ::core::ffi::c_void, domainserviceid: ::windows::core::GUID, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateInstanceFromComponents2: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPlayReadyContentResolver(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPlayReadyContentResolver {
    type Vtable = IPlayReadyContentResolver_Vtbl;
}
unsafe impl ::windows::core::Interface for IPlayReadyContentResolver {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfbfd2523_906d_4982_a6b8_6849565a7ce8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyContentResolver_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ServiceRequest: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contentheader: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Media_Protection_PlayReady\"`*"]
#[repr(transparent)]
pub struct IPlayReadyDomain(::windows::core::IUnknown);
impl IPlayReadyDomain {
    pub fn AccountId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AccountId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ServiceId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ServiceId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Revision(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Revision)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn FriendlyName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FriendlyName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DomainJoinUrl(&self) -> ::windows::core::Result<super::super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DomainJoinUrl)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
::windows::core::interface_hierarchy!(IPlayReadyDomain, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::clone::Clone for IPlayReadyDomain {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPlayReadyDomain {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPlayReadyDomain {}
impl ::core::fmt::Debug for IPlayReadyDomain {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPlayReadyDomain").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IPlayReadyDomain {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{adcc93ac-97e6-43ef-95e4-d7868f3b16a9}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for IPlayReadyDomain {
    type Vtable = IPlayReadyDomain_Vtbl;
}
unsafe impl ::windows::core::Interface for IPlayReadyDomain {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xadcc93ac_97e6_43ef_95e4_d7868f3b16a9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyDomain_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub AccountId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub ServiceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub Revision: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub FriendlyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub DomainJoinUrl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DomainJoinUrl: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPlayReadyDomainIterableFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPlayReadyDomainIterableFactory {
    type Vtable = IPlayReadyDomainIterableFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IPlayReadyDomainIterableFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4df384ee_3121_4df3_a5e8_d0c24c0500fc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyDomainIterableFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, domainaccountid: ::windows::core::GUID, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateInstance: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPlayReadyDomainJoinServiceRequest(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPlayReadyDomainJoinServiceRequest {
    type Vtable = IPlayReadyDomainJoinServiceRequest_Vtbl;
}
unsafe impl ::windows::core::Interface for IPlayReadyDomainJoinServiceRequest {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x171b4a5a_405f_4739_b040_67b9f0c38758);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyDomainJoinServiceRequest_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub DomainAccountId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub SetDomainAccountId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub DomainFriendlyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetDomainFriendlyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DomainServiceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub SetDomainServiceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::GUID) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPlayReadyDomainLeaveServiceRequest(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPlayReadyDomainLeaveServiceRequest {
    type Vtable = IPlayReadyDomainLeaveServiceRequest_Vtbl;
}
unsafe impl ::windows::core::Interface for IPlayReadyDomainLeaveServiceRequest {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x062d58be_97ad_4917_aa03_46d4c252d464);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyDomainLeaveServiceRequest_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub DomainAccountId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub SetDomainAccountId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub DomainServiceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub SetDomainServiceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::GUID) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPlayReadyITADataGenerator(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPlayReadyITADataGenerator {
    type Vtable = IPlayReadyITADataGenerator_Vtbl;
}
unsafe impl ::windows::core::Interface for IPlayReadyITADataGenerator {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x24446b8e_10b9_4530_b25b_901a8029a9b2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyITADataGenerator_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GenerateData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guidcpsystemid: ::windows::core::GUID, countofstreams: u32, configuration: *mut ::core::ffi::c_void, format: PlayReadyITADataFormat, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GenerateData: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPlayReadyIndividualizationServiceRequest(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPlayReadyIndividualizationServiceRequest {
    type Vtable = IPlayReadyIndividualizationServiceRequest_Vtbl;
}
unsafe impl ::windows::core::Interface for IPlayReadyIndividualizationServiceRequest {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x21f5a86b_008c_4611_ab2f_aaa6c69f0e24);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyIndividualizationServiceRequest_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
}
#[doc = "*Required features: `\"Media_Protection_PlayReady\"`*"]
#[repr(transparent)]
pub struct IPlayReadyLicense(::windows::core::IUnknown);
impl IPlayReadyLicense {
    pub fn FullyEvaluated(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FullyEvaluated)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn UsableForPlay(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).UsableForPlay)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ExpirationDate(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ExpirationDate)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ExpireAfterFirstPlay(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ExpireAfterFirstPlay)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn DomainAccountID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DomainAccountID)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ChainDepth(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ChainDepth)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetKIDAtChainDepth(&self, chaindepth: u32) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetKIDAtChainDepth)(::windows::core::Vtable::as_raw(this), chaindepth, result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
::windows::core::interface_hierarchy!(IPlayReadyLicense, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::clone::Clone for IPlayReadyLicense {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPlayReadyLicense {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPlayReadyLicense {}
impl ::core::fmt::Debug for IPlayReadyLicense {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPlayReadyLicense").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IPlayReadyLicense {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{ee474c4e-fa3c-414d-a9f2-3ffc1ef832d4}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for IPlayReadyLicense {
    type Vtable = IPlayReadyLicense_Vtbl;
}
unsafe impl ::windows::core::Interface for IPlayReadyLicense {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xee474c4e_fa3c_414d_a9f2_3ffc1ef832d4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyLicense_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub FullyEvaluated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub UsableForPlay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ExpirationDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ExpirationDate: usize,
    pub ExpireAfterFirstPlay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub DomainAccountID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub ChainDepth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub GetKIDAtChainDepth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, chaindepth: u32, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPlayReadyLicense2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPlayReadyLicense2 {
    type Vtable = IPlayReadyLicense2_Vtbl;
}
unsafe impl ::windows::core::Interface for IPlayReadyLicense2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x30f4e7a7_d8e3_48a0_bcda_ff9f40530436);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyLicense2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub SecureStopId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub SecurityLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub InMemoryOnly: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub ExpiresInRealTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Media_Protection_PlayReady\"`*"]
#[repr(transparent)]
pub struct IPlayReadyLicenseAcquisitionServiceRequest(::windows::core::IUnknown);
impl IPlayReadyLicenseAcquisitionServiceRequest {
    pub fn ContentHeader(&self) -> ::windows::core::Result<PlayReadyContentHeader> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ContentHeader)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetContentHeader(&self, value: &PlayReadyContentHeader) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetContentHeader)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn DomainServiceId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DomainServiceId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetDomainServiceId(&self, value: ::windows::core::GUID) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetDomainServiceId)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn ProtectionSystem(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = &::windows::core::Interface::cast::<super::IMediaProtectionServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ProtectionSystem)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Type(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = &::windows::core::Interface::cast::<super::IMediaProtectionServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Type)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Uri(&self) -> ::windows::core::Result<super::super::super::Foundation::Uri> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Uri)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetUri(&self, value: &super::super::super::Foundation::Uri) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetUri)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn ResponseCustomData(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ResponseCustomData)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ChallengeCustomData(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ChallengeCustomData)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetChallengeCustomData(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetChallengeCustomData)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn BeginServiceRequest(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BeginServiceRequest)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn NextServiceRequest(&self) -> ::windows::core::Result<IPlayReadyServiceRequest> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NextServiceRequest)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GenerateManualEnablingChallenge(&self) -> ::windows::core::Result<PlayReadySoapMessage> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GenerateManualEnablingChallenge)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ProcessManualEnablingResponse(&self, responsebytes: &[u8]) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ProcessManualEnablingResponse)(::windows::core::Vtable::as_raw(this), responsebytes.len() as u32, responsebytes.as_ptr(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
::windows::core::interface_hierarchy!(IPlayReadyLicenseAcquisitionServiceRequest, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::convert::TryFrom<IPlayReadyLicenseAcquisitionServiceRequest> for super::IMediaProtectionServiceRequest {
    type Error = ::windows::core::Error;
    fn try_from(value: IPlayReadyLicenseAcquisitionServiceRequest) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IPlayReadyLicenseAcquisitionServiceRequest> for super::IMediaProtectionServiceRequest {
    type Error = ::windows::core::Error;
    fn try_from(value: &IPlayReadyLicenseAcquisitionServiceRequest) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&IPlayReadyLicenseAcquisitionServiceRequest> for ::windows::core::InParam<super::IMediaProtectionServiceRequest> {
    type Error = ::windows::core::Error;
    fn try_from(value: &IPlayReadyLicenseAcquisitionServiceRequest) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<IPlayReadyLicenseAcquisitionServiceRequest> for IPlayReadyServiceRequest {
    type Error = ::windows::core::Error;
    fn try_from(value: IPlayReadyLicenseAcquisitionServiceRequest) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IPlayReadyLicenseAcquisitionServiceRequest> for IPlayReadyServiceRequest {
    type Error = ::windows::core::Error;
    fn try_from(value: &IPlayReadyLicenseAcquisitionServiceRequest) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&IPlayReadyLicenseAcquisitionServiceRequest> for ::windows::core::InParam<IPlayReadyServiceRequest> {
    type Error = ::windows::core::Error;
    fn try_from(value: &IPlayReadyLicenseAcquisitionServiceRequest) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::clone::Clone for IPlayReadyLicenseAcquisitionServiceRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPlayReadyLicenseAcquisitionServiceRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPlayReadyLicenseAcquisitionServiceRequest {}
impl ::core::fmt::Debug for IPlayReadyLicenseAcquisitionServiceRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPlayReadyLicenseAcquisitionServiceRequest").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IPlayReadyLicenseAcquisitionServiceRequest {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{5d85ff45-3e9f-4f48-93e1-9530c8d58c3e}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for IPlayReadyLicenseAcquisitionServiceRequest {
    type Vtable = IPlayReadyLicenseAcquisitionServiceRequest_Vtbl;
}
unsafe impl ::windows::core::Interface for IPlayReadyLicenseAcquisitionServiceRequest {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5d85ff45_3e9f_4f48_93e1_9530c8d58c3e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyLicenseAcquisitionServiceRequest_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ContentHeader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetContentHeader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DomainServiceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub SetDomainServiceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::GUID) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPlayReadyLicenseAcquisitionServiceRequest2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPlayReadyLicenseAcquisitionServiceRequest2 {
    type Vtable = IPlayReadyLicenseAcquisitionServiceRequest2_Vtbl;
}
unsafe impl ::windows::core::Interface for IPlayReadyLicenseAcquisitionServiceRequest2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb7fa5eb5_fe0c_b225_bc60_5a9edd32ceb5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyLicenseAcquisitionServiceRequest2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub SessionId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPlayReadyLicenseAcquisitionServiceRequest3(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPlayReadyLicenseAcquisitionServiceRequest3 {
    type Vtable = IPlayReadyLicenseAcquisitionServiceRequest3_Vtbl;
}
unsafe impl ::windows::core::Interface for IPlayReadyLicenseAcquisitionServiceRequest3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x394e5f4d_7f75_430d_b2e7_7f75f34b2d75);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyLicenseAcquisitionServiceRequest3_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateLicenseIterable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contentheader: *mut ::core::ffi::c_void, fullyevaluated: bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateLicenseIterable: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPlayReadyLicenseIterableFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPlayReadyLicenseIterableFactory {
    type Vtable = IPlayReadyLicenseIterableFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IPlayReadyLicenseIterableFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd4179f08_0837_4978_8e68_be4293c8d7a6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyLicenseIterableFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contentheader: *mut ::core::ffi::c_void, fullyevaluated: bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateInstance: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPlayReadyLicenseManagement(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPlayReadyLicenseManagement {
    type Vtable = IPlayReadyLicenseManagement_Vtbl;
}
unsafe impl ::windows::core::Interface for IPlayReadyLicenseManagement {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaaeb2141_0957_4405_b892_8bf3ec5dadd9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyLicenseManagement_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub DeleteLicenses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contentheader: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DeleteLicenses: usize,
}
#[doc = "*Required features: `\"Media_Protection_PlayReady\"`*"]
#[repr(transparent)]
pub struct IPlayReadyLicenseSession(::windows::core::IUnknown);
impl IPlayReadyLicenseSession {
    pub fn CreateLAServiceRequest(&self) -> ::windows::core::Result<IPlayReadyLicenseAcquisitionServiceRequest> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateLAServiceRequest)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ConfigureMediaProtectionManager(&self, mpm: &super::MediaProtectionManager) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).ConfigureMediaProtectionManager)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(mpm)).ok() }
    }
}
::windows::core::interface_hierarchy!(IPlayReadyLicenseSession, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::clone::Clone for IPlayReadyLicenseSession {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPlayReadyLicenseSession {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPlayReadyLicenseSession {}
impl ::core::fmt::Debug for IPlayReadyLicenseSession {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPlayReadyLicenseSession").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IPlayReadyLicenseSession {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{a1723a39-87fa-4fdd-abbb-a9720e845259}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for IPlayReadyLicenseSession {
    type Vtable = IPlayReadyLicenseSession_Vtbl;
}
unsafe impl ::windows::core::Interface for IPlayReadyLicenseSession {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa1723a39_87fa_4fdd_abbb_a9720e845259);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyLicenseSession_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateLAServiceRequest: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ConfigureMediaProtectionManager: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mpm: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Media_Protection_PlayReady\"`*"]
#[repr(transparent)]
pub struct IPlayReadyLicenseSession2(::windows::core::IUnknown);
impl IPlayReadyLicenseSession2 {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateLicenseIterable(&self, contentheader: &PlayReadyContentHeader, fullyevaluated: bool) -> ::windows::core::Result<PlayReadyLicenseIterable> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateLicenseIterable)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(contentheader), fullyevaluated, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CreateLAServiceRequest(&self) -> ::windows::core::Result<IPlayReadyLicenseAcquisitionServiceRequest> {
        let this = &::windows::core::Interface::cast::<IPlayReadyLicenseSession>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateLAServiceRequest)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ConfigureMediaProtectionManager(&self, mpm: &super::MediaProtectionManager) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPlayReadyLicenseSession>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).ConfigureMediaProtectionManager)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(mpm)).ok() }
    }
}
::windows::core::interface_hierarchy!(IPlayReadyLicenseSession2, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::convert::TryFrom<IPlayReadyLicenseSession2> for IPlayReadyLicenseSession {
    type Error = ::windows::core::Error;
    fn try_from(value: IPlayReadyLicenseSession2) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IPlayReadyLicenseSession2> for IPlayReadyLicenseSession {
    type Error = ::windows::core::Error;
    fn try_from(value: &IPlayReadyLicenseSession2) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&IPlayReadyLicenseSession2> for ::windows::core::InParam<IPlayReadyLicenseSession> {
    type Error = ::windows::core::Error;
    fn try_from(value: &IPlayReadyLicenseSession2) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::clone::Clone for IPlayReadyLicenseSession2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPlayReadyLicenseSession2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPlayReadyLicenseSession2 {}
impl ::core::fmt::Debug for IPlayReadyLicenseSession2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPlayReadyLicenseSession2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IPlayReadyLicenseSession2 {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{4909be3a-3aed-4656-8ad7-ee0fd7799510}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for IPlayReadyLicenseSession2 {
    type Vtable = IPlayReadyLicenseSession2_Vtbl;
}
unsafe impl ::windows::core::Interface for IPlayReadyLicenseSession2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4909be3a_3aed_4656_8ad7_ee0fd7799510);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyLicenseSession2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateLicenseIterable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contentheader: *mut ::core::ffi::c_void, fullyevaluated: bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateLicenseIterable: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPlayReadyLicenseSessionFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPlayReadyLicenseSessionFactory {
    type Vtable = IPlayReadyLicenseSessionFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IPlayReadyLicenseSessionFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x62492699_6527_429e_98be_48d798ac2739);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyLicenseSessionFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, configuration: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateInstance: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPlayReadyMeteringReportServiceRequest(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPlayReadyMeteringReportServiceRequest {
    type Vtable = IPlayReadyMeteringReportServiceRequest_Vtbl;
}
unsafe impl ::windows::core::Interface for IPlayReadyMeteringReportServiceRequest {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc12b231c_0ecd_4f11_a185_1e24a4a67fb7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyMeteringReportServiceRequest_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub MeteringCertificate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows::core::HRESULT,
    pub SetMeteringCertificate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, meteringCertBytes_array_size: u32, meteringcertbytes: *const u8) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPlayReadyRevocationServiceRequest(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPlayReadyRevocationServiceRequest {
    type Vtable = IPlayReadyRevocationServiceRequest_Vtbl;
}
unsafe impl ::windows::core::Interface for IPlayReadyRevocationServiceRequest {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x543d66ac_faf0_4560_84a5_0e4acec939e4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyRevocationServiceRequest_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPlayReadySecureStopIterableFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPlayReadySecureStopIterableFactory {
    type Vtable = IPlayReadySecureStopIterableFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IPlayReadySecureStopIterableFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5f1f0165_4214_4d9e_81eb_e89f9d294aee);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadySecureStopIterableFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, publisherCertBytes_array_size: u32, publishercertbytes: *const u8, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateInstance: usize,
}
#[doc = "*Required features: `\"Media_Protection_PlayReady\"`*"]
#[repr(transparent)]
pub struct IPlayReadySecureStopServiceRequest(::windows::core::IUnknown);
impl IPlayReadySecureStopServiceRequest {
    pub fn SessionID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SessionID)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StartTime(&self) -> ::windows::core::Result<super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).StartTime)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn UpdateTime(&self) -> ::windows::core::Result<super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).UpdateTime)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Stopped(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Stopped)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn PublisherCertificate(&self) -> ::windows::core::Result<::windows::core::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PublisherCertificate)(::windows::core::Vtable::as_raw(this), ::windows::core::Array::<u8>::set_abi_len(result__.assume_init_mut()), result__.as_mut_ptr() as *mut _ as _).and_then(|| result__.assume_init())
        }
    }
    pub fn ProtectionSystem(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = &::windows::core::Interface::cast::<super::IMediaProtectionServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ProtectionSystem)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Type(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = &::windows::core::Interface::cast::<super::IMediaProtectionServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Type)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Uri(&self) -> ::windows::core::Result<super::super::super::Foundation::Uri> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Uri)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetUri(&self, value: &super::super::super::Foundation::Uri) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetUri)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn ResponseCustomData(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ResponseCustomData)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ChallengeCustomData(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ChallengeCustomData)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetChallengeCustomData(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetChallengeCustomData)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn BeginServiceRequest(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BeginServiceRequest)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn NextServiceRequest(&self) -> ::windows::core::Result<IPlayReadyServiceRequest> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NextServiceRequest)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GenerateManualEnablingChallenge(&self) -> ::windows::core::Result<PlayReadySoapMessage> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GenerateManualEnablingChallenge)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ProcessManualEnablingResponse(&self, responsebytes: &[u8]) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ProcessManualEnablingResponse)(::windows::core::Vtable::as_raw(this), responsebytes.len() as u32, responsebytes.as_ptr(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
::windows::core::interface_hierarchy!(IPlayReadySecureStopServiceRequest, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::convert::TryFrom<IPlayReadySecureStopServiceRequest> for super::IMediaProtectionServiceRequest {
    type Error = ::windows::core::Error;
    fn try_from(value: IPlayReadySecureStopServiceRequest) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IPlayReadySecureStopServiceRequest> for super::IMediaProtectionServiceRequest {
    type Error = ::windows::core::Error;
    fn try_from(value: &IPlayReadySecureStopServiceRequest) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&IPlayReadySecureStopServiceRequest> for ::windows::core::InParam<super::IMediaProtectionServiceRequest> {
    type Error = ::windows::core::Error;
    fn try_from(value: &IPlayReadySecureStopServiceRequest) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<IPlayReadySecureStopServiceRequest> for IPlayReadyServiceRequest {
    type Error = ::windows::core::Error;
    fn try_from(value: IPlayReadySecureStopServiceRequest) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IPlayReadySecureStopServiceRequest> for IPlayReadyServiceRequest {
    type Error = ::windows::core::Error;
    fn try_from(value: &IPlayReadySecureStopServiceRequest) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&IPlayReadySecureStopServiceRequest> for ::windows::core::InParam<IPlayReadyServiceRequest> {
    type Error = ::windows::core::Error;
    fn try_from(value: &IPlayReadySecureStopServiceRequest) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::clone::Clone for IPlayReadySecureStopServiceRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPlayReadySecureStopServiceRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPlayReadySecureStopServiceRequest {}
impl ::core::fmt::Debug for IPlayReadySecureStopServiceRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPlayReadySecureStopServiceRequest").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IPlayReadySecureStopServiceRequest {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{b5501ee5-01bf-4401-9677-05630a6a4cc8}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for IPlayReadySecureStopServiceRequest {
    type Vtable = IPlayReadySecureStopServiceRequest_Vtbl;
}
unsafe impl ::windows::core::Interface for IPlayReadySecureStopServiceRequest {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb5501ee5_01bf_4401_9677_05630a6a4cc8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadySecureStopServiceRequest_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub SessionID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub StartTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartTime: usize,
    #[cfg(feature = "Foundation")]
    pub UpdateTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UpdateTime: usize,
    pub Stopped: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub PublisherCertificate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPlayReadySecureStopServiceRequestFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPlayReadySecureStopServiceRequestFactory {
    type Vtable = IPlayReadySecureStopServiceRequestFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IPlayReadySecureStopServiceRequestFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0e448ac9_e67e_494e_9f49_6285438c76cf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadySecureStopServiceRequestFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, publisherCertBytes_array_size: u32, publishercertbytes: *const u8, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateInstanceFromSessionID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sessionid: ::windows::core::GUID, publisherCertBytes_array_size: u32, publishercertbytes: *const u8, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Media_Protection_PlayReady\"`*"]
#[repr(transparent)]
pub struct IPlayReadyServiceRequest(::windows::core::IUnknown);
impl IPlayReadyServiceRequest {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Uri(&self) -> ::windows::core::Result<super::super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Uri)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetUri(&self, value: &super::super::super::Foundation::Uri) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetUri)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn ResponseCustomData(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ResponseCustomData)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ChallengeCustomData(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ChallengeCustomData)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetChallengeCustomData(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetChallengeCustomData)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn BeginServiceRequest(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BeginServiceRequest)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn NextServiceRequest(&self) -> ::windows::core::Result<IPlayReadyServiceRequest> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NextServiceRequest)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GenerateManualEnablingChallenge(&self) -> ::windows::core::Result<PlayReadySoapMessage> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GenerateManualEnablingChallenge)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ProcessManualEnablingResponse(&self, responsebytes: &[u8]) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ProcessManualEnablingResponse)(::windows::core::Vtable::as_raw(this), responsebytes.len() as u32, responsebytes.as_ptr(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ProtectionSystem(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = &::windows::core::Interface::cast::<super::IMediaProtectionServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ProtectionSystem)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Type(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = &::windows::core::Interface::cast::<super::IMediaProtectionServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Type)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
::windows::core::interface_hierarchy!(IPlayReadyServiceRequest, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::convert::TryFrom<IPlayReadyServiceRequest> for super::IMediaProtectionServiceRequest {
    type Error = ::windows::core::Error;
    fn try_from(value: IPlayReadyServiceRequest) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IPlayReadyServiceRequest> for super::IMediaProtectionServiceRequest {
    type Error = ::windows::core::Error;
    fn try_from(value: &IPlayReadyServiceRequest) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&IPlayReadyServiceRequest> for ::windows::core::InParam<super::IMediaProtectionServiceRequest> {
    type Error = ::windows::core::Error;
    fn try_from(value: &IPlayReadyServiceRequest) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::clone::Clone for IPlayReadyServiceRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPlayReadyServiceRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPlayReadyServiceRequest {}
impl ::core::fmt::Debug for IPlayReadyServiceRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPlayReadyServiceRequest").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IPlayReadyServiceRequest {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{8bad2836-a703-45a6-a180-76f3565aa725}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for IPlayReadyServiceRequest {
    type Vtable = IPlayReadyServiceRequest_Vtbl;
}
unsafe impl ::windows::core::Interface for IPlayReadyServiceRequest {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8bad2836_a703_45a6_a180_76f3565aa725);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyServiceRequest_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub Uri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Uri: usize,
    #[cfg(feature = "Foundation")]
    pub SetUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetUri: usize,
    pub ResponseCustomData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ChallengeCustomData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetChallengeCustomData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub BeginServiceRequest: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BeginServiceRequest: usize,
    pub NextServiceRequest: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GenerateManualEnablingChallenge: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ProcessManualEnablingResponse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, responseBytes_array_size: u32, responsebytes: *const u8, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPlayReadySoapMessage(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPlayReadySoapMessage {
    type Vtable = IPlayReadySoapMessage_Vtbl;
}
unsafe impl ::windows::core::Interface for IPlayReadySoapMessage {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb659fcb5_ce41_41ba_8a0d_61df5fffa139);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadySoapMessage_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetMessageBody: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub MessageHeaders: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    MessageHeaders: usize,
    #[cfg(feature = "Foundation")]
    pub Uri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Uri: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPlayReadyStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPlayReadyStatics {
    type Vtable = IPlayReadyStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IPlayReadyStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5e69c00d_247c_469a_8f31_5c1a1571d9c6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub DomainJoinServiceRequestType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub DomainLeaveServiceRequestType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub IndividualizationServiceRequestType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub LicenseAcquirerServiceRequestType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub MeteringReportServiceRequestType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub RevocationServiceRequestType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub MediaProtectionSystemId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub PlayReadySecurityVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPlayReadyStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPlayReadyStatics2 {
    type Vtable = IPlayReadyStatics2_Vtbl;
}
unsafe impl ::windows::core::Interface for IPlayReadyStatics2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1f8d6a92_5f9a_423e_9466_b33969af7a3d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyStatics2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub PlayReadyCertificateSecurityLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPlayReadyStatics3(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPlayReadyStatics3 {
    type Vtable = IPlayReadyStatics3_Vtbl;
}
unsafe impl ::windows::core::Interface for IPlayReadyStatics3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3fa33f71_2dd3_4bed_ae49_f7148e63e710);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyStatics3_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub SecureStopServiceRequestType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub CheckSupportedHardware: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwdrmfeature: PlayReadyHardwareDRMFeatures, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPlayReadyStatics4(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPlayReadyStatics4 {
    type Vtable = IPlayReadyStatics4_Vtbl;
}
unsafe impl ::windows::core::Interface for IPlayReadyStatics4 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x50a91300_d824_4231_9d5e_78ef8844c7d7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyStatics4_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub InputTrustAuthorityToCreate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ProtectionSystemId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPlayReadyStatics5(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPlayReadyStatics5 {
    type Vtable = IPlayReadyStatics5_Vtbl;
}
unsafe impl ::windows::core::Interface for IPlayReadyStatics5 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x230a7075_dfa0_4f8e_a779_cefea9c6824b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyStatics5_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub HardwareDRMDisabledAtTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    HardwareDRMDisabledAtTime: usize,
    #[cfg(feature = "Foundation")]
    pub HardwareDRMDisabledUntilTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    HardwareDRMDisabledUntilTime: usize,
    pub ResetHardwareDRMDisabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Media_Protection_PlayReady\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct NDClient(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl NDClient {
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RegistrationCompleted(&self, handler: &super::super::super::Foundation::TypedEventHandler<NDClient, INDRegistrationCompletedEventArgs>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RegistrationCompleted)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RemoveRegistrationCompleted(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveRegistrationCompleted)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn ProximityDetectionCompleted(&self, handler: &super::super::super::Foundation::TypedEventHandler<NDClient, INDProximityDetectionCompletedEventArgs>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ProximityDetectionCompleted)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RemoveProximityDetectionCompleted(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveProximityDetectionCompleted)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn LicenseFetchCompleted(&self, handler: &super::super::super::Foundation::TypedEventHandler<NDClient, INDLicenseFetchCompletedEventArgs>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LicenseFetchCompleted)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RemoveLicenseFetchCompleted(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveLicenseFetchCompleted)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn ReRegistrationNeeded(&self, handler: &super::super::super::Foundation::TypedEventHandler<NDClient, ::windows::core::IInspectable>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReRegistrationNeeded)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RemoveReRegistrationNeeded(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveReRegistrationNeeded)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn ClosedCaptionDataReceived(&self, handler: &super::super::super::Foundation::TypedEventHandler<NDClient, INDClosedCaptionDataReceivedEventArgs>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ClosedCaptionDataReceived)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RemoveClosedCaptionDataReceived(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveClosedCaptionDataReceived)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn StartAsync<P0, E0, P1, E1>(&self, contenturl: &super::super::super::Foundation::Uri, startasyncoptions: u32, registrationcustomdata: P0, licensefetchdescriptor: P1) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<INDStartResult>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<INDCustomData>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
        P1: ::std::convert::TryInto<::windows::core::InParam<INDLicenseFetchDescriptor>, Error = E1>,
        E1: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).StartAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(contenturl), startasyncoptions, registrationcustomdata.try_into().map_err(|e| e.into())?.abi(), licensefetchdescriptor.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn LicenseFetchAsync<P0, E0>(&self, licensefetchdescriptor: P0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<INDLicenseFetchResult>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<INDLicenseFetchDescriptor>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LicenseFetchAsync)(::windows::core::Vtable::as_raw(this), licensefetchdescriptor.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn ReRegistrationAsync<P0, E0>(&self, registrationcustomdata: P0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<INDCustomData>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReRegistrationAsync)(::windows::core::Vtable::as_raw(this), registrationcustomdata.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Close)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn CreateInstance<P0, E0, P1, E1, P2, E2>(downloadengine: P0, streamparser: P1, pmessenger: P2) -> ::windows::core::Result<NDClient>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<INDDownloadEngine>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
        P1: ::std::convert::TryInto<::windows::core::InParam<INDStreamParser>, Error = E1>,
        E1: ::std::convert::Into<::windows::core::Error>,
        P2: ::std::convert::TryInto<::windows::core::InParam<INDMessenger>, Error = E2>,
        E2: ::std::convert::Into<::windows::core::Error>,
    {
        Self::INDClientFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateInstance)(::windows::core::Vtable::as_raw(this), downloadengine.try_into().map_err(|e| e.into())?.abi(), streamparser.try_into().map_err(|e| e.into())?.abi(), pmessenger.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    #[cfg(feature = "deprecated")]
    pub fn INDClientFactory<R, F: FnOnce(&INDClientFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<NDClient, INDClientFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for NDClient {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for NDClient {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for NDClient {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for NDClient {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NDClient").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for NDClient {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.PlayReady.NDClient;{3bd6781b-61b8-46e2-99a5-8abcb6b9f7d6})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Vtable for NDClient {
    type Vtable = INDClient_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for NDClient {
    const IID: ::windows::core::GUID = <INDClient as ::windows::core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for NDClient {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.NDClient";
}
#[cfg(feature = "deprecated")]
::windows::core::interface_hierarchy!(NDClient, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[doc = "*Required features: `\"Media_Protection_PlayReady\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct NDCustomData(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl NDCustomData {
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn CustomDataTypeID(&self) -> ::windows::core::Result<::windows::core::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CustomDataTypeID)(::windows::core::Vtable::as_raw(this), ::windows::core::Array::<u8>::set_abi_len(result__.assume_init_mut()), result__.as_mut_ptr() as *mut _ as _).and_then(|| result__.assume_init())
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn CustomData(&self) -> ::windows::core::Result<::windows::core::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CustomData)(::windows::core::Vtable::as_raw(this), ::windows::core::Array::<u8>::set_abi_len(result__.assume_init_mut()), result__.as_mut_ptr() as *mut _ as _).and_then(|| result__.assume_init())
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn CreateInstance(customdatatypeidbytes: &[u8], customdatabytes: &[u8]) -> ::windows::core::Result<NDCustomData> {
        Self::INDCustomDataFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateInstance)(::windows::core::Vtable::as_raw(this), customdatatypeidbytes.len() as u32, customdatatypeidbytes.as_ptr(), customdatabytes.len() as u32, customdatabytes.as_ptr(), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    #[cfg(feature = "deprecated")]
    pub fn INDCustomDataFactory<R, F: FnOnce(&INDCustomDataFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<NDCustomData, INDCustomDataFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for NDCustomData {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for NDCustomData {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for NDCustomData {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for NDCustomData {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NDCustomData").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for NDCustomData {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.PlayReady.NDCustomData;{f5cb0fdc-2d09-4f19-b5e1-76a0b3ee9267})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Vtable for NDCustomData {
    type Vtable = INDCustomData_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for NDCustomData {
    const IID: ::windows::core::GUID = <INDCustomData as ::windows::core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for NDCustomData {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.NDCustomData";
}
#[cfg(feature = "deprecated")]
::windows::core::interface_hierarchy!(NDCustomData, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "deprecated")]
impl ::core::convert::TryFrom<NDCustomData> for INDCustomData {
    type Error = ::windows::core::Error;
    fn try_from(value: NDCustomData) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::TryFrom<&NDCustomData> for INDCustomData {
    type Error = ::windows::core::Error;
    fn try_from(value: &NDCustomData) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::TryFrom<&NDCustomData> for ::windows::core::InParam<INDCustomData> {
    type Error = ::windows::core::Error;
    fn try_from(value: &NDCustomData) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[doc = "*Required features: `\"Media_Protection_PlayReady\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct NDDownloadEngineNotifier(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl NDDownloadEngineNotifier {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<NDDownloadEngineNotifier, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn OnStreamOpened(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).OnStreamOpened)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn OnPlayReadyObjectReceived(&self, databytes: &[u8]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).OnPlayReadyObjectReceived)(::windows::core::Vtable::as_raw(this), databytes.len() as u32, databytes.as_ptr()).ok() }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn OnContentIDReceived<P0, E0>(&self, licensefetchdescriptor: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<INDLicenseFetchDescriptor>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).OnContentIDReceived)(::windows::core::Vtable::as_raw(this), licensefetchdescriptor.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn OnDataReceived(&self, databytes: &[u8], bytesreceived: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).OnDataReceived)(::windows::core::Vtable::as_raw(this), databytes.len() as u32, databytes.as_ptr(), bytesreceived).ok() }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn OnEndOfStream(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).OnEndOfStream)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn OnNetworkError(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).OnNetworkError)(::windows::core::Vtable::as_raw(this)).ok() }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for NDDownloadEngineNotifier {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for NDDownloadEngineNotifier {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for NDDownloadEngineNotifier {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for NDDownloadEngineNotifier {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NDDownloadEngineNotifier").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for NDDownloadEngineNotifier {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.PlayReady.NDDownloadEngineNotifier;{d720b4d4-f4b8-4530-a809-9193a571e7fc})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Vtable for NDDownloadEngineNotifier {
    type Vtable = INDDownloadEngineNotifier_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for NDDownloadEngineNotifier {
    const IID: ::windows::core::GUID = <INDDownloadEngineNotifier as ::windows::core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for NDDownloadEngineNotifier {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.NDDownloadEngineNotifier";
}
#[cfg(feature = "deprecated")]
::windows::core::interface_hierarchy!(NDDownloadEngineNotifier, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "deprecated")]
impl ::core::convert::TryFrom<NDDownloadEngineNotifier> for INDDownloadEngineNotifier {
    type Error = ::windows::core::Error;
    fn try_from(value: NDDownloadEngineNotifier) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::TryFrom<&NDDownloadEngineNotifier> for INDDownloadEngineNotifier {
    type Error = ::windows::core::Error;
    fn try_from(value: &NDDownloadEngineNotifier) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::TryFrom<&NDDownloadEngineNotifier> for ::windows::core::InParam<INDDownloadEngineNotifier> {
    type Error = ::windows::core::Error;
    fn try_from(value: &NDDownloadEngineNotifier) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[doc = "*Required features: `\"Media_Protection_PlayReady\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct NDLicenseFetchDescriptor(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl NDLicenseFetchDescriptor {
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn ContentIDType(&self) -> ::windows::core::Result<NDContentIDType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ContentIDType)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn ContentID(&self) -> ::windows::core::Result<::windows::core::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ContentID)(::windows::core::Vtable::as_raw(this), ::windows::core::Array::<u8>::set_abi_len(result__.assume_init_mut()), result__.as_mut_ptr() as *mut _ as _).and_then(|| result__.assume_init())
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn LicenseFetchChallengeCustomData(&self) -> ::windows::core::Result<INDCustomData> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LicenseFetchChallengeCustomData)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn SetLicenseFetchChallengeCustomData<P0, E0>(&self, licensefetchchallengecustomdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<INDCustomData>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetLicenseFetchChallengeCustomData)(::windows::core::Vtable::as_raw(this), licensefetchchallengecustomdata.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn CreateInstance<P0, E0>(contentidtype: NDContentIDType, contentidbytes: &[u8], licensefetchchallengecustomdata: P0) -> ::windows::core::Result<NDLicenseFetchDescriptor>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<INDCustomData>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::INDLicenseFetchDescriptorFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateInstance)(::windows::core::Vtable::as_raw(this), contentidtype, contentidbytes.len() as u32, contentidbytes.as_ptr(), licensefetchchallengecustomdata.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    #[cfg(feature = "deprecated")]
    pub fn INDLicenseFetchDescriptorFactory<R, F: FnOnce(&INDLicenseFetchDescriptorFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<NDLicenseFetchDescriptor, INDLicenseFetchDescriptorFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for NDLicenseFetchDescriptor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for NDLicenseFetchDescriptor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for NDLicenseFetchDescriptor {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for NDLicenseFetchDescriptor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NDLicenseFetchDescriptor").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for NDLicenseFetchDescriptor {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.PlayReady.NDLicenseFetchDescriptor;{5498d33a-e686-4935-a567-7ca77ad20fa4})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Vtable for NDLicenseFetchDescriptor {
    type Vtable = INDLicenseFetchDescriptor_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for NDLicenseFetchDescriptor {
    const IID: ::windows::core::GUID = <INDLicenseFetchDescriptor as ::windows::core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for NDLicenseFetchDescriptor {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.NDLicenseFetchDescriptor";
}
#[cfg(feature = "deprecated")]
::windows::core::interface_hierarchy!(NDLicenseFetchDescriptor, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "deprecated")]
impl ::core::convert::TryFrom<NDLicenseFetchDescriptor> for INDLicenseFetchDescriptor {
    type Error = ::windows::core::Error;
    fn try_from(value: NDLicenseFetchDescriptor) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::TryFrom<&NDLicenseFetchDescriptor> for INDLicenseFetchDescriptor {
    type Error = ::windows::core::Error;
    fn try_from(value: &NDLicenseFetchDescriptor) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::TryFrom<&NDLicenseFetchDescriptor> for ::windows::core::InParam<INDLicenseFetchDescriptor> {
    type Error = ::windows::core::Error;
    fn try_from(value: &NDLicenseFetchDescriptor) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[doc = "*Required features: `\"Media_Protection_PlayReady\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct NDStorageFileHelper(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl NDStorageFileHelper {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<NDStorageFileHelper, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Storage\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage", feature = "deprecated"))]
    pub fn GetFileURLs<P0, E0>(&self, file: P0) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::super::Storage::IStorageFile>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetFileURLs)(::windows::core::Vtable::as_raw(this), file.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for NDStorageFileHelper {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for NDStorageFileHelper {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for NDStorageFileHelper {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for NDStorageFileHelper {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NDStorageFileHelper").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for NDStorageFileHelper {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.PlayReady.NDStorageFileHelper;{d8f0bef8-91d2-4d47-a3f9-eaff4edb729f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Vtable for NDStorageFileHelper {
    type Vtable = INDStorageFileHelper_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for NDStorageFileHelper {
    const IID: ::windows::core::GUID = <INDStorageFileHelper as ::windows::core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for NDStorageFileHelper {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.NDStorageFileHelper";
}
#[cfg(feature = "deprecated")]
::windows::core::interface_hierarchy!(NDStorageFileHelper, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "deprecated")]
impl ::core::convert::TryFrom<NDStorageFileHelper> for INDStorageFileHelper {
    type Error = ::windows::core::Error;
    fn try_from(value: NDStorageFileHelper) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::TryFrom<&NDStorageFileHelper> for INDStorageFileHelper {
    type Error = ::windows::core::Error;
    fn try_from(value: &NDStorageFileHelper) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::TryFrom<&NDStorageFileHelper> for ::windows::core::InParam<INDStorageFileHelper> {
    type Error = ::windows::core::Error;
    fn try_from(value: &NDStorageFileHelper) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[doc = "*Required features: `\"Media_Protection_PlayReady\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct NDStreamParserNotifier(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl NDStreamParserNotifier {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<NDStreamParserNotifier, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn OnContentIDReceived<P0, E0>(&self, licensefetchdescriptor: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<INDLicenseFetchDescriptor>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).OnContentIDReceived)(::windows::core::Vtable::as_raw(this), licensefetchdescriptor.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Media_Core\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Core", feature = "deprecated"))]
    pub fn OnMediaStreamDescriptorCreated<P0, E0, P1, E1>(&self, audiostreamdescriptors: P0, videostreamdescriptors: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::super::Foundation::Collections::IVector<super::super::Core::AudioStreamDescriptor>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
        P1: ::std::convert::TryInto<::windows::core::InParam<super::super::super::Foundation::Collections::IVector<super::super::Core::VideoStreamDescriptor>>, Error = E1>,
        E1: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).OnMediaStreamDescriptorCreated)(::windows::core::Vtable::as_raw(this), audiostreamdescriptors.try_into().map_err(|e| e.into())?.abi(), videostreamdescriptors.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Core\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Media_Core", feature = "deprecated"))]
    pub fn OnSampleParsed(&self, streamid: u32, streamtype: NDMediaStreamType, streamsample: &super::super::Core::MediaStreamSample, pts: i64, ccformat: NDClosedCaptionFormat, ccdatabytes: &[u8]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).OnSampleParsed)(::windows::core::Vtable::as_raw(this), streamid, streamtype, ::core::mem::transmute_copy(streamsample), pts, ccformat, ccdatabytes.len() as u32, ccdatabytes.as_ptr()).ok() }
    }
    #[doc = "*Required features: `\"Media_Core\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Media_Core", feature = "deprecated"))]
    pub fn OnBeginSetupDecryptor<P0, E0>(&self, descriptor: P0, keyid: ::windows::core::GUID, probytes: &[u8]) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Core::IMediaStreamDescriptor>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).OnBeginSetupDecryptor)(::windows::core::Vtable::as_raw(this), descriptor.try_into().map_err(|e| e.into())?.abi(), keyid, probytes.len() as u32, probytes.as_ptr()).ok() }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for NDStreamParserNotifier {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for NDStreamParserNotifier {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for NDStreamParserNotifier {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for NDStreamParserNotifier {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NDStreamParserNotifier").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for NDStreamParserNotifier {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.PlayReady.NDStreamParserNotifier;{c167acd0-2ce6-426c-ace5-5e9275fea715})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Vtable for NDStreamParserNotifier {
    type Vtable = INDStreamParserNotifier_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for NDStreamParserNotifier {
    const IID: ::windows::core::GUID = <INDStreamParserNotifier as ::windows::core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for NDStreamParserNotifier {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.NDStreamParserNotifier";
}
#[cfg(feature = "deprecated")]
::windows::core::interface_hierarchy!(NDStreamParserNotifier, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "deprecated")]
impl ::core::convert::TryFrom<NDStreamParserNotifier> for INDStreamParserNotifier {
    type Error = ::windows::core::Error;
    fn try_from(value: NDStreamParserNotifier) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::TryFrom<&NDStreamParserNotifier> for INDStreamParserNotifier {
    type Error = ::windows::core::Error;
    fn try_from(value: &NDStreamParserNotifier) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::TryFrom<&NDStreamParserNotifier> for ::windows::core::InParam<INDStreamParserNotifier> {
    type Error = ::windows::core::Error;
    fn try_from(value: &NDStreamParserNotifier) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[doc = "*Required features: `\"Media_Protection_PlayReady\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct NDTCPMessenger(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl NDTCPMessenger {
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn SendRegistrationRequestAsync(&self, sessionidbytes: &[u8], challengedatabytes: &[u8]) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<INDSendResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SendRegistrationRequestAsync)(::windows::core::Vtable::as_raw(this), sessionidbytes.len() as u32, sessionidbytes.as_ptr(), challengedatabytes.len() as u32, challengedatabytes.as_ptr(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn SendProximityDetectionStartAsync(&self, pdtype: NDProximityDetectionType, transmitterchannelbytes: &[u8], sessionidbytes: &[u8], challengedatabytes: &[u8]) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<INDSendResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SendProximityDetectionStartAsync)(::windows::core::Vtable::as_raw(this), pdtype, transmitterchannelbytes.len() as u32, transmitterchannelbytes.as_ptr(), sessionidbytes.len() as u32, sessionidbytes.as_ptr(), challengedatabytes.len() as u32, challengedatabytes.as_ptr(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn SendProximityDetectionResponseAsync(&self, pdtype: NDProximityDetectionType, transmitterchannelbytes: &[u8], sessionidbytes: &[u8], responsedatabytes: &[u8]) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<INDSendResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SendProximityDetectionResponseAsync)(::windows::core::Vtable::as_raw(this), pdtype, transmitterchannelbytes.len() as u32, transmitterchannelbytes.as_ptr(), sessionidbytes.len() as u32, sessionidbytes.as_ptr(), responsedatabytes.len() as u32, responsedatabytes.as_ptr(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn SendLicenseFetchRequestAsync(&self, sessionidbytes: &[u8], challengedatabytes: &[u8]) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<INDSendResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SendLicenseFetchRequestAsync)(::windows::core::Vtable::as_raw(this), sessionidbytes.len() as u32, sessionidbytes.as_ptr(), challengedatabytes.len() as u32, challengedatabytes.as_ptr(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn CreateInstance(remotehostname: &::windows::core::HSTRING, remotehostport: u32) -> ::windows::core::Result<NDTCPMessenger> {
        Self::INDTCPMessengerFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateInstance)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(remotehostname), remotehostport, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    #[cfg(feature = "deprecated")]
    pub fn INDTCPMessengerFactory<R, F: FnOnce(&INDTCPMessengerFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<NDTCPMessenger, INDTCPMessengerFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for NDTCPMessenger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for NDTCPMessenger {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for NDTCPMessenger {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for NDTCPMessenger {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NDTCPMessenger").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for NDTCPMessenger {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.PlayReady.NDTCPMessenger;{d42df95d-a75b-47bf-8249-bc83820da38a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Vtable for NDTCPMessenger {
    type Vtable = INDMessenger_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for NDTCPMessenger {
    const IID: ::windows::core::GUID = <INDMessenger as ::windows::core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for NDTCPMessenger {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.NDTCPMessenger";
}
#[cfg(feature = "deprecated")]
::windows::core::interface_hierarchy!(NDTCPMessenger, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "deprecated")]
impl ::core::convert::TryFrom<NDTCPMessenger> for INDMessenger {
    type Error = ::windows::core::Error;
    fn try_from(value: NDTCPMessenger) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::TryFrom<&NDTCPMessenger> for INDMessenger {
    type Error = ::windows::core::Error;
    fn try_from(value: &NDTCPMessenger) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::TryFrom<&NDTCPMessenger> for ::windows::core::InParam<INDMessenger> {
    type Error = ::windows::core::Error;
    fn try_from(value: &NDTCPMessenger) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[doc = "*Required features: `\"Media_Protection_PlayReady\"`*"]
#[repr(transparent)]
pub struct PlayReadyContentHeader(::windows::core::IUnknown);
impl PlayReadyContentHeader {
    pub fn KeyId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).KeyId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn KeyIdString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).KeyIdString)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LicenseAcquisitionUrl(&self) -> ::windows::core::Result<super::super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LicenseAcquisitionUrl)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LicenseAcquisitionUserInterfaceUrl(&self) -> ::windows::core::Result<super::super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LicenseAcquisitionUserInterfaceUrl)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn DomainServiceId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DomainServiceId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn EncryptionType(&self) -> ::windows::core::Result<PlayReadyEncryptionAlgorithm> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).EncryptionType)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CustomAttributes(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CustomAttributes)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn DecryptorSetup(&self) -> ::windows::core::Result<PlayReadyDecryptorSetup> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DecryptorSetup)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetSerializedHeader(&self) -> ::windows::core::Result<::windows::core::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetSerializedHeader)(::windows::core::Vtable::as_raw(this), ::windows::core::Array::<u8>::set_abi_len(result__.assume_init_mut()), result__.as_mut_ptr() as *mut _ as _).and_then(|| result__.assume_init())
        }
    }
    pub fn HeaderWithEmbeddedUpdates(&self) -> ::windows::core::Result<PlayReadyContentHeader> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).HeaderWithEmbeddedUpdates)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn KeyIds(&self) -> ::windows::core::Result<::windows::core::Array<::windows::core::GUID>> {
        let this = &::windows::core::Interface::cast::<IPlayReadyContentHeader2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).KeyIds)(::windows::core::Vtable::as_raw(this), ::windows::core::Array::<::windows::core::GUID>::set_abi_len(result__.assume_init_mut()), result__.as_mut_ptr() as *mut _ as _).and_then(|| result__.assume_init())
        }
    }
    pub fn KeyIdStrings(&self) -> ::windows::core::Result<::windows::core::Array<::windows::core::HSTRING>> {
        let this = &::windows::core::Interface::cast::<IPlayReadyContentHeader2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).KeyIdStrings)(::windows::core::Vtable::as_raw(this), ::windows::core::Array::<::windows::core::HSTRING>::set_abi_len(result__.assume_init_mut()), result__.as_mut_ptr() as *mut _ as _).and_then(|| result__.assume_init())
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateInstanceFromWindowsMediaDrmHeader(headerbytes: &[u8], licenseacquisitionurl: &super::super::super::Foundation::Uri, licenseacquisitionuserinterfaceurl: &super::super::super::Foundation::Uri, customattributes: &::windows::core::HSTRING, domainserviceid: ::windows::core::GUID) -> ::windows::core::Result<PlayReadyContentHeader> {
        Self::IPlayReadyContentHeaderFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateInstanceFromWindowsMediaDrmHeader)(::windows::core::Vtable::as_raw(this), headerbytes.len() as u32, headerbytes.as_ptr(), ::core::mem::transmute_copy(licenseacquisitionurl), ::core::mem::transmute_copy(licenseacquisitionuserinterfaceurl), ::core::mem::transmute_copy(customattributes), domainserviceid, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateInstanceFromComponents(contentkeyid: ::windows::core::GUID, contentkeyidstring: &::windows::core::HSTRING, contentencryptionalgorithm: PlayReadyEncryptionAlgorithm, licenseacquisitionurl: &super::super::super::Foundation::Uri, licenseacquisitionuserinterfaceurl: &super::super::super::Foundation::Uri, customattributes: &::windows::core::HSTRING, domainserviceid: ::windows::core::GUID) -> ::windows::core::Result<PlayReadyContentHeader> {
        Self::IPlayReadyContentHeaderFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateInstanceFromComponents)(::windows::core::Vtable::as_raw(this), contentkeyid, ::core::mem::transmute_copy(contentkeyidstring), contentencryptionalgorithm, ::core::mem::transmute_copy(licenseacquisitionurl), ::core::mem::transmute_copy(licenseacquisitionuserinterfaceurl), ::core::mem::transmute_copy(customattributes), domainserviceid, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn CreateInstanceFromPlayReadyHeader(headerbytes: &[u8]) -> ::windows::core::Result<PlayReadyContentHeader> {
        Self::IPlayReadyContentHeaderFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateInstanceFromPlayReadyHeader)(::windows::core::Vtable::as_raw(this), headerbytes.len() as u32, headerbytes.as_ptr(), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateInstanceFromComponents2(dwflags: u32, contentkeyids: &[::windows::core::GUID], contentkeyidstrings: &[::windows::core::HSTRING], contentencryptionalgorithm: PlayReadyEncryptionAlgorithm, licenseacquisitionurl: &super::super::super::Foundation::Uri, licenseacquisitionuserinterfaceurl: &super::super::super::Foundation::Uri, customattributes: &::windows::core::HSTRING, domainserviceid: ::windows::core::GUID) -> ::windows::core::Result<PlayReadyContentHeader> {
        Self::IPlayReadyContentHeaderFactory2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateInstanceFromComponents2)(::windows::core::Vtable::as_raw(this), dwflags, contentkeyids.len() as u32, contentkeyids.as_ptr(), contentkeyidstrings.len() as u32, ::core::mem::transmute(contentkeyidstrings.as_ptr()), contentencryptionalgorithm, ::core::mem::transmute_copy(licenseacquisitionurl), ::core::mem::transmute_copy(licenseacquisitionuserinterfaceurl), ::core::mem::transmute_copy(customattributes), domainserviceid, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPlayReadyContentHeaderFactory<R, F: FnOnce(&IPlayReadyContentHeaderFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<PlayReadyContentHeader, IPlayReadyContentHeaderFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IPlayReadyContentHeaderFactory2<R, F: FnOnce(&IPlayReadyContentHeaderFactory2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<PlayReadyContentHeader, IPlayReadyContentHeaderFactory2> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for PlayReadyContentHeader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PlayReadyContentHeader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PlayReadyContentHeader {}
impl ::core::fmt::Debug for PlayReadyContentHeader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PlayReadyContentHeader").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PlayReadyContentHeader {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.PlayReady.PlayReadyContentHeader;{9a438a6a-7f4c-452e-88bd-0148c6387a2c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for PlayReadyContentHeader {
    type Vtable = IPlayReadyContentHeader_Vtbl;
}
unsafe impl ::windows::core::Interface for PlayReadyContentHeader {
    const IID: ::windows::core::GUID = <IPlayReadyContentHeader as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PlayReadyContentHeader {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.PlayReadyContentHeader";
}
::windows::core::interface_hierarchy!(PlayReadyContentHeader, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[doc = "*Required features: `\"Media_Protection_PlayReady\"`*"]
pub struct PlayReadyContentResolver;
impl PlayReadyContentResolver {
    pub fn ServiceRequest(contentheader: &PlayReadyContentHeader) -> ::windows::core::Result<IPlayReadyServiceRequest> {
        Self::IPlayReadyContentResolver(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ServiceRequest)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(contentheader), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPlayReadyContentResolver<R, F: FnOnce(&IPlayReadyContentResolver) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<PlayReadyContentResolver, IPlayReadyContentResolver> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for PlayReadyContentResolver {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.PlayReadyContentResolver";
}
#[doc = "*Required features: `\"Media_Protection_PlayReady\"`*"]
#[repr(transparent)]
pub struct PlayReadyDomain(::windows::core::IUnknown);
impl PlayReadyDomain {
    pub fn AccountId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AccountId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ServiceId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ServiceId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Revision(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Revision)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn FriendlyName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FriendlyName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DomainJoinUrl(&self) -> ::windows::core::Result<super::super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DomainJoinUrl)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for PlayReadyDomain {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PlayReadyDomain {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PlayReadyDomain {}
impl ::core::fmt::Debug for PlayReadyDomain {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PlayReadyDomain").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PlayReadyDomain {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.PlayReady.PlayReadyDomain;{adcc93ac-97e6-43ef-95e4-d7868f3b16a9})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for PlayReadyDomain {
    type Vtable = IPlayReadyDomain_Vtbl;
}
unsafe impl ::windows::core::Interface for PlayReadyDomain {
    const IID: ::windows::core::GUID = <IPlayReadyDomain as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PlayReadyDomain {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.PlayReadyDomain";
}
::windows::core::interface_hierarchy!(PlayReadyDomain, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::convert::TryFrom<PlayReadyDomain> for IPlayReadyDomain {
    type Error = ::windows::core::Error;
    fn try_from(value: PlayReadyDomain) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PlayReadyDomain> for IPlayReadyDomain {
    type Error = ::windows::core::Error;
    fn try_from(value: &PlayReadyDomain) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&PlayReadyDomain> for ::windows::core::InParam<IPlayReadyDomain> {
    type Error = ::windows::core::Error;
    fn try_from(value: &PlayReadyDomain) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[doc = "*Required features: `\"Media_Protection_PlayReady\"`, `\"Foundation_Collections\"`*"]
#[cfg(feature = "Foundation_Collections")]
#[repr(transparent)]
pub struct PlayReadyDomainIterable(::windows::core::IUnknown);
#[cfg(feature = "Foundation_Collections")]
impl PlayReadyDomainIterable {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IIterator<IPlayReadyDomain>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).First)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateInstance(domainaccountid: ::windows::core::GUID) -> ::windows::core::Result<PlayReadyDomainIterable> {
        Self::IPlayReadyDomainIterableFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateInstance)(::windows::core::Vtable::as_raw(this), domainaccountid, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPlayReadyDomainIterableFactory<R, F: FnOnce(&IPlayReadyDomainIterableFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<PlayReadyDomainIterable, IPlayReadyDomainIterableFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::clone::Clone for PlayReadyDomainIterable {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::cmp::PartialEq for PlayReadyDomainIterable {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::cmp::Eq for PlayReadyDomainIterable {}
#[cfg(feature = "Foundation_Collections")]
impl ::core::fmt::Debug for PlayReadyDomainIterable {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PlayReadyDomainIterable").field(&self.0).finish()
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::RuntimeType for PlayReadyDomainIterable {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.PlayReady.PlayReadyDomainIterable;pinterface({faa585ea-6214-4217-afda-7f46de5869b3};{adcc93ac-97e6-43ef-95e4-d7868f3b16a9}))");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::Vtable for PlayReadyDomainIterable {
    type Vtable = super::super::super::Foundation::Collections::IIterable_Vtbl<IPlayReadyDomain>;
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::Interface for PlayReadyDomainIterable {
    const IID: ::windows::core::GUID = <super::super::super::Foundation::Collections::IIterable<IPlayReadyDomain> as ::windows::core::Interface>::IID;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::RuntimeName for PlayReadyDomainIterable {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.PlayReadyDomainIterable";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for PlayReadyDomainIterable {
    type Item = IPlayReadyDomain;
    type IntoIter = super::super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for &PlayReadyDomainIterable {
    type Item = IPlayReadyDomain;
    type IntoIter = super::super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
#[cfg(feature = "Foundation_Collections")]
::windows::core::interface_hierarchy!(PlayReadyDomainIterable, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<PlayReadyDomainIterable> for super::super::super::Foundation::Collections::IIterable<IPlayReadyDomain> {
    type Error = ::windows::core::Error;
    fn try_from(value: PlayReadyDomainIterable) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&PlayReadyDomainIterable> for super::super::super::Foundation::Collections::IIterable<IPlayReadyDomain> {
    type Error = ::windows::core::Error;
    fn try_from(value: &PlayReadyDomainIterable) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&PlayReadyDomainIterable> for ::windows::core::InParam<super::super::super::Foundation::Collections::IIterable<IPlayReadyDomain>> {
    type Error = ::windows::core::Error;
    fn try_from(value: &PlayReadyDomainIterable) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[doc = "*Required features: `\"Media_Protection_PlayReady\"`, `\"Foundation_Collections\"`*"]
#[cfg(feature = "Foundation_Collections")]
#[repr(transparent)]
pub struct PlayReadyDomainIterator(::windows::core::IUnknown);
#[cfg(feature = "Foundation_Collections")]
impl PlayReadyDomainIterator {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Current(&self) -> ::windows::core::Result<IPlayReadyDomain> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Current)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn HasCurrent(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).HasCurrent)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn MoveNext(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MoveNext)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMany(&self, items: &mut [::core::option::Option<IPlayReadyDomain>]) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetMany)(::windows::core::Vtable::as_raw(this), items.len() as u32, ::core::mem::transmute_copy(&items), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::clone::Clone for PlayReadyDomainIterator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::cmp::PartialEq for PlayReadyDomainIterator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::cmp::Eq for PlayReadyDomainIterator {}
#[cfg(feature = "Foundation_Collections")]
impl ::core::fmt::Debug for PlayReadyDomainIterator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PlayReadyDomainIterator").field(&self.0).finish()
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::RuntimeType for PlayReadyDomainIterator {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.PlayReady.PlayReadyDomainIterator;pinterface({6a79e863-4300-459a-9966-cbb660963ee1};{adcc93ac-97e6-43ef-95e4-d7868f3b16a9}))");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::Vtable for PlayReadyDomainIterator {
    type Vtable = super::super::super::Foundation::Collections::IIterator_Vtbl<IPlayReadyDomain>;
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::Interface for PlayReadyDomainIterator {
    const IID: ::windows::core::GUID = <super::super::super::Foundation::Collections::IIterator<IPlayReadyDomain> as ::windows::core::Interface>::IID;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::RuntimeName for PlayReadyDomainIterator {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.PlayReadyDomainIterator";
}
#[cfg(feature = "Foundation_Collections")]
::windows::core::interface_hierarchy!(PlayReadyDomainIterator, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<PlayReadyDomainIterator> for super::super::super::Foundation::Collections::IIterator<IPlayReadyDomain> {
    type Error = ::windows::core::Error;
    fn try_from(value: PlayReadyDomainIterator) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&PlayReadyDomainIterator> for super::super::super::Foundation::Collections::IIterator<IPlayReadyDomain> {
    type Error = ::windows::core::Error;
    fn try_from(value: &PlayReadyDomainIterator) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&PlayReadyDomainIterator> for ::windows::core::InParam<super::super::super::Foundation::Collections::IIterator<IPlayReadyDomain>> {
    type Error = ::windows::core::Error;
    fn try_from(value: &PlayReadyDomainIterator) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[doc = "*Required features: `\"Media_Protection_PlayReady\"`*"]
#[repr(transparent)]
pub struct PlayReadyDomainJoinServiceRequest(::windows::core::IUnknown);
impl PlayReadyDomainJoinServiceRequest {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<PlayReadyDomainJoinServiceRequest, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn ProtectionSystem(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = &::windows::core::Interface::cast::<super::IMediaProtectionServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ProtectionSystem)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Type(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = &::windows::core::Interface::cast::<super::IMediaProtectionServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Type)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn DomainAccountId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DomainAccountId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetDomainAccountId(&self, value: ::windows::core::GUID) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetDomainAccountId)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn DomainFriendlyName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DomainFriendlyName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetDomainFriendlyName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetDomainFriendlyName)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn DomainServiceId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DomainServiceId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetDomainServiceId(&self, value: ::windows::core::GUID) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetDomainServiceId)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Uri(&self) -> ::windows::core::Result<super::super::super::Foundation::Uri> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Uri)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetUri(&self, value: &super::super::super::Foundation::Uri) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetUri)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn ResponseCustomData(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ResponseCustomData)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ChallengeCustomData(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ChallengeCustomData)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetChallengeCustomData(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetChallengeCustomData)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn BeginServiceRequest(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BeginServiceRequest)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn NextServiceRequest(&self) -> ::windows::core::Result<IPlayReadyServiceRequest> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NextServiceRequest)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GenerateManualEnablingChallenge(&self) -> ::windows::core::Result<PlayReadySoapMessage> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GenerateManualEnablingChallenge)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ProcessManualEnablingResponse(&self, responsebytes: &[u8]) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ProcessManualEnablingResponse)(::windows::core::Vtable::as_raw(this), responsebytes.len() as u32, responsebytes.as_ptr(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for PlayReadyDomainJoinServiceRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PlayReadyDomainJoinServiceRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PlayReadyDomainJoinServiceRequest {}
impl ::core::fmt::Debug for PlayReadyDomainJoinServiceRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PlayReadyDomainJoinServiceRequest").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PlayReadyDomainJoinServiceRequest {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.PlayReady.PlayReadyDomainJoinServiceRequest;{171b4a5a-405f-4739-b040-67b9f0c38758})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for PlayReadyDomainJoinServiceRequest {
    type Vtable = IPlayReadyDomainJoinServiceRequest_Vtbl;
}
unsafe impl ::windows::core::Interface for PlayReadyDomainJoinServiceRequest {
    const IID: ::windows::core::GUID = <IPlayReadyDomainJoinServiceRequest as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PlayReadyDomainJoinServiceRequest {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.PlayReadyDomainJoinServiceRequest";
}
::windows::core::interface_hierarchy!(PlayReadyDomainJoinServiceRequest, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::convert::TryFrom<PlayReadyDomainJoinServiceRequest> for super::IMediaProtectionServiceRequest {
    type Error = ::windows::core::Error;
    fn try_from(value: PlayReadyDomainJoinServiceRequest) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PlayReadyDomainJoinServiceRequest> for super::IMediaProtectionServiceRequest {
    type Error = ::windows::core::Error;
    fn try_from(value: &PlayReadyDomainJoinServiceRequest) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&PlayReadyDomainJoinServiceRequest> for ::windows::core::InParam<super::IMediaProtectionServiceRequest> {
    type Error = ::windows::core::Error;
    fn try_from(value: &PlayReadyDomainJoinServiceRequest) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<PlayReadyDomainJoinServiceRequest> for IPlayReadyServiceRequest {
    type Error = ::windows::core::Error;
    fn try_from(value: PlayReadyDomainJoinServiceRequest) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PlayReadyDomainJoinServiceRequest> for IPlayReadyServiceRequest {
    type Error = ::windows::core::Error;
    fn try_from(value: &PlayReadyDomainJoinServiceRequest) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&PlayReadyDomainJoinServiceRequest> for ::windows::core::InParam<IPlayReadyServiceRequest> {
    type Error = ::windows::core::Error;
    fn try_from(value: &PlayReadyDomainJoinServiceRequest) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[doc = "*Required features: `\"Media_Protection_PlayReady\"`*"]
#[repr(transparent)]
pub struct PlayReadyDomainLeaveServiceRequest(::windows::core::IUnknown);
impl PlayReadyDomainLeaveServiceRequest {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<PlayReadyDomainLeaveServiceRequest, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn ProtectionSystem(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = &::windows::core::Interface::cast::<super::IMediaProtectionServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ProtectionSystem)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Type(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = &::windows::core::Interface::cast::<super::IMediaProtectionServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Type)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn DomainAccountId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DomainAccountId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetDomainAccountId(&self, value: ::windows::core::GUID) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetDomainAccountId)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn DomainServiceId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DomainServiceId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetDomainServiceId(&self, value: ::windows::core::GUID) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetDomainServiceId)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Uri(&self) -> ::windows::core::Result<super::super::super::Foundation::Uri> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Uri)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetUri(&self, value: &super::super::super::Foundation::Uri) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetUri)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn ResponseCustomData(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ResponseCustomData)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ChallengeCustomData(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ChallengeCustomData)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetChallengeCustomData(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetChallengeCustomData)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn BeginServiceRequest(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BeginServiceRequest)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn NextServiceRequest(&self) -> ::windows::core::Result<IPlayReadyServiceRequest> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NextServiceRequest)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GenerateManualEnablingChallenge(&self) -> ::windows::core::Result<PlayReadySoapMessage> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GenerateManualEnablingChallenge)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ProcessManualEnablingResponse(&self, responsebytes: &[u8]) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ProcessManualEnablingResponse)(::windows::core::Vtable::as_raw(this), responsebytes.len() as u32, responsebytes.as_ptr(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for PlayReadyDomainLeaveServiceRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PlayReadyDomainLeaveServiceRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PlayReadyDomainLeaveServiceRequest {}
impl ::core::fmt::Debug for PlayReadyDomainLeaveServiceRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PlayReadyDomainLeaveServiceRequest").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PlayReadyDomainLeaveServiceRequest {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.PlayReady.PlayReadyDomainLeaveServiceRequest;{062d58be-97ad-4917-aa03-46d4c252d464})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for PlayReadyDomainLeaveServiceRequest {
    type Vtable = IPlayReadyDomainLeaveServiceRequest_Vtbl;
}
unsafe impl ::windows::core::Interface for PlayReadyDomainLeaveServiceRequest {
    const IID: ::windows::core::GUID = <IPlayReadyDomainLeaveServiceRequest as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PlayReadyDomainLeaveServiceRequest {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.PlayReadyDomainLeaveServiceRequest";
}
::windows::core::interface_hierarchy!(PlayReadyDomainLeaveServiceRequest, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::convert::TryFrom<PlayReadyDomainLeaveServiceRequest> for super::IMediaProtectionServiceRequest {
    type Error = ::windows::core::Error;
    fn try_from(value: PlayReadyDomainLeaveServiceRequest) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PlayReadyDomainLeaveServiceRequest> for super::IMediaProtectionServiceRequest {
    type Error = ::windows::core::Error;
    fn try_from(value: &PlayReadyDomainLeaveServiceRequest) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&PlayReadyDomainLeaveServiceRequest> for ::windows::core::InParam<super::IMediaProtectionServiceRequest> {
    type Error = ::windows::core::Error;
    fn try_from(value: &PlayReadyDomainLeaveServiceRequest) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<PlayReadyDomainLeaveServiceRequest> for IPlayReadyServiceRequest {
    type Error = ::windows::core::Error;
    fn try_from(value: PlayReadyDomainLeaveServiceRequest) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PlayReadyDomainLeaveServiceRequest> for IPlayReadyServiceRequest {
    type Error = ::windows::core::Error;
    fn try_from(value: &PlayReadyDomainLeaveServiceRequest) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&PlayReadyDomainLeaveServiceRequest> for ::windows::core::InParam<IPlayReadyServiceRequest> {
    type Error = ::windows::core::Error;
    fn try_from(value: &PlayReadyDomainLeaveServiceRequest) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[doc = "*Required features: `\"Media_Protection_PlayReady\"`*"]
#[repr(transparent)]
pub struct PlayReadyITADataGenerator(::windows::core::IUnknown);
impl PlayReadyITADataGenerator {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<PlayReadyITADataGenerator, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GenerateData<P0, E0>(&self, guidcpsystemid: ::windows::core::GUID, countofstreams: u32, configuration: P0, format: PlayReadyITADataFormat) -> ::windows::core::Result<::windows::core::Array<u8>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::super::Foundation::Collections::IPropertySet>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GenerateData)(::windows::core::Vtable::as_raw(this), guidcpsystemid, countofstreams, configuration.try_into().map_err(|e| e.into())?.abi(), format, ::windows::core::Array::<u8>::set_abi_len(result__.assume_init_mut()), result__.as_mut_ptr() as *mut _ as _).and_then(|| result__.assume_init())
        }
    }
}
impl ::core::clone::Clone for PlayReadyITADataGenerator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PlayReadyITADataGenerator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PlayReadyITADataGenerator {}
impl ::core::fmt::Debug for PlayReadyITADataGenerator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PlayReadyITADataGenerator").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PlayReadyITADataGenerator {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.PlayReady.PlayReadyITADataGenerator;{24446b8e-10b9-4530-b25b-901a8029a9b2})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for PlayReadyITADataGenerator {
    type Vtable = IPlayReadyITADataGenerator_Vtbl;
}
unsafe impl ::windows::core::Interface for PlayReadyITADataGenerator {
    const IID: ::windows::core::GUID = <IPlayReadyITADataGenerator as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PlayReadyITADataGenerator {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.PlayReadyITADataGenerator";
}
::windows::core::interface_hierarchy!(PlayReadyITADataGenerator, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[doc = "*Required features: `\"Media_Protection_PlayReady\"`*"]
#[repr(transparent)]
pub struct PlayReadyIndividualizationServiceRequest(::windows::core::IUnknown);
impl PlayReadyIndividualizationServiceRequest {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<PlayReadyIndividualizationServiceRequest, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn ProtectionSystem(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = &::windows::core::Interface::cast::<super::IMediaProtectionServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ProtectionSystem)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Type(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = &::windows::core::Interface::cast::<super::IMediaProtectionServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Type)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Uri(&self) -> ::windows::core::Result<super::super::super::Foundation::Uri> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Uri)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetUri(&self, value: &super::super::super::Foundation::Uri) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetUri)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn ResponseCustomData(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ResponseCustomData)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ChallengeCustomData(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ChallengeCustomData)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetChallengeCustomData(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetChallengeCustomData)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn BeginServiceRequest(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BeginServiceRequest)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn NextServiceRequest(&self) -> ::windows::core::Result<IPlayReadyServiceRequest> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NextServiceRequest)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GenerateManualEnablingChallenge(&self) -> ::windows::core::Result<PlayReadySoapMessage> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GenerateManualEnablingChallenge)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ProcessManualEnablingResponse(&self, responsebytes: &[u8]) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ProcessManualEnablingResponse)(::windows::core::Vtable::as_raw(this), responsebytes.len() as u32, responsebytes.as_ptr(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for PlayReadyIndividualizationServiceRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PlayReadyIndividualizationServiceRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PlayReadyIndividualizationServiceRequest {}
impl ::core::fmt::Debug for PlayReadyIndividualizationServiceRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PlayReadyIndividualizationServiceRequest").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PlayReadyIndividualizationServiceRequest {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.PlayReady.PlayReadyIndividualizationServiceRequest;{21f5a86b-008c-4611-ab2f-aaa6c69f0e24})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for PlayReadyIndividualizationServiceRequest {
    type Vtable = IPlayReadyIndividualizationServiceRequest_Vtbl;
}
unsafe impl ::windows::core::Interface for PlayReadyIndividualizationServiceRequest {
    const IID: ::windows::core::GUID = <IPlayReadyIndividualizationServiceRequest as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PlayReadyIndividualizationServiceRequest {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.PlayReadyIndividualizationServiceRequest";
}
::windows::core::interface_hierarchy!(PlayReadyIndividualizationServiceRequest, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::convert::TryFrom<PlayReadyIndividualizationServiceRequest> for super::IMediaProtectionServiceRequest {
    type Error = ::windows::core::Error;
    fn try_from(value: PlayReadyIndividualizationServiceRequest) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PlayReadyIndividualizationServiceRequest> for super::IMediaProtectionServiceRequest {
    type Error = ::windows::core::Error;
    fn try_from(value: &PlayReadyIndividualizationServiceRequest) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&PlayReadyIndividualizationServiceRequest> for ::windows::core::InParam<super::IMediaProtectionServiceRequest> {
    type Error = ::windows::core::Error;
    fn try_from(value: &PlayReadyIndividualizationServiceRequest) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<PlayReadyIndividualizationServiceRequest> for IPlayReadyServiceRequest {
    type Error = ::windows::core::Error;
    fn try_from(value: PlayReadyIndividualizationServiceRequest) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PlayReadyIndividualizationServiceRequest> for IPlayReadyServiceRequest {
    type Error = ::windows::core::Error;
    fn try_from(value: &PlayReadyIndividualizationServiceRequest) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&PlayReadyIndividualizationServiceRequest> for ::windows::core::InParam<IPlayReadyServiceRequest> {
    type Error = ::windows::core::Error;
    fn try_from(value: &PlayReadyIndividualizationServiceRequest) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[doc = "*Required features: `\"Media_Protection_PlayReady\"`*"]
#[repr(transparent)]
pub struct PlayReadyLicense(::windows::core::IUnknown);
impl PlayReadyLicense {
    pub fn FullyEvaluated(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FullyEvaluated)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn UsableForPlay(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).UsableForPlay)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ExpirationDate(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ExpirationDate)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ExpireAfterFirstPlay(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ExpireAfterFirstPlay)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn DomainAccountID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DomainAccountID)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ChainDepth(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ChainDepth)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetKIDAtChainDepth(&self, chaindepth: u32) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetKIDAtChainDepth)(::windows::core::Vtable::as_raw(this), chaindepth, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SecureStopId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = &::windows::core::Interface::cast::<IPlayReadyLicense2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SecureStopId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SecurityLevel(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<IPlayReadyLicense2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SecurityLevel)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn InMemoryOnly(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IPlayReadyLicense2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).InMemoryOnly)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ExpiresInRealTime(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IPlayReadyLicense2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ExpiresInRealTime)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for PlayReadyLicense {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PlayReadyLicense {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PlayReadyLicense {}
impl ::core::fmt::Debug for PlayReadyLicense {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PlayReadyLicense").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PlayReadyLicense {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.PlayReady.PlayReadyLicense;{ee474c4e-fa3c-414d-a9f2-3ffc1ef832d4})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for PlayReadyLicense {
    type Vtable = IPlayReadyLicense_Vtbl;
}
unsafe impl ::windows::core::Interface for PlayReadyLicense {
    const IID: ::windows::core::GUID = <IPlayReadyLicense as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PlayReadyLicense {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.PlayReadyLicense";
}
::windows::core::interface_hierarchy!(PlayReadyLicense, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::convert::TryFrom<PlayReadyLicense> for IPlayReadyLicense {
    type Error = ::windows::core::Error;
    fn try_from(value: PlayReadyLicense) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PlayReadyLicense> for IPlayReadyLicense {
    type Error = ::windows::core::Error;
    fn try_from(value: &PlayReadyLicense) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&PlayReadyLicense> for ::windows::core::InParam<IPlayReadyLicense> {
    type Error = ::windows::core::Error;
    fn try_from(value: &PlayReadyLicense) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[doc = "*Required features: `\"Media_Protection_PlayReady\"`*"]
#[repr(transparent)]
pub struct PlayReadyLicenseAcquisitionServiceRequest(::windows::core::IUnknown);
impl PlayReadyLicenseAcquisitionServiceRequest {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<PlayReadyLicenseAcquisitionServiceRequest, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn ProtectionSystem(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = &::windows::core::Interface::cast::<super::IMediaProtectionServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ProtectionSystem)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Type(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = &::windows::core::Interface::cast::<super::IMediaProtectionServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Type)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ContentHeader(&self) -> ::windows::core::Result<PlayReadyContentHeader> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ContentHeader)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetContentHeader(&self, value: &PlayReadyContentHeader) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetContentHeader)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn DomainServiceId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DomainServiceId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetDomainServiceId(&self, value: ::windows::core::GUID) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetDomainServiceId)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn SessionId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = &::windows::core::Interface::cast::<IPlayReadyLicenseAcquisitionServiceRequest2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SessionId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateLicenseIterable(&self, contentheader: &PlayReadyContentHeader, fullyevaluated: bool) -> ::windows::core::Result<PlayReadyLicenseIterable> {
        let this = &::windows::core::Interface::cast::<IPlayReadyLicenseAcquisitionServiceRequest3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateLicenseIterable)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(contentheader), fullyevaluated, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Uri(&self) -> ::windows::core::Result<super::super::super::Foundation::Uri> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Uri)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetUri(&self, value: &super::super::super::Foundation::Uri) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetUri)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn ResponseCustomData(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ResponseCustomData)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ChallengeCustomData(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ChallengeCustomData)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetChallengeCustomData(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetChallengeCustomData)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn BeginServiceRequest(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BeginServiceRequest)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn NextServiceRequest(&self) -> ::windows::core::Result<IPlayReadyServiceRequest> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NextServiceRequest)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GenerateManualEnablingChallenge(&self) -> ::windows::core::Result<PlayReadySoapMessage> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GenerateManualEnablingChallenge)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ProcessManualEnablingResponse(&self, responsebytes: &[u8]) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ProcessManualEnablingResponse)(::windows::core::Vtable::as_raw(this), responsebytes.len() as u32, responsebytes.as_ptr(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for PlayReadyLicenseAcquisitionServiceRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PlayReadyLicenseAcquisitionServiceRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PlayReadyLicenseAcquisitionServiceRequest {}
impl ::core::fmt::Debug for PlayReadyLicenseAcquisitionServiceRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PlayReadyLicenseAcquisitionServiceRequest").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PlayReadyLicenseAcquisitionServiceRequest {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.PlayReady.PlayReadyLicenseAcquisitionServiceRequest;{5d85ff45-3e9f-4f48-93e1-9530c8d58c3e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for PlayReadyLicenseAcquisitionServiceRequest {
    type Vtable = IPlayReadyLicenseAcquisitionServiceRequest_Vtbl;
}
unsafe impl ::windows::core::Interface for PlayReadyLicenseAcquisitionServiceRequest {
    const IID: ::windows::core::GUID = <IPlayReadyLicenseAcquisitionServiceRequest as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PlayReadyLicenseAcquisitionServiceRequest {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.PlayReadyLicenseAcquisitionServiceRequest";
}
::windows::core::interface_hierarchy!(PlayReadyLicenseAcquisitionServiceRequest, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::convert::TryFrom<PlayReadyLicenseAcquisitionServiceRequest> for super::IMediaProtectionServiceRequest {
    type Error = ::windows::core::Error;
    fn try_from(value: PlayReadyLicenseAcquisitionServiceRequest) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PlayReadyLicenseAcquisitionServiceRequest> for super::IMediaProtectionServiceRequest {
    type Error = ::windows::core::Error;
    fn try_from(value: &PlayReadyLicenseAcquisitionServiceRequest) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&PlayReadyLicenseAcquisitionServiceRequest> for ::windows::core::InParam<super::IMediaProtectionServiceRequest> {
    type Error = ::windows::core::Error;
    fn try_from(value: &PlayReadyLicenseAcquisitionServiceRequest) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<PlayReadyLicenseAcquisitionServiceRequest> for IPlayReadyLicenseAcquisitionServiceRequest {
    type Error = ::windows::core::Error;
    fn try_from(value: PlayReadyLicenseAcquisitionServiceRequest) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PlayReadyLicenseAcquisitionServiceRequest> for IPlayReadyLicenseAcquisitionServiceRequest {
    type Error = ::windows::core::Error;
    fn try_from(value: &PlayReadyLicenseAcquisitionServiceRequest) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&PlayReadyLicenseAcquisitionServiceRequest> for ::windows::core::InParam<IPlayReadyLicenseAcquisitionServiceRequest> {
    type Error = ::windows::core::Error;
    fn try_from(value: &PlayReadyLicenseAcquisitionServiceRequest) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<PlayReadyLicenseAcquisitionServiceRequest> for IPlayReadyServiceRequest {
    type Error = ::windows::core::Error;
    fn try_from(value: PlayReadyLicenseAcquisitionServiceRequest) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PlayReadyLicenseAcquisitionServiceRequest> for IPlayReadyServiceRequest {
    type Error = ::windows::core::Error;
    fn try_from(value: &PlayReadyLicenseAcquisitionServiceRequest) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&PlayReadyLicenseAcquisitionServiceRequest> for ::windows::core::InParam<IPlayReadyServiceRequest> {
    type Error = ::windows::core::Error;
    fn try_from(value: &PlayReadyLicenseAcquisitionServiceRequest) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[doc = "*Required features: `\"Media_Protection_PlayReady\"`, `\"Foundation_Collections\"`*"]
#[cfg(feature = "Foundation_Collections")]
#[repr(transparent)]
pub struct PlayReadyLicenseIterable(::windows::core::IUnknown);
#[cfg(feature = "Foundation_Collections")]
impl PlayReadyLicenseIterable {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<PlayReadyLicenseIterable, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IIterator<IPlayReadyLicense>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).First)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateInstance(contentheader: &PlayReadyContentHeader, fullyevaluated: bool) -> ::windows::core::Result<PlayReadyLicenseIterable> {
        Self::IPlayReadyLicenseIterableFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateInstance)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(contentheader), fullyevaluated, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPlayReadyLicenseIterableFactory<R, F: FnOnce(&IPlayReadyLicenseIterableFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<PlayReadyLicenseIterable, IPlayReadyLicenseIterableFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::clone::Clone for PlayReadyLicenseIterable {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::cmp::PartialEq for PlayReadyLicenseIterable {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::cmp::Eq for PlayReadyLicenseIterable {}
#[cfg(feature = "Foundation_Collections")]
impl ::core::fmt::Debug for PlayReadyLicenseIterable {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PlayReadyLicenseIterable").field(&self.0).finish()
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::RuntimeType for PlayReadyLicenseIterable {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.PlayReady.PlayReadyLicenseIterable;pinterface({faa585ea-6214-4217-afda-7f46de5869b3};{ee474c4e-fa3c-414d-a9f2-3ffc1ef832d4}))");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::Vtable for PlayReadyLicenseIterable {
    type Vtable = super::super::super::Foundation::Collections::IIterable_Vtbl<IPlayReadyLicense>;
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::Interface for PlayReadyLicenseIterable {
    const IID: ::windows::core::GUID = <super::super::super::Foundation::Collections::IIterable<IPlayReadyLicense> as ::windows::core::Interface>::IID;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::RuntimeName for PlayReadyLicenseIterable {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.PlayReadyLicenseIterable";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for PlayReadyLicenseIterable {
    type Item = IPlayReadyLicense;
    type IntoIter = super::super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for &PlayReadyLicenseIterable {
    type Item = IPlayReadyLicense;
    type IntoIter = super::super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
#[cfg(feature = "Foundation_Collections")]
::windows::core::interface_hierarchy!(PlayReadyLicenseIterable, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<PlayReadyLicenseIterable> for super::super::super::Foundation::Collections::IIterable<IPlayReadyLicense> {
    type Error = ::windows::core::Error;
    fn try_from(value: PlayReadyLicenseIterable) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&PlayReadyLicenseIterable> for super::super::super::Foundation::Collections::IIterable<IPlayReadyLicense> {
    type Error = ::windows::core::Error;
    fn try_from(value: &PlayReadyLicenseIterable) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&PlayReadyLicenseIterable> for ::windows::core::InParam<super::super::super::Foundation::Collections::IIterable<IPlayReadyLicense>> {
    type Error = ::windows::core::Error;
    fn try_from(value: &PlayReadyLicenseIterable) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[doc = "*Required features: `\"Media_Protection_PlayReady\"`, `\"Foundation_Collections\"`*"]
#[cfg(feature = "Foundation_Collections")]
#[repr(transparent)]
pub struct PlayReadyLicenseIterator(::windows::core::IUnknown);
#[cfg(feature = "Foundation_Collections")]
impl PlayReadyLicenseIterator {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Current(&self) -> ::windows::core::Result<IPlayReadyLicense> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Current)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn HasCurrent(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).HasCurrent)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn MoveNext(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MoveNext)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMany(&self, items: &mut [::core::option::Option<IPlayReadyLicense>]) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetMany)(::windows::core::Vtable::as_raw(this), items.len() as u32, ::core::mem::transmute_copy(&items), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::clone::Clone for PlayReadyLicenseIterator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::cmp::PartialEq for PlayReadyLicenseIterator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::cmp::Eq for PlayReadyLicenseIterator {}
#[cfg(feature = "Foundation_Collections")]
impl ::core::fmt::Debug for PlayReadyLicenseIterator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PlayReadyLicenseIterator").field(&self.0).finish()
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::RuntimeType for PlayReadyLicenseIterator {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.PlayReady.PlayReadyLicenseIterator;pinterface({6a79e863-4300-459a-9966-cbb660963ee1};{ee474c4e-fa3c-414d-a9f2-3ffc1ef832d4}))");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::Vtable for PlayReadyLicenseIterator {
    type Vtable = super::super::super::Foundation::Collections::IIterator_Vtbl<IPlayReadyLicense>;
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::Interface for PlayReadyLicenseIterator {
    const IID: ::windows::core::GUID = <super::super::super::Foundation::Collections::IIterator<IPlayReadyLicense> as ::windows::core::Interface>::IID;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::RuntimeName for PlayReadyLicenseIterator {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.PlayReadyLicenseIterator";
}
#[cfg(feature = "Foundation_Collections")]
::windows::core::interface_hierarchy!(PlayReadyLicenseIterator, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<PlayReadyLicenseIterator> for super::super::super::Foundation::Collections::IIterator<IPlayReadyLicense> {
    type Error = ::windows::core::Error;
    fn try_from(value: PlayReadyLicenseIterator) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&PlayReadyLicenseIterator> for super::super::super::Foundation::Collections::IIterator<IPlayReadyLicense> {
    type Error = ::windows::core::Error;
    fn try_from(value: &PlayReadyLicenseIterator) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&PlayReadyLicenseIterator> for ::windows::core::InParam<super::super::super::Foundation::Collections::IIterator<IPlayReadyLicense>> {
    type Error = ::windows::core::Error;
    fn try_from(value: &PlayReadyLicenseIterator) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[doc = "*Required features: `\"Media_Protection_PlayReady\"`*"]
pub struct PlayReadyLicenseManagement;
impl PlayReadyLicenseManagement {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DeleteLicenses(contentheader: &PlayReadyContentHeader) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        Self::IPlayReadyLicenseManagement(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DeleteLicenses)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(contentheader), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPlayReadyLicenseManagement<R, F: FnOnce(&IPlayReadyLicenseManagement) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<PlayReadyLicenseManagement, IPlayReadyLicenseManagement> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for PlayReadyLicenseManagement {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.PlayReadyLicenseManagement";
}
#[doc = "*Required features: `\"Media_Protection_PlayReady\"`*"]
#[repr(transparent)]
pub struct PlayReadyLicenseSession(::windows::core::IUnknown);
impl PlayReadyLicenseSession {
    pub fn CreateLAServiceRequest(&self) -> ::windows::core::Result<IPlayReadyLicenseAcquisitionServiceRequest> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateLAServiceRequest)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ConfigureMediaProtectionManager(&self, mpm: &super::MediaProtectionManager) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).ConfigureMediaProtectionManager)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(mpm)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateLicenseIterable(&self, contentheader: &PlayReadyContentHeader, fullyevaluated: bool) -> ::windows::core::Result<PlayReadyLicenseIterable> {
        let this = &::windows::core::Interface::cast::<IPlayReadyLicenseSession2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateLicenseIterable)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(contentheader), fullyevaluated, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateInstance<P0, E0>(configuration: P0) -> ::windows::core::Result<PlayReadyLicenseSession>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::super::Foundation::Collections::IPropertySet>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::IPlayReadyLicenseSessionFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateInstance)(::windows::core::Vtable::as_raw(this), configuration.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPlayReadyLicenseSessionFactory<R, F: FnOnce(&IPlayReadyLicenseSessionFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<PlayReadyLicenseSession, IPlayReadyLicenseSessionFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for PlayReadyLicenseSession {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PlayReadyLicenseSession {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PlayReadyLicenseSession {}
impl ::core::fmt::Debug for PlayReadyLicenseSession {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PlayReadyLicenseSession").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PlayReadyLicenseSession {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.PlayReady.PlayReadyLicenseSession;{a1723a39-87fa-4fdd-abbb-a9720e845259})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for PlayReadyLicenseSession {
    type Vtable = IPlayReadyLicenseSession_Vtbl;
}
unsafe impl ::windows::core::Interface for PlayReadyLicenseSession {
    const IID: ::windows::core::GUID = <IPlayReadyLicenseSession as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PlayReadyLicenseSession {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.PlayReadyLicenseSession";
}
::windows::core::interface_hierarchy!(PlayReadyLicenseSession, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::convert::TryFrom<PlayReadyLicenseSession> for IPlayReadyLicenseSession {
    type Error = ::windows::core::Error;
    fn try_from(value: PlayReadyLicenseSession) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PlayReadyLicenseSession> for IPlayReadyLicenseSession {
    type Error = ::windows::core::Error;
    fn try_from(value: &PlayReadyLicenseSession) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&PlayReadyLicenseSession> for ::windows::core::InParam<IPlayReadyLicenseSession> {
    type Error = ::windows::core::Error;
    fn try_from(value: &PlayReadyLicenseSession) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<PlayReadyLicenseSession> for IPlayReadyLicenseSession2 {
    type Error = ::windows::core::Error;
    fn try_from(value: PlayReadyLicenseSession) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PlayReadyLicenseSession> for IPlayReadyLicenseSession2 {
    type Error = ::windows::core::Error;
    fn try_from(value: &PlayReadyLicenseSession) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&PlayReadyLicenseSession> for ::windows::core::InParam<IPlayReadyLicenseSession2> {
    type Error = ::windows::core::Error;
    fn try_from(value: &PlayReadyLicenseSession) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[doc = "*Required features: `\"Media_Protection_PlayReady\"`*"]
#[repr(transparent)]
pub struct PlayReadyMeteringReportServiceRequest(::windows::core::IUnknown);
impl PlayReadyMeteringReportServiceRequest {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<PlayReadyMeteringReportServiceRequest, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn ProtectionSystem(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = &::windows::core::Interface::cast::<super::IMediaProtectionServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ProtectionSystem)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Type(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = &::windows::core::Interface::cast::<super::IMediaProtectionServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Type)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn MeteringCertificate(&self) -> ::windows::core::Result<::windows::core::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MeteringCertificate)(::windows::core::Vtable::as_raw(this), ::windows::core::Array::<u8>::set_abi_len(result__.assume_init_mut()), result__.as_mut_ptr() as *mut _ as _).and_then(|| result__.assume_init())
        }
    }
    pub fn SetMeteringCertificate(&self, meteringcertbytes: &[u8]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetMeteringCertificate)(::windows::core::Vtable::as_raw(this), meteringcertbytes.len() as u32, meteringcertbytes.as_ptr()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Uri(&self) -> ::windows::core::Result<super::super::super::Foundation::Uri> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Uri)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetUri(&self, value: &super::super::super::Foundation::Uri) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetUri)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn ResponseCustomData(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ResponseCustomData)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ChallengeCustomData(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ChallengeCustomData)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetChallengeCustomData(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetChallengeCustomData)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn BeginServiceRequest(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BeginServiceRequest)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn NextServiceRequest(&self) -> ::windows::core::Result<IPlayReadyServiceRequest> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NextServiceRequest)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GenerateManualEnablingChallenge(&self) -> ::windows::core::Result<PlayReadySoapMessage> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GenerateManualEnablingChallenge)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ProcessManualEnablingResponse(&self, responsebytes: &[u8]) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ProcessManualEnablingResponse)(::windows::core::Vtable::as_raw(this), responsebytes.len() as u32, responsebytes.as_ptr(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for PlayReadyMeteringReportServiceRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PlayReadyMeteringReportServiceRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PlayReadyMeteringReportServiceRequest {}
impl ::core::fmt::Debug for PlayReadyMeteringReportServiceRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PlayReadyMeteringReportServiceRequest").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PlayReadyMeteringReportServiceRequest {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.PlayReady.PlayReadyMeteringReportServiceRequest;{c12b231c-0ecd-4f11-a185-1e24a4a67fb7})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for PlayReadyMeteringReportServiceRequest {
    type Vtable = IPlayReadyMeteringReportServiceRequest_Vtbl;
}
unsafe impl ::windows::core::Interface for PlayReadyMeteringReportServiceRequest {
    const IID: ::windows::core::GUID = <IPlayReadyMeteringReportServiceRequest as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PlayReadyMeteringReportServiceRequest {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.PlayReadyMeteringReportServiceRequest";
}
::windows::core::interface_hierarchy!(PlayReadyMeteringReportServiceRequest, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::convert::TryFrom<PlayReadyMeteringReportServiceRequest> for super::IMediaProtectionServiceRequest {
    type Error = ::windows::core::Error;
    fn try_from(value: PlayReadyMeteringReportServiceRequest) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PlayReadyMeteringReportServiceRequest> for super::IMediaProtectionServiceRequest {
    type Error = ::windows::core::Error;
    fn try_from(value: &PlayReadyMeteringReportServiceRequest) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&PlayReadyMeteringReportServiceRequest> for ::windows::core::InParam<super::IMediaProtectionServiceRequest> {
    type Error = ::windows::core::Error;
    fn try_from(value: &PlayReadyMeteringReportServiceRequest) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<PlayReadyMeteringReportServiceRequest> for IPlayReadyServiceRequest {
    type Error = ::windows::core::Error;
    fn try_from(value: PlayReadyMeteringReportServiceRequest) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PlayReadyMeteringReportServiceRequest> for IPlayReadyServiceRequest {
    type Error = ::windows::core::Error;
    fn try_from(value: &PlayReadyMeteringReportServiceRequest) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&PlayReadyMeteringReportServiceRequest> for ::windows::core::InParam<IPlayReadyServiceRequest> {
    type Error = ::windows::core::Error;
    fn try_from(value: &PlayReadyMeteringReportServiceRequest) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[doc = "*Required features: `\"Media_Protection_PlayReady\"`*"]
#[repr(transparent)]
pub struct PlayReadyRevocationServiceRequest(::windows::core::IUnknown);
impl PlayReadyRevocationServiceRequest {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<PlayReadyRevocationServiceRequest, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn ProtectionSystem(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = &::windows::core::Interface::cast::<super::IMediaProtectionServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ProtectionSystem)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Type(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = &::windows::core::Interface::cast::<super::IMediaProtectionServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Type)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Uri(&self) -> ::windows::core::Result<super::super::super::Foundation::Uri> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Uri)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetUri(&self, value: &super::super::super::Foundation::Uri) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetUri)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn ResponseCustomData(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ResponseCustomData)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ChallengeCustomData(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ChallengeCustomData)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetChallengeCustomData(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetChallengeCustomData)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn BeginServiceRequest(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BeginServiceRequest)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn NextServiceRequest(&self) -> ::windows::core::Result<IPlayReadyServiceRequest> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NextServiceRequest)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GenerateManualEnablingChallenge(&self) -> ::windows::core::Result<PlayReadySoapMessage> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GenerateManualEnablingChallenge)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ProcessManualEnablingResponse(&self, responsebytes: &[u8]) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ProcessManualEnablingResponse)(::windows::core::Vtable::as_raw(this), responsebytes.len() as u32, responsebytes.as_ptr(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for PlayReadyRevocationServiceRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PlayReadyRevocationServiceRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PlayReadyRevocationServiceRequest {}
impl ::core::fmt::Debug for PlayReadyRevocationServiceRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PlayReadyRevocationServiceRequest").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PlayReadyRevocationServiceRequest {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.PlayReady.PlayReadyRevocationServiceRequest;{543d66ac-faf0-4560-84a5-0e4acec939e4})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for PlayReadyRevocationServiceRequest {
    type Vtable = IPlayReadyRevocationServiceRequest_Vtbl;
}
unsafe impl ::windows::core::Interface for PlayReadyRevocationServiceRequest {
    const IID: ::windows::core::GUID = <IPlayReadyRevocationServiceRequest as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PlayReadyRevocationServiceRequest {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.PlayReadyRevocationServiceRequest";
}
::windows::core::interface_hierarchy!(PlayReadyRevocationServiceRequest, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::convert::TryFrom<PlayReadyRevocationServiceRequest> for super::IMediaProtectionServiceRequest {
    type Error = ::windows::core::Error;
    fn try_from(value: PlayReadyRevocationServiceRequest) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PlayReadyRevocationServiceRequest> for super::IMediaProtectionServiceRequest {
    type Error = ::windows::core::Error;
    fn try_from(value: &PlayReadyRevocationServiceRequest) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&PlayReadyRevocationServiceRequest> for ::windows::core::InParam<super::IMediaProtectionServiceRequest> {
    type Error = ::windows::core::Error;
    fn try_from(value: &PlayReadyRevocationServiceRequest) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<PlayReadyRevocationServiceRequest> for IPlayReadyServiceRequest {
    type Error = ::windows::core::Error;
    fn try_from(value: PlayReadyRevocationServiceRequest) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PlayReadyRevocationServiceRequest> for IPlayReadyServiceRequest {
    type Error = ::windows::core::Error;
    fn try_from(value: &PlayReadyRevocationServiceRequest) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&PlayReadyRevocationServiceRequest> for ::windows::core::InParam<IPlayReadyServiceRequest> {
    type Error = ::windows::core::Error;
    fn try_from(value: &PlayReadyRevocationServiceRequest) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[doc = "*Required features: `\"Media_Protection_PlayReady\"`, `\"Foundation_Collections\"`*"]
#[cfg(feature = "Foundation_Collections")]
#[repr(transparent)]
pub struct PlayReadySecureStopIterable(::windows::core::IUnknown);
#[cfg(feature = "Foundation_Collections")]
impl PlayReadySecureStopIterable {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IIterator<IPlayReadySecureStopServiceRequest>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).First)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateInstance(publishercertbytes: &[u8]) -> ::windows::core::Result<PlayReadySecureStopIterable> {
        Self::IPlayReadySecureStopIterableFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateInstance)(::windows::core::Vtable::as_raw(this), publishercertbytes.len() as u32, publishercertbytes.as_ptr(), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPlayReadySecureStopIterableFactory<R, F: FnOnce(&IPlayReadySecureStopIterableFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<PlayReadySecureStopIterable, IPlayReadySecureStopIterableFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::clone::Clone for PlayReadySecureStopIterable {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::cmp::PartialEq for PlayReadySecureStopIterable {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::cmp::Eq for PlayReadySecureStopIterable {}
#[cfg(feature = "Foundation_Collections")]
impl ::core::fmt::Debug for PlayReadySecureStopIterable {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PlayReadySecureStopIterable").field(&self.0).finish()
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::RuntimeType for PlayReadySecureStopIterable {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.PlayReady.PlayReadySecureStopIterable;pinterface({faa585ea-6214-4217-afda-7f46de5869b3};{b5501ee5-01bf-4401-9677-05630a6a4cc8}))");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::Vtable for PlayReadySecureStopIterable {
    type Vtable = super::super::super::Foundation::Collections::IIterable_Vtbl<IPlayReadySecureStopServiceRequest>;
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::Interface for PlayReadySecureStopIterable {
    const IID: ::windows::core::GUID = <super::super::super::Foundation::Collections::IIterable<IPlayReadySecureStopServiceRequest> as ::windows::core::Interface>::IID;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::RuntimeName for PlayReadySecureStopIterable {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.PlayReadySecureStopIterable";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for PlayReadySecureStopIterable {
    type Item = IPlayReadySecureStopServiceRequest;
    type IntoIter = super::super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for &PlayReadySecureStopIterable {
    type Item = IPlayReadySecureStopServiceRequest;
    type IntoIter = super::super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
#[cfg(feature = "Foundation_Collections")]
::windows::core::interface_hierarchy!(PlayReadySecureStopIterable, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<PlayReadySecureStopIterable> for super::super::super::Foundation::Collections::IIterable<IPlayReadySecureStopServiceRequest> {
    type Error = ::windows::core::Error;
    fn try_from(value: PlayReadySecureStopIterable) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&PlayReadySecureStopIterable> for super::super::super::Foundation::Collections::IIterable<IPlayReadySecureStopServiceRequest> {
    type Error = ::windows::core::Error;
    fn try_from(value: &PlayReadySecureStopIterable) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&PlayReadySecureStopIterable> for ::windows::core::InParam<super::super::super::Foundation::Collections::IIterable<IPlayReadySecureStopServiceRequest>> {
    type Error = ::windows::core::Error;
    fn try_from(value: &PlayReadySecureStopIterable) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[doc = "*Required features: `\"Media_Protection_PlayReady\"`, `\"Foundation_Collections\"`*"]
#[cfg(feature = "Foundation_Collections")]
#[repr(transparent)]
pub struct PlayReadySecureStopIterator(::windows::core::IUnknown);
#[cfg(feature = "Foundation_Collections")]
impl PlayReadySecureStopIterator {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Current(&self) -> ::windows::core::Result<IPlayReadySecureStopServiceRequest> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Current)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn HasCurrent(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).HasCurrent)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn MoveNext(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MoveNext)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMany(&self, items: &mut [::core::option::Option<IPlayReadySecureStopServiceRequest>]) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetMany)(::windows::core::Vtable::as_raw(this), items.len() as u32, ::core::mem::transmute_copy(&items), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::clone::Clone for PlayReadySecureStopIterator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::cmp::PartialEq for PlayReadySecureStopIterator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::cmp::Eq for PlayReadySecureStopIterator {}
#[cfg(feature = "Foundation_Collections")]
impl ::core::fmt::Debug for PlayReadySecureStopIterator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PlayReadySecureStopIterator").field(&self.0).finish()
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::RuntimeType for PlayReadySecureStopIterator {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.PlayReady.PlayReadySecureStopIterator;pinterface({6a79e863-4300-459a-9966-cbb660963ee1};{b5501ee5-01bf-4401-9677-05630a6a4cc8}))");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::Vtable for PlayReadySecureStopIterator {
    type Vtable = super::super::super::Foundation::Collections::IIterator_Vtbl<IPlayReadySecureStopServiceRequest>;
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::Interface for PlayReadySecureStopIterator {
    const IID: ::windows::core::GUID = <super::super::super::Foundation::Collections::IIterator<IPlayReadySecureStopServiceRequest> as ::windows::core::Interface>::IID;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::RuntimeName for PlayReadySecureStopIterator {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.PlayReadySecureStopIterator";
}
#[cfg(feature = "Foundation_Collections")]
::windows::core::interface_hierarchy!(PlayReadySecureStopIterator, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<PlayReadySecureStopIterator> for super::super::super::Foundation::Collections::IIterator<IPlayReadySecureStopServiceRequest> {
    type Error = ::windows::core::Error;
    fn try_from(value: PlayReadySecureStopIterator) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&PlayReadySecureStopIterator> for super::super::super::Foundation::Collections::IIterator<IPlayReadySecureStopServiceRequest> {
    type Error = ::windows::core::Error;
    fn try_from(value: &PlayReadySecureStopIterator) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&PlayReadySecureStopIterator> for ::windows::core::InParam<super::super::super::Foundation::Collections::IIterator<IPlayReadySecureStopServiceRequest>> {
    type Error = ::windows::core::Error;
    fn try_from(value: &PlayReadySecureStopIterator) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[doc = "*Required features: `\"Media_Protection_PlayReady\"`*"]
#[repr(transparent)]
pub struct PlayReadySecureStopServiceRequest(::windows::core::IUnknown);
impl PlayReadySecureStopServiceRequest {
    pub fn ProtectionSystem(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = &::windows::core::Interface::cast::<super::IMediaProtectionServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ProtectionSystem)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Type(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = &::windows::core::Interface::cast::<super::IMediaProtectionServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Type)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SessionID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SessionID)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StartTime(&self) -> ::windows::core::Result<super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).StartTime)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn UpdateTime(&self) -> ::windows::core::Result<super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).UpdateTime)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Stopped(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Stopped)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn PublisherCertificate(&self) -> ::windows::core::Result<::windows::core::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PublisherCertificate)(::windows::core::Vtable::as_raw(this), ::windows::core::Array::<u8>::set_abi_len(result__.assume_init_mut()), result__.as_mut_ptr() as *mut _ as _).and_then(|| result__.assume_init())
        }
    }
    pub fn CreateInstance(publishercertbytes: &[u8]) -> ::windows::core::Result<PlayReadySecureStopServiceRequest> {
        Self::IPlayReadySecureStopServiceRequestFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateInstance)(::windows::core::Vtable::as_raw(this), publishercertbytes.len() as u32, publishercertbytes.as_ptr(), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn CreateInstanceFromSessionID(sessionid: ::windows::core::GUID, publishercertbytes: &[u8]) -> ::windows::core::Result<PlayReadySecureStopServiceRequest> {
        Self::IPlayReadySecureStopServiceRequestFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateInstanceFromSessionID)(::windows::core::Vtable::as_raw(this), sessionid, publishercertbytes.len() as u32, publishercertbytes.as_ptr(), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Uri(&self) -> ::windows::core::Result<super::super::super::Foundation::Uri> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Uri)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetUri(&self, value: &super::super::super::Foundation::Uri) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetUri)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn ResponseCustomData(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ResponseCustomData)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ChallengeCustomData(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ChallengeCustomData)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetChallengeCustomData(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetChallengeCustomData)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn BeginServiceRequest(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BeginServiceRequest)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn NextServiceRequest(&self) -> ::windows::core::Result<IPlayReadyServiceRequest> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NextServiceRequest)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GenerateManualEnablingChallenge(&self) -> ::windows::core::Result<PlayReadySoapMessage> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GenerateManualEnablingChallenge)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ProcessManualEnablingResponse(&self, responsebytes: &[u8]) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ProcessManualEnablingResponse)(::windows::core::Vtable::as_raw(this), responsebytes.len() as u32, responsebytes.as_ptr(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc(hidden)]
    pub fn IPlayReadySecureStopServiceRequestFactory<R, F: FnOnce(&IPlayReadySecureStopServiceRequestFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<PlayReadySecureStopServiceRequest, IPlayReadySecureStopServiceRequestFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for PlayReadySecureStopServiceRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PlayReadySecureStopServiceRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PlayReadySecureStopServiceRequest {}
impl ::core::fmt::Debug for PlayReadySecureStopServiceRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PlayReadySecureStopServiceRequest").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PlayReadySecureStopServiceRequest {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.PlayReady.PlayReadySecureStopServiceRequest;{b5501ee5-01bf-4401-9677-05630a6a4cc8})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for PlayReadySecureStopServiceRequest {
    type Vtable = IPlayReadySecureStopServiceRequest_Vtbl;
}
unsafe impl ::windows::core::Interface for PlayReadySecureStopServiceRequest {
    const IID: ::windows::core::GUID = <IPlayReadySecureStopServiceRequest as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PlayReadySecureStopServiceRequest {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.PlayReadySecureStopServiceRequest";
}
::windows::core::interface_hierarchy!(PlayReadySecureStopServiceRequest, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::convert::TryFrom<PlayReadySecureStopServiceRequest> for super::IMediaProtectionServiceRequest {
    type Error = ::windows::core::Error;
    fn try_from(value: PlayReadySecureStopServiceRequest) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PlayReadySecureStopServiceRequest> for super::IMediaProtectionServiceRequest {
    type Error = ::windows::core::Error;
    fn try_from(value: &PlayReadySecureStopServiceRequest) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&PlayReadySecureStopServiceRequest> for ::windows::core::InParam<super::IMediaProtectionServiceRequest> {
    type Error = ::windows::core::Error;
    fn try_from(value: &PlayReadySecureStopServiceRequest) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<PlayReadySecureStopServiceRequest> for IPlayReadySecureStopServiceRequest {
    type Error = ::windows::core::Error;
    fn try_from(value: PlayReadySecureStopServiceRequest) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PlayReadySecureStopServiceRequest> for IPlayReadySecureStopServiceRequest {
    type Error = ::windows::core::Error;
    fn try_from(value: &PlayReadySecureStopServiceRequest) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&PlayReadySecureStopServiceRequest> for ::windows::core::InParam<IPlayReadySecureStopServiceRequest> {
    type Error = ::windows::core::Error;
    fn try_from(value: &PlayReadySecureStopServiceRequest) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<PlayReadySecureStopServiceRequest> for IPlayReadyServiceRequest {
    type Error = ::windows::core::Error;
    fn try_from(value: PlayReadySecureStopServiceRequest) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PlayReadySecureStopServiceRequest> for IPlayReadyServiceRequest {
    type Error = ::windows::core::Error;
    fn try_from(value: &PlayReadySecureStopServiceRequest) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&PlayReadySecureStopServiceRequest> for ::windows::core::InParam<IPlayReadyServiceRequest> {
    type Error = ::windows::core::Error;
    fn try_from(value: &PlayReadySecureStopServiceRequest) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[doc = "*Required features: `\"Media_Protection_PlayReady\"`*"]
#[repr(transparent)]
pub struct PlayReadySoapMessage(::windows::core::IUnknown);
impl PlayReadySoapMessage {
    pub fn GetMessageBody(&self) -> ::windows::core::Result<::windows::core::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetMessageBody)(::windows::core::Vtable::as_raw(this), ::windows::core::Array::<u8>::set_abi_len(result__.assume_init_mut()), result__.as_mut_ptr() as *mut _ as _).and_then(|| result__.assume_init())
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn MessageHeaders(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IPropertySet> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MessageHeaders)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Uri(&self) -> ::windows::core::Result<super::super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Uri)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for PlayReadySoapMessage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PlayReadySoapMessage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PlayReadySoapMessage {}
impl ::core::fmt::Debug for PlayReadySoapMessage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PlayReadySoapMessage").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PlayReadySoapMessage {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.PlayReady.PlayReadySoapMessage;{b659fcb5-ce41-41ba-8a0d-61df5fffa139})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for PlayReadySoapMessage {
    type Vtable = IPlayReadySoapMessage_Vtbl;
}
unsafe impl ::windows::core::Interface for PlayReadySoapMessage {
    const IID: ::windows::core::GUID = <IPlayReadySoapMessage as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PlayReadySoapMessage {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.PlayReadySoapMessage";
}
::windows::core::interface_hierarchy!(PlayReadySoapMessage, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[doc = "*Required features: `\"Media_Protection_PlayReady\"`*"]
pub struct PlayReadyStatics;
impl PlayReadyStatics {
    pub fn DomainJoinServiceRequestType() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IPlayReadyStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DomainJoinServiceRequestType)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn DomainLeaveServiceRequestType() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IPlayReadyStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DomainLeaveServiceRequestType)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn IndividualizationServiceRequestType() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IPlayReadyStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IndividualizationServiceRequestType)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn LicenseAcquirerServiceRequestType() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IPlayReadyStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LicenseAcquirerServiceRequestType)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn MeteringReportServiceRequestType() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IPlayReadyStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MeteringReportServiceRequestType)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn RevocationServiceRequestType() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IPlayReadyStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RevocationServiceRequestType)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn MediaProtectionSystemId() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IPlayReadyStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MediaProtectionSystemId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn PlayReadySecurityVersion() -> ::windows::core::Result<u32> {
        Self::IPlayReadyStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PlayReadySecurityVersion)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn PlayReadyCertificateSecurityLevel() -> ::windows::core::Result<u32> {
        Self::IPlayReadyStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PlayReadyCertificateSecurityLevel)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn SecureStopServiceRequestType() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IPlayReadyStatics3(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SecureStopServiceRequestType)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn CheckSupportedHardware(hwdrmfeature: PlayReadyHardwareDRMFeatures) -> ::windows::core::Result<bool> {
        Self::IPlayReadyStatics3(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CheckSupportedHardware)(::windows::core::Vtable::as_raw(this), hwdrmfeature, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn InputTrustAuthorityToCreate() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IPlayReadyStatics4(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).InputTrustAuthorityToCreate)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn ProtectionSystemId() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IPlayReadyStatics4(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ProtectionSystemId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn HardwareDRMDisabledAtTime() -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>> {
        Self::IPlayReadyStatics5(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).HardwareDRMDisabledAtTime)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn HardwareDRMDisabledUntilTime() -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>> {
        Self::IPlayReadyStatics5(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).HardwareDRMDisabledUntilTime)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn ResetHardwareDRMDisabled() -> ::windows::core::Result<()> {
        Self::IPlayReadyStatics5(|this| unsafe { (::windows::core::Vtable::vtable(this).ResetHardwareDRMDisabled)(::windows::core::Vtable::as_raw(this)).ok() })
    }
    #[doc(hidden)]
    pub fn IPlayReadyStatics<R, F: FnOnce(&IPlayReadyStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<PlayReadyStatics, IPlayReadyStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IPlayReadyStatics2<R, F: FnOnce(&IPlayReadyStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<PlayReadyStatics, IPlayReadyStatics2> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IPlayReadyStatics3<R, F: FnOnce(&IPlayReadyStatics3) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<PlayReadyStatics, IPlayReadyStatics3> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IPlayReadyStatics4<R, F: FnOnce(&IPlayReadyStatics4) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<PlayReadyStatics, IPlayReadyStatics4> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IPlayReadyStatics5<R, F: FnOnce(&IPlayReadyStatics5) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<PlayReadyStatics, IPlayReadyStatics5> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for PlayReadyStatics {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.PlayReadyStatics";
}
#[doc = "*Required features: `\"Media_Protection_PlayReady\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NDCertificateFeature(pub i32);
#[cfg(feature = "deprecated")]
impl NDCertificateFeature {
    pub const Transmitter: Self = Self(1i32);
    pub const Receiver: Self = Self(2i32);
    pub const SharedCertificate: Self = Self(3i32);
    pub const SecureClock: Self = Self(4i32);
    pub const AntiRollBackClock: Self = Self(5i32);
    pub const CRLS: Self = Self(9i32);
    pub const PlayReady3Features: Self = Self(13i32);
}
#[cfg(feature = "deprecated")]
impl ::core::marker::Copy for NDCertificateFeature {}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for NDCertificateFeature {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "deprecated")]
impl ::core::default::Default for NDCertificateFeature {
    fn default() -> Self {
        Self(0)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Abi for NDCertificateFeature {
    type Abi = Self;
}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for NDCertificateFeature {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NDCertificateFeature").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for NDCertificateFeature {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Protection.PlayReady.NDCertificateFeature;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_Protection_PlayReady\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NDCertificatePlatformID(pub i32);
#[cfg(feature = "deprecated")]
impl NDCertificatePlatformID {
    pub const Windows: Self = Self(0i32);
    pub const OSX: Self = Self(1i32);
    pub const WindowsOnARM: Self = Self(2i32);
    pub const WindowsMobile7: Self = Self(5i32);
    pub const iOSOnARM: Self = Self(6i32);
    pub const XBoxOnPPC: Self = Self(7i32);
    pub const WindowsPhone8OnARM: Self = Self(8i32);
    pub const WindowsPhone8OnX86: Self = Self(9i32);
    pub const XboxOne: Self = Self(10i32);
    pub const AndroidOnARM: Self = Self(11i32);
    pub const WindowsPhone81OnARM: Self = Self(12i32);
    pub const WindowsPhone81OnX86: Self = Self(13i32);
}
#[cfg(feature = "deprecated")]
impl ::core::marker::Copy for NDCertificatePlatformID {}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for NDCertificatePlatformID {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "deprecated")]
impl ::core::default::Default for NDCertificatePlatformID {
    fn default() -> Self {
        Self(0)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Abi for NDCertificatePlatformID {
    type Abi = Self;
}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for NDCertificatePlatformID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NDCertificatePlatformID").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for NDCertificatePlatformID {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Protection.PlayReady.NDCertificatePlatformID;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_Protection_PlayReady\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NDCertificateType(pub i32);
#[cfg(feature = "deprecated")]
impl NDCertificateType {
    pub const Unknown: Self = Self(0i32);
    pub const PC: Self = Self(1i32);
    pub const Device: Self = Self(2i32);
    pub const Domain: Self = Self(3i32);
    pub const Issuer: Self = Self(4i32);
    pub const CrlSigner: Self = Self(5i32);
    pub const Service: Self = Self(6i32);
    pub const Silverlight: Self = Self(7i32);
    pub const Application: Self = Self(8i32);
    pub const Metering: Self = Self(9i32);
    pub const KeyFileSigner: Self = Self(10i32);
    pub const Server: Self = Self(11i32);
    pub const LicenseSigner: Self = Self(12i32);
}
#[cfg(feature = "deprecated")]
impl ::core::marker::Copy for NDCertificateType {}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for NDCertificateType {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "deprecated")]
impl ::core::default::Default for NDCertificateType {
    fn default() -> Self {
        Self(0)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Abi for NDCertificateType {
    type Abi = Self;
}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for NDCertificateType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NDCertificateType").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for NDCertificateType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Protection.PlayReady.NDCertificateType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_Protection_PlayReady\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NDClosedCaptionFormat(pub i32);
#[cfg(feature = "deprecated")]
impl NDClosedCaptionFormat {
    pub const ATSC: Self = Self(0i32);
    pub const SCTE20: Self = Self(1i32);
    pub const Unknown: Self = Self(2i32);
}
#[cfg(feature = "deprecated")]
impl ::core::marker::Copy for NDClosedCaptionFormat {}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for NDClosedCaptionFormat {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "deprecated")]
impl ::core::default::Default for NDClosedCaptionFormat {
    fn default() -> Self {
        Self(0)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Abi for NDClosedCaptionFormat {
    type Abi = Self;
}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for NDClosedCaptionFormat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NDClosedCaptionFormat").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for NDClosedCaptionFormat {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Protection.PlayReady.NDClosedCaptionFormat;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_Protection_PlayReady\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NDContentIDType(pub i32);
#[cfg(feature = "deprecated")]
impl NDContentIDType {
    pub const KeyID: Self = Self(1i32);
    pub const PlayReadyObject: Self = Self(2i32);
    pub const Custom: Self = Self(3i32);
}
#[cfg(feature = "deprecated")]
impl ::core::marker::Copy for NDContentIDType {}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for NDContentIDType {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "deprecated")]
impl ::core::default::Default for NDContentIDType {
    fn default() -> Self {
        Self(0)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Abi for NDContentIDType {
    type Abi = Self;
}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for NDContentIDType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NDContentIDType").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for NDContentIDType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Protection.PlayReady.NDContentIDType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_Protection_PlayReady\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NDMediaStreamType(pub i32);
#[cfg(feature = "deprecated")]
impl NDMediaStreamType {
    pub const Audio: Self = Self(1i32);
    pub const Video: Self = Self(2i32);
}
#[cfg(feature = "deprecated")]
impl ::core::marker::Copy for NDMediaStreamType {}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for NDMediaStreamType {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "deprecated")]
impl ::core::default::Default for NDMediaStreamType {
    fn default() -> Self {
        Self(0)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Abi for NDMediaStreamType {
    type Abi = Self;
}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for NDMediaStreamType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NDMediaStreamType").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for NDMediaStreamType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Protection.PlayReady.NDMediaStreamType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_Protection_PlayReady\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NDProximityDetectionType(pub i32);
#[cfg(feature = "deprecated")]
impl NDProximityDetectionType {
    pub const UDP: Self = Self(1i32);
    pub const TCP: Self = Self(2i32);
    pub const TransportAgnostic: Self = Self(4i32);
}
#[cfg(feature = "deprecated")]
impl ::core::marker::Copy for NDProximityDetectionType {}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for NDProximityDetectionType {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "deprecated")]
impl ::core::default::Default for NDProximityDetectionType {
    fn default() -> Self {
        Self(0)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Abi for NDProximityDetectionType {
    type Abi = Self;
}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for NDProximityDetectionType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NDProximityDetectionType").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for NDProximityDetectionType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Protection.PlayReady.NDProximityDetectionType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_Protection_PlayReady\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NDStartAsyncOptions(pub i32);
#[cfg(feature = "deprecated")]
impl NDStartAsyncOptions {
    pub const MutualAuthentication: Self = Self(1i32);
    pub const WaitForLicenseDescriptor: Self = Self(2i32);
}
#[cfg(feature = "deprecated")]
impl ::core::marker::Copy for NDStartAsyncOptions {}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for NDStartAsyncOptions {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "deprecated")]
impl ::core::default::Default for NDStartAsyncOptions {
    fn default() -> Self {
        Self(0)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Abi for NDStartAsyncOptions {
    type Abi = Self;
}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for NDStartAsyncOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NDStartAsyncOptions").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for NDStartAsyncOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Protection.PlayReady.NDStartAsyncOptions;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_Protection_PlayReady\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PlayReadyDecryptorSetup(pub i32);
impl PlayReadyDecryptorSetup {
    pub const Uninitialized: Self = Self(0i32);
    pub const OnDemand: Self = Self(1i32);
}
impl ::core::marker::Copy for PlayReadyDecryptorSetup {}
impl ::core::clone::Clone for PlayReadyDecryptorSetup {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PlayReadyDecryptorSetup {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PlayReadyDecryptorSetup {
    type Abi = Self;
}
impl ::core::fmt::Debug for PlayReadyDecryptorSetup {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PlayReadyDecryptorSetup").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PlayReadyDecryptorSetup {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Protection.PlayReady.PlayReadyDecryptorSetup;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_Protection_PlayReady\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PlayReadyEncryptionAlgorithm(pub i32);
impl PlayReadyEncryptionAlgorithm {
    pub const Unprotected: Self = Self(0i32);
    pub const Aes128Ctr: Self = Self(1i32);
    pub const Cocktail: Self = Self(4i32);
    pub const Aes128Cbc: Self = Self(5i32);
    pub const Unspecified: Self = Self(65535i32);
    pub const Uninitialized: Self = Self(2147483647i32);
}
impl ::core::marker::Copy for PlayReadyEncryptionAlgorithm {}
impl ::core::clone::Clone for PlayReadyEncryptionAlgorithm {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PlayReadyEncryptionAlgorithm {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PlayReadyEncryptionAlgorithm {
    type Abi = Self;
}
impl ::core::fmt::Debug for PlayReadyEncryptionAlgorithm {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PlayReadyEncryptionAlgorithm").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PlayReadyEncryptionAlgorithm {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Protection.PlayReady.PlayReadyEncryptionAlgorithm;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_Protection_PlayReady\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PlayReadyHardwareDRMFeatures(pub i32);
impl PlayReadyHardwareDRMFeatures {
    pub const HardwareDRM: Self = Self(1i32);
    pub const HEVC: Self = Self(2i32);
    pub const Aes128Cbc: Self = Self(3i32);
}
impl ::core::marker::Copy for PlayReadyHardwareDRMFeatures {}
impl ::core::clone::Clone for PlayReadyHardwareDRMFeatures {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PlayReadyHardwareDRMFeatures {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PlayReadyHardwareDRMFeatures {
    type Abi = Self;
}
impl ::core::fmt::Debug for PlayReadyHardwareDRMFeatures {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PlayReadyHardwareDRMFeatures").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PlayReadyHardwareDRMFeatures {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Protection.PlayReady.PlayReadyHardwareDRMFeatures;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_Protection_PlayReady\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PlayReadyITADataFormat(pub i32);
impl PlayReadyITADataFormat {
    pub const SerializedProperties: Self = Self(0i32);
    pub const SerializedProperties_WithContentProtectionWrapper: Self = Self(1i32);
}
impl ::core::marker::Copy for PlayReadyITADataFormat {}
impl ::core::clone::Clone for PlayReadyITADataFormat {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PlayReadyITADataFormat {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PlayReadyITADataFormat {
    type Abi = Self;
}
impl ::core::fmt::Debug for PlayReadyITADataFormat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PlayReadyITADataFormat").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PlayReadyITADataFormat {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Protection.PlayReady.PlayReadyITADataFormat;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
