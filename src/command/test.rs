use crate::{command::Command, option_write_chain, resource_location::ResourceLocation};
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RunfailedTestCommand {
    NumberOfTimes(Option<i32>, Option<bool>, Option<i32>, Option<i32>),
    OnlyRequiredTest(Option<bool>, Option<i32>),
}

impl Display for RunfailedTestCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::NumberOfTimes(number_of_times, until_failed, rotation_steps, tests_per_row) => {
                option_write_chain!(
                    f,
                    number_of_times,
                    until_failed,
                    rotation_steps,
                    tests_per_row
                );

                Ok(())
            }
            Self::OnlyRequiredTest(only_required_tests, number_of_times) => {
                option_write_chain!(f, only_required_tests, number_of_times);

                Ok(())
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::ClearAll(radius) => {
                f.write_str("clearall")?;

                option_write_chain!(f, radius);

                Ok(())
            }
            Self::ClearThat => f.write_str("clearthat"),
            Self::ClearThese => f.write_str("clearthese"),
            Self::Create(location, width, height_depth) => {
                write!(f, "create {}", location)?;

                option_write_chain!(f, width);

                let Some((height, depth)) = height_depth else {
                    return Ok(());
                };

                write!(f, " {} {}", height, depth)?;

                Ok(())
            }
            Self::Locate(location) => write!(f, "locate {}", location),
            Self::Pos(variable) => {
                f.write_str("pos")?;

                option_write_chain!(f, variable);

                Ok(())
            }
            Self::ResetClosest => f.write_str("resetclosest"),
            Self::ResetThat => f.write_str("resetthat"),
            Self::ResetThese => f.write_str("resetthese"),
            Self::Run(location, number_of_times, until_failed, rotation_step, tests_per_row) => {
                write!(f, "run {}", location)?;

                option_write_chain!(
                    f,
                    number_of_times,
                    until_failed,
                    rotation_step,
                    tests_per_row
                );

                Ok(())
            }
            Self::RunClosest(number_of_times, until_failed) => {
                f.write_str("runclosest")?;

                option_write_chain!(f, number_of_times, until_failed);

                Ok(())
            }
            Self::RunThat(number_of_times, until_failed) => {
                f.write_str("runthat")?;

                option_write_chain!(f, number_of_times, until_failed);

                Ok(())
            }
            Self::RunThese(number_of_times, until_failed) => {
                f.write_str("runthese")?;

                option_write_chain!(f, number_of_times, until_failed);

                Ok(())
            }
            Self::RunMultiple(location, amount) => {
                write!(f, "runmultiple {}", location)?;

                option_write_chain!(f, amount);

                Ok(())
            }
            Self::RunFailed(command) => write!(f, "runfailed {}", command),
            Self::Stop => f.write_str("stop"),
            Self::Verify(location) => write!(f, "verify {}", location),
            Self::Export(location) => write!(f, "export {}", location),
            Self::ExportClosest => f.write_str("exportclosest"),
            Self::ExportThat => f.write_str("exportthat"),
            Self::ExportThese => f.write_str("exportthese"),
        }
    }
}

impl From<TestCommand> for Command {
    fn from(value: TestCommand) -> Self {
        Self::Test(value)
    }
}
