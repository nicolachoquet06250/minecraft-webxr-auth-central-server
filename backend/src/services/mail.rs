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

        let body = format_request_mail_html(&payload);
        let email = Message::builder()
            .from(config.smtp_from.parse::<Mailbox>().context("Invalid SMTP_FROM")?)
            .reply_to(payload.sender_email.parse::<Mailbox>().context("Invalid sender email")?)
            .to(recipient.parse::<Mailbox>().context("Invalid recipient email")?)
            .subject(format!("{} {}", subject_prefix, payload.subject))
            .header(ContentType::TEXT_HTML)
            .body(body)
            .context("Failed to build email")?;

        self.send_message(email).await
    }

    pub async fn send_welcome_email(&self, email: &str, username: &str) -> Result<()> {
        let config = self
            .config
            .as_ref()
            .ok_or_else(|| anyhow!("Mail service is not configured"))?;

        let email = Message::builder()
            .from(config.smtp_from.parse::<Mailbox>().context("Invalid SMTP_FROM")?)
            .to(email.parse::<Mailbox>().context("Invalid welcome recipient email")?)
            .subject("Bienvenue sur Voxicraft")
            .header(ContentType::TEXT_HTML)
            .body(format_welcome_mail_html(username))
            .context("Failed to build welcome email")?;

        self.send_message(email).await
    }

    pub async fn send_password_change_code_email(&self, email: &str, username: &str, code: &str) -> Result<()> {
        let config = self
            .config
            .as_ref()
            .ok_or_else(|| anyhow!("Mail service is not configured"))?;

        let email = Message::builder()
            .from(config.smtp_from.parse::<Mailbox>().context("Invalid SMTP_FROM")?)
            .to(email.parse::<Mailbox>().context("Invalid password code recipient email")?)
            .subject("Code de confirmation Voxicraft")
            .header(ContentType::TEXT_HTML)
            .body(format_password_change_code_mail_html(username, code))
            .context("Failed to build password code email")?;

        self.send_message(email).await
    }

    async fn send_message(&self, email: Message) -> Result<()> {
        let config = self
            .config
            .as_ref()
            .ok_or_else(|| anyhow!("Mail service is not configured"))?;

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

fn format_welcome_mail_html(username: &str) -> String {
    let username = escape_html(username);
    format!(
        r#"<!doctype html>
<html lang="fr">
  <body style="margin:0;background:#101820;font-family:Arial,sans-serif;color:#ffffff;">
    <table role="presentation" width="100%" cellspacing="0" cellpadding="0" style="background:#101820;padding:32px 12px;">
      <tr><td align="center">
        <table role="presentation" width="640" cellspacing="0" cellpadding="0" style="max-width:640px;background:#1b2b34;border:4px solid #64ffda;border-radius:16px;overflow:hidden;box-shadow:0 10px 30px rgba(0,0,0,.35);">
          <tr><td style="padding:30px;background:linear-gradient(135deg,#243b4a,#1b2b34);text-align:center;">
            <div style="font-size:34px;line-height:1;margin-bottom:10px;">⛏️</div>
            <h1 style="margin:0;color:#64ffda;font-size:28px;">Bienvenue sur Voxicraft</h1>
            <p style="margin:12px 0 0;color:#d7fff7;font-size:15px;">Votre compte a bien été créé.</p>
          </td></tr>
          <tr><td style="padding:28px 30px;">
            <p style="font-size:16px;line-height:1.7;margin:0 0 18px;">Bonjour <strong style="color:#ffd700;">{username}</strong>,</p>
            <p style="font-size:16px;line-height:1.7;margin:0 0 18px;">Votre espace Voxicraft est prêt. Vous pouvez maintenant gérer votre profil, créer votre avatar personnalisé et enregistrer vos serveurs de jeu.</p>
            <div style="background:#0d171d;border:1px solid rgba(100,255,218,.35);border-radius:12px;padding:18px;margin:24px 0;">
              <p style="margin:0 0 10px;color:#64ffda;font-weight:bold;">Prochaines étapes</p>
              <ul style="margin:0;padding-left:20px;color:#ffffff;line-height:1.8;">
                <li>Personnaliser votre avatar</li>
                <li>Créer ou enregistrer un serveur</li>
                <li>Consulter la documentation API si vous développez une intégration</li>
              </ul>
            </div>
            <p style="font-size:14px;line-height:1.6;color:#a7bbc5;margin:0;">Si vous n’êtes pas à l’origine de cette inscription, vous pouvez ignorer ce message.</p>
          </td></tr>
          <tr><td style="padding:18px 30px;background:#0d171d;color:#7ea7b5;font-size:12px;text-align:center;">Voxicraft Auth Platform · Rust 🦀 + Vue.js 💚</td></tr>
        </table>
      </td></tr>
    </table>
  </body>
</html>"#,
    )
}

fn format_password_change_code_mail_html(username: &str, code: &str) -> String {
    let username = escape_html(username);
    let code = escape_html(code);
    format!(
        r#"<!doctype html>
<html lang="fr">
  <body style="margin:0;background:#111827;font-family:Arial,sans-serif;color:#ffffff;">
    <table role="presentation" width="100%" cellspacing="0" cellpadding="0" style="background:#111827;padding:32px 12px;">
      <tr><td align="center">
        <table role="presentation" width="620" cellspacing="0" cellpadding="0" style="max-width:620px;background:#1f2937;border:4px solid #ffb300;border-radius:16px;overflow:hidden;">
          <tr><td style="padding:28px;text-align:center;background:#0f172a;">
            <div style="font-size:34px;margin-bottom:10px;">🔐</div>
            <h1 style="margin:0;color:#ffb300;font-size:25px;">Code de sécurité</h1>
            <p style="margin:10px 0 0;color:#cbd5e1;">Confirmation de changement de mot de passe</p>
          </td></tr>
          <tr><td style="padding:28px;">
            <p style="font-size:16px;line-height:1.7;margin:0 0 18px;">Bonjour <strong style="color:#64ffda;">{username}</strong>,</p>
            <p style="font-size:16px;line-height:1.7;margin:0 0 18px;">Voici le code à saisir pour confirmer le changement de votre mot de passe :</p>
            <div style="font-size:34px;letter-spacing:10px;text-align:center;color:#111827;background:#ffb300;border-radius:12px;padding:18px 12px;font-weight:bold;margin:24px 0;">{code}</div>
            <p style="font-size:14px;line-height:1.6;color:#cbd5e1;margin:0;">Ce code expire dans 10 minutes. Si vous n’êtes pas à l’origine de cette demande, ne le partagez pas et ignorez ce message.</p>
          </td></tr>
        </table>
      </td></tr>
    </table>
  </body>
</html>"#,
    )
}

fn format_request_mail_html(payload: &MailPayload) -> String {
    let is_support = matches!(payload.kind, MailKind::Support);
    let title = if is_support { "Demande support" } else { "Message contact" };
    let accent = if is_support { "#ffb300" } else { "#64ffda" };
    let badge = if is_support { "SUPPORT" } else { "CONTACT" };
    let subtitle = if is_support {
        "Une demande d'assistance a été envoyée depuis l'espace utilisateur."
    } else {
        "Un message a été envoyé depuis le formulaire de contact public."
    };
    let metadata = payload
        .metadata
        .iter()
        .map(|(label, value)| format!(
            "<tr><td style=\"padding:8px 0;color:#90a4ae;\">{}</td><td style=\"padding:8px 0;color:#ffffff;font-weight:bold;text-align:right;\">{}</td></tr>",
            escape_html(label),
            escape_html(value),
        ))
        .collect::<Vec<_>>()
        .join("");

    format!(
        r#"<!doctype html>
<html lang="fr">
  <body style="margin:0;background:#121212;font-family:Arial,sans-serif;color:#ffffff;">
    <table role="presentation" width="100%" cellspacing="0" cellpadding="0" style="background:#121212;padding:28px 12px;">
      <tr><td align="center">
        <table role="presentation" width="680" cellspacing="0" cellpadding="0" style="max-width:680px;background:#1f1f1f;border-left:8px solid {accent};border-radius:14px;overflow:hidden;">
          <tr><td style="padding:24px 28px;background:#2b2b2b;">
            <span style="display:inline-block;background:{accent};color:#111;padding:6px 10px;border-radius:999px;font-weight:bold;font-size:12px;letter-spacing:.08em;">{badge}</span>
            <h1 style="margin:16px 0 6px;color:{accent};font-size:24px;">{title}</h1>
            <p style="margin:0;color:#cfd8dc;font-size:14px;">{subtitle}</p>
          </td></tr>
          <tr><td style="padding:24px 28px;">
            <table role="presentation" width="100%" cellspacing="0" cellpadding="0" style="border-collapse:collapse;margin-bottom:20px;">
              <tr><td style="padding:8px 0;color:#90a4ae;">Nom</td><td style="padding:8px 0;color:#ffffff;font-weight:bold;text-align:right;">{sender_name}</td></tr>
              <tr><td style="padding:8px 0;color:#90a4ae;">Email</td><td style="padding:8px 0;color:#ffffff;font-weight:bold;text-align:right;">{sender_email}</td></tr>
              <tr><td style="padding:8px 0;color:#90a4ae;">Sujet</td><td style="padding:8px 0;color:#ffffff;font-weight:bold;text-align:right;">{subject}</td></tr>
              {metadata}
            </table>
            <div style="background:#111;border:1px solid rgba(255,255,255,.12);border-radius:10px;padding:18px;">
              <p style="margin:0 0 8px;color:{accent};font-weight:bold;">Message</p>
              <div style="white-space:pre-wrap;color:#ffffff;line-height:1.7;font-size:15px;">{message}</div>
            </div>
          </td></tr>
        </table>
      </td></tr>
    </table>
  </body>
</html>"#,
        accent = accent,
        badge = badge,
        title = title,
        subtitle = subtitle,
        sender_name = escape_html(&payload.sender_name),
        sender_email = escape_html(&payload.sender_email),
        subject = escape_html(&payload.subject),
        metadata = metadata,
        message = escape_html(&payload.message),
    )
}

fn escape_html(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}
