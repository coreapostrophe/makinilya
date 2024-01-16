pub fn get_styles() -> clap::builder::Styles {
    clap::builder::Styles::styled()
        .placeholder(
            anstyle::Style::new()
                .bold()
                .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Cyan))),
        )
        .usage(anstyle::Style::new().underline())
        .header(anstyle::Style::new().underline())
        .error(anstyle::Style::new().fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Red))))
        .literal(
            anstyle::Style::new()
                .bold()
                .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Green))),
        )
}
