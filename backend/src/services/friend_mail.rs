use anyhow::{anyhow, Context, Result};
use lettre::{
    message::{header::ContentType, Mailbox},
    transport::smtp::authentication::Credentials,
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
};
use std::env;

use super::MailService;

const ACCENT_CYAN: &str = "#64ffda";
const ACCENT_GOLD: &str = "#ffd700";
const BRAND_LOGO_URL: &str = "https://central.voxicraft.fr/favicon.png";

impl MailService {
    pub async fn send_friend_request_email(
        &self,
        email: &str,
        username: &str,
        requester_username: &str,
        accept_url: &str,
    ) -> Result<()> {
        let config = FriendMailConfig::from_env()?;
        let body = friend_request_mail_html(username, requester_username, accept_url);
        let email = Message::builder()
            .from(config.smtp_from.parse::<Mailbox>().context("Invalid SMTP_FROM")?)
            .to(email.parse::<Mailbox>().context("Invalid recipient email")?)
            .subject("Nouvelle demande d'ami Voxicraft")
            .header(ContentType::TEXT_HTML)
            .body(body)
            .context("Failed to build friend request email")?;

        let credentials = Credentials::new(config.smtp_username, config.smtp_password);
        let mailer = if config.starttls {
            AsyncSmtpTransport::<Tokio1Executor>::relay(&config.smtp_host)
                .context("Failed to create STARTTLS SMTP relay")?
        } else {
            AsyncSmtpTransport::<Tokio1Executor>::builder_dangerous(&config.smtp_host)
        }
        .port(config.smtp_port)
        .credentials(credentials)
        .build();

        mailer.send(email).await.context("Failed to send friend request email")?;
        Ok(())
    }
}

struct FriendMailConfig {
    smtp_host: String,
    smtp_port: u16,
    smtp_username: String,
    smtp_password: String,
    smtp_from: String,
    starttls: bool,
}

impl FriendMailConfig {
    fn from_env() -> Result<Self> {
        let smtp_host = env::var("SMTP_HOST").unwrap_or_default();
        let smtp_username = env::var("SMTP_USERNAME").unwrap_or_default();
        let smtp_password = env::var("SMTP_PASSWORD").unwrap_or_default();
        let smtp_from = env::var("SMTP_FROM").unwrap_or_default();

        if smtp_host.is_empty() || smtp_username.is_empty() || smtp_password.is_empty() || smtp_from.is_empty() {
            return Err(anyhow!("SMTP mail service is not configured"));
        }

        let smtp_port = env::var("SMTP_PORT").ok().and_then(|value| value.parse::<u16>().ok()).unwrap_or(587);
        let starttls = env::var("SMTP_STARTTLS").map(|value| value != "false" && value != "0").unwrap_or(true);

        Ok(Self { smtp_host, smtp_port, smtp_username, smtp_password, smtp_from, starttls })
    }
}

fn friend_request_mail_html(username: &str, requester_username: &str, accept_url: &str) -> String {
    let message = format!(
        "Bonjour <strong style=\"color:{gold};\">{username}</strong>,<br><br><strong style=\"color:{cyan};\">{requester}</strong> vous a envoyé une demande d'ami sur Voxicraft VR.<br><br>Vous pouvez accepter l'invitation directement avec le bouton ci-dessous. Si vous n'êtes pas connecté, vous serez redirigé vers la page de connexion puis l'invitation sera acceptée automatiquement.",
        gold = ACCENT_GOLD,
        cyan = ACCENT_CYAN,
        username = escape_html(username),
        requester = escape_html(requester_username),
    );
    let content = format!(
        "{hero}<tr><td style=\"padding:0 30px 30px;\"><table role=\"presentation\" width=\"100%\" cellspacing=\"0\" cellpadding=\"0\" style=\"background:rgba(0,0,0,.35);border:3px solid #5d4037;border-radius:10px;box-shadow:6px 6px 0 rgba(0,0,0,.38);\"><tr><td style=\"padding:24px 26px;text-align:center;\"><p style=\"margin:0 0 20px;color:#ffffff;font-size:14px;line-height:1.9;text-shadow:2px 2px 0 rgba(0,0,0,.35);\">{message}</p>{button}</td></tr></table></td></tr>",
        hero = hero_block("Nouvelle demande d'ami", "Un joueur souhaite vous ajouter à sa liste d'amis.", "INVITATION", ACCENT_CYAN),
        message = message,
        button = action_button("Accepter l'invitation", accept_url, ACCENT_CYAN),
    );
    mail_shell(&content, ACCENT_CYAN)
}

fn mail_shell(content: &str, accent: &str) -> String {
    format!(
        "<!doctype html><html lang=\"fr\"><head><meta charset=\"utf-8\"><meta name=\"viewport\" content=\"width=device-width,initial-scale=1\"></head><body style=\"margin:0;padding:0;background:#0f2409;color:#ffffff;font-family:'Courier New',Arial,sans-serif;\"><table role=\"presentation\" width=\"100%\" cellspacing=\"0\" cellpadding=\"0\" style=\"min-width:100%;background:linear-gradient(135deg,#2d5016 0%,#1a3a0f 50%,#0f2409 100%);padding:34px 12px;\"><tr><td align=\"center\"><table role=\"presentation\" width=\"680\" cellspacing=\"0\" cellpadding=\"0\" style=\"width:100%;max-width:680px;background:rgba(139,69,19,.94);border:4px solid #5d4037;border-radius:10px;box-shadow:8px 8px 0 rgba(0,0,0,.5);overflow:hidden;\"><tr><td style=\"height:12px;background:linear-gradient(90deg,#3e2723 0%,{accent} 50%,#3e2723 100%);font-size:0;line-height:0;\">&nbsp;</td></tr>{content}{footer}</table></td></tr></table></body></html>",
        accent = accent,
        content = content,
        footer = brand_footer(accent),
    )
}

fn hero_block(title: &str, description: &str, badge: &str, accent: &str) -> String {
    format!(
        "<tr><td style=\"padding:30px 30px 24px;text-align:center;background:rgba(62,39,35,.72);\"><div style=\"display:inline-block;background:{accent};color:#101820;border:3px solid #3e2723;border-bottom-width:5px;border-right-width:4px;border-radius:0;padding:8px 12px;font-size:11px;font-weight:800;letter-spacing:.08em;text-transform:uppercase;box-shadow:4px 4px 0 rgba(0,0,0,.45);\">{badge}</div><h1 style=\"margin:22px 0 12px;color:#ffd700;font-size:26px;line-height:1.3;text-shadow:4px 4px 0 rgba(0,0,0,.5);font-family:'Courier New',Arial,sans-serif;\">{title}</h1><p style=\"margin:0;color:#d7ccc8;font-size:14px;line-height:1.8;text-shadow:2px 2px 0 rgba(0,0,0,.35);\">{description}</p></td></tr>",
        accent = accent,
        badge = escape_html(badge),
        title = escape_html(title),
        description = escape_html(description),
    )
}

fn action_button(label: &str, href: &str, accent: &str) -> String {
    format!(
        "<a href=\"{href}\" style=\"display:inline-block;background:{accent};color:#101820;border:3px solid #3e2723;border-bottom-width:6px;border-right-width:5px;border-radius:0;padding:13px 18px;font-size:13px;font-weight:800;letter-spacing:.04em;text-decoration:none;text-transform:uppercase;box-shadow:5px 5px 0 rgba(0,0,0,.45);\">{label}</a>",
        href = escape_html(href),
        label = escape_html(label),
        accent = accent,
    )
}

fn brand_footer(accent: &str) -> String {
    format!(
        "<tr><td style=\"padding:0 30px 30px;\"><table role=\"presentation\" width=\"100%\" cellspacing=\"0\" cellpadding=\"0\" style=\"border-top:1px solid rgba(255,255,255,.14);\"><tr><td align=\"center\" style=\"padding:22px 0 12px;\"><table role=\"presentation\" cellspacing=\"0\" cellpadding=\"0\" style=\"margin:0 auto;\"><tr><td style=\"vertical-align:middle;padding-right:12px;\"><img src=\"{logo}\" alt=\"Voxicraft VR\" width=\"44\" height=\"44\" style=\"display:block;width:44px;height:44px;border:0;outline:none;text-decoration:none;\" /></td><td style=\"vertical-align:middle;text-align:left;\"><div style=\"font-size:20px;line-height:1.2;color:#ffd700;font-weight:800;letter-spacing:.06em;text-shadow:3px 3px 0 rgba(0,0,0,.5);\">Voxicraft VR</div><div style=\"margin-top:4px;color:#d7ccc8;font-size:11px;line-height:1.5;\">Auth centrale · serveurs auto-hébergés</div></td></tr></table></td></tr><tr><td align=\"center\" style=\"padding:0 0 6px;\"><a href=\"https://central.voxicraft.fr\" style=\"display:inline-block;margin:0 6px 8px;color:#ffd700;text-decoration:none;font-size:11px;line-height:1.5;\">Central</a><span style=\"color:#5d4037;font-size:11px;\">·</span><a href=\"https://central.voxicraft.fr/friends\" style=\"display:inline-block;margin:0 6px 8px;color:{accent};text-decoration:none;font-size:11px;line-height:1.5;\">Amis</a><span style=\"color:#5d4037;font-size:11px;\">·</span><a href=\"https://central.voxicraft.fr/support\" style=\"display:inline-block;margin:0 6px 8px;color:{accent};text-decoration:none;font-size:11px;line-height:1.5;\">Support</a></td></tr><tr><td align=\"center\" style=\"padding:0 0 2px;\"><p style=\"margin:0;color:#d7ccc8;font-size:10px;line-height:1.7;\">Vous recevez cet email suite à une action effectuée sur votre compte Voxicraft VR.</p></td></tr></table></td></tr>",
        logo = BRAND_LOGO_URL,
        accent = accent,
    )
}

fn escape_html(value: &str) -> String {
    value.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;").replace('"', "&quot;").replace('\'', "&#39;")
}
