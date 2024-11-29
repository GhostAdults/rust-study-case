use bevy::{prelude::*, utils::hashbrown::hash_set::Intersection};


pub(super)  fn plugin(app:&mut App){
    app.register_type::<InteractionPalette>();
    app.add_systems(Update, 
        (
            trigger_on_press,
            apply_interaction_palette
    ));
}

#[derive(Event)]
pub struct OnPress;

#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct InteractionPalette {
    pub none: Color,
    pub hovered: Color,
    pub pressed: Color,
}

fn trigger_on_press(mut commands: Commands,interaction_query: Query<(Entity,&Interaction),Changed<Interaction>>){

    for(entity,interaction) in &interaction_query {
        if matches!(interaction,Interaction::Pressed){
            commands.trigger_targets(OnPress, entity);
        }
    }
}

fn apply_interaction_palette(mut paltte_query:Query<(&Interaction,&InteractionPalette,
&mut BackgroundColor),Changed<Interaction>,>)
{
    for (interaction,palette,mut background) in &mut paltte_query {
        *background = match interaction {
              Interaction::None => palette.none,
              Interaction::Hovered => palette.hovered,
              Interaction::Pressed => palette.pressed
        }.into();
    }
}