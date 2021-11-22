use crate::dojo::mars_rover::location::Location;

pub trait Command {
    fn execute(&self, location: &mut Location);
}
