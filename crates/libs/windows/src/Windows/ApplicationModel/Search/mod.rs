#[cfg(feature = "ApplicationModel_Search_Core")]
pub mod Core;
#[doc(hidden)]
#[repr(transparent)]
pub struct ILocalContentSuggestionSettings(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ILocalContentSuggestionSettings {
    type Vtable = ILocalContentSuggestionSettings_Vtbl;
}
unsafe impl ::windows::core::Interface for ILocalContentSuggestionSettings {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeeaeb062_743d_456e_84a3_23f06f2d15d7);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILocalContentSuggestionSettings_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub SetEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub Enabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))]
    pub Locations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage")))]
    Locations: usize,
    pub SetAqsFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AqsFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub PropertiesToMatch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    PropertiesToMatch: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct ISearchPane(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Vtable for ISearchPane {
    type Vtable = ISearchPane_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for ISearchPane {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfdacec38_3700_4d73_91a1_2f998674238a);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct ISearchPane_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub SetSearchHistoryEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetSearchHistoryEnabled: usize,
    #[cfg(feature = "deprecated")]
    pub SearchHistoryEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SearchHistoryEnabled: usize,
    #[cfg(feature = "deprecated")]
    pub SetSearchHistoryContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetSearchHistoryContext: usize,
    #[cfg(feature = "deprecated")]
    pub SearchHistoryContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SearchHistoryContext: usize,
    #[cfg(feature = "deprecated")]
    pub SetPlaceholderText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetPlaceholderText: usize,
    #[cfg(feature = "deprecated")]
    pub PlaceholderText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    PlaceholderText: usize,
    #[cfg(feature = "deprecated")]
    pub QueryText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    QueryText: usize,
    #[cfg(feature = "deprecated")]
    pub Language: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Language: usize,
    #[cfg(feature = "deprecated")]
    pub Visible: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Visible: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub VisibilityChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    VisibilityChanged: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveVisibilityChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveVisibilityChanged: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub QueryChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    QueryChanged: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveQueryChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveQueryChanged: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub SuggestionsRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    SuggestionsRequested: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveSuggestionsRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveSuggestionsRequested: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub QuerySubmitted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    QuerySubmitted: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveQuerySubmitted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveQuerySubmitted: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub ResultSuggestionChosen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    ResultSuggestionChosen: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveResultSuggestionChosen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveResultSuggestionChosen: usize,
    #[cfg(feature = "deprecated")]
    pub SetLocalContentSuggestionSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, settings: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetLocalContentSuggestionSettings: usize,
    #[cfg(feature = "deprecated")]
    pub ShowOverloadDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ShowOverloadDefault: usize,
    #[cfg(feature = "deprecated")]
    pub ShowOverloadWithQuery: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, query: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ShowOverloadWithQuery: usize,
    #[cfg(feature = "deprecated")]
    pub SetShowOnKeyboardInput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetShowOnKeyboardInput: usize,
    #[cfg(feature = "deprecated")]
    pub ShowOnKeyboardInput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ShowOnKeyboardInput: usize,
    #[cfg(feature = "deprecated")]
    pub TrySetQueryText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, query: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    TrySetQueryText: usize,
}
#[doc = "*Required features: `\"ApplicationModel_Search\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct ISearchPaneQueryChangedEventArgs(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl ISearchPaneQueryChangedEventArgs {
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn QueryText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).QueryText)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Language)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn LinguisticDetails(&self) -> ::windows::core::Result<SearchPaneQueryLinguisticDetails> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LinguisticDetails)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
#[cfg(feature = "deprecated")]
::windows::core::interface_hierarchy!(ISearchPaneQueryChangedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for ISearchPaneQueryChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for ISearchPaneQueryChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for ISearchPaneQueryChangedEventArgs {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for ISearchPaneQueryChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISearchPaneQueryChangedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for ISearchPaneQueryChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{3c064fe9-2351-4248-a529-7110f464a785}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Vtable for ISearchPaneQueryChangedEventArgs {
    type Vtable = ISearchPaneQueryChangedEventArgs_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for ISearchPaneQueryChangedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3c064fe9_2351_4248_a529_7110f464a785);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct ISearchPaneQueryChangedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub QueryText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    QueryText: usize,
    #[cfg(feature = "deprecated")]
    pub Language: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Language: usize,
    #[cfg(feature = "deprecated")]
    pub LinguisticDetails: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    LinguisticDetails: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISearchPaneQueryLinguisticDetails(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ISearchPaneQueryLinguisticDetails {
    type Vtable = ISearchPaneQueryLinguisticDetails_Vtbl;
}
unsafe impl ::windows::core::Interface for ISearchPaneQueryLinguisticDetails {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x82fb460e_0940_4b6d_b8d0_642b30989e15);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISearchPaneQueryLinguisticDetails_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub QueryTextAlternatives: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    QueryTextAlternatives: usize,
    pub QueryTextCompositionStart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub QueryTextCompositionLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct ISearchPaneQuerySubmittedEventArgs(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Vtable for ISearchPaneQuerySubmittedEventArgs {
    type Vtable = ISearchPaneQuerySubmittedEventArgs_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for ISearchPaneQuerySubmittedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x143ba4fc_e9c5_4736_91b2_e8eb9cb88356);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct ISearchPaneQuerySubmittedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub QueryText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    QueryText: usize,
    #[cfg(feature = "deprecated")]
    pub Language: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Language: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct ISearchPaneQuerySubmittedEventArgsWithLinguisticDetails(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Vtable for ISearchPaneQuerySubmittedEventArgsWithLinguisticDetails {
    type Vtable = ISearchPaneQuerySubmittedEventArgsWithLinguisticDetails_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for ISearchPaneQuerySubmittedEventArgsWithLinguisticDetails {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x460c92e5_4c32_4538_a4d4_b6b4400d140f);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct ISearchPaneQuerySubmittedEventArgsWithLinguisticDetails_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub LinguisticDetails: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    LinguisticDetails: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct ISearchPaneResultSuggestionChosenEventArgs(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Vtable for ISearchPaneResultSuggestionChosenEventArgs {
    type Vtable = ISearchPaneResultSuggestionChosenEventArgs_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for ISearchPaneResultSuggestionChosenEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc8316cc0_aed2_41e0_bce0_c26ca74f85ec);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct ISearchPaneResultSuggestionChosenEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub Tag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Tag: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct ISearchPaneStatics(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Vtable for ISearchPaneStatics {
    type Vtable = ISearchPaneStatics_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for ISearchPaneStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9572adf1_8f1d_481f_a15b_c61655f16a0e);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct ISearchPaneStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub GetForCurrentView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    GetForCurrentView: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct ISearchPaneStaticsWithHideThisApplication(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Vtable for ISearchPaneStaticsWithHideThisApplication {
    type Vtable = ISearchPaneStaticsWithHideThisApplication_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for ISearchPaneStaticsWithHideThisApplication {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00732830_50f1_4d03_99ac_c6644c8ed8b5);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct ISearchPaneStaticsWithHideThisApplication_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub HideThisApplication: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    HideThisApplication: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct ISearchPaneSuggestionsRequest(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Vtable for ISearchPaneSuggestionsRequest {
    type Vtable = ISearchPaneSuggestionsRequest_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for ISearchPaneSuggestionsRequest {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x81b10b1c_e561_4093_9b4d_2ad482794a53);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct ISearchPaneSuggestionsRequest_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub IsCanceled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    IsCanceled: usize,
    #[cfg(feature = "deprecated")]
    pub SearchSuggestionCollection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SearchSuggestionCollection: usize,
    #[cfg(feature = "deprecated")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    GetDeferral: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct ISearchPaneSuggestionsRequestDeferral(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Vtable for ISearchPaneSuggestionsRequestDeferral {
    type Vtable = ISearchPaneSuggestionsRequestDeferral_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for ISearchPaneSuggestionsRequestDeferral {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa0d009f7_8748_4ee2_ad44_afa6be997c51);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct ISearchPaneSuggestionsRequestDeferral_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub Complete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Complete: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct ISearchPaneSuggestionsRequestedEventArgs(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Vtable for ISearchPaneSuggestionsRequestedEventArgs {
    type Vtable = ISearchPaneSuggestionsRequestedEventArgs_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for ISearchPaneSuggestionsRequestedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc89b8a2f_ac56_4460_8d2f_80023bec4fc5);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct ISearchPaneSuggestionsRequestedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub Request: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Request: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct ISearchPaneVisibilityChangedEventArgs(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Vtable for ISearchPaneVisibilityChangedEventArgs {
    type Vtable = ISearchPaneVisibilityChangedEventArgs_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for ISearchPaneVisibilityChangedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3c4d3046_ac4b_49f2_97d6_020e6182cb9c);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct ISearchPaneVisibilityChangedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub Visible: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Visible: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISearchQueryLinguisticDetails(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ISearchQueryLinguisticDetails {
    type Vtable = ISearchQueryLinguisticDetails_Vtbl;
}
unsafe impl ::windows::core::Interface for ISearchQueryLinguisticDetails {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x46a1205b_69c9_4745_b72f_a8a4fc8f24ae);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISearchQueryLinguisticDetails_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub QueryTextAlternatives: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    QueryTextAlternatives: usize,
    pub QueryTextCompositionStart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub QueryTextCompositionLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISearchQueryLinguisticDetailsFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ISearchQueryLinguisticDetailsFactory {
    type Vtable = ISearchQueryLinguisticDetailsFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for ISearchQueryLinguisticDetailsFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcac6c3b8_3c64_4dfd_ad9f_479e4d4065a4);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISearchQueryLinguisticDetailsFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, querytextalternatives: *mut ::core::ffi::c_void, querytextcompositionstart: u32, querytextcompositionlength: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateInstance: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISearchSuggestionCollection(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ISearchSuggestionCollection {
    type Vtable = ISearchSuggestionCollection_Vtbl;
}
unsafe impl ::windows::core::Interface for ISearchSuggestionCollection {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x323a8a4b_fbea_4446_abbc_3da7915fdd3a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISearchSuggestionCollection_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Size: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub AppendQuerySuggestion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, text: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub AppendQuerySuggestions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, suggestions: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AppendQuerySuggestions: usize,
    #[cfg(feature = "Storage_Streams")]
    pub AppendResultSuggestion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, text: *mut ::core::ffi::c_void, detailtext: *mut ::core::ffi::c_void, tag: *mut ::core::ffi::c_void, image: *mut ::core::ffi::c_void, imagealternatetext: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    AppendResultSuggestion: usize,
    pub AppendSearchSeparator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, label: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISearchSuggestionsRequest(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ISearchSuggestionsRequest {
    type Vtable = ISearchSuggestionsRequest_Vtbl;
}
unsafe impl ::windows::core::Interface for ISearchSuggestionsRequest {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4e4e26a7_44e5_4039_9099_6000ead1f0c6);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISearchSuggestionsRequest_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub IsCanceled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SearchSuggestionCollection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISearchSuggestionsRequestDeferral(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ISearchSuggestionsRequestDeferral {
    type Vtable = ISearchSuggestionsRequestDeferral_Vtbl;
}
unsafe impl ::windows::core::Interface for ISearchSuggestionsRequestDeferral {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb71598a9_c065_456d_a845_1eccec5dc28b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISearchSuggestionsRequestDeferral_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Complete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"ApplicationModel_Search\"`*"]
#[repr(transparent)]
pub struct LocalContentSuggestionSettings(::windows::core::IUnknown);
impl LocalContentSuggestionSettings {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<LocalContentSuggestionSettings, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn SetEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetEnabled)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn Enabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Enabled)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))]
    pub fn Locations(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::Storage::StorageFolder>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Locations)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetAqsFilter(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetAqsFilter)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn AqsFilter(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AqsFilter)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn PropertiesToMatch(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PropertiesToMatch)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for LocalContentSuggestionSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for LocalContentSuggestionSettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LocalContentSuggestionSettings {}
impl ::core::fmt::Debug for LocalContentSuggestionSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LocalContentSuggestionSettings").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for LocalContentSuggestionSettings {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Search.LocalContentSuggestionSettings;{eeaeb062-743d-456e-84a3-23f06f2d15d7})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for LocalContentSuggestionSettings {
    type Vtable = ILocalContentSuggestionSettings_Vtbl;
}
unsafe impl ::windows::core::Interface for LocalContentSuggestionSettings {
    const IID: ::windows::core::GUID = <ILocalContentSuggestionSettings as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for LocalContentSuggestionSettings {
    const NAME: &'static str = "Windows.ApplicationModel.Search.LocalContentSuggestionSettings";
}
::windows::core::interface_hierarchy!(LocalContentSuggestionSettings, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[doc = "*Required features: `\"ApplicationModel_Search\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct SearchPane(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl SearchPane {
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn SetSearchHistoryEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetSearchHistoryEnabled)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn SearchHistoryEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SearchHistoryEnabled)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn SetSearchHistoryContext(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetSearchHistoryContext)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn SearchHistoryContext(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SearchHistoryContext)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn SetPlaceholderText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetPlaceholderText)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn PlaceholderText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PlaceholderText)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn QueryText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).QueryText)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Language)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn Visible(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Visible)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn VisibilityChanged(&self, handler: &super::super::Foundation::TypedEventHandler<SearchPane, SearchPaneVisibilityChangedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).VisibilityChanged)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RemoveVisibilityChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveVisibilityChanged)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn QueryChanged(&self, handler: &super::super::Foundation::TypedEventHandler<SearchPane, SearchPaneQueryChangedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).QueryChanged)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RemoveQueryChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveQueryChanged)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn SuggestionsRequested(&self, handler: &super::super::Foundation::TypedEventHandler<SearchPane, SearchPaneSuggestionsRequestedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SuggestionsRequested)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RemoveSuggestionsRequested(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveSuggestionsRequested)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn QuerySubmitted(&self, handler: &super::super::Foundation::TypedEventHandler<SearchPane, SearchPaneQuerySubmittedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).QuerySubmitted)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RemoveQuerySubmitted(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveQuerySubmitted)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn ResultSuggestionChosen(&self, handler: &super::super::Foundation::TypedEventHandler<SearchPane, SearchPaneResultSuggestionChosenEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ResultSuggestionChosen)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RemoveResultSuggestionChosen(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveResultSuggestionChosen)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn SetLocalContentSuggestionSettings(&self, settings: &LocalContentSuggestionSettings) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetLocalContentSuggestionSettings)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(settings)).ok() }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn ShowOverloadDefault(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).ShowOverloadDefault)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn ShowOverloadWithQuery(&self, query: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).ShowOverloadWithQuery)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(query)).ok() }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn SetShowOnKeyboardInput(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetShowOnKeyboardInput)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn ShowOnKeyboardInput(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ShowOnKeyboardInput)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn TrySetQueryText(&self, query: &::windows::core::HSTRING) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TrySetQueryText)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(query), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn GetForCurrentView() -> ::windows::core::Result<SearchPane> {
        Self::ISearchPaneStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetForCurrentView)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn HideThisApplication() -> ::windows::core::Result<()> {
        Self::ISearchPaneStaticsWithHideThisApplication(|this| unsafe { (::windows::core::Vtable::vtable(this).HideThisApplication)(::windows::core::Vtable::as_raw(this)).ok() })
    }
    #[doc(hidden)]
    #[cfg(feature = "deprecated")]
    pub fn ISearchPaneStatics<R, F: FnOnce(&ISearchPaneStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<SearchPane, ISearchPaneStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    #[cfg(feature = "deprecated")]
    pub fn ISearchPaneStaticsWithHideThisApplication<R, F: FnOnce(&ISearchPaneStaticsWithHideThisApplication) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<SearchPane, ISearchPaneStaticsWithHideThisApplication> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for SearchPane {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for SearchPane {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for SearchPane {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for SearchPane {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SearchPane").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for SearchPane {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Search.SearchPane;{fdacec38-3700-4d73-91a1-2f998674238a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Vtable for SearchPane {
    type Vtable = ISearchPane_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for SearchPane {
    const IID: ::windows::core::GUID = <ISearchPane as ::windows::core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for SearchPane {
    const NAME: &'static str = "Windows.ApplicationModel.Search.SearchPane";
}
#[cfg(feature = "deprecated")]
::windows::core::interface_hierarchy!(SearchPane, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[doc = "*Required features: `\"ApplicationModel_Search\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct SearchPaneQueryChangedEventArgs(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl SearchPaneQueryChangedEventArgs {
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn QueryText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).QueryText)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Language)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn LinguisticDetails(&self) -> ::windows::core::Result<SearchPaneQueryLinguisticDetails> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LinguisticDetails)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for SearchPaneQueryChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for SearchPaneQueryChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for SearchPaneQueryChangedEventArgs {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for SearchPaneQueryChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SearchPaneQueryChangedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for SearchPaneQueryChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Search.SearchPaneQueryChangedEventArgs;{3c064fe9-2351-4248-a529-7110f464a785})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Vtable for SearchPaneQueryChangedEventArgs {
    type Vtable = ISearchPaneQueryChangedEventArgs_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for SearchPaneQueryChangedEventArgs {
    const IID: ::windows::core::GUID = <ISearchPaneQueryChangedEventArgs as ::windows::core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for SearchPaneQueryChangedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Search.SearchPaneQueryChangedEventArgs";
}
#[cfg(feature = "deprecated")]
::windows::core::interface_hierarchy!(SearchPaneQueryChangedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "deprecated")]
impl ::core::convert::TryFrom<SearchPaneQueryChangedEventArgs> for ISearchPaneQueryChangedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: SearchPaneQueryChangedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::TryFrom<&SearchPaneQueryChangedEventArgs> for ISearchPaneQueryChangedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &SearchPaneQueryChangedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::TryFrom<&SearchPaneQueryChangedEventArgs> for ::windows::core::InParam<ISearchPaneQueryChangedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &SearchPaneQueryChangedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for SearchPaneQueryChangedEventArgs {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for SearchPaneQueryChangedEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Search\"`*"]
#[repr(transparent)]
pub struct SearchPaneQueryLinguisticDetails(::windows::core::IUnknown);
impl SearchPaneQueryLinguisticDetails {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn QueryTextAlternatives(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).QueryTextAlternatives)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn QueryTextCompositionStart(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).QueryTextCompositionStart)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn QueryTextCompositionLength(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).QueryTextCompositionLength)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for SearchPaneQueryLinguisticDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SearchPaneQueryLinguisticDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SearchPaneQueryLinguisticDetails {}
impl ::core::fmt::Debug for SearchPaneQueryLinguisticDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SearchPaneQueryLinguisticDetails").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SearchPaneQueryLinguisticDetails {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Search.SearchPaneQueryLinguisticDetails;{82fb460e-0940-4b6d-b8d0-642b30989e15})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for SearchPaneQueryLinguisticDetails {
    type Vtable = ISearchPaneQueryLinguisticDetails_Vtbl;
}
unsafe impl ::windows::core::Interface for SearchPaneQueryLinguisticDetails {
    const IID: ::windows::core::GUID = <ISearchPaneQueryLinguisticDetails as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for SearchPaneQueryLinguisticDetails {
    const NAME: &'static str = "Windows.ApplicationModel.Search.SearchPaneQueryLinguisticDetails";
}
::windows::core::interface_hierarchy!(SearchPaneQueryLinguisticDetails, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for SearchPaneQueryLinguisticDetails {}
unsafe impl ::core::marker::Sync for SearchPaneQueryLinguisticDetails {}
#[doc = "*Required features: `\"ApplicationModel_Search\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct SearchPaneQuerySubmittedEventArgs(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl SearchPaneQuerySubmittedEventArgs {
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn QueryText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).QueryText)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Language)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn LinguisticDetails(&self) -> ::windows::core::Result<SearchPaneQueryLinguisticDetails> {
        let this = &::windows::core::Interface::cast::<ISearchPaneQuerySubmittedEventArgsWithLinguisticDetails>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LinguisticDetails)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for SearchPaneQuerySubmittedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for SearchPaneQuerySubmittedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for SearchPaneQuerySubmittedEventArgs {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for SearchPaneQuerySubmittedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SearchPaneQuerySubmittedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for SearchPaneQuerySubmittedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Search.SearchPaneQuerySubmittedEventArgs;{143ba4fc-e9c5-4736-91b2-e8eb9cb88356})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Vtable for SearchPaneQuerySubmittedEventArgs {
    type Vtable = ISearchPaneQuerySubmittedEventArgs_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for SearchPaneQuerySubmittedEventArgs {
    const IID: ::windows::core::GUID = <ISearchPaneQuerySubmittedEventArgs as ::windows::core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for SearchPaneQuerySubmittedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Search.SearchPaneQuerySubmittedEventArgs";
}
#[cfg(feature = "deprecated")]
::windows::core::interface_hierarchy!(SearchPaneQuerySubmittedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for SearchPaneQuerySubmittedEventArgs {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for SearchPaneQuerySubmittedEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Search\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct SearchPaneResultSuggestionChosenEventArgs(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl SearchPaneResultSuggestionChosenEventArgs {
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn Tag(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Tag)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for SearchPaneResultSuggestionChosenEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for SearchPaneResultSuggestionChosenEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for SearchPaneResultSuggestionChosenEventArgs {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for SearchPaneResultSuggestionChosenEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SearchPaneResultSuggestionChosenEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for SearchPaneResultSuggestionChosenEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Search.SearchPaneResultSuggestionChosenEventArgs;{c8316cc0-aed2-41e0-bce0-c26ca74f85ec})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Vtable for SearchPaneResultSuggestionChosenEventArgs {
    type Vtable = ISearchPaneResultSuggestionChosenEventArgs_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for SearchPaneResultSuggestionChosenEventArgs {
    const IID: ::windows::core::GUID = <ISearchPaneResultSuggestionChosenEventArgs as ::windows::core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for SearchPaneResultSuggestionChosenEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Search.SearchPaneResultSuggestionChosenEventArgs";
}
#[cfg(feature = "deprecated")]
::windows::core::interface_hierarchy!(SearchPaneResultSuggestionChosenEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for SearchPaneResultSuggestionChosenEventArgs {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for SearchPaneResultSuggestionChosenEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Search\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct SearchPaneSuggestionsRequest(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl SearchPaneSuggestionsRequest {
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn IsCanceled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsCanceled)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn SearchSuggestionCollection(&self) -> ::windows::core::Result<SearchSuggestionCollection> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SearchSuggestionCollection)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<SearchPaneSuggestionsRequestDeferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDeferral)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for SearchPaneSuggestionsRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for SearchPaneSuggestionsRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for SearchPaneSuggestionsRequest {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for SearchPaneSuggestionsRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SearchPaneSuggestionsRequest").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for SearchPaneSuggestionsRequest {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Search.SearchPaneSuggestionsRequest;{81b10b1c-e561-4093-9b4d-2ad482794a53})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Vtable for SearchPaneSuggestionsRequest {
    type Vtable = ISearchPaneSuggestionsRequest_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for SearchPaneSuggestionsRequest {
    const IID: ::windows::core::GUID = <ISearchPaneSuggestionsRequest as ::windows::core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for SearchPaneSuggestionsRequest {
    const NAME: &'static str = "Windows.ApplicationModel.Search.SearchPaneSuggestionsRequest";
}
#[cfg(feature = "deprecated")]
::windows::core::interface_hierarchy!(SearchPaneSuggestionsRequest, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for SearchPaneSuggestionsRequest {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for SearchPaneSuggestionsRequest {}
#[doc = "*Required features: `\"ApplicationModel_Search\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct SearchPaneSuggestionsRequestDeferral(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl SearchPaneSuggestionsRequestDeferral {
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn Complete(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Complete)(::windows::core::Vtable::as_raw(this)).ok() }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for SearchPaneSuggestionsRequestDeferral {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for SearchPaneSuggestionsRequestDeferral {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for SearchPaneSuggestionsRequestDeferral {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for SearchPaneSuggestionsRequestDeferral {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SearchPaneSuggestionsRequestDeferral").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for SearchPaneSuggestionsRequestDeferral {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Search.SearchPaneSuggestionsRequestDeferral;{a0d009f7-8748-4ee2-ad44-afa6be997c51})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Vtable for SearchPaneSuggestionsRequestDeferral {
    type Vtable = ISearchPaneSuggestionsRequestDeferral_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for SearchPaneSuggestionsRequestDeferral {
    const IID: ::windows::core::GUID = <ISearchPaneSuggestionsRequestDeferral as ::windows::core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for SearchPaneSuggestionsRequestDeferral {
    const NAME: &'static str = "Windows.ApplicationModel.Search.SearchPaneSuggestionsRequestDeferral";
}
#[cfg(feature = "deprecated")]
::windows::core::interface_hierarchy!(SearchPaneSuggestionsRequestDeferral, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for SearchPaneSuggestionsRequestDeferral {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for SearchPaneSuggestionsRequestDeferral {}
#[doc = "*Required features: `\"ApplicationModel_Search\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct SearchPaneSuggestionsRequestedEventArgs(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl SearchPaneSuggestionsRequestedEventArgs {
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn QueryText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ISearchPaneQueryChangedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).QueryText)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ISearchPaneQueryChangedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Language)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn LinguisticDetails(&self) -> ::windows::core::Result<SearchPaneQueryLinguisticDetails> {
        let this = &::windows::core::Interface::cast::<ISearchPaneQueryChangedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LinguisticDetails)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn Request(&self) -> ::windows::core::Result<SearchPaneSuggestionsRequest> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Request)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for SearchPaneSuggestionsRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for SearchPaneSuggestionsRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for SearchPaneSuggestionsRequestedEventArgs {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for SearchPaneSuggestionsRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SearchPaneSuggestionsRequestedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for SearchPaneSuggestionsRequestedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Search.SearchPaneSuggestionsRequestedEventArgs;{c89b8a2f-ac56-4460-8d2f-80023bec4fc5})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Vtable for SearchPaneSuggestionsRequestedEventArgs {
    type Vtable = ISearchPaneSuggestionsRequestedEventArgs_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for SearchPaneSuggestionsRequestedEventArgs {
    const IID: ::windows::core::GUID = <ISearchPaneSuggestionsRequestedEventArgs as ::windows::core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for SearchPaneSuggestionsRequestedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Search.SearchPaneSuggestionsRequestedEventArgs";
}
#[cfg(feature = "deprecated")]
::windows::core::interface_hierarchy!(SearchPaneSuggestionsRequestedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "deprecated")]
impl ::core::convert::TryFrom<SearchPaneSuggestionsRequestedEventArgs> for ISearchPaneQueryChangedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: SearchPaneSuggestionsRequestedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::TryFrom<&SearchPaneSuggestionsRequestedEventArgs> for ISearchPaneQueryChangedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &SearchPaneSuggestionsRequestedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::TryFrom<&SearchPaneSuggestionsRequestedEventArgs> for ::windows::core::InParam<ISearchPaneQueryChangedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &SearchPaneSuggestionsRequestedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for SearchPaneSuggestionsRequestedEventArgs {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for SearchPaneSuggestionsRequestedEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Search\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct SearchPaneVisibilityChangedEventArgs(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl SearchPaneVisibilityChangedEventArgs {
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn Visible(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Visible)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for SearchPaneVisibilityChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for SearchPaneVisibilityChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for SearchPaneVisibilityChangedEventArgs {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for SearchPaneVisibilityChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SearchPaneVisibilityChangedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for SearchPaneVisibilityChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Search.SearchPaneVisibilityChangedEventArgs;{3c4d3046-ac4b-49f2-97d6-020e6182cb9c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Vtable for SearchPaneVisibilityChangedEventArgs {
    type Vtable = ISearchPaneVisibilityChangedEventArgs_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for SearchPaneVisibilityChangedEventArgs {
    const IID: ::windows::core::GUID = <ISearchPaneVisibilityChangedEventArgs as ::windows::core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for SearchPaneVisibilityChangedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Search.SearchPaneVisibilityChangedEventArgs";
}
#[cfg(feature = "deprecated")]
::windows::core::interface_hierarchy!(SearchPaneVisibilityChangedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for SearchPaneVisibilityChangedEventArgs {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for SearchPaneVisibilityChangedEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Search\"`*"]
#[repr(transparent)]
pub struct SearchQueryLinguisticDetails(::windows::core::IUnknown);
impl SearchQueryLinguisticDetails {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn QueryTextAlternatives(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).QueryTextAlternatives)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn QueryTextCompositionStart(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).QueryTextCompositionStart)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn QueryTextCompositionLength(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).QueryTextCompositionLength)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateInstance<P0, E0>(querytextalternatives: P0, querytextcompositionstart: u32, querytextcompositionlength: u32) -> ::windows::core::Result<SearchQueryLinguisticDetails>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::ISearchQueryLinguisticDetailsFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateInstance)(::windows::core::Vtable::as_raw(this), querytextalternatives.try_into().map_err(|e| e.into())?.abi(), querytextcompositionstart, querytextcompositionlength, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISearchQueryLinguisticDetailsFactory<R, F: FnOnce(&ISearchQueryLinguisticDetailsFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<SearchQueryLinguisticDetails, ISearchQueryLinguisticDetailsFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for SearchQueryLinguisticDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SearchQueryLinguisticDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SearchQueryLinguisticDetails {}
impl ::core::fmt::Debug for SearchQueryLinguisticDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SearchQueryLinguisticDetails").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SearchQueryLinguisticDetails {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Search.SearchQueryLinguisticDetails;{46a1205b-69c9-4745-b72f-a8a4fc8f24ae})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for SearchQueryLinguisticDetails {
    type Vtable = ISearchQueryLinguisticDetails_Vtbl;
}
unsafe impl ::windows::core::Interface for SearchQueryLinguisticDetails {
    const IID: ::windows::core::GUID = <ISearchQueryLinguisticDetails as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for SearchQueryLinguisticDetails {
    const NAME: &'static str = "Windows.ApplicationModel.Search.SearchQueryLinguisticDetails";
}
::windows::core::interface_hierarchy!(SearchQueryLinguisticDetails, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for SearchQueryLinguisticDetails {}
unsafe impl ::core::marker::Sync for SearchQueryLinguisticDetails {}
#[doc = "*Required features: `\"ApplicationModel_Search\"`*"]
#[repr(transparent)]
pub struct SearchSuggestionCollection(::windows::core::IUnknown);
impl SearchSuggestionCollection {
    pub fn Size(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Size)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn AppendQuerySuggestion(&self, text: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).AppendQuerySuggestion)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(text)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn AppendQuerySuggestions<P0, E0>(&self, suggestions: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).AppendQuerySuggestions)(::windows::core::Vtable::as_raw(this), suggestions.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn AppendResultSuggestion<P0, E0>(&self, text: &::windows::core::HSTRING, detailtext: &::windows::core::HSTRING, tag: &::windows::core::HSTRING, image: P0, imagealternatetext: &::windows::core::HSTRING) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Storage::Streams::IRandomAccessStreamReference>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).AppendResultSuggestion)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(text), ::core::mem::transmute_copy(detailtext), ::core::mem::transmute_copy(tag), image.try_into().map_err(|e| e.into())?.abi(), ::core::mem::transmute_copy(imagealternatetext)).ok() }
    }
    pub fn AppendSearchSeparator(&self, label: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).AppendSearchSeparator)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(label)).ok() }
    }
}
impl ::core::clone::Clone for SearchSuggestionCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SearchSuggestionCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SearchSuggestionCollection {}
impl ::core::fmt::Debug for SearchSuggestionCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SearchSuggestionCollection").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SearchSuggestionCollection {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Search.SearchSuggestionCollection;{323a8a4b-fbea-4446-abbc-3da7915fdd3a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for SearchSuggestionCollection {
    type Vtable = ISearchSuggestionCollection_Vtbl;
}
unsafe impl ::windows::core::Interface for SearchSuggestionCollection {
    const IID: ::windows::core::GUID = <ISearchSuggestionCollection as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for SearchSuggestionCollection {
    const NAME: &'static str = "Windows.ApplicationModel.Search.SearchSuggestionCollection";
}
::windows::core::interface_hierarchy!(SearchSuggestionCollection, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for SearchSuggestionCollection {}
unsafe impl ::core::marker::Sync for SearchSuggestionCollection {}
#[doc = "*Required features: `\"ApplicationModel_Search\"`*"]
#[repr(transparent)]
pub struct SearchSuggestionsRequest(::windows::core::IUnknown);
impl SearchSuggestionsRequest {
    pub fn IsCanceled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsCanceled)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SearchSuggestionCollection(&self) -> ::windows::core::Result<SearchSuggestionCollection> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SearchSuggestionCollection)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetDeferral(&self) -> ::windows::core::Result<SearchSuggestionsRequestDeferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDeferral)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for SearchSuggestionsRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SearchSuggestionsRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SearchSuggestionsRequest {}
impl ::core::fmt::Debug for SearchSuggestionsRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SearchSuggestionsRequest").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SearchSuggestionsRequest {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Search.SearchSuggestionsRequest;{4e4e26a7-44e5-4039-9099-6000ead1f0c6})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for SearchSuggestionsRequest {
    type Vtable = ISearchSuggestionsRequest_Vtbl;
}
unsafe impl ::windows::core::Interface for SearchSuggestionsRequest {
    const IID: ::windows::core::GUID = <ISearchSuggestionsRequest as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for SearchSuggestionsRequest {
    const NAME: &'static str = "Windows.ApplicationModel.Search.SearchSuggestionsRequest";
}
::windows::core::interface_hierarchy!(SearchSuggestionsRequest, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for SearchSuggestionsRequest {}
unsafe impl ::core::marker::Sync for SearchSuggestionsRequest {}
#[doc = "*Required features: `\"ApplicationModel_Search\"`*"]
#[repr(transparent)]
pub struct SearchSuggestionsRequestDeferral(::windows::core::IUnknown);
impl SearchSuggestionsRequestDeferral {
    pub fn Complete(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Complete)(::windows::core::Vtable::as_raw(this)).ok() }
    }
}
impl ::core::clone::Clone for SearchSuggestionsRequestDeferral {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SearchSuggestionsRequestDeferral {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SearchSuggestionsRequestDeferral {}
impl ::core::fmt::Debug for SearchSuggestionsRequestDeferral {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SearchSuggestionsRequestDeferral").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SearchSuggestionsRequestDeferral {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Search.SearchSuggestionsRequestDeferral;{b71598a9-c065-456d-a845-1eccec5dc28b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for SearchSuggestionsRequestDeferral {
    type Vtable = ISearchSuggestionsRequestDeferral_Vtbl;
}
unsafe impl ::windows::core::Interface for SearchSuggestionsRequestDeferral {
    const IID: ::windows::core::GUID = <ISearchSuggestionsRequestDeferral as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for SearchSuggestionsRequestDeferral {
    const NAME: &'static str = "Windows.ApplicationModel.Search.SearchSuggestionsRequestDeferral";
}
::windows::core::interface_hierarchy!(SearchSuggestionsRequestDeferral, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for SearchSuggestionsRequestDeferral {}
unsafe impl ::core::marker::Sync for SearchSuggestionsRequestDeferral {}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
