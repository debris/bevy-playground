use bevy::prelude::*;

#[derive(Message)]
pub struct DisplayNotification {
    pub at: Vec2,
    pub text: String,
}

#[derive(Component)]
struct Notification(Timer);

fn process_dispay_notification_messaage(
    mut commands: Commands,
    mut reader: MessageReader<DisplayNotification>
) {
    for notification in reader.read() {
        commands.spawn((
            Notification(Timer::from_seconds(1.0, TimerMode::Once)),
            Text2d::new(notification.text),
        ));
    }
}

