#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum PermissionLevel {
    /// * No permission.
    All,
    /// * The player can bypass spawn protection.
    Moderator,
    /// * More commands are available.
    /// * The player can use command blocks.
    /// * The player can copy the server-side NBT data of an entity or a block entity when pressing the F3+I debug hotkey, and copy the client-side NBT data when pressing ⇧ Shift+F3+I.
    /// * The player can use F3+F4 (game mode switcher) and F3+N debug hotkey (toggle between Spectator and the previous game mode).
    /// * The player can change or lock difficulty in Options screen. Note that the player in a singleplayer world or the owner of a LAN world can change or lock difficulty without a permission level of 2.
    /// * With "Operator Items Tab" option turned on, the player can find operator items and an "Operator Utilities" tab in the creative inventory.
    /// * Target selectors can be used in commands like /tell and raw JSON texts.
    Gamemaster,
    /// * Commands related to multiplayer management are available.
    Admin,
    /// * All commands are available, including commands related to server management.
    Owner,
}

impl TryFrom<u8> for PermissionLevel {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(PermissionLevel::All),
            1 => Ok(PermissionLevel::Moderator),
            2 => Ok(PermissionLevel::Gamemaster),
            3 => Ok(PermissionLevel::Admin),
            4 => Ok(PermissionLevel::Owner),
            _ => Err(()),
        }
    }
}
