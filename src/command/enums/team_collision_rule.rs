use crate::create_enum;

create_enum!(
    "camelCase",
    TeamCollisionRule,
    [],
    Always,
    Never,
    PushOtherTeams,
    PushOwnTeam
);
