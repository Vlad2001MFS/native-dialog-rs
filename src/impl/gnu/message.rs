use super::{should_use, UseCommand};
use crate::{Dialog, Error, MessageAlert, MessageConfirm, MessageType, Result};
use std::process::Command;

impl Dialog for MessageAlert<'_> {
    type Output = ();

    fn show(&mut self) -> Result<Self::Output> {
        match should_use() {
            Some(UseCommand::KDialog(command)) => {
                dialog_implementation_kdialog(ImplementationParams {
                    command,
                    title: self.title,
                    text: self.text,
                    typ: self.typ,
                    ask: false,
                })?;
                Ok(())
            }
            Some(UseCommand::Zenity(command)) => {
                dialog_implementation_zenity(ImplementationParams {
                    command,
                    title: self.title,
                    text: self.text,
                    typ: self.typ,
                    ask: false,
                })?;
                Ok(())
            }
            None => Err(Error::NoImplementation),
        }
    }
}

impl Dialog for MessageConfirm<'_> {
    type Output = bool;

    fn show(&mut self) -> Result<Self::Output> {
        match should_use() {
            Some(UseCommand::KDialog(command)) => {
                dialog_implementation_kdialog(ImplementationParams {
                    command,
                    title: self.title,
                    text: self.text,
                    typ: self.typ,
                    ask: true,
                })
            }
            Some(UseCommand::Zenity(command)) => {
                dialog_implementation_zenity(ImplementationParams {
                    command,
                    title: self.title,
                    text: self.text,
                    typ: self.typ,
                    ask: true,
                })
            }
            None => Err(Error::NoImplementation),
        }
    }
}

struct ImplementationParams<'a> {
    command: Command,
    title: &'a str,
    text: &'a str,
    typ: MessageType,
    ask: bool,
}

fn dialog_implementation_kdialog(mut params: ImplementationParams) -> Result<bool> {
    let command = &mut params.command;

    if params.ask {
        command.arg("--yesno");
    } else {
        command.arg("--msgbox");
    }

    command.arg(params.text);

    command.arg("--title");
    command.arg(params.title);

    match params.typ {
        MessageType::Info => command.arg("--icon=dialog-information"),
        MessageType::Warning => command.arg("--icon=dialog-warning"),
        MessageType::Error => command.arg("--icon=dialog-error"),
    };

    let output = command.output()?;

    match output.status.code() {
        Some(0) => Ok(true),
        Some(_) => Ok(false),
        _ => Err(Error::UnexpectedOutput("kdialog")),
    }
}

fn dialog_implementation_zenity(mut params: ImplementationParams) -> Result<bool> {
    let command = &mut params.command;

    command.arg("--width=400");

    if params.ask {
        command.arg("--question");
        match params.typ {
            MessageType::Info => command.arg("--icon-name=dialog-information"),
            MessageType::Warning => command.arg("--icon-name=dialog-warning"),
            MessageType::Error => command.arg("--icon-name=dialog-error"),
        };
    } else {
        match params.typ {
            MessageType::Info => command.arg("--info"),
            MessageType::Warning => command.arg("--warning"),
            MessageType::Error => command.arg("--error"),
        };
    }

    command.arg("--title");
    command.arg(params.title);

    command.arg("--text");
    command.arg(params.text);

    let output = command.output()?;

    match output.status.code() {
        Some(0) => Ok(true),
        Some(_) => Ok(false),
        _ => Err(Error::UnexpectedOutput("zenity")),
    }
}
