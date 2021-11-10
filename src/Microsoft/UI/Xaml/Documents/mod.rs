#![allow(
    unused_variables,
    non_upper_case_globals,
    non_snake_case,
    unused_unsafe,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
#[doc = "*Required features: `UI_Xaml_Documents`*"]
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct Block(pub ::windows::runtime::IInspectable);
impl Block {
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn TextAlignment(&self) -> ::windows::runtime::Result<super::TextAlignment> {
        let this = self;
        unsafe {
            let mut result__: super::TextAlignment = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::TextAlignment>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetTextAlignment(&self, value: super::TextAlignment) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn HorizontalTextAlignment(&self) -> ::windows::runtime::Result<super::TextAlignment> {
        let this = self;
        unsafe {
            let mut result__: super::TextAlignment = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::TextAlignment>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetHorizontalTextAlignment(
        &self,
        value: super::TextAlignment,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn LineHeight(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetLineHeight(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).11)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn LineStackingStrategy(&self) -> ::windows::runtime::Result<super::LineStackingStrategy> {
        let this = self;
        unsafe {
            let mut result__: super::LineStackingStrategy = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::LineStackingStrategy>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetLineStackingStrategy(
        &self,
        value: super::LineStackingStrategy,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).13)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn Margin(&self) -> ::windows::runtime::Result<super::Thickness> {
        let this = self;
        unsafe {
            let mut result__: super::Thickness = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Thickness>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetMargin<'a, Param0: ::windows::runtime::IntoParam<'a, super::Thickness>>(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).15)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn TextAlignmentProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::IBlockStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn HorizontalTextAlignmentProperty() -> ::windows::runtime::Result<super::DependencyProperty>
    {
        Self::IBlockStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn LineHeightProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::IBlockStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn LineStackingStrategyProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::IBlockStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn MarginProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::IBlockStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn GetValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetValue<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>,
    >(
        &self,
        dp: Param0,
        value: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn ClearValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn ReadLocalValue<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn GetAnimationBaseValue<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn RegisterPropertyChangedCallback<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
        Param1: ::windows::runtime::IntoParam<'a, super::DependencyPropertyChangedCallback>,
    >(
        &self,
        dp: Param0,
        callback: Param1,
    ) -> ::windows::runtime::Result<i64> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: i64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                callback.into_param().abi(),
                &mut result__,
            )
            .from_abi::<i64>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn UnregisterPropertyChangedCallback<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
        token: i64,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).12)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                token,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn Dispatcher(&self) -> ::windows::runtime::Result<::windows::UI::Core::CoreDispatcher> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Core::CoreDispatcher>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Dispatching`*"]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::runtime::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn Name(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn FontSize(&self) -> ::windows::runtime::Result<f64> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetFontSize(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Media`*"]
    pub fn FontFamily(&self) -> ::windows::runtime::Result<super::Media::FontFamily> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::FontFamily>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Media`*"]
    pub fn SetFontFamily<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Media::FontFamily>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn FontWeight(&self) -> ::windows::runtime::Result<::windows::UI::Text::FontWeight> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::UI::Text::FontWeight = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::FontWeight>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetFontWeight<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::UI::Text::FontWeight>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).12)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn FontStyle(&self) -> ::windows::runtime::Result<::windows::UI::Text::FontStyle> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::UI::Text::FontStyle = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::FontStyle>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetFontStyle(
        &self,
        value: ::windows::UI::Text::FontStyle,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).14)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn FontStretch(&self) -> ::windows::runtime::Result<::windows::UI::Text::FontStretch> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::UI::Text::FontStretch = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::FontStretch>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetFontStretch(
        &self,
        value: ::windows::UI::Text::FontStretch,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).16)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn CharacterSpacing(&self) -> ::windows::runtime::Result<i32> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetCharacterSpacing(&self, value: i32) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).18)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Media`*"]
    pub fn Foreground(&self) -> ::windows::runtime::Result<super::Media::Brush> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Media`*"]
    pub fn SetForeground<'a, Param0: ::windows::runtime::IntoParam<'a, super::Media::Brush>>(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).20)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn Language(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).21)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetLanguage<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).22)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn IsTextScaleFactorEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).23)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetIsTextScaleFactorEnabled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).24)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn TextDecorations(
        &self,
    ) -> ::windows::runtime::Result<::windows::UI::Text::TextDecorations> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::UI::Text::TextDecorations = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).25)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::TextDecorations>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetTextDecorations(
        &self,
        value: ::windows::UI::Text::TextDecorations,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).26)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn ContentStart(&self) -> ::windows::runtime::Result<TextPointer> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).27)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn ContentEnd(&self) -> ::windows::runtime::Result<TextPointer> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).28)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn ElementStart(&self) -> ::windows::runtime::Result<TextPointer> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).29)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn ElementEnd(&self) -> ::windows::runtime::Result<TextPointer> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).30)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn AllowFocusOnInteraction(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).31)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetAllowFocusOnInteraction(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).32)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn AccessKey(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).33)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetAccessKey<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).34)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn ExitDisplayModeOnAccessKeyInvoked(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).35)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetExitDisplayModeOnAccessKeyInvoked(
        &self,
        value: bool,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).36)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn IsAccessKeyScope(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).37)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetIsAccessKeyScope(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).38)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn AccessKeyScopeOwner(&self) -> ::windows::runtime::Result<super::DependencyObject> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).39)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyObject>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetAccessKeyScopeOwner<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).40)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Input`*"]
    pub fn KeyTipPlacementMode(
        &self,
    ) -> ::windows::runtime::Result<super::Input::KeyTipPlacementMode> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: super::Input::KeyTipPlacementMode = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).41)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Input::KeyTipPlacementMode>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Input`*"]
    pub fn SetKeyTipPlacementMode(
        &self,
        value: super::Input::KeyTipPlacementMode,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).42)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn KeyTipHorizontalOffset(&self) -> ::windows::runtime::Result<f64> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).43)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetKeyTipHorizontalOffset(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).44)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn KeyTipVerticalOffset(&self) -> ::windows::runtime::Result<f64> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).45)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetKeyTipVerticalOffset(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).46)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn XamlRoot(&self) -> ::windows::runtime::Result<super::XamlRoot> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).47)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::XamlRoot>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetXamlRoot<'a, Param0: ::windows::runtime::IntoParam<'a, super::XamlRoot>>(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).48)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Input`*"]
    pub fn AccessKeyDisplayRequested<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                TextElement,
                super::Input::AccessKeyDisplayRequestedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).49)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn RemoveAccessKeyDisplayRequested<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).50)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Input`*"]
    pub fn AccessKeyDisplayDismissed<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                TextElement,
                super::Input::AccessKeyDisplayDismissedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).51)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn RemoveAccessKeyDisplayDismissed<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).52)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Input`*"]
    pub fn AccessKeyInvoked<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                TextElement,
                super::Input::AccessKeyInvokedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).53)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn RemoveAccessKeyInvoked<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).54)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn FindName<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(
        &self,
        name: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).55)(
                ::core::mem::transmute_copy(this),
                name.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    pub fn IBlockStatics<R, F: FnOnce(&IBlockStatics) -> ::windows::runtime::Result<R>>(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<Block, IBlockStatics> =
            ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for Block {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Xaml.Documents.Block;{8149d507-672f-5fd5-a10a-351389ba9659})",
    );
}
unsafe impl ::windows::runtime::Interface for Block {
    type Vtable = IBlock_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2169099527,
        26415,
        24533,
        [161, 10, 53, 19, 137, 186, 150, 89],
    );
}
impl ::windows::runtime::RuntimeName for Block {
    const NAME: &'static str = "Microsoft.UI.Xaml.Documents.Block";
}
impl ::core::convert::From<Block> for ::windows::runtime::IUnknown {
    fn from(value: Block) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&Block> for ::windows::runtime::IUnknown {
    fn from(value: &Block) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for Block {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a Block {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<Block> for ::windows::runtime::IInspectable {
    fn from(value: Block) -> Self {
        value.0
    }
}
impl ::core::convert::From<&Block> for ::windows::runtime::IInspectable {
    fn from(value: &Block) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for Block {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a Block {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<Block> for TextElement {
    fn from(value: Block) -> Self {
        ::core::convert::Into::<TextElement>::into(&value)
    }
}
impl ::core::convert::From<&Block> for TextElement {
    fn from(value: &Block) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, TextElement> for Block {
    fn into_param(self) -> ::windows::runtime::Param<'a, TextElement> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<TextElement>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, TextElement> for &Block {
    fn into_param(self) -> ::windows::runtime::Param<'a, TextElement> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<TextElement>::into(
            ::core::clone::Clone::clone(self),
        ))
    }
}
impl ::core::convert::From<Block> for super::DependencyObject {
    fn from(value: Block) -> Self {
        ::core::convert::Into::<super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&Block> for super::DependencyObject {
    fn from(value: &Block) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::DependencyObject> for Block {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::DependencyObject> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(
            self,
        ))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::DependencyObject> for &Block {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::DependencyObject> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(
            ::core::clone::Clone::clone(self),
        ))
    }
}
unsafe impl ::core::marker::Send for Block {}
unsafe impl ::core::marker::Sync for Block {}
#[doc = "*Required features: `UI_Xaml_Documents`*"]
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct BlockCollection(pub ::windows::runtime::IInspectable);
impl BlockCollection {
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn GetAt(&self, index: u32) -> ::windows::runtime::Result<Block> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                index,
                &mut result__,
            )
            .from_abi::<Block>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn Size(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn GetView(
        &self,
    ) -> ::windows::runtime::Result<::windows::Foundation::Collections::IVectorView<Block>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Collections::IVectorView<Block>>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn IndexOf<'a, Param0: ::windows::runtime::IntoParam<'a, Block>>(
        &self,
        value: Param0,
        index: &mut u32,
    ) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
                index,
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetAt<'a, Param1: ::windows::runtime::IntoParam<'a, Block>>(
        &self,
        index: u32,
        value: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                index,
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn InsertAt<'a, Param1: ::windows::runtime::IntoParam<'a, Block>>(
        &self,
        index: u32,
        value: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).11)(
                ::core::mem::transmute_copy(this),
                index,
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn RemoveAt(&self, index: u32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).12)(
                ::core::mem::transmute_copy(this),
                index,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn Append<'a, Param0: ::windows::runtime::IntoParam<'a, Block>>(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).13)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn RemoveAtEnd(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).14)(::core::mem::transmute_copy(this)).ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn Clear(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).15)(::core::mem::transmute_copy(this)).ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn GetMany(
        &self,
        startindex: u32,
        items: &mut [<Block as ::windows::runtime::DefaultType>::DefaultType],
    ) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(
                ::core::mem::transmute_copy(this),
                startindex,
                items.len() as u32,
                ::core::mem::transmute_copy(&items),
                &mut result__,
            )
            .from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn ReplaceAll(
        &self,
        items: &[<Block as ::windows::runtime::DefaultType>::DefaultType],
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).17)(
                ::core::mem::transmute_copy(this),
                items.len() as u32,
                ::core::mem::transmute(items.as_ptr()),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn First(
        &self,
    ) -> ::windows::runtime::Result<::windows::Foundation::Collections::IIterator<Block>> {
        let this = &::windows::runtime::Interface::cast::<
            ::windows::Foundation::Collections::IIterable<Block>,
        >(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Collections::IIterator<Block>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for BlockCollection {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Documents.BlockCollection;pinterface({913337e9-11a1-4345-a3a2-4e7f956e222d};rc(Microsoft.UI.Xaml.Documents.Block;{8149d507-672f-5fd5-a10a-351389ba9659})))" ) ;
}
unsafe impl ::windows::runtime::Interface for BlockCollection {
    type Vtable = ::windows::Foundation::Collections::IVector_abi<Block>;
    const IID : :: windows :: runtime :: GUID = :: windows :: runtime :: GUID :: from_signature ( < ::windows::Foundation::Collections:: IVector :: < Block > as :: windows :: runtime :: RuntimeType > :: SIGNATURE ) ;
}
impl ::windows::runtime::RuntimeName for BlockCollection {
    const NAME: &'static str = "Microsoft.UI.Xaml.Documents.BlockCollection";
}
impl ::core::convert::From<BlockCollection> for ::windows::runtime::IUnknown {
    fn from(value: BlockCollection) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&BlockCollection> for ::windows::runtime::IUnknown {
    fn from(value: &BlockCollection) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for BlockCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a BlockCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<BlockCollection> for ::windows::runtime::IInspectable {
    fn from(value: BlockCollection) -> Self {
        value.0
    }
}
impl ::core::convert::From<&BlockCollection> for ::windows::runtime::IInspectable {
    fn from(value: &BlockCollection) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for BlockCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a BlockCollection
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<BlockCollection> for ::windows::Foundation::Collections::IVector<Block> {
    fn from(value: BlockCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BlockCollection>
    for ::windows::Foundation::Collections::IVector<Block>
{
    fn from(value: &BlockCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::Foundation::Collections::IVector<Block>>
    for BlockCollection
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<'a, ::windows::Foundation::Collections::IVector<Block>> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::Foundation::Collections::IVector<Block>>
    for &BlockCollection
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<'a, ::windows::Foundation::Collections::IVector<Block>> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<BlockCollection>
    for ::windows::Foundation::Collections::IIterable<Block>
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: BlockCollection) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&BlockCollection>
    for ::windows::Foundation::Collections::IIterable<Block>
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: &BlockCollection) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::Foundation::Collections::IIterable<Block>>
    for BlockCollection
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<'a, ::windows::Foundation::Collections::IIterable<Block>> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::Foundation::Collections::IIterable<Block>>
    for &BlockCollection
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<'a, ::windows::Foundation::Collections::IIterable<Block>> {
        ::core::convert::TryInto::<::windows::Foundation::Collections::IIterable<Block>>::try_into(
            self,
        )
        .map(::windows::runtime::Param::Owned)
        .unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::core::marker::Send for BlockCollection {}
unsafe impl ::core::marker::Sync for BlockCollection {}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::core::iter::IntoIterator for BlockCollection {
    type Item = Block;
    type IntoIter = ::windows::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::core::iter::IntoIterator for &BlockCollection {
    type Item = Block;
    type IntoIter = ::windows::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::windows::Foundation::Collections::VectorIterator::new(
            ::core::convert::TryInto::try_into(self).ok(),
        )
    }
}
#[doc = "*Required features: `UI_Xaml_Documents`*"]
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct Bold(pub ::windows::runtime::IInspectable);
impl Bold {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            Bold,
            ::windows::runtime::IActivationFactory,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn GetValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetValue<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>,
    >(
        &self,
        dp: Param0,
        value: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn ClearValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn ReadLocalValue<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn GetAnimationBaseValue<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn RegisterPropertyChangedCallback<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
        Param1: ::windows::runtime::IntoParam<'a, super::DependencyPropertyChangedCallback>,
    >(
        &self,
        dp: Param0,
        callback: Param1,
    ) -> ::windows::runtime::Result<i64> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: i64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                callback.into_param().abi(),
                &mut result__,
            )
            .from_abi::<i64>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn UnregisterPropertyChangedCallback<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
        token: i64,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).12)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                token,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn Dispatcher(&self) -> ::windows::runtime::Result<::windows::UI::Core::CoreDispatcher> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Core::CoreDispatcher>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Dispatching`*"]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::runtime::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn Inlines(&self) -> ::windows::runtime::Result<InlineCollection> {
        let this = &::windows::runtime::Interface::cast::<ISpan>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<InlineCollection>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetInlines<'a, Param0: ::windows::runtime::IntoParam<'a, InlineCollection>>(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ISpan>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn Name(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn FontSize(&self) -> ::windows::runtime::Result<f64> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetFontSize(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Media`*"]
    pub fn FontFamily(&self) -> ::windows::runtime::Result<super::Media::FontFamily> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::FontFamily>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Media`*"]
    pub fn SetFontFamily<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Media::FontFamily>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn FontWeight(&self) -> ::windows::runtime::Result<::windows::UI::Text::FontWeight> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::UI::Text::FontWeight = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::FontWeight>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetFontWeight<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::UI::Text::FontWeight>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).12)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn FontStyle(&self) -> ::windows::runtime::Result<::windows::UI::Text::FontStyle> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::UI::Text::FontStyle = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::FontStyle>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetFontStyle(
        &self,
        value: ::windows::UI::Text::FontStyle,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).14)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn FontStretch(&self) -> ::windows::runtime::Result<::windows::UI::Text::FontStretch> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::UI::Text::FontStretch = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::FontStretch>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetFontStretch(
        &self,
        value: ::windows::UI::Text::FontStretch,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).16)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn CharacterSpacing(&self) -> ::windows::runtime::Result<i32> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetCharacterSpacing(&self, value: i32) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).18)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Media`*"]
    pub fn Foreground(&self) -> ::windows::runtime::Result<super::Media::Brush> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Media`*"]
    pub fn SetForeground<'a, Param0: ::windows::runtime::IntoParam<'a, super::Media::Brush>>(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).20)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn Language(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).21)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetLanguage<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).22)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn IsTextScaleFactorEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).23)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetIsTextScaleFactorEnabled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).24)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn TextDecorations(
        &self,
    ) -> ::windows::runtime::Result<::windows::UI::Text::TextDecorations> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::UI::Text::TextDecorations = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).25)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::TextDecorations>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetTextDecorations(
        &self,
        value: ::windows::UI::Text::TextDecorations,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).26)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn ContentStart(&self) -> ::windows::runtime::Result<TextPointer> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).27)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn ContentEnd(&self) -> ::windows::runtime::Result<TextPointer> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).28)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn ElementStart(&self) -> ::windows::runtime::Result<TextPointer> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).29)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn ElementEnd(&self) -> ::windows::runtime::Result<TextPointer> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).30)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn AllowFocusOnInteraction(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).31)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetAllowFocusOnInteraction(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).32)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn AccessKey(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).33)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetAccessKey<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).34)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn ExitDisplayModeOnAccessKeyInvoked(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).35)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetExitDisplayModeOnAccessKeyInvoked(
        &self,
        value: bool,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).36)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn IsAccessKeyScope(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).37)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetIsAccessKeyScope(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).38)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn AccessKeyScopeOwner(&self) -> ::windows::runtime::Result<super::DependencyObject> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).39)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyObject>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetAccessKeyScopeOwner<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).40)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Input`*"]
    pub fn KeyTipPlacementMode(
        &self,
    ) -> ::windows::runtime::Result<super::Input::KeyTipPlacementMode> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: super::Input::KeyTipPlacementMode = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).41)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Input::KeyTipPlacementMode>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Input`*"]
    pub fn SetKeyTipPlacementMode(
        &self,
        value: super::Input::KeyTipPlacementMode,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).42)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn KeyTipHorizontalOffset(&self) -> ::windows::runtime::Result<f64> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).43)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetKeyTipHorizontalOffset(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).44)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn KeyTipVerticalOffset(&self) -> ::windows::runtime::Result<f64> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).45)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetKeyTipVerticalOffset(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).46)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn XamlRoot(&self) -> ::windows::runtime::Result<super::XamlRoot> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).47)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::XamlRoot>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetXamlRoot<'a, Param0: ::windows::runtime::IntoParam<'a, super::XamlRoot>>(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).48)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Input`*"]
    pub fn AccessKeyDisplayRequested<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                TextElement,
                super::Input::AccessKeyDisplayRequestedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).49)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn RemoveAccessKeyDisplayRequested<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).50)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Input`*"]
    pub fn AccessKeyDisplayDismissed<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                TextElement,
                super::Input::AccessKeyDisplayDismissedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).51)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn RemoveAccessKeyDisplayDismissed<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).52)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Input`*"]
    pub fn AccessKeyInvoked<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                TextElement,
                super::Input::AccessKeyInvokedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).53)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn RemoveAccessKeyInvoked<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).54)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn FindName<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(
        &self,
        name: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).55)(
                ::core::mem::transmute_copy(this),
                name.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for Bold {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Xaml.Documents.Bold;{241a5f5a-c164-597f-b0db-fac7431297f2})",
    );
}
unsafe impl ::windows::runtime::Interface for Bold {
    type Vtable = IBold_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        605708122,
        49508,
        22911,
        [176, 219, 250, 199, 67, 18, 151, 242],
    );
}
impl ::windows::runtime::RuntimeName for Bold {
    const NAME: &'static str = "Microsoft.UI.Xaml.Documents.Bold";
}
impl ::core::convert::From<Bold> for ::windows::runtime::IUnknown {
    fn from(value: Bold) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&Bold> for ::windows::runtime::IUnknown {
    fn from(value: &Bold) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for Bold {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a Bold {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<Bold> for ::windows::runtime::IInspectable {
    fn from(value: Bold) -> Self {
        value.0
    }
}
impl ::core::convert::From<&Bold> for ::windows::runtime::IInspectable {
    fn from(value: &Bold) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for Bold {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a Bold {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<Bold> for Span {
    fn from(value: Bold) -> Self {
        ::core::convert::Into::<Span>::into(&value)
    }
}
impl ::core::convert::From<&Bold> for Span {
    fn from(value: &Bold) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, Span> for Bold {
    fn into_param(self) -> ::windows::runtime::Param<'a, Span> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<Span>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, Span> for &Bold {
    fn into_param(self) -> ::windows::runtime::Param<'a, Span> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<Span>::into(
            ::core::clone::Clone::clone(self),
        ))
    }
}
impl ::core::convert::From<Bold> for Inline {
    fn from(value: Bold) -> Self {
        ::core::convert::Into::<Inline>::into(&value)
    }
}
impl ::core::convert::From<&Bold> for Inline {
    fn from(value: &Bold) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, Inline> for Bold {
    fn into_param(self) -> ::windows::runtime::Param<'a, Inline> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<Inline>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, Inline> for &Bold {
    fn into_param(self) -> ::windows::runtime::Param<'a, Inline> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<Inline>::into(
            ::core::clone::Clone::clone(self),
        ))
    }
}
impl ::core::convert::From<Bold> for TextElement {
    fn from(value: Bold) -> Self {
        ::core::convert::Into::<TextElement>::into(&value)
    }
}
impl ::core::convert::From<&Bold> for TextElement {
    fn from(value: &Bold) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, TextElement> for Bold {
    fn into_param(self) -> ::windows::runtime::Param<'a, TextElement> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<TextElement>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, TextElement> for &Bold {
    fn into_param(self) -> ::windows::runtime::Param<'a, TextElement> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<TextElement>::into(
            ::core::clone::Clone::clone(self),
        ))
    }
}
impl ::core::convert::From<Bold> for super::DependencyObject {
    fn from(value: Bold) -> Self {
        ::core::convert::Into::<super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&Bold> for super::DependencyObject {
    fn from(value: &Bold) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::DependencyObject> for Bold {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::DependencyObject> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(
            self,
        ))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::DependencyObject> for &Bold {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::DependencyObject> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(
            ::core::clone::Clone::clone(self),
        ))
    }
}
unsafe impl ::core::marker::Send for Bold {}
unsafe impl ::core::marker::Sync for Bold {}
#[doc = "*Required features: `UI_Xaml_Documents`*"]
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct Glyphs(pub ::windows::runtime::IInspectable);
impl Glyphs {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            Glyphs,
            ::windows::runtime::IActivationFactory,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn UnicodeString(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetUnicodeString<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn Indices(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetIndices<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn FontUri(&self) -> ::windows::runtime::Result<::windows::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Uri>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetFontUri<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::Foundation::Uri>>(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).11)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Media`*"]
    pub fn StyleSimulations(&self) -> ::windows::runtime::Result<super::Media::StyleSimulations> {
        let this = self;
        unsafe {
            let mut result__: super::Media::StyleSimulations = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::StyleSimulations>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Media`*"]
    pub fn SetStyleSimulations(
        &self,
        value: super::Media::StyleSimulations,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).13)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn FontRenderingEmSize(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetFontRenderingEmSize(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).15)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn OriginX(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetOriginX(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).17)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn OriginY(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetOriginY(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).19)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Media`*"]
    pub fn Fill(&self) -> ::windows::runtime::Result<super::Media::Brush> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Media`*"]
    pub fn SetFill<'a, Param0: ::windows::runtime::IntoParam<'a, super::Media::Brush>>(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).21)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn IsColorFontEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).22)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetIsColorFontEnabled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).23)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn ColorFontPaletteIndex(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).24)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetColorFontPaletteIndex(&self, value: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).25)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn UnicodeStringProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::IGlyphsStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn IndicesProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::IGlyphsStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn FontUriProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::IGlyphsStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn StyleSimulationsProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::IGlyphsStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn FontRenderingEmSizeProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::IGlyphsStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn OriginXProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::IGlyphsStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn OriginYProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::IGlyphsStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn FillProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::IGlyphsStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn IsColorFontEnabledProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::IGlyphsStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn ColorFontPaletteIndexProperty() -> ::windows::runtime::Result<super::DependencyProperty>
    {
        Self::IGlyphsStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Composition")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Composition`*"]
    pub fn PopulatePropertyInfo<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Composition::AnimationPropertyInfo>,
    >(
        &self,
        propertyname: Param0,
        propertyinfo: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<
            super::super::Composition::IAnimationObject,
        >(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                propertyinfo.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Composition")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Composition`*"]
    pub fn GetVisualInternal(
        &self,
    ) -> ::windows::runtime::Result<super::super::Composition::Visual> {
        let this = &::windows::runtime::Interface::cast::<
            super::super::Composition::IVisualElement2,
        >(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Composition::Visual>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn GetValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetValue<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>,
    >(
        &self,
        dp: Param0,
        value: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn ClearValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn ReadLocalValue<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn GetAnimationBaseValue<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn RegisterPropertyChangedCallback<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
        Param1: ::windows::runtime::IntoParam<'a, super::DependencyPropertyChangedCallback>,
    >(
        &self,
        dp: Param0,
        callback: Param1,
    ) -> ::windows::runtime::Result<i64> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: i64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                callback.into_param().abi(),
                &mut result__,
            )
            .from_abi::<i64>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn UnregisterPropertyChangedCallback<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
        token: i64,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).12)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                token,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn Dispatcher(&self) -> ::windows::runtime::Result<::windows::UI::Core::CoreDispatcher> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Core::CoreDispatcher>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Dispatching`*"]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::runtime::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn Triggers(&self) -> ::windows::runtime::Result<super::TriggerCollection> {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::TriggerCollection>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn Resources(&self) -> ::windows::runtime::Result<super::ResourceDictionary> {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::ResourceDictionary>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetResources<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::ResourceDictionary>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn Tag(&self) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetTag<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn Language(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetLanguage<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).12)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn ActualWidth(&self) -> ::windows::runtime::Result<f64> {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn ActualHeight(&self) -> ::windows::runtime::Result<f64> {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn Width(&self) -> ::windows::runtime::Result<f64> {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetWidth(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).16)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn Height(&self) -> ::windows::runtime::Result<f64> {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetHeight(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).18)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn MinWidth(&self) -> ::windows::runtime::Result<f64> {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetMinWidth(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).20)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn MaxWidth(&self) -> ::windows::runtime::Result<f64> {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).21)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetMaxWidth(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).22)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn MinHeight(&self) -> ::windows::runtime::Result<f64> {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).23)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetMinHeight(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).24)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn MaxHeight(&self) -> ::windows::runtime::Result<f64> {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).25)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetMaxHeight(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).26)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn HorizontalAlignment(&self) -> ::windows::runtime::Result<super::HorizontalAlignment> {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__: super::HorizontalAlignment = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).27)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::HorizontalAlignment>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetHorizontalAlignment(
        &self,
        value: super::HorizontalAlignment,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).28)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn VerticalAlignment(&self) -> ::windows::runtime::Result<super::VerticalAlignment> {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__: super::VerticalAlignment = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).29)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::VerticalAlignment>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetVerticalAlignment(
        &self,
        value: super::VerticalAlignment,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).30)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn Margin(&self) -> ::windows::runtime::Result<super::Thickness> {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__: super::Thickness = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).31)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Thickness>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetMargin<'a, Param0: ::windows::runtime::IntoParam<'a, super::Thickness>>(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).32)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn Name(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).33)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetName<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).34)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn BaseUri(&self) -> ::windows::runtime::Result<::windows::Foundation::Uri> {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).35)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Uri>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn DataContext(&self) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).36)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetDataContext<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).37)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn AllowFocusOnInteraction(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).38)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetAllowFocusOnInteraction(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).39)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn FocusVisualMargin(&self) -> ::windows::runtime::Result<super::Thickness> {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__: super::Thickness = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).40)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Thickness>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetFocusVisualMargin<'a, Param0: ::windows::runtime::IntoParam<'a, super::Thickness>>(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).41)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn FocusVisualSecondaryThickness(&self) -> ::windows::runtime::Result<super::Thickness> {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__: super::Thickness = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).42)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Thickness>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetFocusVisualSecondaryThickness<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Thickness>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).43)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn FocusVisualPrimaryThickness(&self) -> ::windows::runtime::Result<super::Thickness> {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__: super::Thickness = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).44)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Thickness>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetFocusVisualPrimaryThickness<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Thickness>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).45)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Media`*"]
    pub fn FocusVisualSecondaryBrush(&self) -> ::windows::runtime::Result<super::Media::Brush> {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).46)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Media`*"]
    pub fn SetFocusVisualSecondaryBrush<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Media::Brush>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).47)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Media`*"]
    pub fn FocusVisualPrimaryBrush(&self) -> ::windows::runtime::Result<super::Media::Brush> {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).48)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Media`*"]
    pub fn SetFocusVisualPrimaryBrush<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Media::Brush>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).49)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn AllowFocusWhenDisabled(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).50)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetAllowFocusWhenDisabled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).51)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn Style(&self) -> ::windows::runtime::Result<super::Style> {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).52)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Style>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetStyle<'a, Param0: ::windows::runtime::IntoParam<'a, super::Style>>(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).53)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn Parent(&self) -> ::windows::runtime::Result<super::DependencyObject> {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).54)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyObject>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn FlowDirection(&self) -> ::windows::runtime::Result<super::FlowDirection> {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__: super::FlowDirection = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).55)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::FlowDirection>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetFlowDirection(&self, value: super::FlowDirection) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).56)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn RequestedTheme(&self) -> ::windows::runtime::Result<super::ElementTheme> {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__: super::ElementTheme = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).57)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::ElementTheme>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetRequestedTheme(&self, value: super::ElementTheme) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).58)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn IsLoaded(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).59)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn ActualTheme(&self) -> ::windows::runtime::Result<super::ElementTheme> {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__: super::ElementTheme = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).60)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::ElementTheme>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn Loaded<'a, Param0: ::windows::runtime::IntoParam<'a, super::RoutedEventHandler>>(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).61)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn RemoveLoaded<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).62)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn Unloaded<'a, Param0: ::windows::runtime::IntoParam<'a, super::RoutedEventHandler>>(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).63)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn RemoveUnloaded<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).64)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn DataContextChanged<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                super::FrameworkElement,
                super::DataContextChangedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).65)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn RemoveDataContextChanged<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).66)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SizeChanged<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::SizeChangedEventHandler>,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).67)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn RemoveSizeChanged<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).68)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn LayoutUpdated<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            ::windows::Foundation::EventHandler<::windows::runtime::IInspectable>,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).69)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn RemoveLayoutUpdated<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).70)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn Loading<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                super::FrameworkElement,
                ::windows::runtime::IInspectable,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).71)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn RemoveLoading<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).72)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn ActualThemeChanged<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                super::FrameworkElement,
                ::windows::runtime::IInspectable,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).73)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn RemoveActualThemeChanged<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).74)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn EffectiveViewportChanged<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                super::FrameworkElement,
                super::EffectiveViewportChangedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).75)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn RemoveEffectiveViewportChanged<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).76)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn FindName<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(
        &self,
        name: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).77)(
                ::core::mem::transmute_copy(this),
                name.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Data")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Data`*"]
    pub fn SetBinding<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
        Param1: ::windows::runtime::IntoParam<'a, super::Data::BindingBase>,
    >(
        &self,
        dp: Param0,
        binding: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).78)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                binding.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Data")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Data`*"]
    pub fn GetBindingExpression<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
    ) -> ::windows::runtime::Result<super::Data::BindingExpression> {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).79)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::Data::BindingExpression>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn InvalidateViewport(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IFrameworkElementProtected>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn DesiredSize(&self) -> ::windows::runtime::Result<::windows::Foundation::Size> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::Size = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Size>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn AllowDrop(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetAllowDrop(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn Opacity(&self) -> ::windows::runtime::Result<f64> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetOpacity(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Media`*"]
    pub fn Clip(&self) -> ::windows::runtime::Result<super::Media::RectangleGeometry> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::RectangleGeometry>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Media`*"]
    pub fn SetClip<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Media::RectangleGeometry>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).12)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Media`*"]
    pub fn RenderTransform(&self) -> ::windows::runtime::Result<super::Media::Transform> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::Transform>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Media`*"]
    pub fn SetRenderTransform<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Media::Transform>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).14)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Media`*"]
    pub fn Projection(&self) -> ::windows::runtime::Result<super::Media::Projection> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::Projection>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Media`*"]
    pub fn SetProjection<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Media::Projection>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).16)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Media_Media3D")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Media_Media3D`*"]
    pub fn Transform3D(&self) -> ::windows::runtime::Result<super::Media::Media3D::Transform3D> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::Media3D::Transform3D>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media_Media3D")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Media_Media3D`*"]
    pub fn SetTransform3D<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Media::Media3D::Transform3D>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).18)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn RenderTransformOrigin(
        &self,
    ) -> ::windows::runtime::Result<::windows::Foundation::Point> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::Point = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Point>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetRenderTransformOrigin<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::Foundation::Point>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).20)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn IsHitTestVisible(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).21)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetIsHitTestVisible(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).22)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn Visibility(&self) -> ::windows::runtime::Result<super::Visibility> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: super::Visibility = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).23)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Visibility>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetVisibility(&self, value: super::Visibility) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).24)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn RenderSize(&self) -> ::windows::runtime::Result<::windows::Foundation::Size> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::Size = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).25)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Size>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn UseLayoutRounding(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).26)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetUseLayoutRounding(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).27)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Media_Animation")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Media_Animation`*"]
    pub fn Transitions(
        &self,
    ) -> ::windows::runtime::Result<super::Media::Animation::TransitionCollection> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).28)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::Animation::TransitionCollection>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media_Animation")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Media_Animation`*"]
    pub fn SetTransitions<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Media::Animation::TransitionCollection>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).29)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Media`*"]
    pub fn CacheMode(&self) -> ::windows::runtime::Result<super::Media::CacheMode> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).30)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::CacheMode>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Media`*"]
    pub fn SetCacheMode<'a, Param0: ::windows::runtime::IntoParam<'a, super::Media::CacheMode>>(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).31)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn IsTapEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).32)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetIsTapEnabled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).33)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn IsDoubleTapEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).34)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetIsDoubleTapEnabled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).35)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn CanDrag(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).36)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetCanDrag(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).37)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn IsRightTapEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).38)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetIsRightTapEnabled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).39)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn IsHoldingEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).40)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetIsHoldingEnabled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).41)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Input`*"]
    pub fn ManipulationMode(&self) -> ::windows::runtime::Result<super::Input::ManipulationModes> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: super::Input::ManipulationModes = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).42)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Input::ManipulationModes>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Input`*"]
    pub fn SetManipulationMode(
        &self,
        value: super::Input::ManipulationModes,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).43)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Input`*"]
    pub fn PointerCaptures(
        &self,
    ) -> ::windows::runtime::Result<
        ::windows::Foundation::Collections::IVectorView<super::Input::Pointer>,
    > {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).44)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Collections::IVectorView<super::Input::Pointer>>(
                result__,
            )
        }
    }
    #[cfg(feature = "UI_Xaml_Controls_Primitives")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Controls_Primitives`*"]
    pub fn ContextFlyout(
        &self,
    ) -> ::windows::runtime::Result<super::Controls::Primitives::FlyoutBase> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).45)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Controls::Primitives::FlyoutBase>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Controls_Primitives")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Controls_Primitives`*"]
    pub fn SetContextFlyout<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Controls::Primitives::FlyoutBase>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).46)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Media`*"]
    pub fn CompositeMode(&self) -> ::windows::runtime::Result<super::Media::ElementCompositeMode> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: super::Media::ElementCompositeMode = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).47)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::ElementCompositeMode>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Media`*"]
    pub fn SetCompositeMode(
        &self,
        value: super::Media::ElementCompositeMode,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).48)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Media`*"]
    pub fn Lights(
        &self,
    ) -> ::windows::runtime::Result<
        ::windows::Foundation::Collections::IVector<super::Media::XamlLight>,
    > {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).49)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Collections::IVector<super::Media::XamlLight>>(
                result__,
            )
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn CanBeScrollAnchor(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).50)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetCanBeScrollAnchor(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).51)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn ExitDisplayModeOnAccessKeyInvoked(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).52)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetExitDisplayModeOnAccessKeyInvoked(
        &self,
        value: bool,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).53)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn IsAccessKeyScope(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).54)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetIsAccessKeyScope(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).55)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn AccessKeyScopeOwner(&self) -> ::windows::runtime::Result<super::DependencyObject> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).56)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyObject>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetAccessKeyScopeOwner<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).57)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn AccessKey(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).58)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetAccessKey<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).59)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Input`*"]
    pub fn KeyTipPlacementMode(
        &self,
    ) -> ::windows::runtime::Result<super::Input::KeyTipPlacementMode> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: super::Input::KeyTipPlacementMode = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).60)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Input::KeyTipPlacementMode>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Input`*"]
    pub fn SetKeyTipPlacementMode(
        &self,
        value: super::Input::KeyTipPlacementMode,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).61)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn KeyTipHorizontalOffset(&self) -> ::windows::runtime::Result<f64> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).62)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetKeyTipHorizontalOffset(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).63)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn KeyTipVerticalOffset(&self) -> ::windows::runtime::Result<f64> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).64)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetKeyTipVerticalOffset(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).65)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn KeyTipTarget(&self) -> ::windows::runtime::Result<super::DependencyObject> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).66)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyObject>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetKeyTipTarget<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).67)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Input`*"]
    pub fn XYFocusKeyboardNavigation(
        &self,
    ) -> ::windows::runtime::Result<super::Input::XYFocusKeyboardNavigationMode> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: super::Input::XYFocusKeyboardNavigationMode = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).68)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Input::XYFocusKeyboardNavigationMode>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Input`*"]
    pub fn SetXYFocusKeyboardNavigation(
        &self,
        value: super::Input::XYFocusKeyboardNavigationMode,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).69)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Input`*"]
    pub fn XYFocusUpNavigationStrategy(
        &self,
    ) -> ::windows::runtime::Result<super::Input::XYFocusNavigationStrategy> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: super::Input::XYFocusNavigationStrategy = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).70)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Input::XYFocusNavigationStrategy>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Input`*"]
    pub fn SetXYFocusUpNavigationStrategy(
        &self,
        value: super::Input::XYFocusNavigationStrategy,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).71)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Input`*"]
    pub fn XYFocusDownNavigationStrategy(
        &self,
    ) -> ::windows::runtime::Result<super::Input::XYFocusNavigationStrategy> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: super::Input::XYFocusNavigationStrategy = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).72)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Input::XYFocusNavigationStrategy>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Input`*"]
    pub fn SetXYFocusDownNavigationStrategy(
        &self,
        value: super::Input::XYFocusNavigationStrategy,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).73)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Input`*"]
    pub fn XYFocusLeftNavigationStrategy(
        &self,
    ) -> ::windows::runtime::Result<super::Input::XYFocusNavigationStrategy> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: super::Input::XYFocusNavigationStrategy = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).74)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Input::XYFocusNavigationStrategy>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Input`*"]
    pub fn SetXYFocusLeftNavigationStrategy(
        &self,
        value: super::Input::XYFocusNavigationStrategy,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).75)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Input`*"]
    pub fn XYFocusRightNavigationStrategy(
        &self,
    ) -> ::windows::runtime::Result<super::Input::XYFocusNavigationStrategy> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: super::Input::XYFocusNavigationStrategy = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).76)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Input::XYFocusNavigationStrategy>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Input`*"]
    pub fn SetXYFocusRightNavigationStrategy(
        &self,
        value: super::Input::XYFocusNavigationStrategy,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).77)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Input`*"]
    pub fn KeyboardAccelerators(
        &self,
    ) -> ::windows::runtime::Result<
        ::windows::Foundation::Collections::IVector<super::Input::KeyboardAccelerator>,
    > {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).78)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Collections::IVector<
                super::Input::KeyboardAccelerator,
            >>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn KeyboardAcceleratorPlacementTarget(
        &self,
    ) -> ::windows::runtime::Result<super::DependencyObject> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).79)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyObject>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetKeyboardAcceleratorPlacementTarget<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).80)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Input`*"]
    pub fn KeyboardAcceleratorPlacementMode(
        &self,
    ) -> ::windows::runtime::Result<super::Input::KeyboardAcceleratorPlacementMode> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: super::Input::KeyboardAcceleratorPlacementMode =
                ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).81)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Input::KeyboardAcceleratorPlacementMode>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Input`*"]
    pub fn SetKeyboardAcceleratorPlacementMode(
        &self,
        value: super::Input::KeyboardAcceleratorPlacementMode,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).82)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn HighContrastAdjustment(
        &self,
    ) -> ::windows::runtime::Result<super::ElementHighContrastAdjustment> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: super::ElementHighContrastAdjustment = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).83)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::ElementHighContrastAdjustment>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetHighContrastAdjustment(
        &self,
        value: super::ElementHighContrastAdjustment,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).84)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Input`*"]
    pub fn TabFocusNavigation(
        &self,
    ) -> ::windows::runtime::Result<super::Input::KeyboardNavigationMode> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: super::Input::KeyboardNavigationMode = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).85)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Input::KeyboardNavigationMode>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Input`*"]
    pub fn SetTabFocusNavigation(
        &self,
        value: super::Input::KeyboardNavigationMode,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).86)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn OpacityTransition(&self) -> ::windows::runtime::Result<super::ScalarTransition> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).87)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::ScalarTransition>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetOpacityTransition<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::ScalarTransition>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).88)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn Translation(
        &self,
    ) -> ::windows::runtime::Result<::windows::Foundation::Numerics::Vector3> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::Numerics::Vector3 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).89)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Numerics::Vector3>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetTranslation<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::Foundation::Numerics::Vector3>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).90)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn TranslationTransition(&self) -> ::windows::runtime::Result<super::Vector3Transition> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).91)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Vector3Transition>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetTranslationTransition<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Vector3Transition>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).92)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn Rotation(&self) -> ::windows::runtime::Result<f32> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).93)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetRotation(&self, value: f32) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).94)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn RotationTransition(&self) -> ::windows::runtime::Result<super::ScalarTransition> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).95)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::ScalarTransition>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetRotationTransition<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::ScalarTransition>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).96)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn Scale(&self) -> ::windows::runtime::Result<::windows::Foundation::Numerics::Vector3> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::Numerics::Vector3 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).97)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Numerics::Vector3>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetScale<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::Foundation::Numerics::Vector3>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).98)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn ScaleTransition(&self) -> ::windows::runtime::Result<super::Vector3Transition> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).99)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Vector3Transition>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetScaleTransition<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Vector3Transition>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).100)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn TransformMatrix(
        &self,
    ) -> ::windows::runtime::Result<::windows::Foundation::Numerics::Matrix4x4> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::Numerics::Matrix4x4 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).101)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Numerics::Matrix4x4>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetTransformMatrix<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::Foundation::Numerics::Matrix4x4>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).102)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn CenterPoint(
        &self,
    ) -> ::windows::runtime::Result<::windows::Foundation::Numerics::Vector3> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::Numerics::Vector3 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).103)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Numerics::Vector3>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetCenterPoint<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::Foundation::Numerics::Vector3>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).104)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn RotationAxis(
        &self,
    ) -> ::windows::runtime::Result<::windows::Foundation::Numerics::Vector3> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::Numerics::Vector3 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).105)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Numerics::Vector3>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetRotationAxis<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::Foundation::Numerics::Vector3>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).106)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn ActualOffset(
        &self,
    ) -> ::windows::runtime::Result<::windows::Foundation::Numerics::Vector3> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::Numerics::Vector3 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).107)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Numerics::Vector3>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn ActualSize(
        &self,
    ) -> ::windows::runtime::Result<::windows::Foundation::Numerics::Vector2> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::Numerics::Vector2 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).108)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Numerics::Vector2>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn XamlRoot(&self) -> ::windows::runtime::Result<super::XamlRoot> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).109)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::XamlRoot>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetXamlRoot<'a, Param0: ::windows::runtime::IntoParam<'a, super::XamlRoot>>(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).110)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Media`*"]
    pub fn Shadow(&self) -> ::windows::runtime::Result<super::Media::Shadow> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).111)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::Shadow>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Media`*"]
    pub fn SetShadow<'a, Param0: ::windows::runtime::IntoParam<'a, super::Media::Shadow>>(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).112)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn RasterizationScale(&self) -> ::windows::runtime::Result<f64> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).113)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetRasterizationScale(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).114)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn FocusState(&self) -> ::windows::runtime::Result<super::FocusState> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: super::FocusState = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).115)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::FocusState>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn UseSystemFocusVisuals(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).116)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetUseSystemFocusVisuals(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).117)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn XYFocusLeft(&self) -> ::windows::runtime::Result<super::DependencyObject> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).118)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyObject>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetXYFocusLeft<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).119)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn XYFocusRight(&self) -> ::windows::runtime::Result<super::DependencyObject> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).120)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyObject>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetXYFocusRight<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).121)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn XYFocusUp(&self) -> ::windows::runtime::Result<super::DependencyObject> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).122)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyObject>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetXYFocusUp<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>>(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).123)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn XYFocusDown(&self) -> ::windows::runtime::Result<super::DependencyObject> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).124)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyObject>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetXYFocusDown<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).125)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn IsTabStop(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).126)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetIsTabStop(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).127)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn TabIndex(&self) -> ::windows::runtime::Result<i32> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).128)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetTabIndex(&self, value: i32) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).129)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Input`*"]
    pub fn KeyUp<'a, Param0: ::windows::runtime::IntoParam<'a, super::Input::KeyEventHandler>>(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).130)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn RemoveKeyUp<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).131)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Input`*"]
    pub fn KeyDown<'a, Param0: ::windows::runtime::IntoParam<'a, super::Input::KeyEventHandler>>(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).132)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn RemoveKeyDown<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).133)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn GotFocus<'a, Param0: ::windows::runtime::IntoParam<'a, super::RoutedEventHandler>>(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).134)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn RemoveGotFocus<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).135)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn LostFocus<'a, Param0: ::windows::runtime::IntoParam<'a, super::RoutedEventHandler>>(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).136)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn RemoveLostFocus<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).137)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn DragStarting<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                super::UIElement,
                super::DragStartingEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).138)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn RemoveDragStarting<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).139)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn DropCompleted<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                super::UIElement,
                super::DropCompletedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).140)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn RemoveDropCompleted<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).141)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Input`*"]
    pub fn CharacterReceived<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                super::UIElement,
                super::Input::CharacterReceivedRoutedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).142)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn RemoveCharacterReceived<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).143)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn DragEnter<'a, Param0: ::windows::runtime::IntoParam<'a, super::DragEventHandler>>(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).144)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn RemoveDragEnter<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).145)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn DragLeave<'a, Param0: ::windows::runtime::IntoParam<'a, super::DragEventHandler>>(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).146)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn RemoveDragLeave<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).147)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn DragOver<'a, Param0: ::windows::runtime::IntoParam<'a, super::DragEventHandler>>(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).148)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn RemoveDragOver<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).149)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn Drop<'a, Param0: ::windows::runtime::IntoParam<'a, super::DragEventHandler>>(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).150)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn RemoveDrop<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).151)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Input`*"]
    pub fn PointerPressed<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Input::PointerEventHandler>,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).152)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn RemovePointerPressed<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).153)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Input`*"]
    pub fn PointerMoved<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Input::PointerEventHandler>,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).154)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn RemovePointerMoved<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).155)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Input`*"]
    pub fn PointerReleased<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Input::PointerEventHandler>,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).156)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn RemovePointerReleased<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).157)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Input`*"]
    pub fn PointerEntered<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Input::PointerEventHandler>,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).158)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn RemovePointerEntered<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).159)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Input`*"]
    pub fn PointerExited<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Input::PointerEventHandler>,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).160)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn RemovePointerExited<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).161)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Input`*"]
    pub fn PointerCaptureLost<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Input::PointerEventHandler>,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).162)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn RemovePointerCaptureLost<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).163)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Input`*"]
    pub fn PointerCanceled<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Input::PointerEventHandler>,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).164)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn RemovePointerCanceled<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).165)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Input`*"]
    pub fn PointerWheelChanged<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Input::PointerEventHandler>,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).166)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn RemovePointerWheelChanged<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).167)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Input`*"]
    pub fn Tapped<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Input::TappedEventHandler>,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).168)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn RemoveTapped<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).169)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Input`*"]
    pub fn DoubleTapped<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Input::DoubleTappedEventHandler>,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).170)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn RemoveDoubleTapped<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).171)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Input`*"]
    pub fn Holding<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Input::HoldingEventHandler>,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).172)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn RemoveHolding<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).173)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Input`*"]
    pub fn ContextRequested<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                super::UIElement,
                super::Input::ContextRequestedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).174)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn RemoveContextRequested<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).175)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn ContextCanceled<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<super::UIElement, super::RoutedEventArgs>,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).176)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn RemoveContextCanceled<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).177)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Input`*"]
    pub fn RightTapped<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Input::RightTappedEventHandler>,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).178)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn RemoveRightTapped<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).179)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Input`*"]
    pub fn ManipulationStarting<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Input::ManipulationStartingEventHandler>,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).180)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn RemoveManipulationStarting<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).181)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Input`*"]
    pub fn ManipulationInertiaStarting<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Input::ManipulationInertiaStartingEventHandler>,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).182)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn RemoveManipulationInertiaStarting<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).183)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Input`*"]
    pub fn ManipulationStarted<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Input::ManipulationStartedEventHandler>,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).184)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn RemoveManipulationStarted<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).185)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Input`*"]
    pub fn ManipulationDelta<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Input::ManipulationDeltaEventHandler>,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).186)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn RemoveManipulationDelta<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).187)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Input`*"]
    pub fn ManipulationCompleted<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Input::ManipulationCompletedEventHandler>,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).188)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn RemoveManipulationCompleted<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).189)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Input`*"]
    pub fn AccessKeyDisplayRequested<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                super::UIElement,
                super::Input::AccessKeyDisplayRequestedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).190)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn RemoveAccessKeyDisplayRequested<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).191)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Input`*"]
    pub fn AccessKeyDisplayDismissed<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                super::UIElement,
                super::Input::AccessKeyDisplayDismissedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).192)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn RemoveAccessKeyDisplayDismissed<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).193)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Input`*"]
    pub fn AccessKeyInvoked<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                super::UIElement,
                super::Input::AccessKeyInvokedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).194)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn RemoveAccessKeyInvoked<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).195)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Input`*"]
    pub fn ProcessKeyboardAccelerators<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                super::UIElement,
                super::Input::ProcessKeyboardAcceleratorEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).196)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn RemoveProcessKeyboardAccelerators<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).197)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Input`*"]
    pub fn GettingFocus<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                super::UIElement,
                super::Input::GettingFocusEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).198)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn RemoveGettingFocus<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).199)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Input`*"]
    pub fn LosingFocus<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                super::UIElement,
                super::Input::LosingFocusEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).200)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn RemoveLosingFocus<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).201)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Input`*"]
    pub fn NoFocusCandidateFound<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                super::UIElement,
                super::Input::NoFocusCandidateFoundEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).202)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn RemoveNoFocusCandidateFound<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).203)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Input`*"]
    pub fn PreviewKeyDown<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Input::KeyEventHandler>,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).204)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn RemovePreviewKeyDown<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).205)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Input`*"]
    pub fn PreviewKeyUp<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Input::KeyEventHandler>,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).206)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn RemovePreviewKeyUp<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).207)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn BringIntoViewRequested<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                super::UIElement,
                super::BringIntoViewRequestedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).208)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn RemoveBringIntoViewRequested<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).209)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn Measure<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::Foundation::Size>>(
        &self,
        availablesize: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).210)(
                ::core::mem::transmute_copy(this),
                availablesize.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn Arrange<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::Foundation::Rect>>(
        &self,
        finalrect: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).211)(
                ::core::mem::transmute_copy(this),
                finalrect.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Input`*"]
    pub fn CapturePointer<'a, Param0: ::windows::runtime::IntoParam<'a, super::Input::Pointer>>(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).212)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Input`*"]
    pub fn ReleasePointerCapture<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Input::Pointer>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).213)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn ReleasePointerCaptures(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).214)(::core::mem::transmute_copy(this))
                .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn AddHandler<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::RoutedEvent>,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>,
    >(
        &self,
        routedevent: Param0,
        handler: Param1,
        handledeventstoo: bool,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).215)(
                ::core::mem::transmute_copy(this),
                routedevent.into_param().abi(),
                handler.into_param().abi(),
                handledeventstoo,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn RemoveHandler<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::RoutedEvent>,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>,
    >(
        &self,
        routedevent: Param0,
        handler: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).216)(
                ::core::mem::transmute_copy(this),
                routedevent.into_param().abi(),
                handler.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Media`*"]
    pub fn TransformToVisual<'a, Param0: ::windows::runtime::IntoParam<'a, super::UIElement>>(
        &self,
        visual: Param0,
    ) -> ::windows::runtime::Result<super::Media::GeneralTransform> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).217)(
                ::core::mem::transmute_copy(this),
                visual.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::Media::GeneralTransform>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn InvalidateMeasure(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).218)(::core::mem::transmute_copy(this))
                .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn InvalidateArrange(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).219)(::core::mem::transmute_copy(this))
                .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn UpdateLayout(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).220)(::core::mem::transmute_copy(this))
                .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn CancelDirectManipulations(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).221)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "UI_Input")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Input`*"]
    pub fn StartDragAsync<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Input::PointerPoint>,
    >(
        &self,
        pointerpoint: Param0,
    ) -> ::windows::runtime::Result<
        ::windows::Foundation::IAsyncOperation<
            ::windows::ApplicationModel::DataTransfer::DataPackageOperation,
        >,
    > {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).222)(
                ::core::mem::transmute_copy(this),
                pointerpoint.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::IAsyncOperation<
                ::windows::ApplicationModel::DataTransfer::DataPackageOperation,
            >>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn StartBringIntoView(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).223)(::core::mem::transmute_copy(this))
                .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn StartBringIntoViewWithOptions<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::BringIntoViewOptions>,
    >(
        &self,
        options: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).224)(
                ::core::mem::transmute_copy(this),
                options.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Input`*"]
    pub fn TryInvokeKeyboardAccelerator<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Input::ProcessKeyboardAcceleratorEventArgs>,
    >(
        &self,
        args: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).225)(
                ::core::mem::transmute_copy(this),
                args.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn Focus(&self, value: super::FocusState) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).226)(
                ::core::mem::transmute_copy(this),
                value,
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "UI_Composition")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Composition`*"]
    pub fn StartAnimation<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Composition::ICompositionAnimationBase>,
    >(
        &self,
        animation: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).227)(
                ::core::mem::transmute_copy(this),
                animation.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Composition")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Composition`*"]
    pub fn StopAnimation<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Composition::ICompositionAnimationBase>,
    >(
        &self,
        animation: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).228)(
                ::core::mem::transmute_copy(this),
                animation.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Input")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Input`*"]
    pub fn ProtectedCursor(&self) -> ::windows::runtime::Result<super::super::Input::InputCursor> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElementProtected>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Input::InputCursor>(result__)
        }
    }
    #[cfg(feature = "UI_Input")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Input`*"]
    pub fn SetProtectedCursor<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Input::InputCursor>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IUIElementProtected>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn IGlyphsStatics<R, F: FnOnce(&IGlyphsStatics) -> ::windows::runtime::Result<R>>(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<Glyphs, IGlyphsStatics> =
            ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for Glyphs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Xaml.Documents.Glyphs;{0fbf8cfe-18e7-5e45-9fa3-d2d0927958f4})",
    );
}
unsafe impl ::windows::runtime::Interface for Glyphs {
    type Vtable = IGlyphs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        264211710,
        6375,
        24133,
        [159, 163, 210, 208, 146, 121, 88, 244],
    );
}
impl ::windows::runtime::RuntimeName for Glyphs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Documents.Glyphs";
}
impl ::core::convert::From<Glyphs> for ::windows::runtime::IUnknown {
    fn from(value: Glyphs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&Glyphs> for ::windows::runtime::IUnknown {
    fn from(value: &Glyphs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for Glyphs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a Glyphs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<Glyphs> for ::windows::runtime::IInspectable {
    fn from(value: Glyphs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&Glyphs> for ::windows::runtime::IInspectable {
    fn from(value: &Glyphs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for Glyphs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a Glyphs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<Glyphs> for super::super::Composition::IAnimationObject {
    type Error = ::windows::runtime::Error;
    fn try_from(value: Glyphs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<&Glyphs> for super::super::Composition::IAnimationObject {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &Glyphs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Composition::IAnimationObject> for Glyphs {
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<'a, super::super::Composition::IAnimationObject> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Composition::IAnimationObject>
    for &Glyphs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<'a, super::super::Composition::IAnimationObject> {
        ::core::convert::TryInto::<super::super::Composition::IAnimationObject>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<Glyphs> for super::super::Composition::IVisualElement {
    type Error = ::windows::runtime::Error;
    fn try_from(value: Glyphs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<&Glyphs> for super::super::Composition::IVisualElement {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &Glyphs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Composition::IVisualElement> for Glyphs {
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<'a, super::super::Composition::IVisualElement> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Composition::IVisualElement> for &Glyphs {
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<'a, super::super::Composition::IVisualElement> {
        ::core::convert::TryInto::<super::super::Composition::IVisualElement>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<Glyphs> for super::super::Composition::IVisualElement2 {
    type Error = ::windows::runtime::Error;
    fn try_from(value: Glyphs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<&Glyphs> for super::super::Composition::IVisualElement2 {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &Glyphs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Composition::IVisualElement2> for Glyphs {
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<'a, super::super::Composition::IVisualElement2> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Composition::IVisualElement2> for &Glyphs {
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<'a, super::super::Composition::IVisualElement2> {
        ::core::convert::TryInto::<super::super::Composition::IVisualElement2>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::From<Glyphs> for super::FrameworkElement {
    fn from(value: Glyphs) -> Self {
        ::core::convert::Into::<super::FrameworkElement>::into(&value)
    }
}
impl ::core::convert::From<&Glyphs> for super::FrameworkElement {
    fn from(value: &Glyphs) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::FrameworkElement> for Glyphs {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::FrameworkElement> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::FrameworkElement>::into(
            self,
        ))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::FrameworkElement> for &Glyphs {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::FrameworkElement> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::FrameworkElement>::into(
            ::core::clone::Clone::clone(self),
        ))
    }
}
impl ::core::convert::From<Glyphs> for super::UIElement {
    fn from(value: Glyphs) -> Self {
        ::core::convert::Into::<super::UIElement>::into(&value)
    }
}
impl ::core::convert::From<&Glyphs> for super::UIElement {
    fn from(value: &Glyphs) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::UIElement> for Glyphs {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::UIElement> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::UIElement>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::UIElement> for &Glyphs {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::UIElement> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::UIElement>::into(
            ::core::clone::Clone::clone(self),
        ))
    }
}
impl ::core::convert::From<Glyphs> for super::DependencyObject {
    fn from(value: Glyphs) -> Self {
        ::core::convert::Into::<super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&Glyphs> for super::DependencyObject {
    fn from(value: &Glyphs) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::DependencyObject> for Glyphs {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::DependencyObject> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(
            self,
        ))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::DependencyObject> for &Glyphs {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::DependencyObject> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(
            ::core::clone::Clone::clone(self),
        ))
    }
}
unsafe impl ::core::marker::Send for Glyphs {}
unsafe impl ::core::marker::Sync for Glyphs {}
#[doc = "*Required features: `UI_Xaml_Documents`*"]
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct Hyperlink(pub ::windows::runtime::IInspectable);
impl Hyperlink {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            Hyperlink,
            ::windows::runtime::IActivationFactory,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn NavigateUri(&self) -> ::windows::runtime::Result<::windows::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Uri>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetNavigateUri<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::Foundation::Uri>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn UnderlineStyle(&self) -> ::windows::runtime::Result<UnderlineStyle> {
        let this = self;
        unsafe {
            let mut result__: UnderlineStyle = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<UnderlineStyle>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetUnderlineStyle(&self, value: UnderlineStyle) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn XYFocusLeft(&self) -> ::windows::runtime::Result<super::DependencyObject> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyObject>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetXYFocusLeft<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).11)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn XYFocusRight(&self) -> ::windows::runtime::Result<super::DependencyObject> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyObject>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetXYFocusRight<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).13)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn XYFocusUp(&self) -> ::windows::runtime::Result<super::DependencyObject> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyObject>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetXYFocusUp<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>>(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).15)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn XYFocusDown(&self) -> ::windows::runtime::Result<super::DependencyObject> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyObject>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetXYFocusDown<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).17)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn ElementSoundMode(&self) -> ::windows::runtime::Result<super::ElementSoundMode> {
        let this = self;
        unsafe {
            let mut result__: super::ElementSoundMode = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::ElementSoundMode>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetElementSoundMode(
        &self,
        value: super::ElementSoundMode,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).19)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn FocusState(&self) -> ::windows::runtime::Result<super::FocusState> {
        let this = self;
        unsafe {
            let mut result__: super::FocusState = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::FocusState>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Input`*"]
    pub fn XYFocusUpNavigationStrategy(
        &self,
    ) -> ::windows::runtime::Result<super::Input::XYFocusNavigationStrategy> {
        let this = self;
        unsafe {
            let mut result__: super::Input::XYFocusNavigationStrategy = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).21)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Input::XYFocusNavigationStrategy>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Input`*"]
    pub fn SetXYFocusUpNavigationStrategy(
        &self,
        value: super::Input::XYFocusNavigationStrategy,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).22)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Input`*"]
    pub fn XYFocusDownNavigationStrategy(
        &self,
    ) -> ::windows::runtime::Result<super::Input::XYFocusNavigationStrategy> {
        let this = self;
        unsafe {
            let mut result__: super::Input::XYFocusNavigationStrategy = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).23)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Input::XYFocusNavigationStrategy>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Input`*"]
    pub fn SetXYFocusDownNavigationStrategy(
        &self,
        value: super::Input::XYFocusNavigationStrategy,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).24)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Input`*"]
    pub fn XYFocusLeftNavigationStrategy(
        &self,
    ) -> ::windows::runtime::Result<super::Input::XYFocusNavigationStrategy> {
        let this = self;
        unsafe {
            let mut result__: super::Input::XYFocusNavigationStrategy = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).25)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Input::XYFocusNavigationStrategy>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Input`*"]
    pub fn SetXYFocusLeftNavigationStrategy(
        &self,
        value: super::Input::XYFocusNavigationStrategy,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).26)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Input`*"]
    pub fn XYFocusRightNavigationStrategy(
        &self,
    ) -> ::windows::runtime::Result<super::Input::XYFocusNavigationStrategy> {
        let this = self;
        unsafe {
            let mut result__: super::Input::XYFocusNavigationStrategy = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).27)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Input::XYFocusNavigationStrategy>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Input`*"]
    pub fn SetXYFocusRightNavigationStrategy(
        &self,
        value: super::Input::XYFocusNavigationStrategy,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).28)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn IsTabStop(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).29)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetIsTabStop(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).30)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn TabIndex(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).31)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetTabIndex(&self, value: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).32)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn Click<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<Hyperlink, HyperlinkClickEventArgs>,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<::windows::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).33)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn RemoveClick<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).34)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn GotFocus<'a, Param0: ::windows::runtime::IntoParam<'a, super::RoutedEventHandler>>(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<::windows::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).35)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn RemoveGotFocus<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).36)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn LostFocus<'a, Param0: ::windows::runtime::IntoParam<'a, super::RoutedEventHandler>>(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<::windows::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).37)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn RemoveLostFocus<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).38)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn Focus(&self, value: super::FocusState) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).39)(
                ::core::mem::transmute_copy(this),
                value,
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn NavigateUriProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::IHyperlinkStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn UnderlineStyleProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::IHyperlinkStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn XYFocusLeftProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::IHyperlinkStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn XYFocusRightProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::IHyperlinkStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn XYFocusUpProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::IHyperlinkStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn XYFocusDownProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::IHyperlinkStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn ElementSoundModeProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::IHyperlinkStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn FocusStateProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::IHyperlinkStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn XYFocusUpNavigationStrategyProperty(
    ) -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::IHyperlinkStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn XYFocusDownNavigationStrategyProperty(
    ) -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::IHyperlinkStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn XYFocusLeftNavigationStrategyProperty(
    ) -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::IHyperlinkStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn XYFocusRightNavigationStrategyProperty(
    ) -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::IHyperlinkStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn IsTabStopProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::IHyperlinkStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn TabIndexProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::IHyperlinkStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn GetValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetValue<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>,
    >(
        &self,
        dp: Param0,
        value: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn ClearValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn ReadLocalValue<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn GetAnimationBaseValue<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn RegisterPropertyChangedCallback<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
        Param1: ::windows::runtime::IntoParam<'a, super::DependencyPropertyChangedCallback>,
    >(
        &self,
        dp: Param0,
        callback: Param1,
    ) -> ::windows::runtime::Result<i64> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: i64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                callback.into_param().abi(),
                &mut result__,
            )
            .from_abi::<i64>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn UnregisterPropertyChangedCallback<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
        token: i64,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).12)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                token,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn Dispatcher(&self) -> ::windows::runtime::Result<::windows::UI::Core::CoreDispatcher> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Core::CoreDispatcher>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Dispatching`*"]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::runtime::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn Inlines(&self) -> ::windows::runtime::Result<InlineCollection> {
        let this = &::windows::runtime::Interface::cast::<ISpan>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<InlineCollection>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetInlines<'a, Param0: ::windows::runtime::IntoParam<'a, InlineCollection>>(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ISpan>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn Name(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn FontSize(&self) -> ::windows::runtime::Result<f64> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetFontSize(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Media`*"]
    pub fn FontFamily(&self) -> ::windows::runtime::Result<super::Media::FontFamily> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::FontFamily>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Media`*"]
    pub fn SetFontFamily<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Media::FontFamily>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn FontWeight(&self) -> ::windows::runtime::Result<::windows::UI::Text::FontWeight> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::UI::Text::FontWeight = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::FontWeight>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetFontWeight<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::UI::Text::FontWeight>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).12)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn FontStyle(&self) -> ::windows::runtime::Result<::windows::UI::Text::FontStyle> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::UI::Text::FontStyle = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::FontStyle>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetFontStyle(
        &self,
        value: ::windows::UI::Text::FontStyle,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).14)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn FontStretch(&self) -> ::windows::runtime::Result<::windows::UI::Text::FontStretch> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::UI::Text::FontStretch = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::FontStretch>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetFontStretch(
        &self,
        value: ::windows::UI::Text::FontStretch,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).16)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn CharacterSpacing(&self) -> ::windows::runtime::Result<i32> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetCharacterSpacing(&self, value: i32) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).18)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Media`*"]
    pub fn Foreground(&self) -> ::windows::runtime::Result<super::Media::Brush> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Media`*"]
    pub fn SetForeground<'a, Param0: ::windows::runtime::IntoParam<'a, super::Media::Brush>>(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).20)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn Language(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).21)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetLanguage<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).22)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn IsTextScaleFactorEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).23)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetIsTextScaleFactorEnabled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).24)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn TextDecorations(
        &self,
    ) -> ::windows::runtime::Result<::windows::UI::Text::TextDecorations> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::UI::Text::TextDecorations = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).25)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::TextDecorations>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetTextDecorations(
        &self,
        value: ::windows::UI::Text::TextDecorations,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).26)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn ContentStart(&self) -> ::windows::runtime::Result<TextPointer> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).27)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn ContentEnd(&self) -> ::windows::runtime::Result<TextPointer> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).28)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn ElementStart(&self) -> ::windows::runtime::Result<TextPointer> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).29)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn ElementEnd(&self) -> ::windows::runtime::Result<TextPointer> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).30)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn AllowFocusOnInteraction(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).31)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetAllowFocusOnInteraction(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).32)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn AccessKey(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).33)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetAccessKey<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).34)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn ExitDisplayModeOnAccessKeyInvoked(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).35)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetExitDisplayModeOnAccessKeyInvoked(
        &self,
        value: bool,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).36)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn IsAccessKeyScope(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).37)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetIsAccessKeyScope(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).38)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn AccessKeyScopeOwner(&self) -> ::windows::runtime::Result<super::DependencyObject> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).39)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyObject>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetAccessKeyScopeOwner<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).40)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Input`*"]
    pub fn KeyTipPlacementMode(
        &self,
    ) -> ::windows::runtime::Result<super::Input::KeyTipPlacementMode> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: super::Input::KeyTipPlacementMode = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).41)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Input::KeyTipPlacementMode>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Input`*"]
    pub fn SetKeyTipPlacementMode(
        &self,
        value: super::Input::KeyTipPlacementMode,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).42)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn KeyTipHorizontalOffset(&self) -> ::windows::runtime::Result<f64> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).43)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetKeyTipHorizontalOffset(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).44)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn KeyTipVerticalOffset(&self) -> ::windows::runtime::Result<f64> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).45)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetKeyTipVerticalOffset(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).46)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn XamlRoot(&self) -> ::windows::runtime::Result<super::XamlRoot> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).47)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::XamlRoot>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetXamlRoot<'a, Param0: ::windows::runtime::IntoParam<'a, super::XamlRoot>>(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).48)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Input`*"]
    pub fn AccessKeyDisplayRequested<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                TextElement,
                super::Input::AccessKeyDisplayRequestedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).49)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn RemoveAccessKeyDisplayRequested<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).50)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Input`*"]
    pub fn AccessKeyDisplayDismissed<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                TextElement,
                super::Input::AccessKeyDisplayDismissedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).51)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn RemoveAccessKeyDisplayDismissed<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).52)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Input`*"]
    pub fn AccessKeyInvoked<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                TextElement,
                super::Input::AccessKeyInvokedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).53)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn RemoveAccessKeyInvoked<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).54)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn FindName<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(
        &self,
        name: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).55)(
                ::core::mem::transmute_copy(this),
                name.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    pub fn IHyperlinkStatics<R, F: FnOnce(&IHyperlinkStatics) -> ::windows::runtime::Result<R>>(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<Hyperlink, IHyperlinkStatics> =
            ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for Hyperlink {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Xaml.Documents.Hyperlink;{ac09bd16-cdfa-54c2-8d03-a474181545b1})",
    );
}
unsafe impl ::windows::runtime::Interface for Hyperlink {
    type Vtable = IHyperlink_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2886319382,
        52730,
        21698,
        [141, 3, 164, 116, 24, 21, 69, 177],
    );
}
impl ::windows::runtime::RuntimeName for Hyperlink {
    const NAME: &'static str = "Microsoft.UI.Xaml.Documents.Hyperlink";
}
impl ::core::convert::From<Hyperlink> for ::windows::runtime::IUnknown {
    fn from(value: Hyperlink) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&Hyperlink> for ::windows::runtime::IUnknown {
    fn from(value: &Hyperlink) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for Hyperlink {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a Hyperlink {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<Hyperlink> for ::windows::runtime::IInspectable {
    fn from(value: Hyperlink) -> Self {
        value.0
    }
}
impl ::core::convert::From<&Hyperlink> for ::windows::runtime::IInspectable {
    fn from(value: &Hyperlink) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for Hyperlink {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a Hyperlink {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<Hyperlink> for Span {
    fn from(value: Hyperlink) -> Self {
        ::core::convert::Into::<Span>::into(&value)
    }
}
impl ::core::convert::From<&Hyperlink> for Span {
    fn from(value: &Hyperlink) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, Span> for Hyperlink {
    fn into_param(self) -> ::windows::runtime::Param<'a, Span> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<Span>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, Span> for &Hyperlink {
    fn into_param(self) -> ::windows::runtime::Param<'a, Span> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<Span>::into(
            ::core::clone::Clone::clone(self),
        ))
    }
}
impl ::core::convert::From<Hyperlink> for Inline {
    fn from(value: Hyperlink) -> Self {
        ::core::convert::Into::<Inline>::into(&value)
    }
}
impl ::core::convert::From<&Hyperlink> for Inline {
    fn from(value: &Hyperlink) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, Inline> for Hyperlink {
    fn into_param(self) -> ::windows::runtime::Param<'a, Inline> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<Inline>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, Inline> for &Hyperlink {
    fn into_param(self) -> ::windows::runtime::Param<'a, Inline> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<Inline>::into(
            ::core::clone::Clone::clone(self),
        ))
    }
}
impl ::core::convert::From<Hyperlink> for TextElement {
    fn from(value: Hyperlink) -> Self {
        ::core::convert::Into::<TextElement>::into(&value)
    }
}
impl ::core::convert::From<&Hyperlink> for TextElement {
    fn from(value: &Hyperlink) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, TextElement> for Hyperlink {
    fn into_param(self) -> ::windows::runtime::Param<'a, TextElement> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<TextElement>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, TextElement> for &Hyperlink {
    fn into_param(self) -> ::windows::runtime::Param<'a, TextElement> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<TextElement>::into(
            ::core::clone::Clone::clone(self),
        ))
    }
}
impl ::core::convert::From<Hyperlink> for super::DependencyObject {
    fn from(value: Hyperlink) -> Self {
        ::core::convert::Into::<super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&Hyperlink> for super::DependencyObject {
    fn from(value: &Hyperlink) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::DependencyObject> for Hyperlink {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::DependencyObject> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(
            self,
        ))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::DependencyObject> for &Hyperlink {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::DependencyObject> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(
            ::core::clone::Clone::clone(self),
        ))
    }
}
unsafe impl ::core::marker::Send for Hyperlink {}
unsafe impl ::core::marker::Sync for Hyperlink {}
#[doc = "*Required features: `UI_Xaml_Documents`*"]
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct HyperlinkClickEventArgs(pub ::windows::runtime::IInspectable);
impl HyperlinkClickEventArgs {
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn OriginalSource(&self) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::IRoutedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for HyperlinkClickEventArgs {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Documents.HyperlinkClickEventArgs;{f8f89552-873d-5ef5-82bf-c79a9509b07c})" ) ;
}
unsafe impl ::windows::runtime::Interface for HyperlinkClickEventArgs {
    type Vtable = IHyperlinkClickEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        4177040722,
        34621,
        24309,
        [130, 191, 199, 154, 149, 9, 176, 124],
    );
}
impl ::windows::runtime::RuntimeName for HyperlinkClickEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Documents.HyperlinkClickEventArgs";
}
impl ::core::convert::From<HyperlinkClickEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: HyperlinkClickEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&HyperlinkClickEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &HyperlinkClickEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for HyperlinkClickEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &'a HyperlinkClickEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<HyperlinkClickEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: HyperlinkClickEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&HyperlinkClickEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &HyperlinkClickEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for HyperlinkClickEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a HyperlinkClickEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<HyperlinkClickEventArgs> for super::RoutedEventArgs {
    fn from(value: HyperlinkClickEventArgs) -> Self {
        ::core::convert::Into::<super::RoutedEventArgs>::into(&value)
    }
}
impl ::core::convert::From<&HyperlinkClickEventArgs> for super::RoutedEventArgs {
    fn from(value: &HyperlinkClickEventArgs) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::RoutedEventArgs> for HyperlinkClickEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::RoutedEventArgs> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::RoutedEventArgs>::into(
            self,
        ))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::RoutedEventArgs> for &HyperlinkClickEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::RoutedEventArgs> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::RoutedEventArgs>::into(
            ::core::clone::Clone::clone(self),
        ))
    }
}
unsafe impl ::core::marker::Send for HyperlinkClickEventArgs {}
unsafe impl ::core::marker::Sync for HyperlinkClickEventArgs {}
#[repr(transparent)]
#[doc(hidden)]
pub struct IBlock(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBlock {
    type Vtable = IBlock_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2169099527,
        26415,
        24533,
        [161, 10, 53, 19, 137, 186, 150, 89],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IBlock_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut super::TextAlignment,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: super::TextAlignment,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut super::TextAlignment,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: super::TextAlignment,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut f64,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: f64,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut super::LineStackingStrategy,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: super::LineStackingStrategy,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut super::Thickness,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: super::Thickness,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBlockFactory(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBlockFactory {
    type Vtable = IBlockFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        566060828,
        13282,
        22255,
        [190, 55, 161, 40, 232, 152, 69, 44],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IBlockFactory_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        baseinterface: ::windows::runtime::RawPtr,
        innerinterface: *mut ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBlockStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBlockStatics {
    type Vtable = IBlockStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2198859487,
        39590,
        22221,
        [152, 62, 5, 85, 0, 23, 27, 69],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IBlockStatics_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBold(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBold {
    type Vtable = IBold_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        605708122,
        49508,
        22911,
        [176, 219, 250, 199, 67, 18, 151, 242],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IBold_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGlyphs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IGlyphs {
    type Vtable = IGlyphs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        264211710,
        6375,
        24133,
        [159, 163, 210, 208, 146, 121, 88, 244],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IGlyphs_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "UI_Xaml_Media")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut super::Media::StyleSimulations,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: super::Media::StyleSimulations,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut f64,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: f64,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut f64,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: f64,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut f64,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: f64,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "UI_Xaml_Media")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: i32,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGlyphsStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IGlyphsStatics {
    type Vtable = IGlyphsStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2375951386,
        15886,
        20736,
        [142, 222, 224, 8, 3, 79, 248, 174],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IGlyphsStatics_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHyperlink(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHyperlink {
    type Vtable = IHyperlink_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2886319382,
        52730,
        21698,
        [141, 3, 164, 116, 24, 21, 69, 177],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IHyperlink_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut UnderlineStyle,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: UnderlineStyle,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut super::ElementSoundMode,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: super::ElementSoundMode,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut super::FocusState,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "UI_Xaml_Input")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut super::Input::XYFocusNavigationStrategy,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))] usize,
    #[cfg(feature = "UI_Xaml_Input")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: super::Input::XYFocusNavigationStrategy,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))] usize,
    #[cfg(feature = "UI_Xaml_Input")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut super::Input::XYFocusNavigationStrategy,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))] usize,
    #[cfg(feature = "UI_Xaml_Input")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: super::Input::XYFocusNavigationStrategy,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))] usize,
    #[cfg(feature = "UI_Xaml_Input")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut super::Input::XYFocusNavigationStrategy,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))] usize,
    #[cfg(feature = "UI_Xaml_Input")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: super::Input::XYFocusNavigationStrategy,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))] usize,
    #[cfg(feature = "UI_Xaml_Input")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut super::Input::XYFocusNavigationStrategy,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))] usize,
    #[cfg(feature = "UI_Xaml_Input")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: super::Input::XYFocusNavigationStrategy,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        handler: ::windows::runtime::RawPtr,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        handler: ::windows::runtime::RawPtr,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        handler: ::windows::runtime::RawPtr,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: super::FocusState,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHyperlinkClickEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHyperlinkClickEventArgs {
    type Vtable = IHyperlinkClickEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        4177040722,
        34621,
        24309,
        [130, 191, 199, 154, 149, 9, 176, 124],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IHyperlinkClickEventArgs_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHyperlinkStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHyperlinkStatics {
    type Vtable = IHyperlinkStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3778386164,
        31687,
        23225,
        [136, 91, 112, 243, 47, 140, 149, 49],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IHyperlinkStatics_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInline(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInline {
    type Vtable = IInline_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2168275578,
        35200,
        23161,
        [168, 250, 242, 121, 25, 207, 178, 79],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IInline_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInlineFactory(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInlineFactory {
    type Vtable = IInlineFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        4247075382,
        64043,
        23344,
        [137, 168, 159, 87, 120, 113, 236, 7],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IInlineFactory_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        baseinterface: ::windows::runtime::RawPtr,
        innerinterface: *mut ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInlineUIContainer(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInlineUIContainer {
    type Vtable = IInlineUIContainer_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3576282870,
        49242,
        23469,
        [133, 232, 100, 1, 39, 207, 134, 245],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IInlineUIContainer_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IItalic(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IItalic {
    type Vtable = IItalic_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3392978621,
        31373,
        23930,
        [143, 223, 83, 142, 138, 104, 15, 108],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IItalic_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILineBreak(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ILineBreak {
    type Vtable = ILineBreak_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        154170777,
        31938,
        24404,
        [177, 6, 114, 134, 32, 193, 111, 118],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ILineBreak_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IParagraph(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IParagraph {
    type Vtable = IParagraph_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2664844407,
        12957,
        20527,
        [162, 87, 245, 131, 152, 237, 171, 81],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IParagraph_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut f64,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: f64,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IParagraphStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IParagraphStatics {
    type Vtable = IParagraphStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1320721073,
        26312,
        24512,
        [170, 95, 72, 200, 9, 44, 235, 95],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IParagraphStatics_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRun(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRun {
    type Vtable = IRun_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        529551929,
        14283,
        22795,
        [145, 50, 63, 251, 119, 65, 144, 110],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IRun_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut super::FlowDirection,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: super::FlowDirection,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRunStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRunStatics {
    type Vtable = IRunStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        85671003,
        30208,
        20901,
        [128, 197, 147, 235, 80, 253, 104, 79],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IRunStatics_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISpan(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpan {
    type Vtable = ISpan_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2444836173,
        20008,
        22457,
        [191, 251, 53, 102, 194, 163, 194, 161],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpan_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISpanFactory(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpanFactory {
    type Vtable = ISpanFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2800253974,
        49525,
        21960,
        [187, 211, 206, 64, 249, 208, 166, 128],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpanFactory_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        baseinterface: ::windows::runtime::RawPtr,
        innerinterface: *mut ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ITextElement(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITextElement {
    type Vtable = ITextElement_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2703407650,
        33599,
        21024,
        [164, 126, 108, 213, 7, 83, 26, 190],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextElement_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut f64,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: f64,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "UI_Xaml_Media")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Text::FontWeight,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::windows::UI::Text::FontWeight,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Text::FontStyle,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::windows::UI::Text::FontStyle,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Text::FontStretch,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::windows::UI::Text::FontStretch,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "UI_Xaml_Media")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Text::TextDecorations,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::windows::UI::Text::TextDecorations,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "UI_Xaml_Input")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut super::Input::KeyTipPlacementMode,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))] usize,
    #[cfg(feature = "UI_Xaml_Input")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: super::Input::KeyTipPlacementMode,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut f64,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: f64,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut f64,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: f64,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "UI_Xaml_Input")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        handler: ::windows::runtime::RawPtr,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "UI_Xaml_Input")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        handler: ::windows::runtime::RawPtr,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "UI_Xaml_Input")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        handler: ::windows::runtime::RawPtr,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        name: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ITextElementFactory(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITextElementFactory {
    type Vtable = ITextElementFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3746691989,
        42470,
        23318,
        [142, 136, 159, 124, 191, 162, 52, 177],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextElementFactory_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ITextElementOverrides(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITextElementOverrides {
    type Vtable = ITextElementOverrides_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1102058368,
        58527,
        24538,
        [140, 114, 172, 193, 172, 30, 145, 223],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextElementOverrides_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ITextElementStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITextElementStatics {
    type Vtable = ITextElementStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3384105241,
        57854,
        23245,
        [186, 199, 201, 215, 244, 19, 179, 92],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextElementStatics_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ITextHighlighter(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITextHighlighter {
    type Vtable = ITextHighlighter_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3075926113,
        7467,
        24431,
        [129, 253, 197, 26, 91, 192, 104, 255],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextHighlighter_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "UI_Xaml_Media")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ITextHighlighterBase(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITextHighlighterBase {
    type Vtable = ITextHighlighterBase_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1545710320,
        14871,
        21608,
        [138, 172, 190, 20, 219, 14, 216, 193],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextHighlighterBase_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ITextHighlighterBaseFactory(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITextHighlighterBaseFactory {
    type Vtable = ITextHighlighterBaseFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3767657569,
        21419,
        22942,
        [170, 234, 128, 10, 220, 114, 218, 79],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextHighlighterBaseFactory_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ITextHighlighterFactory(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITextHighlighterFactory {
    type Vtable = ITextHighlighterFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1774661919,
        49177,
        23443,
        [181, 17, 129, 65, 133, 67, 186, 183],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextHighlighterFactory_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        baseinterface: ::windows::runtime::RawPtr,
        innerinterface: *mut ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ITextHighlighterStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITextHighlighterStatics {
    type Vtable = ITextHighlighterStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1232405626,
        34733,
        20898,
        [151, 124, 231, 113, 222, 79, 64, 53],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextHighlighterStatics_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ITextPointer(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITextPointer {
    type Vtable = ITextPointer_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2217653125,
        60993,
        22832,
        [151, 155, 67, 143, 167, 82, 90, 81],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextPointer_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut LogicalDirection,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        direction: LogicalDirection,
        result__: *mut ::windows::Foundation::Rect,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        offset: i32,
        direction: LogicalDirection,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ITypography(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITypography {
    type Vtable = ITypography_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        4196917987,
        48734,
        23841,
        [154, 94, 144, 207, 16, 42, 248, 40],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ITypography_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ITypographyStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITypographyStatics {
    type Vtable = ITypographyStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1442727221,
        8485,
        21306,
        [173, 168, 39, 190, 43, 158, 17, 147],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ITypographyStatics_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        result__: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        value: i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        value: bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        result__: *mut super::FontEastAsianLanguage,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        value: super::FontEastAsianLanguage,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        result__: *mut super::FontEastAsianWidths,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        value: super::FontEastAsianWidths,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        value: bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        value: bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        value: bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        value: bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        result__: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        value: i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        result__: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        value: i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        value: bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        result__: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        value: i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        value: bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        value: bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        value: bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        value: bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        value: bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        value: bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        value: bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        value: bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        value: bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        value: bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        value: bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        value: bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        value: bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        value: bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        value: bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        value: bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        value: bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        value: bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        value: bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        value: bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        result__: *mut super::FontCapitals,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        value: super::FontCapitals,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        value: bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        value: bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        value: bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        value: bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        result__: *mut super::FontFraction,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        value: super::FontFraction,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        result__: *mut super::FontNumeralStyle,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        value: super::FontNumeralStyle,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        result__: *mut super::FontNumeralAlignment,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        value: super::FontNumeralAlignment,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        value: bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        value: bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        result__: *mut super::FontVariants,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        value: super::FontVariants,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IUnderline(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IUnderline {
    type Vtable = IUnderline_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1756032110,
        60017,
        24274,
        [184, 62, 145, 104, 75, 37, 239, 185],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IUnderline_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `UI_Xaml_Documents`*"]
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct Inline(pub ::windows::runtime::IInspectable);
impl Inline {
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn GetValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetValue<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>,
    >(
        &self,
        dp: Param0,
        value: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn ClearValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn ReadLocalValue<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn GetAnimationBaseValue<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn RegisterPropertyChangedCallback<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
        Param1: ::windows::runtime::IntoParam<'a, super::DependencyPropertyChangedCallback>,
    >(
        &self,
        dp: Param0,
        callback: Param1,
    ) -> ::windows::runtime::Result<i64> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: i64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                callback.into_param().abi(),
                &mut result__,
            )
            .from_abi::<i64>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn UnregisterPropertyChangedCallback<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
        token: i64,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).12)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                token,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn Dispatcher(&self) -> ::windows::runtime::Result<::windows::UI::Core::CoreDispatcher> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Core::CoreDispatcher>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Dispatching`*"]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::runtime::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn Name(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn FontSize(&self) -> ::windows::runtime::Result<f64> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetFontSize(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Media`*"]
    pub fn FontFamily(&self) -> ::windows::runtime::Result<super::Media::FontFamily> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::FontFamily>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Media`*"]
    pub fn SetFontFamily<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Media::FontFamily>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn FontWeight(&self) -> ::windows::runtime::Result<::windows::UI::Text::FontWeight> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::UI::Text::FontWeight = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::FontWeight>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetFontWeight<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::UI::Text::FontWeight>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).12)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn FontStyle(&self) -> ::windows::runtime::Result<::windows::UI::Text::FontStyle> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::UI::Text::FontStyle = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::FontStyle>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetFontStyle(
        &self,
        value: ::windows::UI::Text::FontStyle,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).14)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn FontStretch(&self) -> ::windows::runtime::Result<::windows::UI::Text::FontStretch> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::UI::Text::FontStretch = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::FontStretch>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetFontStretch(
        &self,
        value: ::windows::UI::Text::FontStretch,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).16)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn CharacterSpacing(&self) -> ::windows::runtime::Result<i32> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetCharacterSpacing(&self, value: i32) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).18)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Media`*"]
    pub fn Foreground(&self) -> ::windows::runtime::Result<super::Media::Brush> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Media`*"]
    pub fn SetForeground<'a, Param0: ::windows::runtime::IntoParam<'a, super::Media::Brush>>(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).20)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn Language(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).21)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetLanguage<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).22)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn IsTextScaleFactorEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).23)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetIsTextScaleFactorEnabled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).24)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn TextDecorations(
        &self,
    ) -> ::windows::runtime::Result<::windows::UI::Text::TextDecorations> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::UI::Text::TextDecorations = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).25)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::TextDecorations>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetTextDecorations(
        &self,
        value: ::windows::UI::Text::TextDecorations,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).26)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn ContentStart(&self) -> ::windows::runtime::Result<TextPointer> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).27)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn ContentEnd(&self) -> ::windows::runtime::Result<TextPointer> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).28)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn ElementStart(&self) -> ::windows::runtime::Result<TextPointer> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).29)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn ElementEnd(&self) -> ::windows::runtime::Result<TextPointer> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).30)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn AllowFocusOnInteraction(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).31)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetAllowFocusOnInteraction(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).32)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn AccessKey(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).33)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetAccessKey<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).34)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn ExitDisplayModeOnAccessKeyInvoked(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).35)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetExitDisplayModeOnAccessKeyInvoked(
        &self,
        value: bool,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).36)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn IsAccessKeyScope(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).37)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetIsAccessKeyScope(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).38)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn AccessKeyScopeOwner(&self) -> ::windows::runtime::Result<super::DependencyObject> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).39)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyObject>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetAccessKeyScopeOwner<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).40)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Input`*"]
    pub fn KeyTipPlacementMode(
        &self,
    ) -> ::windows::runtime::Result<super::Input::KeyTipPlacementMode> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: super::Input::KeyTipPlacementMode = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).41)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Input::KeyTipPlacementMode>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Input`*"]
    pub fn SetKeyTipPlacementMode(
        &self,
        value: super::Input::KeyTipPlacementMode,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).42)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn KeyTipHorizontalOffset(&self) -> ::windows::runtime::Result<f64> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).43)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetKeyTipHorizontalOffset(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).44)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn KeyTipVerticalOffset(&self) -> ::windows::runtime::Result<f64> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).45)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetKeyTipVerticalOffset(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).46)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn XamlRoot(&self) -> ::windows::runtime::Result<super::XamlRoot> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).47)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::XamlRoot>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetXamlRoot<'a, Param0: ::windows::runtime::IntoParam<'a, super::XamlRoot>>(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).48)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Input`*"]
    pub fn AccessKeyDisplayRequested<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                TextElement,
                super::Input::AccessKeyDisplayRequestedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).49)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn RemoveAccessKeyDisplayRequested<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).50)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Input`*"]
    pub fn AccessKeyDisplayDismissed<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                TextElement,
                super::Input::AccessKeyDisplayDismissedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).51)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn RemoveAccessKeyDisplayDismissed<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).52)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Input`*"]
    pub fn AccessKeyInvoked<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                TextElement,
                super::Input::AccessKeyInvokedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).53)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn RemoveAccessKeyInvoked<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).54)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn FindName<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(
        &self,
        name: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).55)(
                ::core::mem::transmute_copy(this),
                name.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for Inline {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Xaml.Documents.Inline;{813d427a-8980-5a79-a8fa-f27919cfb24f})",
    );
}
unsafe impl ::windows::runtime::Interface for Inline {
    type Vtable = IInline_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2168275578,
        35200,
        23161,
        [168, 250, 242, 121, 25, 207, 178, 79],
    );
}
impl ::windows::runtime::RuntimeName for Inline {
    const NAME: &'static str = "Microsoft.UI.Xaml.Documents.Inline";
}
impl ::core::convert::From<Inline> for ::windows::runtime::IUnknown {
    fn from(value: Inline) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&Inline> for ::windows::runtime::IUnknown {
    fn from(value: &Inline) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for Inline {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a Inline {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<Inline> for ::windows::runtime::IInspectable {
    fn from(value: Inline) -> Self {
        value.0
    }
}
impl ::core::convert::From<&Inline> for ::windows::runtime::IInspectable {
    fn from(value: &Inline) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for Inline {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a Inline {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<Inline> for TextElement {
    fn from(value: Inline) -> Self {
        ::core::convert::Into::<TextElement>::into(&value)
    }
}
impl ::core::convert::From<&Inline> for TextElement {
    fn from(value: &Inline) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, TextElement> for Inline {
    fn into_param(self) -> ::windows::runtime::Param<'a, TextElement> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<TextElement>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, TextElement> for &Inline {
    fn into_param(self) -> ::windows::runtime::Param<'a, TextElement> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<TextElement>::into(
            ::core::clone::Clone::clone(self),
        ))
    }
}
impl ::core::convert::From<Inline> for super::DependencyObject {
    fn from(value: Inline) -> Self {
        ::core::convert::Into::<super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&Inline> for super::DependencyObject {
    fn from(value: &Inline) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::DependencyObject> for Inline {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::DependencyObject> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(
            self,
        ))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::DependencyObject> for &Inline {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::DependencyObject> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(
            ::core::clone::Clone::clone(self),
        ))
    }
}
unsafe impl ::core::marker::Send for Inline {}
unsafe impl ::core::marker::Sync for Inline {}
#[doc = "*Required features: `UI_Xaml_Documents`*"]
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct InlineCollection(pub ::windows::runtime::IInspectable);
impl InlineCollection {
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn GetAt(&self, index: u32) -> ::windows::runtime::Result<Inline> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                index,
                &mut result__,
            )
            .from_abi::<Inline>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn Size(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn GetView(
        &self,
    ) -> ::windows::runtime::Result<::windows::Foundation::Collections::IVectorView<Inline>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Collections::IVectorView<Inline>>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn IndexOf<'a, Param0: ::windows::runtime::IntoParam<'a, Inline>>(
        &self,
        value: Param0,
        index: &mut u32,
    ) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
                index,
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetAt<'a, Param1: ::windows::runtime::IntoParam<'a, Inline>>(
        &self,
        index: u32,
        value: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                index,
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn InsertAt<'a, Param1: ::windows::runtime::IntoParam<'a, Inline>>(
        &self,
        index: u32,
        value: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).11)(
                ::core::mem::transmute_copy(this),
                index,
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn RemoveAt(&self, index: u32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).12)(
                ::core::mem::transmute_copy(this),
                index,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn Append<'a, Param0: ::windows::runtime::IntoParam<'a, Inline>>(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).13)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn RemoveAtEnd(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).14)(::core::mem::transmute_copy(this)).ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn Clear(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).15)(::core::mem::transmute_copy(this)).ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn GetMany(
        &self,
        startindex: u32,
        items: &mut [<Inline as ::windows::runtime::DefaultType>::DefaultType],
    ) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(
                ::core::mem::transmute_copy(this),
                startindex,
                items.len() as u32,
                ::core::mem::transmute_copy(&items),
                &mut result__,
            )
            .from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn ReplaceAll(
        &self,
        items: &[<Inline as ::windows::runtime::DefaultType>::DefaultType],
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).17)(
                ::core::mem::transmute_copy(this),
                items.len() as u32,
                ::core::mem::transmute(items.as_ptr()),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn First(
        &self,
    ) -> ::windows::runtime::Result<::windows::Foundation::Collections::IIterator<Inline>> {
        let this = &::windows::runtime::Interface::cast::<
            ::windows::Foundation::Collections::IIterable<Inline>,
        >(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Collections::IIterator<Inline>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for InlineCollection {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Documents.InlineCollection;pinterface({913337e9-11a1-4345-a3a2-4e7f956e222d};rc(Microsoft.UI.Xaml.Documents.Inline;{813d427a-8980-5a79-a8fa-f27919cfb24f})))" ) ;
}
unsafe impl ::windows::runtime::Interface for InlineCollection {
    type Vtable = ::windows::Foundation::Collections::IVector_abi<Inline>;
    const IID : :: windows :: runtime :: GUID = :: windows :: runtime :: GUID :: from_signature ( < ::windows::Foundation::Collections:: IVector :: < Inline > as :: windows :: runtime :: RuntimeType > :: SIGNATURE ) ;
}
impl ::windows::runtime::RuntimeName for InlineCollection {
    const NAME: &'static str = "Microsoft.UI.Xaml.Documents.InlineCollection";
}
impl ::core::convert::From<InlineCollection> for ::windows::runtime::IUnknown {
    fn from(value: InlineCollection) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&InlineCollection> for ::windows::runtime::IUnknown {
    fn from(value: &InlineCollection) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for InlineCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a InlineCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<InlineCollection> for ::windows::runtime::IInspectable {
    fn from(value: InlineCollection) -> Self {
        value.0
    }
}
impl ::core::convert::From<&InlineCollection> for ::windows::runtime::IInspectable {
    fn from(value: &InlineCollection) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for InlineCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a InlineCollection
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<InlineCollection>
    for ::windows::Foundation::Collections::IVector<Inline>
{
    fn from(value: InlineCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InlineCollection>
    for ::windows::Foundation::Collections::IVector<Inline>
{
    fn from(value: &InlineCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::Foundation::Collections::IVector<Inline>>
    for InlineCollection
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<'a, ::windows::Foundation::Collections::IVector<Inline>> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::Foundation::Collections::IVector<Inline>>
    for &InlineCollection
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<'a, ::windows::Foundation::Collections::IVector<Inline>> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<InlineCollection>
    for ::windows::Foundation::Collections::IIterable<Inline>
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: InlineCollection) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&InlineCollection>
    for ::windows::Foundation::Collections::IIterable<Inline>
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: &InlineCollection) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::Foundation::Collections::IIterable<Inline>>
    for InlineCollection
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<'a, ::windows::Foundation::Collections::IIterable<Inline>> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::Foundation::Collections::IIterable<Inline>>
    for &InlineCollection
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<'a, ::windows::Foundation::Collections::IIterable<Inline>> {
        ::core::convert::TryInto::<::windows::Foundation::Collections::IIterable<Inline>>::try_into(
            self,
        )
        .map(::windows::runtime::Param::Owned)
        .unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::core::marker::Send for InlineCollection {}
unsafe impl ::core::marker::Sync for InlineCollection {}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::core::iter::IntoIterator for InlineCollection {
    type Item = Inline;
    type IntoIter = ::windows::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::core::iter::IntoIterator for &InlineCollection {
    type Item = Inline;
    type IntoIter = ::windows::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::windows::Foundation::Collections::VectorIterator::new(
            ::core::convert::TryInto::try_into(self).ok(),
        )
    }
}
#[doc = "*Required features: `UI_Xaml_Documents`*"]
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct InlineUIContainer(pub ::windows::runtime::IInspectable);
impl InlineUIContainer {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            InlineUIContainer,
            ::windows::runtime::IActivationFactory,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn Child(&self) -> ::windows::runtime::Result<super::UIElement> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::UIElement>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetChild<'a, Param0: ::windows::runtime::IntoParam<'a, super::UIElement>>(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn GetValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetValue<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>,
    >(
        &self,
        dp: Param0,
        value: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn ClearValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn ReadLocalValue<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn GetAnimationBaseValue<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn RegisterPropertyChangedCallback<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
        Param1: ::windows::runtime::IntoParam<'a, super::DependencyPropertyChangedCallback>,
    >(
        &self,
        dp: Param0,
        callback: Param1,
    ) -> ::windows::runtime::Result<i64> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: i64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                callback.into_param().abi(),
                &mut result__,
            )
            .from_abi::<i64>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn UnregisterPropertyChangedCallback<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
        token: i64,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).12)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                token,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn Dispatcher(&self) -> ::windows::runtime::Result<::windows::UI::Core::CoreDispatcher> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Core::CoreDispatcher>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Dispatching`*"]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::runtime::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn Name(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn FontSize(&self) -> ::windows::runtime::Result<f64> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetFontSize(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Media`*"]
    pub fn FontFamily(&self) -> ::windows::runtime::Result<super::Media::FontFamily> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::FontFamily>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Media`*"]
    pub fn SetFontFamily<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Media::FontFamily>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn FontWeight(&self) -> ::windows::runtime::Result<::windows::UI::Text::FontWeight> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::UI::Text::FontWeight = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::FontWeight>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetFontWeight<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::UI::Text::FontWeight>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).12)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn FontStyle(&self) -> ::windows::runtime::Result<::windows::UI::Text::FontStyle> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::UI::Text::FontStyle = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::FontStyle>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetFontStyle(
        &self,
        value: ::windows::UI::Text::FontStyle,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).14)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn FontStretch(&self) -> ::windows::runtime::Result<::windows::UI::Text::FontStretch> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::UI::Text::FontStretch = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::FontStretch>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetFontStretch(
        &self,
        value: ::windows::UI::Text::FontStretch,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).16)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn CharacterSpacing(&self) -> ::windows::runtime::Result<i32> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetCharacterSpacing(&self, value: i32) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).18)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Media`*"]
    pub fn Foreground(&self) -> ::windows::runtime::Result<super::Media::Brush> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Media`*"]
    pub fn SetForeground<'a, Param0: ::windows::runtime::IntoParam<'a, super::Media::Brush>>(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).20)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn Language(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).21)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetLanguage<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).22)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn IsTextScaleFactorEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).23)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetIsTextScaleFactorEnabled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).24)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn TextDecorations(
        &self,
    ) -> ::windows::runtime::Result<::windows::UI::Text::TextDecorations> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::UI::Text::TextDecorations = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).25)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::TextDecorations>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetTextDecorations(
        &self,
        value: ::windows::UI::Text::TextDecorations,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).26)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn ContentStart(&self) -> ::windows::runtime::Result<TextPointer> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).27)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn ContentEnd(&self) -> ::windows::runtime::Result<TextPointer> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).28)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn ElementStart(&self) -> ::windows::runtime::Result<TextPointer> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).29)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn ElementEnd(&self) -> ::windows::runtime::Result<TextPointer> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).30)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn AllowFocusOnInteraction(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).31)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetAllowFocusOnInteraction(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).32)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn AccessKey(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).33)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetAccessKey<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).34)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn ExitDisplayModeOnAccessKeyInvoked(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).35)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetExitDisplayModeOnAccessKeyInvoked(
        &self,
        value: bool,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).36)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn IsAccessKeyScope(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).37)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetIsAccessKeyScope(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).38)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn AccessKeyScopeOwner(&self) -> ::windows::runtime::Result<super::DependencyObject> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).39)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyObject>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetAccessKeyScopeOwner<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).40)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Input`*"]
    pub fn KeyTipPlacementMode(
        &self,
    ) -> ::windows::runtime::Result<super::Input::KeyTipPlacementMode> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: super::Input::KeyTipPlacementMode = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).41)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Input::KeyTipPlacementMode>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Input`*"]
    pub fn SetKeyTipPlacementMode(
        &self,
        value: super::Input::KeyTipPlacementMode,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).42)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn KeyTipHorizontalOffset(&self) -> ::windows::runtime::Result<f64> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).43)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetKeyTipHorizontalOffset(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).44)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn KeyTipVerticalOffset(&self) -> ::windows::runtime::Result<f64> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).45)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetKeyTipVerticalOffset(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).46)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn XamlRoot(&self) -> ::windows::runtime::Result<super::XamlRoot> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).47)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::XamlRoot>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetXamlRoot<'a, Param0: ::windows::runtime::IntoParam<'a, super::XamlRoot>>(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).48)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Input`*"]
    pub fn AccessKeyDisplayRequested<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                TextElement,
                super::Input::AccessKeyDisplayRequestedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).49)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn RemoveAccessKeyDisplayRequested<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).50)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Input`*"]
    pub fn AccessKeyDisplayDismissed<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                TextElement,
                super::Input::AccessKeyDisplayDismissedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).51)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn RemoveAccessKeyDisplayDismissed<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).52)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Input`*"]
    pub fn AccessKeyInvoked<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                TextElement,
                super::Input::AccessKeyInvokedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).53)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn RemoveAccessKeyInvoked<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).54)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn FindName<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(
        &self,
        name: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).55)(
                ::core::mem::transmute_copy(this),
                name.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for InlineUIContainer {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Xaml.Documents.InlineUIContainer;{d529bef6-c05a-5bad-85e8-640127cf86f5})",
    );
}
unsafe impl ::windows::runtime::Interface for InlineUIContainer {
    type Vtable = IInlineUIContainer_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3576282870,
        49242,
        23469,
        [133, 232, 100, 1, 39, 207, 134, 245],
    );
}
impl ::windows::runtime::RuntimeName for InlineUIContainer {
    const NAME: &'static str = "Microsoft.UI.Xaml.Documents.InlineUIContainer";
}
impl ::core::convert::From<InlineUIContainer> for ::windows::runtime::IUnknown {
    fn from(value: InlineUIContainer) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&InlineUIContainer> for ::windows::runtime::IUnknown {
    fn from(value: &InlineUIContainer) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for InlineUIContainer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a InlineUIContainer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<InlineUIContainer> for ::windows::runtime::IInspectable {
    fn from(value: InlineUIContainer) -> Self {
        value.0
    }
}
impl ::core::convert::From<&InlineUIContainer> for ::windows::runtime::IInspectable {
    fn from(value: &InlineUIContainer) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for InlineUIContainer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a InlineUIContainer
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<InlineUIContainer> for Inline {
    fn from(value: InlineUIContainer) -> Self {
        ::core::convert::Into::<Inline>::into(&value)
    }
}
impl ::core::convert::From<&InlineUIContainer> for Inline {
    fn from(value: &InlineUIContainer) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, Inline> for InlineUIContainer {
    fn into_param(self) -> ::windows::runtime::Param<'a, Inline> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<Inline>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, Inline> for &InlineUIContainer {
    fn into_param(self) -> ::windows::runtime::Param<'a, Inline> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<Inline>::into(
            ::core::clone::Clone::clone(self),
        ))
    }
}
impl ::core::convert::From<InlineUIContainer> for TextElement {
    fn from(value: InlineUIContainer) -> Self {
        ::core::convert::Into::<TextElement>::into(&value)
    }
}
impl ::core::convert::From<&InlineUIContainer> for TextElement {
    fn from(value: &InlineUIContainer) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, TextElement> for InlineUIContainer {
    fn into_param(self) -> ::windows::runtime::Param<'a, TextElement> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<TextElement>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, TextElement> for &InlineUIContainer {
    fn into_param(self) -> ::windows::runtime::Param<'a, TextElement> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<TextElement>::into(
            ::core::clone::Clone::clone(self),
        ))
    }
}
impl ::core::convert::From<InlineUIContainer> for super::DependencyObject {
    fn from(value: InlineUIContainer) -> Self {
        ::core::convert::Into::<super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&InlineUIContainer> for super::DependencyObject {
    fn from(value: &InlineUIContainer) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::DependencyObject> for InlineUIContainer {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::DependencyObject> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(
            self,
        ))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::DependencyObject> for &InlineUIContainer {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::DependencyObject> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(
            ::core::clone::Clone::clone(self),
        ))
    }
}
unsafe impl ::core::marker::Send for InlineUIContainer {}
unsafe impl ::core::marker::Sync for InlineUIContainer {}
#[doc = "*Required features: `UI_Xaml_Documents`*"]
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct Italic(pub ::windows::runtime::IInspectable);
impl Italic {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            Italic,
            ::windows::runtime::IActivationFactory,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn GetValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetValue<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>,
    >(
        &self,
        dp: Param0,
        value: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn ClearValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn ReadLocalValue<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn GetAnimationBaseValue<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn RegisterPropertyChangedCallback<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
        Param1: ::windows::runtime::IntoParam<'a, super::DependencyPropertyChangedCallback>,
    >(
        &self,
        dp: Param0,
        callback: Param1,
    ) -> ::windows::runtime::Result<i64> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: i64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                callback.into_param().abi(),
                &mut result__,
            )
            .from_abi::<i64>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn UnregisterPropertyChangedCallback<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
        token: i64,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).12)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                token,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn Dispatcher(&self) -> ::windows::runtime::Result<::windows::UI::Core::CoreDispatcher> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Core::CoreDispatcher>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Dispatching`*"]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::runtime::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn Inlines(&self) -> ::windows::runtime::Result<InlineCollection> {
        let this = &::windows::runtime::Interface::cast::<ISpan>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<InlineCollection>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetInlines<'a, Param0: ::windows::runtime::IntoParam<'a, InlineCollection>>(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ISpan>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn Name(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn FontSize(&self) -> ::windows::runtime::Result<f64> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetFontSize(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Media`*"]
    pub fn FontFamily(&self) -> ::windows::runtime::Result<super::Media::FontFamily> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::FontFamily>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Media`*"]
    pub fn SetFontFamily<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Media::FontFamily>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn FontWeight(&self) -> ::windows::runtime::Result<::windows::UI::Text::FontWeight> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::UI::Text::FontWeight = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::FontWeight>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetFontWeight<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::UI::Text::FontWeight>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).12)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn FontStyle(&self) -> ::windows::runtime::Result<::windows::UI::Text::FontStyle> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::UI::Text::FontStyle = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::FontStyle>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetFontStyle(
        &self,
        value: ::windows::UI::Text::FontStyle,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).14)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn FontStretch(&self) -> ::windows::runtime::Result<::windows::UI::Text::FontStretch> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::UI::Text::FontStretch = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::FontStretch>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetFontStretch(
        &self,
        value: ::windows::UI::Text::FontStretch,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).16)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn CharacterSpacing(&self) -> ::windows::runtime::Result<i32> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetCharacterSpacing(&self, value: i32) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).18)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Media`*"]
    pub fn Foreground(&self) -> ::windows::runtime::Result<super::Media::Brush> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Media`*"]
    pub fn SetForeground<'a, Param0: ::windows::runtime::IntoParam<'a, super::Media::Brush>>(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).20)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn Language(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).21)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetLanguage<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).22)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn IsTextScaleFactorEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).23)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetIsTextScaleFactorEnabled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).24)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn TextDecorations(
        &self,
    ) -> ::windows::runtime::Result<::windows::UI::Text::TextDecorations> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::UI::Text::TextDecorations = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).25)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::TextDecorations>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetTextDecorations(
        &self,
        value: ::windows::UI::Text::TextDecorations,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).26)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn ContentStart(&self) -> ::windows::runtime::Result<TextPointer> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).27)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn ContentEnd(&self) -> ::windows::runtime::Result<TextPointer> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).28)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn ElementStart(&self) -> ::windows::runtime::Result<TextPointer> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).29)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn ElementEnd(&self) -> ::windows::runtime::Result<TextPointer> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).30)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn AllowFocusOnInteraction(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).31)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetAllowFocusOnInteraction(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).32)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn AccessKey(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).33)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetAccessKey<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).34)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn ExitDisplayModeOnAccessKeyInvoked(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).35)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetExitDisplayModeOnAccessKeyInvoked(
        &self,
        value: bool,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).36)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn IsAccessKeyScope(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).37)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetIsAccessKeyScope(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).38)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn AccessKeyScopeOwner(&self) -> ::windows::runtime::Result<super::DependencyObject> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).39)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyObject>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetAccessKeyScopeOwner<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).40)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Input`*"]
    pub fn KeyTipPlacementMode(
        &self,
    ) -> ::windows::runtime::Result<super::Input::KeyTipPlacementMode> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: super::Input::KeyTipPlacementMode = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).41)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Input::KeyTipPlacementMode>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Input`*"]
    pub fn SetKeyTipPlacementMode(
        &self,
        value: super::Input::KeyTipPlacementMode,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).42)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn KeyTipHorizontalOffset(&self) -> ::windows::runtime::Result<f64> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).43)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetKeyTipHorizontalOffset(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).44)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn KeyTipVerticalOffset(&self) -> ::windows::runtime::Result<f64> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).45)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetKeyTipVerticalOffset(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).46)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn XamlRoot(&self) -> ::windows::runtime::Result<super::XamlRoot> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).47)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::XamlRoot>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetXamlRoot<'a, Param0: ::windows::runtime::IntoParam<'a, super::XamlRoot>>(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).48)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Input`*"]
    pub fn AccessKeyDisplayRequested<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                TextElement,
                super::Input::AccessKeyDisplayRequestedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).49)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn RemoveAccessKeyDisplayRequested<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).50)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Input`*"]
    pub fn AccessKeyDisplayDismissed<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                TextElement,
                super::Input::AccessKeyDisplayDismissedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).51)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn RemoveAccessKeyDisplayDismissed<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).52)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Input`*"]
    pub fn AccessKeyInvoked<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                TextElement,
                super::Input::AccessKeyInvokedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).53)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn RemoveAccessKeyInvoked<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).54)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn FindName<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(
        &self,
        name: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).55)(
                ::core::mem::transmute_copy(this),
                name.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for Italic {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Xaml.Documents.Italic;{ca3cbebd-7a8d-5d7a-8fdf-538e8a680f6c})",
    );
}
unsafe impl ::windows::runtime::Interface for Italic {
    type Vtable = IItalic_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3392978621,
        31373,
        23930,
        [143, 223, 83, 142, 138, 104, 15, 108],
    );
}
impl ::windows::runtime::RuntimeName for Italic {
    const NAME: &'static str = "Microsoft.UI.Xaml.Documents.Italic";
}
impl ::core::convert::From<Italic> for ::windows::runtime::IUnknown {
    fn from(value: Italic) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&Italic> for ::windows::runtime::IUnknown {
    fn from(value: &Italic) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for Italic {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a Italic {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<Italic> for ::windows::runtime::IInspectable {
    fn from(value: Italic) -> Self {
        value.0
    }
}
impl ::core::convert::From<&Italic> for ::windows::runtime::IInspectable {
    fn from(value: &Italic) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for Italic {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a Italic {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<Italic> for Span {
    fn from(value: Italic) -> Self {
        ::core::convert::Into::<Span>::into(&value)
    }
}
impl ::core::convert::From<&Italic> for Span {
    fn from(value: &Italic) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, Span> for Italic {
    fn into_param(self) -> ::windows::runtime::Param<'a, Span> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<Span>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, Span> for &Italic {
    fn into_param(self) -> ::windows::runtime::Param<'a, Span> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<Span>::into(
            ::core::clone::Clone::clone(self),
        ))
    }
}
impl ::core::convert::From<Italic> for Inline {
    fn from(value: Italic) -> Self {
        ::core::convert::Into::<Inline>::into(&value)
    }
}
impl ::core::convert::From<&Italic> for Inline {
    fn from(value: &Italic) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, Inline> for Italic {
    fn into_param(self) -> ::windows::runtime::Param<'a, Inline> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<Inline>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, Inline> for &Italic {
    fn into_param(self) -> ::windows::runtime::Param<'a, Inline> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<Inline>::into(
            ::core::clone::Clone::clone(self),
        ))
    }
}
impl ::core::convert::From<Italic> for TextElement {
    fn from(value: Italic) -> Self {
        ::core::convert::Into::<TextElement>::into(&value)
    }
}
impl ::core::convert::From<&Italic> for TextElement {
    fn from(value: &Italic) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, TextElement> for Italic {
    fn into_param(self) -> ::windows::runtime::Param<'a, TextElement> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<TextElement>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, TextElement> for &Italic {
    fn into_param(self) -> ::windows::runtime::Param<'a, TextElement> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<TextElement>::into(
            ::core::clone::Clone::clone(self),
        ))
    }
}
impl ::core::convert::From<Italic> for super::DependencyObject {
    fn from(value: Italic) -> Self {
        ::core::convert::Into::<super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&Italic> for super::DependencyObject {
    fn from(value: &Italic) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::DependencyObject> for Italic {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::DependencyObject> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(
            self,
        ))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::DependencyObject> for &Italic {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::DependencyObject> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(
            ::core::clone::Clone::clone(self),
        ))
    }
}
unsafe impl ::core::marker::Send for Italic {}
unsafe impl ::core::marker::Sync for Italic {}
#[doc = "*Required features: `UI_Xaml_Documents`*"]
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct LineBreak(pub ::windows::runtime::IInspectable);
impl LineBreak {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            LineBreak,
            ::windows::runtime::IActivationFactory,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn GetValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetValue<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>,
    >(
        &self,
        dp: Param0,
        value: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn ClearValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn ReadLocalValue<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn GetAnimationBaseValue<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn RegisterPropertyChangedCallback<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
        Param1: ::windows::runtime::IntoParam<'a, super::DependencyPropertyChangedCallback>,
    >(
        &self,
        dp: Param0,
        callback: Param1,
    ) -> ::windows::runtime::Result<i64> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: i64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                callback.into_param().abi(),
                &mut result__,
            )
            .from_abi::<i64>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn UnregisterPropertyChangedCallback<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
        token: i64,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).12)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                token,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn Dispatcher(&self) -> ::windows::runtime::Result<::windows::UI::Core::CoreDispatcher> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Core::CoreDispatcher>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Dispatching`*"]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::runtime::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn Name(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn FontSize(&self) -> ::windows::runtime::Result<f64> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetFontSize(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Media`*"]
    pub fn FontFamily(&self) -> ::windows::runtime::Result<super::Media::FontFamily> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::FontFamily>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Media`*"]
    pub fn SetFontFamily<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Media::FontFamily>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn FontWeight(&self) -> ::windows::runtime::Result<::windows::UI::Text::FontWeight> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::UI::Text::FontWeight = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::FontWeight>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetFontWeight<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::UI::Text::FontWeight>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).12)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn FontStyle(&self) -> ::windows::runtime::Result<::windows::UI::Text::FontStyle> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::UI::Text::FontStyle = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::FontStyle>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetFontStyle(
        &self,
        value: ::windows::UI::Text::FontStyle,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).14)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn FontStretch(&self) -> ::windows::runtime::Result<::windows::UI::Text::FontStretch> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::UI::Text::FontStretch = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::FontStretch>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetFontStretch(
        &self,
        value: ::windows::UI::Text::FontStretch,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).16)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn CharacterSpacing(&self) -> ::windows::runtime::Result<i32> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetCharacterSpacing(&self, value: i32) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).18)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Media`*"]
    pub fn Foreground(&self) -> ::windows::runtime::Result<super::Media::Brush> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Media`*"]
    pub fn SetForeground<'a, Param0: ::windows::runtime::IntoParam<'a, super::Media::Brush>>(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).20)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn Language(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).21)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetLanguage<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).22)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn IsTextScaleFactorEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).23)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetIsTextScaleFactorEnabled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).24)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn TextDecorations(
        &self,
    ) -> ::windows::runtime::Result<::windows::UI::Text::TextDecorations> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::UI::Text::TextDecorations = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).25)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::TextDecorations>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetTextDecorations(
        &self,
        value: ::windows::UI::Text::TextDecorations,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).26)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn ContentStart(&self) -> ::windows::runtime::Result<TextPointer> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).27)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn ContentEnd(&self) -> ::windows::runtime::Result<TextPointer> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).28)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn ElementStart(&self) -> ::windows::runtime::Result<TextPointer> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).29)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn ElementEnd(&self) -> ::windows::runtime::Result<TextPointer> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).30)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn AllowFocusOnInteraction(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).31)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetAllowFocusOnInteraction(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).32)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn AccessKey(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).33)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetAccessKey<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).34)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn ExitDisplayModeOnAccessKeyInvoked(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).35)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetExitDisplayModeOnAccessKeyInvoked(
        &self,
        value: bool,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).36)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn IsAccessKeyScope(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).37)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetIsAccessKeyScope(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).38)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn AccessKeyScopeOwner(&self) -> ::windows::runtime::Result<super::DependencyObject> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).39)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyObject>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetAccessKeyScopeOwner<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).40)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Input`*"]
    pub fn KeyTipPlacementMode(
        &self,
    ) -> ::windows::runtime::Result<super::Input::KeyTipPlacementMode> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: super::Input::KeyTipPlacementMode = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).41)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Input::KeyTipPlacementMode>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Input`*"]
    pub fn SetKeyTipPlacementMode(
        &self,
        value: super::Input::KeyTipPlacementMode,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).42)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn KeyTipHorizontalOffset(&self) -> ::windows::runtime::Result<f64> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).43)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetKeyTipHorizontalOffset(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).44)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn KeyTipVerticalOffset(&self) -> ::windows::runtime::Result<f64> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).45)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetKeyTipVerticalOffset(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).46)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn XamlRoot(&self) -> ::windows::runtime::Result<super::XamlRoot> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).47)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::XamlRoot>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetXamlRoot<'a, Param0: ::windows::runtime::IntoParam<'a, super::XamlRoot>>(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).48)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Input`*"]
    pub fn AccessKeyDisplayRequested<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                TextElement,
                super::Input::AccessKeyDisplayRequestedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).49)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn RemoveAccessKeyDisplayRequested<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).50)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Input`*"]
    pub fn AccessKeyDisplayDismissed<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                TextElement,
                super::Input::AccessKeyDisplayDismissedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).51)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn RemoveAccessKeyDisplayDismissed<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).52)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Input`*"]
    pub fn AccessKeyInvoked<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                TextElement,
                super::Input::AccessKeyInvokedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).53)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn RemoveAccessKeyInvoked<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).54)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn FindName<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(
        &self,
        name: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).55)(
                ::core::mem::transmute_copy(this),
                name.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for LineBreak {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Xaml.Documents.LineBreak;{09307599-7cc2-5f54-b106-728620c16f76})",
    );
}
unsafe impl ::windows::runtime::Interface for LineBreak {
    type Vtable = ILineBreak_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        154170777,
        31938,
        24404,
        [177, 6, 114, 134, 32, 193, 111, 118],
    );
}
impl ::windows::runtime::RuntimeName for LineBreak {
    const NAME: &'static str = "Microsoft.UI.Xaml.Documents.LineBreak";
}
impl ::core::convert::From<LineBreak> for ::windows::runtime::IUnknown {
    fn from(value: LineBreak) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&LineBreak> for ::windows::runtime::IUnknown {
    fn from(value: &LineBreak) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for LineBreak {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a LineBreak {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<LineBreak> for ::windows::runtime::IInspectable {
    fn from(value: LineBreak) -> Self {
        value.0
    }
}
impl ::core::convert::From<&LineBreak> for ::windows::runtime::IInspectable {
    fn from(value: &LineBreak) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for LineBreak {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a LineBreak {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<LineBreak> for Inline {
    fn from(value: LineBreak) -> Self {
        ::core::convert::Into::<Inline>::into(&value)
    }
}
impl ::core::convert::From<&LineBreak> for Inline {
    fn from(value: &LineBreak) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, Inline> for LineBreak {
    fn into_param(self) -> ::windows::runtime::Param<'a, Inline> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<Inline>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, Inline> for &LineBreak {
    fn into_param(self) -> ::windows::runtime::Param<'a, Inline> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<Inline>::into(
            ::core::clone::Clone::clone(self),
        ))
    }
}
impl ::core::convert::From<LineBreak> for TextElement {
    fn from(value: LineBreak) -> Self {
        ::core::convert::Into::<TextElement>::into(&value)
    }
}
impl ::core::convert::From<&LineBreak> for TextElement {
    fn from(value: &LineBreak) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, TextElement> for LineBreak {
    fn into_param(self) -> ::windows::runtime::Param<'a, TextElement> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<TextElement>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, TextElement> for &LineBreak {
    fn into_param(self) -> ::windows::runtime::Param<'a, TextElement> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<TextElement>::into(
            ::core::clone::Clone::clone(self),
        ))
    }
}
impl ::core::convert::From<LineBreak> for super::DependencyObject {
    fn from(value: LineBreak) -> Self {
        ::core::convert::Into::<super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&LineBreak> for super::DependencyObject {
    fn from(value: &LineBreak) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::DependencyObject> for LineBreak {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::DependencyObject> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(
            self,
        ))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::DependencyObject> for &LineBreak {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::DependencyObject> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(
            ::core::clone::Clone::clone(self),
        ))
    }
}
unsafe impl ::core::marker::Send for LineBreak {}
unsafe impl ::core::marker::Sync for LineBreak {}
#[doc = "*Required features: `UI_Xaml_Documents`*"]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: marker :: Copy,
    :: core :: clone :: Clone,
    :: core :: default :: Default,
    :: core :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct LogicalDirection(pub i32);
impl LogicalDirection {
    pub const Backward: LogicalDirection = LogicalDirection(0i32);
    pub const Forward: LogicalDirection = LogicalDirection(1i32);
}
impl ::core::convert::From<i32> for LogicalDirection {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for LogicalDirection {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for LogicalDirection {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Xaml.Documents.LogicalDirection;i4)",
    );
}
impl ::windows::runtime::DefaultType for LogicalDirection {
    type DefaultType = Self;
}
#[doc = "*Required features: `UI_Xaml_Documents`*"]
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct Paragraph(pub ::windows::runtime::IInspectable);
impl Paragraph {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            Paragraph,
            ::windows::runtime::IActivationFactory,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn Inlines(&self) -> ::windows::runtime::Result<InlineCollection> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<InlineCollection>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn TextIndent(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetTextIndent(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn TextIndentProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::IParagraphStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn GetValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetValue<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>,
    >(
        &self,
        dp: Param0,
        value: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn ClearValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn ReadLocalValue<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn GetAnimationBaseValue<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn RegisterPropertyChangedCallback<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
        Param1: ::windows::runtime::IntoParam<'a, super::DependencyPropertyChangedCallback>,
    >(
        &self,
        dp: Param0,
        callback: Param1,
    ) -> ::windows::runtime::Result<i64> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: i64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                callback.into_param().abi(),
                &mut result__,
            )
            .from_abi::<i64>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn UnregisterPropertyChangedCallback<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
        token: i64,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).12)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                token,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn Dispatcher(&self) -> ::windows::runtime::Result<::windows::UI::Core::CoreDispatcher> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Core::CoreDispatcher>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Dispatching`*"]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::runtime::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn TextAlignment(&self) -> ::windows::runtime::Result<super::TextAlignment> {
        let this = &::windows::runtime::Interface::cast::<IBlock>(self)?;
        unsafe {
            let mut result__: super::TextAlignment = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::TextAlignment>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetTextAlignment(&self, value: super::TextAlignment) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IBlock>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn HorizontalTextAlignment(&self) -> ::windows::runtime::Result<super::TextAlignment> {
        let this = &::windows::runtime::Interface::cast::<IBlock>(self)?;
        unsafe {
            let mut result__: super::TextAlignment = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::TextAlignment>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetHorizontalTextAlignment(
        &self,
        value: super::TextAlignment,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IBlock>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn LineHeight(&self) -> ::windows::runtime::Result<f64> {
        let this = &::windows::runtime::Interface::cast::<IBlock>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetLineHeight(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IBlock>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).11)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn LineStackingStrategy(&self) -> ::windows::runtime::Result<super::LineStackingStrategy> {
        let this = &::windows::runtime::Interface::cast::<IBlock>(self)?;
        unsafe {
            let mut result__: super::LineStackingStrategy = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::LineStackingStrategy>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetLineStackingStrategy(
        &self,
        value: super::LineStackingStrategy,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IBlock>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).13)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn Margin(&self) -> ::windows::runtime::Result<super::Thickness> {
        let this = &::windows::runtime::Interface::cast::<IBlock>(self)?;
        unsafe {
            let mut result__: super::Thickness = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Thickness>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetMargin<'a, Param0: ::windows::runtime::IntoParam<'a, super::Thickness>>(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IBlock>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).15)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn Name(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn FontSize(&self) -> ::windows::runtime::Result<f64> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetFontSize(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Media`*"]
    pub fn FontFamily(&self) -> ::windows::runtime::Result<super::Media::FontFamily> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::FontFamily>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Media`*"]
    pub fn SetFontFamily<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Media::FontFamily>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn FontWeight(&self) -> ::windows::runtime::Result<::windows::UI::Text::FontWeight> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::UI::Text::FontWeight = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::FontWeight>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetFontWeight<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::UI::Text::FontWeight>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).12)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn FontStyle(&self) -> ::windows::runtime::Result<::windows::UI::Text::FontStyle> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::UI::Text::FontStyle = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::FontStyle>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetFontStyle(
        &self,
        value: ::windows::UI::Text::FontStyle,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).14)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn FontStretch(&self) -> ::windows::runtime::Result<::windows::UI::Text::FontStretch> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::UI::Text::FontStretch = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::FontStretch>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetFontStretch(
        &self,
        value: ::windows::UI::Text::FontStretch,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).16)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn CharacterSpacing(&self) -> ::windows::runtime::Result<i32> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetCharacterSpacing(&self, value: i32) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).18)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Media`*"]
    pub fn Foreground(&self) -> ::windows::runtime::Result<super::Media::Brush> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Media`*"]
    pub fn SetForeground<'a, Param0: ::windows::runtime::IntoParam<'a, super::Media::Brush>>(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).20)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn Language(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).21)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetLanguage<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).22)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn IsTextScaleFactorEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).23)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetIsTextScaleFactorEnabled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).24)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn TextDecorations(
        &self,
    ) -> ::windows::runtime::Result<::windows::UI::Text::TextDecorations> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::UI::Text::TextDecorations = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).25)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::TextDecorations>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetTextDecorations(
        &self,
        value: ::windows::UI::Text::TextDecorations,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).26)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn ContentStart(&self) -> ::windows::runtime::Result<TextPointer> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).27)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn ContentEnd(&self) -> ::windows::runtime::Result<TextPointer> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).28)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn ElementStart(&self) -> ::windows::runtime::Result<TextPointer> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).29)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn ElementEnd(&self) -> ::windows::runtime::Result<TextPointer> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).30)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn AllowFocusOnInteraction(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).31)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetAllowFocusOnInteraction(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).32)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn AccessKey(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).33)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetAccessKey<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).34)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn ExitDisplayModeOnAccessKeyInvoked(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).35)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetExitDisplayModeOnAccessKeyInvoked(
        &self,
        value: bool,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).36)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn IsAccessKeyScope(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).37)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetIsAccessKeyScope(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).38)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn AccessKeyScopeOwner(&self) -> ::windows::runtime::Result<super::DependencyObject> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).39)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyObject>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetAccessKeyScopeOwner<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).40)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Input`*"]
    pub fn KeyTipPlacementMode(
        &self,
    ) -> ::windows::runtime::Result<super::Input::KeyTipPlacementMode> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: super::Input::KeyTipPlacementMode = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).41)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Input::KeyTipPlacementMode>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Input`*"]
    pub fn SetKeyTipPlacementMode(
        &self,
        value: super::Input::KeyTipPlacementMode,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).42)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn KeyTipHorizontalOffset(&self) -> ::windows::runtime::Result<f64> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).43)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetKeyTipHorizontalOffset(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).44)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn KeyTipVerticalOffset(&self) -> ::windows::runtime::Result<f64> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).45)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetKeyTipVerticalOffset(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).46)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn XamlRoot(&self) -> ::windows::runtime::Result<super::XamlRoot> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).47)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::XamlRoot>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetXamlRoot<'a, Param0: ::windows::runtime::IntoParam<'a, super::XamlRoot>>(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).48)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Input`*"]
    pub fn AccessKeyDisplayRequested<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                TextElement,
                super::Input::AccessKeyDisplayRequestedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).49)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn RemoveAccessKeyDisplayRequested<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).50)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Input`*"]
    pub fn AccessKeyDisplayDismissed<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                TextElement,
                super::Input::AccessKeyDisplayDismissedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).51)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn RemoveAccessKeyDisplayDismissed<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).52)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Input`*"]
    pub fn AccessKeyInvoked<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                TextElement,
                super::Input::AccessKeyInvokedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).53)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn RemoveAccessKeyInvoked<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).54)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn FindName<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(
        &self,
        name: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).55)(
                ::core::mem::transmute_copy(this),
                name.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    pub fn IParagraphStatics<R, F: FnOnce(&IParagraphStatics) -> ::windows::runtime::Result<R>>(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<Paragraph, IParagraphStatics> =
            ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for Paragraph {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Xaml.Documents.Paragraph;{9ed64c77-329d-502f-a257-f58398edab51})",
    );
}
unsafe impl ::windows::runtime::Interface for Paragraph {
    type Vtable = IParagraph_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2664844407,
        12957,
        20527,
        [162, 87, 245, 131, 152, 237, 171, 81],
    );
}
impl ::windows::runtime::RuntimeName for Paragraph {
    const NAME: &'static str = "Microsoft.UI.Xaml.Documents.Paragraph";
}
impl ::core::convert::From<Paragraph> for ::windows::runtime::IUnknown {
    fn from(value: Paragraph) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&Paragraph> for ::windows::runtime::IUnknown {
    fn from(value: &Paragraph) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for Paragraph {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a Paragraph {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<Paragraph> for ::windows::runtime::IInspectable {
    fn from(value: Paragraph) -> Self {
        value.0
    }
}
impl ::core::convert::From<&Paragraph> for ::windows::runtime::IInspectable {
    fn from(value: &Paragraph) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for Paragraph {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a Paragraph {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<Paragraph> for Block {
    fn from(value: Paragraph) -> Self {
        ::core::convert::Into::<Block>::into(&value)
    }
}
impl ::core::convert::From<&Paragraph> for Block {
    fn from(value: &Paragraph) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, Block> for Paragraph {
    fn into_param(self) -> ::windows::runtime::Param<'a, Block> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<Block>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, Block> for &Paragraph {
    fn into_param(self) -> ::windows::runtime::Param<'a, Block> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<Block>::into(
            ::core::clone::Clone::clone(self),
        ))
    }
}
impl ::core::convert::From<Paragraph> for TextElement {
    fn from(value: Paragraph) -> Self {
        ::core::convert::Into::<TextElement>::into(&value)
    }
}
impl ::core::convert::From<&Paragraph> for TextElement {
    fn from(value: &Paragraph) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, TextElement> for Paragraph {
    fn into_param(self) -> ::windows::runtime::Param<'a, TextElement> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<TextElement>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, TextElement> for &Paragraph {
    fn into_param(self) -> ::windows::runtime::Param<'a, TextElement> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<TextElement>::into(
            ::core::clone::Clone::clone(self),
        ))
    }
}
impl ::core::convert::From<Paragraph> for super::DependencyObject {
    fn from(value: Paragraph) -> Self {
        ::core::convert::Into::<super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&Paragraph> for super::DependencyObject {
    fn from(value: &Paragraph) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::DependencyObject> for Paragraph {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::DependencyObject> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(
            self,
        ))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::DependencyObject> for &Paragraph {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::DependencyObject> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(
            ::core::clone::Clone::clone(self),
        ))
    }
}
unsafe impl ::core::marker::Send for Paragraph {}
unsafe impl ::core::marker::Sync for Paragraph {}
#[doc = "*Required features: `UI_Xaml_Documents`*"]
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct Run(pub ::windows::runtime::IInspectable);
impl Run {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            Run,
            ::windows::runtime::IActivationFactory,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn Text(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetText<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn FlowDirection(&self) -> ::windows::runtime::Result<super::FlowDirection> {
        let this = self;
        unsafe {
            let mut result__: super::FlowDirection = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::FlowDirection>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetFlowDirection(&self, value: super::FlowDirection) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn FlowDirectionProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::IRunStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn GetValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetValue<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>,
    >(
        &self,
        dp: Param0,
        value: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn ClearValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn ReadLocalValue<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn GetAnimationBaseValue<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn RegisterPropertyChangedCallback<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
        Param1: ::windows::runtime::IntoParam<'a, super::DependencyPropertyChangedCallback>,
    >(
        &self,
        dp: Param0,
        callback: Param1,
    ) -> ::windows::runtime::Result<i64> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: i64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                callback.into_param().abi(),
                &mut result__,
            )
            .from_abi::<i64>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn UnregisterPropertyChangedCallback<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
        token: i64,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).12)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                token,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn Dispatcher(&self) -> ::windows::runtime::Result<::windows::UI::Core::CoreDispatcher> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Core::CoreDispatcher>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Dispatching`*"]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::runtime::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn Name(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn FontSize(&self) -> ::windows::runtime::Result<f64> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetFontSize(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Media`*"]
    pub fn FontFamily(&self) -> ::windows::runtime::Result<super::Media::FontFamily> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::FontFamily>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Media`*"]
    pub fn SetFontFamily<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Media::FontFamily>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn FontWeight(&self) -> ::windows::runtime::Result<::windows::UI::Text::FontWeight> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::UI::Text::FontWeight = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::FontWeight>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetFontWeight<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::UI::Text::FontWeight>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).12)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn FontStyle(&self) -> ::windows::runtime::Result<::windows::UI::Text::FontStyle> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::UI::Text::FontStyle = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::FontStyle>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetFontStyle(
        &self,
        value: ::windows::UI::Text::FontStyle,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).14)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn FontStretch(&self) -> ::windows::runtime::Result<::windows::UI::Text::FontStretch> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::UI::Text::FontStretch = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::FontStretch>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetFontStretch(
        &self,
        value: ::windows::UI::Text::FontStretch,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).16)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn CharacterSpacing(&self) -> ::windows::runtime::Result<i32> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetCharacterSpacing(&self, value: i32) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).18)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Media`*"]
    pub fn Foreground(&self) -> ::windows::runtime::Result<super::Media::Brush> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Media`*"]
    pub fn SetForeground<'a, Param0: ::windows::runtime::IntoParam<'a, super::Media::Brush>>(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).20)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn Language(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).21)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetLanguage<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).22)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn IsTextScaleFactorEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).23)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetIsTextScaleFactorEnabled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).24)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn TextDecorations(
        &self,
    ) -> ::windows::runtime::Result<::windows::UI::Text::TextDecorations> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::UI::Text::TextDecorations = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).25)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::TextDecorations>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetTextDecorations(
        &self,
        value: ::windows::UI::Text::TextDecorations,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).26)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn ContentStart(&self) -> ::windows::runtime::Result<TextPointer> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).27)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn ContentEnd(&self) -> ::windows::runtime::Result<TextPointer> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).28)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn ElementStart(&self) -> ::windows::runtime::Result<TextPointer> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).29)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn ElementEnd(&self) -> ::windows::runtime::Result<TextPointer> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).30)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn AllowFocusOnInteraction(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).31)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetAllowFocusOnInteraction(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).32)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn AccessKey(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).33)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetAccessKey<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).34)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn ExitDisplayModeOnAccessKeyInvoked(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).35)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetExitDisplayModeOnAccessKeyInvoked(
        &self,
        value: bool,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).36)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn IsAccessKeyScope(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).37)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetIsAccessKeyScope(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).38)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn AccessKeyScopeOwner(&self) -> ::windows::runtime::Result<super::DependencyObject> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).39)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyObject>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetAccessKeyScopeOwner<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).40)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Input`*"]
    pub fn KeyTipPlacementMode(
        &self,
    ) -> ::windows::runtime::Result<super::Input::KeyTipPlacementMode> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: super::Input::KeyTipPlacementMode = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).41)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Input::KeyTipPlacementMode>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Input`*"]
    pub fn SetKeyTipPlacementMode(
        &self,
        value: super::Input::KeyTipPlacementMode,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).42)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn KeyTipHorizontalOffset(&self) -> ::windows::runtime::Result<f64> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).43)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetKeyTipHorizontalOffset(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).44)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn KeyTipVerticalOffset(&self) -> ::windows::runtime::Result<f64> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).45)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetKeyTipVerticalOffset(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).46)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn XamlRoot(&self) -> ::windows::runtime::Result<super::XamlRoot> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).47)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::XamlRoot>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetXamlRoot<'a, Param0: ::windows::runtime::IntoParam<'a, super::XamlRoot>>(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).48)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Input`*"]
    pub fn AccessKeyDisplayRequested<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                TextElement,
                super::Input::AccessKeyDisplayRequestedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).49)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn RemoveAccessKeyDisplayRequested<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).50)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Input`*"]
    pub fn AccessKeyDisplayDismissed<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                TextElement,
                super::Input::AccessKeyDisplayDismissedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).51)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn RemoveAccessKeyDisplayDismissed<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).52)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Input`*"]
    pub fn AccessKeyInvoked<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                TextElement,
                super::Input::AccessKeyInvokedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).53)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn RemoveAccessKeyInvoked<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).54)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn FindName<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(
        &self,
        name: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).55)(
                ::core::mem::transmute_copy(this),
                name.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    pub fn IRunStatics<R, F: FnOnce(&IRunStatics) -> ::windows::runtime::Result<R>>(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<Run, IRunStatics> =
            ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for Run {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Xaml.Documents.Run;{1f905239-37cb-590b-9132-3ffb7741906e})",
    );
}
unsafe impl ::windows::runtime::Interface for Run {
    type Vtable = IRun_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        529551929,
        14283,
        22795,
        [145, 50, 63, 251, 119, 65, 144, 110],
    );
}
impl ::windows::runtime::RuntimeName for Run {
    const NAME: &'static str = "Microsoft.UI.Xaml.Documents.Run";
}
impl ::core::convert::From<Run> for ::windows::runtime::IUnknown {
    fn from(value: Run) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&Run> for ::windows::runtime::IUnknown {
    fn from(value: &Run) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for Run {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a Run {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<Run> for ::windows::runtime::IInspectable {
    fn from(value: Run) -> Self {
        value.0
    }
}
impl ::core::convert::From<&Run> for ::windows::runtime::IInspectable {
    fn from(value: &Run) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for Run {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a Run {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<Run> for Inline {
    fn from(value: Run) -> Self {
        ::core::convert::Into::<Inline>::into(&value)
    }
}
impl ::core::convert::From<&Run> for Inline {
    fn from(value: &Run) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, Inline> for Run {
    fn into_param(self) -> ::windows::runtime::Param<'a, Inline> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<Inline>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, Inline> for &Run {
    fn into_param(self) -> ::windows::runtime::Param<'a, Inline> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<Inline>::into(
            ::core::clone::Clone::clone(self),
        ))
    }
}
impl ::core::convert::From<Run> for TextElement {
    fn from(value: Run) -> Self {
        ::core::convert::Into::<TextElement>::into(&value)
    }
}
impl ::core::convert::From<&Run> for TextElement {
    fn from(value: &Run) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, TextElement> for Run {
    fn into_param(self) -> ::windows::runtime::Param<'a, TextElement> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<TextElement>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, TextElement> for &Run {
    fn into_param(self) -> ::windows::runtime::Param<'a, TextElement> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<TextElement>::into(
            ::core::clone::Clone::clone(self),
        ))
    }
}
impl ::core::convert::From<Run> for super::DependencyObject {
    fn from(value: Run) -> Self {
        ::core::convert::Into::<super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&Run> for super::DependencyObject {
    fn from(value: &Run) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::DependencyObject> for Run {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::DependencyObject> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(
            self,
        ))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::DependencyObject> for &Run {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::DependencyObject> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(
            ::core::clone::Clone::clone(self),
        ))
    }
}
unsafe impl ::core::marker::Send for Run {}
unsafe impl ::core::marker::Sync for Run {}
#[doc = "*Required features: `UI_Xaml_Documents`*"]
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct Span(pub ::windows::runtime::IInspectable);
impl Span {
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn Inlines(&self) -> ::windows::runtime::Result<InlineCollection> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<InlineCollection>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetInlines<'a, Param0: ::windows::runtime::IntoParam<'a, InlineCollection>>(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn new() -> ::windows::runtime::Result<Span> {
        Self::ISpanFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                ::core::ptr::null_mut(),
                &mut ::core::option::Option::<::windows::runtime::IInspectable>::None as *mut _
                    as _,
                &mut result__,
            )
            .from_abi::<Span>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn GetValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetValue<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>,
    >(
        &self,
        dp: Param0,
        value: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn ClearValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn ReadLocalValue<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn GetAnimationBaseValue<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn RegisterPropertyChangedCallback<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
        Param1: ::windows::runtime::IntoParam<'a, super::DependencyPropertyChangedCallback>,
    >(
        &self,
        dp: Param0,
        callback: Param1,
    ) -> ::windows::runtime::Result<i64> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: i64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                callback.into_param().abi(),
                &mut result__,
            )
            .from_abi::<i64>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn UnregisterPropertyChangedCallback<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
        token: i64,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).12)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                token,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn Dispatcher(&self) -> ::windows::runtime::Result<::windows::UI::Core::CoreDispatcher> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Core::CoreDispatcher>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Dispatching`*"]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::runtime::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn Name(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn FontSize(&self) -> ::windows::runtime::Result<f64> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetFontSize(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Media`*"]
    pub fn FontFamily(&self) -> ::windows::runtime::Result<super::Media::FontFamily> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::FontFamily>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Media`*"]
    pub fn SetFontFamily<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Media::FontFamily>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn FontWeight(&self) -> ::windows::runtime::Result<::windows::UI::Text::FontWeight> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::UI::Text::FontWeight = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::FontWeight>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetFontWeight<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::UI::Text::FontWeight>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).12)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn FontStyle(&self) -> ::windows::runtime::Result<::windows::UI::Text::FontStyle> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::UI::Text::FontStyle = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::FontStyle>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetFontStyle(
        &self,
        value: ::windows::UI::Text::FontStyle,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).14)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn FontStretch(&self) -> ::windows::runtime::Result<::windows::UI::Text::FontStretch> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::UI::Text::FontStretch = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::FontStretch>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetFontStretch(
        &self,
        value: ::windows::UI::Text::FontStretch,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).16)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn CharacterSpacing(&self) -> ::windows::runtime::Result<i32> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetCharacterSpacing(&self, value: i32) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).18)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Media`*"]
    pub fn Foreground(&self) -> ::windows::runtime::Result<super::Media::Brush> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Media`*"]
    pub fn SetForeground<'a, Param0: ::windows::runtime::IntoParam<'a, super::Media::Brush>>(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).20)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn Language(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).21)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetLanguage<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).22)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn IsTextScaleFactorEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).23)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetIsTextScaleFactorEnabled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).24)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn TextDecorations(
        &self,
    ) -> ::windows::runtime::Result<::windows::UI::Text::TextDecorations> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::UI::Text::TextDecorations = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).25)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::TextDecorations>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetTextDecorations(
        &self,
        value: ::windows::UI::Text::TextDecorations,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).26)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn ContentStart(&self) -> ::windows::runtime::Result<TextPointer> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).27)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn ContentEnd(&self) -> ::windows::runtime::Result<TextPointer> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).28)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn ElementStart(&self) -> ::windows::runtime::Result<TextPointer> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).29)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn ElementEnd(&self) -> ::windows::runtime::Result<TextPointer> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).30)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn AllowFocusOnInteraction(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).31)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetAllowFocusOnInteraction(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).32)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn AccessKey(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).33)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetAccessKey<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).34)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn ExitDisplayModeOnAccessKeyInvoked(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).35)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetExitDisplayModeOnAccessKeyInvoked(
        &self,
        value: bool,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).36)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn IsAccessKeyScope(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).37)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetIsAccessKeyScope(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).38)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn AccessKeyScopeOwner(&self) -> ::windows::runtime::Result<super::DependencyObject> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).39)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyObject>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetAccessKeyScopeOwner<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).40)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Input`*"]
    pub fn KeyTipPlacementMode(
        &self,
    ) -> ::windows::runtime::Result<super::Input::KeyTipPlacementMode> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: super::Input::KeyTipPlacementMode = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).41)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Input::KeyTipPlacementMode>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Input`*"]
    pub fn SetKeyTipPlacementMode(
        &self,
        value: super::Input::KeyTipPlacementMode,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).42)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn KeyTipHorizontalOffset(&self) -> ::windows::runtime::Result<f64> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).43)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetKeyTipHorizontalOffset(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).44)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn KeyTipVerticalOffset(&self) -> ::windows::runtime::Result<f64> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).45)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetKeyTipVerticalOffset(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).46)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn XamlRoot(&self) -> ::windows::runtime::Result<super::XamlRoot> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).47)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::XamlRoot>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetXamlRoot<'a, Param0: ::windows::runtime::IntoParam<'a, super::XamlRoot>>(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).48)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Input`*"]
    pub fn AccessKeyDisplayRequested<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                TextElement,
                super::Input::AccessKeyDisplayRequestedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).49)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn RemoveAccessKeyDisplayRequested<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).50)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Input`*"]
    pub fn AccessKeyDisplayDismissed<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                TextElement,
                super::Input::AccessKeyDisplayDismissedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).51)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn RemoveAccessKeyDisplayDismissed<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).52)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Input`*"]
    pub fn AccessKeyInvoked<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                TextElement,
                super::Input::AccessKeyInvokedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).53)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn RemoveAccessKeyInvoked<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).54)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn FindName<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(
        &self,
        name: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).55)(
                ::core::mem::transmute_copy(this),
                name.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    pub fn ISpanFactory<R, F: FnOnce(&ISpanFactory) -> ::windows::runtime::Result<R>>(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<Span, ISpanFactory> =
            ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for Span {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Xaml.Documents.Span;{91b93d4d-4e28-57b9-bffb-3566c2a3c2a1})",
    );
}
unsafe impl ::windows::runtime::Interface for Span {
    type Vtable = ISpan_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2444836173,
        20008,
        22457,
        [191, 251, 53, 102, 194, 163, 194, 161],
    );
}
impl ::windows::runtime::RuntimeName for Span {
    const NAME: &'static str = "Microsoft.UI.Xaml.Documents.Span";
}
impl ::core::convert::From<Span> for ::windows::runtime::IUnknown {
    fn from(value: Span) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&Span> for ::windows::runtime::IUnknown {
    fn from(value: &Span) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for Span {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a Span {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<Span> for ::windows::runtime::IInspectable {
    fn from(value: Span) -> Self {
        value.0
    }
}
impl ::core::convert::From<&Span> for ::windows::runtime::IInspectable {
    fn from(value: &Span) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for Span {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a Span {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<Span> for Inline {
    fn from(value: Span) -> Self {
        ::core::convert::Into::<Inline>::into(&value)
    }
}
impl ::core::convert::From<&Span> for Inline {
    fn from(value: &Span) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, Inline> for Span {
    fn into_param(self) -> ::windows::runtime::Param<'a, Inline> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<Inline>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, Inline> for &Span {
    fn into_param(self) -> ::windows::runtime::Param<'a, Inline> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<Inline>::into(
            ::core::clone::Clone::clone(self),
        ))
    }
}
impl ::core::convert::From<Span> for TextElement {
    fn from(value: Span) -> Self {
        ::core::convert::Into::<TextElement>::into(&value)
    }
}
impl ::core::convert::From<&Span> for TextElement {
    fn from(value: &Span) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, TextElement> for Span {
    fn into_param(self) -> ::windows::runtime::Param<'a, TextElement> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<TextElement>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, TextElement> for &Span {
    fn into_param(self) -> ::windows::runtime::Param<'a, TextElement> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<TextElement>::into(
            ::core::clone::Clone::clone(self),
        ))
    }
}
impl ::core::convert::From<Span> for super::DependencyObject {
    fn from(value: Span) -> Self {
        ::core::convert::Into::<super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&Span> for super::DependencyObject {
    fn from(value: &Span) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::DependencyObject> for Span {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::DependencyObject> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(
            self,
        ))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::DependencyObject> for &Span {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::DependencyObject> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(
            ::core::clone::Clone::clone(self),
        ))
    }
}
unsafe impl ::core::marker::Send for Span {}
unsafe impl ::core::marker::Sync for Span {}
#[doc = "*Required features: `UI_Xaml_Documents`*"]
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct TextElement(pub ::windows::runtime::IInspectable);
impl TextElement {
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn Name(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn FontSize(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetFontSize(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Media`*"]
    pub fn FontFamily(&self) -> ::windows::runtime::Result<super::Media::FontFamily> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::FontFamily>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Media`*"]
    pub fn SetFontFamily<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Media::FontFamily>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn FontWeight(&self) -> ::windows::runtime::Result<::windows::UI::Text::FontWeight> {
        let this = self;
        unsafe {
            let mut result__: ::windows::UI::Text::FontWeight = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::FontWeight>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetFontWeight<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::UI::Text::FontWeight>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).12)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn FontStyle(&self) -> ::windows::runtime::Result<::windows::UI::Text::FontStyle> {
        let this = self;
        unsafe {
            let mut result__: ::windows::UI::Text::FontStyle = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::FontStyle>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetFontStyle(
        &self,
        value: ::windows::UI::Text::FontStyle,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).14)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn FontStretch(&self) -> ::windows::runtime::Result<::windows::UI::Text::FontStretch> {
        let this = self;
        unsafe {
            let mut result__: ::windows::UI::Text::FontStretch = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::FontStretch>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetFontStretch(
        &self,
        value: ::windows::UI::Text::FontStretch,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).16)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn CharacterSpacing(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetCharacterSpacing(&self, value: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).18)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Media`*"]
    pub fn Foreground(&self) -> ::windows::runtime::Result<super::Media::Brush> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Media`*"]
    pub fn SetForeground<'a, Param0: ::windows::runtime::IntoParam<'a, super::Media::Brush>>(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).20)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn Language(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).21)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetLanguage<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).22)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn IsTextScaleFactorEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).23)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetIsTextScaleFactorEnabled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).24)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn TextDecorations(
        &self,
    ) -> ::windows::runtime::Result<::windows::UI::Text::TextDecorations> {
        let this = self;
        unsafe {
            let mut result__: ::windows::UI::Text::TextDecorations = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).25)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::TextDecorations>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetTextDecorations(
        &self,
        value: ::windows::UI::Text::TextDecorations,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).26)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn ContentStart(&self) -> ::windows::runtime::Result<TextPointer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).27)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn ContentEnd(&self) -> ::windows::runtime::Result<TextPointer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).28)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn ElementStart(&self) -> ::windows::runtime::Result<TextPointer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).29)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn ElementEnd(&self) -> ::windows::runtime::Result<TextPointer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).30)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn AllowFocusOnInteraction(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).31)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetAllowFocusOnInteraction(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).32)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn AccessKey(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).33)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetAccessKey<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).34)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn ExitDisplayModeOnAccessKeyInvoked(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).35)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetExitDisplayModeOnAccessKeyInvoked(
        &self,
        value: bool,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).36)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn IsAccessKeyScope(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).37)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetIsAccessKeyScope(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).38)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn AccessKeyScopeOwner(&self) -> ::windows::runtime::Result<super::DependencyObject> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).39)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyObject>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetAccessKeyScopeOwner<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).40)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Input`*"]
    pub fn KeyTipPlacementMode(
        &self,
    ) -> ::windows::runtime::Result<super::Input::KeyTipPlacementMode> {
        let this = self;
        unsafe {
            let mut result__: super::Input::KeyTipPlacementMode = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).41)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Input::KeyTipPlacementMode>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Input`*"]
    pub fn SetKeyTipPlacementMode(
        &self,
        value: super::Input::KeyTipPlacementMode,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).42)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn KeyTipHorizontalOffset(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).43)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetKeyTipHorizontalOffset(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).44)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn KeyTipVerticalOffset(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).45)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetKeyTipVerticalOffset(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).46)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn XamlRoot(&self) -> ::windows::runtime::Result<super::XamlRoot> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).47)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::XamlRoot>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetXamlRoot<'a, Param0: ::windows::runtime::IntoParam<'a, super::XamlRoot>>(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).48)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Input`*"]
    pub fn AccessKeyDisplayRequested<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                TextElement,
                super::Input::AccessKeyDisplayRequestedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<::windows::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).49)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn RemoveAccessKeyDisplayRequested<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).50)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Input`*"]
    pub fn AccessKeyDisplayDismissed<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                TextElement,
                super::Input::AccessKeyDisplayDismissedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<::windows::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).51)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn RemoveAccessKeyDisplayDismissed<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).52)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Input`*"]
    pub fn AccessKeyInvoked<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                TextElement,
                super::Input::AccessKeyInvokedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<::windows::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).53)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn RemoveAccessKeyInvoked<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).54)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn FindName<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(
        &self,
        name: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).55)(
                ::core::mem::transmute_copy(this),
                name.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn FontSizeProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::ITextElementStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn FontFamilyProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::ITextElementStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn FontWeightProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::ITextElementStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn FontStyleProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::ITextElementStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn FontStretchProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::ITextElementStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn CharacterSpacingProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::ITextElementStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn ForegroundProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::ITextElementStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn LanguageProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::ITextElementStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn IsTextScaleFactorEnabledProperty(
    ) -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::ITextElementStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn TextDecorationsProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::ITextElementStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn AllowFocusOnInteractionProperty() -> ::windows::runtime::Result<super::DependencyProperty>
    {
        Self::ITextElementStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn AccessKeyProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::ITextElementStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn ExitDisplayModeOnAccessKeyInvokedProperty(
    ) -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::ITextElementStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn IsAccessKeyScopeProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::ITextElementStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn AccessKeyScopeOwnerProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::ITextElementStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn KeyTipPlacementModeProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::ITextElementStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).21)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn KeyTipHorizontalOffsetProperty() -> ::windows::runtime::Result<super::DependencyProperty>
    {
        Self::ITextElementStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).22)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn KeyTipVerticalOffsetProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::ITextElementStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).23)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn GetValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetValue<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>,
    >(
        &self,
        dp: Param0,
        value: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn ClearValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn ReadLocalValue<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn GetAnimationBaseValue<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn RegisterPropertyChangedCallback<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
        Param1: ::windows::runtime::IntoParam<'a, super::DependencyPropertyChangedCallback>,
    >(
        &self,
        dp: Param0,
        callback: Param1,
    ) -> ::windows::runtime::Result<i64> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: i64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                callback.into_param().abi(),
                &mut result__,
            )
            .from_abi::<i64>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn UnregisterPropertyChangedCallback<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
        token: i64,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).12)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                token,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn Dispatcher(&self) -> ::windows::runtime::Result<::windows::UI::Core::CoreDispatcher> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Core::CoreDispatcher>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Dispatching`*"]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::runtime::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
    pub fn ITextElementStatics<
        R,
        F: FnOnce(&ITextElementStatics) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<TextElement, ITextElementStatics> =
            ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for TextElement {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Xaml.Documents.TextElement;{a122ba22-833f-5220-a47e-6cd507531abe})",
    );
}
unsafe impl ::windows::runtime::Interface for TextElement {
    type Vtable = ITextElement_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2703407650,
        33599,
        21024,
        [164, 126, 108, 213, 7, 83, 26, 190],
    );
}
impl ::windows::runtime::RuntimeName for TextElement {
    const NAME: &'static str = "Microsoft.UI.Xaml.Documents.TextElement";
}
impl ::core::convert::From<TextElement> for ::windows::runtime::IUnknown {
    fn from(value: TextElement) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&TextElement> for ::windows::runtime::IUnknown {
    fn from(value: &TextElement) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for TextElement {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a TextElement {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<TextElement> for ::windows::runtime::IInspectable {
    fn from(value: TextElement) -> Self {
        value.0
    }
}
impl ::core::convert::From<&TextElement> for ::windows::runtime::IInspectable {
    fn from(value: &TextElement) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for TextElement {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a TextElement {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<TextElement> for super::DependencyObject {
    fn from(value: TextElement) -> Self {
        ::core::convert::Into::<super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&TextElement> for super::DependencyObject {
    fn from(value: &TextElement) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::DependencyObject> for TextElement {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::DependencyObject> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(
            self,
        ))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::DependencyObject> for &TextElement {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::DependencyObject> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(
            ::core::clone::Clone::clone(self),
        ))
    }
}
unsafe impl ::core::marker::Send for TextElement {}
unsafe impl ::core::marker::Sync for TextElement {}
#[doc = "*Required features: `UI_Xaml_Documents`*"]
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct TextHighlighter(pub ::windows::runtime::IInspectable);
impl TextHighlighter {
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn Ranges(
        &self,
    ) -> ::windows::runtime::Result<::windows::Foundation::Collections::IVector<TextRange>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Collections::IVector<TextRange>>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Media`*"]
    pub fn Foreground(&self) -> ::windows::runtime::Result<super::Media::Brush> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Media`*"]
    pub fn SetForeground<'a, Param0: ::windows::runtime::IntoParam<'a, super::Media::Brush>>(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Media`*"]
    pub fn Background(&self) -> ::windows::runtime::Result<super::Media::Brush> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Media`*"]
    pub fn SetBackground<'a, Param0: ::windows::runtime::IntoParam<'a, super::Media::Brush>>(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn ForegroundProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::ITextHighlighterStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn BackgroundProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::ITextHighlighterStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn new() -> ::windows::runtime::Result<TextHighlighter> {
        Self::ITextHighlighterFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                ::core::ptr::null_mut(),
                &mut ::core::option::Option::<::windows::runtime::IInspectable>::None as *mut _
                    as _,
                &mut result__,
            )
            .from_abi::<TextHighlighter>(result__)
        })
    }
    pub fn ITextHighlighterStatics<
        R,
        F: FnOnce(&ITextHighlighterStatics) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            TextHighlighter,
            ITextHighlighterStatics,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ITextHighlighterFactory<
        R,
        F: FnOnce(&ITextHighlighterFactory) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            TextHighlighter,
            ITextHighlighterFactory,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for TextHighlighter {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Xaml.Documents.TextHighlighter;{b756e861-1d2b-5f6f-81fd-c51a5bc068ff})",
    );
}
unsafe impl ::windows::runtime::Interface for TextHighlighter {
    type Vtable = ITextHighlighter_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3075926113,
        7467,
        24431,
        [129, 253, 197, 26, 91, 192, 104, 255],
    );
}
impl ::windows::runtime::RuntimeName for TextHighlighter {
    const NAME: &'static str = "Microsoft.UI.Xaml.Documents.TextHighlighter";
}
impl ::core::convert::From<TextHighlighter> for ::windows::runtime::IUnknown {
    fn from(value: TextHighlighter) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&TextHighlighter> for ::windows::runtime::IUnknown {
    fn from(value: &TextHighlighter) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for TextHighlighter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a TextHighlighter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<TextHighlighter> for ::windows::runtime::IInspectable {
    fn from(value: TextHighlighter) -> Self {
        value.0
    }
}
impl ::core::convert::From<&TextHighlighter> for ::windows::runtime::IInspectable {
    fn from(value: &TextHighlighter) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for TextHighlighter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a TextHighlighter
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for TextHighlighter {}
unsafe impl ::core::marker::Sync for TextHighlighter {}
#[doc = "*Required features: `UI_Xaml_Documents`*"]
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct TextHighlighterBase(pub ::windows::runtime::IInspectable);
impl TextHighlighterBase {
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn GetValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetValue<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>,
    >(
        &self,
        dp: Param0,
        value: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn ClearValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn ReadLocalValue<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn GetAnimationBaseValue<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn RegisterPropertyChangedCallback<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
        Param1: ::windows::runtime::IntoParam<'a, super::DependencyPropertyChangedCallback>,
    >(
        &self,
        dp: Param0,
        callback: Param1,
    ) -> ::windows::runtime::Result<i64> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: i64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                callback.into_param().abi(),
                &mut result__,
            )
            .from_abi::<i64>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn UnregisterPropertyChangedCallback<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
        token: i64,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).12)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                token,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn Dispatcher(&self) -> ::windows::runtime::Result<::windows::UI::Core::CoreDispatcher> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Core::CoreDispatcher>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Dispatching`*"]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::runtime::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for TextHighlighterBase {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Documents.TextHighlighterBase;{5c21aaf0-3a17-5468-8aac-be14db0ed8c1})" ) ;
}
unsafe impl ::windows::runtime::Interface for TextHighlighterBase {
    type Vtable = ITextHighlighterBase_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1545710320,
        14871,
        21608,
        [138, 172, 190, 20, 219, 14, 216, 193],
    );
}
impl ::windows::runtime::RuntimeName for TextHighlighterBase {
    const NAME: &'static str = "Microsoft.UI.Xaml.Documents.TextHighlighterBase";
}
impl ::core::convert::From<TextHighlighterBase> for ::windows::runtime::IUnknown {
    fn from(value: TextHighlighterBase) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&TextHighlighterBase> for ::windows::runtime::IUnknown {
    fn from(value: &TextHighlighterBase) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for TextHighlighterBase {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &'a TextHighlighterBase
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<TextHighlighterBase> for ::windows::runtime::IInspectable {
    fn from(value: TextHighlighterBase) -> Self {
        value.0
    }
}
impl ::core::convert::From<&TextHighlighterBase> for ::windows::runtime::IInspectable {
    fn from(value: &TextHighlighterBase) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for TextHighlighterBase
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a TextHighlighterBase
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<TextHighlighterBase> for super::DependencyObject {
    fn from(value: TextHighlighterBase) -> Self {
        ::core::convert::Into::<super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&TextHighlighterBase> for super::DependencyObject {
    fn from(value: &TextHighlighterBase) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::DependencyObject> for TextHighlighterBase {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::DependencyObject> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(
            self,
        ))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::DependencyObject> for &TextHighlighterBase {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::DependencyObject> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(
            ::core::clone::Clone::clone(self),
        ))
    }
}
unsafe impl ::core::marker::Send for TextHighlighterBase {}
unsafe impl ::core::marker::Sync for TextHighlighterBase {}
#[doc = "*Required features: `UI_Xaml_Documents`*"]
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct TextPointer(pub ::windows::runtime::IInspectable);
impl TextPointer {
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn Parent(&self) -> ::windows::runtime::Result<super::DependencyObject> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyObject>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn VisualParent(&self) -> ::windows::runtime::Result<super::FrameworkElement> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::FrameworkElement>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn LogicalDirection(&self) -> ::windows::runtime::Result<LogicalDirection> {
        let this = self;
        unsafe {
            let mut result__: LogicalDirection = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<LogicalDirection>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn Offset(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn GetCharacterRect(
        &self,
        direction: LogicalDirection,
    ) -> ::windows::runtime::Result<::windows::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::Rect = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                direction,
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Rect>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn GetPositionAtOffset(
        &self,
        offset: i32,
        direction: LogicalDirection,
    ) -> ::windows::runtime::Result<TextPointer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(
                ::core::mem::transmute_copy(this),
                offset,
                direction,
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for TextPointer {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Xaml.Documents.TextPointer;{842eb385-ee41-5930-979b-438fa7525a51})",
    );
}
unsafe impl ::windows::runtime::Interface for TextPointer {
    type Vtable = ITextPointer_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2217653125,
        60993,
        22832,
        [151, 155, 67, 143, 167, 82, 90, 81],
    );
}
impl ::windows::runtime::RuntimeName for TextPointer {
    const NAME: &'static str = "Microsoft.UI.Xaml.Documents.TextPointer";
}
impl ::core::convert::From<TextPointer> for ::windows::runtime::IUnknown {
    fn from(value: TextPointer) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&TextPointer> for ::windows::runtime::IUnknown {
    fn from(value: &TextPointer) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for TextPointer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a TextPointer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<TextPointer> for ::windows::runtime::IInspectable {
    fn from(value: TextPointer) -> Self {
        value.0
    }
}
impl ::core::convert::From<&TextPointer> for ::windows::runtime::IInspectable {
    fn from(value: &TextPointer) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for TextPointer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a TextPointer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for TextPointer {}
unsafe impl ::core::marker::Sync for TextPointer {}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `UI_Xaml_Documents`*"]
pub struct TextRange {
    pub StartIndex: i32,
    pub Length: i32,
}
impl TextRange {}
impl ::core::default::Default for TextRange {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for TextRange {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("TextRange")
            .field("StartIndex", &self.StartIndex)
            .field("Length", &self.Length)
            .finish()
    }
}
impl ::core::cmp::PartialEq for TextRange {
    fn eq(&self, other: &Self) -> bool {
        self.StartIndex == other.StartIndex && self.Length == other.Length
    }
}
impl ::core::cmp::Eq for TextRange {}
unsafe impl ::windows::runtime::Abi for TextRange {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for TextRange {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"struct(Microsoft.UI.Xaml.Documents.TextRange;i4;i4)",
    );
}
impl ::windows::runtime::DefaultType for TextRange {
    type DefaultType = Self;
}
#[doc = "*Required features: `UI_Xaml_Documents`*"]
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct Typography(pub ::windows::runtime::IInspectable);
impl Typography {
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn AnnotationAlternatesProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn GetAnnotationAlternates<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::runtime::Result<i32> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetAnnotationAlternates<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
        value: i32,
    ) -> ::windows::runtime::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn EastAsianExpertFormsProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn GetEastAsianExpertForms<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::runtime::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetEastAsianExpertForms<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
        value: bool,
    ) -> ::windows::runtime::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).11)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn EastAsianLanguageProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn GetEastAsianLanguage<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::runtime::Result<super::FontEastAsianLanguage> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: super::FontEastAsianLanguage = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::FontEastAsianLanguage>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetEastAsianLanguage<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
        value: super::FontEastAsianLanguage,
    ) -> ::windows::runtime::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).14)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn EastAsianWidthsProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn GetEastAsianWidths<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::runtime::Result<super::FontEastAsianWidths> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: super::FontEastAsianWidths = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::FontEastAsianWidths>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetEastAsianWidths<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
        value: super::FontEastAsianWidths,
    ) -> ::windows::runtime::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).17)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn StandardLigaturesProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn GetStandardLigatures<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::runtime::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetStandardLigatures<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
        value: bool,
    ) -> ::windows::runtime::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).20)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn ContextualLigaturesProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).21)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn GetContextualLigatures<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::runtime::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).22)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetContextualLigatures<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
        value: bool,
    ) -> ::windows::runtime::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).23)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn DiscretionaryLigaturesProperty() -> ::windows::runtime::Result<super::DependencyProperty>
    {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).24)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn GetDiscretionaryLigatures<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::runtime::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).25)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetDiscretionaryLigatures<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
        value: bool,
    ) -> ::windows::runtime::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).26)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn HistoricalLigaturesProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).27)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn GetHistoricalLigatures<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::runtime::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).28)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetHistoricalLigatures<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
        value: bool,
    ) -> ::windows::runtime::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).29)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn StandardSwashesProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).30)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn GetStandardSwashes<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::runtime::Result<i32> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).31)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetStandardSwashes<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
        value: i32,
    ) -> ::windows::runtime::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).32)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn ContextualSwashesProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).33)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn GetContextualSwashes<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::runtime::Result<i32> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).34)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetContextualSwashes<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
        value: i32,
    ) -> ::windows::runtime::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).35)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn ContextualAlternatesProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).36)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn GetContextualAlternates<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::runtime::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).37)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetContextualAlternates<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
        value: bool,
    ) -> ::windows::runtime::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).38)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn StylisticAlternatesProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).39)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn GetStylisticAlternates<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::runtime::Result<i32> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).40)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetStylisticAlternates<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
        value: i32,
    ) -> ::windows::runtime::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).41)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn StylisticSet1Property() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).42)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn GetStylisticSet1<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::runtime::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).43)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetStylisticSet1<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
        value: bool,
    ) -> ::windows::runtime::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).44)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn StylisticSet2Property() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).45)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn GetStylisticSet2<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::runtime::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).46)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetStylisticSet2<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
        value: bool,
    ) -> ::windows::runtime::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).47)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn StylisticSet3Property() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).48)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn GetStylisticSet3<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::runtime::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).49)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetStylisticSet3<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
        value: bool,
    ) -> ::windows::runtime::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).50)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn StylisticSet4Property() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).51)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn GetStylisticSet4<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::runtime::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).52)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetStylisticSet4<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
        value: bool,
    ) -> ::windows::runtime::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).53)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn StylisticSet5Property() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).54)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn GetStylisticSet5<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::runtime::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).55)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetStylisticSet5<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
        value: bool,
    ) -> ::windows::runtime::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).56)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn StylisticSet6Property() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).57)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn GetStylisticSet6<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::runtime::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).58)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetStylisticSet6<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
        value: bool,
    ) -> ::windows::runtime::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).59)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn StylisticSet7Property() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).60)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn GetStylisticSet7<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::runtime::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).61)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetStylisticSet7<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
        value: bool,
    ) -> ::windows::runtime::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).62)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn StylisticSet8Property() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).63)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn GetStylisticSet8<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::runtime::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).64)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetStylisticSet8<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
        value: bool,
    ) -> ::windows::runtime::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).65)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn StylisticSet9Property() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).66)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn GetStylisticSet9<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::runtime::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).67)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetStylisticSet9<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
        value: bool,
    ) -> ::windows::runtime::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).68)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn StylisticSet10Property() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).69)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn GetStylisticSet10<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::runtime::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).70)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetStylisticSet10<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
        value: bool,
    ) -> ::windows::runtime::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).71)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn StylisticSet11Property() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).72)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn GetStylisticSet11<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::runtime::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).73)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetStylisticSet11<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
        value: bool,
    ) -> ::windows::runtime::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).74)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn StylisticSet12Property() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).75)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn GetStylisticSet12<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::runtime::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).76)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetStylisticSet12<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
        value: bool,
    ) -> ::windows::runtime::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).77)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn StylisticSet13Property() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).78)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn GetStylisticSet13<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::runtime::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).79)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetStylisticSet13<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
        value: bool,
    ) -> ::windows::runtime::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).80)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn StylisticSet14Property() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).81)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn GetStylisticSet14<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::runtime::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).82)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetStylisticSet14<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
        value: bool,
    ) -> ::windows::runtime::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).83)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn StylisticSet15Property() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).84)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn GetStylisticSet15<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::runtime::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).85)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetStylisticSet15<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
        value: bool,
    ) -> ::windows::runtime::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).86)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn StylisticSet16Property() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).87)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn GetStylisticSet16<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::runtime::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).88)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetStylisticSet16<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
        value: bool,
    ) -> ::windows::runtime::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).89)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn StylisticSet17Property() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).90)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn GetStylisticSet17<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::runtime::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).91)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetStylisticSet17<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
        value: bool,
    ) -> ::windows::runtime::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).92)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn StylisticSet18Property() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).93)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn GetStylisticSet18<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::runtime::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).94)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetStylisticSet18<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
        value: bool,
    ) -> ::windows::runtime::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).95)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn StylisticSet19Property() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).96)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn GetStylisticSet19<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::runtime::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).97)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetStylisticSet19<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
        value: bool,
    ) -> ::windows::runtime::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).98)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn StylisticSet20Property() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).99)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn GetStylisticSet20<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::runtime::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).100)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetStylisticSet20<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
        value: bool,
    ) -> ::windows::runtime::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).101)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn CapitalsProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).102)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn GetCapitals<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>>(
        element: Param0,
    ) -> ::windows::runtime::Result<super::FontCapitals> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: super::FontCapitals = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).103)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::FontCapitals>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetCapitals<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>>(
        element: Param0,
        value: super::FontCapitals,
    ) -> ::windows::runtime::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).104)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn CapitalSpacingProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).105)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn GetCapitalSpacing<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::runtime::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).106)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetCapitalSpacing<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
        value: bool,
    ) -> ::windows::runtime::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).107)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn KerningProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).108)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn GetKerning<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>>(
        element: Param0,
    ) -> ::windows::runtime::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).109)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetKerning<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>>(
        element: Param0,
        value: bool,
    ) -> ::windows::runtime::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).110)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn CaseSensitiveFormsProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).111)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn GetCaseSensitiveForms<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::runtime::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).112)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetCaseSensitiveForms<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
        value: bool,
    ) -> ::windows::runtime::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).113)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn HistoricalFormsProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).114)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn GetHistoricalForms<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::runtime::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).115)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetHistoricalForms<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
        value: bool,
    ) -> ::windows::runtime::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).116)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn FractionProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).117)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn GetFraction<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>>(
        element: Param0,
    ) -> ::windows::runtime::Result<super::FontFraction> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: super::FontFraction = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).118)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::FontFraction>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetFraction<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>>(
        element: Param0,
        value: super::FontFraction,
    ) -> ::windows::runtime::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).119)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn NumeralStyleProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).120)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn GetNumeralStyle<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::runtime::Result<super::FontNumeralStyle> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: super::FontNumeralStyle = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).121)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::FontNumeralStyle>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetNumeralStyle<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
        value: super::FontNumeralStyle,
    ) -> ::windows::runtime::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).122)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn NumeralAlignmentProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).123)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn GetNumeralAlignment<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::runtime::Result<super::FontNumeralAlignment> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: super::FontNumeralAlignment = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).124)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::FontNumeralAlignment>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetNumeralAlignment<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
        value: super::FontNumeralAlignment,
    ) -> ::windows::runtime::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).125)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SlashedZeroProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).126)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn GetSlashedZero<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::runtime::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).127)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetSlashedZero<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
        value: bool,
    ) -> ::windows::runtime::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).128)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn MathematicalGreekProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).129)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn GetMathematicalGreek<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::runtime::Result<bool> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).130)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetMathematicalGreek<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
        value: bool,
    ) -> ::windows::runtime::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).131)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn VariantsProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).132)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn GetVariants<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>>(
        element: Param0,
    ) -> ::windows::runtime::Result<super::FontVariants> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__: super::FontVariants = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).133)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::FontVariants>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetVariants<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>>(
        element: Param0,
        value: super::FontVariants,
    ) -> ::windows::runtime::Result<()> {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).134)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    pub fn ITypographyStatics<
        R,
        F: FnOnce(&ITypographyStatics) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<Typography, ITypographyStatics> =
            ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for Typography {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Xaml.Documents.Typography;{fa27e2e3-be5e-5d21-9a5e-90cf102af828})",
    );
}
unsafe impl ::windows::runtime::Interface for Typography {
    type Vtable = ITypography_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        4196917987,
        48734,
        23841,
        [154, 94, 144, 207, 16, 42, 248, 40],
    );
}
impl ::windows::runtime::RuntimeName for Typography {
    const NAME: &'static str = "Microsoft.UI.Xaml.Documents.Typography";
}
impl ::core::convert::From<Typography> for ::windows::runtime::IUnknown {
    fn from(value: Typography) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&Typography> for ::windows::runtime::IUnknown {
    fn from(value: &Typography) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for Typography {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a Typography {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<Typography> for ::windows::runtime::IInspectable {
    fn from(value: Typography) -> Self {
        value.0
    }
}
impl ::core::convert::From<&Typography> for ::windows::runtime::IInspectable {
    fn from(value: &Typography) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for Typography {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a Typography {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for Typography {}
unsafe impl ::core::marker::Sync for Typography {}
#[doc = "*Required features: `UI_Xaml_Documents`*"]
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct Underline(pub ::windows::runtime::IInspectable);
impl Underline {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            Underline,
            ::windows::runtime::IActivationFactory,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn GetValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetValue<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>,
    >(
        &self,
        dp: Param0,
        value: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn ClearValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn ReadLocalValue<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn GetAnimationBaseValue<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn RegisterPropertyChangedCallback<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
        Param1: ::windows::runtime::IntoParam<'a, super::DependencyPropertyChangedCallback>,
    >(
        &self,
        dp: Param0,
        callback: Param1,
    ) -> ::windows::runtime::Result<i64> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: i64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                callback.into_param().abi(),
                &mut result__,
            )
            .from_abi::<i64>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn UnregisterPropertyChangedCallback<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
        token: i64,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).12)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                token,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn Dispatcher(&self) -> ::windows::runtime::Result<::windows::UI::Core::CoreDispatcher> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Core::CoreDispatcher>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Dispatching`*"]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::runtime::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn Inlines(&self) -> ::windows::runtime::Result<InlineCollection> {
        let this = &::windows::runtime::Interface::cast::<ISpan>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<InlineCollection>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetInlines<'a, Param0: ::windows::runtime::IntoParam<'a, InlineCollection>>(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ISpan>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn Name(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn FontSize(&self) -> ::windows::runtime::Result<f64> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetFontSize(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Media`*"]
    pub fn FontFamily(&self) -> ::windows::runtime::Result<super::Media::FontFamily> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::FontFamily>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Media`*"]
    pub fn SetFontFamily<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Media::FontFamily>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn FontWeight(&self) -> ::windows::runtime::Result<::windows::UI::Text::FontWeight> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::UI::Text::FontWeight = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::FontWeight>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetFontWeight<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::UI::Text::FontWeight>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).12)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn FontStyle(&self) -> ::windows::runtime::Result<::windows::UI::Text::FontStyle> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::UI::Text::FontStyle = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::FontStyle>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetFontStyle(
        &self,
        value: ::windows::UI::Text::FontStyle,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).14)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn FontStretch(&self) -> ::windows::runtime::Result<::windows::UI::Text::FontStretch> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::UI::Text::FontStretch = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::FontStretch>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetFontStretch(
        &self,
        value: ::windows::UI::Text::FontStretch,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).16)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn CharacterSpacing(&self) -> ::windows::runtime::Result<i32> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetCharacterSpacing(&self, value: i32) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).18)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Media`*"]
    pub fn Foreground(&self) -> ::windows::runtime::Result<super::Media::Brush> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Media`*"]
    pub fn SetForeground<'a, Param0: ::windows::runtime::IntoParam<'a, super::Media::Brush>>(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).20)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn Language(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).21)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetLanguage<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).22)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn IsTextScaleFactorEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).23)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetIsTextScaleFactorEnabled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).24)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn TextDecorations(
        &self,
    ) -> ::windows::runtime::Result<::windows::UI::Text::TextDecorations> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::UI::Text::TextDecorations = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).25)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::TextDecorations>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetTextDecorations(
        &self,
        value: ::windows::UI::Text::TextDecorations,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).26)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn ContentStart(&self) -> ::windows::runtime::Result<TextPointer> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).27)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn ContentEnd(&self) -> ::windows::runtime::Result<TextPointer> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).28)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn ElementStart(&self) -> ::windows::runtime::Result<TextPointer> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).29)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn ElementEnd(&self) -> ::windows::runtime::Result<TextPointer> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).30)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn AllowFocusOnInteraction(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).31)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetAllowFocusOnInteraction(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).32)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn AccessKey(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).33)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetAccessKey<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).34)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn ExitDisplayModeOnAccessKeyInvoked(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).35)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetExitDisplayModeOnAccessKeyInvoked(
        &self,
        value: bool,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).36)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn IsAccessKeyScope(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).37)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetIsAccessKeyScope(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).38)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn AccessKeyScopeOwner(&self) -> ::windows::runtime::Result<super::DependencyObject> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).39)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyObject>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetAccessKeyScopeOwner<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).40)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Input`*"]
    pub fn KeyTipPlacementMode(
        &self,
    ) -> ::windows::runtime::Result<super::Input::KeyTipPlacementMode> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: super::Input::KeyTipPlacementMode = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).41)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Input::KeyTipPlacementMode>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Input`*"]
    pub fn SetKeyTipPlacementMode(
        &self,
        value: super::Input::KeyTipPlacementMode,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).42)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn KeyTipHorizontalOffset(&self) -> ::windows::runtime::Result<f64> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).43)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetKeyTipHorizontalOffset(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).44)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn KeyTipVerticalOffset(&self) -> ::windows::runtime::Result<f64> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).45)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetKeyTipVerticalOffset(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).46)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn XamlRoot(&self) -> ::windows::runtime::Result<super::XamlRoot> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).47)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::XamlRoot>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn SetXamlRoot<'a, Param0: ::windows::runtime::IntoParam<'a, super::XamlRoot>>(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).48)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Input`*"]
    pub fn AccessKeyDisplayRequested<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                TextElement,
                super::Input::AccessKeyDisplayRequestedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).49)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn RemoveAccessKeyDisplayRequested<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).50)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Input`*"]
    pub fn AccessKeyDisplayDismissed<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                TextElement,
                super::Input::AccessKeyDisplayDismissedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).51)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn RemoveAccessKeyDisplayDismissed<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).52)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    #[doc = "*Required features: `UI_Xaml_Documents`, `UI_Xaml_Input`*"]
    pub fn AccessKeyInvoked<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                TextElement,
                super::Input::AccessKeyInvokedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).53)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn RemoveAccessKeyInvoked<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).54)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Documents`*"]
    pub fn FindName<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(
        &self,
        name: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).55)(
                ::core::mem::transmute_copy(this),
                name.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for Underline {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Xaml.Documents.Underline;{68aaec6e-ea71-5ed2-b83e-91684b25efb9})",
    );
}
unsafe impl ::windows::runtime::Interface for Underline {
    type Vtable = IUnderline_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1756032110,
        60017,
        24274,
        [184, 62, 145, 104, 75, 37, 239, 185],
    );
}
impl ::windows::runtime::RuntimeName for Underline {
    const NAME: &'static str = "Microsoft.UI.Xaml.Documents.Underline";
}
impl ::core::convert::From<Underline> for ::windows::runtime::IUnknown {
    fn from(value: Underline) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&Underline> for ::windows::runtime::IUnknown {
    fn from(value: &Underline) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for Underline {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a Underline {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<Underline> for ::windows::runtime::IInspectable {
    fn from(value: Underline) -> Self {
        value.0
    }
}
impl ::core::convert::From<&Underline> for ::windows::runtime::IInspectable {
    fn from(value: &Underline) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for Underline {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a Underline {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<Underline> for Span {
    fn from(value: Underline) -> Self {
        ::core::convert::Into::<Span>::into(&value)
    }
}
impl ::core::convert::From<&Underline> for Span {
    fn from(value: &Underline) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, Span> for Underline {
    fn into_param(self) -> ::windows::runtime::Param<'a, Span> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<Span>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, Span> for &Underline {
    fn into_param(self) -> ::windows::runtime::Param<'a, Span> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<Span>::into(
            ::core::clone::Clone::clone(self),
        ))
    }
}
impl ::core::convert::From<Underline> for Inline {
    fn from(value: Underline) -> Self {
        ::core::convert::Into::<Inline>::into(&value)
    }
}
impl ::core::convert::From<&Underline> for Inline {
    fn from(value: &Underline) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, Inline> for Underline {
    fn into_param(self) -> ::windows::runtime::Param<'a, Inline> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<Inline>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, Inline> for &Underline {
    fn into_param(self) -> ::windows::runtime::Param<'a, Inline> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<Inline>::into(
            ::core::clone::Clone::clone(self),
        ))
    }
}
impl ::core::convert::From<Underline> for TextElement {
    fn from(value: Underline) -> Self {
        ::core::convert::Into::<TextElement>::into(&value)
    }
}
impl ::core::convert::From<&Underline> for TextElement {
    fn from(value: &Underline) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, TextElement> for Underline {
    fn into_param(self) -> ::windows::runtime::Param<'a, TextElement> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<TextElement>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, TextElement> for &Underline {
    fn into_param(self) -> ::windows::runtime::Param<'a, TextElement> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<TextElement>::into(
            ::core::clone::Clone::clone(self),
        ))
    }
}
impl ::core::convert::From<Underline> for super::DependencyObject {
    fn from(value: Underline) -> Self {
        ::core::convert::Into::<super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&Underline> for super::DependencyObject {
    fn from(value: &Underline) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::DependencyObject> for Underline {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::DependencyObject> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(
            self,
        ))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::DependencyObject> for &Underline {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::DependencyObject> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(
            ::core::clone::Clone::clone(self),
        ))
    }
}
unsafe impl ::core::marker::Send for Underline {}
unsafe impl ::core::marker::Sync for Underline {}
#[doc = "*Required features: `UI_Xaml_Documents`*"]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: marker :: Copy,
    :: core :: clone :: Clone,
    :: core :: default :: Default,
    :: core :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct UnderlineStyle(pub i32);
impl UnderlineStyle {
    pub const None: UnderlineStyle = UnderlineStyle(0i32);
    pub const Single: UnderlineStyle = UnderlineStyle(1i32);
}
impl ::core::convert::From<i32> for UnderlineStyle {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for UnderlineStyle {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for UnderlineStyle {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Xaml.Documents.UnderlineStyle;i4)",
    );
}
impl ::windows::runtime::DefaultType for UnderlineStyle {
    type DefaultType = Self;
}
