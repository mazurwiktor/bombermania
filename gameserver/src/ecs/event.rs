enum EventType {
    PlayerLogin,
    PlayerLogout,
    PlayerMove,
    WorldReset,
}

trait Event {
    fn new(et: &EventType) -> Self;
}
