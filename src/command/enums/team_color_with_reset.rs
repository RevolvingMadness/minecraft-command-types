use crate::command::enums::team_color::TeamColor;
use crate::create_enum;

create_enum!(
    TeamColorWithReset,
    [],
    Black,
    DarkBlue,
    DarkGreen,
    DarkAqua,
    DarkRed,
    DarkPurple,
    Gold,
    Gray,
    DarkGray,
    Blue,
    Green,
    Aqua,
    Red,
    LightPurple,
    Yellow,
    White,
    Reset
);

impl TryInto<TeamColor> for TeamColorWithReset {
    type Error = ();

    fn try_into(self) -> Result<TeamColor, Self::Error> {
        match self {
            Self::Black => Ok(TeamColor::Black),
            Self::DarkBlue => Ok(TeamColor::DarkBlue),
            Self::DarkGreen => Ok(TeamColor::DarkGreen),
            Self::DarkAqua => Ok(TeamColor::DarkAqua),
            Self::DarkRed => Ok(TeamColor::DarkRed),
            Self::DarkPurple => Ok(TeamColor::DarkPurple),
            Self::Gold => Ok(TeamColor::Gold),
            Self::Gray => Ok(TeamColor::Gray),
            Self::DarkGray => Ok(TeamColor::DarkGray),
            Self::Blue => Ok(TeamColor::Blue),
            Self::Green => Ok(TeamColor::Green),
            Self::Aqua => Ok(TeamColor::Aqua),
            Self::Red => Ok(TeamColor::Red),
            Self::LightPurple => Ok(TeamColor::LightPurple),
            Self::Yellow => Ok(TeamColor::Yellow),
            Self::White => Ok(TeamColor::White),
            Self::Reset => Err(()),
        }
    }
}
