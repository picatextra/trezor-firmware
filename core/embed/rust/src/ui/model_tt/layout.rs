use core::convert::{TryFrom, TryInto};

use crate::{
    error::Error,
    micropython::{buffer::Buffer, map::Map, obj::Obj, qstr::Qstr},
    ui::{
        component::{base::ComponentExt, text::paragraphs::Paragraphs, FormattedText},
        layout::obj::LayoutObj,
    },
    util,
};

use super::{
    component::{
        Bip39Input, Button, ButtonMsg, DialogMsg, Frame, HoldToConfirm, HoldToConfirmMsg,
        MnemonicKeyboard, MnemonicKeyboardMsg, PassphraseKeyboard, PassphraseKeyboardMsg,
        PinKeyboard, PinKeyboardMsg, Slip39Input, SwipePage,
    },
    constant, theme,
};

impl<T> TryFrom<DialogMsg<T, ButtonMsg, ButtonMsg>> for Obj
where
    Obj: TryFrom<T>,
    Error: From<<Obj as TryFrom<T>>::Error>,
{
    type Error = Error;

    fn try_from(val: DialogMsg<T, ButtonMsg, ButtonMsg>) -> Result<Self, Self::Error> {
        match val {
            DialogMsg::Content(c) => Ok(c.try_into()?),
            DialogMsg::Left(ButtonMsg::Clicked) => 1.try_into(),
            DialogMsg::Right(ButtonMsg::Clicked) => 2.try_into(),
            _ => Ok(Obj::const_none()),
        }
    }
}

impl<T> TryFrom<HoldToConfirmMsg<T>> for Obj
where
    Obj: TryFrom<T>,
    Error: From<<Obj as TryFrom<T>>::Error>,
{
    type Error = Error;

    fn try_from(val: HoldToConfirmMsg<T>) -> Result<Self, Self::Error> {
        match val {
            HoldToConfirmMsg::Content(c) => Ok(c.try_into()?),
            HoldToConfirmMsg::Confirmed => 1.try_into(),
            HoldToConfirmMsg::Cancelled => 2.try_into(),
        }
    }
}

impl TryFrom<PinKeyboardMsg> for Obj {
    type Error = Error;

    fn try_from(val: PinKeyboardMsg) -> Result<Self, Self::Error> {
        match val {
            PinKeyboardMsg::Confirmed => 1.try_into(),
            PinKeyboardMsg::Cancelled => 2.try_into(),
        }
    }
}

impl TryFrom<MnemonicKeyboardMsg> for Obj {
    type Error = Error;

    fn try_from(val: MnemonicKeyboardMsg) -> Result<Self, Self::Error> {
        match val {
            MnemonicKeyboardMsg::Confirmed => Ok(Obj::const_true()),
        }
    }
}

impl TryFrom<PassphraseKeyboardMsg> for Obj {
    type Error = Error;

    fn try_from(val: PassphraseKeyboardMsg) -> Result<Self, Self::Error> {
        match val {
            PassphraseKeyboardMsg::Confirmed => Ok(Obj::const_true()),
            PassphraseKeyboardMsg::Cancelled => Ok(Obj::const_none()),
        }
    }
}

#[no_mangle]
extern "C" fn ui_layout_new_example(_param: Obj) -> Obj {
    let block = move || {
        let layout = LayoutObj::new(HoldToConfirm::new(constant::screen(), |area| {
            FormattedText::new::<theme::TTDefaultText>(
                area,
                "Testing text layout, with some text, and some more text. And {param}",
            )
            .with(b"param", b"parameters!")
        }))?;
        Ok(layout.into())
    };
    unsafe { util::try_or_raise(block) }
}

#[no_mangle]
extern "C" fn ui_layout_new_confirm_action(
    n_args: usize,
    args: *const Obj,
    kwargs: *const Map,
) -> Obj {
    let block = move |_args: &[Obj], kwargs: &Map| {
        let title: Buffer = kwargs.get(Qstr::MP_QSTR_title)?.try_into()?;
        let action: Option<Buffer> = kwargs.get(Qstr::MP_QSTR_action)?.try_into_option()?;
        let description: Option<Buffer> =
            kwargs.get(Qstr::MP_QSTR_description)?.try_into_option()?;
        let verb: Option<Buffer> = kwargs.get(Qstr::MP_QSTR_verb)?.try_into_option()?;
        let reverse: bool = kwargs.get(Qstr::MP_QSTR_reverse)?.try_into()?;

        let obj = LayoutObj::new(
            Frame::new(theme::borders_scroll(), title, |area| {
                SwipePage::new(
                    area,
                    theme::BG,
                    |area| {
                        let action = action.unwrap_or("".into());
                        let description = description.unwrap_or("".into());
                        let mut para = Paragraphs::new(area);
                        if !reverse {
                            para = para
                                .add::<theme::TTDefaultText>(theme::FONT_BOLD, action)
                                .add::<theme::TTDefaultText>(theme::FONT_NORMAL, description);
                        } else {
                            para = para
                                .add::<theme::TTDefaultText>(theme::FONT_NORMAL, description)
                                .add::<theme::TTDefaultText>(theme::FONT_BOLD, action);
                        }
                        para
                    },
                    |area| {
                        Button::array2(
                            area,
                            |area| Button::with_icon(area, theme::ICON_CANCEL),
                            |msg| (matches!(msg, ButtonMsg::Clicked)).then(|| false),
                            |area| {
                                Button::with_text(area, verb.unwrap_or("CONFIRM".into()))
                                    .styled(theme::button_confirm())
                            },
                            |msg| (matches!(msg, ButtonMsg::Clicked)).then(|| true),
                        )
                    },
                )
            })
            .into_child(),
        )?;
        Ok(obj.into())
    };
    unsafe { util::try_with_args_and_kwargs(n_args, args, kwargs, block) }
}

#[no_mangle]
extern "C" fn ui_layout_new_pin(n_args: usize, args: *const Obj, kwargs: *const Map) -> Obj {
    let block = move |_args: &[Obj], kwargs: &Map| {
        let prompt: Buffer = kwargs.get(Qstr::MP_QSTR_prompt)?.try_into()?;
        let subprompt: Buffer = kwargs.get(Qstr::MP_QSTR_subprompt)?.try_into()?;
        let allow_cancel: Option<bool> =
            kwargs.get(Qstr::MP_QSTR_allow_cancel)?.try_into_option()?;
        let warning: Option<Buffer> = kwargs.get(Qstr::MP_QSTR_warning)?.try_into_option()?;
        let obj = LayoutObj::new(
            PinKeyboard::new(
                theme::borders(),
                prompt,
                subprompt,
                warning,
                allow_cancel.unwrap_or(true),
            )
            .into_child(),
        )?;
        Ok(obj.into())
    };
    unsafe { util::try_with_args_and_kwargs(n_args, args, kwargs, block) }
}

#[no_mangle]
extern "C" fn ui_layout_new_passphrase(n_args: usize, args: *const Obj, kwargs: *const Map) -> Obj {
    let block = move |_args: &[Obj], kwargs: &Map| {
        let _prompt: Buffer = kwargs.get(Qstr::MP_QSTR_prompt)?.try_into()?;
        let _max_len: u32 = kwargs.get(Qstr::MP_QSTR_max_len)?.try_into()?;
        let obj = LayoutObj::new(PassphraseKeyboard::new(theme::borders()).into_child())?;
        Ok(obj.into())
    };
    unsafe { util::try_with_args_and_kwargs(n_args, args, kwargs, block) }
}

#[no_mangle]
extern "C" fn ui_layout_new_bip39(n_args: usize, args: *const Obj, kwargs: *const Map) -> Obj {
    let block = move |_args: &[Obj], kwargs: &Map| {
        let _prompt: Buffer = kwargs.get(Qstr::MP_QSTR_prompt)?.try_into()?;
        let obj = LayoutObj::new(
            MnemonicKeyboard::<Bip39Input>::new(theme::borders(), b"Type word 11 of 12")
                .into_child(),
        )?;
        Ok(obj.into())
    };
    unsafe { util::try_with_args_and_kwargs(n_args, args, kwargs, block) }
}

#[no_mangle]
extern "C" fn ui_layout_new_slip39(n_args: usize, args: *const Obj, kwargs: *const Map) -> Obj {
    let block = move |_args: &[Obj], kwargs: &Map| {
        let _prompt: Buffer = kwargs.get(Qstr::MP_QSTR_prompt)?.try_into()?;
        let obj = LayoutObj::new(
            MnemonicKeyboard::<Slip39Input>::new(theme::borders(), b"Type word 13 of 20")
                .into_child(),
        )?;
        Ok(obj.into())
    };
    unsafe { util::try_with_args_and_kwargs(n_args, args, kwargs, block) }
}

#[cfg(test)]
mod tests {
    use crate::{
        trace::Trace,
        ui::{
            component::Child,
            model_tt::component::{Button, Dialog},
        },
    };

    use super::*;

    fn trace(val: &impl Trace) -> String {
        let mut t = Vec::new();
        val.trace(&mut t);
        String::from_utf8(t).unwrap()
    }

    #[test]
    fn trace_example_layout() {
        let layout = Child::new(Dialog::new(
            constant::screen(),
            |area| {
                FormattedText::new::<theme::TTDefaultText>(
                    area,
                    "Testing text layout, with some text, and some more text. And {param}",
                )
                .with(b"param", b"parameters!")
            },
            |area| Button::with_text(area, b"Left"),
            |area| Button::with_text(area, b"Right"),
        ));
        assert_eq!(
            trace(&layout),
            "<Dialog content:<Text content:Testing text layout, with\nsome text, and some more\ntext. And parameters! > left:<Button text:Left > right:<Button text:Right > >",
        )
    }
}
