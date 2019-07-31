
#[derive(Debug, PartialEq, Clone, Copy)]
#[allow(unused)]
pub enum SceneType {
    Intro,
    Cutscene,
    Game,
    Menu,
    Pause,
    Credits,
    Outro,
    Exit,
}

#[derive(Debug, PartialEq)]
pub enum SceneReturn {
    Good,
    Finished,
//    Err(String),
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum PlayerType {
    Player1,
    Player2,
    Undetermined,
}

// I should probably define a player struct with a PlayerType and other stuff in it
// and then I can pass the two players' data as a tuple to summarize in outro with a winner PlayerType