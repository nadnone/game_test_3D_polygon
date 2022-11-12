use sdl2::EventPump;

pub struct EventControls {
    player_pos: [f32; 3]
}


impl EventControls {

    // TODO FAIRE LES EVENT INPUT CLAVIER SOURIS

    pub fn init(x: f32, y: f32, z: f32) -> EventControls
    {
        return EventControls {
            player_pos: [x, y, z]
        };
    }

    pub fn get_pos_camera(&self) -> [f32; 3]
    {
        return self.player_pos;
    }
    

    pub fn controls(&mut self , event_pump: &mut EventPump) -> bool
    {
        // si il n'y pas d'event, osef
        if event_pump.poll_event().is_none()
        {
            return false;
        }



        let keyboard_events = event_pump.keyboard_state();

        if keyboard_events.is_scancode_pressed(sdl2::keyboard::Scancode::W)
        {
            self.player_pos[2] += -0.5;
        }
        else if keyboard_events.is_scancode_pressed(sdl2::keyboard::Scancode::S)
        {
            self.player_pos[2] += 0.5;
        }


        if keyboard_events.is_scancode_pressed(sdl2::keyboard::Scancode::A)
        {
            self.player_pos[0] += 10.0;
        }
        else if keyboard_events.is_scancode_pressed(sdl2::keyboard::Scancode::D)
        {
            self.player_pos[0] += -10.0;
        }


        //  quitter le programme
        if keyboard_events.is_scancode_pressed(sdl2::keyboard::Scancode::Escape)
        {
            return true;
        }



        return false;
    }




}