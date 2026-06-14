use anyhow::{anyhow, Context, Result};
use lettre::{
    message::{header::ContentType, Mailbox},
    transport::smtp::authentication::Credentials,
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
};
use std::env;

#[derive(Clone)]
pub struct MailService {
    config: Option<MailConfig>,
}

#[derive(Clone)]
struct MailConfig {
    smtp_host: String,
    smtp_port: u16,
    smtp_username: String,
    smtp_password: String,
    smtp_from: String,
    contact_to: String,
    support_to: String,
    starttls: bool,
}

#[derive(Debug, Clone)]
pub struct MailPayload {
    pub sender_name: String,
    pub sender_email: String,
    pub subject: String,
    pub message: String,
    pub kind: MailKind,
    pub metadata: Vec<(String, String)>,
}

#[derive(Debug, Clone)]
pub enum MailKind {
    Contact,
    Support,
}

impl MailService {
    pub fn from_env() -> Self {
        let smtp_host = env::var("SMTP_HOST").unwrap_or_default();
        let smtp_username = env::var("SMTP_USERNAME").unwrap_or_default();
        let smtp_password = env::var("SMTP_PASSWORD").unwrap_or_default();
        let smtp_from = env::var("SMTP_FROM").unwrap_or_default();

        if smtp_host.is_empty() || smtp_username.is_empty() || smtp_password.is_empty() || smtp_from.is_empty() {
            tracing::warn!("SMTP mail service disabled: SMTP_HOST, SMTP_USERNAME, SMTP_PASSWORD or SMTP_FROM is missing");
            return Self { config: None };
        }

        let contact_to = env::var("CONTACT_EMAIL").unwrap_or_else(|_| smtp_from.clone());
        let support_to = env::var("SUPPORT_EMAIL").unwrap_or_else(|_| contact_to.clone());
        let smtp_port = env::var("SMTP_PORT")
            .ok()
            .and_then(|value| value.parse::<u16>().ok())
            .unwrap_or(587);
        let starttls = env::var("SMTP_STARTTLS")
            .map(|value| value != "false" && value != "0")
            .unwrap_or(true);

        Self {
            config: Some(MailConfig {
                smtp_host,
                smtp_port,
                smtp_username,
                smtp_password,
                smtp_from,
                contact_to,
                support_to,
                starttls,
            }),
        }
    }

    pub fn is_enabled(&self) -> bool {
        self.config.is_some()
    }

    pub async fn send(&self, payload: MailPayload) -> Result<()> {
        let config = self
            .config
            .as_ref()
            .ok_or_else(|| anyhow!("Mail service is not configured"))?;

        let recipient = match payload.kind {
            MailKind::Contact => &config.contact_to,
            MailKind::Support => &config.support_to,
        };

        let subject_prefix = match payload.kind {
            MailKind::Contact => "[Voxicraft Contact]",
            MailKind::Support => "[Voxicraft Support]",
        };

        let body = format_mail_body(&payload);
        let email = Message::builder()
            .from(config.smtp_from.parse::<Mailbox>().context("Invalid SMTP_FROM")?)
            .reply_to(payload.sender_email.parse::<Mailbox>().context("Invalid sender email")?)
            .to(recipient.parse::<Mailbox>().context("Invalid recipient email")?)
            .subject(format!("{} {}", subject_prefix, payload.subject))
            .header(ContentType::TEXT_PLAIN)
            .body(body)
            .context("Failed to build email")?;

        let credentials = Credentials::new(config.smtp_username.clone(), config.smtp_password.clone());
        let mailer = if config.starttls {
            AsyncSmtpTransport::<Tokio1Executor>::relay(&config.smtp_host)
                .context("Failed to create STARTTLS SMTP relay")?
        } else {
            AsyncSmtpTransport::<Tokio1Executor>::builder_dangerous(&config.smtp_host)
        }
        .port(config.smtp_port)
        .credentials(credentials)
        .build();

        mailer.send(email).await.context("Failed to send email")?;
        Ok(())
    }
}

fn format_mail_body(payload: &MailPayload) -> String {
    let metadata = payload
        .metadata
        .iter()
        .map(|(label, value)| format!("{}: {}", label, value))
        .collect::<Vec<_>>()
        .join("\n");

    format!(
        "Nom: {}\nEmail: {}\nSujet: {}\n{}\n\nMessage:\n{}",
        payload.sender_name,
        payload.sender_email,
        payload.subject,
        metadata,
        payload.message
    )
}
