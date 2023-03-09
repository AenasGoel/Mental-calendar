use time;


struct Reminder {
    time: time::Time,
    date: time::Date,
    repeating: todo!(),
    importance: i8,
    ringtone: option<String>,
}
