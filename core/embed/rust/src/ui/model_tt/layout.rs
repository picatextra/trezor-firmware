use core::convert::{TryFrom, TryInto};

use crate::{
    error::Error,
    micropython::{buffer::Buffer, map::Map, module::Module, obj::Obj, qstr::Qstr},
    ui::{
        component::{base::ComponentExt, text::paragraphs::Paragraphs},
        layout::{
            obj::LayoutObj,
            result::{CANCELLED, CONFIRMED},
        },
    },
    util,
};

use super::{
    component::{
        Button, ButtonMsg, DialogMsg, Frame, HoldToConfirmMsg, PinKeyboard, PinKeyboardMsg,
        SwipePage,
    },
    theme,
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
            DialogMsg::Left(ButtonMsg::Clicked) => Ok(CANCELLED.as_obj()),
            DialogMsg::Right(ButtonMsg::Clicked) => Ok(CONFIRMED.as_obj()),
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
            HoldToConfirmMsg::Confirmed => Ok(CONFIRMED.as_obj()),
            HoldToConfirmMsg::Cancelled => Ok(CANCELLED.as_obj()),
        }
    }
}

impl TryFrom<PinKeyboardMsg> for Obj {
    type Error = Error;

    fn try_from(val: PinKeyboardMsg) -> Result<Self, Self::Error> {
        let result: (Obj, Obj) = match val {
            PinKeyboardMsg::Cancelled => (CANCELLED.as_obj(), Obj::const_none()),
            PinKeyboardMsg::Confirmed(pin) => (CONFIRMED.as_obj(), pin.as_slice().try_into()?),
        };
        result.try_into()
    }
}

extern "C" fn new_request_pin(_param: Obj) -> Obj {
    let block = move || {
        let layout = LayoutObj::new(PinKeyboard::new(theme::borders(), b"Enter PIN", b""))?;
        Ok(layout.into())
    };
    unsafe { util::try_or_raise(block) }
}

extern "C" fn new_confirm_action(n_args: usize, args: *const Obj, kwargs: *mut Map) -> Obj {
    let block = move |_args: &[Obj], kwargs: &Map| {
        let title: Buffer = kwargs.get(Qstr::MP_QSTR_title)?.try_into()?;
        let action: Option<Buffer> = kwargs.get(Qstr::MP_QSTR_action)?.try_into_option()?;
        let description: Option<Buffer> =
            kwargs.get(Qstr::MP_QSTR_description)?.try_into_option()?;
        let verb: Option<Buffer> = kwargs.get(Qstr::MP_QSTR_verb)?.try_into_option()?;
        let reverse: bool = kwargs.get(Qstr::MP_QSTR_reverse)?.try_into()?;

        let obj = LayoutObj::new(
            Frame::new(theme::borders(), title, |area| {
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
pub static mp_module_trezorui2: Module = obj_module!(&obj_dict!(obj_map! {
    Qstr::MP_QSTR___name__ => Qstr::MP_QSTR_trezorui2.to_obj(),
    Qstr::MP_QSTR_CONFIRMED => CONFIRMED.as_obj(),
    Qstr::MP_QSTR_CANCELLED => CANCELLED.as_obj(),
    Qstr::MP_QSTR_confirm_action => obj_fn_kw!(0, new_confirm_action).as_obj(),
    Qstr::MP_QSTR_request_pin => obj_fn_1!(new_request_pin).as_obj(),
}));

#[cfg(test)]
mod tests {
    use crate::{
        trace::Trace,
        ui::{
            component::{Child, FormattedText},
            display,
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
            display::screen(),
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
