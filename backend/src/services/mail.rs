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
        let smtp_port = env::var("SMTP_PORT").ok().and_then(|value| value.parse::<u16>().ok()).unwrap_or(587);
        let starttls = env::var("SMTP_STARTTLS").map(|value| value != "false" && value != "0").unwrap_or(true);

        Self {
            config: Some(MailConfig { smtp_host, smtp_port, smtp_username, smtp_password, smtp_from, contact_to, support_to, starttls }),
        }
    }

    pub fn is_enabled(&self) -> bool {
        self.config.is_some()
    }

    pub async fn send(&self, payload: MailPayload) -> Result<()> {
        let config = self.config.as_ref().ok_or_else(|| anyhow!("Mail service is not configured"))?;
        let recipient = match payload.kind { MailKind::Contact => &config.contact_to, MailKind::Support => &config.support_to };
        let subject_prefix = match payload.kind { MailKind::Contact => "[Voxicraft Contact]", MailKind::Support => "[Voxicraft Support]" };
        self.send_html(recipient, &format!("{} {}", subject_prefix, payload.subject), &format_request_mail_html(&payload)).await
    }

    pub async fn send_welcome_email(&self, email: &str, username: &str) -> Result<()> {
        self.send_html(email, "Bienvenue sur Voxicraft", &simple_mail_html("Bienvenue sur Voxicraft", &format!("Bonjour {}, votre compte a bien été créé.", escape_html(username)), "#64ffda")).await
    }

    pub async fn send_password_change_code_email(&self, email: &str, username: &str, code: &str) -> Result<()> {
        self.send_html(email, "Code de confirmation Voxicraft", &simple_mail_html("Code de confirmation", &format!("Bonjour {}, votre code est : <strong>{}</strong>", escape_html(username), escape_html(code)), "#ffb300")).await
    }

    pub async fn send_password_changed_email(&self, email: &str, username: &str) -> Result<()> {
        self.send_html(email, "Mot de passe Voxicraft modifié", &simple_mail_html("Mot de passe modifié", &format!("Bonjour {}, votre mot de passe a bien été modifié.", escape_html(username)), "#7cfc9a")).await
    }

    pub async fn send_avatar_created_email(&self, email: &str, username: &str, avatar_name: &str, preview_image_data_url: &str) -> Result<()> {
        self.send_html(email, "Nouvel avatar Voxicraft enregistré", &avatar_mail_html("Nouvel avatar enregistré", "Une copie de votre avatar a été enregistrée.", "COPIE CRÉÉE", "#64ffda", username, avatar_name, preview_image_data_url)).await
    }

    pub async fn send_avatar_updated_email(&self, email: &str, username: &str, avatar_name: &str, preview_image_data_url: &str) -> Result<()> {
        self.send_html(email, "Avatar Voxicraft modifié", &avatar_mail_html("Avatar modifié", "Votre avatar original a été écrasé avec vos modifications.", "ORIGINAL ÉCRASÉ", "#ffb300", username, avatar_name, preview_image_data_url)).await
    }

    async fn send_html(&self, recipient: &str, subject: &str, body: &str) -> Result<()> {
        let config = self.config.as_ref().ok_or_else(|| anyhow!("Mail service is not configured"))?;
        let email = Message::builder()
            .from(config.smtp_from.parse::<Mailbox>().context("Invalid SMTP_FROM")?)
            .to(recipient.parse::<Mailbox>().context("Invalid recipient email")?)
            .subject(subject)
            .header(ContentType::TEXT_HTML)
            .body(body.to_string())
            .context("Failed to build email")?;
        self.send_message(email).await
    }

    async fn send_message(&self, email: Message) -> Result<()> {
        let config = self.config.as_ref().ok_or_else(|| anyhow!("Mail service is not configured"))?;
        let credentials = Credentials::new(config.smtp_username.clone(), config.smtp_password.clone());
        let mailer = if config.starttls {
            AsyncSmtpTransport::<Tokio1Executor>::relay(&config.smtp_host).context("Failed to create STARTTLS SMTP relay")?
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

fn avatar_mail_html(title: &str, description: &str, badge: &str, accent: &str, username: &str, avatar_name: &str, preview_image_data_url: &str) -> String {
    format!(
        r#"<!doctype html><html lang="fr"><body style="margin:0;background:#101820;font-family:Arial,sans-serif;color:#ffffff;"><table role="presentation" width="100%" cellspacing="0" cellpadding="0" style="background:#101820;padding:32px 12px;"><tr><td align="center"><table role="presentation" width="640" cellspacing="0" cellpadding="0" style="max-width:640px;background:#1b2b34;border:4px solid {accent};border-radius:16px;overflow:hidden;"><tr><td style="padding:28px;text-align:center;background:#0d171d;"><span style="display:inline-block;background:{accent};color:#101820;padding:7px 12px;border-radius:999px;font-size:12px;font-weight:800;letter-spacing:.08em;">{badge}</span><h1 style="margin:18px 0 8px;color:{accent};font-size:26px;">{title}</h1><p style="margin:0;color:#d7fff7;font-size:15px;">{description}</p></td></tr><tr><td style="padding:28px 30px;"><p>Bonjour <strong style="color:#ffd700;">{username}</strong>,</p><p>Avatar concerné : <strong style="color:{accent};">{avatar_name}</strong></p><div style="background:#0b1419;border:1px solid rgba(255,255,255,.14);border-radius:14px;padding:22px;text-align:center;"><img src="{preview_image_data_url}" alt="Aperçu de l'avatar {avatar_name}" width="280" style="display:block;margin:0 auto;max-width:100%;height:auto;border:0;outline:none;text-decoration:none;" /></div></td></tr></table></td></tr></table></body></html>"#,
        accent = accent,
        badge = escape_html(badge),
        title = escape_html(title),
        description = escape_html(description),
        username = escape_html(username),
        avatar_name = escape_html(avatar_name),
        preview_image_data_url = escape_html(preview_image_data_url),
    )
}

fn simple_mail_html(title: &str, message: &str, accent: &str) -> String {
    format!(r#"<!doctype html><html lang="fr"><body style="margin:0;background:#101820;font-family:Arial,sans-serif;color:#ffffff;"><table role="presentation" width="100%" cellspacing="0" cellpadding="0" style="background:#101820;padding:32px 12px;"><tr><td align="center"><table role="presentation" width="620" cellspacing="0" cellpadding="0" style="max-width:620px;background:#1b2b34;border:4px solid {accent};border-radius:16px;overflow:hidden;"><tr><td style="padding:28px;"><h1 style="margin:0 0 18px;color:{accent};">{title}</h1><p style="line-height:1.7;">{message}</p></td></tr></table></td></tr></table></body></html>"#, accent = accent, title = escape_html(title), message = message)
}

fn format_request_mail_html(payload: &MailPayload) -> String {
    let is_support = matches!(payload.kind, MailKind::Support);
    let title = if is_support { "Demande support" } else { "Message contact" };
    let accent = if is_support { "#ffb300" } else { "#64ffda" };
    let badge = if is_support { "SUPPORT" } else { "CONTACT" };
    let metadata = payload.metadata.iter().map(|(label, value)| format!("<tr><td>{}</td><td>{}</td></tr>", escape_html(label), escape_html(value))).collect::<Vec<_>>().join("");
    format!(r#"<!doctype html><html lang="fr"><body style="margin:0;background:#121212;font-family:Arial,sans-serif;color:#ffffff;"><table role="presentation" width="100%" cellspacing="0" cellpadding="0" style="background:#121212;padding:28px 12px;"><tr><td align="center"><table role="presentation" width="680" cellspacing="0" cellpadding="0" style="max-width:680px;background:#1f1f1f;border-left:8px solid {accent};border-radius:14px;overflow:hidden;"><tr><td style="padding:24px 28px;background:#2b2b2b;"><span style="display:inline-block;background:{accent};color:#111;padding:6px 10px;border-radius:999px;font-weight:bold;font-size:12px;">{badge}</span><h1 style="color:{accent};">{title}</h1></td></tr><tr><td style="padding:24px 28px;"><table width="100%"><tr><td>Nom</td><td>{sender_name}</td></tr><tr><td>Email</td><td>{sender_email}</td></tr><tr><td>Sujet</td><td>{subject}</td></tr>{metadata}</table><div style="white-space:pre-wrap;background:#111;border-radius:10px;padding:18px;margin-top:20px;">{message}</div></td></tr></table></td></tr></table></body></html>"#, accent = accent, badge = badge, title = title, sender_name = escape_html(&payload.sender_name), sender_email = escape_html(&payload.sender_email), subject = escape_html(&payload.subject), metadata = metadata, message = escape_html(&payload.message))
}

fn escape_html(value: &str) -> String {
    value.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;").replace('"', "&quot;").replace('\'', "&#39;")
}
