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

    /*
        TODO REVOIR LA MATRICE DES OBJECTS
        
        [X,Y,Z, Tx, Ty, Tz] = T = translation
    */
    pub fn controls(&mut self, objects: &mut Vec<(Vec<[f32; 3]>, (Vec<[f32; 3]>, (Vec<[f32; 3]>, Vec<[f32; 3]>, Vec<[f32; 3]>, f32)))> , event_pump: &mut EventPump)
    {
        // si il n'y pas d'event, osef
        if event_pump.poll_event().is_none()
        {
            return;
        }



        let keyboard_events = event_pump.keyboard_state();

        if keyboard_events.is_scancode_pressed(sdl2::keyboard::Scancode::W)
        {
            self.player_pos[2] = 10.0;
        }
        else if keyboard_events.is_scancode_pressed(sdl2::keyboard::Scancode::S)
        {
            self.player_pos[2] = -10.0;
        }


        if keyboard_events.is_scancode_pressed(sdl2::keyboard::Scancode::A)
        {
            self.player_pos[0] = 10.0;
        }
        else if keyboard_events.is_scancode_pressed(sdl2::keyboard::Scancode::D)
        {
            self.player_pos[0] = -10.0;
        }




        Self::update_change(objects, self.player_pos);


    }




    pub fn update_change(objects: &mut Vec<(Vec<[f32; 3]>, (Vec<[f32; 3]>, (Vec<[f32; 3]>, Vec<[f32; 3]>, Vec<[f32; 3]>, f32)))>, player_pos: [f32; 3])
    {
        
        for objet in objects {
            

            for m in objet.0.iter_mut() {
                
                for i in 0..3 {
                    
                    m[i] += player_pos[i];

                }

            }

        }

    }
}