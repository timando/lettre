//! The sendmail transport sends the email using the local sendmail command.
//!

use sendmail::error::SendmailResult;
use std::io::prelude::*;
use std::io::Read;
use std::process::{Command, Stdio};
use SendableEmail;
use Transport;

pub mod error;

/// Sends an email using the `sendmail` command
#[derive(Debug, Default)]
#[cfg_attr(feature = "serde-impls", derive(Serialize, Deserialize))]
pub struct SendmailTransport {
    command: String,
}

impl SendmailTransport {
    /// Creates a new transport with the default `/usr/sbin/sendmail` command
    pub fn new() -> SendmailTransport {
        SendmailTransport {
            command: "/usr/sbin/sendmail".to_string(),
        }
    }

    /// Creates a new transport to the given sendmail command
    pub fn new_with_command<S: Into<String>>(command: S) -> SendmailTransport {
        SendmailTransport {
            command: command.into(),
        }
    }
}

impl<'a> Transport<'a> for SendmailTransport {
    type Result = SendmailResult;

    fn send(&mut self, email: SendableEmail) -> SendmailResult {
        let message_id = email.message_id().to_string();

        // Spawn the sendmail command
        let mut process = Command::new(&self.command)
            .arg("-i")
            .arg("-f")
            .arg(
                email
                    .envelope()
                    .from()
                    .map(|x| x.as_ref())
                    .unwrap_or("\"\""),
            )
            .args(email.envelope.to())
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?;

        let mut message_content = String::new();
        let _ = email.message().read_to_string(&mut message_content);

        process
            .stdin
            .as_mut()
            .unwrap()
            .write_all(message_content.as_bytes())?;

        info!("Wrote {} message to stdin", message_id);

        let output = process.wait_with_output()?;

        if output.status.success() {
            Ok(())
        } else {
            // TODO display stderr
            Err(error::Error::Client("The message could not be sent"))
        }
    }
}
