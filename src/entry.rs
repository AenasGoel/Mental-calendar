use time;

struct Entry {
    time: time::Time,
    date: time::Date,
    duration: time::Duration,
    group: String,
    reminder: Option<()>,
    title: String,



}

impl Entry {
    fn new(time: time::Time, date: time::Date, duration: time::Duration, group: String, reminder: Option<()>, title: String) -> Self { Self { time, date, duration, group, reminder, title } }

    fn change_time(self, new_time: time::Time) -> self {
        self { new_time, date, duration, group, reminder, title }
    }

    fn change_date(self, new_date: time::Date) -> self {
        self { time, new_date, duration, group, reminder, title }
    }

    fn change_duration(self, new_duration: time::Duration) -> self {
        self { time, date, new_duration, group, reminder, title }
    }


}
