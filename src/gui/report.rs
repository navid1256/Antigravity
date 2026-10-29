//! «Save report»: everything needed to say why Antigravity is or is not
//! answering, in one paste.
//!
//! «I installed the unlocker, but it does not seem to help» plus a screenshot of a green card
//! was undiagnosable (G50): the one fact that mattered - the client was not
//! going through us at all - was nowhere on screen. This puts every fact the
//! window and the relay have into plain text a user can send to the group.
//!
//! Nothing secret goes in: paths are masked the way the window shows them, the
//! built-in exits are named only as «built-in exit» (I46), and the relay's
//! log - which is included - never names an address of ours in the first place.

use std::fmt::Write as _;

use crate::gate::{Report, View};
use crate::ops::{State, Status};
use crate::utils::mask_path;

/// How much of the relay's own log goes in: enough to cover the last few
/// minutes of a busy session, short enough to paste into a chat.
const LOG_LINES: usize = 60;

pub fn build(status: Option<&Status>, view: &View) -> String {
    let mut out = String::new();
    let now = crate::utils::local_clock().map_or_else(String::new, |c| c.hms());
    let _ = writeln!(
        out,
        "Antigravity Unlocker v{} — report {}",
        crate::update::current_version(),
        now
    );

    match status {
        Some(s) => status_part(&mut out, s),
        None => {
            let _ = writeln!(out, "System status has not been read yet.");
        }
    }

    let _ = writeln!(out, "\n— Antigravity (its own logs)");
    match view.answered {
        Some(a) => {
            let _ = writeln!(
                out,
                "Last model response: {} (in 12 h: {})",
                super::status::ago_text(a.ago),
                a.count
            );
        }
        None => {
            let _ = writeln!(out, "No model responses in the logs over the last 12 h.");
        }
    }
    match view.refused_long {
        Some(r) => {
            let _ = writeln!(
                out,
                "Last error 400: {} (entries in 10 min: {})",
                super::status::ago_text(r.ago),
                view.seen.map_or(0, |s| s.count)
            );
        }
        None => {
            let _ = writeln!(out, "No error 400 in the logs over the last 12 h.");
        }
    }
    if let Some(o) = &view.overloaded {
        let model = if o.model.is_empty() {
            String::new()
        } else {
            format!(", model {}", o.model)
        };
        let _ = writeln!(
            out,
            "Error 503 (Google has no available capacity{model}): {}, entries in 10 min: {}",
            super::status::ago_text(o.ago),
            o.count
        );
    }
    if let Some(e) = &view.server_errors {
        let _ = writeln!(
            out,
            "Google server errors (503/429/500) in 12 h: {}, latest {}",
            e.count,
            super::status::ago_text(e.ago)
        );
    }
    if let Some(u) = &view.unauthorized {
        let _ = writeln!(
            out,
            "Error 401 (credentials rejected): {}, entries in 10 min: {}",
            super::status::ago_text(u.ago),
            u.count
        );
    }
    // When, and on which host - never the link: it carries a one-time token,
    // and the report is made to be sent to someone else.
    if let Some(v) = &view.verify {
        let host = v
            .url
            .strip_prefix("https://")
            .and_then(|r| r.split(['/', '?', '#']).next())
            .unwrap_or("");
        let _ = writeln!(
            out,
            "Google requested account verification: {} ({host})",
            super::status::ago_text(v.ago)
        );
    }

    // Fresh from the file, not the window's copy: the route table's ages move
    // every pass and the watcher does not wake the window for them.
    let _ = writeln!(out, "\n— Bypass service (its record)");
    match crate::gate::read() {
        Some(r) => relay_part(&mut out, &r),
        None => {
            let _ = writeln!(out, "No record — the service is not running or is outdated.");
        }
    }

    let _ = writeln!(out, "\n— Service log (latest entries)");
    let log = std::fs::read_to_string(crate::dns_forwarder::log_path()).unwrap_or_default();
    let lines: Vec<&str> = log.lines().collect();
    let from = lines.len().saturating_sub(LOG_LINES);
    let own = crate::upstream::configured().map(|u| u.display());
    for line in &lines[from..] {
        let _ = writeln!(out, "{}", mask_addresses(line, own.as_deref()));
    }
    if lines.is_empty() {
        let _ = writeln!(out, "(empty)");
    }
    out
}

fn onoff(s: &State) -> String {
    match s {
        State::On => "on".to_string(),
        State::Off => "off".to_string(),
        State::Partial(n) => format!("partial ({n})"),
        State::OffNote(n) => format!("off ({n})"),
        State::Blocked(n) => format!("unavailable ({n})"),
    }
}

fn status_part(out: &mut String, s: &Status) {
    let _ = writeln!(
        out,
        "Administrator rights: {}",
        if s.admin { "yes" } else { "no" }
    );
    let _ = writeln!(out, "\n— Installations");
    for row in &s.installs {
        let path = row
            .path
            .as_ref()
            .map(|p| mask_path(&p.display().to_string()))
            .unwrap_or_else(|| "not found".to_string());
        let patched = match row.patched {
            Some(true) => "patched",
            Some(false) => "NOT patched",
            None => "not checked",
        };
        let _ = writeln!(out, "{}: {} — {}", row.label, path, patched);
    }
    let _ = writeln!(out, "\n— Switches");
    let _ = writeln!(out, "Account sign-in unlock: {}", onoff(&s.client_patch));
    let _ = writeln!(out, "Auto-patch: {}", onoff(&s.watchdog));
    let _ = writeln!(out, "Bypass via DNS: {}", onoff(&s.dns));
    let _ = writeln!(out, "Local proxy: {}", onoff(&s.local_proxy));
    let _ = writeln!(out, "Built-in exits: {}", onoff(&s.builtin_exits));
    let key_present = if crate::settings::group_key().is_empty() { "absent" } else { "present" };
    let _ = writeln!(
        out,
        "Group proxy: {}, key: {}, PC code: {}",
        onoff(&s.group_proxy),
        key_present,
        crate::hwid::pc_code()
    );
    let _ = writeln!(out, "Custom proxy: {}", onoff(&s.own_proxy));
    let _ = writeln!(
        out,
        "Service: {}{}; rules for DNS: {}",
        if s.relay_running {
            "running"
        } else {
            "NOT running"
        },
        if s.relay_outdated {
            ", outdated"
        } else {
            ""
        },
        if s.rules { "present" } else { "ABSENT" }
    );
    if let Some(vpn) = s.vpn {
        let _ = writeln!(out, "VPN (based on sockets of Antigravity): {:?}", vpn);
    }
    if let Some(eg) = s.relay_egress {
        let _ = writeln!(out, "Service sockets: {:?}", eg);
    }
}

fn relay_part(out: &mut String, r: &Report) {
    let _ = writeln!(
        out,
        "Service version: {} (this program expects {}), record {} s ago",
        r.version,
        crate::dns_forwarder::RELAY_VERSION,
        r.age().as_secs()
    );
    let _ = writeln!(
        out,
        "Gate hosts use local addresses: {}",
        if r.loopback { "yes" } else { "no" }
    );
    for b in &r.blockers {
        let _ = writeln!(out, "Blocking the bypass: {}", super::status::blocker_line(b));
    }
    if crate::proxy::port() != crate::proxy::DEFAULT_PORT {
        let _ = writeln!(
            out,
            "Local proxy port: {} (moved from the default {})",
            crate::proxy::port(),
            crate::proxy::DEFAULT_PORT
        );
    }
    if r.started_at != 0 {
        let _ = writeln!(
            out,
            "Last internet response received by the service: {}",
            if r.reached_at == 0 {
                "never since startup".to_string()
            } else {
                format!(
                    "{} s ago",
                    crate::gate::now_unix().saturating_sub(r.reached_at)
                )
            }
        );
    }
    let _ = writeln!(
        out,
        "VPN holds the default route: {}{}",
        if r.tunnel { "yes" } else { "no" },
        if r.vpn_exit.is_empty() {
            String::new()
        } else {
            format!(", exit: {}", r.vpn_exit)
        }
    );
    // The table's own first usable row, not `r.route`: that one is the
    // hysteresis memory (`routes::leader`) and can still name a route the order
    // has moved past, which put a headline in the report contradicting the
    // table printed right under it (field report, 2026-09-20).
    let first = r
        .routes
        .iter()
        .find(|row| row.usable)
        .map(|row| row.label.as_str())
        .unwrap_or(r.route.as_str());
    let _ = writeln!(out, "Current first route: {}", non_empty(first));
    if let Some(ok) = &r.last_ok {
        let _ = writeln!(
            out,
            "Last model response seen by the service: {} s ago, route: {}",
            crate::gate::now_unix().saturating_sub(ok.at),
            non_empty(&ok.route)
        );
    }
    if let Some(e) = &r.last_400 {
        let _ = writeln!(
            out,
            "Last error 400 seen by the service: {} s ago, entries: {}, route: {}{}",
            crate::gate::now_unix().saturating_sub(e.at),
            e.count,
            non_empty(&e.route),
            if e.bypassed {
                " (outside the bypass)"
            } else {
                ""
            }
        );
        if !e.acted.is_empty() {
            let _ = writeln!(out, "Action taken: {}", e.acted);
        }
    }
    let _ = writeln!(out, "Routes (in selection order):");
    for row in &r.routes {
        let mut parts: Vec<String> = Vec::new();
        // One state word, not two: «available … deferred for another 10 min» is what a
        // benched-but-still-offered route used to print, and it reads as a
        // contradiction to the person pasting it.
        parts.push(match (row.usable, row.bench_left) {
            (false, _) => "not currently used".to_string(),
            (true, Some(b)) => format!("deferred for another {} min, but queued", b / 60 + 1),
            (true, None) => "available".to_string(),
        });
        if let Some(ms) = row.latency_ms {
            parts.push(format!("{ms} ms"));
        }
        if row.proven {
            parts.push("verified by a model response".to_string());
        }
        if let Some(a) = row.ok_ago {
            parts.push(format!("response {a} s ago"));
        }
        if let Some(a) = row.refused_ago {
            parts.push(format!("error 400 {a} s ago"));
        }
        // The tally, not just the two timestamps: «response 34 s ago, error 400
        // 14 s ago» is the same line for a route that answers nine times out
        // of ten and for one that answers once in fifty, and three field
        // reports (2026-09-21) turned on telling those apart.
        if row.answers > 0 || row.refusals > 0 {
            parts.push(format!(
                "responses {}, refusals {}",
                row.answers, row.refusals
            ));
        }
        if let Some(b) = row.bench_left.filter(|_| !row.usable) {
            parts.push(format!("deferred for another {} min", b / 60 + 1));
        }
        if row.open > 0 {
            parts.push(format!("open connections: {}", row.open));
        }
        let _ = writeln!(out, "  {} — {}", row.label, parts.join(", "));
    }
}

/// One line of the relay's log with the user's own proxy, and any route's exit
/// address, taken out.
///
/// Since `2.15.0_1` the relay no longer writes that address at all, but a log
/// written by an older build is still in the file after the upgrade, and it did:
/// `custom proxy user:***@host:port: <why>` - their login and their server, on
/// the way into a public chat. Matching only the proxy configured *now* missed
/// every line from one they had before or had since removed, so this goes by
/// shape: any `…:***@…` credential token, and the address right after
/// «custom proxy», whatever it is. The configured value is masked too, for any
/// line that names it some other way.
fn mask_addresses(line: &str, configured: Option<&str>) -> String {
    const MASK: &str = "<your proxy>";
    // Older Russian builds share this log file with the English build.
    const LABELS: [&str; 2] = [
        "custom proxy ",
        "\u{441}\u{432}\u{43e}\u{439} \u{43f}\u{440}\u{43e}\u{43a}\u{441}\u{438} ",
    ];
    let mut out: String = line
        .split(' ')
        .map(|token| {
            if token.contains(":***@") {
                // Keep the colon that separated the address from the reason.
                if token.ends_with(':') {
                    format!("{MASK}:")
                } else {
                    MASK.to_string()
                }
            } else {
                token.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join(" ");
    // `custom proxy host:port: why` from before the credential form existed: an
    // address token is the one that still has a colon inside it (`host:port`).
    for label in LABELS {
        let Some(at) = out.find(label) else { continue };
        let rest = &out[at + label.len()..];
        let token = rest.split(' ').next().unwrap_or("");
        let bare = token.strip_suffix(':').unwrap_or(token);
        if bare.contains(':') && bare != MASK {
            let masked = if token.ends_with(':') {
                format!("{MASK}:")
            } else {
                MASK.to_string()
            };
            out = format!(
                "{}{}{}",
                &out[..at + label.len()],
                masked,
                &rest[token.len()..]
            );
        }
    }
    // Older builds also named a route's exit address when it moved: for the
    // user's own proxy that is their server, for a built-in exit usually the
    // exit itself (I46). The country after it stays - it is what the line says.
    for phrase in [
        "exits via ",
        "changed exit to ",
        "\u{432}\u{44b}\u{445}\u{43e}\u{434}\u{438}\u{442} \u{447}\u{435}\u{440}\u{435}\u{437} ",
        "\u{441}\u{43c}\u{435}\u{43d}\u{438}\u{43b} \u{432}\u{44b}\u{445}\u{43e}\u{434} \u{43d}\u{430} ",
    ] {
        if let Some(at) = out.find(phrase) {
            let start = at + phrase.len();
            let len = out[start..].find(' ').unwrap_or(out.len() - start);
            out.replace_range(start..start + len, "<address>");
        }
    }
    match configured {
        Some(own) if !own.is_empty() => out.replace(own, MASK),
        _ => out,
    }
}

fn non_empty(s: &str) -> &str {
    if s.is_empty() {
        "—"
    } else {
        s
    }
}

#[cfg(test)]
mod tests {
    /// What «Save report» writes to the file on this machine, minus the
    /// system scan the window adds. Reads, never asserts content.
    ///
    ///     cargo test prints_the_report -- --ignored --nocapture
    #[test]
    #[ignore = "reads the live relay record and log; run with --ignored"]
    fn prints_the_report() {
        let text = super::build(None, &crate::gate::View::default());
        println!("{text}");
        assert!(text.contains("report"));
    }

    /// The line that says whether a route half-works. Two timestamps read the
    /// same for a route answering nine times in ten and one answering once in
    /// fifty, and three field reports (2026-09-21) turned on telling them
    /// apart.
    #[test]
    fn a_route_row_says_how_often_it_answered_and_how_often_it_was_refused() {
        let relay = crate::gate::Report {
            at: crate::gate::now_unix(),
            routes: vec![crate::routes::Row {
                label: "direct".into(),
                usable: true,
                latency_ms: Some(479),
                proven: true,
                ok_ago: Some(34),
                refused_ago: Some(14),
                answers: 21,
                refusals: 30,
                open: 2,
                ..Default::default()
            }],
            ..Default::default()
        };
        // `relay_part`, not `build`: the report reads the live record off the
        // disk on purpose (the table's ages move every pass and nothing wakes
        // the window for them), so a synthesised one only reaches this half.
        let mut text = String::new();
        super::relay_part(&mut text, &relay);
        assert!(
            text.contains("responses 21, refusals 30"),
            "the tally is missing:\n{text}"
        );
        // …and the refusal is still shown, which is the half the split of
        // `bad_at`/`refused_at` must not have cost (G76).
        assert!(text.contains("error 400 14 s ago"), "{text}");
    }

    use super::mask_addresses as mask;

    /// A line an older relay wrote about a proxy the user has since changed
    /// keeps nothing of it: not the login, not the server.
    #[test]
    fn an_old_credential_line_is_masked_whatever_is_configured_now() {
        let line = "12:00:01 proxy        custom proxy ivan:***@my.server.example:3128: unavailable: timed out";
        for now in [None, Some("other:***@elsewhere.example:8080")] {
            let got = mask(line, now);
            assert!(
                !got.contains("ivan") && !got.contains("my.server.example"),
                "{got}"
            );
            assert!(
                got.contains("custom proxy <your proxy>: unavailable: timed out"),
                "{got}"
            );
        }
    }

    /// The same from before credentials were masked in the display: a bare
    /// `host:port` after the label.
    #[test]
    fn an_old_bare_address_line_is_masked() {
        let got = mask(
            "12:00:01 proxy        custom proxy 10.0.0.5:1080: proxy closed the connection",
            None,
        );
        assert_eq!(
            got,
            "12:00:01 proxy        custom proxy <your proxy>: proxy closed the connection"
        );
    }

    /// Lines about the route by name stay readable: nothing there is an address.
    #[test]
    fn route_lines_are_left_alone() {
        for line in [
            "12:00:01 proxy        custom proxy -> daily-cloudcode-pa.googleapis.com",
            "12:00:01 proxy        custom proxy: unavailable: timed out",
            "12:00:01 proxy        gate-host route: custom proxy (378 ms), previously direct (325 ms)",
            "12:00:01 proxy        built-in exit #1 is not responding: unavailable: timed out",
        ] {
            assert_eq!(mask(line, None), line);
        }
    }

    /// An older relay's exit-change lines keep the country, lose the address.
    #[test]
    fn an_old_exit_address_is_masked_and_the_country_kept() {
        assert_eq!(
            mask("12:00:01 proxy        custom proxy exits via 203.0.113.9 (RU) — this is a blocked region", None),
            "12:00:01 proxy        custom proxy exits via <address> (RU) — this is a blocked region"
        );
        assert_eq!(
            mask("12:00:01 proxy        built-in exit #1 changed exit to 203.0.113.9 (NL) — using it again", None),
            "12:00:01 proxy        built-in exit #1 changed exit to <address> (NL) — using it again"
        );
    }

    /// The proxy configured now is masked wherever it appears.
    #[test]
    fn russian_logs_from_before_the_update_still_hide_proxy_addresses() {
        let label = "\u{441}\u{432}\u{43e}\u{439} \u{43f}\u{440}\u{43e}\u{43a}\u{441}\u{438}";
        let got = mask(&format!("12:00:01 proxy {label} 10.0.0.5:1080: unavailable"), None);
        assert!(!got.contains("10.0.0.5"), "{got}");
        assert!(got.contains("<your proxy>"), "{got}");

        for phrase in [
            "\u{432}\u{44b}\u{445}\u{43e}\u{434}\u{438}\u{442} \u{447}\u{435}\u{440}\u{435}\u{437}",
            "\u{441}\u{43c}\u{435}\u{43d}\u{438}\u{43b} \u{432}\u{44b}\u{445}\u{43e}\u{434} \u{43d}\u{430}",
        ] {
            let line = format!("12:00:01 proxy {label} {phrase} 203.0.113.9 (NL)");
            let got = mask(&line, None);
            assert!(!got.contains("203.0.113.9"), "{got}");
            assert!(got.contains("<address> (NL)"), "{got}");
        }
    }

    #[test]
    fn the_configured_proxy_is_masked_anywhere() {
        let got = mask("something about 127.0.0.1:1371 and more", Some("127.0.0.1:1371"));
        assert_eq!(got, "something about <your proxy> and more");
    }
}
