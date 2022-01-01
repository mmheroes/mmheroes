use super::*;

#[derive(Clone, Debug)]
pub struct GameState {
    pub(in crate::logic) player: Player,
    pub(in crate::logic) current_day_index: usize,
    pub(in crate::logic) current_time: Time,
    pub(in crate::logic) timetable: timetable::Timetable,
    pub(in crate::logic) location: Location,
    pub(in crate::logic) classmates: Classmates,
    pub(in crate::logic) additional_computer_science_exams: u8,
}

impl GameState {
    pub(in crate::logic) fn new(
        player: Player,
        timetable: timetable::Timetable,
        location: Location,
    ) -> GameState {
        GameState {
            player,
            current_day_index: 0,
            current_time: Time(8),
            timetable,
            location,
            classmates: Classmates::new(),
            additional_computer_science_exams: 0,
        }
    }

    pub fn current_day(&self) -> &Day {
        &self.timetable.days()[self.current_day_index]
    }

    pub fn current_time(&self) -> Time {
        self.current_time
    }

    pub fn player(&self) -> &Player {
        &self.player
    }

    pub fn timetable(&self) -> &timetable::Timetable {
        &self.timetable
    }

    pub fn location(&self) -> Location {
        self.location
    }

    pub fn classmates(&self) -> &Classmates {
        &self.classmates
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum Location {
    PUNK = 1,
    PDMI = 2,
    ComputerClass = 3,
    Dorm = 4,
    Mausoleum = 5,
}

impl Location {
    pub fn is_exam_here_on_day(self, subject: Subject, today: &Day) -> bool {
        today
            .exam(subject)
            .map_or(false, |exam| exam.location() == self)
    }

    pub fn is_exam_here_now(self, subject: Subject, today: &Day, time: Time) -> bool {
        today.exam(subject).map_or(false, |exam| {
            exam.location() == self && time >= exam.from() && time < exam.to()
        })
    }
}
