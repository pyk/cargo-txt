## rustdoc file name

The current approach to determining the file json path is over-engineered.

we should follow the simple approach: if crate name have hypen `-`, we should
simply replace it with the underscore.

## type_alias module

All data is required in it should fails explicitly if data is not found. for
example this is wrong:

```
fn extract_aliased_type(document: &Html) -> error::Result<String> {
    let selector = Selector::parse("#aliased-type + pre.rust.item-decl").map_err(|error| {
        error::HtmlExtractError::SelectorParseFailed {
            selector: "#aliased-type + pre.rust.item-decl".to_string(),
            error: error.to_string(),
        }
    })?;

    let element = document.select(&selector).next();

    let Some(element) = element else {
        return Ok(String::new());
    };

    let code_selector =
        Selector::parse("code").map_err(|error| error::HtmlExtractError::SelectorParseFailed {
            selector: "code".to_string(),
            error: error.to_string(),
        })?;
    let code_element = element.select(&code_selector).next().ok_or_else(|| {
        error::HtmlExtractError::ElementNotFound {
            selector: "#aliased-type + pre.rust.item-decl code".to_string(),
        }
    })?;

    Ok(code_element.text().collect::<String>().trim().to_string())
}
```

as it returns empty string if aliased_type not found. Samething also happen in
other function such as `extract_variants` where it returns empty array.
