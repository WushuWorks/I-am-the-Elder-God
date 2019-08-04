#[derive(Debug, PartialEq, Clone, Copy)]
pub enum SceneType {
    Intro,
    Game,
    Outro,
}

#[derive(Debug, PartialEq)]
pub enum SceneReturn {
    Good,
    Finished,
}
