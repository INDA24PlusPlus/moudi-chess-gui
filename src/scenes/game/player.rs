use ::core::panic;

use local::LocalPlayer;
use raylib::prelude::*;
use remote_send::RemoteSendPlayer;
use remote_recv::RemoteRecvPlayer;

pub mod local;
pub mod remote_send;
pub mod remote_recv;

pub enum PlayerTypes {
    Local(LocalPlayer),
    RemoteSend(RemoteSendPlayer),
    RemoteRecv(RemoteRecvPlayer),
}

pub trait Player {
    /// Caleld when GameState::ongoing
    fn on_ongoing(&mut self, rl: &mut RaylibHandle);
    /// Called when GameState::Promotion
    ///
    /// Return if to call update_promotion
    fn on_promotion(&mut self) -> bool;
    /// Called when GameState::End
    fn on_end(&mut self);
    /// Perform operations before attempting to play move and check with server if this is an okay
    /// move
    fn on_move_piece(&mut self) -> bool;

    /// Get slot to move selected piece to
    fn get_move(&self) -> Option<i32>;
    /// Get current selected slot by player
    fn get_selected_slot(&self) -> Option<i32>;
    /// Get current mouse position
    fn get_mouse(&self) -> (i32, i32);

    fn clear_selected(&mut self);

}

impl Player for PlayerTypes {
    fn on_ongoing(&mut self, rl: &mut RaylibHandle) {
        match self {
            Self::Local(local) => local.on_ongoing(rl),
            Self::RemoteSend(remote_send) => remote_send.on_ongoing(rl),
            Self::RemoteRecv(remote_recv) => remote_recv.on_ongoing(rl),
        }
    }

    fn on_promotion(&mut self) -> bool { 
        match self {
            Self::Local(local) => local.on_promotion(),
            Self::RemoteSend(remote_send) => remote_send.on_promotion(),
            Self::RemoteRecv(remote_recv) => remote_recv.on_promotion(),
        }
    }

    fn on_end(&mut self) {
        match self {
            Self::Local(local) => local.on_end(),
            Self::RemoteSend(remote_send) => remote_send.on_end(),
            Self::RemoteRecv(remote_recv) => remote_recv.on_end(),
        }
    }

    fn on_move_piece(&mut self) -> bool {
        match self {
            Self::Local(local) => local.on_move_piece(),
            Self::RemoteSend(remote_send) => remote_send.on_move_piece(),
            Self::RemoteRecv(remote_recv) => remote_recv.on_move_piece(),
        }
    }

    fn get_move(&self) -> Option<i32> {
        match self {
            Self::Local(local) => local.get_move(),
            Self::RemoteSend(remote_send) => remote_send.get_move(),
            Self::RemoteRecv(remote_recv) => remote_recv.get_move(),
        }
    }

    fn get_selected_slot(&self) -> Option<i32> {
        match self {
            Self::Local(local) => local.get_selected_slot(),
            Self::RemoteSend(remote_send) => remote_send.get_selected_slot(),
            Self::RemoteRecv(remote_recv) => remote_recv.get_selected_slot(),
        }
    }

    fn get_mouse(&self) -> (i32, i32) {
        match self {
            Self::Local(local) => local.get_mouse(),
            Self::RemoteSend(remote_send) => remote_send.get_mouse(),
            Self::RemoteRecv(remote_recv) => remote_recv.get_mouse(),
        }
    }

    fn clear_selected(&mut self) {
        match self {
            Self::Local(local) => local.clear_selected(),
            Self::RemoteSend(remote_send) => remote_send.clear_selected(),
            Self::RemoteRecv(remote_recv) => remote_recv.clear_selected(),
        }
    }
}
