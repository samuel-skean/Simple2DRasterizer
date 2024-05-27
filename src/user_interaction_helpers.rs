use rfd::FileDialog;
use sdl2::messagebox::{
    show_message_box, show_simple_message_box, ButtonData, ClickedButton, MessageBoxButtonFlag,
    MessageBoxColorScheme, MessageBoxFlag,
};

pub type AlertKind = MessageBoxFlag;

const fn message_box_flag_as_str(flag: AlertKind) -> &'static str {
    match flag {
        AlertKind::ERROR => "ERROR",
        AlertKind::WARNING => "WARNING",
        AlertKind::INFORMATION => "INFORMATION",
        // I wish I could provide a useful error, but it seems I can't if I want
        // this function to be const. And I kinda do want it to be const!
        _ => unimplemented!(),
    }
}

const fn we_are_alerting_through_the_gui(from_arg: bool) -> bool {
    !from_arg || cfg!(feature = "interactive-alerts-about-args")
}
/// Report non-fatal error or warning through both the GUI and stderr,
/// respecting the compilation flag interactive-alerts-about-args.
pub fn alert(from_arg: bool, kind: AlertKind, title: &str, message: &str, window: &sdl2::video::Window) {
    report_alert_on_only_command_line(kind, message);
    if we_are_alerting_through_the_gui(from_arg) {
        eprintln!("This needs your attention on the currently open window.");
        show_simple_message_box(kind, title, message, window).unwrap()
    } else {
        std::process::exit(1); // TODO: Make this a graceful shutdown w.r.t. the window.
    }

}

fn report_alert_on_only_command_line(kind: AlertKind, message: &str) {
    let kind_str = message_box_flag_as_str(kind);
    eprintln!("{kind_str}: {message}");
}

#[must_use]
pub fn confer_with_user(
    kind: MessageBoxFlag,
    title: &str,
    message: &str,
    window: &sdl2::video::Window,
    cancel_button_name: &str,
    confirmation_button_name: &str,
) -> bool {
    let kind_str = message_box_flag_as_str(kind);
    eprintln!("{kind_str}: {message}.\n This needs your attention on the currently open window.");
    // Stretch TODO: Also prompt the user at the command line. This would
    // probably necessitate hella async though... not my goal for this project.
    let cancel_button = ButtonData {
        flags: MessageBoxButtonFlag::ESCAPEKEY_DEFAULT,
        button_id: 0,
        text: cancel_button_name,
    };
    let save_button = ButtonData {
        flags: MessageBoxButtonFlag::RETURNKEY_DEFAULT,
        button_id: 1,
        text: confirmation_button_name,
    };
    let buttons = [cancel_button, save_button];
    let color_scheme = MessageBoxColorScheme {
        background: (255, 255, 255),
        text: (0, 0, 0),
        button_border: (255, 255, 0),
        button_background: (198, 198, 198),
        button_selected: (0, 0, 0),
    };
    let message_box_answer = show_message_box(kind, &buttons, title, message, window, color_scheme)
        .expect("Displaying a fancy message box failed.");

    match message_box_answer {
        ClickedButton::CustomButton(b) if b.button_id == buttons[1].button_id => true,
        ClickedButton::CustomButton(_) | ClickedButton::CloseButton => false,
    }
}

pub fn file_dialog(allowed_extensions: &[&str]) -> FileDialog {
    let mut dialog = FileDialog::new().set_directory(".");
    for extension in allowed_extensions {
        // Stretch TODO: Figure out how to avoid converting the &str into a String.
        dialog = dialog.add_filter(extension.to_string(), &[extension])
    }
    dialog
}

// ---
// The following functions are about specific user interactions in this program, not general facilities:
// ---

pub fn alert_about_error(
    from_arg: bool,
    error: anyhow::Error,
    message_before_error: &str,
    message_after_error: &str,
    window: &sdl2::video::Window,
) {
    let error_as_string = error.to_string();
    alert(
        from_arg,
        AlertKind::ERROR,
        "Invalid World File",
        format!("{message_before_error}{error_as_string}{message_after_error}").as_str(),
        window,
    );
}

const fn world_file_message_after_error(specified_as_arg: bool) -> &'static str {
    if we_are_alerting_through_the_gui(specified_as_arg) {
        "\nYou will be prompted for a new world file once you dismiss this message."
    } else {
        ""
    }
}

pub fn alert_about_invalid_world_file(
    specified_as_arg: bool,
    error: anyhow::Error,
    window: &sdl2::video::Window,
) {
    let message_before_error = if specified_as_arg {
        "The world file specified as an argument failed to parse with this error:\n"
    } else {
        "The world file failed to parse with this error:\n"
    };

    alert_about_error(
        specified_as_arg,
        error,
        message_before_error,
        world_file_message_after_error(specified_as_arg),
        window,
    );
}

pub fn alert_about_io_error_with_world_file(
    specified_as_arg: bool,
    error: anyhow::Error,
    window: &sdl2::video::Window,
) {
    let message_before_error = if specified_as_arg {
        "There was an I/O error in reading the world file specified as an argument. Its description reads:\n"
    } else {
        "THIS WAS SLIGHTLY UNEXPECTED! :P\nPlease contact the maintainer with this error message.\nThere was an I/O error in reading the world file specified. Its description reads:\n"
    };
    alert_about_error(
        specified_as_arg,
        error,
        message_before_error,
        world_file_message_after_error(specified_as_arg),
        &window
    );
}

pub fn report_file_dialog_failure(window: &sdl2::video::Window) {
    alert(
        false,
        AlertKind::INFORMATION,
        "No Path Provided",
        "We didn't get a path back from the dialog box.",
        window,
    )
}
