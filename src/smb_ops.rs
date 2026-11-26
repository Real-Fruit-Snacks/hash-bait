//! Native SMB operations via the pure-Rust `smb` crate — no smbclient,
//! no Python, no runtime dependencies. Upload, delete and share enumeration
//! over SMB2/3 with signing and encryption support.

use smb::{
    Client, ClientConfig, CreateOptions, FileAccessMask, FileAttributes, FileCreateArgs,
    FileDispositionInformation, Resource, UncPath, WriteAt,
};

/// Credentials for a session. "Anonymous" is a guest attempt: the NTLM
/// implementation cannot express a true null session (empty identity), so
/// the portable equivalent is user `guest` with an empty password — on hosts
/// that map unknown or guest users to a guest account (Samba `map to guest`,
/// typical NAS boxes) this is the anonymous path.
pub struct Creds {
    pub username: String,
    pub password: String,
    pub anonymous: bool,
}

impl Creds {
    /// Build from a parsed (domain, user) pair and resolved password.
    pub fn new(domain: Option<&str>, user: &str, password: String) -> Creds {
        let username = match domain {
            Some(d) if !d.is_empty() => format!("{d}\\{user}"),
            _ => user.to_string(),
        };
        Creds {
            username,
            password,
            anonymous: false,
        }
    }

    pub fn anonymous() -> Creds {
        Creds {
            username: "guest".to_string(),
            password: String::new(),
            anonymous: true,
        }
    }
}

fn client_for(creds: &Creds) -> Client {
    let mut config = ClientConfig::default();
    if creds.anonymous {
        // Null/guest sessions cannot sign; the crate refuses them otherwise.
        config.connection.allow_unsigned_guest_access = true;
    }
    Client::new(config)
}

/// Connect a tree to //target/share. Returns the client holding the session.
pub fn connect(target: &str, share: &str, creds: &Creds) -> Result<Client, String> {
    let client = client_for(creds);
    let unc = UncPath::new(target)
        .and_then(|u| u.with_share(share))
        .map_err(|e| format!("invalid target '{target}' or share '{share}': {e}"))?;
    client
        .share_connect(&unc, &creds.username, creds.password.clone())
        .map_err(|e| {
            if creds.anonymous {
                format!("guest session to //{target}/{share} failed (host refuses guest access — try -u): {e}")
            } else {
                format!("authentication to //{target}/{share} failed: {e}")
            }
        })?;
    Ok(client)
}

fn remote_path(dir: Option<&str>, name: &str) -> String {
    match dir {
        Some(d) if !d.is_empty() => format!("{}\\{}", d.trim_end_matches('\\'), name),
        _ => name.to_string(),
    }
}

/// Write payloads to the connected share. Each file is created (overwritten
/// if present), written in one shot, and closed.
pub fn put_files(
    client: &Client,
    target: &str,
    share: &str,
    dir: Option<&str>,
    files: &[(String, String)],
) -> Result<(), String> {
    let base = UncPath::new(target)
        .and_then(|u| u.with_share(share))
        .map_err(|e| format!("invalid target: {e}"))?;
    for (name, body) in files {
        let unc = base.clone().with_path(&remote_path(dir, name));
        let resource = client
            .create_file(
                &unc,
                &FileCreateArgs::make_overwrite(FileAttributes::new(), CreateOptions::new()),
            )
            .map_err(|e| format!("cannot create {name} on //{target}/{share}: {e}"))?;
        let file = match resource {
            Resource::File(f) => f,
            _ => return Err(format!("{name} on //{target}/{share} is not a file")),
        };
        file.write_at(body.as_bytes(), 0)
            .map_err(|e| format!("cannot write {name} on //{target}/{share}: {e}"))?;
    }
    Ok(())
}

/// Delete payloads from the connected share: open with delete access, mark
/// delete-pending, close. Missing files are reported but do not stop the rest.
pub fn del_files(
    client: &Client,
    target: &str,
    share: &str,
    dir: Option<&str>,
    names: &[String],
) -> Result<Vec<String>, String> {
    let base = UncPath::new(target)
        .and_then(|u| u.with_share(share))
        .map_err(|e| format!("invalid target: {e}"))?;
    let mut missing = Vec::new();
    for name in names {
        let unc = base.clone().with_path(&remote_path(dir, name));
        let resource = client.create_file(
            &unc,
            &FileCreateArgs::make_open_existing(FileAccessMask::new().with_delete(true)),
        );
        let file = match resource {
            Ok(Resource::File(f)) => f,
            Ok(_) => {
                missing.push(name.clone());
                continue;
            }
            Err(_) => {
                missing.push(name.clone());
                continue;
            }
        };
        file.set_info(FileDispositionInformation::default())
            .map_err(|e| format!("cannot delete {name} on //{target}/{share}: {e}"))?;
    }
    Ok(missing)
}

/// Resolve an NDR string pointer to a plain String (empty when unresolved).
fn ndr_to_string(
    ptr: &smb_rpc::ndr64::ptr::NdrPtr<smb_rpc::ndr64::string::NdrString<u16>>,
) -> String {
    use smb_rpc::ndr64::ptr::NdrPtr;
    match ptr {
        NdrPtr::Resolved(Some(aligned)) => aligned.to_string(),
        _ => String::new(),
    }
}

/// Enumerate shares via IPC$ + srvsvc. Returns (name, remark) pairs.
pub fn list_shares(target: &str, creds: &Creds) -> Result<Vec<(String, String)>, String> {
    let client = client_for(creds);
    client
        .ipc_connect(target, &creds.username, creds.password.clone())
        .map_err(|e| format!("cannot connect IPC$ on {target}: {e}"))?;
    let shares = client
        .list_shares(target)
        .map_err(|e| format!("share enumeration on {target} failed: {e}"))?;
    Ok(shares
        .iter()
        .map(|s| (ndr_to_string(&s.netname), ndr_to_string(&s.remark)))
        .collect())
}
