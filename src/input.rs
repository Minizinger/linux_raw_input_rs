use keys::Keys;

#[derive(Debug)]
pub struct Input {
    pub e_type: u16,
    pub code: u16,
    pub value: i32
}
#[derive(Debug, PartialEq, Eq)]
pub enum EventType {
    Push,
    Release
}

impl Input {
    pub fn from_read(read: &[u8; 24]) -> Input{
        Input{
            e_type: ((read[17] << 1) as u16) + (read[16] as u16),
            code: ((read[19] << 1) as u16) + (read[18] as u16),
            value: ((read[23] << 3) as i32) + ((read[22] << 2) as i32) + ((read[21] << 1) as i32) + (read[20]as i32)
        }
    }
    pub fn is_key_event(&self) -> bool{
        self.e_type == 1
    }
    pub fn event_type(&self) -> EventType{
        match self.value {
            1 => EventType::Push,
            _ => EventType::Release
        }
    }
    pub fn get_key(&self) -> Keys{
        Keys::from_code(self.code)
    }
}