use super::worker::Job;
pub enum Message {
    NewJob(Job),
    Terminate,
}
