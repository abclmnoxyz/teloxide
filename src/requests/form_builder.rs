use std::{borrow::Cow, path::PathBuf};

use reqwest::multipart::Form;

use crate::{
    requests::utils::{file_from_memory_to_part, file_to_part},
    types::{
        ChatId, InlineKeyboardMarkup, InputFile, InputMedia, MaskPosition,
        ParseMode, ReplyMarkup,
    },
};

/// This is a convenient struct that builds `reqwest::multipart::Form`
/// from scratch.
pub(crate) struct FormBuilder {
    form: Form,
}

impl FormBuilder {
    pub(crate) fn new() -> Self {
        Self { form: Form::new() }
    }

    /// Add the supplied key-value pair to this `FormBuilder`.
    pub async fn add<'a, T, N>(self, name: N, value: &T) -> Self
    where
        N: Into<Cow<'a, str>>,
        T: IntoFormValue,
    {
        let name = name.into().into_owned();
        match value.into_form_value() {
            Some(FormValue::Str(string)) => {
                Self { form: self.form.text(name, string) }
            }
            Some(FormValue::File(path)) => self.add_file(name, path).await,
            Some(FormValue::Memory { file_name, data }) => {
                self.add_file_from_memory(name, file_name, data)
            }
            None => self,
        }
    }

    // used in SendMediaGroup
    pub async fn add_file<'a, N>(self, name: N, path_to_file: PathBuf) -> Self
    where
        N: Into<Cow<'a, str>>,
    {
        Self {
            form: self.form.part(
                name.into().into_owned(),
                file_to_part(path_to_file).await,
            ),
        }
    }

    fn add_file_from_memory<'a, N>(
        self,
        name: N,
        file_name: String,
        data: Cow<'static, [u8]>,
    ) -> Self
    where
        N: Into<Cow<'a, str>>,
    {
        Self {
            form: self.form.part(
                name.into().into_owned(),
                file_from_memory_to_part(data, file_name),
            ),
        }
    }

    pub fn build(self) -> Form {
        self.form
    }
}

pub(crate) enum FormValue {
    File(PathBuf),
    Memory { file_name: String, data: Cow<'static, [u8]> },
    Str(String),
}

pub(crate) trait IntoFormValue {
    fn into_form_value(&self) -> Option<FormValue>;
}

macro_rules! impl_for_struct {
    ($($name:ty),*) => {
        $(
            impl IntoFormValue for $name {
                fn into_form_value(&self) -> Option<FormValue> {
                    let json = serde_json::to_string(self)
                        .expect("serde_json::to_string failed");
                    Some(FormValue::Str(json))
                }
            }
        )*
    };
}

impl_for_struct!(
    bool,
    i32,
    i64,
    u32,
    ReplyMarkup,
    InlineKeyboardMarkup,
    MaskPosition
);

impl<T> IntoFormValue for Option<T>
where
    T: IntoFormValue,
{
    fn into_form_value(&self) -> Option<FormValue> {
        self.as_ref().and_then(IntoFormValue::into_form_value)
    }
}

// TODO: fix InputMedia implementation of IntoFormValue (for now it doesn't
// encode files :|)
impl IntoFormValue for Vec<InputMedia> {
    fn into_form_value(&self) -> Option<FormValue> {
        let json =
            serde_json::to_string(self).expect("serde_json::to_string failed");
        Some(FormValue::Str(json))
    }
}

impl IntoFormValue for Vec<String> {
    fn into_form_value(&self) -> Option<FormValue> {
        let json =
            serde_json::to_string(self).expect("serde_json::to_string failed");
        Some(FormValue::Str(json))
    }
}

impl IntoFormValue for InputMedia {
    fn into_form_value(&self) -> Option<FormValue> {
        let json =
            serde_json::to_string(self).expect("serde_json::to_string failed");
        Some(FormValue::Str(json))
    }
}

impl IntoFormValue for str {
    fn into_form_value(&self) -> Option<FormValue> {
        Some(FormValue::Str(self.to_owned()))
    }
}

impl IntoFormValue for ParseMode {
    fn into_form_value(&self) -> Option<FormValue> {
        let string = match self {
            ParseMode::MarkdownV2 => String::from("MarkdownV2"),
            ParseMode::HTML => String::from("HTML"),
            #[allow(deprecated)]
            ParseMode::Markdown => String::from("Markdown"),
            ParseMode::Fanbook => String::from("Fanbook"),
        };
        Some(FormValue::Str(string))
    }
}

impl IntoFormValue for ChatId {
    fn into_form_value(&self) -> Option<FormValue> {
        let string = match self {
            ChatId::Id(id) => id.to_string(),
            ChatId::ChannelUsername(username) => username.clone(),
        };
        Some(FormValue::Str(string))
    }
}

impl IntoFormValue for String {
    fn into_form_value(&self) -> Option<FormValue> {
        Some(FormValue::Str(self.clone()))
    }
}

impl IntoFormValue for InputFile {
    fn into_form_value(&self) -> Option<FormValue> {
        match self {
            InputFile::File(path) => Some(FormValue::File(path.clone())),
            InputFile::Memory { file_name, data } => Some(FormValue::Memory {
                file_name: file_name.clone(),
                data: data.clone(),
            }),
            InputFile::Url(url) => Some(FormValue::Str(url.clone())),
            InputFile::FileId(file_id) => Some(FormValue::Str(file_id.clone())),
        }
    }
}
