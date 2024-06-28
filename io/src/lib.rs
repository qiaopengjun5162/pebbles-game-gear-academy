#![no_std] // 禁用标准库，仅使用适合嵌入式或最小环境的核心功能。

use gmeta::{In, InOut, Metadata, Out}; // 从 gmeta crate 导入用于定义元数据的类型。
use gstd::prelude::*; // 从 gstd 导入 prelude，用于常见的实用程序和特性。

/// Pebbles 游戏的元数据。
pub struct PebblesMetadata;

impl Metadata for PebblesMetadata {
    type Init = In<PebblesInit>; // 游戏的初始化参数。
    type Handle = InOut<PebblesAction, PebblesEvent>; // 处理的动作及对应的事件。
    type State = Out<GameState>; // 游戏的状态表示。
    type Reply = (); // 回复类型（在此示例中未使用）。
    type Others = (); // 其他元数据（在此示例中未使用）。
    type Signal = (); // 信号类型（在此示例中未使用）。
}

// 游戏初始化参数的结构体。
#[derive(Debug, Default, Clone, Encode, Decode, TypeInfo)]
pub struct PebblesInit {
    pub difficulty: DifficultyLevel, // 游戏的难度级别。
    pub pebbles_count: u32,          // 总的石子数量。
    pub max_pebbles_per_turn: u32,   // 每回合最多可以拿的石子数。
}

// 表示游戏难度级别的枚举。
#[derive(Debug, Default, Clone, Encode, Decode, TypeInfo, PartialEq, Eq)]
pub enum DifficultyLevel {
    #[default]
    Easy, // 简单难度级别。
    Hard, // 困难难度级别。
}

// 游戏中可以执行的动作的枚举。
#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub enum PebblesAction {
    Turn(u32), // 通过拿一定数量的石子来进行一回合。
    GiveUp,    // 放弃游戏的动作。
    Restart {
        difficulty: DifficultyLevel, // 使用新参数重新开始游戏。
        pebbles_count: u32,
        max_pebbles_per_turn: u32,
    },
}

// 游戏中发生的事件的枚举。
#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub enum PebblesEvent {
    CounterTurn(u32), // 程序的反击动作。
    Won(Player),      // 表示某个玩家获胜的事件。
}

// 表示游戏玩家的枚举。
#[derive(Debug, Default, Clone, Encode, Decode, TypeInfo)]
pub enum Player {
    #[default]
    User, // 用户玩家。
    Program, // 程序玩家。
}

// 表示游戏状态的结构体。
#[derive(Debug, Default, Clone, Encode, Decode, TypeInfo)]
pub struct GameState {
    pub pebbles_count: u32,          // 初始的石子总数。
    pub max_pebbles_per_turn: u32,   // 每回合最多可以拿的石子数。
    pub pebbles_remaining: u32,      // 剩余的石子数量。
    pub difficulty: DifficultyLevel, // 当前的难度级别。
    pub first_player: Player,        // 第一个行动的玩家。
    pub winner: Option<Player>,      // 游戏的获胜者（如果游戏已结束）。
}
