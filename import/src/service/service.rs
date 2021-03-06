use std::collections::HashSet;

use chrono::{Datelike, NaiveDate};

use crate::create_id_type;

create_id_type!(ServiceId);

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Service {
    start: NaiveDate,
    end: NaiveDate,
    weekdays: [bool; 7],
    added: HashSet<NaiveDate>,
    removed: HashSet<NaiveDate>,
}

impl Service {
    pub(crate) fn new(start: NaiveDate, end: NaiveDate, weekdays: [bool; 7]) -> Service {
        Service {
            start,
            end,
            weekdays,
            added: HashSet::new(),
            removed: HashSet::new(),
        }
    }

    pub(super) fn add_date(&mut self, date: NaiveDate) {
        self.added.insert(date);
    }

    pub(super) fn remove_date(&mut self, date: NaiveDate) {
        self.removed.insert(date);
    }

    pub(crate) fn available_at(&self, date: NaiveDate) -> bool {
        (self.regularly_available_at(date) && !self.removed.contains(&date))
            || self.added.contains(&date)
    }

    fn regularly_available_at(&self, date: NaiveDate) -> bool {
        let day = date.weekday().num_days_from_monday() as usize;
        self.start <= date && date <= self.end && self.weekdays[day]
    }
}

#[cfg(test)]
pub(crate) mod fixtures {
    use super::*;
    use std::collections::HashMap;
    use std::rc::Rc;
    use test_utils::map;

    macro_rules! services {
        ($($service:ident: $start:expr, $end:expr, $weekdays:expr);* $(;)?) => (
            $(
                pub(crate) fn $service() -> Service {
                    let start = NaiveDate::from_ymd($start.0, $start.1, $start.2);
                    let end = NaiveDate::from_ymd($end.0, $end.1, $end.2);
                    Service::new(start, end, $weekdays)
                }
            )*

            pub(crate) fn by_id() -> HashMap<ServiceId, Rc<Service>> {
                map! {
                    $( stringify!($service) => Rc::new($service()) ),*
                }
            }
        );
    }

    services! {
        mon_fri: (2019, 1, 1), (2019, 12, 31), [true, true, true, true, true, false, false];
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fixtures::services;

    #[test]
    fn test_regularly_available() {
        let service = services::mon_fri();
        let date = NaiveDate::from_ymd(2019, 1, 7);
        assert!(service.regularly_available_at(date));
        assert!(service.available_at(date));
    }

    #[test]
    fn test_regularly_unavailable() {
        let service = services::mon_fri();
        let date = NaiveDate::from_ymd(2019, 1, 5);
        assert!(!service.regularly_available_at(date));
        assert!(!service.available_at(date));
    }

    #[test]
    fn test_exceptionally_available() {
        let mut service = services::mon_fri();
        let date = NaiveDate::from_ymd(2019, 1, 5);
        service.add_date(date);
        assert!(!service.regularly_available_at(date));
        assert!(service.available_at(date));
    }

    #[test]
    fn test_exceptionally_unavailable() {
        let mut service = services::mon_fri();
        let date = NaiveDate::from_ymd(2019, 1, 7);
        service.remove_date(date);
        assert!(service.regularly_available_at(date));
        assert!(!service.available_at(date));
    }
}
