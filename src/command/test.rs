use crate::resource_location::ResourceLocation;
use minecraft_command_types_derive::HasMacro;
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
                f.write_str("clearall")?;

                if let Some(radius) = radius {
                    write!(f, " {}", radius)?;
                }

                Ok(())
            }
            TestCommand::ClearThat => f.write_str("clearthat"),
            TestCommand::ClearThese => f.write_str("clearthese"),
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
                f.write_str("pos")?;

                if let Some(variable) = variable {
                    write!(f, " {}", variable)?;
                }

                Ok(())
            }
            TestCommand::ResetClosest => f.write_str("resetclosest"),
            TestCommand::ResetThat => f.write_str("resetthat"),
            TestCommand::ResetThese => f.write_str("resetthese"),
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
                f.write_str("runclosest")?;

                if let Some(number_of_times) = number_of_times {
                    write!(f, " {}", number_of_times)?;

                    if let Some(until_failed) = until_failed {
                        write!(f, " {}", until_failed)?;
                    }
                }

                Ok(())
            }
            TestCommand::RunThat(number_of_times, until_failed) => {
                f.write_str("runthat")?;

                if let Some(number_of_times) = number_of_times {
                    write!(f, " {}", number_of_times)?;

                    if let Some(until_failed) = until_failed {
                        write!(f, " {}", until_failed)?;
                    }
                }

                Ok(())
            }
            TestCommand::RunThese(number_of_times, until_failed) => {
                f.write_str("runthese")?;

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
            TestCommand::Stop => f.write_str("stop"),
            TestCommand::Verify(location) => write!(f, "verify {}", location),
            TestCommand::Export(location) => write!(f, "export {}", location),
            TestCommand::ExportClosest => f.write_str("exportclosest"),
            TestCommand::ExportThat => f.write_str("exportthat"),
            TestCommand::ExportThese => f.write_str("exportthese"),
        }
    }
}
