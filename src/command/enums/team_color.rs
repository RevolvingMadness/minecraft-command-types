use crate::command::enums::team_color_with_reset::TeamColorWithReset;
use crate::create_enum;

create_enum!(
    TeamColor,
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
    White
);

impl From<TeamColor> for TeamColorWithReset {
    fn from(team_color: TeamColor) -> Self {
        match team_color {
            TeamColor::Black => Self::Black,
            TeamColor::DarkBlue => Self::DarkBlue,
            TeamColor::DarkGreen => Self::DarkGreen,
            TeamColor::DarkAqua => Self::DarkAqua,
            TeamColor::DarkRed => Self::DarkRed,
            TeamColor::DarkPurple => Self::DarkPurple,
            TeamColor::Gold => Self::Gold,
            TeamColor::Gray => Self::Gray,
            TeamColor::DarkGray => Self::DarkGray,
            TeamColor::Blue => Self::Blue,
            TeamColor::Green => Self::Green,
            TeamColor::Aqua => Self::Aqua,
            TeamColor::Red => Self::Red,
            TeamColor::LightPurple => Self::LightPurple,
            TeamColor::Yellow => Self::Yellow,
            TeamColor::White => Self::White,
        }
    }
}
