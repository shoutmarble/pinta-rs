use glam::DVec2;

use crate::state::AppState;

pub fn begin_stroke(state: &mut AppState, image: DVec2) {
    state.history.push(format!("Pencil down at {:.0}, {:.0}", image.x, image.y));
}

pub fn extend_stroke(state: &mut AppState, image: DVec2) {
    if let Some(session) = state.pencil_session.as_mut() {
        session.points.push(image);
    }
}

pub fn end_stroke(state: &mut AppState, image: DVec2) {
    if let Some(session) = state.pencil_session.take() {
        state.history.push(format!(
            "Pencil stroke: {} points, end {:.0}, {:.0}",
            session.points.len(),
            image.x,
            image.y
        ));
    }
}
