macro_rules! form {
    ($($k:expr => $v:expr),* $(,)?) => {{
        let form = reqwasm::http::FormData::new()?;
        $(form.append_with_str($k, $v);)*
        Ok(form)
    }};
}

pub(crate) use form;
