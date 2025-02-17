#[doc(hidden)]
#[repr(transparent)]
pub struct IBluetoothLEAdvertisement(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IBluetoothLEAdvertisement {
    type Vtable = IBluetoothLEAdvertisement_Vtbl;
}
unsafe impl ::windows::core::Interface for IBluetoothLEAdvertisement {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x066fb2b7_33d1_4e7d_8367_cf81d0f79653);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEAdvertisement_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub Flags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Flags: usize,
    #[cfg(feature = "Foundation")]
    pub SetFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetFlags: usize,
    pub LocalName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetLocalName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub ServiceUuids: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ServiceUuids: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ManufacturerData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ManufacturerData: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub DataSections: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    DataSections: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetManufacturerDataByCompanyId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, companyid: u16, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetManufacturerDataByCompanyId: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetSectionsByType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: u8, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetSectionsByType: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBluetoothLEAdvertisementBytePattern(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IBluetoothLEAdvertisementBytePattern {
    type Vtable = IBluetoothLEAdvertisementBytePattern_Vtbl;
}
unsafe impl ::windows::core::Interface for IBluetoothLEAdvertisementBytePattern {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfbfad7f2_b9c5_4a08_bc51_502f8ef68a79);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEAdvertisementBytePattern_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub DataType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
    pub SetDataType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u8) -> ::windows::core::HRESULT,
    pub Offset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i16) -> ::windows::core::HRESULT,
    pub SetOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub Data: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Data: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetData: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBluetoothLEAdvertisementBytePatternFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IBluetoothLEAdvertisementBytePatternFactory {
    type Vtable = IBluetoothLEAdvertisementBytePatternFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IBluetoothLEAdvertisementBytePatternFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc2e24d73_fd5c_4ec3_be2a_9ca6fa11b7bd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEAdvertisementBytePatternFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, datatype: u8, offset: i16, data: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Create: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBluetoothLEAdvertisementDataSection(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IBluetoothLEAdvertisementDataSection {
    type Vtable = IBluetoothLEAdvertisementDataSection_Vtbl;
}
unsafe impl ::windows::core::Interface for IBluetoothLEAdvertisementDataSection {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd7213314_3a43_40f9_b6f0_92bfefc34ae3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEAdvertisementDataSection_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub DataType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
    pub SetDataType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u8) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub Data: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Data: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetData: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBluetoothLEAdvertisementDataSectionFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IBluetoothLEAdvertisementDataSectionFactory {
    type Vtable = IBluetoothLEAdvertisementDataSectionFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IBluetoothLEAdvertisementDataSectionFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe7a40942_a845_4045_bf7e_3e9971db8a6b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEAdvertisementDataSectionFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, datatype: u8, data: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Create: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBluetoothLEAdvertisementDataTypesStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IBluetoothLEAdvertisementDataTypesStatics {
    type Vtable = IBluetoothLEAdvertisementDataTypesStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IBluetoothLEAdvertisementDataTypesStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3bb6472f_0606_434b_a76e_74159f0684d3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEAdvertisementDataTypesStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Flags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
    pub IncompleteService16BitUuids: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
    pub CompleteService16BitUuids: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
    pub IncompleteService32BitUuids: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
    pub CompleteService32BitUuids: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
    pub IncompleteService128BitUuids: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
    pub CompleteService128BitUuids: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
    pub ShortenedLocalName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
    pub CompleteLocalName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
    pub TxPowerLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
    pub PeripheralConnectionIntervalRange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
    pub ServiceSolicitation16BitUuids: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
    pub ServiceSolicitation32BitUuids: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
    pub ServiceSolicitation128BitUuids: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
    pub ServiceData16BitUuids: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
    pub ServiceData32BitUuids: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
    pub ServiceData128BitUuids: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
    pub PublicTargetAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
    pub RandomTargetAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
    pub Appearance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
    pub AdvertisingInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
    pub ManufacturerSpecificData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBluetoothLEAdvertisementFilter(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IBluetoothLEAdvertisementFilter {
    type Vtable = IBluetoothLEAdvertisementFilter_Vtbl;
}
unsafe impl ::windows::core::Interface for IBluetoothLEAdvertisementFilter {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x131eb0d3_d04e_47b1_837e_49405bf6f80f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEAdvertisementFilter_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Advertisement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetAdvertisement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub BytePatterns: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    BytePatterns: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBluetoothLEAdvertisementPublisher(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IBluetoothLEAdvertisementPublisher {
    type Vtable = IBluetoothLEAdvertisementPublisher_Vtbl;
}
unsafe impl ::windows::core::Interface for IBluetoothLEAdvertisementPublisher {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcde820f9_d9fa_43d6_a264_ddd8b7da8b78);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEAdvertisementPublisher_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut BluetoothLEAdvertisementPublisherStatus) -> ::windows::core::HRESULT,
    pub Advertisement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub StatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StatusChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveStatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStatusChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBluetoothLEAdvertisementPublisher2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IBluetoothLEAdvertisementPublisher2 {
    type Vtable = IBluetoothLEAdvertisementPublisher2_Vtbl;
}
unsafe impl ::windows::core::Interface for IBluetoothLEAdvertisementPublisher2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfbdb545e_56f1_510f_a434_217fbd9e7bd2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEAdvertisementPublisher2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub PreferredTransmitPowerLevelInDBm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PreferredTransmitPowerLevelInDBm: usize,
    #[cfg(feature = "Foundation")]
    pub SetPreferredTransmitPowerLevelInDBm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetPreferredTransmitPowerLevelInDBm: usize,
    pub UseExtendedAdvertisement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetUseExtendedAdvertisement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub IsAnonymous: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsAnonymous: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub IncludeTransmitPowerLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIncludeTransmitPowerLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBluetoothLEAdvertisementPublisherFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IBluetoothLEAdvertisementPublisherFactory {
    type Vtable = IBluetoothLEAdvertisementPublisherFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IBluetoothLEAdvertisementPublisherFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5c5f065e_b863_4981_a1af_1c544d8b0c0d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEAdvertisementPublisherFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, advertisement: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBluetoothLEAdvertisementPublisherStatusChangedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IBluetoothLEAdvertisementPublisherStatusChangedEventArgs {
    type Vtable = IBluetoothLEAdvertisementPublisherStatusChangedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IBluetoothLEAdvertisementPublisherStatusChangedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x09c2bd9f_2dff_4b23_86ee_0d14fb94aeae);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEAdvertisementPublisherStatusChangedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut BluetoothLEAdvertisementPublisherStatus) -> ::windows::core::HRESULT,
    pub Error: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::BluetoothError) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBluetoothLEAdvertisementPublisherStatusChangedEventArgs2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IBluetoothLEAdvertisementPublisherStatusChangedEventArgs2 {
    type Vtable = IBluetoothLEAdvertisementPublisherStatusChangedEventArgs2_Vtbl;
}
unsafe impl ::windows::core::Interface for IBluetoothLEAdvertisementPublisherStatusChangedEventArgs2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8f62790e_dc88_5c8b_b34e_10b321850f88);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEAdvertisementPublisherStatusChangedEventArgs2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub SelectedTransmitPowerLevelInDBm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SelectedTransmitPowerLevelInDBm: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBluetoothLEAdvertisementReceivedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IBluetoothLEAdvertisementReceivedEventArgs {
    type Vtable = IBluetoothLEAdvertisementReceivedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IBluetoothLEAdvertisementReceivedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x27987ddf_e596_41be_8d43_9e6731d4a913);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEAdvertisementReceivedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub RawSignalStrengthInDBm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i16) -> ::windows::core::HRESULT,
    pub BluetoothAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT,
    pub AdvertisementType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut BluetoothLEAdvertisementType) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Timestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Timestamp: usize,
    pub Advertisement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBluetoothLEAdvertisementReceivedEventArgs2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IBluetoothLEAdvertisementReceivedEventArgs2 {
    type Vtable = IBluetoothLEAdvertisementReceivedEventArgs2_Vtbl;
}
unsafe impl ::windows::core::Interface for IBluetoothLEAdvertisementReceivedEventArgs2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x12d9c87b_0399_5f0e_a348_53b02b6b162e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEAdvertisementReceivedEventArgs2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub BluetoothAddressType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::BluetoothAddressType) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub TransmitPowerLevelInDBm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TransmitPowerLevelInDBm: usize,
    pub IsAnonymous: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsConnectable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsScannable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsDirected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsScanResponse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBluetoothLEAdvertisementWatcher(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IBluetoothLEAdvertisementWatcher {
    type Vtable = IBluetoothLEAdvertisementWatcher_Vtbl;
}
unsafe impl ::windows::core::Interface for IBluetoothLEAdvertisementWatcher {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa6ac336f_f3d3_4297_8d6c_c81ea6623f40);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEAdvertisementWatcher_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub MinSamplingInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MinSamplingInterval: usize,
    #[cfg(feature = "Foundation")]
    pub MaxSamplingInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaxSamplingInterval: usize,
    #[cfg(feature = "Foundation")]
    pub MinOutOfRangeTimeout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MinOutOfRangeTimeout: usize,
    #[cfg(feature = "Foundation")]
    pub MaxOutOfRangeTimeout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaxOutOfRangeTimeout: usize,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut BluetoothLEAdvertisementWatcherStatus) -> ::windows::core::HRESULT,
    pub ScanningMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut BluetoothLEScanningMode) -> ::windows::core::HRESULT,
    pub SetScanningMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: BluetoothLEScanningMode) -> ::windows::core::HRESULT,
    pub SignalStrengthFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetSignalStrengthFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AdvertisementFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetAdvertisementFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Received: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Received: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveReceived: usize,
    #[cfg(feature = "Foundation")]
    pub Stopped: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Stopped: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveStopped: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStopped: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBluetoothLEAdvertisementWatcher2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IBluetoothLEAdvertisementWatcher2 {
    type Vtable = IBluetoothLEAdvertisementWatcher2_Vtbl;
}
unsafe impl ::windows::core::Interface for IBluetoothLEAdvertisementWatcher2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x01bf26bc_b164_5805_90a3_e8a7997ff225);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEAdvertisementWatcher2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub AllowExtendedAdvertisements: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetAllowExtendedAdvertisements: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBluetoothLEAdvertisementWatcherFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IBluetoothLEAdvertisementWatcherFactory {
    type Vtable = IBluetoothLEAdvertisementWatcherFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IBluetoothLEAdvertisementWatcherFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9aaf2d56_39ac_453e_b32a_85c657e017f1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEAdvertisementWatcherFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, advertisementfilter: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBluetoothLEAdvertisementWatcherStoppedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IBluetoothLEAdvertisementWatcherStoppedEventArgs {
    type Vtable = IBluetoothLEAdvertisementWatcherStoppedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IBluetoothLEAdvertisementWatcherStoppedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdd40f84d_e7b9_43e3_9c04_0685d085fd8c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEAdvertisementWatcherStoppedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Error: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::BluetoothError) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBluetoothLEManufacturerData(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IBluetoothLEManufacturerData {
    type Vtable = IBluetoothLEManufacturerData_Vtbl;
}
unsafe impl ::windows::core::Interface for IBluetoothLEManufacturerData {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x912dba18_6963_4533_b061_4694dafb34e5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEManufacturerData_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CompanyId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub SetCompanyId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub Data: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Data: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetData: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBluetoothLEManufacturerDataFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IBluetoothLEManufacturerDataFactory {
    type Vtable = IBluetoothLEManufacturerDataFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IBluetoothLEManufacturerDataFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc09b39f8_319a_441e_8de5_66a81e877a6c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEManufacturerDataFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, companyid: u16, data: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Create: usize,
}
#[doc = "*Required features: `\"Devices_Bluetooth_Advertisement\"`*"]
#[repr(transparent)]
pub struct BluetoothLEAdvertisement(::windows::core::IUnknown);
impl BluetoothLEAdvertisement {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<BluetoothLEAdvertisement, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Flags(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<BluetoothLEAdvertisementFlags>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Flags)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetFlags<P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::super::Foundation::IReference<BluetoothLEAdvertisementFlags>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetFlags)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    pub fn LocalName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LocalName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetLocalName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetLocalName)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ServiceUuids(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<::windows::core::GUID>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ServiceUuids)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ManufacturerData(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<BluetoothLEManufacturerData>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ManufacturerData)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn DataSections(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<BluetoothLEAdvertisementDataSection>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DataSections)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetManufacturerDataByCompanyId(&self, companyid: u16) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<BluetoothLEManufacturerData>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetManufacturerDataByCompanyId)(::windows::core::Vtable::as_raw(this), companyid, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetSectionsByType(&self, r#type: u8) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<BluetoothLEAdvertisementDataSection>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetSectionsByType)(::windows::core::Vtable::as_raw(this), r#type, result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for BluetoothLEAdvertisement {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BluetoothLEAdvertisement {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BluetoothLEAdvertisement {}
impl ::core::fmt::Debug for BluetoothLEAdvertisement {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BluetoothLEAdvertisement").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for BluetoothLEAdvertisement {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisement;{066fb2b7-33d1-4e7d-8367-cf81d0f79653})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for BluetoothLEAdvertisement {
    type Vtable = IBluetoothLEAdvertisement_Vtbl;
}
unsafe impl ::windows::core::Interface for BluetoothLEAdvertisement {
    const IID: ::windows::core::GUID = <IBluetoothLEAdvertisement as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for BluetoothLEAdvertisement {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisement";
}
::windows::core::interface_hierarchy!(BluetoothLEAdvertisement, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for BluetoothLEAdvertisement {}
unsafe impl ::core::marker::Sync for BluetoothLEAdvertisement {}
#[doc = "*Required features: `\"Devices_Bluetooth_Advertisement\"`*"]
#[repr(transparent)]
pub struct BluetoothLEAdvertisementBytePattern(::windows::core::IUnknown);
impl BluetoothLEAdvertisementBytePattern {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<BluetoothLEAdvertisementBytePattern, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn DataType(&self) -> ::windows::core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DataType)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetDataType(&self, value: u8) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetDataType)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn Offset(&self) -> ::windows::core::Result<i16> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Offset)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetOffset(&self, value: i16) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetOffset)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Data(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Data)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn SetData<P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::super::Storage::Streams::IBuffer>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetData)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Create<P0, E0>(datatype: u8, offset: i16, data: P0) -> ::windows::core::Result<BluetoothLEAdvertisementBytePattern>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::super::Storage::Streams::IBuffer>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::IBluetoothLEAdvertisementBytePatternFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Create)(::windows::core::Vtable::as_raw(this), datatype, offset, data.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IBluetoothLEAdvertisementBytePatternFactory<R, F: FnOnce(&IBluetoothLEAdvertisementBytePatternFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<BluetoothLEAdvertisementBytePattern, IBluetoothLEAdvertisementBytePatternFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for BluetoothLEAdvertisementBytePattern {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BluetoothLEAdvertisementBytePattern {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BluetoothLEAdvertisementBytePattern {}
impl ::core::fmt::Debug for BluetoothLEAdvertisementBytePattern {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BluetoothLEAdvertisementBytePattern").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for BluetoothLEAdvertisementBytePattern {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementBytePattern;{fbfad7f2-b9c5-4a08-bc51-502f8ef68a79})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for BluetoothLEAdvertisementBytePattern {
    type Vtable = IBluetoothLEAdvertisementBytePattern_Vtbl;
}
unsafe impl ::windows::core::Interface for BluetoothLEAdvertisementBytePattern {
    const IID: ::windows::core::GUID = <IBluetoothLEAdvertisementBytePattern as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for BluetoothLEAdvertisementBytePattern {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementBytePattern";
}
::windows::core::interface_hierarchy!(BluetoothLEAdvertisementBytePattern, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for BluetoothLEAdvertisementBytePattern {}
unsafe impl ::core::marker::Sync for BluetoothLEAdvertisementBytePattern {}
#[doc = "*Required features: `\"Devices_Bluetooth_Advertisement\"`*"]
#[repr(transparent)]
pub struct BluetoothLEAdvertisementDataSection(::windows::core::IUnknown);
impl BluetoothLEAdvertisementDataSection {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<BluetoothLEAdvertisementDataSection, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn DataType(&self) -> ::windows::core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DataType)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetDataType(&self, value: u8) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetDataType)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Data(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Data)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn SetData<P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::super::Storage::Streams::IBuffer>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetData)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Create<P0, E0>(datatype: u8, data: P0) -> ::windows::core::Result<BluetoothLEAdvertisementDataSection>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::super::Storage::Streams::IBuffer>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::IBluetoothLEAdvertisementDataSectionFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Create)(::windows::core::Vtable::as_raw(this), datatype, data.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IBluetoothLEAdvertisementDataSectionFactory<R, F: FnOnce(&IBluetoothLEAdvertisementDataSectionFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<BluetoothLEAdvertisementDataSection, IBluetoothLEAdvertisementDataSectionFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for BluetoothLEAdvertisementDataSection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BluetoothLEAdvertisementDataSection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BluetoothLEAdvertisementDataSection {}
impl ::core::fmt::Debug for BluetoothLEAdvertisementDataSection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BluetoothLEAdvertisementDataSection").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for BluetoothLEAdvertisementDataSection {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementDataSection;{d7213314-3a43-40f9-b6f0-92bfefc34ae3})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for BluetoothLEAdvertisementDataSection {
    type Vtable = IBluetoothLEAdvertisementDataSection_Vtbl;
}
unsafe impl ::windows::core::Interface for BluetoothLEAdvertisementDataSection {
    const IID: ::windows::core::GUID = <IBluetoothLEAdvertisementDataSection as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for BluetoothLEAdvertisementDataSection {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementDataSection";
}
::windows::core::interface_hierarchy!(BluetoothLEAdvertisementDataSection, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for BluetoothLEAdvertisementDataSection {}
unsafe impl ::core::marker::Sync for BluetoothLEAdvertisementDataSection {}
#[doc = "*Required features: `\"Devices_Bluetooth_Advertisement\"`*"]
pub struct BluetoothLEAdvertisementDataTypes;
impl BluetoothLEAdvertisementDataTypes {
    pub fn Flags() -> ::windows::core::Result<u8> {
        Self::IBluetoothLEAdvertisementDataTypesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Flags)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn IncompleteService16BitUuids() -> ::windows::core::Result<u8> {
        Self::IBluetoothLEAdvertisementDataTypesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IncompleteService16BitUuids)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn CompleteService16BitUuids() -> ::windows::core::Result<u8> {
        Self::IBluetoothLEAdvertisementDataTypesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CompleteService16BitUuids)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn IncompleteService32BitUuids() -> ::windows::core::Result<u8> {
        Self::IBluetoothLEAdvertisementDataTypesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IncompleteService32BitUuids)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn CompleteService32BitUuids() -> ::windows::core::Result<u8> {
        Self::IBluetoothLEAdvertisementDataTypesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CompleteService32BitUuids)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn IncompleteService128BitUuids() -> ::windows::core::Result<u8> {
        Self::IBluetoothLEAdvertisementDataTypesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IncompleteService128BitUuids)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn CompleteService128BitUuids() -> ::windows::core::Result<u8> {
        Self::IBluetoothLEAdvertisementDataTypesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CompleteService128BitUuids)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn ShortenedLocalName() -> ::windows::core::Result<u8> {
        Self::IBluetoothLEAdvertisementDataTypesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ShortenedLocalName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn CompleteLocalName() -> ::windows::core::Result<u8> {
        Self::IBluetoothLEAdvertisementDataTypesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CompleteLocalName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn TxPowerLevel() -> ::windows::core::Result<u8> {
        Self::IBluetoothLEAdvertisementDataTypesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TxPowerLevel)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn PeripheralConnectionIntervalRange() -> ::windows::core::Result<u8> {
        Self::IBluetoothLEAdvertisementDataTypesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PeripheralConnectionIntervalRange)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn ServiceSolicitation16BitUuids() -> ::windows::core::Result<u8> {
        Self::IBluetoothLEAdvertisementDataTypesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ServiceSolicitation16BitUuids)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn ServiceSolicitation32BitUuids() -> ::windows::core::Result<u8> {
        Self::IBluetoothLEAdvertisementDataTypesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ServiceSolicitation32BitUuids)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn ServiceSolicitation128BitUuids() -> ::windows::core::Result<u8> {
        Self::IBluetoothLEAdvertisementDataTypesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ServiceSolicitation128BitUuids)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn ServiceData16BitUuids() -> ::windows::core::Result<u8> {
        Self::IBluetoothLEAdvertisementDataTypesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ServiceData16BitUuids)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn ServiceData32BitUuids() -> ::windows::core::Result<u8> {
        Self::IBluetoothLEAdvertisementDataTypesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ServiceData32BitUuids)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn ServiceData128BitUuids() -> ::windows::core::Result<u8> {
        Self::IBluetoothLEAdvertisementDataTypesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ServiceData128BitUuids)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn PublicTargetAddress() -> ::windows::core::Result<u8> {
        Self::IBluetoothLEAdvertisementDataTypesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PublicTargetAddress)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn RandomTargetAddress() -> ::windows::core::Result<u8> {
        Self::IBluetoothLEAdvertisementDataTypesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RandomTargetAddress)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn Appearance() -> ::windows::core::Result<u8> {
        Self::IBluetoothLEAdvertisementDataTypesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Appearance)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn AdvertisingInterval() -> ::windows::core::Result<u8> {
        Self::IBluetoothLEAdvertisementDataTypesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AdvertisingInterval)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn ManufacturerSpecificData() -> ::windows::core::Result<u8> {
        Self::IBluetoothLEAdvertisementDataTypesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ManufacturerSpecificData)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IBluetoothLEAdvertisementDataTypesStatics<R, F: FnOnce(&IBluetoothLEAdvertisementDataTypesStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<BluetoothLEAdvertisementDataTypes, IBluetoothLEAdvertisementDataTypesStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for BluetoothLEAdvertisementDataTypes {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementDataTypes";
}
#[doc = "*Required features: `\"Devices_Bluetooth_Advertisement\"`*"]
#[repr(transparent)]
pub struct BluetoothLEAdvertisementFilter(::windows::core::IUnknown);
impl BluetoothLEAdvertisementFilter {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<BluetoothLEAdvertisementFilter, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Advertisement(&self) -> ::windows::core::Result<BluetoothLEAdvertisement> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Advertisement)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetAdvertisement(&self, value: &BluetoothLEAdvertisement) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetAdvertisement)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn BytePatterns(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<BluetoothLEAdvertisementBytePattern>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BytePatterns)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for BluetoothLEAdvertisementFilter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BluetoothLEAdvertisementFilter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BluetoothLEAdvertisementFilter {}
impl ::core::fmt::Debug for BluetoothLEAdvertisementFilter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BluetoothLEAdvertisementFilter").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for BluetoothLEAdvertisementFilter {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementFilter;{131eb0d3-d04e-47b1-837e-49405bf6f80f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for BluetoothLEAdvertisementFilter {
    type Vtable = IBluetoothLEAdvertisementFilter_Vtbl;
}
unsafe impl ::windows::core::Interface for BluetoothLEAdvertisementFilter {
    const IID: ::windows::core::GUID = <IBluetoothLEAdvertisementFilter as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for BluetoothLEAdvertisementFilter {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementFilter";
}
::windows::core::interface_hierarchy!(BluetoothLEAdvertisementFilter, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for BluetoothLEAdvertisementFilter {}
unsafe impl ::core::marker::Sync for BluetoothLEAdvertisementFilter {}
#[doc = "*Required features: `\"Devices_Bluetooth_Advertisement\"`*"]
#[repr(transparent)]
pub struct BluetoothLEAdvertisementPublisher(::windows::core::IUnknown);
impl BluetoothLEAdvertisementPublisher {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<BluetoothLEAdvertisementPublisher, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Status(&self) -> ::windows::core::Result<BluetoothLEAdvertisementPublisherStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Status)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Advertisement(&self) -> ::windows::core::Result<BluetoothLEAdvertisement> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Advertisement)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Start(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Start)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    pub fn Stop(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Stop)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StatusChanged(&self, handler: &super::super::super::Foundation::TypedEventHandler<BluetoothLEAdvertisementPublisher, BluetoothLEAdvertisementPublisherStatusChangedEventArgs>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).StatusChanged)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveStatusChanged(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveStatusChanged)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PreferredTransmitPowerLevelInDBm(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<i16>> {
        let this = &::windows::core::Interface::cast::<IBluetoothLEAdvertisementPublisher2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PreferredTransmitPowerLevelInDBm)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetPreferredTransmitPowerLevelInDBm<P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::super::Foundation::IReference<i16>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IBluetoothLEAdvertisementPublisher2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetPreferredTransmitPowerLevelInDBm)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    pub fn UseExtendedAdvertisement(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IBluetoothLEAdvertisementPublisher2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).UseExtendedAdvertisement)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetUseExtendedAdvertisement(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IBluetoothLEAdvertisementPublisher2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetUseExtendedAdvertisement)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn IsAnonymous(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IBluetoothLEAdvertisementPublisher2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsAnonymous)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetIsAnonymous(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IBluetoothLEAdvertisementPublisher2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetIsAnonymous)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn IncludeTransmitPowerLevel(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IBluetoothLEAdvertisementPublisher2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IncludeTransmitPowerLevel)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetIncludeTransmitPowerLevel(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IBluetoothLEAdvertisementPublisher2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetIncludeTransmitPowerLevel)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn Create(advertisement: &BluetoothLEAdvertisement) -> ::windows::core::Result<BluetoothLEAdvertisementPublisher> {
        Self::IBluetoothLEAdvertisementPublisherFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Create)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(advertisement), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IBluetoothLEAdvertisementPublisherFactory<R, F: FnOnce(&IBluetoothLEAdvertisementPublisherFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<BluetoothLEAdvertisementPublisher, IBluetoothLEAdvertisementPublisherFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for BluetoothLEAdvertisementPublisher {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BluetoothLEAdvertisementPublisher {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BluetoothLEAdvertisementPublisher {}
impl ::core::fmt::Debug for BluetoothLEAdvertisementPublisher {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BluetoothLEAdvertisementPublisher").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for BluetoothLEAdvertisementPublisher {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementPublisher;{cde820f9-d9fa-43d6-a264-ddd8b7da8b78})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for BluetoothLEAdvertisementPublisher {
    type Vtable = IBluetoothLEAdvertisementPublisher_Vtbl;
}
unsafe impl ::windows::core::Interface for BluetoothLEAdvertisementPublisher {
    const IID: ::windows::core::GUID = <IBluetoothLEAdvertisementPublisher as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for BluetoothLEAdvertisementPublisher {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementPublisher";
}
::windows::core::interface_hierarchy!(BluetoothLEAdvertisementPublisher, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for BluetoothLEAdvertisementPublisher {}
unsafe impl ::core::marker::Sync for BluetoothLEAdvertisementPublisher {}
#[doc = "*Required features: `\"Devices_Bluetooth_Advertisement\"`*"]
#[repr(transparent)]
pub struct BluetoothLEAdvertisementPublisherStatusChangedEventArgs(::windows::core::IUnknown);
impl BluetoothLEAdvertisementPublisherStatusChangedEventArgs {
    pub fn Status(&self) -> ::windows::core::Result<BluetoothLEAdvertisementPublisherStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Status)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Error(&self) -> ::windows::core::Result<super::BluetoothError> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Error)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SelectedTransmitPowerLevelInDBm(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<i16>> {
        let this = &::windows::core::Interface::cast::<IBluetoothLEAdvertisementPublisherStatusChangedEventArgs2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SelectedTransmitPowerLevelInDBm)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for BluetoothLEAdvertisementPublisherStatusChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BluetoothLEAdvertisementPublisherStatusChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BluetoothLEAdvertisementPublisherStatusChangedEventArgs {}
impl ::core::fmt::Debug for BluetoothLEAdvertisementPublisherStatusChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BluetoothLEAdvertisementPublisherStatusChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for BluetoothLEAdvertisementPublisherStatusChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementPublisherStatusChangedEventArgs;{09c2bd9f-2dff-4b23-86ee-0d14fb94aeae})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for BluetoothLEAdvertisementPublisherStatusChangedEventArgs {
    type Vtable = IBluetoothLEAdvertisementPublisherStatusChangedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for BluetoothLEAdvertisementPublisherStatusChangedEventArgs {
    const IID: ::windows::core::GUID = <IBluetoothLEAdvertisementPublisherStatusChangedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for BluetoothLEAdvertisementPublisherStatusChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementPublisherStatusChangedEventArgs";
}
::windows::core::interface_hierarchy!(BluetoothLEAdvertisementPublisherStatusChangedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for BluetoothLEAdvertisementPublisherStatusChangedEventArgs {}
unsafe impl ::core::marker::Sync for BluetoothLEAdvertisementPublisherStatusChangedEventArgs {}
#[doc = "*Required features: `\"Devices_Bluetooth_Advertisement\"`*"]
#[repr(transparent)]
pub struct BluetoothLEAdvertisementReceivedEventArgs(::windows::core::IUnknown);
impl BluetoothLEAdvertisementReceivedEventArgs {
    pub fn RawSignalStrengthInDBm(&self) -> ::windows::core::Result<i16> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RawSignalStrengthInDBm)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn BluetoothAddress(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BluetoothAddress)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn AdvertisementType(&self) -> ::windows::core::Result<BluetoothLEAdvertisementType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AdvertisementType)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows::core::Result<super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Timestamp)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Advertisement(&self) -> ::windows::core::Result<BluetoothLEAdvertisement> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Advertisement)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn BluetoothAddressType(&self) -> ::windows::core::Result<super::BluetoothAddressType> {
        let this = &::windows::core::Interface::cast::<IBluetoothLEAdvertisementReceivedEventArgs2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BluetoothAddressType)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TransmitPowerLevelInDBm(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<i16>> {
        let this = &::windows::core::Interface::cast::<IBluetoothLEAdvertisementReceivedEventArgs2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TransmitPowerLevelInDBm)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsAnonymous(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IBluetoothLEAdvertisementReceivedEventArgs2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsAnonymous)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsConnectable(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IBluetoothLEAdvertisementReceivedEventArgs2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsConnectable)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsScannable(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IBluetoothLEAdvertisementReceivedEventArgs2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsScannable)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsDirected(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IBluetoothLEAdvertisementReceivedEventArgs2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsDirected)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsScanResponse(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IBluetoothLEAdvertisementReceivedEventArgs2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsScanResponse)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for BluetoothLEAdvertisementReceivedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BluetoothLEAdvertisementReceivedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BluetoothLEAdvertisementReceivedEventArgs {}
impl ::core::fmt::Debug for BluetoothLEAdvertisementReceivedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BluetoothLEAdvertisementReceivedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for BluetoothLEAdvertisementReceivedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementReceivedEventArgs;{27987ddf-e596-41be-8d43-9e6731d4a913})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for BluetoothLEAdvertisementReceivedEventArgs {
    type Vtable = IBluetoothLEAdvertisementReceivedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for BluetoothLEAdvertisementReceivedEventArgs {
    const IID: ::windows::core::GUID = <IBluetoothLEAdvertisementReceivedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for BluetoothLEAdvertisementReceivedEventArgs {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementReceivedEventArgs";
}
::windows::core::interface_hierarchy!(BluetoothLEAdvertisementReceivedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for BluetoothLEAdvertisementReceivedEventArgs {}
unsafe impl ::core::marker::Sync for BluetoothLEAdvertisementReceivedEventArgs {}
#[doc = "*Required features: `\"Devices_Bluetooth_Advertisement\"`*"]
#[repr(transparent)]
pub struct BluetoothLEAdvertisementWatcher(::windows::core::IUnknown);
impl BluetoothLEAdvertisementWatcher {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<BluetoothLEAdvertisementWatcher, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MinSamplingInterval(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MinSamplingInterval)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MaxSamplingInterval(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MaxSamplingInterval)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MinOutOfRangeTimeout(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MinOutOfRangeTimeout)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MaxOutOfRangeTimeout(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MaxOutOfRangeTimeout)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Status(&self) -> ::windows::core::Result<BluetoothLEAdvertisementWatcherStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Status)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ScanningMode(&self) -> ::windows::core::Result<BluetoothLEScanningMode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ScanningMode)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetScanningMode(&self, value: BluetoothLEScanningMode) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetScanningMode)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn SignalStrengthFilter(&self) -> ::windows::core::Result<super::BluetoothSignalStrengthFilter> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SignalStrengthFilter)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetSignalStrengthFilter(&self, value: &super::BluetoothSignalStrengthFilter) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetSignalStrengthFilter)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn AdvertisementFilter(&self) -> ::windows::core::Result<BluetoothLEAdvertisementFilter> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AdvertisementFilter)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetAdvertisementFilter(&self, value: &BluetoothLEAdvertisementFilter) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetAdvertisementFilter)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Start(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Start)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    pub fn Stop(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Stop)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Received(&self, handler: &super::super::super::Foundation::TypedEventHandler<BluetoothLEAdvertisementWatcher, BluetoothLEAdvertisementReceivedEventArgs>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Received)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveReceived(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveReceived)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Stopped(&self, handler: &super::super::super::Foundation::TypedEventHandler<BluetoothLEAdvertisementWatcher, BluetoothLEAdvertisementWatcherStoppedEventArgs>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Stopped)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveStopped(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveStopped)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    pub fn AllowExtendedAdvertisements(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IBluetoothLEAdvertisementWatcher2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AllowExtendedAdvertisements)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetAllowExtendedAdvertisements(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IBluetoothLEAdvertisementWatcher2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetAllowExtendedAdvertisements)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn Create(advertisementfilter: &BluetoothLEAdvertisementFilter) -> ::windows::core::Result<BluetoothLEAdvertisementWatcher> {
        Self::IBluetoothLEAdvertisementWatcherFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Create)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(advertisementfilter), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IBluetoothLEAdvertisementWatcherFactory<R, F: FnOnce(&IBluetoothLEAdvertisementWatcherFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<BluetoothLEAdvertisementWatcher, IBluetoothLEAdvertisementWatcherFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for BluetoothLEAdvertisementWatcher {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BluetoothLEAdvertisementWatcher {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BluetoothLEAdvertisementWatcher {}
impl ::core::fmt::Debug for BluetoothLEAdvertisementWatcher {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BluetoothLEAdvertisementWatcher").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for BluetoothLEAdvertisementWatcher {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementWatcher;{a6ac336f-f3d3-4297-8d6c-c81ea6623f40})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for BluetoothLEAdvertisementWatcher {
    type Vtable = IBluetoothLEAdvertisementWatcher_Vtbl;
}
unsafe impl ::windows::core::Interface for BluetoothLEAdvertisementWatcher {
    const IID: ::windows::core::GUID = <IBluetoothLEAdvertisementWatcher as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for BluetoothLEAdvertisementWatcher {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementWatcher";
}
::windows::core::interface_hierarchy!(BluetoothLEAdvertisementWatcher, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for BluetoothLEAdvertisementWatcher {}
unsafe impl ::core::marker::Sync for BluetoothLEAdvertisementWatcher {}
#[doc = "*Required features: `\"Devices_Bluetooth_Advertisement\"`*"]
#[repr(transparent)]
pub struct BluetoothLEAdvertisementWatcherStoppedEventArgs(::windows::core::IUnknown);
impl BluetoothLEAdvertisementWatcherStoppedEventArgs {
    pub fn Error(&self) -> ::windows::core::Result<super::BluetoothError> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Error)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for BluetoothLEAdvertisementWatcherStoppedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BluetoothLEAdvertisementWatcherStoppedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BluetoothLEAdvertisementWatcherStoppedEventArgs {}
impl ::core::fmt::Debug for BluetoothLEAdvertisementWatcherStoppedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BluetoothLEAdvertisementWatcherStoppedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for BluetoothLEAdvertisementWatcherStoppedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementWatcherStoppedEventArgs;{dd40f84d-e7b9-43e3-9c04-0685d085fd8c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for BluetoothLEAdvertisementWatcherStoppedEventArgs {
    type Vtable = IBluetoothLEAdvertisementWatcherStoppedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for BluetoothLEAdvertisementWatcherStoppedEventArgs {
    const IID: ::windows::core::GUID = <IBluetoothLEAdvertisementWatcherStoppedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for BluetoothLEAdvertisementWatcherStoppedEventArgs {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementWatcherStoppedEventArgs";
}
::windows::core::interface_hierarchy!(BluetoothLEAdvertisementWatcherStoppedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for BluetoothLEAdvertisementWatcherStoppedEventArgs {}
unsafe impl ::core::marker::Sync for BluetoothLEAdvertisementWatcherStoppedEventArgs {}
#[doc = "*Required features: `\"Devices_Bluetooth_Advertisement\"`*"]
#[repr(transparent)]
pub struct BluetoothLEManufacturerData(::windows::core::IUnknown);
impl BluetoothLEManufacturerData {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<BluetoothLEManufacturerData, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn CompanyId(&self) -> ::windows::core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CompanyId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetCompanyId(&self, value: u16) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetCompanyId)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Data(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Data)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn SetData<P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::super::Storage::Streams::IBuffer>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetData)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Create<P0, E0>(companyid: u16, data: P0) -> ::windows::core::Result<BluetoothLEManufacturerData>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::super::Storage::Streams::IBuffer>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::IBluetoothLEManufacturerDataFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Create)(::windows::core::Vtable::as_raw(this), companyid, data.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IBluetoothLEManufacturerDataFactory<R, F: FnOnce(&IBluetoothLEManufacturerDataFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<BluetoothLEManufacturerData, IBluetoothLEManufacturerDataFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for BluetoothLEManufacturerData {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BluetoothLEManufacturerData {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BluetoothLEManufacturerData {}
impl ::core::fmt::Debug for BluetoothLEManufacturerData {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BluetoothLEManufacturerData").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for BluetoothLEManufacturerData {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Bluetooth.Advertisement.BluetoothLEManufacturerData;{912dba18-6963-4533-b061-4694dafb34e5})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for BluetoothLEManufacturerData {
    type Vtable = IBluetoothLEManufacturerData_Vtbl;
}
unsafe impl ::windows::core::Interface for BluetoothLEManufacturerData {
    const IID: ::windows::core::GUID = <IBluetoothLEManufacturerData as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for BluetoothLEManufacturerData {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Advertisement.BluetoothLEManufacturerData";
}
::windows::core::interface_hierarchy!(BluetoothLEManufacturerData, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for BluetoothLEManufacturerData {}
unsafe impl ::core::marker::Sync for BluetoothLEManufacturerData {}
#[doc = "*Required features: `\"Devices_Bluetooth_Advertisement\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct BluetoothLEAdvertisementFlags(pub u32);
impl BluetoothLEAdvertisementFlags {
    pub const None: Self = Self(0u32);
    pub const LimitedDiscoverableMode: Self = Self(1u32);
    pub const GeneralDiscoverableMode: Self = Self(2u32);
    pub const ClassicNotSupported: Self = Self(4u32);
    pub const DualModeControllerCapable: Self = Self(8u32);
    pub const DualModeHostCapable: Self = Self(16u32);
}
impl ::core::marker::Copy for BluetoothLEAdvertisementFlags {}
impl ::core::clone::Clone for BluetoothLEAdvertisementFlags {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for BluetoothLEAdvertisementFlags {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for BluetoothLEAdvertisementFlags {
    type Abi = Self;
}
impl ::core::fmt::Debug for BluetoothLEAdvertisementFlags {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BluetoothLEAdvertisementFlags").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for BluetoothLEAdvertisementFlags {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for BluetoothLEAdvertisementFlags {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for BluetoothLEAdvertisementFlags {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for BluetoothLEAdvertisementFlags {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for BluetoothLEAdvertisementFlags {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows::core::RuntimeType for BluetoothLEAdvertisementFlags {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementFlags;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_Bluetooth_Advertisement\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct BluetoothLEAdvertisementPublisherStatus(pub i32);
impl BluetoothLEAdvertisementPublisherStatus {
    pub const Created: Self = Self(0i32);
    pub const Waiting: Self = Self(1i32);
    pub const Started: Self = Self(2i32);
    pub const Stopping: Self = Self(3i32);
    pub const Stopped: Self = Self(4i32);
    pub const Aborted: Self = Self(5i32);
}
impl ::core::marker::Copy for BluetoothLEAdvertisementPublisherStatus {}
impl ::core::clone::Clone for BluetoothLEAdvertisementPublisherStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for BluetoothLEAdvertisementPublisherStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for BluetoothLEAdvertisementPublisherStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for BluetoothLEAdvertisementPublisherStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BluetoothLEAdvertisementPublisherStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for BluetoothLEAdvertisementPublisherStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementPublisherStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_Bluetooth_Advertisement\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct BluetoothLEAdvertisementType(pub i32);
impl BluetoothLEAdvertisementType {
    pub const ConnectableUndirected: Self = Self(0i32);
    pub const ConnectableDirected: Self = Self(1i32);
    pub const ScannableUndirected: Self = Self(2i32);
    pub const NonConnectableUndirected: Self = Self(3i32);
    pub const ScanResponse: Self = Self(4i32);
    pub const Extended: Self = Self(5i32);
}
impl ::core::marker::Copy for BluetoothLEAdvertisementType {}
impl ::core::clone::Clone for BluetoothLEAdvertisementType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for BluetoothLEAdvertisementType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for BluetoothLEAdvertisementType {
    type Abi = Self;
}
impl ::core::fmt::Debug for BluetoothLEAdvertisementType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BluetoothLEAdvertisementType").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for BluetoothLEAdvertisementType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_Bluetooth_Advertisement\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct BluetoothLEAdvertisementWatcherStatus(pub i32);
impl BluetoothLEAdvertisementWatcherStatus {
    pub const Created: Self = Self(0i32);
    pub const Started: Self = Self(1i32);
    pub const Stopping: Self = Self(2i32);
    pub const Stopped: Self = Self(3i32);
    pub const Aborted: Self = Self(4i32);
}
impl ::core::marker::Copy for BluetoothLEAdvertisementWatcherStatus {}
impl ::core::clone::Clone for BluetoothLEAdvertisementWatcherStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for BluetoothLEAdvertisementWatcherStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for BluetoothLEAdvertisementWatcherStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for BluetoothLEAdvertisementWatcherStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BluetoothLEAdvertisementWatcherStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for BluetoothLEAdvertisementWatcherStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementWatcherStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_Bluetooth_Advertisement\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct BluetoothLEScanningMode(pub i32);
impl BluetoothLEScanningMode {
    pub const Passive: Self = Self(0i32);
    pub const Active: Self = Self(1i32);
    pub const None: Self = Self(2i32);
}
impl ::core::marker::Copy for BluetoothLEScanningMode {}
impl ::core::clone::Clone for BluetoothLEScanningMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for BluetoothLEScanningMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for BluetoothLEScanningMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for BluetoothLEScanningMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BluetoothLEScanningMode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for BluetoothLEScanningMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Bluetooth.Advertisement.BluetoothLEScanningMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
