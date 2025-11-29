use crate::resource_location::ResourceLocation;
use minecraft_command_types_proc_macros::HasMacro;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Eq, PartialEq, Hash, HasMacro)]
pub enum RunfailedTestCommand {
    NumberOfTimes(Option<i32>, Option<bool>, Option<i32>, Option<i32>),
    OnlyRequiredTest(Option<bool>, Option<i32>),
}

impl Display for RunfailedTestCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            RunfailedTestCommand::NumberOfTimes(
                number_of_times,
                until_failed,
                rotation_steps,
                tests_per_row,
            ) => {
                if let Some(number_of_times) = number_of_times {
                    write!(f, " {}", number_of_times)?;

                    if let Some(until_failed) = until_failed {
                        write!(f, " {}", until_failed)?;

                        if let Some(rotation_steps) = rotation_steps {
                            write!(f, " {}", rotation_steps)?;

                            if let Some(tests_per_row) = tests_per_row {
                                write!(f, " {}", tests_per_row)?;
                            }
                        }
                    }
                }

                Ok(())
            }
            RunfailedTestCommand::OnlyRequiredTest(only_required_tests, number_of_times) => {
                if let Some(only_required_tests) = only_required_tests {
                    write!(f, " {}", only_required_tests)?;

                    if let Some(number_of_times) = number_of_times {
                        write!(f, " {}", number_of_times)?;
                    }
                }

                Ok(())
            }
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, HasMacro)]
pub enum TestCommand {
    ClearAll(Option<i32>),
    ClearThat,
    ClearThese,
    Create(ResourceLocation, Option<i32>, Option<(i32, i32)>),
    Locate(ResourceLocation),
    Pos(Option<String>),
    ResetClosest,
    ResetThat,
    ResetThese,
    Run(
        ResourceLocation,
        Option<i32>,
        Option<bool>,
        Option<i32>,
        Option<i32>,
    ),
    RunClosest(Option<i32>, Option<bool>),
    RunThat(Option<i32>, Option<bool>),
    RunThese(Option<i32>, Option<bool>),
    RunMultiple(ResourceLocation, Option<i32>),
    RunFailed(RunfailedTestCommand),
    Stop,
    Verify(ResourceLocation),
    Export(ResourceLocation),
    ExportClosest,
    ExportThat,
    ExportThese,
}

impl Display for TestCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TestCommand::ClearAll(radius) => {
                "clearall".fmt(f)?;

                if let Some(radius) = radius {
                    write!(f, " {}", radius)?;
                }

                Ok(())
            }
            TestCommand::ClearThat => "clearthat".fmt(f),
            TestCommand::ClearThese => "clearthese".fmt(f),
            TestCommand::Create(location, width, height_depth) => {
                write!(f, "create {}", location)?;

                if let Some(width) = width {
                    write!(f, " {}", width)?;

                    if let Some((height, depth)) = height_depth {
                        write!(f, " {} {}", height, depth)?;
                    }
                }

                Ok(())
            }
            TestCommand::Locate(location) => write!(f, "locate {}", location),
            TestCommand::Pos(variable) => {
                "pos".fmt(f)?;

                if let Some(variable) = variable {
                    write!(f, " {}", variable)?;
                }

                Ok(())
            }
            TestCommand::ResetClosest => "resetclosest".fmt(f),
            TestCommand::ResetThat => "resetthat".fmt(f),
            TestCommand::ResetThese => "resetthese".fmt(f),
            TestCommand::Run(
                location,
                number_of_times,
                until_failed,
                rotation_step,
                tests_per_row,
            ) => {
                write!(f, "run {}", location)?;

                if let Some(number_of_times) = number_of_times {
                    write!(f, " {}", number_of_times)?;

                    if let Some(until_failed) = until_failed {
                        write!(f, " {}", until_failed)?;

                        if let Some(rotation_steps) = rotation_step {
                            write!(f, " {}", rotation_steps)?;

                            if let Some(tests_per_row) = tests_per_row {
                                write!(f, " {}", tests_per_row)?;
                            }
                        }
                    }
                }

                Ok(())
            }
            TestCommand::RunClosest(number_of_times, until_failed) => {
                "runclosest".fmt(f)?;

                if let Some(number_of_times) = number_of_times {
                    write!(f, " {}", number_of_times)?;

                    if let Some(until_failed) = until_failed {
                        write!(f, " {}", until_failed)?;
                    }
                }

                Ok(())
            }
            TestCommand::RunThat(number_of_times, until_failed) => {
                "runthat".fmt(f)?;

                if let Some(number_of_times) = number_of_times {
                    write!(f, " {}", number_of_times)?;

                    if let Some(until_failed) = until_failed {
                        write!(f, " {}", until_failed)?;
                    }
                }

                Ok(())
            }
            TestCommand::RunThese(number_of_times, until_failed) => {
                "runthese".fmt(f)?;

                if let Some(number_of_times) = number_of_times {
                    write!(f, " {}", number_of_times)?;

                    if let Some(until_failed) = until_failed {
                        write!(f, " {}", until_failed)?;
                    }
                }

                Ok(())
            }
            TestCommand::RunMultiple(location, amount) => {
                write!(f, "runmultiple {}", location)?;

                if let Some(amount) = amount {
                    write!(f, " {}", amount)?;
                }

                Ok(())
            }
            TestCommand::RunFailed(command) => write!(f, "runfailed {}", command),
            TestCommand::Stop => "stop".fmt(f),
            TestCommand::Verify(location) => write!(f, "verify {}", location),
            TestCommand::Export(location) => write!(f, "export {}", location),
            TestCommand::ExportClosest => "exportclosest".fmt(f),
            TestCommand::ExportThat => "exportthat".fmt(f),
            TestCommand::ExportThese => "exportthese".fmt(f),
        }
    }
}
