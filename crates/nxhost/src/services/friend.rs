// Mush8 - friend.rs
// Constantin Clerc (c22dev) - 0.1
use crate::ipc::{Act, Service, Table};
use std::sync::Arc;

const NOTIFICATION_QUEUE_EMPTY: u32 = (15 << 9) | 121;

pub fn open() -> Arc<dyn Service> {
    Arc::new(Table {
        name: "friend",
        cmds: &[
            (0, Act::Obj(friend_service)),
            (1, Act::Obj(notification_service)),
        ],
    })
}

fn friend_service() -> Arc<dyn Service> {
    Arc::new(Table { name: "friend::IFriendService", cmds: &[(0, Act::Event(false)), (1, Act::Ok)] })
}

fn notification_service() -> Arc<dyn Service> {
    Arc::new(Table {
        name: "friend::INotificationService",
        cmds: &[
            (0, Act::Event(false)),
            (1, Act::Ok),
            (2, Act::Err(NOTIFICATION_QUEUE_EMPTY)),
        ],
    })
}
