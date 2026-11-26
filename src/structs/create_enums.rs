
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Languages {
    Python = 0,
    JS = 1,
    TS = 2,
    Rust = 3,
    Go = 4,
    Java = 5,
    C = 6,
}
impl TryFrom<usize> for Languages {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Languages::Python),
            1 => Ok(Languages::JS),
            2 => Ok(Languages::TS),
            3 => Ok(Languages::Rust),
            4 => Ok(Languages::Go),
            5 => Ok(Languages::Java),
            6 => Ok(Languages::C),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
pub enum ExtraChoices {
    Readme,
    Gitignore,
    IDMatching,
    APIRoute,
}
impl TryFrom<usize> for ExtraChoices {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ExtraChoices::Readme),
            1 => Ok(ExtraChoices::Gitignore),
            2 => Ok(ExtraChoices::IDMatching),
            3 => Ok(ExtraChoices::APIRoute),
            _ => Err(()),
        }
    }
}

#[derive(PartialEq, Clone)]
pub enum Step {
    ChooseName = 0,
    ChooseProjectType,
    ChooseLanguage,
    ChooseExtras,
    Confirmation,
}