#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum PermissionLevel {
    All,
    Moderator,
    Gamemaster,
    Admin,
    Owner,
}

impl TryFrom<u8> for PermissionLevel {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::All),
            1 => Ok(Self::Moderator),
            2 => Ok(Self::Gamemaster),
            3 => Ok(Self::Admin),
            4 => Ok(Self::Owner),
            _ => Err(()),
        }
    }
}
