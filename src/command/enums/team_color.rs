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
    fn from(team_color: TeamColor) -> TeamColorWithReset {
        match team_color {
            TeamColor::Black => TeamColorWithReset::Black,
            TeamColor::DarkBlue => TeamColorWithReset::DarkBlue,
            TeamColor::DarkGreen => TeamColorWithReset::DarkGreen,
            TeamColor::DarkAqua => TeamColorWithReset::DarkAqua,
            TeamColor::DarkRed => TeamColorWithReset::DarkRed,
            TeamColor::DarkPurple => TeamColorWithReset::DarkPurple,
            TeamColor::Gold => TeamColorWithReset::Gold,
            TeamColor::Gray => TeamColorWithReset::Gray,
            TeamColor::DarkGray => TeamColorWithReset::DarkGray,
            TeamColor::Blue => TeamColorWithReset::Blue,
            TeamColor::Green => TeamColorWithReset::Green,
            TeamColor::Aqua => TeamColorWithReset::Aqua,
            TeamColor::Red => TeamColorWithReset::Red,
            TeamColor::LightPurple => TeamColorWithReset::LightPurple,
            TeamColor::Yellow => TeamColorWithReset::Yellow,
            TeamColor::White => TeamColorWithReset::White,
        }
    }
}
