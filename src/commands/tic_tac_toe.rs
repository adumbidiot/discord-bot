use serenity::{
    client::Context,
    framework::standard::{
        Args,
        Command,
        CommandError,
        CommandOptions,
    },
    model::{
        channel::Message,
        id::UserId,
    },
};
use std::{
    collections::HashMap,
    sync::{
        Arc,
        RwLock,
    },
};
use ttt::{
    NodeIndex,
    AI,
};

#[derive(Debug, Clone, PartialEq)]
enum Team {
    X,
    O,
    Neither,
}

impl Team {
    fn inverse(&self) -> Option<Team> {
        match self {
            Team::X => Some(Team::O),
            Team::O => Some(Team::X),
            Team::Neither => None,
        }
    }

    fn as_char(&self) -> char {
        match self {
            Team::X => 'X',
            Team::O => 'O',
            Team::Neither => ' ',
        }
    }

    fn as_u8(&self) -> u8 {
        match self {
            Team::Neither => 0,
            Team::X => 1,
            Team::O => 2,
        }
    }

    fn from_u8(val: u8) -> Option<Team> {
        match val {
            0 => Some(Team::Neither),
            1 => Some(Team::X),
            2 => Some(Team::O),
            _ => None,
        }
    }

    fn from_char(val: char) -> Option<Team> {
        match val {
            'X' | 'x' => Some(Team::X),
            'O' | 'o' => Some(Team::O),
            ' ' => Some(Team::Neither),
            _ => None,
        }
    }
}

impl Into<u8> for Team {
    fn into(self) -> u8 {
        self.as_u8()
    }
}

impl From<Team> for char {
    fn from(team: Team) -> char {
        team.as_char()
    }
}

#[derive(Debug, Clone)]
struct State {
    board: NodeIndex,
    player_team: Team,
}

pub struct TicTacToe {
    opts: Arc<CommandOptions>,
    state: RwLock<HashMap<UserId, State>>,
    ai: AI,
}

impl TicTacToe {
    pub fn new() -> TicTacToe {
        let mut opts = CommandOptions::default();
        opts.desc = Some(String::from("Play Tic Tac Toe"));

        let mut ttt_compiler = ttt::Compiler::new();
        ttt_compiler.compilation = Some(Box::new(ttt::ttt::TTTCompilation::new()));
        ttt_compiler
            .init_compilation()
            .expect("Error Starting Compilation");

        while ttt_compiler.queue.len() != 0 {
            ttt_compiler.process().unwrap();
        }

        while ttt_compiler.winners.len() != 0 {
            ttt_compiler.post_process().unwrap();
        }

        while ttt_compiler.unscored_nodes.len() != 0 {
            ttt_compiler.score_nodes().unwrap();
        }

        let mut ttt_ai = ttt::AI::new();
        ttt_ai.load(ttt_compiler.export().unwrap());

        TicTacToe {
            opts: Arc::from(opts),
            state: Default::default(),
            ai: ttt_ai,
        }
    }
}

impl Command for TicTacToe {
    fn execute(&self, _: &mut Context, msg: &Message, mut args: Args) -> Result<(), CommandError> {
        let input: String = args.single()?;

        let author_id = msg.author.id;

        let team_option = input.parse().ok().and_then(Team::from_char);
        if team_option.is_some() && team_option != Some(Team::Neither) {
            let team = team_option.unwrap();
            let node_index = match team {
                Team::X => self.ai.get_move(0, Team::X.into()).unwrap(),
                Team::O => 0,
                _ => unreachable!(),
            };
            msg.channel_id.say(render_board_basic(&node_index))?;
            self.state.write().unwrap().insert(
                author_id,
                State {
                    board: node_index,
                    player_team: team.inverse().unwrap(),
                },
            );
        } else {
            let turn: u8 = match input.parse() {
                Ok(t) => t,
                Err(_) => {
                    msg.channel_id.say("Unrecognized Input")?;
                    return Ok(());
                }
            };

            let state = match self.state.read().unwrap().get(&author_id) {
                Some(state) => state.clone(),
                None => {
                    msg.channel_id.say("No game detected. Start a new one.")?;
                    return Ok(());
                }
            };

            let tile = (3 as NodeIndex).pow(turn.into());

            if ((state.board / tile) % 3) != 0 {
                msg.channel_id.say("Invalid Move")?;
            } else {
                let ai_team = state.player_team.inverse().unwrap();
                let player_node_index = state.board + (tile * state.player_team.as_u8() as u128);

                if is_tie(player_node_index.clone()) {
                    msg.channel_id.say("Tie")?;
                    self.state.write().unwrap().remove(&author_id);
                }

                let winner = Team::from_u8(ttt::ttt::get_winner(&player_node_index, 3)).unwrap();
                if winner != Team::Neither {
                    msg.channel_id
                        .say(&format!("Winner: {}", winner.as_char()))?;
                    self.state.write().unwrap().remove(&author_id);
                }

                let node_index = self.ai.get_move(player_node_index, ai_team.into()).unwrap();

                self.state
                    .write()
                    .unwrap()
                    .get_mut(&author_id)
                    .unwrap()
                    .board = node_index;

                msg.channel_id.say(render_board_basic(&node_index))?;

                if is_tie(node_index.clone()) {
                    msg.channel_id.say("Tie")?;
                    self.state.write().unwrap().remove(&author_id);
                }

                let winner = Team::from_u8(ttt::ttt::get_winner(&node_index, 3)).unwrap();
                if winner != Team::Neither {
                    msg.channel_id
                        .say(&format!("Winner: {}", winner.as_char()))?;
                    self.state.write().unwrap().remove(&author_id);
                }
            }
        }

        Ok(())
    }

    fn options(&self) -> Arc<CommandOptions> {
        self.opts.clone()
    }
}

#[derive(Debug)]
struct TileIter {
    node_index: NodeIndex,
    count: usize,
}

impl TileIter {
    fn new(node_index: NodeIndex) -> Self {
        TileIter {
            node_index,
            count: 0,
        }
    }
}

impl Iterator for TileIter {
    type Item = Team;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count == 9 {
            return None;
        }

        self.count += 1;
        let ret = Team::from_u8((self.node_index % 3) as u8).unwrap();
        self.node_index /= 3;

        Some(ret)
    }
}

fn is_tie(node_index: NodeIndex) -> bool {
    TileIter::new(node_index).all(|team| team != Team::Neither)
}

fn render_board_basic(node_index: &NodeIndex) -> String {
    let board_size = 3;
    let reserve_size = 2 * board_size * board_size;
    let start = String::with_capacity(reserve_size);

    (b'0'..b'9')
        .zip(TileIter::new(node_index.clone()))
        .enumerate()
        .map(|(i, (tile, value))| {
            let sep = if (i + 1) % 3 == 0 { '\n' } else { ' ' };

            match value {
                Team::Neither => [tile as char, sep],
                _ => [value.into(), sep],
            }
        })
        .fold(start, |mut state, el| {
            state.extend(&el);
            state
        })
}
