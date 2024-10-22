fn input_handler(
    input: Res<ButtonInput<KeyCode>>,
    mut input_buffer: ResMut<InputBuffer>,
    heads: Query<&Head, With<Head>>,
    mut game_pause_writer: EventWriter<GamePauseEvent>,
) {
    if input.just_pressed(KeyCode::Space) {
        game_pause_writer.send(GamePauseEvent);
    }
    if let Some(head) = heads.iter().next() {
        let pressed_direction = if input.pressed(KeyCode::ArrowLeft)
            || input.pressed(KeyCode::KeyA)
            || input.pressed(KeyCode::KeyH)
        {
            Some(Direction::Left)
        } else if input.pressed(KeyCode::ArrowDown)
            || input.pressed(KeyCode::KeyS)
            || input.pressed(KeyCode::KeyJ)
        {
            Some(Direction::Down)
        } else if input.pressed(KeyCode::ArrowUp)
            || input.pressed(KeyCode::KeyW)
            || input.pressed(KeyCode::KeyK)
        {
            Some(Direction::Up)
        } else if input.pressed(KeyCode::ArrowRight)
            || input.pressed(KeyCode::KeyD)
            || input.pressed(KeyCode::KeyL)
        {
            Some(Direction::Right)
        } else {
            None
        };

        if let Some(direction) = pressed_direction {
            if !input_buffer.0.contains(direction) {
                if input_buffer.0.len() == 0 {
                    if direction != head.direction && direction != head.direction.opposite() {
                        input_buffer.0.push(direction);
                    }
                } else {
                    input_buffer.0.push(direction);
                }
            }
        }
    }
}

