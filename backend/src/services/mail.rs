use anyhow::{anyhow, Context, Result};
use base64::{engine::general_purpose::STANDARD as BASE64_STANDARD, Engine as _};
use lettre::{
    message::{header::ContentType, Attachment, Mailbox, MultiPart, SinglePart},
    transport::smtp::authentication::Credentials,
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
};
use std::env;

const AVATAR_PREVIEW_CONTENT_ID: &str = "voxicraft-avatar-preview";
const ACCENT_CYAN: &str = "#64ffda";
const ACCENT_GOLD: &str = "#ffd700";
const ACCENT_GREEN: &str = "#4caf50";
const ACCENT_AMBER: &str = "#ffb300";
const ACCENT_SUCCESS: &str = "#7cfc9a";

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

struct InlineImage {
    content_id: String,
    content_type: ContentType,
    bytes: Vec<u8>,
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

    pub fn is_enabled(&self) -> bool { self.config.is_some() }

    pub async fn send(&self, payload: MailPayload) -> Result<()> {
        let config = self.config.as_ref().ok_or_else(|| anyhow!("Mail service is not configured"))?;
        let recipient = match payload.kind { MailKind::Contact => &config.contact_to, MailKind::Support => &config.support_to };
        let subject_prefix = match payload.kind { MailKind::Contact => "[Voxicraft Contact]", MailKind::Support => "[Voxicraft Support]" };
        self.send_html(recipient, &format!("{} {}", subject_prefix, payload.subject), &format_request_mail_html(&payload)).await
    }

    pub async fn send_welcome_email(&self, email: &str, username: &str) -> Result<()> {
        self.send_html(email, "Bienvenue sur Voxicraft", &simple_mail_html("Bienvenue sur Voxicraft", "COMPTE CRÉÉ", ACCENT_CYAN, &format!("Bonjour <strong style=\"color:{};\">{}</strong>, votre compte a bien été créé.", ACCENT_GOLD, escape_html(username)))).await
    }

    pub async fn send_password_change_code_email(&self, email: &str, username: &str, code: &str) -> Result<()> {
        self.send_html(email, "Code de confirmation Voxicraft", &simple_mail_html("Code de confirmation", "SÉCURITÉ", ACCENT_AMBER, &format!("Bonjour <strong style=\"color:{};\">{}</strong>, utilisez ce code pour confirmer votre changement de mot de passe : {}", ACCENT_GOLD, escape_html(username), code_badge(code)))).await
    }

    pub async fn send_password_changed_email(&self, email: &str, username: &str) -> Result<()> {
        self.send_html(email, "Mot de passe Voxicraft modifié", &simple_mail_html("Mot de passe modifié", "PROTECTION ACTIVE", ACCENT_SUCCESS, &format!("Bonjour <strong style=\"color:{};\">{}</strong>, votre mot de passe a bien été modifié.", ACCENT_GOLD, escape_html(username)))).await
    }

    pub async fn send_avatar_created_email(&self, email: &str, username: &str, avatar_name: &str, preview_image_data_url: &str) -> Result<()> {
        self.send_avatar_email(email, "Nouvel avatar Voxicraft enregistré", "Nouvel avatar enregistré", "Une copie de votre avatar a été enregistrée dans votre profil.", "COPIE CRÉÉE", ACCENT_CYAN, username, avatar_name, preview_image_data_url).await
    }

    pub async fn send_avatar_updated_email(&self, email: &str, username: &str, avatar_name: &str, preview_image_data_url: &str) -> Result<()> {
        self.send_avatar_email(email, "Avatar Voxicraft modifié", "Avatar modifié", "Votre avatar original a été écrasé avec vos modifications.", "ORIGINAL ÉCRASÉ", ACCENT_AMBER, username, avatar_name, preview_image_data_url).await
    }

    async fn send_avatar_email(&self, recipient: &str, subject: &str, title: &str, description: &str, badge: &str, accent: &str, username: &str, avatar_name: &str, preview_image_data_url: &str) -> Result<()> {
        let inline_image = inline_image_from_data_url(preview_image_data_url, AVATAR_PREVIEW_CONTENT_ID).context("Invalid avatar preview image data URL")?;
        let image_src = format!("cid:{}", inline_image.content_id);
        let body = avatar_mail_html(title, description, badge, accent, username, avatar_name, &image_src);
        self.send_html_with_inline_image(recipient, subject, &body, inline_image).await
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

    async fn send_html_with_inline_image(&self, recipient: &str, subject: &str, body: &str, inline_image: InlineImage) -> Result<()> {
        let config = self.config.as_ref().ok_or_else(|| anyhow!("Mail service is not configured"))?;
        let image_part = Attachment::new_inline(inline_image.content_id).body(inline_image.bytes, inline_image.content_type);
        let email = Message::builder()
            .from(config.smtp_from.parse::<Mailbox>().context("Invalid SMTP_FROM")?)
            .to(recipient.parse::<Mailbox>().context("Invalid recipient email")?)
            .subject(subject)
            .multipart(MultiPart::related().singlepart(SinglePart::html(body.to_string())).singlepart(image_part))
            .context("Failed to build email with inline avatar preview")?;
        self.send_message(email).await
    }

    async fn send_message(&self, email: Message) -> Result<()> {
        let config = self.config.as_ref().ok_or_else(|| anyhow!("Mail service is not configured"))?;
        let credentials = Credentials::new(config.smtp_username.clone(), config.smtp_password.clone());
        let mailer = if config.starttls {
            AsyncSmtpTransport::<Tokio1Executor>::relay(&config.smtp_host).context("Failed to create STARTTLS SMTP relay")?
        } else {
            AsyncSmtpTransport::<Tokio1Executor>::builder_dangerous(&config.smtp_host)
        }.port(config.smtp_port).credentials(credentials).build();
        mailer.send(email).await.context("Failed to send email")?;
        Ok(())
    }
}

fn avatar_mail_html(title: &str, description: &str, badge: &str, accent: &str, username: &str, avatar_name: &str, image_src: &str) -> String {
    let content = format!(
        r#"{hero}<tr><td style="padding:0 30px 30px;"><table role="presentation" width="100%" cellspacing="0" cellpadding="0" style="background:rgba(0,0,0,.35);border:3px solid #5d4037;border-radius:10px;box-shadow:6px 6px 0 rgba(0,0,0,.38);"><tr><td style="padding:22px 22px 10px;text-align:center;"><p style="margin:0 0 12px;color:#d7ccc8;font-size:14px;line-height:1.8;">Bonjour <strong style="color:#ffd700;">{username}</strong>,</p><p style="margin:0;color:#ffffff;font-size:14px;line-height:1.8;">Avatar concerné : <strong style="color:{accent};">{avatar_name}</strong></p></td></tr><tr><td align="center" style="padding:10px 22px 26px;"><table role="presentation" cellspacing="0" cellpadding="0" style="background:#0f2409;border:4px solid #3e2723;border-radius:12px;box-shadow:8px 8px 0 rgba(0,0,0,.45);"><tr><td style="padding:14px 18px;text-align:center;"><img src="{image_src}" alt="Aperçu de l'avatar {avatar_name}" width="280" style="display:block;margin:0 auto;max-width:100%;height:auto;border:0;outline:none;text-decoration:none;" /></td></tr></table></td></tr></table></td></tr>"#,
        hero = hero_block(title, description, badge, accent), username = escape_html(username), avatar_name = escape_html(avatar_name), image_src = escape_html(image_src), accent = accent,
    );
    mail_shell(&content, accent)
}

fn simple_mail_html(title: &str, badge: &str, accent: &str, message_html: &str) -> String {
    let content = format!(
        r#"{hero}<tr><td style="padding:0 30px 30px;"><table role="presentation" width="100%" cellspacing="0" cellpadding="0" style="background:rgba(0,0,0,.35);border:3px solid #5d4037;border-radius:10px;box-shadow:6px 6px 0 rgba(0,0,0,.38);"><tr><td style="padding:24px 26px;"><p style="margin:0;color:#ffffff;font-size:14px;line-height:1.9;text-shadow:2px 2px 0 rgba(0,0,0,.35);">{message}</p></td></tr></table></td></tr>"#,
        hero = hero_block(title, "Notification de votre compte Voxicraft.", badge, accent), message = message_html,
    );
    mail_shell(&content, accent)
}

fn format_request_mail_html(payload: &MailPayload) -> String {
    let is_support = matches!(payload.kind, MailKind::Support);
    let title = if is_support { "Demande support" } else { "Message contact" };
    let accent = if is_support { ACCENT_AMBER } else { ACCENT_CYAN };
    let badge = if is_support { "SUPPORT" } else { "CONTACT" };
    let description = if is_support { "Une demande d'assistance a été envoyée depuis Voxicraft." } else { "Un message a été envoyé depuis le formulaire de contact Voxicraft." };
    let metadata = payload.metadata.iter().map(|(label, value)| info_row(label, value, accent)).collect::<Vec<_>>().join("");
    let content = format!(
        r#"{hero}<tr><td style="padding:0 30px 30px;"><table role="presentation" width="100%" cellspacing="0" cellpadding="0" style="background:rgba(0,0,0,.35);border:3px solid #5d4037;border-radius:10px;box-shadow:6px 6px 0 rgba(0,0,0,.38);"><tr><td style="padding:22px;"><table role="presentation" width="100%" cellspacing="0" cellpadding="0" style="border-collapse:separate;border-spacing:0 10px;">{sender_name}{sender_email}{subject}{metadata}</table><div style="margin-top:14px;background:#101820;border:3px solid #424242;border-radius:8px;padding:18px;color:#ffffff;font-family:'Courier New',monospace;font-size:14px;line-height:1.8;white-space:pre-wrap;box-shadow:inset 0 0 0 1px rgba(255,255,255,.05);">{message}</div></td></tr></table></td></tr>"#,
        hero = hero_block(title, description, badge, accent), sender_name = info_row("Nom", &payload.sender_name, accent), sender_email = info_row("Email", &payload.sender_email, accent), subject = info_row("Sujet", &payload.subject, accent), metadata = metadata, message = escape_html(&payload.message),
    );
    mail_shell(&content, accent)
}

fn mail_shell(content: &str, accent: &str) -> String {
    format!(
        r#"<!doctype html><html lang="fr"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"></head><body style="margin:0;padding:0;background:#0f2409;color:#ffffff;font-family:'Courier New',Arial,sans-serif;"><table role="presentation" width="100%" cellspacing="0" cellpadding="0" style="min-width:100%;background:linear-gradient(135deg,#2d5016 0%,#1a3a0f 50%,#0f2409 100%);padding:34px 12px;"><tr><td align="center"><table role="presentation" width="680" cellspacing="0" cellpadding="0" style="width:100%;max-width:680px;background:rgba(139,69,19,.94);border:4px solid #5d4037;border-radius:10px;box-shadow:8px 8px 0 rgba(0,0,0,.5);overflow:hidden;"><tr><td style="height:12px;background:linear-gradient(90deg,#3e2723 0%,{accent} 50%,#3e2723 100%);font-size:0;line-height:0;">&nbsp;</td></tr>{content}{brand_footer}</table></td></tr></table></body></html>"#,
        accent = accent,
        content = content,
        brand_footer = brand_footer(accent),
    )
}

fn brand_footer(accent: &str) -> String {
    format!(
        r#"<tr><td style="padding:0 30px 30px;"><table role="presentation" width="100%" cellspacing="0" cellpadding="0" style="border-top:1px solid rgba(255,255,255,.14);"><tr><td align="center" style="padding:22px 0 12px;"><table role="presentation" cellspacing="0" cellpadding="0" style="margin:0 auto;"><tr><td style="vertical-align:middle;padding-right:12px;"><table role="presentation" cellspacing="0" cellpadding="0" style="width:44px;height:44px;background:#0f2409;border:3px solid #3e2723;box-shadow:4px 4px 0 rgba(0,0,0,.45);"><tr><td style="width:14px;height:14px;background:#4caf50;border:1px solid #2e7d32;font-size:0;line-height:0;">&nbsp;</td><td style="width:14px;height:14px;background:#66bb6a;border:1px solid #2e7d32;font-size:0;line-height:0;">&nbsp;</td><td style="width:14px;height:14px;background:#ffd700;border:1px solid #8b6f00;font-size:0;line-height:0;">&nbsp;</td></tr><tr><td style="width:14px;height:14px;background:#388e3c;border:1px solid #1b5e20;font-size:0;line-height:0;">&nbsp;</td><td style="width:14px;height:14px;background:{accent};border:1px solid #101820;font-size:0;line-height:0;">&nbsp;</td><td style="width:14px;height:14px;background:#8b4513;border:1px solid #5d4037;font-size:0;line-height:0;">&nbsp;</td></tr><tr><td style="width:14px;height:14px;background:#3e2723;border:1px solid #1b120f;font-size:0;line-height:0;">&nbsp;</td><td style="width:14px;height:14px;background:#5d4037;border:1px solid #3e2723;font-size:0;line-height:0;">&nbsp;</td><td style="width:14px;height:14px;background:#101820;border:1px solid #000000;font-size:0;line-height:0;">&nbsp;</td></tr></table></td><td style="vertical-align:middle;text-align:left;"><div style="font-size:20px;line-height:1.2;color:#ffd700;font-weight:800;letter-spacing:.06em;text-shadow:3px 3px 0 rgba(0,0,0,.5);">Voxicraft</div><div style="margin-top:4px;color:#d7ccc8;font-size:11px;line-height:1.5;">Auth centrale · serveurs auto-hébergés</div></td></tr></table></td></tr><tr><td align="center" style="padding:0 0 6px;"><a href="https://central.voxicraft.fr" style="display:inline-block;margin:0 6px 8px;color:#ffd700;text-decoration:none;font-size:11px;line-height:1.5;">Central</a><span style="color:#5d4037;font-size:11px;">·</span><a href="https://central.voxicraft.fr/profile" style="display:inline-block;margin:0 6px 8px;color:{accent};text-decoration:none;font-size:11px;line-height:1.5;">Profil</a><span style="color:#5d4037;font-size:11px;">·</span><a href="https://central.voxicraft.fr/support" style="display:inline-block;margin:0 6px 8px;color:{accent};text-decoration:none;font-size:11px;line-height:1.5;">Support</a></td></tr><tr><td align="center" style="padding:0 0 2px;"><p style="margin:0;color:#d7ccc8;font-size:10px;line-height:1.7;">Vous recevez cet email suite à une action effectuée sur votre compte Voxicraft.</p></td></tr></table></td></tr>"#,
        accent = accent,
    )
}

fn hero_block(title: &str, description: &str, badge: &str, accent: &str) -> String {
    format!(
        r#"<tr><td style="padding:30px 30px 24px;text-align:center;background:rgba(62,39,35,.72);"><div style="display:inline-block;background:{accent};color:#101820;border:3px solid #3e2723;border-bottom-width:5px;border-right-width:4px;border-radius:0;padding:8px 12px;font-size:11px;font-weight:800;letter-spacing:.08em;text-transform:uppercase;box-shadow:4px 4px 0 rgba(0,0,0,.45);">{badge}</div><h1 style="margin:22px 0 12px;color:#ffd700;font-size:26px;line-height:1.3;text-shadow:4px 4px 0 rgba(0,0,0,.5);font-family:'Courier New',Arial,sans-serif;">{title}</h1><p style="margin:0;color:#d7ccc8;font-size:14px;line-height:1.8;text-shadow:2px 2px 0 rgba(0,0,0,.35);">{description}</p></td></tr>"#,
        accent = accent, badge = escape_html(badge), title = escape_html(title), description = escape_html(description),
    )
}

fn info_row(label: &str, value: &str, accent: &str) -> String {
    format!(
        r#"<tr><td style="width:34%;padding:11px 12px;background:#3e2723;border:2px solid #5d4037;color:#ffd700;font-size:12px;line-height:1.5;vertical-align:top;">{label}</td><td style="padding:11px 12px;background:#101820;border:2px solid #424242;color:{accent};font-size:13px;line-height:1.5;vertical-align:top;word-break:break-word;">{value}</td></tr>"#,
        label = escape_html(label), value = escape_html(value), accent = accent,
    )
}

fn code_badge(code: &str) -> String {
    format!(
        r#"<span style="display:inline-block;margin:14px 0 4px;padding:12px 16px;background:#101820;border:3px solid #424242;color:#ffd700;font-family:'Courier New',monospace;font-size:22px;letter-spacing:.18em;box-shadow:4px 4px 0 rgba(0,0,0,.45);">{}</span>"#,
        escape_html(code),
    )
}

fn inline_image_from_data_url(data_url: &str, content_id: &str) -> Result<InlineImage> {
    let Some(rest) = data_url.strip_prefix("data:") else { return Err(anyhow!("Avatar preview is not a data URL")); };
    let Some((metadata, encoded)) = rest.split_once(',') else { return Err(anyhow!("Avatar preview data URL has no payload")); };
    let Some(mime_type) = metadata.strip_suffix(";base64") else { return Err(anyhow!("Avatar preview data URL is not base64 encoded")); };
    if !matches!(mime_type, "image/png" | "image/jpeg" | "image/webp" | "image/svg+xml") { return Err(anyhow!("Unsupported avatar preview MIME type")); }
    let bytes = BASE64_STANDARD.decode(encoded).context("Failed to decode avatar preview image")?;
    let content_type = ContentType::parse(mime_type).map_err(|_| anyhow!("Invalid avatar preview MIME type"))?;
    Ok(InlineImage { content_id: content_id.to_string(), content_type, bytes })
}

fn escape_html(value: &str) -> String {
    value.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;").replace('"', "&quot;").replace('\'', "&#39;")
}
