use nix::unistd::geteuid;
use privdrop::PrivDrop;

pub(crate) fn privdrop() {
    if !geteuid().is_root() {
        return;
    }
    PrivDrop::default()
        .chroot("/var/empty")
        .user("nobody")
        .apply()
        .unwrap_or_else(|e| eprintln!("Failed to drop privileges: {}", e));
}
