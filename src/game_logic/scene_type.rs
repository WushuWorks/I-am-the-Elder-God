
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

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum PlayerType {
    Player1,
    Player2,
    Undetermined,
}
