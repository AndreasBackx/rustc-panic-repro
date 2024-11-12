use indoc::formatdoc;

fn main() {
    formatdoc!(
        r#"
            **Settings summary:**
            * **Timezone**: {timezone}
            * **Ignores code freeze**: {ignores_code_freeze}
            * **Allowed push time**: {allowed_push_time}
            * **Pushes on holidays**: {is_pushing_on_holidays}
            * [**aaaaaaaaaaaaaaaaaaaa 🖼**](https://aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaapaste_or_everstore_handle={materialized_config_pastry})
            "#,
    )
}
