use crate::create_enum;

create_enum!(
    "camelCase",
    TeamVisibility,
    [],
    Always,
    HideForOtherTeams,
    HideForOwnTeam,
    Never
);
