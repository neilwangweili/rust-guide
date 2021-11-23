use crate::dojo::mars_rover_journey::coordinate::Coordinate;

pub trait Command {
    fn execute(&self, location: &mut Coordinate);
}
