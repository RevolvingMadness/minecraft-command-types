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
            TeamColorWithReset::Black => Ok(TeamColor::Black),
            TeamColorWithReset::DarkBlue => Ok(TeamColor::DarkBlue),
            TeamColorWithReset::DarkGreen => Ok(TeamColor::DarkGreen),
            TeamColorWithReset::DarkAqua => Ok(TeamColor::DarkAqua),
            TeamColorWithReset::DarkRed => Ok(TeamColor::DarkRed),
            TeamColorWithReset::DarkPurple => Ok(TeamColor::DarkPurple),
            TeamColorWithReset::Gold => Ok(TeamColor::Gold),
            TeamColorWithReset::Gray => Ok(TeamColor::Gray),
            TeamColorWithReset::DarkGray => Ok(TeamColor::DarkGray),
            TeamColorWithReset::Blue => Ok(TeamColor::Blue),
            TeamColorWithReset::Green => Ok(TeamColor::Green),
            TeamColorWithReset::Aqua => Ok(TeamColor::Aqua),
            TeamColorWithReset::Red => Ok(TeamColor::Red),
            TeamColorWithReset::LightPurple => Ok(TeamColor::LightPurple),
            TeamColorWithReset::Yellow => Ok(TeamColor::Yellow),
            TeamColorWithReset::White => Ok(TeamColor::White),
            TeamColorWithReset::Reset => Err(()),
        }
    }
}
