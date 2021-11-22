use crate::dojo::mars_rover::location::Location;

pub trait Command: Sized {
    fn execute(&self, location: &mut Location);
}
