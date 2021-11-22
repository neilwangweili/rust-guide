use crate::dojo::mars_rover_journey::location::Location;

pub trait Command {
    fn execute(&self, location: &mut Location);
}
